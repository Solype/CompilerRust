use crate::converter::expressions::Callee;
use crate::converter::ir;
use crate::converter::scope::LoopLabels;
use crate::lexer::span::Span;
use crate::parser::postparser::structs::{
    Argument, Assign, Declare, Expr, ExprKind, For, If, Init, Statement, StatementKind, Type, While,
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

    fn convert_expr(
        &mut self,
        expr: &Expr,
        scope: &Scope,
        ir_func: &mut ir::structs::Function,
        instrs: &mut Vec<ir::structs::Instr>, // on pousse dedans : pas de Vec à concaténer
    ) -> Result<ir::structs::Operand, ConvertError> {
        if let Some(leaf) = operand(expr, scope, &mut self.ir) {
            return Ok(leaf);
        }
        let ty = self.get_type(expr, scope)?; // vérifs + type du résultat
        match &expr.kind {
            ExprKind::Binary { op, left, right } => {
                let left = self.convert_expr(left, scope, ir_func, instrs)?;
                let right = self.convert_expr(right, scope, ir_func, instrs)?;
                let dest = ir_func.new_var(ty);
                instrs.push(ir::structs::Instr::Binary {
                    dest,
                    op: *op,
                    left,
                    right,
                });
                Ok(ir::structs::Operand::Var(dest))
            }
            ExprKind::Unary { op, operand } => {
                let src = self.convert_expr(operand, scope, ir_func, instrs)?;
                let dest = ir_func.new_var(ty);
                instrs.push(ir::structs::Instr::Unary { dest, op: *op, src });
                Ok(ir::structs::Operand::Var(dest))
            }
            ExprKind::Call { verb, args } => {
                let callee = self.resolve_call(verb, args, scope, expr.span)?;
                let mut operands = Vec::new();
                for arg in args {
                    match arg {
                        Argument::Value { value, .. } | Argument::Previous(value) => {
                            operands.push(self.convert_expr(value, scope, ir_func, instrs)?);
                        }
                        Argument::ToType(_) => {}
                    }
                }
                let dest = ir_func.new_var(ty);
                match callee {
                    Callee::Function(func) => instrs.push(ir::structs::Instr::Call {
                        dest: Some(dest),
                        symbol: func.mangled_name.clone(),
                        args: operands,
                    }),
                    // 바꾸다 : une seule valeur, le cast lit les deux types dans vars
                    Callee::Cast(_) => instrs.push(ir::structs::Instr::Cast {
                        dest,
                        src: operands.remove(0),
                    }),
                }
                return Ok(ir::structs::Operand::Var(dest));
            }
            // Unary, Cast : même schéma ; Call : dest: Some(temporaire)
            ExprKind::Str(str) => Ok(ir::structs::Operand::Str(self.ir.string_id(str))),
            ExprKind::Bool(b) => Ok(ir::structs::Operand::Bool(*b)),
            ExprKind::Int(i) => Ok(ir::structs::Operand::Int(*i)),
            ExprKind::Float(f) => Ok(ir::structs::Operand::Float(self.ir.float_id(*f, false))),
            ExprKind::SizeOf(ty) => Ok(ir::structs::Operand::Int(ir::structs::get_size_of(ty))),
            _ => unimplemented!(),
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
        let (label_start, label_end) = parent.generate_labels("loop", "start");
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
                scope.declare(&for_loop.variable, variable, span)?;
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
        let (label_start, label_end) = parent.generate_labels("loop", "start");
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
        // Before the declaration: in `가는 가이에요`, 가 is not declared yet, declare refuses it
        let oper = operand(&init.value, scope, &mut self.ir);
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
            None => unimplemented!(),
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
        let inst: Vec<ir::structs::Instr> = match operand(&assign.value, scope, &mut self.ir) {
            Some(oper) => vec![ir::structs::Instr::Copy {
                dest: var.var_id,
                src: oper,
            }],
            None => unimplemented!(),
        };
        return Ok(inst);
    }

    /// `… 반환해요.`
    fn convert_return(
        &mut self,
        scope: &Scope,
        value: &'a Expr,
        span: Span,
        ir_func: &mut ir::structs::Function,
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
        let mut instrs: Vec<ir::structs::Instr> = vec![];
        let value = self.convert_expr(value, scope, ir_func, &mut instrs)?;
        instrs.push(ir::structs::Instr::Return(value));
        return Ok(instrs);
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
                    Callee::Cast(_) => {
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
        parent: &Scope,
        if_statement: &'a If,
        ir_func: &mut ir::structs::Function,
    ) -> Result<Vec<ir::structs::Instr>, ConvertError> {
        let _cond = self.convert_condition(&if_statement.condition, parent)?;
        let (otherwise_label, end_label) = parent.generate_labels("condition", "otherwise");
        let mut block_instr = self.convert_block(&if_statement.then, parent, ir_func)?;
        if let Some(otherwise_block) = &if_statement.otherwise {
            block_instr.push(ir::structs::Instr::Label(
                ir_func.label_id(&otherwise_label),
            ));
            let mut instr = self.convert_block(&otherwise_block, parent, ir_func)?;
            block_instr.append(&mut instr);
        }
        block_instr.push(ir::structs::Instr::Label(ir_func.label_id(&end_label)));
        return Ok(block_instr);
    }

    /// `그만해요`
    fn convert_break(
        &self,
        scope: &Scope,
        span: Span,
        ir_func: &mut ir::structs::Function,
    ) -> Result<Vec<ir::structs::Instr>, ConvertError> {
        // The end of the innermost loop: the same name gives the Label its loop places
        if let Some(loop_labels) = scope.in_loop() {
            let end = ir_func.label_id(&loop_labels.label_end);
            return Ok(vec![ir::structs::Instr::Jump(end)]);
        }
        return Err(ConvertError::new(ConvertErrorKind::BreakOutOfLoop, span));
    }

    /// `넘어가요`
    fn convert_continue(
        &self,
        scope: &Scope,
        span: Span,
        ir_func: &mut ir::structs::Function,
    ) -> Result<Vec<ir::structs::Instr>, ConvertError> {
        // The start of the innermost loop: the same name gives the Label its loop places
        // shortcut: a 세면서 must jump to its increment instead, once the loop emits one
        if let Some(loop_labels) = scope.in_loop() {
            let start = ir_func.label_id(&loop_labels.label_start);
            return Ok(vec![ir::structs::Instr::Jump(start)]);
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
                    self.convert_return(&new_scope, value, statement.span, ir_func)?
                }
                StatementKind::Call(call) => self.convert_call(&new_scope, call)?,
                StatementKind::If(if_statement) => {
                    self.convert_if(&new_scope, if_statement, ir_func)?
                }
                StatementKind::While(while_loop) => {
                    self.convert_while_loop(&new_scope, while_loop, ir_func)?
                }
                StatementKind::For(for_loop) => {
                    self.convert_for_loop(&new_scope, for_loop, statement.span, ir_func)?
                }
                StatementKind::Break => self.convert_break(&new_scope, statement.span, ir_func)?,
                StatementKind::Continue => {
                    self.convert_continue(&new_scope, statement.span, ir_func)?
                }
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
