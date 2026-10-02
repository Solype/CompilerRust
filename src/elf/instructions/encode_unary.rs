use super::{
    enums::*,
    modrm::*,
    register::*,
    struct_encode_information::*,
    utils::{emit_rex, emit_size_prefix},
};

#[derive(Debug, Clone, Copy)]
enum UnaryEncoding {
    /// plain opcode without ModRM
    Simple { opcode: &'static [u8] },

    /// opcode + ModRM /digit
    ModRm { opcode: u8, modrm_ext: u8 },
}

impl UnaryOp {
    fn encoding(self, size: Size) -> UnaryEncoding {
        match self {
            // ==========================================
            // rm unary ops
            // ==========================================
            UnaryOp::Inc => UnaryEncoding::ModRm {
                opcode: if size == Size::U8 { 0xFE } else { 0xFF },
                modrm_ext: 0,
            },

            UnaryOp::Dec => UnaryEncoding::ModRm {
                opcode: if size == Size::U8 { 0xFE } else { 0xFF },
                modrm_ext: 1,
            },

            UnaryOp::Not => UnaryEncoding::ModRm {
                opcode: if size == Size::U8 { 0xF6 } else { 0xF7 },
                modrm_ext: 2,
            },

            UnaryOp::Neg => UnaryEncoding::ModRm {
                opcode: if size == Size::U8 { 0xF6 } else { 0xF7 },
                modrm_ext: 3,
            },

            // ==========================================
            // no operand
            // ==========================================
            UnaryOp::Nop => UnaryEncoding::Simple { opcode: &[0x90] },

            // ==========================================
            // sign extension accumulator -> high regs
            // ==========================================
            UnaryOp::Cwd => UnaryEncoding::Simple {
                opcode: &[0x66, 0x99],
            }, // AX -> DX:AX (16-bit)
            UnaryOp::Cdq => UnaryEncoding::Simple { opcode: &[0x99] }, // EAX -> EDX:EAX (32-bit)
            UnaryOp::Cqo => UnaryEncoding::Simple {
                opcode: &[0x48, 0x99],
            }, // REX.W + CQO
            UnaryOp::Cbw => UnaryEncoding::Simple {
                opcode: &[0x66, 0x98],
            },
            UnaryOp::Cwde => UnaryEncoding::Simple { opcode: &[0x98] },
            UnaryOp::Cdqe => UnaryEncoding::Simple {
                opcode: &[0x48, 0x98],
            },

            UnaryOp::Ud2 => UnaryEncoding::Simple {
                opcode: &[0x0F, 0x0B],
            },
            UnaryOp::Hlt => UnaryEncoding::Simple { opcode: &[0xF4] },
            UnaryOp::Pause => UnaryEncoding::Simple {
                opcode: &[0xF3, 0x90],
            },
            UnaryOp::Fwait => UnaryEncoding::Simple { opcode: &[0x9B] },

            // ==========================================
            // carry flag ops
            // ==========================================
            UnaryOp::Clc => UnaryEncoding::Simple { opcode: &[0xF8] },
            UnaryOp::Stc => UnaryEncoding::Simple { opcode: &[0xF9] },
            UnaryOp::Cld => UnaryEncoding::Simple { opcode: &[0xFC] },
            UnaryOp::Std => UnaryEncoding::Simple { opcode: &[0xFD] },
            UnaryOp::Cmc => UnaryEncoding::Simple { opcode: &[0xF5] },
            UnaryOp::Cli => UnaryEncoding::Simple { opcode: &[0xFA] },
            UnaryOp::Sti => UnaryEncoding::Simple { opcode: &[0xFB] },
            UnaryOp::Lahf => UnaryEncoding::Simple { opcode: &[0x9F] },
            UnaryOp::Sahf => UnaryEncoding::Simple { opcode: &[0x9E] },
        }
    }
}

pub(super) fn encode_unary(op: UnaryOp, dst: &Operand, size: Size) -> EncodeInformation {
    let enc = op.encoding(size);

    match enc {
        // ==================================================
        // Simple opcodes (nop, cdq, cqo, clc, stc, cmc...)
        // ==================================================
        UnaryEncoding::Simple { opcode } => EncodeInformation {
            data: opcode.to_vec(),
            ..Default::default()
        },

        // ==================================================
        // ModRM unary ops (inc/dec/not/neg)
        // ==================================================
        UnaryEncoding::ModRm { opcode, modrm_ext } => {
            let mut v = EncodeInformation::new();

            emit_size_prefix(&mut v, size);
            emit_rex(&mut v, size, None, dst);

            v.push(opcode);

            let reg_field = Operand::Reg(Register {
                class: RegisterClass::Gpr,
                index: modrm_ext,
            });

            let modrm = mod_rm_encode(dst, &reg_field);

            v.append(modrm);
            v
        }
    }
}
