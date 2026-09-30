use crate::elf::instructions::{register::*, *};
use super::helpers::*;

/// Point d'entrée : exit(42)
pub fn start() -> Vec<Instruction> {
    vec![
        bin(BinOp::Mov, reg(RAX), imm(60), None),
        bin(BinOp::Mov, reg(RDI), imm(42), None),
        sys(SysOp::Syscall),
    ]
}
