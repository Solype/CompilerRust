//! Conversion tests (`ConvOp` family): reference bytes produced by GNU as.

use super::*;
use crate::elf::instructions::encode_conversion::encode_conversion;

fn encode(dst: Register, src: Operand, size: Size) -> Vec<u8> {
    encode_conversion(ConvOp::Cvtsi2sd, &dst, &src, size).data
}

// ==========================================
// Register source
// ==========================================

#[test]
fn reg64() {
    assert_eq!(encode(XMM0, Operand::Reg(RAX), Size::U64), [0xF2, 0x48, 0x0F, 0x2A, 0xC0]);
}

#[test]
fn reg32_has_no_rex() {
    assert_eq!(encode(XMM0, Operand::Reg(RAX), Size::U32), [0xF2, 0x0F, 0x2A, 0xC0]);
}

#[test]
fn extended_xmm_sets_rex_r() {
    assert_eq!(encode(XMM15, Operand::Reg(RDI), Size::U64), [0xF2, 0x4C, 0x0F, 0x2A, 0xFF]);
}

#[test]
fn extended_gpr_sets_rex_b() {
    assert_eq!(encode(XMM3, Operand::Reg(R8), Size::U64), [0xF2, 0x49, 0x0F, 0x2A, 0xD8]);
    assert_eq!(encode(XMM3, Operand::Reg(R8), Size::U32), [0xF2, 0x41, 0x0F, 0x2A, 0xD8]);
}

#[test]
fn extended_both() {
    assert_eq!(encode(XMM8, Operand::Reg(R15), Size::U64), [0xF2, 0x4D, 0x0F, 0x2A, 0xC7]);
}

#[test]
fn prefix_comes_before_rex() {
    let bytes = encode(XMM9, Operand::Reg(R12), Size::U64);
    assert_eq!(bytes[0], 0xF2);
    assert_eq!(bytes[1] & 0xF0, 0x40);
}

// ==========================================
// Memory source
// ==========================================

#[test]
fn mem64() {
    assert_eq!(encode(XMM1, mem(at(RBX)), Size::U64), [0xF2, 0x48, 0x0F, 0x2A, 0x0B]);
}

#[test]
fn mem32() {
    assert_eq!(encode(XMM1, mem(at(RBX)), Size::U32), [0xF2, 0x0F, 0x2A, 0x0B]);
}

#[test]
fn mem_with_disp8() {
    let src = Operand::MemoryAddress(MemAddress::new().base(RBP).disp(8));
    assert_eq!(encode(XMM1, src, Size::U64), [0xF2, 0x48, 0x0F, 0x2A, 0x4D, 0x08]);
}

#[test]
fn mem_rsp_base_needs_sib() {
    assert_eq!(encode(XMM2, mem(at(RSP)), Size::U64), [0xF2, 0x48, 0x0F, 0x2A, 0x14, 0x24]);
}

#[test]
fn mem_extended_xmm() {
    assert_eq!(encode(XMM9, mem(at(RAX)), Size::U64), [0xF2, 0x4C, 0x0F, 0x2A, 0x08]);
}

#[test]
fn mem_extended_base() {
    assert_eq!(encode(XMM1, mem(at(R12)), Size::U64), [0xF2, 0x49, 0x0F, 0x2A, 0x0C, 0x24]);
}

#[test]
fn rip_relative_symbol_emits_relocation() {
    let src = Operand::MemoryAddress(MemAddress::symbol("my_float"));
    let info = encode_conversion(ConvOp::Cvtsi2sd, &XMM2, &src, Size::U64);

    assert_eq!(info.data, [0xF2, 0x48, 0x0F, 0x2A, 0x15, 0x00, 0x00, 0x00, 0x00]);
    assert_eq!(info.relocations.len(), 1);
    assert_eq!(info.relocations[0].sym, "my_float");
    assert_eq!(info.relocations[0].offset, 5);
}

// ==========================================
// Dispatch via Instruction::encode
// ==========================================

#[test]
fn instruction_uses_default_size() {
    let ins = Instruction::Convert { op: ConvOp::Cvtsi2sd, dst: XMM0, src: Operand::Reg(RAX), size: None };
    assert_eq!(ins.encode(Size::U64).data, [0xF2, 0x48, 0x0F, 0x2A, 0xC0]);
    assert_eq!(ins.encode(Size::U32).data, [0xF2, 0x0F, 0x2A, 0xC0]);
}

#[test]
fn instruction_explicit_size_wins() {
    let ins = Instruction::Convert { op: ConvOp::Cvtsi2sd, dst: XMM0, src: Operand::Reg(RAX), size: Some(Size::U32) };
    assert_eq!(ins.encode(Size::U64).data, [0xF2, 0x0F, 0x2A, 0xC0]);
}

// ==========================================
// cvtsi2ss: same form as cvtsi2sd, F3 prefix
// ==========================================

fn encode_ss(dst: Register, src: Operand, size: Size) -> Vec<u8> {
    encode_conversion(ConvOp::Cvtsi2ss, &dst, &src, size).data
}

#[test]
fn ss_reg64() {
    assert_eq!(encode_ss(XMM0, Operand::Reg(RAX), Size::U64), [0xF3, 0x48, 0x0F, 0x2A, 0xC0]);
}

#[test]
fn ss_reg32_has_no_rex() {
    assert_eq!(encode_ss(XMM0, Operand::Reg(RAX), Size::U32), [0xF3, 0x0F, 0x2A, 0xC0]);
}

#[test]
fn ss_extended_xmm_sets_rex_r() {
    assert_eq!(encode_ss(XMM15, Operand::Reg(RDI), Size::U64), [0xF3, 0x4C, 0x0F, 0x2A, 0xFF]);
}

#[test]
fn ss_extended_gpr_sets_rex_b() {
    assert_eq!(encode_ss(XMM3, Operand::Reg(R8), Size::U64), [0xF3, 0x49, 0x0F, 0x2A, 0xD8]);
    assert_eq!(encode_ss(XMM3, Operand::Reg(R8), Size::U32), [0xF3, 0x41, 0x0F, 0x2A, 0xD8]);
}

#[test]
fn ss_extended_both() {
    assert_eq!(encode_ss(XMM8, Operand::Reg(R15), Size::U64), [0xF3, 0x4D, 0x0F, 0x2A, 0xC7]);
}

#[test]
fn ss_mem64() {
    assert_eq!(encode_ss(XMM1, mem(at(RBX)), Size::U64), [0xF3, 0x48, 0x0F, 0x2A, 0x0B]);
}

#[test]
fn ss_mem32() {
    assert_eq!(encode_ss(XMM1, mem(at(RBX)), Size::U32), [0xF3, 0x0F, 0x2A, 0x0B]);
}

#[test]
fn ss_mem_with_disp8() {
    let src = Operand::MemoryAddress(MemAddress::new().base(RBP).disp(8));
    assert_eq!(encode_ss(XMM1, src, Size::U64), [0xF3, 0x48, 0x0F, 0x2A, 0x4D, 0x08]);
}

#[test]
fn ss_mem_rsp_base_needs_sib() {
    assert_eq!(encode_ss(XMM2, mem(at(RSP)), Size::U64), [0xF3, 0x48, 0x0F, 0x2A, 0x14, 0x24]);
}

#[test]
fn ss_mem_extended_xmm() {
    assert_eq!(encode_ss(XMM9, mem(at(RAX)), Size::U64), [0xF3, 0x4C, 0x0F, 0x2A, 0x08]);
}

#[test]
fn ss_mem_extended_base() {
    assert_eq!(encode_ss(XMM1, mem(at(R12)), Size::U64), [0xF3, 0x49, 0x0F, 0x2A, 0x0C, 0x24]);
}

#[test]
fn ss_rip_relative_symbol_emits_relocation() {
    let src = Operand::MemoryAddress(MemAddress::symbol("my_float"));
    let info = encode_conversion(ConvOp::Cvtsi2ss, &XMM2, &src, Size::U64);

    assert_eq!(info.data, [0xF3, 0x48, 0x0F, 0x2A, 0x15, 0x00, 0x00, 0x00, 0x00]);
    assert_eq!(info.relocations.len(), 1);
    assert_eq!(info.relocations[0].sym, "my_float");
    assert_eq!(info.relocations[0].offset, 5);
}

#[test]
#[should_panic(expected = "destination must be an XMM register")]
fn ss_gpr_destination_panics() {
    encode_ss(RAX, Operand::Reg(RBX), Size::U64);
}

#[test]
#[should_panic(expected = "integer source must be 32 or 64 bits")]
fn ss_size_u16_panics() {
    encode_ss(XMM0, Operand::Reg(RAX), Size::U16);
}

// ==========================================
// Flottant -> entier : cvt[t]sd2si, cvt[t]ss2si
// ==========================================

fn enc(op: ConvOp, dst: Register, src: Operand, size: Size) -> Vec<u8> {
    encode_conversion(op, &dst, &src, size).data
}

#[test]
fn cvttsd2si_reg() {
    assert_eq!(enc(ConvOp::Cvttsd2si, RAX, Operand::Reg(XMM0), Size::U64), [0xF2, 0x48, 0x0F, 0x2C, 0xC0]);
    assert_eq!(enc(ConvOp::Cvttsd2si, RAX, Operand::Reg(XMM0), Size::U32), [0xF2, 0x0F, 0x2C, 0xC0]);
}

#[test]
fn cvttsd2si_extended_gpr_sets_rex_r() {
    assert_eq!(enc(ConvOp::Cvttsd2si, R15, Operand::Reg(XMM8), Size::U64), [0xF2, 0x4D, 0x0F, 0x2C, 0xF8]);
}

#[test]
fn cvttsd2si_extended_xmm_sets_rex_b() {
    assert_eq!(enc(ConvOp::Cvttsd2si, RDI, Operand::Reg(XMM15), Size::U64), [0xF2, 0x49, 0x0F, 0x2C, 0xFF]);
}

#[test]
fn cvttsd2si_mem() {
    assert_eq!(enc(ConvOp::Cvttsd2si, RCX, mem(at(RBX)), Size::U64), [0xF2, 0x48, 0x0F, 0x2C, 0x0B]);
    assert_eq!(enc(ConvOp::Cvttsd2si, RDX, mem(at(RSP)), Size::U64), [0xF2, 0x48, 0x0F, 0x2C, 0x14, 0x24]);
    let src = Operand::MemoryAddress(MemAddress::new().base(RBP).disp(8));
    assert_eq!(enc(ConvOp::Cvttsd2si, R9, src, Size::U64), [0xF2, 0x4C, 0x0F, 0x2C, 0x4D, 0x08]);
}

#[test]
fn cvttsd2si_rip_relative_symbol_emits_relocation() {
    let src = Operand::MemoryAddress(MemAddress::symbol("my_float"));
    let info = encode_conversion(ConvOp::Cvttsd2si, &RAX, &src, Size::U64);

    assert_eq!(info.data, [0xF2, 0x48, 0x0F, 0x2C, 0x05, 0x00, 0x00, 0x00, 0x00]);
    assert_eq!(info.relocations.len(), 1);
    assert_eq!(info.relocations[0].sym, "my_float");
    assert_eq!(info.relocations[0].offset, 5);
}

#[test]
fn cvttss2si() {
    assert_eq!(enc(ConvOp::Cvttss2si, RAX, Operand::Reg(XMM0), Size::U64), [0xF3, 0x48, 0x0F, 0x2C, 0xC0]);
    assert_eq!(enc(ConvOp::Cvttss2si, RAX, Operand::Reg(XMM0), Size::U32), [0xF3, 0x0F, 0x2C, 0xC0]);
    assert_eq!(enc(ConvOp::Cvttss2si, R15, Operand::Reg(XMM8), Size::U64), [0xF3, 0x4D, 0x0F, 0x2C, 0xF8]);
    assert_eq!(enc(ConvOp::Cvttss2si, RCX, mem(at(RBX)), Size::U32), [0xF3, 0x0F, 0x2C, 0x0B]);
}

#[test]
fn cvtsd2si() {
    assert_eq!(enc(ConvOp::Cvtsd2si, RAX, Operand::Reg(XMM0), Size::U64), [0xF2, 0x48, 0x0F, 0x2D, 0xC0]);
    assert_eq!(enc(ConvOp::Cvtsd2si, RAX, Operand::Reg(XMM0), Size::U32), [0xF2, 0x0F, 0x2D, 0xC0]);
    assert_eq!(enc(ConvOp::Cvtsd2si, R12, Operand::Reg(XMM9), Size::U64), [0xF2, 0x4D, 0x0F, 0x2D, 0xE1]);
    assert_eq!(enc(ConvOp::Cvtsd2si, RCX, mem(at(RBX)), Size::U64), [0xF2, 0x48, 0x0F, 0x2D, 0x0B]);
}

#[test]
fn cvtss2si() {
    assert_eq!(enc(ConvOp::Cvtss2si, RAX, Operand::Reg(XMM0), Size::U64), [0xF3, 0x48, 0x0F, 0x2D, 0xC0]);
    assert_eq!(enc(ConvOp::Cvtss2si, RAX, Operand::Reg(XMM0), Size::U32), [0xF3, 0x0F, 0x2D, 0xC0]);
    assert_eq!(enc(ConvOp::Cvtss2si, R12, Operand::Reg(XMM9), Size::U64), [0xF3, 0x4D, 0x0F, 0x2D, 0xE1]);
    assert_eq!(enc(ConvOp::Cvtss2si, RCX, mem(at(RBX)), Size::U32), [0xF3, 0x0F, 0x2D, 0x0B]);
}

// ==========================================
// Flottant -> flottant : cvtsd2ss, cvtss2sd
// ==========================================

#[test]
fn cvtsd2ss_reg() {
    assert_eq!(enc(ConvOp::Cvtsd2ss, XMM0, Operand::Reg(XMM1), Size::U64), [0xF2, 0x0F, 0x5A, 0xC1]);
    assert_eq!(enc(ConvOp::Cvtsd2ss, XMM8, Operand::Reg(XMM15), Size::U64), [0xF2, 0x45, 0x0F, 0x5A, 0xC7]);
    assert_eq!(enc(ConvOp::Cvtsd2ss, XMM2, Operand::Reg(XMM9), Size::U64), [0xF2, 0x41, 0x0F, 0x5A, 0xD1]);
}

#[test]
fn cvtsd2ss_mem() {
    assert_eq!(enc(ConvOp::Cvtsd2ss, XMM1, mem(at(RBX)), Size::U64), [0xF2, 0x0F, 0x5A, 0x0B]);
    assert_eq!(enc(ConvOp::Cvtsd2ss, XMM1, mem(at(R12)), Size::U64), [0xF2, 0x41, 0x0F, 0x5A, 0x0C, 0x24]);
}

#[test]
fn cvtsd2ss_rip_relative_symbol_emits_relocation() {
    let src = Operand::MemoryAddress(MemAddress::symbol("my_float"));
    let info = encode_conversion(ConvOp::Cvtsd2ss, &XMM2, &src, Size::U64);

    assert_eq!(info.data, [0xF2, 0x0F, 0x5A, 0x15, 0x00, 0x00, 0x00, 0x00]);
    assert_eq!(info.relocations.len(), 1);
    assert_eq!(info.relocations[0].sym, "my_float");
    assert_eq!(info.relocations[0].offset, 4);
}

#[test]
fn cvtss2sd_reg() {
    assert_eq!(enc(ConvOp::Cvtss2sd, XMM0, Operand::Reg(XMM1), Size::U64), [0xF3, 0x0F, 0x5A, 0xC1]);
    assert_eq!(enc(ConvOp::Cvtss2sd, XMM8, Operand::Reg(XMM15), Size::U64), [0xF3, 0x45, 0x0F, 0x5A, 0xC7]);
    assert_eq!(enc(ConvOp::Cvtss2sd, XMM9, Operand::Reg(XMM2), Size::U64), [0xF3, 0x44, 0x0F, 0x5A, 0xCA]);
}

#[test]
fn cvtss2sd_mem() {
    assert_eq!(enc(ConvOp::Cvtss2sd, XMM1, mem(at(RBX)), Size::U64), [0xF3, 0x0F, 0x5A, 0x0B]);
    assert_eq!(enc(ConvOp::Cvtss2sd, XMM1, mem(at(RSP)), Size::U64), [0xF3, 0x0F, 0x5A, 0x0C, 0x24]);
}

#[test]
fn cvtss2sd_rip_relative_symbol_emits_relocation() {
    let src = Operand::MemoryAddress(MemAddress::symbol("my_float"));
    let info = encode_conversion(ConvOp::Cvtss2sd, &XMM2, &src, Size::U64);

    assert_eq!(info.data, [0xF3, 0x0F, 0x5A, 0x15, 0x00, 0x00, 0x00, 0x00]);
    assert_eq!(info.relocations[0].offset, 4);
}

#[test]
fn float_to_float_ignores_size() {
    // no REX.W even with a 64-bit default size
    let ins = Instruction::Convert { op: ConvOp::Cvtsd2ss, dst: XMM0, src: Operand::Reg(XMM1), size: None };
    assert_eq!(ins.encode(Size::U64).data, [0xF2, 0x0F, 0x5A, 0xC1]);
    assert_eq!(ins.encode(Size::U32).data, [0xF2, 0x0F, 0x5A, 0xC1]);
}

// ==========================================
// Invalid operands
// ==========================================

#[test]
#[should_panic(expected = "destination must be an XMM register")]
fn gpr_destination_panics() {
    encode(RAX, Operand::Reg(RBX), Size::U64);
}

#[test]
#[should_panic(expected = "source must be a GPR or a memory address")]
fn xmm_source_panics() {
    encode(XMM0, Operand::Reg(XMM1), Size::U64);
}

#[test]
#[should_panic(expected = "source must be a GPR or a memory address")]
fn immediate_source_panics() {
    encode(XMM0, Operand::Imm(42), Size::U64);
}

#[test]
#[should_panic(expected = "integer source must be 32 or 64 bits")]
fn size_u16_panics() {
    encode(XMM0, Operand::Reg(RAX), Size::U16);
}

#[test]
#[should_panic(expected = "integer source must be 32 or 64 bits")]
fn size_u8_panics() {
    encode(XMM0, Operand::Reg(RAX), Size::U8);
}

#[test]
#[should_panic(expected = "destination must be a GPR")]
fn float_to_int_xmm_destination_panics() {
    enc(ConvOp::Cvttsd2si, XMM0, Operand::Reg(XMM1), Size::U64);
}

#[test]
#[should_panic(expected = "source must be an XMM register or a memory address")]
fn float_to_int_gpr_source_panics() {
    enc(ConvOp::Cvttsd2si, RAX, Operand::Reg(RBX), Size::U64);
}

#[test]
#[should_panic(expected = "integer destination must be 32 or 64 bits")]
fn float_to_int_size_u16_panics() {
    enc(ConvOp::Cvtss2si, RAX, Operand::Reg(XMM0), Size::U16);
}

#[test]
#[should_panic(expected = "destination must be an XMM register")]
fn float_to_float_gpr_destination_panics() {
    enc(ConvOp::Cvtsd2ss, RAX, Operand::Reg(XMM1), Size::U64);
}

#[test]
#[should_panic(expected = "source must be an XMM register or a memory address")]
fn float_to_float_gpr_source_panics() {
    enc(ConvOp::Cvtss2sd, XMM0, Operand::Reg(RAX), Size::U64);
}

#[test]
#[should_panic(expected = "source must be an XMM register or a memory address")]
fn float_to_float_immediate_source_panics() {
    enc(ConvOp::Cvtss2sd, XMM0, Operand::Imm(1), Size::U64);
}
