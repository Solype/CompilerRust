use std::collections::BTreeMap;

use crate::lexer::span::Span;
use crate::parser::postparser::structs::{Function, Type};

use super::convert_error::{ConvertError, ConvertErrorKind};

pub(super) struct Variable {
    pub(super) ty: Type,
    /// `고정된`: its value cannot change
    pub(super) constant: bool,
}

pub(super) enum BlockKind<'p> {
    Loop(),
    Block,
    /// The body of a function: its name, its parameters and its return type
    Function(&'p Function),
}

/// Two `Function` kinds are equal when they are the body of the same function of the program
impl PartialEq for BlockKind<'_> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Loop(), Self::Loop()) | (Self::Block, Self::Block) => true,
            (Self::Function(a), Self::Function(b)) => std::ptr::eq(*a, *b),
            _ => false,
        }
    }
}

/// The variables declared in one block, inside the scope of the block around it
pub(super) struct Scope<'p> {
    pub(super) variables: BTreeMap<String, Variable>,
    kind: BlockKind<'p>,
    /// The block around this one; `None` for the body of a function
    parent: Option<&'p Scope<'p>>,
}

impl<'p> Scope<'p> {
    pub(super) fn new(kind: BlockKind<'p>, parent: Option<&'p Scope<'p>>) -> Self {
        Self {
            variables: BTreeMap::new(),
            kind,
            parent,
        }
    }

    /// Whether this block or a block around it is a loop
    pub(super) fn in_loop(&self) -> bool {
        self.kind == BlockKind::Loop() || self.parent.is_some_and(|parent| parent.in_loop())
    }

    /// The function whose body holds this block
    pub(super) fn function(&self) -> Option<&'p Function> {
        match self.kind {
            BlockKind::Block | BlockKind::Loop() => self.parent.and_then(|p| p.function()),
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
