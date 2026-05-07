use super::enums::*;

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
            matches!(
                op,
                BinOp::Add
                    | BinOp::Sub
                    | BinOp::Adc
                    | BinOp::Sbb
                    | BinOp::And
                    | BinOp::Or
                    | BinOp::Xor
                    | BinOp::Xchg
            ) && matches!(dst, Operand::MemoryAddress(_))
        }

        _ => false,
    }
}

fn is_rep_compatible(ins: &Instruction) -> bool {
    match ins {

        // ==========================================
        // REP-compatible string instructions
        // ==========================================
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

        // ==========================================
        // REPE/REPNE-compatible instructions
        // ==========================================
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
    // Encode underlying instruction
    // ==========================================
    let mut enc = ins.encode(default_size);

    // ==========================================
    // Emit prefixes
    // ==========================================
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

    // ==========================================
    // Final encoding
    // ==========================================
    let prefix_len = prefix_bytes.len();
    let mut data =
        Vec::with_capacity(prefix_len + enc.data.len());

    data.extend(prefix_bytes);
    data.extend(enc.data);
    enc.relocations.iter_mut().for_each(|rel| { rel.offset += prefix_len });
    enc.data = data;

    enc
}
