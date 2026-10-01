//! Application ownership of retained analyses with waveform presentation.
//!
//! Numerical data and validation belong to `rspice-results`; this wrapper adds
//! host timestamps, live display conventions and the application waveform type.

use super::*;
use rspice_results::analysis_result::AnalysisResult as RetainedAnalysisResult;
#[cfg(test)]
use std::collections::BTreeMap;
use std::ops::{Deref, DerefMut};

use rspice_results::analysis_result::{
    LIVE_MONTE_CARLO_PARTIAL_MESSAGE, LIVE_TRANSIENT_PARTIAL_MESSAGE,
};

/// A retained analysis whose waveforms include application display choices.
#[derive(Debug, Clone)]
pub struct AnalysisResult {
    pub data: RetainedAnalysisResult<WaveformData>,
}

impl AsRef<RetainedAnalysisResult<WaveformData>> for AnalysisResult {
    fn as_ref(&self) -> &RetainedAnalysisResult<WaveformData> {
        &self.data
    }
}
impl AsMut<RetainedAnalysisResult<WaveformData>> for AnalysisResult {
    fn as_mut(&mut self) -> &mut RetainedAnalysisResult<WaveformData> {
        &mut self.data
    }
}

impl Deref for AnalysisResult {
    type Target = RetainedAnalysisResult<WaveformData>;
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
impl DerefMut for AnalysisResult {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl<'a> From<&'a AnalysisResult>
    for rspice_results::monte_carlo_checkpoint::MonteCarloCheckpointValidationRef<'a>
{
    fn from(analysis: &'a AnalysisResult) -> Self {
        (&analysis.data).into()
    }
}

impl AnalysisResult {
    /// Construct a presentation-only transient result from accepted solver
    /// points. It is deliberately unsuccessful until the engine returns its
    /// terminal result, which keeps measurement, export, and qualification
    /// paths from mistaking an in-flight prefix for complete evidence.
    pub(crate) fn live_transient_partial(
        id: u64,
        analysis_type: AnalysisType,
        label: impl Into<String>,
    ) -> Self {
        Self::failed(id, analysis_type, label, LIVE_TRANSIENT_PARTIAL_MESSAGE)
    }

    pub(crate) fn live_monte_carlo_partial(
        label: impl Into<String>,
        checkpoint: MonteCarloCheckpointEvidence,
    ) -> Self {
        let mut result = Self::failed(
            1,
            AnalysisType::MonteCarlo,
            label,
            LIVE_MONTE_CARLO_PARTIAL_MESSAGE,
        );
        result.monte_carlo_checkpoint = Some(checkpoint);
        result
    }

    /// Get current timestamp as Unix epoch seconds
    fn current_timestamp() -> f64 {
        crate::time_compat::unix_epoch().as_secs_f64()
    }

    /// Check if this analysis has any viewable data
    #[cfg(test)]
    pub fn has_data(&self) -> bool {
        !self.waveforms.is_empty()
            || self.dc_op.is_some()
            || self
                .result_payload
                .as_ref()
                .is_some_and(AnalysisResultPayload::has_data)
    }

    /// Create a new successful analysis result at the host's current time.
    pub fn new(id: u64, analysis_type: AnalysisType, label: impl Into<String>) -> Self {
        Self {
            data: RetainedAnalysisResult::new(id, analysis_type, label, Self::current_timestamp()),
        }
    }
    /// Retain a failed analysis with its original diagnostic.
    pub fn failed(
        id: u64,
        analysis_type: AnalysisType,
        label: impl Into<String>,
        error: impl Into<String>,
    ) -> Self {
        Self {
            data: RetainedAnalysisResult::failed(
                id,
                analysis_type,
                label,
                error,
                Self::current_timestamp(),
            ),
        }
    }
    /// Exact prepared-task provenance for current retained results.
    #[must_use]
    pub fn provenance(&self) -> Option<&AnalysisResultProvenance> {
        self.data.provenance()
    }
    /// Add waveform data to this analysis
    pub fn with_waveforms(mut self, waveforms: Vec<WaveformData>) -> Self {
        self.data = self.data.with_waveforms(waveforms);
        self
    }
    /// Add DC operating point data
    pub fn with_dc_op(mut self, dc_op: DcOpResult) -> Self {
        self.data = self.data.with_dc_op(dc_op);
        self
    }
    /// Attach the per-device operating-point report.
    pub fn with_device_op(mut self, report: rspice_core::circuit::DeviceOpReport) -> Self {
        self.data = self.data.with_device_op(report);
        self
    }
    /// Attach the ranked noise-contributor summary.
    pub fn with_noise_summary(mut self, summary: NoiseSummary) -> Self {
        self.data = self.data.with_noise_summary(summary);
        self
    }
    /// Attach exact source metadata for an advanced result family.
    #[must_use]
    pub fn with_family_metadata(mut self, metadata: AnalysisResultFamilyMetadata) -> Self {
        self.data = self.data.with_family_metadata(metadata);
        self
    }
    /// Attach exact analysis-native result evidence.
    #[must_use]
    pub fn with_result_payload(mut self, payload: AnalysisResultPayload) -> Self {
        self.data = self.data.with_result_payload(payload);
        self
    }
    /// Attach evaluated `.MEAS` results.
    pub fn with_measurements(mut self, measurements: Vec<rspice_core::MeasureResult>) -> Self {
        self.data = self.data.with_measurements(measurements);
        self
    }
    /// Attach the exact prepared task that produced this result.
    #[must_use]
    pub fn with_provenance(mut self, provenance: AnalysisResultProvenance) -> Self {
        self.data = self.data.with_provenance(provenance);
        self
    }
}

#[cfg(test)]
mod retained_payload_tests;
