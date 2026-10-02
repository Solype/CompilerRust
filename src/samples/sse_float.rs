use crate::elf::instructions::{register::*, *};
use super::helpers::*;

/// SSE scalar floats, in double (`None`) then single precision (`DWORD`)
pub fn sse_float() -> Vec<Instruction> {
    let f = || var("my_float");
    let mut code = Vec::new();
    for size in [None, DWORD] {
        code.extend([
            bin(BinOp::LoadF,  reg(XMM0), f(), size),
            bin(BinOp::StoreF, reg(XMM0), f(), size),
            bin(BinOp::LoadF,  reg(XMM1), reg(XMM0), size),
            bin(BinOp::AddF,   reg(XMM0), f(), size),
            bin(BinOp::AddF,   reg(XMM1), reg(XMM0), size),
            bin(BinOp::SubF,   reg(XMM2), reg(XMM1), size),
            bin(BinOp::SubF,   reg(XMM2), f(), size),
            bin(BinOp::MulF,   reg(XMM3), reg(XMM2), size),
            bin(BinOp::MulF,   reg(XMM3), f(), size),
            bin(BinOp::DivF,   reg(XMM4), reg(XMM3), size),
            bin(BinOp::DivF,   reg(XMM4), f(), size),
            bin(BinOp::ComiF,  reg(XMM5), reg(XMM4), size),
            bin(BinOp::ComiF,  reg(XMM5), f(), size),
            bin(BinOp::UcomiF, reg(XMM6), reg(XMM5), size),
            bin(BinOp::UcomiF, reg(XMM6), f(), size),
            bin(BinOp::XorF,   reg(XMM7), reg(XMM7), size),
            bin(BinOp::XorF,   reg(XMM7), f(), size),
            bin(BinOp::AndF,   reg(XMM8), reg(XMM7), size),
            bin(BinOp::AndF,   reg(XMM8), f(), size),
            bin(BinOp::SqrtF,  reg(XMM9), reg(XMM8), size),
            bin(BinOp::SqrtF,  reg(XMM9), f(), size),
            bin(BinOp::MinF,   reg(XMM10), reg(XMM9), size),
            bin(BinOp::MinF,   reg(XMM10), f(), size),
            bin(BinOp::MaxF,   reg(XMM11), reg(XMM10), size),
            bin(BinOp::MaxF,   reg(XMM11), f(), size),
        ]);
    }
    code.push(ret());
    code
}
