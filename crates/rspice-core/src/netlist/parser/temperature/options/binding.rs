//! Transactional group evaluation and isolated failed-pass discovery.
use super::*;

pub(super) enum ScopeBinding {
    Incomplete,
    Complete(Vec<Value>),
    Failed {
        error: ParseError,
        selected: [Option<(usize, Value)>; 2],
    },
}

impl PendingScope {
    pub(super) fn bind(
        &self,
        scopes: &mut LexicalScopes,
        root: &mut ParamContext,
        frames: &mut [SubcktFrame],
        abort: &dyn AbortSignal,
    ) -> Result<ScopeBinding, ParseWithAbortError> {
        let environment = scopes.environment(self.id, root, frames, abort)?;
        // Probe every operand before drawing from a live stream or publishing
        // bindings. Failed groups may discover temperatures, but only a fresh
        // source pass can validate and publish them.
        match self.stage(&environment.isolated(), abort)?.0 {
            ScopeBinding::Complete(_) => {}
            other => return Ok(other),
        }
        let (result, resolver) = self.stage(&environment, abort)?;
        if matches!(result, ScopeBinding::Complete(_)) {
            scopes.materialize_closed(&resolver, &environment, abort)?;
            scopes.materialize_active(&resolver, root, frames, abort)?;
        }
        Ok(result)
    }

    fn stage(
        &self,
        environment: &ScopeEnvironment,
        abort: &dyn AbortSignal,
    ) -> Result<(ScopeBinding, ParameterResolver), ParseWithAbortError> {
        let mut resolver = ParameterResolver::default();
        let mut values = Vec::with_capacity(self.entries.len());
        let mut selected = [None; 2];
        let mut first_error = None;
        for pending in &self.entries {
            ensure_parse_not_aborted(abort)?;
            match pending.evaluate(self.id, environment, &mut resolver, abort) {
                Ok(Some(value)) => {
                    values.push(value);
                    selected[pending.option.index()] = Some((pending.ordinal, value));
                }
                // Even after an ordinary error, an unfinished ancestor can
                // redefine a demanded declaration. Delay the entire group.
                Ok(None) => return Ok((ScopeBinding::Incomplete, resolver)),
                Err(
                    error @ (ParseWithAbortError::Aborted
                    | ParseWithAbortError::Parse(ParseError::ResourceLimit(_))),
                ) => return Err(error),
                Err(ParseWithAbortError::Parse(error)) => {
                    first_error.get_or_insert(error);
                }
            }
        }
        let result = first_error.map_or_else(
            || ScopeBinding::Complete(values),
            |error| ScopeBinding::Failed { error, selected },
        );
        Ok((result, resolver))
    }
}

impl PendingTemperatureOption {
    fn evaluate(
        &self,
        scope: usize,
        environment: &ScopeEnvironment,
        resolver: &mut ParameterResolver,
        abort: &dyn AbortSignal,
    ) -> Result<Option<Value>, ParseWithAbortError> {
        let params = environment.parameters(scope);
        let mut expression = self.expression.clone();
        expression.begin_evaluation();
        let value = loop {
            ensure_parse_not_aborted(abort)?;
            match expression
                .resume_with(params, &mut |name| {
                    Ok(self
                        .bound_values
                        .get(name)
                        .copied()
                        .or_else(|| resolver.scoped_value(scope, name, environment)))
                })
                .map_err(|error| self.error(error.to_string()))?
            {
                PreparedProgress::Complete(value) => break value,
                PreparedProgress::MissingParameter(name) => {
                    match resolver.resolve_binding(scope, &name, environment, abort) {
                        Ok(_) => {}
                        Err(ParameterResolutionError::IncompleteScope(_)) => return Ok(None),
                        Err(ParameterResolutionError::Aborted) => {
                            return Err(ParseWithAbortError::Aborted);
                        }
                        Err(error) => return Err(self.error(error.to_string()).into()),
                    }
                }
            }
        };
        let value = if params.expression_dialect() == ExpressionDialect::Xyce {
            crate::netlist::expr::normalize_xyce_expression_result(value)
        } else {
            value
        }
        .re * self.sign;
        parse_celsius_option(self.option.name(), value, self.origin.line)
            .map(Some)
            .map_err(|error| self.error(error.to_string()).into())
    }
}
