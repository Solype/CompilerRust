use elf_derive::BinaryLogicSize;

use super::traits::ElfWritable;
use std::{fmt::Debug};

#[repr(C)]
#[derive(Debug, Default, Clone, Copy, BinaryLogicSize)]
pub struct ElfShdr<T>
where
    T: Debug + Copy,
{
    pub sh_name: u32,        // Offset into .shstrtab (section name string table)
    pub sh_type: u32,        // Section type (SHT_PROGBITS, SHT_SYMTAB, etc.)
    pub sh_flags: T,         // Section flags (SHF_WRITE, SHF_ALLOC, SHF_EXECINSTR...) → u32/u64
    pub sh_addr: T,          // Virtual address in memory (0 if not loaded)
    pub sh_offset: T,        // Offset of section in the file (in bytes)
    pub sh_size: T,          // Section size in bytes
    pub sh_link: u32,        // Index of a related section (depends on sh_type)
    pub sh_info: u32,        // Extra information (depends on sh_type)
    pub sh_addralign: T,     // Required alignment (power of 2, e.g., 4, 8, 16)
    pub sh_entsize: T,       // Entry size if section holds a table (0 otherwise)
}

impl<T> ElfShdr<T>
where
    T: Debug + Copy + Default,
{
    pub fn strtab(sh_name: u32) -> Self {
        Self {
            sh_name: sh_name,
            sh_type: ShType::StrTab as u32,
            ..Default::default()
        }
    }
}

impl<T> ElfWritable for ElfShdr<T>
where
    T: ElfWritable + Copy + Debug,
{
    fn write<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&self.sh_name.to_le_bytes())?;
        writer.write_all(&self.sh_type.to_le_bytes())?;

        self.sh_flags.write(writer)?;
        self.sh_addr.write(writer)?;
        self.sh_offset.write(writer)?;
        self.sh_size.write(writer)?;

        writer.write_all(&self.sh_link.to_le_bytes())?;
        writer.write_all(&self.sh_info.to_le_bytes())?;

        self.sh_addralign.write(writer)?;
        self.sh_entsize.write(writer)?;

        Ok(())
    }
}

/// Section types (sh_type)
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum ShType {
    Null     = 0,
    ProgBits = 1,
    SymTab   = 2,
    StrTab   = 3,
    Rela     = 4,
    Hash     = 5,
    Dynamic  = 6,
    Note     = 7,
    NoBits   = 8,
    Rel      = 9,
    ShLib    = 10,
    DynSym   = 11,
}

/// Section flags (sh_flags) – use u64 for ELF64, u32 for ELF32
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum ShFlags {
    NoFlag          = 0x0,
    Write           = 0x1,
    Alloc           = 0x2,
    ExecInstr       = 0x4,
    Merge           = 0x10,
    Strings         = 0x20,
    InfoLink        = 0x40,
    LinkOrder       = 0x80,
    OsNonConforming = 0x100,
    Group           = 0x200,
    TLS             = 0x400,
}

/// Known section names
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum SectionName {
    Null,
    Text,
    Data,
    Bss,
    Rodata,
    Symtab,
    Strtab,
    ShStrtab,
    RelaText,
    Comment,
    Dynamic,
    DynSym,
    DynStr,
    Got,
    Plt,
}

impl SectionName {
    pub fn as_str(&self) -> &'static str {
        match self {
            SectionName::Null      => "",
            SectionName::Text      => ".text",
            SectionName::Data      => ".data",
            SectionName::Bss       => ".bss",
            SectionName::Rodata    => ".rodata",
            SectionName::Symtab    => ".symtab",
            SectionName::Strtab    => ".strtab",
            SectionName::ShStrtab  => ".shstrtab",
            SectionName::RelaText  => ".rela.text",
            SectionName::Comment   => ".comment",
            SectionName::Dynamic   => ".dynamic",
            SectionName::DynSym    => ".dynsym",
            SectionName::DynStr    => ".dynstr",
            SectionName::Got       => ".got",
            SectionName::Plt       => ".plt",
        }
    }
}
