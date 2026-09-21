//! Borrowed rate equations for integrals driven entirely by prescribed time.

use super::*;

pub(crate) struct PrescribedIntegralRate<'a> {
    pub name: String,
    equation: &'a Equation,
    environment: BehavioralEnvironment,
    source_start: usize,
}

impl PrescribedIntegralRate<'_> {
    /// None means that the rate reads a physical circuit coordinate.
    pub(crate) fn dependencies(&self) -> Option<Vec<usize>> {
        self.equation
            .inputs
            .iter()
            .map(|input| match input {
                Input::Integral(index) => Some(self.source_start + index),
                _ => None,
            })
            .collect()
    }

    pub(crate) fn sample(
        &self,
        time: Value,
        integral: impl Fn(usize) -> Value,
    ) -> Result<Value, String> {
        let mut environment = self.environment;
        environment.time = time;
        let (value, _) = self
            .equation
            .evaluate(
                |input| match input {
                    Input::Integral(index) => (integral(self.source_start + index), 0.0.into()),
                    _ => (Value::NAN, 0.0.into()),
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
                )
            }));
        for (name, equations, temperature, frequency, gmin, expression_dialect) in sources {
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
                });
            }
        }
        Ok(plans)
    }
}
