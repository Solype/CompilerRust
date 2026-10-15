use std::collections::BTreeMap;

use crate::lexer::span::Span;
use crate::parser::postparser::structs::{
    Argument, Assign, BinaryOp, Declare, Expr, ExprKind, For, Function, If, Init, Program,
    Statement, StatementKind, Type, UnaryOp, While,
};

use super::convert_error::{type_name, ConvertError, ConvertErrorKind};

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

    /// Whether this block or a block around it is a loop
    fn in_loop(&self) -> bool {
        self.kind == BlockKind::Loop() || self.parent.is_some_and(|parent| parent.in_loop())
    }
}

pub struct Converter<'a> {
    program: &'a Program,
    functions: BTreeMap<String, usize>,
}

impl<'a> Converter<'a> {
    pub fn new(program: &'a Program) -> Self {
        Self {
            program,
            functions: BTreeMap::new(),
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
            ExprKind::Unary { op, operand } => {
                let ty = self.get_type(operand, scope)?;
                let accepted = match op {
                    UnaryOp::Neg => is_number(&ty),
                    UnaryOp::BitNot => matches!(ty, Type::Int { .. }),
                    UnaryOp::Not => ty == Type::Bool,
                };
                if !accepted {
                    return Err(operands_error(unary_name(op), &[&ty], expr.span));
                }
                Ok(ty)
            }
            ExprKind::Binary { op, left, right } => {
                let left_ty = self.get_type(left, scope)?;
                let right_ty = self.get_type(right, scope)?;
                // A literal takes the type of the other side: `작은 + 1` is a 짧은 정수
                let ty = if fits(right, &right_ty, &left_ty) {
                    Some(&left_ty)
                } else if fits(left, &left_ty, &right_ty) {
                    Some(&right_ty)
                } else {
                    None
                };
                let accepted = ty.filter(|ty| match op {
                    BinaryOp::Or | BinaryOp::And => **ty == Type::Bool,
                    BinaryOp::Eq | BinaryOp::NotEq => true,
                    BinaryOp::Lt
                    | BinaryOp::Le
                    | BinaryOp::Gt
                    | BinaryOp::Ge
                    | BinaryOp::Add
                    | BinaryOp::Sub
                    | BinaryOp::Mul
                    | BinaryOp::Div => is_number(ty),
                    BinaryOp::BitOr
                    | BinaryOp::BitXor
                    | BinaryOp::BitAnd
                    | BinaryOp::Shl
                    | BinaryOp::Shr
                    | BinaryOp::Rem => matches!(ty, Type::Int { .. }),
                });
                let Some(ty) = accepted else {
                    let types = [&left_ty, &right_ty];
                    return Err(operands_error(binary_name(op), &types, expr.span));
                };
                match op {
                    BinaryOp::Or
                    | BinaryOp::And
                    | BinaryOp::Eq
                    | BinaryOp::NotEq
                    | BinaryOp::Lt
                    | BinaryOp::Le
                    | BinaryOp::Gt
                    | BinaryOp::Ge => Ok(Type::Bool),
                    _ => Ok(ty.clone()),
                }
            }
            ExprKind::Call { verb, args } => self
                .get_call_type(verb, args, scope, expr.span)?
                .ok_or_else(|| {
                    let kind = ConvertErrorKind::NoValue(verb.clone());
                    ConvertError::new(kind, expr.span)
                }),
        }
    }

    /// The return type of the function `verb` that takes `args`, once the arguments are checked;
    /// `None` for a function without `TYPE을 반환하는`
    fn get_call_type(
        &self,
        verb: &str,
        args: &[Argument],
        scope: &Scope,
        span: Span,
    ) -> Result<Option<Type>, ConvertError> {
        let mut types = Vec::new();
        // The expression of each argument, `None` for a `정수로`
        let mut values = Vec::new();
        for arg in args {
            match arg {
                Argument::Value { value, .. } | Argument::Previous(value) => {
                    types.push(self.get_type(value, scope)?);
                    values.push(Some(value));
                }
                // `정수로`: the target type of 바꾸다
                Argument::ToType(ty) => {
                    types.push(ty.clone());
                    values.push(None);
                }
            }
        }
        // `바꾸다`: built in, gives its value as the type of its `로` argument
        if verb == "바꾸다" {
            if let Some(Argument::ToType(ty)) =
                args.iter().find(|arg| matches!(arg, Argument::ToType(_)))
            {
                return Ok(Some(ty.clone()));
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
        let types_names = || types.iter().map(type_name).collect::<Vec<_>>().join(", ");
        let exact = by_name
            .or_else(|| self.functions.get(&fullname).filter(&mangled))
            .or_else(variadic);
        let func = match exact {
            Some(index) => &self.program.functions[*index],
            // `작은과 1을 더해요`: the functions whose parameters take the literals
            None => {
                let candidates: Vec<&Function> = self
                    .program
                    .functions
                    .iter()
                    .filter(|func| func.name == verb && func.body.is_some() && func.name != "main")
                    .filter(|func| {
                        func.params.len() == types.len()
                            || (func.variadic && types.len() > func.params.len())
                    })
                    .filter(|func| {
                        func.params.iter().zip(values.iter().zip(&types)).all(
                            |(param, (value, ty))| match value {
                                Some(value) => fits(value, ty, &param.ty),
                                None => *ty == param.ty,
                            },
                        )
                    })
                    .collect();
                match candidates[..] {
                    [func] => func,
                    [] => {
                        let kind = ConvertErrorKind::UnknownFunction {
                            verb: verb.to_string(),
                            types: types_names(),
                        };
                        return Err(ConvertError::new(kind, span));
                    }
                    _ => {
                        let kind = ConvertErrorKind::AmbiguousCall {
                            verb: verb.to_string(),
                            types: types_names(),
                        };
                        return Err(ConvertError::new(kind, span));
                    }
                }
            }
        };
        return Ok(func.return_type.clone());
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

    /// `수를 1부터 끝까지 세면서 { … }`: the variable and the bounds are the same 정수 type
    fn convert_for_loop(
        &mut self,
        parent: &Scope,
        for_loop: &'a For,
        span: Span,
    ) -> Result<(), ConvertError> {
        let mut scope = Scope::new(BlockKind::Loop(), Some(parent));
        let ty = match &for_loop.ty {
            // `정수 수를 …`: declared by the loop, in its own scope
            Some(ty) => {
                let variable = Variable {
                    ty: ty.clone(),
                    constant: false,
                };
                scope.variables.insert(for_loop.variable.clone(), variable);
                ty.clone()
            }
            // `수를 …`: a variable of the blocks around, that the loop changes
            None => {
                let Some(var) = self.find_variable(parent, &for_loop.variable) else {
                    let kind = ConvertErrorKind::UnknownVariable(for_loop.variable.clone());
                    return Err(ConvertError::new(kind, span));
                };
                if var.constant {
                    let kind = ConvertErrorKind::AssignToConstant(for_loop.variable.clone());
                    return Err(ConvertError::new(kind, span));
                }
                var.ty.clone()
            }
        };
        if !matches!(ty, Type::Int { .. }) {
            return Err(ConvertError::new(ConvertErrorKind::NotInteger, span));
        }
        for bound in [&for_loop.from, &for_loop.until] {
            if !fits(bound, &self.get_type(bound, parent)?, &ty) {
                return Err(ConvertError::new(ConvertErrorKind::NotInteger, bound.span));
            }
        }
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
        let assign_value_ty = self.get_type(&assign.value, scope)?;
        if !fits(&assign.value, &assign_value_ty, &var.ty) {
            let kind = ConvertErrorKind::WrongAssignType {
                name: assign.target.clone(),
                expected: type_name(&var.ty),
                found: type_name(&assign_value_ty),
            };
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
        let Some(decl_ret_ty) = &fnc.return_type else {
            let kind = ConvertErrorKind::ReturnWithoutType(fnc.name.clone());
            return Err(ConvertError::new(kind, span));
        };
        if !fits(value, &ret_ty, decl_ret_ty) {
            let kind = ConvertErrorKind::WrongReturnType {
                expected: type_name(decl_ret_ty),
                found: type_name(&ret_ty),
            };
            return Err(ConvertError::new(kind, span));
        }
        Ok(())
    }

    /// `… printf해요.`: a call whose result is unused
    fn convert_call(&mut self, scope: &Scope, call: &'a Expr) -> Result<(), ConvertError> {
        match &call.kind {
            // Alone, a call may return nothing
            ExprKind::Call { verb, args } => {
                self.get_call_type(verb, args, scope, call.span)?;
            }
            _ => {
                self.get_type(call, scope)?;
            }
        }
        Ok(())
    }

    /// `만약 … { … } 아니면 { … }`
    fn convert_if(&mut self, parent: &Scope, if_statement: &'a If) -> Result<(), ConvertError> {
        let _cond = self.convert_condition(&if_statement.condition, parent)?;
        self.convert_block(&if_statement.then, parent)?;
        if let Some(otherwise_block) = &if_statement.otherwise {
            self.convert_block(&otherwise_block, parent)?;
        }
        return Ok(());
    }

    /// `그만해요`
    fn convert_break(&self, scope: &Scope, span: Span) -> Result<(), ConvertError> {
        if scope.in_loop() {
            return Ok(());
        }
        return Err(ConvertError::new(ConvertErrorKind::BreakOutOfLoop, span));
    }

    /// `넘어가요`
    fn convert_continue(&self, scope: &Scope, span: Span) -> Result<(), ConvertError> {
        if scope.in_loop() {
            return Ok(());
        }
        return Err(ConvertError::new(ConvertErrorKind::ContinueOutOfLoop, span));
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
                StatementKind::For(for_loop) => {
                    self.convert_for_loop(&new_scope, for_loop, statement.span)?
                }
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
        self.convert_block(statements, &scope)?;
        if func.return_type.is_some() && !always_returns(statements) {
            let kind = ConvertErrorKind::MissingReturn(func.name.clone());
            return Err(ConvertError::new(kind, func.span));
        }
        Ok(())
    }

    /// The program has a `main` that returns a 정수
    fn check_main(&self) -> Result<(), ConvertError> {
        let Some(main) = self.program.functions.iter().find(|func| func.name == "main") else {
            return Err(ConvertError::new(ConvertErrorKind::NoMain, Span::new(0, 0)));
        };
        if !matches!(main.return_type, Some(Type::Int { .. })) {
            return Err(ConvertError::new(ConvertErrorKind::MainNotInteger, main.span));
        }
        Ok(())
    }

    pub fn convert(&mut self) -> Result<(), ConvertError> {
        // Get the global variables
        // Get the structures
        self.get_all_functions()?;
        self.check_main()?;
        let program = self.program;
        for func in &program.functions {
            self.convert_func(func)?;
        }
        // for func in self.program.functions.iter() {}
        Ok(())
    }
}

/// Whether every path through `block` ends with `반환해요`: a loop may run zero times, so only a
/// `만약` whose two branches return counts
fn always_returns(block: &[Statement]) -> bool {
    return block.iter().any(|statement| match &statement.kind {
        StatementKind::Return(_) => true,
        StatementKind::If(If {
            then,
            otherwise: Some(otherwise),
            ..
        }) => always_returns(then) && always_returns(otherwise),
        _ => false,
    });
}

/// Whether `expr`, of type `ty`, goes where `expected` goes: the same type, or a literal that
/// takes the expected type (`0` for a 짧은 정수, `1.5` for a 짧은 소수, `빈 주소` for any 주소)
fn fits(expr: &Expr, ty: &Type, expected: &Type) -> bool {
    if ty == expected {
        return true;
    }
    return match (ty, expected) {
        (Type::Int { .. }, Type::Int { .. }) | (Type::Float { .. }, Type::Float { .. }) => {
            is_literal(expr)
        }
        (Type::Null, Type::Address(_)) => matches!(expr.kind, ExprKind::Null),
        _ => false,
    };
}

/// A number written in the source, maybe with `-` or computed from other literals: `(1 + 2)`
fn is_literal(expr: &Expr) -> bool {
    return match &expr.kind {
        ExprKind::Int(_) | ExprKind::Float(_) => true,
        ExprKind::Unary { operand, .. } => is_literal(operand),
        ExprKind::Binary { left, right, .. } => is_literal(left) && is_literal(right),
        _ => false,
    };
}

fn is_number(ty: &Type) -> bool {
    return matches!(ty, Type::Int { .. } | Type::Float { .. });
}

/// `op` cannot take operands of `types`
fn operands_error(op: String, types: &[&Type], span: Span) -> ConvertError {
    let types = types
        .iter()
        .map(|ty| type_name(ty))
        .collect::<Vec<_>>()
        .join(", ");
    let kind = ConvertErrorKind::WrongOperands { op, types };
    return ConvertError::new(kind, span);
}

/// `op` as the source writes it: `아닌` for the negation of a condition
fn unary_name(op: &UnaryOp) -> String {
    return match op {
        UnaryOp::Not => "아닌".to_string(),
        _ => op.to_string(),
    };
}

/// `op` as the source writes it: `또는` and `그리고` for the conditions
fn binary_name(op: &BinaryOp) -> String {
    return match op {
        BinaryOp::Or => "또는".to_string(),
        BinaryOp::And => "그리고".to_string(),
        _ => op.to_string(),
    };
}
