use crate::parser::postparser::structs::{Expr, ExprKind, Type};

/// Whether `expr`, of type `ty`, goes where `expected` goes: the same type, or a literal that
/// takes the expected type (`0` for a 짧은 정수, `1.5` for a 짧은 소수, `빈 주소` for any 주소)
pub(super) fn fits(expr: &Expr, ty: &Type, expected: &Type) -> bool {
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
