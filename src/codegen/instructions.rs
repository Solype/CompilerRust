use crate::elf::instructions::Operand::Sym;
use crate::elf::instructions::register::{R8, R9, RBP, RCX, RDI, RDX, RSI, RSP, Register};
use crate::elf::instructions::{BinOp, CtrlOp, Instruction, register::RAX};

use crate::converter::ir::structs::{Operand as IrOp, VarId};
use crate::elf::instructions::{Operand as ElfOper, StackOp};

fn convert_operand(op: &IrOp) -> ElfOper {
    return match op {
        IrOp::Int(n) => ElfOper::Imm(*n),
        IrOp::Bool(b) => ElfOper::Imm(*b as i64),
        IrOp::Null => ElfOper::Imm(0),
        _ => unimplemented!(),
    };
}

pub fn func_epilogue() -> Vec<Instruction> {
    vec![
        Instruction::Binary {
            op: BinOp::Mov,
            dst: ElfOper::Reg(RSP),
            src: ElfOper::Reg(RBP),
            size: None,
        },
        Instruction::Stack {
            op: StackOp::Pop,
            value: ElfOper::Reg(RBP),
            size: None,
        },
        Instruction::Ctrl {
            op: CtrlOp::Ret,
            target: ElfOper::NoOperand,
        },
    ]
}

pub fn generate_return(op: &IrOp) -> Vec<Instruction> {
    let mut epi = func_epilogue();
    epi.insert(
        0,
        Instruction::Binary {
            op: BinOp::Mov,
            dst: ElfOper::Reg(RAX),
            src: convert_operand(op),
            size: None,
        },
    );
    return epi;
}

pub fn float_parameter(_op: &IrOp, _i_f: &mut i32) -> Instruction {
    unimplemented!();
}

/// Registers of the integer arguments, in order (System V)
const PARAM_REGS: [Register; 6] = [RDI, RSI, RDX, RCX, R8, R9];

/// `mov` of the argument into the register of its rank `i`, then the next rank
pub fn normal_param(op: &IrOp, i: &mut i32) -> Instruction {
    // shortcut: no 7th integer argument (it goes on the stack), add it for calls of more than 6
    let Some(reg) = PARAM_REGS.get(*i as usize) else {
        unimplemented!()
    };
    *i += 1;
    return Instruction::Binary {
        op: BinOp::Mov,
        dst: ElfOper::Reg(*reg),
        src: convert_operand(op),
        size: None,
    };
}

pub fn generate_call(dest: &Option<VarId>, symbol: &str, args: &[IrOp]) -> Vec<Instruction> {
    if dest.is_some() {
        unimplemented!();
    }
    let mut instr: Vec<Instruction> = vec![];
    let mut i = 0;
    let mut i_f = 0;

    for arg in args {
        let new_instr = match arg {
            IrOp::Float(_) => float_parameter(arg, &mut i_f),
            _ => normal_param(arg, &mut i),
        };
        instr.push(new_instr);
    }
    instr.push(Instruction::Binary {
        op: BinOp::Mov,
        dst: ElfOper::Reg(RAX),
        src: ElfOper::Imm(i64::from(i_f)),
        size: None,
    });
    instr.push(Instruction::Ctrl {
        op: CtrlOp::Call,
        target: Sym(symbol.to_string()),
    });
    return instr;
}
