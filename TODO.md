TODO x86/x86_64 IR

==================================================
PROGRESS (estimate)
==================================================

Backend (x86-64 encoder + ELF) : ~75-80 %
Full compiler                  : ~30 %

    x86-64 encoder + ELF           ~75-80 %
    Lexer                          ~5 %  (automaton, not wired in yet)
    Parser + AST                   0 %
    Semantic analysis              0 %
    Code generation AST -> IR      0 %
    CLI                            0 %

==================================================
DONE
==================================================

ELF:
[X] ELF headers (ehdr, shdr, phdr), in 32 and 64 bits
[X] Strtab / shstrtab
[X] Sections with alignment padding
[X] Global symbols (functions and data)
[X] Undefined symbols (external, resolved at link time)
[X] Local symbols (labels)
[X] Relocations (rela), including 8-bit ones and under a lock prefix
[X] R_X86_64_PLT32 for call / jmp / jcc rel32 (as GNU as >= 2.31 does),
    needed to call the dynamic libc in PIE (gcc -pie)
[X] R_X86_64_32S for sign-extended absolutes (ModRM disp32, imm32 of a
    64-bit ALU op, push imm32), R_X86_64_8 / 16 for mov r8/r16, sym
[X] Object file linkable with ld

Encoding:
[X] ModRM / SIB / displacements
[X] Addressing [base + index * scale + disp]
[X] RIP-relative and absolute addressing
[X] REX prefix, extended registers R8 to R15
[X] 8 / 16 / 32 / 64-bit sizes
[X] Immediates as i64
[X] Registers XMM0 to XMM15
[X] EncodeInformation to factor the encoding

Data transfer:
[X] Mov (reg, imm, mem, sym, imm64)
[X] Movzx / Movsx
[X] Xchg
[X] Lea
[X] CMovCC (every condition)

Arithmetic / logic:
[X] Add / Sub / Adc / Sbb
[X] And / Or / Xor
[X] Cmp / Test
[X] Inc / Dec / Neg / Not
[X] Mul / Div / Idiv
[X] Imul with 1, 2 and 3 operands
[X] Shl / Shr / Sar / Rol / Ror (imm and cl)
[X] Bt / Bts / Btr / Btc
[X] Bsf / Bsr
[X] SetCC (every condition)

SSE scalar floats (ss if U32, sd otherwise):
[X] LoadF / StoreF (movss / movsd)
[X] AddF / SubF / MulF / DivF
[X] ComiF / UcomiF
[X] Cvtsi2sd (integer -> double)

Control:
[X] Jmp / Call / Ret / IRet
[X] JmpCC (every condition)
[X] Loop / Loope / Loopne
[X] Syscall / Sysenter / Int

Stack:
[X] Push / Pop
[X] Pushf / Popf
[X] Enter / Leave

Strings:
[X] Movs / Cmps / Scas / Lods / Stos

Prefixes:
[X] Lock / Rep / Repe / Repne (with validation)
[X] Segment override (Cs, Ds, Es, Ss, Fs, Gs)

Instructions without operand:
[X] Nop (1 to 9 bytes)
[X] Cbw / Cwde / Cdqe
[X] Cwd / Cdq / Cqo
[X] Clc / Stc / Cmc
[X] Cld / Std
[X] Cli / Sti
[X] Lahf / Sahf
[X] Ud2 / Hlt / Pause / Fwait

Atomics:
[X] Xadd
[X] Cmpxchg

Tests:
[X] 1119 unit tests (src/elf/instructions/encode_tests/), reference bytes
    produced by GNU as, one family per file
[X] Relocation tests (offset, type, addend) compared with readelf
[X] Symbol table tests (src/elf/file/symbol_tests.rs)
[X] Fixed the 199 wrong encodings found (see TEST_REPORT.md)

Organization:
[X] main.rs reduced to building the ELF
[X] Test programs moved to src/samples/
[X] One test function per instruction family (same name in the binary)

==================================================
TODO: PRIORITY (needed by the compiler)
==================================================

Backend:
[ ] Indirect call / jmp (function pointers, GOT, switch tables):
    [X] call reg / jmp reg          (FF /2, FF /4)
    [X] call [mem] / jmp [mem]      (FF /2, FF /4 + memory ModRM)
    [X] call [rip+sym]              (PC32 relocation)
    [ ] call [rip+sym@GOTPCREL]     (new RelocKind, for the dynamic libc / PIE)
[X] Encoding fixes found:
    [X] IRet: 48 CF (iretq) in 64 bits, CF (iretd) in 32 bits
    [X] Absolute [disp32] ([sym], [base+sym]): R_X86_64_32S like GNU as
    [X] add/sub/... r64, sym: emitted an imm64 (4 extra bytes), now imm32 + 32S
    [X] Relocation symbols resolved when writing (resolve_relocations):
        a name never defined becomes GLOBAL UND (it was LOCAL UND: segfault),
        a global called before its definition is no longer stored among locals
    [X] 32-bit addend (rel) written on the field size (loop rel8 panicked)
[X] int <-> float conversions (one line per opcode in ConvOp):
    [X] Cvtsi2sd
    [X] Cvtsi2ss
    [X] Cvttsd2si / Cvttss2si / Cvtsd2si / Cvtss2si
    [X] Cvtsd2ss / Cvtss2sd
[X] Add Movsxd
[X] Movzx / Movsx into a 64-bit destination (REX.W)
[X] Separate source/destination sizes (Extend family: src_size + size)
[ ] Handle distinct memory sizes
[ ] Add ImmediateFloat(f64) (float constants in .data/.rodata)
[ ] Separate Label and Sym
[ ] Review the dst/src order of StoreF (the register is in dst)
[ ] Common scalar SSE:
    [ ] Xorps / Xorpd (zeroing, sign flip)
    [ ] Andpd / Andps (absolute value)
    [ ] Movq / Movd (GPR <-> XMM, bitcast and passing constants)
    [ ] Movaps / Movapd (XMM -> XMM copy)
    [ ] Sqrtsd / Sqrtss
    [ ] Minsd / Maxsd / Minss / Maxss
[ ] Add automated tests:
    [X] encoding (compare bytes with a GNU as reference)
    [X] objdump (sorting out equivalent encodings)
    [ ] ndisasm
    [ ] decoding
    [ ] panic! on immediates outside i32 (emit_imm_sx32)

Front-end:
[ ] Lexer:
    [ ] Wire lexical_analisys into main.rs
    [ ] Define the language tokens
    [ ] Positions (line/column) for errors
[ ] Parser:
    [ ] Source language grammar
    [ ] AST construction
    [ ] Syntax error messages
[ ] Semantic analysis:
    [ ] Symbol table / scopes
    [ ] Type checking
[ ] Code generation:
    [ ] AST -> Vec<Instruction>
    [ ] Register allocation
    [ ] Stack handling (local variables, prologue/epilogue)
    [ ] x86_64 SysV calling convention
    [ ] Global variables and constants in .data/.rodata
[ ] CLI:
    [ ] Input file
    [ ] Output file
    [ ] Options

==================================================
TODO: LATER (not needed by the compiler)
==================================================

[ ] Add SIMD registers YMM, ZMM (XMM done)
[ ] Add segment registers
[ ] Add control/debug registers

[ ] Short jumps (jmp / jcc rel8) when the target is close
[ ] Ret imm16
[ ] Int3 (short form CC instead of CD 03)
[ ] Endbr64 (if the target enables CET / IBT)

[ ] Encoding optimizations (valid but one byte too long):
    [ ] Remove the useless REX.W from push / pop
    [X] Remove the superfluous REX 0x40 from movzx esi, al
    [ ] Short accumulator form (05 / A9 instead of 81 / F7)

[ ] Keep the test generator (gen_encode_tests.py) in the repository

[ ] Add minimal SSE2:
    [ ] Movups
    [ ] Movdqa
    [ ] Movdqu
    [ ] Addps
    [ ] Subps
    [ ] Mulps
    [ ] Divps

[ ] Add SIMD integer:
    [ ] Pxor
    [ ] Pand
    [ ] Paddd

[ ] Add SIMD shuffle:
    [ ] Shufps
    [ ] Pshufd

[ ] Add basic AVX
[ ] Add BMI1/BMI2

[ ] Add Popcnt
[ ] Add Lzcnt
[ ] Add Tzcnt
[ ] Add Bswap

[ ] Add memory fences:
    [ ] Mfence
    [ ] Lfence
    [ ] Sfence

[ ] Add:
    [ ] Cmpxchg8b
    [ ] Cmpxchg16b

[ ] Add:
    [ ] Cpuid
    [ ] Rdtsc
    [ ] Rdmsr
    [ ] Wrmsr

[ ] Add:
    [ ] Jecxz
    [ ] Jrcxz

[ ] Add string ops:
    [ ] Ins
    [ ] Outs

[ ] Add x87 FPU support:
    [ ] Fld
    [ ] Fstp
    [ ] Fadd
    [ ] Fmul
    [ ] Fcom

[ ] Add:
    [ ] Prefetch
    [ ] Clflush

[ ] Think about a generic model:
    [ ] Opcode
    [ ] Nullary
    [ ] Unary
    [ ] Binary
    [ ] Ternary

[ ] Check full x86_64 SysV ABI coverage
[ ] Check REX/VEX/EVEX encoding
