//! Relocation symbols resolved when writing: a name that is never defined
//! becomes external (GLOBAL UND), a global defined after its call stays
//! among the globals, a forward-referenced label stays LOCAL.

use std::fmt::Debug;

use super::file::ElfFile;
use super::super::{
    elfsym::{make_st_info, ElfSym, StBind, StType, StVis, SHN_UNDEF},
    shdr::{ElfShdr, SectionName, ShFlags, ShType},
    traits::{ElfWritable, UsizeCompatible},
    instructions::{CtrlOp, Instruction},
};
use crate::samples::helpers::*;

fn text_section<T>(elf: &mut ElfFile<T>) -> usize
where T: Copy + ElfWritable + Debug + Default + UsizeCompatible,
{
    elf.add_section(
        SectionName::Text.as_str().to_string(),
        ElfShdr {
            sh_type: ShType::ProgBits as u32,
            sh_flags: T::from_usize(ShFlags::Alloc as usize | ShFlags::ExecInstr as usize),
            sh_addralign: T::from_usize(16),
            ..Default::default()
        },
    )
}

fn function<T>(elf: &mut ElfFile<T>, text: usize, name: &str, code: Vec<Instruction>)
where T: Copy + ElfWritable + Debug + Default + UsizeCompatible,
{
    let info = make_st_info(StBind::Global, StType::Func);
    elf.add_symbol_to_section(text, name.to_string(), &code, info, StVis::Default as u8);
}

fn symbol<'a, T>(elf: &'a mut ElfFile<T>, name: &str) -> (usize, &'a ElfSym<T>)
where T: Copy + ElfWritable + Debug + Default + UsizeCompatible,
{
    let name_idx = elf.strtab.name(&name.to_string());
    let idx = elf.symtab.get_index_from_name(name_idx).unwrap_or_else(|| panic!("{name} missing"));
    (idx, elf.symtab.get(idx).unwrap())
}

fn bind<T: Default>(sym: &ElfSym<T>) -> u8 {
    sym.st_info >> 4
}

/// Every local before sh_info (first_global_index), every global after it
fn check_local_global_split<T>(elf: &ElfFile<T>)
where T: Copy + ElfWritable + Debug + Default + UsizeCompatible,
{
    let first_global = elf.symtab.first_global_index;
    for (i, sym) in elf.symtab.symbols.iter().enumerate() {
        let local = bind(sym) == StBind::Local as u8;
        assert_eq!(local, i < first_global, "symbol {i} on the wrong side of sh_info ({first_global})");
    }
}

/// Indexes of the symbols targeted by the rela relocations of `section`
fn rela_symbols(elf: &ElfFile<u64>, section: usize) -> Vec<usize> {
    elf.relas[&section].iter().map(|r| (r.r_info >> 32) as usize).collect()
}

#[test]
fn undeclared_extern_becomes_global_undef() {
    let mut elf = ElfFile::<u64>::default();
    let text = text_section(&mut elf);
    function(&mut elf, text, "_start", vec![call("exit")]);
    elf.resolve_relocations();

    check_local_global_split(&elf);
    let (idx, sym) = symbol(&mut elf, "exit");
    assert_eq!(bind(sym), StBind::Global as u8);
    assert_eq!(sym.st_shndx, SHN_UNDEF);
    assert_eq!(rela_symbols(&elf, text), vec![idx]);
}

#[test]
fn forward_global_stays_global() {
    let mut elf = ElfFile::<u64>::default();
    let text = text_section(&mut elf);
    function(&mut elf, text, "_start", vec![call("later")]);
    function(&mut elf, text, "later", vec![ret()]);
    elf.resolve_relocations();

    check_local_global_split(&elf);
    let (idx, sym) = symbol(&mut elf, "later");
    assert_eq!(bind(sym), StBind::Global as u8);
    assert_eq!(sym.st_shndx as usize, text);
    assert_eq!(rela_symbols(&elf, text), vec![idx]);
}

#[test]
fn forward_label_stays_local() {
    let mut elf = ElfFile::<u64>::default();
    let text = text_section(&mut elf);
    function(&mut elf, text, "_start", vec![jmp("done"), ret(), label("done"), ret()]);
    elf.resolve_relocations();

    check_local_global_split(&elf);
    let (idx, sym) = symbol(&mut elf, "done");
    assert_eq!(bind(sym), StBind::Local as u8);
    assert_eq!(sym.st_shndx as usize, text);
    // jmp rel32 (5 bytes) + ret
    assert_eq!(sym.st_value, 6);
    assert_eq!(rela_symbols(&elf, text), vec![idx]);
}

#[test]
fn locals_added_after_relocations_keep_indexes_right() {
    // Each inserted local label shifts the globals: relocations must still
    // target the right symbol
    let mut elf = ElfFile::<u64>::default();
    let text = text_section(&mut elf);
    function(&mut elf, text, "_start", vec![call("f"), call("exit"), label("a"), ret()]);
    function(&mut elf, text, "f", vec![jmp("b"), label("b"), ret()]);
    elf.resolve_relocations();

    check_local_global_split(&elf);
    let expected: Vec<usize> = ["f", "exit", "b"].iter().map(|n| symbol(&mut elf, n).0).collect();
    assert_eq!(rela_symbols(&elf, text), expected);
}

#[test]
fn rel8_addend_in_32_bits() {
    // In 32 bits the addend is written into the code: 1 byte for loop rel8
    let mut elf = ElfFile::<u32>::default();
    let text = text_section(&mut elf);
    function(&mut elf, text, "_start", vec![label("top"), ctrl(CtrlOp::Loop, "top")]);
    elf.resolve_relocations();

    assert_eq!(elf.sections[text].get_data(), &[0xE2, 0xFF]);
}
