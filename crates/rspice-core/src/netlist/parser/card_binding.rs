//! Authored numeric cards bound against the shared completed lexical scopes.
use super::scopes::{LexicalScopes, ScopeEnvironment};
use super::*;
use crate::netlist::expr::{ParameterEnvironment, ParameterResolutionError, ParameterResolver};

#[derive(Debug)]
pub(super) struct CardBinding {
    pub(super) scope: usize,
    pub(super) stream: TokenStream,
    values: Vec<(String, crate::ComplexValue)>,
    strings: Vec<(String, String)>,
    functions: Vec<crate::netlist::expr::FunctionDef>,
}

impl CardBinding {
    pub(super) fn capture(scope: usize, stream: TokenStream, params: &ParamContext) -> Self {
        let mut values = params
            .all_params()
            .into_iter()
            .filter_map(|(name, _)| params.get_complex(&name).map(|value| (name, value)))
            .collect::<Vec<_>>();
        // Builtins need not have stored bindings, but were already readable at
        // this card. Temperature reconciliation may replay the whole parser.
        for name in ["TEMP", "TEMPER", "TNOM", "VT"] {
            if let Some(value) = params.get_complex(name) {
                values.push((name.to_owned(), value));
            }
        }
        Self {
            scope,
            stream,
            values,
            strings: params.all_string_params(),
            functions: params.all_functions(),
        }
    }

    pub(super) fn context(&self, completed: &ParamContext) -> ParamContext {
        // Preserve resolved source-order bindings, never an old unresolved
        // definition in place of its selected final value.
        let mut params = completed.clone();
        for (name, value) in &self.values {
            params.set_complex(name, *value);
        }
        for (name, value) in &self.strings {
            params.set_string(name, value.clone());
        }
        for function in &self.functions {
            params.import_function(function.clone());
        }
        params
    }

    pub(super) fn bind<T>(
        &self,
        scopes: &mut LexicalScopes,
        root: &ParamContext,
        line: usize,
        origin: &NetlistSourceLocation,
        abort: &dyn AbortSignal,
        mut parse: impl FnMut(&mut TokenStream, &ParamContext) -> Result<T, ParseError>,
    ) -> Result<T, ParseWithAbortError> {
        let environment = scopes.environment(self.scope, root, &[], abort)?;
        self.stage(&environment.isolated(), line, origin, abort, &mut parse)?;
        let (card, resolver) = self.stage(&environment, line, origin, abort, &mut parse)?;
        scopes.materialize_closed(&resolver, &environment, abort)?;
        Ok(card)
    }

    /// Validate a card during an already-failed pass without consuming live
    /// draws or materializing any of the declarations it reaches.
    pub(super) fn probe<T>(
        &self,
        environment: &ScopeEnvironment,
        line: usize,
        origin: &NetlistSourceLocation,
        abort: &dyn AbortSignal,
        mut parse: impl FnMut(&mut TokenStream, &ParamContext) -> Result<T, ParseError>,
    ) -> Result<T, ParseWithAbortError> {
        self.stage(&environment.isolated(), line, origin, abort, &mut parse)
            .map(|(card, _)| card)
    }

    fn stage<T>(
        &self,
        environment: &ScopeEnvironment,
        line: usize,
        origin: &NetlistSourceLocation,
        abort: &dyn AbortSignal,
        parse: &mut impl FnMut(&mut TokenStream, &ParamContext) -> Result<T, ParseError>,
    ) -> Result<(T, ParameterResolver), ParseWithAbortError> {
        let bound = self.context(environment.parameters(self.scope));
        let mut resolver = ParameterResolver::default();
        let mut stream = self.stream.clone();
        stream.begin_numeric_binding();
        loop {
            ensure_parse_not_aborted(abort)?;
            match parse(&mut stream, &bound) {
                Ok(card) => return Ok((card, resolver)),
                Err(error) => {
                    let Some(name) = stream.missing_numeric_parameter().map(str::to_owned) else {
                        return Err(located_error(error, line, origin).into());
                    };
                    if bound.get_string(&name).is_some() {
                        return Err(located_error(error, line, origin).into());
                    }
                    let global = !environment
                        .parameters(self.scope)
                        .has_parameter_binding(&name);
                    let owner = environment.owner_scope(self.scope, &name, global);
                    let params = environment.parameters(owner);
                    let expression = if global {
                        params.get_global_expression(&name)
                    } else {
                        params.get_parameter_expression(&name)
                    };
                    let value = if let Some(expression) = expression {
                        resolver
                            .resolve_scoped(owner, global, &name, expression, environment, abort)
                            .map_err(|error| match error {
                                ParameterResolutionError::Aborted => ParseWithAbortError::Aborted,
                                error => located_error(
                                    ParseError::InvalidValue(error.to_string()),
                                    line,
                                    origin,
                                )
                                .into(),
                            })?
                    } else {
                        let value = if global {
                            params.get_global_complex(&name)
                        } else if params.has_parameter_binding(&name) {
                            params.get_complex(&name)
                        } else {
                            None
                        };
                        value.ok_or_else(|| located_error(error, line, origin))?
                    };
                    stream.resume_numeric_binding(self.stream.checkpoint(), name, value);
                }
            }
        }
    }
}

pub(super) fn located_error(
    error: ParseError,
    line: usize,
    origin: &NetlistSourceLocation,
) -> ParseError {
    match error {
        ParseError::InvalidValue(message) => {
            ParseError::InvalidValue(format!("{origin}: {message}"))
        }
        error => source_map_logical_line_error(error, line, origin, true),
    }
}
