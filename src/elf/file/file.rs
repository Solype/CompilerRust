use std::fmt::Debug;
use std::vec;

use crate::elf::file::section::Section;

use super::super::ehdr;
use super::super::phdr;
use super::super::shdr;
use super::super::traits::ElfWritable;
use super::super::traits::UsizeCompatible;
use super::super::symbol;
use super::strtab::Strtab;

#[allow(dead_code)]
pub struct ElfFile <T>
where T: Copy + ElfWritable + Debug + Default,
{
    pub ehdr: ehdr::ElfEhdr<T>,
    pub shdrs: Vec<shdr::ElfShdr<T>>,
    pub phdrs: Vec<phdr::ElfPhdr<T>>,
    pub sections: Vec<Section>,

    pub strtab: Strtab,
    pub shstrtab: Strtab,
    pub symtab: Vec<symbol::ElfSym<T>>,
}

#[allow(dead_code)]
pub type ElfFile32 = ElfFile<u32>;
pub type ElfFile64 = ElfFile<u64>;

impl<T> Default for ElfFile<T>
where
    T: Copy + ElfWritable + Debug + Default + From<u8> + From<u32>,
{
    fn default() -> Self {
        // Contenu de .shstrta

        Self {
            ehdr: ehdr::ElfEhdr::default(),
            shdrs: vec![ shdr::ElfShdr::<T>::default() ],
            phdrs: vec![],
            sections: vec![ Section::default() ],
            symtab: vec![],
            strtab: Strtab::default(),
            shstrtab: Strtab::default(),
        }
    }
}

#[allow(dead_code)]
impl <T> ElfFile <T>
where T: Copy + ElfWritable + Debug + Default + UsizeCompatible,
{
    pub fn add_section( &mut self, name: shdr::SectionName, binary: Vec<u8>, mut header: shdr::ElfShdr<T>) -> &mut Self
    {
        header.sh_name = self.shstrtab.name(name.as_str().to_string()) as u32;
        self.shdrs.push(header);
        self.sections.push(Section::new(binary));
        self
    }

    pub fn write(&mut self, file : &mut std::fs::File) -> std::io::Result<()>
    {
        self.prepare_writing()?;

        self.ehdr.write(file)?;
        for prog_header in &self.phdrs {
            prog_header.write(file)?;
        }

        for sh in self.sections.iter() {
            sh.write(file)?;
        }

        for sh in self.shdrs.iter() {
            sh.write(file)?;
        }
        Ok(())
    }
}
