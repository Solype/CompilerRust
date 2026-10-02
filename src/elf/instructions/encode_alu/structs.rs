use super::super::{
    enums::*,
};

// ======================================================
// Encoding families
// ======================================================

#[derive(Debug)]
pub(super) enum BinaryEncoding {
    Alu {
        opcode_rm_r: u8,
        opcode_r_rm: u8,
        opcode_imm: u8,
        opcode_imm8: Option<u8>,
        modrm_ext: u8,
    },

    Mov {
        opcode_rm_r: u8,
        opcode_r_rm: u8,
        opcode_imm: u8,
    },

    Xchg {
        opcode: u8,
    },

    Sse {
        prefix: u8,
        opcode: u8,
    }
}

pub(super) fn get_op_codes(op: BinOp, size: Size) -> BinaryEncoding {
    match op {

        // --------------------------------------------------
        // MOV
        // --------------------------------------------------
        BinOp::Mov => BinaryEncoding::Mov {
            opcode_rm_r: if size == Size::U8 { 0x88 } else { 0x89 },
            opcode_r_rm: if size == Size::U8 { 0x8A } else { 0x8B },
            opcode_imm:  if size == Size::U8 { 0xC6 } else { 0xC7 },
        },

        // --------------------------------------------------
        // XCHG
        // --------------------------------------------------
        BinOp::Xchg => BinaryEncoding::Xchg {
            opcode: if size == Size::U8 { 0x86 } else { 0x87 },
        },

        // --------------------------------------------------
        // ALU
        // --------------------------------------------------
        BinOp::Add => BinaryEncoding::Alu {
            opcode_rm_r: if size == Size::U8 { 0x00 } else { 0x01 },
            opcode_r_rm: if size == Size::U8 { 0x02 } else { 0x03 },
            opcode_imm:  if size == Size::U8 { 0x80 } else { 0x81 },
            opcode_imm8: if size == Size::U8 { None } else { Some(0x83) },
            modrm_ext: 0,
        },

        BinOp::Adc => BinaryEncoding::Alu {
            opcode_rm_r: if size == Size::U8 { 0x10 } else { 0x11 },
            opcode_r_rm: if size == Size::U8 { 0x12 } else { 0x13 },
            opcode_imm:  if size == Size::U8 { 0x80 } else { 0x81 },
            opcode_imm8: if size == Size::U8 { None } else { Some(0x83) },
            modrm_ext: 2,
        },

        BinOp::Sbb => BinaryEncoding::Alu {
            opcode_rm_r: if size == Size::U8 { 0x18 } else { 0x19 },
            opcode_r_rm: if size == Size::U8 { 0x1A } else { 0x1B },

            opcode_imm:  if size == Size::U8 { 0x80 } else { 0x81 },
            opcode_imm8: if size == Size::U8 { None } else { Some(0x83) },

            modrm_ext: 3,
        },

        BinOp::Or => BinaryEncoding::Alu {
            opcode_rm_r: if size == Size::U8 { 0x08 } else { 0x09 },
            opcode_r_rm: if size == Size::U8 { 0x0A } else { 0x0B },
            opcode_imm:  if size == Size::U8 { 0x80 } else { 0x81 },
            opcode_imm8: if size == Size::U8 { None } else { Some(0x83) },
            modrm_ext: 1,
        },

        BinOp::And => BinaryEncoding::Alu {
            opcode_rm_r: if size == Size::U8 { 0x20 } else { 0x21 },
            opcode_r_rm: if size == Size::U8 { 0x22 } else { 0x23 },
            opcode_imm:  if size == Size::U8 { 0x80 } else { 0x81 },
            opcode_imm8: if size == Size::U8 { None } else { Some(0x83) },
            modrm_ext: 4,
        },

        BinOp::Sub => BinaryEncoding::Alu {
            opcode_rm_r: if size == Size::U8 { 0x28 } else { 0x29 },
            opcode_r_rm: if size == Size::U8 { 0x2A } else { 0x2B },
            opcode_imm:  if size == Size::U8 { 0x80 } else { 0x81 },
            opcode_imm8: if size == Size::U8 { None } else { Some(0x83) },
            modrm_ext: 5,
        },

        BinOp::Xor => BinaryEncoding::Alu {
            opcode_rm_r: if size == Size::U8 { 0x30 } else { 0x31 },
            opcode_r_rm: if size == Size::U8 { 0x32 } else { 0x33 },
            opcode_imm:  if size == Size::U8 { 0x80 } else { 0x81 },
            opcode_imm8: if size == Size::U8 { None } else { Some(0x83) },
            modrm_ext: 6,
        },

        BinOp::Cmp => BinaryEncoding::Alu {
            opcode_rm_r: if size == Size::U8 { 0x38 } else { 0x39 },
            opcode_r_rm: if size == Size::U8 { 0x3A } else { 0x3B },
            opcode_imm:  if size == Size::U8 { 0x80 } else { 0x81 },
            opcode_imm8: if size == Size::U8 { None } else { Some(0x83) },
            modrm_ext: 7,
        },

        BinOp::Test => BinaryEncoding::Alu {
            opcode_rm_r: if size == Size::U8 { 0x84 } else { 0x85 },
            opcode_r_rm: if size == Size::U8 { 0x84 } else { 0x85 },
            opcode_imm:  if size == Size::U8 { 0xF6 } else { 0xF7 },
            opcode_imm8: None,
            modrm_ext: 0,
        },


        // --------------------------------------------------
        // SSE scalar double
        // --------------------------------------------------
        BinOp::LoadF => {
            match size {
                Size::U64 => BinaryEncoding::Sse { prefix: 0xF2, opcode: 0x10, },
                _ => BinaryEncoding::Sse { prefix: 0xF3, opcode: 0x10, },
            }
        },

        BinOp::StoreF => {
            match size {
                Size::U64 => BinaryEncoding::Sse { prefix: 0xF2, opcode: 0x11 },
                _ => BinaryEncoding::Sse { prefix: 0xF3, opcode: 0x11 },
            }
        }

        BinOp::AddF => {
            match size {
                Size::U64 => BinaryEncoding::Sse { prefix: 0xF2, opcode: 0x58, },
                _ => BinaryEncoding::Sse { prefix: 0xF3, opcode: 0x58, }
            }
        }

        BinOp::SubF => {
            match size {
                Size::U64 => BinaryEncoding::Sse { prefix: 0xF2, opcode: 0x5C, },
                _ => BinaryEncoding::Sse { prefix: 0xF3, opcode: 0x5C, }
            }
        }

        BinOp::MulF => {
            match size {
                Size::U64 => BinaryEncoding::Sse { prefix: 0xF2, opcode: 0x59, },
                _ => BinaryEncoding::Sse { prefix: 0xF3, opcode: 0x59, }
            }
        }

        BinOp::DivF => {
            match size {
                Size::U64 => BinaryEncoding::Sse { prefix: 0xF2, opcode: 0x5E, },
                _ => BinaryEncoding::Sse { prefix: 0xF3, opcode: 0x5E, },
            }
        }

        BinOp::ComiF => {
            match size {
                Size::U64 => BinaryEncoding::Sse { prefix: 0x66, opcode: 0x2F, },
                _ => BinaryEncoding::Sse { prefix: 0x00, opcode: 0x2F, }
            }
        }

        BinOp::UcomiF => {
            match size {
                Size::U64 => BinaryEncoding::Sse { prefix: 0x66, opcode: 0x2E, },
                _ => BinaryEncoding::Sse { prefix: 0x00, opcode: 0x2E, }
            }
        }

        BinOp::XorF => {
            match size {
                Size::U64 => BinaryEncoding::Sse { prefix: 0x66, opcode: 0x57, },
                _ => BinaryEncoding::Sse { prefix: 0x00, opcode: 0x57, }
            }
        }

        // --------------------------------------------------
        // SSE scalar single
        // --------------------------------------------------

    }
}
