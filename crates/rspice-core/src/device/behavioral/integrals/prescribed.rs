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
    pub(crate) fn coordinates(&self) -> impl Iterator<Item = usize> + '_ {
        self.equation
            .inputs
            .iter()
            .filter_map(|input| match *input {
                Input::Node(index) => self.node_bindings[index],
                Input::Branch(index) => self.branch_bindings[index],
                _ => None,
            })
    }

    /// None means that the rate reads a physical circuit coordinate other than ground.
    pub(crate) fn dependencies(&self) -> Option<Vec<usize>> {
        self.dependencies_with_coordinates(|_| false)
    }

    /// Large-signal preparation can additionally know a coordinate through
    /// independent circuit equations. Consumers must use dependencies()
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

    /// Coordinates in the same group have a shared unknown voltage offset.
    /// Only an exact cancellation permits sampling them relative to a local
    /// reference. This changes neither the original circuit equations nor
    /// the physical derivatives used by retained small-signal consumers.
    pub(crate) fn dependencies_with_offsets(
        &self,
        group: impl Fn(usize) -> Option<usize>,
    ) -> Option<Vec<usize>> {
        if let Some(dependencies) = self.dependencies_with_coordinates(|i| group(i).is_none()) {
            return Some(dependencies);
        }
        if !super::affine::invariant(&self.equation.ast, |name| {
            let input = self.equation.inputs[*self.equation.program.node_map.get(name)?];
            match input {
                Input::Node(index) => Some(self.node_bindings[index].and_then(&group)),
                Input::Branch(index) => Some(group(self.branch_bindings[index]?)),
                Input::Integral(_) | Input::Phase(_) => Some(None),
            }
        }) {
            return None;
        }
        Some(
            self.equation
                .inputs
                .iter()
                .filter_map(|input| {
                    if let Input::Integral(index) = input {
                        Some(self.source_start + index)
                    } else {
                        None
                    }
                })
                .collect(),
        )
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn integral_offset_proof_distinguishes_exact_cancellation_from_small_dependence() {
        for (rate, independent) in [
            ("v(a)-v(b)", true),
            (".1*v(a)-.1*v(b)", true),
            ("v(a)/3-v(b)/3", true),
            ("sin(v(a)-v(b))+(v(a)-v(b))^3", true),
            ("v(a)-v(c)", false),
            ("sin(v(a))-sin(v(b))", false),
            ("v(a)-1.0000000000000002*v(b)", false),
            ("(1e20*v(a)+v(a))-1e20*v(b)", false),
            ("v(a)-v(b)+1e-300*v(c)", false),
        ] {
            let mut source =
                BehavioralVoltageSource::new("B".into(), 4, 0, 1, &format!("sdt({rate})")).unwrap();
            source
                .bind_references(
                    |name| {
                        Some(match name {
                            "a" => 1,
                            "b" => 2,
                            _ => 3,
                        })
                    },
                    |_| BehavioralBranchResolution::MissingDevice,
                )
                .unwrap();
            let sources = BehavioralSources {
                voltage_sources: vec![source],
                current_sources: vec![],
            };
            let plans = sources.prescribed_integral_rates().unwrap();
            assert_eq!(
                plans[0]
                    .dependencies_with_offsets(|i| Some(if i < 2 { 1 } else { 3 }))
                    .is_some(),
                independent,
                "{rate}"
            );
            assert!(
                plans[0].dependencies().is_none(),
                "physical derivatives are not frozen"
            );
        }
    }
}
