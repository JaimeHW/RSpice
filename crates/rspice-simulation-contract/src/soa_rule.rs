//! Portable authored safety limits and their validation.

use crate::soa_observation::SoaObservationConfig;
use rspice_results::safety::{SoAParameter, SoaVoltageBasis};

/// An additional limit, or an override of a default voltage limit on its scope.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SoaRuleConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_envelope: Option<rspice_results::safety::SoaCurrentEnvelope>,
    #[serde(
        default,
        skip_serializing_if = "rspice_results::safety::SoaDurationMode::is_default"
    )]
    pub duration_mode: rspice_results::safety::SoaDurationMode,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minimum_duration_s: Option<f64>,
    #[serde(default)]
    pub power_derating: Option<rspice_results::safety::SoaPowerDerating>,
    pub parameter: SoAParameter,
    #[serde(default)]
    pub voltage_basis: SoaVoltageBasis,
    /// Maximum stress in SI units; temperature is absolute kelvin.
    pub max_value: f64,
    #[serde(default)]
    pub devices: Vec<String>,
    #[serde(default)]
    pub models: Vec<String>,
}

impl SoaRuleConfig {
    pub fn validate(&self) -> Result<(), String> {
        if let Some(curve) = &self.current_envelope {
            curve.validate()?;
            if self.max_value <= 0.0 {
                return Err(
                    "SOA current/voltage curves require a positive maximum-current cap".into(),
                );
            }
            if rspice_results::safety::SoaCurrentEnvelope::voltage_parameter(self.parameter)
                .is_none()
            {
                return Err("SOA current/voltage curves apply to Id, Ic or Ia (including directional rules)".into());
            }
        }
        self.duration_mode.validate(self.minimum_duration_s)?;
        if !matches!(
            self.parameter.base_parameter(),
            SoAParameter::Vgs
                | SoAParameter::Vds
                | SoAParameter::Vgd
                | SoAParameter::Vbe
                | SoAParameter::Vce
                | SoAParameter::Vbc
                | SoAParameter::Id
                | SoAParameter::Ic
                | SoAParameter::Ig
                | SoAParameter::Is
                | SoAParameter::Ib
                | SoAParameter::Ie
                | SoAParameter::Temp
                | SoAParameter::Pdiss
                | SoAParameter::Vcsub
                | SoAParameter::Vbsub
                | SoAParameter::Vesub
                | SoAParameter::Isub
                | SoAParameter::Vak
                | SoAParameter::Ia
                | SoAParameter::Vbs
                | SoAParameter::Vbd
                | SoAParameter::Vgb
                | SoAParameter::Ibulk
                | SoAParameter::Ves
                | SoAParameter::Ved
                | SoAParameter::Vge
                | SoAParameter::Ibackgate
                | SoAParameter::VbodyBackgate
        ) {
            return Err(
                "SOA rules support magnitudes and positive/negative limits for Vgs, Vds, Vgd, Vbe, Vce, Vbc, Id, Ig, Is, Ic, Ib and Ie, external body/back-gate/substrate voltage and current, diode anode voltage/current, plus conductive power and absolute operating temperature".into(),
            );
        }
        if !self.max_value.is_finite()
            || self.max_value < 0.0
            || (self.max_value == 0.0 && self.parameter.polarity().is_none())
        {
            return Err("SOA rule limit must be finite and positive (zero is allowed for directional rules)".into());
        }
        if self.voltage_basis == SoaVoltageBasis::IntrinsicNodes
            && self.parameter.intrinsic_voltage_parameter().is_none()
        {
            return Err("Intrinsic voltage basis applies only to voltage rules".into());
        }
        if let Some(curve) = self.power_derating {
            curve.validate()?;
            if self.parameter != SoAParameter::Pdiss {
                return Err("SOA temperature derating applies only to conductive power".into());
            }
        }
        self.scope().validate(1.0)
    }

    pub fn scope(&self) -> SoaObservationConfig {
        SoaObservationConfig {
            devices: self.devices.clone(),
            models: self.models.clone(),
            ..Default::default()
        }
    }
}
