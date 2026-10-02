//! Relocations: offset (from the start of the instruction), type and addend,
//! compared with those GNU as produces (`readelf -r`).

use super::*;

fn reloc(ins: Instruction) -> Relocation {
    let info = ins.encode(Size::U64);
    assert_eq!(info.relocations.len(), 1, "{ins:?}");
    info.relocations[0].clone()
}

fn check(ins: Instruction, target: impl Into<Target>, offset: usize, size: u8, kind: RelocKind, addend: i32) {
    let r = reloc(ins);
    assert_eq!(r.target, target.into());
    assert_eq!(r.offset, offset, "offset");
    assert_eq!(r.size, size, "size");
    assert_eq!(r.kind, kind, "kind");
    assert_eq!(r.addend, addend, "addend");
}

// GNU as >= 2.31: R_X86_64_PLT32 for call / jmp / jcc rel32 (see readelf -r)

#[test]
fn jmp_rel32() {
    check(jmp("target"), "target", 1, 4, RelocKind::Plt32, -4);
}

#[test]
fn call_rel32() {
    check(call("target"), "target", 1, 4, RelocKind::Plt32, -4);
}

#[test]
fn jcc_rel32() {
    check(jcc(ConditionCode::G, "target"), "target", 2, 4, RelocKind::Plt32, -4);
}

#[test]
fn loop_rel8() {
    check(ctrl(CtrlOp::Loop, "target"), "target", 1, 1, RelocKind::Relative, -1);
}

#[test]
fn rip_load() {
    check(bin(BinOp::Mov, reg(RAX), var("my_data"), DWORD), "my_data", 2, 4, RelocKind::Relative, -4);
}

#[test]
fn rip_with_imm8_after() {
    // mov byte ptr [rip+my_data], 0x41: 1 byte after the disp32
    check(bin(BinOp::Mov, var("my_data"), imm(0x41), BYTE), "my_data", 2, 4, RelocKind::Relative, -5);
}

#[test]
fn rip_with_imm32_after() {
    check(bin(BinOp::Mov, var("my_data"), imm(0x12345678), DWORD), "my_data", 2, 4, RelocKind::Relative, -8);
}

#[test]
fn rip_with_rex_and_imm8() {
    check(bin(BinOp::Cmp, var("my_data"), imm(1), QWORD), "my_data", 3, 4, RelocKind::Relative, -5);
}

#[test]
fn rip_behind_lock_prefix() {
    let ins = prefixed(
        vec![Prefix::Lock],
        complex(ComplexBinOp::Cmpxchg, var("my_data"), reg(RCX), None, DWORD),
    );
    check(ins, "my_data", 4, 4, RelocKind::Relative, -4);
}

#[test]
fn rip_sse() {
    check(bin(BinOp::AddF, reg(XMM0), var("my_float"), None), "my_float", 4, 4, RelocKind::Relative, -4);
}

#[test]
fn absolute_imm32() {
    check(bin(BinOp::Mov, reg(R12), sym("my_data"), DWORD), "my_data", 2, 4, RelocKind::Absolute, 0);
}

#[test]
fn rip_base_plus_symbol() {
    // [rbx + my_data] : disp32 absolu, R_X86_64_32S
    let src = mem(at(RBX).sym("my_data"));
    check(bin(BinOp::Mov, reg(RAX), src, DWORD), "my_data", 2, 4, RelocKind::AbsoluteSigned, 0);
}

// With a SIB byte, the disp32 starts one byte further

#[test]
fn sib_base_index_plus_symbol() {
    // mov eax, [rbx+rcx*4+my_data] : 8B 84 8B <disp32>
    let src = mem(at(RBX).index_scale(RCX, Scale::Four).sym("my_data"));
    check(bin(BinOp::Mov, reg(RAX), src, DWORD), "my_data", 3, 4, RelocKind::AbsoluteSigned, 0);
}

#[test]
fn sib_rsp_plus_symbol() {
    // mov eax, [rsp+my_data] : 8B 84 24 <disp32>
    let src = mem(at(RSP).sym("my_data"));
    check(bin(BinOp::Mov, reg(RAX), src, DWORD), "my_data", 3, 4, RelocKind::AbsoluteSigned, 0);
}

#[test]
fn sib_r12_plus_symbol() {
    // mov eax, [r12+my_data] : 41 8B 84 24 <disp32>
    let src = mem(at(R12).sym("my_data"));
    check(bin(BinOp::Mov, reg(RAX), src, DWORD), "my_data", 4, 4, RelocKind::AbsoluteSigned, 0);
}

#[test]
fn sib_index_only_plus_symbol() {
    // mov eax, [rcx*4+my_data] : 8B 04 8D <disp32>
    let src = mem(MemAddress::new().index_scale(RCX, Scale::Four).sym("my_data"));
    check(bin(BinOp::Mov, reg(RAX), src, DWORD), "my_data", 3, 4, RelocKind::AbsoluteSigned, 0);
}

#[test]
fn absolute_symbol() {
    // mov eax, ds:[my_data]: 8B 04 25 <disp32>, R_X86_64_32S without addend
    let src = mem(MemAddress::symbol("my_data").absolute());
    check(bin(BinOp::Mov, reg(RAX), src, DWORD), "my_data", 3, 4, RelocKind::AbsoluteSigned, 0);
}

#[test]
fn call_indirect_rip() {
    // call [rip+my_data] : FF 15 <disp32>, R_X86_64_PC32 addend -4
    check(call_indirect(var("my_data")), "my_data", 2, 4, RelocKind::Relative, -4);
}


// Absolute, sign-extended (R_X86_64_32S) or not (R_X86_64_32 / 64 / 8),
// as GNU as does (see readelf -r)

#[test]
fn absolute_imm64() {
    // movabs rax, offset my_data : 48 B8 <imm64>, R_X86_64_64
    check(bin(BinOp::Mov, reg(RAX), sym("my_data"), QWORD), "my_data", 2, 8, RelocKind::Absolute, 0);
}

#[test]
fn absolute_imm8() {
    // mov al, offset my_data : B0 <imm8>, R_X86_64_8
    check(bin(BinOp::Mov, reg(RAX), sym("my_data"), BYTE), "my_data", 1, 1, RelocKind::Absolute, 0);
}

#[test]
fn alu64_symbol_is_imm32_signed() {
    // add rax, offset my_data : 48 81 C0 <imm32>, R_X86_64_32S
    let ins = bin(BinOp::Add, reg(RAX), sym("my_data"), QWORD);
    assert_eq!(ins.encode(Size::U64).data.len(), 7, "imm32, not imm64");
    check(bin(BinOp::Add, reg(RAX), sym("my_data"), QWORD), "my_data", 3, 4, RelocKind::AbsoluteSigned, 0);
}

#[test]
fn alu32_symbol() {
    // add eax, offset my_data : 81 C0 <imm32>, R_X86_64_32
    check(bin(BinOp::Add, reg(RAX), sym("my_data"), DWORD), "my_data", 2, 4, RelocKind::Absolute, 0);
}

#[test]
fn push_symbol() {
    // push offset my_data : 68 <imm32>, R_X86_64_32S
    check(stack(StackOp::Push, sym("my_data"), None), "my_data", 1, 4, RelocKind::AbsoluteSigned, 0);
}


// Labels: same fields as a symbol, but PC-relative instead of PLT32 for a
// branch (resolved in place by the ELF layer, see symbol_tests.rs)

#[test]
fn jmp_label_is_relative() {
    let l = LabelId::new();
    check(jmp(l), l, 1, 4, RelocKind::Relative, -4);
}

#[test]
fn jcc_label_is_relative() {
    let l = LabelId::new();
    check(jcc(ConditionCode::NE, l), l, 2, 4, RelocKind::Relative, -4);
}

#[test]
fn call_label_is_relative() {
    let l = LabelId::new();
    check(call(l), l, 1, 4, RelocKind::Relative, -4);
}

#[test]
fn loop_label_rel8() {
    let l = LabelId::new();
    check(ctrl(CtrlOp::Loop, l), l, 1, 1, RelocKind::Relative, -1);
}

#[test]
fn lea_rip_label() {
    // lea rax, [rip+.L] : 48 8D 05 <disp32>, PC32 addend -4
    let l = LabelId::new();
    check(lea(RAX, MemAddress::label(l), QWORD), l, 3, 4, RelocKind::Relative, -4);
}

#[test]
fn jump_table_indexed_by_label() {
    // jmp [rcx*8 + .Ltable] : FF 24 CD <disp32>, R_X86_64_32S
    let l = LabelId::new();
    let table = mem(MemAddress::new().index_scale(RCX, Scale::Eight).disp_label(l));
    check(jmp_indirect(table), l, 3, 4, RelocKind::AbsoluteSigned, 0);
}

#[test]
fn push_label() {
    let l = LabelId::new();
    check(stack(StackOp::Push, Operand::Label(l), None), l, 1, 4, RelocKind::AbsoluteSigned, 0);
}

#[test]
fn mov_label_imm64() {
    let l = LabelId::new();
    check(bin(BinOp::Mov, reg(RAX), Operand::Label(l), QWORD), l, 2, 8, RelocKind::Absolute, 0);
}

#[test]
fn labels_are_unique() {
    assert_ne!(LabelId::new(), LabelId::new());
}
