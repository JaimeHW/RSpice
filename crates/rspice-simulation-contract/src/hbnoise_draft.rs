//! Lossless HB-noise draft and its shared solver-facing validation.

use crate::config::validate_noise_sidebands;
use crate::drafts::FrequencySweepDraft;
use crate::drafts::parse::{parse_positive, parse_positive_usize};
use crate::hbnoise_policy::{HbNoiseReference, validate_hbnoise_frequency_options};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HbNoiseDraft {
    #[serde(default = "default_noise_sideband")]
    pub input_sideband: String,
    #[serde(default = "default_noise_sideband")]
    pub output_sideband: String,
    #[serde(default)]
    pub source_resistor: String,
    #[serde(default = "default_noise_reference_temperature")]
    pub reference_temperature: String,
    pub sweep: FrequencySweepDraft,
    pub output_node: String,
    pub output_ref: String,
    pub input_source: String,
    pub max_sideband: String,
    pub integrated_noise: bool,
    pub noise_figure: bool,
    pub contributor_ranking: bool,
}

impl Default for HbNoiseDraft {
    fn default() -> Self {
        Self {
            input_sideband: default_noise_sideband(),
            output_sideband: default_noise_sideband(),
            source_resistor: String::new(),
            reference_temperature: default_noise_reference_temperature(),
            sweep: FrequencySweepDraft::default(),
            output_node: "out".to_owned(),
            output_ref: "0".to_owned(),
            input_source: "V1".to_owned(),
            // A ±4 sideband span fits the default nine-harmonic HB producer;
            // larger spans remain available when the producer retains them.
            max_sideband: "4".to_owned(),
            integrated_noise: true,
            // The user must select the circuit's actual source resistor
            // before enabling the reference-temperature calculation.
            noise_figure: false,
            contributor_ranking: true,
        }
    }
}

fn default_noise_reference_temperature() -> String {
    "290".into()
}

fn default_noise_sideband() -> String {
    "0".into()
}

impl HbNoiseDraft {
    pub fn sidebands(&self) -> Result<(i32, i32), String> {
        let input = self
            .input_sideband
            .trim()
            .parse::<i32>()
            .map_err(|_| "Input sideband must be a signed integer")?;
        let output = self
            .output_sideband
            .trim()
            .parse::<i32>()
            .map_err(|_| "Output sideband must be a signed integer")?;
        let maximum = self
            .max_sideband
            .trim()
            .parse::<usize>()
            .map_err(|_| "Maximum sideband must be a nonnegative integer")?;
        validate_noise_sidebands(input, output, maximum)?;
        Ok((input, output))
    }
    pub fn noise_reference(&self) -> Result<Option<HbNoiseReference>, String> {
        if !self.noise_figure {
            return Ok(None);
        }
        let reference = HbNoiseReference {
            source_resistor: self.source_resistor.trim().into(),
            temperature_kelvin: parse_positive(
                &self.reference_temperature,
                "noise reference temperature (K)",
            )?,
        };
        reference.validate()?;
        Ok(Some(reference))
    }
}

pub fn validate_hbnoise(draft: &HbNoiseDraft) -> Option<String> {
    (|| {
        if draft.sweep.sweep > 2 {
            return Err("frequency sweep mode is outside the supported schema".to_owned());
        }
        let max_sideband =
            draft.max_sideband.trim().parse::<usize>().map_err(|_| {
                "maximum sideband must be an integer from 0 to 2147483647".to_owned()
            })?;
        validate_hbnoise_frequency_options(
            parse_positive(&draft.sweep.start, "start frequency")?,
            parse_positive(&draft.sweep.stop, "stop frequency")?,
            parse_positive_usize(&draft.sweep.points, "sweep point count")?,
            draft.sweep.sweep == 2,
            max_sideband,
            draft.integrated_noise || draft.contributor_ranking,
        )?;
        if draft.output_node.trim().is_empty() {
            return Err("HBNOISE requires an output node".to_owned());
        }
        if draft.input_source.trim().is_empty() {
            return Err("HBNOISE requires an input source".to_owned());
        }
        draft.sidebands()?;
        draft.noise_reference()?;
        Ok(())
    })()
    .err()
}
