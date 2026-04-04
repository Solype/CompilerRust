use super::traits::ElfWritable;
use std::io::Write;


#[derive(Debug, Clone, Copy, Default)]
pub struct ElfSym<T> {
    pub st_name: u32,
    pub st_info: u8,
    pub st_other: u8,
    pub st_shndx: u16,
    pub st_value: T,
    pub st_size: T,
}

impl ElfWritable for ElfSym<u32> {
    fn write(&self, file: &mut std::fs::File) -> std::io::Result<()> {
        file.write_all(&self.st_name.to_le_bytes())?;
        self.st_value.write(file)?;
        self.st_size.write(file)?;
        file.write_all(&self.st_info.to_le_bytes())?;
        file.write_all(&self.st_other.to_le_bytes())?;
        file.write_all(&self.st_shndx.to_le_bytes())?;
        Ok(())
    }
}

impl ElfWritable for ElfSym<u64> {
    fn write(&self, file: &mut std::fs::File) -> std::io::Result<()> {
        file.write_all(&self.st_name.to_le_bytes())?;
        file.write_all(&self.st_info.to_le_bytes())?;
        file.write_all(&self.st_other.to_le_bytes())?;
        file.write_all(&self.st_shndx.to_le_bytes())?;
        self.st_value.write(file)?;
        self.st_size.write(file)?;
        Ok(())
    }
}
