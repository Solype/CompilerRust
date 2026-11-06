converter_tests() { cat "$ROOT"/src/converter/*tests*.rs 2> /dev/null; }
every_error_kind_tested() {
    local kind missing=""
    for kind in $(sed -n '/pub enum ConvertErrorKind/,/^}/p' "$ROOT/src/converter/convert_error.rs" | grep -oE '^    [A-Z][A-Za-z]+'); do
        converter_tests | grep -q "\b$kind\b" || missing+=" $kind"
    done
    [ -z "$missing" ] || { echo "no test names:$missing"; return 1; }
}
ir_tested() { converter_tests | grep -q 'Instr::' || { echo "no test reads Instr::"; return 1; }; }
check "test_file" file_exists 'src/converter/*tests*.rs'
check "every_error_kind" every_error_kind_tested
check "ir_of_small_programs" ir_tested
