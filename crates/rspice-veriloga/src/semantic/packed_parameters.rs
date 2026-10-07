//! Keep exact elaboration constants out of the analog binary64 parameter ABI.

use super::*;
use std::borrow::Cow;

fn refusal(name: &str, detail: &str, span: Span) -> CompileError {
    SemanticError::new(
        SemanticErrorKind::UnsupportedFeature(format!("packed parameter '{name}' {detail}")),
        span,
    )
    .into()
}

/// Both executable lowerings consume this same partition. Never manufacture a
/// numeric placeholder for a value that has X/Z bits or loses integer precision.
pub(crate) fn retain_packed_parameters(
    mut selected: Cow<'_, AnalyzedModule>,
) -> CompileResult<Cow<'_, AnalyzedModule>> {
    let module = selected.as_ref();
    let mut packed = HashMap::new();
    for (index, parameter) in module.parameters.iter().enumerate() {
        if parameter.default.is_none()
            && parameter.dimensions.is_empty()
            && let Some(Expression::Digital(DigitalExpr::FourState(literal))) =
                &parameter.default_expr
        {
            if parameter.range.is_some() {
                return Err(refusal(
                    &parameter.name,
                    "requires typed range validation before it can leave the numeric ABI",
                    literal.span,
                ));
            }
            packed.insert(index, literal.value.clone());
        }
    }
    if packed.is_empty() && module.digital.elaboration_parameters.is_empty() {
        return Ok(selected);
    }
    let names: HashSet<_> = packed
        .keys()
        .map(|index| module.parameters[*index].name.as_str())
        .chain(
            module
                .digital
                .elaboration_parameters
                .iter()
                .map(|parameter| parameter.name.as_str()),
        )
        .collect();
    // Check before partitioning, including names of built-in numeric constants:
    // removing a parameter named M_PI must never reveal the built-in value.
    reject_numeric_uses(module, &names)?;
    if packed.is_empty() {
        return Ok(selected);
    }
    let module = selected.to_mut();
    let mut slots = Vec::with_capacity(module.parameters.len());
    let mut numeric = Vec::with_capacity(module.parameters.len() - packed.len());
    for (index, parameter) in std::mem::take(&mut module.parameters)
        .into_iter()
        .enumerate()
    {
        if let Some(value) = packed.remove(&index) {
            slots.push(Err(module.digital.elaboration_parameters.len()));
            module
                .digital
                .elaboration_parameters
                .push(AnalyzedPackedParameter {
                    name: parameter.name,
                    aliases: Vec::new(),
                    is_public: parameter.is_public,
                    scope: parameter.scope,
                    also_model: parameter.also_model,
                    value,
                });
        } else {
            slots.push(Ok(numeric.len()));
            numeric.push(parameter);
        }
    }
    module.parameters = numeric;
    let mut aliases = Vec::new();
    for mut alias in std::mem::take(&mut module.param_aliases) {
        match slots[alias.target] {
            Ok(index) => {
                alias.target = index;
                aliases.push(alias);
            }
            Err(index) => module.digital.elaboration_parameters[index]
                .aliases
                .push(alias.alias),
        }
    }
    module.param_aliases = aliases;
    Ok(selected)
}

/// Include both analog representations, parameter dependencies and range names.
/// None may fall through to an absent numeric slot or to a built-in constant.
fn reject_numeric_uses(module: &AnalyzedModule, names: &HashSet<&str>) -> CompileResult<()> {
    let mut found = None;
    let mut inspect = |expression: &Expression| {
        flow_probes::visit_expression(expression, &mut |expression| {
            let name = match expression {
                Expression::Identifier(value) => Some(value.name.as_str()),
                Expression::ArrayAccess(value) => Some(value.array.as_str()),
                Expression::Digital(value) => value.base_name().map(|name| name.as_str()),
                _ => None,
            };
            if found.is_none()
                && let Some(name) = name.filter(|name| names.contains(name))
            {
                found = Some((name.to_owned(), expression.span()));
            }
        });
    };
    for contribution in &module.contributions {
        inspect(&contribution.expression);
        if let Some(tolerance) = &contribution.equation_abstol {
            inspect(tolerance);
        }
    }
    flow_probes::visit_statements(&module.statements, &mut inspect);
    let mut regions: Vec<_> = module.body.iter().collect();
    while let Some(region) = regions.pop() {
        match region {
            AnalyzedRegion::Assignment(value) => {
                inspect(&value.expression);
                if let Some(index) = &value.index {
                    inspect(index);
                }
            }
            AnalyzedRegion::Contribution(value) => {
                inspect(&value.expression);
                if let Some(tolerance) = &value.equation_abstol {
                    inspect(tolerance);
                }
            }
            AnalyzedRegion::Conditional {
                condition,
                then_body,
                else_body,
                ..
            } => {
                inspect(condition);
                regions.extend(then_body.iter().chain(else_body));
            }
            AnalyzedRegion::Loop {
                condition, body, ..
            } => {
                inspect(condition);
                regions.extend(body);
            }
            AnalyzedRegion::Initialization { body, .. } => regions.extend(body),
            AnalyzedRegion::Task(task) => task.expressions().for_each(&mut inspect),
        }
    }
    for parameter in &module.parameters {
        if names.contains(parameter.name.as_str()) {
            continue;
        }
        if let Some(default) = &parameter.default_expr {
            inspect(default);
        }
        for dimension in &parameter.dimensions {
            inspect(&dimension.left);
            inspect(&dimension.right);
        }
        if let Some(range) = &parameter.range {
            for expression in range
                .min_expression
                .iter()
                .chain(&range.max_expression)
                .chain(&range.exclude_expressions)
            {
                inspect(expression);
            }
        }
    }
    for parameter in &module.parameters {
        if let Some(range) = &parameter.range {
            for name in range
                .min_parameter
                .iter()
                .chain(&range.max_parameter)
                .chain(&range.exclude_parameters)
            {
                if names.contains(name.as_str()) {
                    return Err(refusal(
                        name,
                        "cannot supply a numeric parameter range",
                        Span::dummy(),
                    ));
                }
            }
        }
    }
    if let Some((name, span)) = found {
        return Err(refusal(
            &name,
            "cannot be read through the analog numeric ABI; a typed conversion is required",
            span,
        ));
    }
    Ok(())
}
