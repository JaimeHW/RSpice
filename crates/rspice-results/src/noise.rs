//! Retained noise summaries, source references, and frequency conversion evidence.

/// Selected-channel signal gain with noise from the configured folding window.
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

/// Physical frequency channels for a spectrum whose stored axis is offset Hz.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PeriodicNoiseConversionEvidence {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sampling: Option<rspice_core::analysis::pnoise::PeriodicNoiseSamplingEvidence>,
    /// Empty when only output noise was requested, without input referral.
    pub input_source: String,
    pub carrier_hz: f64,
    pub input_sideband: i32,
    pub output_sideband: i32,
    pub max_sideband: i32,
}

impl PeriodicNoiseConversionEvidence {
    pub fn validate(&self, band: (f64, f64)) -> Result<(), String> {
        if let Some(sampling) = &self.sampling {
            sampling.validate()?;
            if sampling.carrier_frequency_hz != self.carrier_hz
                || self.output_sideband != 0
                || band.1 > self.carrier_hz / 2.0
            {
                return Err(
                    "Sampled noise has an inconsistent carrier, output sideband or offset band"
                        .into(),
                );
            }
        }
        if (self.input_source.trim().is_empty() && self.input_sideband != 0)
            || !self.carrier_hz.is_finite()
            || self.carrier_hz <= 0.0
            || self.max_sideband < 0
            || self.input_sideband.unsigned_abs() > self.max_sideband as u32
            || self.output_sideband.unsigned_abs() > self.max_sideband as u32
            || [band.0, band.1].into_iter().any(|offset| {
                !offset.is_finite()
                    || offset <= 0.0
                    || !self.input_frequency(offset).is_finite()
                    || !self.output_frequency(offset).is_finite()
            })
            || band.1 < band.0
        {
            return Err("Periodic noise has invalid conversion channels or offset band".into());
        }
        Ok(())
    }

    pub fn input_frequency(&self, offset: f64) -> f64 {
        offset + f64::from(self.input_sideband) * self.carrier_hz
    }

    pub fn output_frequency(&self, offset: f64) -> f64 {
        offset + f64::from(self.output_sideband) * self.carrier_hz
    }
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

/// One row of the ranked noise-contributor table (band-integrated).
#[derive(Debug, Clone, PartialEq)]
pub struct NoiseContributorRow {
    /// Device instance name.
    pub device: String,
    /// Noise mechanism label ("thermal", "flicker", "shot", "burst").
    pub mechanism: String,
    /// Output-referred noise power integrated over the band (V²).
    pub power: f64,
    /// Share of the total integrated output noise (percent).
    pub share_pct: f64,
}

impl NoiseSummary {
    pub fn is_timing(&self) -> bool {
        self.conversion
            .as_ref()
            .and_then(|conversion| conversion.sampling.as_ref())
            .is_some_and(|sampling| sampling.request.is_timing())
    }
    pub fn power_unit(&self) -> &'static str {
        if self.is_timing() { "s²" } else { "V²" }
    }
}

/// Ranked noise summary for a noise analysis: per-device/mechanism
/// contributions plus the band total — the table analog designers read
/// first.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct NoiseSummary {
    pub input_quantity: Option<rspice_core::analysis::noise::NoiseInputQuantity>,
    pub conversion: Option<PeriodicNoiseConversionEvidence>,
    pub noise_figure: Option<std::sync::Arc<NoiseFigureEvidence>>,
    /// Contributors, ranked by integrated power, descending.
    pub rows: Vec<NoiseContributorRow>,
    /// Total integrated output noise over the band (V rms). `None` means the
    /// selected execution policy intentionally omitted this evidence.
    pub total_rms: Option<f64>,
    /// Total integrated input-referred noise, in the resolved source quantity, retained when the
    /// named-source normalization was validated and the policy requested it.
    pub input_rms: Option<f64>,
    /// Analysis band, for the panel header (Hz).
    pub band: (f64, f64),
}

impl NoiseSummary {
    /// Legacy results may lack the source quantity; do not invent a voltage unit for them.
    pub fn input_rms_unit(&self) -> &'static str {
        self.input_quantity
            .map_or("RMS (unit not retained)", |quantity| quantity.rms_unit())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn periodic_noise_conversion_rejects_invalid_channels_and_frequency_overflow() {
        let valid = PeriodicNoiseConversionEvidence {
            sampling: None,
            input_source: "V1".into(),
            carrier_hz: 1e6,
            input_sideband: 1,
            output_sideband: -1,
            max_sideband: 4,
        };
        valid.validate((1e3, 1e4)).unwrap();
        assert_eq!(valid.input_frequency(1e3), 1_001_000.0);
        assert_eq!(valid.output_frequency(1e3), -999_000.0);
        for selector in [i32::MIN, -5, 5] {
            let mut invalid = valid.clone();
            invalid.input_sideband = selector;
            assert!(invalid.validate((1e3, 1e4)).is_err());
            invalid = valid.clone();
            invalid.output_sideband = selector;
            assert!(invalid.validate((1e3, 1e4)).is_err());
        }
        let mut invalid = valid.clone();
        invalid.carrier_hz = f64::MAX;
        invalid.input_sideband = 2;
        assert!(invalid.validate((1e3, 1e4)).is_err());
        assert!(valid.validate((1e4, 1e3)).is_err());
    }
}
