use crate::elf::elfsym::make_st_info;

mod elf;

fn main() -> std::io::Result<()> {
    use std::fs::File;
    use elf::instructions::*;

    let mut file = File::create("output.elf")?;

    let my_data = Operand::MemoryAddress(MemAddress::Direct {
        disp: MemDisplacement::Sym("my_data".to_string()),
    });

    let mem = Operand::MemoryAddress(MemAddress::BaseDisp {
        base: Register::Ebx,
        disp: MemDisplacement::Imm(0),
    });

    // =========================================================
    // FUNCTION: my_func
    // =========================================================
    let func_instr: Vec<Instruction> = vec![
        Instruction::Ctrl { op: CtrlOp::Jmp, target: Operand::Sym("test_local2".to_string()) },
        Instruction::Ctrl { op: CtrlOp::Call, target: Operand::Sym("my_func".to_string()) },
        Instruction::Ctrl { op: CtrlOp::Jmp, target: Operand::Sym("my_func".to_string()) },
        Instruction::Ctrl { op: CtrlOp::Je, target: Operand::Sym("my_func".to_string()) },
        Instruction::Ctrl { op: CtrlOp::Jne, target: Operand::Sym("my_func".to_string()) },
        Instruction::Ctrl { op: CtrlOp::Jg, target: Operand::Sym("my_func".to_string()) },
        Instruction::Ctrl { op: CtrlOp::Jl, target: Operand::Sym("my_func".to_string()) },
        Instruction::Ctrl { op: CtrlOp::Jge, target: Operand::Sym("my_func".to_string()) },
        Instruction::Ctrl { op: CtrlOp::Jle, target: Operand::Sym("my_func".to_string()) },

        Instruction::Binary { op: BinOp::CondMov(CMovCC::Cmove),  dst: Operand::Reg(Register::Eax), src: Operand::Reg(Register::Ebx), size: Some(Size::U32) },
        Instruction::Binary { op: BinOp::CondMov(CMovCC::Cmovne), dst: Operand::Reg(Register::Ecx), src: Operand::Reg(Register::Edx), size: Some(Size::U32) },
        Instruction::Binary { op: BinOp::CondMov(CMovCC::Cmovg),  dst: Operand::Reg(Register::Eax), src: Operand::MemoryAddress(MemAddress::BaseDisp { base: Register::Ebx, disp: MemDisplacement::Imm(0) }), size: Some(Size::U32) },
        Instruction::Binary { op: BinOp::CondMov(CMovCC::Cmovl),  dst: Operand::Reg(Register::Ebx), src: Operand::MemoryAddress(MemAddress::BaseDisp { base: Register::Ecx, disp: MemDisplacement::Imm(4) }), size: Some(Size::U32) },
        Instruction::Binary { op: BinOp::CondMov(CMovCC::Cmovge), dst: Operand::Reg(Register::Edx), src: Operand::Reg(Register::Eax), size: Some(Size::U32) },
        Instruction::Binary { op: BinOp::CondMov(CMovCC::Cmovle), dst: Operand::Reg(Register::Ecx), src: Operand::Reg(Register::Ebx), size: Some(Size::U32) },

        // =====================================================
        // BIT OPS : reg + imm
        // =====================================================
        Instruction::Bit { op: BitOp::Bt,  dst: Operand::Reg(Register::Eax), src: Operand::Imm(3), size: Some(Size::U32) },
        Instruction::Bit { op: BitOp::Bts, dst: Operand::Reg(Register::Ebx), src: Operand::Imm(5), size: Some(Size::U32) },
        Instruction::Bit { op: BitOp::Btr, dst: Operand::Reg(Register::Ecx), src: Operand::Imm(7), size: Some(Size::U32) },
        Instruction::Bit { op: BitOp::Btc, dst: Operand::Reg(Register::Edx), src: Operand::Imm(1), size: Some(Size::U32) },

        // =====================================================
        // BIT OPS : reg + reg
        // =====================================================
        Instruction::Bit { op: BitOp::Bt,  dst: Operand::Reg(Register::Eax), src: Operand::Reg(Register::Ebx), size: Some(Size::U32) },
        Instruction::Bit { op: BitOp::Bts, dst: Operand::Reg(Register::Ecx), src: Operand::Reg(Register::Edx), size: Some(Size::U32) },
        Instruction::Bit { op: BitOp::Btr, dst: Operand::Reg(Register::Ebx), src: Operand::Reg(Register::Eax), size: Some(Size::U32) },
        Instruction::Bit { op: BitOp::Btc, dst: Operand::Reg(Register::Edx), src: Operand::Reg(Register::Ecx), size: Some(Size::U32) },

        // =====================================================
        // BIT OPS : mem + imm
        // =====================================================
        Instruction::Bit {
            op: BitOp::Bt,
            dst: Operand::MemoryAddress(MemAddress::BaseDisp { base: Register::Ebx, disp: MemDisplacement::Imm(8) }),
            src: Operand::Imm(2),
            size: Some(Size::U32),
        },

        Instruction::Bit {
            op: BitOp::Bts,
            dst: Operand::MemoryAddress(MemAddress::BaseDisp { base: Register::Ebx, disp: MemDisplacement::Imm(12) }),
            src: Operand::Imm(4),
            size: Some(Size::U32),
        },

        // =====================================================
        // BIT OPS : mem + reg
        // =====================================================
        Instruction::Bit {
            op: BitOp::Btr,
            dst: Operand::MemoryAddress(MemAddress::BaseDisp { base: Register::Ecx, disp: MemDisplacement::Imm(16) }),
            src: Operand::Reg(Register::Eax),
            size: Some(Size::U32),
        },

        Instruction::Bit {
            op: BitOp::Btc,
            dst: Operand::MemoryAddress(MemAddress::BaseDisp { base: Register::Edx, disp: MemDisplacement::Imm(20) }),
            src: Operand::Reg(Register::Ebx),
            size: Some(Size::U32),
        },

        Instruction::LocalSym("test_local2".to_string()),
        Instruction::Ctrl { op: CtrlOp::Ret, target: Operand::NoOperand },
    ];

    let unary_instr: Vec<Instruction> = vec![
        Instruction::Unary { op: UnaryOp::Inc, dst: Operand::Reg(Register::Eax), size: Some(Size::U8) },
        Instruction::Unary { op: UnaryOp::Inc, dst: Operand::Reg(Register::Ebx), size: Some(Size::U16) },
        Instruction::Unary { op: UnaryOp::Inc, dst: Operand::Reg(Register::Ecx), size: Some(Size::U32) },
        Instruction::Unary { op: UnaryOp::Inc, dst: Operand::MemoryAddress(MemAddress::BaseDisp { base: Register::Ebx, disp: MemDisplacement::Imm(0) }), size: Some(Size::U32) },

        Instruction::Unary { op: UnaryOp::Dec, dst: Operand::Reg(Register::Edx), size: Some(Size::U8) },
        Instruction::Unary { op: UnaryOp::Dec, dst: Operand::Reg(Register::Eax), size: Some(Size::U16) },
        Instruction::Unary { op: UnaryOp::Dec, dst: Operand::Reg(Register::Ebx), size: Some(Size::U32) },
        Instruction::Unary { op: UnaryOp::Dec, dst: Operand::MemoryAddress(MemAddress::BaseDisp { base: Register::Ecx, disp: MemDisplacement::Imm(4) }), size: Some(Size::U32) },

        Instruction::Unary { op: UnaryOp::Neg, dst: Operand::Reg(Register::Ecx), size: Some(Size::U8) },
        Instruction::Unary { op: UnaryOp::Neg, dst: Operand::Reg(Register::Edx), size: Some(Size::U16) },
        Instruction::Unary { op: UnaryOp::Neg, dst: Operand::Reg(Register::Eax), size: Some(Size::U32) },
        Instruction::Unary { op: UnaryOp::Neg, dst: Operand::MemoryAddress(MemAddress::BaseDisp { base: Register::Ebx, disp: MemDisplacement::Imm(8) }), size: Some(Size::U32) },

        Instruction::Unary { op: UnaryOp::Not, dst: Operand::Reg(Register::Ebx), size: Some(Size::U8) },
        Instruction::Unary { op: UnaryOp::Not, dst: Operand::Reg(Register::Ecx), size: Some(Size::U16) },
        Instruction::Unary { op: UnaryOp::Not, dst: Operand::Reg(Register::Edx), size: Some(Size::U32) },
        Instruction::Unary { op: UnaryOp::Not, dst: Operand::MemoryAddress(MemAddress::BaseDisp { base: Register::Eax, disp: MemDisplacement::Imm(12) }), size: Some(Size::U32) },

        // =====================================================
        // PUSH reg
        // =====================================================
        Instruction::Stack { op: StackOp::Push, value: Operand::Reg(Register::Eax), size: Some(Size::U32) },
        Instruction::Stack { op: StackOp::Push, value: Operand::Reg(Register::Ebx), size: Some(Size::U32) },

        // PUSH imm
        Instruction::Stack { op: StackOp::Push, value: Operand::Imm(5), size: Some(Size::U32) },
        Instruction::Stack { op: StackOp::Push, value: Operand::Imm(0x12345678), size: Some(Size::U32) },

        // PUSH symbol
        Instruction::Stack { op: StackOp::Push, value: Operand::Sym("my_data".to_string()), size: Some(Size::U32) },

        // PUSH memory
        Instruction::Stack { op: StackOp::Push, value: Operand::MemoryAddress(MemAddress::BaseDisp { base: Register::Ebx, disp: MemDisplacement::Imm(16), }), size: Some(Size::U32), },

        // =====================================================
        // POP reg
        // =====================================================
        Instruction::Stack { op: StackOp::Pop, value: Operand::Reg(Register::Ecx), size: Some(Size::U32) },
        Instruction::Stack { op: StackOp::Pop, value: Operand::Reg(Register::Edx), size: Some(Size::U32) },

        // POP memory
        Instruction::Stack {
            op: StackOp::Pop,
            value: Operand::MemoryAddress(MemAddress::BaseDisp {
                base: Register::Eax,
                disp: MemDisplacement::Imm(20),
            }),
            size: Some(Size::U32),
        },

        Instruction::Ctrl { op: CtrlOp::Ret, target: Operand::NoOperand },
    ];


    let setcc_instr: Vec<Instruction> = vec![
        Instruction::Binary { op: BinOp::Cmp, dst: Operand::Reg(Register::Eax), src: Operand::Reg(Register::Ebx), size: Some(Size::U32) },
        Instruction::SetCC { op: SetCC::Sete, dst: Operand::Reg(Register::Eax) }, Instruction::SetCC { op: SetCC::Setne, dst: Operand::Reg(Register::Ebx) }, Instruction::SetCC { op: SetCC::Setg, dst: Operand::Reg(Register::Ecx) }, Instruction::SetCC { op: SetCC::Setl, dst: Operand::Reg(Register::Edx) }, Instruction::SetCC { op: SetCC::Setge, dst: Operand::Reg(Register::Eax) }, Instruction::SetCC { op: SetCC::Setle, dst: Operand::Reg(Register::Ebx) }, Instruction::SetCC { op: SetCC::Seta, dst: Operand::Reg(Register::Ecx) }, Instruction::SetCC { op: SetCC::Setb, dst: Operand::Reg(Register::Edx) },
        Instruction::SetCC { op: SetCC::Setae, dst: Operand::Reg(Register::Eax) },
        Instruction::SetCC { op: SetCC::Setbe, dst: Operand::Reg(Register::Ebx) }, 
        Instruction::SetCC { op: SetCC::Sets, dst: Operand::Reg(Register::Ecx) },
        Instruction::SetCC { op: SetCC::Setns, dst: Operand::Reg(Register::Edx) },
        Instruction::SetCC { op: SetCC::Seto, dst: Operand::Reg(Register::Eax) },
        Instruction::SetCC { op: SetCC::Setno, dst: Operand::Reg(Register::Ebx) },
        Instruction::SetCC { op: SetCC::Setp, dst: Operand::Reg(Register::Ecx) },
        Instruction::SetCC { op: SetCC::Setnp, dst: Operand::Reg(Register::Edx) },

        Instruction::SetCC { op: SetCC::Sete, dst: Operand::MemoryAddress(MemAddress::BaseDisp { base: Register::Ebx, disp: MemDisplacement::Imm(0) }) },
        Instruction::SetCC { op: SetCC::Setne, dst: Operand::MemoryAddress(MemAddress::BaseDisp { base: Register::Ebx, disp: MemDisplacement::Imm(1) }) },
        Instruction::SetCC { op: SetCC::Setg, dst: Operand::MemoryAddress(MemAddress::BaseDisp { base: Register::Ebx, disp: MemDisplacement::Imm(2) }) },
        Instruction::SetCC { op: SetCC::Setl, dst: Operand::MemoryAddress(MemAddress::BaseDisp { base: Register::Ebx, disp: MemDisplacement::Imm(3) }) },
        Instruction::SetCC { op: SetCC::Setge, dst: Operand::MemoryAddress(MemAddress::BaseDisp { base: Register::Ebx, disp: MemDisplacement::Imm(4) }) },
        Instruction::SetCC { op: SetCC::Setle, dst: Operand::MemoryAddress(MemAddress::BaseDisp { base: Register::Ebx, disp: MemDisplacement::Imm(5) }) },
        Instruction::SetCC { op: SetCC::Seta, dst: Operand::MemoryAddress(MemAddress::BaseDisp { base: Register::Ebx, disp: MemDisplacement::Imm(6) }) },
        Instruction::SetCC { op: SetCC::Setb, dst: Operand::MemoryAddress(MemAddress::BaseDisp { base: Register::Ebx, disp: MemDisplacement::Imm(7) }) },
        Instruction::SetCC { op: SetCC::Setae, dst: Operand::MemoryAddress(MemAddress::BaseDisp { base: Register::Ebx, disp: MemDisplacement::Imm(8) }) },
        Instruction::SetCC { op: SetCC::Setbe, dst: Operand::MemoryAddress(MemAddress::BaseDisp { base: Register::Ebx, disp: MemDisplacement::Imm(9) }) },
        Instruction::SetCC { op: SetCC::Sets, dst: Operand::MemoryAddress(MemAddress::BaseDisp { base: Register::Ebx, disp: MemDisplacement::Imm(10) }) },
        Instruction::SetCC { op: SetCC::Setns, dst: Operand::MemoryAddress(MemAddress::BaseDisp { base: Register::Ebx, disp: MemDisplacement::Imm(11) }) },
        Instruction::SetCC { op: SetCC::Seto, dst: Operand::MemoryAddress(MemAddress::BaseDisp { base: Register::Ebx, disp: MemDisplacement::Imm(12) }) },
        Instruction::SetCC { op: SetCC::Setno, dst: Operand::MemoryAddress(MemAddress::BaseDisp { base: Register::Ebx, disp: MemDisplacement::Imm(13) }) },
        Instruction::SetCC { op: SetCC::Setp, dst: Operand::MemoryAddress(MemAddress::BaseDisp { base: Register::Ebx, disp: MemDisplacement::Imm(14) }) },
        Instruction::SetCC { op: SetCC::Setnp, dst: Operand::MemoryAddress(MemAddress::BaseDisp { base: Register::Ebx, disp: MemDisplacement::Imm(15) }) },

        Instruction::Ctrl { op: CtrlOp::Ret, target: Operand::NoOperand },
    ];
    // =========================================================
    // _start : FULL BINOP TEST SUITE
    // =========================================================
    let start_instr: Vec<Instruction> = vec![
        // -------------------------------------------------
        // MOV
        // -------------------------------------------------
        Instruction::Binary { op: BinOp::Mov, dst: Operand::Reg(Register::Eax), src: Operand::Imm(0x12), size: Some(Size::U8) },
        Instruction::Binary { op: BinOp::Mov, dst: Operand::Reg(Register::Ebx), src: Operand::Imm(0x1234), size: Some(Size::U16) },
        Instruction::Binary { op: BinOp::Mov, dst: Operand::Reg(Register::Ecx), src: Operand::Imm(0x12345678), size: Some(Size::U32) },
        Instruction::Binary { op: BinOp::Mov, dst: Operand::Reg(Register::Edx), src: Operand::Sym("my_data".to_string()), size: Some(Size::U32) },
        Instruction::Binary { op: BinOp::Mov, dst: Operand::Reg(Register::Eax), src: Operand::Reg(Register::Ebx), size: Some(Size::U32) },
        Instruction::Binary { op: BinOp::Mov, dst: my_data.clone(), src: Operand::Imm(0x41), size: Some(Size::U8) },
        Instruction::Binary { op: BinOp::Mov, dst: my_data.clone(), src: Operand::Reg(Register::Eax), size: Some(Size::U32) },
        Instruction::Binary { op: BinOp::Mov, dst: Operand::Reg(Register::Eax), src: my_data.clone(), size: Some(Size::U32) },

        // -------------------------------------------------
        // MOVZX / MOVSX
        // -------------------------------------------------
        Instruction::Binary { op: BinOp::Movzx, dst: Operand::Reg(Register::Eax), src: Operand::Reg(Register::Ebx), size: Some(Size::U8) },
        Instruction::Binary { op: BinOp::Movzx, dst: Operand::Reg(Register::Ecx), src: mem.clone(), size: Some(Size::U8) },
        Instruction::Binary { op: BinOp::Movsx, dst: Operand::Reg(Register::Edx), src: Operand::Reg(Register::Eax), size: Some(Size::U8) },
        Instruction::Binary { op: BinOp::Movsx, dst: Operand::Reg(Register::Ebx), src: mem.clone(), size: Some(Size::U16) },

        // -------------------------------------------------
        // XCHG
        // -------------------------------------------------
        Instruction::Binary { op: BinOp::Xchg, dst: Operand::Reg(Register::Eax), src: Operand::Reg(Register::Ebx), size: Some(Size::U32) },
        Instruction::Binary { op: BinOp::Xchg, dst: Operand::Reg(Register::Ecx), src: mem.clone(), size: Some(Size::U32) },

        // -------------------------------------------------
        // ADD
        // -------------------------------------------------
        Instruction::Binary { op: BinOp::Add, dst: Operand::Reg(Register::Eax), src: Operand::Imm(5), size: Some(Size::U32) },
        Instruction::Binary { op: BinOp::Add, dst: Operand::Reg(Register::Ebx), src: Operand::Reg(Register::Eax), size: Some(Size::U32) },
        Instruction::Binary { op: BinOp::Add, dst: mem.clone(), src: Operand::Imm(3), size: Some(Size::U32) },

        // -------------------------------------------------
        // SUB
        // -------------------------------------------------
        Instruction::Binary { op: BinOp::Sub, dst: Operand::Reg(Register::Ecx), src: Operand::Imm(10), size: Some(Size::U32) },
        Instruction::Binary { op: BinOp::Sub, dst: Operand::Reg(Register::Edx), src: Operand::Reg(Register::Ecx), size: Some(Size::U32) },
        Instruction::Binary { op: BinOp::Sub, dst: mem.clone(), src: Operand::Imm(1), size: Some(Size::U32) },

        // -------------------------------------------------
        // AND
        // -------------------------------------------------
        Instruction::Binary { op: BinOp::And, dst: Operand::Reg(Register::Eax), src: Operand::Imm(0xFF), size: Some(Size::U32) },
        Instruction::Binary { op: BinOp::And, dst: Operand::Reg(Register::Ebx), src: Operand::Reg(Register::Eax), size: Some(Size::U32) },
        Instruction::Binary { op: BinOp::And, dst: mem.clone(), src: Operand::Reg(Register::Ebx), size: Some(Size::U32) },

        // -------------------------------------------------
        // OR
        // -------------------------------------------------
        Instruction::Binary { op: BinOp::Or, dst: Operand::Reg(Register::Ecx), src: Operand::Imm(0x10), size: Some(Size::U32) },
        Instruction::Binary { op: BinOp::Or, dst: Operand::Reg(Register::Edx), src: Operand::Reg(Register::Ecx), size: Some(Size::U32) },

        // -------------------------------------------------
        // XOR
        // -------------------------------------------------
        Instruction::Binary { op: BinOp::Xor, dst: Operand::Reg(Register::Eax), src: Operand::Imm(0xFF), size: Some(Size::U32) },
        Instruction::Binary { op: BinOp::Xor, dst: Operand::Reg(Register::Ebx), src: Operand::Reg(Register::Eax), size: Some(Size::U32) },
        Instruction::Binary { op: BinOp::Xor, dst: Operand::Reg(Register::Ecx), src: mem.clone(), size: Some(Size::U32) },

        // -------------------------------------------------
        // CMP
        // -------------------------------------------------
        Instruction::Binary { op: BinOp::Cmp, dst: Operand::Reg(Register::Eax), src: Operand::Imm(42), size: Some(Size::U32) },
        Instruction::Binary { op: BinOp::Cmp, dst: Operand::Reg(Register::Ebx), src: Operand::Reg(Register::Eax), size: Some(Size::U32) },
        Instruction::Binary { op: BinOp::Cmp, dst: Operand::Reg(Register::Ecx), src: mem.clone(), size: Some(Size::U32) },

        // -------------------------------------------------
        // TEST
        // -------------------------------------------------
        Instruction::Binary { op: BinOp::Test, dst: Operand::Reg(Register::Eax), src: Operand::Imm(1), size: Some(Size::U32) },
        Instruction::Binary { op: BinOp::Test, dst: Operand::Reg(Register::Ebx), src: Operand::Reg(Register::Eax), size: Some(Size::U32) },
        Instruction::Binary { op: BinOp::Test, dst: mem.clone(), src: Operand::Imm(0xFF), size: Some(Size::U32) },

        // -------------------------------------------------
        // SHL / SHR / SAR
        // -------------------------------------------------
        Instruction::Shift{ op: ShiftOp::Shl, dst: Operand::Reg(Register::Eax), src: Operand::Imm(1), size: Some(Size::U32) },
        Instruction::Shift{ op: ShiftOp::Shl, dst: Operand::Reg(Register::Ebx), src: Operand::Imm(3), size: Some(Size::U32) },
        Instruction::Shift{ op: ShiftOp::Shl, dst: Operand::Reg(Register::Ecx), src: Operand::Reg(Register::Ecx), size: Some(Size::U32) },

        Instruction::Shift{ op: ShiftOp::Shr, dst: Operand::Reg(Register::Edx), src: Operand::Imm(1), size: Some(Size::U32) },
        Instruction::Shift{ op: ShiftOp::Shr, dst: mem.clone(), src: Operand::Imm(2), size: Some(Size::U32) },

        Instruction::Shift{ op: ShiftOp::Sar, dst: Operand::Reg(Register::Eax), src: Operand::Imm(1), size: Some(Size::U32) },
        Instruction::Shift{ op: ShiftOp::Sar, dst: mem.clone(), src: Operand::Reg(Register::Ecx), size: Some(Size::U32) },

        // -------------------------------------------------
        // ROL / ROR
        // -------------------------------------------------
        Instruction::Shift { op: ShiftOp::Rol, dst: Operand::Reg(Register::Ebx), src: Operand::Imm(1), size: Some(Size::U32) },
        Instruction::Shift { op: ShiftOp::Rol, dst: Operand::Reg(Register::Ecx), src: Operand::Imm(4), size: Some(Size::U32) },
        Instruction::Shift { op: ShiftOp::Ror, dst: Operand::Reg(Register::Edx), src: Operand::Imm(1), size: Some(Size::U32) },
        Instruction::Shift { op: ShiftOp::Ror, dst: mem.clone(), src: Operand::Reg(Register::Ecx), size: Some(Size::U32) },

        // -------------------------------------------------
        // Labels / Calls
        // -------------------------------------------------
        // Instruction::LocalSym("test_local".to_string()),
        Instruction::Ctrl { op: CtrlOp::Call, target: Operand::Sym("my_func".to_string()) },
        Instruction::Ctrl { op: CtrlOp::Call, target: Operand::Sym("my_exit".to_string()) },

        // -------------------------------------------------
        // Syscalls
        // -------------------------------------------------
        Instruction::Binary { op: BinOp::Mov, dst: Operand::Reg(Register::Eax), src: Operand::Imm(60), size: None },
        Instruction::Binary { op: BinOp::Mov, dst: Operand::Reg(Register::Edi), src: Operand::Imm(42), size: None },
        Instruction::Sys { op: SysOp::Syscall },

        Instruction::Binary { op: BinOp::Mov, dst: Operand::Reg(Register::Eax), src: Operand::Imm(1), size: None },
        Instruction::Binary { op: BinOp::Mov, dst: Operand::Reg(Register::Ebx), src: Operand::Imm(42), size: None },
        Instruction::Sys { op: SysOp::Int(0x80) },
        Instruction::Sys { op: SysOp::Sysenter },
    ];


    let func_instr2: Vec<Instruction> = vec![
        Instruction::Ctrl { op: CtrlOp::Jmp, target: Operand::Sym("test_local3".to_string()) },
        Instruction::Ctrl { op: CtrlOp::Call, target: Operand::Sym("my_func".to_string()) },
        Instruction::Ctrl { op: CtrlOp::Jmp, target: Operand::Sym("my_func".to_string()) },

        Instruction::Ctrl { op: CtrlOp::Je,  target: Operand::Sym("my_func".to_string()) },
        Instruction::Ctrl { op: CtrlOp::Jne, target: Operand::Sym("my_func".to_string()) },
        Instruction::Ctrl { op: CtrlOp::Jg,  target: Operand::Sym("my_func".to_string()) },
        Instruction::Ctrl { op: CtrlOp::Jl,  target: Operand::Sym("my_func".to_string()) },
        Instruction::Ctrl { op: CtrlOp::Jge, target: Operand::Sym("my_func".to_string()) },
        Instruction::Ctrl { op: CtrlOp::Jle, target: Operand::Sym("my_func".to_string()) },
        Instruction::Ctrl { op: CtrlOp::Ja,  target: Operand::Sym("my_func".to_string()) },
        Instruction::Ctrl { op: CtrlOp::Jae, target: Operand::Sym("my_func".to_string()) },
        Instruction::Ctrl { op: CtrlOp::Jb,  target: Operand::Sym("my_func".to_string()) },
        Instruction::Ctrl { op: CtrlOp::Jbe, target: Operand::Sym("my_func".to_string()) },
        Instruction::Ctrl { op: CtrlOp::Js,  target: Operand::Sym("my_func".to_string()) },
        Instruction::Ctrl { op: CtrlOp::Jns, target: Operand::Sym("my_func".to_string()) },
        Instruction::Ctrl { op: CtrlOp::Jo,  target: Operand::Sym("my_func".to_string()) },
        Instruction::Ctrl { op: CtrlOp::Jno, target: Operand::Sym("my_func".to_string()) },
        Instruction::Ctrl { op: CtrlOp::Jp,  target: Operand::Sym("my_func".to_string()) },
        Instruction::Ctrl { op: CtrlOp::Jnp, target: Operand::Sym("my_func".to_string()) },

        Instruction::Binary { op: BinOp::CondMov(CMovCC::Cmove),   dst: Operand::Reg(Register::Eax), src: Operand::Reg(Register::Ebx), size: Some(Size::U32) },
        Instruction::Binary { op: BinOp::CondMov(CMovCC::Cmovne),  dst: Operand::Reg(Register::Ecx), src: Operand::Reg(Register::Edx), size: Some(Size::U32) },
        Instruction::Binary { op: BinOp::CondMov(CMovCC::Cmovg),   dst: Operand::Reg(Register::Eax), src: Operand::MemoryAddress(MemAddress::BaseDisp { base: Register::Ebx, disp: MemDisplacement::Imm(0) }), size: Some(Size::U32) },
        Instruction::Binary { op: BinOp::CondMov(CMovCC::Cmovl),   dst: Operand::Reg(Register::Ebx), src: Operand::MemoryAddress(MemAddress::BaseDisp { base: Register::Ecx, disp: MemDisplacement::Imm(4) }), size: Some(Size::U32) },
        Instruction::Binary { op: BinOp::CondMov(CMovCC::Cmovge),  dst: Operand::Reg(Register::Edx), src: Operand::Reg(Register::Eax), size: Some(Size::U32) },
        Instruction::Binary { op: BinOp::CondMov(CMovCC::Cmovle),  dst: Operand::Reg(Register::Ecx), src: Operand::Reg(Register::Ebx), size: Some(Size::U32) },

        Instruction::Binary { op: BinOp::CondMov(CMovCC::Cmova),   dst: Operand::Reg(Register::Eax), src: Operand::Reg(Register::Ecx), size: Some(Size::U32) },
        Instruction::Binary { op: BinOp::CondMov(CMovCC::Cmovae),  dst: Operand::Reg(Register::Ebx), src: Operand::Reg(Register::Edx), size: Some(Size::U32) },
        Instruction::Binary { op: BinOp::CondMov(CMovCC::Cmovb),   dst: Operand::Reg(Register::Ecx), src: Operand::Reg(Register::Eax), size: Some(Size::U32) },
        Instruction::Binary { op: BinOp::CondMov(CMovCC::Cmovbe),  dst: Operand::Reg(Register::Edx), src: Operand::Reg(Register::Ebx), size: Some(Size::U32) },

        Instruction::Binary { op: BinOp::CondMov(CMovCC::Cmovs),   dst: Operand::Reg(Register::Eax), src: Operand::Reg(Register::Edx), size: Some(Size::U32) },
        Instruction::Binary { op: BinOp::CondMov(CMovCC::Cmovns),  dst: Operand::Reg(Register::Ebx), src: Operand::Reg(Register::Ecx), size: Some(Size::U32) },

        Instruction::Binary { op: BinOp::CondMov(CMovCC::Cmovo),   dst: Operand::Reg(Register::Ecx), src: Operand::Reg(Register::Ebx), size: Some(Size::U32) },
        Instruction::Binary { op: BinOp::CondMov(CMovCC::Cmovno),  dst: Operand::Reg(Register::Edx), src: Operand::Reg(Register::Eax), size: Some(Size::U32) },

        Instruction::Binary { op: BinOp::CondMov(CMovCC::Cmovp),   dst: Operand::Reg(Register::Eax), src: Operand::Reg(Register::Ebx), size: Some(Size::U32) },
        Instruction::Binary { op: BinOp::CondMov(CMovCC::Cmovnp),  dst: Operand::Reg(Register::Ebx), src: Operand::Reg(Register::Ecx), size: Some(Size::U32) },

        Instruction::Unary { op: UnaryOp::Nop, dst: Operand::NoOperand, size: None },

        Instruction::Unary { op: UnaryOp::Cwd, dst: Operand::NoOperand, size: Some(Size::U16) },
        Instruction::Unary { op: UnaryOp::Cdq, dst: Operand::NoOperand, size: Some(Size::U32) },
        Instruction::Unary { op: UnaryOp::Cqo, dst: Operand::NoOperand, size: Some(Size::U64) },

        Instruction::Unary { op: UnaryOp::Clc, dst: Operand::NoOperand, size: None },
        Instruction::Unary { op: UnaryOp::Stc, dst: Operand::NoOperand, size: None },
        Instruction::Unary { op: UnaryOp::Cmc, dst: Operand::NoOperand, size: None },

        Instruction::Ctrl { op: CtrlOp::Ret, target: Operand::NoOperand },

        Instruction::LocalSym("test_local3".to_string()),
        Instruction::Ctrl { op: CtrlOp::Ret, target: Operand::NoOperand },
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