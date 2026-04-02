use std::mem::size_of;
use super::sys_info_getter::*;
use super::phdr;
use super::shdr;

/// ELF File Header (generic over 32-bit / 64-bit)
#[derive(Clone, Copy)]
#[allow(dead_code)]
pub struct Elf64Ehdr {
    pub e_ident: [u8; 16],

    pub e_type: u16,
    pub e_machine: u16,
    pub e_version: u32,

    pub e_entry: u64,
    pub e_phoff: u64,
    pub e_shoff: u64,

    pub e_flags: u32,

    pub e_ehsize: u16,
    pub e_phentsize: u16,
    pub e_phnum: u16,

    pub e_shentsize: u16,
    pub e_shnum: u16,
    pub e_shstrndx: u16,
}

impl Default for Elf64Ehdr {
    fn default() -> Self {
        let sys = get_system_info();

        Self {
            e_ident: build_ident(ElfClass::Bit64 as u8, sys.endian),
            e_type: ElfType::Dyn as u16,
            e_machine: get_machine(&sys.arch),
            e_version: 1,

            e_entry: 0,
            e_phoff: 0,
            e_shoff: 0,

            e_flags: 0,

            e_ehsize: size_of::<Self>() as u16,
            e_phentsize: size_of::<phdr::Elf64Phdr>() as u16,
            e_phnum: 0,

            e_shentsize: size_of::<shdr::Elf64Shdr>() as u16,
            e_shnum: 0,
            e_shstrndx: 0,
        }
    }
}

#[derive(Clone, Copy)]
#[allow(dead_code)]
pub struct Elf32Ehdr {
    pub e_ident: [u8; 16],

    pub e_type: u16,
    pub e_machine: u16,
    pub e_version: u32,

    pub e_entry: u32,
    pub e_phoff: u32,
    pub e_shoff: u32,

    pub e_flags: u32,

    pub e_ehsize: u16,
    pub e_phentsize: u16,
    pub e_phnum: u16,

    pub e_shentsize: u16,
    pub e_shnum: u16,
    pub e_shstrndx: u16,
}

impl Default for Elf32Ehdr {
    fn default() -> Self {
        let sys = get_system_info();

        Self {
            e_ident: build_ident(ElfClass::Bit32 as u8, sys.endian),
            e_type: ElfType::Exec as u16,
            e_machine: get_machine(&sys.arch),
            e_version: 1,

            e_entry: 0,
            e_phoff: 0,
            e_shoff: 0,

            e_flags: 0,

            e_ehsize: size_of::<Self>() as u16,
            e_phentsize: size_of::<phdr::Elf32Phdr>() as u16,
            e_phnum: 0,

            e_shentsize: size_of::<shdr::Elf64Shdr>() as u16,
            e_shnum: 0,
            e_shstrndx: 0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
#[repr(u8)]
pub enum ElfClass {
    None = 0,
    Bit32 = 1,
    Bit64 = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
#[repr(u16)]
pub enum ElfType {
    None = 0,        // ET_NONE
    Rel = 1,         // ET_REL
    Exec = 2,        // ET_EXEC
    Dyn = 3,         // ET_DYN
    Core = 4,        // ET_CORE
    // OS-specific
    LoOs = 0xfe00,
    HiOs = 0xfeff,
    // Processor-specific
    LoProc = 0xff00,
    HiProc = 0xffff,
}

fn build_ident(class: u8, endian: u8) -> [u8; 16] {
    let mut e_ident = [0u8; 16];
    e_ident[0] = 0x7f;
    e_ident[1] = b'E';
    e_ident[2] = b'L';
    e_ident[3] = b'F';
    e_ident[4] = class;
    e_ident[5] = endian;
    e_ident[6] = 1;
    e_ident[7] = 0;
    e_ident
}
