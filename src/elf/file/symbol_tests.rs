//! Relocation symbols resolved when writing: a name that is never defined
//! becomes external (GLOBAL UND), a global defined after its call stays
//! among the globals, a forward-referenced label stays LOCAL.
//! Objects are placed at an offset multiple of their alignment, and can hold
//! relocations (pointers, jump tables) resolved like those of instructions.

use std::fmt::Debug;

use super::file::ElfFile;
use super::symbols::natural_alignment;
use super::super::{
    elfsym::{make_st_info, ElfSym, StBind, StType, StVis, SHN_UNDEF},
    shdr::{ElfShdr, SectionName, ShFlags, ShType},
    traits::{ElfWritable, UsizeCompatible},
    instructions::{CtrlOp, Instruction, Relocation, RelocKind},
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

fn data_section(elf: &mut ElfFile<u64>, align: u64) -> usize {
    elf.add_section(
        SectionName::Rodata.as_str().to_string(),
        ElfShdr {
            sh_type: ShType::ProgBits as u32,
            sh_flags: ShFlags::Alloc as u64,
            sh_addralign: align,
            ..Default::default()
        },
    )
}

fn object(elf: &mut ElfFile<u64>, section: usize, name: &str, bytes: &[u8], align: usize) {
    let info = make_st_info(StBind::Global, StType::Object);
    elf.add_symbol_to_section_raw(section, name.to_string(), &bytes.to_vec(), info, StVis::Default as u8, align);
}

#[test]
fn object_is_padded_to_its_alignment() {
    // f64 then a 16-byte SSE mask: the mask goes to offset 16, not 8
    let mut elf = ElfFile::<u64>::default();
    let rodata = data_section(&mut elf, 16);
    object(&mut elf, rodata, "value", &2.5f64.to_le_bytes(), 8);
    object(&mut elf, rodata, "mask", &[0xFF; 16], 16);

    assert_eq!(symbol(&mut elf, "value").1.st_value, 0);
    assert_eq!(symbol(&mut elf, "mask").1.st_value, 16);
    let data = elf.sections[rodata].get_data();
    assert_eq!(data.len(), 32);
    assert_eq!(&data[8..16], &[0; 8], "zero padding");
}

#[test]
fn aligned_offset_adds_no_padding() {
    let mut elf = ElfFile::<u64>::default();
    let rodata = data_section(&mut elf, 16);
    object(&mut elf, rodata, "a", &[1; 16], 16);
    object(&mut elf, rodata, "b", &[2; 16], 16);
    object(&mut elf, rodata, "c", b"hi\0", 1);
    object(&mut elf, rodata, "d", b"x", 1);

    assert_eq!(symbol(&mut elf, "b").1.st_value, 16);
    assert_eq!(symbol(&mut elf, "d").1.st_value, 35);
    assert_eq!(elf.sections[rodata].get_data().len(), 36);
}

#[test]
fn section_alignment_is_raised() {
    // an object aligned on 16 in a section aligned on 4: the section moves to 16
    let mut elf = ElfFile::<u64>::default();
    let rodata = data_section(&mut elf, 4);
    object(&mut elf, rodata, "mask", &[0; 16], 16);
    assert_eq!(elf.shdrs[rodata].sh_addralign, 16);

    // but never lowered
    object(&mut elf, rodata, "byte", &[0], 1);
    assert_eq!(elf.shdrs[rodata].sh_addralign, 16);
}

#[test]
fn executable_section_is_padded_with_int3() {
    let mut elf = ElfFile::<u64>::default();
    let text = text_section(&mut elf);
    object(&mut elf, text, "a", &[0x90], 1);
    object(&mut elf, text, "b", &[0x90], 8);
    assert_eq!(elf.sections[text].get_data(), &[0x90, 0xCC, 0xCC, 0xCC, 0xCC, 0xCC, 0xCC, 0xCC, 0x90]);
}

#[test]
#[should_panic(expected = "alignment 12 is not a power of two")]
fn non_power_of_two_alignment_panics() {
    let mut elf = ElfFile::<u64>::default();
    let rodata = data_section(&mut elf, 16);
    object(&mut elf, rodata, "x", &[0; 12], 12);
}

#[test]
fn natural_alignment_values() {
    assert_eq!(natural_alignment(0), 1);
    assert_eq!(natural_alignment(1), 1);  // byte, string of 1
    assert_eq!(natural_alignment(3), 4);  // "hi\0"
    assert_eq!(natural_alignment(4), 4);  // f32, i32
    assert_eq!(natural_alignment(8), 8);  // f64, pointer
    assert_eq!(natural_alignment(16), 16); // SSE mask
    assert_eq!(natural_alignment(24), 16); // capped
    assert_eq!(natural_alignment(4096), 16);
}

/// 8-byte absolute address of `sym` at `offset` (`.quad sym`)
fn quad(offset: usize, sym: &str) -> Relocation {
    Relocation { sym: sym.to_string(), offset, size: 8, kind: RelocKind::Absolute, addend: 0 }
}

fn relocated(elf: &mut ElfFile<u64>, section: usize, name: &str, bytes: &[u8], align: usize, relocs: &[Relocation]) {
    let info = make_st_info(StBind::Global, StType::Object);
    elf.add_symbol_to_section_relocated(section, name.to_string(), &bytes.to_vec(), info, StVis::Default as u8, align, relocs);
}

/// (offset, symbol index, type, addend) of the rela entries of `section`
fn relas(elf: &ElfFile<u64>, section: usize) -> Vec<(u64, usize, u32, i64)> {
    elf.relas[&section]
        .iter()
        .map(|r| (r.r_offset, (r.r_info >> 32) as usize, r.r_info as u32, r.r_addend as i64))
        .collect()
}

#[test]
fn data_pointer_to_a_string() {
    // char *msg = "hello";  ->  msg: .quad hello  (R_X86_64_64)
    let mut elf = ElfFile::<u64>::default();
    let rodata = data_section(&mut elf, 1);
    let data = data_section(&mut elf, 8);
    object(&mut elf, rodata, "hello", b"hello\0", 1);
    relocated(&mut elf, data, "msg", &[0; 8], 8, &[quad(0, "hello")]);
    elf.resolve_relocations();

    let hello = symbol(&mut elf, "hello").0;
    assert_eq!(relas(&elf, data), vec![(0, hello, 1, 0)]);
    assert_eq!(elf.sections[data].get_data(), &[0; 8], "rela: the data keeps zeros");
}

#[test]
fn function_table_resolves_later_and_external_symbols() {
    // ops: .quad add_fn, strlen  (add_fn defined afterwards, strlen never)
    let mut elf = ElfFile::<u64>::default();
    let data = data_section(&mut elf, 8);
    let text = text_section(&mut elf);
    relocated(&mut elf, data, "ops", &[0; 16], 8, &[quad(0, "add_fn"), quad(8, "strlen")]);
    function(&mut elf, text, "add_fn", vec![ret()]);
    elf.resolve_relocations();

    check_local_global_split(&elf);
    let (add_fn, _) = symbol(&mut elf, "add_fn");
    let (strlen, sym) = symbol(&mut elf, "strlen");
    assert_eq!(bind(sym), StBind::Global as u8);
    assert_eq!(sym.st_shndx, SHN_UNDEF);
    assert_eq!(relas(&elf, data), vec![(0, add_fn, 1, 0), (8, strlen, 1, 0)]);
}

#[test]
fn jump_table_points_to_local_labels() {
    // switch table in .rodata: .quad .Lcase0, .Lcase1
    let mut elf = ElfFile::<u64>::default();
    let text = text_section(&mut elf);
    let rodata = data_section(&mut elf, 8);
    function(&mut elf, text, "f", vec![label(".Lcase0"), ret(), label(".Lcase1"), ret()]);
    relocated(&mut elf, rodata, "table", &[0; 16], 8, &[quad(0, ".Lcase0"), quad(8, ".Lcase1")]);
    elf.resolve_relocations();

    check_local_global_split(&elf);
    let (case0, sym0) = symbol(&mut elf, ".Lcase0");
    assert_eq!((bind(sym0), sym0.st_value), (StBind::Local as u8, 0));
    let (case1, sym1) = symbol(&mut elf, ".Lcase1");
    assert_eq!((bind(sym1), sym1.st_value), (StBind::Local as u8, 1));
    assert_eq!(relas(&elf, rodata), vec![(0, case0, 1, 0), (8, case1, 1, 0)]);
}

#[test]
fn data_relocation_offset_includes_alignment_padding() {
    // a 1-byte object then a pointer aligned on 8: the pointer is at 8, and so is its relocation
    let mut elf = ElfFile::<u64>::default();
    let data = data_section(&mut elf, 8);
    object(&mut elf, data, "flag", &[1], 1);
    relocated(&mut elf, data, "ptr", &[0; 8], 8, &[quad(0, "flag")]);
    elf.resolve_relocations();

    assert_eq!(symbol(&mut elf, "ptr").1.st_value, 8);
    assert_eq!(relas(&elf, data)[0].0, 8);
}

#[test]
fn data_relocation_keeps_kind_size_and_addend() {
    // .long sym + 4 (R_X86_64_32) and .long sym - . (R_X86_64_PC32, offset table)
    let mut elf = ElfFile::<u64>::default();
    let data = data_section(&mut elf, 4);
    let abs32 = Relocation { sym: "x".into(), offset: 0, size: 4, kind: RelocKind::Absolute, addend: 4 };
    let pc32 = Relocation { sym: "x".into(), offset: 4, size: 4, kind: RelocKind::Relative, addend: 0 };
    relocated(&mut elf, data, "t", &[0; 8], 4, &[abs32, pc32]);
    elf.resolve_relocations();

    let x = symbol(&mut elf, "x").0;
    assert_eq!(relas(&elf, data), vec![(0, x, 10, 4), (4, x, 2, 0)]);
}

#[test]
#[should_panic(expected = "relocation of target at offset 4 (8 bytes) goes past the end of the data (8 bytes)")]
fn data_relocation_past_the_end_panics() {
    let mut elf = ElfFile::<u64>::default();
    let data = data_section(&mut elf, 8);
    relocated(&mut elf, data, "p", &[0; 8], 8, &[quad(4, "target")]);
}

#[test]
fn data_relocation_in_32_bits_writes_the_addend() {
    // rel (32 bits): no r_addend field, the addend goes into the data
    let mut elf = ElfFile::<u32>::default();
    let data = elf.add_section(
        SectionName::Data.as_str().to_string(),
        ElfShdr { sh_type: ShType::ProgBits as u32, sh_flags: ShFlags::Alloc as u32, sh_addralign: 4, ..Default::default() },
    );
    let info = make_st_info(StBind::Global, StType::Object);
    let reloc = Relocation { sym: "x".into(), offset: 0, size: 4, kind: RelocKind::Absolute, addend: 0x10 };
    elf.add_symbol_to_section_relocated(data, "p".into(), &vec![0; 4], info, StVis::Default as u8, 4, &[reloc]);
    elf.resolve_relocations();

    assert_eq!(elf.sections[data].get_data(), &[0x10, 0, 0, 0]);
    let rel = &elf.rels[&data][0];
    assert_eq!(rel.r_info & 0xFF, 1, "R_386_32");
}
