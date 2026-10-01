//! Tests d'encodage générés : octets de référence produits par GNU as.
//! `#[ignore = "BUG: ..."]` marque un encodage faux (voir `cargo test -- --ignored`).

use super::*;

cases! {
    /// add ebx, 0x7f
    add_ebx_0x7f: bin(BinOp::Add, reg(RBX), imm(0x7F), DWORD) => [0x83, 0xC3, 0x7F];
    /// add rbx, -1
    add_rbx_minus_1: bin(BinOp::Add, reg(RBX), imm(-1), QWORD) => [0x48, 0x83, 0xC3, 0xFF];
    /// add ebx, 0x1000
    add_ebx_0x1000: bin(BinOp::Add, reg(RBX), imm(0x1000), DWORD) => [0x81, 0xC3, 0x00, 0x10, 0x00, 0x00];
    /// add eax, 0x1000
    add_eax_0x1000: bin(BinOp::Add, reg(RAX), imm(0x1000), DWORD) => [0x05, 0x00, 0x10, 0x00, 0x00] | [0x81, 0xC0, 0x00, 0x10, 0x00, 0x00];
    /// add cl, 0x12
    add_cl_0x12: bin(BinOp::Add, reg(RCX), imm(0x12), BYTE) => [0x80, 0xC1, 0x12];
    /// add cx, 0x1234
    add_cx_0x1234: bin(BinOp::Add, reg(RCX), imm(0x1234), WORD) => [0x66, 0x81, 0xC1, 0x34, 0x12];
    /// add r9, 5
    add_r9_5: bin(BinOp::Add, reg(R9), imm(5), QWORD) => [0x49, 0x83, 0xC1, 0x05];
    /// add bl, al
    add_bl_al: bin(BinOp::Add, reg(RBX), reg(RAX), BYTE) => [0x00, 0xC3] | [0x02, 0xD8];
    /// add bx, ax
    add_bx_ax: bin(BinOp::Add, reg(RBX), reg(RAX), WORD) => [0x66, 0x01, 0xC3] | [0x66, 0x03, 0xD8];
    /// add ebx, eax
    add_ebx_eax: bin(BinOp::Add, reg(RBX), reg(RAX), DWORD) => [0x01, 0xC3] | [0x03, 0xD8];
    /// add rbx, rax
    add_rbx_rax: bin(BinOp::Add, reg(RBX), reg(RAX), QWORD) => [0x48, 0x01, 0xC3] | [0x48, 0x03, 0xD8];
    /// add r8, r15
    add_r8_r15: bin(BinOp::Add, reg(R8), reg(R15), QWORD) => [0x4D, 0x01, 0xF8] | [0x4D, 0x03, 0xC7];
    /// add dword ptr [rbx], 3
    add_dword_rbx_3: bin(BinOp::Add, mem(at(RBX)), imm(3), DWORD) => [0x83, 0x03, 0x03];
    /// add qword ptr [rbx+8], 0x1000
    #[ignore = "BUG: produit `add QWORD PTR [rbx+0x8],0x1000; add BYTE PTR [rax],al`"]
    add_qword_rbx_plus_8_0x1000: bin(BinOp::Add, mem(at(RBX).disp(8)), imm(0x1000), QWORD) => [0x48, 0x81, 0x43, 0x08, 0x00, 0x10, 0x00, 0x00];
    /// add dword ptr [rbx], ecx
    add_dword_rbx_ecx: bin(BinOp::Add, mem(at(RBX)), reg(RCX), DWORD) => [0x01, 0x0B];
    /// add ecx, dword ptr [rbx]
    add_ecx_dword_rbx: bin(BinOp::Add, reg(RCX), mem(at(RBX)), DWORD) => [0x03, 0x0B];
    /// add r10, qword ptr [rip+my_data]
    add_r10_qword_rip_plus_my_data: bin(BinOp::Add, reg(R10), mem(MemAddress::symbol("my_data")), QWORD) => [0x4C, 0x03, 0x15, 0x00, 0x00, 0x00, 0x00];
    /// adc ebx, 0x7f
    adc_ebx_0x7f: bin(BinOp::Adc, reg(RBX), imm(0x7F), DWORD) => [0x83, 0xD3, 0x7F];
    /// adc rbx, -1
    adc_rbx_minus_1: bin(BinOp::Adc, reg(RBX), imm(-1), QWORD) => [0x48, 0x83, 0xD3, 0xFF];
    /// adc ebx, 0x1000
    adc_ebx_0x1000: bin(BinOp::Adc, reg(RBX), imm(0x1000), DWORD) => [0x81, 0xD3, 0x00, 0x10, 0x00, 0x00];
    /// adc eax, 0x1000
    adc_eax_0x1000: bin(BinOp::Adc, reg(RAX), imm(0x1000), DWORD) => [0x15, 0x00, 0x10, 0x00, 0x00] | [0x81, 0xD0, 0x00, 0x10, 0x00, 0x00];
    /// adc cl, 0x12
    adc_cl_0x12: bin(BinOp::Adc, reg(RCX), imm(0x12), BYTE) => [0x80, 0xD1, 0x12];
    /// adc cx, 0x1234
    adc_cx_0x1234: bin(BinOp::Adc, reg(RCX), imm(0x1234), WORD) => [0x66, 0x81, 0xD1, 0x34, 0x12];
    /// adc r9, 5
    adc_r9_5: bin(BinOp::Adc, reg(R9), imm(5), QWORD) => [0x49, 0x83, 0xD1, 0x05];
    /// adc bl, al
    adc_bl_al: bin(BinOp::Adc, reg(RBX), reg(RAX), BYTE) => [0x10, 0xC3] | [0x12, 0xD8];
    /// adc bx, ax
    adc_bx_ax: bin(BinOp::Adc, reg(RBX), reg(RAX), WORD) => [0x66, 0x11, 0xC3] | [0x66, 0x13, 0xD8];
    /// adc ebx, eax
    adc_ebx_eax: bin(BinOp::Adc, reg(RBX), reg(RAX), DWORD) => [0x11, 0xC3] | [0x13, 0xD8];
    /// adc rbx, rax
    adc_rbx_rax: bin(BinOp::Adc, reg(RBX), reg(RAX), QWORD) => [0x48, 0x11, 0xC3] | [0x48, 0x13, 0xD8];
    /// adc r8, r15
    adc_r8_r15: bin(BinOp::Adc, reg(R8), reg(R15), QWORD) => [0x4D, 0x11, 0xF8] | [0x4D, 0x13, 0xC7];
    /// adc dword ptr [rbx], 3
    adc_dword_rbx_3: bin(BinOp::Adc, mem(at(RBX)), imm(3), DWORD) => [0x83, 0x13, 0x03];
    /// adc qword ptr [rbx+8], 0x1000
    #[ignore = "BUG: produit `adc QWORD PTR [rbx+0x8],0x1000; add BYTE PTR [rax],al`"]
    adc_qword_rbx_plus_8_0x1000: bin(BinOp::Adc, mem(at(RBX).disp(8)), imm(0x1000), QWORD) => [0x48, 0x81, 0x53, 0x08, 0x00, 0x10, 0x00, 0x00];
    /// adc dword ptr [rbx], ecx
    adc_dword_rbx_ecx: bin(BinOp::Adc, mem(at(RBX)), reg(RCX), DWORD) => [0x11, 0x0B];
    /// adc ecx, dword ptr [rbx]
    adc_ecx_dword_rbx: bin(BinOp::Adc, reg(RCX), mem(at(RBX)), DWORD) => [0x13, 0x0B];
    /// adc r10, qword ptr [rip+my_data]
    adc_r10_qword_rip_plus_my_data: bin(BinOp::Adc, reg(R10), mem(MemAddress::symbol("my_data")), QWORD) => [0x4C, 0x13, 0x15, 0x00, 0x00, 0x00, 0x00];
    /// sub ebx, 0x7f
    sub_ebx_0x7f: bin(BinOp::Sub, reg(RBX), imm(0x7F), DWORD) => [0x83, 0xEB, 0x7F];
    /// sub rbx, -1
    sub_rbx_minus_1: bin(BinOp::Sub, reg(RBX), imm(-1), QWORD) => [0x48, 0x83, 0xEB, 0xFF];
    /// sub ebx, 0x1000
    sub_ebx_0x1000: bin(BinOp::Sub, reg(RBX), imm(0x1000), DWORD) => [0x81, 0xEB, 0x00, 0x10, 0x00, 0x00];
    /// sub eax, 0x1000
    sub_eax_0x1000: bin(BinOp::Sub, reg(RAX), imm(0x1000), DWORD) => [0x2D, 0x00, 0x10, 0x00, 0x00] | [0x81, 0xE8, 0x00, 0x10, 0x00, 0x00];
    /// sub cl, 0x12
    sub_cl_0x12: bin(BinOp::Sub, reg(RCX), imm(0x12), BYTE) => [0x80, 0xE9, 0x12];
    /// sub cx, 0x1234
    sub_cx_0x1234: bin(BinOp::Sub, reg(RCX), imm(0x1234), WORD) => [0x66, 0x81, 0xE9, 0x34, 0x12];
    /// sub r9, 5
    sub_r9_5: bin(BinOp::Sub, reg(R9), imm(5), QWORD) => [0x49, 0x83, 0xE9, 0x05];
    /// sub bl, al
    sub_bl_al: bin(BinOp::Sub, reg(RBX), reg(RAX), BYTE) => [0x28, 0xC3] | [0x2A, 0xD8];
    /// sub bx, ax
    sub_bx_ax: bin(BinOp::Sub, reg(RBX), reg(RAX), WORD) => [0x66, 0x29, 0xC3] | [0x66, 0x2B, 0xD8];
    /// sub ebx, eax
    sub_ebx_eax: bin(BinOp::Sub, reg(RBX), reg(RAX), DWORD) => [0x29, 0xC3] | [0x2B, 0xD8];
    /// sub rbx, rax
    sub_rbx_rax: bin(BinOp::Sub, reg(RBX), reg(RAX), QWORD) => [0x48, 0x29, 0xC3] | [0x48, 0x2B, 0xD8];
    /// sub r8, r15
    sub_r8_r15: bin(BinOp::Sub, reg(R8), reg(R15), QWORD) => [0x4D, 0x29, 0xF8] | [0x4D, 0x2B, 0xC7];
    /// sub dword ptr [rbx], 3
    sub_dword_rbx_3: bin(BinOp::Sub, mem(at(RBX)), imm(3), DWORD) => [0x83, 0x2B, 0x03];
    /// sub qword ptr [rbx+8], 0x1000
    #[ignore = "BUG: produit `sub QWORD PTR [rbx+0x8],0x1000; add BYTE PTR [rax],al`"]
    sub_qword_rbx_plus_8_0x1000: bin(BinOp::Sub, mem(at(RBX).disp(8)), imm(0x1000), QWORD) => [0x48, 0x81, 0x6B, 0x08, 0x00, 0x10, 0x00, 0x00];
    /// sub dword ptr [rbx], ecx
    sub_dword_rbx_ecx: bin(BinOp::Sub, mem(at(RBX)), reg(RCX), DWORD) => [0x29, 0x0B];
    /// sub ecx, dword ptr [rbx]
    sub_ecx_dword_rbx: bin(BinOp::Sub, reg(RCX), mem(at(RBX)), DWORD) => [0x2B, 0x0B];
    /// sub r10, qword ptr [rip+my_data]
    sub_r10_qword_rip_plus_my_data: bin(BinOp::Sub, reg(R10), mem(MemAddress::symbol("my_data")), QWORD) => [0x4C, 0x2B, 0x15, 0x00, 0x00, 0x00, 0x00];
    /// sbb ebx, 0x7f
    sbb_ebx_0x7f: bin(BinOp::Sbb, reg(RBX), imm(0x7F), DWORD) => [0x83, 0xDB, 0x7F];
    /// sbb rbx, -1
    sbb_rbx_minus_1: bin(BinOp::Sbb, reg(RBX), imm(-1), QWORD) => [0x48, 0x83, 0xDB, 0xFF];
    /// sbb ebx, 0x1000
    sbb_ebx_0x1000: bin(BinOp::Sbb, reg(RBX), imm(0x1000), DWORD) => [0x81, 0xDB, 0x00, 0x10, 0x00, 0x00];
    /// sbb eax, 0x1000
    sbb_eax_0x1000: bin(BinOp::Sbb, reg(RAX), imm(0x1000), DWORD) => [0x1D, 0x00, 0x10, 0x00, 0x00] | [0x81, 0xD8, 0x00, 0x10, 0x00, 0x00];
    /// sbb cl, 0x12
    sbb_cl_0x12: bin(BinOp::Sbb, reg(RCX), imm(0x12), BYTE) => [0x80, 0xD9, 0x12];
    /// sbb cx, 0x1234
    sbb_cx_0x1234: bin(BinOp::Sbb, reg(RCX), imm(0x1234), WORD) => [0x66, 0x81, 0xD9, 0x34, 0x12];
    /// sbb r9, 5
    sbb_r9_5: bin(BinOp::Sbb, reg(R9), imm(5), QWORD) => [0x49, 0x83, 0xD9, 0x05];
    /// sbb bl, al
    sbb_bl_al: bin(BinOp::Sbb, reg(RBX), reg(RAX), BYTE) => [0x18, 0xC3] | [0x1A, 0xD8];
    /// sbb bx, ax
    sbb_bx_ax: bin(BinOp::Sbb, reg(RBX), reg(RAX), WORD) => [0x66, 0x19, 0xC3] | [0x66, 0x1B, 0xD8];
    /// sbb ebx, eax
    sbb_ebx_eax: bin(BinOp::Sbb, reg(RBX), reg(RAX), DWORD) => [0x19, 0xC3] | [0x1B, 0xD8];
    /// sbb rbx, rax
    sbb_rbx_rax: bin(BinOp::Sbb, reg(RBX), reg(RAX), QWORD) => [0x48, 0x19, 0xC3] | [0x48, 0x1B, 0xD8];
    /// sbb r8, r15
    sbb_r8_r15: bin(BinOp::Sbb, reg(R8), reg(R15), QWORD) => [0x4D, 0x19, 0xF8] | [0x4D, 0x1B, 0xC7];
    /// sbb dword ptr [rbx], 3
    sbb_dword_rbx_3: bin(BinOp::Sbb, mem(at(RBX)), imm(3), DWORD) => [0x83, 0x1B, 0x03];
    /// sbb qword ptr [rbx+8], 0x1000
    #[ignore = "BUG: produit `sbb QWORD PTR [rbx+0x8],0x1000; add BYTE PTR [rax],al`"]
    sbb_qword_rbx_plus_8_0x1000: bin(BinOp::Sbb, mem(at(RBX).disp(8)), imm(0x1000), QWORD) => [0x48, 0x81, 0x5B, 0x08, 0x00, 0x10, 0x00, 0x00];
    /// sbb dword ptr [rbx], ecx
    sbb_dword_rbx_ecx: bin(BinOp::Sbb, mem(at(RBX)), reg(RCX), DWORD) => [0x19, 0x0B];
    /// sbb ecx, dword ptr [rbx]
    sbb_ecx_dword_rbx: bin(BinOp::Sbb, reg(RCX), mem(at(RBX)), DWORD) => [0x1B, 0x0B];
    /// sbb r10, qword ptr [rip+my_data]
    sbb_r10_qword_rip_plus_my_data: bin(BinOp::Sbb, reg(R10), mem(MemAddress::symbol("my_data")), QWORD) => [0x4C, 0x1B, 0x15, 0x00, 0x00, 0x00, 0x00];
    /// and ebx, 0x7f
    and_ebx_0x7f: bin(BinOp::And, reg(RBX), imm(0x7F), DWORD) => [0x83, 0xE3, 0x7F];
    /// and rbx, -1
    and_rbx_minus_1: bin(BinOp::And, reg(RBX), imm(-1), QWORD) => [0x48, 0x83, 0xE3, 0xFF];
    /// and ebx, 0x1000
    and_ebx_0x1000: bin(BinOp::And, reg(RBX), imm(0x1000), DWORD) => [0x81, 0xE3, 0x00, 0x10, 0x00, 0x00];
    /// and eax, 0x1000
    and_eax_0x1000: bin(BinOp::And, reg(RAX), imm(0x1000), DWORD) => [0x25, 0x00, 0x10, 0x00, 0x00] | [0x81, 0xE0, 0x00, 0x10, 0x00, 0x00];
    /// and cl, 0x12
    and_cl_0x12: bin(BinOp::And, reg(RCX), imm(0x12), BYTE) => [0x80, 0xE1, 0x12];
    /// and cx, 0x1234
    and_cx_0x1234: bin(BinOp::And, reg(RCX), imm(0x1234), WORD) => [0x66, 0x81, 0xE1, 0x34, 0x12];
    /// and r9, 5
    and_r9_5: bin(BinOp::And, reg(R9), imm(5), QWORD) => [0x49, 0x83, 0xE1, 0x05];
    /// and bl, al
    and_bl_al: bin(BinOp::And, reg(RBX), reg(RAX), BYTE) => [0x20, 0xC3] | [0x22, 0xD8];
    /// and bx, ax
    and_bx_ax: bin(BinOp::And, reg(RBX), reg(RAX), WORD) => [0x66, 0x21, 0xC3] | [0x66, 0x23, 0xD8];
    /// and ebx, eax
    and_ebx_eax: bin(BinOp::And, reg(RBX), reg(RAX), DWORD) => [0x21, 0xC3] | [0x23, 0xD8];
    /// and rbx, rax
    and_rbx_rax: bin(BinOp::And, reg(RBX), reg(RAX), QWORD) => [0x48, 0x21, 0xC3] | [0x48, 0x23, 0xD8];
    /// and r8, r15
    and_r8_r15: bin(BinOp::And, reg(R8), reg(R15), QWORD) => [0x4D, 0x21, 0xF8] | [0x4D, 0x23, 0xC7];
    /// and dword ptr [rbx], 3
    and_dword_rbx_3: bin(BinOp::And, mem(at(RBX)), imm(3), DWORD) => [0x83, 0x23, 0x03];
    /// and qword ptr [rbx+8], 0x1000
    #[ignore = "BUG: produit `and QWORD PTR [rbx+0x8],0x1000; add BYTE PTR [rax],al`"]
    and_qword_rbx_plus_8_0x1000: bin(BinOp::And, mem(at(RBX).disp(8)), imm(0x1000), QWORD) => [0x48, 0x81, 0x63, 0x08, 0x00, 0x10, 0x00, 0x00];
    /// and dword ptr [rbx], ecx
    and_dword_rbx_ecx: bin(BinOp::And, mem(at(RBX)), reg(RCX), DWORD) => [0x21, 0x0B];
    /// and ecx, dword ptr [rbx]
    and_ecx_dword_rbx: bin(BinOp::And, reg(RCX), mem(at(RBX)), DWORD) => [0x23, 0x0B];
    /// and r10, qword ptr [rip+my_data]
    and_r10_qword_rip_plus_my_data: bin(BinOp::And, reg(R10), mem(MemAddress::symbol("my_data")), QWORD) => [0x4C, 0x23, 0x15, 0x00, 0x00, 0x00, 0x00];
    /// or ebx, 0x7f
    or_ebx_0x7f: bin(BinOp::Or, reg(RBX), imm(0x7F), DWORD) => [0x83, 0xCB, 0x7F];
    /// or rbx, -1
    or_rbx_minus_1: bin(BinOp::Or, reg(RBX), imm(-1), QWORD) => [0x48, 0x83, 0xCB, 0xFF];
    /// or ebx, 0x1000
    or_ebx_0x1000: bin(BinOp::Or, reg(RBX), imm(0x1000), DWORD) => [0x81, 0xCB, 0x00, 0x10, 0x00, 0x00];
    /// or eax, 0x1000
    or_eax_0x1000: bin(BinOp::Or, reg(RAX), imm(0x1000), DWORD) => [0x0D, 0x00, 0x10, 0x00, 0x00] | [0x81, 0xC8, 0x00, 0x10, 0x00, 0x00];
    /// or cl, 0x12
    or_cl_0x12: bin(BinOp::Or, reg(RCX), imm(0x12), BYTE) => [0x80, 0xC9, 0x12];
    /// or cx, 0x1234
    or_cx_0x1234: bin(BinOp::Or, reg(RCX), imm(0x1234), WORD) => [0x66, 0x81, 0xC9, 0x34, 0x12];
    /// or r9, 5
    or_r9_5: bin(BinOp::Or, reg(R9), imm(5), QWORD) => [0x49, 0x83, 0xC9, 0x05];
    /// or bl, al
    or_bl_al: bin(BinOp::Or, reg(RBX), reg(RAX), BYTE) => [0x08, 0xC3] | [0x0A, 0xD8];
    /// or bx, ax
    or_bx_ax: bin(BinOp::Or, reg(RBX), reg(RAX), WORD) => [0x66, 0x09, 0xC3] | [0x66, 0x0B, 0xD8];
    /// or ebx, eax
    or_ebx_eax: bin(BinOp::Or, reg(RBX), reg(RAX), DWORD) => [0x09, 0xC3] | [0x0B, 0xD8];
    /// or rbx, rax
    or_rbx_rax: bin(BinOp::Or, reg(RBX), reg(RAX), QWORD) => [0x48, 0x09, 0xC3] | [0x48, 0x0B, 0xD8];
    /// or r8, r15
    or_r8_r15: bin(BinOp::Or, reg(R8), reg(R15), QWORD) => [0x4D, 0x09, 0xF8] | [0x4D, 0x0B, 0xC7];
    /// or dword ptr [rbx], 3
    or_dword_rbx_3: bin(BinOp::Or, mem(at(RBX)), imm(3), DWORD) => [0x83, 0x0B, 0x03];
    /// or qword ptr [rbx+8], 0x1000
    #[ignore = "BUG: produit `or QWORD PTR [rbx+0x8],0x1000; add BYTE PTR [rax],al`"]
    or_qword_rbx_plus_8_0x1000: bin(BinOp::Or, mem(at(RBX).disp(8)), imm(0x1000), QWORD) => [0x48, 0x81, 0x4B, 0x08, 0x00, 0x10, 0x00, 0x00];
    /// or dword ptr [rbx], ecx
    or_dword_rbx_ecx: bin(BinOp::Or, mem(at(RBX)), reg(RCX), DWORD) => [0x09, 0x0B];
    /// or ecx, dword ptr [rbx]
    or_ecx_dword_rbx: bin(BinOp::Or, reg(RCX), mem(at(RBX)), DWORD) => [0x0B, 0x0B];
    /// or r10, qword ptr [rip+my_data]
    or_r10_qword_rip_plus_my_data: bin(BinOp::Or, reg(R10), mem(MemAddress::symbol("my_data")), QWORD) => [0x4C, 0x0B, 0x15, 0x00, 0x00, 0x00, 0x00];
    /// xor ebx, 0x7f
    xor_ebx_0x7f: bin(BinOp::Xor, reg(RBX), imm(0x7F), DWORD) => [0x83, 0xF3, 0x7F];
    /// xor rbx, -1
    xor_rbx_minus_1: bin(BinOp::Xor, reg(RBX), imm(-1), QWORD) => [0x48, 0x83, 0xF3, 0xFF];
    /// xor ebx, 0x1000
    xor_ebx_0x1000: bin(BinOp::Xor, reg(RBX), imm(0x1000), DWORD) => [0x81, 0xF3, 0x00, 0x10, 0x00, 0x00];
    /// xor eax, 0x1000
    xor_eax_0x1000: bin(BinOp::Xor, reg(RAX), imm(0x1000), DWORD) => [0x35, 0x00, 0x10, 0x00, 0x00] | [0x81, 0xF0, 0x00, 0x10, 0x00, 0x00];
    /// xor cl, 0x12
    xor_cl_0x12: bin(BinOp::Xor, reg(RCX), imm(0x12), BYTE) => [0x80, 0xF1, 0x12];
    /// xor cx, 0x1234
    xor_cx_0x1234: bin(BinOp::Xor, reg(RCX), imm(0x1234), WORD) => [0x66, 0x81, 0xF1, 0x34, 0x12];
    /// xor r9, 5
    xor_r9_5: bin(BinOp::Xor, reg(R9), imm(5), QWORD) => [0x49, 0x83, 0xF1, 0x05];
    /// xor bl, al
    xor_bl_al: bin(BinOp::Xor, reg(RBX), reg(RAX), BYTE) => [0x30, 0xC3] | [0x32, 0xD8];
    /// xor bx, ax
    xor_bx_ax: bin(BinOp::Xor, reg(RBX), reg(RAX), WORD) => [0x66, 0x31, 0xC3] | [0x66, 0x33, 0xD8];
    /// xor ebx, eax
    xor_ebx_eax: bin(BinOp::Xor, reg(RBX), reg(RAX), DWORD) => [0x31, 0xC3] | [0x33, 0xD8];
    /// xor rbx, rax
    xor_rbx_rax: bin(BinOp::Xor, reg(RBX), reg(RAX), QWORD) => [0x48, 0x31, 0xC3] | [0x48, 0x33, 0xD8];
    /// xor r8, r15
    xor_r8_r15: bin(BinOp::Xor, reg(R8), reg(R15), QWORD) => [0x4D, 0x31, 0xF8] | [0x4D, 0x33, 0xC7];
    /// xor dword ptr [rbx], 3
    xor_dword_rbx_3: bin(BinOp::Xor, mem(at(RBX)), imm(3), DWORD) => [0x83, 0x33, 0x03];
    /// xor qword ptr [rbx+8], 0x1000
    #[ignore = "BUG: produit `xor QWORD PTR [rbx+0x8],0x1000; add BYTE PTR [rax],al`"]
    xor_qword_rbx_plus_8_0x1000: bin(BinOp::Xor, mem(at(RBX).disp(8)), imm(0x1000), QWORD) => [0x48, 0x81, 0x73, 0x08, 0x00, 0x10, 0x00, 0x00];
    /// xor dword ptr [rbx], ecx
    xor_dword_rbx_ecx: bin(BinOp::Xor, mem(at(RBX)), reg(RCX), DWORD) => [0x31, 0x0B];
    /// xor ecx, dword ptr [rbx]
    xor_ecx_dword_rbx: bin(BinOp::Xor, reg(RCX), mem(at(RBX)), DWORD) => [0x33, 0x0B];
    /// xor r10, qword ptr [rip+my_data]
    xor_r10_qword_rip_plus_my_data: bin(BinOp::Xor, reg(R10), mem(MemAddress::symbol("my_data")), QWORD) => [0x4C, 0x33, 0x15, 0x00, 0x00, 0x00, 0x00];
    /// cmp ebx, 0x7f
    cmp_ebx_0x7f: bin(BinOp::Cmp, reg(RBX), imm(0x7F), DWORD) => [0x83, 0xFB, 0x7F];
    /// cmp rbx, -1
    cmp_rbx_minus_1: bin(BinOp::Cmp, reg(RBX), imm(-1), QWORD) => [0x48, 0x83, 0xFB, 0xFF];
    /// cmp ebx, 0x1000
    cmp_ebx_0x1000: bin(BinOp::Cmp, reg(RBX), imm(0x1000), DWORD) => [0x81, 0xFB, 0x00, 0x10, 0x00, 0x00];
    /// cmp eax, 0x1000
    cmp_eax_0x1000: bin(BinOp::Cmp, reg(RAX), imm(0x1000), DWORD) => [0x3D, 0x00, 0x10, 0x00, 0x00] | [0x81, 0xF8, 0x00, 0x10, 0x00, 0x00];
    /// cmp cl, 0x12
    cmp_cl_0x12: bin(BinOp::Cmp, reg(RCX), imm(0x12), BYTE) => [0x80, 0xF9, 0x12];
    /// cmp cx, 0x1234
    cmp_cx_0x1234: bin(BinOp::Cmp, reg(RCX), imm(0x1234), WORD) => [0x66, 0x81, 0xF9, 0x34, 0x12];
    /// cmp r9, 5
    cmp_r9_5: bin(BinOp::Cmp, reg(R9), imm(5), QWORD) => [0x49, 0x83, 0xF9, 0x05];
    /// cmp bl, al
    cmp_bl_al: bin(BinOp::Cmp, reg(RBX), reg(RAX), BYTE) => [0x38, 0xC3] | [0x3A, 0xD8];
    /// cmp bx, ax
    cmp_bx_ax: bin(BinOp::Cmp, reg(RBX), reg(RAX), WORD) => [0x66, 0x39, 0xC3] | [0x66, 0x3B, 0xD8];
    /// cmp ebx, eax
    cmp_ebx_eax: bin(BinOp::Cmp, reg(RBX), reg(RAX), DWORD) => [0x39, 0xC3] | [0x3B, 0xD8];
    /// cmp rbx, rax
    cmp_rbx_rax: bin(BinOp::Cmp, reg(RBX), reg(RAX), QWORD) => [0x48, 0x39, 0xC3] | [0x48, 0x3B, 0xD8];
    /// cmp r8, r15
    cmp_r8_r15: bin(BinOp::Cmp, reg(R8), reg(R15), QWORD) => [0x4D, 0x39, 0xF8] | [0x4D, 0x3B, 0xC7];
    /// cmp dword ptr [rbx], 3
    cmp_dword_rbx_3: bin(BinOp::Cmp, mem(at(RBX)), imm(3), DWORD) => [0x83, 0x3B, 0x03];
    /// cmp qword ptr [rbx+8], 0x1000
    #[ignore = "BUG: produit `cmp QWORD PTR [rbx+0x8],0x1000; add BYTE PTR [rax],al`"]
    cmp_qword_rbx_plus_8_0x1000: bin(BinOp::Cmp, mem(at(RBX).disp(8)), imm(0x1000), QWORD) => [0x48, 0x81, 0x7B, 0x08, 0x00, 0x10, 0x00, 0x00];
    /// cmp dword ptr [rbx], ecx
    cmp_dword_rbx_ecx: bin(BinOp::Cmp, mem(at(RBX)), reg(RCX), DWORD) => [0x39, 0x0B];
    /// cmp ecx, dword ptr [rbx]
    cmp_ecx_dword_rbx: bin(BinOp::Cmp, reg(RCX), mem(at(RBX)), DWORD) => [0x3B, 0x0B];
    /// cmp r10, qword ptr [rip+my_data]
    cmp_r10_qword_rip_plus_my_data: bin(BinOp::Cmp, reg(R10), mem(MemAddress::symbol("my_data")), QWORD) => [0x4C, 0x3B, 0x15, 0x00, 0x00, 0x00, 0x00];
    /// test eax, 0x7f
    test_eax_0x7f: bin(BinOp::Test, reg(RAX), imm(0x7F), DWORD) => [0xA9, 0x7F, 0x00, 0x00, 0x00] | [0xF7, 0xC0, 0x7F, 0x00, 0x00, 0x00];
    /// test ebx, 0x7f
    test_ebx_0x7f: bin(BinOp::Test, reg(RBX), imm(0x7F), DWORD) => [0xF7, 0xC3, 0x7F, 0x00, 0x00, 0x00];
    /// test bl, 1
    test_bl_1: bin(BinOp::Test, reg(RBX), imm(1), BYTE) => [0xF6, 0xC3, 0x01];
    /// test r9, 1
    #[ignore = "BUG: produit `test r9,0x1; add BYTE PTR [rax],al`"]
    test_r9_1: bin(BinOp::Test, reg(R9), imm(1), QWORD) => [0x49, 0xF7, 0xC1, 0x01, 0x00, 0x00, 0x00];
    /// test bl, al
    test_bl_al: bin(BinOp::Test, reg(RBX), reg(RAX), BYTE) => [0x84, 0xC3] | [0x84, 0xD8];
    /// test bx, ax
    test_bx_ax: bin(BinOp::Test, reg(RBX), reg(RAX), WORD) => [0x66, 0x85, 0xC3] | [0x66, 0x85, 0xD8];
    /// test ebx, eax
    test_ebx_eax: bin(BinOp::Test, reg(RBX), reg(RAX), DWORD) => [0x85, 0xC3] | [0x85, 0xD8];
    /// test rbx, rax
    test_rbx_rax: bin(BinOp::Test, reg(RBX), reg(RAX), QWORD) => [0x48, 0x85, 0xC3] | [0x48, 0x85, 0xD8];
    /// test dword ptr [rbx], 3
    test_dword_rbx_3: bin(BinOp::Test, mem(at(RBX)), imm(3), DWORD) => [0xF7, 0x03, 0x03, 0x00, 0x00, 0x00];
    /// test dword ptr [rbx], ecx
    test_dword_rbx_ecx: bin(BinOp::Test, mem(at(RBX)), reg(RCX), DWORD) => [0x85, 0x0B];
}
