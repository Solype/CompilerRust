check "close_jump_is_rel8" disasm_has cases/close_jumps.kr ':\s+(eb|7[0-9a-f]) [0-9a-f]{2}\s+j'
check "no_rel32_for_a_close_target" disasm_lacks cases/close_jumps.kr ':\s+(e9|0f 8[0-9a-f]) .*\sj'
