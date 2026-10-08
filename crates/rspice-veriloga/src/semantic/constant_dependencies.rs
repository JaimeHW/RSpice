//! Bind numeric slots to the constants consumed by compiled structure.
use super::*;
use crate::canonical_ir::digital_lower::ElaborationDependencies;

pub(super) fn protect(
    module: &mut AnalyzedModule,
    dependencies: &ElaborationDependencies,
    span: Span,
) -> Result<(), SemanticError> {
    for parameter in &mut module.parameters {
        if dependencies.given.contains(&parameter.name) {
            parameter.elaboration_given = Some(parameter.is_given);
        }
        if matches!(
            parameter.default_expr,
            Some(Expression::Digital(DigitalExpr::FourState(_)))
        ) {
            continue;
        }
        if let Some(value) = dependencies.values.get(&parameter.name) {
            let value = value.ok_or_else(|| {
                SemanticError::new(
                    SemanticErrorKind::UnsupportedFeature(format!(
                        "parameter '{}' requires an exact finite numeric elaboration value",
                        parameter.name
                    )),
                    span,
                )
            })?;
            parameter.elaboration_value = Some(value);
        }
    }
    Ok(())
}

/// A lexical variable, including a function argument, is not a constant array
/// bound even if a public parameter has the same name. Queries use the public
/// namespace and deliberately do not treat their argument as a variable read.
pub(super) fn bound_expression(
    analyzer: &SemanticAnalyzer,
    expression: &Expression,
) -> Option<Expression> {
    let mut expression = expression.clone();
    let mut pending = vec![&mut expression];
    while let Some(expression) = pending.pop() {
        if let Expression::Identifier(identifier) = expression {
            if analyzer.lookup_substitution(&identifier.name).is_some()
                || analyzer
                    .array_bound_shadows
                    .iter()
                    .any(|names| names.contains(&identifier.name))
            {
                return None;
            }
            continue;
        }
        if matches!(expression, Expression::SystemFunction(function)
            if function.name.eq_ignore_ascii_case("$param_given") || function.name.eq_ignore_ascii_case("param_given"))
        {
            continue;
        }
        flow_probes::for_child_mut(expression, &mut |child| pending.push(child));
    }
    Some(expression)
}
