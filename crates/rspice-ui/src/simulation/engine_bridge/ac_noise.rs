use std::collections::HashMap;

use rspice_core::abort_signal::AbortSignal;
use rspice_core::analysis::noise::NoiseResult;

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

        let output_node = config.output_node.trim();
        if output_node.is_empty() {
            return Err(SimulationError::InvalidConfig(
                "Noise output node is required".to_string(),
            ));
        }
        let input_source = config.input_source.trim();
        if input_source.is_empty() {
            return Err(SimulationError::InvalidConfig(
                "Noise input source is required for input-referred noise".to_string(),
            ));
        }
        let output_reference = nonempty_trimmed(&config.reference_node);

        let noise_results = engine
            .run_noise_named_with_input_source_and_abort(
                netlist,
                output_node,
                output_reference,
                input_source,
                &frequencies,
                config.default_temperature(),
                abort,
            )
            .map_err(|e| self.translate_error(e))?;
        ensure_not_aborted(abort)?;

        if noise_results.is_empty() {
            return Ok(SimulationResult::default());
        }

        // The named API above validates the selected independent source and
        // computes a real transfer normalization before returning. Only that
        // successful path is allowed to publish an input-referred spectrum.
        let (output_noise, input_noise, contributors) =
            collect_noise_series(&noise_results, true, abort)?;

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
                // The current UI row contract stores only static broad-class
                // labels. The core summary still keeps distinct mechanism
                // rows; the later result-contract tranche can expose the
                // owned canonical mechanism string without collapsing them.
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
            input_noise,
            contributors,
            summary: Some(summary),
        })
    }
}

fn nonempty_trimmed(value: &str) -> Option<&str> {
    let trimmed = value.trim();
    (!trimmed.is_empty()).then_some(trimmed)
}

type NoiseSeries = (Vec<f64>, Option<Vec<f64>>, HashMap<String, Vec<f64>>);

fn collect_noise_series(
    results: &[NoiseResult],
    input_is_normalized: bool,
    abort: &dyn AbortSignal,
) -> Result<NoiseSeries, SimulationError> {
    let point_count = results.len();
    let mut output_noise = Vec::with_capacity(point_count);
    let mut input_noise = input_is_normalized.then(|| Vec::with_capacity(point_count));
    let mut contributors: HashMap<String, Vec<f64>> = HashMap::new();

    for (point_index, result) in results.iter().enumerate() {
        ensure_not_aborted(abort)?;
        output_noise.push(result.output_noise_density);
        if let Some(input_noise) = input_noise.as_mut() {
            input_noise.push(result.input_referred_density);
        }

        // The result contract exposes one waveform per device while the core
        // reports one contribution per device mechanism. Aggregate mechanisms
        // at the same frequency instead of appending them, which would corrupt
        // the waveform length whenever a device owns multiple noise sources.
        for contribution in &result.contributions {
            ensure_not_aborted(abort)?;
            let values = contributors
                .entry(contribution.identity.device.clone())
                .or_insert_with(|| vec![0.0; point_count]);
            values[point_index] += contribution.output_contribution;
        }
    }

    Ok((output_noise, input_noise, contributors))
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

#[cfg(test)]
mod tests {
    use rspice_core::abort_signal::NoAbort;
    use rspice_core::analysis::advanced::NoiseContribution;
    use rspice_core::analysis::{NoiseSourceIdentity, NoiseSourceType};

    use super::*;
    use crate::simulation::config::AcSweepType;

    const DIFFERENTIAL_NOISE_DECK: &str = "\
V1 in 0 0 AC 1
R1 in p 1k
R2 p n 2k
R3 n 0 3k
.end
";

    fn exact_noise_config(input_source: &str) -> NoiseAnalysisConfig {
        NoiseAnalysisConfig {
            output_node: "p".to_string(),
            reference_node: "n".to_string(),
            input_source: input_source.to_string(),
            sweep_type: AcSweepType::Linear,
            num_points: 2,
            start_freq: 1.0e3,
            stop_freq: 2.0e3,
        }
    }

    #[test]
    fn bridge_noise_uses_named_differential_input_referred_solver() {
        let netlist = rspice_core::Netlist::parse(DIFFERENTIAL_NOISE_DECK).expect("deck parses");
        let config = exact_noise_config("v1");
        let frequencies = config.generate_frequencies();
        let expected = rspice_core::Engine::default()
            .run_noise_named_with_input_source(
                &netlist,
                "p",
                Some("n"),
                "V1",
                &frequencies,
                config.default_temperature(),
            )
            .expect("exact core noise analysis runs");

        let actual = EngineBridge::new()
            .run_noise(&netlist, &config, &NoAbort)
            .expect("bridge noise analysis runs");
        let SimulationResult::Noise {
            output_noise,
            input_noise,
            ..
        } = actual
        else {
            panic!("bridge must return a noise result");
        };

        assert_eq!(
            output_noise,
            expected
                .iter()
                .map(|point| point.output_noise_density)
                .collect::<Vec<_>>()
        );
        assert_eq!(
            input_noise.expect("validated input normalization must be retained"),
            expected
                .iter()
                .map(|point| point.input_referred_density)
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn bridge_noise_fails_closed_for_missing_input_source() {
        let netlist = rspice_core::Netlist::parse(DIFFERENTIAL_NOISE_DECK).expect("deck parses");
        let error = EngineBridge::new()
            .run_noise(&netlist, &exact_noise_config("missing"), &NoAbort)
            .expect_err("unknown input source must fail");

        assert!(matches!(error, SimulationError::CircuitError(_)));
        assert!(error.to_string().contains("missing"));
    }

    #[test]
    fn result_policy_never_fabricates_input_noise_and_aggregates_device_mechanisms() {
        let point = NoiseResult {
            frequency: 1.0e3,
            node_names: Vec::new(),
            branch_names: Vec::new(),
            voltages: Vec::new(),
            currents: Vec::new(),
            output_noise_density: 7.0,
            input_referred_density: 11.0,
            input_gain_squared: 1.0,
            contribution_catalog: Vec::new(),
            contributions: vec![
                NoiseContribution {
                    identity: NoiseSourceIdentity::mechanism("M1", "thermal"),
                    noise_type: NoiseSourceType::Thermal,
                    output_contribution: 2.0,
                    input_contribution: 3.0,
                    percentage: 0.0,
                },
                NoiseContribution {
                    identity: NoiseSourceIdentity::mechanism("M1", "flicker"),
                    noise_type: NoiseSourceType::Flicker,
                    output_contribution: 5.0,
                    input_contribution: 8.0,
                    percentage: 0.0,
                },
            ],
        };
        let mut second = point.clone();
        second.frequency = 2.0e3;
        second.output_noise_density = 13.0;
        second.input_referred_density = 17.0;
        second.contributions[0].output_contribution = 7.0;
        second.contributions[1].output_contribution = 6.0;

        let (output, input, contributors) =
            collect_noise_series(&[point, second], false, &NoAbort).expect("conversion succeeds");

        assert_eq!(output, vec![7.0, 13.0]);
        assert_eq!(
            input, None,
            "output-only results must not be labeled inoise"
        );
        assert_eq!(contributors.get("M1"), Some(&vec![7.0, 13.0]));
    }
}
