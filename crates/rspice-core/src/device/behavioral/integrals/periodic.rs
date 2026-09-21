//! Continuous F/Q equations for explicit integral coordinates. Collocation
//! samples supply the coordinates; no transient time/history update is used.

use super::*;
use crate::device::MatrixStamper;

/// The solver owns physical MNA coordinates followed by integral coordinates.
/// Additional scalar inputs, such as torus phases, are read-only parameters.
#[derive(Clone, Copy)]
pub(crate) struct BehavioralFqPoint<'a> {
    pub inputs: &'a [Value],
    pub time: Value,
    pub num_nodes: usize,
    pub unknowns: usize,
    pub integral_start: usize,
}

struct Sample {
    value: Value,
    partials: Vec<(usize, Value)>,
}

impl Equation {
    fn periodic_sample(
        &self,
        point: BehavioralFqPoint<'_>,
        environment: BehavioralEnvironment,
        binding: impl Fn(Input) -> Option<usize>,
        source_boundary: bool,
    ) -> Result<Sample, String> {
        let bindings = self.inputs.iter().copied().map(binding).collect::<Vec<_>>();
        let values = bindings
            .iter()
            .map(|binding| match binding {
                None => Ok(0.0),
                Some(index) => point.inputs.get(*index).copied().ok_or_else(|| {
                    "periodic expression input is outside the registered basis".to_owned()
                }),
            })
            .collect::<Result<Vec<_>, _>>()?;
        let mut directions = vec![Derivative::from(0.0); values.len()];
        let (value, _) = compiled_expression_node_direction(
            &self.ast,
            &self.program,
            &values,
            &directions,
            environment,
        )
        .ok_or("periodic expression has no analytic F/Q evaluator")?;
        let finish = |value| {
            if source_boundary {
                normalize_expression_boundary(value, environment.expression_dialect)
            } else {
                value
            }
        };
        let value = finish(value);
        if !value.is_finite() {
            return Err("periodic expression has a non-finite value".into());
        }
        let mut columns = bindings
            .iter()
            .flatten()
            .copied()
            .filter(|index| *index < point.unknowns)
            .collect::<Vec<_>>();
        columns.sort_unstable();
        columns.dedup();
        let mut partials = Vec::with_capacity(columns.len());
        for column in columns {
            for (direction, binding) in directions.iter_mut().zip(&bindings) {
                *direction = Value::from(*binding == Some(column)).into();
            }
            let (_, derivative) = compiled_expression_node_direction(
                &self.ast,
                &self.program,
                &values,
                &directions,
                environment,
            )
            .ok_or("periodic expression has no analytic F/Q derivative")?;
            let derivative = finish(derivative.binary64());
            if !derivative.is_finite() {
                return Err("periodic expression has a non-finite F/Q derivative".into());
            }
            partials.push((column, derivative));
        }
        Ok(Sample { value, partials })
    }
}

macro_rules! sample_source {
    ($kind:ty) => {
        impl $kind {
            /// A constant nonzero rate has no periodic primitive, even when
            /// it is smaller than the nonlinear residual tolerance. Establish
            /// this from the expression, not from aliased waveform samples.
            pub(crate) fn validate_periodic_integral_rates(&self) -> Result<(), String> {
                let Some(equations) = &self.integral_equations else { return Ok(()); };
                let context = self.periodicity_context();
                for (index, equation) in equations.rates.iter().enumerate() {
                    if equation.inputs.is_empty()
                        && let Some(value) = crate::expr::constant_value(&equation.ast, &context)
                        && value != 0.0
                    {
                        return Err(format!(
                            "behavioral source '{}' SDT {index} has constant nonzero input {value:e}; its integral cannot be periodic",
                            self.name,
                        ));
                    }
                }
                Ok(())
            }

            fn periodic_fq_sample(
                &mut self,
                point: BehavioralFqPoint<'_>,
                state_start: usize,
                f: &mut impl MatrixStamper,
                q: &mut impl MatrixStamper,
            ) -> Result<Sample, String> {
                let Some(equations) = &self.integral_equations else {
                    self.linearize_at_time(point.inputs, point.time)
                        .map_err(|error| error.to_string())?;
                    return Ok(Sample {
                        value: self
                            .evaluate(point.inputs, point.time)
                            .map_err(|error| error.to_string())?,
                        partials: self
                            .linearized_partials()
                            .filter(|(column, _)| *column < point.unknowns)
                            .collect(),
                    });
                };
                let environment = BehavioralEnvironment {
                    time: point.time,
                    frequency: self.frequency,
                    temperature: self.temperature,
                    gmin: self.gmin,
                    expression_dialect: self.expression_dialect,
                    logarithm_domain: LogarithmDomain::Ieee,
                };
                let binding = |input| match input {
                    Input::Node(index) => self.node_bindings[index],
                    Input::Branch(index) => self.branch_bindings[index],
                    Input::Integral(index) => Some(state_start + index),
                };
                for (index, equation) in equations.rates.iter().enumerate() {
                    let sample = equation
                        .periodic_sample(point, environment, binding, false)
                        .map_err(|error| {
                            format!("behavioral source '{}' SDT {index}: {error}", self.name)
                        })?;
                    let state = state_start + index;
                    // dz/dt = input: F = -input and Q = z. MatrixStamper's
                    // RHS holds source-minus-F/Q; its Jacobians hold dF/dx,dQ/dx.
                    f.stamp_rhs(state + 1, sample.value);
                    for (column, partial) in sample.partials {
                        f.stamp(state + 1, column + 1, -partial);
                    }
                    q.stamp_rhs(state + 1, -point.inputs[state]);
                    q.stamp(state + 1, state + 1, 1.0);
                }
                equations
                    .output
                    .periodic_sample(point, environment, binding, true)
                    .map_err(|error| format!("behavioral source '{}': {error}", self.name))
            }
        }
    };
}
sample_source!(BehavioralVoltageSource);
sample_source!(BehavioralCurrentSource);

impl BehavioralCurrentSource {
    /// Lead-current observation uses the same explicit state as the residual.
    pub(crate) fn evaluate_periodic_output(
        &mut self,
        point: BehavioralFqPoint<'_>,
        state_start: usize,
    ) -> Result<Value, String> {
        let Some(equations) = &self.integral_equations else {
            return self
                .evaluate(point.inputs, point.time)
                .map_err(|error| error.to_string());
        };
        if state_start
            .checked_add(self.program.sdt_count)
            .is_none_or(|end| end > point.unknowns)
            || point.unknowns > point.inputs.len()
        {
            return Err(format!(
                "behavioral current source '{}' has an incomplete periodic integral basis",
                self.name
            ));
        }
        let environment = BehavioralEnvironment {
            time: point.time,
            frequency: self.frequency,
            temperature: self.temperature,
            gmin: self.gmin,
            expression_dialect: self.expression_dialect,
            logarithm_domain: LogarithmDomain::Ieee,
        };
        equations
            .output
            .periodic_sample(
                point,
                environment,
                |input| match input {
                    Input::Node(index) => self.node_bindings[index],
                    Input::Branch(index) => self.branch_bindings[index],
                    Input::Integral(index) => Some(state_start + index),
                },
                true,
            )
            .map(|sample| sample.value)
            .map_err(|error| format!("behavioral current source '{}': {error}", self.name))
    }
}

impl BehavioralSources {
    /// Stamp continuous source equations and each explicitly allocated SDT
    /// coordinate. The caller must allocate every coordinate; neither a zero
    /// integration constant nor a zero mean is inferred from a periodic input.
    pub(crate) fn stamp_periodic_fq(
        &mut self,
        point: BehavioralFqPoint<'_>,
        f: &mut impl MatrixStamper,
        q: &mut impl MatrixStamper,
    ) -> Result<(), String> {
        if !point.time.is_finite()
            || point.num_nodes > point.integral_start
            || point.integral_start.checked_add(self.integral_count()) != Some(point.unknowns)
            || point.unknowns > point.inputs.len()
            || point.inputs.iter().any(|value| !value.is_finite())
        {
            return Err(
                "behavioral periodic sample does not match its explicit integral basis".into(),
            );
        }
        // Preserve the established physical contribution order. Integral
        // identity remains voltage sources first, then current sources.
        let mut state_start = point.integral_start
            + self
                .voltage_sources
                .iter()
                .map(|source| source.program.sdt_count)
                .sum::<usize>();
        for source in &mut self.current_sources {
            let sample = source.periodic_fq_sample(point, state_start, f, q)?;
            state_start += source.program.sdt_count;
            f.stamp_rhs(source.node_pos, -sample.value);
            f.stamp_rhs(source.node_neg, sample.value);
            for (column, partial) in sample.partials {
                f.stamp(source.node_pos, column + 1, partial);
                f.stamp(source.node_neg, column + 1, -partial);
            }
        }
        state_start = point.integral_start;
        for source in &mut self.voltage_sources {
            let sample = source.periodic_fq_sample(point, state_start, f, q)?;
            state_start += source.program.sdt_count;
            let row = point.num_nodes + source.branch_ordinal;
            // The linear branch owns V(pos)-V(neg), this owns -expression.
            f.stamp_rhs(row, sample.value);
            for (column, partial) in sample.partials {
                f.stamp(row, column + 1, -partial);
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Stamp {
        rhs: Vec<Value>,
        jacobian: Vec<Vec<Value>>,
    }
    impl Stamp {
        fn new(size: usize) -> Self {
            Self {
                rhs: vec![0.0; size],
                jacobian: vec![vec![0.0; size]; size],
            }
        }
    }
    impl MatrixStamper for Stamp {
        fn stamp(&mut self, row: usize, column: usize, value: Value) {
            if row != 0 && column != 0 {
                self.jacobian[row - 1][column - 1] += value;
            }
        }
        fn stamp_rhs(&mut self, row: usize, value: Value) {
            if row != 0 {
                self.rhs[row - 1] += value;
            }
        }
    }

    #[test]
    fn explicit_integral_fq_retains_nested_coordinates_and_physical_jacobians() {
        let mut voltage = BehavioralVoltageSource::new(
            "bv".into(),
            2,
            0,
            2,
            ".3+2*sdt(v(in)+v(alias)-v(out))+4*sdt(sdt(i(vsense)))+v(phase)",
        )
        .unwrap();
        voltage
            .bind_references(
                |name| {
                    Some(match name {
                        "in" | "alias" => 1,
                        "out" => 2,
                        "phase" => 9,
                        _ => unreachable!(),
                    })
                },
                |_| BehavioralBranchResolution::Branch(2),
            )
            .unwrap();
        let mut current =
            BehavioralCurrentSource::new("bi".into(), 2, 0, "5*sdt(v(in)*v(out))").unwrap();
        current
            .bind_references(
                |name| Some(if name == "in" { 1 } else { 2 }),
                |_| BehavioralBranchResolution::MissingDevice,
            )
            .unwrap();
        let mut sources = BehavioralSources {
            voltage_sources: vec![voltage],
            current_sources: vec![current],
        };
        sources.reset_integrals(&[100.0; 4]).unwrap();
        let accepted = sources.accepted_history();
        // Physical coordinates: in, out, I(vsense), I(bv). Four explicit
        // integral states follow. The last input is a read-only torus phase.
        let values = [2.0, 3.0, 4.0, 5.0, 0.1, 0.2, 0.3, 0.4, 7.0];
        for time in [0.0, 17.0] {
            let mut f = Stamp::new(8);
            let mut q = Stamp::new(8);
            let point = BehavioralFqPoint {
                inputs: &values,
                time,
                num_nodes: 2,
                unknowns: 8,
                integral_start: 4,
            };
            sources.stamp_periodic_fq(point, &mut f, &mut q).unwrap();
            for (actual, expected) in f.rhs.iter().zip([0.0, -2.0, 0.0, 8.7, 1.0, 4.0, 0.2, 6.0]) {
                assert!((actual - expected).abs() < 2e-15);
            }
            assert_eq!(q.rhs, [0.0, 0.0, 0.0, 0.0, -0.1, -0.2, -0.3, -0.4]);
            let mut expected_f = vec![vec![0.0; 8]; 8];
            for (row, col, value) in [
                (1, 7, 5.0),
                (3, 4, -2.0),
                (3, 6, -4.0),
                (4, 0, -2.0),
                (4, 1, 1.0),
                (5, 2, -1.0),
                (6, 5, -1.0),
                (7, 0, -3.0),
                (7, 1, -2.0),
            ] {
                expected_f[row][col] = value;
            }
            assert_eq!(f.jacobian, expected_f);
            for row in 0..8 {
                for col in 0..8 {
                    assert_eq!(q.jacobian[row][col], Value::from(row >= 4 && row == col));
                }
            }
            assert_eq!(sources.accepted_history(), accepted);
            let mut invalid = point;
            invalid.integral_start = 3;
            assert!(sources.stamp_periodic_fq(invalid, &mut f, &mut q).is_err());
        }
    }
}
