use super::super::parser_token::{ParserToken, ParserTokenKind};

use super::first::First;
use super::structs::NTerm::StartSymbol;
use super::structs::{NTerm, ParserNode, ParserProduction, same_terminal};
use std::collections::{BTreeSet, HashMap};
use std::fmt;
use std::hash::{Hash, Hasher};
use std::mem::discriminant;

fn token_matches_node(token: &ParserTokenKind, node: &ParserNode) -> bool {
    return match node {
        ParserNode::Term(kind) => same_terminal(token, kind),
        ParserNode::NTerm(_) => false,
    };
}

/// Same terminal (compared on its kind) or same non-terminal
fn same_node(a: &ParserNode, b: &ParserNode) -> bool {
    return match (a, b) {
        (ParserNode::Term(a), ParserNode::Term(b)) => same_terminal(a, b),
        (ParserNode::NTerm(a), ParserNode::NTerm(b)) => a == b,
        _ => false,
    };
}

/// A terminal compared on its kind: `Name("main")` == `Name("")`
#[derive(Clone, Copy)]
struct TerminalKey<'a>(&'a ParserTokenKind);

impl PartialEq for TerminalKey<'_> {
    fn eq(&self, other: &Self) -> bool {
        same_terminal(self.0, other.0)
    }
}

impl Eq for TerminalKey<'_> {}

impl fmt::Debug for TerminalKey<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl Hash for TerminalKey<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Only the variant: equal terminals always have the same variant
        discriminant(self.0).hash(state);
    }
}

////////////////////////////////
// Actual Algo
////////////////////////////////

/// A bookmark in a rule: `ReturnType → 정수 · 를 주는   [Name]`
/// "I have read `정수`, I expect `를 주는`, and once the rule is done the next token is `Name`"
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug, Default)]
struct Item {
    /// Index of the rule in `rules`
    rule: usize,
    /// Position of the `·` in the right side: 0 = nothing read yet, `right.len()` = rule done
    point: usize,
    /// Id (index in `terms`) of the token expected right after the whole rule: when the rule is
    /// done and this token comes, the parser reduces
    lookahead: usize,
}

impl Item {
    pub fn new(rule: usize, lookahead: usize) -> Self {
        return Self {
            rule: rule,
            lookahead: lookahead,
            ..Default::default()
        };
    }

    pub fn from_rule(rule_id: usize, term_id: usize) -> Self {
        return Self {
            rule: rule_id,
            point: 0,
            lookahead: term_id,
        };
    }

    /// The node just after the point, `None` when the point is at the end of the rule
    pub fn next_node(&self, rules: &'static [ParserProduction]) -> Option<&'static ParserNode> {
        if self.point >= rules[self.rule].right.len() {
            return None;
        }
        return Some(&rules[self.rule].right[self.point]);
    }

    /// The same item with its point one node further: the node after the point has been read
    pub fn advance(&self) -> Self {
        return Self {
            rule: self.rule,
            point: self.point + 1,
            lookahead: self.lookahead,
        };
    }
}

/// The content of a cell of the table: what the parser does
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ParsingState {
    /// Shift: read the token and push the state n
    S(usize),
    /// Reduce: the rule n is done, replace its right side on the stack by its left side
    R(usize),
    /// `Eof` after `StartSymbol → Items · Eof`: the whole program is valid
    Accept,
    /// After a reduce to a non-terminal, push the state n
    Goto(usize),
}

/// A cell of the table: a state and the symbol seen in it
#[derive(PartialEq, Eq, Hash, Debug)]
enum ParsingEntry<'a> {
    /// ACTION[state, terminal]
    Action(usize, TerminalKey<'a>),
    /// GOTO[state, non-terminal]
    Goto(usize, NTerm),
}

/// The LR(1) table: for each state and each symbol, what the parser does
pub struct ParsingTable {
    /// Only the filled cells: an empty cell is a syntax error
    table: HashMap<ParsingEntry<'static>, ParsingState>,
    /// Every distinct terminal of the rules; its index is its id
    terms: Vec<&'static ParserTokenKind>,
    /// Terminal → id, the index in `terms`
    term_id: HashMap<TerminalKey<'static>, usize>,
    /// Index of the `StartSymbol` rule, where the first state starts
    start_rule: usize,
}

impl ParsingTable {
    /// The cell Action(state, token); `None` is a syntax error. Only the kind of the token
    /// counts: `Name("main")` finds the column `Name`
    pub fn get(&self, state: usize, tok: &ParserToken) -> Option<ParsingState> {
        return self
            .table
            .get(&ParsingEntry::Action(state, TerminalKey(&tok.kind)))
            .copied();
    }

    /// The cell Goto(state, non_terminal): the state to push after a reduce to `non_terminal`
    pub fn get_goto(&self, state: usize, non_terminal: NTerm) -> Option<usize> {
        return match self.table.get(&ParsingEntry::Goto(state, non_terminal)) {
            Some(ParsingState::Goto(n)) => Some(*n),
            _ => None,
        };
    }

    pub fn new(rules: &'static [ParserProduction]) -> Self {
        // 1. What the algorithm needs from the grammar
        let (terms, term_id) = ParsingTable::get_terms(rules);
        let first = First::new(rules);
        let start_rule = ParsingTable::get_first_rule(rules);
        // `StartSymbol → Items Eof`: nothing comes after the whole program, so Eof by convention
        let eof = term_id[&TerminalKey(&ParserTokenKind::Eof)];

        // 2. State 0: the closure of `StartSymbol → · Items Eof   [Eof]`
        let state_0 = ParsingTable::closure(
            rules,
            &first,
            &term_id,
            BTreeSet::from([Item::new(start_rule, eof)]),
        );

        // 3. Every state, and its row of the table. `states[n]` is the state n; new states are
        // pushed at the end, so the loop stops once the last one adds nothing new
        let mut states = vec![state_0.clone()];
        let mut state_id = HashMap::from([(state_0, 0)]);
        let mut table = HashMap::new();
        let mut conflicts = 0;
        let mut i = 0;

        while i < states.len() {
            let state = states[i].clone();

            // Reduces: an item with its point at the end reduces its rule on its lookahead
            for item in state.iter().filter(|item| item.next_node(rules).is_none()) {
                let entry = ParsingEntry::Action(i, TerminalKey(terms[item.lookahead]));
                conflicts += ParsingTable::fill(&mut table, entry, ParsingState::R(item.rule));
            }

            // Shifts and gotos: one transition per symbol found just after a point
            for symbol in ParsingTable::symbols_after_point(rules, &state) {
                let next = ParsingTable::goto(rules, &first, &term_id, &state, symbol);
                let n = ParsingTable::state_number(&mut states, &mut state_id, next);
                let (entry, action) = ParsingTable::transition(i, symbol, n);
                conflicts += ParsingTable::fill(&mut table, entry, action);
            }
            i += 1;
        }
        if conflicts > 0 {
            eprintln!("{} états, {conflicts} conflits", states.len());
        }

        return Self {
            table,
            terms,
            term_id,
            start_rule,
        };
    }

    /// The symbols just after a point in `state`, each one once
    fn symbols_after_point(
        rules: &'static [ParserProduction],
        state: &BTreeSet<Item>,
    ) -> Vec<&'static ParserNode> {
        let mut symbols: Vec<&'static ParserNode> = Vec::new();
        for item in state.iter() {
            if let Some(node) = item.next_node(rules) {
                if !symbols.iter().any(|symbol| same_node(symbol, node)) {
                    symbols.push(node);
                }
            }
        }
        return symbols;
    }

    /// The number of `state`, which is added at the end of `states` if it was never seen
    fn state_number(
        states: &mut Vec<BTreeSet<Item>>,
        state_id: &mut HashMap<BTreeSet<Item>, usize>,
        state: BTreeSet<Item>,
    ) -> usize {
        if let Some(&n) = state_id.get(&state) {
            return n;
        }
        states.push(state.clone());
        state_id.insert(state, states.len() - 1);
        return states.len() - 1;
    }

    /// The cell and the action of the transition `state --symbol--> n`
    fn transition(
        state: usize,
        symbol: &'static ParserNode,
        n: usize,
    ) -> (ParsingEntry<'static>, ParsingState) {
        return match symbol {
            // Reading Eof after `StartSymbol → Items · Eof`: the whole program is read
            ParserNode::Term(kind) if same_terminal(kind, &ParserTokenKind::Eof) => (
                ParsingEntry::Action(state, TerminalKey(kind)),
                ParsingState::Accept,
            ),
            ParserNode::Term(kind) => (
                ParsingEntry::Action(state, TerminalKey(kind)),
                ParsingState::S(n),
            ),
            ParserNode::NTerm(non_terminal) => (
                ParsingEntry::Goto(state, *non_terminal),
                ParsingState::Goto(n),
            ),
        };
    }

    /// Writes `action` in the cell `entry`; a cell already holding another action is a conflict:
    /// it is printed, the first action is kept, and 1 is returned
    fn fill(
        table: &mut HashMap<ParsingEntry<'static>, ParsingState>,
        entry: ParsingEntry<'static>,
        action: ParsingState,
    ) -> usize {
        match table.get(&entry) {
            Some(old) if *old != action => {
                eprintln!("conflit dans {entry:?} : {old:?} ou {action:?}");
                return 1;
            }
            _ => {
                table.insert(entry, action);
                return 0;
            }
        }
    }

    /// Adds to `items`, until nothing new appears, the items `X → · …` of every non-terminal X
    /// found just after a point
    fn closure(
        rules: &'static [ParserProduction],
        first: &First,
        term_id: &HashMap<TerminalKey<'static>, usize>,
        mut items: BTreeSet<Item>,
    ) -> BTreeSet<Item> {
        let mut can_stop = false;

        while !can_stop {
            can_stop = true;
            let mut found = Vec::new();
            for elem in items.iter() {
                match elem.next_node(rules) {
                    // Point at the end, or before a terminal: nothing to unroll
                    None | Some(ParserNode::Term(_)) => {}
                    Some(ParserNode::NTerm(non_terminal)) => {
                        found.extend(ParsingTable::get_new_items(
                            rules,
                            first,
                            term_id,
                            elem,
                            *non_terminal,
                        ));
                    }
                }
            }
            // Stop once a whole pass adds nothing new
            for item in found {
                if items.insert(item) {
                    can_stop = false;
                }
            }
        }
        return items;
    }

    /// The state reached from `state` after reading `symbol`
    fn goto(
        rules: &'static [ParserProduction],
        first: &First,
        term_id: &HashMap<TerminalKey<'static>, usize>,
        state: &BTreeSet<Item>,
        symbol: &ParserNode,
    ) -> BTreeSet<Item> {
        let mut moved = BTreeSet::new();

        // 1. Keep the items of `state` with `symbol` just after their point
        let state_thaat_can_continue: Vec<&Item> = state
            .iter()
            .filter(|item| matches!(item.next_node(rules), Some(node) if same_node(node, symbol)))
            .collect();

        // 2. Move their point one step forward, into `moved`
        for elem in state_thaat_can_continue {
            moved.insert(elem.advance());
        }

        // 3. The closure of `moved`
        return ParsingTable::closure(rules, first, term_id, moved);
    }

    /// `elem` has its point just before `non_terminal`: the items `non_terminal → · …` to add, one
    /// per rule of `non_terminal` and per token that can follow it
    fn get_new_items(
        rules: &'static [ParserProduction],
        first: &First,
        term_id: &HashMap<TerminalKey<'static>, usize>,
        elem: &Item,
        non_terminal: NTerm,
    ) -> Vec<Item> {
        let mut new_items = Vec::new();

        // 1. What comes after `non_terminal` in the rule of `elem`
        let rest = &rules[elem.rule].right[elem.point + 1..];

        // 2. The lookaheads: FIRST of step 1 as ids, plus `elem.lookahead` if step 1 can vanish
        let (toks, nullable) = first.of_sequence(rest);
        let mut lookaheads: Vec<usize> = vec![];

        for f in toks {
            lookaheads.push(term_id[&TerminalKey(f)]);
        }
        if nullable {
            lookaheads.push(elem.lookahead);
        }

        // 3. One item per rule of `non_terminal` and per lookahead
        for (idx, rule) in rules.iter().enumerate() {
            if rule.left == non_terminal {
                for elem in lookaheads.iter() {
                    new_items.push(Item::new(idx, *elem));
                }
            }
        }

        return new_items;
    }

    /// The only rule `StartSymbol → … Eof`
    fn get_first_rule(rules: &'static [ParserProduction]) -> usize {
        let mut start_rules = rules
            .iter()
            .enumerate()
            .filter(|(_, rule)| rule.left == StartSymbol);
        let Some((idx, rule)) = start_rules.next() else {
            panic!("No rule with start symbol");
        };
        if start_rules.next().is_some() {
            panic!("More than one rule with start symbol");
        }
        match rule.right.last() {
            Some(ParserNode::Term(kind)) if same_terminal(kind, &ParserTokenKind::Eof) => idx,
            _ => panic!("The start symbol rule does not end with Eof"),
        }
    }

    fn get_terms(
        rules: &'static [ParserProduction],
    ) -> (
        Vec<&'static ParserTokenKind>,
        HashMap<TerminalKey<'static>, usize>,
    ) {
        let mut terms = Vec::new();
        let mut term_id = HashMap::new();
        for rule in rules {
            for node in rule.right {
                if let ParserNode::Term(kind) = node {
                    term_id.entry(TerminalKey(kind)).or_insert_with(|| {
                        terms.push(kind);
                        terms.len() - 1
                    });
                }
            }
        }
        return (terms, term_id);
    }
}
