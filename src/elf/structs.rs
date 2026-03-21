
#[repr(C, packed)]
#[derive(Debug, Default)]
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
