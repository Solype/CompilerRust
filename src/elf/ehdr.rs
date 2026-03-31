use std::io::Write;
use std::fs::File;
use crate::elf::ehdr;

use super::traits::*;
use super::sys_info_getter::*;

#[repr(C, packed)]
#[derive(Debug)]
pub struct Elf64Ehdr {
    pub e_ident: [u8; 16],      // Magic number et autres infos
    pub e_type: u16,            // Type de fichier (ET_EXEC, ET_REL, etc.)
    pub e_machine: u16,         // Architecture (EM_X86_64, EM_ARM, etc.)
    pub e_version: u32,         // Version d'ELF
    pub e_entry: u64,           // Point d'entrée (adresse du code à exécuter)
    pub e_phoff: u64,           // Offset du tableau des en-têtes de programme
    pub e_shoff: u64,           // Offset du tableau des en-têtes de section
    pub e_flags: u32,           // Flags spécifiques au processeur
    pub e_ehsize: u16,          // Taille de l'en-tête ELF
    pub e_phentsize: u16,       // Taille d'une entrée dans le tableau des en-têtes de programme
    pub e_phnum: u16,           // Nombre d'entrées dans le tableau des en-têtes de programme
    pub e_shentsize: u16,       // Taille d'une entrée dans le tableau des en-têtes de section
    pub e_shnum: u16,           // Nombre d'entrées dans le tableau des en-têtes de section
    pub e_shstrndx: u16,        // Index de la section contenant les noms des sections
}

#[repr(C, packed)]
#[derive(Debug)]
pub struct Elf32Ehdr {
    pub e_ident: [u8; 16],
    pub e_type: u16,
    pub e_machine: u16,
    pub e_version: u32,
    pub e_entry: u32,
    pub e_phoff: u32,
    pub e_shoff: u32,
    pub e_flags: u32,
    pub e_ehsize: u16,
    pub e_phentsize: u16,
    pub e_phnum: u16,
    pub e_shentsize: u16,
    pub e_shnum: u16,
    pub e_shstrndx: u16,
}

impl Default for Elf64Ehdr {
    fn default() -> Self {
        let sys = get_system_info();

        let mut e_ident = [0u8; 16];
        e_ident[0] = 0x7f;
        e_ident[1] = b'E';
        e_ident[2] = b'L';
        e_ident[3] = b'F';
        e_ident[4] = 2; // 64-bit
        e_ident[5] = sys.endian;
        e_ident[6] = 1; // version
        e_ident[7] = 0; // SYSV ABI

        Self {
            e_ident,
            e_type: 2, // ET_EXEC (executable)
            e_machine: get_machine(&sys.arch),
            e_version: 1,
            e_entry: 0,
            e_phoff: 0,
            e_shoff: 0,
            e_flags: 0,
            e_ehsize: std::mem::size_of::<Elf64Ehdr>() as u16,
            e_phentsize: 0,
            e_phnum: 0,
            e_shentsize: 0,
            e_shnum: 0,
            e_shstrndx: 0,
        }
    }
}

impl Default for Elf32Ehdr {
    fn default() -> Self {
        let sys = get_system_info();

        let mut e_ident = [0u8; 16];
        e_ident[0] = 0x7f;
        e_ident[1] = b'E';
        e_ident[2] = b'L';
        e_ident[3] = b'F';
        e_ident[4] = 1; // 🔥 32-bit (différence ici)
        e_ident[5] = sys.endian;
        e_ident[6] = 1;
        e_ident[7] = 0;

        Self {
            e_ident,
            e_type: 2,
            e_machine: get_machine(&sys.arch),
            e_version: 1,
            e_entry: 0,
            e_phoff: 0,
            e_shoff: 0,
            e_flags: 0,
            e_ehsize: std::mem::size_of::<Elf32Ehdr>() as u16,
            e_phentsize: 0,
            e_phnum: 0,
            e_shentsize: 0,
            e_shnum: 0,
            e_shstrndx: 0,
        }
    }
}

impl Writable for Elf64Ehdr {
    fn write(&self, file: &mut File) -> std::io::Result<()> {
        file.write_all(&self.e_ident)?;
        file.write_all(&self.e_type.to_le_bytes())?;
        file.write_all(&self.e_machine.to_le_bytes())?;
        file.write_all(&self.e_version.to_le_bytes())?;
        file.write_all(&self.e_entry.to_le_bytes())?;
        file.write_all(&self.e_phoff.to_le_bytes())?;
        file.write_all(&self.e_shoff.to_le_bytes())?;
        file.write_all(&self.e_flags.to_le_bytes())?;
        file.write_all(&self.e_ehsize.to_le_bytes())?;
        file.write_all(&self.e_phentsize.to_le_bytes())?;
        file.write_all(&self.e_phnum.to_le_bytes())?;
        file.write_all(&self.e_shentsize.to_le_bytes())?;
        file.write_all(&self.e_shnum.to_le_bytes())?;
        file.write_all(&self.e_shstrndx.to_le_bytes())?;
        return Ok(())
    }
}

impl Writable for Elf32Ehdr {
    fn write(&self, file: &mut File) -> std::io::Result<()> {
        file.write_all(&self.e_ident)?;
        file.write_all(&self.e_type.to_le_bytes())?;
        file.write_all(&self.e_machine.to_le_bytes())?;
        file.write_all(&self.e_version.to_le_bytes())?;
        file.write_all(&self.e_entry.to_le_bytes())?;
        file.write_all(&self.e_phoff.to_le_bytes())?;
        file.write_all(&self.e_shoff.to_le_bytes())?;
        file.write_all(&self.e_flags.to_le_bytes())?;
        file.write_all(&self.e_ehsize.to_le_bytes())?;
        file.write_all(&self.e_phentsize.to_le_bytes())?;
        file.write_all(&self.e_phnum.to_le_bytes())?;
        file.write_all(&self.e_shentsize.to_le_bytes())?;
        file.write_all(&self.e_shnum.to_le_bytes())?;
        file.write_all(&self.e_shstrndx.to_le_bytes())?;
        Ok(())
    }
}
