use crate::{
    codegen::{
        arithmetic::{generate_binary, generate_unary},
        instructions::{
            PARAM_FREGS, PARAM_REGS, convert_operand, func_epilogue, generate_call, generate_cast,
            generate_copy, generate_jmp_if_not, generate_return,
        },
        utils::var_size,
    },
    converter::ir::structs::{Function, Instr, Program},
    elf::{
        elfsym::{StBind, StType, StVis, make_st_info},
        file::ElfFile64,
        instructions::{
            BinOp, CtrlOp, Instruction, LabelId, StackOp,
            register::{RBP, RSP},
        },
        shdr::{
            ElfShdr,
            SectionName::{self},
            ShFlags, ShType,
        },
    },
    parser::postparser::structs::Type,
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
        Instr::Cast { dest, src } => generate_cast(dest, src, ctx),
        Instr::Label(lab) => vec![Instruction::Label(ctx.labels[lab.0])],
        Instr::Jump(lab) => vec![Instruction::Ctrl {
            op: CtrlOp::Jmp,
            target: ElfOper::Label(ctx.labels[lab.0]),
        }],
        Instr::JumpIfNot { cond, target } => generate_jmp_if_not(cond, target, ctx),
        // _ => {
        //     println!("{:?}", instr);
        //     unimplemented!()
        // }
    };
}

/// The offset under rbp of each variable: the sizes of the variables up to it, its own included
fn var_offsets(func: &Function) -> Vec<i32> {
    return func
        .vars
        .iter()
        .scan(0, |end, ty| {
            *end += ty.size() as i32;
            Some(*end)
        })
        .collect();
}

/// `push rbp ; mov rbp, rsp ; sub rsp, frame`, the frame being the sizes of the variables rounded
/// up to 16; then each parameter goes from its register into its case
fn func_prologue(func: &Function, ctx: &Context) -> Vec<Instruction> {
    let sum: i64 = func.vars.iter().map(|ty| ty.size() as i64).sum();
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
    for (var, reg) in func
        .params
        .iter()
        .filter(|p| !p.ty.is_float())
        .zip(PARAM_REGS)
    {
        prologue.push(Instruction::Binary {
            op: BinOp::Mov,
            dst: convert_operand(&IrOp::Var(var.clone()), ctx),
            src: ElfOper::Reg(reg),
            size: var_size(var),
        });
    }
    for (var, reg) in func
        .params
        .iter()
        .filter(|p| p.ty.is_float())
        .zip(PARAM_FREGS)
    {
        prologue.push(Instruction::Binary {
            op: BinOp::StoreF,
            dst: convert_operand(&IrOp::Var(var.clone()), ctx),
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
    ///
    pub return_type: Option<Type>,
    /// The offset under rbp of each variable, see var_offsets
    pub offsets: Vec<i32>,
    /// The symbol of each string of the program: Operand::Str(id) is strings[id]
    pub data_names: &'a [String],
    /// The symbol of each float constant: Operand::Float(id) is float_names[id]
    pub float_names: &'a [String],
    /// Each float constant and whether it is a 짧은 소수 (4 bytes): its only size
    pub floats: &'a [(f64, bool)],
    /// The label of each IR label of the function: Label(id) is labels[id]
    pub labels: Vec<LabelId>,
}

/// The object file ostrf `prog`: each string in .rodata as a local symbol `.str.N`, each function
/// with a body in .text as a global symbol; an 외부 function becomes an undefined symbol
pub fn generate(prog: &Program) -> ElfFile64 {
    let mut elf_file = ElfFile64::default();
    let text = add_section(&mut elf_file, SectionName::Text, ShFlags::ExecInstr, 16);
    let _data = add_section(&mut elf_file, SectionName::Data, ShFlags::Write, 4);
    let rodata = add_section(&mut elf_file, SectionName::Rodata, ShFlags::NoFlag, 1);
    let data_names: Vec<String> = (0..prog.strings.len())
        .map(|id| format!(".dt.{id}"))
        .collect();
    let float_names: Vec<String> = (0..prog.floats.len())
        .map(|id| format!(".float.{id}"))
        .collect();

    // each string ends with 0, as C expects it
    for (name, string) in data_names.iter().zip(&prog.strings) {
        let mut bytes = string.as_bytes().to_vec();
        bytes.push(0);
        add_data_to_roddata(&mut elf_file, rodata, name, &bytes, 1);
    }
    for (name, float) in float_names.iter().zip(&prog.floats) {
        let bytes: Vec<u8> = if float.1 {
            (float.0 as f32).to_le_bytes().to_vec()
        } else {
            float.0.to_le_bytes().to_vec()
        };
        add_data_to_roddata(&mut elf_file, rodata, name, &bytes, 8);
    }

    for func in &prog.funcs {
        let Some(body) = &func.body else {
            continue; // 외부 : pas de corps
        };
        let ctx = Context {
            offsets: var_offsets(func),
            data_names: &data_names,
            float_names: &float_names,
            floats: &prog.floats,
            // IR labels are numbered per function, a LabelId is unique in the whole object file
            labels: func.labels.iter().map(|_| LabelId::new()).collect(),
            return_type: func.return_type.clone(),
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

fn add_data_to_roddata(
    elf_file: &mut ElfFile64,
    rodata: usize,
    name: &String,
    bytes: &Vec<u8>,
    align: usize,
) {
    elf_file.add_symbol_to_section_raw(
        rodata,
        name.clone(),
        &bytes,
        make_st_info(StBind::Local, StType::Object),
        StVis::Default as u8,
        align,
    );
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
