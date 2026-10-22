use super::generation::Context;
use crate::codegen::utils::var_size;
use crate::elf::instructions::Operand::Sym;
use crate::elf::instructions::register::{R8, R9, RBP, RCX, RDI, RDX, RSI, RSP, Register};
use crate::elf::instructions::{BinOp, CtrlOp, Instruction, register::RAX};

use crate::converter::ir::structs::{Operand as IrOp, VarId};
use crate::elf::instructions::{MemAddress, Operand as ElfOper, StackOp};

/// `ctx.offsets`: the offset under rbp of each variable, see var_offsets
pub fn convert_operand(op: &IrOp, ctx: &Context) -> ElfOper {
    return match op {
        IrOp::Var(var) => {
            ElfOper::MemoryAddress(MemAddress::new().base(RBP).disp(-ctx.offsets[var.0]))
        }
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

pub fn generate_return(op: &IrOp, ctx: &Context) -> Vec<Instruction> {
    let mut epi = func_epilogue();
    epi.insert(
        0,
        Instruction::Binary {
            op: BinOp::Mov,
            dst: ElfOper::Reg(RAX),
            src: convert_operand(op, ctx),
            size: None,
        },
    );
    return epi;
}

pub fn float_parameter(_op: &IrOp, _i_f: &mut i32) -> Instruction {
    unimplemented!();
}

/// Registers of the integer arguments, in order (System V)
pub const PARAM_REGS: [Register; 6] = [RDI, RSI, RDX, RCX, R8, R9];

/// `mov` of the argument into the register of its rank `i`, then the next rank
pub fn normal_param(op: &IrOp, i: &mut i32, ctx: &Context) -> Instruction {
    // shortcut: no 7th integer argument (it goes on the stack), add it for calls of more than 6
    let Some(reg) = PARAM_REGS.get(*i as usize) else {
        unimplemented!()
    };
    *i += 1;
    return Instruction::Binary {
        op: BinOp::Mov,
        dst: ElfOper::Reg(*reg),
        src: convert_operand(op, ctx),
        size: None,
    };
}

pub fn generate_copy(dest: &VarId, src: &IrOp, ctx: &Context) -> Vec<Instruction> {
    let ope = convert_operand(src, ctx);
    let var = convert_operand(&IrOp::Var(*dest), ctx);
    // x86 has no memory to memory mov: the value goes through rax
    return vec![
        Instruction::Binary {
            op: BinOp::Mov,
            dst: ElfOper::Reg(RAX),
            src: ope,
            size: var_size(dest),
        },
        Instruction::Binary {
            op: BinOp::Mov,
            dst: var,
            src: ElfOper::Reg(RAX),
            size: var_size(dest),
        },
    ];
}

pub fn generate_call(
    dest: &Option<VarId>,
    symbol: &str,
    args: &[IrOp],
    ctx: &Context,
) -> Vec<Instruction> {
    let mut instr: Vec<Instruction> = vec![];
    let mut i = 0;
    let mut i_f = 0;

    for arg in args {
        let new_instr = match arg {
            IrOp::Float(_) => float_parameter(arg, &mut i_f),
            _ => normal_param(arg, &mut i, ctx),
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
    // the result comes back in rax
    if let Some(var) = dest {
        instr.push(Instruction::Binary {
            op: BinOp::Mov,
            dst: convert_operand(&IrOp::Var(*var), ctx),
            src: ElfOper::Reg(RAX),
            size: var_size(var),
        });
    }
    return instr;
}
