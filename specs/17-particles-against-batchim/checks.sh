# Error or warning: the issue says "check", the design only gives the wording of the message.
BATCHIM='받침|batchim'
program() { # program <label> <source>: writes it to a file and prints its path
    printf '%s\n' "$2" > "$WORK/17_$1.kr"
    echo "$WORK/17_$1.kr"
}
wrong_program() { check "wrong_$1" complains "$(program "wrong_$1" "$2")" "$BATCHIM"; }
right_program() { check "right_$1" silent "$(program "right_$1" "$2")"; }
in_main() { printf '정수를 반환하는 main() {\n    %s\n}\n' "$1"; }
wrong() { wrong_program "$1" "$(in_main "$2")"; }
right() { right_program "$1" "$(in_main "$2")"; }
ADD=$'정수를 반환하는 더하다(정수 가, 정수 나) {\n    (가 + 나)를 반환해요.\n}\n'
PRINTF='외부 정수를 반환하는 printf(문자 주소 형식, …)'

# Hangul name: last syllable of the name
wrong name_vowel_eul '나는 1이에요. 나을 반환해요.'
wrong name_batchim_reul '끝은 1이에요. 끝를 반환해요.'
wrong name_batchim_neun '끝는 1이에요. 끝을 반환해요.'
wrong name_vowel_imyeon '가는 참이에요. 만약 가이면 { 1을 반환해요. } 0을 반환해요.'
right name_vowel '나는 1이에요. 나를 반환해요.'
right name_batchim '끝은 1이에요. 끝을 반환해요.'

# Type, keyword, boolean: last syllable of the word
wrong_program type_eul '정수을 반환하는 main() { 0을 반환해요. }'
wrong type_ieyo '나는 정수이에요. 0을 반환해요.'
wrong bool_yeyo '가는 참예요. 0을 반환해요.'
wrong bool_myeon '만약 참면 { 1을 반환해요. } 0을 반환해요.'
wrong_program value_yeyo "$ADD"'정수를 반환하는 main() { 합은 3과 4를 더한 값예요. 합을 반환해요. }'
right_program value_ieyo "$ADD"'정수를 반환하는 main() { 합은 3과 4를 더한 값이에요. 합을 반환해요. }'
right bool_ieyo '가는 참이에요. 0을 반환해요.'

# Number: its Sino-Korean reading
wrong number_1_reul '1를 반환해요.'
wrong number_2_eul '2을 반환해요.'
wrong number_3_reul '3를 반환해요.'
wrong number_4_eul '4을 반환해요.'
wrong number_5_eul '5을 반환해요.'
wrong number_6_reul '6를 반환해요.'
wrong number_7_reul '7를 반환해요.'
wrong number_8_reul '8를 반환해요.'
wrong number_9_eul '9을 반환해요.'
wrong number_0_yeyo '가는 0예요. 가를 반환해요.'
wrong number_10_reul '10를 반환해요.'
wrong number_100_reul '100를 반환해요.'
wrong number_20_reul '20를 반환해요.'
wrong number_25_eul '25을 반환해요.'
wrong number_negative_reul '-3를 반환해요.'
right number_3 '3을 반환해요.'
right number_4 '4를 반환해요.'
right number_0 '가는 0이에요. 가를 반환해요.'
right number_10 '10을 반환해요.'
right number_25 '25를 반환해요.'
right number_negative '-3을 반환해요.'

# `)`: the last token before it, recursively
wrong paren_name '가는 1이에요. 나는 2예요. (가 + 나)을 반환해요.'
wrong paren_number '가는 1이에요. (가 + 3)를 반환해요.'
wrong paren_nested '가는 1이에요. ((가 + 3))를 반환해요.'
right paren_name '가는 1이에요. 나는 2예요. (가 + 나)를 반환해요.'
right paren_number '가는 1이에요. (가 + 3)을 반환해요.'

# Latin name and string: both forms are accepted
right latin_reul 'abc는 1이에요. abc를 반환해요.'
right latin_eul 'abc은 1이에요. abc을 반환해요.'
right_program string_gwa "$PRINTF"$'\n''정수를 반환하는 main() { "%ld\n"과 3을 printf해요. 0을 반환해요. }'
right_program string_wa "$PRINTF"$'\n''정수를 반환하는 main() { "%ld\n"와 3을 printf해요. 0을 반환해요. }'

# (으)로
wrong euro_after_vowel '가는 1이에요. 나는 소수예요. 가를 소수으로 바꿔서 나에 넣어요. 0을 반환해요.'
right ro_after_vowel '가는 1이에요. 나는 소수예요. 가를 소수로 바꿔서 나에 넣어요. 0을 반환해요.'
manual "로 after a ㄹ 받침 (결말로): names take no (으)로 and no type ends in ㄹ, so only a unit test of the rule can show it"
