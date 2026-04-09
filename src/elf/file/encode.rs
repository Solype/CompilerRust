use std::fmt::Debug;

use super::file::ElfFile;
use super::super::{
    rel::{ElfRel, ElfRela},
    traits::{ElfWritable, UsizeCompatible},
    instructions,
    elfsym,
};

#[derive(Debug)]
#[allow(dead_code)]
pub enum EncodeError {
    SymbolError,
    InvalidSection,
    WrongSection,
}

impl <T> ElfFile<T>
where T: Copy + ElfWritable + Debug + Default + UsizeCompatible,
{
    fn get_abs_reloc_type(&self) -> u32 {
        match size_of::<T>() {
            4 => 1,       // R_386_32
            8 => 1,   // R_X86_64_32
            _ => 0
        }
    }

    fn get_rel_reloc_type(&self) -> u32 {
        match size_of::<T>() {
            4 => 2,       // R_386_PC32
            8 => 2,    // R_X86_64_PC32
            _ => 0 
        }
    }

    fn encode_single_instruction(
        &mut self,
        instr: &instructions::enums::Instruction,
        sec_ndx: usize,
    ) -> Result<(), EncodeError> {
        let encode = instr.encode();

        let base_offset = self.sections[sec_ndx].get_data().len();

        if encode.relocations.len() != 0 {
            println!("identified {} relocation !", encode.relocations.len());
        }

        self.sections[sec_ndx].add_data(&encode.data);

        for info in encode.relocations {
            let r_offset = base_offset + info.offset;

            let name_idx = self.strtab.name(info.sym);

            let sym_idx = if let Some(idx) = self.symtab.get_ndx(name_idx) {
                idx
            } else {
                return Result::Err(EncodeError::SymbolError);
            };

            let r_type = match info.kind {
                instructions::enums::RelocKind::Absolute => self.get_abs_reloc_type(),
                instructions::enums::RelocKind::Relative => self.get_rel_reloc_type(),
            };

            let r_info = ElfRel::pack_info(*sym_idx as u32, r_type);
            if info.addend != 0 {
                self.relas.entry(sec_ndx).or_insert_with(Vec::new).push(ElfRela {
                    r_offset: T::from_usize(r_offset),
                    r_info,
                    r_addend: T::from_usize(info.addend as usize),
                });
            } else {
                self.rels.entry(sec_ndx).or_insert_with(Vec::new).push(ElfRel {
                    r_offset: T::from_usize(r_offset),
                    r_info,
                });
            }
        }
        Ok(())
    }

    pub fn add_symbol_to_section(&mut self, section_ndx: usize, name: String, data: &Vec<instructions::enums::Instruction>, info: u8, other: u8)
    -> Result<(), EncodeError>
    {
        if self.sections.len() <= section_ndx {
            println!("Section n{} needed but it only has {} secitons", section_ndx, self.sections.len());
            return Result::Err(EncodeError::WrongSection);
        }

        let name_ndx = self.strtab.name(name);
        let size_before = self.sections[section_ndx].get_data().len();

        for ins in data {
            self.encode_single_instruction(ins, section_ndx)?;
        }
        let size_after = self.sections[section_ndx].get_data().len();

        let _ = self.symtab.add(elfsym::ElfSym {
            st_name: name_ndx as u32,
            st_info: info,
            st_shndx: section_ndx as u16,
            st_value: T::from_usize(size_before),
            st_size: T::from_usize(size_after - size_before),
            st_other: other,
        });
        Ok(())
    }
}

