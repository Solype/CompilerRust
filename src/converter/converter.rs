use std::collections::BTreeMap;
use std::fs::TryLockError::Error;
use std::os::linux::raw::stat;

use crate::lexer::span::Span;
use crate::parser::postparser::structs::{
    Argument, Assign, BinaryOp, Declare, Expr, ExprKind, For, Function, If, Init, Param, Program,
    Statement, StatementKind, Type, UnaryOp, While,
};
use crate::samples::helpers::ret;

use super::convert_error::{ConvertError, ConvertErrorKind};

struct Variable {
    ty: Type,
    /// `고정된`: its value cannot change
    constant: bool,
}

enum BlockKind<'p> {
    Loop(),
    Block,
    /// The body of a function: its name, its parameters and its return type
    Function(&'p Function),
}

/// The variables declared in one block, inside the scope of the block around it
struct Scope<'p> {
    variables: BTreeMap<String, Variable>,
    kind: BlockKind<'p>,
    /// The block around this one; `None` for the body of a function
    parent: Option<&'p Scope<'p>>,
}

impl<'p> Scope<'p> {
    fn new(kind: BlockKind<'p>, parent: Option<&'p Scope<'p>>) -> Self {
        Self {
            variables: BTreeMap::new(),
            kind,
            parent,
        }
    }
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
            ExprKind::Unary { operand, .. } => self.get_type(operand, scope),
            ExprKind::Binary { op, left, right } => {
                let left = self.get_type(left, scope)?;
                // Checks the names of the right side too
                self.get_type(right, scope)?;
                match op {
                    BinaryOp::Or
                    | BinaryOp::And
                    | BinaryOp::Eq
                    | BinaryOp::NotEq
                    | BinaryOp::Lt
                    | BinaryOp::Le
                    | BinaryOp::Gt
                    | BinaryOp::Ge => Ok(Type::Bool),
                    _ => Ok(left),
                }
            }
            ExprKind::Call { verb, args } => self.get_call_type(verb, args, scope, expr.span),
        }
    }

    /// The return type of the function `verb` that takes `args`, once the arguments are checked
    fn get_call_type(
        &self,
        verb: &str,
        args: &[Argument],
        scope: &Scope,
        span: Span,
    ) -> Result<Type, ConvertError> {
        let mut types = Vec::new();
        for arg in args {
            match arg {
                Argument::Value { value, .. } | Argument::Previous(value) => {
                    types.push(self.get_type(value, scope)?)
                }
                // `정수로`: the target type of 바꾸다
                Argument::ToType(ty) => types.push(ty.clone()),
            }
        }
        // `바꾸다`: built in, gives its value as the type of its `로` argument
        if verb == "바꾸다" {
            if let Some(Argument::ToType(ty)) =
                args.iter().find(|arg| matches!(arg, Argument::ToType(_)))
            {
                return Ok(ty.clone());
            }
        }
        let suffixes: Vec<String> = types.iter().map(|ty| format!("${ty}")).collect();
        let fullname = verb.to_string() + &suffixes.concat();
        // `외부` and `main`: their name alone, any arguments their parameters accept
        let by_name = self.functions.get(verb).filter(|index| {
            let func = &self.program.functions[**index];
            func.params.len() == types.len() || (func.variadic && types.len() > func.params.len())
        });
        // The others: `printf` without arguments must not find the `외부` printf
        let mangled = |index: &&usize| {
            let func = &self.program.functions[**index];
            func.body.is_some() && func.name != "main"
        };
        // Variadic: the types of the first arguments, then `$...`
        let variadic = || {
            (0..=types.len()).rev().find_map(|count| {
                self.functions
                    .get(&(verb.to_string() + &suffixes[..count].concat() + "$..."))
                    .filter(&mangled)
            })
        };
        let Some(index) = by_name
            .or_else(|| self.functions.get(&fullname).filter(&mangled))
            .or_else(variadic)
        else {
            let kind = ConvertErrorKind::UnknownFunction {
                verb: verb.to_string(),
                types: types
                    .iter()
                    .map(Type::to_string)
                    .collect::<Vec<_>>()
                    .join(", "),
            };
            return Err(ConvertError::new(kind, span));
        };
        let func = &self.program.functions[*index];
        return Ok(func.return_type.clone().unwrap_or(Type::Null));
    }

    fn find_scope_function<'s>(&self, scope: &'s Scope) -> Option<&'s Function> {
        match scope.kind {
            BlockKind::Block | BlockKind::Loop() => {
                scope.parent.and_then(|p| self.find_scope_function(p))
            }
            BlockKind::Function(f) => Some(f),
        }
    }

    /// The variable `name` of `scope`, or else of the blocks around it
    fn find_variable<'s>(&self, scope: &'s Scope, name: &str) -> Option<&'s Variable> {
        match scope.variables.get(name) {
            Some(variable) => Some(variable),
            None => scope
                .parent
                .and_then(|parent| self.find_variable(parent, name)),
        }
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

    fn convert_for_loop(&mut self, parent: &Scope, for_loop: &'a For) -> Result<(), ConvertError> {
        let mut scope = Scope::new(BlockKind::Loop(), Some(parent));
        let ty = self.get_type(&for_loop.from, &scope)?;
        scope.variables.insert(
            for_loop.variable.clone(),
            Variable {
                ty,
                constant: false,
            },
        );
        return self.convert_block(&for_loop.body, &scope);
    }

    fn convert_while_loop(
        &mut self,
        parent: &Scope,
        while_loop: &'a While,
    ) -> Result<(), ConvertError> {
        let scope = Scope::new(BlockKind::Loop(), Some(parent));
        let _cond = self.convert_condition(&while_loop.condition, parent)?;
        return self.convert_block(&while_loop.body, &scope);
    }

    /// `수는 정수예요.`: adds the variable to `scope`
    fn convert_declare(
        &mut self,
        scope: &mut Scope,
        declare: &'a Declare,
        span: Span,
    ) -> Result<(), ConvertError> {
        self.declare(
            scope,
            &declare.name,
            Variable {
                ty: declare.ty.clone(),
                constant: false,
            },
            span,
        )
    }

    /// `결과는 0이에요.`: adds the variable to `scope`
    fn convert_init(
        &mut self,
        scope: &mut Scope,
        init: &'a Init,
        span: Span,
    ) -> Result<(), ConvertError> {
        self.declare(
            scope,
            &init.name,
            Variable {
                ty: self.get_type(&init.value, scope)?,
                constant: init.constant,
            },
            span,
        )
    }

    /// `… 결과에 넣어요.`
    fn convert_assign(
        &mut self,
        scope: &Scope,
        assign: &'a Assign,
        span: Span,
    ) -> Result<(), ConvertError> {
        let Some(var) = self.find_variable(scope, &assign.target) else {
            let kind = ConvertErrorKind::UnknownVariable(assign.target.clone());
            return Err(ConvertError::new(kind, span));
        };
        if var.constant {
            let kind = ConvertErrorKind::AssignToConstant(assign.target.clone());
            return Err(ConvertError::new(kind, span));
        }
        Ok(())
    }

    /// `… 반환해요.`
    fn convert_return<'s>(
        &mut self,
        scope: &'s Scope,
        value: &'a Expr,
        span: Span,
    ) -> Result<(), ConvertError> {
        let ret_ty = self.get_type(value, scope)?;
        let Some(fnc) = self.find_scope_function(scope) else {
            return Err(ConvertError {
                kind: ConvertErrorKind::ReturnWithoutType("".to_string()),
                span,
            });
        };
        let decl_ret_ty = fnc.return_type.clone().unwrap_or(Type::Null);
        if decl_ret_ty != ret_ty {
            let kind = match &fnc.return_type {
                None => ConvertErrorKind::ReturnWithoutType(fnc.name.clone()),
                Some(_) => ConvertErrorKind::WrongReturnType {
                    expected: decl_ret_ty.to_string(),
                    found: ret_ty.to_string(),
                },
            };
            return Err(ConvertError { kind, span });
        }
        Ok(())
    }

    /// `… printf해요.`: a call whose result is unused
    fn convert_call(&mut self, scope: &Scope, call: &'a Expr) -> Result<(), ConvertError> {
        self.get_type(call, scope)?;
        Ok(())
    }

    /// `만약 … { … } 아니면 { … }`
    fn convert_if(&mut self, parent: &Scope, if_statement: &'a If) -> Result<(), ConvertError> {
        let _cond = self.convert_condition(&if_statement.condition, parent)?;
        return self.convert_block(&if_statement.then, parent);
    }

    /// `그만해요`
    fn convert_break(&self, scope: &Scope, span: Span) -> Result<(), ConvertError> {
        Ok(())
    }

    /// `넘어가요`
    fn convert_continue(&self, scope: &Scope, span: Span) -> Result<(), ConvertError> {
        Ok(())
    }

    /// Checks `block` in its own scope, which starts with the variables of `scope` (the
    /// parameters, the variable of a `세면서`)
    fn convert_block(&mut self, block: &'a [Statement], scope: &Scope) -> Result<(), ConvertError> {
        let mut new_scope = Scope::new(BlockKind::Block, Some(scope));
        for statement in block {
            match &statement.kind {
                StatementKind::Declare(declare) => {
                    self.convert_declare(&mut new_scope, declare, statement.span)?
                }
                StatementKind::Init(init) => {
                    self.convert_init(&mut new_scope, init, statement.span)?
                }
                StatementKind::Assign(assign) => {
                    self.convert_assign(&new_scope, assign, statement.span)?
                }
                StatementKind::Return(value) => {
                    self.convert_return(&new_scope, value, statement.span)?
                }
                StatementKind::Call(call) => self.convert_call(&new_scope, call)?,
                StatementKind::If(if_statement) => self.convert_if(&new_scope, if_statement)?,
                StatementKind::While(while_loop) => {
                    self.convert_while_loop(&new_scope, while_loop)?
                }
                StatementKind::For(for_loop) => self.convert_for_loop(&new_scope, for_loop)?,
                StatementKind::Break => self.convert_break(&new_scope, statement.span)?,
                StatementKind::Continue => self.convert_continue(&new_scope, statement.span)?,
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
        let mut scope = Scope::new(BlockKind::Function(func), None);
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
        self.convert_block(statements, &scope)
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
