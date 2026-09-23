//! Method-of-steps shooting coordinates on each line's full physical delay.
use super::*;

#[derive(Debug, Clone, Default)]
pub(in crate::engine::pss) struct PssDelayBasis {
    lines: Vec<LineCoordinates>,
    dimension: usize,
}

#[derive(Debug, Clone)]
struct LineCoordinates {
    index: usize,
    intervals: usize,
    delay: Value,
}

impl LineCoordinates {
    fn offset(&self, knot: usize) -> Value {
        if knot == self.intervals {
            0.0
        } else {
            -self.delay * ((self.intervals - knot) as Value / self.intervals as Value)
        }
    }
}

impl PssDelayBasis {
    fn push(
        &mut self,
        line: LineCoordinates,
        limits: crate::ResourceLimits,
    ) -> Result<(), SimulationError> {
        self.dimension = self
            .dimension
            .saturating_add(2usize.saturating_mul(line.intervals.saturating_add(1)));
        crate::ResourceLimitError::ensure(
            crate::ResourceKind::AnalysisPoints,
            self.dimension,
            limits.max_analysis_points,
        )?;
        crate::ResourceLimitError::ensure(
            crate::ResourceKind::ResultValues,
            self.dimension
                .saturating_mul(self.dimension)
                .saturating_mul(4),
            limits.max_result_values,
        )?;
        let mut previous = line.offset(0);
        for knot in 1..=line.intervals {
            let time = line.offset(knot);
            if !time.is_finite() || time <= previous {
                return Err(SimulationError::Circuit(
                    "PSS delay-state knot times cannot be represented distinctly".into(),
                ));
            }
            previous = time;
        }
        self.lines.push(line);
        Ok(())
    }

    pub(super) fn new(
        circuit: &CircuitData,
        period: Value,
        steps: usize,
        limits: crate::ResourceLimits,
        abort: &dyn AbortSignal,
    ) -> Result<Self, SimulationError> {
        if !period.is_finite() || period <= 0.0 || steps == 0 {
            return Err(SimulationError::Circuit(
                "PSS delay-state grid requires a finite positive period and a nonzero step count"
                    .into(),
            ));
        }
        let mut basis = Self::default();
        for (index, line) in circuit.tlines.iter().enumerate() {
            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            if line.is_memoryless_two_port() {
                continue;
            }
            let count = (line.delay() / period * steps as Value).ceil().max(8.0);
            if !count.is_finite() || count >= (usize::MAX / 2 - 1) as Value {
                return Err(SimulationError::Circuit(format!(
                    "PSS line '{}' requires an unrepresentable delay-state grid",
                    line.name
                )));
            }
            let intervals = count as usize;
            basis.push(
                LineCoordinates {
                    index,
                    intervals,
                    delay: line.delay(),
                },
                limits,
            )?;
        }
        Ok(basis)
    }

    pub(in crate::engine::pss) fn refined(
        &self,
        limits: crate::ResourceLimits,
    ) -> Result<Self, SimulationError> {
        let mut basis = Self::default();
        for coordinates in &self.lines {
            let intervals = coordinates.intervals.checked_mul(2).ok_or_else(|| {
                SimulationError::Circuit("PSS delay-state refinement overflowed".into())
            })?;
            basis.push(
                LineCoordinates {
                    intervals,
                    ..coordinates.clone()
                },
                limits,
            )?;
        }
        Ok(basis)
    }

    /// The authenticated retained basis names carry the selected history grid.
    /// It can be finer than the initial time mesh, so reconstruct its actual
    /// knot count rather than guessing it from the configured carrier period.
    fn from_names(
        circuit: &CircuitData,
        names: &[String],
        limits: crate::ResourceLimits,
    ) -> Result<Self, SimulationError> {
        let mut basis = Self::default();
        for (index, line) in circuit.tlines.iter().enumerate() {
            if line.is_memoryless_two_port() {
                continue;
            }
            let prefix = format!("W1:{}[0/", line.name);
            let intervals = names
                .get(basis.dimension)
                .and_then(|name| name.strip_prefix(&prefix))
                .and_then(|name| name.strip_suffix(']'))
                .and_then(|count| count.parse::<usize>().ok())
                .filter(|count| *count >= 8)
                .ok_or_else(|| {
                    SimulationError::Circuit(format!(
                        "retained PSS delay-state grid does not match line '{}'",
                        line.name
                    ))
                })?;
            basis.push(
                LineCoordinates {
                    index,
                    intervals,
                    delay: line.delay(),
                },
                limits,
            )?;
        }
        if basis.names(circuit) != names {
            return Err(SimulationError::Circuit(
                "retained PSS delay-state coordinates are inconsistent".into(),
            ));
        }
        Ok(basis)
    }

    pub(super) fn dimension(&self) -> usize {
        self.dimension
    }

    pub(super) fn names(&self, circuit: &CircuitData) -> Vec<String> {
        self.lines
            .iter()
            .flat_map(|coordinates| {
                (0..=coordinates.intervals).flat_map(move |knot| {
                    [1, 2].map(|port| {
                        format!(
                            "W{port}:{}[{knot}/{}]",
                            circuit.tlines[coordinates.index].name, coordinates.intervals
                        )
                    })
                })
            })
            .collect()
    }

    pub(super) fn extract(&self, circuit: &CircuitData) -> Vec<Value> {
        self.lines
            .iter()
            .flat_map(|coordinates| {
                let line = &circuit.tlines[coordinates.index];
                (0..=coordinates.intervals).flat_map(move |knot| {
                    let time = line.accepted_history_time() + coordinates.offset(knot);
                    [
                        line.lossless_wave_at(time, true),
                        line.lossless_wave_at(time, false),
                    ]
                })
            })
            .collect()
    }

    pub(super) fn set(
        &self,
        circuit: &mut CircuitData,
        values: &[Value],
    ) -> Result<(), SimulationError> {
        if values.len() != self.dimension || values.iter().any(|value| !value.is_finite()) {
            return Err(SimulationError::Circuit(
                "PSS delay state has an invalid shape or value".into(),
            ));
        }
        let mut cursor = 0;
        for coordinates in &self.lines {
            let line = &mut circuit.tlines[coordinates.index];
            line.reset();
            for knot in 0..=coordinates.intervals {
                // Only the outgoing waves are independent DDE coordinates.
                // This canonical V=wave/I=0 embedding supplies the native
                // interpolator; accepted future samples carry actual V and I.
                line.update_history(
                    coordinates.offset(knot),
                    values[cursor],
                    0.0,
                    values[cursor + 1],
                    0.0,
                );
                cursor += 2;
            }
        }
        Ok(())
    }
}

impl PssCircuit {
    pub(in crate::engine) fn restore_delay_basis(
        &mut self,
        names: &[String],
        limits: crate::ResourceLimits,
    ) -> Result<(), SimulationError> {
        let prefix = self.physical_state_dimension()
            + self.behavioral_sources.integral_count()
            + self.capacitors.integral_count();
        let names = names.get(prefix..).ok_or_else(|| {
            SimulationError::Circuit(
                "retained PSS basis omitted physical/integral coordinates".into(),
            )
        })?;
        self.delay_basis = PssDelayBasis::from_names(&self.circuit, names, limits)?;
        Ok(())
    }

    pub(in crate::engine) fn configure_delay_basis(
        &mut self,
        period: Value,
        steps: usize,
        limits: crate::ResourceLimits,
        abort: &dyn AbortSignal,
    ) -> Result<(), SimulationError> {
        self.delay_basis = PssDelayBasis::new(&self.circuit, period, steps, limits, abort)?;
        Ok(())
    }

    pub(in crate::engine::pss) fn accept_delay_history(&mut self, solution: &[Value], time: Value) {
        for line in &mut self.circuit.tlines {
            if line.is_memoryless_two_port() {
                continue;
            }
            let node = |node: usize| if node == 0 { 0.0 } else { solution[node - 1] };
            let voltage = |positive, negative| node(positive) - node(negative);
            let v1 = voltage(line.node1_pos, line.node1_neg);
            let v2 = voltage(line.node2_pos, line.node2_neg);
            let (i1, i2) = if let Some((one, two)) = line.ltra_branch_matrix_indices() {
                (solution[one - 1], solution[two - 1])
            } else {
                line.transient_port_response(time).port_currents(v1, v2)
            };
            line.update_history(time, v1, i1, v2, i2);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::NoAbort;

    #[test]
    fn pss_delay_state_preserves_nonperiodic_memory_and_worker_isolation() {
        let engine = Engine::new(Default::default());
        let netlist = Netlist::parse(
            "Delay state\nVIN in 0 SIN(0.3 0.8 1)\nRS in near 50\nT1 near 0 far 0 Z0=50 TD=2\nRL far 0 50\n.end\n"
        ).unwrap();
        let data = engine.build_circuit(&netlist).unwrap();
        let mut circuit = PssCircuit::new(data).unwrap();
        circuit
            .configure_delay_basis(1.0, 8, Default::default(), &NoAbort)
            .unwrap();
        assert_eq!(circuit.state_dimension(), 34);
        let values: Vec<_> = (0..17)
            .flat_map(|k| [1.0 + 0.01 * (k * k) as Value, -0.4 + 0.07 * k as Value])
            .collect();
        circuit.set_state(&values).unwrap();
        assert_eq!(circuit.extract_state(), values);
        let mut worker = circuit.clone();
        worker.set_state(&vec![0.0; 34]).unwrap();
        assert_eq!(circuit.extract_state(), values);
        assert_eq!(circuit.perturbation_scale(33, 0.0, 1e-6, 1e-8), 1.0);

        let mut matrix = engine.build_matrix(&circuit).unwrap();
        circuit.link_indices(&matrix);
        let seed = engine
            .pss_initial_node_solution(&mut circuit, &NoAbort)
            .unwrap();
        engine
            .pss_run_tran_internal(
                &mut circuit,
                &mut matrix,
                seed,
                PssTraversal {
                    tstop: 1.0,
                    max_step: 0.125,
                    fixed_grid: true,
                    integration_method: None,
                    retain_waveform: false,
                },
                None,
                &NoAbort,
            )
            .unwrap();
        let final_state = circuit.extract_state();
        // The first half of a two-period delay is still the independently
        // supplied history. Wrapping one carrier cycle would fail this check.
        for k in 0..=8 {
            assert_eq!(final_state[2 * k], values[2 * (k + 8)]);
            assert_eq!(final_state[2 * k + 1], values[2 * (k + 8) + 1]);
        }
        for k in 9..=16 {
            let time = (k - 8) as Value / 8.0;
            let expected = 0.3 + 0.8 * (std::f64::consts::TAU * time).sin();
            assert!((final_state[2 * k] - expected).abs() < 1e-8);
            assert!(final_state[2 * k + 1].abs() < 1e-8);
        }
        let reference = circuit.clone();
        circuit.tlines[0].rebase_lossless_history(0.0).unwrap();
        assert_eq!(circuit.extract_state(), final_state);
        for time in [0.125, 0.375, 0.875] {
            let original = reference.tlines[0].transient_port_response(time + 1.0);
            let rebased = circuit.tlines[0].transient_port_response(time);
            assert_eq!(original.i_eq_port1(), rebased.i_eq_port1());
            assert_eq!(original.i_eq_port2(), rebased.i_eq_port2());
        }
        circuit.set_state(&values).unwrap();
        circuit.delay_basis = circuit.delay_basis.refined(Default::default()).unwrap();
        let refined = circuit.extract_state();
        for knot in 0..17 {
            assert_eq!(
                &refined[4 * knot..4 * knot + 2],
                &values[2 * knot..2 * knot + 2]
            );
        }
        let names = circuit.state_basis_names();
        worker
            .restore_delay_basis(&names, Default::default())
            .unwrap();
        assert_eq!(worker.state_basis_names(), names);
        worker.set_state(&refined).unwrap();
        assert_eq!(worker.extract_state(), refined);
        let mut malformed = names;
        *malformed.last_mut().unwrap() = "W2:T1[wrong]".into();
        assert!(
            worker
                .restore_delay_basis(&malformed, Default::default())
                .is_err()
        );
        let mut limits = crate::ResourceLimits::default();
        limits.max_result_values = 100;
        assert!(circuit.delay_basis.refined(limits).is_err());
        assert!(
            circuit
                .configure_delay_basis(0.0, 8, Default::default(), &NoAbort)
                .is_err()
        );
    }
}
