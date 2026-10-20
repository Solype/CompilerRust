use crate::{
    codegen::instructions::{func_epilogue, generate_call, generate_return},
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

use crate::elf::instructions::Operand as ElfOper;

fn generate_instruction(instr: &Instr) -> Vec<Instruction> {
    return match instr {
        Instr::Return(op) => generate_return(op),
        Instr::Comment(_) => vec![],
        Instr::Call { dest, symbol, args } => generate_call(dest, symbol, args),
        _ => {
            println!("{:?}", instr);
            unimplemented!()
        }
    };
}

fn func_prologue(func: &Function) -> Vec<Instruction> {
    let sum: i64 = func.vars.iter().map(get_size_of).sum();
    let frame = (sum + 15) & !15;

    return vec![
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
}

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

pub fn generate(prog: &Program) -> ElfFile64 {
    let mut elf_file = ElfFile64::default();
    let text = add_section(&mut elf_file, SectionName::Text, ShFlags::ExecInstr, 16);
    let _data = add_section(&mut elf_file, SectionName::Data, ShFlags::Write, 4);

    for func in &prog.funcs {
        let Some(body) = &func.body else {
            continue; // 외부 : pas de corps
        };
        let mut bin_instr: Vec<Instruction> = func_prologue(func);

        for instr in body.iter() {
            let mut new_instr = generate_instruction(instr);
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
