use super::generation::Context;
use crate::codegen::utils::{load_operand, var_size};
use crate::elf::instructions::Operand::Sym;
use crate::elf::instructions::register::{
    R8, R9, RBP, RCX, RDI, RDX, RSI, RSP, Register, XMM0, XMM1, XMM2, XMM3, XMM4, XMM5, XMM6, XMM7,
};
use crate::elf::instructions::{BinOp, ConvOp, CtrlOp, ExtendOp, Instruction, Size, register::RAX};

use crate::converter::ir::structs::{Operand as IrOp, VarId};
use crate::elf::instructions::{MemAddress, Operand as ElfOper, StackOp};
use crate::parser::postparser::structs::Type;

/// `ctx.offsets`: the offset under rbp of each variable, see var_offsets
pub fn convert_operand(op: &IrOp, ctx: &Context) -> ElfOper {
    return match op {
        IrOp::Var(var) => {
            ElfOper::MemoryAddress(MemAddress::new().base(RBP).disp(-ctx.offsets[var.id]))
        }
        IrOp::Int(n, _) => ElfOper::Imm(*n),
        IrOp::Bool(b) => ElfOper::Imm(*b as i64),
        IrOp::Null => ElfOper::Imm(0),
        // a float has no immediate form: it is read from its constant in .rodata
        IrOp::Float(f, _) => {
            ElfOper::MemoryAddress(MemAddress::symbol(&ctx.float_names[*f]).rip_relative())
        }
        _ => unimplemented!(),
    };
}

/// `mov rsp, rbp ; pop rbp ; ret`: undoes the prologue and returns
pub fn func_epilogue() -> Vec<Instruction> {
    vec![
        Instruction::Binary {
            op: BinOp::Mov,
            dst: ElfOper::Reg(RSP),
            src: ElfOper::Reg(RBP),
            size: None,
        },
        Instruction::Stack {
            op: StackOp::Pop,
            value: ElfOper::Reg(RBP),
            size: None,
        },
        Instruction::Ctrl {
            op: CtrlOp::Ret,
            target: ElfOper::NoOperand,
        },
    ]
}

/// `mov rax, op` then the epilogue: the value comes back in rax (System V)
pub fn generate_return(op: &IrOp, ctx: &Context) -> Vec<Instruction> {
    let mut epi = func_epilogue();
    if matches!(ctx.return_type, Some(Type::Float { .. })) {
        epi.insert(0, load_operand(XMM0, op, ctx, None));
    } else {
        epi.insert(0, load_operand(RAX, op, ctx, None));
    }
    return epi;
}

pub const PARAM_FREGS: [Register; 8] = [XMM0, XMM1, XMM2, XMM3, XMM4, XMM5, XMM6, XMM7];

/// The float argument of rank `i_f` into xmm`i_f`; not done yet, panics
pub fn float_parameter(op: &IrOp, i_f: &mut i32, ctx: &Context) -> Instruction {
    let Some(reg) = PARAM_FREGS.get(*i_f as usize) else {
        unimplemented!()
    };
    *i_f += 1;
    return load_operand(*reg, op, ctx, None);
}

/// Registers of the integer arguments, in order (System V)
pub const PARAM_REGS: [Register; 6] = [RDI, RSI, RDX, RCX, R8, R9];

/// `mov` of the argument into the register of its rank `i`, then the next rank
pub fn normal_param(op: &IrOp, i: &mut i32, ctx: &Context) -> Instruction {
    // shortcut: no 7th integer argument (it goes on the stack), add it for calls of more than 6
    let Some(reg) = PARAM_REGS.get(*i as usize) else {
        unimplemented!()
    };
    *i += 1;
    return load_operand(*reg, op, ctx, None);
}

/// The size in bytes of the value of `op`, and true for a 소수
fn operand_kind(op: &IrOp) -> (usize, bool) {
    return match op {
        IrOp::Var(var) => (var.ty.size(), var.ty.is_float()),
        IrOp::Int(_, short) => (if *short { 4 } else { 8 }, false),
        IrOp::Float(_, short) => (if *short { 4 } else { 8 }, true),
        IrOp::Bool(_) => (1, false),
        // an address
        _ => (8, false),
    };
}

/// `dest = 바꾸다(src)`: cvtss2sd / cvtsd2ss between two 소수, cvtsi2sd from a 정수, cvttsd2si
/// to a 정수 (truncated toward 0, as in C), movsxd / movzx to a wider 정수; else a copy
pub fn generate_cast(dest: &VarId, src: &IrOp, ctx: &Context) -> Vec<Instruction> {
    let (src_size, src_float) = operand_kind(src);
    let (dest_size, dest_float) = (dest.ty.size(), dest.ty.is_float());
    let same_kind = src_float == dest_float && (src_size == dest_size || !dest_float);
    // to a smaller 정수, an address or the same type: the low bytes, as a copy sized by dest
    if same_kind && dest_size <= src_size {
        return generate_copy(dest, src, ctx);
    }
    let mut instrs = vec![];
    // cvt and movsx take no immediate: a 정수 or 논리 constant goes through rax, on 8 bytes
    let (mut src_op, mut src_size) = match src {
        IrOp::Int(..) | IrOp::Bool(_) => {
            instrs.push(load_operand(RAX, src, ctx, None));
            (ElfOper::Reg(RAX), 8)
        }
        _ => (convert_operand(src, ctx), src_size),
    };
    // cvtsi2sd and movsxd want 4 or 8 bytes: a 논리, 문자 or 바이트 is widened by movzx first
    if !src_float && src_size == 1 {
        instrs.push(Instruction::Extend {
            op: ExtendOp::Movzx,
            dst: RAX,
            src: src_op,
            src_size: Some(Size::U8),
            size: Some(Size::U64),
        });
        (src_op, src_size) = (ElfOper::Reg(RAX), 8);
    }
    let (op, reg, size) = match (src_float, dest_float) {
        (true, true) if dest_size == 8 => (ConvOp::Cvtss2sd, XMM0, None),
        (true, true) => (ConvOp::Cvtsd2ss, XMM0, None),
        (false, true) if dest_size == 8 => (ConvOp::Cvtsi2sd, XMM0, Some(Size::from(src_size))),
        (false, true) => (ConvOp::Cvtsi2ss, XMM0, Some(Size::from(src_size))),
        // shortcut: a 1-byte 정수 destination is not done (cvt writes 4 or 8 bytes), add it with 논리
        (true, false) if src_size == 8 => (ConvOp::Cvttsd2si, RAX, var_size(dest)),
        (true, false) => (ConvOp::Cvttss2si, RAX, var_size(dest)),
        // shortcut: sign-extended as a signed 정수, movzx / mov eax for a 부호 없는 one
        (false, false) => {
            // on 8 bytes, the value is already in rax: a constant or a widened 1-byte 정수
            if src_size != 8 {
                instrs.push(Instruction::Extend {
                    op: ExtendOp::Movsxd,
                    dst: RAX,
                    src: src_op,
                    src_size: Some(Size::U32),
                    size: Some(Size::U64),
                });
            }
            instrs.push(Instruction::Binary {
                op: BinOp::Mov,
                dst: convert_operand(&IrOp::Var(dest.clone()), ctx),
                src: ElfOper::Reg(RAX),
                size: var_size(dest),
            });
            return instrs;
        }
    };
    instrs.push(Instruction::Convert {
        op,
        dst: reg,
        src: src_op,
        size,
    });
    instrs.push(Instruction::Binary {
        op: if dest_float {
            BinOp::StoreF
        } else {
            BinOp::Mov
        },
        dst: convert_operand(&IrOp::Var(dest.clone()), ctx),
        src: ElfOper::Reg(reg),
        size: var_size(dest),
    });
    return instrs;
}

/// `dest = src`, sized by `dest`: `mov rax, src ; mov [dest], rax`
pub fn generate_copy(dest: &VarId, src: &IrOp, ctx: &Context) -> Vec<Instruction> {
    let var = convert_operand(&IrOp::Var(dest.clone()), ctx);
    // x86 has no memory to memory mov: the value goes through rax
    return vec![
        load_operand(RAX, src, ctx, var_size(dest)),
        Instruction::Binary {
            op: BinOp::Mov,
            dst: var,
            src: ElfOper::Reg(RAX),
            size: var_size(dest),
        },
    ];
}

/// `call symbol` (System V): the arguments in rdi, rsi, rdx, rcx, r8, r9, al = the number of
/// float arguments (read by variadic functions such as printf), then `mov [dest], rax` if kept
pub fn generate_call(
    dest: &Option<VarId>,
    symbol: &str,
    args: &[IrOp],
    ctx: &Context,
) -> Vec<Instruction> {
    let mut instr: Vec<Instruction> = vec![];
    let mut i = 0;
    let mut i_f = 0;

    for arg in args {
        let new_instr = match arg {
            IrOp::Float(..) => float_parameter(arg, &mut i_f, ctx),
            IrOp::Var(var) if var.ty.is_float() => float_parameter(arg, &mut i_f, ctx),
            _ => normal_param(arg, &mut i, ctx),
        };
        instr.push(new_instr);
    }
    instr.push(Instruction::Binary {
        op: BinOp::Mov,
        dst: ElfOper::Reg(RAX),
        src: ElfOper::Imm(i64::from(i_f)),
        size: None,
    });
    instr.push(Instruction::Ctrl {
        op: CtrlOp::Call,
        target: Sym(symbol.to_string()),
    });
    // the result comes back in rax
    if let Some(var) = dest {
        instr.push(Instruction::Binary {
            op: if var.ty.is_float() {
                BinOp::StoreF
            } else {
                BinOp::Mov
            },
            dst: convert_operand(&IrOp::Var(var.clone()), ctx),
            src: ElfOper::Reg(if var.ty.is_float() { XMM0 } else { RAX }),
            size: var_size(var),
        });
    }
    return instr;
}
