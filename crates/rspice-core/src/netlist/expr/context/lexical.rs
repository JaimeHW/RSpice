//! Completion of inherited parse-time bindings without evaluating local bodies.
use super::*;
use crate::abort_signal::AbortSignal;

impl ParamContext {
    pub(crate) fn has_materialized_binding(&self, name: &str, global: bool) -> bool {
        if global {
            !self.global_expressions.contains_key(name)
                && (self.global_params.contains_key(name)
                    || self.global_string_params.contains_key(name))
        } else {
            !self.parameter_expressions.contains_key(name)
                && (self.params.contains_key(name) || self.string_params.contains_key(name))
        }
    }

    pub(crate) fn owns_parameter_definition(&self, name: &str, global: bool) -> bool {
        let definitions = if global {
            &self.scope_global_parameter_definitions
        } else {
            &self.scope_parameter_definitions
        };
        definitions.contains_key(name)
    }

    pub(crate) fn complete_inherited_bindings(
        &mut self,
        parent: &Self,
        abort: &dyn AbortSignal,
    ) -> Result<(), ParameterResolutionError> {
        for global in [false, true] {
            let (numeric, complex, strings, expressions) = if global {
                (
                    &parent.global_params,
                    &parent.global_complex_params,
                    &parent.global_string_params,
                    &parent.global_expressions,
                )
            } else {
                (
                    &parent.params,
                    &parent.complex_params,
                    &parent.string_params,
                    &parent.parameter_expressions,
                )
            };
            let names: BTreeSet<_> = numeric
                .keys()
                .chain(strings.keys())
                .chain(expressions.keys())
                .collect();
            for name in names {
                if abort.is_aborted() {
                    return Err(ParameterResolutionError::Aborted);
                }
                let already_bound = if global {
                    self.global_params.contains_key(name)
                        || self.global_string_params.contains_key(name)
                } else {
                    self.params.contains_key(name) || self.string_params.contains_key(name)
                };
                if already_bound || self.owns_parameter_definition(name, global) {
                    continue;
                }
                let value = complex
                    .get(name)
                    .copied()
                    .or_else(|| numeric.get(name).copied().map(ComplexValue::from));
                if let Some(expression) = expressions.get(name) {
                    if global {
                        self.define_global_expression(name, expression, value);
                    } else {
                        self.define_parameter_expression(name, expression, value);
                    }
                    if let Some(origin) = parent.expression_origin(name, global) {
                        self.retain_expression_origin(name, global, origin);
                    }
                } else if let Some(value) = value {
                    if global {
                        self.set_global_complex(name, value);
                    } else {
                        self.set_complex(name, value);
                    }
                } else if let Some(value) = strings.get(name) {
                    if global {
                        self.set_global_string(name, value);
                    } else {
                        self.set_string(name, value.clone());
                    }
                }
            }
        }
        for (name, function) in &parent.functions {
            if abort.is_aborted() {
                return Err(ParameterResolutionError::Aborted);
            }
            self.functions
                .entry(name.clone())
                .or_insert_with(|| function.clone());
        }
        Ok(())
    }

    /// Isolate a group of contexts, preserving stream sharing and explicit
    /// reseeds within the group. No live draw counter is advanced by a probe.
    pub(crate) fn isolate_random_stream(&mut self, streams: &mut HashMap<usize, RandomState>) {
        let identity = Arc::as_ptr(&self.random.counter) as usize;
        self.random = streams
            .entry(identity)
            .or_insert_with(|| RandomState {
                seed: self.random.seed,
                counter: Arc::new(AtomicU64::new(self.random.counter.load(Ordering::Relaxed))),
            })
            .clone();
    }
}
