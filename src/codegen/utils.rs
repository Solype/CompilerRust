use super::{generation::Context, instructions::convert_operand};
use crate::{
    converter::ir::structs::{Operand as IrOp, VarId},
    elf::instructions::{
        BinOp, Instruction, MemAddress, Operand as ElfOper, Size, register::Register,
    },
};

/// The size of a `mov` that touches `var`: its own size in bytes, so that writing a 4-byte
/// 짧은 정수 does not overflow on its neighbour
pub fn var_size(var: &VarId) -> Option<Size> {
    return Some(Size::from(var.ty.size()));
}

/// `reg = op`: a string is an address, computed by lea, not a value read by mov; an xmm register
/// is loaded by movsd (movss on 4 bytes). A float goes into a general register by mov, bit for bit
pub fn load_operand(reg: Register, op: &IrOp, ctx: &Context, size: Option<Size>) -> Instruction {
    return match op {
        IrOp::DataAddress(id) => Instruction::Lea {
            dst: reg,
            src: ElfOper::MemoryAddress(MemAddress::symbol(&ctx.data_names[*id]).rip_relative()),
            size: None,
        },
        _ => Instruction::Binary {
            // not MovF: movapd reads 16 bytes and wants them 16-byte aligned
            op: if reg.is_xmm() {
                BinOp::LoadF
            } else {
                BinOp::Mov
            },
            dst: ElfOper::Reg(reg),
            src: convert_operand(op, ctx),
            size,
        },
    };
}
