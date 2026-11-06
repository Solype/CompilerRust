one_label_form() {
    source_lacks 'fn generate_labels' src/converter || source_lacks 'fn label_id' src/converter
}
check "return_none_is_unreachable" source_lacks 'ReturnWithoutType\(""' src/converter
check "call_value_arguments" source_lacks 'too_many_arguments' src/converter
check "no_todo_doc_comment" source_lacks '/// TODO !!!' src/converter
check "one_pass_per_expression" source_lacks 'fn fit_operand' src/converter
check "one_label_form" one_label_form
