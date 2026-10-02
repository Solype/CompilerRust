//! Relocation targets resolved when writing: a name that is never defined
//! becomes external (GLOBAL UND), a global defined after its call stays
//! among the globals. Labels never enter the symbol table: a PC-relative
//! reference in the same section is patched in place, any other one points at
//! the section symbol (reference bytes and relocations from GNU as).
//! Objects are placed at an offset multiple of their alignment, and can hold
//! relocations (pointers, jump tables) resolved like those of instructions.

use std::fmt::Debug;

use super::file::ElfFile;
use super::symbols::natural_alignment;
use super::super::{
    elfsym::{make_st_info, ElfSym, StBind, StType, StVis, SHN_UNDEF},
    shdr::{ElfShdr, SectionName, ShFlags, ShType},
    traits::{ElfWritable, UsizeCompatible},
    instructions::{register::RAX, ConditionCode, CtrlOp, Instruction, LabelId, MemAddress, Relocation, RelocKind, Target},
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

/// Names of the symbols (the null one excluded): labels must not be there
fn symbol_names<T>(elf: &ElfFile<T>) -> Vec<String>
where T: Copy + ElfWritable + Debug + Default + UsizeCompatible,
{
    elf.symtab.symbols[1..]
        .iter()
        .filter(|sym| sym.st_info & 0xF != StType::Section as u8)
        .map(|sym| elf.strtab.from_usize(sym.st_name as usize).unwrap())
        .collect()
}

#[test]
fn labels_resolved_in_place() {
    // GNU as ({disp32} forces the rel32 form):
    //   f: .L0: nop ; jne .L0 ; jmp .L1 ; call .L1 ; lea rax, [rip+.L1] ; loop .L0 ; .L1: ret
    let mut elf = ElfFile::<u64>::default();
    let text = text_section(&mut elf);
    let (l0, l1) = (LabelId::new(), LabelId::new());
    function(&mut elf, text, "f", vec![
        label(l0),
        Instruction::Nop(1),
        jcc(ConditionCode::NE, l0),
        jmp(l1),
        call(l1),
        lea(RAX, MemAddress::label(l1), QWORD),
        ctrl(CtrlOp::Loop, l0),
        label(l1),
        ret(),
    ]);
    elf.resolve_relocations();

    assert_eq!(elf.sections[text].get_data(), &[
        0x90,
        0x0F, 0x85, 0xF9, 0xFF, 0xFF, 0xFF,
        0xE9, 0x0E, 0x00, 0x00, 0x00,
        0xE8, 0x09, 0x00, 0x00, 0x00,
        0x48, 0x8D, 0x05, 0x02, 0x00, 0x00, 0x00,
        0xE2, 0xE6,
        0xC3,
    ]);
    assert!(!elf.relas.contains_key(&text), "no relocation left for ld");
    assert_eq!(symbol_names(&elf), vec!["f"], "labels are not symbols");
}

#[test]
fn each_function_jumps_to_its_own_label() {
    // the code generator can reuse the same pattern in every function
    let mut elf = ElfFile::<u64>::default();
    let text = text_section(&mut elf);
    for name in ["f", "g"] {
        let end = LabelId::new();
        function(&mut elf, text, name, vec![jmp(end), ret(), label(end), ret()]);
    }
    elf.resolve_relocations();

    // jmp +1 (over the ret), in both functions
    let f = [0xE9, 0x01, 0x00, 0x00, 0x00, 0xC3, 0xC3];
    assert_eq!(elf.sections[text].get_data(), &[f, f].concat());
    assert_eq!(symbol_names(&elf), vec!["f", "g"]);
}

#[test]
fn relocations_keep_indexes_right_after_section_symbols() {
    // Each inserted section symbol (local) shifts the globals: relocations
    // must still target the right symbol
    let mut elf = ElfFile::<u64>::default();
    let text = text_section(&mut elf);
    let rodata = data_section(&mut elf, 8);
    let case = LabelId::new();
    function(&mut elf, text, "_start", vec![call("f"), call("exit"), label(case), ret()]);
    function(&mut elf, text, "f", vec![ret()]);
    relocated(&mut elf, rodata, "table", &[0; 8], 8, &[quad(0, case)]);
    elf.resolve_relocations();

    check_local_global_split(&elf);
    let expected: Vec<usize> = ["f", "exit"].iter().map(|n| symbol(&mut elf, n).0).collect();
    assert_eq!(rela_symbols(&elf, text), expected);
    assert_eq!(rela_symbols(&elf, rodata), vec![text_section_symbol(&elf, text)]);
}

#[test]
fn loop_resolved_in_place_in_32_bits() {
    // loop to itself: rel8 = -2
    let mut elf = ElfFile::<u32>::default();
    let text = text_section(&mut elf);
    let top = LabelId::new();
    function(&mut elf, text, "_start", vec![label(top), ctrl(CtrlOp::Loop, top)]);
    elf.resolve_relocations();

    assert_eq!(elf.sections[text].get_data(), &[0xE2, 0xFE]);
    assert!(!elf.rels.contains_key(&text));
}

#[test]
fn rel8_addend_in_32_bits() {
    // In 32 bits the addend is written into the code: 1 byte for loop rel8
    let mut elf = ElfFile::<u32>::default();
    let text = text_section(&mut elf);
    function(&mut elf, text, "_start", vec![ctrl(CtrlOp::Loop, "elsewhere")]);
    elf.resolve_relocations();

    assert_eq!(elf.sections[text].get_data(), &[0xE2, 0xFF]);
}

#[test]
#[should_panic(expected = "is out of range of its 1-byte displacement (-131 bytes away)")]
fn loop_too_far_panics() {
    let mut elf = ElfFile::<u64>::default();
    let text = text_section(&mut elf);
    let top = LabelId::new();
    let mut code = vec![label(top)];
    code.extend((0..129).map(|_| Instruction::Nop(1)));
    code.push(ctrl(CtrlOp::Loop, top));
    function(&mut elf, text, "_start", code);
    elf.resolve_relocations();
}

#[test]
#[should_panic(expected = "is used but never defined")]
fn undefined_label_panics() {
    let mut elf = ElfFile::<u64>::default();
    let text = text_section(&mut elf);
    function(&mut elf, text, "_start", vec![jmp(LabelId::new())]);
    elf.resolve_relocations();
}

#[test]
#[should_panic(expected = "defined twice")]
fn label_defined_twice_panics() {
    let mut elf = ElfFile::<u64>::default();
    let text = text_section(&mut elf);
    let l = LabelId::new();
    function(&mut elf, text, "_start", vec![label(l), ret(), label(l)]);
}

/// Index of the STT_SECTION symbol of `section`
fn text_section_symbol<T>(elf: &ElfFile<T>, section: usize) -> usize
where T: Copy + ElfWritable + Debug + Default + UsizeCompatible,
{
    let sym = elf.symtab.symbols.iter().position(|sym| {
        sym.st_info == make_st_info(StBind::Local, StType::Section) && sym.st_shndx as usize == section
    });
    sym.unwrap_or_else(|| panic!("no section symbol for section {section}"))
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
fn quad(offset: usize, target: impl Into<Target>) -> Relocation {
    Relocation { target: target.into(), offset, size: 8, kind: RelocKind::Absolute, addend: 0 }
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
fn jump_table_points_to_the_text_section() {
    // switch table in .rodata: .quad .Lcase0, .Lcase1
    // GNU as: R_X86_64_64 .text + offset of each label, no label symbol
    let mut elf = ElfFile::<u64>::default();
    let text = text_section(&mut elf);
    let rodata = data_section(&mut elf, 8);
    let (case0, case1) = (LabelId::new(), LabelId::new());
    function(&mut elf, text, "f", vec![label(case0), ret(), label(case1), ret()]);
    relocated(&mut elf, rodata, "table", &[0; 16], 8, &[quad(0, case0), quad(8, case1)]);
    elf.resolve_relocations();

    check_local_global_split(&elf);
    let text_sym = text_section_symbol(&elf, text);
    assert_eq!(relas(&elf, rodata), vec![(0, text_sym, 1, 0), (8, text_sym, 1, 1)]);
    assert_eq!(symbol_names(&elf), vec!["f", "table"]);
}

#[test]
fn rip_relative_label_in_another_section() {
    // GNU as: lea rax, [rip+.LC0] with .LC0 at offset 8 of .rodata
    //   -> R_X86_64_PC32 .rodata + 4 (8 - 4)
    let mut elf = ElfFile::<u64>::default();
    let text = text_section(&mut elf);
    let rodata = data_section(&mut elf, 8);
    object(&mut elf, rodata, "before", &[0; 8], 8);
    let lc0 = LabelId::new();
    elf.define_label(rodata, lc0);
    elf.add_bytes_with_relocations(rodata, b"hi\0".to_vec(), &[]);
    function(&mut elf, text, "g", vec![lea(RAX, MemAddress::label(lc0), QWORD), ret()]);
    elf.resolve_relocations();

    let rodata_sym = text_section_symbol(&elf, rodata);
    assert_eq!(relas(&elf, text), vec![(3, rodata_sym, 2, 4)]);
}

#[test]
fn label_in_32_bits_writes_offset_as_addend() {
    // rel: the label offset goes into the data, the relocation points at .text
    let mut elf = ElfFile::<u32>::default();
    let text = text_section(&mut elf);
    let data = elf.add_section(
        SectionName::Data.as_str().to_string(),
        ElfShdr { sh_type: ShType::ProgBits as u32, sh_flags: ShFlags::Alloc as u32, sh_addralign: 4, ..Default::default() },
    );
    let case = LabelId::new();
    function(&mut elf, text, "f", vec![ret(), ret(), label(case), ret()]);
    let info = make_st_info(StBind::Global, StType::Object);
    let reloc = Relocation { target: case.into(), offset: 0, size: 4, kind: RelocKind::Absolute, addend: 0 };
    elf.add_symbol_to_section_relocated(data, "table".into(), &vec![0; 4], info, StVis::Default as u8, 4, &[reloc]);
    elf.resolve_relocations();

    assert_eq!(elf.sections[data].get_data(), &[2, 0, 0, 0]);
    let rel = &elf.rels[&data][0];
    assert_eq!(rel.r_info >> 8, text_section_symbol(&elf, text) as u32);
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
    let abs32 = Relocation { target: "x".into(), offset: 0, size: 4, kind: RelocKind::Absolute, addend: 4 };
    let pc32 = Relocation { target: "x".into(), offset: 4, size: 4, kind: RelocKind::Relative, addend: 0 };
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
    let reloc = Relocation { target: "x".into(), offset: 0, size: 4, kind: RelocKind::Absolute, addend: 0x10 };
    elf.add_symbol_to_section_relocated(data, "p".into(), &vec![0; 4], info, StVis::Default as u8, 4, &[reloc]);
    elf.resolve_relocations();

    assert_eq!(elf.sections[data].get_data(), &[0x10, 0, 0, 0]);
    let rel = &elf.rels[&data][0];
    assert_eq!(rel.r_info & 0xFF, 1, "R_386_32");
}
