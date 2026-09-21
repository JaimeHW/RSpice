//! Borrowed integral rates with explicit circuit, integral, and phase bindings.

use super::*;

pub(crate) struct PrescribedIntegralRate<'a> {
    pub name: String,
    equation: &'a Equation,
    environment: BehavioralEnvironment,
    source_start: usize,
    node_bindings: &'a [Option<usize>],
    branch_bindings: &'a [Option<usize>],
}

impl PrescribedIntegralRate<'_> {
    /// None means that the rate reads a physical circuit coordinate other than ground.
    pub(crate) fn dependencies(&self) -> Option<Vec<usize>> {
        self.dependencies_with_coordinates(|_| false)
    }

    /// Large-signal preparation can additionally know a coordinate through
    /// exact independent-source constraints. Consumers must use dependencies()
    /// so these physical inputs retain their small-signal rate derivatives.
    pub(crate) fn dependencies_with_coordinates(
        &self,
        known: impl Fn(usize) -> bool,
    ) -> Option<Vec<usize>> {
        self.equation
            .inputs
            .iter()
            .copied()
            .try_fold(Vec::new(), |mut dependencies, input| match input {
                Input::Integral(index) => {
                    dependencies.push(self.source_start + index);
                    Some(dependencies)
                }
                Input::Phase(_) => Some(dependencies),
                Input::Node(index) if self.node_bindings[index].is_none_or(&known) => {
                    Some(dependencies)
                }
                Input::Branch(index) if self.branch_bindings[index].is_some_and(&known) => {
                    Some(dependencies)
                }
                _ => None,
            })
    }

    pub(crate) fn sample_with_coordinates(
        &self,
        time: Value,
        phases: &[Value],
        integral: impl Fn(usize) -> Value,
        coordinate: impl Fn(usize) -> Value,
    ) -> Result<Value, String> {
        let mut environment = self.environment;
        environment.time = time;
        let (value, _) = self
            .equation
            .evaluate(
                |input| match input {
                    Input::Integral(index) => (integral(self.source_start + index), 0.0.into()),
                    Input::Phase(index) => {
                        (phases.get(index).copied().unwrap_or(Value::NAN), 0.0.into())
                    }
                    Input::Node(index) => (
                        self.node_bindings[index].map_or(0.0, &coordinate),
                        0.0.into(),
                    ),
                    Input::Branch(index) => (
                        self.branch_bindings[index].map_or(Value::NAN, &coordinate),
                        0.0.into(),
                    ),
                },
                environment,
            )
            .ok_or_else(|| {
                format!(
                    "prescribed integral '{}' has no analytic rate evaluator",
                    self.name
                )
            })?;
        if !value.is_finite() {
            return Err(format!(
                "prescribed integral '{}' has a non-finite rate at t={time:e}",
                self.name
            ));
        }
        Ok(value)
    }
}

impl BehavioralSources {
    /// Canonical voltage-first compiler-postorder, including coupled rates.
    /// Callers select a rate only after all its integral dependencies qualify.
    pub(crate) fn prescribed_integral_rates(
        &self,
    ) -> Result<Vec<PrescribedIntegralRate<'_>>, String> {
        let mut plans = Vec::new();
        plans
            .try_reserve_exact(self.integral_count())
            .map_err(|error| format!("prescribed integral rate-plan allocation failed: {error}"))?;
        let sources = self
            .voltage_sources
            .iter()
            .map(|s| {
                (
                    &s.name,
                    &s.integral_equations,
                    s.temperature,
                    s.frequency,
                    s.gmin,
                    s.expression_dialect,
                    &s.node_bindings,
                    &s.branch_bindings,
                )
            })
            .chain(self.current_sources.iter().map(|s| {
                (
                    &s.name,
                    &s.integral_equations,
                    s.temperature,
                    s.frequency,
                    s.gmin,
                    s.expression_dialect,
                    &s.node_bindings,
                    &s.branch_bindings,
                )
            }));
        for (
            name,
            equations,
            temperature,
            frequency,
            gmin,
            expression_dialect,
            node_bindings,
            branch_bindings,
        ) in sources
        {
            let Some(equations) = equations else {
                continue;
            };
            let source_start = plans.len();
            for (index, equation) in equations.rates.iter().enumerate() {
                plans.push(PrescribedIntegralRate {
                    name: format!("B:{name}:sdt:{index}"),
                    equation,
                    environment: BehavioralEnvironment {
                        time: 0.0,
                        frequency,
                        temperature,
                        gmin,
                        expression_dialect,
                        logarithm_domain: LogarithmDomain::Ieee,
                    },
                    source_start,
                    node_bindings,
                    branch_bindings,
                });
            }
        }
        Ok(plans)
    }
}
