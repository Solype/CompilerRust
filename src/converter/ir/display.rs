use std::fmt;

use super::structs::{Function, Instr, Label, Operand, Program, VarId};

/// The IR as three address code: the strings, then one function after the other
impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, string) in self.strings.iter().enumerate() {
            writeln!(f, "s{i} = {string:?}")?;
        }
        for (i, float) in self.floats.iter().enumerate() {
            writeln!(f, "f{i} = {float}")?;
        }
        for function in &self.funcs {
            writeln!(f)?;
            write!(f, "{function}")?;
        }
        return Ok(());
    }
}

/// `fn 더하다$int$int(v0, v1) -> int`, its variables and their type, then its instructions
impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.body.is_none() {
            write!(f, "extern ")?;
        }
        let mut params: Vec<String> = self.params.iter().map(VarId::to_string).collect();
        if self.variadic {
            params.push("...".to_string());
        }
        write!(f, "fn {}({})", self.symbol, params.join(", "))?;
        if let Some(return_type) = &self.return_type {
            write!(f, " -> {return_type}")?;
        }
        writeln!(f)?;
        let Some(body) = &self.body else {
            return Ok(());
        };
        for (i, ty) in self.vars.iter().enumerate() {
            writeln!(f, "    var v{i}: {ty}")?;
        }
        for instr in body {
            match instr {
                Instr::Label(_) => writeln!(f, "{instr}")?,
                _ => writeln!(f, "    {instr}")?,
            }
        }
        return Ok(());
    }
}

impl fmt::Display for VarId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "v{}", self.0)
    }
}

impl fmt::Display for Label {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "L{}", self.0)
    }
}

/// `v0`, `42`, `1.5`, `true`, `s0` (the index in the strings), `null`
impl fmt::Display for Operand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return match self {
            Operand::Var(var) => write!(f, "{var}"),
            Operand::Int(value) => write!(f, "{value}"),
            Operand::Float(value) => write!(f, "{value:?}"),
            Operand::Bool(value) => write!(f, "{value}"),
            Operand::Str(index) => write!(f, "s{index}"),
            Operand::Null => write!(f, "null"),
        };
    }
}

impl fmt::Display for Instr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return match self {
            Instr::Copy { dest, src } => write!(f, "{dest} = {src}"),
            Instr::Unary { dest, op, src } => write!(f, "{dest} = {op}{src}"),
            Instr::Binary {
                dest,
                op,
                left,
                right,
            } => write!(f, "{dest} = {left} {op} {right}"),
            Instr::Cast { dest, src } => write!(f, "{dest} = cast {src}"),
            Instr::Call { dest, symbol, args } => {
                let args: Vec<String> = args.iter().map(Operand::to_string).collect();
                if let Some(dest) = dest {
                    write!(f, "{dest} = ")?;
                }
                write!(f, "call {symbol}({})", args.join(", "))
            }
            Instr::Label(label) => write!(f, "{label}:"),
            Instr::Jump(label) => write!(f, "jump {label}"),
            Instr::JumpIfNot { cond, target } => write!(f, "jump_if_not {cond}, {target}"),
            Instr::Return(value) => write!(f, "return {value}"),
            Instr::Comment(text) => write!(f, "// {text}"),
        };
    }
}
