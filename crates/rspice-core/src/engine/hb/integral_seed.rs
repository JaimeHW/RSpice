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

    /// What a shooting orbit gives each operator site a generated model
    /// carries in harmonic balance as unknowns of its own: the spectra of the
    /// site's input and output rows, by the names harmonic balance gives
    /// them. The input is read off the model at every accepted point with
    /// each site's output held there; the output is the network's image of
    /// it, `jkω·U` for a `ddt` and `U/(jkω)` for an `idt`, whose DC term
    /// puts its value at the period start where the shooting state does
    /// (`start` reads a coordinate by its basis name). An `idt` the shooting
    /// state does not carry has an integrand identically zero, so its value
    /// is the initial condition the model reads at the period start. The
    /// outputs held at each point are those spectra's samples, after a first
    /// pass that holds each `ddt` at zero and each `idt` at its start.
    #[cfg(feature = "veriloga-builtins-base")]
    #[allow(clippy::too_many_arguments)]
    pub(in crate::engine) fn hb_project_generated_sites<'a>(
        &self,
        circuit: &CircuitData,
        start: &dyn Fn(&str) -> Option<Value>,
        times: &[Value],
        period: Value,
        harmonics: usize,
        nodes: impl IntoIterator<Item = (&'a str, &'a [Value])>,
        branches: impl IntoIterator<Item = (&'a str, &'a [Value])>,
        abort: &dyn AbortSignal,
    ) -> Result<Vec<(String, Vec<Complex64>)>, SimulationError> {
        use crate::analysis::fourier::{FourierError, FourierQuadrature};
        use crate::analysis::harmonic_balance::generated_site_row_name;
        use crate::device::veriloga_builtins::{GeneratedAnalysisKind, GeneratedDdtCoefficients};
        use rspice_veriloga_runtime::GeneratedPeriodicKind;
        let mut projected = Vec::new();
        if circuit
            .generated_veriloga_devices()
            .iter()
            .all(|device| device.harmonic_balance_periodic_sites().is_empty())
        {
            return Ok(projected);
        }
        let failure = |detail: String| {
            SimulationError::Circuit(format!("generated operator-site projection: {detail}"))
        };
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
        let size = circuit.matrix_size();
        if waveforms.len() != size {
            return Err(failure("the orbit's MNA basis is incomplete".to_owned()));
        }
        let num_nodes = circuit.num_nodes();
        let simparams = circuit.generated_simulation_parameters;
        let omega = std::f64::consts::TAU / period;
        let spectrum = |values: &[Value]| -> Result<Vec<Complex64>, SimulationError> {
            let error = |error| match error {
                FourierError::Aborted => SimulationError::Aborted,
                other => failure(other.to_string()),
            };
            let quadrature = FourierQuadrature::new(times, values, period, abort).map_err(error)?;
            (0..=harmonics)
                .map(|harmonic| {
                    let (magnitude, phase) = quadrature
                        .component(harmonic as Value / period, harmonic, abort)
                        .map_err(error)?;
                    Ok(if harmonic == 0 {
                        Complex64::new(magnitude, 0.0)
                    } else {
                        Complex64::from_polar(0.5 * magnitude, phase.to_radians())
                    })
                })
                .collect()
        };
        let sample = |spectrum: &[Complex64], time: Value| {
            spectrum.iter().enumerate().skip(1).fold(
                spectrum.first().map_or(0.0, |dc| dc.re),
                |value, (harmonic, coefficient)| {
                    value
                        + 2.0
                            * (coefficient
                                * Complex64::from_polar(1.0, omega * harmonic as Value * time))
                            .re
                },
            )
        };
        let mut solution = vec![0.0; size];
        for device in circuit.generated_veriloga_devices().iter() {
            let kinds = device.harmonic_balance_periodic_sites();
            if kinds.is_empty() {
                continue;
            }
            let count = kinds.len();
            let first_integral = count - device.periodic_integral_count();
            let name = device.instance_name.as_str();
            let evaluation = |error: &dyn std::fmt::Display| {
                failure(format!(
                    "generated Verilog-A device '{name}' failed: {error}"
                ))
            };
            // Set up as harmonic balance's sampler sets up its static probe.
            let mut probe = device.clone();
            let mut rows: Vec<usize> = probe
                .nodes
                .iter()
                .filter(|&&node| node > 0)
                .map(|&node| node - 1)
                .chain(
                    probe
                        .branches
                        .iter()
                        .filter(|&&branch| branch > 0)
                        .map(|&branch| num_nodes + branch - 1),
                )
                .collect();
            rows.sort_unstable();
            rows.dedup();
            let triplets: Vec<(usize, usize, Value)> = rows
                .iter()
                .flat_map(|&row| rows.iter().map(move |&col| (row, col, 0.0)))
                .collect();
            let mut sink = crate::solver::StaticMatrix::from_triplets(size, size, &triplets)
                .map_err(|error| evaluation(&error))?;
            let mut rhs = vec![0.0; size];
            probe.install_periodic_frame();
            probe.set_initially_off(false);
            probe.link_static_stamps(&sink, num_nodes);
            probe
                .begin_analysis(GeneratedAnalysisKind::Tran, simparams, num_nodes)
                .map_err(|error| evaluation(&error))?;
            probe.set_timepoint(0.0, omega.recip(), GeneratedDdtCoefficients::inactive());
            let mut stamp =
                |probe: &mut crate::device::veriloga_builtins::BuiltinVerilogAInstance,
                 solution: &[Value]| {
                    sink.clear_values();
                    rhs.fill(0.0);
                    probe
                        .stamp_probe(
                            &mut sink,
                            &mut rhs,
                            solution,
                            num_nodes,
                            GeneratedAnalysisKind::Tran,
                            simparams,
                        )
                        .map_err(|error| evaluation(&error))
                };
            let seeds = vec![None; count];
            let mut held = vec![0.0; count];
            let reference = vec![0.0; size];
            for initial_step in [true, false] {
                probe.set_analysis_step(initial_step, false);
                if let Some(frame) = probe.periodic_frame_mut() {
                    frame.prepare(&held, &seeds);
                }
                stamp(&mut probe, &reference)?;
            }
            let mut starts: Vec<Option<Value>> = (0..count)
                .map(|site| {
                    site.checked_sub(first_integral).and_then(|slot| {
                        start(&super::super::pss::generated_integral_coordinate_name(
                            name, slot,
                        ))
                    })
                })
                .collect();
            let mut inputs = vec![vec![0.0; times.len()]; count];
            let mut input_spectra = vec![Vec::new(); count];
            let mut output_spectra: Vec<Vec<Complex64>> = vec![Vec::new(); count];
            for pass in 0..2 {
                for (point, &time) in times.iter().enumerate() {
                    if abort.is_aborted() {
                        return Err(SimulationError::Aborted);
                    }
                    for (value, waveform) in solution.iter_mut().zip(&waveforms) {
                        *value = waveform[point];
                    }
                    for (site, output) in held.iter_mut().enumerate() {
                        *output = if pass == 0 {
                            starts[site].unwrap_or(0.0)
                        } else {
                            sample(&output_spectra[site], time)
                        };
                    }
                    probe
                        .periodic_frame_mut()
                        .ok_or_else(|| failure(format!("'{name}' lost its periodic frame")))?
                        .prepare(&held, &seeds);
                    stamp(&mut probe, &solution)?;
                    let frame = probe
                        .periodic_frame()
                        .ok_or_else(|| failure(format!("'{name}' lost its periodic frame")))?;
                    for (site, input) in inputs.iter_mut().enumerate() {
                        if point == 0 && site >= first_integral && starts[site].is_none() {
                            let condition = frame.initial_condition(site);
                            starts[site] = Some(if condition.is_finite() {
                                condition
                            } else {
                                0.0
                            });
                        }
                        // A site the model did not evaluate here is
                        // evaluated on no sample: its input is zero.
                        input[point] = match frame.input(site) {
                            value if value.is_nan() => 0.0,
                            value if value.is_finite() => value,
                            _ => {
                                return Err(failure(format!(
                                    "site {site} of '{name}' has a non-finite input"
                                )));
                            }
                        };
                    }
                }
                for site in 0..count {
                    let input = spectrum(&inputs[site])?;
                    let mut output: Vec<Complex64> = input
                        .iter()
                        .enumerate()
                        .map(|(harmonic, &value)| {
                            let rate = Complex64::new(0.0, omega * harmonic as Value);
                            match kinds[site] {
                                GeneratedPeriodicKind::Derivative => rate * value,
                                GeneratedPeriodicKind::Integral if harmonic == 0 => Complex64::ZERO,
                                GeneratedPeriodicKind::Integral => value / rate,
                            }
                        })
                        .collect();
                    if matches!(kinds[site], GeneratedPeriodicKind::Integral) {
                        let swing: Value = output.iter().skip(1).map(|value| 2.0 * value.re).sum();
                        output[0] = Complex64::new(starts[site].unwrap_or(0.0) - swing, 0.0);
                    }
                    input_spectra[site] = input;
                    output_spectra[site] = output;
                }
            }
            for (site, (input, output)) in input_spectra.into_iter().zip(output_spectra).enumerate()
            {
                projected.push((
                    generated_site_row_name(name, kinds[site], site, "input"),
                    input,
                ));
                projected.push((
                    generated_site_row_name(name, kinds[site], site, "output"),
                    output,
                ));
            }
        }
        Ok(projected)
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
            // The nested integral is t^2/2 in periods. Linear interpolation
            // over the tolerance run's steps, at most a tenth of the period,
            // moves it by at most 0.1^2/8 per sample.
            let quadratic_mean = (n - 1.0) * (2.0 * n - 1.0) / (12.0 * n * n);
            let interpolation =
                rate * rate * state.mna_branch_currents[physical + 2][0].re - quadratic_mean;
            assert!(
                interpolation.abs() <= 0.1_f64.powi(2) / 8.0,
                "{interpolation}"
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
