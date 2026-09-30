use crate::elf::instructions::{register::*, *};
use super::helpers::*;

/// cmovcc pour chaque condition, depuis un registre puis depuis la mémoire
pub fn conditional_move() -> Vec<Instruction> {
    let mut code = Vec::new();
    for (i, cc) in ALL_CC.into_iter().enumerate() {
        code.push(cmov(cc, GPRS[i % 4], reg(GPRS[(i + 1) % 4]), DWORD));
    }
    code.extend([
        cmov(ConditionCode::G, RAX, mem(at(RBX)), DWORD),
        cmov(ConditionCode::L, RBX, mem(at(RCX).disp(4)), DWORD),
        ret(),
    ]);
    code
}
