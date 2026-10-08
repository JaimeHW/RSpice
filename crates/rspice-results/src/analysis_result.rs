//! Retained analysis data, scalar evidence and cross-field validation.
//!
//! Exact samples have one owner; waveform decoration does not participate in
//! numerical validation or immutable content identity. Hosts supply timestamps.

use crate::analysis_payload::{AnalysisResultPayload, ScalarEvidenceCandidate};
use crate::analysis_type::AnalysisType;
use crate::convergence_attribution::ConvergenceAttribution;
use crate::convergence_quality::TransientConvergenceEvidence;
use crate::family_metadata::AnalysisResultFamilyMetadata;
use crate::monte_carlo_checkpoint::MonteCarloCheckpointEvidence;
use crate::noise::NoiseSummary;
use crate::operating_point::DcOpResult;
use crate::provenance::AnalysisResultProvenance;
use crate::result_digest::AnalysisResultDataRef;
use crate::result_import::ResultImportSource;
use crate::saved_output::SavedOutputReceipt;
use crate::waveform::RetainedWaveform;
use std::collections::BTreeMap;

mod native_scalar_units;
mod quasi_periodic;
mod saved_output;
mod validation;

pub const LIVE_TRANSIENT_PARTIAL_MESSAGE: &str =
    "Transient analysis is running; displayed samples are provisional";
pub const LIVE_MONTE_CARLO_PARTIAL_MESSAGE: &str =
    "Monte Carlo analysis is running; committed trials are checkpointed";

/// Retained analysis evidence with exact waveform storage supplied by its owner.
/// The default stores plain source data. Presentation owners may decorate a
/// waveform while exposing the same canonical samples through `AsRef`.
#[derive(Debug, Clone)]
pub struct AnalysisResult<W = RetainedWaveform> {
    /// Unique ID within the simulation run
    pub id: u64,
    /// Analysis type for viewer selection
    pub analysis_type: AnalysisType,
    /// Human-readable label with parameters (e.g., "AC (1Hz-1GHz)")
    pub label: String,
    /// Unix timestamp when analysis completed
    pub timestamp: f64,
    /// Time-domain or frequency-domain waveforms (for sweep analyses)
    pub waveforms: Vec<W>,
    /// DC operating point data (for DC Op analysis)
    pub dc_op: Option<DcOpResult>,
    /// Per-device operating point report (bias + small-signal parameters,
    /// the Spectre-style OP info), for DC Op analyses.
    pub device_op: Option<rspice_core::circuit::DeviceOpReport>,
    /// Ranked, band-integrated noise contributors, for noise analyses.
    pub noise_summary: Option<NoiseSummary>,
    /// Exact typed metadata for multi-run and advanced result families. This
    /// is source evidence, not presentation state, and must survive project
    /// and session persistence unchanged.
    pub family_metadata: Option<AnalysisResultFamilyMetadata>,
    /// Exact analysis-native evidence such as pole/zero roots, sensitivity
    /// rows, or scalar-only results. This is immutable retained data, not a
    /// viewer cache.
    pub result_payload: Option<AnalysisResultPayload>,
    /// Exact units for native payload scalars. Absent on historical results;
    /// explicit Unknown is distinct from their original numeric semantics.
    pub native_scalar_units: Option<BTreeMap<String, rspice_core::analysis::MeasurementUnit>>,
    /// Portable committed trials retained even when the task is interrupted.
    /// This is never a substitute for a completed statistical result.
    pub monte_carlo_checkpoint: Option<MonteCarloCheckpointEvidence>,
    /// Evaluated `.MEAS` results for this analysis (specs-matrix rows).
    pub measurements: Vec<rspice_core::MeasureResult>,
    /// Authenticated application receipts for plan-owned saved-output
    /// contracts. The materialized waveform remains in `waveforms`; the
    /// receipt proves why it exists and records deferred/suppressed outcomes.
    pub saved_output_receipts: Vec<SavedOutputReceipt>,
    /// Whether this analysis completed successfully
    pub success: bool,
    /// Error message if analysis failed
    pub error_message: Option<String>,
    /// The design objects the engine named for this failure, when it could
    /// name any. `None` covers every successful run and every failure the
    /// engine could not attribute — a parse error names no conductor.
    pub failure_attribution: Option<ConvergenceAttribution>,
    /// Quality of this result's transient source. Missing historical or
    /// imported evidence stays unknown; it is never inferred from smooth data.
    pub convergence: Option<std::sync::Arc<TransientConvergenceEvidence>>,
    /// Exact prepared-task identity. Missing only for migrated legacy result
    /// history that was written before source instance IDs existed.
    pub provenance: Option<AnalysisResultProvenance>,
    /// External adapter attribution; absent for native and historical unknown sources.
    pub import_source: Option<ResultImportSource>,
}

impl<W> AnalysisResult<W> {
    pub fn imported_coordinate(&self) -> Option<&crate::result_import::ResultImportCoordinate> {
        self.import_source.as_ref()?.coordinate.as_ref()
    }

    /// Unit of retained waveform coordinates, with import declarations taking
    /// precedence over analysis defaults (including an explicitly unknown unit).
    pub fn waveform_coordinate_unit(&self) -> Option<&str> {
        if let Some(coordinate) = self.imported_coordinate() {
            return coordinate.unit.as_deref();
        }
        if let Some(AnalysisResultPayload::DcSweep { evidence }) = &self.result_payload {
            return if evidence.source.starts_with(['I', 'i']) {
                Some("A")
            } else if evidence.source.starts_with(['V', 'v']) {
                Some("V")
            } else {
                None
            };
        }
        if self.analysis_type == AnalysisType::DcSweep {
            // DC can sweep voltage, current, temperature or a parameter.
            return None;
        }
        let unit = self.analysis_type.axis_info().1;
        (!unit.is_empty()).then_some(unit)
    }

    /// Change the waveform wrapper while retaining every analysis field and exact data owner.
    pub fn map_waveforms<V>(self, map: impl FnMut(W) -> V) -> AnalysisResult<V> {
        let Self {
            id,
            analysis_type,
            label,
            timestamp,
            waveforms,
            dc_op,
            device_op,
            noise_summary,
            family_metadata,
            result_payload,
            native_scalar_units,
            monte_carlo_checkpoint,
            measurements,
            saved_output_receipts,
            success,
            error_message,
            failure_attribution,
            convergence,
            provenance,
            import_source,
        } = self;
        AnalysisResult {
            id,
            analysis_type,
            label,
            timestamp,
            waveforms: waveforms.into_iter().map(map).collect(),
            dc_op,
            device_op,
            noise_summary,
            family_metadata,
            result_payload,
            native_scalar_units,
            monte_carlo_checkpoint,
            measurements,
            saved_output_receipts,
            success,
            error_message,
            failure_attribution,
            convergence,
            provenance,
            import_source,
        }
    }
}

impl<W> AsRef<AnalysisResult<W>> for AnalysisResult<W> {
    fn as_ref(&self) -> &AnalysisResult<W> {
        self
    }
}

impl<W> AsMut<AnalysisResult<W>> for AnalysisResult<W> {
    fn as_mut(&mut self) -> &mut AnalysisResult<W> {
        self
    }
}

impl<'a, W> From<&'a AnalysisResult<W>>
    for crate::monte_carlo_checkpoint::MonteCarloCheckpointValidationRef<'a>
{
    fn from(analysis: &'a AnalysisResult<W>) -> Self {
        Self {
            analysis_type: analysis.analysis_type,
            provenance: analysis.provenance.as_ref(),
            import_source: analysis.import_source.as_ref(),
            success: analysis.success,
            family_metadata: analysis.family_metadata.as_ref(),
        }
    }
}

impl<W: AsRef<RetainedWaveform>> AnalysisResult<W> {
    #[must_use]
    pub fn is_live_partial(&self) -> bool {
        !self.success
            && matches!(
                self.error_message.as_deref(),
                Some(LIVE_TRANSIENT_PARTIAL_MESSAGE | LIVE_MONTE_CARLO_PARTIAL_MESSAGE)
            )
    }

    /// Exact prepared-task provenance for current retained results.
    #[must_use]
    pub fn provenance(&self) -> Option<&AnalysisResultProvenance> {
        self.provenance.as_ref()
    }

    /// Exact scalar evidence exposed to specification and result-document
    /// consumers. Explicit `.MEAS` results take precedence over a same-named
    /// analysis-native scalar so one execution cannot be counted twice.
    pub fn scalar_evidence(&self, name: &str) -> Vec<ScalarEvidenceCandidate> {
        let name = name.trim();
        if name.is_empty() {
            return Vec::new();
        }

        let measurements = self
            .measurements
            .iter()
            .filter(|measurement| measurement.name.eq_ignore_ascii_case(name))
            .map(|measurement| ScalarEvidenceCandidate {
                unit: measurement.units.as_ref().map(|units| units.value.clone()),
                value: measurement.value.filter(|value| value.is_finite()),
                passed: measurement.passed && measurement.error.is_none(),
            })
            .collect::<Vec<_>>();
        if !measurements.is_empty() {
            return measurements;
        }

        self.result_payload
            .as_ref()
            .and_then(|payload| payload.scalar_evidence(name))
            .map(|mut evidence| {
                evidence.unit = self.native_scalar_unit(name);
                evidence
            })
            .into_iter()
            .collect()
    }

    /// Canonical discoverable scalar names for the active retained dataset.
    /// These are evidence keys, not synthesized stability booleans.
    pub fn scalar_evidence_names(&self) -> Vec<String> {
        let mut names = self
            .measurements
            .iter()
            .map(|measurement| measurement.name.clone())
            .collect::<Vec<_>>();
        if let Some(payload) = &self.result_payload {
            names.extend(payload.scalar_evidence_names());
        }
        names
    }

    /// Create a new successful analysis result
    pub fn new(
        id: u64,
        analysis_type: AnalysisType,
        label: impl Into<String>,
        timestamp: f64,
    ) -> Self {
        Self {
            id,
            analysis_type,
            label: label.into(),
            timestamp,
            waveforms: Vec::new(),
            dc_op: None,
            device_op: None,
            noise_summary: None,
            family_metadata: None,
            result_payload: None,
            native_scalar_units: None,
            monte_carlo_checkpoint: None,
            measurements: Vec::new(),
            saved_output_receipts: Vec::new(),
            success: true,
            error_message: None,
            failure_attribution: None,
            convergence: None,
            provenance: None,
            import_source: None,
        }
    }

    /// Create a failed analysis result
    pub fn failed(
        id: u64,
        analysis_type: AnalysisType,
        label: impl Into<String>,
        error: impl Into<String>,
        timestamp: f64,
    ) -> Self {
        Self {
            id,
            analysis_type,
            label: label.into(),
            timestamp,
            waveforms: Vec::new(),
            dc_op: None,
            device_op: None,
            noise_summary: None,
            family_metadata: None,
            result_payload: None,
            native_scalar_units: None,
            monte_carlo_checkpoint: None,
            measurements: Vec::new(),
            saved_output_receipts: Vec::new(),
            success: false,
            error_message: Some(error.into()),
            failure_attribution: None,
            convergence: None,
            provenance: None,
            import_source: None,
        }
    }

    /// Add waveform data to this analysis
    pub fn with_waveforms(mut self, waveforms: Vec<W>) -> Self {
        self.waveforms = waveforms;
        self
    }

    /// Add DC operating point data
    pub fn with_dc_op(mut self, dc_op: DcOpResult) -> Self {
        self.dc_op = Some(dc_op);
        self
    }

    /// Attach the per-device operating-point report.
    pub fn with_device_op(mut self, report: rspice_core::circuit::DeviceOpReport) -> Self {
        if !report.is_empty() {
            self.device_op = Some(report);
        }
        self
    }

    /// Attach the ranked noise-contributor summary.
    pub fn with_noise_summary(mut self, summary: NoiseSummary) -> Self {
        // An empty contributor table is meaningful for the `SummaryOnly`
        // retention policy. The integrated totals and exact analysis band are
        // still authoritative result evidence and must survive conversion and
        // project persistence even when individual contributors were omitted.
        self.noise_summary = Some(summary);
        self
    }

    /// Attach exact source metadata for an advanced result family.
    #[must_use]
    pub fn with_family_metadata(mut self, metadata: AnalysisResultFamilyMetadata) -> Self {
        debug_assert!(metadata.validate_for(self.analysis_type).is_ok());
        self.family_metadata = Some(metadata);
        self
    }

    /// What to call this result where a reader sees its analysis named.
    ///
    /// A recorded `.FFT` is retained in the Fourier family, because that is
    /// the family the Results contract already defines for a retained
    /// coefficient spectrum. The family is not the analysis, though, and a
    /// reader must never be told an FFT was a `.four`: the payload is what
    /// says which request produced the result, so it is what answers here.
    #[must_use]
    pub fn kind_display_name(&self) -> &'static str {
        match &self.result_payload {
            Some(AnalysisResultPayload::FftSpectrum { .. }) => "FFT",
            _ => self.analysis_type.display_name(),
        }
    }

    /// The SPICE card a reader would author to reproduce this result.
    #[must_use]
    pub fn spice_card(&self) -> &'static str {
        match &self.result_payload {
            Some(AnalysisResultPayload::FftSpectrum { .. }) => ".fft",
            _ => self.analysis_type.spice_command(),
        }
    }

    /// Attach exact analysis-native result evidence.
    #[must_use]
    pub fn with_result_payload(mut self, payload: AnalysisResultPayload) -> Self {
        debug_assert!(payload.validate_for(self.analysis_type).is_ok());
        self.result_payload = Some(payload);
        self
    }

    /// Attach evaluated `.MEAS` results.
    pub fn with_measurements(mut self, measurements: Vec<rspice_core::MeasureResult>) -> Self {
        self.measurements = measurements;
        self
    }

    /// Attach the exact prepared task that produced this result.
    #[must_use]
    pub fn with_provenance(mut self, provenance: AnalysisResultProvenance) -> Self {
        self.provenance = Some(provenance);
        self
    }

    /// Borrow exact evidence for canonical identity and retention accounting.
    pub fn result_data_ref(
        &self,
    ) -> AnalysisResultDataRef<'_, impl ExactSizeIterator<Item = &RetainedWaveform> + Clone> {
        AnalysisResultDataRef {
            analysis_type: self.analysis_type,
            success: self.success,
            error_message: self.error_message.as_deref(),
            import_source: self.import_source.as_ref(),
            convergence: self.convergence.as_deref(),
            waveforms: self.waveforms.iter().map(AsRef::as_ref),
            dc_op: self.dc_op.as_ref(),
            device_op: self.device_op.as_ref(),
            noise_summary: self.noise_summary.as_ref(),
            family_metadata: self.family_metadata.as_ref(),
            result_payload: self.result_payload.as_ref(),
            measurements: &self.measurements,
            saved_output_receipts: &self.saved_output_receipts,
            monte_carlo_checkpoint: self.monte_carlo_checkpoint.as_ref(),
            native_scalar_units: self.native_scalar_units.as_ref(),
        }
    }
}
