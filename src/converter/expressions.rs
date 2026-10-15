use crate::lexer::span::Span;
use crate::parser::postparser::structs::{
    Argument, BinaryOp, Expr, ExprKind, Function, Type, UnaryOp,
};

use super::convert_error::{ConvertError, ConvertErrorKind};
use super::converter::Converter;
use super::scope::Scope;
use super::types::{fits, is_number, type_name};

impl<'a> Converter<'a> {
    /// The type of `expr`, once its names and its calls are checked
    pub(super) fn get_type(&self, expr: &Expr, scope: &Scope) -> Result<Type, ConvertError> {
        match &expr.kind {
            ExprKind::Int(_) => Ok(Type::Int {
                short: false,
                unsigned: false,
            }),
            ExprKind::Float(_) => Ok(Type::Float { short: false }),
            // `"…"`: a 문자 주소, like the format of printf
            ExprKind::Str(_) => Ok(Type::Address(Box::new(Type::Char))),
            ExprKind::Bool(_) => Ok(Type::Bool),
            ExprKind::Name(name) => match scope.find_variable(name) {
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
    pub(super) fn get_call_type(
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
