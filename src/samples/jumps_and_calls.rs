use crate::elf::instructions::*;
use super::helpers::*;

/// jmp, call, jcc, loop*, ret, iret
pub fn jumps_and_calls() -> Vec<Instruction> {
    let mut code = vec![
        jmp("jump_target"),           // label local, en avant
        call("jumps_and_calls"),      // symbole global
        jmp("jumps_and_calls"),
        call("my_exit"),              // symbole externe, défini dans test.asm
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
