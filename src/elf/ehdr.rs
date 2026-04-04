use super::traits::ElfWritable;
use elf_derive::BinaryLogicSize;

use std::fmt::Debug;
use std::io::Write;


use super::sys_info_getter::*;
use super::phdr;
use super::shdr;

#[derive(Debug, Clone, Copy, BinaryLogicSize)]
#[allow(dead_code)]
pub struct ElfEhdr<T>
where T: Copy + ElfWritable + Debug + Default
{
    pub e_ident: [u8; 16],

    pub e_type: u16,
    pub e_machine: u16,
    pub e_version: u32,

    pub e_entry: T,
    pub e_phoff: T,
    pub e_shoff: T,

    pub e_flags: u32,

    pub e_ehsize: u16,
    pub e_phentsize: u16,
    pub e_phnum: u16,

    pub e_shentsize: u16,
    pub e_shnum: u16,
    pub e_shstrndx: u16,
}

impl<T> ElfWritable for ElfEhdr<T>
where
    T: Copy + ElfWritable + Debug + Default,
{
    fn write(&self, file: &mut std::fs::File) -> std::io::Result<()> {
        // e_ident (16 bytes brut)
        file.write_all(&self.e_ident)?;

        // champs scalaires
        file.write_all(&self.e_type.to_le_bytes())?;
        file.write_all(&self.e_machine.to_le_bytes())?;
        file.write_all(&self.e_version.to_le_bytes())?;

        // champs génériques (32/64)
        self.e_entry.write(file)?;
        self.e_phoff.write(file)?;
        self.e_shoff.write(file)?;

        file.write_all(&self.e_flags.to_le_bytes())?;

        file.write_all(&self.e_ehsize.to_le_bytes())?;
        file.write_all(&self.e_phentsize.to_le_bytes())?;
        file.write_all(&self.e_phnum.to_le_bytes())?;

        file.write_all(&self.e_shentsize.to_le_bytes())?;
        file.write_all(&self.e_shnum.to_le_bytes())?;
        file.write_all(&self.e_shstrndx.to_le_bytes())?;

        Ok(())
    }
}

impl <T> Default for ElfEhdr<T>
where T: Copy + ElfWritable + Debug + Default
{
    fn default() -> Self {
        let sys = get_system_info();

        Self {
            e_ident: build_ident(ElfClass::Bit64 as u8, sys.endian),
            e_type: ElfType::Rel as u16,
            e_machine: get_machine(&sys.arch),
            e_version: 1,

            e_entry: T::default(),
            e_phoff: T::default(),
            e_shoff: T::default(),

            e_flags: 0,

            e_ehsize: Self::mem_len() as u16,
            e_phentsize: phdr::ElfPhdr::<T>::mem_len() as u16,
            e_phnum: 0,

            e_shentsize: shdr::ElfShdr::<T>::mem_len() as u16,
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
