use super::helpers::*;
use crate::elf::instructions::{register::*, *};

/// mov (toutes tailles et adressages), movzx, movsx
pub fn mov() -> Vec<Instruction> {
    return vec![
        bin(BinOp::Mov, reg(R8), imm(0x12), BYTE),
        bin(BinOp::Mov, reg(R9), imm(0x1234), WORD),
        bin(BinOp::Mov, reg(R10), imm(0x12345678), DWORD),
        bin(BinOp::Mov, reg(RAX), imm(0x123456789abcdef0), None),
        bin(BinOp::Mov, reg(RDX), imm(0x401000), None),
        bin(BinOp::Mov, reg(R11), reg(RBX), DWORD),
        bin(BinOp::Mov, reg(R12), sym("my_data"), DWORD),
        bin(BinOp::Mov, var("my_data"), imm(0x41), BYTE),
        bin(BinOp::Mov, var("my_data"), reg(RAX), DWORD),
        bin(BinOp::Mov, var("my_data"), reg(R15), DWORD),
        bin(BinOp::Mov, reg(R13), var("my_data"), DWORD),
        bin(BinOp::Mov, reg(RAX), mem(at(RBX).index(RCX)), DWORD),
        bin(
            BinOp::Mov,
            reg(RAX),
            mem(at(RBX).index(RCX).scale(Scale::Four)),
            DWORD,
        ),
        bin(
            BinOp::Mov,
            mem(at(RBX).index(RCX).scale(Scale::Eight).disp(16)),
            reg(RAX),
            DWORD,
        ),
        bin(BinOp::Movzx, reg(RAX), reg(RBX), BYTE),
        bin(BinOp::Movzx, reg(RAX), reg(R10), BYTE),
        bin(BinOp::Movzx, reg(RCX), mem(at(RBX)), BYTE),
        bin(BinOp::Movsx, reg(RDX), reg(RAX), BYTE),
        bin(BinOp::Movsx, reg(RCX), mem(at(RBX)), BYTE),
        bin(BinOp::Movsx, reg(RBX), mem(at(RBX)), WORD),
        ret(),
    ];
}
