use std::io::Write;
use super::traits::*;

#[repr(C, packed)]
#[derive(Debug, Default)]
pub struct Elf32Chdr {
    pub ch_type: u32,
    pub ch_size: u32,
    pub ch_addralign: u32,
}

#[repr(C, packed)]
#[derive(Debug, Default)]
pub struct Elf64Chdr {
    pub ch_type: u32,
    pub ch_reserved: u32,
    pub ch_size: u64,
    pub ch_addralign: u64,
}

impl Writable for Elf32Chdr {
    fn write(&self, file: &mut std::fs::File) -> std::io::Result<()> {
        file.write_all(&self.ch_type.to_le_bytes())?;
        file.write_all(&self.ch_size.to_le_bytes())?;
        file.write_all(&self.ch_addralign.to_le_bytes())?;
        Ok(())
    }
}

impl Writable for Elf64Chdr {
    fn write(&self, file: &mut std::fs::File) -> std::io::Result<()> {
        file.write_all(&self.ch_type.to_le_bytes())?;
        file.write_all(&self.ch_reserved.to_le_bytes())?;
        file.write_all(&self.ch_size.to_le_bytes())?;
        file.write_all(&self.ch_addralign.to_le_bytes())?;
        Ok(())
    }
}