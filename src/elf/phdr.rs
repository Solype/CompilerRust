// Énumération pour les types de segments
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

// Énumération pour les flags de segment
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum _SegmentFlags {
    Execute = 0x1,  // PF_X
    Write   = 0x2,  // PF_W
    Read    = 0x4,  // PF_R
}

// Structure pour l'en-tête de programme ELF 32 bits
#[repr(C, packed)]
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Elf32Phdr {
    pub p_type:   u32,  // Type de segment
    pub p_offset: u32,  // Offset dans le fichier
    pub p_vaddr:  u32,  // Adresse virtuelle
    pub p_paddr:  u32,  // Adresse physique
    pub p_filesz: u32,  // Taille dans le fichier
    pub p_memsz:  u32,  // Taille en mémoire
    pub p_flags:  u32,  // Flags de segment
    pub p_align:  u32,  // Alignement
}

// Structure pour l'en-tête de programme ELF 64 bits
#[repr(C, packed)]
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Elf64Phdr {
    pub p_type:   u32,  // Type de segment
    pub p_flags:  u32,  // Flags de segment
    pub p_offset: u64,  // Offset dans le fichier
    pub p_vaddr:  u64,  // Adresse virtuelle
    pub p_paddr:  u64,  // Adresse physique
    pub p_filesz: u64,  // Taille dans le fichier
    pub p_memsz:  u64,  // Taille en mémoire
    pub p_align:  u64,  // Alignement
}
