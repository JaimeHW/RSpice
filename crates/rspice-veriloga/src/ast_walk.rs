//! Shared iterative traversal of expression syntax.
use super::*;

pub(crate) fn visit_expression(expression: &Expression, visit: &mut impl FnMut(&Expression)) {
    visit_expression_at_depth(expression, &mut |expression, _| {
        visit(expression);
        true
    });
}

/// [`visit_expression`], also passing each node's depth in the tree (the root
/// is 1). A node for which `visit` returns `false` is not descended into.
pub(crate) fn visit_expression_at_depth(
    expression: &Expression,
    visit: &mut impl FnMut(&Expression, usize) -> bool,
) {
    enum Pending<'a> {
        Expression(&'a Expression),
        Element(&'a ArrayLiteralElement),
    }
    fn below(expression: &Expression, depth: usize) -> (Pending<'_>, usize) {
        (Pending::Expression(expression), depth + 1)
    }
    let mut pending = vec![(Pending::Expression(expression), 1)];
    while let Some((next, depth)) = pending.pop() {
        let expression = match next {
            Pending::Expression(expression) => expression,
            Pending::Element(ArrayLiteralElement::Value(expression)) => expression,
            Pending::Element(ArrayLiteralElement::Replication(replication)) => {
                pending.extend(
                    replication
                        .elements
                        .iter()
                        .map(|element| (Pending::Element(element), depth + 1)),
                );
                &replication.count
            }
        };
        if !visit(expression, depth) {
            continue;
        }
        let child = |expression| below(expression, depth);
        match expression {
            Expression::Binary(expr) => {
                pending.push(below(&expr.left, depth));
                pending.push(below(&expr.right, depth));
            }
            Expression::Unary(expr) => pending.push(below(&expr.operand, depth)),
            Expression::Conditional(expr) => {
                pending.push(below(&expr.condition, depth));
                pending.push(below(&expr.then_expr, depth));
                pending.push(below(&expr.else_expr, depth));
            }
            Expression::Call(expr) => pending.extend(expr.args.iter().map(child)),
            Expression::SystemFunction(expr) => pending.extend(expr.args.iter().map(child)),
            Expression::ArrayAccess(expr) => pending.extend(expr.children().map(child)),
            Expression::ArrayLiteral(expr) => pending.extend(
                expr.elements
                    .iter()
                    .map(|element| (Pending::Element(element), depth + 1)),
            ),
            Expression::AnalogOperator(AnalogOperator::Limit {
                proposed,
                candidate,
                type_metadata,
                ..
            }) => {
                pending.push(below(proposed, depth));
                pending.push(below(candidate, depth));
                if let Some(value) = type_metadata {
                    pending.push(below(value, depth));
                }
            }
            Expression::NoiseSource(NoiseSource::White { power, .. }) => {
                pending.push(below(power, depth))
            }
            Expression::NoiseSource(NoiseSource::Flicker {
                power, exponent, ..
            }) => {
                pending.push(below(power, depth));
                pending.push(below(exponent, depth));
            }
            Expression::NoiseSource(NoiseSource::Table { data, .. }) => {
                pending.extend(data.iter().map(child))
            }
            Expression::Digital(expr) => pending.extend(expr.children().into_iter().map(child)),
            Expression::BranchAccess(BranchAccess::Nodes {
                pos_indices,
                neg_indices,
                ..
            }) => {
                pending.extend(pos_indices.iter().chain(neg_indices).map(child));
            }
            Expression::BranchAccess(BranchAccess::Branch { index, .. }) => {
                pending.extend(index.iter().map(|value| below(value, depth)));
            }
            Expression::Number(_)
            | Expression::StringLit(_)
            | Expression::Identifier(_)
            | Expression::NullArgument(_)
            | Expression::AnalogOperator(AnalogOperator::LimiterArgument { .. }) => {}
        }
    }
}
