use elf_derive::BinaryLogicSize;

use super::traits::ElfWritable;
use std::{fmt::Debug, io::Write};

#[repr(C)]
#[derive(Debug, Default, Clone, Copy, BinaryLogicSize)]
pub struct ElfShdr<T>
where
    T: Debug + Copy,
{
    pub sh_name: u32,
    pub sh_type: u32,     // ✅ corrigé
    pub sh_flags: T,
    pub sh_addr: T,
    pub sh_offset: T,
    pub sh_size: T,
    pub sh_link: u32,
    pub sh_info: u32,
    pub sh_addralign: T,
    pub sh_entsize: T,
}

impl<T> ElfWritable for ElfShdr<T>
where
    T: ElfWritable + Copy + Debug,
{
    fn write(&self, file: &mut std::fs::File) -> std::io::Result<()> {
        file.write_all(&self.sh_name.to_le_bytes())?;
        file.write_all(&self.sh_type.to_le_bytes())?;

        self.sh_flags.write(file)?;
        self.sh_addr.write(file)?;
        self.sh_offset.write(file)?;
        self.sh_size.write(file)?;

        file.write_all(&self.sh_link.to_le_bytes())?;
        file.write_all(&self.sh_info.to_le_bytes())?;

        self.sh_addralign.write(file)?;
        self.sh_entsize.write(file)?;

        Ok(())
    }
}
