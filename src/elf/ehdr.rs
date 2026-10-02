use super::traits::ElfWritable;
use elf_derive::BinaryLogicSize;

use std::fmt::Debug;


use super::phdr;
use super::shdr;

#[derive(BinaryLogicSize, Debug, Clone, Copy)]
pub struct ElfEhdr<T>
where T: Copy + ElfWritable + Debug + Default
{
    pub e_ident: [u8; 16],   // ELF identification (magic, class, endianness, version, ABI)

    pub e_type: u16,         // Object file type (ET_REL, ET_EXEC, ET_DYN, etc.)
    pub e_machine: u16,      // Target architecture (e.g., EM_X86_64)
    pub e_version: u32,      // ELF version (usually 1)

    pub e_entry: T,          // Entry point virtual address (0 for relocatable files)
    pub e_phoff: T,          // Offset to program header table (in bytes)
    pub e_shoff: T,          // Offset to section header table (in bytes)

    pub e_flags: u32,        // Processor-specific flags

    pub e_ehsize: u16,       // ELF header size (sizeof ElfEhdr)
    pub e_phentsize: u16,    // Size of one program header entry
    pub e_phnum: u16,        // Number of program header entries

    pub e_shentsize: u16,    // Size of one section header entry
    pub e_shnum: u16,        // Number of section header entries
    pub e_shstrndx: u16,     // Index of the section name string table (.shstrtab)
}

impl<T> ElfWritable for ElfEhdr<T>
where
    T: Copy + ElfWritable + Debug + Default,
{
    fn write<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        // e_ident (16 bytes brut)
        writer.write_all(&self.e_ident)?;

        // champs scalaires
        writer.write_all(&self.e_type.to_le_bytes())?;
        writer.write_all(&self.e_machine.to_le_bytes())?;
        writer.write_all(&self.e_version.to_le_bytes())?;

        // generic fields (32/64)
        self.e_entry.write(writer)?;
        self.e_phoff.write(writer)?;
        self.e_shoff.write(writer)?;

        writer.write_all(&self.e_flags.to_le_bytes())?;

        writer.write_all(&self.e_ehsize.to_le_bytes())?;
        writer.write_all(&self.e_phentsize.to_le_bytes())?;
        writer.write_all(&self.e_phnum.to_le_bytes())?;

        writer.write_all(&self.e_shentsize.to_le_bytes())?;
        writer.write_all(&self.e_shnum.to_le_bytes())?;
        writer.write_all(&self.e_shstrndx.to_le_bytes())?;

        Ok(())
    }
}

impl <T> Default for ElfEhdr<T>
where T: Copy + ElfWritable + Debug + Default
{
    fn default() -> Self {
        let (bit, machine): (u8, u16) = match size_of::<T>() {
            4 => (ElfClass::Bit32 as u8, 0x03),
            8 => (ElfClass::Bit64 as u8, 0x3E),
            _ => panic!("Doe not match a recognisable size u32 or u64"),
        };
        Self {
            e_ident: build_ident(bit, 1),
            e_type: ElfType::Rel as u16,
            e_machine: machine,
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
