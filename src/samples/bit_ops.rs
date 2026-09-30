use crate::elf::instructions::{register::*, *};
use super::helpers::*;

/// bt, bts, btr, btc : reg/imm, reg/reg, mem/imm, mem/reg
pub fn bit_ops() -> Vec<Instruction> {
    let mut code = Vec::new();
    for (i, op) in [BitOp::Bt, BitOp::Bts, BitOp::Btr, BitOp::Btc].into_iter().enumerate() {
        let disp = 8 + 4 * i as i32;
        code.push(bit(op, reg(GPRS[i]), imm(3 + i as i64), DWORD));
        code.push(bit(op, reg(GPRS[i]), reg(GPRS[(i + 1) % 4]), DWORD));
        code.push(bit(op, mem(at(RBX).disp(disp)), imm(2), DWORD));
        code.push(bit(op, mem(at(RBX).disp(disp)), reg(RAX), DWORD));
    }
    code.push(ret());
    code
}
