use crate::converter::expressions::Callee;
use crate::converter::ir;
use crate::converter::ir::structs::Instr::Cast;
use crate::converter::scope::LoopLabels;
use crate::lexer::span::Span;
use crate::parser::postparser::structs::{
    Assign, Declare, Expr, ExprKind, For, Function, If, Init, Statement, StatementKind, Type, While,
};

use super::convert_error::{ConvertError, ConvertErrorKind};
use super::converter::Converter;
use super::expressions::operand;
use super::scope::{BlockKind, Scope, Variable};
use super::types::{fits, range_error, type_name};

impl<'a> Converter<'a> {
    /// `condition` must be a 논리
    fn convert_condition(
        &self,
        condition: &Expr,
        scope: &Scope,
    ) -> Result<Vec<ir::structs::Instr>, ConvertError> {
        match self.get_type(condition, scope)? {
            Type::Bool => Ok(vec![]),
            _ => Err(ConvertError::new(ConvertErrorKind::NotBool, condition.span)),
        }
    }

    /// `수를 1부터 끝까지 세면서 { … }`: the variable and the bounds are the same 정수 type
    fn convert_for_loop(
        &mut self,
        parent: &Scope,
        for_loop: &'a For,
        span: Span,
        ir_func: &mut ir::structs::Function,
    ) -> Result<Vec<ir::structs::Instr>, ConvertError> {
        let (label_start, label_end) = parent.generate_loop_label();
        let mut scope = Scope::new(
            BlockKind::Loop(LoopLabels {
                label_start: label_start.clone(),
                label_end: label_end.clone(),
            }),
            Some(parent),
        );
        let ty = match &for_loop.ty {
            // `정수 수를 …`: declared by the loop, in its own scope
            Some(ty) => {
                let variable = Variable {
                    ty: ty.clone(),
                    constant: false,
                    var_id: ir_func.new_var(ty.clone()),
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
        let mut instr = self.convert_block(&for_loop.body, &scope, ir_func)?;
        instr.insert(0, ir::structs::Instr::Label(ir_func.label_id(&label_start)));
        instr.push(ir::structs::Instr::Jump(ir_func.label_id(&label_start)));
        instr.push(ir::structs::Instr::Label(ir_func.label_id(&label_end)));
        return Ok(instr);
    }

    fn convert_while_loop(
        &mut self,
        parent: &Scope,
        while_loop: &'a While,
        ir_func: &mut ir::structs::Function,
    ) -> Result<Vec<ir::structs::Instr>, ConvertError> {
        let (label_start, label_end) = parent.generate_loop_label();
        let scope = Scope::new(
            BlockKind::Loop(LoopLabels {
                label_start,
                label_end,
            }),
            Some(parent),
        );
        let _cond = self.convert_condition(&while_loop.condition, parent)?;
        return self.convert_block(&while_loop.body, &scope, ir_func);
    }

    /// `수는 정수예요.`: adds the variable to `scope`
    fn convert_declare(
        &mut self,
        scope: &mut Scope,
        declare: &'a Declare,
        span: Span,
        ir_func: &mut ir::structs::Function,
    ) -> Result<Vec<ir::structs::Instr>, ConvertError> {
        scope.declare(
            &declare.name,
            Variable {
                ty: declare.ty.clone(),
                constant: false,
                var_id: ir_func.new_var(declare.ty.clone()),
            },
            span,
        )?;
        let inst: Vec<ir::structs::Instr> = vec![];
        return Ok(inst);
    }

    /// `결과는 0이에요.`: adds the variable to `scope`
    fn convert_init(
        &mut self,
        scope: &mut Scope,
        init: &'a Init,
        span: Span,
        ir_func: &mut ir::structs::Function,
    ) -> Result<Vec<ir::structs::Instr>, ConvertError> {
        let ty = self.get_type(&init.value, scope)?;
        // Before the declaration: in `가는 가이에요`, the value is the 가 of the blocks around
        let oper = operand(&init.value, scope);
        let var = ir_func.new_var(ty.clone());

        let variable = Variable {
            var_id: var,
            ty,
            constant: init.constant,
        };
        scope.declare(&init.name, variable, span)?;

        let inst: Vec<ir::structs::Instr> = match oper {
            Some(oper) => vec![ir::structs::Instr::Copy {
                dest: var,
                src: oper,
            }],
            None => vec![],
        };

        return Ok(inst);
    }

    /// `… 결과에 넣어요.`
    fn convert_assign(
        &mut self,
        scope: &Scope,
        assign: &'a Assign,
        span: Span,
    ) -> Result<Vec<ir::structs::Instr>, ConvertError> {
        // check if the variable exists
        let Some(var) = scope.find_variable(&assign.target) else {
            let kind = ConvertErrorKind::UnknownVariable(assign.target.clone());
            return Err(ConvertError::new(kind, span));
        };
        // check if the var is const
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
        let inst: Vec<ir::structs::Instr> = match operand(&assign.value, scope) {
            Some(oper) => vec![ir::structs::Instr::Copy {
                dest: var.var_id,
                src: oper,
            }],
            None => vec![],
        };
        return Ok(inst);
    }

    /// `… 반환해요.`
    fn convert_return(
        &mut self,
        scope: &Scope,
        value: &'a Expr,
        span: Span,
    ) -> Result<Vec<ir::structs::Instr>, ConvertError> {
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
        Ok(vec![])
    }

    /// `… printf해요.`: a call whose result is unused
    fn convert_call(
        &mut self,
        scope: &Scope,
        call: &'a Expr,
    ) -> Result<Vec<ir::structs::Instr>, ConvertError> {
        let mut instr: Vec<ir::structs::Instr> = vec![];

        match &call.kind {
            // Alone, a call may return nothing
            ExprKind::Call { verb, args } => {
                let res: Callee = self.resolve_call(verb, args, scope, call.span)?;
                match res {
                    Callee::Function(f) => {
                        instr.push(ir::structs::Instr::Call {
                            dest: None,
                            symbol: f.mangled_name.clone(),
                            args: vec![],
                        });
                    }
                    Callee::Cast(ty) => {
                        // instr.push(ir);
                    }
                }
            }
            _ => {
                self.get_type(call, scope)?;
            }
        }
        Ok(instr)
    }

    /// `만약 … { … } 아니면 { … }`
    fn convert_if(
        &mut self,
        parent: &mut Scope,
        if_statement: &'a If,
        ir_func: &mut ir::structs::Function,
    ) -> Result<Vec<ir::structs::Instr>, ConvertError> {
        let _cond = self.convert_condition(&if_statement.condition, parent)?;
        // Each branch takes its own count: two loops in `만약` and `아니면` get distinct labels
        parent.block_count += 1;
        self.convert_block(&if_statement.then, parent, ir_func)?;
        if let Some(otherwise_block) = &if_statement.otherwise {
            parent.block_count += 1;
            self.convert_block(&otherwise_block, parent, ir_func)?;
        }
        return Ok(vec![]);
    }

    /// `그만해요`
    fn convert_break(
        &self,
        scope: &Scope,
        span: Span,
    ) -> Result<Vec<ir::structs::Instr>, ConvertError> {
        if let Some(loop_labels) = scope.in_loop() {
            return Ok(vec![]);
            // return Ok(vec![ir::structs::Instr::Jump(ir::structs::Label::)]);
        }
        return Err(ConvertError::new(ConvertErrorKind::BreakOutOfLoop, span));
    }

    /// `넘어가요`
    fn convert_continue(
        &self,
        scope: &Scope,
        span: Span,
    ) -> Result<Vec<ir::structs::Instr>, ConvertError> {
        if scope.in_loop().is_some() {
            return Ok(vec![]);
        }
        return Err(ConvertError::new(ConvertErrorKind::ContinueOutOfLoop, span));
    }

    /// Checks `block` in its own scope, which starts with the variables of `scope` (the
    /// parameters, the variable of a `세면서`); its variables get their id from `ir_func`
    pub(super) fn convert_block(
        &mut self,
        block: &'a [Statement],
        scope: &Scope,
        ir_func: &mut ir::structs::Function,
    ) -> Result<Vec<ir::structs::Instr>, ConvertError> {
        let mut new_scope = Scope::new(BlockKind::Block, Some(scope));
        let mut instructions: Vec<ir::structs::Instr> = vec![];

        for statement in block {
            instructions.push(ir::structs::Instr::Comment(source_line(statement)));
            let mut new_inst = match &statement.kind {
                StatementKind::Declare(declare) => {
                    self.convert_declare(&mut new_scope, declare, statement.span, ir_func)?
                }
                StatementKind::Init(init) => {
                    self.convert_init(&mut new_scope, init, statement.span, ir_func)?
                }
                StatementKind::Assign(assign) => {
                    self.convert_assign(&new_scope, assign, statement.span)?
                }
                StatementKind::Return(value) => {
                    self.convert_return(&new_scope, value, statement.span)?
                }
                StatementKind::Call(call) => self.convert_call(&new_scope, call)?,
                StatementKind::If(if_statement) => {
                    self.convert_if(&mut new_scope, if_statement, ir_func)?
                }
                StatementKind::While(while_loop) => {
                    new_scope.block_count += 1;
                    self.convert_while_loop(&new_scope, while_loop, ir_func)?
                }
                StatementKind::For(for_loop) => {
                    new_scope.block_count += 1;
                    self.convert_for_loop(&new_scope, for_loop, statement.span, ir_func)?
                }
                StatementKind::Break => self.convert_break(&new_scope, statement.span)?,
                StatementKind::Continue => self.convert_continue(&new_scope, statement.span)?,
            };
            instructions.append(&mut new_inst);
        }
        Ok(instructions)
    }
}

/// The first line of `statement` as the AST shows it, without its blocks: the comment above
/// its instructions
fn source_line(statement: &Statement) -> String {
    return match &statement.kind {
        StatementKind::Declare(Declare { name, ty }) => format!("let {name}: {ty}"),
        StatementKind::Init(Init {
            name,
            value,
            constant,
        }) => format!(
            "{} {name} = {value}",
            if *constant { "const" } else { "let" }
        ),
        StatementKind::Assign(Assign { target, value }) => format!("{target} = {value}"),
        StatementKind::Return(value) => format!("return {value}"),
        StatementKind::Call(call) => call.to_string(),
        StatementKind::If(If { condition, .. }) => format!("if {condition}"),
        StatementKind::While(While { condition, .. }) => format!("while {condition}"),
        StatementKind::For(For {
            variable,
            from,
            until,
            ..
        }) => format!("for {variable} in {from}..{until}"),
        StatementKind::Break => "break".to_string(),
        StatementKind::Continue => "continue".to_string(),
    };
}
