use super::helpers::*;
use crate::elf::instructions::{register::*, *};

/// push, pop, pushf, popf, enter, leave
pub fn stack_ops() -> Vec<Instruction> {
    let mut code = vec![
        stack(StackOp::Push, reg(RAX), DWORD),
        stack(StackOp::Push, reg(RBX), DWORD),
        stack(StackOp::Push, imm(5), DWORD),
        stack(StackOp::Push, imm(0x12345678), DWORD),
        // stack(StackOp::Push, sym("my_data"), DWORD),
        stack(StackOp::Push, mem(at(RCX).disp(16)), DWORD),
        stack(StackOp::Pop, reg(RCX), DWORD),
        stack(StackOp::Pop, reg(RDX), DWORD),
        stack(StackOp::Pop, mem(at(RAX).disp(20)), DWORD),
    ];
    for size in [WORD, DWORD, None] {
        code.push(stack(StackOp::Pushf, Operand::NoOperand, size));
        code.push(stack(StackOp::Popf, Operand::NoOperand, size));
    }
    code.extend([
        stack(StackOp::Enter(0), imm(32), None),
        stack(StackOp::Leave, Operand::NoOperand, None),
        ret(),
    ]);
    return code;
}
