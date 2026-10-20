use crate::{
    codegen::instructions::generate_return,
    converter::ir::structs::{Instr, Program},
    elf::{
        elfsym::{StBind, StType, StVis, make_st_info},
        file::ElfFile64,
        instructions::Instruction,
        shdr::{ElfShdr, SectionName, ShFlags, ShType},
    },
};

fn generate_instruction(instr: &Instr) -> Vec<Instruction> {
    return match instr {
        Instr::Return(op) => generate_return(op),
        Instr::Comment(_) => vec![],
        _ => {
            println!("{:?}", instr);
            unimplemented!()
        }
    };
}

pub fn generate(prog: &Program) -> ElfFile64 {
    let mut elf_file = ElfFile64::default();
    let text = add_section(&mut elf_file, SectionName::Text, ShFlags::ExecInstr, 16);
    let _data = add_section(&mut elf_file, SectionName::Data, ShFlags::Write, 4);

    for func in &prog.funcs {
        let Some(body) = &func.body else {
            continue; // 외부 : pas de corps
        };
        let mut bin_instr: Vec<Instruction> = vec![];

        for instr in body.iter() {
            let mut new_instr = generate_instruction(&instr);
            bin_instr.append(&mut new_instr);
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
