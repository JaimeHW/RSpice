//! String parameters: values fixed when a module is elaborated.
//!
//! A `string` parameter has no run-time slot. Its value is closed when the
//! module is elaborated, from its declaration, an instance override or a
//! source specialization, and every comparison that reads it is folded to a
//! constant before analysis, so neither executable lowering sees one. Any
//! other read keeps the diagnostic it had.
use super::*;

/// The value a string parameter takes from `expression`: a string literal,
/// or the name of another string parameter among `definitions`.
pub(super) fn close(
    expression: &Expression,
    definitions: &[ParameterDecl],
) -> Result<SmolStr, String> {
    let mut expression = expression;
    // Each step follows one declaration, so a longer chain is a cycle.
    for _ in 0..=definitions.len() {
        match expression {
            Expression::StringLit(literal) => return Ok(literal.value.clone()),
            Expression::Identifier(identifier) => {
                let Some(declaration) = definitions.iter().rev().find(|declaration| {
                    declaration.name == identifier.name
                        && declaration.param_type == ParamType::String
                        && declaration.dimensions.is_empty()
                }) else {
                    break;
                };
                let Some(default) = &declaration.default else {
                    break;
                };
                expression = default;
            }
            _ => break,
        }
    }
    Err("a string parameter takes a string literal or the name of another string parameter".into())
}

/// Every scalar string parameter and localparam of `module`, with its value.
pub(super) fn closed_values(module: &Module) -> CompileResult<HashMap<SmolStr, SmolStr>> {
    let definitions: Vec<_> = module
        .parameters
        .iter()
        .chain(&module.localparams)
        .cloned()
        .collect();
    let mut values = HashMap::new();
    for declaration in &definitions {
        if declaration.param_type != ParamType::String || !declaration.dimensions.is_empty() {
            continue;
        }
        let Some(default) = &declaration.default else {
            continue;
        };
        let value = close(default, &definitions).map_err(|message| -> CompileError {
            SemanticError::new(
                SemanticErrorKind::UnsupportedFeature(format!(
                    "string parameter '{}': {message}",
                    declaration.name
                )),
                default.span(),
            )
            .into()
        })?;
        values.insert(declaration.name.clone(), value);
    }
    Ok(values)
}

/// Refuse a string parameter whose closed value lies outside its `from
/// '{...}` set or inside an `exclude '{...}` set or `exclude "x"` (LRM 2.4
/// section 3.4.2). The value is closed per instance, so this is the instance
/// check.
fn validate_value_sets(module: &Module, values: &HashMap<SmolStr, SmolStr>) -> CompileResult<()> {
    let definitions: Vec<_> = module
        .parameters
        .iter()
        .chain(&module.localparams)
        .cloned()
        .collect();
    let listed = |strings: &[SmolStr]| {
        let quoted: Vec<_> = strings.iter().map(|value| format!("\"{value}\"")).collect();
        format!("{{{}}}", quoted.join(", "))
    };
    for declaration in &module.parameters {
        if declaration.param_type != ParamType::String {
            continue;
        }
        let Some(range) = &declaration.range else {
            continue;
        };
        let refuse = |value: &SmolStr, reason: String, span: Span| -> CompileError {
            SemanticError::new(
                SemanticErrorKind::StringParameterValue {
                    name: declaration.name.clone(),
                    detail: format!("value \"{value}\" {reason}"),
                },
                span,
            )
            .into()
        };
        if let Some(bound) = range.bounds.first() {
            return Err(SemanticError::new(
                SemanticErrorKind::UnsupportedFeature(format!(
                    "string parameter '{}' takes a value set such as `from '{{\"a\", \"b\"}}`, not a numeric interval",
                    declaration.name
                )),
                bound.span,
            )
            .into());
        }
        let Some(value) = values.get(&declaration.name) else {
            continue;
        };
        for set in &range.string_sets {
            let member = set.values.contains(value);
            if set.excluded && member {
                return Err(refuse(
                    value,
                    format!("is excluded by {}", listed(&set.values)),
                    set.span,
                ));
            }
            if !set.excluded && !member {
                return Err(refuse(
                    value,
                    format!("is not one of its allowed values {}", listed(&set.values)),
                    set.span,
                ));
            }
        }
        for excluded in &range.exclude {
            let excluded_value =
                close(excluded, &definitions).map_err(|message| -> CompileError {
                    SemanticError::new(
                        SemanticErrorKind::UnsupportedFeature(format!(
                            "excluded value of string parameter '{}': {message}",
                            declaration.name
                        )),
                        excluded.span(),
                    )
                    .into()
                })?;
            if excluded_value == *value {
                return Err(refuse(
                    value,
                    format!("is excluded by {}", listed(&[excluded_value])),
                    excluded.span(),
                ));
            }
        }
    }
    Ok(())
}

/// `module` with each string declaration given its closed value, every
/// `$simparam$str("module")` given the module's name, and every comparison of
/// two strings folded to the integer it denotes.
pub(super) fn substituted(module: &Module) -> CompileResult<Option<Module>> {
    let values = closed_values(module)?;
    validate_value_sets(module, &values)?;
    if values.is_empty() && !module.reserved_identifiers.contains("$simparam$str") {
        return Ok(None);
    }
    let mut module = module.clone();
    for declaration in module.parameters.iter_mut().chain(&mut module.localparams) {
        if declaration.param_type == ParamType::String
            && let Some(value) = values.get(&declaration.name)
            && let Some(default) = &mut declaration.default
        {
            let span = default.span();
            *default = Expression::StringLit(StringLit {
                value: value.clone(),
                span,
            });
        }
    }
    crate::parser::substitute_string_parameters(&mut module, values);
    Ok(Some(module))
}
