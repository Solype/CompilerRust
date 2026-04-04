use elf_derive::BinaryLogicSize;
use super::traits::ElfWritable;
use std::io::Write;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum _SegmentType {
    Null    = 0,    // PT_NULL
    Load    = 1,    // PT_LOAD
    Dynamic = 2,    // PT_DYNAMIC
    Interp  = 3,    // PT_INTERP
    Note    = 4,    // PT_NOTE
    Shlib   = 5,    // PT_SHLIB
    Phdr    = 6,    // PTPHDR
    Tls     = 7,    // PT_TLS
}

// Énumération pour les flags de segment
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum _SegmentFlags {
    Execute = 0x1,  // PF_X
    Write   = 0x2,  // PF_W
    Read    = 0x4,  // PF_R
}

#[derive(Debug, Default, Clone, Copy, BinaryLogicSize)]
pub struct ElfPhdr<T> {
    pub p_type: u32,
    pub p_flags: u32,
    pub p_offset: T,
    pub p_vaddr: T,
    pub p_paddr: T,
    pub p_filesz: T,
    pub p_memsz: T,
    pub p_align: T,
}


impl ElfWritable for ElfPhdr<u32>
{
    fn write(&self, file: &mut std::fs::File) -> std::io::Result<()> {
        // ELF32
        file.write_all(&self.p_type.to_le_bytes())?;
        self.p_offset.write(file)?;
        self.p_vaddr.write(file)?;
        self.p_paddr.write(file)?;
        self.p_filesz.write(file)?;
        self.p_memsz.write(file)?;
        file.write_all(&self.p_flags.to_le_bytes())?;
        self.p_align.write(file)?;
        Ok(())
    }
}

impl ElfWritable for ElfPhdr<u64>
{
    fn write(&self, file: &mut std::fs::File) -> std::io::Result<()> {
        // ELF64
        file.write_all(&self.p_type.to_le_bytes())?;
        file.write_all(&self.p_flags.to_le_bytes())?;
        self.p_offset.write(file)?;
        self.p_vaddr.write(file)?;
        self.p_paddr.write(file)?;
        self.p_filesz.write(file)?;
        self.p_memsz.write(file)?;
        self.p_align.write(file)?;
        Ok(())
    }
}
