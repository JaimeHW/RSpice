use std::collections::HashMap;

use super::EngineBridge;
use crate::simulation::config::{AcAnalysisConfig, NoiseAnalysisConfig};
use crate::simulation::results::{SimulationResult, WaveformData};
use crate::simulation::runner::SimulationError;

impl EngineBridge {
    /// Run AC small-signal analysis.
    pub(super) fn run_ac(
        &self,
        netlist: &rspice_core::Netlist,
        config: &AcAnalysisConfig,
    ) -> Result<SimulationResult, SimulationError> {
        let engine = self.engine_for_netlist(netlist);
        let frequencies = config.generate_frequencies();

        if frequencies.is_empty() {
            return Err(SimulationError::InvalidConfig(
                "Invalid frequency sweep configuration".to_string(),
            ));
        }

        let ac_results = engine
            .run_ac(netlist, &frequencies)
            .map_err(|e| self.translate_error(e))?;
        if ac_results.is_empty() {
            return Ok(SimulationResult::default());
        }

        let first_result = &ac_results[0];
        let mut waveforms = HashMap::new();

        for node_idx in 0..first_result.voltages.len() {
            let mut real_values = Vec::with_capacity(frequencies.len());
            let mut imag_values = Vec::with_capacity(frequencies.len());

            for result in &ac_results {
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
            let mut real_values = Vec::with_capacity(frequencies.len());
            let mut imag_values = Vec::with_capacity(frequencies.len());

            for result in &ac_results {
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

        Ok(SimulationResult::Ac {
            frequencies,
            waveforms,
        })
    }

    /// Run noise analysis.
    pub(super) fn run_noise(
        &self,
        netlist: &rspice_core::Netlist,
        config: &NoiseAnalysisConfig,
    ) -> Result<SimulationResult, SimulationError> {
        let engine = self.engine_for_netlist(netlist);
        let frequencies = config.generate_frequencies();

        if frequencies.is_empty() {
            return Err(SimulationError::InvalidConfig(
                "Invalid noise frequency sweep configuration".to_string(),
            ));
        }

        let output_node = self.resolve_node_index(&config.output_node, netlist);
        if output_node == 0 {
            return Err(SimulationError::InvalidConfig(format!(
                "Invalid output node '{}' for noise analysis",
                config.output_node
            )));
        }

        let noise_results = engine
            .run_noise(
                netlist,
                output_node,
                &frequencies,
                config.default_temperature(),
            )
            .map_err(|e| self.translate_error(e))?;

        if noise_results.is_empty() {
            return Ok(SimulationResult::default());
        }

        let mut output_noise = Vec::with_capacity(frequencies.len());
        let mut input_noise = Vec::with_capacity(frequencies.len());
        let mut contributors: HashMap<String, Vec<f64>> = HashMap::new();

        for result in &noise_results {
            output_noise.push(result.output_noise_density);
            input_noise.push(result.input_referred_density);

            for contrib in &result.contributions {
                contributors
                    .entry(contrib.device_name.clone())
                    .or_insert_with(|| Vec::with_capacity(frequencies.len()))
                    .push(contrib.output_contribution);
            }
        }

        for values in contributors.values_mut() {
            while values.len() < frequencies.len() {
                values.push(0.0);
            }
        }

        Ok(SimulationResult::Noise {
            frequencies,
            output_noise,
            input_noise: Some(input_noise),
            contributors,
        })
    }

    fn resolve_node_index(&self, name: &str, netlist: &rspice_core::Netlist) -> usize {
        let lower = name.to_lowercase();
        if lower == "0" || lower == "gnd" || lower == "ground" {
            return 0;
        }

        if let Ok(idx) = name.parse::<usize>() {
            return idx;
        }

        let engine = self.engine_for_netlist(netlist);
        if let Ok(dc) = engine.run_dc_op(netlist) {
            let upper = name.to_ascii_uppercase();
            if let Some(idx) = dc
                .node_names
                .iter()
                .position(|n| n.to_ascii_uppercase() == upper)
            {
                return idx;
            }
        }

        0
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
