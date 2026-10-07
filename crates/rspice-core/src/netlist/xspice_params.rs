//! Classify deferred string aliases before numeric instance binding.

use super::xspice_parser::{XspiceParamValue, parse_xspice_string_value};
use super::{ParamContext, ParseError, ParseWithAbortError, ensure_parse_not_aborted};
use crate::abort_signal::AbortSignal;

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

/// Allocate a replacement only for cards containing a bound string alias.
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

impl XspiceInstanceParams<'_> {
    /// Resolve only string bindings here. Parsing retains numeric expressions
    /// for the ordinary instance resolver, so it cannot sample or freeze a
    /// sibling/model binding before the complete numeric context is available.
    pub(crate) fn materialize_string_aliases(
        self,
        scope: &ParamContext,
        abort: &dyn AbortSignal,
    ) -> Result<Option<MaterializedXspiceParams>, ParseWithAbortError> {
        let mut materialized = None;
        for (name, expression) in self
            .expr_params
            .iter()
            .chain(self.string_expr_params)
            .chain(self.string_vector_expr_params)
        {
            ensure_parse_not_aborted(abort)?;
            let Some(value) = scope.get_string(expression) else {
                continue;
            };
            let parsed = parse_xspice_string_value(name, value, scope, abort)?;
            materialized
                .get_or_insert_with(|| MaterializedXspiceParams::copy_from(self))
                .replace(name, parsed)?;
        }
        Ok(materialized)
    }
}
