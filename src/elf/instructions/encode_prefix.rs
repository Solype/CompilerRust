use super::enums::*;

fn is_lock_compatible(ins: &Instruction) -> bool {
    match ins {
        // ==========================================
        // Atomic RMW instructions
        // ==========================================
        Instruction::ComplexBinary { op, src, .. } => {
            matches!(op, ComplexBinOp::Xadd | ComplexBinOp::Cmpxchg)
                && matches!(src, Operand::MemoryAddress(_))
        }

        // ==========================================
        // Bit operations on memory
        // ==========================================
        Instruction::Bit { dst, .. } => {
            matches!(dst, Operand::MemoryAddress(_))
        }

        // ==========================================
        // ALU ops on memory
        // lock add [mem], eax
        // ==========================================
        Instruction::Binary { op, dst, .. } => {
            matches!(
                op,
                BinOp::Add
                    | BinOp::Sub
                    | BinOp::Adc
                    | BinOp::Sbb
                    | BinOp::And
                    | BinOp::Or
                    | BinOp::Xor
            ) && matches!(dst, Operand::MemoryAddress(_))
        }

        _ => false,
    }
}

fn is_rep_compatible(ins: &Instruction) -> bool {
    match ins {
        Instruction::String { op, .. } => {
            matches!(
                op,
                StringOp::Movs
                    | StringOp::Stos
                    | StringOp::Lods
            )
        }

        _ => false,
    }
}

fn is_repe_compatible(ins: &Instruction) -> bool {
    match ins {
        Instruction::String { op, .. } => {
            matches!(
                op,
                StringOp::Cmps
                    | StringOp::Scas
            )
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
        panic!("Cannot apply prefix on another prefix instruction");
    }

    // ==========================================
    // Validate prefixes
    // ==========================================
    for p in prefixes {

        match p {

            // --------------------------------------
            // LOCK
            // --------------------------------------
            Prefix::Lock => {
                if !is_lock_compatible(ins) {
                    panic!("LOCK prefix not valid for instruction: {:?}", ins);
                }
            }

            // --------------------------------------
            // REP
            // --------------------------------------
            Prefix::Rep => {
                if !is_rep_compatible(ins) {
                    panic!("REP prefix not valid for instruction: {:?}", ins);
                }
            }

            // --------------------------------------
            // REPE / REPZ
            // --------------------------------------
            Prefix::Repe => {
                if !is_repe_compatible(ins) {
                    panic!("REPE prefix not valid for instruction: {:?}", ins);
                }
            }

            // --------------------------------------
            // REPNE / REPNZ
            // --------------------------------------
            Prefix::Repne => {
                if !is_repe_compatible(ins) {
                    panic!("REPNE prefix not valid for instruction: {:?}", ins);
                }
            }
        }
    }

    // ==========================================
    // Encode instruction
    // ==========================================
    let mut enc = ins.encode(default_size);

    let mut prefix_bytes = Vec::with_capacity(prefixes.len());

    for p in prefixes {
        let byte = match p {
            Prefix::Lock  => 0xF0,
            Prefix::Rep   => 0xF3,
            Prefix::Repe  => 0xF3,
            Prefix::Repne => 0xF2,
        };

        prefix_bytes.push(byte);
    }

    let mut data = Vec::with_capacity(prefix_bytes.len() + enc.data.len());

    data.extend(prefix_bytes);
    data.extend(enc.data);

    enc.data = data;

    enc
}
