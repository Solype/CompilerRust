use elf_derive::BinaryLogicSize;

use super::traits::ElfWritable;

#[derive(Debug, Clone, Copy, Default, BinaryLogicSize)]
pub struct ElfSym<T> {
    pub st_name: u32,
    pub st_info: u8,
    pub st_other: u8,
    pub st_shndx: u16,
    pub st_value: T,
    pub st_size: T,
}

impl<T> ElfWritable for ElfSym<T>
where
    T: Copy + ElfWritable,
{
    fn write<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        match std::mem::size_of::<T>() {
            4 => {
                // ELF32 layout
                writer.write_all(&self.st_name.to_le_bytes())?;
                self.st_value.write(writer)?;
                self.st_size.write(writer)?;
                writer.write_all(&self.st_info.to_le_bytes())?;
                writer.write_all(&self.st_other.to_le_bytes())?;
                writer.write_all(&self.st_shndx.to_le_bytes())?;
            }

            8 => {
                // ELF64 layout
                writer.write_all(&self.st_name.to_le_bytes())?;
                writer.write_all(&self.st_info.to_le_bytes())?;
                writer.write_all(&self.st_other.to_le_bytes())?;
                writer.write_all(&self.st_shndx.to_le_bytes())?;
                self.st_value.write(writer)?;
                self.st_size.write(writer)?;
            }

            _ => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "ElfSym<T>: T must be u32 or u64",
                ));
            }
        }

        Ok(())
    }
}

impl<T> ElfSym<T>
where
    T: Copy + ElfWritable,
{
    pub fn to_bytes(&self) -> std::io::Result<Vec<u8>> {
        let mut buffer = Vec::new();
        self.write(&mut buffer)?;
        Ok(buffer)
    }
}
