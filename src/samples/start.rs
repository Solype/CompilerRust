use super::helpers::*;
use crate::elf::instructions::{register::*, *};

/// Entry point: exit(42)
pub fn start() -> Vec<Instruction> {
    vec![
        bin(BinOp::Mov, reg(RAX), imm(60), None),
        bin(BinOp::Mov, reg(RDI), imm(42), None),
        sys(SysOp::Syscall),
    ]
}
