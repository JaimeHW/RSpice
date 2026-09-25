//! Portable harmonic-balance noise reference and sweep admission.

use rspice_core::Value;

/// Exact retained-HB noise request.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HbNoiseReference {
    pub source_resistor: String,
    pub temperature_kelvin: Value,
}

impl HbNoiseReference {
    pub fn validate(&self) -> Result<(), String> {
        if self.source_resistor.trim().is_empty()
            || self.source_resistor.chars().any(char::is_whitespace)
        {
            return Err("HBNOISE noise figure requires one source resistor name".into());
        }
        if !self.temperature_kelvin.is_finite() || self.temperature_kelvin <= 0.0 {
            return Err("HBNOISE reference temperature must be finite and positive kelvin".into());
        }
        Ok(())
    }
}

/// Shared authoring and execution bounds, including a one-frequency spectrum.
pub fn validate_hbnoise_frequency_options(
    start: Value,
    stop: Value,
    points: usize,
    linear: bool,
    max_sideband: usize,
    band_evidence: bool,
) -> Result<(), String> {
    if !start.is_finite() || start <= 0.0 || !stop.is_finite() || stop < start {
        return Err("HBNOISE frequencies must be finite with 0 < start <= stop".into());
    }
    if points == 0 {
        return Err("HBNOISE points per unit must be greater than zero".into());
    }
    if max_sideband > i32::MAX as usize {
        return Err("HBNOISE maximum sideband must be within 0..=2147483647".into());
    }
    if band_evidence && (start == stop || (linear && points == 1)) {
        return Err("HBNOISE integrated noise and contributor ranking require at least two distinct frequencies; disable both for a spot spectrum".into());
    }
    Ok(())
}
