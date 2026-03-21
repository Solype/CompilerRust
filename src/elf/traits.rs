use std::fs::File;
use std::io::Write;
use super::structs;

pub trait Writable {
    fn write(&self, file: &mut File) -> std::io::Result<()>;
}

impl Writable for structs::Elf64Ehdr {
    fn write(&self, file: &mut File) -> std::io::Result<()> {
        file.write_all(&self.e_ident)?;
        // Écrire e_type (u16, 2 octets, little-endian)
        file.write_all(&self.e_type.to_le_bytes())?;
        // Écrire e_machine (u16)
        file.write_all(&self.e_machine.to_le_bytes())?;
        // Écrire e_version (u32)
        file.write_all(&self.e_version.to_le_bytes())?;
        // Écrire e_entry (u64)
        file.write_all(&self.e_entry.to_le_bytes())?;
        // Écrire e_phoff (u64)
        file.write_all(&self.e_phoff.to_le_bytes())?;
        // Écrire e_shoff (u64)
        file.write_all(&self.e_shoff.to_le_bytes())?;
        // Écrire e_flags (u32)
        file.write_all(&self.e_flags.to_le_bytes())?;
        // Écrire e_ehsize (u16)
        file.write_all(&self.e_ehsize.to_le_bytes())?;
        // Écrire e_phentsize (u16)
        file.write_all(&self.e_phentsize.to_le_bytes())?;
        // Écrire e_phnum (u16)
        file.write_all(&self.e_phnum.to_le_bytes())?;
        // Écrire e_shentsize (u16)
        file.write_all(&self.e_shentsize.to_le_bytes())?;
        // Écrire e_shnum (u16)
        file.write_all(&self.e_shnum.to_le_bytes())?;
        // Écrire e_shstrndx (u16)
        file.write_all(&self.e_shstrndx.to_le_bytes())?;
        return Ok(())
    }
}
