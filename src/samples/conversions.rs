use crate::elf::instructions::{register::*, *};
use super::helpers::*;

/// Conversions entier -> flottant, depuis un registre puis depuis la mémoire
pub fn conversions() -> Vec<Instruction> {
    let mut code = Vec::new();
    for size in [None, DWORD] {
        code.extend([
            convert(ConvOp::Cvtsi2sd, XMM0, reg(RAX), size),
            convert(ConvOp::Cvtsi2sd, XMM9, reg(R12), size),
            convert(ConvOp::Cvtsi2sd, XMM1, mem(at(RBX)), size),
            convert(ConvOp::Cvtsi2sd, XMM2, var("my_float"), size),
        ]);
    }
    code.push(ret());
    code
}
