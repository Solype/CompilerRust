use std::fmt::Debug;


use super::super::instructions::{enums::RelocKind, LabelId, Relocation, Size, Target};

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
            (4, 4) => 1,   // R_386_32 (no sign extension in 32 bits)

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
            (4, 4) => 2,  // R_386_PC32 (GNU as only uses the PLT for PIC)

            // ===== x86_64 =====
            (8, 4) => 4,  // R_X86_64_PLT32

            _ => panic!("Unsupported PLT relocation size"),
        }
    }

    fn reloc_type(&self, kind: &RelocKind, size: u8) -> u32 {
        let size = size as usize;
        match kind {
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
        let encode = instr.encode(Size::from(size_of::<T>()));
        self.add_bytes_with_relocations(sec_ndx, encode.data, &encode.relocations);
    }

    /// Appends `data` to the section; each relocation's `offset` is relative
    /// to the start of `data`. Used for both instructions and data objects.
    pub(super) fn add_bytes_with_relocations(
        &mut self,
        sec_ndx: usize,
        data: Vec<u8>,
        relocations: &[Relocation],
    ){
        let base_offset = self.sections[sec_ndx].get_data().len();

        // The target is resolved when writing (resolve_relocations), once
        // every label and symbol is known
        for info in relocations {
            let size = info.size as usize;
            if info.offset + size > data.len() {
                panic!(
                    "relocation of {} at offset {} ({} bytes) goes past the end of the data ({} bytes)",
                    info.target, info.offset, size, data.len()
                );
            }

            self.pending_relocs.push(PendingReloc {
                section: sec_ndx,
                offset: base_offset + info.offset,
                target: info.target.clone(),
                kind: info.kind.clone(),
                size: info.size,
                addend: info.addend,
            });
        }

        self.sections[sec_ndx].add_data(&data);
    }

    /// Defines `id` at the current end of the section
    pub fn define_label(&mut self, section_ndx: usize, id: LabelId)
    {
        let offset = self.sections[section_ndx].get_data().len();
        if self.labels.insert(id, (section_ndx, offset)).is_some() {
            panic!("label {} defined twice", id);
        }
    }

    /// Writes `value` on the `size` bytes of the field at `offset`
    fn patch(&mut self, section: usize, offset: usize, size: u8, value: i64)
    {
        let size = size as usize;
        let data = self.sections[section].get_data_mut();
        data[offset..offset + size].copy_from_slice(&value.to_le_bytes()[..size]);
    }

    /// Builds the rel / rela entries once every label and symbol is known.
    ///
    /// - A PC-relative jump or access to a label of the same section is
    ///   resolved in place, with no relocation (as GNU as does).
    /// - Any other label reference points at the section symbol, with the
    ///   label offset added to the addend.
    /// - A symbol that is never defined becomes global undefined (external),
    ///   as with GNU as.
    pub(super) fn resolve_relocations(&mut self)
    {
        let pending = std::mem::take(&mut self.pending_relocs);

        // Labels first: patch in place or rewrite as section symbol + offset
        let mut remaining = Vec::with_capacity(pending.len());
        for mut reloc in pending {
            let Target::Label(id) = reloc.target else {
                remaining.push(reloc);
                continue;
            };

            let (label_section, label_offset) = *self.labels.get(&id)
                .unwrap_or_else(|| panic!("label {} is used but never defined", id));
            let pc_relative = matches!(reloc.kind, RelocKind::Relative | RelocKind::Plt32);

            if pc_relative && label_section == reloc.section {
                let value = label_offset as i64 + reloc.addend as i64 - reloc.offset as i64;
                let bits = reloc.size as u32 * 8;
                let fits = bits == 64 || (-(1i64 << (bits - 1))..(1i64 << (bits - 1))).contains(&value);
                if !fits {
                    panic!(
                        "label {} is out of range of its {}-byte displacement ({} bytes away)",
                        id, reloc.size, value
                    );
                }
                self.patch(reloc.section, reloc.offset, reloc.size, value);
                continue;
            }

            reloc.addend += label_offset as i32;
            remaining.push(reloc);
        }

        // Appending a global at the end of the table shifts no index, but a
        // section symbol (local) does: create them all first, then read the
        // final indexes
        for reloc in &remaining {
            match &reloc.target {
                Target::Label(id) => {
                    let section = self.labels[id].0;
                    self.symtab.section_symbol(section as u16);
                }
                Target::Sym(name) => {
                    let name_idx = self.strtab.name(name);
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
            }
        }

        for reloc in remaining {
            let sym_ndx = match &reloc.target {
                Target::Label(id) => self.symtab.section_symbol(self.labels[id].0 as u16),
                Target::Sym(name) => {
                    let name_idx = self.strtab.name(name);
                    self.symtab.get_index_from_name(name_idx).unwrap()
                }
            };
            let r_info = ElfRel::pack_info(sym_ndx as u32, self.reloc_type(&reloc.kind, reloc.size));
            let r_offset = T::from_usize(reloc.offset);

            match size_of::<T>() {
                // rel: the addend is written into the data, on the field size
                4 => {
                    self.patch(reloc.section, reloc.offset, reloc.size, reloc.addend as i64);
                    self.rels.entry(reloc.section).or_default().push(ElfRel { r_offset, r_info });
                }
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
            if let instructions::Instruction::Label(id) = ins {
                self.define_label(section_ndx, *id);
                continue;
            }

            self.encode_single_instruction(ins, section_ndx);
        }
    }
}
