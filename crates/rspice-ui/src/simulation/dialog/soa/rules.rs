//! Persisted authoring rows for scoped SOA voltage, current and temperature limits.

use super::*;
use crate::services::safety::SoAParameter;
use crate::services::simulation_runner::SoaRuleConfig;

const PARAMETERS: [SoAParameter; 25] = [
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
];

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SoaRuleDraft {
    pub parameter: usize,
    pub max_value: String,
    pub devices: String,
    pub models: String,
}

impl Default for SoaRuleDraft {
    fn default() -> Self {
        Self {
            parameter: 2,
            max_value: "1.8".into(),
            devices: String::new(),
            models: String::new(),
        }
    }
}

impl SoaRuleDraft {
    pub const PARAMETER_LABELS: [&'static str; 25] = [
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
    ];
    pub fn is_temperature(&self) -> bool {
        PARAMETERS.get(self.parameter) == Some(&SoAParameter::Temp)
    }

    pub fn is_current(&self) -> bool {
        PARAMETERS
            .get(self.parameter)
            .is_some_and(|p| matches!(p.base_parameter(), SoAParameter::Id | SoAParameter::Ic))
    }

    pub(super) fn from_config(config: &SoaRuleConfig) -> Self {
        Self {
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
            parameter,
            max_value: maximum,
            devices: self.devices.split_whitespace().map(str::to_owned).collect(),
            models: self.models.split_whitespace().map(str::to_owned).collect(),
        };
        config.validate()?;
        Ok(config)
    }
}
