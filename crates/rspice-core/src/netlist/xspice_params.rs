//! Classify deferred vector literals and string aliases before numeric instance binding.

use super::xspice_parser::{
    XspiceParamValue, parse_deferred_xspice_vector, parse_xspice_string_value,
};
use super::{ModelDef, ParamContext, ParseError, ParseWithAbortError, ensure_parse_not_aborted};
use crate::abort_signal::AbortSignal;
use std::collections::HashSet;

/// One XSPICE instance card's parameters, split by the form the parser left
/// them in: resolved scalars, unresolved scalar expressions, and the string,
/// string-vector and real-vector forms with their own unresolved variants.
/// A resolver reads all eight to decide what a parameter means, so they are
/// one card rather than eight lists.
#[derive(Clone, Copy)]
pub(crate) struct XspiceInstanceParams<'a> {
    pub params: &'a [(String, f64)],
    pub expr_params: &'a [(String, String)],
    pub string_params: &'a [(String, String)],
    pub string_expr_params: &'a [(String, String)],
    pub string_vector_params: &'a [(String, Vec<String>)],
    pub string_vector_expr_params: &'a [(String, String)],
    pub real_vector_params: &'a [(String, Vec<f64>)],
    pub real_vector_expr_params: &'a [(String, Vec<String>)],
}

/// Allocate a replacement only for a deferred vector literal or bound string alias.
/// The common numeric-only path continues to borrow the authored slices.
pub(crate) struct MaterializedXspiceParams {
    params: Vec<(String, f64)>,
    expr_params: Vec<(String, String)>,
    string_params: Vec<(String, String)>,
    string_expr_params: Vec<(String, String)>,
    string_vector_params: Vec<(String, Vec<String>)>,
    string_vector_expr_params: Vec<(String, String)>,
    real_vector_params: Vec<(String, Vec<f64>)>,
    real_vector_expr_params: Vec<(String, Vec<String>)>,
}

impl MaterializedXspiceParams {
    fn copy_from(input: XspiceInstanceParams<'_>) -> Self {
        Self {
            params: input.params.to_vec(),
            expr_params: input.expr_params.to_vec(),
            string_params: input.string_params.to_vec(),
            string_expr_params: input.string_expr_params.to_vec(),
            string_vector_params: input.string_vector_params.to_vec(),
            string_vector_expr_params: input.string_vector_expr_params.to_vec(),
            real_vector_params: input.real_vector_params.to_vec(),
            real_vector_expr_params: input.real_vector_expr_params.to_vec(),
        }
    }

    pub(crate) fn as_ref(&self) -> XspiceInstanceParams<'_> {
        XspiceInstanceParams {
            params: &self.params,
            expr_params: &self.expr_params,
            string_params: &self.string_params,
            string_expr_params: &self.string_expr_params,
            string_vector_params: &self.string_vector_params,
            string_vector_expr_params: &self.string_vector_expr_params,
            real_vector_params: &self.real_vector_params,
            real_vector_expr_params: &self.real_vector_expr_params,
        }
    }

    fn replace(&mut self, name: &str, value: XspiceParamValue) -> Result<(), ParseError> {
        self.params
            .retain(|(key, _)| !key.eq_ignore_ascii_case(name));
        self.expr_params
            .retain(|(key, _)| !key.eq_ignore_ascii_case(name));
        self.string_params
            .retain(|(key, _)| !key.eq_ignore_ascii_case(name));
        self.string_expr_params
            .retain(|(key, _)| !key.eq_ignore_ascii_case(name));
        self.string_vector_params
            .retain(|(key, _)| !key.eq_ignore_ascii_case(name));
        self.string_vector_expr_params
            .retain(|(key, _)| !key.eq_ignore_ascii_case(name));
        self.real_vector_params
            .retain(|(key, _)| !key.eq_ignore_ascii_case(name));
        self.real_vector_expr_params
            .retain(|(key, _)| !key.eq_ignore_ascii_case(name));
        let name = name.to_string();
        match value {
            XspiceParamValue::Resolved(value) => self.params.push((
                name,
                super::expr::require_real(value)
                    .map_err(|error| ParseError::InvalidValue(error.to_string()))?,
            )),
            XspiceParamValue::Deferred(value) => self.expr_params.push((name, value)),
            XspiceParamValue::String(value) => self.string_params.push((name, value)),
            XspiceParamValue::StringDeferred(value) => self.string_expr_params.push((name, value)),
            XspiceParamValue::StringVector(value) => self.string_vector_params.push((name, value)),
            XspiceParamValue::StringVectorDeferred(value) => {
                self.string_vector_expr_params.push((name, value))
            }
            XspiceParamValue::RealVector(value) => self.real_vector_params.push((name, value)),
            XspiceParamValue::RealVectorDeferred(value) => {
                self.real_vector_expr_params.push((name, value))
            }
        }
        Ok(())
    }
}

/// Numeric consumers retain the enclosing values for self-references while
/// explicit non-scalar siblings mask every path to an enclosing numeric value.
#[derive(Clone, Copy)]
pub(crate) struct XspiceNumericScope<'a> {
    pub parameters: &'a ParamContext,
    pub non_scalar_fields: &'a HashSet<String>,
}

impl XspiceNumericScope<'_> {
    pub(crate) fn evaluate_real(
        self,
        field: &str,
        expression: &str,
        abort: &dyn AbortSignal,
    ) -> Result<f64, super::expr::ExpressionEvaluationError> {
        super::expr::evaluate_instance_expression(
            expression,
            self.parameters,
            self.non_scalar_fields,
            self.non_scalar_fields,
            field,
            abort,
        )
        .and_then(|value| super::expr::require_real(value).map_err(Into::into))
    }
}

impl XspiceInstanceParams<'_> {
    pub(crate) fn non_scalar_fields(self) -> HashSet<String> {
        self.string_params
            .iter()
            .chain(self.string_expr_params)
            .map(|(name, _)| name)
            .chain(self.string_vector_params.iter().map(|(name, _)| name))
            .chain(self.string_vector_expr_params.iter().map(|(name, _)| name))
            .chain(self.real_vector_params.iter().map(|(name, _)| name))
            .chain(self.real_vector_expr_params.iter().map(|(name, _)| name))
            .map(|name| name.to_ascii_uppercase())
            .collect()
    }

    /// Classify ambiguous literals and string bindings. Retain numeric expressions
    /// for the ordinary instance resolver, so it cannot sample or freeze a
    /// sibling/model binding before the complete numeric context is available.
    pub(crate) fn materialize_deferred_values(
        self,
        scope: &ParamContext,
        model: Option<&ModelDef>,
        abort: &dyn AbortSignal,
    ) -> Result<Option<MaterializedXspiceParams>, ParseWithAbortError> {
        let mut materialized = None;
        let mut numeric_fields = None;
        for (name, expression) in self
            .expr_params
            .iter()
            .chain(self.string_expr_params)
            .chain(self.string_vector_expr_params)
        {
            ensure_parse_not_aborted(abort)?;
            let Some(value) =
                parse_deferred_xspice_vector(expression).or_else(|| scope.get_string(expression))
            else {
                continue;
            };
            let numeric_fields = numeric_fields.get_or_insert_with(|| {
                self.params
                    .iter()
                    .map(|(name, _)| name)
                    // A string alias will become a string or vector channel, never
                    // a scalar numeric binding, regardless of assignment order.
                    .chain(
                        self.expr_params
                            .iter()
                            .filter(|(_, expression)| scope.get_string(expression).is_none())
                            .map(|(name, _)| name),
                    )
                    .chain(model.into_iter().flat_map(|model| {
                        model
                            .params
                            .iter()
                            .map(|(name, _)| name)
                            .chain(model.expr_params.iter().map(|(name, _)| name))
                            .filter(|name| !scope.has_any_parameter_binding(name))
                    }))
                    .map(|name| name.to_ascii_uppercase())
                    .collect::<HashSet<_>>()
            });
            let parsed = parse_xspice_string_value(name, value, scope, numeric_fields, abort)?;
            materialized
                .get_or_insert_with(|| MaterializedXspiceParams::copy_from(self))
                .replace(name, parsed)?;
        }
        Ok(materialized)
    }
}
