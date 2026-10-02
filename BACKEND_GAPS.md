# Backend gaps

State of the backend on October 2, 2026 (1225 tests passing), checked against the code rather than
only against `TODO.md`.

**In short:** the instruction encoder is nearly complete for a compiler. What is mostly missing is
the ELF layer around **data**: code can be written well, data cannot be described well yet.

## 1. Blocking for code generation

### a. ~~Per-object alignment in a section~~ (done)

**Fixed:** `add_symbol_to_section_raw` takes an `align` and pads the section before the object
(zeros, `int3` in an executable section); `natural_alignment(size)` gives the default and the
section's `sh_addralign` is raised when needed. The issue was:

`add_symbol_to_section_raw` put each object **right after the previous one**
(`st_value = current section size`). The alignment given to `add_section` only applies to the
**start of the section**. A 16-byte mask for `xorpd` / `andpd` placed after an 8-byte `double`
ends up at offset 8, misaligned, and the program crashes. See [Alignment in detail](#alignment-in-detail).

→ Add an `align` parameter when adding an object, which inserts padding before it.

### b. Relocations in data

Relocations can **only come from instructions**. There is no way to put the address of a symbol
into `.data` / `.rodata`, which is needed for:

- `char *msg = "hello";` (a global pointer to a string);
- function pointer arrays, vtables;
- **`switch` jump tables**: `jmp [rbx+rcx*8]` exists, but the table it reads cannot be filled.

→ An API such as `add_object_with_relocs(…, bytes, &[(offset, sym, RelocKind)])` feeding the same
`pending_relocs` as instructions. With `R_X86_64_64` this is the equivalent of `.quad sym`.

### c. Separate `Label` and `Sym` (already in `TODO.md`)

A string is both a local label and a global symbol. The code generator will produce thousands of
labels (`.L_if_3`, `.L_loop_12`, …) that must not collide with user function names nor show up in
the symbol table.

## 2. Useful, not blocking

| Gap | Why | Effort |
| --- | --- | --- |
| **`.bss`** | `SectionName::Bss` and `ShType::NoBits` exist, but space cannot be reserved without writing bytes: uninitialized globals would go to `.data` with zeros in the file. Works, wastes space. | Small |
| **`GOTPCREL`** | Needed to read a libc **variable** in PIE (`stdout`, `errno`). Function **calls** already work through PLT32. | Small |
| **Local jumps resolved in place** | Every `jmp` / `jcc` to a label of the same function produces a relocation resolved by `ld`. Correct, but GNU as resolves them itself: smaller `.o`, and `rel8` (2 bytes instead of 5-6) becomes possible. | Medium |
| **dst/src order of `StoreF`** | The register is in `dst` while it is the source: a trap for the code generator. | Small |
| **"Distinct memory sizes"** | In `TODO.md`, scope unclear. | ? |

## 3. Tests and tooling

- Test the `panic!` of `emit_imm_sx32` (immediate outside i32).
- Missing test tooling listed in `TODO.md`: `ndisasm`, decoding.
- **Put `gen_encode_tests.py` in the repository**: it only exists in a temporary session folder,
  the one part of the project that can be lost.

## 4. Already in place for code generation

- **Integers**: full arithmetic, `imul` / `idiv` / `cqo`, shifts, `setcc` / `cmovcc`,
  `movzx` / `movsx` / `movsxd`.
- **Floats**: `sd` / `ss` arithmetic, `sqrt`, `min` / `max`, negation (`xorpd`), `fabs` (`andpd`),
  register copy and aligned 16-byte load / store (`movapd` / `movaps`), comparisons
  (`comisd` / `ucomisd`), every int ↔ float conversion, `movd` / `movq`.
- **Control**: direct and indirect `call` / `jmp`, libc calls in PIE (PLT32), `syscall`.
- **ELF**: symbols resolved when writing (automatic externals, globals defined later), relocations
  matching GNU as.

## 5. Later, not needed

Short encodings (`05` / `A9`, `int3`, `REX.W` of `push` / `pop`), packed SSE and AVX, BMI,
`popcnt` / `lzcnt` / `tzcnt` / `bswap`, fences, x87.

## Recommended order

1. ~~**Per-object alignment**~~: done.
2. **Relocations in data**: no `switch`, global pointers or function tables without it.
3. **Separate `Label` / `Sym`** before writing the code generator, since it changes the IR it emits.

---

## Alignment in detail

### What it is

An address is aligned on `N` when it is a multiple of `N`. A `double` is naturally aligned on 8,
a 128-bit SSE value on 16.

### What the CPU requires

| Access | Misaligned |
| --- | --- |
| Integer `mov`, `add`, … | Allowed, at most slower (two cache lines) |
| Scalar SSE: `movsd`, `addsd`, `sqrtsd`, `comisd`, … (8 / 4 bytes) | Allowed |
| **Packed SSE with a memory operand: `xorpd`, `andpd`, `xorps`, `andps`, `movaps`, …** (16 bytes) | **#GP → SIGSEGV** |
| `lock` across two cache lines | Allowed but very slow ("split lock"), may be trapped by the kernel |

`XorF` and `AndF` are 128-bit operations even though they serve scalar code (negation, `fabs`):
their memory operand is 16 bytes and must be 16-byte aligned.

### Three levels of alignment

1. **Section** (`sh_addralign`): `ld` places the section at an address multiple of it.
   Already handled by `add_section`.
2. **Object inside the section**: the object's offset in the section must be a multiple of its
   alignment. Since the section starts aligned, `section address + offset` is then aligned too.
   **Now handled** by `add_symbol_to_section_raw` (objects used to be appended back to back).
3. **Stack** (code generator, not ELF): SysV requires `rsp % 16 == 0` right before a `call`. The
   prologue / frame size must keep it, otherwise libc functions using `movaps` on the stack crash.

### Reproduction

```rust
let rodata = add_section(&mut elf_file, SectionName::Rodata, ShFlags::NoFlag, 16);
add_object(&mut elf_file, rodata, "value", &2.5f64.to_le_bytes());   // offset 0, 8 bytes
add_object(&mut elf_file, rodata, "sign_mask", &mask_16_bytes);      // offset 8, not 16
// _start: movsd xmm0, [value] ; xorpd xmm0, [sign_mask] ; ...
```

```
sign_mask  0x401008   ← not a multiple of 16
Segmentation fault (core dumped)
```

The same code with `sign_mask` added first (offset 0) runs and returns the expected result, and
so does the code above since the fix (`sign_mask` moves to `0x401010`).

### Fix (implemented)

When adding an object with alignment `align` (a power of two):

1. `offset = align_up(section.len(), align)`, where `align_up(x, a) = (x + a - 1) & !(a - 1)`
   (already in `packing.rs`).
2. Pad the section with zeros up to `offset` (with NOPs or `int3` in `.text`).
3. Set the symbol's `st_value = offset`, then append the bytes.
4. Raise the section's `sh_addralign` to `align` if it is smaller, otherwise level 1 no longer
   guarantees level 2.

A natural default is the object size rounded to a power of two, capped at 16
(`f64` → 8, 16-byte mask → 16). It over-aligns some objects (a 3-byte string gets 4), which is
harmless; strings can pass 1 to avoid the padding.
