use crate::ElfWritable;
use elf_derive::ElfWrite;

#[repr(C)]
#[derive(Debug, Clone, Copy, Default, ElfWrite)]
pub struct Elf32Sym {
    pub st_name: u32,   // Symbol name (index into .strtab)
    pub st_value: u32,  // Symbol value (address or offset)
    pub st_size: u32,   // Symbol size in bytes
    pub st_info: u8,    // Symbol type and binding (packed)
    pub st_other: u8,   // Symbol visibility
    pub st_shndx: u16,  // Section index
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default, ElfWrite)]
#[allow(dead_code)]
pub struct Elf64Sym {
    pub st_name: u32,   // Symbol name (index into .strtab)
    pub st_info: u8,    // Symbol type and binding (packed)
    pub st_other: u8,   // Symbol visibility
    pub st_shndx: u16,  // Section index
    pub st_value: u64,  // Symbol value (address or offset)
    pub st_size: u64,   // Symbol size in bytes
}
