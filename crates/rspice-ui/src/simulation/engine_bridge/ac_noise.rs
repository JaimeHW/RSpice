//! AC and noise analysis over the engine bridge.

use std::collections::HashMap;

use rspice_core::abort_signal::AbortSignal;
use rspice_core::analysis::noise::NoiseResult;

use super::{EngineBridge, ensure_not_aborted};
use crate::simulation::config::{
    AcAnalysisConfig, NoiseAnalysisConfig, NoiseContributionDetail, NoiseIntegrationMode,
};
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

        let noise_results = if let Some(table_name) = config.data_table_name.as_deref() {
            engine
                .run_noise_data_named_with_input_source_and_abort(
                    netlist,
                    output_node,
                    output_reference,
                    input_source,
                    table_name,
                    config.default_temperature(),
                    abort,
                )
                .map(|(_, results)| results)
                .map_err(|e| self.translate_error(e))?
        } else {
            let frequencies = config.generate_frequencies();
            ensure_not_aborted(abort)?;
            if frequencies.is_empty() {
                return Err(SimulationError::InvalidConfig(
                    "Invalid noise frequency sweep configuration".to_string(),
                ));
            }
            engine
                .run_noise_named_with_input_source_and_abort(
                    netlist,
                    output_node,
                    output_reference,
                    input_source,
                    &frequencies,
                    config.default_temperature(),
                    abort,
                )
                .map_err(|e| self.translate_error(e))?
        };
        ensure_not_aborted(abort)?;

        if noise_results.is_empty() {
            return Ok(SimulationResult::default());
        }

        // The named API above validates the selected independent source and
        // computes a real transfer normalization before returning. Only that
        // successful path is allowed to publish an input-referred spectrum.
        let frequencies = noise_results
            .iter()
            .map(|result| result.frequency)
            .collect::<Vec<_>>();
        let (output_noise, input_noise, contributors) =
            collect_noise_series(&noise_results, true, abort)?;

        // Ranked band-integrated contributor summary — the table the noise
        // viewer's right panel shows. Consumes the per-frequency results
        // last; everything above only borrowed them.
        let band = frequencies.iter().copied().fold(
            (f64::INFINITY, f64::NEG_INFINITY),
            |(minimum, maximum), frequency| (minimum.min(frequency), maximum.max(frequency)),
        );
        ensure_not_aborted(abort)?;
        let mut integration_results = noise_results;
        integration_results.sort_by(|left, right| left.frequency.total_cmp(&right.frequency));
        let integrated = rspice_core::analysis::IntegratedNoise::new(integration_results);
        ensure_not_aborted(abort)?;
        let contribution_summary = integrated.contribution_summary();
        ensure_not_aborted(abort)?;
        let mut rows = Vec::with_capacity(contribution_summary.len());
        for contribution in contribution_summary {
            ensure_not_aborted(abort)?;
            rows.push(crate::state::NoiseContributorRow {
                device: contribution.device_name,
                mechanism: contribution.mechanism,
                power: contribution.integrated_power,
                share_pct: contribution.percentage,
            });
        }
        let (rows, contributors) =
            retain_contribution_evidence(rows, contributors, config.contribution_detail);
        let (total_rms, input_rms) = retain_integrated_totals(
            integrated.total_output_noise(),
            integrated.total_input_referred_noise(),
            config.integration_mode,
        );
        let summary = crate::state::NoiseSummary {
            rows,
            total_rms,
            input_rms,
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

        for contribution in &result.contributions {
            ensure_not_aborted(abort)?;
            let mechanism = contribution
                .identity
                .mechanism
                .as_deref()
                .unwrap_or_else(|| contribution.noise_type.label());
            let values = contributors
                .entry(contributor_key(&contribution.identity.device, mechanism))
                .or_insert_with(|| vec![0.0; point_count]);
            values[point_index] += contribution.output_contribution;
        }
    }

    Ok((output_noise, input_noise, contributors))
}

fn contributor_key(device: &str, mechanism: &str) -> String {
    format!("{device} · {mechanism}")
}

fn retain_contribution_evidence(
    mut rows: Vec<crate::state::NoiseContributorRow>,
    mut contributors: HashMap<String, Vec<f64>>,
    detail: NoiseContributionDetail,
) -> (
    Vec<crate::state::NoiseContributorRow>,
    HashMap<String, Vec<f64>>,
) {
    let retain_count = match detail {
        NoiseContributionDetail::AllContributors => rows.len(),
        NoiseContributionDetail::Top50 => 50,
        NoiseContributionDetail::Top20 => 20,
        NoiseContributionDetail::SummaryOnly => 0,
    };
    rows.truncate(retain_count);
    if detail == NoiseContributionDetail::SummaryOnly {
        contributors.clear();
    } else {
        let retained = rows
            .iter()
            .map(|row| contributor_key(&row.device, &row.mechanism))
            .collect::<std::collections::HashSet<_>>();
        contributors.retain(|name, _| retained.contains(name));
    }
    (rows, contributors)
}

fn retain_integrated_totals(
    output_rms: f64,
    input_rms: f64,
    mode: NoiseIntegrationMode,
) -> (Option<f64>, Option<f64>) {
    match mode {
        NoiseIntegrationMode::Enabled => (Some(output_rms), Some(input_rms)),
        NoiseIntegrationMode::OutputNoiseOnly => (Some(output_rms), None),
        NoiseIntegrationMode::Disabled => (None, None),
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

#[cfg(test)]
mod tests {
    use rspice_core::abort_signal::NoAbort;
    use rspice_core::analysis::NoiseContribution;
    use rspice_core::analysis::{NoiseSourceIdentity, NoiseSourceType};

    use super::*;
    use crate::simulation::config::AcSweepType;

    const DIFFERENTIAL_NOISE_DECK: &str = "\
differential noise bridge
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
            ..NoiseAnalysisConfig::default()
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
    fn result_policy_never_fabricates_input_noise_and_preserves_mechanisms() {
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
        assert_eq!(
            contributors.get(&contributor_key("M1", "thermal")),
            Some(&vec![2.0, 7.0])
        );
        assert_eq!(
            contributors.get(&contributor_key("M1", "flicker")),
            Some(&vec![5.0, 6.0])
        );
    }

    #[test]
    fn noise_retention_and_integration_policies_are_enforced_exactly() {
        let rows = (0..60)
            .map(|index| crate::state::NoiseContributorRow {
                device: format!("R{index}"),
                mechanism: "thermal".to_owned(),
                power: (60 - index) as f64,
                share_pct: (60 - index) as f64 / 18.3,
            })
            .collect::<Vec<_>>();
        let contributors = rows
            .iter()
            .map(|row| {
                (
                    contributor_key(&row.device, &row.mechanism),
                    vec![row.power],
                )
            })
            .collect::<HashMap<_, _>>();

        for (detail, expected) in [
            (NoiseContributionDetail::Top20, 20),
            (NoiseContributionDetail::Top50, 50),
        ] {
            let (retained_rows, retained_contributors) =
                retain_contribution_evidence(rows.clone(), contributors.clone(), detail);
            assert_eq!(retained_rows.len(), expected);
            assert_eq!(retained_contributors.len(), expected);
            assert!(
                retained_rows
                    .windows(2)
                    .all(|rows| rows[0].power >= rows[1].power)
            );
        }

        let (summary_rows, summary_contributors) = retain_contribution_evidence(
            rows.clone(),
            contributors.clone(),
            NoiseContributionDetail::SummaryOnly,
        );
        assert!(summary_rows.is_empty());
        assert!(summary_contributors.is_empty());
        let (all_rows, all_contributors) = retain_contribution_evidence(
            rows,
            contributors,
            NoiseContributionDetail::AllContributors,
        );
        assert_eq!(all_rows.len(), 60);
        assert_eq!(all_contributors.len(), 60);

        assert_eq!(
            retain_integrated_totals(3.0, 2.0, NoiseIntegrationMode::Enabled),
            (Some(3.0), Some(2.0))
        );
        assert_eq!(
            retain_integrated_totals(3.0, 2.0, NoiseIntegrationMode::OutputNoiseOnly),
            (Some(3.0), None)
        );
        assert_eq!(
            retain_integrated_totals(3.0, 2.0, NoiseIntegrationMode::Disabled),
            (None, None)
        );
    }

    /// A sweep that yields no frequencies must be refused, not reported as a
    /// successful run with an empty spectrum -- an empty noise result reads as
    /// "no noise", which is the opposite of "not measured".
    #[test]
    fn a_degenerate_sweep_is_refused_rather_than_returned_empty() {
        let netlist = rspice_core::Netlist::parse(DIFFERENTIAL_NOISE_DECK).expect("deck parses");
        let mut config = exact_noise_config("v1");
        config.num_points = 0;
        assert!(
            config.generate_frequencies().is_empty(),
            "the sweep under test must actually be degenerate"
        );

        let error = EngineBridge::new()
            .run_noise(&netlist, &config, &NoAbort)
            .expect_err("an empty sweep is a configuration error");

        assert!(matches!(error, SimulationError::InvalidConfig(_)));
    }
}
