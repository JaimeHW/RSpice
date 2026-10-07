//! Shared scalar model evaluation for construction and thermal material updates.

use super::*;
use crate::abort_signal::AbortSignal;
use std::collections::HashSet;

/// Evaluate an instance expression against authoritative retained definitions.
/// Failed forward attempts use an isolated stream; only successful evaluation
/// consumes the live stream. Pending siblings also mask transitive reads.
pub(crate) fn evaluate_instance_expression(
    expression: &str,
    context: &ParamContext,
    pending_names: &HashSet<String>,
    non_scalar_fields: &HashSet<String>,
    current_name: &str,
    abort: &dyn AbortSignal,
) -> Result<ComplexValue, ExpressionEvaluationError> {
    let describe_field = |error| match error {
        ExpressionEvaluationError::Expression(ExprError::UndefinedParam(ref name))
            if !name.eq_ignore_ascii_case(current_name)
                && non_scalar_fields.contains(&name.to_ascii_uppercase()) =>
        {
            ExprError::InvalidArgument(format!(
                "instance field '{name}' is nonnumeric in scalar expressions"
            ))
            .into()
        }
        error => error,
    };
    if !context.has_retained_parameter_expressions() {
        return super::api::eval_expression_complex_with_probe_and_resolver(
            expression,
            context,
            &mut |parameter| {
                if !parameter.eq_ignore_ascii_case(current_name)
                    && pending_names.contains(&parameter.to_ascii_uppercase())
                {
                    Err(ExprError::UndefinedParam(parameter.to_owned()))
                } else {
                    Ok(None)
                }
            },
            abort,
        )
        .map_err(describe_field);
    }
    let evaluate = |context: &ParamContext| {
        let blocked = pending_names
            .iter()
            .filter(|name| !name.eq_ignore_ascii_case(current_name))
            .cloned()
            .collect();
        ParameterResolver::with_blocked_parameters(blocked)
            .evaluate_expression(expression, context, abort)
            .map_err(|error| match error {
                ParameterResolutionError::Aborted => ExpressionEvaluationError::Aborted,
                ParameterResolutionError::Expression(error) => error.into(),
                error => ExprError::InvalidArgument(error.to_string()).into(),
            })
    };
    evaluate(&context.isolated_random_clone())
        .and_then(|_| evaluate(context))
        .map_err(describe_field)
}

/// Resolve scalar instance fields in dependency order, publishing each
/// successful value for sibling expressions. An unresolved attempt must not
/// consume statistical samples before a later pass can resolve its bindings.
pub(crate) fn resolve_real_instance_expressions(
    context: &ParamContext,
    expressions: &[(String, String)],
    non_scalar_fields: &HashSet<String>,
    abort: &dyn AbortSignal,
) -> Result<Vec<(String, Value)>, ParameterResolutionError> {
    check_abort(abort)?;
    let mut context = context.clone();
    let mut pending = expressions.iter().collect::<Vec<_>>();
    let mut pending_names = expressions
        .iter()
        .map(|(name, _)| name.to_ascii_uppercase())
        .collect::<HashSet<_>>();
    pending_names.extend(non_scalar_fields.iter().cloned());
    let mut resolved = Vec::with_capacity(expressions.len());

    while !pending.is_empty() {
        let mut unresolved = Vec::new();
        let mut first_error = None;
        let mut progress = false;
        for entry @ (name, expression) in pending {
            // A pending explicit field shadows an enclosing/model fallback
            // for sibling reads. Its own expression can still use its previous
            // enclosing value, as in GAIN={GAIN*2}.
            let value = evaluate_instance_expression(
                expression,
                &context,
                &pending_names,
                non_scalar_fields,
                name,
                abort,
            );
            match value {
                Ok(value) => {
                    let value = require_real(value).map_err(|error| {
                        ParameterResolutionError::Definition(format!(
                            "instance expression parameter '{name}': {error}"
                        ))
                    })?;
                    if !value.is_finite() {
                        return Err(ParameterResolutionError::Definition(format!(
                            "instance expression parameter '{name}' resolved to non-finite value {value}"
                        )));
                    }
                    context.set(name, value);
                    pending_names.remove(&name.to_ascii_uppercase());
                    resolved.push((name.clone(), value));
                    progress = true;
                }
                Err(ExpressionEvaluationError::Aborted) => {
                    return Err(ParameterResolutionError::Aborted);
                }
                Err(ExpressionEvaluationError::Expression(error)) => {
                    let waiting_for_sibling = matches!(&error,
                        ExprError::UndefinedParam(dependency)
                        if !dependency.eq_ignore_ascii_case(name)
                            && pending_names.contains(&dependency.to_ascii_uppercase())
                            && !non_scalar_fields.contains(&dependency.to_ascii_uppercase()));
                    // A declared sibling is waiting on another definition;
                    // report the actual missing leaf or invalid expression.
                    if !waiting_for_sibling && first_error.is_none() {
                        first_error = Some((name, error));
                    }
                    unresolved.push(entry);
                }
            }
        }
        if !progress {
            let diagnostic = match first_error {
                Some((name, error)) => {
                    format!("instance expression parameter '{name}' could not be resolved: {error}")
                }
                None => format!(
                    "instance expression parameters could not be resolved (cyclic dependency): {}",
                    unresolved
                        .iter()
                        .map(|(name, _)| name.as_str())
                        .collect::<Vec<_>>()
                        .join(", ")
                ),
            };
            return Err(ParameterResolutionError::Definition(diagnostic));
        }
        pending = unresolved;
    }
    Ok(resolved)
}

pub(crate) struct ModelEvaluationContext<'a> {
    context: ParamContext,
    abort: &'a dyn AbortSignal,
    resolved_expressions: HashMap<String, ComplexValue>,
    expression_errors: HashMap<String, ExprError>,
}

impl<'a> ModelEvaluationContext<'a> {
    pub(crate) fn new(context: ParamContext, abort: &'a dyn AbortSignal) -> Self {
        Self {
            context,
            abort,
            resolved_expressions: HashMap::new(),
            expression_errors: HashMap::new(),
        }
    }

    pub(crate) fn evaluate(&self, expression: &str) -> Result<Value, ExpressionEvaluationError> {
        eval_expression_complex_with_abort(expression, &self.context, self.abort)
            .and_then(|value| require_real(value).map_err(Into::into))
    }

    /// Model construction materializes its defaults. Instance overrides must
    /// still see the enclosing symbolic graph after those defaults are bound.
    pub(crate) fn retain_instance_expressions(&mut self, enclosing: &ParamContext) {
        for (name, expression) in enclosing.all_parameter_expressions() {
            let value = self.context.get_complex(&name);
            self.context
                .define_parameter_expression(&name, expression, value);
        }
        for (name, expression) in enclosing.all_global_expressions() {
            let value = self.context.get_global_complex(&name);
            self.context
                .define_global_expression(&name, expression, value);
        }
    }

    pub(crate) fn real_model_expression(
        &self,
        name: &str,
        expression: &str,
    ) -> Result<Value, ExpressionEvaluationError> {
        if let Some(error) = self.expression_errors.get(&name.to_ascii_uppercase()) {
            return Err(ExpressionEvaluationError::Expression(error.clone()));
        }
        let value = match self.resolved_expressions.get(&name.to_ascii_uppercase()) {
            Some(value) => Ok(*value),
            None => eval_expression_complex_with_abort(expression, &self.context, self.abort),
        }?;
        require_real(value).map_err(Into::into)
    }
}

impl std::ops::Deref for ModelEvaluationContext<'_> {
    type Target = ParamContext;
    fn deref(&self) -> &Self::Target {
        &self.context
    }
}
impl std::ops::DerefMut for ModelEvaluationContext<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.context
    }
}

/// A model's nominal temperature is evaluated at construction and retained
/// unchanged while a thermal device updates its operating temperature.
#[derive(Clone, Copy)]
pub(crate) enum ModelNominalTemperature {
    Default(Value),
    Resolved(Value),
}

impl<'a> ModelEvaluationContext<'a> {
    pub(crate) fn resolve(
        enclosing: &ParamContext,
        mut params: ParamContext,
        model_params: &[(String, Value)],
        expr_params: &[(String, String)],
        current_temp_c: Value,
        nominal: ModelNominalTemperature,
        abort: &'a dyn AbortSignal,
    ) -> Result<Self, ParameterResolutionError> {
        check_abort(abort)?;
        let (tnom_c, fixed_nominal) = match nominal {
            ModelNominalTemperature::Default(value) => (value, false),
            ModelNominalTemperature::Resolved(value) => (value, true),
        };
        let numeric_tnom = model_params
            .iter()
            .find(|(name, _)| name.eq_ignore_ascii_case("TNOM"))
            .map(|(_, value)| *value);
        let nominal_value = if fixed_nominal {
            tnom_c
        } else {
            numeric_tnom.unwrap_or(tnom_c)
        };
        set_model_temperature_scalars(&mut params, current_temp_c, nominal_value);
        let mut ctx = Self::new(params, abort);
        let enclosing_binding = |name: &str| {
            enclosing.has_any_parameter_binding(name)
                && !MODEL_TEMPERATURE_PARAMETERS
                    .iter()
                    .any(|temperature| name.eq_ignore_ascii_case(temperature))
        };
        for (name, value) in model_params {
            check_abort(abort)?;
            // A model field is a fallback expression binding, not a replacement
            // for an enclosing .PARAM/.GLOBAL_PARAM. Card temperature quantities
            // retain their separate construction-time override rules.
            if !(enclosing_binding(name) || fixed_nominal && name.eq_ignore_ascii_case("TNOM")) {
                ctx.set(name, *value);
            }
        }

        if !fixed_nominal
            && numeric_tnom.is_none()
            && let Some((name, expression)) = expr_params
                .iter()
                .find(|(name, _)| name.eq_ignore_ascii_case("TNOM"))
        {
            // TNOM defines the context for all remaining model values. Resolve
            // only its dependencies first, retaining their samples for the main
            // pass instead of evaluating the entire model twice.
            let mut nominal_params = ctx.context.clone();
            let mut model_dependencies = Vec::new();
            for (name, expression) in expr_params {
                check_abort(abort)?;
                if !nominal_params.has_any_parameter_binding(name) {
                    nominal_params.define_parameter_expression(name, expression, None);
                    model_dependencies.push(name);
                }
            }
            let mut resolver = ParameterResolver::default();
            let value = resolver
                .evaluate_expression(expression, &nominal_params, abort)
                .map_err(|error| match error {
                    ParameterResolutionError::Aborted => ParameterResolutionError::Aborted,
                    error => ParameterResolutionError::Definition(format!(
                        "parameter '{}' could not be resolved: {}",
                        name, error
                    )),
                })?;
            resolver.materialize_into(&mut ctx.context);
            for name in model_dependencies {
                if let Some(value) =
                    resolver.scoped_value(0, &name.to_ascii_uppercase(), &nominal_params)
                {
                    ctx.set_complex(name, value);
                    ctx.resolved_expressions
                        .insert(name.to_ascii_uppercase(), value);
                }
            }
            ctx.set_complex("TNOM", value);
            ctx.resolved_expressions.insert("TNOM".to_owned(), value);
        }
        if fixed_nominal {
            ctx.resolved_expressions
                .insert("TNOM".to_owned(), ComplexValue::new(tnom_c, 0.0));
        }
        materialize_available_parameter_expressions_with_abort(&mut ctx.context, abort)?;
        for (name, expression) in expr_params {
            check_abort(abort)?;
            if !ctx.has_any_parameter_binding(name) {
                ctx.define_parameter_expression(name, expression, None);
            }
        }
        let mut resolver = ParameterResolver::default();
        for (name, expression) in expr_params {
            check_abort(abort)?;
            let key = name.to_ascii_uppercase();
            if ctx.resolved_expressions.contains_key(&key) {
                continue;
            }
            // A field with an enclosing binding has its own model value, but its
            // name in another expression still denotes that enclosing parameter.
            let cached = (!enclosing_binding(name))
                .then(|| resolver.scoped_value(0, &key, &ctx.context))
                .flatten();
            let result = match cached {
                Some(value) => Ok(value),
                None => resolver.evaluate_expression(expression, &ctx.context, abort),
            };
            match result {
                Ok(value) => {
                    if !enclosing_binding(name) {
                        ctx.set_complex(name, value);
                    }
                    ctx.resolved_expressions.insert(key, value);
                }
                Err(ParameterResolutionError::Aborted) => {
                    return Err(ParameterResolutionError::Aborted);
                }
                Err(error) => {
                    let error = match error {
                        ParameterResolutionError::Expression(error) => error,
                        error => ExprError::InvalidArgument(error.to_string()),
                    };
                    ctx.expression_errors.insert(key, error);
                }
            }
        }
        Ok(ctx)
    }
}

fn check_abort(abort: &dyn AbortSignal) -> Result<(), ParameterResolutionError> {
    if abort.is_aborted() {
        Err(ParameterResolutionError::Aborted)
    } else {
        Ok(())
    }
}

pub(crate) fn set_model_temperature_scalars(ctx: &mut ParamContext, temp_c: Value, tnom_c: Value) {
    ctx.set("TEMP", temp_c);
    ctx.set("TEMPER", temp_c);
    ctx.set("TNOM", tnom_c);
    ctx.set(
        "VT",
        crate::constants::thermal_voltage(crate::constants::celsius_to_kelvin(temp_c)),
    );
}
