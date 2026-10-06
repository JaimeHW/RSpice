//! Lexical environments shared by deferred parser consumers.
use super::*;
use crate::netlist::expr::{ParameterEnvironment, ParameterResolutionError, ParameterResolver};
use std::collections::BTreeMap;

#[derive(Debug, Default)]
pub(super) struct LexicalScopes {
    next: usize,
    active: Vec<(usize, usize)>,
    retained: BTreeMap<usize, Scope>,
}

#[derive(Debug)]
struct Scope {
    parent: usize,
    params: Option<ParamContext>,
}

pub(super) struct ScopeEnvironment(BTreeMap<usize, (usize, ParamContext)>);

impl ParameterEnvironment for ScopeEnvironment {
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

impl LexicalScopes {
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

    pub(super) fn environment(
        &self,
        scope: usize,
        root: &ParamContext,
        abort: &dyn AbortSignal,
    ) -> Result<ScopeEnvironment, ParseWithAbortError> {
        let mut ancestry = Vec::new();
        let mut current = scope;
        while current != 0 {
            ensure_parse_not_aborted(abort)?;
            ancestry.push(current);
            current = self.retained[&current].parent;
        }
        let mut environment = ScopeEnvironment(BTreeMap::from([(0, (0, root.clone()))]));
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
                .map_err(|error| match error {
                    ParameterResolutionError::Aborted => ParseWithAbortError::Aborted,
                    error => ParseError::InvalidValue(error.to_string()).into(),
                })?;
            environment.0.insert(scope, (saved.parent, params));
        }
        Ok(environment)
    }

    pub(super) fn materialize_closed(
        &mut self,
        resolver: &ParameterResolver,
        environment: &ScopeEnvironment,
        abort: &dyn AbortSignal,
    ) -> Result<(), ParseWithAbortError> {
        for &scope in environment.0.keys().rev().filter(|&&scope| scope != 0) {
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
        Ok(())
    }
}

impl ScopeEnvironment {
    pub(super) fn isolated(&self) -> Self {
        let mut probe = Self(self.0.clone());
        let mut streams = HashMap::new();
        for (_, params) in probe.0.values_mut() {
            params.isolate_random_stream(&mut streams);
        }
        probe
    }
}
