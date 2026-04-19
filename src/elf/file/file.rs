use std::collections::HashMap;
use std::fmt::Debug;
use std::{default, vec};

use crate::elf::rel;

use super::{
    section::Section,
    strtab::Strtab,
};

use super::super::{
    ehdr, phdr, shdr,
    traits::{ElfWritable, UsizeCompatible},
    rel::{ElfRel, ElfRela},
    elfsym::SymbolCollection,
};

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
    pub symtab: SymbolCollection<T>,
    pub rels: HashMap<usize, Vec<ElfRel<T>>>,
    pub relas: HashMap<usize, Vec<ElfRela<T>>>
}

#[allow(dead_code)]
pub type ElfFile32 = ElfFile<u32>;
#[allow(dead_code)]
pub type ElfFile64 = ElfFile<u64>;

impl<T> Default for ElfFile<T>
where
    T: Copy + ElfWritable + Debug + Default + From<u8> + From<u32>,
{
    fn default() -> Self {
        Self {
            ehdr: ehdr::ElfEhdr::default(),
            shdrs: vec![ shdr::ElfShdr::<T>::default() ],
            phdrs: vec![],
            sections: vec![ Section::default() ],
            symtab: SymbolCollection::new(),
            strtab: Strtab::default(),
            shstrtab: Strtab::default(),
            rels: HashMap::<usize, Vec<ElfRel<T>>>::default(),
            relas: HashMap::<usize, Vec<ElfRela<T>>>::default(),
        }
    }
}

pub enum SymbolType {
    Function,
    Object,
}

impl <T> ElfFile <T>
where T: Copy + ElfWritable + Debug + Default + UsizeCompatible,
{

    pub fn add_section( &mut self, name: String, mut header: shdr::ElfShdr<T>) -> usize
    {
        let section_ndx = self.shdrs.len();
        header.sh_name = self.shstrtab.name(&name) as u32;
        self.shdrs.push(header);
        self.sections.push(Section::default());
        section_ndx
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
