//! The operands IEEE 1364-2005 lets a concatenation take: no unsized constant
//! number (section 5.1.14) and no real value (section 5.1.1).

use super::{SemanticAnalyzer, SemanticErrorKind};
use crate::ast::*;
use smol_str::SmolStr;

/// Why an operand cannot stand in a concatenation.
enum Refused {
    Unsized,
    Real,
}

impl SemanticAnalyzer {
    /// Check the operands of one concatenation `{...}`, its replications'
    /// included. `is_real` says whether a name is a real variable or
    /// parameter. A nested concatenation is checked where the caller's walk
    /// reaches it.
    ///
    /// Section 5.1.14: "Unsized constant numbers shall not be allowed in
    /// concatenations", because each operand's size is part of the
    /// concatenation's. Section 5.1.1 lists the concatenation and replication
    /// operators among those a real operand does not take. A replication's
    /// count is a constant expression, not an operand, and may be unsized.
    pub(super) fn check_concatenation_operands(
        &mut self,
        literal: &ArrayLiteralExpr,
        is_real: &dyn Fn(&SmolStr) -> bool,
    ) {
        if literal.assignment_pattern {
            return;
        }
        // In source order: the stack holds the elements still to visit last
        // first.
        let mut pending: Vec<&ArrayLiteralElement> = literal.elements.iter().rev().collect();
        while let Some(element) = pending.pop() {
            let value = match element {
                ArrayLiteralElement::Value(value) => value,
                ArrayLiteralElement::Replication(replication) => {
                    pending.extend(replication.elements.iter().rev());
                    continue;
                }
            };
            let refused = match value {
                Expression::Number(number) => match crate::four_state::decode(&number.raw) {
                    Ok(literal) => literal.declared_width.is_none().then_some(Refused::Unsized),
                    Err(_) => match crate::numeric_literal::parse_integer_literal(&number.raw) {
                        Ok(Some(_)) => Some(Refused::Unsized),
                        Ok(None) => Some(Refused::Real),
                        Err(_) => None,
                    },
                },
                Expression::Digital(DigitalExpr::FourState(literal)) => literal
                    .value
                    .declared_width
                    .is_none()
                    .then_some(Refused::Unsized),
                Expression::Identifier(identifier) if is_real(&identifier.name) => {
                    Some(Refused::Real)
                }
                _ => None,
            };
            let message = match refused {
                None => continue,
                Some(Refused::Unsized) => format!(
                    "the unsized constant `{}` is an operand of a concatenation; IEEE 1364-2005 \
                     section 5.1.14 allows no unsized constant number in one",
                    Self::operand_text(value)
                ),
                Some(Refused::Real) => format!(
                    "the real value `{}` is an operand of a concatenation; IEEE 1364-2005 \
                     section 5.1.1 gives no real operand to a concatenation or a replication",
                    Self::operand_text(value)
                ),
            };
            self.record_error_at(SemanticErrorKind::InvalidExpression(message), value.span());
        }
    }

    fn operand_text(value: &Expression) -> SmolStr {
        match value {
            Expression::Number(number) => number.raw.clone(),
            Expression::Digital(DigitalExpr::FourState(literal)) => literal.value.raw.clone(),
            Expression::Identifier(identifier) => identifier.name.clone(),
            _ => SmolStr::default(),
        }
    }

    /// The concatenations in the value of each scalar parameter. A parameter
    /// array's value is an array literal of elements, not a concatenation.
    pub(super) fn check_parameter_concatenations(&mut self, parameters: &[&ParameterDecl]) {
        let real: std::collections::HashSet<SmolStr> = parameters
            .iter()
            .filter(|parameter| {
                parameter.type_is_explicit && parameter.param_type == ParamType::Real
            })
            .map(|parameter| parameter.name.clone())
            .collect();
        for parameter in parameters
            .iter()
            .filter(|parameter| parameter.dimensions.is_empty())
        {
            let Some(default) = &parameter.default else {
                continue;
            };
            let mut literals = Vec::new();
            crate::ast::visit_expression(default, &mut |expression| {
                if let Expression::ArrayLiteral(literal) = expression {
                    literals.push(literal.clone());
                }
            });
            for literal in &literals {
                self.check_concatenation_operands(literal, &|name| real.contains(name));
            }
        }
    }
}
