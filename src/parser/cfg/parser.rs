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

        // Loop, with `state` = top of `stack` and `token` = tokens[pos]:
        // 1. Look up the cell Action(state, token)
        // 2. S(n): push n, then go to the next token
        // 3. R(r): pop as many states as `self.rules[r].right` has nodes, then push
        //    Goto(new top, `self.rules[r].left`); the token is not read, it is looked at again
        // 4. Accept: return Ok
        // 5. No cell: return Err(token)

        todo!()
    }
}
