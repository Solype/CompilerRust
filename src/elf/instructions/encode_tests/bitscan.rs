//! Generated encoding tests: reference bytes produced by GNU as.
//! `#[ignore = "BUG: ..."]` marks a wrong encoding (see `cargo test -- --ignored`).

use super::*;

cases! {
    /// bsf ax, bx
    bsf_ax_bx: bitscan(BitScanOp::Bsf, RAX, reg(RBX), WORD) => [0x66, 0x0F, 0xBC, 0xC3];
    /// bsf eax, ebx
    bsf_eax_ebx: bitscan(BitScanOp::Bsf, RAX, reg(RBX), DWORD) => [0x0F, 0xBC, 0xC3];
    /// bsf rax, rbx
    bsf_rax_rbx: bitscan(BitScanOp::Bsf, RAX, reg(RBX), QWORD) => [0x48, 0x0F, 0xBC, 0xC3];
    /// bsf r8, rbx
    bsf_r8_rbx: bitscan(BitScanOp::Bsf, R8, reg(RBX), QWORD) => [0x4C, 0x0F, 0xBC, 0xC3];
    /// bsf rax, r15
    bsf_rax_r15: bitscan(BitScanOp::Bsf, RAX, reg(R15), QWORD) => [0x49, 0x0F, 0xBC, 0xC7];
    /// bsf rax, qword ptr [rbx]
    bsf_rax_qword_rbx: bitscan(BitScanOp::Bsf, RAX, mem(at(RBX)), QWORD) => [0x48, 0x0F, 0xBC, 0x03];
    /// bsf eax, dword ptr [rbx+rcx*4]
    bsf_eax_dword_rbx_plus_rcx_x_4: bitscan(BitScanOp::Bsf, RAX, mem(at(RBX).index_scale(RCX, Scale::Four)), DWORD) => [0x0F, 0xBC, 0x04, 0x8B];
    /// bsr ax, bx
    bsr_ax_bx: bitscan(BitScanOp::Bsr, RAX, reg(RBX), WORD) => [0x66, 0x0F, 0xBD, 0xC3];
    /// bsr eax, ebx
    bsr_eax_ebx: bitscan(BitScanOp::Bsr, RAX, reg(RBX), DWORD) => [0x0F, 0xBD, 0xC3];
    /// bsr rax, rbx
    bsr_rax_rbx: bitscan(BitScanOp::Bsr, RAX, reg(RBX), QWORD) => [0x48, 0x0F, 0xBD, 0xC3];
    /// bsr r8, rbx
    bsr_r8_rbx: bitscan(BitScanOp::Bsr, R8, reg(RBX), QWORD) => [0x4C, 0x0F, 0xBD, 0xC3];
    /// bsr rax, r15
    bsr_rax_r15: bitscan(BitScanOp::Bsr, RAX, reg(R15), QWORD) => [0x49, 0x0F, 0xBD, 0xC7];
    /// bsr rax, qword ptr [rbx]
    bsr_rax_qword_rbx: bitscan(BitScanOp::Bsr, RAX, mem(at(RBX)), QWORD) => [0x48, 0x0F, 0xBD, 0x03];
    /// bsr eax, dword ptr [rbx+rcx*4]
    bsr_eax_dword_rbx_plus_rcx_x_4: bitscan(BitScanOp::Bsr, RAX, mem(at(RBX).index_scale(RCX, Scale::Four)), DWORD) => [0x0F, 0xBD, 0x04, 0x8B];
    /// bsf rax, qword ptr [r9]
    bsf_rax_qword_r9: bitscan(BitScanOp::Bsf, RAX, mem(at(R9)), QWORD) => [0x49, 0x0F, 0xBC, 0x01];
    /// bsr r11d, dword ptr [r9]
    bsr_r11d_dword_r9: bitscan(BitScanOp::Bsr, R11, mem(at(R9)), DWORD) => [0x45, 0x0F, 0xBD, 0x19];
    /// bsf rax, qword ptr [rbx+r8*2]
    bsf_rax_qword_rbx_plus_r8_x_2: bitscan(BitScanOp::Bsf, RAX, mem(at(RBX).index_scale(R8, Scale::Two)), QWORD) => [0x4A, 0x0F, 0xBC, 0x04, 0x43];
    /// bsr r11d, dword ptr [rbx+r8*2]
    bsr_r11d_dword_rbx_plus_r8_x_2: bitscan(BitScanOp::Bsr, R11, mem(at(RBX).index_scale(R8, Scale::Two)), DWORD) => [0x46, 0x0F, 0xBD, 0x1C, 0x43];
    /// bsf rax, qword ptr [r10+rax]
    bsf_rax_qword_r10_plus_rax: bitscan(BitScanOp::Bsf, RAX, mem(at(R10).index(RAX)), QWORD) => [0x49, 0x0F, 0xBC, 0x04, 0x02];
    /// bsr r11d, dword ptr [r10+rax]
    bsr_r11d_dword_r10_plus_rax: bitscan(BitScanOp::Bsr, R11, mem(at(R10).index(RAX)), DWORD) => [0x45, 0x0F, 0xBD, 0x1C, 0x02];
}
