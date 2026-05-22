use crate::elf::{elfsym::make_st_info, instructions::register::*};
mod elf;
use elf::instructions::register as Register;


fn main() -> std::io::Result<()> {
    use std::fs::File;
    use elf::instructions::*;
    let mut file = File::create("output.elf")?;
    let _my_data = Operand::MemoryAddress(MemAddress::new().sym("my_data"));
    let mem = Operand::MemoryAddress(MemAddress::new().base(Register::RBX));
    // =========================================================
    // FUNCTION: my_func
    // =========================================================
    let func_instr: Vec<Instruction> = vec![
        Instruction::Ctrl { op: CtrlOp::Jmp, target: Operand::Sym("test_local2".to_string()) },
        Instruction::Ctrl { op: CtrlOp::Call, target: Operand::Sym("my_func".to_string()) },
        Instruction::Ctrl { op: CtrlOp::Jmp, target: Operand::Sym("my_func".to_string()) },
        Instruction::Ctrl { op: CtrlOp::JmpCC(ConditionCode::E), target: Operand::Sym("my_func".to_string()) },
        Instruction::Ctrl { op: CtrlOp::JmpCC(ConditionCode::NE), target: Operand::Sym("my_func".to_string()) },
        Instruction::Ctrl { op: CtrlOp::JmpCC(ConditionCode::G), target: Operand::Sym("my_func".to_string()) },
        Instruction::Ctrl { op: CtrlOp::JmpCC(ConditionCode::L), target: Operand::Sym("my_func".to_string()) },
        Instruction::Ctrl { op: CtrlOp::JmpCC(ConditionCode::GE), target: Operand::Sym("my_func".to_string()) },
        Instruction::Ctrl { op: CtrlOp::JmpCC(ConditionCode::LE), target: Operand::Sym("my_func".to_string()) },
        Instruction::CMovCC { cc: ConditionCode::E,  dst: Register::RAX, src: Operand::Reg(Register::RBX), size: Some(Size::U32) },
        Instruction::CMovCC { cc: ConditionCode::NE, dst: Register::RCX, src: Operand::Reg(Register::RDX), size: Some(Size::U32) },
        Instruction::CMovCC { cc: ConditionCode::G,  dst: Register::RAX, src: Operand::MemoryAddress(MemAddress::new().base(Register::RBX)), size: Some(Size::U32)},
        Instruction::CMovCC { cc: ConditionCode::L,  dst: Register::RBX, src: Operand::MemoryAddress(MemAddress::new().base(Register::RBX).disp(4)), size: Some(Size::U32) },
        Instruction::CMovCC { cc: ConditionCode::GE, dst: Register::RDX, src: Operand::Reg(Register::RAX), size: Some(Size::U32) },
        Instruction::CMovCC { cc: ConditionCode::LE, dst: Register::RCX, src: Operand::Reg(Register::RBX), size: Some(Size::U32) },
        // =====================================================
        // BIT OPS : reg + imm
        // =====================================================
        Instruction::Bit { op: BitOp::Bt,  dst: Operand::Reg(Register::RAX), src: Operand::Imm(3), size: Some(Size::U32) },
        Instruction::Bit { op: BitOp::Bts, dst: Operand::Reg(Register::RBX), src: Operand::Imm(5), size: Some(Size::U32) },
        Instruction::Bit { op: BitOp::Btr, dst: Operand::Reg(Register::RCX), src: Operand::Imm(7), size: Some(Size::U32) },
        Instruction::Bit { op: BitOp::Btc, dst: Operand::Reg(Register::RDX), src: Operand::Imm(1), size: Some(Size::U32) },
        // =====================================================
        // BIT OPS : reg + reg
        // =====================================================
        Instruction::Bit { op: BitOp::Bt,  dst: Operand::Reg(Register::RAX), src: Operand::Reg(Register::RBX), size: Some(Size::U32) },
        Instruction::Bit { op: BitOp::Bts, dst: Operand::Reg(Register::RCX), src: Operand::Reg(Register::RDX), size: Some(Size::U32) },
        Instruction::Bit { op: BitOp::Btr, dst: Operand::Reg(Register::RBX), src: Operand::Reg(Register::RAX), size: Some(Size::U32) },
        Instruction::Bit { op: BitOp::Btc, dst: Operand::Reg(Register::RDX), src: Operand::Reg(Register::RCX), size: Some(Size::U32) },
        // =====================================================
        // BIT OPS : mem + imm
        // =====================================================
        Instruction::Bit {
            op: BitOp::Bt,
            dst: Operand::MemoryAddress(MemAddress::new().base(Register::RBX).disp(8)),
            src: Operand::Imm(2),
            size: Some(Size::U32),
        },
        Instruction::Bit {
            op: BitOp::Bts,
            dst: Operand::MemoryAddress(MemAddress::new().base(Register::RBX).disp(12)),
            src: Operand::Imm(4),
            size: Some(Size::U32),
        },
        // =====================================================
        // BIT OPS : mem + reg
        // =====================================================
        Instruction::Bit {
            op: BitOp::Btr,
            dst: Operand::MemoryAddress(MemAddress::new().base(Register::RCX).disp(16)),
            src: Operand::Reg(Register::RAX),
            size: Some(Size::U32),
        },
        Instruction::Bit {
            op: BitOp::Btc,
            dst: Operand::MemoryAddress(MemAddress::new().base(Register::RBX).disp(20)),
            src: Operand::Reg(Register::RBX),
            size: Some(Size::U32),
        },
        // Instruction::LocalSym("test_local2".to_string()),
        Instruction::Ctrl { op: CtrlOp::Ret, target: Operand::NoOperand },
    ];
    let unary_instr: Vec<Instruction> = vec![
        Instruction::Unary { op: UnaryOp::Inc, dst: Operand::Reg(Register::RAX), size: Some(Size::U8) },
        Instruction::Unary { op: UnaryOp::Inc, dst: Operand::Reg(Register::RBX), size: Some(Size::U16) },
        Instruction::Unary { op: UnaryOp::Inc, dst: Operand::Reg(Register::RCX), size: Some(Size::U32) },
        Instruction::Unary { op: UnaryOp::Inc, dst: Operand::MemoryAddress(MemAddress::new().base(Register::RBX)), size: Some(Size::U32) },
        Instruction::Unary { op: UnaryOp::Dec, dst: Operand::Reg(Register::RDX), size: Some(Size::U8) },
        Instruction::Unary { op: UnaryOp::Dec, dst: Operand::Reg(Register::RAX), size: Some(Size::U16) },
        Instruction::Unary { op: UnaryOp::Dec, dst: Operand::Reg(Register::RBX), size: Some(Size::U32) },
        Instruction::Unary { op: UnaryOp::Dec, dst: Operand::MemoryAddress(MemAddress::new().base(Register::RCX).disp(4)), size: Some(Size::U32) },
        Instruction::Unary { op: UnaryOp::Neg, dst: Operand::Reg(Register::RCX), size: Some(Size::U8) },
        Instruction::Unary { op: UnaryOp::Neg, dst: Operand::Reg(Register::RDX), size: Some(Size::U16) },
        Instruction::Unary { op: UnaryOp::Neg, dst: Operand::Reg(Register::RAX), size: Some(Size::U32) },
        Instruction::Unary { op: UnaryOp::Neg, dst: Operand::MemoryAddress(MemAddress::new().base(Register::RBX).disp(8)), size: Some(Size::U32) },
        Instruction::Unary { op: UnaryOp::Not, dst: Operand::Reg(Register::RBX), size: Some(Size::U8) },
        Instruction::Unary { op: UnaryOp::Not, dst: Operand::Reg(Register::RCX), size: Some(Size::U16) },
        Instruction::Unary { op: UnaryOp::Not, dst: Operand::Reg(Register::RDX), size: Some(Size::U32) },
        Instruction::Unary { op: UnaryOp::Not, dst: Operand::MemoryAddress(MemAddress::new().base(Register::RAX).disp(12)), size: Some(Size::U32) },
        // =====================================================
        // PUSH reg
        // =====================================================
        Instruction::Stack { op: StackOp::Push, value: Operand::Reg(Register::RAX), size: Some(Size::U32) },
        Instruction::Stack { op: StackOp::Push, value: Operand::Reg(Register::RBX), size: Some(Size::U32) },
        // PUSH imm
        Instruction::Stack { op: StackOp::Push, value: Operand::Imm(5), size: Some(Size::U32) },
        Instruction::Stack { op: StackOp::Push, value: Operand::Imm(0x12345678), size: Some(Size::U32) },
        // PUSH symbol
        // Instruction::Stack { op: StackOp::Push, value: Operand::Sym("my_data".to_string()), size: Some(Size::U32) },
        // PUSH memory
        Instruction::Stack { op: StackOp::Push, value: Operand::MemoryAddress(MemAddress::new().base(Register::RCX).disp(16)), size: Some(Size::U32), },
        // =====================================================
        // POP reg
        // =====================================================
        Instruction::Stack { op: StackOp::Pop, value: Operand::Reg(Register::RCX), size: Some(Size::U32) },
        Instruction::Stack { op: StackOp::Pop, value: Operand::Reg(Register::RDX), size: Some(Size::U32) },
        // POP memory
        Instruction::Stack {
            op: StackOp::Pop,
            value: Operand::MemoryAddress(MemAddress::new().base(Register::RAX).disp(20)),
            size: Some(Size::U32),
        },
        Instruction::Ctrl { op: CtrlOp::Ret, target: Operand::NoOperand },
    ];

    let setcc_instr: Vec<Instruction> = vec![
        Instruction::Binary { op: BinOp::Cmp, dst: Operand::Reg(Register::RAX), src: Operand::Reg(Register::RBX), size: Some(Size::U32) },
        Instruction::SetCC { op: ConditionCode::E, dst: Operand::Reg(Register::RAX) }, Instruction::SetCC { op: ConditionCode::NE, dst: Operand::Reg(Register::RBX) }, Instruction::SetCC { op: ConditionCode::G, dst: Operand::Reg(Register::RCX) }, Instruction::SetCC { op: ConditionCode::L, dst: Operand::Reg(Register::RDX) }, Instruction::SetCC { op: ConditionCode::GE, dst: Operand::Reg(Register::RAX) }, Instruction::SetCC { op: ConditionCode::LE, dst: Operand::Reg(Register::RBX) }, Instruction::SetCC { op: ConditionCode::A, dst: Operand::Reg(Register::RCX) }, Instruction::SetCC { op: ConditionCode::B, dst: Operand::Reg(Register::RDX) },
        Instruction::SetCC { op: ConditionCode::AE, dst: Operand::Reg(Register::RAX) },
        Instruction::SetCC { op: ConditionCode::BE, dst: Operand::Reg(Register::RBX) }, 
        Instruction::SetCC { op: ConditionCode::S, dst: Operand::Reg(Register::RCX) },
        Instruction::SetCC { op: ConditionCode::NS, dst: Operand::Reg(Register::RDX) },
        Instruction::SetCC { op: ConditionCode::O, dst: Operand::Reg(Register::RAX) },
        Instruction::SetCC { op: ConditionCode::NO, dst: Operand::Reg(Register::RBX) },
        Instruction::SetCC { op: ConditionCode::P, dst: Operand::Reg(Register::RCX) },
        Instruction::SetCC { op: ConditionCode::NP, dst: Operand::Reg(Register::RDX) },
        Instruction::SetCC { op: ConditionCode::E, dst: Operand::MemoryAddress(MemAddress::new().base(Register::RBX).disp(0))},
        Instruction::SetCC { op: ConditionCode::NE, dst: Operand::MemoryAddress(MemAddress::new().base(Register::RBX).disp(1))},
        Instruction::SetCC { op: ConditionCode::G, dst: Operand::MemoryAddress(MemAddress::new().base(Register::RBX).disp(2))},
        Instruction::SetCC { op: ConditionCode::L, dst: Operand::MemoryAddress(MemAddress::new().base(Register::RBX).disp(3))},
        Instruction::SetCC { op: ConditionCode::GE, dst: Operand::MemoryAddress(MemAddress::new().base(Register::RBX).disp(4))},
        Instruction::SetCC { op: ConditionCode::LE, dst: Operand::MemoryAddress(MemAddress::new().base(Register::RBX).disp(5))},
        Instruction::SetCC { op: ConditionCode::A, dst: Operand::MemoryAddress(MemAddress::new().base(Register::RBX).disp(6))},
        Instruction::SetCC { op: ConditionCode::B, dst: Operand::MemoryAddress(MemAddress::new().base(Register::RBX).disp(7))},
        Instruction::SetCC { op: ConditionCode::AE, dst: Operand::MemoryAddress(MemAddress::new().base(Register::RBX).disp(8))},
        Instruction::SetCC { op: ConditionCode::BE, dst: Operand::MemoryAddress(MemAddress::new().base(Register::RBX).disp(9))},
        Instruction::SetCC { op: ConditionCode::S, dst: Operand::MemoryAddress(MemAddress::new().base(Register::RBX).disp(10))},
        Instruction::SetCC { op: ConditionCode::NS, dst: Operand::MemoryAddress(MemAddress::new().base(Register::RBX).disp(11))},
        Instruction::SetCC { op: ConditionCode::O, dst: Operand::MemoryAddress(MemAddress::new().base(Register::RBX).disp(12))},
        Instruction::SetCC { op: ConditionCode::NO, dst: Operand::MemoryAddress(MemAddress::new().base(Register::RBX).disp(13))},
        Instruction::SetCC { op: ConditionCode::P, dst: Operand::MemoryAddress(MemAddress::new().base(Register::RBX).disp(14))},
        Instruction::SetCC { op: ConditionCode::NP, dst: Operand::MemoryAddress(MemAddress::new().base(Register::RBX).disp(15))},
        Instruction::Ctrl { op: CtrlOp::Ret, target: Operand::NoOperand },
    ];
    // =========================================================
    // _start : FULL BINOP TEST SUITE
    // =========================================================
    let start_instr: Vec<Instruction> = vec![
        Instruction::Binary { op: BinOp::LoadF, dst: Operand::Reg(XMM0), src: Operand::MemoryAddress(MemAddress::symbol("my_float")), size: None },
        Instruction::Binary { op: BinOp::LoadF, dst: Operand::Reg(XMM1), src: Operand::Reg(XMM0), size: None },
        Instruction::Binary { op: BinOp::StoreF, dst: Operand::Reg(XMM0), src: Operand::MemoryAddress(MemAddress::symbol("my_float")), size: None },
        Instruction::Binary { op: BinOp::AddF, dst: Operand::Reg(XMM0), src: Operand::MemoryAddress(MemAddress::symbol("my_float")), size: None },
        Instruction::Binary { op: BinOp::AddF, dst: Operand::Reg(XMM1), src: Operand::Reg(XMM0), size: None },
        Instruction::Binary { op: BinOp::SubF, dst: Operand::Reg(XMM2), src: Operand::Reg(XMM1), size: None },
        Instruction::Binary { op: BinOp::SubF, dst: Operand::Reg(XMM2), src: Operand::MemoryAddress(MemAddress::symbol("my_float")), size: None },
        Instruction::Binary { op: BinOp::MulF, dst: Operand::Reg(XMM3), src: Operand::Reg(XMM2), size: None },
        Instruction::Binary { op: BinOp::MulF, dst: Operand::Reg(XMM3), src: Operand::MemoryAddress(MemAddress::symbol("my_float")), size: None },
        Instruction::Binary { op: BinOp::DivF, dst: Operand::Reg(XMM4), src: Operand::Reg(XMM3), size: None },
        Instruction::Binary { op: BinOp::DivF, dst: Operand::Reg(XMM4), src: Operand::MemoryAddress(MemAddress::symbol("my_float")), size: None },
        Instruction::Binary { op: BinOp::ComiF, dst: Operand::Reg(XMM5), src: Operand::Reg(XMM4), size: None },
        Instruction::Binary { op: BinOp::ComiF, dst: Operand::Reg(XMM5), src: Operand::MemoryAddress(MemAddress::symbol("my_float")), size: None },
        Instruction::Binary { op: BinOp::UcomiF, dst: Operand::Reg(XMM6), src: Operand::Reg(XMM5), size: None },
        Instruction::Binary { op: BinOp::UcomiF, dst: Operand::Reg(XMM6), src: Operand::MemoryAddress(MemAddress::symbol("my_float")), size: None },
        
        Instruction::Binary { op: BinOp::LoadF, dst: Operand::Reg(XMM0), src: Operand::MemoryAddress(MemAddress::symbol("my_float")), size: Some(Size::U32) },
        Instruction::Binary { op: BinOp::StoreF , dst: Operand::Reg(XMM0), src: Operand::MemoryAddress(MemAddress::symbol("my_float")), size: Some(Size::U32) },
        Instruction::Binary { op: BinOp::LoadF, dst: Operand::Reg(XMM1), src: Operand::Reg(XMM0), size: Some(Size::U32) },
        Instruction::Binary { op: BinOp::AddF, dst: Operand::Reg(XMM0), src: Operand::MemoryAddress(MemAddress::symbol("my_float")), size: Some(Size::U32) },
        Instruction::Binary { op: BinOp::AddF, dst: Operand::Reg(XMM1), src: Operand::Reg(XMM0), size: Some(Size::U32) },
        Instruction::Binary { op: BinOp::SubF, dst: Operand::Reg(XMM2), src: Operand::Reg(XMM1), size: Some(Size::U32) },
        Instruction::Binary { op: BinOp::SubF, dst: Operand::Reg(XMM2), src: Operand::MemoryAddress(MemAddress::symbol("my_float")), size: Some(Size::U32) },
        Instruction::Binary { op: BinOp::MulF, dst: Operand::Reg(XMM3), src: Operand::Reg(XMM2), size: Some(Size::U32) },
        Instruction::Binary { op: BinOp::MulF, dst: Operand::Reg(XMM3), src: Operand::MemoryAddress(MemAddress::symbol("my_float")), size: Some(Size::U32) },
        Instruction::Binary { op: BinOp::DivF, dst: Operand::Reg(XMM4), src: Operand::Reg(XMM3), size: Some(Size::U32) },
        Instruction::Binary { op: BinOp::DivF, dst: Operand::Reg(XMM4), src: Operand::MemoryAddress(MemAddress::symbol("my_float")), size: Some(Size::U32) },
        Instruction::Binary { op: BinOp::ComiF, dst: Operand::Reg(XMM5), src: Operand::Reg(XMM4), size: Some(Size::U32) },
        Instruction::Binary { op: BinOp::ComiF, dst: Operand::Reg(XMM5), src: Operand::MemoryAddress(MemAddress::symbol("my_float")), size: Some(Size::U32) },
        Instruction::Binary { op: BinOp::UcomiF, dst: Operand::Reg(XMM6), src: Operand::Reg(XMM5), size: Some(Size::U32) },
        Instruction::Binary { op: BinOp::UcomiF, dst: Operand::Reg(XMM6), src: Operand::MemoryAddress(MemAddress::symbol("my_float")), size: Some(Size::U32) },
        Instruction::Nop(1),
        Instruction::Nop(2),
        Instruction::Nop(3),
        Instruction::Nop(4),
        Instruction::Nop(5),
        Instruction::Nop(6),
        Instruction::Nop(7),
        Instruction::Nop(8),
        Instruction::Nop(9),
        Instruction::Stack { op: StackOp::Enter(0), value: Operand::Imm(32), size: None, },
        Instruction::Stack { op: StackOp::Leave, value: Operand::NoOperand, size: None, },

        Instruction::Prefix {
            prefix: vec![Prefix::Cs, Prefix::Ds, Prefix::Es, Prefix::Ss, Prefix::Gs, Prefix::Fs,],
            ins: Box::new(
                Instruction::Binary {
                    op: BinOp::Mov,
                    dst: Operand::Reg(Register::RAX),
                    src: Operand::MemoryAddress(MemAddress::new().disp(0x28)),
                    size: Some(Size::U64),
                }
            ),
        },

        // ========================================================
        // XCHG TEST
        // ========================================================

        // eax = 123
        Instruction::Binary {
            op: BinOp::Mov,
            dst: Operand::Reg(Register::RAX),
            src: Operand::Imm(123),
            size: Some(Size::U32),
        },

        // ebx = 456
        Instruction::Binary {
            op: BinOp::Mov,
            dst: Operand::Reg(Register::RBX),
            src: Operand::Imm(456),
            size: Some(Size::U32),
        },

        // xchg eax, ebx
        //
        // after:
        //   eax = 456
        //   ebx = 123
        //
        Instruction::Binary {
            op: BinOp::Xchg,
            dst: Operand::Reg(Register::RAX),
            src: Operand::Reg(Register::RBX),
            size: Some(Size::U32),
        },

        // ========================================================
        // SIMPLE MEMORY XCHG
        // ========================================================

        // eax = 999
        Instruction::Binary {
            op: BinOp::Mov,
            dst: Operand::Reg(Register::RAX),
            src: Operand::Imm(999),
            size: Some(Size::U32),
        },

        // xchg [my_value], eax
        //
        // memory gets eax
        // eax gets old memory value
        //
        Instruction::Binary {
            op: BinOp::Xchg,
            dst: Operand::MemoryAddress(
                MemAddress::symbol("my_data")
            ),
            src: Operand::Reg(Register::RAX),
            size: Some(Size::U32),
        },

        // ========================================================
        // CMPXCHG SUCCESS
        // ========================================================

        // eax = expected old value
        //
        // cmpxchg compares:
        //   eax vs [my_lock]
        //
        // if equal:
        //   [my_lock] = ecx
        //   ZF = 1
        //
        // else:
        //   eax = [my_lock]
        //   ZF = 0
        //

        // eax = 0
        Instruction::Binary {
            op: BinOp::Mov,
            dst: Operand::Reg(Register::RAX),
            src: Operand::Imm(0),
            size: Some(Size::U32),
        },

        // ecx = 1
        Instruction::Binary {
            op: BinOp::Mov,
            dst: Operand::Reg(Register::RCX),
            src: Operand::Imm(1),
            size: Some(Size::U32),
        },
        
        // lock cmpxchg [my_lock], ecx
        //
        // if my_lock == eax (0):
        //     my_lock = ecx (1)
        //     ZF = 1
        //
        Instruction::Prefix {
            prefix: vec![Prefix::Lock],
            ins: Box::new(
                Instruction::ComplexBinary {
                    op: ComplexBinOp::Cmpxchg,
                    dst: Operand::MemoryAddress(
                        MemAddress::symbol("my_lock")
                    ),
                    src: Operand::Reg(Register::RCX),
                    extra: None,
                    size: Some(Size::U32),
                }
            ),
        },

        // ========================================================
        // CMPXCHG FAILURE
        // ========================================================

        // eax = 0 again
        Instruction::Binary {
            op: BinOp::Mov,
            dst: Operand::Reg(Register::RAX),
            src: Operand::Imm(0),
            size: Some(Size::U32),
        },

        // edx = 2
        Instruction::Binary {
            op: BinOp::Mov,
            dst: Operand::Reg(Register::RDX),
            src: Operand::Imm(2),
            size: Some(Size::U32),
        },

        // lock cmpxchg [my_lock], edx
        //
        // now my_lock is already 1
        //
        // compare:
        //   eax (0) vs my_lock (1)
        //
        // fail:
        //   eax = my_lock
        //   ZF = 0
        //
        Instruction::Prefix {
            prefix: vec![Prefix::Lock],
            ins: Box::new(
                Instruction::ComplexBinary {
                    op: ComplexBinOp::Cmpxchg,
                    dst: Operand::MemoryAddress(
                        MemAddress::symbol("my_lock")
                    ),
                    src: Operand::Reg(Register::RDX),
                    extra: None,
                    size: Some(Size::U32),
                }
            ),
        },

        // -------------------------------------------------
        // Syscalls
        // -------------------------------------------------
        Instruction::Binary { op: BinOp::Mov, dst: Operand::Reg(Register::RAX), src: Operand::Imm(60), size: None },
        Instruction::Binary { op: BinOp::Mov, dst: Operand::Reg(Register::RDI), src: Operand::Imm(42), size: None },

        Instruction::Sys { op: SysOp::Syscall },

        Instruction::Binary { op: BinOp::Mov, dst: Operand::Reg(Register::RAX), src: Operand::Imm(1), size: None },
        Instruction::Binary { op: BinOp::Mov, dst: Operand::Reg(Register::RBX), src: Operand::Imm(42), size: None },

        Instruction::Sys { op: SysOp::Int(0x80) },
        Instruction::Sys { op: SysOp::Sysenter },

        // -------------------------------------------------
        // MOV
        // -------------------------------------------------
        Instruction::Binary { op: BinOp::Mov, dst: Operand::Reg(Register::R8), src: Operand::Imm(0x12), size: Some(Size::U8), },
        Instruction::Binary { op: BinOp::Mov, dst: Operand::Reg(Register::R9), src: Operand::Imm(0x1234), size: Some(Size::U16), },
        Instruction::Binary { op: BinOp::Mov, dst: Operand::Reg(Register::R10), src: Operand::Imm(0x12345678), size: Some(Size::U32), },
        Instruction::Binary { op: BinOp::Mov, dst: Operand::Reg(Register::R11), src: Operand::Reg(Register::RBX), size: Some(Size::U32), },
        Instruction::Binary { op: BinOp::Mov, dst: Operand::Reg(Register::R12), src: Operand::Sym("my_data".to_string()), size: Some(Size::U32), },
        Instruction::Binary { op: BinOp::Mov, dst: Operand::MemoryAddress(MemAddress::new().sym("my_data")), src: Operand::Imm(0x41), size: Some(Size::U8), },
        Instruction::Binary { op: BinOp::Mov, dst: Operand::MemoryAddress(MemAddress::new().sym("my_data")), src: Operand::Reg(Register::RAX), size: Some(Size::U32), },
        Instruction::Binary { op: BinOp::Mov, dst: Operand::Reg(Register::R13), src: Operand::MemoryAddress(MemAddress::new().sym("my_data")), size: Some(Size::U32), },
        Instruction::Binary { op: BinOp::Mov, dst: Operand::Reg(Register::R14), src: Operand::MemoryAddress(MemAddress::new().sym("my_data")), size: Some(Size::U32), },
        Instruction::Binary { op: BinOp::Mov, dst: Operand::MemoryAddress(MemAddress::new().sym("my_data")), src: Operand::Reg(Register::R15), size: Some(Size::U32), },
        Instruction::Binary {
            op: BinOp::Mov,
            dst: Operand::Reg(Register::RAX),
            src: Operand::MemoryAddress(MemAddress::new().base(Register::RBX).index(Register::RCX)),
            size: Some(Size::U32),
        },
        Instruction::Binary {
            op: BinOp::Mov,
            dst: Operand::Reg(Register::RAX),
            src: Operand::MemoryAddress(MemAddress::new().base(Register::RBX).index(Register::RCX).scale(Scale::Four)),
            size: Some(Size::U32),
        },
        Instruction::Binary {
            op: BinOp::Mov,
            dst: Operand::MemoryAddress(MemAddress::new().base(Register::RBX).index(Register::RCX).scale(Scale::Eight).disp(16)),
            src: Operand::Reg(Register::RAX),
            size: Some(Size::U32),
        },
        Instruction::Binary {
            op: BinOp::Movzx,
            dst: Operand::Reg(Register::RAX),
            src: Operand::Reg(Register::RBX),
            size: Some(Size::U8),
        },
        Instruction::Binary {
            op: BinOp::Movsx,
            dst: Operand::Reg(Register::RCX),
            src: Operand::MemoryAddress(MemAddress::new().base(Register::RBX)),
            size: Some(Size::U8),
        },
        Instruction::Binary {
            op: BinOp::Mov,
            dst: Operand::Reg(Register::RAX),
            src: Operand::Imm(0x123456789abcdef0),
            size: None,
        },
        Instruction::Binary {
            op: BinOp::Mov,
            dst: Operand::Reg(Register::RDX),
            src: Operand::Imm(0x401000),
            size: None,
        },
        // -------------------------------------------------
        // MOVZX / MOVSX
        // -------------------------------------------------
        Instruction::Binary { op: BinOp::Movzx, dst: Operand::Reg(Register::RAX), src: Operand::Reg(Register::R10), size: Some(Size::U8) },
        Instruction::Binary { op: BinOp::Movzx, dst: Operand::Reg(Register::RCX), src: mem.clone(), size: Some(Size::U8) },
        Instruction::Binary { op: BinOp::Movsx, dst: Operand::Reg(Register::RDX), src: Operand::Reg(Register::RAX), size: Some(Size::U8) },
        Instruction::Binary { op: BinOp::Movsx, dst: Operand::Reg(Register::RBX), src: mem.clone(), size: Some(Size::U16) },
        // -------------------------------------------------
        // XCHG
        // -------------------------------------------------
        Instruction::Binary { op: BinOp::Xchg, dst: Operand::Reg(Register::R11), src: Operand::Reg(Register::RBX), size: Some(Size::U32) },
        Instruction::Binary { op: BinOp::Xchg, dst: Operand::Reg(Register::RCX), src: mem.clone(), size: Some(Size::U32) },
        // -------------------------------------------------
        // ADD
        // -------------------------------------------------
        Instruction::Binary { op: BinOp::Add, dst: Operand::Reg(Register::RAX), src: Operand::Imm(5), size: Some(Size::U32) },
        Instruction::Binary { op: BinOp::Add, dst: Operand::Reg(Register::RBX), src: Operand::Reg(Register::RAX), size: Some(Size::U32) },
        Instruction::Binary { op: BinOp::Add, dst: mem.clone(), src: Operand::Imm(3), size: Some(Size::U32) },

        Instruction::Binary { op: BinOp::Adc, dst: Operand::Reg(Register::RAX), src: Operand::Imm(5), size: Some(Size::U32) },
        Instruction::Binary { op: BinOp::Adc, dst: Operand::Reg(Register::RBX), src: Operand::Reg(Register::RAX), size: Some(Size::U32) },
        Instruction::Binary { op: BinOp::Adc, dst: mem.clone(), src: Operand::Imm(3), size: Some(Size::U32) },
        Instruction::Binary { op: BinOp::Sbb, dst: Operand::Reg(Register::RAX), src: Operand::Imm(5), size: Some(Size::U32) },
        Instruction::Binary { op: BinOp::Sbb, dst: Operand::Reg(Register::RBX), src: Operand::Reg(Register::RAX), size: Some(Size::U32) },
        Instruction::Binary { op: BinOp::Sbb, dst: mem.clone(), src: Operand::Imm(3), size: Some(Size::U32) },
        // ===== BSF =====
        Instruction::BitScan { op: BitScanOp::Bsf, dst: Register::RAX, src: Operand::Reg(Register::RBX), size: Some(Size::U64), },
        Instruction::BitScan { op: BitScanOp::Bsf, dst: Register::RCX, src: Operand::Reg(Register::RDX), size: Some(Size::U64), },
        Instruction::BitScan { op: BitScanOp::Bsf, dst: Register::RAX, src: Operand::Reg(Register::RBX), size: Some(Size::U32), },
        Instruction::BitScan { op: BitScanOp::Bsf, dst: Register::RAX, src: Operand::Reg(Register::RBX), size: Some(Size::U16), },
        // mémoire
        Instruction::BitScan {
            op: BitScanOp::Bsf,
            dst: Register::RAX,
            src: Operand::MemoryAddress(MemAddress::new().base(Register::RBX)),
            size: Some(Size::U64),
        },
        // ===== BSR =====
        Instruction::BitScan { op: BitScanOp::Bsr, dst: Register::RAX, src: Operand::Reg(Register::RBX), size: Some(Size::U64), },
        Instruction::BitScan { op: BitScanOp::Bsr, dst: Register::RCX, src: Operand::Reg(Register::RDX), size: Some(Size::U64), },
        Instruction::BitScan { op: BitScanOp::Bsr, dst: Register::RAX, src: Operand::Reg(Register::RBX), size: Some(Size::U32), },
        Instruction::BitScan { op: BitScanOp::Bsr, dst: Register::RAX, src: Operand::Reg(Register::RBX), size: Some(Size::U16), },
        // mémoire
        Instruction::BitScan {
            op: BitScanOp::Bsr,
            dst: Register::RAX,
            src: Operand::MemoryAddress(MemAddress::new().base(Register::RBX).index(Register::RCX)),
            size: Some(Size::U64),
        },
        Instruction::Stack { op: StackOp::Pushf, value: Operand::NoOperand, size: Some(Size::U16) },
        Instruction::Stack { op: StackOp::Popf, value: Operand::NoOperand, size: Some(Size::U16) },
    
        Instruction::Stack { op: StackOp::Pushf, value: Operand::NoOperand, size: Some(Size::U32) },
        Instruction::Stack { op: StackOp::Popf, value: Operand::NoOperand, size: Some(Size::U32) },
        Instruction::Stack { op: StackOp::Pushf, value: Operand::NoOperand, size: None },
        Instruction::Stack { op: StackOp::Popf, value: Operand::NoOperand, size: None },
        Instruction::Unary { op: UnaryOp::Cli, dst: Operand::NoOperand, size: None, },
        Instruction::Unary { op: UnaryOp::Sti, dst: Operand::NoOperand, size: None, },
        Instruction::Unary { op: UnaryOp::Lahf, dst: Operand::NoOperand, size: None, },
        Instruction::Unary { op: UnaryOp::Sahf, dst: Operand::NoOperand, size: None, },
        Instruction::Unary { op: UnaryOp::Cbw, dst: Operand::NoOperand, size: None },
        Instruction::Unary { op: UnaryOp::Cwde, dst: Operand::NoOperand, size: None },
        Instruction::Unary { op: UnaryOp::Cdqe, dst: Operand::NoOperand, size: None },
        // -------------------------------------------------
        // SUB
        // -------------------------------------------------
        Instruction::Binary { op: BinOp::Sub, dst: Operand::Reg(Register::RCX), src: Operand::Imm(10), size: Some(Size::U32) },
        Instruction::Binary { op: BinOp::Sub, dst: Operand::Reg(Register::RDX), src: Operand::Reg(Register::RCX), size: Some(Size::U32) },
        Instruction::Binary { op: BinOp::Sub, dst: mem.clone(), src: Operand::Imm(1), size: Some(Size::U32) },
        // -------------------------------------------------
        // AND
        // -------------------------------------------------
        Instruction::Binary { op: BinOp::And, dst: Operand::Reg(Register::RAX), src: Operand::Imm(0xFF), size: Some(Size::U32) },
        Instruction::Binary { op: BinOp::And, dst: Operand::Reg(Register::RBX), src: Operand::Reg(Register::RAX), size: Some(Size::U32) },
        Instruction::Binary { op: BinOp::And, dst: mem.clone(), src: Operand::Reg(Register::RBX), size: Some(Size::U32) },
        // -------------------------------------------------
        // OR
        // -------------------------------------------------
        Instruction::Binary { op: BinOp::Or, dst: Operand::Reg(Register::RCX), src: Operand::Imm(0x10), size: Some(Size::U32) },
        Instruction::Binary { op: BinOp::Or, dst: Operand::Reg(Register::RDX), src: Operand::Reg(Register::RCX), size: Some(Size::U32) },
        // -------------------------------------------------
        // XOR
        // -------------------------------------------------
        Instruction::Binary { op: BinOp::Xor, dst: Operand::Reg(Register::RAX), src: Operand::Imm(0xFF), size: Some(Size::U32) },
        Instruction::Binary { op: BinOp::Xor, dst: Operand::Reg(Register::RBX), src: Operand::Reg(Register::RAX), size: Some(Size::U32) },
        Instruction::Binary { op: BinOp::Xor, dst: Operand::Reg(Register::RCX), src: mem.clone(), size: Some(Size::U32) },
        // -------------------------------------------------
        // CMP
        // -------------------------------------------------
        Instruction::Binary { op: BinOp::Cmp, dst: Operand::Reg(Register::RAX), src: Operand::Imm(42), size: Some(Size::U32) },
        Instruction::Binary { op: BinOp::Cmp, dst: Operand::Reg(Register::RBX), src: Operand::Reg(Register::RAX), size: Some(Size::U32) },
        Instruction::Binary { op: BinOp::Cmp, dst: Operand::Reg(Register::RCX), src: mem.clone(), size: Some(Size::U32) },
        // -------------------------------------------------
        // TEST
        // -------------------------------------------------
        Instruction::Binary { op: BinOp::Test, dst: Operand::Reg(Register::RAX), src: Operand::Imm(1), size: Some(Size::U32) },
        Instruction::Binary { op: BinOp::Test, dst: Operand::Reg(Register::RBX), src: Operand::Reg(Register::RAX), size: Some(Size::U32) },
        Instruction::Binary { op: BinOp::Test, dst: mem.clone(), src: Operand::Imm(0xFF), size: Some(Size::U32) },
        // -------------------------------------------------
        // SHL / SHR / SAR
        // -------------------------------------------------
        Instruction::Shift{ op: ShiftOp::Shl, dst: Operand::Reg(Register::RAX), src: Operand::Imm(1), size: Some(Size::U32) },
        Instruction::Shift{ op: ShiftOp::Shl, dst: Operand::Reg(Register::RBX), src: Operand::Imm(3), size: Some(Size::U32) },
        Instruction::Shift{ op: ShiftOp::Shl, dst: Operand::Reg(Register::RCX), src: Operand::Reg(Register::RCX), size: Some(Size::U32) },
        Instruction::Shift{ op: ShiftOp::Shr, dst: Operand::Reg(Register::RDX), src: Operand::Imm(1), size: Some(Size::U32) },
        Instruction::Shift{ op: ShiftOp::Shr, dst: mem.clone(), src: Operand::Imm(2), size: Some(Size::U32) },
        Instruction::Shift{ op: ShiftOp::Sar, dst: Operand::Reg(Register::RAX), src: Operand::Imm(1), size: Some(Size::U32) },
        Instruction::Shift{ op: ShiftOp::Sar, dst: mem.clone(), src: Operand::Reg(Register::RCX), size: Some(Size::U32) },
        // -------------------------------------------------
        // ROL / ROR
        // -------------------------------------------------
        Instruction::Shift { op: ShiftOp::Rol, dst: Operand::Reg(Register::RBX), src: Operand::Imm(1), size: Some(Size::U32) },
        Instruction::Shift { op: ShiftOp::Rol, dst: Operand::Reg(Register::RCX), src: Operand::Imm(4), size: Some(Size::U32) },
        Instruction::Shift { op: ShiftOp::Ror, dst: Operand::Reg(Register::RDX), src: Operand::Imm(1), size: Some(Size::U32) },
        Instruction::Shift { op: ShiftOp::Ror, dst: mem.clone(), src: Operand::Reg(Register::RCX), size: Some(Size::U32) },
        // -------------------------------------------------
        // Labels / Calls
        // -------------------------------------------------
        // Instruction::LocalSym("test_local".to_string()),
        // Instruction::Ctrl { op: CtrlOp::Call, target: Operand::Sym("my_func".to_string()) },
        Instruction::Ctrl { op: CtrlOp::Call, target: Operand::Sym("my_exit".to_string()) },

        Instruction::Lea { dst: Register::RAX, src: Operand::MemoryAddress(MemAddress::new().base(Register::RBX)), size: Some(Size::U32), },
        Instruction::Lea { dst: Register::RAX, src: Operand::MemoryAddress(MemAddress::new().base(Register::RBX).disp(8)), size: Some(Size::U32) },
        Instruction::Lea { dst: Register::RAX, src: Operand::MemoryAddress(MemAddress::new().base(Register::RBP).disp(-16)), size: Some(Size::U32) },
        Instruction::Lea { dst: Register::RAX, src: Operand::MemoryAddress(MemAddress::new().base(Register::RCX).index_scale(Register::RCX, Scale::Four)), size: Some(Size::U32) },
        Instruction::Lea { dst: Register::RAX, src: Operand::MemoryAddress(MemAddress::symbol("my_data")), size: None },
        Instruction::Lea { dst: Register::RAX, src: Operand::MemoryAddress(MemAddress::new().base(Register::RBX)), size: Some(Size::U32) },
        Instruction::Lea { dst: Register::RAX, src: Operand::MemoryAddress(MemAddress::new().base(Register::RBX).disp(8)), size: Some(Size::U32) },
        Instruction::Lea { dst: Register::RAX, src: Operand::MemoryAddress(MemAddress::new().base(Register::RBP).disp(-16)), size: Some(Size::U32) },
        Instruction::Lea { dst: Register::RAX, src: Operand::MemoryAddress(MemAddress::new().base(Register::RCX).index_scale(Register::RCX, Scale::Four)), size: Some(Size::U32) },
    
        Instruction::ComplexBinary { op: ComplexBinOp::Div, dst: Operand::NoOperand, src: Operand::Reg(Register::RCX), extra: None, size: Some(Size::U32), },
        Instruction::ComplexBinary { op: ComplexBinOp::Div, dst: Operand::NoOperand, src: Operand::MemoryAddress(MemAddress::new().base(Register::RBX).disp(8)), extra: None, size: Some(Size::U32) },
        Instruction::ComplexBinary { op: ComplexBinOp::Div, dst: Operand::NoOperand, src: Operand::MemoryAddress(MemAddress::new().base(Register::RBP).disp(-16)), extra: None, size: Some(Size::U32) },
        Instruction::ComplexBinary { op: ComplexBinOp::Div, dst: Operand::NoOperand, src: Operand::MemoryAddress(MemAddress::new().base(Register::RBX).index(Register::RCX)), extra: None, size: Some(Size::U32) },
        Instruction::ComplexBinary { op: ComplexBinOp::Div, dst: Operand::NoOperand, src: Operand::MemoryAddress(MemAddress::new().base(Register::RBX).index_scale(Register::RCX, Scale::Four)), extra: None, size: Some(Size::U32) },
        Instruction::ComplexBinary { op: ComplexBinOp::Div, dst: Operand::NoOperand, src: Operand::MemoryAddress(MemAddress::new().base(Register::RBX).index_scale(Register::RCX, Scale::Eight).disp(16)), extra: None, size: Some(Size::U32) },
        Instruction::ComplexBinary { op: ComplexBinOp::Div, dst: Operand::NoOperand, src: Operand::Reg(Register::RBX), extra: None, size: None },
        Instruction::ComplexBinary { op: ComplexBinOp::Div, dst: Operand::NoOperand, src: Operand::MemoryAddress(MemAddress::new().base(Register::RBX)), extra: None, size: None },

        Instruction::ComplexBinary { op: ComplexBinOp::Idiv, dst: Operand::NoOperand, src: Operand::Reg(Register::RBX), extra: None, size: Some(Size::U32) },
        Instruction::ComplexBinary { op: ComplexBinOp::Idiv, dst: Operand::NoOperand, src: Operand::MemoryAddress(MemAddress::new().base(Register::RBX).disp(4)), extra: None, size: Some(Size::U32) },

        Instruction::ComplexBinary { op: ComplexBinOp::Mul, dst: Operand::NoOperand, src: Operand::Reg(Register::RBX), extra: None, size: Some(Size::U32) },
        Instruction::ComplexBinary { op: ComplexBinOp::Mul, dst: Operand::NoOperand, src: Operand::MemoryAddress(MemAddress::new().base(Register::RBX)), extra: None, size: Some(Size::U32) },

        Instruction::ComplexBinary { op: ComplexBinOp::Imul, dst: Operand::NoOperand, src: Operand::Reg(Register::RBX), extra: None, size: Some(Size::U32) },
        Instruction::ComplexBinary { op: ComplexBinOp::Imul, dst: Operand::NoOperand, src: Operand::MemoryAddress(MemAddress::new().base(Register::RBX).index_scale(Register::RCX, Scale::Two)), extra: None, size: None },
        Instruction::ComplexBinary { op: ComplexBinOp::Imul, dst: Operand::Reg(Register::RAX), src: Operand::Reg(Register::RBX), extra: None, size: Some(Size::U32) },
        Instruction::ComplexBinary { op: ComplexBinOp::Imul, dst: Operand::Reg(Register::RCX), src: Operand::MemoryAddress(MemAddress::new().base(Register::RBX)), extra: None, size: Some(Size::U32) },
        Instruction::ComplexBinary { op: ComplexBinOp::Imul, dst: Operand::Reg(Register::RDX), src: Operand::MemoryAddress(MemAddress::new().base(Register::RBP).disp(-8)), extra: None, size: None },
        Instruction::ComplexBinary { op: ComplexBinOp::Imul, dst: Operand::Reg(Register::RAX), src: Operand::MemoryAddress(MemAddress::new().base(Register::RBX).index_scale(Register::RCX, Scale::Four)), extra: None, size: Some(Size::U32) },
        Instruction::ComplexBinary { op: ComplexBinOp::Imul, dst: Operand::Reg(Register::RAX), src: Operand::Reg(Register::RBX), extra: Some(Operand::Imm(-255)), size: Some(Size::U32), },
        Instruction::ComplexBinary { op: ComplexBinOp::Imul, dst: Operand::Reg(Register::RCX), src: Operand::Reg(Register::RDX), extra: Some(Operand::Imm(127)), size: Some(Size::U32), },
        Instruction::ComplexBinary { op: ComplexBinOp::Imul, dst: Operand::Reg(Register::RAX), src: Operand::Reg(Register::RBX), extra: Some(Operand::Imm(128)), size: Some(Size::U32), },
        Instruction::ComplexBinary { op: ComplexBinOp::Imul, dst: Operand::Reg(Register::RDX), src: Operand::MemoryAddress(MemAddress::new().base(Register::RBX)), extra: Some(Operand::Imm(1000)), size: Some(Size::U32) },
        Instruction::ComplexBinary { op: ComplexBinOp::Imul, dst: Operand::Reg(Register::RAX), src: Operand::MemoryAddress(MemAddress::new().base(Register::RBX).index_scale(Register::RCX, Scale::Eight).disp(16)), extra: Some(Operand::Imm(9)), size: None },
    ];

    let func_instr2: Vec<Instruction> = vec![
        Instruction::Ctrl { op: CtrlOp::Jmp, target: Operand::Sym("test_local3".to_string()) },
        Instruction::Ctrl { op: CtrlOp::Call, target: Operand::Sym("my_func".to_string()) },
        Instruction::Ctrl { op: CtrlOp::Jmp, target: Operand::Sym("my_func".to_string()) },

        Instruction::Ctrl { op: CtrlOp::JmpCC(ConditionCode::E),  target: Operand::Sym("my_func".to_string()) },
        Instruction::Ctrl { op: CtrlOp::JmpCC(ConditionCode::NE), target: Operand::Sym("my_func".to_string()) },
        Instruction::Ctrl { op: CtrlOp::JmpCC(ConditionCode::G),  target: Operand::Sym("my_func".to_string()) },
        Instruction::Ctrl { op: CtrlOp::JmpCC(ConditionCode::L),  target: Operand::Sym("my_func".to_string()) },
        Instruction::Ctrl { op: CtrlOp::JmpCC(ConditionCode::GE), target: Operand::Sym("my_func".to_string()) },
        Instruction::Ctrl { op: CtrlOp::JmpCC(ConditionCode::LE), target: Operand::Sym("my_func".to_string()) },
        Instruction::Ctrl { op: CtrlOp::JmpCC(ConditionCode::A),  target: Operand::Sym("my_func".to_string()) },
        Instruction::Ctrl { op: CtrlOp::JmpCC(ConditionCode::AE), target: Operand::Sym("my_func".to_string()) },
        Instruction::Ctrl { op: CtrlOp::JmpCC(ConditionCode::B),  target: Operand::Sym("my_func".to_string()) },
        Instruction::Ctrl { op: CtrlOp::JmpCC(ConditionCode::BE), target: Operand::Sym("my_func".to_string()) },
        Instruction::Ctrl { op: CtrlOp::JmpCC(ConditionCode::S),  target: Operand::Sym("my_func".to_string()) },
        Instruction::Ctrl { op: CtrlOp::JmpCC(ConditionCode::NS), target: Operand::Sym("my_func".to_string()) },
        Instruction::Ctrl { op: CtrlOp::JmpCC(ConditionCode::O),  target: Operand::Sym("my_func".to_string()) },
        Instruction::Ctrl { op: CtrlOp::JmpCC(ConditionCode::NO), target: Operand::Sym("my_func".to_string()) },
        Instruction::Ctrl { op: CtrlOp::JmpCC(ConditionCode::P),  target: Operand::Sym("my_func".to_string()) },
        Instruction::Ctrl { op: CtrlOp::JmpCC(ConditionCode::NP), target: Operand::Sym("my_func".to_string()) },

        Instruction::LocalSym("loop1".to_string()),
        Instruction::Ctrl { op: CtrlOp::Loop, target: Operand::Sym("loop1".to_string()) },
        Instruction::Ctrl { op: CtrlOp::Loope, target: Operand::Sym("loop1".to_string()) },
        Instruction::Ctrl { op: CtrlOp::Loopne, target: Operand::Sym("loop1".to_string()) },

        Instruction::CMovCC { cc: ConditionCode::E,   dst: Register::RAX, src: Operand::Reg(Register::RBX), size: Some(Size::U32) },
        Instruction::CMovCC { cc: ConditionCode::NE,  dst: Register::RCX, src: Operand::Reg(Register::RDX), size: Some(Size::U32) },
        Instruction::CMovCC { cc: ConditionCode::G, dst: Register::RAX, src: Operand::MemoryAddress(MemAddress::new().base(Register::RBX)), size: Some(Size::U32) },
        Instruction::CMovCC { cc: ConditionCode::L, dst: Register::RBX, src: Operand::MemoryAddress(MemAddress::new().base(Register::RCX).disp(4)), size: Some(Size::U32) },        Instruction::CMovCC { cc: ConditionCode::GE,  dst: Register::RDX, src: Operand::Reg(Register::RAX), size: Some(Size::U32) },
        Instruction::CMovCC { cc: ConditionCode::LE,  dst: Register::RCX, src: Operand::Reg(Register::RBX), size: Some(Size::U32) },
        Instruction::CMovCC { cc: ConditionCode::A,   dst: Register::RAX, src: Operand::Reg(Register::RCX), size: Some(Size::U32) },
        Instruction::CMovCC { cc: ConditionCode::AE,  dst: Register::RBX, src: Operand::Reg(Register::RDX), size: Some(Size::U32) },
        Instruction::CMovCC { cc: ConditionCode::B,   dst: Register::RCX, src: Operand::Reg(Register::RAX), size: Some(Size::U32) },
        Instruction::CMovCC { cc: ConditionCode::BE,  dst: Register::RDX, src: Operand::Reg(Register::RBX), size: Some(Size::U32) },
        Instruction::CMovCC { cc: ConditionCode::S,   dst: Register::RAX, src: Operand::Reg(Register::RDX), size: Some(Size::U32) },
        Instruction::CMovCC { cc: ConditionCode::NS,  dst: Register::RBX, src: Operand::Reg(Register::RCX), size: Some(Size::U32) },
        Instruction::CMovCC { cc: ConditionCode::O,   dst: Register::RCX, src: Operand::Reg(Register::RBX), size: Some(Size::U32) },
        Instruction::CMovCC { cc: ConditionCode::NO,  dst: Register::RDX, src: Operand::Reg(Register::RAX), size: Some(Size::U32) },
        Instruction::CMovCC { cc: ConditionCode::P,   dst: Register::RAX, src: Operand::Reg(Register::RBX), size: Some(Size::U32) },
        Instruction::CMovCC { cc: ConditionCode::NP,  dst: Register::RBX, src: Operand::Reg(Register::RCX), size: Some(Size::U32) },
        Instruction::Unary { op: UnaryOp::Nop, dst: Operand::NoOperand, size: None },
        Instruction::Unary { op: UnaryOp::Cwd, dst: Operand::NoOperand, size: Some(Size::U16) },
        Instruction::Unary { op: UnaryOp::Cdq, dst: Operand::NoOperand, size: Some(Size::U32) },
        Instruction::Unary { op: UnaryOp::Cqo, dst: Operand::NoOperand, size: None },
        Instruction::Unary { op: UnaryOp::Clc, dst: Operand::NoOperand, size: None },
        Instruction::Unary { op: UnaryOp::Stc, dst: Operand::NoOperand, size: None },
        Instruction::Unary { op: UnaryOp::Cld, dst: Operand::NoOperand, size: None },
        Instruction::Unary { op: UnaryOp::Std, dst: Operand::NoOperand, size: None },
        Instruction::Unary { op: UnaryOp::Ud2, dst: Operand::NoOperand, size: None },
        Instruction::Unary { op: UnaryOp::Hlt, dst: Operand::NoOperand, size: None },
        Instruction::Unary { op: UnaryOp::Pause, dst: Operand::NoOperand, size: None },
        Instruction::Unary { op: UnaryOp::Fwait, dst: Operand::NoOperand, size: None },
        Instruction::Unary { op: UnaryOp::Cmc, dst: Operand::NoOperand, size: None },
        Instruction::Ctrl { op: CtrlOp::Ret, target: Operand::NoOperand },
        Instruction::LocalSym("test_local3".to_string()),
        Instruction::Ctrl { op: CtrlOp::Ret, target: Operand::NoOperand },
        Instruction::String { op: StringOp::Cmps, size: None },
        Instruction::String { op: StringOp::Lods, size: None },
        Instruction::Prefix {
            prefix: vec![Prefix::Repe],
            ins: Box::new(
                Instruction::String {
                    op: StringOp::Cmps,
                    size: Some(Size::U8),
                }
            ),
        },
        Instruction::Prefix {
            prefix: vec![Prefix::Repne],
            ins: Box::new(
                Instruction::String {
                    op: StringOp::Scas,
                    size: Some(Size::U32),
                }
            ),
        },
        Instruction::Prefix {
            prefix: vec![Prefix::Lock],
            ins: Box::new(Instruction::ComplexBinary {op: ComplexBinOp::Cmpxchg,
                dst: Operand::MemoryAddress(MemAddress::new().base(Register::RAX)),
                src: Operand::Reg(Register::RBX),
                extra: None,
                size: Some(Size::U32),
            })
        },
        Instruction::Prefix {
            prefix: vec![Prefix::Lock],
            ins: Box::new(
                Instruction::Binary {
                    op: BinOp::Add,
                    dst: Operand::MemoryAddress(MemAddress::symbol("my_data")),
                    src: Operand::Imm(1),
                    size: Some(Size::U8),
                },
            ),
        },
        Instruction::Binary {
            op: BinOp::Add,
            dst: Operand::MemoryAddress(MemAddress::symbol("my_data")),
            src: Operand::Imm(1),
            size: Some(Size::U8),
        },
        Instruction::Prefix { prefix: vec![Prefix::Rep], ins: Box::new(Instruction::String { op: StringOp::Movs, size: None })},
        Instruction::String { op: StringOp::Movs, size: None },
        Instruction::String { op: StringOp::Scas, size: None },
        Instruction::String { op: StringOp::Stos, size: None },
        Instruction::Ctrl { op: CtrlOp::IRet, target: Operand::NoOperand },
        Instruction::ComplexBinary {
            op: ComplexBinOp::Xadd,
            dst: Operand::Reg(Register::RAX),
            src: Operand::Reg(Register::RBX),
            extra: None,
            size: Some(Size::U32),
        },
        Instruction::ComplexBinary {
            op: ComplexBinOp::Xadd,
            dst: Operand::MemoryAddress(MemAddress::new().base(Register::RAX)),
            src: Operand::Reg(Register::RBX),
            extra: None,
            size: Some(Size::U32),
        },
        Instruction::ComplexBinary {
            op: ComplexBinOp::Cmpxchg,
            dst: Operand::MemoryAddress(MemAddress::new().base(Register::RAX)),
            src: Operand::Reg(Register::RBX),
            extra: None,
            size: Some(Size::U32),
        },
    ];
    // =========================================================
    // ELF SETUP (inchangé)
    // =========================================================
    let mut elf_file = elf::file::ElfFile64::default();
    elf_file.declare_non_defined_sym(&"my_exit".to_string(), elf::file::SymbolType::Function);
    let section_data = elf_file.add_section(
        elf::shdr::SectionName::Data.as_str().to_string(),
        elf::shdr::ElfShdr {
            sh_type: elf::shdr::ShType::ProgBits as u32,
            sh_flags: (elf::shdr::ShFlags::Alloc as u64 | elf::shdr::ShFlags::Write as u64),
            sh_addralign: 4,
            ..Default::default()
        }
    );
    elf_file.add_symbol_to_section_raw(
        section_data,
        "my_data".to_string(),
        &vec![b'a', b'b', b'c', b'd', b'e', b'f', b'g', 0],
        make_st_info(elf::elfsym::StBind::Global, elf::elfsym::StType::Object),
        elf::elfsym::StVis::Default as u8,
    );

    elf_file.add_symbol_to_section_raw(
        section_data,
        "my_float".to_string(),
        &0.1f64.to_le_bytes().to_vec(),
        make_st_info(
            elf::elfsym::StBind::Global,
            elf::elfsym::StType::Object,
        ),
        elf::elfsym::StVis::Default as u8,
    );

    elf_file.add_symbol_to_section_raw(
        section_data,
        "my_lock".to_string(),
        &0u32.to_le_bytes().to_vec(),
        make_st_info(
            elf::elfsym::StBind::Global,
            elf::elfsym::StType::Object,
        ),
        elf::elfsym::StVis::Default as u8,
    );

    let text_section = elf_file.add_section(
        elf::shdr::SectionName::Text.as_str().to_string(),
        elf::shdr::ElfShdr {
            sh_type: elf::shdr::ShType::ProgBits as u32,
            sh_flags: (elf::shdr::ShFlags::Alloc as u64 | elf::shdr::ShFlags::ExecInstr as u64),
            sh_addralign: 16,
            ..Default::default()
        }
    );
    elf_file.add_symbol_to_section(
        text_section,
        "my_func".to_string(),
        &func_instr,
        make_st_info(elf::elfsym::StBind::Global, elf::elfsym::StType::Func),
        elf::elfsym::StVis::Default as u8,
    );
    elf_file.add_symbol_to_section(
        text_section,
        "my_other_other_func".to_string(),
        &setcc_instr,
        make_st_info(elf::elfsym::StBind::Global, elf::elfsym::StType::Func),
        elf::elfsym::StVis::Default as u8,
    );
    elf_file.add_symbol_to_section(
        text_section,
        "my_other_ctrl_func".to_string(),
        &func_instr2,
        make_st_info(elf::elfsym::StBind::Global, elf::elfsym::StType::Func),
        elf::elfsym::StVis::Default as u8,
    );
    elf_file.add_symbol_to_section(
        text_section,
        "my_other_func".to_string(),
        &unary_instr, 
        make_st_info(elf::elfsym::StBind::Global, elf::elfsym::StType::Func),
        elf::elfsym::StVis::Default as u8,
    );
    elf_file.add_symbol_to_section(
        text_section,
        "_start".to_string(),
        &start_instr,
        make_st_info(elf::elfsym::StBind::Global, elf::elfsym::StType::Func),
        elf::elfsym::StVis::Default as u8,
    );
    elf_file.write(&mut file)?;
    println!("ELF généré : output.elf");
    Ok(())
}