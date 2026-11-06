no_conflict() {
    compile "$ROOT/sample/01_return.kr"
    if grep -q 'conflict' <<< "$err"; then
        echo "$(grep -c '^conflict' <<< "$err") conflicts: $(tail -n 1 <<< "$err")"
        return 1
    fi
}
check "rules_have_no_conflict" no_conflict
check "test_of_zero_conflicts" source_has 'assert.*conflicts' src/parser/cfg
check "syntax_error_lists_expected_tokens" rejects cases/syntax_error.kr '\bexpected'
manual "Conflicts: print the state, its items, the terminal and both actions (no conflict left to see it on)"
manual "Optional: LALR(1), merge the states with the same core if the table gets too big"
