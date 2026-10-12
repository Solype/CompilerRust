use std::collections::BTreeMap;
use std::os::linux::raw::stat;

use crate::lexer::span::Span;
use crate::parser::postparser::structs::{
    Argument, BinaryOp, Expr, ExprKind, Function, Param, Program, Statement, StatementKind, Type,
    UnaryOp,
};

use super::convert_error::{ConvertError, ConvertErrorKind};

struct Variable {
    ty: Type,
    /// `고정된`: its value cannot change
    constant: bool,
}

enum BlockKind {
    Loop(),
    Block,
}

/// The variables declared in one block
#[derive(Default)]
struct Scope {
    variables: BTreeMap<String, Variable>,
    prev_blocks: Vec<BlockKind>,
}

pub struct Converter<'a> {
    program: &'a Program,
    functions: BTreeMap<String, usize>,
    /// The number of loops around the statement being checked
    loops: usize,
}

impl<'a> Converter<'a> {
    pub fn new(program: &'a Program) -> Self {
        Self {
            program,
            functions: BTreeMap::new(),
            loops: 0,
        }
    }

    pub fn get_all_functions(&mut self) -> Result<(), ConvertError> {
        for (i, func) in self.program.functions.iter().enumerate() {
            // `외부` and `main`: the libc symbols, their name as is for the linker
            if func.body.is_none() || func.name == "main" {
                if self.functions.contains_key(&func.name) {
                    let kind = ConvertErrorKind::SameSymbol(func.name.clone());
                    return Err(ConvertError::new(kind, func.span));
                }
                self.functions.insert(func.name.clone(), i);
                continue;
            }
            let mut fullname = func.name.clone()
                + &func
                    .params
                    .iter()
                    .map(|p| format!("${}", p.ty))
                    .collect::<String>();
            if func.variadic {
                fullname += "$...";
            }
            if self.functions.contains_key(&fullname) {
                let kind = ConvertErrorKind::SameSignature(func.name.clone());
                return Err(ConvertError::new(kind, func.span));
            }
            self.functions.insert(fullname, i);
        }
        Ok(())
    }

    /// The type of `expr`, once its names and its calls are checked
    fn get_type(&self, expr: &Expr, scope: &Scope) -> Result<Type, ConvertError> {
        match &expr.kind {
            ExprKind::Int(_) => Ok(Type::Int {
                short: false,
                unsigned: false,
            }),
            ExprKind::Float(_) => Ok(Type::Float { short: false }),
            // `"…"`: a 문자 주소, like the format of printf
            ExprKind::Str(_) => Ok(Type::Address(Box::new(Type::Char))),
            ExprKind::Bool(_) => Ok(Type::Bool),
            ExprKind::Name(name) => match self.find_variable(scope, name) {
                Some(variable) => Ok(variable.ty.clone()),
                None => {
                    let kind = ConvertErrorKind::UnknownVariable(name.clone());
                    Err(ConvertError::new(kind, expr.span))
                }
            },
            ExprKind::Null => Ok(Type::Null),
            // `정수의 크기`: a size, never negative
            ExprKind::SizeOf(_) => Ok(Type::Int {
                short: false,
                unsigned: true,
            }),
            _ => unimplemented!(),
        }
    }

    /// The variable `name` of `scope`
    fn find_variable<'s>(&self, scope: &'s Scope, name: &str) -> Option<&'s Variable> {
        scope.variables.get(name)
    }

    /// Adds `name` to `scope`, if no variable of `scope` has it
    fn declare(
        &mut self,
        scope: &mut Scope,
        name: &str,
        variable: Variable,
        span: Span,
    ) -> Result<(), ConvertError> {
        if scope.variables.contains_key(name) {
            let kind = ConvertErrorKind::AlreadyDeclared(name.to_string());
            return Err(ConvertError::new(kind, span));
        }
        scope.variables.insert(name.to_string(), variable);
        Ok(())
    }

    /// `condition` must be a 논리
    fn convert_condition(&self, condition: &Expr, scope: &Scope) -> Result<(), ConvertError> {
        match self.get_type(condition, scope)? {
            Type::Bool => Ok(()),
            _ => Err(ConvertError::new(ConvertErrorKind::NotBool, condition.span)),
        }
    }

    // fn convert_for_loop(&self, )

    /// Checks `block` in its own scope, which starts with the variables of `scope` (the
    /// parameters, the variable of a `세면서`)
    fn convert_block(
        &mut self,
        block: &'a [Statement],
        mut scope: Scope,
    ) -> Result<(), ConvertError> {
        for statement in block {
            match &statement.kind {
                StatementKind::Declare(declare) => unimplemented!(),
                StatementKind::Init(init) => unimplemented!(),
                StatementKind::Assign(assign) => unimplemented!(),
                StatementKind::Return(_) => unimplemented!(),
                StatementKind::Call(_) => unimplemented!(),
                StatementKind::If(if_statement) => unimplemented!(),
                StatementKind::While(while_loop) => unimplemented!(),
                StatementKind::For(for_loop) => unimplemented!(),
                StatementKind::Break => unimplemented!(),
                StatementKind::Continue => unimplemented!(),
                _ => unimplemented!(),
            }
        }
        Ok(())
    }

    /// Checks the body of `func`, its parameters in the same scope as its variables
    fn convert_func(&mut self, func: &'a Function) -> Result<(), ConvertError> {
        // `외부`: nothing to check
        let Some(statements) = &func.body else {
            return Ok(());
        };
        let mut scope = Scope::default();
        for param in &func.params {
            if scope.variables.contains_key(&param.name) {
                let kind = ConvertErrorKind::AlreadyDeclared(param.name.clone());
                return Err(ConvertError::new(kind, param.span));
            }
            let variable = Variable {
                ty: param.ty.clone(),
                constant: false,
            };
            scope.variables.insert(param.name.clone(), variable);
        }
        self.convert_block(statements, scope)
    }

    pub fn convert(&mut self) -> Result<(), ConvertError> {
        // Get the global variables
        // Get the structures
        self.get_all_functions()?;
        let program = self.program;
        for func in &program.functions {
            self.convert_func(func)?;
        }
        // for func in self.program.functions.iter() {}
        Ok(())
    }
}
