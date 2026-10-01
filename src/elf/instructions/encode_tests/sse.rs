//! Tests d'encodage générés : octets de référence produits par GNU as.
//! `#[ignore = "BUG: ..."]` marque un encodage faux (voir `cargo test -- --ignored`).

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
    #[ignore = "BUG: produit `movsd xmm3,QWORD PTR [rcx]`"]
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
    #[ignore = "BUG: produit `addsd xmm3,QWORD PTR [rcx]`"]
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
    #[ignore = "BUG: produit `subsd xmm3,QWORD PTR [rcx]`"]
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
    #[ignore = "BUG: produit `mulsd xmm3,QWORD PTR [rcx]`"]
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
    #[ignore = "BUG: produit `divsd xmm3,QWORD PTR [rcx]`"]
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
    #[ignore = "BUG: produit `comisd xmm3,QWORD PTR [rcx]`"]
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
    #[ignore = "BUG: produit `ucomisd xmm3,QWORD PTR [rcx]`"]
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
    #[ignore = "BUG: produit `movss xmm3,DWORD PTR [rcx]`"]
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
    #[ignore = "BUG: produit `addss xmm3,DWORD PTR [rcx]`"]
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
    #[ignore = "BUG: produit `subss xmm3,DWORD PTR [rcx]`"]
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
    #[ignore = "BUG: produit `mulss xmm3,DWORD PTR [rcx]`"]
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
    #[ignore = "BUG: produit `divss xmm3,DWORD PTR [rcx]`"]
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
    #[ignore = "BUG: produit `comiss xmm3,DWORD PTR [rcx]`"]
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
    #[ignore = "BUG: produit `ucomiss xmm3,DWORD PTR [rcx]`"]
    ucomiss_xmm3_dword_r9: bin(BinOp::UcomiF, reg(XMM3), mem(at(R9)), DWORD) => [0x41, 0x0F, 0x2E, 0x19];
    /// movss dword ptr [rip+my_data], xmm0
    movss_dword_rip_plus_my_data_xmm0: bin(BinOp::StoreF, reg(XMM0), mem(MemAddress::symbol("my_data")), DWORD) => [0xF3, 0x0F, 0x11, 0x05, 0x00, 0x00, 0x00, 0x00];
    /// movss dword ptr [rbx], xmm10
    movss_dword_rbx_xmm10: bin(BinOp::StoreF, reg(XMM10), mem(at(RBX)), DWORD) => [0xF3, 0x44, 0x0F, 0x11, 0x13];
}
