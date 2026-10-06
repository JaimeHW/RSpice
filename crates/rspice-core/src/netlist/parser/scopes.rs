//! Lexical environments shared by deferred parser consumers.
use super::*;
use crate::netlist::expr::{ParameterEnvironment, ParameterResolutionError, ParameterResolver};
use std::collections::{BTreeMap, HashSet};

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

pub(super) struct ScopeEnvironment {
    contexts: BTreeMap<usize, (usize, ParamContext)>,
    incomplete: HashSet<usize>,
}

impl ParameterEnvironment for ScopeEnvironment {
    fn parameters(&self, scope: usize) -> &ParamContext {
        &self.contexts[&scope].1
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
            scope = self.contexts[&scope].0;
        }
        scope
    }
    fn ensure_ready(&self, scope: usize) -> Result<(), ParameterResolutionError> {
        if self.incomplete.contains(&scope) {
            Err(ParameterResolutionError::IncompleteScope(scope))
        } else {
            Ok(())
        }
    }
}

impl LexicalScopes {
    pub(super) fn current(&self) -> usize {
        self.active.last().map_or(0, |entry| entry.0)
    }

    pub(super) fn is_descendant(&self, mut scope: usize, ancestor: usize) -> bool {
        while scope != ancestor && scope != 0 {
            scope = self.retained[&scope].parent;
        }
        scope == ancestor
    }

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
        frames: &[SubcktFrame],
        abort: &dyn AbortSignal,
    ) -> Result<ScopeEnvironment, ParseWithAbortError> {
        let mut ancestry = Vec::new();
        let mut current = scope;
        while current != 0 {
            ensure_parse_not_aborted(abort)?;
            let (parent, params) =
                if let Some(index) = self.active.iter().position(|entry| entry.0 == current) {
                    (self.active[index].1, &frames[index].local_params)
                } else {
                    let saved = &self.retained[&current];
                    (
                        saved.parent,
                        saved.params.as_ref().expect("retained scope closed"),
                    )
                };
            ancestry.push((current, parent, params));
            current = parent;
        }
        let closing = self.current();
        let mut incomplete: HashSet<_> = self
            .active
            .iter()
            .map(|entry| entry.0)
            .filter(|&id| id != closing)
            .collect();
        if closing != 0 {
            incomplete.insert(0);
        }
        let mut environment = ScopeEnvironment {
            contexts: BTreeMap::from([(0, (0, root.clone()))]),
            incomplete,
        };
        for &(scope, parent, saved) in ancestry.iter().rev() {
            ensure_parse_not_aborted(abort)?;
            let mut params = saved.clone();
            params
                .complete_inherited_bindings(environment.parameters(parent), abort)
                .map_err(|error| match error {
                    ParameterResolutionError::Aborted => ParseWithAbortError::Aborted,
                    error => ParseError::InvalidValue(error.to_string()).into(),
                })?;
            environment.contexts.insert(scope, (parent, params));
        }
        Ok(environment)
    }

    pub(super) fn materialize_closed(
        &mut self,
        resolver: &ParameterResolver,
        environment: &ScopeEnvironment,
        abort: &dyn AbortSignal,
    ) -> Result<(), ParseWithAbortError> {
        for &scope in environment
            .contexts
            .keys()
            .rev()
            .filter(|&&scope| scope != 0)
        {
            ensure_parse_not_aborted(abort)?;
            if let Some(params) = self
                .retained
                .get_mut(&scope)
                .and_then(|saved| saved.params.as_mut())
            {
                resolver.materialize_scope(scope, params);
            }
        }
        Ok(())
    }

    pub(super) fn materialize_active(
        &self,
        resolver: &ParameterResolver,
        root: &mut ParamContext,
        frames: &mut [SubcktFrame],
        abort: &dyn AbortSignal,
    ) -> Result<(), ParseWithAbortError> {
        ensure_parse_not_aborted(abort)?;
        resolver.materialize_into(root);
        for ((scope, _), frame) in self.active.iter().zip(frames) {
            ensure_parse_not_aborted(abort)?;
            resolver.materialize_scope(*scope, &mut frame.local_params);
        }
        Ok(())
    }
}

impl ScopeEnvironment {
    pub(super) fn isolated(&self) -> Self {
        let mut probe = Self {
            contexts: self.contexts.clone(),
            incomplete: self.incomplete.clone(),
        };
        let mut streams = HashMap::new();
        for (_, params) in probe.contexts.values_mut() {
            params.isolate_random_stream(&mut streams);
        }
        probe
    }
}
