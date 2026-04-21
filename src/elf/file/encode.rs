use std::fmt::Debug;


use super::super::instructions::{EncodeInformation, Relocation, Size};

use super::file::ElfFile;
use super::super::{
    elfsym::{SHN_UNDEF, make_st_info, ElfSym, StVis, StBind, StType},
    rel::{ElfRel, ElfRela, shift_r_info},
    traits::{ElfWritable, UsizeCompatible},
    instructions,
};

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

    fn handle_local_symbol_relocation(&mut self, new_global_ndx: usize)
    {
        for (_, rel_list) in self.rels.iter_mut() {
            for rel in rel_list {
                rel.r_info = shift_r_info(rel.r_info, new_global_ndx);
            }
        }

        for (_, rel_list) in self.relas.iter_mut() {
            for rel in rel_list {
                rel.r_info = shift_r_info(rel.r_info, new_global_ndx);
            }
        }
    }

    fn encode_single_instruction(
        &mut self,
        instr: &instructions::enums::Instruction,
        sec_ndx: usize,
    ){
        let mut encode = instr.encode(Size::from(size_of::<T>()));

        let base_offset = self.sections[sec_ndx].get_data().len();

        for info in encode.relocations.clone() {
            let r_offset = base_offset + info.offset;
            let name_idx = self.strtab.name(&info.sym);

            let sym_ndx = if let Some(idx) = self.symtab.get_index_from_name(name_idx) {
                idx
            } else {
                let undef_sym = ElfSym {
                    st_name: name_idx as u32,
                    st_info: make_st_info(StBind::Local, StType::NoType, ),
                    st_other: StVis::Default as u8,
                    st_shndx: SHN_UNDEF,
                    st_value: T::from_usize(0),
                    st_size: T::from_usize(0),
                };
                let ndx = self.symtab.add(undef_sym);
                self.handle_local_symbol_relocation(ndx);
                ndx
            };
            self.add_relocation(&mut encode, &info, sym_ndx, sec_ndx, r_offset);
        }

        self.sections[sec_ndx].add_data(&encode.data);
    }

    pub fn encode_instructions(&mut self, section_ndx: usize, data: &Vec<instructions::enums::Instruction>)
    {
        for ins in data {
            if let instructions::Instruction::LocalSym(name) = ins {
                let name_idx = self.strtab.name(name);

                let must_reloc = self.symtab.get(name_idx).is_none();

                let sym_ndx = self.symtab.add(ElfSym {
                    st_name: name_idx as u32,
                    st_info: make_st_info(StBind::Local, StType::NoType,),
                    st_other: StVis::Default as u8,
                    st_shndx: section_ndx as u16,
                    st_value: T::from_usize(self.sections[section_ndx].get_data().len()),
                    st_size: T::from_usize(0),
                });
                if must_reloc {
                    self.handle_local_symbol_relocation(sym_ndx);
                }
                continue;
            }

            self.encode_single_instruction(ins, section_ndx);
        }
    }
}

