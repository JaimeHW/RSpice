//! Bind one scoped analysis through the shared lexical environment.
use super::*;
use crate::netlist::expr::{ParameterEnvironment, ParameterResolutionError, ParameterResolver};
use crate::netlist::parser::scopes::{LexicalScopes, ScopeEnvironment};

pub(super) fn bind(
    scopes: &mut LexicalScopes,
    pending: &PendingCard,
    context: AnalysisCardContext<'_>,
    abort: &dyn AbortSignal,
) -> Result<ParsedAnalysisCard, ParseWithAbortError> {
    let environment = scopes.environment(pending.scope, context.params, abort)?;
    stage(pending, context, &environment.isolated(), abort)?;
    let (card, resolver) = stage(pending, context, &environment, abort)?;
    scopes.materialize_closed(&resolver, &environment, abort)?;
    Ok(card)
}

fn stage(
    pending: &PendingCard,
    context: AnalysisCardContext<'_>,
    environment: &ScopeEnvironment,
    abort: &dyn AbortSignal,
) -> Result<(ParsedAnalysisCard, ParameterResolver), ParseWithAbortError> {
    let bound = pending.context(environment.parameters(pending.scope));
    let mut resolver = ParameterResolver::default();
    let mut stream = pending.stream.clone();
    stream.begin_numeric_binding();
    loop {
        ensure_parse_not_aborted(abort)?;
        match ParsedAnalysisCard::stage(
            AnalysisHead::parse(&pending.command).expect("saved analysis head"),
            &pending.command,
            &mut stream,
            AnalysisCardContext {
                params: &bound,
                ..context
            },
        ) {
            Ok(card) => return Ok((card, resolver)),
            Err(error) => {
                let Some(name) = stream.missing_numeric_parameter().map(str::to_owned) else {
                    return Err(located_error(error, context.line_num, context.origin).into());
                };
                if bound.get_string(&name).is_some() {
                    return Err(located_error(error, context.line_num, context.origin).into());
                }
                let global = !environment
                    .parameters(pending.scope)
                    .has_parameter_binding(&name);
                let owner = environment.owner_scope(pending.scope, &name, global);
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
                                context.line_num,
                                context.origin,
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
                    value.ok_or_else(|| located_error(error, context.line_num, context.origin))?
                };
                stream.resume_numeric_binding(&pending.stream, name, value);
            }
        }
    }
}
