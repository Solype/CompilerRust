//! Relocations : offset (depuis le début de l'instruction), type et addend,
//! comparés à ceux que produit GNU as (`readelf -r`).

use super::*;

fn reloc(ins: Instruction) -> Relocation {
    let info = ins.encode(Size::U64);
    assert_eq!(info.relocations.len(), 1, "{ins:?}");
    info.relocations[0].clone()
}

fn check(ins: Instruction, sym: &str, offset: usize, size: u8, relative: bool, addend: i32) {
    let r = reloc(ins);
    assert_eq!(r.sym, sym);
    assert_eq!(r.offset, offset, "offset");
    assert_eq!(r.size, size, "size");
    assert_eq!(matches!(r.kind, RelocKind::Relative), relative, "kind");
    assert_eq!(r.addend, addend, "addend");
}

#[test]
fn jmp_rel32() {
    check(jmp("target"), "target", 1, 4, true, -4);
}

#[test]
fn call_rel32() {
    check(call("target"), "target", 1, 4, true, -4);
}

#[test]
fn jcc_rel32() {
    check(jcc(ConditionCode::G, "target"), "target", 2, 4, true, -4);
}

#[test]
fn loop_rel8() {
    check(ctrl(CtrlOp::Loop, "target"), "target", 1, 1, true, -1);
}

#[test]
fn rip_load() {
    check(bin(BinOp::Mov, reg(RAX), var("my_data"), DWORD), "my_data", 2, 4, true, -4);
}

#[test]
fn rip_with_imm8_after() {
    // mov byte ptr [rip+my_data], 0x41 : 1 octet après le disp32
    check(bin(BinOp::Mov, var("my_data"), imm(0x41), BYTE), "my_data", 2, 4, true, -5);
}

#[test]
fn rip_with_imm32_after() {
    check(bin(BinOp::Mov, var("my_data"), imm(0x12345678), DWORD), "my_data", 2, 4, true, -8);
}

#[test]
fn rip_with_rex_and_imm8() {
    check(bin(BinOp::Cmp, var("my_data"), imm(1), QWORD), "my_data", 3, 4, true, -5);
}

#[test]
fn rip_behind_lock_prefix() {
    let ins = prefixed(
        vec![Prefix::Lock],
        complex(ComplexBinOp::Cmpxchg, var("my_data"), reg(RCX), None, DWORD),
    );
    check(ins, "my_data", 4, 4, true, -4);
}

#[test]
fn rip_sse() {
    check(bin(BinOp::AddF, reg(XMM0), var("my_float"), None), "my_float", 4, 4, true, -4);
}

#[test]
fn absolute_imm32() {
    check(bin(BinOp::Mov, reg(R12), sym("my_data"), DWORD), "my_data", 2, 4, false, 0);
}

#[test]
fn rip_base_plus_symbol() {
    // [rbx + my_data] : disp32 absolu
    let src = mem(at(RBX).sym("my_data"));
    check(bin(BinOp::Mov, reg(RAX), src, DWORD), "my_data", 2, 4, false, 0);
}

// Avec un octet SIB, le disp32 commence un octet plus loin

#[test]
fn sib_base_index_plus_symbol() {
    // mov eax, [rbx+rcx*4+my_data] : 8B 84 8B <disp32>
    let src = mem(at(RBX).index_scale(RCX, Scale::Four).sym("my_data"));
    check(bin(BinOp::Mov, reg(RAX), src, DWORD), "my_data", 3, 4, false, 0);
}

#[test]
fn sib_rsp_plus_symbol() {
    // mov eax, [rsp+my_data] : 8B 84 24 <disp32>
    let src = mem(at(RSP).sym("my_data"));
    check(bin(BinOp::Mov, reg(RAX), src, DWORD), "my_data", 3, 4, false, 0);
}

#[test]
fn sib_r12_plus_symbol() {
    // mov eax, [r12+my_data] : 41 8B 84 24 <disp32>
    let src = mem(at(R12).sym("my_data"));
    check(bin(BinOp::Mov, reg(RAX), src, DWORD), "my_data", 4, 4, false, 0);
}

#[test]
fn sib_index_only_plus_symbol() {
    // mov eax, [rcx*4+my_data] : 8B 04 8D <disp32>
    let src = mem(MemAddress::new().index_scale(RCX, Scale::Four).sym("my_data"));
    check(bin(BinOp::Mov, reg(RAX), src, DWORD), "my_data", 3, 4, false, 0);
}

#[test]
fn absolute_symbol() {
    // mov eax, ds:[my_data] : 8B 04 25 <disp32>, R_X86_64_32S sans addend
    let src = mem(MemAddress::symbol("my_data").absolute());
    check(bin(BinOp::Mov, reg(RAX), src, DWORD), "my_data", 3, 4, false, 0);
}
