use crate::elf::instructions::{register::*, *};
use super::helpers::*;

/// setcc pour chaque condition, vers un registre puis vers `[rbx + i]`
pub fn conditional_set() -> Vec<Instruction> {
    let mut code = vec![bin(BinOp::Cmp, reg(RAX), reg(RBX), DWORD)];
    for (i, cc) in ALL_CC.into_iter().enumerate() {
        code.push(setcc(cc, reg(GPRS[i % 4])));
    }
    for (i, cc) in ALL_CC.into_iter().enumerate() {
        code.push(setcc(cc, mem(at(RBX).disp(i as i32))));
    }
    code.push(ret());
    code
}
