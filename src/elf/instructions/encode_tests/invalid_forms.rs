//! Instruction forms the encoder must reject.

use super::*;

fn encode(ins: Instruction) {
    ins.encode(Size::U64);
}

#[test]
#[should_panic(expected = "unsupported target for rel32")]
fn jcc_register_target() {
    // Conditional jumps have no indirect form
    encode(Instruction::Ctrl { op: CtrlOp::JmpCC(ConditionCode::G), target: reg(RAX) });
}

#[test]
#[should_panic(expected = "unsupported target for rel8")]
fn loop_register_target() {
    encode(Instruction::Ctrl { op: CtrlOp::Loop, target: reg(RCX) });
}

#[test]
#[should_panic(expected = "destination must be a GPR")]
fn extend_xmm_destination() {
    encode(extend(ExtendOp::Movzx, XMM0, reg(RBX), BYTE, DWORD));
}

#[test]
#[should_panic(expected = "source must be a GPR or a memory address")]
fn extend_immediate_source() {
    encode(extend(ExtendOp::Movsx, RAX, imm(1), BYTE, DWORD));
}

#[test]
#[should_panic(expected = "source must be 8 or 16 bits")]
fn movzx_without_source_size() {
    // the source size cannot be inferred from a register
    encode(extend(ExtendOp::Movzx, RAX, reg(RBX), None, DWORD));
}

#[test]
#[should_panic(expected = "source must be 8 or 16 bits")]
fn movsx_dword_source() {
    // that is movsxd
    encode(extend(ExtendOp::Movsx, RAX, reg(RBX), DWORD, QWORD));
}

#[test]
#[should_panic(expected = "destination must be wider than the source")]
fn movzx_word_to_word() {
    encode(extend(ExtendOp::Movzx, RAX, reg(RBX), WORD, WORD));
}

#[test]
#[should_panic(expected = "destination must be 16, 32 or 64 bits")]
fn movsx_byte_destination() {
    encode(extend(ExtendOp::Movsx, RAX, reg(RBX), BYTE, BYTE));
}

#[test]
#[should_panic(expected = "source must be 32 bits")]
fn movsxd_word_source() {
    encode(extend(ExtendOp::Movsxd, RAX, reg(RBX), WORD, QWORD));
}

#[test]
#[should_panic(expected = "destination must be 64 bits")]
fn movsxd_dword_destination() {
    encode(extend(ExtendOp::Movsxd, RAX, reg(RBX), DWORD, DWORD));
}
