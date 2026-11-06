# #6: the IR names come from the issue; indexing and reading through an address have no syntax yet.
IR=src/converter/ir/structs.rs
check "ir_load" source_has 'Load *\{' "$IR"
check "ir_store" source_has 'Store *\{' "$IR"
check "ir_addr_of" source_has 'AddrOf *\{' "$IR"
check "codegen_load" source_has 'Instr::Load' src/codegen
check "codegen_store" source_has 'Instr::Store' src/codegen
check "codegen_addr_of" source_has 'Instr::AddrOf' src/codegen
manual "Choose a syntax to read and write through an address, and to index an array (docs/LANGUAGE_DESIGN.md), then add its specs here"
manual "Add a sample that runs"
