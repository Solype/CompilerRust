use super::helpers::*;
use crate::elf::instructions::{register::*, *};

/// shl, shr, sar, rol, ror: by immediate and by cl, on register and memory
pub fn shift_rotate() -> Vec<Instruction> {
    let mut code = Vec::new();
    for op in [
        ShiftOp::Shl,
        ShiftOp::Shr,
        ShiftOp::Sar,
        ShiftOp::Rol,
        ShiftOp::Ror,
    ] {
        code.push(shift(op, reg(RAX), imm(1), DWORD));
        code.push(shift(op, reg(RBX), imm(3), DWORD));
        code.push(shift(op, reg(RDX), reg(RCX), DWORD));
        code.push(shift(op, mem(at(RBX)), imm(2), DWORD));
        code.push(shift(op, mem(at(RBX)), reg(RCX), DWORD));
    }
    code.push(ret());
    code
}
