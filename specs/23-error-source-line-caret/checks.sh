source_line_and_caret() {
    compile cases/unknown_name.kr
    [ "$code" -eq 1 ] || { echo "compiler exit $code, expected 1 ($(first_error))"; return 1; }
    local second third
    second="$(sed -n 2p <<< "$err")"
    third="$(sed -n 3p <<< "$err")"
    [ "$second" = "    가는 1이에요. 없음을 반환해요." ] || { echo "line 2 is not the source line: '$second'"; return 1; }
    # 18 columns: 4 spaces, 가는 (4), space, 1, 이에요 (6), ., space
    [ "${third%%^*}" = "                  " ] || { echo "the ^ is not at display column 19: '$third'"; return 1; }
}
keeps_location() {
    compile cases/unknown_name.kr
    grep -qE '^cases/unknown_name\.kr:4:[0-9]+: ' <<< "$err" || { echo "first line: $(head -n 1 <<< "$err")"; return 1; }
}
check "keeps_file_line_col" keeps_location
check "source_line_and_caret" source_line_and_caret
