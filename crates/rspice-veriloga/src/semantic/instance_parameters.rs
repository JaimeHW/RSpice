//! Shared assignment of instance overrides in their declaring scope.
use super::*;

pub(super) fn constants(source: &Module) -> DigitalConstants {
    DigitalConstants::from_module(source)
}

/// Concrete reference discovery must retain the same array replacement rule as
/// ordinary hierarchy elaboration (VAMS-2023 3.4.4). Re-expanding a replicated
/// default at the new width is not an explicit array assignment.
pub(super) fn validate_array_replacements(
    declared: &Module,
    effective: &Module,
    provided: &[(usize, String)],
    path: &str,
) -> CompileResult<()> {
    if provided.is_empty() {
        return Ok(());
    }
    let before = constants(declared);
    let after = constants(effective);
    for (index, parameter) in declared.parameters.iter().enumerate() {
        if parameter.dimensions.is_empty() || provided.iter().any(|(given, _)| *given == index) {
            continue;
        }
        let extent = |dimension: &ArrayDimension, constants: &DigitalConstants| {
            let bound = |expression: &Expression| {
                crate::canonical_ir::digital_lower::elaboration_constant(
                    expression,
                    constants,
                    declared.time_scale,
                )
                .and_then(|value| match value {
                    crate::numeric_literal::NumericLiteralValue::Integer(value) => Some(value),
                    crate::numeric_literal::NumericLiteralValue::Real(value) => {
                        SemanticAnalyzer::exact_const_i64(value)
                    }
                })
            };
            bound(&dimension.start)
                .zip(bound(&dimension.end))
                .and_then(|(left, right)| left.abs_diff(right).checked_add(1))
        };
        let mut changed = false;
        for dimension in &parameter.dimensions {
            let (Some(before), Some(after)) = (extent(dimension, &before), extent(dimension, &after))
            else {
                return Err(SemanticError::new(
                    SemanticErrorKind::InvalidExpression(format!(
                        "array '{}' at instance '{path}' requires finite integer bounds",
                        parameter.name
                    )),
                    dimension.span,
                )
                .into());
            };
            changed |= before != after;
        }
        if changed {
            return Err(SemanticError::new(
                SemanticErrorKind::InvalidExpression(format!(
                    "instance '{path}' changes the size of parameter array '{}' without an explicit replacement array value (VAMS-2023 3.4.4)",
                    parameter.name
                )),
                parameter.span,
            )
            .into());
        }
    }
    Ok(())
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

/// One ordered/named override contract for source discovery and body elaboration.
pub(super) fn bind_overrides(
    instance: &ModuleInstance,
    names: &[SmolStr],
    aliases: &HashMap<SmolStr, usize>,
    path: &str,
) -> CompileResult<HashMap<usize, Expression>> {
    let has_named = instance.parameters.iter().any(|value| value.name.is_some());
    let has_ordered = instance.parameters.iter().any(|value| value.name.is_none());
    if has_named && has_ordered {
        return Err(SemanticError::new(
            SemanticErrorKind::UnsupportedFeature(format!(
                "instance '{path}' mixes named and ordered parameter overrides"
            )),
            instance.span,
        )
        .into());
    }
    if has_ordered && instance.parameters.len() > names.len() {
        return Err(SemanticError::new(
            SemanticErrorKind::ArgumentCountMismatch {
                name: path.into(),
                expected: format!("at most {} parameter overrides", names.len()),
                got: instance.parameters.len(),
            },
            instance.span,
        )
        .into());
    }
    let mut overrides = HashMap::new();
    for (ordered, parameter) in instance.parameters.iter().enumerate() {
        let index = match &parameter.name {
            Some(name) => names
                .iter()
                .position(|candidate| candidate == name)
                .or_else(|| aliases.get(name).copied())
                .ok_or_else(|| {
                    SemanticError::new(
                        SemanticErrorKind::UndeclaredSymbol { name: name.clone() },
                        parameter.span,
                    )
                })?,
            None => ordered,
        };
        if overrides.insert(index, parameter.value.clone()).is_some() {
            return Err(SemanticError::new(
                SemanticErrorKind::DuplicateSymbol {
                    name: names[index].clone(),
                    first_defined: parameter.span,
                },
                parameter.span,
            )
            .into());
        }
    }
    Ok(overrides)
}
