//! Shared assignment of instance overrides in their declaring scope.
use super::*;

pub(super) fn constants(source: &Module) -> DigitalConstants {
    DigitalConstants::from_module(source)
}

impl SemanticAnalyzer {
    /// External scalar specialization obeys the same declaration-size contract
    /// as source instances, including aliases and $param_given dependencies.
    pub(crate) fn validate_root_array_replacements(effective: &Module) -> CompileResult<()> {
        let Some(parameters) = &effective.unspecialized_parameters else {
            return Ok(());
        };
        if !parameters
            .iter()
            .any(|parameter| !parameter.dimensions.is_empty())
        {
            return Ok(());
        }
        let mut declared = effective.clone();
        declared.parameters = parameters.clone();
        let provided: Vec<_> = effective
            .parameters
            .iter()
            .enumerate()
            .filter_map(|(index, parameter)| parameter.is_given.then_some(index))
            .collect();
        validate_array_replacements(&declared, effective, &provided, &effective.name)
    }
}

/// Concrete reference discovery must retain the same array replacement rule as
/// ordinary hierarchy elaboration (VAMS-2023 3.4.4). Re-expanding a replicated
/// default at the new width is not an explicit array assignment.
pub(super) fn validate_array_replacements(
    declared: &Module,
    effective: &Module,
    provided: &[usize],
    path: &str,
) -> CompileResult<()> {
    if provided.is_empty() {
        return Ok(());
    }
    let before = constants(declared);
    let after = constants(effective);
    for (index, parameter) in declared.parameters.iter().enumerate() {
        if parameter.dimensions.is_empty() || provided.contains(&index) {
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
            let (Some(before), Some(after)) =
                (extent(dimension, &before), extent(dimension, &after))
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

/// The author's error in an override that failed to close, when there is one.
///
/// IEEE 1364-2005 section 12.2 makes a parameter value a constant expression
/// in the scope that assigns it, so an operand that scope never declares is
/// undeclared, and one it declares as a net or variable is not constant. Both
/// read as a missing lowering if the closing failure is reported as is.
///
/// Only a failure is reclassified, never a success. A scope with generate
/// constructs declares names this check does not see, so it is left alone.
pub(super) fn override_operand_error(
    expression: &Expression,
    parent: &Module,
) -> Option<CompileError> {
    if !parent.generates.is_empty() || parent.generate_template.is_some() {
        return None;
    }
    let constant = |name: &str| {
        parent
            .parameters
            .iter()
            .chain(&parent.localparams)
            .any(|parameter| parameter.name == name)
            || parent.aliasparams.iter().any(|alias| alias.alias == name)
            || parent
                .genvars
                .iter()
                .any(|genvar| genvar.names.iter().any(|declared| declared == name))
    };
    let other = |name: &str| {
        parent.ports.iter().any(|port| port.name == name)
            || parent
                .port_declarations
                .iter()
                .any(|declaration| declaration.names.iter().any(|declared| declared == name))
            || parent
                .variables
                .iter()
                .any(|declaration| declaration.items.iter().any(|item| item.name == name))
            || parent
                .nets
                .iter()
                .any(|declaration| declaration.names.iter().any(|declared| declared == name))
            || parent.branches.iter().any(|branch| branch.name == name)
            || parent
                .digital_nets
                .iter()
                .flat_map(|declaration| &declaration.items)
                .chain(
                    parent
                        .digital_variables
                        .iter()
                        .flat_map(|declaration| &declaration.items),
                )
                .any(|item| item.name == name)
            || parent
                .instances
                .iter()
                .any(|instance| instance.name == name)
            || parent
                .functions
                .iter()
                .any(|function| function.name == name)
            || parent
                .digital_routines
                .iter()
                .any(|routine| routine.name == name)
    };
    let mut found = None;
    crate::ast::visit_expression(expression, &mut |operand| {
        if found.is_some() {
            return;
        }
        let Expression::Identifier(identifier) = operand else {
            return;
        };
        let name = identifier.name.as_str();
        if name.contains('.') || constant(name) {
            return;
        }
        let kind = if other(name) {
            SemanticErrorKind::InvalidExpression(format!(
                "`{name}` is not a constant; IEEE 1364-2005 section 12.2 makes a parameter \
                 value a constant expression, which a net or variable is not"
            ))
        } else {
            SemanticErrorKind::UndeclaredSymbol {
                name: identifier.name.clone(),
            }
        };
        found = Some(SemanticError::new(kind, identifier.span).into());
    });
    found
}

pub(super) fn close_override(
    declaration: &ParameterDecl,
    expression: Expression,
    parent: &DigitalConstants,
    time_scale: crate::time_scale::ModuleTimeScale,
) -> Result<Expression, String> {
    if declaration.param_type == ParamType::String && declaration.dimensions.is_empty() {
        let span = expression.span();
        return super::string_parameters::close(&expression, &parent.definitions)
            .map(|value| Expression::StringLit(StringLit { value, span }));
    }
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
            // Debug-escaped and quoted, so no value can spell another key.
            Expression::StringLit(string) => {
                write!(key, "str:{:?};", string.value.as_str()).unwrap();
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
