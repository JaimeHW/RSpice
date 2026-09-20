//! One device-noise catalog for driven noise and periodic port covariance.

use super::*;

impl Engine {
    pub(in crate::engine::hb) fn validate_periodic_noise_circuit(
        &self,
        circuit: &CircuitData,
    ) -> Result<(), SimulationError> {
        validate_resistor_noise_metadata(circuit)?;
        if let Some(summary) = periodic_capability::summarize(
            &periodic_capability::cyclostationary_noise_gaps(circuit),
        ) {
            return Err(SimulationError::unsupported_capability(
                "analysis.pnoise.colored_noise",
                format!(
                    "driven pnoise requires exact cyclostationary colored-noise folding, which is not implemented for {summary}; set the listed noise coefficient exactly to zero to disable that mechanism"
                ),
            ));
        }
        let temperature = self.config.temperature;
        let physical_constants = pnoise_physical_constants(self.config.spice_dialect);
        if !temperature.is_finite() || temperature <= 0.0 {
            return Err(SimulationError::Circuit(format!(
                "pnoise absolute analysis temperature must be finite and positive, got {temperature} K"
            )));
        }
        if !physical_constants.boltzmann.is_finite()
            || physical_constants.boltzmann <= 0.0
            || !physical_constants.electron_charge.is_finite()
            || physical_constants.electron_charge <= 0.0
        {
            return Err(SimulationError::Circuit(format!(
                "pnoise physical constants must be finite and positive, got k={}, q={}",
                physical_constants.boltzmann, physical_constants.electron_charge
            )));
        }

        Ok(())
    }

    /// Exclusions name exact materialized resistors, never contributor labels.
    /// Periodic port analysis excludes its external source terminations here,
    /// before covariance accumulation, avoiding subtraction of thermal totals.
    pub(in crate::engine::hb) fn prepare_periodic_noise_sources(
        &self,
        circuit: &CircuitData,
        solver: &mut HbSolver,
        state: &HbSolverState,
        excluded_resistors: &[String],
        abort: &dyn AbortSignal,
    ) -> Result<Vec<PeriodicNoiseSource>, SimulationError> {
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        self.validate_periodic_noise_circuit(circuit)?;
        let num_nodes = circuit.num_nodes();
        let node_names = self.hb_build_node_names(circuit, num_nodes);
        let branch_names = solver.try_periodic_mna_branch_names().map_err(|error| {
            SimulationError::Circuit(format!("periodic noise branch metadata failed: {error}"))
        })?;
        let op_harmonics = state
            .x
            .first()
            .and_then(|row| row.len().checked_sub(1))
            .ok_or_else(|| {
                SimulationError::Circuit("periodic noise state has no node spectra".into())
            })?;
        let temperature = self.config.temperature;
        let physical_constants = pnoise_physical_constants(self.config.spice_dialect);
        let k_b = physical_constants.boltzmann;

        // Stationary resistor thermal sources: 4kT*G between the resistor
        // terminals (DC-only intensity spectrum).
        let mut sources: Vec<PeriodicNoiseSource> = Vec::new();

        // `.OPTIONS RSHUNT` is a physical resistor from every electrical
        // node to ground. It is distinct from numerical GMIN and therefore
        // contributes one independent stationary 4kT*G source per stamped
        // electrical row. Private DAE state rows are deliberately excluded.
        let shunt_conductance = circuit.global_shunt_conductance();
        if !shunt_conductance.is_finite() || shunt_conductance < 0.0 {
            return Err(SimulationError::Circuit(format!(
                "pnoise .OPTIONS RSHUNT conductance is invalid ({shunt_conductance})"
            )));
        }
        if shunt_conductance > 0.0 {
            let shunt_density = checked_scaled_positive_product(
                &[4.0, k_b, temperature, shunt_conductance],
                "pnoise .OPTIONS RSHUNT thermal-noise density",
            )?;
            for node_index in 0..num_nodes {
                if abort.is_aborted() {
                    return Err(SimulationError::Aborted);
                }
                if circuit.is_non_electrical_state_matrix_index(node_index) {
                    continue;
                }
                let node_name = node_names.get(node_index).ok_or_else(|| {
                    SimulationError::Circuit(format!(
                        "pnoise .OPTIONS RSHUNT node metadata is missing for electrical row {node_index}"
                    ))
                })?;
                sources.push(PeriodicNoiseSource {
                    name: format!("RSHUNT:{node_name} thermal"),
                    node_pos: node_index,
                    node_neg: usize::MAX,
                    psd: vec![Complex64::new(shunt_density.mantissa, 0.0)],
                    binary_scale_exponent: shunt_density.exponent,
                    flicker: None,
                });
            }
        }

        for i in 0..circuit.resistors.len() {
            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            if !circuit.resistors.noisy[i]
                || excluded_resistors
                    .iter()
                    .any(|name| name.eq_ignore_ascii_case(&circuit.resistors.names[i]))
            {
                continue;
            }
            let g = circuit.resistors.small_signal_conductance(i);
            if !g.is_finite() || g < 0.0 {
                return Err(SimulationError::Circuit(format!(
                    "pnoise resistor '{}' has invalid conductance {g}",
                    circuit.resistors.names[i]
                )));
            }
            if g == 0.0 {
                continue;
            }
            let np = circuit.resistors.stamps[i].pp.row;
            let nn = circuit.resistors.stamps[i].nn.row;
            let name = circuit
                .resistors
                .names
                .get(i)
                .cloned()
                .unwrap_or_else(|| format!("R#{i}"));
            let source_temperature = circuit.resistor_noise_temperature(i, temperature);
            if !source_temperature.is_finite() || source_temperature <= 0.0 {
                return Err(SimulationError::Circuit(format!(
                    "pnoise resistor '{name}' absolute noise temperature must be finite and positive, got {source_temperature} K"
                )));
            }
            let thermal_density = checked_scaled_positive_product(
                &[4.0, k_b, source_temperature, g],
                &format!("pnoise resistor '{name}' thermal-noise density"),
            )?;
            sources.push(PeriodicNoiseSource {
                name: format!("{name} thermal"),
                node_pos: Self::hb_node_to_solver_index(np, num_nodes),
                node_neg: Self::hb_node_to_solver_index(nn, num_nodes),
                psd: vec![Complex64::new(thermal_density.mantissa, 0.0)],
                binary_scale_exponent: thermal_density.exponent,
                flicker: None,
            });

            if let Some(crate::circuit::ResistorFlickerNoise {
                coefficient,
                binary_scale,
                af: 2.0,
                ef: exponent,
            }) = circuit.resistors.flicker[i]
                && coefficient != 0.0
            {
                // AF=2 models a resistance fluctuation multiplied by SIGNED
                // current. Use the exact voltage spectrum and fold DC G^2
                // into the scaled coefficient, avoiding an intermediate
                // current square (or a rectified waveform/FFT).
                let density = checked_scaled_positive_product(
                    &[
                        coefficient,
                        circuit.resistors.conductances[i],
                        circuit.resistors.conductances[i],
                    ],
                    &format!("pnoise resistor '{name}' flicker coefficient"),
                )?;
                let node_pos = Self::hb_node_to_solver_index(np, num_nodes);
                let node_neg = Self::hb_node_to_solver_index(nn, num_nodes);
                let mut modulation = Vec::new();
                modulation
                    .try_reserve_exact(op_harmonics + 1)
                    .map_err(|error| {
                        SimulationError::Circuit(format!(
                            "pnoise resistor '{name}' modulation allocation failed: {error}"
                        ))
                    })?;
                for harmonic in 0..=op_harmonics {
                    let voltage = |node: usize| {
                        if node >= num_nodes {
                            Ok(Complex64::default())
                        } else {
                            state.x.get(node).and_then(|row| row.get(harmonic)).copied().ok_or_else(|| {
                                SimulationError::Circuit(format!("pnoise resistor '{name}' lacks node {node} harmonic {harmonic}"))
                            })
                        }
                    };
                    modulation.push(voltage(node_pos)? - voltage(node_neg)?);
                }
                sources.push(PeriodicNoiseSource {
                    name: format!("{name} flicker"),
                    node_pos,
                    node_neg,
                    psd: vec![Complex64::default()],
                    binary_scale_exponent: density.exponent.checked_add(binary_scale).ok_or_else(|| {
                        SimulationError::Circuit(format!("pnoise resistor '{name}' coefficient exceeds the retained binary exponent range"))
                    })?,
                    flicker: Some(PeriodicFlickerNoise {
                        coefficient: density.mantissa,
                        exponent,
                        modulation,
                    }),
                });
            }
        }

        for i in 0..circuit.resistor_branches.len() {
            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            if !circuit.resistor_branches.noisy[i]
                || excluded_resistors
                    .iter()
                    .any(|name| name.eq_ignore_ascii_case(&circuit.resistor_branches.names[i]))
            {
                continue;
            }
            let name = &circuit.resistor_branches.names[i];
            let resistance = circuit.resistor_branches.small_signal_resistances[i];
            if !resistance.is_finite() || resistance < 0.0 {
                return Err(SimulationError::Circuit(format!(
                    "pnoise branch-form resistor '{name}' has invalid noise resistance {resistance}"
                )));
            }
            // An exact ideal short has zero Thevenin noise voltage, and a
            // parallel Norton source cannot perturb its constrained terminals.
            if resistance == 0.0 {
                continue;
            }
            let source_temperature = circuit.resistor_branches.noise_temperature(i, temperature);
            if !source_temperature.is_finite() || source_temperature <= 0.0 {
                return Err(SimulationError::Circuit(format!(
                    "pnoise branch-form resistor '{name}' absolute noise temperature must be finite and positive, got {source_temperature} K"
                )));
            }
            let thermal_density = checked_scaled_positive_ratio(
                &[4.0, k_b, source_temperature],
                resistance,
                &format!("pnoise branch-form resistor '{name}' thermal-noise density"),
            )?;
            sources.push(PeriodicNoiseSource {
                name: format!("{name} thermal"),
                node_pos: Self::hb_node_to_solver_index(
                    circuit.resistor_branches.node_pos[i],
                    num_nodes,
                ),
                node_neg: Self::hb_node_to_solver_index(
                    circuit.resistor_branches.node_neg[i],
                    num_nodes,
                ),
                psd: vec![Complex64::new(thermal_density.mantissa, 0.0)],
                binary_scale_exponent: thermal_density.exponent,
                flicker: None,
            });
            if let Some(crate::circuit::ResistorFlickerNoise {
                coefficient,
                binary_scale,
                af: 2.0,
                ef: exponent,
            }) = circuit.resistor_branches.flicker[i]
                && coefficient != 0.0
            {
                let branch = circuit.resistor_branches.branch_indices[i]
                    .checked_sub(1)
                    .ok_or_else(|| {
                        SimulationError::Circuit(format!(
                            "pnoise resistor '{name}' has an invalid branch ordinal"
                        ))
                    })?;
                if branch_names.get(branch) != Some(name) {
                    return Err(SimulationError::Circuit(format!(
                        "pnoise resistor '{name}' has misaligned branch-current metadata"
                    )));
                }
                let current = state.mna_branch_currents.get(branch).ok_or_else(|| {
                    SimulationError::Circuit(format!(
                        "pnoise resistor '{name}' lacks its periodic branch current"
                    ))
                })?;
                let mut modulation = Vec::new();
                modulation
                    .try_reserve_exact(current.len())
                    .map_err(|error| {
                        SimulationError::Circuit(format!(
                            "pnoise resistor '{name}' modulation allocation failed: {error}"
                        ))
                    })?;
                modulation.extend_from_slice(current);
                sources.push(PeriodicNoiseSource {
                    name: format!("{name} flicker"),
                    node_pos: Self::hb_node_to_solver_index(
                        circuit.resistor_branches.node_pos[i],
                        num_nodes,
                    ),
                    node_neg: Self::hb_node_to_solver_index(
                        circuit.resistor_branches.node_neg[i],
                        num_nodes,
                    ),
                    psd: vec![Complex64::default()],
                    binary_scale_exponent: binary_scale,
                    flicker: Some(PeriodicFlickerNoise {
                        coefficient,
                        exponent,
                        modulation,
                    }),
                });
            }
        }

        // Cyclostationary device sources from the converged waveforms.
        sources.extend(
            solver
                .device_noise_sources(state, temperature, physical_constants)
                .map_err(|error| {
                    SimulationError::Circuit(format!(
                        "pnoise nonlinear-device source construction failed: {error}"
                    ))
                })?,
        );
        sources.extend(self.native_bjt_periodic_noise_sources(
            solver,
            state,
            temperature,
            abort,
        )?);
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        Ok(sources)
    }
}
