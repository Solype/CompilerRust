use elf_derive::BinaryLogicSize;

use super::traits::ElfWritable;

#[derive(Debug, Clone, Copy, Default, BinaryLogicSize)]
pub struct ElfSym<T> {
    pub st_name: u32,   // Index into the string table (.strtab) for the symbol's name
    pub st_info: u8,    // Symbol type and binding attributes (STB_*, STT_*)
    pub st_other: u8,   // Symbol visibility (STV_*) and other info
    pub st_shndx: u16,  // Section index this symbol refers to, or special values (SHN_UNDEF, SHN_ABS, etc.)
    pub st_value: T,    // Value of the symbol (e.g., address or offset)
    pub st_size: T,     // Size of the symbol (0 if not applicable)
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum StBind {
    Local = 0,       // Local symbol, not visible outside object file
    Global = 1,      // Global symbol, visible to all object files
    Weak = 2,        // Weak symbol, overridden by global
    // 3..=10 reserved
    Num = 10,        // Number of defined bindings
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum StType {
    NoType = 0,      // Not specified
    Object = 1,      // Data object
    Func = 2,        // Function or code
    Section = 3,     // Section
    File = 4,        // File name symbol
    Common = 5,      // Common data
    TLS = 6,         // Thread-local storage
    // 7..=12 reserved
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum StVis {
    Default = 0,     // Normal visibility
    Internal = 1,    // Processor specific
    Hidden = 2,      // Not visible to other objects
    Protected = 3,   // Visible but not preemptable
}

pub fn make_st_info(bind: StBind, typ: StType) -> u8 {
    ((bind as u8) << 4) | ((typ as u8) & 0xF)
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
