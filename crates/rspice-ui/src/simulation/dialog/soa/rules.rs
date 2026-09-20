//! Persisted authoring rows for scoped SOA voltage, current, power and temperature limits.

use super::*;
use crate::services::safety::SoAParameter;
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
