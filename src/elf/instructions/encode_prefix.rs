use super::{
    enums::*, 
    struct_encode_information::*,
};

fn is_segment_override_compatible(ins: &Instruction) -> bool {
    match ins {

        // ==========================================
        // Any instruction touching memory
        // ==========================================
        Instruction::Binary { dst, src, .. } => {
            matches!(dst, Operand::MemoryAddress(_))
                || matches!(src, Operand::MemoryAddress(_))
        }

        Instruction::ComplexBinary { dst, src, .. } => {
            matches!(dst, Operand::MemoryAddress(_))
                || matches!(src, Operand::MemoryAddress(_))
        }

        Instruction::Unary { dst, .. } => {
            matches!(dst, Operand::MemoryAddress(_))
        }

        Instruction::Bit { dst, src, .. } => {
            matches!(dst, Operand::MemoryAddress(_))
                || matches!(src, Operand::MemoryAddress(_))
        }

        Instruction::CMovCC { src, .. } => {
            matches!(src, Operand::MemoryAddress(_))
        }

        Instruction::Lea { src, .. } => {
            matches!(src, Operand::MemoryAddress(_))
        }

        Instruction::Shift { dst, .. } => {
            matches!(dst, Operand::MemoryAddress(_))
        }

        _ => false,
    }
}

fn is_lock_compatible(ins: &Instruction) -> bool {
    match ins {

        // ==========================================
        // Atomic read-modify-write instructions
        // ==========================================
        Instruction::ComplexBinary { op, dst, .. } => {
            matches!(op, ComplexBinOp::Xadd | ComplexBinOp::Cmpxchg)
                && matches!(dst, Operand::MemoryAddress(_))
        }

        // ==========================================
        // Bit test instructions on memory
        // lock bts [mem], eax
        // ==========================================
        Instruction::Bit { dst, .. } => {
            matches!(dst, Operand::MemoryAddress(_))
        }

        // ==========================================
        // ALU instructions on memory
        // lock add [mem], eax
        // ==========================================
        Instruction::Binary { op, dst, .. } => {
            matches!(op, BinOp::Add | BinOp::Sub | BinOp::Adc | BinOp::Sbb | BinOp::And | BinOp::Or | BinOp::Xor | BinOp::Xchg )
            && matches!(dst, Operand::MemoryAddress(_))
        }

        _ => false,
    }
}

fn is_rep_compatible(ins: &Instruction) -> bool {
    match ins {
        Instruction::String { op, .. } => {
            matches!( op, StringOp::Movs | StringOp::Stos | StringOp::Lods )
        }
        _ => false,
    }
}

fn is_repe_compatible(ins: &Instruction) -> bool {
    match ins {
        Instruction::String { op, .. } => {
            matches!( op, StringOp::Cmps | StringOp::Scas )
        }
        _ => false,
    }
}

pub(super) fn encode_prefix(
    prefixes: &Vec<Prefix>,
    ins: &Instruction,
    default_size: Size,
) -> EncodeInformation {

    // ==========================================
    // Prevent nested prefixes
    // ==========================================
    if let Instruction::Prefix { .. } = ins {
        panic!("Nested prefix instructions are forbidden");
    }

    // ==========================================
    // Prefix consistency checks
    // ==========================================
    let has_lock  = prefixes.iter().any(|p| matches!(p, Prefix::Lock));
    let has_rep   = prefixes.iter().any(|p| matches!(p, Prefix::Rep));
    let has_repe  = prefixes.iter().any(|p| matches!(p, Prefix::Repe));
    let has_repne = prefixes.iter().any(|p| matches!(p, Prefix::Repne));

    // LOCK cannot coexist with REP*
    if has_lock && (has_rep || has_repe || has_repne) {
        panic!("LOCK cannot be combined with REP/REPE/REPNE");
    }

    // REP family mutually exclusive
    let rep_count =
        has_rep as u8 +
        has_repe as u8 +
        has_repne as u8;

    if rep_count > 1 {
        panic!("REP / REPE / REPNE are mutually exclusive");
    }

    // ==========================================
    // Validate individual prefixes
    // ==========================================
    for p in prefixes {

        match p {
            Prefix::Cs | Prefix::Ds | Prefix::Es | Prefix::Ss | Prefix::Fs | Prefix::Gs => {
                if !is_segment_override_compatible(ins) {
                    panic!("Segment override prefix not valid for instruction: {:?}", ins );
                }
            }

            Prefix::Lock => {
                if !is_lock_compatible(ins) {
                    panic!("LOCK prefix not valid for instruction: {:?}", ins);
                }
            }
            Prefix::Rep => {
                if !is_rep_compatible(ins) {
                    panic!("REP prefix not valid for instruction: {:?}", ins);
                }
            }
            Prefix::Repe | Prefix::Repne => {
                if !is_repe_compatible(ins) {
                    panic!("{:?} prefix not valid for instruction: {:?}", p, ins);
                }
            }
        }
    }

    let mut enc = ins.encode(default_size);
    let mut prefix_bytes = Vec::with_capacity(prefixes.len());

    for p in prefixes {

        let byte = match p {
            Prefix::Lock  => 0xF0,
            Prefix::Rep   => 0xF3,
            Prefix::Repe  => 0xF3,
            Prefix::Repne => 0xF2,
            Prefix::Es => 0x26,
            Prefix::Cs => 0x2E,
            Prefix::Ss => 0x36,
            Prefix::Ds => 0x3E,
            Prefix::Fs => 0x64,
            Prefix::Gs => 0x65,
        };

        prefix_bytes.push(byte);
    }
    let prefix_len = prefix_bytes.len();
    let mut data =
        Vec::with_capacity(prefix_len + enc.data.len());

    data.extend(prefix_bytes);
    data.extend(enc.data);
    enc.relocations.iter_mut().for_each(|rel| { rel.offset += prefix_len });
    enc.data = data;

    enc
}
