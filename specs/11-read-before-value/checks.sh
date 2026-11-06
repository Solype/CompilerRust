# Error or warning, as the issue leaves it open. An error would also refuse the programs of #33.
check "read_in_return" complains cases/read_in_return.kr "나"
check "read_in_expression" complains cases/read_in_expression.kr "나"
check "assigned_first_stays_silent" silent cases/assigned_first.kr
manual "Decide between an error and a warning (an error also refuses what specs/33 reads)"
