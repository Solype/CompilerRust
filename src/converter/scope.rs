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
    pub(super) block_count: usize,
    kind: BlockKind<'p>,
    /// The block around this one; `None` for the body of a function
    parent: Option<&'p Scope<'p>>,
}

impl<'p> Scope<'p> {
    pub(super) fn new(kind: BlockKind<'p>, parent: Option<&'p Scope<'p>>) -> Self {
        Self {
            variables: BTreeMap::new(),
            block_count: 0,
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

    /// The start and end labels of a loop in this block: the `block_count` of the blocks from the
    /// function down to this one, unique in the function as each block opened in a block takes
    /// the next count of its parent
    pub(super) fn generate_loop_label(&self) -> (String, String) {
        let mut counts = vec![self.block_count.to_string()];
        let mut curscope = self;
        while let Some(parent) = curscope.parent {
            match curscope.kind {
                BlockKind::Loop(_) => counts.push("loop".to_string()),
                BlockKind::Function(f) => counts.push(f.name.clone()),
                BlockKind::Block => counts.push("B".to_string()),
            }
            counts.push(parent.block_count.to_string());
            curscope = parent;
        }
        counts.reverse();
        let base_name = format!("loop_{}", counts.join("_"));
        return (format!("{base_name}_start"), format!("{base_name}_end"));
    }

    pub(super) fn generate_if_labels(&self) -> (String, String) {
        let mut counts = vec![self.block_count.to_string()];
        let mut curscope = self;
        while let Some(parent) = curscope.parent {
            match curscope.kind {
                BlockKind::Loop(_) => counts.push("loop".to_string()),
                BlockKind::Function(f) => counts.push(f.name.clone()),
                BlockKind::Block => counts.push("B".to_string()),
            }
            counts.push(parent.block_count.to_string());
            curscope = parent;
        }
        counts.reverse();
        let base_name = format!("condition_{}", counts.join("_"));
        return (format!("{base_name}_otherwise"), format!("{base_name}_end"));
    }

    /// Adds `name` to this block, if no variable of this block has it
    pub(super) fn declare(
        &mut self,
        name: &str,
        variable: Variable,
        span: Span,
    ) -> Result<(), ConvertError> {
        if self.variables.contains_key(name) {
            let kind = ConvertErrorKind::AlreadyDeclared(name.to_string());
            return Err(ConvertError::new(kind, span));
        }
        self.variables.insert(name.to_string(), variable);
        Ok(())
    }
}
