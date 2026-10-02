use std::fmt::Debug;

use super::{
    file::ElfFile,
    section::Section,
    super::{
        traits::{ElfWritable, UsizeCompatible},
        shdr,
        ehdr,
        phdr,
        elfsym,
        rel::*,
    }
};


fn align_up(offset: usize, align: usize) -> usize {
    if align == 0 { return offset; } // safe fallback
    (offset + (align - 1)) & !(align - 1)
}

impl <T> ElfFile<T>
where T: Copy + ElfWritable + Debug + Default + UsizeCompatible,
{
    fn pack_strtab(&mut self) -> usize
    {
        let strtab_ndx = self.shdrs.len();
        let strtab_name = self.shstrtab.name(&shdr::SectionName::Strtab.as_str().to_string());
        self.sections.push(Section::new(self.strtab.to_vec()));
        self.shdrs.push(shdr::ElfShdr::strtab(strtab_name as u32));

        return  strtab_ndx;
    }

    fn pack_symtab(&mut self, strtab_ndx: usize) -> usize
    {
        if self.symtab.symbols.is_empty() { return 0; }
        let mut sym_binary: Vec<u8> = vec![];

        for sym in self.symtab.symbols.iter() {
            sym_binary.extend(sym.to_bytes().expect("ElfSym to_bytes failed"));
        }

        let section_id = self.add_section(
            shdr::SectionName::Symtab.as_str().to_string(),
            shdr::ElfShdr { 
                sh_type: shdr::ShType::SymTab as u32,
                sh_link: strtab_ndx as u32,
                sh_addralign: T::from_usize(elfsym::ElfSym::<T>::mem_len()),
                sh_entsize: T::from_usize(elfsym::ElfSym::<T>::mem_len()),
                sh_info: self.symtab.first_global_index as u32,
                ..Default::default()
            }
        );
        self.sections[section_id].set_data(sym_binary);
        section_id
    }

    fn pack_rel(&mut self, symtab_ndx: usize)
    {
        if self.rels.is_empty() {
            return;
        }

        let entsize = ElfRel::<T>::mem_len();
        let rels = std::mem::take(&mut self.rels);

        for (target_section_ndx, relocs) in rels {
            let mut rel_binary: Vec<u8> = vec![];

            for rel in relocs {
                rel_binary.extend(rel.to_bytes().expect("ElfRel to_bytes failed"));
            }

            let target_name_ndx = self.shdrs[target_section_ndx].sh_name;
            
            let rel_section_name_opt = self.shstrtab.from_usize(target_name_ndx as usize);
    
            if let Some(section_name) = rel_section_name_opt {

                let new_name = format!(".rel{}", section_name);
    
                let section_id = self.add_section(
                    new_name,
                    shdr::ElfShdr {
                        sh_type: shdr::ShType::Rel as u32,
                        sh_link: symtab_ndx as u32,
                        sh_info: target_section_ndx as u32, // ⭐ clé ici
                        sh_addralign: T::from_usize(entsize),
                        sh_entsize: T::from_usize(entsize),
                        ..Default::default()
                    }
                );
    
                self.sections[section_id].set_data(rel_binary);
            }
        }
    }

    fn pack_rela(&mut self, symtab_ndx: usize)
    {
        if self.relas.is_empty() {
            return;
        }

        let entsize = ElfRela::<T>::mem_len();
        let rels = std::mem::take(&mut self.relas);

        for (target_section_ndx, relocs) in rels {
            let mut rel_binary: Vec<u8> = vec![];

            for rel in relocs {
                rel_binary.extend(
                    rel.to_bytes().expect("ElfRel to_bytes failed")
                );
            }

            let target_name_ndx = self.shdrs[target_section_ndx].sh_name;
            let rel_section_name_opt = self.shstrtab.from_usize(target_name_ndx as usize);

            if let Some(section_name) = rel_section_name_opt {

                let new_name = format!(".rela{}", section_name);
    
                let section_id = self.add_section(
                    new_name,
                    shdr::ElfShdr {
                        sh_type: shdr::ShType::Rela as u32,
                        sh_link: symtab_ndx as u32,
                        sh_info: target_section_ndx as u32, // ⭐ clé ici
                        sh_addralign: T::from_usize(entsize),
                        sh_entsize: T::from_usize(entsize),
                        ..Default::default()
                    }
                );
    
                self.sections[section_id].set_data(rel_binary);
            }
        }
    }

    fn pack_shstrtab(&mut self)
    {
        self.ehdr.e_shstrndx = self.shdrs.len() as u16;
        let shstrtab_name = self.shstrtab.name(&shdr::SectionName::ShStrtab.as_str().to_string());
        self.sections.push(Section::new(self.shstrtab.to_vec()));
        self.shdrs.push(shdr::ElfShdr::strtab(shstrtab_name as u32));
    }


    fn update_section_headers(&mut self)
    {
        let mut section_offset = ehdr::ElfEhdr::<T>::mem_len() 
            + phdr::ElfPhdr::<T>::mem_len() * self.phdrs.len();

        let iter_sh = self.shdrs.iter_mut();
        let iter_section = self.sections.iter_mut();
        for (sh, section) in iter_sh.zip(iter_section) {
            sh.sh_size = T::from_usize(section.get_data().len());
            let new_off = align_up(section_offset, sh.sh_addralign.to_usize());
            if new_off > section_offset {
                section.set_padding(new_off - section_offset);
            }
            sh.sh_offset = T::from_usize(new_off);
            section_offset = new_off + section.get_data().len();
        }
        self.ehdr.e_shoff = T::from_usize(section_offset);
    }

    pub(super) fn prepare_writing(&mut self) -> std::io::Result<()>
    {
        if self.sections.len() != self.shdrs.len() {
            return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "ElfFile<T>.write : difference between the number of section and number of shdrs",
                ));
        }

        if self.phdrs.len() != 0 {
            self.ehdr.e_phoff = T::from_usize(ehdr::ElfEhdr::<T>::mem_len());
        }

        // Avant pack_strtab / pack_symtab : peut ajouter des symboles externes
        self.resolve_relocations();

        let strtab_ndx = self.pack_strtab();
        let sym_ndx = self.pack_symtab(strtab_ndx);
        self.pack_rel(sym_ndx);
        self.pack_rela(sym_ndx);
        self.pack_shstrtab();

        self.ehdr.e_shnum = self.shdrs.len() as u16;
        self.update_section_headers();
        Ok(())
    }

}