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
        extend(ExtendOp::Movzx, RAX, reg(RBX), BYTE, DWORD),
        extend(ExtendOp::Movzx, RAX, reg(R10), BYTE, DWORD),
        extend(ExtendOp::Movzx, RCX, mem(at(RBX)), BYTE, DWORD),
        extend(ExtendOp::Movsx, RDX, reg(RAX), BYTE, DWORD),
        extend(ExtendOp::Movsx, RCX, mem(at(RBX)), BYTE, DWORD),
        extend(ExtendOp::Movsx, RBX, mem(at(RBX)), WORD, DWORD),
        extend(ExtendOp::Movzx, RAX, reg(RBX), BYTE, QWORD),
        extend(ExtendOp::Movzx, RBX, mem(at(RBX)), WORD, QWORD),
        extend(ExtendOp::Movsx, RAX, reg(RSI), BYTE, QWORD),
        extend(ExtendOp::Movsx, R9, mem(at(R9)), WORD, QWORD),
        extend(ExtendOp::Movsxd, RAX, reg(RBX), None, None),
        extend(ExtendOp::Movsxd, R15, mem(at(RBX).index_scale(R8, Scale::Two)), None, None),
        extend(ExtendOp::Movsxd, RCX, var("my_data"), None, None),
        ret(),
    ];
}
