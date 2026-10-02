use std::collections::HashSet;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct FiniteAutomataState(pub usize);

#[derive(Clone)]
pub enum FiniteAutomataTransition {
    Function(fn(char) -> bool),
    Char(char),
}

pub struct FiniteAutomata {
    nb_state: usize,
    final_states: HashSet<FiniteAutomataState>,
    start_state: FiniteAutomataState,
    table: Vec<Vec<(FiniteAutomataTransition, FiniteAutomataState)>>,
}


impl FiniteAutomata {
    pub fn new(nb_state: usize, start_state: FiniteAutomataState) -> Self {
        let table:Vec<Vec<(FiniteAutomataTransition, FiniteAutomataState)>> = vec![ vec![] ; nb_state as usize ];

        Self {
            nb_state,
            final_states: HashSet::new(),
            start_state,
            table,
        }
    }

    pub fn add_transition(
        &mut self,
        start: FiniteAutomataState,
        transition: FiniteAutomataTransition,
        dest: FiniteAutomataState,
    ) {
        if start.0 >= self.nb_state || dest.0 >= self.nb_state {
            panic!("Invalid state id !");
        }
        self.table[start.0].push((transition, dest));
    }

    pub fn add_final_state(&mut self, state: FiniteAutomataState) {
        self.final_states.insert(state);
    }

    fn parse_char(
        &self,
        state : FiniteAutomataState,
        c: char
    ) -> Option<FiniteAutomataState> {
        for cell in &self.table[state.0] {
            let transition : &FiniteAutomataTransition = &cell.0;
            let state : FiniteAutomataState = cell.1;
            match transition {
                FiniteAutomataTransition::Char(test_c) => {
                    if c == *test_c {
                        return Some(state);
                    }
                }
                FiniteAutomataTransition::Function(fct) => {
                    if fct(c) {
                        return Some(state);
                    }
                }
            }
        }
        None
    }

    pub fn run_automata(
        &self,
        stream: &str,
        start: usize,
    ) -> Option<usize> {
        let mut current_state = self.start_state;

        for (i, c) in stream.chars().enumerate().skip(start) {
            match self.parse_char(current_state, c) {
                Some(state) => current_state = state,
                None => break,
            }

            if self.final_states.contains(&current_state) {
                return Some(i + 1); // +1 = position after the match
            }
        }
        None
    }
}
