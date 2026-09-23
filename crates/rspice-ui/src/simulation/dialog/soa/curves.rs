//! Editable numeric rows for datasheet SOA boundaries.
use super::parse_si_value;
use crate::services::safety::{
    SoaCurrentEnvelope, SoaPulseCurve, SoaPulseInterpolation, SoaVoltageInterpolation,
};

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct SoaEnvelopeDraft {
    pub enabled: bool,
    pub source: String,
    pub conditions: String,
    pub voltages: String,
    pub dc_currents: String,
    pub pulse_width: String,
    pub logarithmic_voltage: bool,
    pub interpolate_pulses: bool,
    pub pulses: Vec<SoaPulseDraft>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SoaPulseDraft {
    pub duration: String,
    pub currents: String,
}

fn numbers(text: &str) -> Result<Vec<f64>, String> {
    text.split(|c: char| c.is_whitespace() || c == ',')
        .filter(|s| !s.is_empty())
        .map(|value| {
            parse_si_value(value).map_err(|e| format!("Invalid SOA curve value '{value}': {e}"))
        })
        .collect()
}
fn text(values: &[f64]) -> String {
    values
        .iter()
        .map(f64::to_string)
        .collect::<Vec<_>>()
        .join(" ")
}

impl SoaEnvelopeDraft {
    pub fn from_config(curve: Option<&SoaCurrentEnvelope>) -> Self {
        let Some(curve) = curve else {
            return Self::default();
        };
        Self {
            enabled: true,
            source: curve.source.clone(),
            conditions: curve.conditions.clone(),
            voltages: text(&curve.voltages_v),
            dc_currents: curve
                .dc_currents_a
                .as_ref()
                .map_or_else(String::new, |v| text(v)),
            pulse_width: curve
                .pulse_width_s
                .map(|v| v.to_string())
                .unwrap_or_default(),
            logarithmic_voltage: curve.voltage_interpolation
                == SoaVoltageInterpolation::Logarithmic,
            interpolate_pulses: curve.pulse_interpolation == SoaPulseInterpolation::Logarithmic,
            pulses: curve
                .pulses
                .iter()
                .map(|p| SoaPulseDraft {
                    duration: p.duration_s.to_string(),
                    currents: text(&p.currents_a),
                })
                .collect(),
        }
    }

    pub fn to_config(&self) -> Result<Option<SoaCurrentEnvelope>, String> {
        if !self.enabled {
            return Ok(None);
        }
        let curve = SoaCurrentEnvelope {
            source: self.source.trim().into(),
            conditions: self.conditions.trim().into(),
            voltages_v: numbers(&self.voltages)?,
            dc_currents_a: if self.dc_currents.trim().is_empty() {
                None
            } else {
                Some(numbers(&self.dc_currents)?)
            },
            pulse_width_s: if self.pulse_width.trim().is_empty() {
                None
            } else {
                Some(
                    parse_si_value(&self.pulse_width)
                        .map_err(|e| format!("Invalid SOA pulse width: {e}"))?,
                )
            },
            voltage_interpolation: if self.logarithmic_voltage {
                SoaVoltageInterpolation::Logarithmic
            } else {
                SoaVoltageInterpolation::Linear
            },
            pulse_interpolation: if self.interpolate_pulses {
                SoaPulseInterpolation::Logarithmic
            } else {
                SoaPulseInterpolation::LongerPulse
            },
            pulses: self
                .pulses
                .iter()
                .map(|p| {
                    Ok(SoaPulseCurve {
                        duration_s: parse_si_value(&p.duration)
                            .map_err(|e| format!("Invalid SOA curve duration: {e}"))?,
                        currents_a: numbers(&p.currents)?,
                    })
                })
                .collect::<Result<_, String>>()?,
        };
        curve.validate()?;
        Ok(Some(curve))
    }
}
