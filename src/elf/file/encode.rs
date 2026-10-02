use std::fmt::Debug;


use super::super::instructions::{enums::RelocKind, Relocation, Size};

use super::file::{ElfFile, PendingReloc};
use super::super::{
    elfsym::{SHN_UNDEF, make_st_info, ElfSym, StVis, StBind, StType},
    rel::{ElfRel, ElfRela},
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
            (8, 1) => 14,  // R_X86_64_8
            (8, 2) => 12,  // R_X86_64_16
            (8, 4) => 10,  // R_X86_64_32
            (8, 8) => 1,   // R_X86_64_64

            _ => panic!("Unsupported absolute relocation size"),
        }
    }

    fn get_abs_signed_reloc_type(&self, reloc_size: usize) -> u32 {
        match (size_of::<T>(), reloc_size) {
            // ===== x86 =====
            (4, 4) => 1,   // R_386_32 (pas d'extension en 32 bits)

            // ===== x86_64 =====
            (8, 4) => 11,  // R_X86_64_32S

            _ => panic!("Unsupported signed absolute relocation size"),
        }
    }

    fn get_rel_reloc_type(&self, reloc_size: usize) -> u32 {
        match (size_of::<T>(), reloc_size) {
            // ===== x86 =====
            (4, 1) => 15, // R_386_PC8
            (4, 4) => 2,  // R_386_PC32

            // ===== x86_64 =====
            (8, 1) => 15, // R_X86_64_PC8
            (8, 4) => 2,  // R_X86_64_PC32
            (8, 8) => 24, // R_X86_64_PC64

            _ => panic!("Unsupported relative relocation size"),
        }
    }

    fn get_plt_reloc_type(&self, reloc_size: usize) -> u32 {
        match (size_of::<T>(), reloc_size) {
            // ===== x86 =====
            (4, 4) => 2,  // R_386_PC32 (GNU as n'utilise pas la PLT hors PIC)

            // ===== x86_64 =====
            (8, 4) => 4,  // R_X86_64_PLT32

            _ => panic!("Unsupported PLT relocation size"),
        }
    }

    fn reloc_type(&self, info: &Relocation) -> u32 {
        let size = info.size as usize;
        match info.kind {
            RelocKind::Absolute => self.get_abs_reloc_type(size),
            RelocKind::AbsoluteSigned => self.get_abs_signed_reloc_type(size),
            RelocKind::Relative => self.get_rel_reloc_type(size),
            RelocKind::Plt32 => self.get_plt_reloc_type(size),
        }
    }

    fn encode_single_instruction(
        &mut self,
        instr: &instructions::enums::Instruction,
        sec_ndx: usize,
    ){
        let mut encode = instr.encode(Size::from(size_of::<T>()));

        let base_offset = self.sections[sec_ndx].get_data().len();

        // Le symbole est résolu à l'écriture (resolve_relocations), quand
        // on sait s'il est local, global ou externe
        for info in &encode.relocations {
            // En 32 bits (rel), l'addend est écrit dans le code, sur la
            // taille du champ relogé
            if size_of::<T>() == 4 {
                let size = info.size as usize;
                let bytes = info.addend.to_le_bytes();
                encode.data[info.offset..info.offset + size].copy_from_slice(&bytes[..size]);
            }

            self.pending_relocs.push(PendingReloc {
                section: sec_ndx,
                offset: base_offset + info.offset,
                sym: info.sym.clone(),
                r_type: self.reloc_type(info),
                addend: info.addend,
            });
        }

        self.sections[sec_ndx].add_data(&encode.data);
    }

    /// Construit les entrées rel / rela une fois tous les symboles connus.
    /// Un symbole jamais défini devient global non défini (externe), comme
    /// avec GNU as.
    pub(super) fn resolve_relocations(&mut self)
    {
        let pending = std::mem::take(&mut self.pending_relocs);

        // Ajouter un global en fin de table ne décale aucun index : on crée
        // d'abord tous les externes, puis on lit les index définitifs
        for reloc in &pending {
            let name_idx = self.strtab.name(&reloc.sym);
            if self.symtab.get_index_from_name(name_idx).is_none() {
                self.symtab.add(ElfSym {
                    st_name: name_idx as u32,
                    st_info: make_st_info(StBind::Global, StType::NoType),
                    st_other: StVis::Default as u8,
                    st_shndx: SHN_UNDEF,
                    st_value: T::from_usize(0),
                    st_size: T::from_usize(0),
                });
            }
        }

        for reloc in pending {
            let name_idx = self.strtab.name(&reloc.sym);
            let sym_ndx = self.symtab.get_index_from_name(name_idx).unwrap();
            let r_info = ElfRel::pack_info(sym_ndx as u32, reloc.r_type);
            let r_offset = T::from_usize(reloc.offset);

            match size_of::<T>() {
                4 => self.rels.entry(reloc.section).or_default().push(ElfRel { r_offset, r_info }),
                8 => self.relas.entry(reloc.section).or_default().push(ElfRela {
                    r_offset,
                    r_info,
                    r_addend: T::from_usize(reloc.addend as usize),
                }),
                _ => panic!("Invalid template")
            }
        }
    }

    pub fn encode_instructions(&mut self, section_ndx: usize, data: &Vec<instructions::enums::Instruction>)
    {
        for ins in data {
            if let instructions::Instruction::LocalSym(name) = ins {
                let name_idx = self.strtab.name(name);

                self.symtab.add(ElfSym {
                    st_name: name_idx as u32,
                    st_info: make_st_info(StBind::Local, StType::NoType,),
                    st_other: StVis::Default as u8,
                    st_shndx: section_ndx as u16,
                    st_value: T::from_usize(self.sections[section_ndx].get_data().len()),
                    st_size: T::from_usize(0),
                });
                continue;
            }

            self.encode_single_instruction(ins, section_ndx);
        }
    }
}

