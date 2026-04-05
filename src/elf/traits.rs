use std::io::{Result};

pub trait ElfWritable {
    fn write<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()>;
}

impl ElfWritable for u8 {
    fn write<W: std::io::Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_all(&[*self])
    }
}

impl ElfWritable for u16 {
    fn write<W: std::io::Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_all(&self.to_le_bytes())
    }
}

impl ElfWritable for u32 {
    fn write<W: std::io::Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_all(&self.to_le_bytes())
    }
}

impl ElfWritable for u64 {
    fn write<W: std::io::Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_all(&self.to_le_bytes())
    }
}

impl<T: ElfWritable, const N: usize> ElfWritable for [T; N] {
    fn write<W: std::io::Write>(&self, writer: &mut W) -> Result<()> {
        for item in self {
            item.write(writer)?;
        }
        Ok(())
    }
}

pub trait UsizeCompatible {
    fn from_usize(v: usize) -> Self;
    fn to_usize(&self) -> usize;
}

impl UsizeCompatible for u32 {
    fn from_usize(v: usize) -> Self {
        v as u32
    }
    fn to_usize(&self) -> usize {
        *self as usize
    }
}

impl UsizeCompatible for u64 {
    fn from_usize(v: usize) -> Self {
        v as u64
    }
    fn to_usize(&self) -> usize {
        *self as usize
    }
}
