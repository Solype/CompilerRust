GOT='[Gg][Oo][Tt][Pp][Cc][Rr][Ee][Ll]'
check "reloc_kind" source_has "$GOT" src/elf/instructions/enums.rs
check "reloc_type_in_elf" source_has "$GOT" src/elf/file/encode.rs
check "encoder_test" source_has "RelocKind::$GOT" src/elf/instructions/encode_tests/relocations.rs
manual "Read a libc variable in PIE (stdout, errno): add a program once the language can declare one"
