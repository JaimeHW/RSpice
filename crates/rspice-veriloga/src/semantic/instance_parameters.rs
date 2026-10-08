//! Shared assignment of instance overrides in their declaring scope.
use super::*;

pub(super) fn constants(source: &Module) -> DigitalConstants {
    DigitalConstants::from_module(source)
}

pub(super) fn close_override(
    declaration: &ParameterDecl,
    expression: Expression,
    parent: &DigitalConstants,
    time_scale: crate::time_scale::ModuleTimeScale,
) -> Result<Expression, String> {
    let mut declaration = declaration.clone();
    declaration.default = Some(expression);
    if declaration.packed_range.is_some() || declaration.signedness.is_some() {
        // Resolve the RHS in its parent. Width/sign assignment belongs to the
        // child's complete effective scope, after all overrides are installed.
        declaration.packed_range = None;
        declaration.signedness = None;
        declaration.type_is_explicit = false;
        declaration.param_type = ParamType::Real;
    }
    crate::canonical_ir::digital_lower::parameter_override_literal(&declaration, parent, time_scale)
}

/// A closed override's value and topology identify source specialization.
/// Both hierarchy passes use this representation, independent of source spans.
pub(super) fn override_identity(expression: &Expression) -> Result<String, String> {
    use std::fmt::Write;
    fn expression_key(expression: &Expression, key: &mut String) -> Result<(), String> {
        match expression {
            Expression::Number(number) => {
                write!(key, "real:{:016x};", number.value.to_bits()).unwrap();
            }
            Expression::Digital(DigitalExpr::FourState(literal)) => {
                write!(
                    key,
                    "bits:{}:{};",
                    literal.value.raw.len(),
                    literal.value.raw
                )
                .unwrap();
            }
            Expression::ArrayLiteral(pattern) if pattern.assignment_pattern => {
                key.push('[');
                elements_key(&pattern.elements, key)?;
                key.push(']');
            }
            _ => return Err("closed hierarchy parameter has no constant identity".into()),
        }
        Ok(())
    }
    fn elements_key(elements: &[ArrayLiteralElement], key: &mut String) -> Result<(), String> {
        for element in elements {
            match element {
                ArrayLiteralElement::Value(value) => expression_key(value, key)?,
                ArrayLiteralElement::Replication(replication) => {
                    let Expression::Number(count) = replication.count.as_ref() else {
                        return Err("closed array replication has no integer count".into());
                    };
                    // The integer spelling remains exact above 2^53.
                    write!(key, "repeat:{}[", count.raw).unwrap();
                    elements_key(&replication.elements, key)?;
                    key.push(']');
                }
            }
        }
        Ok(())
    }
    let mut key = String::new();
    expression_key(expression, &mut key)?;
    Ok(key)
}
