//! Shared iterative traversal of expression syntax.
use super::*;

pub(crate) fn visit_expression(expression: &Expression, visit: &mut impl FnMut(&Expression)) {
    enum Pending<'a> {
        Expression(&'a Expression),
        Element(&'a ArrayLiteralElement),
    }
    let mut pending = vec![Pending::Expression(expression)];
    while let Some(next) = pending.pop() {
        let expression = match next {
            Pending::Expression(expression) => expression,
            Pending::Element(ArrayLiteralElement::Value(expression)) => expression,
            Pending::Element(ArrayLiteralElement::Replication(replication)) => {
                pending.extend(replication.elements.iter().map(Pending::Element));
                &replication.count
            }
        };
        visit(expression);
        match expression {
            Expression::Binary(expr) => {
                pending.push(Pending::Expression(&expr.left));
                pending.push(Pending::Expression(&expr.right));
            }
            Expression::Unary(expr) => pending.push(Pending::Expression(&expr.operand)),
            Expression::Conditional(expr) => {
                pending.push(Pending::Expression(&expr.condition));
                pending.push(Pending::Expression(&expr.then_expr));
                pending.push(Pending::Expression(&expr.else_expr));
            }
            Expression::Call(expr) => pending.extend(expr.args.iter().map(Pending::Expression)),
            Expression::SystemFunction(expr) => {
                pending.extend(expr.args.iter().map(Pending::Expression))
            }
            Expression::ArrayAccess(expr) => {
                pending.extend(expr.children().map(Pending::Expression))
            }
            Expression::ArrayLiteral(expr) => {
                pending.extend(expr.elements.iter().map(Pending::Element))
            }
            Expression::AnalogOperator(AnalogOperator::Limit {
                proposed,
                candidate,
                type_metadata,
                ..
            }) => {
                pending.push(Pending::Expression(proposed));
                pending.push(Pending::Expression(candidate));
                if let Some(value) = type_metadata {
                    pending.push(Pending::Expression(value));
                }
            }
            Expression::NoiseSource(NoiseSource::White { power, .. }) => {
                pending.push(Pending::Expression(power))
            }
            Expression::NoiseSource(NoiseSource::Flicker {
                power, exponent, ..
            }) => {
                pending.push(Pending::Expression(power));
                pending.push(Pending::Expression(exponent));
            }
            Expression::NoiseSource(NoiseSource::Table { data, .. }) => {
                pending.extend(data.iter().map(Pending::Expression))
            }
            Expression::Digital(expr) => {
                pending.extend(expr.children().into_iter().map(Pending::Expression))
            }
            Expression::BranchAccess(BranchAccess::Nodes {
                pos_indices,
                neg_indices,
                ..
            }) => {
                pending.extend(
                    pos_indices
                        .iter()
                        .chain(neg_indices)
                        .map(Pending::Expression),
                );
            }
            Expression::BranchAccess(BranchAccess::Branch { index, .. }) => {
                pending.extend(index.iter().map(|value| Pending::Expression(value)));
            }
            Expression::Number(_)
            | Expression::StringLit(_)
            | Expression::Identifier(_)
            | Expression::NullArgument(_)
            | Expression::AnalogOperator(AnalogOperator::LimiterArgument { .. }) => {}
        }
    }
}
