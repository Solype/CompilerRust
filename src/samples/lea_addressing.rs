use super::helpers::*;
use crate::elf::instructions::{register::*, *};

/// lea on the various addressing modes
pub fn lea_addressing() -> Vec<Instruction> {
    return vec![
        lea(RAX, at(RBX), DWORD),
        lea(RAX, at(RBX).disp(8), DWORD),
        lea(RAX, at(RBP).disp(-16), DWORD),
        lea(RAX, at(RCX).index_scale(RCX, Scale::Four), DWORD),
        lea(RAX, MemAddress::symbol("my_data"), None),
        ret(),
    ];
}
