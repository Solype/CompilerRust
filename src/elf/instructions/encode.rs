use std::vec;
use crate::elf::instructions::encode_unary::encode_unary;

use super::encode_alu::encode_binary;
use super::enums::*;



impl Instruction {

    pub fn encode(&self, default_size: Size) -> EncodeInformation {
        match self {

            // =========================
            // BINARY (MOV, ADD, etc.)
            // =========================
            Instruction::Binary { op, dst, src, size } => {
                let size = size.unwrap_or(default_size);
                encode_binary(*op, dst, src, size)
            }

            // =========================
            // CONTROL FLOW
            // =========================
            Instruction::Ctrl { op, target } => op.encode(target),

            // =========================
            // SYSTEM
            // =========================
            Instruction::Sys { op } => match op {
                SysOp::Int(n) => EncodeInformation { data: vec![0xCD, *n], ..Default::default() },
                SysOp::Syscall => EncodeInformation { data: vec![0x0F, 0x05], ..Default::default() },
                SysOp::Sysenter => EncodeInformation { data: vec![0x0F, 0x34], ..Default::default() },
            },

            
            // =========================
            // UNARY
            // =========================
            Instruction::Unary { op, dst, size } => {
                let size = size.unwrap_or(default_size);
                encode_unary(*op, dst, size)
            },

            // =========================
            // LOCAL SYMBOL FOR JMP
            // =========================
            Instruction::LocalSym(_) => EncodeInformation::default(),

            _ => unimplemented!()
        }
    }
}