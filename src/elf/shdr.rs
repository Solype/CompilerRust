use std::io::Write;

use crate::ElfWritable;
use elf_derive::ElfWrite;

#[repr(C)]
#[derive(Debug, Default, Clone, Copy, ElfWrite)]
pub struct Elf64Shdr {
    pub sh_name: u32,
    pub sh_type: ShType,
    pub sh_flags: ShFlags,
    pub sh_addr: u64,
    pub sh_offset: u64,
    pub sh_size: u64,
    pub sh_link: u32,
    pub sh_info: u32,
    pub sh_addralign: u64,
    pub sh_entsize: u64,
}
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, ElfWrite)]
pub struct Elf32Shdr {
    pub sh_name: u32,
    pub sh_type: ShType,
    pub sh_flags: ShFlags,
    pub sh_addr: u32,
    pub sh_offset: u32,
    pub sh_size: u32,
    pub sh_link: u32,
    pub sh_info: u32,
    pub sh_addralign: u32,
    pub sh_entsize: u32,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
#[allow(dead_code)]
pub enum ShType {
    #[default]
    Null     = 0,  // Section inactive
    ProgBits = 1,  // Données programmables (.text, .data)
    SymTab   = 2,  // Table des symboles
    StrTab   = 3,  // Table de chaînes
    Rela     = 4,  // Relocations avec addend
    Hash     = 5,  // Table de hash
    Dynamic  = 6,  // Infos dynamiques
    Note     = 7,  // Notes
    NoBits   = 8,  // Pas de données (ex: .bss)
    Rel      = 9,  // Relocations sans addend
    ShLib    = 10, // Réservé
    DynSym   = 11, // Symboles dynamiques
}


#[derive(Debug, Clone, Copy, Default)]
pub struct ShFlags(pub u64);

#[allow(dead_code)]
impl ShFlags {
    pub const WRITE: Self = Self(0x1);
    pub const ALLOC: Self = Self(0x2);
    pub const EXECINSTR: Self = Self(0x4);

    pub const fn empty() -> Self {
        Self(0)
    }

    pub fn bits(self) -> u64 {
        self.0
    }
}



impl ElfWritable for ShType {
    fn write(&self, file: &mut std::fs::File) -> std::io::Result<()> {
        file.write(&(*self as u32).to_le_bytes())?;
        Ok(())
    }
}

impl ElfWritable for ShFlags {
    fn write(&self, file: &mut std::fs::File) -> std::io::Result<()> {
        file.write_all(&self.0.to_le_bytes())?;
        Ok(())
    }
}