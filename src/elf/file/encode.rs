use std::fmt::Debug;

use crate::elf::instructions::{EncodeInformation, Relocation};

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
    fn get_abs_reloc_type(&self, reloc_size: usize) -> u32 {
        match (size_of::<T>(), reloc_size) {
            // ===== x86 =====
            (4, 4) => 1,   // R_386_32

            // ===== x86_64 =====
            (8, 4) => 10,  // R_X86_64_32
            (8, 8) => 1,   // R_X86_64_64

            _ => panic!("Unsupported absolute relocation size"),
        }
    }

    fn get_rel_reloc_type(&self, reloc_size: usize) -> u32 {
        match (size_of::<T>(), reloc_size) {
            // ===== x86 =====
            (4, 4) => 2,   // R_386_PC32

            // ===== x86_64 =====
            (8, 4) => 2,   // R_X86_64_PC32
            (8, 8) => 24,  // R_X86_64_PC64

            _ => panic!("Unsupported relative relocation size"),
        }
    }

    fn add_relocation(
        &mut self,
        encode : &mut EncodeInformation,
        info : &Relocation,
        sym_ndx : usize,
        sec_ndx: usize,
        r_offset : usize
    ) {
        let r_type = match info.kind {
            instructions::enums::RelocKind::Absolute => self.get_abs_reloc_type(info.size as usize),
            instructions::enums::RelocKind::Relative => self.get_rel_reloc_type(info.size as usize),
        };

        let r_info = ElfRel::pack_info(sym_ndx as u32, r_type);
        match size_of::<T>() {
            4 => {
                let bytes = (info.addend as i32).to_le_bytes();

                encode.data[info.offset..info.offset + 4].copy_from_slice(&bytes);
                self.rels.entry(sec_ndx).or_insert_with(Vec::new).push(ElfRel {
                    r_offset: T::from_usize(r_offset),
                    r_info,
                });
            },
            8 => {
                self.relas.entry(sec_ndx).or_insert_with(Vec::new).push(ElfRela {
                    r_offset: T::from_usize(r_offset),
                    r_info,
                    r_addend: T::from_usize(info.addend as usize),
                });
            },
            _ => panic!("Invalid template")
        }
    }

    fn encode_single_instruction(
        &mut self,
        instr: &instructions::enums::Instruction,
        sec_ndx: usize,
    ) -> Result<(), EncodeError> {
        let mut encode = instr.encode();

        let base_offset = self.sections[sec_ndx].get_data().len();

        if encode.relocations.len() != 0 {
            println!("identified {} relocation !", encode.relocations.len());
        }

        for info in &encode.relocations.clone() {
            let r_offset = base_offset + info.offset;
            let name_idx = self.strtab.name(&info.sym);

            let sym_ndx = if let Some(idx) = self.symtab.get_ndx(name_idx) {
                idx
            } else {
                println!("Symbol not found !");
                return Result::Err(EncodeError::SymbolError);
            };
            self.add_relocation(&mut encode, info, *sym_ndx, sec_ndx, r_offset);
        }
        self.sections[sec_ndx].add_data(&encode.data);
        Ok(())
    }

    pub fn add_symbol_to_section(&mut self, section_ndx: usize, name: String, data: &Vec<instructions::enums::Instruction>, info: u8, other: u8)
    -> Result<(), EncodeError>
    {
        if self.sections.len() <= section_ndx {
            println!("Section n{} needed but it only has {} secitons", section_ndx, self.sections.len());
            return Result::Err(EncodeError::WrongSection);
        }

        let name_ndx = self.strtab.name(&name);
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

