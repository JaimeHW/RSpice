//! Persisted authoring rows for scoped SOA voltage, current, power and temperature limits.

use super::*;
use crate::services::safety::{SoAParameter, SoaDurationMode};
use crate::services::simulation_runner::SoaRuleConfig;

const PARAMETERS: [SoAParameter; 83] = [
    SoAParameter::Vgs,
    SoAParameter::Vds,
    SoAParameter::Vgd,
    SoAParameter::Vbe,
    SoAParameter::Vce,
    SoAParameter::Vbc,
    SoAParameter::Id,
    SoAParameter::Ic,
    SoAParameter::VgsPositive,
    SoAParameter::VgsNegative,
    SoAParameter::VdsPositive,
    SoAParameter::VdsNegative,
    SoAParameter::VgdPositive,
    SoAParameter::VgdNegative,
    SoAParameter::VbePositive,
    SoAParameter::VbeNegative,
    SoAParameter::VcePositive,
    SoAParameter::VceNegative,
    SoAParameter::VbcPositive,
    SoAParameter::VbcNegative,
    SoAParameter::IdPositive,
    SoAParameter::IdNegative,
    SoAParameter::IcPositive,
    SoAParameter::IcNegative,
    SoAParameter::Temp,
    SoAParameter::Pdiss,
    SoAParameter::Ig,
    SoAParameter::IgPositive,
    SoAParameter::IgNegative,
    SoAParameter::Is,
    SoAParameter::IsPositive,
    SoAParameter::IsNegative,
    SoAParameter::Ib,
    SoAParameter::IbPositive,
    SoAParameter::IbNegative,
    SoAParameter::Ie,
    SoAParameter::IePositive,
    SoAParameter::IeNegative,
    SoAParameter::Vbs,
    SoAParameter::VbsPositive,
    SoAParameter::VbsNegative,
    SoAParameter::Vbd,
    SoAParameter::VbdPositive,
    SoAParameter::VbdNegative,
    SoAParameter::Vgb,
    SoAParameter::VgbPositive,
    SoAParameter::VgbNegative,
    SoAParameter::Ibulk,
    SoAParameter::IbulkPositive,
    SoAParameter::IbulkNegative,
    SoAParameter::Ves,
    SoAParameter::VesPositive,
    SoAParameter::VesNegative,
    SoAParameter::Ved,
    SoAParameter::VedPositive,
    SoAParameter::VedNegative,
    SoAParameter::Vge,
    SoAParameter::VgePositive,
    SoAParameter::VgeNegative,
    SoAParameter::Ibackgate,
    SoAParameter::IbackgatePositive,
    SoAParameter::IbackgateNegative,
    SoAParameter::VbodyBackgate,
    SoAParameter::VbodyBackgatePositive,
    SoAParameter::VbodyBackgateNegative,
    SoAParameter::Vcsub,
    SoAParameter::VcsubPositive,
    SoAParameter::VcsubNegative,
    SoAParameter::Vbsub,
    SoAParameter::VbsubPositive,
    SoAParameter::VbsubNegative,
    SoAParameter::Vesub,
    SoAParameter::VesubPositive,
    SoAParameter::VesubNegative,
    SoAParameter::Isub,
    SoAParameter::IsubPositive,
    SoAParameter::IsubNegative,
    SoAParameter::Vak,
    SoAParameter::VakPositive,
    SoAParameter::VakNegative,
    SoAParameter::Ia,
    SoAParameter::IaPositive,
    SoAParameter::IaNegative,
];

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SoaRuleDraft {
    #[serde(default)]
    pub current_envelope: super::SoaEnvelopeDraft,
    #[serde(default)]
    pub cumulative_duration: bool,
    #[serde(default)]
    pub recovery_time: String,
    #[serde(default)]
    pub minimum_duration: String,
    #[serde(default)]
    pub derate_power: bool,
    #[serde(default = "default_derating_temperature")]
    pub derating_temperature_celsius: String,
    #[serde(default = "default_derating_slope")]
    pub derating_watts_per_kelvin: String,
    #[serde(default)]
    pub intrinsic_voltage: bool,
    pub parameter: usize,
    pub max_value: String,
    pub devices: String,
    pub models: String,
}

fn default_derating_temperature() -> String {
    "25".into()
}
fn default_derating_slope() -> String {
    "1m".into()
}

impl Default for SoaRuleDraft {
    fn default() -> Self {
        Self {
            current_envelope: Default::default(),
            cumulative_duration: false,
            recovery_time: String::new(),
            minimum_duration: String::new(),
            derate_power: false,
            derating_temperature_celsius: default_derating_temperature(),
            derating_watts_per_kelvin: default_derating_slope(),
            intrinsic_voltage: false,
            parameter: 2,
            max_value: "1.8".into(),
            devices: String::new(),
            models: String::new(),
        }
    }
}

impl SoaRuleDraft {
    pub fn supports_current_envelope(&self) -> bool {
        PARAMETERS.get(self.parameter).is_some_and(|p| {
            crate::services::safety::SoaCurrentEnvelope::voltage_parameter(*p).is_some()
        })
    }
    pub const PARAMETER_LABELS: [&'static str; 83] = [
        "Vgs",
        "Vds",
        "Vgd",
        "Vbe",
        "Vce",
        "Vbc",
        "Id",
        "Ic",
        "Vgs positive",
        "Vgs negative",
        "Vds positive",
        "Vds negative",
        "Vgd positive",
        "Vgd negative",
        "Vbe positive",
        "Vbe negative",
        "Vce positive",
        "Vce negative",
        "Vbc positive",
        "Vbc negative",
        "Id positive",
        "Id negative",
        "Ic positive",
        "Ic negative",
        "Operating temperature",
        "Conductive power dissipation",
        "Ig",
        "Ig positive",
        "Ig negative",
        "Is",
        "Is positive",
        "Is negative",
        "Ib",
        "Ib positive",
        "Ib negative",
        "Ie",
        "Ie positive",
        "Ie negative",
        "Vbs",
        "Vbs positive",
        "Vbs negative",
        "Vbd",
        "Vbd positive",
        "Vbd negative",
        "Vgb",
        "Vgb positive",
        "Vgb negative",
        "Bulk / body-contact current",
        "Bulk / body-contact current positive",
        "Bulk / body-contact current negative",
        "Ves (back gate to source)",
        "Ves (back gate to source) positive",
        "Ves (back gate to source) negative",
        "Ved (back gate to drain)",
        "Ved (back gate to drain) positive",
        "Ved (back gate to drain) negative",
        "Vge (gate to back gate)",
        "Vge (gate to back gate) positive",
        "Vge (gate to back gate) negative",
        "Back-gate current",
        "Back-gate current positive",
        "Back-gate current negative",
        "Body contact to back gate voltage",
        "Body contact to back gate voltage positive",
        "Body contact to back gate voltage negative",
        "Collector to substrate voltage",
        "Collector to substrate voltage positive",
        "Collector to substrate voltage negative",
        "Base to substrate voltage",
        "Base to substrate voltage positive",
        "Base to substrate voltage negative",
        "Emitter to substrate voltage",
        "Emitter to substrate voltage positive",
        "Emitter to substrate voltage negative",
        "Substrate current",
        "Substrate current positive",
        "Substrate current negative",
        "Diode anode to cathode voltage",
        "Diode anode to cathode voltage positive",
        "Diode anode to cathode voltage negative",
        "Diode anode current",
        "Diode anode current positive",
        "Diode anode current negative",
    ];
    pub fn is_voltage(&self) -> bool {
        PARAMETERS
            .get(self.parameter)
            .is_some_and(|p| p.intrinsic_voltage_parameter().is_some())
    }

    pub fn is_power(&self) -> bool {
        PARAMETERS.get(self.parameter) == Some(&SoAParameter::Pdiss)
    }

    pub fn is_temperature(&self) -> bool {
        PARAMETERS.get(self.parameter) == Some(&SoAParameter::Temp)
    }

    pub fn is_current(&self) -> bool {
        PARAMETERS
            .get(self.parameter)
            .is_some_and(|p| p.is_current())
    }

    pub(super) fn from_config(config: &SoaRuleConfig) -> Self {
        Self {
            current_envelope: super::SoaEnvelopeDraft::from_config(
                config.current_envelope.as_ref(),
            ),
            cumulative_duration: matches!(config.duration_mode, SoaDurationMode::Cumulative { .. }),
            recovery_time: match config.duration_mode {
                SoaDurationMode::Cumulative {
                    recovery_time_s: Some(value),
                } => value.to_string(),
                _ => String::new(),
            },
            minimum_duration: config
                .minimum_duration_s
                .map(|value| value.to_string())
                .unwrap_or_default(),
            derate_power: config.power_derating.is_some(),
            derating_temperature_celsius: config
                .power_derating
                .map(|curve| {
                    rspice_core::constants::kelvin_to_celsius(curve.reference_temperature_kelvin)
                        .to_string()
                })
                .unwrap_or_else(default_derating_temperature),
            derating_watts_per_kelvin: config
                .power_derating
                .map(|curve| curve.watts_per_kelvin.to_string())
                .unwrap_or_else(default_derating_slope),
            intrinsic_voltage: config.voltage_basis
                == crate::services::safety::SoaVoltageBasis::IntrinsicNodes,
            parameter: PARAMETERS
                .iter()
                .position(|parameter| *parameter == config.parameter)
                .unwrap_or(usize::MAX),
            max_value: if config.parameter == SoAParameter::Temp {
                rspice_core::constants::kelvin_to_celsius(config.max_value).to_string()
            } else {
                config.max_value.to_string()
            },
            devices: config.devices.join(" "),
            models: config.models.join(" "),
        }
    }

    pub(super) fn to_config(&self) -> Result<SoaRuleConfig, String> {
        let parameter = PARAMETERS
            .get(self.parameter)
            .copied()
            .ok_or("Unknown SOA rule parameter")?;
        let authored = parse_si_value(&self.max_value)
            .map_err(|error| format!("Invalid SOA rule limit: {error}"))?;
        let maximum = if parameter == SoAParameter::Temp {
            let kelvin = rspice_core::constants::celsius_to_kelvin(authored);
            if !kelvin.is_finite() || kelvin <= 0.0 {
                return Err("SOA maximum temperature must be above -273.15 °C".into());
            }
            kelvin
        } else {
            authored
        };
        let config = SoaRuleConfig {
            duration_mode: if self.cumulative_duration && !self.minimum_duration.trim().is_empty() {
                SoaDurationMode::Cumulative {
                    recovery_time_s: if self.recovery_time.trim().is_empty() {
                        None
                    } else {
                        Some(
                            parse_si_value(&self.recovery_time)
                                .map_err(|error| format!("Invalid SOA recovery time: {error}"))?,
                        )
                    },
                }
            } else {
                SoaDurationMode::PerExcursion
            },
            minimum_duration_s: if self.minimum_duration.trim().is_empty() {
                None
            } else {
                Some(
                    parse_si_value(&self.minimum_duration)
                        .map_err(|error| format!("Invalid SOA duration threshold: {error}"))?,
                )
            },
            current_envelope: if self.supports_current_envelope() {
                self.current_envelope.to_config()?
            } else {
                None
            },
            power_derating: if self.is_power() && self.derate_power {
                Some(crate::services::safety::SoaPowerDerating {
                    reference_temperature_kelvin: rspice_core::constants::celsius_to_kelvin(
                        parse_si_value(&self.derating_temperature_celsius).map_err(|e| {
                            format!("Invalid SOA derating reference temperature: {e}")
                        })?,
                    ),
                    watts_per_kelvin: parse_si_value(&self.derating_watts_per_kelvin)
                        .map_err(|e| format!("Invalid SOA derating slope: {e}"))?,
                })
            } else {
                None
            },
            voltage_basis: if self.is_voltage() && self.intrinsic_voltage {
                crate::services::safety::SoaVoltageBasis::IntrinsicNodes
            } else {
                Default::default()
            },
            parameter,
            max_value: maximum,
            devices: self.devices.split_whitespace().map(str::to_owned).collect(),
            models: self.models.split_whitespace().map(str::to_owned).collect(),
        };
        config.validate()?;
        Ok(config)
    }
}
