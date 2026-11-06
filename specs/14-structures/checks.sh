# #14: no syntax exists yet, so nothing here can be a program. Add them once the design is written.
check "design_section" source_has '^#+ .*(Structures|구조)' docs/LANGUAGE_DESIGN.md
check "keyword" source_has '구조' src/parser/preparser/words.rs
check "ast_node" source_has 'Struct' src/parser/postparser/structs.rs
check "size_of_every_type" source_lacks '_ => unimplemented' src/parser/postparser/structs.rs
manual "Sizes, alignment and field offsets: add programs under specs/14-structures once the syntax exists"
manual "Field access in the IR and the codegen (needs Load / Store of #6)"
