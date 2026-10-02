use crate::elf::instructions::{register::*, *};
use super::helpers::*;

/// Integer <-> float and double <-> single conversions, from a register then from memory,
/// and movd / movq raw copies between GPR and XMM
pub fn conversions() -> Vec<Instruction> {
    let mut code = Vec::new();
    for op in [ConvOp::Cvtsi2sd, ConvOp::Cvtsi2ss] {
        for size in [None, DWORD] {
            code.extend([
                convert(op, XMM0, reg(RAX), size),
                convert(op, XMM9, reg(R12), size),
                convert(op, XMM1, mem(at(RBX)), size),
                convert(op, XMM2, var("my_float"), size),
            ]);
        }
    }
    for op in [ConvOp::Cvtsd2si, ConvOp::Cvtss2si, ConvOp::Cvttsd2si, ConvOp::Cvttss2si] {
        for size in [None, DWORD] {
            code.extend([
                convert(op, RAX, reg(XMM0), size),
                convert(op, R12, reg(XMM9), size),
                convert(op, RCX, mem(at(RBX)), size),
                convert(op, RDX, var("my_float"), size),
            ]);
        }
    }
    for op in [ConvOp::Cvtsd2ss, ConvOp::Cvtss2sd] {
        code.extend([
            convert(op, XMM0, reg(XMM1), None),
            convert(op, XMM8, reg(XMM15), None),
            convert(op, XMM1, mem(at(RBX)), None),
            convert(op, XMM2, var("my_float"), None),
        ]);
    }
    for op in [ConvOp::Movd, ConvOp::Movq] {
        code.extend([
            convert(op, XMM0, reg(RAX), None),
            convert(op, XMM9, reg(R10), None),
            convert(op, RAX, reg(XMM0), None),
            convert(op, R11, reg(XMM13), None),
            convert(op, XMM1, mem(at(RBX).disp(8)), None),
            convert(op, XMM2, var("my_float"), None),
        ]);
    }
    code.push(convert(ConvOp::Movq, XMM10, reg(XMM3), None));
    code.push(ret());
    code
}
