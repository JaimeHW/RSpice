//! Retained single-sideband noise figure and its physical source reference.

/// Sideband-zero signal gain with noise from the configured folding window.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NoiseFigureEvidence {
    pub input_source: String,
    pub source_resistor: String,
    pub source_resistance_ohm: f64,
    pub source_temperature_kelvin: f64,
    pub reference_temperature_kelvin: f64,
    pub frequencies: Vec<f64>,
    pub decibels: Vec<f64>,
}

impl NoiseFigureEvidence {
    pub fn validate(&self) -> Result<(), String> {
        if self.input_source.trim().is_empty()
            || self.source_resistor.trim().is_empty()
            || [
                self.source_resistance_ohm,
                self.source_temperature_kelvin,
                self.reference_temperature_kelvin,
            ]
            .into_iter()
            .any(|value| !value.is_finite() || value <= 0.0)
        {
            return Err("Noise figure has an invalid source reference".into());
        }
        if self.frequencies.is_empty()
            || self.frequencies.len() != self.decibels.len()
            || self
                .frequencies
                .iter()
                .any(|value| !value.is_finite() || *value <= 0.0)
            || self.frequencies.windows(2).any(|pair| pair[1] <= pair[0])
            || self.decibels.iter().any(|value| !value.is_finite())
        {
            return Err("Noise figure has an invalid frequency or decibel series".into());
        }
        Ok(())
    }
}
