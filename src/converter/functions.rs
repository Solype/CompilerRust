use crate::converter::ir;
use crate::lexer::span::Span;
use crate::parser::postparser::structs::{Function, If, Statement, StatementKind, Type};

use super::convert_error::{ConvertError, ConvertErrorKind};
use super::converter::Converter;
use super::scope::{BlockKind, Scope, Variable};

impl<'a> Converter<'a> {
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

    /// Checks the body of `func`, its parameters in the same scope as its variables
    pub(super) fn convert_func(
        &mut self,
        func: &'a Function,
    ) -> Result<ir::structs::Function, ConvertError> {
        // `외부`: nothing to check

        let mut ir_func = ir::structs::Function::new(
            func.mangled_name.clone(),
            func.return_type.clone(),
            func.variadic,
        );

        let Some(statements) = &func.body else {
            return Ok(ir_func);
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
                var_id: ir_func.new_var(param.ty.clone()),
            };
            ir_func.params.push(variable.var_id.clone());
            scope.variables.insert(param.name.clone(), variable);
        }
        ir_func.body = Some(self.convert_block(statements, &scope, &mut ir_func)?);

        if func.return_type.is_some() && !always_returns(statements) {
            let kind = ConvertErrorKind::MissingReturn(func.name.clone());
            return Err(ConvertError::new(kind, func.span));
        }
        Ok(ir_func)
    }

    /// The program has a `main` that returns a 정수
    pub(super) fn check_main(&self) -> Result<(), ConvertError> {
        let Some(main) = self
            .program
            .functions
            .iter()
            .find(|func| func.name == "main")
        else {
            return Err(ConvertError::new(ConvertErrorKind::NoMain, Span::new(0, 0)));
        };
        if !matches!(main.return_type, Some(Type::Int { .. })) {
            return Err(ConvertError::new(
                ConvertErrorKind::MainNotInteger,
                main.span,
            ));
        }
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
