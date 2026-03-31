use super::ehdr;
use super::chdr;
use super::shdr;

pub struct elf_file_64 {
    pub ehdr : ehdr::Elf64Ehdr,
    pub shdrs : Vec<shdr::Elf64Shdr>,
    pub chdrs : Vec<chdr::Elf64Chdr>
}

pub struct elf_file_32 {
    pub ehdr : ehdr::Elf32Ehdr,
    pub shdrs : Vec<shdr::Elf32Shdr>,
    pub chdrs : Vec<chdr::Elf32Chdr>
}