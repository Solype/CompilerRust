use super::modrm::*;
use super::enums::*;
use super::encode_alu::emit_size_prefix;

#[derive(Debug, Clone, Copy)]
enum UnaryEncoding {
    /// opcode simple sans ModRM
    Simple {
        opcode: &'static [u8],
    },

    /// opcode + ModRM /digit
    ModRm {
        opcode: u8,
        modrm_ext: u8,
    },
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
            UnaryOp::Nop => UnaryEncoding::Simple {
                opcode: &[0x90],
            },

            // ==========================================
            // sign extension accumulator -> high regs
            // ==========================================
            UnaryOp::Cwd => UnaryEncoding::Simple {
                opcode: &[0x99], // AX -> DX:AX (16-bit)
            },

            UnaryOp::Cdq => UnaryEncoding::Simple {
                opcode: &[0x99], // EAX -> EDX:EAX (32-bit)
            },

            UnaryOp::Cqo => UnaryEncoding::Simple {
                opcode: &[0x48, 0x99], // REX.W + CQO
            },

            // ==========================================
            // carry flag ops
            // ==========================================
            UnaryOp::Clc => UnaryEncoding::Simple {
                opcode: &[0xF8],
            },

            UnaryOp::Stc => UnaryEncoding::Simple {
                opcode: &[0xF9],
            },

            UnaryOp::Cmc => UnaryEncoding::Simple {
                opcode: &[0xF5],
            },
            _ => unreachable!()
        }
    }
}

fn encode_cli_sli(op: UnaryOp) -> EncodeInformation
{
    let opcode : u8 = match op {
        UnaryOp::Cli => 0xFA,
        UnaryOp::Sti => 0xFB,
        _ => unreachable!()
    };
    EncodeInformation { data: vec![opcode], ..Default::default() }
}

pub(super) fn encode_unary(
    op: UnaryOp,
    dst: &Operand,
    size: Size,
) -> EncodeInformation {

    if op == UnaryOp::Cli || op == UnaryOp::Sti {
        return encode_cli_sli(op);
    }

    let enc = op.encoding(size);

    match enc {
        // ==================================================
        // Simple opcodes (nop, cdq, cqo, clc, stc, cmc...)
        // ==================================================
        UnaryEncoding::Simple { opcode } => {
            let mut v = Vec::new();
            emit_size_prefix(&mut v, size);   // <-- important
            v.extend_from_slice(opcode);

            EncodeInformation {
                data: v,
                ..Default::default()
            }
        }

        // ==================================================
        // ModRM unary ops (inc/dec/not/neg)
        // ==================================================
        UnaryEncoding::ModRm { opcode, modrm_ext } => {
            let rm_u8 = match dst {
                Operand::Reg(r) => *r as u8,
                _ => 0,
            };

            let mut v = Vec::new();

            emit_size_prefix(&mut v, size);
            emit_rex(&mut v, size, None, Some(rm_u8));

            v.push(opcode);

            let reg_field =
                Operand::Reg(Register::try_from(modrm_ext).unwrap());

            let base = v.len();
            let modrm = mod_rm_encode(dst, &reg_field);

            v.extend(modrm.data);

            let relocations = modrm
                .relocations
                .into_iter()
                .map(|mut r| {
                    r.offset += base;
                    r
                })
                .collect();

            EncodeInformation {
                data: v,
                relocations,
            }
        }
    }
}
