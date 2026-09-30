use crate::elf::instructions::{register::*, *};
use super::helpers::*;

/// bsf, bsr sur 16, 32, 64 bits et en mémoire
pub fn bit_scan() -> Vec<Instruction> {
    let mut code = Vec::new();
    for op in [BitScanOp::Bsf, BitScanOp::Bsr] {
        code.extend([
            bitscan(op, RAX, reg(RBX), QWORD),
            bitscan(op, RCX, reg(RDX), QWORD),
            bitscan(op, RAX, reg(RBX), DWORD),
            bitscan(op, RAX, reg(RBX), WORD),
            bitscan(op, RAX, mem(at(RBX)), QWORD),
            bitscan(op, RAX, mem(at(RBX).index(RCX)), QWORD),
        ]);
    }
    code.push(ret());
    code
}
