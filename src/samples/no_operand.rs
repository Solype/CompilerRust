use crate::elf::instructions::*;
use super::helpers::*;

/// Instructions without explicit operand: nop, flags, sign extensions, ...
pub fn no_operand() -> Vec<Instruction> {
    let mut code: Vec<Instruction> = (1..=9).map(Instruction::Nop).collect();
    code.extend([
        nullary(UnaryOp::Nop),
        // Extensions de signe de rax / rdx:rax
        nullary(UnaryOp::Cbw),
        nullary(UnaryOp::Cwde),
        nullary(UnaryOp::Cdqe),
        unary(UnaryOp::Cwd, Operand::NoOperand, WORD),
        unary(UnaryOp::Cdq, Operand::NoOperand, DWORD),
        nullary(UnaryOp::Cqo),
        // Flags
        nullary(UnaryOp::Clc),
        nullary(UnaryOp::Stc),
        nullary(UnaryOp::Cmc),
        nullary(UnaryOp::Cld),
        nullary(UnaryOp::Std),
        nullary(UnaryOp::Cli),
        nullary(UnaryOp::Sti),
        nullary(UnaryOp::Lahf),
        nullary(UnaryOp::Sahf),
        // Divers
        nullary(UnaryOp::Pause),
        nullary(UnaryOp::Fwait),
        nullary(UnaryOp::Ud2),
        nullary(UnaryOp::Hlt),
        ret(),
    ]);
    code
}
