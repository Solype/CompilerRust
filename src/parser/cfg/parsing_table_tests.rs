use super::parsing_table::ParsingTable;
use super::rules::RULES;

/// `LR_DEBUG=1 cargo test closure_step_by_step -- --ignored --nocapture`: the transitions
/// `LR_DEBUG=closure …`: also each closure step by step, with Enter to go on
#[test]
#[ignore]
fn closure_step_by_step() {
    ParsingTable::new(RULES);
}
