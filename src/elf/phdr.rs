use elf_derive::BinaryLogicSize;
use super::traits::ElfWritable;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum _SegmentType {
    Null    = 0,    // PT_NULL
    Load    = 1,    // PT_LOAD
    Dynamic = 2,    // PT_DYNAMIC
    Interp  = 3,    // PT_INTERP
    Note    = 4,    // PT_NOTE
    Shlib   = 5,    // PT_SHLIB
    Phdr    = 6,    // PTPHDR
    Tls     = 7,    // PT_TLS
}

// Segment flags
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum _SegmentFlags {
    Execute = 0x1,  // PF_X
    Write   = 0x2,  // PF_W
    Read    = 0x4,  // PF_R
}

#[derive(Debug, Default, Clone, Copy, BinaryLogicSize)]
pub struct ElfPhdr<T> {
    pub p_type: u32,         // Segment type (PT_LOAD, PT_DYNAMIC, PT_INTERP, etc.)
    pub p_flags: u32,        // Segment flags (PF_X, PF_W, PF_R)

    pub p_offset: T,         // Offset of segment in file (in bytes)
    pub p_vaddr: T,          // Virtual address in memory
    pub p_paddr: T,          // Physical address (unused on most systems)

    pub p_filesz: T,         // Size of segment in file (in bytes)
    pub p_memsz: T,          // Size of segment in memory (can be larger than filesz)

    pub p_align: T,          // Alignment (must be power of 2, e.g., 0x1000)
}


impl<T> ElfWritable for ElfPhdr<T>
where
    T: Copy + ElfWritable,
{
    fn write<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        match std::mem::size_of::<T>() {
            4 => {
                // ELF32 layout
                writer.write_all(&self.p_type.to_le_bytes())?;
                self.p_offset.write(writer)?;
                self.p_vaddr.write(writer)?;
                self.p_paddr.write(writer)?;
                self.p_filesz.write(writer)?;
                self.p_memsz.write(writer)?;
                writer.write_all(&self.p_flags.to_le_bytes())?;
                self.p_align.write(writer)?;
            }

            8 => {
                // ELF64 layout
                writer.write_all(&self.p_type.to_le_bytes())?;
                writer.write_all(&self.p_flags.to_le_bytes())?;
                self.p_offset.write(writer)?;
                self.p_vaddr.write(writer)?;
                self.p_paddr.write(writer)?;
                self.p_filesz.write(writer)?;
                self.p_memsz.write(writer)?;
                self.p_align.write(writer)?;
            }

            _ => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "ElfPhdr<T>: T must be u32 or u64",
                ));
            }
        }

        Ok(())
    }
}
