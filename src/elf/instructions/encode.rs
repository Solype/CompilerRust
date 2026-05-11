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
    struct_encode_information::*,
    encode_bitscan::encode_bitscan,
    encode_complexbin::encode_complex_binary,
    encode_prefix::encode_prefix,
    encode_str::encode_str,
    encode_alu::encode_binary
};

impl Instruction {

    pub fn encode(&self, default_size: Size) -> EncodeInformation {
        match self {
            Instruction::Binary { op, dst, src, size } => encode_binary(*op, dst, src, size.unwrap_or(default_size)),
            Instruction::Ctrl { op, target } => op.encode(target),
            Instruction::Unary { op, dst, size } => encode_unary(*op, dst, size.unwrap_or(default_size)),
            Instruction::Stack { op, value, size } => encode_stack(*op, value, size.unwrap_or(default_size)),
            Instruction::SetCC { op, dst } => encode_setcc(*op, dst),
            Instruction::Prefix { prefix, ins } => encode_prefix(prefix, ins, default_size),
            Instruction::Bit { op, dst, src, size } => encode_bit(*op, dst, src, size.unwrap_or(default_size)),
            Instruction::Shift { op, dst, src, size } => encode_shift_rotate(op, dst, src, size.unwrap_or(default_size)),
            Instruction::CMovCC { cc, dst, src, size } => encode_cmovcc(cc, dst, src, size.unwrap_or(default_size)),
            Instruction::Lea { src, dst, size } => encode_lea(dst, src, size.unwrap_or(default_size)),
            Instruction::ComplexBinary { op, dst, src, extra, size } => encode_complex_binary(op, dst, src, extra, size.unwrap_or(default_size)),
            Instruction::BitScan { op, dst, src, size } => encode_bitscan(op, dst, src, size.unwrap_or(default_size)),
            Instruction::String { op, size } => encode_str(op, size.unwrap_or(default_size)),
            Instruction::LocalSym(_) => EncodeInformation::default(),
            Instruction::Sys { op } => match op {
                SysOp::Int(n) => EncodeInformation { data: vec![0xCD, *n], ..Default::default() },
                SysOp::Syscall => EncodeInformation { data: vec![0x0F, 0x05], ..Default::default() },
                SysOp::Sysenter => EncodeInformation { data: vec![0x0F, 0x34], ..Default::default() },
            },
            Instruction::Nop(val) => encode_nop(val),
        }
    }
}

pub(super) fn encode_nop(val: &u8) -> EncodeInformation {

    let data = match val {

        // ==========================================
        // 1-byte NOP
        // ==========================================
        1 => vec![ 0x90 ],

        // ==========================================
        // 2-byte NOP
        // ==========================================
        2 => vec![ 0x66, 0x90, ],

        // ==========================================
        // 3-byte NOP
        // ==========================================
        3 => vec![ 0x0F, 0x1F, 0x00, ],

        // ==========================================
        // 4-byte NOP
        // ==========================================
        4 => vec![ 0x0F, 0x1F, 0x40, 0x00, ],

        // ==========================================
        // 5-byte NOP
        // ==========================================
        5 => vec![ 0x0F, 0x1F, 0x44, 0x00, 0x00, ],

        // ==========================================
        // 6-byte NOP
        // ==========================================
        6 => vec![ 0x66, 0x0F, 0x1F, 0x44, 0x00, 0x00, ],

        // ==========================================
        // 7-byte NOP
        // ==========================================
        7 => vec![ 0x0F, 0x1F, 0x80, 0x00, 0x00, 0x00, 0x00, ],

        // ==========================================
        // 8-byte NOP
        // ==========================================
        8 => vec![ 0x0F, 0x1F, 0x84, 0x00, 0x00, 0x00, 0x00, 0x00, ],

        // ==========================================
        // 9-byte NOP
        // ==========================================
        9 => vec![ 0x66, 0x0F, 0x1F, 0x84, 0x00, 0x00, 0x00, 0x00, 0x00, ],

        _ => panic!("unsupported NOP size: {}", val),
    };

    EncodeInformation::from_bytes(data)
}