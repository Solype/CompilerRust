check "reserve_without_bytes" source_has 'ShType::NoBits|SectionName::Bss' src/elf/file src/codegen
manual "Check with readelf -S: .bss has a size and takes no bytes in the file, once something uses it"
