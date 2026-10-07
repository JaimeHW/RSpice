//! Lazy numeric resolution of retained parameter graphs without text expansion.

use super::*;
use crate::abort_signal::AbortSignal;
use std::collections::HashSet;

#[derive(Debug)]
pub(crate) enum ParameterResolutionError {
    Aborted,
    IncompleteScope(usize),
    Definition(String),
    Expression(ExprError),
}

impl From<ExprError> for ParameterResolutionError {
    fn from(error: ExprError) -> Self {
        Self::Expression(error)
    }
}

impl From<ExpressionEvaluationError> for ParameterResolutionError {
    fn from(error: ExpressionEvaluationError) -> Self {
        match error {
            ExpressionEvaluationError::Aborted => Self::Aborted,
            ExpressionEvaluationError::Expression(error) => Self::Expression(error),
        }
    }
}

/// Supplies lexical declaration ownership to the same numeric graph walker used
/// for a single context. Scope identities are local to one resolver operation.
pub(crate) trait ParameterEnvironment {
    fn parameters(&self, scope: usize) -> &ParamContext;
    fn owner_scope(&self, scope: usize, name: &str, global: bool) -> usize;
    fn ensure_ready(&self, _scope: usize) -> Result<(), ParameterResolutionError> {
        Ok(())
    }
}

impl ParameterEnvironment for ParamContext {
    fn parameters(&self, _scope: usize) -> &ParamContext {
        self
    }
    fn owner_scope(&self, scope: usize, _name: &str, _global: bool) -> usize {
        scope
    }
}

struct PendingParameter {
    scope: usize,
    namespace: usize,
    name: String,
    program: PreparedExpression,
}

/// A scope owns one cache: a shared static dependency is evaluated once even
/// when several expressions read it. Suspended programs retain lazy branches,
/// complex values, numeric provenance and their exact statistical position.
#[derive(Default)]
pub(crate) struct ParameterResolver {
    values: HashMap<usize, [HashMap<String, ComplexValue>; 2]>,
}

impl ParameterResolver {
    pub(crate) fn resolve(
        &mut self,
        name: &str,
        expression: &str,
        params: &ParamContext,
        abort: &dyn AbortSignal,
    ) -> Result<ComplexValue, ParameterResolutionError> {
        self.resolve_scoped(0, false, name, expression, params, abort)
    }

    pub(crate) fn resolve_global(
        &mut self,
        name: &str,
        expression: &str,
        params: &ParamContext,
        abort: &dyn AbortSignal,
    ) -> Result<ComplexValue, ParameterResolutionError> {
        self.resolve_scoped(0, true, name, expression, params, abort)
    }

    pub(crate) fn scoped_value(
        &self,
        scope: usize,
        name: &str,
        environment: &impl ParameterEnvironment,
    ) -> Option<ComplexValue> {
        let global = !environment.parameters(scope).has_parameter_binding(name);
        let owner = environment.owner_scope(scope, name, global);
        self.values.get(&owner)?[usize::from(global)]
            .get(name)
            .copied()
    }

    /// Publish successfully resolved bindings once. Global expression bodies
    /// remain authoritative; ordinary static definitions become numeric values.
    pub(crate) fn materialize_into(&self, params: &mut ParamContext) {
        self.materialize_scope(0, params);
    }

    pub(crate) fn materialize_scope(&self, scope: usize, params: &mut ParamContext) {
        let Some(values) = self.values.get(&scope) else {
            return;
        };
        for (name, value) in &values[0] {
            if params.get_parameter_expression(name).is_some() {
                params.set_complex(name, *value);
            }
        }
        for (name, value) in &values[1] {
            if let Some(expression) = params.get_global_expression(name).map(str::to_owned) {
                let origin = params.expression_origin(name, true).cloned();
                params.define_global_expression(name, expression, Some(*value));
                if let Some(origin) = origin {
                    params.retain_expression_origin(name, true, &origin);
                }
            }
        }
    }

    pub(crate) fn resolve_scoped(
        &mut self,
        scope: usize,
        global: bool,
        name: &str,
        expression: &str,
        environment: &impl ParameterEnvironment,
        abort: &dyn AbortSignal,
    ) -> Result<ComplexValue, ParameterResolutionError> {
        if abort.is_aborted() {
            return Err(ParameterResolutionError::Aborted);
        }
        environment.ensure_ready(scope)?;
        let namespace = usize::from(global);
        let params = environment.parameters(scope);
        let name = name.to_ascii_uppercase();
        if let Some(value) = self
            .values
            .get(&scope)
            .and_then(|values| values[namespace].get(&name))
        {
            return Ok(*value);
        }
        let captured = if namespace == 1 {
            params.get_global_complex(&name)
        } else if params.has_parameter_binding(&name) {
            params.get_complex(&name)
        } else {
            None
        };
        if let Some(value) = captured
            && super::behavioral::captures_static_statistical_value(expression, params)
        {
            self.values.entry(scope).or_default()[namespace].insert(name, value);
            return Ok(value);
        }
        let mut stack = vec![Self::pending(
            scope,
            namespace,
            name.clone(),
            expression,
            params,
            abort,
        )?];
        let mut active = HashSet::from([(scope, namespace, name.clone())]);
        while let Some(current) = stack.last_mut() {
            if abort.is_aborted() {
                return Err(ParameterResolutionError::Aborted);
            }
            let params = environment.parameters(current.scope);
            match current.program.resume_with_abort(
                params,
                &mut |dependency| Ok(self.scoped_value(current.scope, dependency, environment)),
                abort,
            )? {
                PreparedProgress::Complete(value) => {
                    let value =
                        if params.expression_dialect() == crate::config::ExpressionDialect::Xyce {
                            normalize_xyce_expression_result(value)
                        } else {
                            value
                        };
                    let current = stack.pop().expect("completed stack entry exists");
                    active.remove(&(current.scope, current.namespace, current.name.clone()));
                    self.values.entry(current.scope).or_default()[current.namespace]
                        .insert(current.name, value);
                }
                PreparedProgress::MissingParameter(dependency) => {
                    let namespace = usize::from(!params.has_parameter_binding(&dependency));
                    let scope = environment.owner_scope(current.scope, &dependency, namespace == 1);
                    environment.ensure_ready(scope)?;
                    let params = environment.parameters(scope);
                    if active.contains(&(scope, namespace, dependency.clone())) {
                        let mut names = stack
                            .iter()
                            .map(|entry| entry.name.as_str())
                            .collect::<Vec<_>>();
                        names.push(&dependency);
                        return Err(ExprError::InvalidArgument(format!(
                            "Detected cyclic parameter dependency: {}",
                            names.join(" -> ")
                        ))
                        .into());
                    }
                    let expression = if namespace == 0 {
                        params.get_parameter_expression(&dependency)
                    } else {
                        params.get_global_expression(&dependency)
                    };
                    // A child can retain an inherited symbolic copy after its
                    // owner has already materialized the declaration.
                    let Some(expression) = expression else {
                        let value = if namespace == 1 {
                            params.get_global_complex(&dependency)
                        } else if params.has_parameter_binding(&dependency) {
                            params.get_complex(&dependency)
                        } else {
                            None
                        };
                        let value =
                            value.ok_or_else(|| ExprError::UndefinedParam(dependency.clone()))?;
                        self.values.entry(scope).or_default()[namespace].insert(dependency, value);
                        continue;
                    };
                    let pending = Self::pending(
                        scope,
                        namespace,
                        dependency.clone(),
                        expression,
                        params,
                        abort,
                    )?;
                    active.insert((scope, namespace, dependency));
                    stack.push(pending);
                }
            }
        }
        Ok(self.values[&scope][namespace][&name])
    }

    /// Resolve a demanded lexical binding after its declaration scope closes.
    /// Captured numeric values are supplied by the consumer before this path.
    pub(crate) fn resolve_binding(
        &mut self,
        scope: usize,
        name: &str,
        environment: &impl ParameterEnvironment,
        abort: &dyn AbortSignal,
    ) -> Result<ComplexValue, ParameterResolutionError> {
        if abort.is_aborted() {
            return Err(ParameterResolutionError::Aborted);
        }
        let global = !environment.parameters(scope).has_parameter_binding(name);
        let owner = environment.owner_scope(scope, name, global);
        environment.ensure_ready(owner)?;
        let params = environment.parameters(owner);
        let expression = if global {
            params.get_global_expression(name)
        } else {
            params.get_parameter_expression(name)
        };
        if let Some(expression) = expression {
            return self.resolve_scoped(owner, global, name, expression, environment, abort);
        }
        let value = if global {
            params.get_global_complex(name)
        } else if params.has_parameter_binding(name) {
            params.get_complex(name)
        } else {
            None
        }
        .ok_or_else(|| ExprError::UndefinedParam(name.to_owned()))?;
        self.values.entry(owner).or_default()[usize::from(global)].insert(name.to_owned(), value);
        Ok(value)
    }

    fn pending(
        scope: usize,
        namespace: usize,
        name: String,
        expression: &str,
        params: &ParamContext,
        abort: &dyn AbortSignal,
    ) -> Result<PendingParameter, ParameterResolutionError> {
        let parsed = parse_expression_with_abort(expression, abort)
            .map_err(ExpressionEvaluationError::from)?;
        let mut program = PreparedExpression::compile_with_abort(&parsed, params, abort)?;
        let mut authoritative = HashSet::new();
        program.visit_runtime_parameters(|name| {
            let retained = if params.has_parameter_binding(name) {
                params.get_parameter_expression(name)
            } else {
                params.get_global_expression(name)
            };
            if let Some(expression) = retained {
                let captured_statistical_value = params.get_complex(name).is_some()
                    && super::behavioral::captures_static_statistical_value(expression, params);
                if !captured_statistical_value {
                    authoritative.insert(name.to_owned());
                }
            }
        });
        if !authoritative.is_empty() {
            program = PreparedExpression::compile_with_external_parameters(
                &parsed,
                params,
                &authoritative,
                abort,
            )?;
        }
        program.begin_evaluation();
        Ok(PendingParameter {
            scope,
            namespace,
            name,
            program,
        })
    }
}

impl std::fmt::Display for ParameterResolutionError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Aborted => formatter.write_str("parameter resolution was cancelled"),
            Self::IncompleteScope(scope) => {
                write!(formatter, "parameter scope {scope} is still open")
            }
            Self::Definition(message) => formatter.write_str(message),
            Self::Expression(error) => error.fmt(formatter),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::abort_signal::{CountingAbort, NoAbort};

    #[test]
    fn lexical_dependencies_use_their_owner_and_share_samples_between_children() {
        struct Scopes([ParamContext; 3]);
        impl ParameterEnvironment for Scopes {
            fn parameters(&self, scope: usize) -> &ParamContext {
                &self.0[scope]
            }
            fn owner_scope(&self, scope: usize, name: &str, global: bool) -> usize {
                if global || matches!(name, "ALIAS" | "BASE") {
                    0
                } else {
                    scope
                }
            }
        }
        let mut parent = ParamContext::new();
        parent.set_random_seed(73);
        parent.define_parameter_expression("alias", "base+aunif(0,1)", None);
        parent.set("base", 5.0);
        let mut child = parent.clone();
        child.set("base", 100.0);
        child.define_parameter_expression("local", "alias+base", None);
        let sibling = child.clone();
        let scopes = Scopes([parent, child, sibling]);
        let reference = scopes.0[0].isolated_random_clone();
        let sample = eval_expression("aunif(0,1)", &reference).unwrap();
        let mut resolver = ParameterResolver::default();
        for scope in [1, 2] {
            assert_eq!(
                resolver
                    .resolve_scoped(scope, false, "local", "alias+base", &scopes, &NoAbort)
                    .unwrap(),
                ComplexValue::from((5.0 + sample) + 100.0)
            );
        }
        assert_eq!(
            eval_expression("aunif(0,1)", &scopes.0[0]).unwrap(),
            eval_expression("aunif(0,1)", &reference).unwrap()
        );
        let mut materialized = scopes.0[0].clone();
        resolver.materialize_scope(0, &mut materialized);
        assert_eq!(materialized.get("alias"), Some(5.0 + sample));
        assert_eq!(materialized.get("local"), None);
    }

    #[test]
    fn dependency_suspension_keeps_function_frames_complex_values_and_random_order() {
        let mut params = ParamContext::new();
        params.set_random_seed(73);
        params.set_complex("z", ComplexValue::new(3.0, 4.0));
        params.define_parameter_expression("a", "aunif(0,1)+img(z)", None);
        params.define_function("twice", vec!["x".into()], "x+x");
        let mut resolver = ParameterResolver::default();
        let value = resolver
            .resolve("total", "aunif(0,1)+twice(a)+aunif(0,1)", &params, &NoAbort)
            .unwrap();
        let mut reference = ParamContext::new();
        reference.set_random_seed(73);
        let first = eval_expression("aunif(0,1)", &reference).unwrap();
        let dependency = eval_expression("aunif(0,1)", &reference).unwrap() + 4.0;
        let last = eval_expression("aunif(0,1)", &reference).unwrap();
        assert_eq!(
            value,
            ComplexValue::from(first + (dependency + dependency) + last)
        );
        assert_eq!(
            eval_expression("aunif(0,1)", &params).unwrap(),
            eval_expression("aunif(0,1)", &reference).unwrap()
        );
        assert_eq!(
            resolver
                .resolve("a", "aunif(0,1)+img(z)", &params, &NoAbort)
                .unwrap(),
            ComplexValue::from(dependency)
        );
    }

    #[test]
    fn lazy_branches_do_not_resolve_unused_missing_or_cyclic_bindings() {
        let mut params = ParamContext::new();
        params.define_parameter_expression("a", "b", None);
        params.define_parameter_expression("b", "a", None);
        let mut resolver = ParameterResolver::default();
        assert_eq!(
            resolver
                .resolve("selected", "if(1,85,a+missing)", &params, &NoAbort)
                .unwrap(),
            ComplexValue::from(85.0)
        );
        let error = resolver.resolve("a", "b", &params, &NoAbort).unwrap_err();
        assert!(error.to_string().contains("A -> B -> A"), "{error}");
    }

    #[test]
    fn cancellation_stops_between_dependency_resumptions() {
        let mut params = ParamContext::new();
        params.define_parameter_expression("a", "b", None);
        params.define_parameter_expression("b", "85", None);
        let abort = CountingAbort::new(1);
        assert!(matches!(
            ParameterResolver::default().resolve("a", "b", &params, &abort),
            Err(ParameterResolutionError::Aborted)
        ));
        assert_eq!(abort.polls_after_abort(), 0);
    }

    #[test]
    fn retained_globals_are_authoritative_and_function_formals_shadow_bindings() {
        let mut params = ParamContext::new();
        params.set("later", 5.0);
        params.define_global_expression("g", "later", Some(2.0.into()));
        params.define_parameter_expression("x", "g", None);
        params.define_function("twice", vec!["X".into()], "x+x");
        assert_eq!(
            ParameterResolver::default()
                .resolve("total", "twice(3)+x", &params, &NoAbort)
                .unwrap(),
            11.0.into()
        );
        params.define_parameter_expression("g", "later+1", None);
        assert_eq!(
            ParameterResolver::default()
                .resolve_global("g", "g+1", &params, &NoAbort)
                .unwrap(),
            7.0.into()
        );
    }

    #[test]
    fn long_dependency_chains_use_an_explicit_stack() {
        let mut params = ParamContext::new();
        for index in 0..2000 {
            params.define_parameter_expression(
                &format!("p{index}"),
                format!("p{}+1", index + 1),
                None,
            );
        }
        params.set("p2000", 85.0);
        assert_eq!(
            ParameterResolver::default()
                .resolve("p0", "p1+1", &params, &NoAbort)
                .unwrap(),
            ComplexValue::from(2085.0)
        );
    }

    #[test]
    fn global_sample_projection_is_not_read_from_the_ordinary_namespace() {
        let mut params = ParamContext::new();
        params.set("g", 2.0);
        params.define_global_expression("g", "limit(0,1)", Some(7.0.into()));
        let mut resolver = ParameterResolver::default();
        assert_eq!(
            resolver
                .resolve_global("g", "limit(0,1)", &params, &NoAbort)
                .unwrap(),
            7.0.into()
        );
        resolver.materialize_into(&mut params);
        assert_eq!(params.get("g"), Some(2.0));
        assert_eq!(params.get_global_complex("g"), Some(7.0.into()));
        let reference = ParamContext::new();
        assert_eq!(
            eval_expression("aunif(0,1)", &params).unwrap(),
            eval_expression("aunif(0,1)", &reference).unwrap()
        );
    }

    #[test]
    fn sample_detection_observes_function_overloads_formals_and_runtime_bodies() {
        let mut params = ParamContext::new();
        params.define_function("static_sample", vec!["TIME".into()], "time+limit(0,1)");
        params.define_function("live_sample", vec!["X".into()], "x+time+limit(0,1)");
        for (expression, expected) in [
            ("static_sample(2)", true),
            ("live_sample(2)", false),
            ("limit(2,0,1)", false),
            ("limit(2,1)", true),
        ] {
            assert_eq!(
                super::super::behavioral::captures_static_statistical_value(expression, &params),
                expected,
                "{expression}"
            );
        }
        params.define_function("limit", vec!["X".into(), "Y".into()], "x+y");
        assert!(
            !super::super::behavioral::captures_static_statistical_value("limit(2,1)", &params)
        );
    }

    #[test]
    fn a_captured_sample_does_not_hide_a_new_runtime_parameter_dependency() {
        let mut params = ParamContext::new();
        params.define_function("sample", vec!["X".into()], "x+aunif(0,1)");
        params.define_global_expression("sampled", "sample(base)", Some(2.0.into()));
        params.define_global_expression("base", "live", Some(1.0.into()));
        params.define_parameter_expression("live", "1+TIME", None);
        assert!(
            !super::super::behavioral::captures_static_statistical_value("sample(base)", &params)
        );
        let prepared = prepare_behavioral_expression("sampled", &params).unwrap();
        assert!(
            behavioral_expression_references_runtime_quantity(&prepared),
            "{prepared}"
        );
        params.set("base", 3.0);
        assert!(super::super::behavioral::captures_static_statistical_value(
            "sample(base)",
            &params
        ));
    }

    #[test]
    fn available_materialization_shares_complex_global_dependencies_once() {
        let mut params = ParamContext::new();
        params.define_global_expression("z", "aunif(0,1)+2j", None);
        params.define_global_expression("a", "z+z", None);
        params.define_global_expression("b", "img(z)", None);
        let reference = ParamContext::new();
        let z = ComplexValue::new(eval_expression("aunif(0,1)", &reference).unwrap(), 2.0);
        assert_eq!(materialize_available_parameter_expressions(&mut params), 3);
        assert_eq!(params.get_complex("z"), Some(z));
        assert_eq!(params.get_complex("a"), Some(z + z));
        assert_eq!(params.get("b"), Some(2.0));
        assert_eq!(
            eval_expression("aunif(0,1)", &params).unwrap(),
            eval_expression("aunif(0,1)", &reference).unwrap()
        );
    }
}
