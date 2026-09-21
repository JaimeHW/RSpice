//! Constitutive current outputs from the solved periodic state.

use super::*;

impl Engine {
    #[allow(clippy::too_many_arguments)]
    pub(super) fn hb_device_current_spectra(
        &self,
        circuit: &CircuitData,
        solver: &mut HbSolver,
        state: &HbSolverState,
        result: &HbResult,
        config: &HbConfig,
        drive_tones: &[HbDriveTone],
        abort: &dyn AbortSignal,
    ) -> Result<Vec<HbCurrentSpectrum>, SimulationError> {
        let count = result.num_harmonics + 1;
        let mut spectra = Vec::new();
        let mut push = |probe: String, coefficients: Vec<Complex64>| {
            if coefficients.len() != count
                || coefficients
                    .iter()
                    .any(|c| !c.re.is_finite() || !c.im.is_finite())
                || coefficients[0].im != 0.0
            {
                return Err(SimulationError::Circuit(format!(
                    "HB current '{probe}' has an invalid spectrum"
                )));
            }
            spectra.push(HbCurrentSpectrum {
                probe,
                coefficients,
            });
            Ok(())
        };
        for index in 0..circuit.resistors.len() {
            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            let name = &circuit.resistors.names[index];
            if result
                .mna_branch_currents
                .iter()
                .any(|branch| branch.device_name.eq_ignore_ascii_case(name))
            {
                continue;
            }
            let stamp = &circuit.resistors.stamps[index];
            let conductance = circuit.resistors.conductances[index];
            let coefficients =
                Self::hb_terminal_voltage_spectrum(result, stamp.pp.row, stamp.nn.row)
                    .into_iter()
                    .map(|v| v * conductance)
                    .collect();
            push(format!("I({name})"), coefficients)?;
        }
        for index in 0..circuit.current_sources.len() {
            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            let sources = &circuit.current_sources;
            let name = &sources.names[index];
            let harmonics = Self::hb_drive_harmonics_for_source(drive_tones, name);
            let spectrum = Self::hb_source_spectrum(
                sources.dc_values[index],
                sources.ac_magnitudes.get(index).copied().unwrap_or(0.0),
                sources.ac_phases.get(index).copied().unwrap_or(0.0),
                sources
                    .source_specs
                    .get(index)
                    .and_then(|source| source.as_ref()),
                config,
                &harmonics,
                self.config.spice_dialect,
            )?;
            let mut coefficients = vec![Complex64::ZERO; count];
            coefficients[0].re = spectrum.dc;
            for (harmonic, amplitude, phase) in spectrum.harmonics {
                let coefficient = coefficients.get_mut(harmonic).ok_or_else(|| {
                    SimulationError::Circuit(format!(
                        "HB current '{name}' exceeds the solved harmonic grid"
                    ))
                })?;
                *coefficient += Complex64::from_polar(amplitude, phase);
            }
            push(format!("I({name})"), coefficients)?;
        }
        for index in 0..circuit.vccs.len() {
            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            let source = &circuit.vccs;
            let coefficients = Self::hb_terminal_voltage_spectrum(
                result,
                source.ctrl_pos[index],
                source.ctrl_neg[index],
            )
            .into_iter()
            .map(|v| v * source.transconductances[index])
            .collect();
            push(format!("I({})", source.names[index]), coefficients)?;
        }
        for index in 0..circuit.cccs.len() {
            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            let source = &circuit.cccs;
            let branch = source.ctrl_branch[index]
                .checked_sub(1)
                .and_then(|index| result.mna_branch_currents.get(index))
                .ok_or_else(|| {
                    SimulationError::Circuit(format!(
                        "HB current '{}' lost its control branch",
                        source.names[index]
                    ))
                })?;
            push(
                format!("I({})", source.names[index]),
                branch
                    .coefficients
                    .iter()
                    .map(|i| *i * source.gains[index])
                    .collect(),
            )?;
        }
        let mut nonlinear =
            solver
                .nonlinear_lead_spectra(state, abort)
                .map_err(|error| match error {
                    crate::analysis::HbError::Aborted => SimulationError::Aborted,
                    error => {
                        SimulationError::Circuit(format!("HB current observation failed: {error}"))
                    }
                })?;
        // MOS overlap charge is stamped in the solver's linear capacitance
        // operator. Add it to the corresponding physical lead observations.
        for mos in &circuit.mosfets.devices {
            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            let leads = nonlinear
                .iter_mut()
                .find(|lead| lead.name.eq_ignore_ascii_case(&mos.name))
                .ok_or_else(|| {
                    SimulationError::Circuit(format!(
                        "HB current '{}' lost its resolved device",
                        mos.name
                    ))
                })?;
            let (cgs, cgd, cgb) = mos.overlap_capacitances();
            for (terminal, node, capacitance) in [
                (2, mos.node_source, cgs),
                (0, mos.node_drain, cgd),
                (3, mos.node_bulk, cgb),
            ] {
                let voltage = Self::hb_terminal_voltage_spectrum(result, mos.node_gate, node);
                for (harmonic, voltage) in voltage.into_iter().enumerate() {
                    let current = Complex64::new(
                        0.0,
                        std::f64::consts::TAU * result.harmonic_frequencies[harmonic] * capacitance,
                    ) * voltage;
                    leads.currents[1][harmonic] += current;
                    leads.currents[terminal][harmonic] -= current;
                }
            }
        }
        for leads in nonlinear {
            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            push(format!("I({})", leads.name), leads.currents[0].clone())?;
            for (terminal, coefficients) in leads.terminals.iter().zip(leads.currents) {
                push(format!("@{}[i{terminal}]", leads.name), coefficients)?;
            }
        }
        Ok(spectra)
    }
}

#[cfg(test)]
mod tests;
