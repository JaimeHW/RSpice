use super::*;
use crate::product::{AnalysisInstanceId, ContentDigest, ObjectRevision};
use std::collections::HashSet;

/// Durable source domain for a prepared analysis identity.
///
/// Simulation-plan IDs are owned by the project's stable plan/tombstones.
/// Manual-deck IDs are deterministic projections of an imported source deck
/// and intentionally have no plan object. `LegacyUnclassified` is reserved
/// for truthful migration of result schemas that persisted an ID but not its
/// domain; current execution must never create it.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AnalysisResultSourceDomain {
    SimulationPlan,
    ManualDeck,
    #[default]
    LegacyUnclassified,
}

/// Immutable identity of the prepared analysis task that produced a result.
///
/// A result created by the current execution pipeline always carries this
/// record. `AnalysisResult::provenance == None` is reserved exclusively for
/// result history migrated from project formats that predate prepared-task
/// identities; callers must never infer an identity from analysis kind or
/// display order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnalysisResultProvenance {
    source_domain: AnalysisResultSourceDomain,
    source_instance_id: AnalysisInstanceId,
    source_revision: ObjectRevision,
    prepared_snapshot_digest: ContentDigest,
    dependency_ids: Vec<AnalysisInstanceId>,
}

impl AnalysisResultProvenance {
    /// Build a complete, internally consistent prepared-task provenance
    /// record. Dependency order is retained exactly as it appeared in the
    /// frozen prepared snapshot.
    pub fn new(
        source_instance_id: AnalysisInstanceId,
        source_revision: ObjectRevision,
        prepared_snapshot_digest: ContentDigest,
        dependency_ids: Vec<AnalysisInstanceId>,
    ) -> Result<Self, String> {
        Self::new_with_source_domain(
            AnalysisResultSourceDomain::SimulationPlan,
            source_instance_id,
            source_revision,
            prepared_snapshot_digest,
            dependency_ids,
        )
    }

    pub fn new_with_source_domain(
        source_domain: AnalysisResultSourceDomain,
        source_instance_id: AnalysisInstanceId,
        source_revision: ObjectRevision,
        prepared_snapshot_digest: ContentDigest,
        dependency_ids: Vec<AnalysisInstanceId>,
    ) -> Result<Self, String> {
        let mut unique_dependencies = HashSet::with_capacity(dependency_ids.len());
        for dependency_id in &dependency_ids {
            if *dependency_id == source_instance_id {
                return Err(format!(
                    "analysis instance {source_instance_id} cannot depend on itself"
                ));
            }
            if !unique_dependencies.insert(*dependency_id) {
                return Err(format!(
                    "analysis instance {source_instance_id} repeats dependency {dependency_id}"
                ));
            }
        }

        Ok(Self {
            source_domain,
            source_instance_id,
            source_revision,
            prepared_snapshot_digest,
            dependency_ids,
        })
    }

    #[must_use]
    pub const fn source_domain(&self) -> AnalysisResultSourceDomain {
        self.source_domain
    }

    #[must_use]
    pub const fn source_instance_id(&self) -> AnalysisInstanceId {
        self.source_instance_id
    }

    #[must_use]
    pub const fn source_revision(&self) -> ObjectRevision {
        self.source_revision
    }

    #[must_use]
    pub const fn prepared_snapshot_digest(&self) -> ContentDigest {
        self.prepared_snapshot_digest
    }

    #[must_use]
    pub fn dependency_ids(&self) -> &[AnalysisInstanceId] {
        &self.dependency_ids
    }
}

/// Operating point data for a single node or device terminal
#[derive(Debug, Clone, PartialEq)]
pub struct OperatingPointValue {
    /// Node or terminal name (e.g., "V(out)", "I(R1)")
    pub name: String,
    /// Value in base units (volts, amps, etc.)
    pub value: f64,
    /// Unit string for display (e.g., "V", "A", "W")
    pub unit: String,
}

/// DC operating point results - node voltages and branch currents
#[derive(Debug, Clone, Default)]
pub struct DcOpResult {
    /// Node voltages
    pub node_voltages: Vec<OperatingPointValue>,
    /// Branch currents
    pub branch_currents: Vec<OperatingPointValue>,
    /// Power dissipation by device
    pub power_dissipation: Vec<OperatingPointValue>,
}

/// One row of the ranked noise-contributor table (band-integrated).
#[derive(Debug, Clone, PartialEq)]
pub struct NoiseContributorRow {
    /// Device instance name.
    pub device: String,
    /// Noise mechanism label ("thermal", "flicker", "shot", "burst").
    pub mechanism: &'static str,
    /// Output-referred noise power integrated over the band (V²).
    pub power: f64,
    /// Share of the total integrated output noise (percent).
    pub share_pct: f64,
}

/// Ranked noise summary for a noise analysis: per-device/mechanism
/// contributions plus the band total — the table analog designers read
/// first.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct NoiseSummary {
    /// Contributors, ranked by integrated power, descending.
    pub rows: Vec<NoiseContributorRow>,
    /// Total integrated output noise over the band (V rms).
    pub total_rms: f64,
    /// Analysis band, for the panel header (Hz).
    pub band: (f64, f64),
}

/// Single analysis result with metadata and waveforms.
///
/// This represents one analysis within a simulation run, containing
/// all the data needed to display results in the appropriate viewer.
#[derive(Debug, Clone)]
pub struct AnalysisResult {
    /// Unique ID within the simulation run
    pub id: u64,
    /// Analysis type for viewer selection
    pub analysis_type: AnalysisType,
    /// Human-readable label with parameters (e.g., "AC (1Hz-1GHz)")
    pub label: String,
    /// Unix timestamp when analysis completed
    pub timestamp: f64,
    /// Time-domain or frequency-domain waveforms (for sweep analyses)
    pub waveforms: Vec<WaveformData>,
    /// DC operating point data (for DC Op analysis)
    pub dc_op: Option<DcOpResult>,
    /// Per-device operating point report (bias + small-signal parameters,
    /// the Spectre-style OP info), for DC Op analyses.
    pub device_op: Option<rspice_core::circuit::DeviceOpReport>,
    /// Ranked, band-integrated noise contributors, for noise analyses.
    pub noise_summary: Option<NoiseSummary>,
    /// Evaluated `.MEAS` results for this analysis (specs-matrix rows).
    pub measurements: Vec<rspice_core::MeasureResult>,
    /// Whether this analysis completed successfully
    pub success: bool,
    /// Error message if analysis failed
    pub error_message: Option<String>,
    /// Exact prepared-task identity. Missing only for migrated legacy result
    /// history that was written before source instance IDs existed.
    pub provenance: Option<AnalysisResultProvenance>,
}

impl AnalysisResult {
    /// Create a new successful analysis result
    pub fn new(id: u64, analysis_type: AnalysisType, label: impl Into<String>) -> Self {
        Self {
            id,
            analysis_type,
            label: label.into(),
            timestamp: Self::current_timestamp(),
            waveforms: Vec::new(),
            dc_op: None,
            device_op: None,
            noise_summary: None,
            measurements: Vec::new(),
            success: true,
            error_message: None,
            provenance: None,
        }
    }

    /// Create a failed analysis result
    pub fn failed(
        id: u64,
        analysis_type: AnalysisType,
        label: impl Into<String>,
        error: impl Into<String>,
    ) -> Self {
        Self {
            id,
            analysis_type,
            label: label.into(),
            timestamp: Self::current_timestamp(),
            waveforms: Vec::new(),
            dc_op: None,
            device_op: None,
            noise_summary: None,
            measurements: Vec::new(),
            success: false,
            error_message: Some(error.into()),
            provenance: None,
        }
    }

    /// Add waveform data to this analysis
    pub fn with_waveforms(mut self, waveforms: Vec<WaveformData>) -> Self {
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
        if !summary.rows.is_empty() {
            self.noise_summary = Some(summary);
        }
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

    /// Get current timestamp as Unix epoch seconds
    fn current_timestamp() -> f64 {
        crate::common::time_compat::unix_epoch().as_secs_f64()
    }

    /// Check if this analysis has any viewable data
    pub fn has_data(&self) -> bool {
        !self.waveforms.is_empty() || self.dc_op.is_some()
    }
}
