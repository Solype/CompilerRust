use super::super::{
    enums::*,
    modrm::mod_rm_encode,
    struct_encode_information::*,
    utils::{emit_rex, emit_size_prefix},
};
use super::structs::*;

pub(super) fn encode_reg_mem(
    dst: &Operand,
    src: &Operand,
    size: Size,
    enc: &BinaryEncoding,
) -> EncodeInformation {
    let (is_reg_dst, reg_op, rm_op) = match (dst, src) {
        (Operand::Reg(_), _) => (true, dst, src),
        (_, Operand::Reg(_)) => (false, src, dst),
        _ => unreachable!(),
    };

    let reg = match reg_op {
        Operand::Reg(r) => r,
        _ => unreachable!(),
    };

    let mut v = EncodeInformation::new();

    match *enc {
        // --------------------------------------------------
        // Integer MOV / ALU
        // --------------------------------------------------
        BinaryEncoding::Mov {
            opcode_rm_r,
            opcode_r_rm,
            ..
        }
        | BinaryEncoding::Alu {
            opcode_rm_r,
            opcode_r_rm,
            ..
        } => {
            emit_size_prefix(&mut v, size);
            emit_rex(&mut v, size, Some(*reg), rm_op);

            let opcode = if is_reg_dst { opcode_r_rm } else { opcode_rm_r };

            v.push(opcode);
        }

        // --------------------------------------------------
        // XCHG
        // --------------------------------------------------
        BinaryEncoding::Xchg { opcode } => {
            emit_size_prefix(&mut v, size);
            emit_rex(&mut v, size, Some(*reg), rm_op);

            v.push(opcode);
        }

        // --------------------------------------------------
        // MOVZX / MOVSX
        // --------------------------------------------------
        BinaryEncoding::MovExtend { prefix, opcode } => {
            // `size` est celle de la source (byte ou word, déjà portée par
            // l'opcode B6/B7/BE/BF) : la destination est toujours 32 bits,
            // donc ni préfixe 66 ni REX.W
            emit_rex(&mut v, size, Some(*reg), rm_op);

            v.push(prefix);
            v.push(opcode);
        }

        // --------------------------------------------------
        // SSE
        // --------------------------------------------------
        BinaryEncoding::Sse { prefix, opcode } => {
            // mandatory SSE prefix
            if prefix != 0 {
                v.push(prefix);
            }

            // IMPORTANT:
            // SSE scalar instructions DO NOT use REX.W
            //
            // We only emit extension bits.
            //
            emit_rex(&mut v, Size::U8, Some(*reg), rm_op);

            // SSE escape opcode
            v.push(0x0F);

            // actual opcode
            v.push(opcode);
        }
    }

    let modrm = mod_rm_encode(rm_op, reg_op);

    v.append(modrm);

    v
}
