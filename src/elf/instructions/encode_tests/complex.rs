//! Tests d'encodage générés : octets de référence produits par GNU as.
//! `#[ignore = "BUG: ..."]` marque un encodage faux (voir `cargo test -- --ignored`).

use super::*;

cases! {
    /// mul bl
    mul_bl: complex(ComplexBinOp::Mul, Operand::NoOperand, reg(RBX), None, BYTE) => [0xF6, 0xE3];
    /// mul bx
    mul_bx: complex(ComplexBinOp::Mul, Operand::NoOperand, reg(RBX), None, WORD) => [0x66, 0xF7, 0xE3];
    /// mul ebx
    mul_ebx: complex(ComplexBinOp::Mul, Operand::NoOperand, reg(RBX), None, DWORD) => [0xF7, 0xE3];
    /// mul rbx
    mul_rbx: complex(ComplexBinOp::Mul, Operand::NoOperand, reg(RBX), None, QWORD) => [0x48, 0xF7, 0xE3];
    /// mul r9
    mul_r9: complex(ComplexBinOp::Mul, Operand::NoOperand, reg(R9), None, QWORD) => [0x49, 0xF7, 0xE1];
    /// mul dword ptr [rbx+8]
    mul_dword_rbx_plus_8: complex(ComplexBinOp::Mul, Operand::NoOperand, mem(at(RBX).disp(8)), None, DWORD) => [0xF7, 0x63, 0x08];
    /// mul qword ptr [rbx+rcx*8+16]
    mul_qword_rbx_plus_rcx_x_8_plus_16: complex(ComplexBinOp::Mul, Operand::NoOperand, mem(at(RBX).index_scale(RCX, Scale::Eight).disp(16)), None, QWORD) => [0x48, 0xF7, 0x64, 0xCB, 0x10];
    /// div bl
    div_bl: complex(ComplexBinOp::Div, Operand::NoOperand, reg(RBX), None, BYTE) => [0xF6, 0xF3];
    /// div bx
    div_bx: complex(ComplexBinOp::Div, Operand::NoOperand, reg(RBX), None, WORD) => [0x66, 0xF7, 0xF3];
    /// div ebx
    div_ebx: complex(ComplexBinOp::Div, Operand::NoOperand, reg(RBX), None, DWORD) => [0xF7, 0xF3];
    /// div rbx
    div_rbx: complex(ComplexBinOp::Div, Operand::NoOperand, reg(RBX), None, QWORD) => [0x48, 0xF7, 0xF3];
    /// div r9
    div_r9: complex(ComplexBinOp::Div, Operand::NoOperand, reg(R9), None, QWORD) => [0x49, 0xF7, 0xF1];
    /// div dword ptr [rbx+8]
    div_dword_rbx_plus_8: complex(ComplexBinOp::Div, Operand::NoOperand, mem(at(RBX).disp(8)), None, DWORD) => [0xF7, 0x73, 0x08];
    /// div qword ptr [rbx+rcx*8+16]
    div_qword_rbx_plus_rcx_x_8_plus_16: complex(ComplexBinOp::Div, Operand::NoOperand, mem(at(RBX).index_scale(RCX, Scale::Eight).disp(16)), None, QWORD) => [0x48, 0xF7, 0x74, 0xCB, 0x10];
    /// idiv bl
    idiv_bl: complex(ComplexBinOp::Idiv, Operand::NoOperand, reg(RBX), None, BYTE) => [0xF6, 0xFB];
    /// idiv bx
    idiv_bx: complex(ComplexBinOp::Idiv, Operand::NoOperand, reg(RBX), None, WORD) => [0x66, 0xF7, 0xFB];
    /// idiv ebx
    idiv_ebx: complex(ComplexBinOp::Idiv, Operand::NoOperand, reg(RBX), None, DWORD) => [0xF7, 0xFB];
    /// idiv rbx
    idiv_rbx: complex(ComplexBinOp::Idiv, Operand::NoOperand, reg(RBX), None, QWORD) => [0x48, 0xF7, 0xFB];
    /// idiv r9
    idiv_r9: complex(ComplexBinOp::Idiv, Operand::NoOperand, reg(R9), None, QWORD) => [0x49, 0xF7, 0xF9];
    /// idiv dword ptr [rbx+8]
    idiv_dword_rbx_plus_8: complex(ComplexBinOp::Idiv, Operand::NoOperand, mem(at(RBX).disp(8)), None, DWORD) => [0xF7, 0x7B, 0x08];
    /// idiv qword ptr [rbx+rcx*8+16]
    idiv_qword_rbx_plus_rcx_x_8_plus_16: complex(ComplexBinOp::Idiv, Operand::NoOperand, mem(at(RBX).index_scale(RCX, Scale::Eight).disp(16)), None, QWORD) => [0x48, 0xF7, 0x7C, 0xCB, 0x10];
    /// imul bl
    imul_bl: complex(ComplexBinOp::Imul, Operand::NoOperand, reg(RBX), None, BYTE) => [0xF6, 0xEB];
    /// imul bx
    imul_bx: complex(ComplexBinOp::Imul, Operand::NoOperand, reg(RBX), None, WORD) => [0x66, 0xF7, 0xEB];
    /// imul ebx
    imul_ebx: complex(ComplexBinOp::Imul, Operand::NoOperand, reg(RBX), None, DWORD) => [0xF7, 0xEB];
    /// imul rbx
    imul_rbx: complex(ComplexBinOp::Imul, Operand::NoOperand, reg(RBX), None, QWORD) => [0x48, 0xF7, 0xEB];
    /// imul r9
    imul_r9: complex(ComplexBinOp::Imul, Operand::NoOperand, reg(R9), None, QWORD) => [0x49, 0xF7, 0xE9];
    /// imul dword ptr [rbx+8]
    imul_dword_rbx_plus_8: complex(ComplexBinOp::Imul, Operand::NoOperand, mem(at(RBX).disp(8)), None, DWORD) => [0xF7, 0x6B, 0x08];
    /// imul qword ptr [rbx+rcx*8+16]
    imul_qword_rbx_plus_rcx_x_8_plus_16: complex(ComplexBinOp::Imul, Operand::NoOperand, mem(at(RBX).index_scale(RCX, Scale::Eight).disp(16)), None, QWORD) => [0x48, 0xF7, 0x6C, 0xCB, 0x10];
    /// imul ax, bx
    imul_ax_bx: complex(ComplexBinOp::Imul, reg(RAX), reg(RBX), None, WORD) => [0x66, 0x0F, 0xAF, 0xC3];
    /// imul eax, ebx
    imul_eax_ebx: complex(ComplexBinOp::Imul, reg(RAX), reg(RBX), None, DWORD) => [0x0F, 0xAF, 0xC3];
    /// imul rax, rbx
    imul_rax_rbx: complex(ComplexBinOp::Imul, reg(RAX), reg(RBX), None, QWORD) => [0x48, 0x0F, 0xAF, 0xC3];
    /// imul r10, r11
    imul_r10_r11: complex(ComplexBinOp::Imul, reg(R10), reg(R11), None, QWORD) => [0x4D, 0x0F, 0xAF, 0xD3];
    /// imul ecx, dword ptr [rbx]
    imul_ecx_dword_rbx: complex(ComplexBinOp::Imul, reg(RCX), mem(at(RBX)), None, DWORD) => [0x0F, 0xAF, 0x0B];
    /// imul rdx, qword ptr [rbp-16]
    imul_rdx_qword_rbp_minus_16: complex(ComplexBinOp::Imul, reg(RDX), mem(at(RBP).disp(-16)), None, QWORD) => [0x48, 0x0F, 0xAF, 0x55, 0xF0];
    /// imul eax, ebx, 127
    imul_eax_ebx_127: complex(ComplexBinOp::Imul, reg(RAX), reg(RBX), Some(imm(127)), DWORD) => [0x6B, 0xC3, 0x7F];
    /// imul eax, ebx, -128
    imul_eax_ebx_minus_128: complex(ComplexBinOp::Imul, reg(RAX), reg(RBX), Some(imm(-128)), DWORD) => [0x6B, 0xC3, 0x80];
    /// imul eax, ebx, 128
    imul_eax_ebx_128: complex(ComplexBinOp::Imul, reg(RAX), reg(RBX), Some(imm(128)), DWORD) => [0x69, 0xC3, 0x80, 0x00, 0x00, 0x00];
    /// imul eax, ebx, -255
    imul_eax_ebx_minus_255: complex(ComplexBinOp::Imul, reg(RAX), reg(RBX), Some(imm(-255)), DWORD) => [0x69, 0xC3, 0x01, 0xFF, 0xFF, 0xFF];
    /// imul eax, ebx, 1000
    imul_eax_ebx_1000: complex(ComplexBinOp::Imul, reg(RAX), reg(RBX), Some(imm(1000)), DWORD) => [0x69, 0xC3, 0xE8, 0x03, 0x00, 0x00];
    /// imul r8, r9, 3
    imul_r8_r9_3: complex(ComplexBinOp::Imul, reg(R8), reg(R9), Some(imm(3)), QWORD) => [0x4D, 0x6B, 0xC1, 0x03];
    /// imul ax, bx, 3
    imul_ax_bx_3: complex(ComplexBinOp::Imul, reg(RAX), reg(RBX), Some(imm(3)), WORD) => [0x66, 0x6B, 0xC3, 0x03];
    /// imul edx, dword ptr [rbx], 1000
    imul_edx_dword_rbx_1000: complex(ComplexBinOp::Imul, reg(RDX), mem(at(RBX)), Some(imm(1000)), DWORD) => [0x69, 0x13, 0xE8, 0x03, 0x00, 0x00];
    /// xadd al, bl
    xadd_al_bl: complex(ComplexBinOp::Xadd, reg(RAX), reg(RBX), None, BYTE) => [0x0F, 0xC0, 0xD8];
    /// xadd byte ptr [rbx], cl
    xadd_byte_rbx_cl: complex(ComplexBinOp::Xadd, mem(at(RBX)), reg(RCX), None, BYTE) => [0x0F, 0xC0, 0x0B];
    /// xadd ax, bx
    xadd_ax_bx: complex(ComplexBinOp::Xadd, reg(RAX), reg(RBX), None, WORD) => [0x66, 0x0F, 0xC1, 0xD8];
    /// xadd word ptr [rbx], cx
    xadd_word_rbx_cx: complex(ComplexBinOp::Xadd, mem(at(RBX)), reg(RCX), None, WORD) => [0x66, 0x0F, 0xC1, 0x0B];
    /// xadd eax, ebx
    xadd_eax_ebx: complex(ComplexBinOp::Xadd, reg(RAX), reg(RBX), None, DWORD) => [0x0F, 0xC1, 0xD8];
    /// xadd dword ptr [rbx], ecx
    xadd_dword_rbx_ecx: complex(ComplexBinOp::Xadd, mem(at(RBX)), reg(RCX), None, DWORD) => [0x0F, 0xC1, 0x0B];
    /// xadd rax, rbx
    xadd_rax_rbx: complex(ComplexBinOp::Xadd, reg(RAX), reg(RBX), None, QWORD) => [0x48, 0x0F, 0xC1, 0xD8];
    /// xadd qword ptr [rbx], rcx
    xadd_qword_rbx_rcx: complex(ComplexBinOp::Xadd, mem(at(RBX)), reg(RCX), None, QWORD) => [0x48, 0x0F, 0xC1, 0x0B];
    /// xadd r8, r14
    xadd_r8_r14: complex(ComplexBinOp::Xadd, reg(R8), reg(R14), None, QWORD) => [0x4D, 0x0F, 0xC1, 0xF0];
    /// xadd dword ptr [rip+my_data], ecx
    xadd_dword_rip_plus_my_data_ecx: complex(ComplexBinOp::Xadd, mem(MemAddress::symbol("my_data")), reg(RCX), None, DWORD) => [0x0F, 0xC1, 0x0D, 0x00, 0x00, 0x00, 0x00];
    /// cmpxchg al, bl
    cmpxchg_al_bl: complex(ComplexBinOp::Cmpxchg, reg(RAX), reg(RBX), None, BYTE) => [0x0F, 0xB0, 0xD8];
    /// cmpxchg byte ptr [rbx], cl
    cmpxchg_byte_rbx_cl: complex(ComplexBinOp::Cmpxchg, mem(at(RBX)), reg(RCX), None, BYTE) => [0x0F, 0xB0, 0x0B];
    /// cmpxchg ax, bx
    cmpxchg_ax_bx: complex(ComplexBinOp::Cmpxchg, reg(RAX), reg(RBX), None, WORD) => [0x66, 0x0F, 0xB1, 0xD8];
    /// cmpxchg word ptr [rbx], cx
    cmpxchg_word_rbx_cx: complex(ComplexBinOp::Cmpxchg, mem(at(RBX)), reg(RCX), None, WORD) => [0x66, 0x0F, 0xB1, 0x0B];
    /// cmpxchg eax, ebx
    cmpxchg_eax_ebx: complex(ComplexBinOp::Cmpxchg, reg(RAX), reg(RBX), None, DWORD) => [0x0F, 0xB1, 0xD8];
    /// cmpxchg dword ptr [rbx], ecx
    cmpxchg_dword_rbx_ecx: complex(ComplexBinOp::Cmpxchg, mem(at(RBX)), reg(RCX), None, DWORD) => [0x0F, 0xB1, 0x0B];
    /// cmpxchg rax, rbx
    cmpxchg_rax_rbx: complex(ComplexBinOp::Cmpxchg, reg(RAX), reg(RBX), None, QWORD) => [0x48, 0x0F, 0xB1, 0xD8];
    /// cmpxchg qword ptr [rbx], rcx
    cmpxchg_qword_rbx_rcx: complex(ComplexBinOp::Cmpxchg, mem(at(RBX)), reg(RCX), None, QWORD) => [0x48, 0x0F, 0xB1, 0x0B];
    /// cmpxchg r8, r14
    cmpxchg_r8_r14: complex(ComplexBinOp::Cmpxchg, reg(R8), reg(R14), None, QWORD) => [0x4D, 0x0F, 0xB1, 0xF0];
    /// cmpxchg dword ptr [rip+my_data], ecx
    cmpxchg_dword_rip_plus_my_data_ecx: complex(ComplexBinOp::Cmpxchg, mem(MemAddress::symbol("my_data")), reg(RCX), None, DWORD) => [0x0F, 0xB1, 0x0D, 0x00, 0x00, 0x00, 0x00];
    /// mul qword ptr [r9]
    mul_qword_r9: complex(ComplexBinOp::Mul, Operand::NoOperand, mem(at(R9)), None, QWORD) => [0x49, 0xF7, 0x21];
    /// mul qword ptr [rbx+r8*2]
    mul_qword_rbx_plus_r8_x_2: complex(ComplexBinOp::Mul, Operand::NoOperand, mem(at(RBX).index_scale(R8, Scale::Two)), None, QWORD) => [0x4A, 0xF7, 0x24, 0x43];
    /// mul qword ptr [r10+rax]
    mul_qword_r10_plus_rax: complex(ComplexBinOp::Mul, Operand::NoOperand, mem(at(R10).index(RAX)), None, QWORD) => [0x49, 0xF7, 0x24, 0x02];
    /// div qword ptr [r9]
    div_qword_r9: complex(ComplexBinOp::Div, Operand::NoOperand, mem(at(R9)), None, QWORD) => [0x49, 0xF7, 0x31];
    /// div qword ptr [rbx+r8*2]
    div_qword_rbx_plus_r8_x_2: complex(ComplexBinOp::Div, Operand::NoOperand, mem(at(RBX).index_scale(R8, Scale::Two)), None, QWORD) => [0x4A, 0xF7, 0x34, 0x43];
    /// div qword ptr [r10+rax]
    div_qword_r10_plus_rax: complex(ComplexBinOp::Div, Operand::NoOperand, mem(at(R10).index(RAX)), None, QWORD) => [0x49, 0xF7, 0x34, 0x02];
    /// idiv qword ptr [r9]
    idiv_qword_r9: complex(ComplexBinOp::Idiv, Operand::NoOperand, mem(at(R9)), None, QWORD) => [0x49, 0xF7, 0x39];
    /// idiv qword ptr [rbx+r8*2]
    idiv_qword_rbx_plus_r8_x_2: complex(ComplexBinOp::Idiv, Operand::NoOperand, mem(at(RBX).index_scale(R8, Scale::Two)), None, QWORD) => [0x4A, 0xF7, 0x3C, 0x43];
    /// idiv qword ptr [r10+rax]
    idiv_qword_r10_plus_rax: complex(ComplexBinOp::Idiv, Operand::NoOperand, mem(at(R10).index(RAX)), None, QWORD) => [0x49, 0xF7, 0x3C, 0x02];
    /// imul qword ptr [r9]
    imul_qword_r9: complex(ComplexBinOp::Imul, Operand::NoOperand, mem(at(R9)), None, QWORD) => [0x49, 0xF7, 0x29];
    /// imul qword ptr [rbx+r8*2]
    imul_qword_rbx_plus_r8_x_2: complex(ComplexBinOp::Imul, Operand::NoOperand, mem(at(RBX).index_scale(R8, Scale::Two)), None, QWORD) => [0x4A, 0xF7, 0x2C, 0x43];
    /// imul qword ptr [r10+rax]
    imul_qword_r10_plus_rax: complex(ComplexBinOp::Imul, Operand::NoOperand, mem(at(R10).index(RAX)), None, QWORD) => [0x49, 0xF7, 0x2C, 0x02];
    /// imul rax, qword ptr [r9]
    imul_rax_qword_r9: complex(ComplexBinOp::Imul, reg(RAX), mem(at(R9)), None, QWORD) => [0x49, 0x0F, 0xAF, 0x01];
    /// imul r12, qword ptr [r9]
    imul_r12_qword_r9: complex(ComplexBinOp::Imul, reg(R12), mem(at(R9)), None, QWORD) => [0x4D, 0x0F, 0xAF, 0x21];
    /// imul r12d, ebx
    imul_r12d_ebx: complex(ComplexBinOp::Imul, reg(R12), reg(RBX), None, DWORD) => [0x44, 0x0F, 0xAF, 0xE3];
    /// imul rax, qword ptr [rbx+r8*2], 3
    imul_rax_qword_rbx_plus_r8_x_2_3: complex(ComplexBinOp::Imul, reg(RAX), mem(at(RBX).index_scale(R8, Scale::Two)), Some(imm(3)), QWORD) => [0x4A, 0x6B, 0x04, 0x43, 0x03];
    /// imul r12d, dword ptr [r9], 1000
    imul_r12d_dword_r9_1000: complex(ComplexBinOp::Imul, reg(R12), mem(at(R9)), Some(imm(1000)), DWORD) => [0x45, 0x69, 0x21, 0xE8, 0x03, 0x00, 0x00];
    /// xadd qword ptr [r9], r11
    xadd_qword_r9_r11: complex(ComplexBinOp::Xadd, mem(at(R9)), reg(R11), None, QWORD) => [0x4D, 0x0F, 0xC1, 0x19];
    /// xadd qword ptr [rbx+r8*2], r11
    xadd_qword_rbx_plus_r8_x_2_r11: complex(ComplexBinOp::Xadd, mem(at(RBX).index_scale(R8, Scale::Two)), reg(R11), None, QWORD) => [0x4E, 0x0F, 0xC1, 0x1C, 0x43];
    /// xadd qword ptr [r10+rax], r11
    xadd_qword_r10_plus_rax_r11: complex(ComplexBinOp::Xadd, mem(at(R10).index(RAX)), reg(R11), None, QWORD) => [0x4D, 0x0F, 0xC1, 0x1C, 0x02];
    /// cmpxchg qword ptr [r9], r11
    cmpxchg_qword_r9_r11: complex(ComplexBinOp::Cmpxchg, mem(at(R9)), reg(R11), None, QWORD) => [0x4D, 0x0F, 0xB1, 0x19];
    /// cmpxchg qword ptr [rbx+r8*2], r11
    cmpxchg_qword_rbx_plus_r8_x_2_r11: complex(ComplexBinOp::Cmpxchg, mem(at(RBX).index_scale(R8, Scale::Two)), reg(R11), None, QWORD) => [0x4E, 0x0F, 0xB1, 0x1C, 0x43];
    /// cmpxchg qword ptr [r10+rax], r11
    cmpxchg_qword_r10_plus_rax_r11: complex(ComplexBinOp::Cmpxchg, mem(at(R10).index(RAX)), reg(R11), None, QWORD) => [0x4D, 0x0F, 0xB1, 0x1C, 0x02];
    /// imul rax, rbx, 1000
    imul_rax_rbx_1000_qword: complex(ComplexBinOp::Imul, reg(RAX), reg(RBX), Some(imm(1000)), QWORD) => [0x48, 0x69, 0xC3, 0xE8, 0x03, 0x00, 0x00];
}
