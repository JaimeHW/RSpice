//! Preserve packed constant calculations at a real conversion boundary.
//!
//! Numeric parameter references remain symbolic. Immutable exact parameters
//! can close a default or a packed operand at a known real conversion boundary.

use super::*;

/// Only values that have no numeric parameter slot may participate here.
/// Numeric parameters stay symbolic even if their current defaults are known.
#[derive(Default)]
pub(super) struct ExactParameterConstants {
    names: HashSet<SmolStr>,
    source: DigitalConstants,
}

impl ExactParameterConstants {
    pub(super) fn source(&self) -> &DigitalConstants {
        &self.source
    }

    pub(super) fn retain(&mut self, declaration: &ParameterDecl, default: Option<&Expression>) {
        let Some(value @ Expression::Digital(DigitalExpr::FourState(_))) = default else {
            return;
        };
        if !declaration.dimensions.is_empty() || declaration.param_type == ParamType::String {
            return;
        }
        let mut declaration = declaration.clone();
        declaration.default = Some(value.clone());
        self.names.insert(declaration.name.clone());
        self.source.definitions.push(declaration);
    }
}

enum RealShape {
    Known(bool),
    FromOperands(Vec<*const Expression>),
}

impl SemanticAnalyzer {
    /// A default may close over immutable elaboration parameters, but must not
    /// capture a numeric parameter whose runtime value can still be changed.
    pub(super) fn parameter_operand_is_closed(&self, expression: &Expression) -> bool {
        let mut closed = true;
        flow_probes::visit_expression(expression, &mut |value| {
            let name = match value {
                Expression::Identifier(value) => Some(&value.name),
                Expression::ArrayAccess(value) => Some(&value.array),
                Expression::Digital(value) => value.base_name(),
                _ => None,
            };
            if let Some(name) = name {
                closed &= self.exact_parameter_constants.names.contains(name);
            }
        });
        closed
    }

    pub(super) fn fold_real_parameter_operands(
        &self,
        parameter: &ParameterDecl,
        expression: &Expression,
        module: &Module,
    ) -> Option<Expression> {
        let mut packed = false;
        flow_probes::visit_expression(expression, &mut |value| {
            packed |= matches!(value, Expression::Identifier(value)
                if self.exact_parameter_constants.names.contains(&value.name))
                || matches!(value, Expression::Digital(_) | Expression::ArrayLiteral(_))
                || matches!(value, Expression::Number(value) if value.raw.contains('\''));
        });
        if !packed {
            return None;
        }
        // Untyped parameters still require general final-value type inference.
        // Their historical scalar Real placeholder is not evidence of a real
        // conversion: treating it as one would erase an integer assignment's
        // wider context. Use only types the declaration actually guarantees.
        let real_names: HashSet<_> = module
            .parameters
            .iter()
            .chain(&module.localparams)
            .filter(|value| {
                value.type_is_explicit
                    && value.param_type == ParamType::Real
                    && value.dimensions.is_empty()
            })
            .map(|value| &value.name)
            .collect();
        let mut rewritten = expression.clone();
        let mut descriptions = Vec::new();
        let mut pending = vec![&mut rewritten];
        while let Some(value) = pending.pop() {
            let shape = match value {
                Expression::Identifier(value) => RealShape::Known(real_names.contains(&value.name)),
                Expression::Number(value) => RealShape::Known(
                    !value.raw.contains('\'')
                        && parse_integer_literal(&value.raw).ok().flatten().is_none(),
                ),
                Expression::Binary(value)
                    if matches!(
                        value.op,
                        BinaryOp::Add
                            | BinaryOp::Sub
                            | BinaryOp::Mul
                            | BinaryOp::Div
                            | BinaryOp::Mod
                            | BinaryOp::Pow
                    ) =>
                {
                    RealShape::FromOperands(vec![value.left.as_ref(), value.right.as_ref()])
                }
                Expression::Unary(value) if matches!(value.op, UnaryOp::Pos | UnaryOp::Neg) => {
                    RealShape::FromOperands(vec![value.operand.as_ref()])
                }
                Expression::Conditional(value) => RealShape::FromOperands(vec![
                    value.then_expr.as_ref(),
                    value.else_expr.as_ref(),
                ]),
                // Calls and implicit parameter types need their own declared
                // conversion rules; never infer realness from a numeric cache.
                _ => RealShape::Known(false),
            };
            descriptions.push((value as *const Expression, shape));
            flow_probes::for_child_mut(value, &mut |child| pending.push(child));
        }
        let mut real = HashMap::new();
        for (key, shape) in descriptions.into_iter().rev() {
            let value = match shape {
                RealShape::Known(value) => value,
                RealShape::FromOperands(operands) => operands.iter().any(|operand| real[operand]),
            };
            real.insert(key, value);
        }
        let is_real = |value: &Expression| real[&(value as *const Expression)];
        let mut changed = false;
        let mut pending = vec![&mut rewritten];
        while let Some(value) = pending.pop() {
            match value {
                Expression::Binary(binary)
                    if matches!(
                        binary.op,
                        BinaryOp::Add
                            | BinaryOp::Sub
                            | BinaryOp::Mul
                            | BinaryOp::Div
                            | BinaryOp::Mod
                            | BinaryOp::Pow
                            | BinaryOp::Eq
                            | BinaryOp::Ne
                            | BinaryOp::Lt
                            | BinaryOp::Le
                            | BinaryOp::Gt
                            | BinaryOp::Ge
                    ) && (is_real(&binary.left) || is_real(&binary.right)) =>
                {
                    changed |= self.fold_closed_real_operand(parameter, &mut binary.left);
                    changed |= self.fold_closed_real_operand(parameter, &mut binary.right);
                }
                Expression::Conditional(conditional)
                    if is_real(&conditional.then_expr) || is_real(&conditional.else_expr) =>
                {
                    changed |= self.fold_closed_real_operand(parameter, &mut conditional.then_expr);
                    changed |= self.fold_closed_real_operand(parameter, &mut conditional.else_expr);
                }
                _ => {}
            }
            flow_probes::for_child_mut(value, &mut |child| pending.push(child));
        }
        changed.then_some(rewritten)
    }

    fn fold_closed_real_operand(
        &self,
        parameter: &ParameterDecl,
        operand: &mut Expression,
    ) -> bool {
        let closed = self.parameter_operand_is_closed(operand);
        let mut packed = false;
        flow_probes::visit_expression(operand, &mut |value| {
            packed |= matches!(value, Expression::Identifier(value)
                if self.exact_parameter_constants.names.contains(&value.name))
                || matches!(value, Expression::Digital(_) | Expression::ArrayLiteral(_))
                || matches!(value, Expression::Number(value) if value.raw.contains('\''));
        });
        if !closed || !packed {
            return false;
        }
        let mut declaration = parameter.clone();
        declaration.span = operand.span();
        declaration.param_type = ParamType::Real;
        declaration.type_is_explicit = true;
        declaration.default = Some(operand.clone());
        // Evaluate the integral operand at its own width and signedness, then
        // perform the real conversion already required by the parent operator.
        let Ok(value) = crate::canonical_ir::digital_lower::parameter_override_literal(
            &declaration,
            self.exact_parameter_constants.source(),
            self.current_time_scale,
        ) else {
            return false;
        };
        *operand = value;
        true
    }
}
