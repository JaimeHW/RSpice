//! Recover behavioral memory on a retained accepted-point trajectory.

use super::*;
use std::collections::HashMap;

fn ordered_waveforms<'a>(
    expected: &[String],
    waveforms: impl IntoIterator<Item = (&'a str, &'a [Value])>,
    points: usize,
    kind: &str,
) -> Result<Vec<&'a [Value]>, SimulationError> {
    let mut indexed = HashMap::new();
    for (name, values) in waveforms {
        if indexed.insert(name.to_ascii_uppercase(), values).is_some() {
            return Err(SimulationError::Circuit(format!(
                "behavioral integral replay contains duplicate {kind} '{name}'"
            )));
        }
    }
    expected.iter().map(|name| {
        let values = indexed.get(&name.to_ascii_uppercase()).copied().ok_or_else(|| {
            SimulationError::Circuit(format!("behavioral integral replay has no {kind} waveform for '{name}'"))
        })?;
        if values.len() != points {
            return Err(SimulationError::Circuit(format!(
                "behavioral integral replay {kind} '{name}' has {} samples for {points} accepted times", values.len()
            )));
        }
        Ok(values)
    }).collect()
}

impl Engine {
    /// Replay only the expression integrators. Physical node/branch values are
    /// taken from every original accepted point, never resampled or re-solved.
    /// This uses the same VM, nested operator order and accepted-time arithmetic
    /// as the producer, starting from its actual integration constants.
    #[allow(clippy::too_many_arguments)]
    pub(in crate::engine) fn hb_replay_integral_samples<'a>(
        &self,
        circuit: &CircuitData,
        initial_integrals: &[Value],
        times: &[Value],
        nodes: impl IntoIterator<Item = (&'a str, &'a [Value])>,
        branches: impl IntoIterator<Item = (&'a str, &'a [Value])>,
        abort: &dyn AbortSignal,
    ) -> Result<Vec<Vec<Value>>, SimulationError> {
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        let source_integrals = circuit.behavioral_sources.integral_count();
        let integral_count = source_integrals + circuit.capacitors.integral_count();
        if initial_integrals.len() != integral_count
            || initial_integrals.iter().any(|value| !value.is_finite())
        {
            return Err(SimulationError::Circuit(
                "behavioral integral replay has invalid initial coordinates".to_owned(),
            ));
        }
        if integral_count == 0 {
            return Ok(Vec::new());
        }
        if times.first().copied() != Some(0.0)
            || times.iter().any(|time| !time.is_finite())
            || times.windows(2).any(|pair| pair[0] >= pair[1])
        {
            return Err(SimulationError::Circuit(
                "behavioral integral replay requires the complete accepted grid starting at zero"
                    .to_owned(),
            ));
        }
        self.ensure_analysis_points(times.len())?;
        // Include the retained physical trajectory and the additional memory
        // rows in the budget before cloning VMs or allocating those rows.
        self.ensure_result_shape(
            times.len(),
            circuit
                .matrix_size()
                .saturating_add(integral_count)
                .saturating_add(1),
        )?;
        let mut rows = (0..integral_count).map(|_| Vec::new()).collect::<Vec<_>>();
        for row in &mut rows {
            row.try_reserve_exact(times.len()).map_err(|_| {
                SimulationError::Circuit("behavioral integral replay allocation failed".to_owned())
            })?;
        }
        let nodes = ordered_waveforms(
            &self.hb_build_node_names(circuit, circuit.num_nodes()),
            nodes,
            times.len(),
            "node",
        )?;
        let branches = ordered_waveforms(
            &circuit.branch_names_sorted(),
            branches,
            times.len(),
            "branch",
        )?;
        let waveforms = nodes.into_iter().chain(branches).collect::<Vec<_>>();
        let mut solution = vec![0.0; circuit.matrix_size()];
        if waveforms.len() != solution.len() {
            return Err(SimulationError::Circuit(
                "behavioral integral replay MNA basis is incomplete".to_owned(),
            ));
        }
        let mut sources = circuit.behavioral_sources.clone();
        sources
            .voltage_sources
            .retain(|source| source.program.sdt_count != 0);
        sources
            .current_sources
            .retain(|source| source.program.sdt_count != 0);
        sources
            .reset_integrals(&initial_integrals[..source_integrals])
            .map_err(SimulationError::Circuit)?;
        let mut capacitors = circuit.capacitors.clone();
        capacitors
            .reset_integrals(&initial_integrals[source_integrals..])
            .map_err(SimulationError::Circuit)?;
        for (sample, &time) in times.iter().enumerate() {
            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            for (value, waveform) in solution.iter_mut().zip(&waveforms) {
                *value = waveform[sample];
                if !value.is_finite() {
                    return Err(SimulationError::Circuit(
                        "behavioral integral replay contains a non-finite physical sample"
                            .to_owned(),
                    ));
                }
            }
            sources
                .accept_transient_step(&solution, time)
                .map_err(|error| SimulationError::Circuit(error.to_string()))?;
            for expression in capacitors
                .value_expressions
                .iter_mut()
                .flatten()
                .filter(|expression| expression.program.sdt_count != 0)
            {
                expression.accept_transient_step(&solution, time);
            }
            for (row, value) in rows.iter_mut().zip(
                sources
                    .accepted_integrals()
                    .chain(capacitors.accepted_integrals()),
            ) {
                if !value.is_finite() {
                    return Err(SimulationError::Circuit(
                        "behavioral integral replay produced a non-finite coordinate".to_owned(),
                    ));
                }
                row.push(value);
            }
        }
        Ok(rows)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tahb_projects_accepted_integral_history_into_the_separate_tail() {
        for rate in [1.0e3, 1.0e9] {
            let engine = Engine::default();
            let netlist = Netlist::parse(&format!(
                "Integral transient seed\nvin in 0 1\nrin in 0 1k\n\
                 bfirst out 0 v={rate}*sdt(v(in))\nrout out 0 1k\n\
                 bnested nested 0 v={rate}*{rate}*sdt(sdt(-1000*i(vin)))\nrnested nested 0 1k\n\
                 bcur sink 0 i={rate}*.001*sdt(v(in))\nrsink sink 0 1k\n.save v(out)\n.end\n"
            ))
            .unwrap();
            let circuit = engine.build_circuit(&netlist).unwrap();
            let nodes = engine.hb_build_node_names(&circuit, circuit.num_nodes());
            let mut branches = circuit.branch_names_sorted();
            let physical = branches.len();
            branches.extend(circuit.behavioral_sources.integral_names());
            let config = HbConfig::new(rate)
                .with_harmonics(3)
                .with_collocation_points(17);
            let mut state = HbSolverState::new(nodes.len(), config.num_harmonics);
            state
                .try_prepare_mna_branches(branches.len(), config.num_harmonics)
                .unwrap();
            engine
                .hb_seed_transient_assisted(
                    &netlist, &config, &mut state, &nodes, &branches, &NoAbort,
                )
                .unwrap();
            let n = config.checked_fft_size().unwrap() as Value;
            for row in [0, 1, 3] {
                assert!(
                    (rate * state.mna_branch_currents[physical + row][0].re
                        - (n - 1.0) / (2.0 * n))
                        .abs()
                        < 1.0e-10
                );
            }
            let quadratic_mean = (n - 1.0) * (2.0 * n - 1.0) / (12.0 * n * n);
            assert!(
                (rate * rate * state.mna_branch_currents[physical + 2][0].re - quadratic_mean)
                    .abs()
                    < 3.0e-5
            );
            for (node, row, scale) in [
                ("out", 0, rate),
                ("nested", 2, rate * rate),
                ("sink", 3, -rate),
            ] {
                let voltage = &state.x[nodes
                    .iter()
                    .position(|name| name.eq_ignore_ascii_case(node))
                    .unwrap()];
                for (physical, integral) in voltage
                    .iter()
                    .zip(&state.mna_branch_currents[physical + row])
                {
                    assert!((*physical - scale * integral).norm() < 1.0e-10);
                }
            }
        }
    }

    #[test]
    fn retained_pss_projection_preserves_solved_integral_constants_and_spectra() {
        for rate in [1.0e3, 1.0e9] {
            let engine = Engine::default();
            let netlist = Netlist::parse(&format!(
                "Integral PSS projection\nvin in 0 sin(.7 .2 {rate})\nrin in 0 1k\n\
                 bvol out 0 v=.2+{rate}*sdt(v(in)-v(out))\nrout out 0 1k\n\
                 bnested nested 0 v={rate}*sdt(v(in)-v(nested)-{rate}*sdt(v(nested)))\nrnested nested 0 1k\n\
                 bcur current 0 i=-{rate}*sdt(-i(vin)-v(current)/1k)\nrcurrent current 0 1k\n.end\n"
            )).unwrap();
            let config = crate::analysis::PssConfig::new(rate)
                .with_points_per_period(128)
                .with_tstab_periods(0)
                .with_tolerance(1.0e-9);
            let point = engine
                .run_pss_operating_point_with_abort(&netlist, config, &NoAbort)
                .unwrap();
            let circuit = engine.build_circuit(&netlist).unwrap();
            let nodes = engine.hb_build_node_names(&circuit, circuit.num_nodes());
            let mut branches = circuit.branch_names_sorted();
            let physical = branches.len();
            branches.extend(circuit.behavioral_sources.integral_names());
            let state = engine
                .hb_state_from_pss_operating_point(
                    &point,
                    &circuit,
                    &HbConfig::new(rate).with_harmonics(3),
                    &nodes,
                    &branches,
                    &NoAbort,
                )
                .unwrap();
            assert_eq!(state.mna_branch_currents.len(), physical + 4);
            let voltage = |name: &str| {
                &state.x[nodes
                    .iter()
                    .position(|node| node.eq_ignore_ascii_case(name))
                    .unwrap()]
            };
            let integral = |name: &str| {
                &state.mna_branch_currents[branches
                    .iter()
                    .position(|branch| branch.eq_ignore_ascii_case(name))
                    .unwrap()]
            };
            for (node, memory, scale, offset) in [
                ("out", "B:bvol:sdt:0", rate, 0.2),
                ("nested", "B:bnested:sdt:1", rate, 0.0),
                ("current", "B:bcur:sdt:0", 1000.0 * rate, 0.0),
            ] {
                for harmonic in 0..=3 {
                    let expected = voltage(node)[harmonic]
                        - if harmonic == 0 {
                            Complex64::new(offset, 0.0)
                        } else {
                            Complex64::new(0.0, 0.0)
                        };
                    assert!(
                        (scale * integral(memory)[harmonic] - expected).norm() < 1.0e-8,
                        "rate={rate}, {memory}, harmonic={harmonic}"
                    );
                }
            }
            assert!((rate * integral("B:bvol:sdt:0")[0].re - 0.5).abs() < 1.0e-7);
            assert!((rate * integral("B:bnested:sdt:0")[0].re - 0.7).abs() < 1.0e-7);
            let expected = Complex64::new(0.0, -0.1) / Complex64::new(1.0, std::f64::consts::TAU);
            assert!((voltage("out")[1] - expected).norm() < 3.0e-5);
            assert!(
                engine
                    .hb_state_from_pss_operating_point(
                        &point,
                        &circuit,
                        &HbConfig::new(rate).with_harmonics(3),
                        &nodes,
                        &branches[..physical],
                        &NoAbort,
                    )
                    .is_err()
            );
        }
    }
}
