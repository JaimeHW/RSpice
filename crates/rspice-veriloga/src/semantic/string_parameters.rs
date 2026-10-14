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

/// `module` with each string declaration given its closed value and every
/// comparison of two strings folded to the integer it denotes.
pub(super) fn substituted(module: &Module) -> CompileResult<Option<Module>> {
    let values = closed_values(module)?;
    if values.is_empty() {
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
