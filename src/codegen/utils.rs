use super::{generation::Context, instructions::convert_operand};
use crate::{
    converter::ir::structs::{Operand as IrOp, VarId},
    elf::instructions::{
        BinOp, ExtendOp, Instruction, MemAddress, Operand as ElfOper, Size, register::Register,
    },
};

/// The size of a `mov` that touches `var`: its own size in bytes, so that writing a 4-byte
/// 짧은 정수 does not overflow on its neighbour
pub fn var_size(var: &VarId) -> Option<Size> {
    Some(Size::from(var.ty.size()))
}

/// `reg = op`: a string is an address, computed by lea, not a value read by mov; an xmm register
/// is loaded by movsd (movss on 4 bytes). A float goes into a general register by mov, bit for bit
pub fn load_operand(reg: Register, op: &IrOp, ctx: &Context, size: Option<Size>) -> Instruction {
    let real_size = size.or(op.size());

    if let IrOp::DataAddress(id) = op {
        return Instruction::Lea {
            dst: reg,
            src: ElfOper::MemoryAddress(MemAddress::symbol(&ctx.data_names[*id]).rip_relative()),
            size: None,
        };
    }

    let src = convert_operand(op, ctx);
    let one_byte = matches!(real_size, Some(Size::U8));
    let is_constant = matches!(src, ElfOper::Imm(_));

    // 1 byte from memory: movzx, since `mov sil` would keep the 56 bits above
    if !reg.is_xmm() && one_byte && !is_constant {
        return Instruction::Extend {
            op: ExtendOp::Movzx,
            dst: reg,
            src,
            src_size: Some(Size::U8),
            size: Some(Size::U64),
        };
    }

    // 1-byte constant: mov on 32 bits, which clears the upper half
    let size = if !reg.is_xmm() && one_byte {
        Some(Size::U32)
    } else {
        real_size
    };

    Instruction::Binary {
        op: if reg.is_xmm() {
            BinOp::LoadF
        } else {
            BinOp::Mov
        },
        dst: ElfOper::Reg(reg),
        src,
        size,
    }
}
