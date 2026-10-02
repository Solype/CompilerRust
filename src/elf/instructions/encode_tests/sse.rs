//! Generated encoding tests: reference bytes produced by GNU as.
//! `#[ignore = "BUG: ..."]` marks a wrong encoding (see `cargo test -- --ignored`).

use super::*;

cases! {
    /// movsd xmm1, xmm0
    movsd_xmm1_xmm0: bin(BinOp::LoadF, reg(XMM1), reg(XMM0), None) => [0xF2, 0x0F, 0x10, 0xC8];
    /// movsd xmm9, xmm2
    movsd_xmm9_xmm2: bin(BinOp::LoadF, reg(XMM9), reg(XMM2), None) => [0xF2, 0x44, 0x0F, 0x10, 0xCA];
    /// movsd xmm2, xmm15
    movsd_xmm2_xmm15: bin(BinOp::LoadF, reg(XMM2), reg(XMM15), None) => [0xF2, 0x41, 0x0F, 0x10, 0xD7];
    /// movsd xmm0, qword ptr [rip+my_data]
    movsd_xmm0_qword_rip_plus_my_data: bin(BinOp::LoadF, reg(XMM0), mem(MemAddress::symbol("my_data")), None) => [0xF2, 0x0F, 0x10, 0x05, 0x00, 0x00, 0x00, 0x00];
    /// movsd xmm3, qword ptr [rbx+8]
    movsd_xmm3_qword_rbx_plus_8: bin(BinOp::LoadF, reg(XMM3), mem(at(RBX).disp(8)), None) => [0xF2, 0x0F, 0x10, 0x5B, 0x08];
    /// movsd xmm3, qword ptr [r9]
    movsd_xmm3_qword_r9: bin(BinOp::LoadF, reg(XMM3), mem(at(R9)), None) => [0xF2, 0x41, 0x0F, 0x10, 0x19];
    /// addsd xmm1, xmm0
    addsd_xmm1_xmm0: bin(BinOp::AddF, reg(XMM1), reg(XMM0), None) => [0xF2, 0x0F, 0x58, 0xC8];
    /// addsd xmm9, xmm2
    addsd_xmm9_xmm2: bin(BinOp::AddF, reg(XMM9), reg(XMM2), None) => [0xF2, 0x44, 0x0F, 0x58, 0xCA];
    /// addsd xmm2, xmm15
    addsd_xmm2_xmm15: bin(BinOp::AddF, reg(XMM2), reg(XMM15), None) => [0xF2, 0x41, 0x0F, 0x58, 0xD7];
    /// addsd xmm0, qword ptr [rip+my_data]
    addsd_xmm0_qword_rip_plus_my_data: bin(BinOp::AddF, reg(XMM0), mem(MemAddress::symbol("my_data")), None) => [0xF2, 0x0F, 0x58, 0x05, 0x00, 0x00, 0x00, 0x00];
    /// addsd xmm3, qword ptr [rbx+8]
    addsd_xmm3_qword_rbx_plus_8: bin(BinOp::AddF, reg(XMM3), mem(at(RBX).disp(8)), None) => [0xF2, 0x0F, 0x58, 0x5B, 0x08];
    /// addsd xmm3, qword ptr [r9]
    addsd_xmm3_qword_r9: bin(BinOp::AddF, reg(XMM3), mem(at(R9)), None) => [0xF2, 0x41, 0x0F, 0x58, 0x19];
    /// subsd xmm1, xmm0
    subsd_xmm1_xmm0: bin(BinOp::SubF, reg(XMM1), reg(XMM0), None) => [0xF2, 0x0F, 0x5C, 0xC8];
    /// subsd xmm9, xmm2
    subsd_xmm9_xmm2: bin(BinOp::SubF, reg(XMM9), reg(XMM2), None) => [0xF2, 0x44, 0x0F, 0x5C, 0xCA];
    /// subsd xmm2, xmm15
    subsd_xmm2_xmm15: bin(BinOp::SubF, reg(XMM2), reg(XMM15), None) => [0xF2, 0x41, 0x0F, 0x5C, 0xD7];
    /// subsd xmm0, qword ptr [rip+my_data]
    subsd_xmm0_qword_rip_plus_my_data: bin(BinOp::SubF, reg(XMM0), mem(MemAddress::symbol("my_data")), None) => [0xF2, 0x0F, 0x5C, 0x05, 0x00, 0x00, 0x00, 0x00];
    /// subsd xmm3, qword ptr [rbx+8]
    subsd_xmm3_qword_rbx_plus_8: bin(BinOp::SubF, reg(XMM3), mem(at(RBX).disp(8)), None) => [0xF2, 0x0F, 0x5C, 0x5B, 0x08];
    /// subsd xmm3, qword ptr [r9]
    subsd_xmm3_qword_r9: bin(BinOp::SubF, reg(XMM3), mem(at(R9)), None) => [0xF2, 0x41, 0x0F, 0x5C, 0x19];
    /// mulsd xmm1, xmm0
    mulsd_xmm1_xmm0: bin(BinOp::MulF, reg(XMM1), reg(XMM0), None) => [0xF2, 0x0F, 0x59, 0xC8];
    /// mulsd xmm9, xmm2
    mulsd_xmm9_xmm2: bin(BinOp::MulF, reg(XMM9), reg(XMM2), None) => [0xF2, 0x44, 0x0F, 0x59, 0xCA];
    /// mulsd xmm2, xmm15
    mulsd_xmm2_xmm15: bin(BinOp::MulF, reg(XMM2), reg(XMM15), None) => [0xF2, 0x41, 0x0F, 0x59, 0xD7];
    /// mulsd xmm0, qword ptr [rip+my_data]
    mulsd_xmm0_qword_rip_plus_my_data: bin(BinOp::MulF, reg(XMM0), mem(MemAddress::symbol("my_data")), None) => [0xF2, 0x0F, 0x59, 0x05, 0x00, 0x00, 0x00, 0x00];
    /// mulsd xmm3, qword ptr [rbx+8]
    mulsd_xmm3_qword_rbx_plus_8: bin(BinOp::MulF, reg(XMM3), mem(at(RBX).disp(8)), None) => [0xF2, 0x0F, 0x59, 0x5B, 0x08];
    /// mulsd xmm3, qword ptr [r9]
    mulsd_xmm3_qword_r9: bin(BinOp::MulF, reg(XMM3), mem(at(R9)), None) => [0xF2, 0x41, 0x0F, 0x59, 0x19];
    /// divsd xmm1, xmm0
    divsd_xmm1_xmm0: bin(BinOp::DivF, reg(XMM1), reg(XMM0), None) => [0xF2, 0x0F, 0x5E, 0xC8];
    /// divsd xmm9, xmm2
    divsd_xmm9_xmm2: bin(BinOp::DivF, reg(XMM9), reg(XMM2), None) => [0xF2, 0x44, 0x0F, 0x5E, 0xCA];
    /// divsd xmm2, xmm15
    divsd_xmm2_xmm15: bin(BinOp::DivF, reg(XMM2), reg(XMM15), None) => [0xF2, 0x41, 0x0F, 0x5E, 0xD7];
    /// divsd xmm0, qword ptr [rip+my_data]
    divsd_xmm0_qword_rip_plus_my_data: bin(BinOp::DivF, reg(XMM0), mem(MemAddress::symbol("my_data")), None) => [0xF2, 0x0F, 0x5E, 0x05, 0x00, 0x00, 0x00, 0x00];
    /// divsd xmm3, qword ptr [rbx+8]
    divsd_xmm3_qword_rbx_plus_8: bin(BinOp::DivF, reg(XMM3), mem(at(RBX).disp(8)), None) => [0xF2, 0x0F, 0x5E, 0x5B, 0x08];
    /// divsd xmm3, qword ptr [r9]
    divsd_xmm3_qword_r9: bin(BinOp::DivF, reg(XMM3), mem(at(R9)), None) => [0xF2, 0x41, 0x0F, 0x5E, 0x19];
    /// comisd xmm1, xmm0
    comisd_xmm1_xmm0: bin(BinOp::ComiF, reg(XMM1), reg(XMM0), None) => [0x66, 0x0F, 0x2F, 0xC8];
    /// comisd xmm9, xmm2
    comisd_xmm9_xmm2: bin(BinOp::ComiF, reg(XMM9), reg(XMM2), None) => [0x66, 0x44, 0x0F, 0x2F, 0xCA];
    /// comisd xmm2, xmm15
    comisd_xmm2_xmm15: bin(BinOp::ComiF, reg(XMM2), reg(XMM15), None) => [0x66, 0x41, 0x0F, 0x2F, 0xD7];
    /// comisd xmm0, qword ptr [rip+my_data]
    comisd_xmm0_qword_rip_plus_my_data: bin(BinOp::ComiF, reg(XMM0), mem(MemAddress::symbol("my_data")), None) => [0x66, 0x0F, 0x2F, 0x05, 0x00, 0x00, 0x00, 0x00];
    /// comisd xmm3, qword ptr [rbx+8]
    comisd_xmm3_qword_rbx_plus_8: bin(BinOp::ComiF, reg(XMM3), mem(at(RBX).disp(8)), None) => [0x66, 0x0F, 0x2F, 0x5B, 0x08];
    /// comisd xmm3, qword ptr [r9]
    comisd_xmm3_qword_r9: bin(BinOp::ComiF, reg(XMM3), mem(at(R9)), None) => [0x66, 0x41, 0x0F, 0x2F, 0x19];
    /// ucomisd xmm1, xmm0
    ucomisd_xmm1_xmm0: bin(BinOp::UcomiF, reg(XMM1), reg(XMM0), None) => [0x66, 0x0F, 0x2E, 0xC8];
    /// ucomisd xmm9, xmm2
    ucomisd_xmm9_xmm2: bin(BinOp::UcomiF, reg(XMM9), reg(XMM2), None) => [0x66, 0x44, 0x0F, 0x2E, 0xCA];
    /// ucomisd xmm2, xmm15
    ucomisd_xmm2_xmm15: bin(BinOp::UcomiF, reg(XMM2), reg(XMM15), None) => [0x66, 0x41, 0x0F, 0x2E, 0xD7];
    /// ucomisd xmm0, qword ptr [rip+my_data]
    ucomisd_xmm0_qword_rip_plus_my_data: bin(BinOp::UcomiF, reg(XMM0), mem(MemAddress::symbol("my_data")), None) => [0x66, 0x0F, 0x2E, 0x05, 0x00, 0x00, 0x00, 0x00];
    /// ucomisd xmm3, qword ptr [rbx+8]
    ucomisd_xmm3_qword_rbx_plus_8: bin(BinOp::UcomiF, reg(XMM3), mem(at(RBX).disp(8)), None) => [0x66, 0x0F, 0x2E, 0x5B, 0x08];
    /// ucomisd xmm3, qword ptr [r9]
    ucomisd_xmm3_qword_r9: bin(BinOp::UcomiF, reg(XMM3), mem(at(R9)), None) => [0x66, 0x41, 0x0F, 0x2E, 0x19];
    /// movsd qword ptr [rip+my_data], xmm0
    movsd_qword_rip_plus_my_data_xmm0: bin(BinOp::StoreF, reg(XMM0), mem(MemAddress::symbol("my_data")), None) => [0xF2, 0x0F, 0x11, 0x05, 0x00, 0x00, 0x00, 0x00];
    /// movsd qword ptr [rbx], xmm10
    movsd_qword_rbx_xmm10: bin(BinOp::StoreF, reg(XMM10), mem(at(RBX)), None) => [0xF2, 0x44, 0x0F, 0x11, 0x13];
    /// movss xmm1, xmm0
    movss_xmm1_xmm0: bin(BinOp::LoadF, reg(XMM1), reg(XMM0), DWORD) => [0xF3, 0x0F, 0x10, 0xC8];
    /// movss xmm9, xmm2
    movss_xmm9_xmm2: bin(BinOp::LoadF, reg(XMM9), reg(XMM2), DWORD) => [0xF3, 0x44, 0x0F, 0x10, 0xCA];
    /// movss xmm2, xmm15
    movss_xmm2_xmm15: bin(BinOp::LoadF, reg(XMM2), reg(XMM15), DWORD) => [0xF3, 0x41, 0x0F, 0x10, 0xD7];
    /// movss xmm0, dword ptr [rip+my_data]
    movss_xmm0_dword_rip_plus_my_data: bin(BinOp::LoadF, reg(XMM0), mem(MemAddress::symbol("my_data")), DWORD) => [0xF3, 0x0F, 0x10, 0x05, 0x00, 0x00, 0x00, 0x00];
    /// movss xmm3, dword ptr [rbx+8]
    movss_xmm3_dword_rbx_plus_8: bin(BinOp::LoadF, reg(XMM3), mem(at(RBX).disp(8)), DWORD) => [0xF3, 0x0F, 0x10, 0x5B, 0x08];
    /// movss xmm3, dword ptr [r9]
    movss_xmm3_dword_r9: bin(BinOp::LoadF, reg(XMM3), mem(at(R9)), DWORD) => [0xF3, 0x41, 0x0F, 0x10, 0x19];
    /// addss xmm1, xmm0
    addss_xmm1_xmm0: bin(BinOp::AddF, reg(XMM1), reg(XMM0), DWORD) => [0xF3, 0x0F, 0x58, 0xC8];
    /// addss xmm9, xmm2
    addss_xmm9_xmm2: bin(BinOp::AddF, reg(XMM9), reg(XMM2), DWORD) => [0xF3, 0x44, 0x0F, 0x58, 0xCA];
    /// addss xmm2, xmm15
    addss_xmm2_xmm15: bin(BinOp::AddF, reg(XMM2), reg(XMM15), DWORD) => [0xF3, 0x41, 0x0F, 0x58, 0xD7];
    /// addss xmm0, dword ptr [rip+my_data]
    addss_xmm0_dword_rip_plus_my_data: bin(BinOp::AddF, reg(XMM0), mem(MemAddress::symbol("my_data")), DWORD) => [0xF3, 0x0F, 0x58, 0x05, 0x00, 0x00, 0x00, 0x00];
    /// addss xmm3, dword ptr [rbx+8]
    addss_xmm3_dword_rbx_plus_8: bin(BinOp::AddF, reg(XMM3), mem(at(RBX).disp(8)), DWORD) => [0xF3, 0x0F, 0x58, 0x5B, 0x08];
    /// addss xmm3, dword ptr [r9]
    addss_xmm3_dword_r9: bin(BinOp::AddF, reg(XMM3), mem(at(R9)), DWORD) => [0xF3, 0x41, 0x0F, 0x58, 0x19];
    /// subss xmm1, xmm0
    subss_xmm1_xmm0: bin(BinOp::SubF, reg(XMM1), reg(XMM0), DWORD) => [0xF3, 0x0F, 0x5C, 0xC8];
    /// subss xmm9, xmm2
    subss_xmm9_xmm2: bin(BinOp::SubF, reg(XMM9), reg(XMM2), DWORD) => [0xF3, 0x44, 0x0F, 0x5C, 0xCA];
    /// subss xmm2, xmm15
    subss_xmm2_xmm15: bin(BinOp::SubF, reg(XMM2), reg(XMM15), DWORD) => [0xF3, 0x41, 0x0F, 0x5C, 0xD7];
    /// subss xmm0, dword ptr [rip+my_data]
    subss_xmm0_dword_rip_plus_my_data: bin(BinOp::SubF, reg(XMM0), mem(MemAddress::symbol("my_data")), DWORD) => [0xF3, 0x0F, 0x5C, 0x05, 0x00, 0x00, 0x00, 0x00];
    /// subss xmm3, dword ptr [rbx+8]
    subss_xmm3_dword_rbx_plus_8: bin(BinOp::SubF, reg(XMM3), mem(at(RBX).disp(8)), DWORD) => [0xF3, 0x0F, 0x5C, 0x5B, 0x08];
    /// subss xmm3, dword ptr [r9]
    subss_xmm3_dword_r9: bin(BinOp::SubF, reg(XMM3), mem(at(R9)), DWORD) => [0xF3, 0x41, 0x0F, 0x5C, 0x19];
    /// mulss xmm1, xmm0
    mulss_xmm1_xmm0: bin(BinOp::MulF, reg(XMM1), reg(XMM0), DWORD) => [0xF3, 0x0F, 0x59, 0xC8];
    /// mulss xmm9, xmm2
    mulss_xmm9_xmm2: bin(BinOp::MulF, reg(XMM9), reg(XMM2), DWORD) => [0xF3, 0x44, 0x0F, 0x59, 0xCA];
    /// mulss xmm2, xmm15
    mulss_xmm2_xmm15: bin(BinOp::MulF, reg(XMM2), reg(XMM15), DWORD) => [0xF3, 0x41, 0x0F, 0x59, 0xD7];
    /// mulss xmm0, dword ptr [rip+my_data]
    mulss_xmm0_dword_rip_plus_my_data: bin(BinOp::MulF, reg(XMM0), mem(MemAddress::symbol("my_data")), DWORD) => [0xF3, 0x0F, 0x59, 0x05, 0x00, 0x00, 0x00, 0x00];
    /// mulss xmm3, dword ptr [rbx+8]
    mulss_xmm3_dword_rbx_plus_8: bin(BinOp::MulF, reg(XMM3), mem(at(RBX).disp(8)), DWORD) => [0xF3, 0x0F, 0x59, 0x5B, 0x08];
    /// mulss xmm3, dword ptr [r9]
    mulss_xmm3_dword_r9: bin(BinOp::MulF, reg(XMM3), mem(at(R9)), DWORD) => [0xF3, 0x41, 0x0F, 0x59, 0x19];
    /// divss xmm1, xmm0
    divss_xmm1_xmm0: bin(BinOp::DivF, reg(XMM1), reg(XMM0), DWORD) => [0xF3, 0x0F, 0x5E, 0xC8];
    /// divss xmm9, xmm2
    divss_xmm9_xmm2: bin(BinOp::DivF, reg(XMM9), reg(XMM2), DWORD) => [0xF3, 0x44, 0x0F, 0x5E, 0xCA];
    /// divss xmm2, xmm15
    divss_xmm2_xmm15: bin(BinOp::DivF, reg(XMM2), reg(XMM15), DWORD) => [0xF3, 0x41, 0x0F, 0x5E, 0xD7];
    /// divss xmm0, dword ptr [rip+my_data]
    divss_xmm0_dword_rip_plus_my_data: bin(BinOp::DivF, reg(XMM0), mem(MemAddress::symbol("my_data")), DWORD) => [0xF3, 0x0F, 0x5E, 0x05, 0x00, 0x00, 0x00, 0x00];
    /// divss xmm3, dword ptr [rbx+8]
    divss_xmm3_dword_rbx_plus_8: bin(BinOp::DivF, reg(XMM3), mem(at(RBX).disp(8)), DWORD) => [0xF3, 0x0F, 0x5E, 0x5B, 0x08];
    /// divss xmm3, dword ptr [r9]
    divss_xmm3_dword_r9: bin(BinOp::DivF, reg(XMM3), mem(at(R9)), DWORD) => [0xF3, 0x41, 0x0F, 0x5E, 0x19];
    /// comiss xmm1, xmm0
    comiss_xmm1_xmm0: bin(BinOp::ComiF, reg(XMM1), reg(XMM0), DWORD) => [0x0F, 0x2F, 0xC8];
    /// comiss xmm9, xmm2
    comiss_xmm9_xmm2: bin(BinOp::ComiF, reg(XMM9), reg(XMM2), DWORD) => [0x44, 0x0F, 0x2F, 0xCA];
    /// comiss xmm2, xmm15
    comiss_xmm2_xmm15: bin(BinOp::ComiF, reg(XMM2), reg(XMM15), DWORD) => [0x41, 0x0F, 0x2F, 0xD7];
    /// comiss xmm0, dword ptr [rip+my_data]
    comiss_xmm0_dword_rip_plus_my_data: bin(BinOp::ComiF, reg(XMM0), mem(MemAddress::symbol("my_data")), DWORD) => [0x0F, 0x2F, 0x05, 0x00, 0x00, 0x00, 0x00];
    /// comiss xmm3, dword ptr [rbx+8]
    comiss_xmm3_dword_rbx_plus_8: bin(BinOp::ComiF, reg(XMM3), mem(at(RBX).disp(8)), DWORD) => [0x0F, 0x2F, 0x5B, 0x08];
    /// comiss xmm3, dword ptr [r9]
    comiss_xmm3_dword_r9: bin(BinOp::ComiF, reg(XMM3), mem(at(R9)), DWORD) => [0x41, 0x0F, 0x2F, 0x19];
    /// ucomiss xmm1, xmm0
    ucomiss_xmm1_xmm0: bin(BinOp::UcomiF, reg(XMM1), reg(XMM0), DWORD) => [0x0F, 0x2E, 0xC8];
    /// ucomiss xmm9, xmm2
    ucomiss_xmm9_xmm2: bin(BinOp::UcomiF, reg(XMM9), reg(XMM2), DWORD) => [0x44, 0x0F, 0x2E, 0xCA];
    /// ucomiss xmm2, xmm15
    ucomiss_xmm2_xmm15: bin(BinOp::UcomiF, reg(XMM2), reg(XMM15), DWORD) => [0x41, 0x0F, 0x2E, 0xD7];
    /// ucomiss xmm0, dword ptr [rip+my_data]
    ucomiss_xmm0_dword_rip_plus_my_data: bin(BinOp::UcomiF, reg(XMM0), mem(MemAddress::symbol("my_data")), DWORD) => [0x0F, 0x2E, 0x05, 0x00, 0x00, 0x00, 0x00];
    /// ucomiss xmm3, dword ptr [rbx+8]
    ucomiss_xmm3_dword_rbx_plus_8: bin(BinOp::UcomiF, reg(XMM3), mem(at(RBX).disp(8)), DWORD) => [0x0F, 0x2E, 0x5B, 0x08];
    /// ucomiss xmm3, dword ptr [r9]
    ucomiss_xmm3_dword_r9: bin(BinOp::UcomiF, reg(XMM3), mem(at(R9)), DWORD) => [0x41, 0x0F, 0x2E, 0x19];
    /// movss dword ptr [rip+my_data], xmm0
    movss_dword_rip_plus_my_data_xmm0: bin(BinOp::StoreF, reg(XMM0), mem(MemAddress::symbol("my_data")), DWORD) => [0xF3, 0x0F, 0x11, 0x05, 0x00, 0x00, 0x00, 0x00];
    /// movss dword ptr [rbx], xmm10
    movss_dword_rbx_xmm10: bin(BinOp::StoreF, reg(XMM10), mem(at(RBX)), DWORD) => [0xF3, 0x44, 0x0F, 0x11, 0x13];
    /// movsd xmm11, qword ptr [rbx+r8*2]
    movsd_xmm11_qword_rbx_plus_r8_x_2: bin(BinOp::LoadF, reg(XMM11), mem(at(RBX).index_scale(R8, Scale::Two)), None) => [0xF2, 0x46, 0x0F, 0x10, 0x1C, 0x43];
    /// addsd xmm11, qword ptr [rbx+r8*2]
    addsd_xmm11_qword_rbx_plus_r8_x_2: bin(BinOp::AddF, reg(XMM11), mem(at(RBX).index_scale(R8, Scale::Two)), None) => [0xF2, 0x46, 0x0F, 0x58, 0x1C, 0x43];
    /// subsd xmm11, qword ptr [rbx+r8*2]
    subsd_xmm11_qword_rbx_plus_r8_x_2: bin(BinOp::SubF, reg(XMM11), mem(at(RBX).index_scale(R8, Scale::Two)), None) => [0xF2, 0x46, 0x0F, 0x5C, 0x1C, 0x43];
    /// mulsd xmm11, qword ptr [rbx+r8*2]
    mulsd_xmm11_qword_rbx_plus_r8_x_2: bin(BinOp::MulF, reg(XMM11), mem(at(RBX).index_scale(R8, Scale::Two)), None) => [0xF2, 0x46, 0x0F, 0x59, 0x1C, 0x43];
    /// divsd xmm11, qword ptr [rbx+r8*2]
    divsd_xmm11_qword_rbx_plus_r8_x_2: bin(BinOp::DivF, reg(XMM11), mem(at(RBX).index_scale(R8, Scale::Two)), None) => [0xF2, 0x46, 0x0F, 0x5E, 0x1C, 0x43];
    /// comisd xmm11, qword ptr [rbx+r8*2]
    comisd_xmm11_qword_rbx_plus_r8_x_2: bin(BinOp::ComiF, reg(XMM11), mem(at(RBX).index_scale(R8, Scale::Two)), None) => [0x66, 0x46, 0x0F, 0x2F, 0x1C, 0x43];
    /// ucomisd xmm11, qword ptr [rbx+r8*2]
    ucomisd_xmm11_qword_rbx_plus_r8_x_2: bin(BinOp::UcomiF, reg(XMM11), mem(at(RBX).index_scale(R8, Scale::Two)), None) => [0x66, 0x46, 0x0F, 0x2E, 0x1C, 0x43];
    /// movsd qword ptr [r9], xmm12
    movsd_qword_r9_xmm12: bin(BinOp::StoreF, reg(XMM12), mem(at(R9)), None) => [0xF2, 0x45, 0x0F, 0x11, 0x21];
    /// xorpd xmm1, xmm0
    xorpd_xmm1_xmm0: bin(BinOp::XorF, reg(XMM1), reg(XMM0), None) => [0x66, 0x0F, 0x57, 0xC8];
    /// xorpd xmm9, xmm2
    xorpd_xmm9_xmm2: bin(BinOp::XorF, reg(XMM9), reg(XMM2), None) => [0x66, 0x44, 0x0F, 0x57, 0xCA];
    /// xorpd xmm2, xmm15
    xorpd_xmm2_xmm15: bin(BinOp::XorF, reg(XMM2), reg(XMM15), None) => [0x66, 0x41, 0x0F, 0x57, 0xD7];
    /// xorpd xmm0, xmm0
    xorpd_xmm0_xmm0: bin(BinOp::XorF, reg(XMM0), reg(XMM0), None) => [0x66, 0x0F, 0x57, 0xC0];
    /// xorpd xmm0, xmmword ptr [rip+my_data]
    xorpd_xmm0_xmmword_rip_plus_my_data: bin(BinOp::XorF, reg(XMM0), mem(MemAddress::symbol("my_data")), None) => [0x66, 0x0F, 0x57, 0x05, 0x00, 0x00, 0x00, 0x00];
    /// xorpd xmm3, xmmword ptr [rbx+8]
    xorpd_xmm3_xmmword_rbx_plus_8: bin(BinOp::XorF, reg(XMM3), mem(at(RBX).disp(8)), None) => [0x66, 0x0F, 0x57, 0x5B, 0x08];
    /// xorpd xmm3, xmmword ptr [r9]
    xorpd_xmm3_xmmword_r9: bin(BinOp::XorF, reg(XMM3), mem(at(R9)), None) => [0x66, 0x41, 0x0F, 0x57, 0x19];
    /// xorps xmm1, xmm0
    xorps_xmm1_xmm0: bin(BinOp::XorF, reg(XMM1), reg(XMM0), DWORD) => [0x0F, 0x57, 0xC8];
    /// xorps xmm9, xmm2
    xorps_xmm9_xmm2: bin(BinOp::XorF, reg(XMM9), reg(XMM2), DWORD) => [0x44, 0x0F, 0x57, 0xCA];
    /// xorps xmm2, xmm15
    xorps_xmm2_xmm15: bin(BinOp::XorF, reg(XMM2), reg(XMM15), DWORD) => [0x41, 0x0F, 0x57, 0xD7];
    /// xorps xmm0, xmm0
    xorps_xmm0_xmm0: bin(BinOp::XorF, reg(XMM0), reg(XMM0), DWORD) => [0x0F, 0x57, 0xC0];
    /// xorps xmm0, xmmword ptr [rip+my_data]
    xorps_xmm0_xmmword_rip_plus_my_data: bin(BinOp::XorF, reg(XMM0), mem(MemAddress::symbol("my_data")), DWORD) => [0x0F, 0x57, 0x05, 0x00, 0x00, 0x00, 0x00];
    /// xorps xmm3, xmmword ptr [rbx+8]
    xorps_xmm3_xmmword_rbx_plus_8: bin(BinOp::XorF, reg(XMM3), mem(at(RBX).disp(8)), DWORD) => [0x0F, 0x57, 0x5B, 0x08];
    /// xorps xmm3, xmmword ptr [r9]
    xorps_xmm3_xmmword_r9: bin(BinOp::XorF, reg(XMM3), mem(at(R9)), DWORD) => [0x41, 0x0F, 0x57, 0x19];
    /// andpd xmm1, xmm0
    andpd_xmm1_xmm0: bin(BinOp::AndF, reg(XMM1), reg(XMM0), None) => [0x66, 0x0F, 0x54, 0xC8];
    /// andpd xmm9, xmm2
    andpd_xmm9_xmm2: bin(BinOp::AndF, reg(XMM9), reg(XMM2), None) => [0x66, 0x44, 0x0F, 0x54, 0xCA];
    /// andpd xmm2, xmm15
    andpd_xmm2_xmm15: bin(BinOp::AndF, reg(XMM2), reg(XMM15), None) => [0x66, 0x41, 0x0F, 0x54, 0xD7];
    /// andpd xmm0, xmm0
    andpd_xmm0_xmm0: bin(BinOp::AndF, reg(XMM0), reg(XMM0), None) => [0x66, 0x0F, 0x54, 0xC0];
    /// andpd xmm0, xmmword ptr [rip+my_data]
    andpd_xmm0_xmmword_rip_plus_my_data: bin(BinOp::AndF, reg(XMM0), mem(MemAddress::symbol("my_data")), None) => [0x66, 0x0F, 0x54, 0x05, 0x00, 0x00, 0x00, 0x00];
    /// andpd xmm3, xmmword ptr [rbx+8]
    andpd_xmm3_xmmword_rbx_plus_8: bin(BinOp::AndF, reg(XMM3), mem(at(RBX).disp(8)), None) => [0x66, 0x0F, 0x54, 0x5B, 0x08];
    /// andpd xmm3, xmmword ptr [r9]
    andpd_xmm3_xmmword_r9: bin(BinOp::AndF, reg(XMM3), mem(at(R9)), None) => [0x66, 0x41, 0x0F, 0x54, 0x19];
    /// andps xmm1, xmm0
    andps_xmm1_xmm0: bin(BinOp::AndF, reg(XMM1), reg(XMM0), DWORD) => [0x0F, 0x54, 0xC8];
    /// andps xmm9, xmm2
    andps_xmm9_xmm2: bin(BinOp::AndF, reg(XMM9), reg(XMM2), DWORD) => [0x44, 0x0F, 0x54, 0xCA];
    /// andps xmm2, xmm15
    andps_xmm2_xmm15: bin(BinOp::AndF, reg(XMM2), reg(XMM15), DWORD) => [0x41, 0x0F, 0x54, 0xD7];
    /// andps xmm0, xmm0
    andps_xmm0_xmm0: bin(BinOp::AndF, reg(XMM0), reg(XMM0), DWORD) => [0x0F, 0x54, 0xC0];
    /// andps xmm0, xmmword ptr [rip+my_data]
    andps_xmm0_xmmword_rip_plus_my_data: bin(BinOp::AndF, reg(XMM0), mem(MemAddress::symbol("my_data")), DWORD) => [0x0F, 0x54, 0x05, 0x00, 0x00, 0x00, 0x00];
    /// andps xmm3, xmmword ptr [rbx+8]
    andps_xmm3_xmmword_rbx_plus_8: bin(BinOp::AndF, reg(XMM3), mem(at(RBX).disp(8)), DWORD) => [0x0F, 0x54, 0x5B, 0x08];
    /// andps xmm3, xmmword ptr [r9]
    andps_xmm3_xmmword_r9: bin(BinOp::AndF, reg(XMM3), mem(at(R9)), DWORD) => [0x41, 0x0F, 0x54, 0x19];
    /// sqrtsd xmm1, xmm0
    sqrtsd_xmm1_xmm0: bin(BinOp::SqrtF, reg(XMM1), reg(XMM0), None) => [0xF2, 0x0F, 0x51, 0xC8];
    /// sqrtsd xmm9, xmm2
    sqrtsd_xmm9_xmm2: bin(BinOp::SqrtF, reg(XMM9), reg(XMM2), None) => [0xF2, 0x44, 0x0F, 0x51, 0xCA];
    /// sqrtsd xmm2, xmm15
    sqrtsd_xmm2_xmm15: bin(BinOp::SqrtF, reg(XMM2), reg(XMM15), None) => [0xF2, 0x41, 0x0F, 0x51, 0xD7];
    /// sqrtsd xmm0, xmm0
    sqrtsd_xmm0_xmm0: bin(BinOp::SqrtF, reg(XMM0), reg(XMM0), None) => [0xF2, 0x0F, 0x51, 0xC0];
    /// sqrtsd xmm0, qword ptr [rip+my_data]
    sqrtsd_xmm0_qword_rip_plus_my_data: bin(BinOp::SqrtF, reg(XMM0), mem(MemAddress::symbol("my_data")), None) => [0xF2, 0x0F, 0x51, 0x05, 0x00, 0x00, 0x00, 0x00];
    /// sqrtsd xmm3, qword ptr [rbx+8]
    sqrtsd_xmm3_qword_rbx_plus_8: bin(BinOp::SqrtF, reg(XMM3), mem(at(RBX).disp(8)), None) => [0xF2, 0x0F, 0x51, 0x5B, 0x08];
    /// sqrtsd xmm3, qword ptr [r9]
    sqrtsd_xmm3_qword_r9: bin(BinOp::SqrtF, reg(XMM3), mem(at(R9)), None) => [0xF2, 0x41, 0x0F, 0x51, 0x19];
    /// sqrtss xmm1, xmm0
    sqrtss_xmm1_xmm0: bin(BinOp::SqrtF, reg(XMM1), reg(XMM0), DWORD) => [0xF3, 0x0F, 0x51, 0xC8];
    /// sqrtss xmm9, xmm2
    sqrtss_xmm9_xmm2: bin(BinOp::SqrtF, reg(XMM9), reg(XMM2), DWORD) => [0xF3, 0x44, 0x0F, 0x51, 0xCA];
    /// sqrtss xmm2, xmm15
    sqrtss_xmm2_xmm15: bin(BinOp::SqrtF, reg(XMM2), reg(XMM15), DWORD) => [0xF3, 0x41, 0x0F, 0x51, 0xD7];
    /// sqrtss xmm0, xmm0
    sqrtss_xmm0_xmm0: bin(BinOp::SqrtF, reg(XMM0), reg(XMM0), DWORD) => [0xF3, 0x0F, 0x51, 0xC0];
    /// sqrtss xmm0, dword ptr [rip+my_data]
    sqrtss_xmm0_dword_rip_plus_my_data: bin(BinOp::SqrtF, reg(XMM0), mem(MemAddress::symbol("my_data")), DWORD) => [0xF3, 0x0F, 0x51, 0x05, 0x00, 0x00, 0x00, 0x00];
    /// sqrtss xmm3, dword ptr [rbx+8]
    sqrtss_xmm3_dword_rbx_plus_8: bin(BinOp::SqrtF, reg(XMM3), mem(at(RBX).disp(8)), DWORD) => [0xF3, 0x0F, 0x51, 0x5B, 0x08];
    /// sqrtss xmm3, dword ptr [r9]
    sqrtss_xmm3_dword_r9: bin(BinOp::SqrtF, reg(XMM3), mem(at(R9)), DWORD) => [0xF3, 0x41, 0x0F, 0x51, 0x19];
    /// minsd xmm1, xmm0
    minsd_xmm1_xmm0: bin(BinOp::MinF, reg(XMM1), reg(XMM0), None) => [0xF2, 0x0F, 0x5D, 0xC8];
    /// minsd xmm9, xmm2
    minsd_xmm9_xmm2: bin(BinOp::MinF, reg(XMM9), reg(XMM2), None) => [0xF2, 0x44, 0x0F, 0x5D, 0xCA];
    /// minsd xmm2, xmm15
    minsd_xmm2_xmm15: bin(BinOp::MinF, reg(XMM2), reg(XMM15), None) => [0xF2, 0x41, 0x0F, 0x5D, 0xD7];
    /// minsd xmm0, xmm0
    minsd_xmm0_xmm0: bin(BinOp::MinF, reg(XMM0), reg(XMM0), None) => [0xF2, 0x0F, 0x5D, 0xC0];
    /// minsd xmm0, qword ptr [rip+my_data]
    minsd_xmm0_qword_rip_plus_my_data: bin(BinOp::MinF, reg(XMM0), mem(MemAddress::symbol("my_data")), None) => [0xF2, 0x0F, 0x5D, 0x05, 0x00, 0x00, 0x00, 0x00];
    /// minsd xmm3, qword ptr [rbx+8]
    minsd_xmm3_qword_rbx_plus_8: bin(BinOp::MinF, reg(XMM3), mem(at(RBX).disp(8)), None) => [0xF2, 0x0F, 0x5D, 0x5B, 0x08];
    /// minsd xmm3, qword ptr [r9]
    minsd_xmm3_qword_r9: bin(BinOp::MinF, reg(XMM3), mem(at(R9)), None) => [0xF2, 0x41, 0x0F, 0x5D, 0x19];
    /// minss xmm1, xmm0
    minss_xmm1_xmm0: bin(BinOp::MinF, reg(XMM1), reg(XMM0), DWORD) => [0xF3, 0x0F, 0x5D, 0xC8];
    /// minss xmm9, xmm2
    minss_xmm9_xmm2: bin(BinOp::MinF, reg(XMM9), reg(XMM2), DWORD) => [0xF3, 0x44, 0x0F, 0x5D, 0xCA];
    /// minss xmm2, xmm15
    minss_xmm2_xmm15: bin(BinOp::MinF, reg(XMM2), reg(XMM15), DWORD) => [0xF3, 0x41, 0x0F, 0x5D, 0xD7];
    /// minss xmm0, xmm0
    minss_xmm0_xmm0: bin(BinOp::MinF, reg(XMM0), reg(XMM0), DWORD) => [0xF3, 0x0F, 0x5D, 0xC0];
    /// minss xmm0, dword ptr [rip+my_data]
    minss_xmm0_dword_rip_plus_my_data: bin(BinOp::MinF, reg(XMM0), mem(MemAddress::symbol("my_data")), DWORD) => [0xF3, 0x0F, 0x5D, 0x05, 0x00, 0x00, 0x00, 0x00];
    /// minss xmm3, dword ptr [rbx+8]
    minss_xmm3_dword_rbx_plus_8: bin(BinOp::MinF, reg(XMM3), mem(at(RBX).disp(8)), DWORD) => [0xF3, 0x0F, 0x5D, 0x5B, 0x08];
    /// minss xmm3, dword ptr [r9]
    minss_xmm3_dword_r9: bin(BinOp::MinF, reg(XMM3), mem(at(R9)), DWORD) => [0xF3, 0x41, 0x0F, 0x5D, 0x19];
    /// maxsd xmm1, xmm0
    maxsd_xmm1_xmm0: bin(BinOp::MaxF, reg(XMM1), reg(XMM0), None) => [0xF2, 0x0F, 0x5F, 0xC8];
    /// maxsd xmm9, xmm2
    maxsd_xmm9_xmm2: bin(BinOp::MaxF, reg(XMM9), reg(XMM2), None) => [0xF2, 0x44, 0x0F, 0x5F, 0xCA];
    /// maxsd xmm2, xmm15
    maxsd_xmm2_xmm15: bin(BinOp::MaxF, reg(XMM2), reg(XMM15), None) => [0xF2, 0x41, 0x0F, 0x5F, 0xD7];
    /// maxsd xmm0, xmm0
    maxsd_xmm0_xmm0: bin(BinOp::MaxF, reg(XMM0), reg(XMM0), None) => [0xF2, 0x0F, 0x5F, 0xC0];
    /// maxsd xmm0, qword ptr [rip+my_data]
    maxsd_xmm0_qword_rip_plus_my_data: bin(BinOp::MaxF, reg(XMM0), mem(MemAddress::symbol("my_data")), None) => [0xF2, 0x0F, 0x5F, 0x05, 0x00, 0x00, 0x00, 0x00];
    /// maxsd xmm3, qword ptr [rbx+8]
    maxsd_xmm3_qword_rbx_plus_8: bin(BinOp::MaxF, reg(XMM3), mem(at(RBX).disp(8)), None) => [0xF2, 0x0F, 0x5F, 0x5B, 0x08];
    /// maxsd xmm3, qword ptr [r9]
    maxsd_xmm3_qword_r9: bin(BinOp::MaxF, reg(XMM3), mem(at(R9)), None) => [0xF2, 0x41, 0x0F, 0x5F, 0x19];
    /// maxss xmm1, xmm0
    maxss_xmm1_xmm0: bin(BinOp::MaxF, reg(XMM1), reg(XMM0), DWORD) => [0xF3, 0x0F, 0x5F, 0xC8];
    /// maxss xmm9, xmm2
    maxss_xmm9_xmm2: bin(BinOp::MaxF, reg(XMM9), reg(XMM2), DWORD) => [0xF3, 0x44, 0x0F, 0x5F, 0xCA];
    /// maxss xmm2, xmm15
    maxss_xmm2_xmm15: bin(BinOp::MaxF, reg(XMM2), reg(XMM15), DWORD) => [0xF3, 0x41, 0x0F, 0x5F, 0xD7];
    /// maxss xmm0, xmm0
    maxss_xmm0_xmm0: bin(BinOp::MaxF, reg(XMM0), reg(XMM0), DWORD) => [0xF3, 0x0F, 0x5F, 0xC0];
    /// maxss xmm0, dword ptr [rip+my_data]
    maxss_xmm0_dword_rip_plus_my_data: bin(BinOp::MaxF, reg(XMM0), mem(MemAddress::symbol("my_data")), DWORD) => [0xF3, 0x0F, 0x5F, 0x05, 0x00, 0x00, 0x00, 0x00];
    /// maxss xmm3, dword ptr [rbx+8]
    maxss_xmm3_dword_rbx_plus_8: bin(BinOp::MaxF, reg(XMM3), mem(at(RBX).disp(8)), DWORD) => [0xF3, 0x0F, 0x5F, 0x5B, 0x08];
    /// maxss xmm3, dword ptr [r9]
    maxss_xmm3_dword_r9: bin(BinOp::MaxF, reg(XMM3), mem(at(R9)), DWORD) => [0xF3, 0x41, 0x0F, 0x5F, 0x19];
}
