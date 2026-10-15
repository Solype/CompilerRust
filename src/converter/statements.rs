use crate::lexer::span::Span;
use crate::parser::postparser::structs::{
    Assign, Declare, Expr, ExprKind, For, If, Init, Statement, StatementKind, Type, While,
};

use super::convert_error::{ConvertError, ConvertErrorKind};
use super::converter::Converter;
use super::scope::{BlockKind, Scope, Variable};
use super::types::{fits, range_error, type_name};

impl<'a> Converter<'a> {
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
                let Some(var) = parent.find_variable(&for_loop.variable) else {
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
            let bound_ty = self.get_type(bound, parent)?;
            if !fits(bound, &bound_ty, &ty) {
                if let Some(err) = range_error(bound, &bound_ty, &ty) {
                    return Err(err);
                }
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
        scope.declare(
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
        let variable = Variable {
            ty: self.get_type(&init.value, scope)?,
            constant: init.constant,
        };
        scope.declare(&init.name, variable, span)
    }

    /// `… 결과에 넣어요.`
    fn convert_assign(
        &mut self,
        scope: &Scope,
        assign: &'a Assign,
        span: Span,
    ) -> Result<(), ConvertError> {
        let Some(var) = scope.find_variable(&assign.target) else {
            let kind = ConvertErrorKind::UnknownVariable(assign.target.clone());
            return Err(ConvertError::new(kind, span));
        };
        if var.constant {
            let kind = ConvertErrorKind::AssignToConstant(assign.target.clone());
            return Err(ConvertError::new(kind, span));
        }
        let assign_value_ty = self.get_type(&assign.value, scope)?;
        if !fits(&assign.value, &assign_value_ty, &var.ty) {
            if let Some(err) = range_error(&assign.value, &assign_value_ty, &var.ty) {
                return Err(err);
            }
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
    fn convert_return(
        &mut self,
        scope: &Scope,
        value: &'a Expr,
        span: Span,
    ) -> Result<(), ConvertError> {
        let ret_ty = self.get_type(value, scope)?;
        let Some(fnc) = scope.function() else {
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
            if let Some(err) = range_error(value, &ret_ty, decl_ret_ty) {
                return Err(err);
            }
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
    pub(super) fn convert_block(
        &mut self,
        block: &'a [Statement],
        scope: &Scope,
    ) -> Result<(), ConvertError> {
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
}
