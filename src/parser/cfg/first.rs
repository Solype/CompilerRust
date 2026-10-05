use std::collections::{HashMap, HashSet};

use crate::parser::parser_token::ParserTokenKind;

use super::structs::{NTerm, ParserNode, ParserProduction};

/// The non-terminals that can derive ε, and the terminals each non-terminal can start with.
/// Terminals are compared with `==`: the rules all use the same placeholder values (`term!`)
pub struct First {
    pub nullable: HashSet<NTerm>,
    pub first: HashMap<NTerm, Vec<&'static ParserTokenKind>>,
}

impl First {
    pub fn new(rules: &'static [ParserProduction]) -> Self {
        let nullable = nullable(rules);
        let first = first_sets(rules, &nullable);
        Self { nullable, first }
    }

    /// FIRST of a sequence of nodes, and whether the whole sequence can derive ε
    pub fn of_sequence(
        &self,
        nodes: &'static [ParserNode],
    ) -> (Vec<&'static ParserTokenKind>, bool) {
        let mut set = Vec::new();
        for node in nodes {
            match node {
                ParserNode::Term(kind) => {
                    insert(&mut set, kind);
                    return (set, false);
                }
                ParserNode::NTerm(non_terminal) => {
                    for kind in self.first.get(non_terminal).into_iter().flatten() {
                        insert(&mut set, kind);
                    }
                    if !self.nullable.contains(non_terminal) {
                        return (set, false);
                    }
                }
            }
        }
        (set, true)
    }
}

/// Fixed point: a rule whose right side is only nullable non-terminals (or empty) makes its left
/// side nullable
fn nullable(rules: &[ParserProduction]) -> HashSet<NTerm> {
    let mut nullable = HashSet::new();
    let mut changed = true;
    while changed {
        changed = false;
        for rule in rules {
            if nullable.contains(&rule.left) {
                continue;
            }
            let all_nullable = rule.right.iter().all(|node| match node {
                ParserNode::Term(_) => false,
                ParserNode::NTerm(non_terminal) => nullable.contains(non_terminal),
            });
            if all_nullable {
                nullable.insert(rule.left);
                changed = true;
            }
        }
    }
    nullable
}

/// Fixed point: for `A → X1 X2 …`, FIRST(A) gets FIRST(X1), then FIRST(X2) if X1 is nullable, and
/// so on
fn first_sets(
    rules: &'static [ParserProduction],
    nullable: &HashSet<NTerm>,
) -> HashMap<NTerm, Vec<&'static ParserTokenKind>> {
    let mut first: HashMap<NTerm, Vec<&'static ParserTokenKind>> = HashMap::new();
    let mut changed = true;
    while changed {
        changed = false;
        for rule in rules {
            let mut found = Vec::new();
            for node in rule.right {
                match node {
                    ParserNode::Term(kind) => {
                        found.push(kind);
                        break;
                    }
                    ParserNode::NTerm(non_terminal) => {
                        found.extend(first.get(non_terminal).into_iter().flatten());
                        if !nullable.contains(non_terminal) {
                            break;
                        }
                    }
                }
            }
            let set = first.entry(rule.left).or_default();
            for kind in found {
                changed |= insert(set, kind);
            }
        }
    }
    first
}

/// Adds `kind` if it is not there yet; true if it was added
fn insert(set: &mut Vec<&'static ParserTokenKind>, kind: &'static ParserTokenKind) -> bool {
    if set.contains(&kind) {
        return false;
    }
    set.push(kind);
    true
}
