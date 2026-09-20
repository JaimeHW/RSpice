//! Persisted authoring rows for scoped SOA voltage and current limits.

use super::*;
use crate::services::safety::SoAParameter;
use crate::services::simulation_runner::SoaRuleConfig;

const PARAMETERS: [SoAParameter; 24] = [
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
    pub const PARAMETER_LABELS: [&'static str; 24] = [
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
    ];
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
            max_value: config.max_value.to_string(),
            devices: config.devices.join(" "),
            models: config.models.join(" "),
        }
    }

    pub(super) fn to_config(&self) -> Result<SoaRuleConfig, String> {
        let parameter = PARAMETERS
            .get(self.parameter)
            .copied()
            .ok_or("Unknown SOA rule parameter")?;
        let config = SoaRuleConfig {
            parameter,
            max_value: parse_si_value(&self.max_value)
                .map_err(|error| format!("Invalid SOA rule limit: {error}"))?,
            devices: self.devices.split_whitespace().map(str::to_owned).collect(),
            models: self.models.split_whitespace().map(str::to_owned).collect(),
        };
        config.validate()?;
        Ok(config)
    }
}
