use std::cell::Cell;
use std::collections::BTreeMap;

use crate::lexer::span::Span;
use crate::parser::postparser::structs::{Function, Type};

use super::convert_error::{ConvertError, ConvertErrorKind};
use super::ir::structs::VarId;

pub(super) struct Variable {
    pub(super) ty: Type,
    /// `고정된`: its value cannot change
    pub(super) constant: bool,
    pub(super) var_id: VarId,
}

#[derive(Debug, Default)]
pub(super) struct LoopLabels {
    pub(super) label_start: String,
    pub(super) label_end: String,
}

pub(super) enum BlockKind<'p> {
    Loop(LoopLabels),
    Block,
    /// The body of a function: its name, its parameters and its return type
    Function(&'p Function),
}

/// Two `Function` kinds are equal when they are the body of the same function of the program
impl PartialEq for BlockKind<'_> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Loop(_), Self::Loop(_)) | (Self::Block, Self::Block) => true,
            (Self::Function(a), Self::Function(b)) => std::ptr::eq(*a, *b),
            _ => false,
        }
    }
}

/// The variables declared in one block, inside the scope of the block around it
pub(super) struct Scope<'p> {
    pub(super) variables: BTreeMap<String, Variable>,
    /// The blocks opened in this one so far; a `Cell` as a block opens with `&` on its parent
    block_count: Cell<usize>,
    /// Its number among the blocks of its parent, from 1; 0 for the body of a function
    index: usize,
    kind: BlockKind<'p>,
    /// The block around this one; `None` for the body of a function
    parent: Option<&'p Scope<'p>>,
}

impl<'p> Scope<'p> {
    pub(super) fn new(kind: BlockKind<'p>, parent: Option<&'p Scope<'p>>) -> Self {
        // The only place that counts the blocks: a new block takes the next number of its parent
        let index = match parent {
            Some(parent) => {
                parent.block_count.set(parent.block_count.get() + 1);
                parent.block_count.get()
            }
            None => 0,
        };
        Self {
            variables: BTreeMap::new(),
            block_count: Cell::new(0),
            index,
            kind,
            parent,
        }
    }

    /// The labels of the innermost loop around this block; `None` outside of a loop
    pub(super) fn in_loop(&self) -> Option<&LoopLabels> {
        return match &self.kind {
            BlockKind::Loop(labels) => Some(labels),
            _ => self.parent.and_then(|parent| parent.in_loop()),
        };
    }

    /// The function whose body holds this block
    pub(super) fn function(&self) -> Option<&'p Function> {
        match self.kind {
            BlockKind::Block | BlockKind::Loop(_) => self.parent.and_then(|p| p.function()),
            BlockKind::Function(f) => Some(f),
        }
    }

    /// The variable `name` of this block, or else of the blocks around it
    pub(super) fn find_variable(&self, name: &str) -> Option<&Variable> {
        match self.variables.get(name) {
            Some(variable) => Some(variable),
            None => self.parent.and_then(|parent| parent.find_variable(name)),
        }
    }

    /// The labels `{prefix}_{path}_{first}` and `{prefix}_{path}_end` of the next block opened in
    /// this one: the numbers of the blocks from the function down to it, unique in the function.
    /// Called just before that block opens: a loop, or the `만약` branch of an if
    pub(super) fn generate_labels(&self, prefix: &str, first: &str) -> (String, String) {
        let mut path = vec![(self.block_count.get() + 1).to_string()];
        let mut curscope = self;
        while let Some(parent) = curscope.parent {
            path.push(curscope.index.to_string());
            curscope = parent;
        }
        path.reverse();
        let base_name = format!("{prefix}_{}", path.join("_"));
        return (format!("{base_name}_{first}"), format!("{base_name}_end"));
    }

    /// Adds `name` to this block, if no variable of this block or of the blocks around has it: a
    /// name of a block already closed is free again
    pub(super) fn declare(
        &mut self,
        name: &str,
        variable: Variable,
        span: Span,
    ) -> Result<(), ConvertError> {
        if self.find_variable(name).is_some() {
            let kind = ConvertErrorKind::AlreadyDeclared(name.to_string());
            return Err(ConvertError::new(kind, span));
        }
        self.variables.insert(name.to_string(), variable);
        Ok(())
    }
}
