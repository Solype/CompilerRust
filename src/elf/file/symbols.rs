
use std::fmt::Debug;

use super::super::{
    elfsym,
    traits::{ElfWritable, UsizeCompatible},
    elfsym::{ElfSym, SHN_UNDEF, make_st_info,},
    instructions::{enums::*},
};


use super::{
    ElfFile,
    SymbolType,
    packing::align_up,
};

use super::super::shdr::ShFlags;

/// Default alignment of an object: its size rounded up to a power of two,
/// capped at 16 (f64 -> 8, 16-byte SSE mask -> 16). Over-aligning is
/// harmless; pass 1 for strings to avoid the padding.
pub fn natural_alignment(size: usize) -> usize {
    size.max(1).next_power_of_two().min(16)
}

impl <T> ElfFile <T>
where T: Copy + ElfWritable + Debug + Default + UsizeCompatible,
{
    /// Adds `data` as a symbol placed at an offset multiple of `align` (a power
    /// of two), padding the section before it. The section alignment is raised
    /// to `align` so the final address is aligned too.
    pub fn add_symbol_to_section_raw(&mut self, section_ndx: usize, name: String, data: &Vec<u8>, info: u8, other: u8, align: usize)
    {
        self.add_symbol_to_section_relocated(section_ndx, name, data, info, other, align, &[]);
    }

    /// Same as `add_symbol_to_section_raw`, with relocations inside the data:
    /// each `Relocation` patches `size` bytes at `offset` (from the start of
    /// `data`) with the address of `sym`, e.g. a pointer or a jump table entry.
    /// `sym` is resolved when writing, like a relocation from an instruction.
    pub fn add_symbol_to_section_relocated(
        &mut self,
        section_ndx: usize,
        name: String,
        data: &Vec<u8>,
        info: u8,
        other: u8,
        align: usize,
        relocations: &[Relocation],
    ) {
        self.align_section(section_ndx, align);

        let name_ndx = self.strtab.name(&name);
        println!("adding symbol : {}, ndx in strtab: {}", name, name_ndx);

        self.symtab.add(elfsym::ElfSym {
            st_name: name_ndx as u32,
            st_info: info,
            st_shndx: section_ndx as u16,
            st_value: T::from_usize(self.sections[section_ndx].get_data().len()),
            st_size: T::from_usize(data.len()),
            st_other: other,
        });
        self.add_bytes_with_relocations(section_ndx, data.clone(), relocations);
    }

    fn align_section(&mut self, section_ndx: usize, align: usize)
    {
        if !align.is_power_of_two() {
            panic!("alignment {} is not a power of two", align);
        }

        let shdr = &mut self.shdrs[section_ndx];
        if shdr.sh_addralign.to_usize() < align {
            shdr.sh_addralign = T::from_usize(align);
        }

        // int3 between objects of an executable section, zeros elsewhere
        let exec = shdr.sh_flags.to_usize() & ShFlags::ExecInstr as usize != 0;
        let fill = if exec { 0xCC } else { 0x00 };

        let len = self.sections[section_ndx].get_data().len();
        let padding = align_up(len, align) - len;
        self.sections[section_ndx].add_data(&vec![fill; padding]);
    }

    pub fn declare_non_defined_sym(&mut self, name: &String, ty: SymbolType)
    {
        let name_offset = self.strtab.name(&name);
        let info = match ty {
            SymbolType::Function => make_st_info(elfsym::StBind::Global, elfsym::StType::Func),
            SymbolType::Object   => make_st_info(elfsym::StBind::Global, elfsym::StType::Object),
        };

        self.symtab.add(ElfSym {
            st_name: name_offset as u32,
            st_info: info,
            st_other: 0,
            st_shndx: SHN_UNDEF,
            st_value: T::from_usize(0),
            st_size: T::from_usize(0),
        });
    }

    pub fn add_symbol_to_section(&mut self, section_ndx: usize, name: String, data: &Vec<Instruction>, info: u8, other: u8)
    {
        if self.sections.len() <= section_ndx {
            panic!("Section n{} needed but it only has {} secitons", section_ndx, self.sections.len());
        }

        let name_ndx = self.strtab.name(&name);
        let size_before = self.sections[section_ndx].get_data().len();

        self.symtab.add(elfsym::ElfSym {
            st_name: name_ndx as u32,
            st_info: info,
            st_shndx: section_ndx as u16,
            st_value: T::from_usize(size_before),
            st_size: T::from_usize(0),
            st_other: other,
        });

        self.encode_instructions(section_ndx, data);

        let size_after = self.sections[section_ndx].get_data().len();
        let sym_size = size_after - size_before;
        if let Some(sym) = self.symtab.get_mut_by_name(name_ndx) {
            sym.st_size = T::from_usize(sym_size)
        }
    }
}
