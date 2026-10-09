use crate::parser::cfg::parsing_table::ParsingState::{self, Accept};

use super::super::parser_token::ParserToken;

use super::parsing_table::ParsingTable;
use super::structs::ParserProduction;

pub struct Parser {
    /// For a reduce: the length of the right side of the rule, and its left side
    rules: &'static [ParserProduction],
    table: ParsingTable,
}

impl Parser {
    pub fn new(rules: &'static [ParserProduction]) -> Self {
        Self {
            rules,
            table: ParsingTable::new(rules),
        }
    }

    /// Reads the tokens of the pre-parser with the table: `Ok` if the program follows the grammar,
    /// else the first token that does not fit
    pub fn parse<'a>(&self, tokens: &'a [ParserToken]) -> Result<(), &'a ParserToken> {
        let mut stack: Vec<usize> = vec![0];
        let mut pos = 0;

        loop {
            let Some(current_state) = stack.last() else {
                panic!();
            };
            let token = &tokens[pos];
            let cell = self.table.get(*current_state, token);
            match cell {
                Some(ParsingState::S(n)) => {
                    stack.push(n);
                    pos += 1;
                }
                Some(ParsingState::R(r)) => {
                    stack.truncate(stack.len() - self.rules[r].right.len());
                    let Some(new_top) = stack.last() else {
                        return Err(token);
                    };
                    let Some(new) = self.table.get_goto(*new_top, self.rules[r].left) else {
                        return Err(token);
                    };
                    stack.push(new);
                }
                Some(Accept) => break,
                None => return Err(token),
                _ => unreachable!(),
            }
        }
        Ok(())
    }
}
