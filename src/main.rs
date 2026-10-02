mod elf;
mod samples;

use std::fs::File;

use elf::elfsym::{make_st_info, StBind, StType, StVis};
use elf::file::{natural_alignment, ElfFile64, SymbolType};
use elf::instructions::{Instruction, RelocKind, Relocation};
use elf::shdr::{ElfShdr, SectionName, ShFlags, ShType};

const OUTPUT: &str = "output.elf";

fn main() -> std::io::Result<()> {
    let mut elf_file = ElfFile64::default();
    elf_file.declare_non_defined_sym(&"my_exit".to_string(), SymbolType::Function);

    let data = add_section(&mut elf_file, SectionName::Data, ShFlags::Write, 4);
    add_object(&mut elf_file, data, "my_data", b"abcdefg\0");
    add_object(&mut elf_file, data, "my_float", &0.1f64.to_le_bytes());
    add_object(&mut elf_file, data, "my_lock", &0u32.to_le_bytes());
    // pointers resolved by ld: a data symbol and a function defined below
    add_pointers(&mut elf_file, data, "my_ptrs", &["my_data", "jumps_and_calls"]);

    let text = add_section(&mut elf_file, SectionName::Text, ShFlags::ExecInstr, 16);
    for (name, code) in samples::families() {
        add_function(&mut elf_file, text, name, code);
    }
    add_function(&mut elf_file, text, "_start", samples::start());

    elf_file.write(&mut File::create(OUTPUT)?)?;
    println!("ELF written: {OUTPUT}");
    Ok(())
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

/// Adds an initialized global variable, at its natural alignment
fn add_object(elf_file: &mut ElfFile64, section: usize, name: &str, bytes: &[u8]) {
    elf_file.add_symbol_to_section_raw(
        section,
        name.to_string(),
        &bytes.to_vec(),
        make_st_info(StBind::Global, StType::Object),
        StVis::Default as u8,
        natural_alignment(bytes.len()),
    );
}

/// Adds a global array of 8-byte pointers to `targets` (`.quad a, b, ...`)
fn add_pointers(elf_file: &mut ElfFile64, section: usize, name: &str, targets: &[&str]) {
    let relocations: Vec<Relocation> = targets
        .iter()
        .enumerate()
        .map(|(i, sym)| Relocation {
            sym: sym.to_string(),
            offset: i * 8,
            size: 8,
            kind: RelocKind::Absolute,
            addend: 0,
        })
        .collect();

    elf_file.add_symbol_to_section_relocated(
        section,
        name.to_string(),
        &vec![0; targets.len() * 8],
        make_st_info(StBind::Global, StType::Object),
        StVis::Default as u8,
        8,
        &relocations,
    );
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
