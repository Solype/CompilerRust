
use super::enums::*;

pub(super) fn encode_prefix(
    prefixes: &Vec<Prefix>,
    ins: &Instruction,
    default_size: Size,
) -> EncodeInformation {

    if let Instruction::Prefix { .. } = ins {
        panic!("Cannot apply prefix on another prefix instruction");
    }

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