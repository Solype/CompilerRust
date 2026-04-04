use super::traits::ElfWritable;

use super::ehdr;
use super::phdr;
use super::shdr;
// use super::traits::RawWritable;

// #[derive(Default, Debug)]
// pub struct ElfFileHeaders32 {
//     pub ehdr: ehdr::Elf32Ehdr,
//     pub shdrs: Vec<shdr::Elf32Shdr>,
//     pub phdrs: Vec<phdr::Elf32Phdr>,
// }

// #[derive(Default, Debug)]
// pub struct ElfFileHeaders64 {
//     pub ehdr: ehdr::Elf64Ehdr,
//     pub shdrs: Vec<shdr::Elf64Shdr>,
//     pub phdrs: Vec<phdr::Elf64Phdr>,
// }

#[derive(Debug, Default)]
pub struct ElfFileHeader<Ehdr, Shdr, Phdr>
{
    pub ehdr: Ehdr,
    pub shdrs: Vec<Shdr>,
    pub phdrs: Vec<Phdr>,
}

pub type ElfFileHeaders64 = ElfFileHeader<ehdr::Elf64Ehdr, shdr::Elf64Shdr, phdr::Elf64Phdr>;
pub type ElfFileHeaders32 = ElfFileHeader<ehdr::Elf32Ehdr, shdr::Elf32Shdr, phdr::Elf32Phdr>;


impl <Ehdr, Shdr, Phdr> ElfFileHeader<Ehdr, Shdr, Phdr>
where
    Ehdr: ElfWritable,
    Shdr: ElfWritable,
    Phdr: ElfWritable,
{
    fn add_program_header(&self) {
        
    }

    fn add_section_header(&self) {
        
    }

    pub fn write(&self, file: &mut std::fs::File) -> std::io::Result<()>
    {
        self.ehdr.write(file)?;
        Ok(())
    }
}

// impl ElfFileHeaders32 {
//     pub fn prepare_writing(&mut self) -> &Self
//     {
//         self.ehdr.e_phnum = self.phdrs.len() as u16;
//         self.ehdr.e_shnum = self.shdrs.len() as u16;

//         for (idx, shdr) in self.shdrs.iter().enumerate() {
//             let cur_type = shdr.sh_type;
//             if cur_type == shdr::ShType::StrTab {
//                 self.ehdr.e_shstrndx = idx as u16;
//                 break;
//             }
//         }
//         return self;
//     }

//     pub fn write(&self, file: &mut std::fs::File) -> std::io::Result<()>
//     {
//         self.ehdr.write(file)?;
//         Ok(())
//     }
// }
