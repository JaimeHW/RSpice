//! Authored independent noise analysis draft.

use serde::{Deserialize, Serialize};

use super::parse::{parse_positive, parse_positive_usize};
use crate::config::{
    AcSweepType, NoiseAnalysisConfig, NoiseContributionDetail, NoiseIntegrationMode, NoiseSweepType,
};

/// Noise draft with an independent, losslessly persisted analysis contract.
///
/// The legacy singleton setup borrowed these two values from AC. Keeping them
/// here removes that hidden cross-analysis mutation. Raw text buffers are
/// retained so an unfinished edit can still round-trip through project save.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NoiseDraft {
    pub output: String,
    /// Legacy output reference. New drafts use `output` for either a
    /// single-ended node or a comma-separated differential expression.
    #[serde(default = "default_noise_reference")]
    pub reference: String,
    pub input: String,
    pub fstart: String,
    pub fstop: String,
    pub points: String,
    pub sweep: NoiseSweepType,
    /// Comma/space-separated frequency values used only by the explicit mode.
    #[serde(default = "default_noise_frequency_list")]
    pub explicit_frequencies: String,
    #[serde(default)]
    pub contribution_detail: NoiseContributionDetail,
    #[serde(default)]
    pub integration_mode: NoiseIntegrationMode,
}

impl Default for NoiseDraft {
    fn default() -> Self {
        Self {
            // The output node and the input source name the user's own
            // circuit. A new draft states neither: `to_config` refuses an
            // empty one with the field that is missing, which is guidance, and
            // a name the design does not carry is a run that fails later.
            output: String::new(),
            reference: default_noise_reference(),
            input: String::new(),
            fstart: "10".to_owned(),
            fstop: "1Meg".to_owned(),
            points: "30".to_owned(),
            sweep: NoiseSweepType::Decade,
            explicit_frequencies: default_noise_frequency_list(),
            contribution_detail: NoiseContributionDetail::Top50,
            integration_mode: NoiseIntegrationMode::Enabled,
        }
    }
}

fn default_noise_reference() -> String {
    "0".to_owned()
}

fn default_noise_frequency_list() -> String {
    "10, 100, 1k, 10k, 100k, 1Meg".to_owned()
}

impl NoiseDraft {
    /// Parse this raw draft into a complete executable configuration.
    pub fn to_config(&self) -> Result<NoiseAnalysisConfig, String> {
        let (output_node, reference_node) =
            parse_noise_output_expression(&self.output, &self.reference)?;
        let input_source = self.input.trim().to_owned();
        if input_source.is_empty() {
            return Err("input source is required".to_owned());
        }

        let (sweep_type, num_points, start_freq, stop_freq, explicit_frequencies) = match self.sweep
        {
            NoiseSweepType::Decade | NoiseSweepType::Octave | NoiseSweepType::Linear => {
                let num_points = parse_positive_usize(&self.points, "noise point count")?;
                let start_freq = parse_positive(&self.fstart, "noise start frequency")?;
                let stop_freq = parse_positive(&self.fstop, "noise stop frequency")?;
                if stop_freq < start_freq {
                    return Err(
                        "noise stop frequency must be at least the start frequency".to_owned()
                    );
                }
                let sweep_type = match self.sweep {
                    NoiseSweepType::Decade => AcSweepType::Decade,
                    NoiseSweepType::Octave => AcSweepType::Octave,
                    NoiseSweepType::Linear => AcSweepType::Linear,
                    _ => unreachable!("matched fixed noise sweep"),
                };
                (sweep_type, num_points, start_freq, stop_freq, None)
            }
            NoiseSweepType::ExplicitFrequencyList => {
                let frequencies = parse_noise_frequency_list(&self.explicit_frequencies)?;
                let start = frequencies.first().copied().unwrap_or_default();
                let stop = frequencies.last().copied().unwrap_or_default();
                (
                    AcSweepType::Decade,
                    frequencies.len(),
                    start,
                    stop,
                    Some(frequencies),
                )
            }
            NoiseSweepType::Unsupported(index) => {
                return Err(format!(
                    "noise sweep mode {index} is outside the supported schema"
                ));
            }
        };

        let config = NoiseAnalysisConfig {
            output_node,
            reference_node,
            input_source,
            sweep_type,
            num_points,
            start_freq,
            stop_freq,
            explicit_frequencies,
            data_table_name: None,
            contribution_detail: self.contribution_detail,
            integration_mode: self.integration_mode,
            temperature_kelvin: rspice_core::constants::TEMP_REFERENCE,
        };
        config
            .validate()
            .map_err(|errors| errors.join("; "))
            .map(|()| config)
    }
}

fn parse_noise_output_expression(
    output: &str,
    legacy_reference: &str,
) -> Result<(String, String), String> {
    let output = output.trim();
    if output.is_empty() {
        return Err("output node or expression is required".to_owned());
    }
    let has_voltage_prefix = output
        .get(..2)
        .is_some_and(|prefix| prefix.eq_ignore_ascii_case("v("));
    if has_voltage_prefix != output.ends_with(')') {
        return Err("voltage output expressions must use V(node) or V(node,reference)".to_owned());
    }
    let inner = if has_voltage_prefix {
        &output[2..output.len() - 1]
    } else {
        output
    };
    let mut nodes = inner.split(',').map(str::trim);
    let positive = nodes.next().unwrap_or_default();
    let explicit_reference = nodes.next();
    if positive.is_empty() || nodes.next().is_some() {
        return Err(
            "output expression must name one node or one differential node pair".to_owned(),
        );
    }
    let reference = explicit_reference.unwrap_or_else(|| legacy_reference.trim());
    if reference.contains(',') || explicit_reference.is_some_and(str::is_empty) {
        return Err("output reference must name exactly one node".to_owned());
    }
    Ok((
        positive.to_owned(),
        if reference.is_empty() {
            default_noise_reference()
        } else {
            reference.to_owned()
        },
    ))
}

fn parse_noise_frequency_list(text: &str) -> Result<Vec<f64>, String> {
    crate::config::parse_explicit_frequency_list(text)
}
