//! Retain only scopes needed by a pending card and resolve demanded bindings.
use super::*;
use crate::netlist::expr::{ParameterEnvironment, ParameterResolutionError, ParameterResolver};
use std::collections::BTreeMap;

#[derive(Debug, Default)]
pub(super) struct AnalysisScopes {
    next: usize,
    active: Vec<(usize, usize)>,
    retained: BTreeMap<usize, Scope>,
}

#[derive(Debug)]
struct Scope {
    parent: usize,
    params: Option<ParamContext>,
}

struct Environment(BTreeMap<usize, (usize, ParamContext)>);

impl ParameterEnvironment for Environment {
    fn parameters(&self, scope: usize) -> &ParamContext {
        &self.0[&scope].1
    }
    fn owner_scope(&self, mut scope: usize, name: &str, global: bool) -> usize {
        while scope != 0
            && !self
                .parameters(scope)
                .owns_parameter_definition(name, global)
            && !self
                .parameters(scope)
                .has_materialized_binding(name, global)
        {
            scope = self.0[&scope].0;
        }
        scope
    }
}

impl AnalysisScopes {
    pub(super) fn open(&mut self) {
        let parent = self.active.last().map_or(0, |entry| entry.0);
        self.next += 1;
        self.active.push((self.next, parent));
    }

    pub(super) fn close(&mut self, params: ParamContext) {
        let (scope, _) = self
            .active
            .pop()
            .expect("analysis scope follows parser scope");
        if let Some(retained) = self.retained.get_mut(&scope) {
            retained.params = Some(params);
        }
    }

    pub(super) fn retain(&mut self) -> usize {
        for &(scope, parent) in &self.active {
            self.retained.entry(scope).or_insert(Scope {
                parent,
                params: None,
            });
        }
        self.active.last().map_or(0, |entry| entry.0)
    }

    pub(super) fn bind(
        &mut self,
        pending: &PendingCard,
        context: AnalysisCardContext<'_>,
        abort: &dyn AbortSignal,
    ) -> Result<ParsedAnalysisCard, ParseWithAbortError> {
        let mut ancestry = Vec::new();
        let mut scope = pending.scope;
        while scope != 0 {
            ensure_parse_not_aborted(abort)?;
            ancestry.push(scope);
            scope = self.retained[&scope].parent;
        }
        let mut environment = Environment(BTreeMap::from([(0, (0, context.params.clone()))]));
        for &scope in ancestry.iter().rev() {
            ensure_parse_not_aborted(abort)?;
            let saved = &self.retained[&scope];
            let mut params = saved
                .params
                .as_ref()
                .expect("subcircuit scope closed")
                .clone();
            params
                .complete_inherited_bindings(environment.parameters(saved.parent), abort)
                .map_err(binding_error)?;
            environment.0.insert(scope, (saved.parent, params));
        }
        let mut probe = Environment(environment.0.clone());
        let mut streams = HashMap::new();
        for (_, params) in probe.0.values_mut() {
            params.isolate_random_stream(&mut streams);
        }
        Self::stage(pending, context, &probe, abort)?;
        let (card, resolver) = Self::stage(pending, context, &environment, abort)?;
        // Only the successful live transaction owns these samples. Keep the
        // original SubcircuitDef bodies intact for per-instance elaboration.
        for scope in ancestry {
            ensure_parse_not_aborted(abort)?;
            resolver.materialize_scope(
                scope,
                self.retained
                    .get_mut(&scope)
                    .unwrap()
                    .params
                    .as_mut()
                    .unwrap(),
            );
        }
        Ok(card)
    }

    fn stage(
        pending: &PendingCard,
        context: AnalysisCardContext<'_>,
        environment: &Environment,
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
                        value
                            .ok_or_else(|| located_error(error, context.line_num, context.origin))?
                    };
                    stream.resume_numeric_binding(&pending.stream, name, value);
                }
            }
        }
    }
}

fn binding_error(error: ParameterResolutionError) -> ParseWithAbortError {
    match error {
        ParameterResolutionError::Aborted => ParseWithAbortError::Aborted,
        error => ParseError::InvalidValue(error.to_string()).into(),
    }
}
