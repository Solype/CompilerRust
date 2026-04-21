use crate::elf::instructions::{EncodeInformation, Operand, Register, ShiftOp, Size, encode_alu::emit_size_prefix, modrm::{emit_rex, mod_rm_encode}};


#[derive(Debug)]
struct ShiftRotate {
    opcode_1: u8,   // shift by 1
    opcode_cl: u8,  // shift by CL
    opcode_imm: u8, // shift by imm8
    modrm_ext: u8,
}

impl ShiftOp {
    fn encode(&self) -> ShiftRotate {
        match self {
            ShiftOp::Rol => ShiftRotate {
                opcode_1: 0xD0,
                opcode_cl: 0xD2,
                opcode_imm: 0xC0,
                modrm_ext: 0,
            },

            ShiftOp::Ror => ShiftRotate {
                opcode_1: 0xD0,
                opcode_cl: 0xD2,
                opcode_imm: 0xC0,
                modrm_ext: 1,
            },

            ShiftOp::Shl => ShiftRotate {
                opcode_1: 0xD0,
                opcode_cl: 0xD2,
                opcode_imm: 0xC0,
                modrm_ext: 4,
            },

            ShiftOp::Shr => ShiftRotate {
                opcode_1: 0xD0,
                opcode_cl: 0xD2,
                opcode_imm: 0xC0,
                modrm_ext: 5,
            },

            ShiftOp::Sar => ShiftRotate {
                opcode_1: 0xD0,
                opcode_cl: 0xD2,
                opcode_imm: 0xC0,
                modrm_ext: 7,
            },
        }
    }
}

pub fn encode_shift_rotate( op: &ShiftOp, dst: &Operand, src: &Operand, size: Size, ) -> EncodeInformation
{

    let (opcode_1, opcode_cl, opcode_imm, ext) = match op.encode() {
        ShiftRotate {
            opcode_1,
            opcode_cl,
            opcode_imm,
            modrm_ext,
        } => (opcode_1, opcode_cl, opcode_imm, modrm_ext),
    };

    let mut v = Vec::new();

    emit_size_prefix(&mut v, size);

    emit_rex(&mut v, size, None, None);
    match src {
        Operand::Imm(1) => v.push(opcode_1),
        Operand::Imm(_) => v.push(opcode_imm),
        Operand::Reg(Register::Ecx) => v.push(opcode_cl),
        _ => unimplemented!("invalid shift count"),
    }

    let reg_field = Operand::Reg(Register::try_from(ext).unwrap());

    let modrm = mod_rm_encode(dst, &reg_field);

    let base = v.len();
    v.extend(modrm.data);

    if let Operand::Imm(n) = src {
        if *n != 1 {
            v.push(*n as u8);
        }
    }

    let relocations = modrm.relocations.into_iter().map(|mut r| { r.offset += base; r }).collect();

    EncodeInformation { data: v, relocations, }
}
