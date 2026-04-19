use std::collections::HashMap;
use std::fmt::Debug;
use std::vec;


use super::{
    symbols::{self,},
    section::Section,
    strtab::Strtab,
};

use super::super::{
    ehdr, phdr, shdr, elfsym,
    traits::{ElfWritable, UsizeCompatible},
    rel::{ElfRel, ElfRela},
    elfsym::{ElfSym, SHN_UNDEF, make_st_info},
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
    pub symtab: symbols::Symbols<T>,
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
            symtab: symbols::Symbols::new(),
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

#[allow(dead_code)]
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

    pub fn add_symbol_to_section_raw(&mut self, section_ndx: usize, name: String, data: &Vec<u8>, info: u8, other: u8)
    {
        let name_ndx = self.strtab.name(&name);
        println!("adding symbol : {}, ndx in strtab: {}", name, name_ndx);

        self.symtab.add(elfsym::ElfSym {
            st_name: name_ndx as u32,
            st_info: info,
            st_shndx: section_ndx as u16,
            st_value: T::from_usize(self.sections[section_ndx].get_data().len()),
            st_size: T::from_usize(data.len()),
            st_other: other,
        });
        self.sections[section_ndx].add_data(&data);
    }

    pub fn declare_non_defined_sym(&mut self, name: &String, ty: SymbolType)
    {
        let name_offset = self.strtab.name(&name);
        let info = match ty {
            SymbolType::Function => make_st_info(elfsym::StBind::Global, elfsym::StType::Func),
            SymbolType::Object   => make_st_info(elfsym::StBind::Global, elfsym::StType::Object),
        };

        self.symtab.add(ElfSym {
            st_name: name_offset as u32,
            st_info: info,
            st_other: 0,
            st_shndx: SHN_UNDEF,
            st_value: T::from_usize(0),
            st_size: T::from_usize(0),
        });
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
