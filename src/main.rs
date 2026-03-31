use std::fs::File;
mod elf;
use elf::traits::Writable;

fn main() -> std::io::Result<()> {
    let mut file = File::create("output.elf")?;

    // Initialiser l'en-tête ELF
    let ehdr = elf::ehdr::Elf64Ehdr::default();
    ehdr.write(&mut file)?;

    println!("Fichier ELF généré : output.elf");
    Ok(())
}
