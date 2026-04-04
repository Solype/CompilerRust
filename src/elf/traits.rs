use std::fs::File;
use std::io::{Write, Result};

pub trait ElfWritable {
    fn write(&self, file: &mut File) -> std::io::Result<()>;
}

impl ElfWritable for u8 {
    fn write(&self, f: &mut File) -> Result<()> {
        f.write_all(&[*self])
    }
}

impl ElfWritable for u16 {
    fn write(&self, f: &mut File) -> Result<()> {
        f.write_all(&self.to_le_bytes())
    }
}

impl ElfWritable for u32 {
    fn write(&self, f: &mut File) -> Result<()> {
        f.write_all(&self.to_le_bytes())
    }
}

impl ElfWritable for u64 {
    fn write(&self, f: &mut File) -> Result<()> {
        f.write_all(&self.to_le_bytes())
    }
}

impl<T: ElfWritable, const N: usize> ElfWritable for [T; N] {
    fn write(&self, file: &mut File) -> Result<()> {
        for item in self {
            item.write(file)?;
        }
        Ok(())
    }
}
