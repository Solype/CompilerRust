
use std::fmt::Debug;

use super::super::{
    elfsym,
    traits::{ElfWritable, UsizeCompatible},
    elfsym::{ElfSym, SHN_UNDEF, make_st_info,},
    instructions::{enums::*},
};


use super::{
    ElfFile,
    SymbolType
};

impl <T> ElfFile <T>
where T: Copy + ElfWritable + Debug + Default + UsizeCompatible,
{
    pub fn add_symbol_to_section_raw(&mut self, section_ndx: usize, name: String, data: &Vec<u8>, info: u8, other: u8)
    {
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
        self.sections[section_ndx].add_data(&data);
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

        let ndx = self.symtab.add(elfsym::ElfSym {
            st_name: name_ndx as u32,
            st_info: info,
            st_shndx: section_ndx as u16,
            st_value: T::from_usize(size_before),
            st_size: T::from_usize(0),
            st_other: other,
        });

        self.encode_instructions(section_ndx, data);

        let size_after = self.sections[section_ndx].get_data().len();
        println!("Taille du symbole {:} : {}", name, size_after);
        if let Some(sym) = self.symtab.get_mut(ndx) {
            println!("Setting size of {:}...", name);
            sym.st_size = T::from_usize(size_after - size_before)
        }
    }
}
