use crate::lexer::span::Span;

use super::parsing_table::ParsingState::{self, Accept};

use super::super::parser_token::ParserToken;

use super::parsing_table::ParsingTable;
use super::structs::ParserProduction;

/// A node of the parse tree: a token read by a shift, or a rule built by a reduce
pub enum TreeNode<'a> {
    /// Borrowed from the tokens given to `parse`
    Leaf(&'a ParserToken),
    /// The rule `rule` (index in `rules`), with one child per node of its right side
    Branch {
        rule: usize,
        children: Vec<TreeNode<'a>>,
    },
}

impl<'a> TreeNode<'a> {
    /// From the start of the first token to the end of the last one; `None` for an empty rule (ε)
    pub fn span(&self) -> Option<Span> {
        let first = self.first_token()?;
        let last = self.last_token()?;
        return Some(Span::new(first.span.start, last.span.end));
    }

    fn first_token(&self) -> Option<&'a ParserToken> {
        return match self {
            TreeNode::Leaf(token) => Some(token),
            TreeNode::Branch { children, .. } => children.iter().find_map(TreeNode::first_token),
        };
    }

    fn last_token(&self) -> Option<&'a ParserToken> {
        return match self {
            TreeNode::Leaf(token) => Some(token),
            TreeNode::Branch { children, .. } => {
                children.iter().rev().find_map(TreeNode::last_token)
            }
        };
    }

    /// One line per node, indented by its depth: the left side of the rule for a branch, the
    /// token for a leaf
    pub fn show(&self, rules: &[ParserProduction]) -> String {
        let mut lines = Vec::new();
        self.show_lines(rules, 0, &mut lines);
        return lines.join("\n");
    }

    fn show_lines(&self, rules: &[ParserProduction], depth: usize, lines: &mut Vec<String>) {
        let indent = "  ".repeat(depth);
        match self {
            TreeNode::Leaf(token) => lines.push(format!("{indent}{:?}", token.kind)),
            TreeNode::Branch { rule, children } => {
                lines.push(format!("{indent}{:?}", rules[*rule].left));
                for child in children {
                    child.show_lines(rules, depth + 1, lines);
                }
            }
        }
    }
}

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

    /// Reads the tokens of the pre-parser with the table: the tree of the program (its `Items`
    /// node: `Eof` is never shifted) if it follows the grammar, else the first token that does not
    /// fit
    pub fn parse<'a>(&self, tokens: &'a [ParserToken]) -> Result<TreeNode<'a>, &'a ParserToken> {
        let mut stack: Vec<usize> = vec![0];
        // Next to `stack`: one piece of tree per state above the state 0
        let mut nodes: Vec<TreeNode<'a>> = Vec::new();
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
                    nodes.push(TreeNode::Leaf(token));
                    pos += 1;
                }
                Some(ParsingState::R(r)) => {
                    stack.truncate(stack.len() - self.rules[r].right.len());
                    // The last pieces are the right side of the rule, in order
                    let children = nodes.split_off(nodes.len() - self.rules[r].right.len());
                    let Some(new_top) = stack.last() else {
                        return Err(token);
                    };
                    let Some(new) = self.table.get_goto(*new_top, self.rules[r].left) else {
                        return Err(token);
                    };
                    stack.push(new);
                    nodes.push(TreeNode::Branch { rule: r, children });
                }
                // `StartSymbol → Items · Eof`: the only piece left is the `Items` node
                Some(Accept) => return Ok(nodes.pop().unwrap()),
                None => return Err(token),
                _ => unreachable!(),
            }
        }
    }
}
