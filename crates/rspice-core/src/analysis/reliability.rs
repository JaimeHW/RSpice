//! Explicit, characterized aging laws and their cumulative mission clocks.
//!
//! No process calibration is built in. A pack must declare its provenance,
//! applicability and numerical validity envelope. `EquivalentTimePower` uses
//! a normalized effective-age clock, not a claim of a microscopic BTI model:
//! `d(age)/dt = (|Vgs|/Vg0)^g (|Vds|/Vd0)^d
//! exp(Ea/kB * (1/T0 - 1/T)) / t0`; parameter changes are `scale * age^n`.
//! The voltage exponents and activation energy therefore describe the CLOCK,
//! not the parameter-shift coefficient. This model has no recovery. Imported
//! fits must use this convention; coefficients from other laws cannot simply
//! be copied. Black's law retains consumed reference lifetime separately from
//! parameter shifts and does not invent a resistance change or failure probability.

use serde::{Deserialize, Serialize};

mod evaluate;
mod validation;

pub use evaluate::{AgingClock, AgingEvaluation, AgingParameterChange, AgingStress};

/// Maximum pack size accepted by the JSON loader, including whitespace.
pub const MAX_AGING_PACK_BYTES: usize = 4 * 1024 * 1024;
/// Hard bound on independently characterized laws in one pack.
pub const MAX_AGING_MODELS: usize = 1024;
/// A Julian year, used explicitly for Studio lifetime checkpoints.
pub const SECONDS_PER_AGING_YEAR: f64 = 365.25 * 86400.0;

/// Numerical failures are reported rather than converted to zero degradation.
#[derive(Debug, thiserror::Error, PartialEq)]
pub enum AgingError {
    #[error("invalid aging model: {0}")]
    Invalid(String),
    #[error("aging stress is outside the declared calibration: {0}")]
    OutsideCalibration(String),
    #[error("aging calculation cannot represent {0}")]
    Numeric(String),
    #[error("aging calculation was cancelled")]
    Aborted,
}

/// A source claim carried with results; reference packs are not sign-off data.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AgingQualification {
    PublicReference,
    UserCharacterized,
    FoundryQualified,
}

/// Independently enabled wear-out mechanisms.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AgingMechanism {
    Hci,
    Nbti,
    Electromigration,
}

/// A finite, inclusive calibration interval. Extrapolation is rejected.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AgingRange {
    pub min: f64,
    pub max: f64,
}

/// Signed terminal voltages, Kelvin temperature, and absolute current density.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AgingValidity {
    pub gate_source_v: AgingRange,
    pub drain_source_v: AgingRange,
    pub temperature_k: AgingRange,
    pub current_density_a_per_m2: AgingRange,
    /// Largest effective exposure in seconds at the law's reference condition.
    pub max_equivalent_seconds: f64,
}

/// The sign of gate stress to which a transistor fit applies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AgingGatePolarity {
    Positive,
    Negative,
    Either,
}

/// `Additive`: aged = fresh + shift. `Relative`: aged = fresh * (1 + shift).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AgingParameterUpdate {
    Additive,
    Relative,
}

/// An explicitly named compact-model parameter, with no guessed VTH/U0 mapping.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AgingParameterScale {
    pub parameter: String,
    pub update: AgingParameterUpdate,
    /// Parameter units for additive updates; dimensionless for relative updates.
    pub scale_at_reference_time: f64,
}

/// Calibrated laws with unambiguous normalization and units.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum AgingLaw {
    EquivalentTimePower {
        reference_time_s: f64,
        reference_gate_magnitude_v: f64,
        reference_drain_magnitude_v: f64,
        reference_temperature_k: f64,
        gate_polarity: AgingGatePolarity,
        clock_gate_exponent: f64,
        clock_drain_exponent: f64,
        clock_activation_energy_ev: f64,
        time_exponent: f64,
        parameters: Vec<AgingParameterScale>,
    },
    BlackElectromigration {
        reference_lifetime_s: f64,
        reference_current_density_a_per_m2: f64,
        reference_temperature_k: f64,
        current_exponent: f64,
        activation_energy_ev: f64,
    },
}

/// One fit; separate fits may characterize different devices and mechanisms.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AgingModel {
    pub id: String,
    pub mechanism: AgingMechanism,
    /// Exact compact-model family/version and geometry/process restrictions.
    pub applicability: String,
    pub validity: AgingValidity,
    pub law: AgingLaw,
}

/// Portable, immutable snapshot of the actual calibration used by a request.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AgingModelPack {
    pub schema_version: u32,
    pub id: String,
    pub process: String,
    pub qualification: AgingQualification,
    pub source: String,
    pub license: String,
    /// Characterization method and limits; a qualification label is not verified.
    pub characterization: String,
    pub models: Vec<AgingModel>,
}

impl AgingModelPack {
    /// Decode a bounded snapshot. Unknown fields/units and missing data fail closed.
    pub fn from_json(json: &str) -> Result<Self, AgingError> {
        if json.len() > MAX_AGING_PACK_BYTES {
            return Err(AgingError::Invalid("model pack exceeds 4 MiB".into()));
        }
        let pack: Self = serde_json::from_str(json)
            .map_err(|error| AgingError::Invalid(format!("model-pack JSON: {error}")))?;
        pack.validate()?;
        Ok(pack)
    }
}

#[cfg(test)]
mod tests;
