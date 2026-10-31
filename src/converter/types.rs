use crate::parser::postparser::structs::{BinaryOp, Expr, ExprKind, Type, UnaryOp};

use super::convert_error::{ConvertError, ConvertErrorKind};

/// Whether `expr`, of type `ty`, goes where `expected` goes: the same type, or a literal that
/// takes the expected type when its value fits in it (`0` for a 짧은 정수, `1.5` for a 짧은 소수,
/// `빈 주소` for any 주소); an address to nothing (`없음은 빈 주소예요.`) takes any address and
/// goes into any, as void* in C
pub(super) fn fits(expr: &Expr, ty: &Type, expected: &Type) -> bool {
    if ty == expected {
        return true;
    }
    return match (ty, expected) {
        (Type::Int { .. }, Type::Int { short, unsigned }) => {
            int_value(expr).is_some_and(|value| int_fits(value, *short, *unsigned))
        }
        (Type::Float { .. }, Type::Float { short }) => {
            float_value(expr).is_some_and(|value| float_fits(value, *short))
        }
        (Type::Null, Type::Address(_)) => matches!(expr.kind, ExprKind::Null),
        (Type::Address(from), Type::Address(to)) => **from == Type::Null || **to == Type::Null,
        _ => false,
    };
}

/// The error of a literal that does not fit in `expected` because of its value, not its type:
/// `-1` for a 부호 없는 정수, `5000000000` for a 짧은 정수
pub(super) fn range_error(expr: &Expr, ty: &Type, expected: &Type) -> Option<ConvertError> {
    let literal = match (ty, expected) {
        (Type::Int { .. }, Type::Int { .. }) => int_value(expr).is_some(),
        (Type::Float { .. }, Type::Float { .. }) => float_value(expr).is_some(),
        _ => false,
    };
    if !literal || fits(expr, ty, expected) {
        return None;
    }
    let kind = ConvertErrorKind::LiteralOutOfRange(type_name(expected));
    return Some(ConvertError::new(kind, expr.span));
}

/// The value of an integer literal, maybe with `-` or computed from other literals: `-(1 + 2)`
/// is -3; `None` for an expression that is no literal, or whose computation overflows
fn int_value(expr: &Expr) -> Option<i128> {
    return match &expr.kind {
        ExprKind::Int(value, _) => Some(*value as i128),
        ExprKind::Unary { op, operand } => {
            let value = int_value(operand)?;
            match op {
                UnaryOp::Neg => value.checked_neg(),
                UnaryOp::BitNot => Some(!value),
                UnaryOp::Not => None,
            }
        }
        ExprKind::Binary { op, left, right } => {
            let (left, right) = (int_value(left)?, int_value(right)?);
            match op {
                BinaryOp::Add => left.checked_add(right),
                BinaryOp::Sub => left.checked_sub(right),
                BinaryOp::Mul => left.checked_mul(right),
                BinaryOp::Div => left.checked_div(right),
                BinaryOp::Rem => left.checked_rem(right),
                BinaryOp::BitAnd => Some(left & right),
                BinaryOp::BitOr => Some(left | right),
                BinaryOp::BitXor => Some(left ^ right),
                BinaryOp::Shl => u32::try_from(right).ok().and_then(|r| left.checked_shl(r)),
                BinaryOp::Shr => u32::try_from(right).ok().and_then(|r| left.checked_shr(r)),
                _ => None,
            }
        }
        _ => None,
    };
}

/// The value of a 소수 literal, maybe with `-` or computed from other literals
fn float_value(expr: &Expr) -> Option<f64> {
    return match &expr.kind {
        ExprKind::Float(value, _) => Some(*value),
        ExprKind::Unary {
            op: UnaryOp::Neg,
            operand,
        } => Some(-float_value(operand)?),
        ExprKind::Binary { op, left, right } => {
            let (left, right) = (float_value(left)?, float_value(right)?);
            match op {
                BinaryOp::Add => Some(left + right),
                BinaryOp::Sub => Some(left - right),
                BinaryOp::Mul => Some(left * right),
                BinaryOp::Div => Some(left / right),
                _ => None,
            }
        }
        _ => None,
    };
}

/// Whether `value` fits in a 정수 of 32 bits when `short`, of 64 bits otherwise, never negative
/// when `unsigned`
fn int_fits(value: i128, short: bool, unsigned: bool) -> bool {
    let (min, max) = match (short, unsigned) {
        (true, false) => (i32::MIN as i128, i32::MAX as i128),
        (true, true) => (0, u32::MAX as i128),
        (false, false) => (i64::MIN as i128, i64::MAX as i128),
        (false, true) => (0, u64::MAX as i128),
    };
    return (min..=max).contains(&value);
}

/// Whether `value` fits in a 소수 of 32 bits when `short`, of 64 bits otherwise
fn float_fits(value: f64, short: bool) -> bool {
    return value.is_finite() && (!short || value.abs() <= f32::MAX as f64);
}

/// Whether `바꾸다` converts a `from` to a `to`: between the numbers, 논리, 문자 and 바이트, between
/// the 주소, and between a 주소 and a 정수 of 64 bits, like the casts of C
pub(super) fn can_cast(from: &Type, to: &Type) -> bool {
    let scalar = |ty: &Type| {
        matches!(
            ty,
            Type::Int { .. } | Type::Float { .. } | Type::Bool | Type::Char | Type::Byte
        )
    };
    let address = |ty: &Type| matches!(ty, Type::Address(_) | Type::Null);
    let long = |ty: &Type| matches!(ty, Type::Int { short: false, .. });
    return from == to
        || (scalar(from) && scalar(to))
        || (address(from) && (matches!(to, Type::Address(_)) || long(to)))
        || (long(from) && matches!(to, Type::Address(_)));
}

pub(super) fn is_number(ty: &Type) -> bool {
    return matches!(ty, Type::Int { .. } | Type::Float { .. });
}

/// `ty` as the source writes it, for the messages
pub(super) fn type_name(ty: &Type) -> String {
    return match ty {
        Type::Int { short, unsigned } => {
            let short = if *short { "짧은 " } else { "" };
            let unsigned = if *unsigned { "부호 없는 " } else { "" };
            format!("{short}{unsigned}정수")
        }
        Type::Float { short: true } => "짧은 소수".to_string(),
        Type::Float { short: false } => "소수".to_string(),
        Type::Bool => "논리".to_string(),
        Type::Char => "문자".to_string(),
        Type::Byte => "바이트".to_string(),
        Type::Address(ty) => format!("{} 주소", type_name(ty)),
        Type::Array(ty, count) => format!("{} {count}개", type_name(ty)),
        Type::Custom(name) => name.clone(),
        Type::Null => "nothing".to_string(),
    };
}
