//! Typed, ordered parameters shared by inspection reports.

use rspice_core::netlist::{ModelDef, ParametricValue};
use serde::Serialize;

#[derive(Serialize)]
pub(super) struct Parameter<'a> {
    name: &'a str,
    #[serde(flatten)]
    value: ParameterValue<'a>,
}

#[derive(Serialize)]
#[serde(tag = "kind", content = "value", rename_all = "snake_case")]
enum ParameterValue<'a> {
    Real(f64),
    Expression(&'a str),
    String(&'a str),
    StringExpression(&'a str),
    StringVector(&'a [String]),
    RealVector(&'a [f64]),
    RealVectorExpression(&'a [String]),
    IntegerVector(&'a [i64]),
}

pub(super) fn scalar_parameters<'a>(
    real: &'a [(String, f64)],
    expressions: &'a [(String, String)],
    strings: &'a [(String, String)],
) -> Vec<Parameter<'a>> {
    real.iter()
        .map(|(name, value)| Parameter {
            name,
            value: ParameterValue::Real(*value),
        })
        .chain(expressions.iter().map(|(name, value)| Parameter {
            name,
            value: ParameterValue::Expression(value),
        }))
        .chain(strings.iter().map(|(name, value)| Parameter {
            name,
            value: ParameterValue::String(value),
        }))
        .collect()
}

pub(super) fn instance_parameters(params: &[(String, ParametricValue)]) -> Vec<Parameter<'_>> {
    params
        .iter()
        .map(|(name, value)| Parameter {
            name,
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
                name,
                value: ParameterValue::StringVector(value),
            }),
    );
    params.extend(
        model
            .real_vector_params
            .iter()
            .map(|(name, value)| Parameter {
                name,
                value: ParameterValue::RealVector(value),
            }),
    );
    params.extend(
        model
            .real_vector_expr_params
            .iter()
            .map(|(name, value)| Parameter {
                name,
                value: ParameterValue::RealVectorExpression(value),
            }),
    );
    params.extend(
        model
            .integer_vector_params
            .iter()
            .map(|(name, value)| Parameter {
                name,
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
