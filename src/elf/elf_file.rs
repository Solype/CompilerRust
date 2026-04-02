use crate::elf::traits::Writable;

use super::ehdr;
use super::phdr;
use super::shdr;
use super::traits;

#[derive(Default)]
pub struct ElfFile<Ehdr, Shdr, Phdr> {
    pub ehdr: Ehdr,
    pub shdrs: Vec<Shdr>,
    pub phdrs: Vec<Phdr>,
}

pub type ElfFile64 = ElfFile<
    ehdr::Elf64Ehdr,
    shdr::Elf64Shdr,
    phdr::Elf64Phdr,
>;

pub type ElfFile32 = ElfFile<
    ehdr::Elf32Ehdr,
    shdr::Elf32Shdr,
    phdr::Elf32Phdr,
>;

impl<Ehdr, Shdr, Phdr> Writable for ElfFile<Ehdr, Shdr, Phdr>
where Ehdr: Writable, Shdr: Writable, Phdr: Writable
{
    fn write(&self, file: &mut std::fs::File) -> std::io::Result<()> {
        self.ehdr.write(file)?;
        Ok(())
    }
}
