use elf_derive::BinaryLogicSize;

use super::traits::ElfWritable;

#[repr(C)]
#[derive(Debug, Clone, Copy, Default, BinaryLogicSize)]
pub struct ElfRel<T>
where
    T: Copy + Default,
{
    pub r_offset: T, // Address of the relocation
    pub r_info: T,   // Relocation type and symbol index
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct ElfRela<T>
where
    T: Copy + Default,
{
    pub r_offset: T,   // Address of the relocation
    pub r_info: T,     // Relocation type and symbol index
    pub r_addend: T,   // Addend for the relocation
}


impl<T> ElfWritable for ElfRel<T>
where
    T: Copy + Default + ElfWritable, // On convertit en u64 pour gérer 32/64 bits
{
    fn write<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        match std::mem::size_of::<T>() {
            4 => {
                self.r_offset.write(writer)?;
                self.r_info.write(writer)?;
            }
            8 => {
                self.r_offset.write(writer)?;
                self.r_info.write(writer)?;
            }
            _ => panic!("Unsupported size for ElfRel"),
        }
        Ok(())
    }
}

impl<T> ElfWritable for ElfRela<T>
where
    T: Copy + Default + ElfWritable,
{
    fn write<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        match std::mem::size_of::<T>() {
            4 => {
                self.r_offset.write(writer)?;
                self.r_info.write(writer)?;
                self.r_addend.write(writer)?;
            }
            8 => {
                self.r_offset.write(writer)?;
                self.r_info.write(writer)?;
                self.r_addend.write(writer)?;
            }
            _ => panic!("Unsupported size for ElfRela"),
        }
        Ok(())
    }
}
