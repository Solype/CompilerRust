use crate::elf::instructions::{BinOp, CtrlOp, Instruction, register::RAX};

use crate::converter::ir::structs::Operand as IrOp;
use crate::elf::instructions::Operand as ElfOper;

fn convert_operand(op: &IrOp) -> ElfOper {
    return match op {
        IrOp::Int(n) => ElfOper::Imm(*n),
        IrOp::Bool(b) => ElfOper::Imm(*b as i64),
        IrOp::Null => ElfOper::Imm(0),
        _ => unimplemented!(),
    };
}

pub fn generate_return(op: &IrOp) -> Vec<Instruction> {
    vec![
        Instruction::Binary {
            op: BinOp::Mov,
            dst: ElfOper::Reg(RAX),
            src: convert_operand(op),
            size: None,
        },
        Instruction::Ctrl {
            op: CtrlOp::Ret,
            target: ElfOper::NoOperand,
        },
    ]
}
