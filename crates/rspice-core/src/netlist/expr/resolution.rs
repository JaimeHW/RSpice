//! Lazy numeric resolution of retained parameter graphs without text expansion.

use super::*;
use crate::abort_signal::AbortSignal;
use std::collections::HashSet;

#[derive(Debug)]
pub(crate) enum ParameterResolutionError {
    Aborted,
    Definition(String),
    Expression(ExprError),
}

impl From<ExprError> for ParameterResolutionError {
    fn from(error: ExprError) -> Self {
        Self::Expression(error)
    }
}

struct PendingParameter {
    namespace: usize,
    name: String,
    program: PreparedExpression,
}

/// A scope owns one cache: a shared static dependency is evaluated once even
/// when several expressions read it. Suspended programs retain lazy branches,
/// complex values, numeric provenance and their exact statistical position.
#[derive(Default)]
pub(crate) struct ParameterResolver {
    values: [HashMap<String, ComplexValue>; 2],
}

impl ParameterResolver {
    pub(crate) fn resolve(
        &mut self,
        name: &str,
        expression: &str,
        params: &ParamContext,
        abort: &dyn AbortSignal,
    ) -> Result<ComplexValue, ParameterResolutionError> {
        self.resolve_in_namespace(0, name, expression, params, abort)
    }

    pub(crate) fn resolve_global(
        &mut self,
        name: &str,
        expression: &str,
        params: &ParamContext,
        abort: &dyn AbortSignal,
    ) -> Result<ComplexValue, ParameterResolutionError> {
        self.resolve_in_namespace(1, name, expression, params, abort)
    }

    pub(crate) fn value(&self, name: &str, params: &ParamContext) -> Option<ComplexValue> {
        self.values[usize::from(!params.has_parameter_binding(name))]
            .get(name)
            .copied()
    }

    /// Publish successfully resolved bindings once. Global expression bodies
    /// remain authoritative; ordinary static definitions become numeric values.
    pub(crate) fn materialize_into(&self, params: &mut ParamContext) {
        for (name, value) in &self.values[0] {
            if params.get_parameter_expression(name).is_some() {
                params.set_complex(name, *value);
            }
        }
        for (name, value) in &self.values[1] {
            if let Some(expression) = params.get_global_expression(name).map(str::to_owned) {
                let origin = params.expression_origin(name, true).cloned();
                params.define_global_expression(name, expression, Some(*value));
                if let Some(origin) = origin {
                    params.retain_expression_origin(name, true, &origin);
                }
            }
        }
    }

    fn resolve_in_namespace(
        &mut self,
        namespace: usize,
        name: &str,
        expression: &str,
        params: &ParamContext,
        abort: &dyn AbortSignal,
    ) -> Result<ComplexValue, ParameterResolutionError> {
        if abort.is_aborted() {
            return Err(ParameterResolutionError::Aborted);
        }
        let name = name.to_ascii_uppercase();
        if let Some(value) = self.values[namespace].get(&name) {
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
            self.values[namespace].insert(name, value);
            return Ok(value);
        }
        let mut stack = vec![Self::pending(namespace, name.clone(), expression, params)?];
        let mut active = HashSet::from([(namespace, name.clone())]);
        while let Some(current) = stack.last_mut() {
            if abort.is_aborted() {
                return Err(ParameterResolutionError::Aborted);
            }
            match current
                .program
                .resume_with(params, &mut |dependency| Ok(self.value(dependency, params)))?
            {
                PreparedProgress::Complete(value) => {
                    let value =
                        if params.expression_dialect() == crate::config::ExpressionDialect::Xyce {
                            normalize_xyce_expression_result(value)
                        } else {
                            value
                        };
                    let current = stack.pop().expect("completed stack entry exists");
                    active.remove(&(current.namespace, current.name.clone()));
                    self.values[current.namespace].insert(current.name, value);
                }
                PreparedProgress::MissingParameter(dependency) => {
                    let namespace = usize::from(!params.has_parameter_binding(&dependency));
                    if active.contains(&(namespace, dependency.clone())) {
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
                    let expression = if params.has_parameter_binding(&dependency) {
                        params.get_parameter_expression(&dependency)
                    } else {
                        params.get_global_expression(&dependency)
                    }
                    .ok_or_else(|| ExprError::UndefinedParam(dependency.clone()))?;
                    let pending = Self::pending(namespace, dependency.clone(), expression, params)?;
                    active.insert((namespace, dependency));
                    stack.push(pending);
                }
            }
        }
        Ok(self.values[namespace][&name])
    }

    fn pending(
        namespace: usize,
        name: String,
        expression: &str,
        params: &ParamContext,
    ) -> Result<PendingParameter, ExprError> {
        let parsed = parse_expression(expression)?;
        let mut program = PreparedExpression::compile(&parsed, params)?;
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
            )?;
        }
        program.begin_evaluation();
        Ok(PendingParameter {
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
