use std::fmt::Debug;
use std::io::Write;
use std::vec;

use crate::elf::ehdr;
use crate::elf::phdr;
use crate::elf::shdr;
use super::strtab::Strtab;
use super::traits::ElfWritable;
use super::traits::FromUsize;
use super::symbol;

#[allow(dead_code)]
pub struct ElfFile <T>
where T: Copy + ElfWritable + Debug + Default,
{
    pub ehdr: ehdr::ElfEhdr<T>,
    pub shdrs: Vec<shdr::ElfShdr<T>>,
    pub phdrs: Vec<phdr::ElfPhdr<T>>,
    pub sections: Vec<Vec<u8>>,

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
            sections: vec![ vec![] ],
            symtab: vec![],
            strtab: Strtab::default(),
            shstrtab: Strtab::default(),
        }
    }
}

#[allow(dead_code)]
impl <T> ElfFile <T>
where T: Copy + ElfWritable + Debug + Default + FromUsize,
{
    pub fn add_section(
        &mut self,
        name: shdr::SectionName,
        binary: Vec<u8>,
        ty: Option<shdr::ShType>,
        flags: Option<u32>,
        align: Option<usize>,
        entsize: Option<usize>,
    ) -> &mut Self {
        // 1. add name in .shstrtab
        let name_offset = self.shstrtab.name(name.as_str().to_string());

        // 2. add default_value
        let ty = ty.unwrap_or(shdr::ShType::ProgBits);
        let flags = flags.unwrap_or(shdr::ShFlags::NoFlag as u32);
        let align = align.unwrap_or(1);
        let entsize = entsize.unwrap_or(0);

        // 3. Header
        let shdr = shdr::ElfShdr {
            sh_name: name_offset as u32,
            sh_type: ty as u32,
            sh_flags: T::from_usize(flags as usize),
            sh_addr: T::default(),
            sh_offset: T::default(),
            sh_size: T::from_usize(binary.len()),
            sh_link: 0,
            sh_info: 0,
            sh_addralign: T::from_usize(align),
            sh_entsize: T::from_usize(entsize),
        };

        // 4. Push
        self.shdrs.push(shdr);
        self.sections.push(binary);

        self.ehdr.e_shnum = self.shdrs.len() as u16;

        self
    }

    fn prepare_writing(&mut self) -> std::io::Result<()>
    {
        if self.sections.len() != self.shdrs.len() {
            return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "ElfFile<T>.write : difference between the number of section and number of shdrs",
                ));
        }

        if !self.symtab.is_empty() {
            let sym_binary: Vec<u8> = self.symtab
                .iter()
                .flat_map(|s| s.to_bytes().expect("ElfSym to_bytes failed"))
                .collect();

            self.add_section(
                shdr::SectionName::Symtab,
                sym_binary,
                Some(shdr::ShType::SymTab),
                None,
                Some(std::mem::size_of::<T>()),
                Some(symbol::ElfSym::<T>::mem_len()),
            );
        }


        let shstrtab_name = self.shstrtab.name(shdr::SectionName::ShStrtab.as_str().to_string());
        let strtab_name = self.shstrtab.name(shdr::SectionName::Strtab.as_str().to_string());

        self.sections.push(self.shstrtab.to_vec());
        self.sections.push(self.strtab.to_vec());
        self.ehdr.e_shstrndx = self.shdrs.len() as u16;
        self.shdrs.push(shdr::ElfShdr::shstrtab(shstrtab_name as u32));
        self.shdrs.push(shdr::ElfShdr::strtab(strtab_name as u32));

        if self.phdrs.len() != 0 {
            self.ehdr.e_phoff = T::from_usize(ehdr::ElfEhdr::<T>::mem_len());
        }
        self.ehdr.e_shnum = self.shdrs.len() as u16;
        Ok(())
    }

    pub fn write(&mut self, file : &mut std::fs::File) -> std::io::Result<()>
    {
        self.prepare_writing()?;

        let post_ehdr_phdr_offset = ehdr::ElfEhdr::<T>::mem_len() + phdr::ElfPhdr::<T>::mem_len() * self.phdrs.len();
        let sections_size_sum : usize = self.sections.iter().map(Vec::len).sum();
        let section_header_offset = post_ehdr_phdr_offset + symbol::ElfSym::<T>::mem_len() * self.symtab.len() + sections_size_sum;
        
        self.ehdr.e_shoff = T::from_usize(section_header_offset);

        self.ehdr.write(file)?;
        for prog_header in &self.phdrs {
            prog_header.write(file)?;
        }

        for section in &self.sections {
            file.write_all(section)?;
        }

        let mut section_offset = post_ehdr_phdr_offset;
        for (i, sh) in self.shdrs.iter_mut().enumerate() {
            sh.sh_offset = T::from_usize(section_offset);
            sh.sh_size = T::from_usize(self.sections[i].len());
            section_offset += self.sections[i].len();
            sh.write(file)?;
        }
        Ok(())
    }
}
