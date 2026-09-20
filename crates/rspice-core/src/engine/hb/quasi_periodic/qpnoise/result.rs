//! Complete QPNOISE primary evidence and deterministic measurement products.
use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QpnoiseUnavailable {
    ZeroInputTransfer,
    OutsideNumericRange,
    NoFrequencyInterval,
    NegativeIntegrationFrequency,
    IncompleteIntegrationBand,
    UndefinedIntegrationSample,
}
/// An undefined referred quantity does not invalidate the output spectrum.
#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(
    tag = "status",
    content = "value",
    rename_all = "snake_case",
    deny_unknown_fields
)]
pub enum QpnoiseValue {
    Finite(Value),
    Unavailable(QpnoiseUnavailable),
}
impl QpnoiseValue {
    pub(super) fn from_nonnegative(value: Value) -> Self {
        if value.is_finite() && value >= 0.0 {
            Self::Finite(value)
        } else {
            Self::Unavailable(QpnoiseUnavailable::OutsideNumericRange)
        }
    }
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct QpnoiseReference {
    pub source_index: usize,
    pub source_resistor: String,
    pub resistance: Value,
    pub temperature: Value,
    pub boltzmann: Value,
    pub lattices: Vec<Vec<i32>>,
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct QpnoiseResultMetadata {
    pub version: u32,
    pub retained_identity: String,
    pub operating_point_identity: String,
    pub request: QpnoiseRequest,
    pub grid: QuasiPeriodicGridConfig,
    pub node_names: Vec<String>,
    pub branch_names: Vec<String>,
    pub ground_policy: crate::netlist::GroundPolicy,
    pub observations: Vec<Vec<(usize, Complex64)>>,
    pub input_source: Option<super::super::qpxf::QpxfInputSource>,
    pub input_lattices: Vec<Vec<i32>>,
    pub reference: Option<QpnoiseReference>,
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct QpnoiseIntegrated {
    /// Output units (V or A) RMS, not squared power.
    pub output_rms: QpnoiseValue,
    pub input_rms: Option<QpnoiseValue>,
    pub contributor_rms: Vec<QpnoiseValue>,
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct QpnoiseContributorRank {
    pub source_index: usize,
    /// Percentage of summed contributor power in the integration band, or
    /// point PSD for a one-point sweep. Zero total gives zero percentages.
    pub percentage: Value,
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct QpnoiseOutputSpectrum {
    pub frequencies_hz: Vec<Value>,
    pub input_transfer: Option<Vec<Complex64>>,
    /// Squared input units / Hz. Uses a unit independent-source excitation.
    pub input_noise: Option<Vec<QpnoiseValue>>,
    pub noise_figure_db: Option<Vec<QpnoiseValue>>,
    pub integrated: Option<QpnoiseIntegrated>,
    /// None if disabled or an unavailable integration band prevents ranking.
    pub ranking: Option<Vec<QpnoiseContributorRank>>,
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct QpnoiseAnalysisResult {
    pub metadata: QpnoiseResultMetadata,
    pub sources: Vec<QuasiPeriodicNoiseSource>,
    pub points: Vec<QuasiPeriodicNoisePoint>,
    /// Row-major C[r,c]=E[y_r conj(y_c)], including total accumulation bounds.
    pub total_covariances: Vec<QuasiPeriodicNoiseCovariance>,
    pub outputs: Vec<QpnoiseOutputSpectrum>,
}
