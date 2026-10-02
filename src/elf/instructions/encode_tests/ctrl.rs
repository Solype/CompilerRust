//! Generated encoding tests: reference bytes produced by GNU as.
//! `#[ignore = "BUG: ..."]` marks a wrong encoding (see `cargo test -- --ignored`).

use super::*;

cases! {
    /// je target
    je_target: jcc(ConditionCode::E, "target") => [0x0F, 0x84, 0x00, 0x00, 0x00, 0x00];
    /// jne target
    jne_target: jcc(ConditionCode::NE, "target") => [0x0F, 0x85, 0x00, 0x00, 0x00, 0x00];
    /// jg target
    jg_target: jcc(ConditionCode::G, "target") => [0x0F, 0x8F, 0x00, 0x00, 0x00, 0x00];
    /// jge target
    jge_target: jcc(ConditionCode::GE, "target") => [0x0F, 0x8D, 0x00, 0x00, 0x00, 0x00];
    /// jl target
    jl_target: jcc(ConditionCode::L, "target") => [0x0F, 0x8C, 0x00, 0x00, 0x00, 0x00];
    /// jle target
    jle_target: jcc(ConditionCode::LE, "target") => [0x0F, 0x8E, 0x00, 0x00, 0x00, 0x00];
    /// ja target
    ja_target: jcc(ConditionCode::A, "target") => [0x0F, 0x87, 0x00, 0x00, 0x00, 0x00];
    /// jae target
    jae_target: jcc(ConditionCode::AE, "target") => [0x0F, 0x83, 0x00, 0x00, 0x00, 0x00];
    /// jb target
    jb_target: jcc(ConditionCode::B, "target") => [0x0F, 0x82, 0x00, 0x00, 0x00, 0x00];
    /// jbe target
    jbe_target: jcc(ConditionCode::BE, "target") => [0x0F, 0x86, 0x00, 0x00, 0x00, 0x00];
    /// js target
    js_target: jcc(ConditionCode::S, "target") => [0x0F, 0x88, 0x00, 0x00, 0x00, 0x00];
    /// jns target
    jns_target: jcc(ConditionCode::NS, "target") => [0x0F, 0x89, 0x00, 0x00, 0x00, 0x00];
    /// jo target
    jo_target: jcc(ConditionCode::O, "target") => [0x0F, 0x80, 0x00, 0x00, 0x00, 0x00];
    /// jno target
    jno_target: jcc(ConditionCode::NO, "target") => [0x0F, 0x81, 0x00, 0x00, 0x00, 0x00];
    /// jp target
    jp_target: jcc(ConditionCode::P, "target") => [0x0F, 0x8A, 0x00, 0x00, 0x00, 0x00];
    /// jnp target
    jnp_target: jcc(ConditionCode::NP, "target") => [0x0F, 0x8B, 0x00, 0x00, 0x00, 0x00];
    /// jmp target
    jmp_target: jmp("target") => [0xE9, 0x00, 0x00, 0x00, 0x00];
    /// call target
    call_target: call("target") => [0xE8, 0x00, 0x00, 0x00, 0x00];
    /// ret
    ret_insn: ret() => [0xC3];
    /// iretq
    iretq_insn: iret() => [0x48, 0xCF];
    /// loop target
    loop_target: ctrl(CtrlOp::Loop, "target") => [0xE2, 0x00];
    /// loope target
    loope_target: ctrl(CtrlOp::Loope, "target") => [0xE1, 0x00];
    /// loopne target
    loopne_target: ctrl(CtrlOp::Loopne, "target") => [0xE0, 0x00];
    /// call rax
    call_rax: call_indirect(reg(RAX)) => [0xFF, 0xD0];
    /// call r11
    call_r11: call_indirect(reg(R11)) => [0x41, 0xFF, 0xD3];
    /// jmp rax
    jmp_rax: jmp_indirect(reg(RAX)) => [0xFF, 0xE0];
    /// jmp r12
    jmp_r12: jmp_indirect(reg(R12)) => [0x41, 0xFF, 0xE4];
    /// call qword ptr [rax]
    call_qword_rax: call_indirect(mem(at(RAX))) => [0xFF, 0x10];
    /// call qword ptr [rax+8]
    call_qword_rax_plus_8: call_indirect(mem(at(RAX).disp(8))) => [0xFF, 0x50, 0x08];
    /// call qword ptr [r12]
    call_qword_r12: call_indirect(mem(at(R12))) => [0x41, 0xFF, 0x14, 0x24];
    /// jmp qword ptr [rbx+rcx*8]
    jmp_qword_rbx_plus_rcx_x_8: jmp_indirect(mem(at(RBX).index_scale(RCX, Scale::Eight))) => [0xFF, 0x24, 0xCB];
    /// jmp qword ptr [r9+r10*8]
    jmp_qword_r9_plus_r10_x_8: jmp_indirect(mem(at(R9).index_scale(R10, Scale::Eight))) => [0x43, 0xFF, 0x24, 0xD1];
    /// call qword ptr [rip+my_data]
    call_qword_rip_plus_my_data: call_indirect(var("my_data")) => [0xFF, 0x15, 0x00, 0x00, 0x00, 0x00];
    /// jmp qword ptr [rip+my_data]
    jmp_qword_rip_plus_my_data: jmp_indirect(var("my_data")) => [0xFF, 0x25, 0x00, 0x00, 0x00, 0x00];
}
