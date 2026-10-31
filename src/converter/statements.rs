use crate::converter::expressions::Callee;
use crate::converter::ir;
use crate::converter::scope::LoopLabels;
use crate::lexer::span::Span;
use crate::parser::postparser::structs::{
    Argument, Assign, BinaryOp, Declare, Expr, ExprKind, For, Function, If, Init, Statement,
    StatementKind, Type, UnaryOp, While,
};

use super::convert_error::{ConvertError, ConvertErrorKind};
use super::converter::Converter;
use super::expressions::operand;
use super::scope::{BlockKind, Scope, Variable};
use super::types::{fits, range_error, type_name};

impl<'a> Converter<'a> {
    /// TODO !!!
    /// The operand of `condition`, which must be a 논리; the instructions that compute it are
    /// pushed into `instrs`
    fn convert_cond(
        &mut self,
        condition: &Expr,
        scope: &Scope,
        ir_func: &mut ir::structs::Function,
        instrs: &mut Vec<ir::structs::Instr>,
    ) -> Result<ir::structs::Operand, ConvertError> {
        match self.get_type(condition, scope)? {
            Type::Bool => self.convert_expr(condition, scope, ir_func, instrs),
            _ => Err(ConvertError::new(ConvertErrorKind::NotBool, condition.span)),
        }
    }

    /// The operand of `expr`; the instructions that compute it are pushed into `instrs`
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
                self.convert_binary(*op, left, right, ty, scope, ir_func, instrs)
            }
            ExprKind::Unary { op, operand } => {
                self.convert_unary(*op, operand, ty, scope, ir_func, instrs)
            }
            ExprKind::Call { verb, args } => {
                self.convert_call_value(verb, args, expr.span, ty, scope, ir_func, instrs)
            }
            ExprKind::Str(str) => Ok(ir::structs::Operand::DataAddress(self.ir.string_id(str))),
            ExprKind::SizeOf(ty) => Ok(ir::structs::Operand::Int(ty.size() as i64, false)),
            _ => unimplemented!(),
        }
    }

    /// `op` as a value of type `expected`, once `fits` accepted it: a literal takes the size of
    /// `expected` (2.5 on 4 bytes for a 짧은 소수), a 소수 of the other size goes through a Cast
    fn fit_operand(
        &mut self,
        op: ir::structs::Operand,
        expected: &Type,
        ir_func: &mut ir::structs::Function,
        instrs: &mut Vec<ir::structs::Instr>,
    ) -> ir::structs::Operand {
        return match (op, expected) {
            (ir::structs::Operand::Float(id), Type::Float { short }) => {
                let value = self.ir.floats[id].0;
                ir::structs::Operand::Float(self.ir.float_id(value, *short))
            }
            (ir::structs::Operand::Int(value, _), Type::Int { short, .. }) => {
                ir::structs::Operand::Int(value, *short)
            }
            // a 소수 computed from literals, (1.5 + 2.0) for a 짧은 소수
            (ir::structs::Operand::Var(var), Type::Float { .. })
                if var.ty.is_float() && var.ty != *expected =>
            {
                let dest = ir_func.new_var(expected.clone());
                instrs.push(ir::structs::Instr::Cast {
                    dest: dest.clone(),
                    src: ir::structs::Operand::Var(var),
                });
                ir::structs::Operand::Var(dest)
            }
            (op, _) => op,
        };
    }

    /// The arguments of a call to `func`, each fitted to its parameter; after the parameters of a
    /// variadic, a 짧은 소수 goes as a 소수, as C promotes it
    fn fit_args(
        &mut self,
        operands: Vec<ir::structs::Operand>,
        func: &Function,
        ir_func: &mut ir::structs::Function,
        instrs: &mut Vec<ir::structs::Instr>,
    ) -> Vec<ir::structs::Operand> {
        let double = Type::Float { short: false };
        let mut args = Vec::new();
        for (i, op) in operands.into_iter().enumerate() {
            let expected = func.params.get(i).map_or(&double, |param| &param.ty);
            args.push(self.fit_operand(op, expected, ir_func, instrs));
        }
        return args;
    }

    /// `left op right` into a new variable of type `ty`
    #[allow(clippy::too_many_arguments)]
    fn convert_binary(
        &mut self,
        op: BinaryOp,
        left: &Expr,
        right: &Expr,
        ty: Type,
        scope: &Scope,
        ir_func: &mut ir::structs::Function,
        instrs: &mut Vec<ir::structs::Instr>,
    ) -> Result<ir::structs::Operand, ConvertError> {
        let operand_ty = self.common_type(left, right, scope)?.unwrap_or(ty.clone());

        let left = self.convert_expr(left, scope, ir_func, instrs)?;
        let right = self.convert_expr(right, scope, ir_func, instrs)?;
        // a literal beside a 짧은 value takes its size; a comparison fits its sides to each other
        let left = self.fit_operand(left, &operand_ty, ir_func, instrs);
        let right = self.fit_operand(right, &operand_ty, ir_func, instrs);
        let dest = ir_func.new_var(ty);
        instrs.push(ir::structs::Instr::Binary {
            dest: dest.clone(),
            op,
            left,
            right,
        });
        return Ok(ir::structs::Operand::Var(dest.clone()));
    }

    /// `op operand` into a new variable of type `ty`
    fn convert_unary(
        &mut self,
        op: UnaryOp,
        operand: &Expr,
        ty: Type,
        scope: &Scope,
        ir_func: &mut ir::structs::Function,
        instrs: &mut Vec<ir::structs::Instr>,
    ) -> Result<ir::structs::Operand, ConvertError> {
        let src = self.convert_expr(operand, scope, ir_func, instrs)?;
        let dest = ir_func.new_var(ty);
        instrs.push(ir::structs::Instr::Unary {
            dest: dest.clone(),
            op,
            src,
        });
        return Ok(ir::structs::Operand::Var(dest));
    }

    /// A call whose result is used: a Call, or a Cast for 바꾸다, into a new variable of type `ty`
    #[allow(clippy::too_many_arguments)]
    fn convert_call_value(
        &mut self,
        verb: &str,
        args: &[Argument],
        span: Span,
        ty: Type,
        scope: &Scope,
        ir_func: &mut ir::structs::Function,
        instrs: &mut Vec<ir::structs::Instr>,
    ) -> Result<ir::structs::Operand, ConvertError> {
        let callee = self.resolve_call(verb, args, scope, span)?;
        let mut operands = self.convert_args(args, scope, ir_func, instrs)?;
        let dest = ir_func.new_var(ty);
        match callee {
            Callee::Function(func) => {
                let args = self.fit_args(operands, func, ir_func, instrs);
                instrs.push(ir::structs::Instr::Call {
                    dest: Some(dest.clone()),
                    symbol: func.mangled_name.clone(),
                    args,
                })
            }
            // 바꾸다 : une seule valeur, le cast lit les deux types dans vars
            Callee::Cast(_) => instrs.push(ir::structs::Instr::Cast {
                dest: dest.clone(),
                src: operands.remove(0),
            }),
        }
        return Ok(ir::structs::Operand::Var(dest));
    }

    /// The operands of the values given to a call; `정수로` (the target type) gives none
    fn convert_args(
        &mut self,
        args: &[Argument],
        scope: &Scope,
        ir_func: &mut ir::structs::Function,
        instrs: &mut Vec<ir::structs::Instr>,
    ) -> Result<Vec<ir::structs::Operand>, ConvertError> {
        let mut operands = Vec::new();
        for arg in args {
            match arg {
                Argument::Value { value, .. } | Argument::Previous(value) => {
                    operands.push(self.convert_expr(value, scope, ir_func, instrs)?);
                }
                Argument::ToType(_) => {}
            }
        }
        return Ok(operands);
    }

    /// `수를 1부터 끝까지 세면서 { … }`: the variable and the bounds are the same 정수 type, `끝`
    /// excluded. `var = from ; Jump check ; start: var = var + 1 ; check: JumpIfNot var < until,
    /// end ; body ; Jump start ; end:`: the increment is at start, so 넘어가요 runs it
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
        let (ty, var) = match &for_loop.ty {
            // `정수 수를 …`: declared by the loop, in its own scope
            Some(ty) => {
                let var = ir_func.new_var(ty.clone());
                let variable = Variable {
                    ty: ty.clone(),
                    constant: false,
                    var_id: var.clone(),
                };
                scope.declare(&for_loop.variable, variable, span)?;
                (ty.clone(), var)
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
                (var.ty.clone(), var.var_id.clone())
            }
        };
        let Type::Int { short, .. } = ty else {
            return Err(ConvertError::new(ConvertErrorKind::NotInteger, span));
        };
        for bound in [&for_loop.from, &for_loop.until] {
            let bound_ty = self.get_type(bound, parent)?;
            if !fits(bound, &bound_ty, &ty) {
                if let Some(err) = range_error(bound, &bound_ty, &ty) {
                    return Err(err);
                }
                return Err(ConvertError::new(ConvertErrorKind::NotInteger, bound.span));
            }
        }
        let label_check = format!("{label_start}_check");
        let mut instrs = vec![];
        let from = self.convert_expr(&for_loop.from, parent, ir_func, &mut instrs)?;
        let from = self.fit_operand(from, &ty, ir_func, &mut instrs);
        instrs.push(ir::structs::Instr::Copy {
            dest: var.clone(),
            src: from,
        });
        instrs.push(ir::structs::Instr::Jump(ir_func.label_id(&label_check)));
        instrs.push(ir::structs::Instr::Label(ir_func.label_id(&label_start)));
        instrs.push(ir::structs::Instr::Binary {
            dest: var.clone(),
            op: BinaryOp::Add,
            left: ir::structs::Operand::Var(var.clone()),
            right: ir::structs::Operand::Int(1, short),
        });
        // until is computed again at each turn, as the condition of a C for
        instrs.push(ir::structs::Instr::Label(ir_func.label_id(&label_check)));
        let until = self.convert_expr(&for_loop.until, parent, ir_func, &mut instrs)?;
        let until = self.fit_operand(until, &ty, ir_func, &mut instrs);
        let cond = ir_func.new_var(Type::Bool);
        instrs.push(ir::structs::Instr::Binary {
            dest: cond.clone(),
            op: BinaryOp::Lt,
            left: ir::structs::Operand::Var(var),
            right: until,
        });
        instrs.push(ir::structs::Instr::JumpIfNot {
            cond: ir::structs::Operand::Var(cond),
            target: ir_func.label_id(&label_end),
        });
        instrs.append(&mut self.convert_block(&for_loop.body, &scope, ir_func)?);
        instrs.push(ir::structs::Instr::Jump(ir_func.label_id(&label_start)));
        instrs.push(ir::structs::Instr::Label(ir_func.label_id(&label_end)));
        return Ok(instrs);
    }

    /// TODO !!!
    fn convert_while_loop(
        &mut self,
        parent: &Scope,
        while_loop: &'a While,
        ir_func: &mut ir::structs::Function,
    ) -> Result<Vec<ir::structs::Instr>, ConvertError> {
        let (label_start, label_end) = parent.generate_labels("loop", "start");
        let scope = Scope::new(
            BlockKind::Loop(LoopLabels {
                label_start: label_start.clone(),
                label_end: label_end.clone(),
            }),
            Some(parent),
        );
        let mut cond_instrs: Vec<ir::structs::Instr> =
            vec![ir::structs::Instr::Label(ir_func.label_id(&label_start))];

        let cond = self.convert_cond(&while_loop.condition, parent, ir_func, &mut cond_instrs)?;

        cond_instrs.push(ir::structs::Instr::JumpIfNot {
            cond,
            target: ir_func.label_id(&label_end),
        });

        let mut block_instrs = self.convert_block(&while_loop.body, &scope, ir_func)?;
        cond_instrs.append(&mut block_instrs);
        cond_instrs.push(ir::structs::Instr::Jump(ir_func.label_id(&label_start)));
        cond_instrs.push(ir::structs::Instr::Label(ir_func.label_id(&label_end)));
        return Ok(cond_instrs);
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
        let ty = match self.get_type(&init.value, scope)? {
            // `없음은 빈 주소예요.`: an address to nothing, the void* of C
            Type::Null => Type::Address(Box::new(Type::Null)),
            ty => ty,
        };
        let mut instrs: Vec<ir::structs::Instr> = vec![];
        let src = self.convert_expr(&init.value, scope, ir_func, &mut instrs)?;

        let var = ir_func.new_var(ty.clone());

        let variable = Variable {
            var_id: var.clone(),
            ty,
            constant: init.constant,
        };
        scope.declare(&init.name, variable, span)?;
        instrs.push(ir::structs::Instr::Copy { dest: var, src });
        return Ok(instrs);
    }

    /// `… 결과에 넣어요.`
    fn convert_assign(
        &mut self,
        scope: &Scope,
        assign: &'a Assign,
        span: Span,
        ir_func: &mut ir::structs::Function,
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

        let mut instrs: Vec<ir::structs::Instr> = vec![];
        let src = self.convert_expr(&assign.value, scope, ir_func, &mut instrs)?;
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
        let src = self.fit_operand(src, &var.ty, ir_func, &mut instrs);
        instrs.push(ir::structs::Instr::Copy {
            dest: var.var_id.clone(),
            src,
        });
        return Ok(instrs);
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
        let value = self.fit_operand(value, decl_ret_ty, ir_func, &mut instrs);
        instrs.push(ir::structs::Instr::Return(value));
        return Ok(instrs);
    }

    /// `… printf해요.`: a call whose result is unused
    fn convert_call(
        &mut self,
        scope: &Scope,
        call: &'a Expr,
        ir_func: &mut ir::structs::Function,
    ) -> Result<Vec<ir::structs::Instr>, ConvertError> {
        let mut instrs: Vec<ir::structs::Instr> = vec![];

        match &call.kind {
            // Alone, a call may return nothing
            ExprKind::Call { verb, args } => {
                let res: Callee = self.resolve_call(verb, args, scope, call.span)?;
                match res {
                    Callee::Function(f) => {
                        let operands = self.convert_args(args, scope, ir_func, &mut instrs)?;
                        let op = ir::structs::Instr::Call {
                            dest: None,
                            symbol: f.mangled_name.clone(),
                            args: self.fit_args(operands, f, ir_func, &mut instrs),
                        };
                        instrs.push(op);
                    }
                    Callee::Cast(_) => unimplemented!(),
                }
            }
            _ => {
                self.get_type(call, scope)?;
            }
        }
        Ok(instrs)
    }

    /// TODO !!!
    /// `만약 … { … } 아니면 { … }`
    fn convert_if(
        &mut self,
        parent: &Scope,
        if_statement: &'a If,
        ir_func: &mut ir::structs::Function,
    ) -> Result<Vec<ir::structs::Instr>, ConvertError> {
        let mut cond_instrs = Vec::new();
        let cond = self.convert_cond(&if_statement.condition, parent, ir_func, &mut cond_instrs)?;
        let (otherwise_label, end_label) = parent.generate_labels("condition", "otherwise");

        let if_not_label = if if_statement.otherwise.is_some() {
            &otherwise_label
        } else {
            &end_label
        };

        cond_instrs.push(ir::structs::Instr::JumpIfNot {
            cond,
            target: ir_func.label_id(&if_not_label),
        });
        // convert the block
        let mut block_instr = self.convert_block(&if_statement.then, parent, ir_func)?;

        if let Some(otherwise_block) = &if_statement.otherwise {
            block_instr.push(ir::structs::Instr::Jump(ir_func.label_id(&end_label)));
            block_instr.push(ir::structs::Instr::Label(
                ir_func.label_id(&otherwise_label),
            ));
            let mut instr = self.convert_block(&otherwise_block, parent, ir_func)?;
            block_instr.append(&mut instr);
        }
        block_instr.push(ir::structs::Instr::Label(ir_func.label_id(&end_label)));
        cond_instrs.append(&mut block_instr);
        return Ok(cond_instrs);
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
        // The start of the innermost loop: the same name gives the Label its loop places; the
        // start of a 세면서 is its increment
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
                    self.convert_assign(&new_scope, assign, statement.span, ir_func)?
                }
                StatementKind::Return(value) => {
                    self.convert_return(&new_scope, value, statement.span, ir_func)?
                }
                StatementKind::Call(call) => self.convert_call(&new_scope, call, ir_func)?,
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
