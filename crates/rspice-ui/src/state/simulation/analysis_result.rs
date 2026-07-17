use super::*;
use crate::product::{AnalysisInstanceId, ContentDigest, ObjectRevision};
use std::collections::{BTreeMap, HashSet};

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

/// Exact per-variable evidence retained from a Monte Carlo execution.
///
/// Histogram bins are deliberately not duplicated here: the presentation
/// histogram is already materialized as a waveform, while these source
/// samples and statistics are the immutable evidence needed to recompute any
/// future distribution view without inventing data.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MonteCarloVariableMetadata {
    pub name: String,
    pub samples: Vec<f64>,
    pub mean: f64,
    pub std_dev: f64,
    pub min: f64,
    pub max: f64,
}

/// Typed, lossless metadata for result families whose execution contract is
/// richer than a collection of plotted waveforms.
///
/// This payload is part of the immutable analysis result. It preserves exact
/// axes, labels, run counts, statistical samples, and convergence outcomes
/// that cannot be reconstructed truthfully from display waveforms alone.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "family", rename_all = "snake_case", deny_unknown_fields)]
pub enum AnalysisResultFamilyMetadata {
    Parametric {
        target: String,
        sweep_values: Vec<f64>,
        failed_points: usize,
    },
    Corner {
        x_values: Vec<f64>,
        x_label: String,
        x_unit: String,
        temperatures_c: Vec<f64>,
        corner_labels: Vec<String>,
        failed_corners: usize,
    },
    MonteCarlo {
        seed: u64,
        runs_requested: usize,
        runs_completed: usize,
        failures: usize,
        all_converged: bool,
        variables: Vec<MonteCarloVariableMetadata>,
    },
    Reliability {
        years: Vec<f64>,
    },
    Optimization {
        iterations: Vec<f64>,
        best_cost: f64,
        best_variables: BTreeMap<String, f64>,
        converged: bool,
    },
    Soa {
        time: Vec<f64>,
    },
}

impl AnalysisResultFamilyMetadata {
    /// Validate retained source evidence independently of any viewer.
    pub fn validate_for(&self, analysis_type: AnalysisType) -> Result<(), String> {
        let compatible = matches!(
            (self, analysis_type),
            (Self::Parametric { .. }, AnalysisType::Parametric)
                | (Self::Corner { .. }, AnalysisType::Corner)
                | (Self::MonteCarlo { .. }, AnalysisType::MonteCarlo)
                | (Self::Reliability { .. }, AnalysisType::Reliability)
                | (Self::Optimization { .. }, AnalysisType::Optimization)
                | (Self::Soa { .. }, AnalysisType::Soa)
        );
        if !compatible {
            return Err(format!(
                "retained family metadata does not match analysis type {analysis_type:?}"
            ));
        }

        match self {
            Self::Parametric {
                target,
                sweep_values,
                ..
            } => {
                require_non_empty(target, "parametric target")?;
                require_finite_values(sweep_values, "parametric sweep values")?;
            }
            Self::Corner {
                x_values,
                x_label,
                temperatures_c,
                corner_labels,
                ..
            } => {
                require_non_empty(x_label, "corner x-axis label")?;
                require_finite_values(x_values, "corner x-axis values")?;
                require_finite_values(temperatures_c, "corner temperatures")?;
                if temperatures_c.len() != x_values.len() || corner_labels.len() != x_values.len() {
                    return Err(
                        "corner x values, temperatures, and labels have different lengths"
                            .to_owned(),
                    );
                }
                if corner_labels.iter().any(|label| label.trim().is_empty()) {
                    return Err("corner metadata contains an empty corner label".to_owned());
                }
            }
            Self::MonteCarlo {
                runs_requested,
                runs_completed,
                failures,
                all_converged,
                variables,
                ..
            } => {
                if runs_completed.saturating_add(*failures) > *runs_requested {
                    return Err(
                        "Monte Carlo completed and failed counts exceed requested runs".to_owned(),
                    );
                }
                if *all_converged && (*failures != 0 || runs_completed != runs_requested) {
                    return Err(
                        "Monte Carlo all_converged contradicts retained run counts".to_owned()
                    );
                }
                let mut names = HashSet::with_capacity(variables.len());
                for variable in variables {
                    require_non_empty(&variable.name, "Monte Carlo variable name")?;
                    if !names.insert(variable.name.as_str()) {
                        return Err(format!(
                            "Monte Carlo metadata repeats variable '{}'",
                            variable.name
                        ));
                    }
                    require_finite_values(&variable.samples, "Monte Carlo samples")?;
                    for (label, value) in [
                        ("mean", variable.mean),
                        ("standard deviation", variable.std_dev),
                        ("minimum", variable.min),
                        ("maximum", variable.max),
                    ] {
                        if !value.is_finite() {
                            return Err(format!(
                                "Monte Carlo variable '{}' has non-finite {label}",
                                variable.name
                            ));
                        }
                    }
                    if variable.std_dev < 0.0 || variable.min > variable.max {
                        return Err(format!(
                            "Monte Carlo variable '{}' has inconsistent statistics",
                            variable.name
                        ));
                    }
                }
            }
            Self::Reliability { years } => {
                require_finite_values(years, "reliability years")?;
            }
            Self::Optimization {
                iterations,
                best_cost,
                best_variables,
                ..
            } => {
                require_finite_values(iterations, "optimization iterations")?;
                if !best_cost.is_finite() {
                    return Err("optimization best cost is non-finite".to_owned());
                }
                for (name, value) in best_variables {
                    require_non_empty(name, "optimization variable name")?;
                    if !value.is_finite() {
                        return Err(format!(
                            "optimization variable '{name}' has a non-finite best value"
                        ));
                    }
                }
            }
            Self::Soa { time } => {
                require_finite_values(time, "SOA time")?;
            }
        }
        Ok(())
    }
}

fn require_non_empty(value: &str, label: &str) -> Result<(), String> {
    if value.trim().is_empty() {
        Err(format!("{label} is empty"))
    } else {
        Ok(())
    }
}

fn require_finite_values(values: &[f64], label: &str) -> Result<(), String> {
    if values.iter().any(|value| !value.is_finite()) {
        Err(format!("{label} contain a non-finite value"))
    } else {
        Ok(())
    }
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
    /// Exact typed metadata for multi-run and advanced result families. This
    /// is source evidence, not presentation state, and must survive project
    /// and session persistence unchanged.
    pub family_metadata: Option<AnalysisResultFamilyMetadata>,
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
            family_metadata: None,
            measurements: Vec::new(),
            saved_output_receipts: Vec::new(),
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
            family_metadata: None,
            measurements: Vec::new(),
            saved_output_receipts: Vec::new(),
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

    /// Attach exact source metadata for an advanced result family.
    #[must_use]
    pub fn with_family_metadata(mut self, metadata: AnalysisResultFamilyMetadata) -> Self {
        debug_assert!(metadata.validate_for(self.analysis_type).is_ok());
        self.family_metadata = Some(metadata);
        self
    }

    /// Attach evaluated `.MEAS` results.
    pub fn with_measurements(mut self, measurements: Vec<rspice_core::MeasureResult>) -> Self {
        self.measurements = measurements;
        self
    }

    /// Attach the exact output-contract receipts produced during completion.
    #[must_use]
    pub fn with_saved_output_receipts(mut self, receipts: Vec<SavedOutputReceipt>) -> Self {
        self.saved_output_receipts = receipts;
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
