use crate::elf::instructions::{register::*, *};
use super::helpers::*;

/// add, adc, sub, sbb, and, or, xor, cmp, test : chacune avec
/// reg/imm, reg/reg, et une forme mémoire
pub fn alu() -> Vec<Instruction> {
    let mut code = Vec::new();
    for op in [BinOp::Add, BinOp::Adc, BinOp::Sub, BinOp::Sbb, BinOp::And, BinOp::Or, BinOp::Xor, BinOp::Cmp, BinOp::Test] {
        code.push(bin(op, reg(RAX), imm(0x7F), DWORD));
        code.push(bin(op, reg(RBX), reg(RAX), DWORD));
        code.push(bin(op, mem(at(RBX)), imm(3), DWORD));
    }
    code.extend([
        bin(BinOp::And, mem(at(RBX)), reg(RBX), DWORD),
        bin(BinOp::Xor, reg(RCX), mem(at(RBX)), DWORD),
        bin(BinOp::Cmp, reg(RCX), mem(at(RBX)), DWORD),
        ret(),
    ]);
    code
}
