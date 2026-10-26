use crate::{
    codegen::{
        arithmetic::{generate_binary, generate_unary},
        instructions::{
            PARAM_REGS, convert_operand, func_epilogue, generate_call, generate_copy,
            generate_return,
        },
        utils::var_size,
    },
    converter::ir::structs::{Function, Instr, Program, get_size_of},
    elf::{
        elfsym::{StBind, StType, StVis, make_st_info},
        file::ElfFile64,
        instructions::{
            BinOp, CtrlOp, Instruction, StackOp,
            register::{RBP, RSP},
        },
        shdr::{ElfShdr, SectionName, ShFlags, ShType},
    },
};

use crate::converter::ir::structs::Operand as IrOp;
use crate::elf::instructions::Operand as ElfOper;

/// The x86 instructions of one IR instruction; panics on one not done yet (Unary, Cast, Label,
/// Jump, JumpIfNot)
fn generate_instruction(instr: &Instr, ctx: &Context) -> Vec<Instruction> {
    return match instr {
        Instr::Return(op) => generate_return(op, ctx),
        Instr::Comment(_) => vec![],
        Instr::Call { dest, symbol, args } => generate_call(dest, symbol, args, ctx),
        Instr::Binary {
            dest,
            op,
            left,
            right,
        } => generate_binary(dest, op, left, right, ctx),
        Instr::Unary { dest, op, src } => generate_unary(dest, op, src, ctx),
        Instr::Copy { dest, src } => generate_copy(dest, src, ctx),
        _ => {
            println!("{:?}", instr);
            unimplemented!()
        }
    };
}

/// The offset under rbp of each variable: the sizes of the variables up to it, its own included
fn var_offsets(func: &Function) -> Vec<i32> {
    return func
        .vars
        .iter()
        .scan(0, |end, ty| {
            *end += get_size_of(ty) as i32;
            Some(*end)
        })
        .collect();
}

/// `push rbp ; mov rbp, rsp ; sub rsp, frame`, the frame being the sizes of the variables rounded
/// up to 16; then each parameter goes from its register into its case
fn func_prologue(func: &Function, ctx: &Context) -> Vec<Instruction> {
    let sum: i64 = func.vars.iter().map(get_size_of).sum();
    let frame = (sum + 15) & !15;

    let mut prologue = vec![
        Instruction::Stack {
            op: StackOp::Push,
            value: ElfOper::Reg(RBP),
            size: None,
        },
        Instruction::Binary {
            op: BinOp::Mov,
            dst: ElfOper::Reg(RBP),
            src: ElfOper::Reg(RSP),
            size: None,
        },
        Instruction::Binary {
            op: BinOp::Sub,
            dst: ElfOper::Reg(RSP),
            src: ElfOper::Imm(frame),
            size: None,
        },
    ];
    // each parameter, given in its register, goes into its case
    // shortcut: no 7th integer parameter (it comes on the stack), add it for more than 6
    for (var, reg) in func.params.iter().zip(PARAM_REGS) {
        prologue.push(Instruction::Binary {
            op: BinOp::Mov,
            dst: convert_operand(&IrOp::Var(*var), ctx),
            src: ElfOper::Reg(reg),
            size: var_size(var),
        });
    }
    return prologue;
}

/// True unless the code already ends with `ret`: a function without 반환하는 falls off its end
fn need_epilogue(bin_instr: &[Instruction]) -> bool {
    return !bin_instr.last().is_some_and(|last| {
        matches!(
            last,
            Instruction::Ctrl {
                op: CtrlOp::Ret,
                target: ElfOper::NoOperand,
            }
        )
    });
}

/// What the code of a function needs to know besides its instructions
pub struct Context<'a> {
    /// The offset under rbp of each variable, see var_offsets
    pub offsets: Vec<i32>,
    /// The symbol of each string of the program: Operand::Str(id) is strings[id]
    pub strings: &'a [String],
}

/// The object file of `prog`: each string in .rodata as a local symbol `.str.N`, each function
/// with a body in .text as a global symbol; an 외부 function becomes an undefined symbol
pub fn generate(prog: &Program) -> ElfFile64 {
    let mut elf_file = ElfFile64::default();
    let text = add_section(&mut elf_file, SectionName::Text, ShFlags::ExecInstr, 16);
    let _data = add_section(&mut elf_file, SectionName::Data, ShFlags::Write, 4);
    let rodata = add_section(&mut elf_file, SectionName::Rodata, ShFlags::NoFlag, 1);
    let strings: Vec<String> = (0..prog.strings.len())
        .map(|id| format!(".str.{id}"))
        .collect();

    // each string ends with 0, as C expects it
    for (name, string) in strings.iter().zip(&prog.strings) {
        let mut bytes = string.as_bytes().to_vec();
        bytes.push(0);
        elf_file.add_symbol_to_section_raw(
            rodata,
            name.clone(),
            &bytes,
            make_st_info(StBind::Local, StType::Object),
            StVis::Default as u8,
            1,
        );
    }

    for func in &prog.funcs {
        let Some(body) = &func.body else {
            continue; // 외부 : pas de corps
        };
        let ctx = Context {
            offsets: var_offsets(func),
            strings: &strings,
        };
        let mut bin_instr: Vec<Instruction> = func_prologue(func, &ctx);

        for instr in body.iter() {
            let mut new_instr = generate_instruction(instr, &ctx);
            bin_instr.append(&mut new_instr);
        }
        if need_epilogue(&bin_instr) {
            bin_instr.append(&mut func_epilogue());
        }
        add_function(&mut elf_file, text, &func.symbol, bin_instr);
    }
    return elf_file;
}

/// Adds an allocated PROGBITS section, with one extra flag (Write, ExecInstr, ...)
fn add_section(elf_file: &mut ElfFile64, name: SectionName, flag: ShFlags, align: u64) -> usize {
    elf_file.add_section(
        name.as_str().to_string(),
        ElfShdr {
            sh_type: ShType::ProgBits as u32,
            sh_flags: ShFlags::Alloc as u64 | flag as u64,
            sh_addralign: align,
            ..Default::default()
        },
    )
}

/// Adds a global function whose body is encoded from `code`
fn add_function(elf_file: &mut ElfFile64, section: usize, name: &str, code: Vec<Instruction>) {
    elf_file.add_symbol_to_section(
        section,
        name.to_string(),
        &code,
        make_st_info(StBind::Global, StType::Func),
        StVis::Default as u8,
    );
}
