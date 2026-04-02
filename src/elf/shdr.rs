use std::io::Write;
use super::traits::*;

#[repr(C, packed)]
#[derive(Debug, Default)]
pub struct ElfShdr<T> {
    pub sh_name: u32,
    pub sh_type: u32,
    pub sh_flags: T,
    pub sh_addr: T,
    pub sh_offset: T,
    pub sh_size: T,
    pub sh_link: u32,
    pub sh_info: u32,
    pub sh_addralign: T,
    pub sh_entsize: T,
}

impl <T> Writable for ElfShdr<T>
where T: Copy + WriteBytes,
{
    fn write(&self, file: &mut std::fs::File) -> std::io::Result<()> {
        let sh_name = self.sh_name; sh_name.write_le(file)?;
        let sh_type = self.sh_type; sh_type.write_le(file)?;
        let sh_flags = self.sh_flags; sh_flags.write_le(file)?;
        let sh_addr = self.sh_addr; sh_addr.write_le(file)?;
        let sh_offset = self.sh_offset; sh_offset.write_le(file)?;
        let sh_size = self.sh_size; sh_size.write_le(file)?;
        let sh_link = self.sh_link; sh_link.write_le(file)?;
        let sh_info = self.sh_info; sh_info.write_le(file)?;
        let sh_addralign = self.sh_addralign; sh_addralign.write_le(file)?;
        let sh_entsize = self.sh_entsize; sh_entsize.write_le(file)?;
        Ok(())
    }
}

pub type Elf32Shdr = ElfShdr<u32>;
pub type Elf64Shdr = ElfShdr<u64>;
