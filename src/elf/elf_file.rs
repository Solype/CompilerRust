use super::elf_headers::ElfFileHeader;
use super::symbol;

#[derive(Default)]
pub struct ElfFile {
    // pub headers: ElfFileHeader,
    pub buffers: Vec<Vec<u8>>,
    pub buffer_offsets: Vec<u32>,
    pub strtab: Vec<u8>,
    pub symtab: Vec<symbol::Elf32Sym>,
}
