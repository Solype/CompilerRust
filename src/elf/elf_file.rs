use std::fmt::Debug;

use crate::elf::ehdr;
use crate::elf::phdr;
use crate::elf::shdr;
use crate::elf::traits::ElfWritable;
use super::symbol;

#[derive(Default)]
#[allow(dead_code)]
pub struct ElfFile <T>
where T: Copy + ElfWritable + Debug + Default,
{
    pub ehdr: ehdr::ElfEhdr<T>,
    pub shdrs: Vec<shdr::ElfShdr<T>>,
    pub phdrs: Vec<phdr::ElfPhdr<T>>,
    pub buffers: Vec<Vec<u8>>,
    pub buffer_offsets: Vec<u32>,
    pub strtab: Vec<u8>,
    pub symtab: Vec<symbol::ElfSym<T>>,
}

pub type ElfFile32 = ElfFile<u32>;
pub type ElfFile64 = ElfFile<u64>;

impl <T> ElfFile <T>
where T: Copy + ElfWritable + Debug + Default,
{
    pub fn add_section(&mut self) -> &mut Self
    {
        self.ehdr.e_shnum += 1;
        return self;
    }

    pub fn write(&self, file : &mut std::fs::File) -> std::io::Result<()> {
        self.ehdr.write(file)?;
        Ok(())
    }
}
