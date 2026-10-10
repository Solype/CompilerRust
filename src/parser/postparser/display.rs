use std::fmt;

use super::structs::{
    Argument, BinaryOp, Expr, ExprKind, Function, Param, Program, Statement, StatementKind, Type,
    UnaryOp,
};

/// The AST as pseudo-code: one function after the other, blocks indented by 4 spaces
impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, function) in self.functions.iter().enumerate() {
            if i > 0 {
                writeln!(f)?;
            }
            write!(f, "{function}")?;
        }
        return Ok(());
    }
}

/// `fn 더하다(int 가, int 나) -> int { … }`, `extern fn printf(char& 형식, ...) -> int`
impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.body.is_none() {
            write!(f, "extern ")?;
        }
        let mut params: Vec<String> = self.params.iter().map(Param::to_string).collect();
        if self.variadic {
            params.push("...".to_string());
        }
        write!(f, "fn {}({})", self.name, params.join(", "))?;
        if let Some(return_type) = &self.return_type {
            write!(f, " -> {return_type}")?;
        }
        return match &self.body {
            Some(body) => {
                writeln!(f, " {{")?;
                block(f, body, 1)?;
                writeln!(f, "}}")
            }
            None => writeln!(f),
        };
    }
}

impl fmt::Display for Param {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.ty, self.name)
    }
}

/// `int`, `short unsigned int`, `float&`, `int[10]`
impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return match self {
            Type::Int { short, unsigned } => {
                if *short {
                    write!(f, "short ")?;
                }
                if *unsigned {
                    write!(f, "unsigned ")?;
                }
                write!(f, "int")
            }
            Type::Float { short: true } => write!(f, "short float"),
            Type::Float { short: false } => write!(f, "float"),
            Type::Bool => write!(f, "bool"),
            Type::Char => write!(f, "char"),
            Type::Byte => write!(f, "byte"),
            Type::Address(ty) => write!(f, "{ty}&"),
            Type::Array(ty, count) => write!(f, "{ty}[{count}]"),
            Type::Custom(name) => write!(f, "{name}"),
        };
    }
}

/// Each statement on its own line at `depth`, the blocks inside one level deeper
fn block(f: &mut fmt::Formatter, statements: &[Statement], depth: usize) -> fmt::Result {
    for statement in statements {
        write!(f, "{}", "    ".repeat(depth))?;
        line(f, statement, depth)?;
    }
    return Ok(());
}

/// One statement, already indented; ends with its newline
fn line(f: &mut fmt::Formatter, statement: &Statement, depth: usize) -> fmt::Result {
    let indent = "    ".repeat(depth);
    return match &statement.kind {
        StatementKind::Declare { name, ty } => writeln!(f, "let {name}: {ty}"),
        StatementKind::Init {
            name,
            value,
            constant,
        } => writeln!(f, "{} {name} = {value}", if *constant { "const" } else { "let" }),
        StatementKind::Assign { target, value } => writeln!(f, "{target} = {value}"),
        StatementKind::Return(value) => writeln!(f, "return {value}"),
        StatementKind::Call(call) => writeln!(f, "{call}"),
        StatementKind::If {
            condition,
            then,
            otherwise,
        } => {
            writeln!(f, "if {condition} {{")?;
            block(f, then, depth + 1)?;
            match otherwise.as_deref() {
                None => writeln!(f, "{indent}}}"),
                // `아니면 만약`: `} else if …` instead of a nested block
                Some([inner]) if matches!(inner.kind, StatementKind::If { .. }) => {
                    write!(f, "{indent}}} else ")?;
                    line(f, inner, depth)
                }
                Some(otherwise) => {
                    writeln!(f, "{indent}}} else {{")?;
                    block(f, otherwise, depth + 1)?;
                    writeln!(f, "{indent}}}")
                }
            }
        }
        StatementKind::While { condition, body } => {
            writeln!(f, "while {condition} {{")?;
            block(f, body, depth + 1)?;
            writeln!(f, "{indent}}}")
        }
        StatementKind::For {
            variable,
            ty,
            from,
            until,
            body,
        } => {
            match ty {
                Some(ty) => write!(f, "for {variable}: {ty}")?,
                None => write!(f, "for {variable}")?,
            }
            writeln!(f, " in {from}..{until} {{")?;
            block(f, body, depth + 1)?;
            writeln!(f, "{indent}}}")
        }
        StatementKind::Break => writeln!(f, "break"),
        StatementKind::Continue => writeln!(f, "continue"),
    };
}

/// An operand of an operator: in parentheses when it is itself an operation, `(a + b) * c`
fn operand(expr: &Expr) -> String {
    return match expr.kind {
        ExprKind::Binary { .. } => format!("({expr})"),
        _ => expr.to_string(),
    };
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return match &self.kind {
            ExprKind::Int(number) => write!(f, "{number}"),
            ExprKind::Float(number) => write!(f, "{number:?}"),
            ExprKind::Str(text) => write!(f, "{text:?}"),
            ExprKind::Bool(value) => write!(f, "{value}"),
            ExprKind::Name(name) => write!(f, "{name}"),
            ExprKind::Null => write!(f, "null"),
            ExprKind::SizeOf(ty) => write!(f, "sizeof({ty})"),
            ExprKind::Unary { op, operand: inner } => write!(f, "{op}{}", operand(inner)),
            ExprKind::Binary { op, left, right } => {
                write!(f, "{} {op} {}", operand(left), operand(right))
            }
            ExprKind::Call { verb, args } => {
                let args: Vec<String> = args.iter().map(Argument::to_string).collect();
                write!(f, "{verb}({})", args.join(", "))
            }
        };
    }
}

/// Only the value: the particle has already given the argument its role
impl fmt::Display for Argument {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return match self {
            Argument::Value { value, .. } => write!(f, "{value}"),
            Argument::Previous(value) => write!(f, "{value}"),
            Argument::ToType(ty) => write!(f, "{ty}"),
        };
    }
}

impl fmt::Display for UnaryOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let symbol = match self {
            UnaryOp::Neg => "-",
            UnaryOp::BitNot => "~",
            UnaryOp::Not => "!",
        };
        return write!(f, "{symbol}");
    }
}

impl fmt::Display for BinaryOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let symbol = match self {
            BinaryOp::Or => "||",
            BinaryOp::And => "&&",
            BinaryOp::BitOr => "|",
            BinaryOp::BitXor => "^",
            BinaryOp::BitAnd => "&",
            BinaryOp::Eq => "==",
            BinaryOp::NotEq => "!=",
            BinaryOp::Lt => "<",
            BinaryOp::Le => "<=",
            BinaryOp::Gt => ">",
            BinaryOp::Ge => ">=",
            BinaryOp::Shl => "<<",
            BinaryOp::Shr => ">>",
            BinaryOp::Add => "+",
            BinaryOp::Sub => "-",
            BinaryOp::Mul => "*",
            BinaryOp::Div => "/",
            BinaryOp::Rem => "%",
        };
        return write!(f, "{symbol}");
    }
}
