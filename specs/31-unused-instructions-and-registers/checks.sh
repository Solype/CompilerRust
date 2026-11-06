# #31: instructions that do not exist yet: a variant in the encoder is all a test can ask for.
# Give each one its bytes in encode_tests/ with the variant.
ENUMS=src/elf/instructions/enums.rs
REGS=src/elf/instructions/register.rs
for register in YMM0 ZMM0 FS CR0 DR0; do
    check "register_$register" source_has "\b$register\b" "$REGS"
done
for op in Movups Movdqa Movdqu Addps Subps Mulps Divps Pxor Pand Paddd Shufps Pshufd \
    Popcnt Lzcnt Tzcnt Bswap Mfence Lfence Sfence Cmpxchg8b Cmpxchg16b Cpuid Rdtsc Rdmsr Wrmsr \
    Jecxz Jrcxz Ins Outs Prefetch Clflush Fld Fstp Fadd Fmul Fcom; do
    check "instruction_$op" source_has "\b$op\b" "$ENUMS"
done
check "avx" source_has 'Vex|Vaddps|Vmov' "$ENUMS" src/elf/instructions/struct_encode_information.rs
check "bmi" source_has 'Andn|Blsr|Bzhi' "$ENUMS"
manual "A generic model (nullary / unary / binary / ternary)"
manual "Check full System V ABI coverage, and the REX / VEX / EVEX encoding"
