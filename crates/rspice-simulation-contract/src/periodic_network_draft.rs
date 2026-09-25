//! Shared PSP/HBSP authoring, including frequency-converting noise references.
use crate::drafts::FrequencySweepDraft;
use crate::drafts::parse::{parse_nonnegative, parse_positive, parse_positive_usize};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NetworkPortDraft {
    pub node_pos: String,
    pub node_neg: String,
    pub z0: String,
}

impl Default for NetworkPortDraft {
    fn default() -> Self {
        Self {
            node_pos: "in".to_owned(),
            node_neg: "0".to_owned(),
            z0: "50".to_owned(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PeriodicNetworkDraft {
    pub sweep: FrequencySweepDraft,
    pub ports: Vec<NetworkPortDraft>,
    pub max_sideband: String,
    /// Relative convergence tolerance for the periodic small-signal solve.
    #[serde(default = "default_periodic_network_reltol")]
    pub reltol: String,
    /// Absolute convergence tolerance for the periodic small-signal solve.
    #[serde(default = "default_periodic_network_abstol")]
    pub abstol: String,
    pub mixed_mode: bool,
    pub noise_parameters: bool,
    #[serde(default)]
    pub noise: PeriodicNetworkNoiseDraft,
}

impl Default for PeriodicNetworkDraft {
    fn default() -> Self {
        let output_port = NetworkPortDraft {
            node_pos: "out".to_owned(),
            ..NetworkPortDraft::default()
        };
        Self {
            sweep: FrequencySweepDraft::default(),
            ports: vec![NetworkPortDraft::default(), output_port],
            // A ±4 sideband span needs harmonic coupling through order 8,
            // which fits the default nine-harmonic HB producer.
            max_sideband: "4".to_owned(),
            reltol: default_periodic_network_reltol(),
            abstol: default_periodic_network_abstol(),
            mixed_mode: false,
            noise_parameters: false,
            noise: PeriodicNetworkNoiseDraft::default(),
        }
    }
}

fn default_periodic_network_reltol() -> String {
    "1e-3".into()
}

fn default_periodic_network_abstol() -> String {
    "1e-12".into()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PeriodicNetworkNoiseDraft {
    pub report_parameters: bool,
    pub input_port: String,
    pub output_port: String,
    pub input_sideband: String,
    pub output_sideband: String,
    pub reference_temperature: String,
    pub termination_temperature: String,
    pub image_sideband: String,
}

impl Default for PeriodicNetworkNoiseDraft {
    fn default() -> Self {
        Self {
            report_parameters: false,
            input_port: "1".into(),
            output_port: "2".into(),
            input_sideband: "0".into(),
            output_sideband: "0".into(),
            reference_temperature: "290".into(),
            termination_temperature: "290".into(),
            image_sideband: String::new(),
        }
    }
}

impl PeriodicNetworkDraft {
    pub fn noise_reference(
        &self,
    ) -> Result<Option<rspice_core::analysis::s_param::PeriodicPortNoiseReference>, String> {
        if !self.noise_parameters || !self.noise.report_parameters {
            return Ok(None);
        }
        let sideband = |value: &str, label: &str| {
            value
                .trim()
                .parse::<i32>()
                .map_err(|_| format!("{label} must be a signed integer"))
        };
        let reference = rspice_core::analysis::s_param::PeriodicPortNoiseReference {
            input_port: parse_positive_usize(&self.noise.input_port, "noise input port")?,
            output_port: parse_positive_usize(&self.noise.output_port, "noise output port")?,
            input_sideband: sideband(&self.noise.input_sideband, "noise input sideband")?,
            output_sideband: sideband(&self.noise.output_sideband, "noise output sideband")?,
            reference_temperature_kelvin: parse_positive(
                &self.noise.reference_temperature,
                "noise reference temperature",
            )?,
            termination_temperature_kelvin: parse_nonnegative(
                &self.noise.termination_temperature,
                "unused-channel temperature",
            )?,
            image_sideband: if self.noise.image_sideband.trim().is_empty() {
                None
            } else {
                Some(sideband(&self.noise.image_sideband, "DSB image sideband")?)
            },
        };
        let max = self
            .max_sideband
            .trim()
            .parse::<i32>()
            .map_err(|_| "maximum sideband must be a nonnegative integer")?;
        if max < 0 {
            return Err("maximum sideband must be a nonnegative integer".into());
        }
        reference.validate(
            (!self.ports.is_empty()).then_some(self.ports.len()),
            -max,
            max,
        )?;
        Ok(Some(reference))
    }
}

pub fn validate_periodic_network(draft: &PeriodicNetworkDraft) -> Option<String> {
    (|| {
        let start = parse_positive(&draft.sweep.start, "start frequency")?;
        let stop = parse_positive(&draft.sweep.stop, "stop frequency")?;
        if stop < start { return Err("stop frequency must be at least start frequency".into()); }
        parse_positive_usize(&draft.sweep.points, "sweep point count")?;
        if draft.sweep.sweep > 2 { return Err("frequency sweep mode is outside the supported schema".into()); }
        if draft
            .max_sideband
            .trim()
            .parse::<u32>()
            .ok()
            .is_none_or(|value| value > i32::MAX as u32)
        {
            return Err("maximum sideband must be an integer from 0 to 2147483647".to_owned());
        }
        parse_positive(&draft.reltol, "relative tolerance")?;
        parse_positive(&draft.abstol, "absolute tolerance")?;
        for (index, port) in draft.ports.iter().enumerate() {
            if port.node_pos.trim().is_empty() || port.node_neg.trim().is_empty() {
                return Err(format!("port {} requires both nodes", index + 1));
            }
            parse_positive(&port.z0, &format!("port {} reference impedance", index + 1))?;
        }
        if draft.mixed_mode {
            if !draft.ports.len().is_multiple_of(2) {
                return Err(
                    "periodic mixed-mode conversion requires an even number of physical ports paired in declaration order"
                        .to_owned(),
                );
            }
            for (pair_index, pair) in draft.ports.chunks_exact(2).enumerate() {
                let positive_z0 = parse_positive(
                    &pair[0].z0,
                    &format!("mixed-mode pair {} positive reference impedance", pair_index + 1),
                )?;
                let negative_z0 = parse_positive(
                    &pair[1].z0,
                    &format!("mixed-mode pair {} negative reference impedance", pair_index + 1),
                )?;
                if positive_z0.to_bits() != negative_z0.to_bits() {
                    return Err(format!(
                        "periodic mixed-mode pair {} has unequal reference impedances ({} and {} ohm)",
                        pair_index + 1,
                        positive_z0,
                        negative_z0
                    ));
                }
            }
        }
        draft.noise_reference()?;
        Ok(())
    })()
    .err()
}

pub fn validate_psp_network(draft: &PeriodicNetworkDraft) -> Option<String> {
    validate_periodic_network(draft)
}
