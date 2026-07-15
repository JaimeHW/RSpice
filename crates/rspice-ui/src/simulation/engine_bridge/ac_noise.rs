use std::collections::HashMap;

use rspice_core::abort_signal::AbortSignal;

use super::{EngineBridge, ensure_not_aborted};
use crate::simulation::config::{AcAnalysisConfig, NoiseAnalysisConfig};
use crate::simulation::results::{SimulationResult, WaveformData};
use crate::simulation::runner::SimulationError;

impl EngineBridge {
    /// Run AC small-signal analysis.
    pub(super) fn run_ac(
        &self,
        netlist: &rspice_core::Netlist,
        config: &AcAnalysisConfig,
        abort: &dyn AbortSignal,
    ) -> Result<SimulationResult, SimulationError> {
        ensure_not_aborted(abort)?;
        let frequencies = config.generate_frequencies();
        ensure_not_aborted(abort)?;
        self.run_ac_frequencies(netlist, frequencies, abort)
    }

    pub(super) fn run_ac_frequencies(
        &self,
        netlist: &rspice_core::Netlist,
        frequencies: Vec<f64>,
        abort: &dyn AbortSignal,
    ) -> Result<SimulationResult, SimulationError> {
        ensure_not_aborted(abort)?;
        if frequencies.is_empty() {
            return Err(SimulationError::InvalidConfig(
                "Invalid frequency sweep configuration".to_string(),
            ));
        }

        let engine = self.engine_for_netlist(netlist);
        let ac_results = engine
            .run_ac_with_abort(netlist, &frequencies, abort)
            .map_err(|e| self.translate_error(e))?;
        ensure_not_aborted(abort)?;
        if ac_results.is_empty() {
            return Ok(SimulationResult::default());
        }

        let first_result = &ac_results[0];
        let mut waveforms = HashMap::new();

        for node_idx in 0..first_result.voltages.len() {
            ensure_not_aborted(abort)?;
            let mut real_values = Vec::with_capacity(frequencies.len());
            let mut imag_values = Vec::with_capacity(frequencies.len());

            for result in &ac_results {
                ensure_not_aborted(abort)?;
                if node_idx < result.voltages.len() {
                    let v = result.voltages[node_idx];
                    real_values.push(v.re);
                    imag_values.push(v.im);
                } else {
                    real_values.push(0.0);
                    imag_values.push(0.0);
                }
            }

            let name = ac_node_waveform_name(first_result, node_idx);
            waveforms.insert(
                name.clone(),
                WaveformData::new_complex(&name, frequencies.clone(), real_values, imag_values),
            );
        }

        for branch_idx in 0..first_result.currents.len() {
            ensure_not_aborted(abort)?;
            let mut real_values = Vec::with_capacity(frequencies.len());
            let mut imag_values = Vec::with_capacity(frequencies.len());

            for result in &ac_results {
                ensure_not_aborted(abort)?;
                if branch_idx < result.currents.len() {
                    let i = result.currents[branch_idx];
                    real_values.push(i.re);
                    imag_values.push(i.im);
                } else {
                    real_values.push(0.0);
                    imag_values.push(0.0);
                }
            }

            let name = ac_branch_waveform_name(first_result, branch_idx);
            waveforms.insert(
                name.clone(),
                WaveformData::new_complex(&name, frequencies.clone(), real_values, imag_values),
            );
        }

        let measurements =
            super::measure::evaluate_measurements(netlist, "AC", &frequencies, &waveforms, abort)?;
        Ok(SimulationResult::Ac {
            frequencies,
            waveforms,
            measurements,
        })
    }

    /// Run noise analysis.
    pub(super) fn run_noise(
        &self,
        netlist: &rspice_core::Netlist,
        config: &NoiseAnalysisConfig,
        abort: &dyn AbortSignal,
    ) -> Result<SimulationResult, SimulationError> {
        ensure_not_aborted(abort)?;
        let engine = self.engine_for_netlist(netlist);
        let frequencies = config.generate_frequencies();
        ensure_not_aborted(abort)?;

        if frequencies.is_empty() {
            return Err(SimulationError::InvalidConfig(
                "Invalid noise frequency sweep configuration".to_string(),
            ));
        }

        let output_node = self.resolve_node_index(&config.output_node, netlist, abort)?;
        if output_node == 0 {
            return Err(SimulationError::InvalidConfig(format!(
                "Invalid output node '{}' for noise analysis",
                config.output_node
            )));
        }

        let noise_results = engine
            .run_noise_with_abort(
                netlist,
                output_node,
                &frequencies,
                config.default_temperature(),
                abort,
            )
            .map_err(|e| self.translate_error(e))?;
        ensure_not_aborted(abort)?;

        if noise_results.is_empty() {
            return Ok(SimulationResult::default());
        }

        let mut output_noise = Vec::with_capacity(frequencies.len());
        let mut input_noise = Vec::with_capacity(frequencies.len());
        let mut contributors: HashMap<String, Vec<f64>> = HashMap::new();

        for result in &noise_results {
            ensure_not_aborted(abort)?;
            output_noise.push(result.output_noise_density);
            input_noise.push(result.input_referred_density);

            for contrib in &result.contributions {
                ensure_not_aborted(abort)?;
                contributors
                    .entry(contrib.identity.device.clone())
                    .or_insert_with(|| Vec::with_capacity(frequencies.len()))
                    .push(contrib.output_contribution);
            }
        }

        for values in contributors.values_mut() {
            ensure_not_aborted(abort)?;
            while values.len() < frequencies.len() {
                ensure_not_aborted(abort)?;
                values.push(0.0);
            }
        }

        // Ranked band-integrated contributor summary — the table the noise
        // viewer's right panel shows. Consumes the per-frequency results
        // last; everything above only borrowed them.
        let band = (
            frequencies.first().copied().unwrap_or(0.0),
            frequencies.last().copied().unwrap_or(0.0),
        );
        ensure_not_aborted(abort)?;
        let integrated = rspice_core::analysis::IntegratedNoise::new(noise_results);
        ensure_not_aborted(abort)?;
        let contribution_summary = integrated.contribution_summary();
        ensure_not_aborted(abort)?;
        let mut rows = Vec::with_capacity(contribution_summary.len());
        for contribution in contribution_summary {
            ensure_not_aborted(abort)?;
            rows.push(crate::state::NoiseContributorRow {
                device: contribution.device_name,
                mechanism: contribution.noise_type.label(),
                power: contribution.integrated_power,
                share_pct: contribution.percentage,
            });
        }
        let summary = crate::state::NoiseSummary {
            rows,
            total_rms: integrated.total_output_noise(),
            band,
        };

        Ok(SimulationResult::Noise {
            frequencies,
            output_noise,
            input_noise: Some(input_noise),
            contributors,
            summary: Some(summary),
        })
    }

    fn resolve_node_index(
        &self,
        name: &str,
        netlist: &rspice_core::Netlist,
        abort: &dyn AbortSignal,
    ) -> Result<usize, SimulationError> {
        ensure_not_aborted(abort)?;
        let lower = name.to_lowercase();
        if lower == "0" || lower == "gnd" || lower == "ground" {
            return Ok(0);
        }

        if let Ok(idx) = name.parse::<usize>() {
            return Ok(idx);
        }

        let engine = self.engine_for_netlist(netlist);
        let dc = engine
            .run_dc_op_with_abort(netlist, abort)
            .map_err(|error| self.translate_error(error))?;
        let upper = name.to_ascii_uppercase();
        for (index, node_name) in dc.node_names.iter().enumerate() {
            ensure_not_aborted(abort)?;
            if node_name.to_ascii_uppercase() == upper {
                return Ok(index);
            }
        }

        Ok(0)
    }
}

fn ac_node_waveform_name(result: &rspice_core::analysis::AcResult, node_idx: usize) -> String {
    result
        .node_names
        .get(node_idx)
        .filter(|name| !name.is_empty())
        .map(|name| format!("V({})", name))
        .unwrap_or_else(|| format!("V({})", node_idx + 1))
}

fn ac_branch_waveform_name(result: &rspice_core::analysis::AcResult, branch_idx: usize) -> String {
    result
        .branch_names
        .get(branch_idx)
        .filter(|name| !name.is_empty())
        .map(|name| format!("I({})", name))
        .unwrap_or_else(|| format!("I({})", branch_idx + 1))
}
