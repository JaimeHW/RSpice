//! Typed, ordered parameters shared by inspection reports.

use rspice_core::netlist::{ModelDef, ParamContext, ParametricValue};
use serde::Serialize;
use std::borrow::Cow;
use std::collections::BTreeSet;

#[derive(Serialize)]
pub(super) struct Parameter<'a> {
    name: Cow<'a, str>,
    #[serde(flatten)]
    value: ParameterValue<'a>,
}

#[derive(Serialize)]
#[serde(tag = "kind", content = "value", rename_all = "snake_case")]
enum ParameterValue<'a> {
    Real(f64),
    Complex { real: f64, imaginary: f64 },
    Expression(&'a str),
    String(&'a str),
    StringExpression(&'a str),
    StringVector(&'a [String]),
    RealVector(&'a [f64]),
    RealVectorExpression(&'a [String]),
    IntegerVector(&'a [i64]),
}

/// Inspect effective bindings without evaluating expressions or advancing the
/// statistical stream. Ordinary bindings shadow globals across value types.
pub(super) fn root_parameters(context: &ParamContext) -> Vec<Parameter<'_>> {
    let names: BTreeSet<_> = context
        .all_params()
        .into_iter()
        .map(|(name, _)| name)
        .chain(
            context
                .all_string_params()
                .into_iter()
                .map(|(name, _)| name),
        )
        .chain(
            context
                .all_parameter_expressions()
                .into_iter()
                .map(|(name, _)| name),
        )
        .chain(
            context
                .all_global_expressions()
                .into_iter()
                .map(|(name, _)| name),
        )
        .collect();
    names
        .into_iter()
        .filter_map(|name| {
            let expression = if context.has_parameter_binding(&name) {
                context.get_parameter_expression(&name)
            } else {
                context.get_global_expression(&name)
            };
            let value = if let Some(expression) = expression {
                ParameterValue::Expression(expression)
            } else if let Some(value) = context.get_string(&name) {
                ParameterValue::String(value)
            } else {
                let value = context.get_complex(&name)?;
                if value.im == 0.0 {
                    ParameterValue::Real(value.re)
                } else {
                    ParameterValue::Complex {
                        real: value.re,
                        imaginary: value.im,
                    }
                }
            };
            Some(Parameter {
                name: Cow::Owned(name),
                value,
            })
        })
        .collect()
}

pub(super) fn scalar_parameters<'a>(
    real: &'a [(String, f64)],
    expressions: &'a [(String, String)],
    strings: &'a [(String, String)],
) -> Vec<Parameter<'a>> {
    real.iter()
        .map(|(name, value)| Parameter {
            name: Cow::Borrowed(name),
            value: ParameterValue::Real(*value),
        })
        .chain(expressions.iter().map(|(name, value)| Parameter {
            name: Cow::Borrowed(name),
            value: ParameterValue::Expression(value),
        }))
        .chain(strings.iter().map(|(name, value)| Parameter {
            name: Cow::Borrowed(name),
            value: ParameterValue::String(value),
        }))
        .collect()
}

pub(super) fn instance_parameters(params: &[(String, ParametricValue)]) -> Vec<Parameter<'_>> {
    params
        .iter()
        .map(|(name, value)| Parameter {
            name: Cow::Borrowed(name),
            value: match value {
                ParametricValue::Resolved(value) => ParameterValue::Real(*value),
                ParametricValue::Expression(value) => ParameterValue::Expression(value),
                ParametricValue::String(value) => ParameterValue::String(value),
                ParametricValue::StringExpression(value) => ParameterValue::StringExpression(value),
            },
        })
        .collect()
}

pub(super) fn model_parameters(model: &ModelDef) -> Vec<Parameter<'_>> {
    let mut params = scalar_parameters(&model.params, &model.expr_params, &model.string_params);
    params.extend(
        model
            .string_vector_params
            .iter()
            .map(|(name, value)| Parameter {
                name: Cow::Borrowed(name),
                value: ParameterValue::StringVector(value),
            }),
    );
    params.extend(
        model
            .real_vector_params
            .iter()
            .map(|(name, value)| Parameter {
                name: Cow::Borrowed(name),
                value: ParameterValue::RealVector(value),
            }),
    );
    params.extend(
        model
            .real_vector_expr_params
            .iter()
            .map(|(name, value)| Parameter {
                name: Cow::Borrowed(name),
                value: ParameterValue::RealVectorExpression(value),
            }),
    );
    params.extend(
        model
            .integer_vector_params
            .iter()
            .map(|(name, value)| Parameter {
                name: Cow::Borrowed(name),
                value: ParameterValue::IntegerVector(value),
            }),
    );
    params
}

impl std::fmt::Display for Parameter<'_> {
    fn fmt(&self, out: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(out, "{} = ", self.name)?;
        match &self.value {
            ParameterValue::Real(value) => write!(out, "{value}"),
            ParameterValue::Complex { real, imaginary } => write!(out, "{real} {imaginary:+}j"),
            ParameterValue::Expression(value) => write!(out, "{{{value}}}"),
            ParameterValue::String(value) => write!(out, "{}", serde_json::json!(value)),
            ParameterValue::StringExpression(value) => write!(out, "{{{value}}} (string)"),
            ParameterValue::StringVector(value) => write!(out, "{}", serde_json::json!(value)),
            ParameterValue::RealVector(value) => write!(out, "{}", serde_json::json!(value)),
            ParameterValue::RealVectorExpression(value) => {
                write!(out, "{} (expressions)", serde_json::json!(value))
            }
            ParameterValue::IntegerVector(value) => write!(out, "{}", serde_json::json!(value)),
        }
    }
}
