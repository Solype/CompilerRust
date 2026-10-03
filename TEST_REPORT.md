# x86-64 encoder test report

October 2, 2026, updated after fixing every bug found · online version (French): https://claude.ai/code/artifact/5f97e6cd-52b8-4205-bc58-d1c65b70267f

## Summary

> The figures below date from this test campaign. Tests have been added since with new
> instructions; the current count is in `TODO_Backend.md` (1248 tests, all passing).

1,011 unit tests now cover every instruction family of the encoder, including with the `r8`–`r15` registers and extended addresses. They found **199 wrong encodings**, due to **12 causes**, and **all of them are fixed**. The fixes also revealed two relocation bugs, fixed as well.

`cargo test` passes entirely: 1,011 tests OK, no failure and no ignored test. A new bug is marked `#[ignore = "BUG: produces ..."]` until it is fixed, and `cargo test -- --ignored` lists them.

The shared addressing code (`utils.rs` / `modrm.rs`) carried 5 of the 12 causes and 42 of the 76 cases of the first run; the 123 cases found afterwards came from encoders that did not pass their operand to `emit_rex` (cause 5).

## Method

The expected bytes come from an independent reference, never from the encoder itself.

1. Each case pairs a Rust instruction with its Intel-syntax assembly equivalent. GNU `as` assembles the latter and provides the expected bytes. For multi-byte NOPs, the reference is the table of the Intel manual (SDM vol. 2B).
2. A Python script generates the Rust test files from this list, then `cargo test` is run.
3. Each failure is disassembled with `objdump`. If it decodes to the same instruction as the reference, the encoding is valid and added as an accepted alternative (`[...] | [...]`). Otherwise it is a bug, marked `#[ignore]` with the disassembly of what is produced.
4. Three families of false positives were reclassified by hand as valid: the redundant `REX.W` on `push`/`pop`, `int 3` encoded `cd 03` instead of `cc`, and the explicit `ds:` prefix that `as` drops.

Relocations (offset, type, addend) are checked separately, against the output of `readelf -r` on the same code assembled by `as`.

## Coverage

Out of 994 encoding cases, 906 produce exactly the bytes of GNU `as`, 88 a different but valid encoding, and none a wrong encoding. The 17 relocation tests all pass.

- **Families:** 18 files (mov, alu, sse, complex, unary, no_operand, sys, shift, bit, bitscan, cmov, setcc, lea, stack, string, prefix, ctrl, conversion), plus `relocations.rs`.
- **Sizes:** 8, 16, 32 and 64 bits, with the extended registers `r8`–`r15` and `xmm8`–`xmm15`.
- **Addressing:** 19 modes, and each family also tested with `r8`–`r15` as base or index (`[r9]`, `[rbx+r8*2]`, `[r10+rax]`), including `[rsp]`, `[rbp]`, `[r12]`, `[r13]`, `[rbp+rcx]`, `[rbx+r8*2]`, `[rcx*4+16]` without base, `[rip+sym]` and the absolute address with an `fs:` segment.
- **Relocations:** offset, type and addend of jumps, of `loop`, and of `[rip+sym]` accesses followed by an immediate or preceded by `lock`.

Results per family, sorted by number of wrong cases:

| Family | Identical to `as` | Valid equivalents | Wrong |
| --- | ---: | ---: | ---: |
| alu | 166 | 49 | 0 |
| shift | 125 | 0 | 0 |
| mov | 94 | 19 | 0 |
| sse | 96 | 0 | 0 |
| complex | 86 | 0 | 0 |
| unary | 60 | 0 | 0 |
| bit | 56 | 0 | 0 |
| stack | 12 | 18 | 0 |
| no_operand | 29 | 0 | 0 |
| string | 29 | 0 | 0 |
| cmov | 28 | 0 | 0 |
| ctrl | 23 | 0 | 0 |
| setcc | 23 | 0 | 0 |
| lea | 23 | 0 | 0 |
| bitscan | 20 | 0 | 0 |
| conversion | 20 | 0 | 0 |
| prefix | 13 | 1 | 0 |
| sys | 3 | 1 | 0 |

The 88 equivalent encodings are concentrated in `alu` (49), `mov` (19) and `stack` (18): they are valid encoding choices, detailed in the possible optimizations.

## Bugs found

The first run found 76 wrong cases; the tests added afterwards for `r8`–`r15` found 123 more, all in cause 5. **The 12 causes and the 199 cases are fixed.**

While fixing cause 7, two relocation bugs appeared that the tests did not cover until then: with a SIB byte (`[rbx+rcx*4+sym]`, `[rsp+sym]`, `[r12+sym]`), the symbol relocation pointed one byte too early, at the SIB; and `.absolute()` with a symbol had an addend of -4 instead of 0. Both are fixed and covered by 5 relocation tests, checked against `readelf -r` on the output of GNU `as`.

| # | Cause | Cases | Example (expected → produced) | File |
| --- | --- | --- | --- | --- |
| 1 | **Fixed.** Opcodes of `jg`, `jl` and `jle` swapped | 3 | `jg` → `jl` | `encode_ctrl.rs` |
| 2 | **Fixed.** `REX.B`/`REX.X` never emitted for the base or index of a memory address | 19 | `[r9]` → `[rcx]`, `[rbx+r8*2]` → `[rbx+rax*2]` | `utils.rs` (`emit_rex`) |
| 3 | **Fixed.** 64-bit immediate emitted where x86 only accepts 32 bits: 4 stray bytes shift the rest of the code | 11 | `add qword [rbx+8], 0x1000`, `test r9, 1` | `utils.rs` (`emit_imm_sx32`), `encode_alu`, `encode_complexbin.rs` |
| 4 | **Fixed.** `r12`/`r13` as base: SIB and disp8 forgotten (test on `index` instead of `low3()`) | 9 | `[r12]`, `[r13]` → invalid bytes | `modrm.rs` |
| 5 | **Fixed.** r/m operand or `reg` register not passed to `emit_rex`: `REX.R`/`REX.X`/`REX.B` missing | 134 | `add qword [r9], 1` → `[rcx]`; `shl r10, 7` → `shl rdx, 7`; `bsf r8, rbx` → `bsf rax, rbx`; `imul r10, r11` → `imul rdx, r11` | `encode_alu/encode.rs`, `encode_shift_rotate.rs`, `encode_bitscan.rs`, `encode_stack.rs`, `encode_complexbin.rs` |
| 6 | **Fixed.** `sil`, `dil`, `spl`, `bpl` without the mandatory `0x40` REX | 10 | `mov sil, al` → `mov dh, al` | `utils.rs` (`emit_rex`) |
| 7 | **Fixed.** `.absolute()` encodes `rip`-relative addressing | 3 | `mov eax, [0x1000]` → `[rip+0x1000]`; `fs:[0x28]` wrong too | `modrm.rs` |
| 8 | **Fixed.** `[rbp+index]` without displacement: `mod=00` with base `101` means "no base" | 2 | `[rbp+rcx]` → invalid | `modrm.rs` |
| 9 | **Fixed.** `[index*scale+disp]` without base encoded with a disp8 | 2 | `[rcx*4+16]` → `[rbp+rcx*4+16]` | `modrm.rs` |
| 10 | **Fixed.** `movzx`/`movsx` with a 16-bit source: the destination becomes 16 bits | 4 | `movzx eax, bx` → `movzx ax, bx` | `encode_alu` |
| 11 | **Fixed.** `cwd` without the `66` prefix | 1 | `cwd` → `cdq` | `encode_unary.rs` |
| 12 | **Fixed.** `lock` rejected on the whole `Unary` family (panic) | 1 | `lock inc qword [rbx]` | `encode_prefix.rs` |

Besides, there is no way to express `movsxd` (32 to 64-bit extension).

## Possible optimizations

The 88 "equivalent" encodings are correct; some cost one byte more than needed.

- **ALU between registers:** the encoder always uses the `reg, r/m` form (`03 d8` instead of `01 c3` for `add ebx, eax`). No size impact.
- **Immediate with `eax`:** `add eax, 0x1000` uses `81 c0` instead of the `05` shortcut, and `test eax, imm` does not use `a9`. 1 byte lost per instruction.
- **`push`/`pop`:** a useless `REX.W` is emitted when `size` is `None`, while 64 bits is already the implicit size. 1 byte lost per instruction.
- **`int 3`:** encoded `cd 03` (2 bytes) instead of the short form `cc`.

## Test layout

All tests are in `src/elf/instructions/encode_tests/`, one file per family; no code file contains any. The `cvtsi2sd` tests, first written in `encode_conversion.rs`, were moved to `encode_tests/conversion.rs`.

- **`mod.rs`** defines `check` and the `cases!` macro: `name: instruction => [bytes] | [other valid encoding];` produces a `#[test]`, with the assembly instruction as a `///` comment.
- **Generated files:** 17 families, from `alu.rs` to `unary.rs`.
- **Hand-written files:** `conversion.rs` (20 tests) and `relocations.rs` (17 tests).
- **`encode_tests` folder rather than `tests`:** the `test*` pattern of `.gitignore` would have excluded the folder from the repository.

Changes elsewhere: `src/elf/instructions/mod.rs` declares `#[cfg(test)] mod encode_tests;`, and `src/samples/mod.rs` makes `helpers` `pub(crate)` so the tests reuse `bin`, `reg`, `mem`, `at`, etc.

To run:

```bash
cargo test                      # 1,011 OK, none ignored
cargo test -- --ignored         # known unfixed bugs (none today)
cargo test encode_tests::mov    # a single family
```

When a bug is fixed, just remove the `#[ignore]` from the matching case.

## Next steps

Every bug found is fixed. What remains is evolution rather than fixing: express a 64-bit destination for `movzx`/`movsx` and add `movsxd`, which needs a second size in the instruction; remove the useless `REX.W` from `push`/`pop` and the superfluous `0x40` REX from `movzx esi, al`; test the `panic!` on immediates outside i32; and keep the test generator in the repository.

- [x] Restore the opcodes of `jg` (`0x8F`), `jl` (`0x8C`) and `jle` (`0x8E`) in `encode_ctrl.rs`
- [x] Make `emit_rex` compute the `REX.B`/`REX.X` bits from the base and index of memory operands (cause 2)
- [x] Restore in `encode_conversion.rs` the "source = general register or memory" check removed with the old `rm` (2 failing tests)
- [x] Emit in `emit_rex` a bare REX for `spl`/`bpl`/`sil`/`dil` in 8 bits, on general registers only (cause 6)
- [x] In `modrm.rs`, test `low3()` instead of `index` for `rsp`/`r12` and `rbp`/`r13`, and fix `[rbp+index]` (causes 4 and 8)
- [x] In `modrm.rs`, encode `.absolute()` with a `0x25` SIB (`mod=00`, `rm=100`, no base nor index) and `[index*scale+disp]` always as `mod=00` + base `101` + disp32 (causes 7 and 9)
- [x] Limit immediates to signed 32 bits for the ALU and `mov` to memory in 64 bits (cause 3)
- [x] Pass the right operand to `emit_rex` in the memory/immediate ALU, shifts, `push`/`pop`, `imul` and `encode_bitscan.rs` (cause 5)
- [x] Fix 16-bit `movzx`/`movsx`, `cwd` and `lock` on unaries (causes 10 to 12)
- [ ] Add the test generator to the repository (for example `tools/gen_encode_tests.py`): it currently lives in a temporary session folder
