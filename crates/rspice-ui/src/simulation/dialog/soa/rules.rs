//! Persisted authoring rows for scoped SOA voltage and current limits.

use super::*;
use crate::services::safety::SoAParameter;
use crate::services::simulation_runner::SoaRuleConfig;

const PARAMETERS: [SoAParameter; 8] = [
    SoAParameter::Vgs,
    SoAParameter::Vds,
    SoAParameter::Vgd,
    SoAParameter::Vbe,
    SoAParameter::Vce,
    SoAParameter::Vbc,
    SoAParameter::Id,
    SoAParameter::Ic,
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
