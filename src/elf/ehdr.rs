use std::io::Write;
use std::fs::File;
use std::mem::size_of;

use super::traits::*;
use super::sys_info_getter::*;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum ElfMachine {
    X86_64 = 62, // x86-64
    Arm   = 40,  // ARM
    // Ajoute d'autres architectures ici si nécessaire
}

/// Valeurs pour `e_ident[EI_CLASS]`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ElfClass {
    None = 0,    // Classe non définie
    Bit32 = 1,  // 32 bits
    Bit64 = 2,  // 64 bits
}

impl ElfClass {
    pub fn class_of<T>() -> ElfClass {
        match size_of::<T>() {
            8 => ElfClass::Bit64,
            4 => ElfClass::Bit32,
            _ => ElfClass::None, 
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum ElfType {
    None = 0,        // ET_NONE
    Rel = 1,         // ET_REL
    Exec = 2,        // ET_EXEC
    Dyn = 3,         // ET_DYN
    Core = 4,        // ET_CORE
    // OS-specific
    LoOs = 0xfe00,
    HiOs = 0xfeff,
    // Processor-specific
    LoProc = 0xff00,
    HiProc = 0xffff,
}



// Structure générique pour les en-têtes ELF
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct ElfEhdr<T, U>
where
    T: Copy,
    U: Copy,
{
    pub e_ident: [u8; 16],
    pub e_type: u16,
    pub e_machine: u16,
    pub e_version: u32,
    pub e_entry: T,
    pub e_phoff: U,
    pub e_shoff: U,
    pub e_flags: u32,
    pub e_ehsize: u16,
    pub e_phentsize: u16,
    pub e_phnum: u16,
    pub e_shentsize: u16,
    pub e_shnum: u16,
    pub e_shstrndx: u16,
}

// Implémentation de Default pour ElfEhdr
impl<T, U> Default for ElfEhdr<T, U>
where
    T: Copy + Default,
    U: Copy + Default,
{
    fn default() -> Self {
        let sys = get_system_info();
        let class = ElfClass::class_of::<T>() as u8;
        let e_ident = build_ident(class, sys.endian);
        Self {
            e_ident,
            e_type: ElfType::Dyn as u16,
            e_machine: get_machine(&sys.arch),
            e_version: 1,
            e_entry: T::default(),
            e_phoff: U::default(),
            e_shoff: U::default(),
            e_flags: 0,
            e_ehsize: size_of::<Self>() as u16,
            e_phentsize: 0,
            e_phnum: 0,
            e_shentsize: 0,
            e_shnum: 0,
            e_shstrndx: 0,
        }
    }
}

// Implémentation de Writable pour ElfEhdr
impl<T, U> Writable for ElfEhdr<T, U>
where
    T: Copy + WriteBytes,
    U: Copy + WriteBytes,
{
    fn write(&self, file: &mut File) -> std::io::Result<()> {
        // Écriture de e_ident
        file.write_all(&self.e_ident)?;

        // Écriture des champs
        let e_type = self.e_type; e_type.write_le(file)?;
        let e_machine = self.e_machine; e_machine.write_le(file)?;
        let e_version = self.e_version; e_version.write_le(file)?;
        let e_entry = self.e_entry; e_entry.write_le(file)?;
        let e_phoff = self.e_phoff; e_phoff.write_le(file)?;
        let e_shoff = self.e_shoff; e_shoff.write_le(file)?;
        let e_flags = self.e_flags; e_flags.write_le(file)?;
        let e_ehsize = self.e_ehsize; e_ehsize.write_le(file)?;
        let e_phentsize = self.e_phentsize; e_phentsize.write_le(file)?;
        let e_phnum = self.e_phnum; e_phnum.write_le(file)?;
        let e_shentsize = self.e_shentsize; e_shentsize.write_le(file)?;
        let e_shnum = self.e_shnum; e_shnum.write_le(file)?;
        let e_shstrndx = self.e_shstrndx; e_shstrndx.write_le(file)?;
        Ok(())
    }
}

// Alias pour les types ELF 32 et 64 bits
pub type Elf64Ehdr = ElfEhdr<u64, u64>;
pub type Elf32Ehdr = ElfEhdr<u32, u32>;

// Fonction pour construire l'identifiant ELF
fn build_ident(class: u8, endian: u8) -> [u8; 16] {
    let mut e_ident = [0u8; 16];
    e_ident[0] = 0x7f;
    e_ident[1] = b'E';
    e_ident[2] = b'L';
    e_ident[3] = b'F';
    e_ident[4] = class; // 1 = 32, 2 = 64
    e_ident[5] = endian;
    e_ident[6] = 1;
    e_ident[7] = 0;
    e_ident
}
