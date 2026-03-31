// Type de fichier ELF
pub const ET_NONE: u16 = 0;     // Aucun type
pub const ET_REL: u16 = 1;      // Fichier objet (relogeable)
pub const ET_EXEC: u16 = 2;     // Fichier exécutable
pub const ET_DYN: u16 = 3;      // Fichier partagé (bibliothèque)

// Architectures
pub const EM_X86_64: u16 = 62;  // x86-64
pub const EM_ARM: u16 = 40;      // ARM

// Magic number pour e_ident
pub const _ELFMAG: [u8; 4] = [0x7F, b'E', b'L', b'F'];
pub const EI_CLASS: usize = 4;  // Index pour 32/64 bits
pub const ELFCLASS64: u8 = 2;    // 64 bits

