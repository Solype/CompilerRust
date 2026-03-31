use std::io::Write;
use super::traits::*;

#[repr(C, packed)]
#[derive(Debug, Default)]
pub struct Elf32Shdr {
    pub sh_name: u32,
    pub sh_type: u32,
    pub sh_flags: u32,
    pub sh_addr: u32,
    pub sh_offset: u32,
    pub sh_size: u32,
    pub sh_link: u32,
    pub sh_info: u32,
    pub sh_addralign: u32,
    pub sh_entsize: u32,
}

#[repr(C, packed)]
#[derive(Debug, Default)]
pub struct Elf64Shdr {
    pub sh_name: u32,
    pub sh_type: u32,
    pub sh_flags: u64,
    pub sh_addr: u64,
    pub sh_offset: u64,
    pub sh_size: u64,
    pub sh_link: u32,
    pub sh_info: u32,
    pub sh_addralign: u64,
    pub sh_entsize: u64,
}

impl Writable for Elf32Shdr {
    fn write(&self, file: &mut std::fs::File) -> std::io::Result<()> {
        file.write_all(&self.sh_name.to_le_bytes())?;
        file.write_all(&self.sh_type.to_le_bytes())?;
        file.write_all(&self.sh_flags.to_le_bytes())?;
        file.write_all(&self.sh_addr.to_le_bytes())?;
        file.write_all(&self.sh_offset.to_le_bytes())?;
        file.write_all(&self.sh_size.to_le_bytes())?;
        file.write_all(&self.sh_link.to_le_bytes())?;
        file.write_all(&self.sh_info.to_le_bytes())?;
        file.write_all(&self.sh_addralign.to_le_bytes())?;
        file.write_all(&self.sh_entsize.to_le_bytes())?;
        Ok(())
    }
}

impl Writable for Elf64Shdr {
    fn write(&self, file: &mut std::fs::File) -> std::io::Result<()> {
        file.write_all(&self.sh_name.to_le_bytes())?;
        file.write_all(&self.sh_type.to_le_bytes())?;
        file.write_all(&self.sh_flags.to_le_bytes())?;
        file.write_all(&self.sh_addr.to_le_bytes())?;
        file.write_all(&self.sh_offset.to_le_bytes())?;
        file.write_all(&self.sh_size.to_le_bytes())?;
        file.write_all(&self.sh_link.to_le_bytes())?;
        file.write_all(&self.sh_info.to_le_bytes())?;
        file.write_all(&self.sh_addralign.to_le_bytes())?;
        file.write_all(&self.sh_entsize.to_le_bytes())?;
        Ok(())
    }
}
