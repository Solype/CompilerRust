use crate::elf::instructions::{register::*, *};
use super::helpers::*;

/// jmp, call (directs et indirects), jcc, loop*, ret, iret
pub fn jumps_and_calls() -> Vec<Instruction> {
    let mut code = vec![
        jmp("jump_target"),           // label local, en avant
        call("jumps_and_calls"),      // symbole global
        jmp("jumps_and_calls"),
        call("my_exit"),              // symbole externe, défini dans test.asm
        call_indirect(reg(R11)),      // pointeur de fonction
        call_indirect(mem(at(RAX).disp(8))),
        call_indirect(var("my_data")),
        jmp_indirect(mem(at(RBX).index_scale(RCX, Scale::Eight))), // table de switch
    ];
    for cc in ALL_CC {
        code.push(jcc(cc, "jumps_and_calls"));
    }
    code.extend([
        label("loop_target"),
        ctrl(CtrlOp::Loop,   "loop_target"),
        ctrl(CtrlOp::Loope,  "loop_target"),
        ctrl(CtrlOp::Loopne, "loop_target"),

        label("jump_target"),
        ret(),
        iret(),
    ]);
    code
}
