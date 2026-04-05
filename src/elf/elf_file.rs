use std::fmt::Debug;
use std::io::Write;
use std::vec;

use crate::elf::ehdr;
use crate::elf::phdr;
use crate::elf::shdr;
use super::strtab::Strtab;
use super::traits::ElfWritable;
use super::traits::UsizeCompatible;
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

fn align_up(offset: usize, align: usize) -> usize {
    if align == 0 { return offset; } // safe fallback
    (offset + (align - 1)) & !(align - 1)
}

#[allow(dead_code)]
impl <T> ElfFile <T>
where T: Copy + ElfWritable + Debug + Default + UsizeCompatible,
{
    pub fn add_section( &mut self, name: shdr::SectionName, binary: Vec<u8>, mut header: shdr::ElfShdr<T>)
    -> &mut Self
    {
        header.sh_name = self.shstrtab.name(name.as_str().to_string()) as u32;
        self.shdrs.push(header);
        self.sections.push(binary);
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

        let strtab_idx = self.shdrs.len();
        let strtab_name = self.shstrtab.name(shdr::SectionName::Strtab.as_str().to_string());
        self.sections.push(self.strtab.to_vec());
        self.shdrs.push(shdr::ElfShdr::strtab(strtab_name as u32));

        if !self.symtab.is_empty() {
            let mut sym_binary: Vec<u8> = vec![];

            for sym in self.symtab.iter() {
                sym_binary.extend(sym.to_bytes().expect("ElfSym to_bytes failed"));
            }

            self.add_section(
                shdr::SectionName::Symtab,
                sym_binary,
                shdr::ElfShdr { 
                    sh_type: shdr::ShType::SymTab as u32,
                    sh_link: strtab_idx as u32,
                    // sh_addralign: T::from_usize(symbol::ElfSym::<T>::mem_len()),
                    sh_entsize: T::from_usize(symbol::ElfSym::<T>::mem_len()),
                    ..Default::default()
                }
            );
        }


        self.ehdr.e_shstrndx = self.shdrs.len() as u16;
        let shstrtab_name = self.shstrtab.name(shdr::SectionName::ShStrtab.as_str().to_string());
        self.sections.push(self.shstrtab.to_vec());
        self.shdrs.push(shdr::ElfShdr::strtab(shstrtab_name as u32));

        if self.phdrs.len() != 0 {
            self.ehdr.e_phoff = T::from_usize(ehdr::ElfEhdr::<T>::mem_len());
        }
        self.ehdr.e_shnum = self.shdrs.len() as u16;
        self.update_section_headers();
        Ok(())
    }

    fn update_section_headers(&mut self)
    {
        let mut section_offset = ehdr::ElfEhdr::<T>::mem_len() 
            + phdr::ElfPhdr::<T>::mem_len() * self.phdrs.len();

        for (sh, section) in self.shdrs.iter_mut().zip(self.sections.iter()) {
            sh.sh_size = T::from_usize(section.len());
            let new_off = align_up(section_offset, sh.sh_addralign.to_usize());
            sh.sh_offset = T::from_usize(new_off);
            section_offset = new_off + section.len();
        }
        self.ehdr.e_shoff = T::from_usize(section_offset);
    }


    pub fn write(&mut self, file : &mut std::fs::File) -> std::io::Result<()>
    {
        self.prepare_writing()?;

        let post_ehdr_phdr_offset = ehdr::ElfEhdr::<T>::mem_len() + phdr::ElfPhdr::<T>::mem_len() * self.phdrs.len();

        self.ehdr.write(file)?;
        for prog_header in &self.phdrs {
            prog_header.write(file)?;
        }

        let mut section_offset = post_ehdr_phdr_offset;

        for (i, sh) in self.shdrs.iter_mut().enumerate() {
            let align = sh.sh_addralign; // convertir T en usize
            let new_off = align_up(section_offset, align.to_usize());

            if new_off > section_offset {
                file.write_all(&vec![0u8; new_off - section_offset])?;
                section_offset = new_off;
            }

            sh.sh_offset = T::from_usize(section_offset);
            sh.sh_size = T::from_usize(self.sections[i].len());

            file.write_all(&self.sections[i])?;

            section_offset += self.sections[i].len();
        }

        for sh in self.shdrs.iter() {
            sh.write(file)?;
        }
        Ok(())
    }
}
