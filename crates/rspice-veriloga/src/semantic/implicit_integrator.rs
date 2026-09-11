//! Solver equations for `idt(input)` with a feedback-determined integration constant.

use super::*;

pub(super) struct ImplicitIntegrator {
    node: SmolStr,
    input: usize,
    guard: Option<Expression>,
    span: Span,
}

impl SemanticAnalyzer {
    pub(super) fn materialize_implicit_integrator(
        &mut self,
        call: &CallExpr,
        module: &mut AnalyzedModule,
        sink: &mut Vec<AnalyzedStatement>,
    ) -> CompileResult<Expression> {
        // Check the authored operator before replacing it with ordinary node
        // accesses. In particular, this must not legalize dynamic guards.
        self.validate_stateful_analog_operator_placement("idt", call.span)?;
        let (node, input_name) = loop {
            self.local_counter += 1;
            let names = ["out", "input"]
                .map(|role| SmolStr::from(format!("__idt_{role}{}", self.local_counter)));
            if names.iter().all(|name| {
                self.symbols.lookup(name).is_none()
                    && !module
                        .variables
                        .iter()
                        .any(|variable| variable.name == *name)
                    && !module.internal_nodes.iter().any(|node| node.name == *name)
                    && !self.user_functions.contains_key(name)
            }) {
                let [node, input] = names;
                break (node, input);
            }
        };
        let index = module.internal_nodes.len();
        self.define_symbol(Symbol {
            name: node.clone(),
            kind: SymbolKind::Node,
            value_type: ValueType::NatureAccess,
            span: call.span,
            attrs: SymbolAttrs {
                discipline: Some("electrical".into()),
                is_internal: true,
                internal_node_index: Some(index),
                ..Default::default()
            },
        })?;
        module.internal_nodes.push(AnalyzedInternalNode {
            is_state: true,
            name: node.clone(),
            discipline: "electrical".into(),
            index,
        });
        let input = module.variables.len();
        self.register_function_temp(module, input_name.clone(), VarType::Real, call.span)?;
        // Snapshot at the call site: a subsequent source assignment must not
        // change the input read by the equation appended after the analog body.
        self.analyze_assignment(
            &AssignmentStmt {
                target: LValue::Variable {
                    name: input_name,
                    span: call.span,
                },
                value: call.args[0].clone(),
                span: call.span,
            },
            module,
            sink,
        )?;
        let guard = self
            .current_guard()
            .as_ref()
            .map(|guard| expand_static_guard(guard, sink))
            .transpose()?;
        self.implicit_integrators.push(ImplicitIntegrator {
            node: node.clone(),
            input,
            guard,
            span: call.span,
        });
        Ok(integrator_output(node, call.span))
    }

    pub(super) fn finish_implicit_integrators(&mut self, module: &mut AnalyzedModule) {
        for integrator in std::mem::take(&mut self.implicit_integrators) {
            let ImplicitIntegrator {
                node,
                input,
                guard,
                span,
            } = integrator;
            let variable = |index: usize| {
                Expression::Identifier(Identifier {
                    name: module.variables[index].name.clone(),
                    span,
                })
            };
            let output = integrator_output(node.clone(), span);
            // VAMS-2023 4.5.4: DC forces the input to zero and lets feedback
            // determine the output. The same first-order equation supplies
            // transient history, AC and noise through the existing DAE path.
            let mut residual = Self::binary_expr(
                BinaryOp::Sub,
                Expression::Call(CallExpr {
                    name: "ddt".into(),
                    args: vec![output.clone()],
                    span,
                }),
                variable(input),
            );
            if let Some(guard) = guard {
                residual = Expression::Conditional(ConditionalExpr {
                    condition: Box::new(guard),
                    then_expr: Box::new(residual),
                    // A disabled instance-static branch must not leave a
                    // floating hidden unknown in the circuit matrix.
                    else_expr: Box::new(output),
                    span,
                });
            }
            let contribution = AnalyzedContribution {
                branch: node,
                declared_branch: None,
                is_current: true,
                indirect: false,
                equation_abstol: None,
                expression: residual,
                site: self.next_analog_site(),
                expression_guard: AnalogSiteGuard::None,
                expr_type: ValueType::Real,
                span,
            };
            self.record_region(AnalyzedRegion::Contribution(contribution.clone()));
            module.contributions.push(contribution);
        }
    }
}

// Flat guards refer to snapshot variables that the structured body replaces
// with its branch conditions. Only instance-static guards reach this lowering,
// so their original expressions can also guard the final solver equation.
// Resolve definitions against preceding statements to keep this walk acyclic.
fn expand_static_guard(
    expression: &Expression,
    statements: &[AnalyzedStatement],
) -> CompileResult<Expression> {
    if matches!(expression, Expression::Binary(_) | Expression::Unary(_)) {
        return rewrite_operator_tree(expression, |node| match node {
            OperatorRewrite::Leaf(expression) => expand_static_guard(expression, statements),
            OperatorRewrite::Binary(binary) => Ok(Expression::Binary(binary)),
            OperatorRewrite::Unary(unary) => Ok(Expression::Unary(unary)),
        });
    }
    if let Expression::Identifier(identifier) = expression
        && let Some(index) = statements.iter().rposition(|statement| {
            matches!(statement,
            AnalyzedStatement::Assignment(assignment) if assignment.target == identifier.name)
        })
        && let AnalyzedStatement::Assignment(assignment) = &statements[index]
    {
        return expand_static_guard(&assignment.expression, &statements[..index]);
    }
    Ok(expression.clone())
}

fn integrator_output(node: SmolStr, span: Span) -> Expression {
    Expression::BranchAccess(BranchAccess::Nodes {
        access: "V".into(),
        kind: Some(AccessKind::Potential),
        pos: node,
        neg: None,
        span,
    })
}
