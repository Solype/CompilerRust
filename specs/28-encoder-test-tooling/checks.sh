generator_kept() {
    [ -n "$(find "$ROOT" -name gen_encode_tests.py -not -path '*/target/*' -not -path '*/.claude/*' -print -quit)" ] || { echo "no gen_encode_tests.py in the repository"; return 1; }
}
check "generator_kept" generator_kept
check "panic_of_emit_imm_sx32_tested" source_has 'out of i32 range' src/elf/instructions/encode_tests
check "compared_with_ndisasm" source_has 'ndisasm' scripts/analyze.sh src/elf/instructions/encode_tests
check "decoding_tests" source_has '[Dd]ecod' src/elf/instructions/encode_tests
