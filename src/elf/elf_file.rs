use std::fmt::Debug;
use std::io::Write;
use std::vec;

use crate::elf::ehdr;
use crate::elf::phdr;
use crate::elf::shdr;
use crate::elf::traits::ElfWritable;
use crate::elf::traits::FromUsize;
use super::symbol;

#[allow(dead_code)]
pub struct ElfFile <T>
where T: Copy + ElfWritable + Debug + Default,
{
    pub ehdr: ehdr::ElfEhdr<T>,
    pub shdrs: Vec<shdr::ElfShdr<T>>,
    pub phdrs: Vec<phdr::ElfPhdr<T>>,
    pub sections: Vec<Vec<u8>>,
    pub symtab: Vec<symbol::ElfSym<T>>,
    pub strtab_idx : u32,
}

#[allow(dead_code)]
pub type ElfFile32 = ElfFile<u32>;
pub type ElfFile64 = ElfFile<u64>;

impl<T> Default for ElfFile<T>
where
    T: Copy + ElfWritable + Debug + Default + From<u8> + From<u32>,
{
    fn default() -> Self {
        // Contenu de .shstrtab
        let mut shstrtab_data: Vec<u8> = vec![0];
        shstrtab_data.extend_from_slice(shdr::SectionName::ShStrtab.as_str().as_bytes());
        shstrtab_data.push(0);


        // Header de .shstrtab
        let shstrtab_hdr = shdr::ElfShdr::<T> {
            sh_name: 1,     // offset de ".shstrtab"
            sh_type: shdr::ShType::StrTab as u32,
            sh_addralign: T::from(1_u8),       // alignement classique pour string table
            sh_flags: T::from(shdr::ShFlags::NoFlag.bits()),
            ..Default::default()
        };

        let name_strstb_hdr = shstrtab_data.len();
        shstrtab_data.extend_from_slice(shdr::SectionName::Strtab.as_str().as_bytes());
        shstrtab_data.push(0);
        let strtab_hdr = shdr::ElfShdr::<T> {
            sh_name : name_strstb_hdr as u32,
            sh_type: shdr::ShType::StrTab as u32,
            sh_flags: T::from(shdr::ShFlags::NoFlag.bits()),
            ..Default::default()
        };

        Self {
            ehdr: ehdr::ElfEhdr {
                e_shnum: 2,        // NULL + .shstrtab
                e_shstrndx: 1,     // index de .shstrtab
                ..Default::default()
            },
            shdrs: vec![ shdr::ElfShdr::<T>::default(), shstrtab_hdr, strtab_hdr ],
            strtab_idx: 2,
            phdrs: vec![],
            sections: vec![ vec![], shstrtab_data, vec![0] ],
            symtab: vec![],
        }
    }
}

#[allow(dead_code)]
impl <T> ElfFile <T>
where T: Copy + ElfWritable + Debug + Default + FromUsize,
{
    pub fn add_section(&mut self, name: shdr::SectionName, binary: Vec<u8>) -> &mut Self {
        // 1. Ajouter le nom dans .shstrtab
        let shstrtab = &mut self.sections[self.ehdr.e_shstrndx as usize];

        let name_offset = shstrtab.len(); // offset AVANT push
        shstrtab.extend_from_slice(name.as_str().as_bytes());
        shstrtab.push(0); // null-terminated

        // 2. Créer le header
        let shdr = shdr::ElfShdr {
            sh_name: name_offset as u32, // ⚠️ offset dans .shstrtab
            sh_type: 1, // SHT_PROGBITS (par défaut)
            sh_flags: T::default(),
            sh_addr: T::default(),
            sh_offset: T::default(), // sera rempli dans write()
            sh_size: T::from_usize(binary.len()),
            sh_link: 0,
            sh_info: 0,
            sh_addralign: T::from_usize(1),
            sh_entsize: T::default(),
        };

        // 3. Push
        self.shdrs.push(shdr);
        self.sections.push(binary);

        // 4. Update header
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

        if self.symtab.len() > 0 {
            self.add_section(shdr::SectionName::ShStrtab,  self.symtab.iter().flat_map(
                |s| { 
                    s.to_bytes().unwrap()
                }).collect());
        }

        if self.phdrs.len() != 0 {
            self.ehdr.e_phoff = T::from_usize(ehdr::ElfEhdr::<T>::mem_len());
        }
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
