

use super::ehdr;
use super::phdr;
use super::shdr;
use super::traits::RawWritable;


#[derive(Default)]
pub struct ElfFile32 {
    pub ehdr: ehdr::Elf32Ehdr,
    pub shdrs: Vec<shdr::Elf32Shdr>,
    pub phdrs: Vec<phdr::Elf32Phdr>,
}

impl ElfFile32 {

    pub fn prepare_writing(&mut self) -> &Self
    {
        self.ehdr.e_phnum = self.phdrs.len() as u16;
        self.ehdr.e_shnum = self.shdrs.len() as u16;

        for (idx, shdr) in self.shdrs.iter().enumerate() {
            let cur_type = shdr.sh_type;
            if cur_type == shdr::ShType::StrTab {
                self.ehdr.e_shstrndx = idx as u16;
                break;
            }
        }
        return self;
    }

    pub fn write(&self, file: &mut std::fs::File) -> std::io::Result<()>
    {
        self.ehdr.write_raw(file)?;
        Ok(())
    }
}
