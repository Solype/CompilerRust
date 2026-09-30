use super::helpers::*;
use crate::elf::instructions::{register::*, *};

/// inc, dec, neg, not sur 8, 16, 32 bits et en mémoire
pub fn unary_ops() -> Vec<Instruction> {
    let mut code = Vec::new();
    for (i, op) in [UnaryOp::Inc, UnaryOp::Dec, UnaryOp::Neg, UnaryOp::Not]
        .into_iter()
        .enumerate()
    {
        code.push(unary(op, reg(GPRS[i]), BYTE));
        code.push(unary(op, reg(GPRS[(i + 1) % 4]), WORD));
        code.push(unary(op, reg(GPRS[(i + 2) % 4]), DWORD));
        code.push(unary(op, mem(at(RBX).disp(4 * i as i32)), DWORD));
    }
    code.push(ret());
    return code;
}
