//! Formes d'instruction que l'encodeur doit refuser.

use super::*;

fn encode(ins: Instruction) {
    ins.encode(Size::U64);
}

#[test]
#[should_panic(expected = "unsupported target for rel32")]
fn jcc_register_target() {
    // Les sauts conditionnels n'ont pas de forme indirecte
    encode(Instruction::Ctrl { op: CtrlOp::JmpCC(ConditionCode::G), target: reg(RAX) });
}

#[test]
#[should_panic(expected = "unsupported target for rel8")]
fn loop_register_target() {
    encode(Instruction::Ctrl { op: CtrlOp::Loop, target: reg(RCX) });
}
