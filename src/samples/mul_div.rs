use super::helpers::*;
use crate::elf::instructions::{register::*, *};

/// div, idiv, mul, imul (1, 2 et 3 opérandes)
pub fn mul_div() -> Vec<Instruction> {
    let none = || Operand::NoOperand;
    return vec![
        complex(ComplexBinOp::Div, none(), reg(RCX), None, DWORD),
        complex(ComplexBinOp::Div, none(), mem(at(RBX).disp(8)), None, DWORD),
        complex(
            ComplexBinOp::Div,
            none(),
            mem(at(RBP).disp(-16)),
            None,
            DWORD,
        ),
        complex(
            ComplexBinOp::Div,
            none(),
            mem(at(RBX).index(RCX)),
            None,
            DWORD,
        ),
        complex(
            ComplexBinOp::Div,
            none(),
            mem(at(RBX).index_scale(RCX, Scale::Four)),
            None,
            DWORD,
        ),
        complex(
            ComplexBinOp::Div,
            none(),
            mem(at(RBX).index_scale(RCX, Scale::Eight).disp(16)),
            None,
            DWORD,
        ),
        complex(ComplexBinOp::Div, none(), reg(RBX), None, None),
        complex(ComplexBinOp::Div, none(), mem(at(RBX)), None, None),
        complex(ComplexBinOp::Idiv, none(), reg(RBX), None, DWORD),
        complex(
            ComplexBinOp::Idiv,
            none(),
            mem(at(RBX).disp(4)),
            None,
            DWORD,
        ),
        complex(ComplexBinOp::Mul, none(), reg(RBX), None, DWORD),
        complex(ComplexBinOp::Mul, none(), mem(at(RBX)), None, DWORD),
        // imul à 1 opérande
        complex(ComplexBinOp::Imul, none(), reg(RBX), None, DWORD),
        complex(
            ComplexBinOp::Imul,
            none(),
            mem(at(RBX).index_scale(RCX, Scale::Two)),
            None,
            None,
        ),
        // imul à 2 opérandes
        complex(ComplexBinOp::Imul, reg(RAX), reg(RBX), None, DWORD),
        complex(ComplexBinOp::Imul, reg(RCX), mem(at(RBX)), None, DWORD),
        complex(
            ComplexBinOp::Imul,
            reg(RDX),
            mem(at(RBP).disp(-8)),
            None,
            None,
        ),
        complex(
            ComplexBinOp::Imul,
            reg(RAX),
            mem(at(RBX).index_scale(RCX, Scale::Four)),
            None,
            DWORD,
        ),
        // imul à 3 opérandes (imm8 et imm32)
        complex(
            ComplexBinOp::Imul,
            reg(RAX),
            reg(RBX),
            Some(imm(-255)),
            DWORD,
        ),
        complex(
            ComplexBinOp::Imul,
            reg(RCX),
            reg(RDX),
            Some(imm(127)),
            DWORD,
        ),
        complex(
            ComplexBinOp::Imul,
            reg(RAX),
            reg(RBX),
            Some(imm(128)),
            DWORD,
        ),
        complex(
            ComplexBinOp::Imul,
            reg(RDX),
            mem(at(RBX)),
            Some(imm(1000)),
            DWORD,
        ),
        complex(
            ComplexBinOp::Imul,
            reg(RAX),
            mem(at(RBX).index_scale(RCX, Scale::Eight).disp(16)),
            Some(imm(9)),
            None,
        ),
        ret(),
    ];
}
