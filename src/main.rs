use std::fs::File;
mod elf;
use elf::traits::Writable;

fn main() -> std::io::Result<()> {
    let mut file = File::create("output.elf")?;

    // Initialiser l'en-tête ELF
    let mut ehdr = elf::structs::Elf64Ehdr::default();
    ehdr.e_ident = [
        0x7F, b'E', b'L', b'F', // Magic number
        elf::enums::ELFCLASS64,              // 64 bits
        1,                       // Little-endian
        1,                       // Version ELF
        0, 0, 0, 0, 0, 0, 0, 0, 0 // Padding
    ];
    ehdr.e_type = elf::enums::ET_EXEC;       // Fichier exécutable
    ehdr.e_machine = elf::enums::EM_X86_64;  // Architecture x86-64
    ehdr.e_version = 1;          // Version actuelle
    ehdr.e_entry = 0x400000;     // Point d'entrée (adresse arbitraire)
    ehdr.e_ehsize = std::mem::size_of::<elf::structs::Elf64Ehdr>() as u16;

    // Écrire l'en-tête
    ehdr.write(&mut file)?;

    println!("Fichier ELF généré : output.elf");
    Ok(())
}
