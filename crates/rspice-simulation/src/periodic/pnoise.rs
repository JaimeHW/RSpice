use crate::error::ServiceRunError;
use rspice_core::Value;
use rspice_simulation_contract::periodic_carrier::PeriodicCarrier;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PnoiseRunError {
    Validation(String),
    Resolution(String),
    Data(String),
}

impl fmt::Display for PnoiseRunError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Validation(message) | Self::Resolution(message) | Self::Data(message) => {
                f.write_str(message)
            }
        }
    }
}

impl std::error::Error for PnoiseRunError {}

impl From<PnoiseRunError> for ServiceRunError {
    fn from(error: PnoiseRunError) -> Self {
        Self::Failure(error.to_string())
    }
}

/// Frequency sweep type for periodic-noise analysis.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum PnoiseFrequencySweep {
    Decade,
    Octave,
    Linear,
}

impl PnoiseFrequencySweep {
    pub fn keyword(self) -> &'static str {
        match self {
            Self::Decade => "dec",
            Self::Octave => "oct",
            Self::Linear => "lin",
        }
    }
}

/// PNoise noise-reference mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum PnoiseReference {
    Output,
    Input,
    Phase,
}

/// Explicit configuration for PNoise execution.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct PnoiseRunConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sampling: Option<rspice_core::analysis::pnoise::PeriodicNoiseSampling>,
    pub input_sideband: i32,
    pub output_sideband: i32,
    pub pss_fundamental_freq: Value,
    pub pss_num_harmonics: usize,
    pub pss_tolerance: Value,
    pub start_freq: Value,
    pub stop_freq: Value,
    pub points_per_unit: usize,
    pub sweep: PnoiseFrequencySweep,
    pub max_sideband: i32,
    pub output_node: String,
    pub output_ref: Option<String>,
    pub input_source: String,
    pub noise_ref: PnoiseReference,
    pub integrated_noise: bool,
    pub noise_summary: bool,
    pub reltol: Value,
    pub abstol: Value,
    /// Which periodic solve this run folds noise around; the card's `FROM=`.
    pub carrier: PeriodicCarrier,
}

impl Default for PnoiseRunConfig {
    fn default() -> Self {
        Self {
            sampling: None,
            input_sideband: 0,
            output_sideband: 0,
            pss_fundamental_freq: 1e6,
            pss_num_harmonics: 10,
            pss_tolerance: 1e-3,
            start_freq: 1.0,
            stop_freq: 1e6,
            points_per_unit: 10,
            sweep: PnoiseFrequencySweep::Decade,
            max_sideband: 5,
            output_node: "VOUT".to_string(),
            output_ref: None,
            input_source: "VIN".to_string(),
            noise_ref: PnoiseReference::Output,
            integrated_noise: false,
            noise_summary: true,
            reltol: 1e-3,
            abstol: 1e-18,
            carrier: PeriodicCarrier::Preceding,
        }
    }
}

impl PnoiseRunConfig {
    pub fn validate_conversion_channels(&self) -> Result<(), String> {
        if let Some(sampling) = &self.sampling {
            sampling.validate()?;
            if self.noise_ref == PnoiseReference::Phase || self.output_sideband != 0 {
                return Err(
                    "Sampled noise requires output or input reference and output sideband zero"
                        .into(),
                );
            }
        }
        rspice_simulation_contract::config::validate_noise_sidebands(
            self.input_sideband,
            self.output_sideband,
            usize::try_from(self.max_sideband)
                .map_err(|_| "Maximum sideband must be nonnegative")?,
        )?;
        if (self.noise_ref == PnoiseReference::Phase && self.output_sideband != 0)
            || (self.noise_ref != PnoiseReference::Input && self.input_sideband != 0)
        {
            return Err("Conversion sidebands apply to driven noise; input sideband requires input-referred noise".into());
        }
        Ok(())
    }
    pub fn validate(&self) -> Result<(), PnoiseRunError> {
        if !self.pss_fundamental_freq.is_finite() || self.pss_fundamental_freq <= 0.0 {
            return Err(PnoiseRunError::Validation(
                "PNOISE requires a positive PSS fundamental frequency".to_string(),
            ));
        }
        if self.pss_num_harmonics == 0 {
            return Err(PnoiseRunError::Validation(
                "PNOISE requires at least one PSS harmonic".to_string(),
            ));
        }
        if !self.pss_tolerance.is_finite() || self.pss_tolerance <= 0.0 {
            return Err(PnoiseRunError::Validation(
                "PNOISE requires a positive PSS tolerance".to_string(),
            ));
        }
        if !self.start_freq.is_finite() || self.start_freq <= 0.0 {
            return Err(PnoiseRunError::Validation(
                "PNOISE start frequency must be positive".to_string(),
            ));
        }
        if !self.stop_freq.is_finite() || self.stop_freq < self.start_freq {
            return Err(PnoiseRunError::Validation(
                "PNOISE stop frequency must be >= start frequency".to_string(),
            ));
        }
        if self.points_per_unit == 0 {
            return Err(PnoiseRunError::Validation(
                "PNOISE points per unit must be greater than zero".to_string(),
            ));
        }
        if self.integrated_noise
            && (self.start_freq == self.stop_freq
                || (self.sweep == PnoiseFrequencySweep::Linear && self.points_per_unit <= 2))
        {
            return Err(PnoiseRunError::Validation(
                "PNOISE integrated noise requires at least two distinct frequency points".into(),
            ));
        }
        if self.max_sideband < 0 {
            return Err(PnoiseRunError::Validation(
                "PNOISE max sideband must be non-negative".to_string(),
            ));
        }
        self.validate_conversion_channels()
            .map_err(PnoiseRunError::Validation)?;
        if self.output_node.trim().is_empty() {
            return Err(PnoiseRunError::Validation(
                "PNOISE output node must be specified".to_string(),
            ));
        }
        if !self.reltol.is_finite() || self.reltol <= 0.0 {
            return Err(PnoiseRunError::Validation(
                "PNOISE relative tolerance must be positive".to_string(),
            ));
        }
        if !self.abstol.is_finite() || self.abstol < 0.0 {
            return Err(PnoiseRunError::Validation(
                "PNOISE absolute tolerance must be non-negative".to_string(),
            ));
        }
        // The carrier itself is not a range check: it names a family, and
        // whether the state this run was handed belongs to that family is
        // `PeriodicCarrierState::accepted_by`, asked where both are in hand.
        Ok(())
    }
}
