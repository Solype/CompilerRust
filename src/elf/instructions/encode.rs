use std::vec;

use super::{
    encode_bitop::encode_bit,
    encode_cond_mov::encode_cmovcc,
    encode_lea::encode_lea,
    encode_setcc::encode_setcc,
    encode_shift_rotate::encode_shift_rotate,
    encode_stack::encode_stack,
    encode_unary::encode_unary,
    enums::*,
    encode_alu::encode_binary
};



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
            // UNARY
            // =========================
            Instruction::Stack { op, value, size } => {
                let size = size.unwrap_or(default_size);
                encode_stack(*op, value, size)
            }

            // =========================
            // SETCC
            // =========================
            Instruction::SetCC { op, dst } => encode_setcc(*op, dst),

            // =========================
            // BITOP
            // =========================
            Instruction::Bit { op, dst, src, size } => {
                let size = size.unwrap_or(default_size);
                encode_bit(*op, dst, src, size)
            },

            Instruction::Shift { op, dst, src, size } => {
                let size = size.unwrap_or(default_size);
                encode_shift_rotate(op, dst, src, size)
            }

            Instruction::CMovCC { cc, dst, src, size } => {
                let size = size.unwrap_or(default_size);
                encode_cmovcc(cc, dst, src, size)
            }

            Instruction::Lea { src, dst, size } => {
                let size = size.unwrap_or(default_size);
                encode_lea(dst, src, size)
            }

            // =========================
            // LOCAL SYMBOL FOR JMP
            // =========================
            Instruction::LocalSym(_) => EncodeInformation::default(),

            _ => unimplemented!()
        }
    }
}