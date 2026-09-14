//! Finite derivative loops whose parameter range bounds every instance.

use super::*;

impl SemanticAnalyzer {
    /// Prove a small upper bound without evaluating a configurable default.
    /// This is deliberately a proof for a subset of loop shapes: failure leaves
    /// the ordinary runtime lowering and its derivative-cycle diagnostic intact.
    pub(super) fn parameter_bounded_ddx_iterations(
        &mut self,
        loop_: &ForStmt,
        init: Option<f64>,
    ) -> Option<usize> {
        let counter = self.resolve_substituted_name(&loop_.var);
        if self.symbols.lookup(&counter)?.value_type != ValueType::Integer {
            return None;
        }
        let Expression::Binary(condition) = &loop_.condition else {
            return None;
        };
        let (Expression::Identifier(index), Expression::Identifier(bound)) =
            (&*condition.left, &*condition.right)
        else {
            return None;
        };
        if self.resolve_substituted_name(&index.name) != counter {
            return None;
        }
        let parameter = self
            .symbols
            .lookup(&self.resolve_substituted_name(&bound.name))?;
        if parameter.kind != SymbolKind::Parameter || parameter.value_type != ValueType::Integer {
            return None;
        }
        let range = parameter.attrs.range.as_ref()?;
        // Using an excluded endpoint can only overestimate the number of
        // iterations. A bound that names another parameter remains dynamic.
        let extreme = match condition.op {
            BinaryOp::Lt | BinaryOp::Le => range.max?,
            BinaryOp::Gt | BinaryOp::Ge => range.min?,
            _ => return None,
        };
        if !extreme.is_finite() {
            return None;
        }
        let mut has_ddx = false;
        if !self.bounded_ddx_body(&loop_.body, &counter, &mut has_ddx) || !has_ddx {
            return None;
        }
        // A constant update is evaluated with the compiler's checked integer
        // rules. Refuse overflow, cycles, and work beyond the existing static
        // expansion budget; no larger implicit derivative limit is introduced.
        let mut value = f64::from(real_to_integer(init?).ok()?);
        for iterations in 0..=Self::MAX_STATIC_UNROLL_ITERATIONS {
            let active = match condition.op {
                BinaryOp::Lt => value < extreme,
                BinaryOp::Le => value <= extreme,
                BinaryOp::Gt => value > extreme,
                BinaryOp::Ge => value >= extreme,
                _ => unreachable!(),
            };
            if !active {
                return Some(iterations);
            }
            self.subst_stack.push(HashMap::from([(
                loop_.var.clone(),
                Self::number_expr(value, loop_.span),
            )]));
            let update = self
                .lower_expression_without_side_effects(&loop_.update.value, "for-loop update")
                .ok()
                .and_then(|expression| self.eval_const_invariant(&expression));
            self.subst_stack.pop();
            let next = update?;
            // Only monotone integer counters are covered. This also excludes
            // arithmetic wrapping through the signed integer boundary.
            if next.fract() != 0.0
                || next < f64::from(i32::MIN)
                || next > f64::from(i32::MAX)
                || match condition.op {
                    BinaryOp::Lt | BinaryOp::Le => next <= value,
                    _ => next >= value,
                }
            {
                return None;
            }
            value = next;
        }
        None
    }

    /// Body writes and calls must not invalidate the counter proof. Functions
    /// with side effects and nested loops need their own effect/bound analysis;
    /// do not infer safety from the visible assignment target alone.
    fn bounded_ddx_body(
        &self,
        statement: &AnalogStatement,
        counter: &SmolStr,
        has_ddx: &mut bool,
    ) -> bool {
        match statement {
            AnalogStatement::Assignment(assignment) => {
                self.resolve_substituted_name(assignment.target_name()) != *counter
                    && self.bounded_ddx_expression(&assignment.value, has_ddx)
                    && match &assignment.target {
                        LValue::ArrayAccess { index, .. } => {
                            self.bounded_ddx_expression(index, has_ddx)
                        }
                        LValue::Variable { .. } => true,
                    }
            }
            AnalogStatement::Block(block) => {
                block.variables.is_empty()
                    && block
                        .statements
                        .iter()
                        .all(|statement| self.bounded_ddx_body(statement, counter, has_ddx))
            }
            AnalogStatement::Conditional(condition) => {
                self.bounded_ddx_expression(&condition.condition, has_ddx)
                    && self.bounded_ddx_body(&condition.then_branch, counter, has_ddx)
                    && condition
                        .else_branch
                        .as_deref()
                        .is_none_or(|statement| self.bounded_ddx_body(statement, counter, has_ddx))
            }
            AnalogStatement::Case(case) => {
                self.bounded_ddx_expression(&case.expr, has_ddx)
                    && case.items.iter().all(|item| {
                        item.matches
                            .iter()
                            .all(|expression| self.bounded_ddx_expression(expression, has_ddx))
                            && self.bounded_ddx_body(&item.statement, counter, has_ddx)
                    })
                    && case
                        .default
                        .as_deref()
                        .is_none_or(|statement| self.bounded_ddx_body(statement, counter, has_ddx))
            }
            AnalogStatement::Null(_) => true,
            AnalogStatement::Contribution(_)
            | AnalogStatement::IndirectContribution(_)
            | AnalogStatement::For(_)
            | AnalogStatement::While(_)
            | AnalogStatement::Repeat(_)
            | AnalogStatement::EventControl(_)
            | AnalogStatement::Call(_)
            | AnalogStatement::Disable(_) => false,
        }
    }

    fn bounded_ddx_expression(&self, expression: &Expression, has_ddx: &mut bool) -> bool {
        let mut safe = true;
        flow_probes::visit_expression(expression, &mut |expression| match expression {
            Expression::Call(call) => {
                *has_ddx |= call.name == "ddx";
                safe &= !self.user_functions.contains_key(&call.name);
                safe &= call.name == "ddx"
                    || !self
                        .functions
                        .get(&call.name)
                        .is_some_and(|function| function.is_analog_operator);
            }
            Expression::SystemFunction(_) => safe = false,
            _ => {}
        });
        safe
    }

    /// Keep real assignments and guards rather than substituting the counter:
    /// every allowed parameter value executes its own prefix and publishes its
    /// actual exit counter, including the zero-iteration case.
    pub(super) fn unroll_guarded_for(
        &mut self,
        loop_: &ForStmt,
        iterations: usize,
        module: &mut AnalyzedModule,
        sink: &mut Vec<AnalyzedStatement>,
    ) -> CompileResult<()> {
        self.analyze_assignment(
            &AssignmentStmt {
                target: LValue::Variable {
                    name: loop_.var.clone(),
                    span: loop_.span,
                },
                value: loop_.init.clone(),
                span: loop_.span,
            },
            module,
            sink,
        )?;
        let guarded = AnalogStatement::Conditional(ConditionalStmt {
            condition: loop_.condition.clone(),
            then_branch: Box::new(AnalogStatement::Block(BlockStmt {
                name: None,
                statements: vec![
                    (*loop_.body).clone(),
                    AnalogStatement::Assignment((*loop_.update).clone()),
                ],
                variables: Vec::new(),
                span: loop_.span,
            })),
            else_branch: None,
            span: loop_.span,
        });
        for _ in 0..iterations {
            self.analyze_statement(&guarded, module, sink)?;
        }
        Ok(())
    }
}
