//! Analysis result provenance.
//!
//! Records where a result came from — which deck, which engine, which
//! analysis — so a plot can always name its source. A result whose
//! provenance does not match the current design is stale, not wrong, and the
//! distinction is what lets the UI say so.

use super::*;
use crate::product::{AnalysisInstanceId, ContentDigest, ObjectRevision};
use std::collections::{BTreeMap, HashSet};

mod family_metadata;

pub use family_metadata::{AnalysisResultFamilyMetadata, MonteCarloVariableMetadata};

const LIVE_TRANSIENT_PARTIAL_MESSAGE: &str =
    "Transient analysis is running; displayed samples are provisional";

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

/// The exact PVT point one result was solved at.
///
/// Only a task the executor expanded to a single point can carry this record.
/// A task that runs once for a whole declared space, or against the deck's own
/// conditions, has no point to name, and the absence is the honest answer:
/// evidence that was never attributed to a point must not answer a question
/// asked about one.
///
/// `nominal` is decided where the run's reference point is known, not
/// reconstructed later from the triple, because "nominal" means *this run's*
/// reference process and temperature and not a fixed convention.
#[derive(Debug, Clone)]
pub struct AnalysisResultPvtPoint {
    process: String,
    supply_voltage: Option<f64>,
    temperature_celsius: f64,
    corner_contract: Option<ContentDigest>,
    nominal: bool,
}

/// Two points are the same point when every recorded quantity is bit-identical.
///
/// Bitwise rather than numeric, so equality is a true equivalence and the
/// record can be `Eq`. The constructor already refuses non-finite quantities,
/// so this never has to answer for a NaN it produced itself.
impl PartialEq for AnalysisResultPvtPoint {
    fn eq(&self, other: &Self) -> bool {
        self.process == other.process
            && self.supply_voltage.map(f64::to_bits) == other.supply_voltage.map(f64::to_bits)
            && self.temperature_celsius.to_bits() == other.temperature_celsius.to_bits()
            && self.corner_contract == other.corner_contract
            && self.nominal == other.nominal
    }
}

impl Eq for AnalysisResultPvtPoint {}

impl AnalysisResultPvtPoint {
    /// Record one attributed point.
    ///
    /// The process is stored under the name the deck, the PDK section and the
    /// run set's process axis already agree on, because the executor's corner
    /// enum sits above everything that has to persist where a result came from.
    pub fn new(
        process: impl Into<String>,
        supply_voltage: Option<f64>,
        temperature_celsius: f64,
        corner_contract: Option<ContentDigest>,
        nominal: bool,
    ) -> Result<Self, String> {
        let process = process.into();
        if process.trim().is_empty() {
            return Err("attributed PVT point requires a process corner name".to_owned());
        }
        if process.chars().any(char::is_control) {
            return Err(format!(
                "attributed PVT process corner contains a control character: {process:?}"
            ));
        }
        if !temperature_celsius.is_finite() {
            return Err("attributed PVT point temperature must be finite".to_owned());
        }
        if supply_voltage.is_some_and(|voltage| !voltage.is_finite()) {
            return Err("attributed PVT point supply voltage must be finite".to_owned());
        }
        Ok(Self {
            process,
            supply_voltage,
            temperature_celsius,
            corner_contract,
            nominal,
        })
    }

    #[must_use]
    pub fn process(&self) -> &str {
        &self.process
    }

    /// The supply this point was solved at. `None` means the run declared no
    /// supply axis, so the deck's own supply stood.
    #[must_use]
    pub const fn supply_voltage(&self) -> Option<f64> {
        self.supply_voltage
    }

    #[must_use]
    pub const fn temperature_celsius(&self) -> f64 {
        self.temperature_celsius
    }

    /// Digest of the corner contract that bound this point's process models.
    /// A temperature-only axis has no contract to name.
    #[must_use]
    pub const fn corner_contract(&self) -> Option<ContentDigest> {
        self.corner_contract
    }

    /// Whether this point is the run's own reference point.
    #[must_use]
    pub const fn is_nominal(&self) -> bool {
        self.nominal
    }

    /// The point spelled the way a results table names it.
    #[must_use]
    pub fn label(&self) -> String {
        let mut label = self.process.clone();
        if let Some(voltage) = self.supply_voltage {
            label.push_str(&format!(" \u{00b7} {voltage} V"));
        }
        label.push_str(&format!(" \u{00b7} {} \u{00b0}C", self.temperature_celsius));
        label
    }
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
    authored_source_instance_id: AnalysisInstanceId,
    source_revision: ObjectRevision,
    prepared_snapshot_digest: ContentDigest,
    dependency_ids: Vec<AnalysisInstanceId>,
    /// The PVT point this result was solved at, when the task was expanded to
    /// exactly one. Never inferred: a task with no point keeps `None`.
    pvt_point: Option<AnalysisResultPvtPoint>,
}

impl AnalysisResultProvenance {
    /// Build a complete, internally consistent prepared-task provenance
    /// record. Dependency order is retained exactly as it appeared in the
    /// frozen prepared snapshot.
    #[cfg(test)]
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

    #[cfg(test)]
    pub fn new_with_source_domain(
        source_domain: AnalysisResultSourceDomain,
        source_instance_id: AnalysisInstanceId,
        source_revision: ObjectRevision,
        prepared_snapshot_digest: ContentDigest,
        dependency_ids: Vec<AnalysisInstanceId>,
    ) -> Result<Self, String> {
        Self::new_with_authored_source_domain(
            source_domain,
            source_instance_id,
            source_instance_id,
            source_revision,
            prepared_snapshot_digest,
            dependency_ids,
        )
    }

    pub fn new_with_authored_source_domain(
        source_domain: AnalysisResultSourceDomain,
        source_instance_id: AnalysisInstanceId,
        authored_source_instance_id: AnalysisInstanceId,
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
            authored_source_instance_id,
            source_revision,
            prepared_snapshot_digest,
            dependency_ids,
            pvt_point: None,
        })
    }

    /// Attach the PVT point the producing task was expanded to.
    ///
    /// Separate from the constructor because attribution is not universal:
    /// most tasks run once for the whole space and must keep no point at all.
    #[must_use]
    pub fn with_pvt_point(mut self, point: Option<AnalysisResultPvtPoint>) -> Self {
        self.pvt_point = point;
        self
    }

    /// The PVT point this result was solved at, or `None` when the producing
    /// task was not point-specific.
    #[must_use]
    pub const fn pvt_point(&self) -> Option<&AnalysisResultPvtPoint> {
        self.pvt_point.as_ref()
    }

    #[must_use]
    pub const fn source_domain(&self) -> AnalysisResultSourceDomain {
        self.source_domain
    }

    #[must_use]
    pub const fn source_instance_id(&self) -> AnalysisInstanceId {
        self.source_instance_id
    }

    /// Stable plan identity that authored this result. It differs from the
    /// execution identity for deterministically expanded PVT points.
    #[must_use]
    pub const fn authored_source_instance_id(&self) -> AnalysisInstanceId {
        self.authored_source_instance_id
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
    pub mechanism: String,
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
    /// Total integrated output noise over the band (V rms). `None` means the
    /// selected execution policy intentionally omitted this evidence.
    pub total_rms: Option<f64>,
    /// Total integrated input-referred noise (V rms), retained only when the
    /// named-source normalization was validated and the policy requested it.
    pub input_rms: Option<f64>,
    /// Analysis band, for the panel header (Hz).
    pub band: (f64, f64),
}

/// One exact complex value retained from an analysis result.
///
/// This is deliberately independent of the pole-zero viewer's presentation
/// model. Root classification is owned by [`AnalysisResultPayload`], while
/// this value preserves the solver's ordered real/imaginary evidence.
#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ComplexResultValue {
    pub real: f64,
    pub imaginary: f64,
}

/// Exact finite/infinite accounting and residual certificate retained for one
/// computed pole or zero spectrum.
#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PoleZeroSpectrumCertificate {
    pub problem_order: u64,
    pub infinite_count: u64,
    pub max_backward_error: f64,
    pub qualification_tolerance: f64,
}

impl PoleZeroSpectrumCertificate {
    #[must_use]
    pub fn canonical_qualification_tolerance(problem_order: u64) -> Option<f64> {
        let problem_order = usize::try_from(problem_order).ok()?;
        rspice_core::analysis::pole_zero::SpectrumCertificate::exact(problem_order, 0)
            .map(|certificate| certificate.qualification_tolerance)
    }

    fn as_core(self) -> Option<rspice_core::analysis::pole_zero::SpectrumCertificate> {
        rspice_core::analysis::pole_zero::SpectrumCertificate::new(
            usize::try_from(self.problem_order).ok()?,
            usize::try_from(self.infinite_count).ok()?,
            self.max_backward_error,
            self.qualification_tolerance,
        )
    }

    #[must_use]
    pub fn finite_count(self) -> Option<u64> {
        self.as_core()
            .and_then(|certificate| u64::try_from(certificate.finite_count()).ok())
    }

    #[must_use]
    pub fn is_strictly_qualified(self) -> bool {
        self.as_core()
            .is_some_and(|certificate| certificate.is_strictly_qualified())
    }
}

/// Qualification state attached to one retained pole or zero vector.
#[derive(Debug, Clone, PartialEq, Default, serde::Serialize, serde::Deserialize)]
#[serde(tag = "status", rename_all = "snake_case", deny_unknown_fields)]
pub enum PoleZeroRootSetEvidence {
    NotRequested,
    QualifiedEmpty {
        certificate: PoleZeroSpectrumCertificate,
    },
    Qualified {
        certificate: PoleZeroSpectrumCertificate,
    },
    Approximate {
        certificate: PoleZeroSpectrumCertificate,
    },
    /// Truthful migration state for results written before certificates were
    /// retained. This state never proves stability.
    #[default]
    LegacyUnknown,
}

impl PoleZeroRootSetEvidence {
    #[must_use]
    pub const fn label(&self) -> &'static str {
        match self {
            Self::NotRequested => "not requested",
            Self::QualifiedEmpty { .. } => "qualified empty",
            Self::Qualified { .. } => "qualified",
            Self::Approximate { .. } => "approximate",
            Self::LegacyUnknown => "legacy unknown",
        }
    }

    #[must_use]
    pub const fn certificate(&self) -> Option<PoleZeroSpectrumCertificate> {
        match self {
            Self::QualifiedEmpty { certificate }
            | Self::Qualified { certificate }
            | Self::Approximate { certificate } => Some(*certificate),
            Self::NotRequested | Self::LegacyUnknown => None,
        }
    }

    #[must_use]
    pub const fn is_qualified(&self) -> bool {
        matches!(self, Self::QualifiedEmpty { .. } | Self::Qualified { .. })
    }

    #[must_use]
    pub fn is_consistent_with_count(&self, root_count: usize) -> bool {
        let Ok(root_count) = u64::try_from(root_count) else {
            return false;
        };
        match self {
            Self::NotRequested => root_count == 0,
            Self::QualifiedEmpty { certificate } => {
                root_count == 0
                    && certificate.is_strictly_qualified()
                    && certificate.finite_count() == Some(0)
            }
            Self::Qualified { certificate } => {
                root_count > 0
                    && certificate.is_strictly_qualified()
                    && certificate.finite_count() == Some(root_count)
            }
            Self::Approximate { certificate } => certificate.as_core().is_some_and(|certificate| {
                !certificate.is_strictly_qualified()
                    && u64::try_from(certificate.finite_count()).ok() == Some(root_count)
            }),
            Self::LegacyUnknown => true,
        }
    }
}

/// Strict residual certificate for one complete retained Floquet spectrum.
///
/// This UI-owned representation is the durable serde contract. Validation is
/// delegated to the core constructor so a project cannot authenticate an
/// inflated qualification tolerance after the numerical contract changes.
#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FloquetSpectrumCertificateEvidence {
    pub problem_order: u64,
    pub max_backward_error: f64,
    pub qualification_tolerance: f64,
}

impl FloquetSpectrumCertificateEvidence {
    #[must_use]
    pub fn canonical_qualification_tolerance(problem_order: u64) -> Option<f64> {
        let problem_order = usize::try_from(problem_order).ok()?;
        Some(
            rspice_core::analysis::FloquetSpectrumCertificate::canonical_qualification_tolerance(
                problem_order,
            ),
        )
    }

    fn as_core(self) -> Option<rspice_core::analysis::FloquetSpectrumCertificate> {
        rspice_core::analysis::FloquetSpectrumCertificate::new(
            usize::try_from(self.problem_order).ok()?,
            self.max_backward_error,
            self.qualification_tolerance,
        )
    }

    #[must_use]
    pub fn is_strictly_qualified(self) -> bool {
        self.as_core().is_some()
    }
}

/// Provenance for a durable Floquet multiplier vector.
#[derive(Debug, Clone, PartialEq, Default, serde::Serialize, serde::Deserialize)]
#[serde(tag = "status", rename_all = "snake_case", deny_unknown_fields)]
pub enum FloquetSpectrumEvidence {
    /// Stability post-processing was not performed.
    NotComputed,
    /// The periodic map is an authenticated zero-order driven map.
    NoDynamicModes,
    /// Every multiplier belongs to a complete strict eigenspectrum.
    Qualified {
        certificate: FloquetSpectrumCertificateEvidence,
    },
    /// Truthful state for a project written before Floquet certificates were
    /// retained. It never proves a stability classification.
    #[default]
    LegacyUnknown,
}

impl FloquetSpectrumEvidence {
    #[must_use]
    pub const fn certificate(&self) -> Option<FloquetSpectrumCertificateEvidence> {
        match self {
            Self::Qualified { certificate } => Some(*certificate),
            Self::NotComputed | Self::NoDynamicModes | Self::LegacyUnknown => None,
        }
    }

    fn is_consistent_with_count(&self, multiplier_count: usize) -> bool {
        match self {
            Self::NotComputed | Self::NoDynamicModes | Self::LegacyUnknown => multiplier_count == 0,
            Self::Qualified { certificate } => {
                multiplier_count > 0
                    && certificate.is_strictly_qualified()
                    && u64::try_from(multiplier_count).ok() == Some(certificate.problem_order)
            }
        }
    }

    fn as_core(&self) -> Option<rspice_core::analysis::FloquetSpectrumEvidence> {
        match self {
            Self::NotComputed => Some(rspice_core::analysis::FloquetSpectrumEvidence::NotComputed),
            Self::NoDynamicModes => {
                Some(rspice_core::analysis::FloquetSpectrumEvidence::NoDynamicModes)
            }
            Self::Qualified { certificate } => {
                Some(rspice_core::analysis::FloquetSpectrumEvidence::Qualified {
                    certificate: certificate.as_core()?,
                })
            }
            Self::LegacyUnknown => {
                Some(rspice_core::analysis::FloquetSpectrumEvidence::LegacyUnknown)
            }
        }
    }
}

/// Orbit policy used to interpret a retained Floquet spectrum.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FloquetOrbitKindEvidence {
    Driven,
    Autonomous,
    /// The producing project did not retain an orbit policy.
    #[default]
    LegacyUnknown,
}

/// Evidence-aware periodic stability verdict.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FloquetStabilityVerdictEvidence {
    Stable,
    Unstable,
    Marginal,
    Indeterminate,
}

/// Rich PSTB classification refining the shared four-state verdict.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PstbStabilityClassificationEvidence {
    Stable,
    UnstableReal,
    UnstableComplex,
    PeriodDoubling,
    NeimarkSacker,
    SaddleNode,
    Marginal,
    Indeterminate,
}

/// One exact multiplier in the complete PSS Floquet vector.
#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PssFloquetMultiplierEvidence {
    pub multiplier: ComplexResultValue,
}

/// One complete PSTB mode. The containing vector is authoritative and is
/// never truncated by presentation limits.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PstbFloquetModeEvidence {
    pub multiplier: ComplexResultValue,
    pub exponent: ComplexResultValue,
    pub probe_participation: f64,
    pub is_unstable: bool,
    pub is_trivial: bool,
    pub subharmonic_order: Option<u64>,
}

/// Analysis basis used to produce a retained sensitivity result.
#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "mode", rename_all = "snake_case", deny_unknown_fields)]
pub enum SensitivityResultMode {
    Dc,
    Ac { frequency_hz: f64 },
}

/// One parameter's exact raw and normalized sensitivity.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SensitivityResultRow {
    pub parameter: String,
    pub raw: f64,
    pub normalized: f64,
}

/// Exact stress metrics retained for one device in a reliability run.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReliabilityStressEvidence {
    pub average_gate_stress_v: f64,
    pub average_drain_stress_v: f64,
    pub average_temperature_k: f64,
    pub duration_s: f64,
}

/// Exact parameter shifts retained at one lifetime checkpoint.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReliabilityShiftEvidence {
    pub threshold_voltage_shift_v: f64,
    pub mobility_shift: f64,
    pub drain_source_resistance_shift: f64,
}

/// One numerically ordered lifetime checkpoint and its exact parameter shifts.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReliabilityCheckpointEvidence {
    pub years: f64,
    pub shift: ReliabilityShiftEvidence,
}

/// Immutable reliability evidence for one analyzed device.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReliabilityDeviceEvidence {
    pub device_id: String,
    pub stress: ReliabilityStressEvidence,
    pub checkpoints: Vec<ReliabilityCheckpointEvidence>,
}

/// One committed digital event on an XSPICE event node.
#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DigitalEventPointEvidence {
    pub time_s: f64,
    /// XSPICE 12-state event code, 0..=12. The producer is
    /// `rspice_core::xspice::DigitalValue::event_code`.
    pub value_code: u8,
}

/// The committed event history of one XSPICE digital node.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DigitalEventTraceEvidence {
    pub node_name: String,
    pub points: Vec<DigitalEventPointEvidence>,
}

/// One committed real-valued event on an XSPICE event node.
#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RealEventPointEvidence {
    pub time_s: f64,
    pub value: f64,
}

/// The committed event history of one XSPICE real-valued node.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RealEventTraceEvidence {
    pub node_name: String,
    pub points: Vec<RealEventPointEvidence>,
}

/// Electrical quantity governed by a retained safe-operating-area rule.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum SoaParameterEvidence {
    GateSourceVoltage,
    DrainSourceVoltage,
    GateDrainVoltage,
    BaseEmitterVoltage,
    CollectorEmitterVoltage,
    BaseCollectorVoltage,
    DrainCurrent,
    CollectorCurrent,
    PowerDissipation,
    Temperature,
}

/// Severity assigned by the SOA rule evaluator.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum SoaViolationSeverityEvidence {
    Warning,
    Violation,
    Critical,
}

/// Verdict for one fully evaluated safe-operating-area rule.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum SoaRuleVerdictEvidence {
    Pass,
    Warning,
    Violation,
    Critical,
}

impl SoaRuleVerdictEvidence {
    /// How this verdict is named wherever it is reported — the SOA sheet, the
    /// component inspector, and the printed evidence table all read it here so
    /// a rule cannot be called one thing on screen and another on paper.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Pass => "PASS",
            Self::Warning => "WARNING",
            Self::Violation => "VIOLATION",
            Self::Critical => "CRITICAL",
        }
    }
}

/// Complete worst-point and sampling evidence for one SOA rule.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SoaEvaluationEvidence {
    pub device_id: String,
    pub parameter: SoaParameterEvidence,
    pub limit_value: f64,
    pub worst_actual_value: f64,
    pub worst_time_s: f64,
    pub sample_count: u64,
    pub unit: String,
    pub description: String,
    pub verdict: SoaRuleVerdictEvidence,
}

/// One exact, source-attributed safe-operating-area violation.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SoaViolationEvidence {
    pub device_id: String,
    pub parameter: SoaParameterEvidence,
    pub limit_value: f64,
    pub actual_value: f64,
    pub time_s: f64,
    pub severity: SoaViolationSeverityEvidence,
}

/// Electrical quantity carried by one side of a retained transfer derivative.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TransferFunctionQuantityEvidence {
    Voltage,
    Current,
}

impl TransferFunctionQuantityEvidence {
    const fn canonical_unit(self) -> &'static str {
        match self {
            Self::Voltage => "V",
            Self::Current => "A",
        }
    }
}

/// Explicit JSON-safe scalar evidence.
///
/// Open-circuit transfer resistances are legitimately infinite. Encoding
/// infinity as a classification keeps persisted evidence standards-compliant
/// while preserving the mathematical result exactly. `Finite` is validated
/// separately so deserialization cannot smuggle NaN or infinity through it.
#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(
    tag = "classification",
    content = "value",
    rename_all = "snake_case",
    deny_unknown_fields
)]
pub enum TransferFunctionScalarEvidence {
    Finite(f64),
    PositiveInfinity,
    NegativeInfinity,
}

impl TransferFunctionScalarEvidence {
    #[must_use]
    pub fn from_f64(value: f64) -> Option<Self> {
        if value.is_nan() {
            None
        } else if value == f64::INFINITY {
            Some(Self::PositiveInfinity)
        } else if value == f64::NEG_INFINITY {
            Some(Self::NegativeInfinity)
        } else {
            Some(Self::Finite(value))
        }
    }

    #[must_use]
    pub const fn as_f64(self) -> f64 {
        match self {
            Self::Finite(value) => value,
            Self::PositiveInfinity => f64::INFINITY,
            Self::NegativeInfinity => f64::NEG_INFINITY,
        }
    }

    fn validate(self, label: &str) -> Result<(), String> {
        if let Self::Finite(value) = self
            && !value.is_finite()
        {
            return Err(format!(
                "transfer-function {label} uses a non-finite value in the finite classification"
            ));
        }
        Ok(())
    }
}

/// Gain-normalization policy actually applied to retained TF evidence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TransferFunctionNormalizationEvidence {
    None,
    RelativeToNominal,
    PerSourceUnit,
}

/// Numerical policy actually applied to the TF operating-point solves.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TransferFunctionAccuracyEvidence {
    Fast,
    Balanced,
    Accurate,
    Robust,
}

macro_rules! op_evidence_enum {
    ($name:ident { $($variant:ident),+ $(,)? }) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
        #[serde(rename_all = "snake_case")]
        pub enum $name { $($variant),+ }
    };
    ($name:ident { default $first:ident, $($variant:ident),+ $(,)? }) => {
        #[derive(Debug, Clone, Copy, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
        #[serde(rename_all = "snake_case")]
        pub enum $name { #[default] $first, $($variant),+ }
    };
}

op_evidence_enum!(OperatingPointTemperatureEvidence {
    PvtRunSet,
    Nominal27C,
    Explicit,
    ActiveRunSetAxis
});
op_evidence_enum!(OperatingPointInitialGuessEvidence {
    Automatic,
    PreviousConverged,
    UserNodeVoltages,
    ZeroState
});
op_evidence_enum!(OperatingPointNodeInitializationEvidence {
    UseIcAndNodeset,
    IgnoreIcAndNodeset,
    ForceIcValues,
    ValidateOnly
});
op_evidence_enum!(OperatingPointHomotopyEvidence {
    Adaptive,
    SourceStepping,
    GminStepping,
    PseudoTransient,
    None
});
op_evidence_enum!(OperatingPointAnnotationEvidence {
    VoltagesAndCurrents,
    VoltagesOnly,
    VoltagesAndDeviceOp,
    None
});
op_evidence_enum!(OperatingPointDeviceDetailEvidence {
    SelectedAndViolations,
    AllDevices,
    ViolationsOnly,
    None
});
op_evidence_enum!(OperatingPointSaveDeviceEvidence {
    Enabled,
    Disabled,
    FinalPointOnly
});
op_evidence_enum!(OperatingPointAccuracyEvidence {
    Fast,
    Balanced,
    Accurate,
    Robust
});
op_evidence_enum!(OperatingPointProcessEvidence {
    default TT,
    SS,
    FF,
    SF,
    FS
});

const fn default_op_run_point_count() -> u64 {
    1
}

fn native_scalar_name_matches(name: &str, canonical: &str, dotted_compatibility: &str) -> bool {
    name.eq_ignore_ascii_case(canonical) || name.eq_ignore_ascii_case(dotted_compatibility)
}

/// Immutable, analysis-native result evidence that is neither waveform data
/// nor presentation state.
///
/// The payload is persisted, content-digested, and selected with its owning
/// [`AnalysisResult`]. Viewers must derive from this value instead of keeping
/// a second mutable copy of engineering data.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum AnalysisResultPayload {
    OperatingPoint {
        temperature_mode: OperatingPointTemperatureEvidence,
        temperature_celsius: f64,
        initial_guess: OperatingPointInitialGuessEvidence,
        node_initialization: OperatingPointNodeInitializationEvidence,
        homotopy: OperatingPointHomotopyEvidence,
        annotation: OperatingPointAnnotationEvidence,
        device_detail: OperatingPointDeviceDetailEvidence,
        save_device_op: OperatingPointSaveDeviceEvidence,
        accuracy: OperatingPointAccuracyEvidence,
        selected_devices: Vec<String>,
        #[serde(default)]
        violation_devices: Vec<String>,
        #[serde(default)]
        violation_source_content_digest: Option<crate::product::ContentDigest>,
        validated_startup_directives: u64,
        #[serde(default)]
        mna_node_names: Vec<String>,
        #[serde(default)]
        mna_branch_names: Vec<String>,
        #[serde(default)]
        mna_solution: Vec<f64>,
        /// Exact executable source plus voltage-corner mutation that produced
        /// this state. Absent only on legacy retained results, which are not
        /// eligible for Previous-converged startup.
        #[serde(default)]
        effective_source_content_digest: Option<crate::product::ContentDigest>,
        #[serde(default)]
        run_point_index: u64,
        #[serde(default = "default_op_run_point_count")]
        run_point_count: u64,
        #[serde(default)]
        run_point_process: OperatingPointProcessEvidence,
        #[serde(default)]
        run_point_supply_voltage: Option<f64>,
        #[serde(default)]
        run_point_nominal_supply_voltage: Option<f64>,
    },
    PoleZero {
        poles: Vec<ComplexResultValue>,
        zeros: Vec<ComplexResultValue>,
        #[serde(default)]
        pole_evidence: PoleZeroRootSetEvidence,
        #[serde(default)]
        zero_evidence: PoleZeroRootSetEvidence,
        /// Finite DC gain when defined. Missing legacy fields deserialize as
        /// unavailable; legacy numeric fields deserialize as `Some(value)`.
        #[serde(default)]
        gain: Option<f64>,
    },
    /// Durable periodic-steady-state Floquet evidence. Execution artifacts
    /// such as the monodromy matrix and shooting state are intentionally not
    /// part of the project result contract.
    PssFloquet {
        /// Absent only on an authenticated schema-v16 migration marker.
        period_s: Option<f64>,
        /// Absent only on an authenticated schema-v16 migration marker.
        fundamental_frequency_hz: Option<f64>,
        /// Absent only on an authenticated schema-v16 migration marker.
        iterations: Option<u64>,
        /// Absent only on an authenticated schema-v16 migration marker.
        residual_norm: Option<f64>,
        multipliers: Vec<PssFloquetMultiplierEvidence>,
        floquet_evidence: FloquetSpectrumEvidence,
        orbit_kind: FloquetOrbitKindEvidence,
        trivial_multiplier_index: Option<u64>,
        stability_verdict: FloquetStabilityVerdictEvidence,
    },
    /// Durable periodic-stability evidence. `modes` is the complete sorted
    /// spectrum; ordinary result waveforms remain presentation-only subsets.
    Pstb {
        /// Absent only on an authenticated schema-v16 migration marker.
        period_s: Option<f64>,
        /// Absent only on an authenticated schema-v16 migration marker.
        fundamental_frequency_hz: Option<f64>,
        /// Exact outer multiplier-magnitude boundary used by the producer.
        stability_threshold: Option<f64>,
        /// Canonical circuit identity whose eigenvector participation is stored.
        probe_instance: Option<String>,
        /// Whether root-of-unity classification was requested.
        detect_subharmonics: Option<bool>,
        modes: Vec<PstbFloquetModeEvidence>,
        floquet_evidence: FloquetSpectrumEvidence,
        orbit_kind: FloquetOrbitKindEvidence,
        trivial_multiplier_index: Option<u64>,
        stability_verdict: FloquetStabilityVerdictEvidence,
        stability_classification: PstbStabilityClassificationEvidence,
        /// None is a real current result when there is no applicable
        /// non-trivial mode; it is never represented by infinity.
        min_stability_margin_db: Option<f64>,
        /// Absent only on an authenticated schema-v16 migration marker.
        max_multiplier_magnitude: Option<f64>,
        /// Absent only on an authenticated schema-v16 migration marker.
        num_unstable: Option<u64>,
        subharmonics: Vec<u64>,
        /// Absent only on an authenticated schema-v16 migration marker.
        converged: Option<bool>,
        /// Absent only on an authenticated schema-v16 migration marker.
        iterations: Option<u64>,
    },
    Sensitivity {
        output: String,
        result_mode: SensitivityResultMode,
        rows: Vec<SensitivityResultRow>,
    },
    ScalarMeasurements {
        values: BTreeMap<String, f64>,
    },
    TransferFunction {
        input_source: String,
        output_expression: String,
        input_quantity: TransferFunctionQuantityEvidence,
        output_quantity: TransferFunctionQuantityEvidence,
        input_unit: String,
        output_unit: String,
        normalization: TransferFunctionNormalizationEvidence,
        accuracy: TransferFunctionAccuracyEvidence,
        gain: Option<TransferFunctionScalarEvidence>,
        input_resistance: Option<TransferFunctionScalarEvidence>,
        output_resistance: Option<TransferFunctionScalarEvidence>,
        /// Nominal source value used by relative normalization, otherwise absent.
        nominal_input: Option<f64>,
        /// Nominal output value used by relative normalization, otherwise absent.
        nominal_output: Option<f64>,
    },
    Reliability {
        devices: Vec<ReliabilityDeviceEvidence>,
    },
    Soa {
        evaluations: Vec<SoaEvaluationEvidence>,
        violations: Vec<SoaViolationEvidence>,
    },
    /// Committed XSPICE event histories from a transient run.
    ///
    /// Events are the sparse schedule the event solver accepted, not the
    /// analog timestep grid, so they are retained as their own evidence
    /// rather than resampled into waveforms.
    TransientEvents {
        digital_traces: Vec<DigitalEventTraceEvidence>,
        real_traces: Vec<RealEventTraceEvidence>,
    },
}

impl AnalysisResultPayload {
    /// Construct the only truthful periodic-stability payload that can be
    /// added while migrating a pre-v17 successful result. No numerical
    /// evidence or orbit policy is inferred from presentation waveforms.
    pub(crate) fn legacy_periodic_marker(analysis_type: AnalysisType) -> Option<Self> {
        match analysis_type {
            AnalysisType::Pss => Some(Self::PssFloquet {
                period_s: None,
                fundamental_frequency_hz: None,
                iterations: None,
                residual_norm: None,
                multipliers: Vec::new(),
                floquet_evidence: FloquetSpectrumEvidence::LegacyUnknown,
                orbit_kind: FloquetOrbitKindEvidence::LegacyUnknown,
                trivial_multiplier_index: None,
                stability_verdict: FloquetStabilityVerdictEvidence::Indeterminate,
            }),
            AnalysisType::Pstb => Some(Self::Pstb {
                period_s: None,
                fundamental_frequency_hz: None,
                stability_threshold: None,
                probe_instance: None,
                detect_subharmonics: None,
                modes: Vec::new(),
                floquet_evidence: FloquetSpectrumEvidence::LegacyUnknown,
                orbit_kind: FloquetOrbitKindEvidence::LegacyUnknown,
                trivial_multiplier_index: None,
                stability_verdict: FloquetStabilityVerdictEvidence::Indeterminate,
                stability_classification: PstbStabilityClassificationEvidence::Indeterminate,
                min_stability_margin_db: None,
                max_multiplier_magnitude: None,
                num_unstable: None,
                subharmonics: Vec::new(),
                converged: None,
                iterations: None,
            }),
            _ => None,
        }
    }

    fn scalar_evidence(&self, name: &str) -> Option<ScalarEvidenceCandidate> {
        let value = match self {
            Self::PssFloquet {
                period_s,
                fundamental_frequency_hz,
                multipliers,
                floquet_evidence,
                ..
            } => {
                if native_scalar_name_matches(name, "pss_period", "pss.period") {
                    *period_s
                } else if native_scalar_name_matches(
                    name,
                    "pss_fundamental_frequency",
                    "pss.fundamental_frequency",
                ) {
                    *fundamental_frequency_hz
                } else if native_scalar_name_matches(name, "pss_mode_count", "pss.mode_count")
                    && matches!(
                        floquet_evidence,
                        FloquetSpectrumEvidence::NoDynamicModes
                            | FloquetSpectrumEvidence::Qualified { .. }
                    )
                {
                    Some(multipliers.len() as f64)
                } else {
                    None
                }
            }
            Self::Pstb {
                period_s,
                fundamental_frequency_hz,
                modes,
                floquet_evidence,
                min_stability_margin_db,
                max_multiplier_magnitude,
                num_unstable,
                ..
            } => {
                if native_scalar_name_matches(name, "pstb_period", "pstb.period") {
                    *period_s
                } else if native_scalar_name_matches(
                    name,
                    "pstb_fundamental_frequency",
                    "pstb.fundamental_frequency",
                ) {
                    *fundamental_frequency_hz
                } else if native_scalar_name_matches(name, "pstb_mode_count", "pstb.mode_count")
                    && matches!(
                        floquet_evidence,
                        FloquetSpectrumEvidence::NoDynamicModes
                            | FloquetSpectrumEvidence::Qualified { .. }
                    )
                {
                    Some(modes.len() as f64)
                } else if native_scalar_name_matches(
                    name,
                    "pstb_unstable_mode_count",
                    "pstb.unstable_mode_count",
                ) {
                    num_unstable.map(|count| count as f64)
                } else if native_scalar_name_matches(
                    name,
                    "pstb_max_multiplier_magnitude",
                    "pstb.max_multiplier_magnitude",
                ) {
                    *max_multiplier_magnitude
                } else if native_scalar_name_matches(
                    name,
                    "pstb_min_stability_margin_db",
                    "pstb.min_stability_margin_db",
                ) {
                    *min_stability_margin_db
                } else {
                    None
                }
            }
            Self::ScalarMeasurements { values } => values
                .iter()
                .find(|(key, _)| key.eq_ignore_ascii_case(name))
                .map(|(_, value)| *value),
            _ => None,
        }?;
        if !value.is_finite() {
            return None;
        }

        Some(ScalarEvidenceCandidate {
            value: Some(value),
            passed: true,
        })
    }

    fn scalar_evidence_names(&self) -> Vec<String> {
        match self {
            Self::PssFloquet {
                period_s,
                fundamental_frequency_hz,
                floquet_evidence,
                ..
            } => {
                let mut names = Vec::with_capacity(3);
                if period_s.is_some() {
                    names.push("pss_period".to_owned());
                }
                if fundamental_frequency_hz.is_some() {
                    names.push("pss_fundamental_frequency".to_owned());
                }
                if matches!(
                    floquet_evidence,
                    FloquetSpectrumEvidence::NoDynamicModes
                        | FloquetSpectrumEvidence::Qualified { .. }
                ) {
                    names.push("pss_mode_count".to_owned());
                }
                names
            }
            Self::Pstb {
                period_s,
                fundamental_frequency_hz,
                floquet_evidence,
                min_stability_margin_db,
                max_multiplier_magnitude,
                num_unstable,
                ..
            } => {
                let mut names = Vec::with_capacity(6);
                if period_s.is_some() {
                    names.push("pstb_period".to_owned());
                }
                if fundamental_frequency_hz.is_some() {
                    names.push("pstb_fundamental_frequency".to_owned());
                }
                if matches!(
                    floquet_evidence,
                    FloquetSpectrumEvidence::NoDynamicModes
                        | FloquetSpectrumEvidence::Qualified { .. }
                ) {
                    names.push("pstb_mode_count".to_owned());
                }
                if num_unstable.is_some() {
                    names.push("pstb_unstable_mode_count".to_owned());
                }
                if max_multiplier_magnitude.is_some() {
                    names.push("pstb_max_multiplier_magnitude".to_owned());
                }
                if min_stability_margin_db.is_some() {
                    names.push("pstb_min_stability_margin_db".to_owned());
                }
                names
            }
            Self::ScalarMeasurements { values } => values.keys().cloned().collect(),
            _ => Vec::new(),
        }
    }

    /// Validate exact retained evidence against the analysis that owns it.
    pub fn validate_for(&self, analysis_type: AnalysisType) -> Result<(), String> {
        match self {
            Self::OperatingPoint {
                temperature_celsius,
                selected_devices,
                violation_devices,
                violation_source_content_digest,
                mna_node_names,
                mna_branch_names,
                mna_solution,
                effective_source_content_digest: _,
                run_point_index,
                run_point_count,
                run_point_supply_voltage,
                run_point_nominal_supply_voltage,
                ..
            } => {
                if analysis_type != AnalysisType::DcOp {
                    return Err(format!(
                        "operating-point payload does not match analysis type {analysis_type:?}"
                    ));
                }
                if !temperature_celsius.is_finite() || *temperature_celsius <= -273.15 {
                    return Err("operating-point payload has an invalid temperature".to_owned());
                }
                if selected_devices.iter().any(|name| {
                    name.is_empty() || name.trim() != name || name.chars().any(char::is_whitespace)
                }) || selected_devices.windows(2).any(|pair| pair[0] >= pair[1])
                {
                    return Err(
                        "operating-point selected devices are not canonical sorted identities"
                            .to_owned(),
                    );
                }
                if violation_devices.iter().any(|name| {
                    name.is_empty() || name.trim() != name || name.chars().any(char::is_whitespace)
                }) || violation_devices.windows(2).any(|pair| pair[0] >= pair[1])
                {
                    return Err(
                        "operating-point violation devices are not canonical sorted identities"
                            .to_owned(),
                    );
                }
                if violation_devices.is_empty() != violation_source_content_digest.is_none() {
                    return Err(
                        "operating-point SOA devices are missing their source identity".to_owned(),
                    );
                }
                let ordered_len = mna_node_names.len().saturating_add(mna_branch_names.len());
                if (!mna_solution.is_empty() && ordered_len != mna_solution.len())
                    || mna_solution.iter().any(|value| !value.is_finite())
                    || mna_node_names
                        .iter()
                        .chain(mna_branch_names)
                        .any(|name| name.is_empty() || name.trim() != name)
                {
                    return Err(
                        "operating-point retained MNA state is incomplete or invalid".to_owned(),
                    );
                }
                if *run_point_count == 0 || *run_point_index >= *run_point_count {
                    return Err("operating-point retained run-point position is invalid".to_owned());
                }
                match (run_point_supply_voltage, run_point_nominal_supply_voltage) {
                    (None, None) => {}
                    (Some(supply), Some(nominal))
                        if supply.is_finite()
                            && *supply > 0.0
                            && nominal.is_finite()
                            && *nominal > 0.0 => {}
                    _ => {
                        return Err(
                            "operating-point retained PVT supply evidence is invalid".to_owned()
                        );
                    }
                }
            }
            Self::PoleZero {
                poles,
                zeros,
                pole_evidence,
                zero_evidence,
                gain,
            } => {
                if analysis_type != AnalysisType::PoleZero {
                    return Err(format!(
                        "pole-zero payload does not match analysis type {analysis_type:?}"
                    ));
                }
                validate_complex_values(poles, "pole")?;
                validate_complex_values(zeros, "zero")?;
                if !pole_evidence.is_consistent_with_count(poles.len()) {
                    return Err(
                        "pole-zero pole evidence is inconsistent with retained roots".to_owned(),
                    );
                }
                if !zero_evidence.is_consistent_with_count(zeros.len()) {
                    return Err(
                        "pole-zero zero evidence is inconsistent with retained roots".to_owned(),
                    );
                }
                if gain.is_some_and(|gain| !gain.is_finite()) {
                    return Err("pole-zero gain is non-finite".to_owned());
                }
            }
            Self::PssFloquet {
                period_s,
                fundamental_frequency_hz,
                iterations,
                residual_norm,
                multipliers,
                floquet_evidence,
                orbit_kind,
                trivial_multiplier_index,
                stability_verdict,
            } => {
                if analysis_type != AnalysisType::Pss {
                    return Err(format!(
                        "PSS Floquet payload does not match analysis type {analysis_type:?}"
                    ));
                }
                validate_pss_floquet_payload(
                    *period_s,
                    *fundamental_frequency_hz,
                    *iterations,
                    *residual_norm,
                    multipliers,
                    floquet_evidence,
                    *orbit_kind,
                    *trivial_multiplier_index,
                    *stability_verdict,
                )?;
            }
            Self::Pstb {
                period_s,
                fundamental_frequency_hz,
                stability_threshold,
                probe_instance,
                detect_subharmonics,
                modes,
                floquet_evidence,
                orbit_kind,
                trivial_multiplier_index,
                stability_verdict,
                stability_classification,
                min_stability_margin_db,
                max_multiplier_magnitude,
                num_unstable,
                subharmonics,
                converged,
                iterations,
            } => {
                if analysis_type != AnalysisType::Pstb {
                    return Err(format!(
                        "PSTB payload does not match analysis type {analysis_type:?}"
                    ));
                }
                validate_pstb_payload(
                    *period_s,
                    *fundamental_frequency_hz,
                    *stability_threshold,
                    probe_instance.as_deref(),
                    *detect_subharmonics,
                    modes,
                    floquet_evidence,
                    *orbit_kind,
                    *trivial_multiplier_index,
                    *stability_verdict,
                    *stability_classification,
                    *min_stability_margin_db,
                    *max_multiplier_magnitude,
                    *num_unstable,
                    subharmonics,
                    *converged,
                    *iterations,
                )?;
            }
            Self::Sensitivity {
                output,
                result_mode,
                rows,
            } => {
                if analysis_type != AnalysisType::Sensitivity {
                    return Err(format!(
                        "sensitivity payload does not match analysis type {analysis_type:?}"
                    ));
                }
                require_non_empty(output, "sensitivity output")?;
                if let SensitivityResultMode::Ac { frequency_hz } = result_mode
                    && (!frequency_hz.is_finite() || *frequency_hz <= 0.0)
                {
                    return Err(
                        "sensitivity AC frequency must be finite and greater than zero".to_owned(),
                    );
                }
                let mut previous_name: Option<&str> = None;
                for row in rows {
                    require_non_empty(&row.parameter, "sensitivity parameter")?;
                    if previous_name.is_some_and(|previous| previous >= row.parameter.as_str()) {
                        return Err(
                            "sensitivity rows must have unique, strictly sorted parameter names"
                                .to_owned(),
                        );
                    }
                    previous_name = Some(&row.parameter);
                    if !row.raw.is_finite() || !row.normalized.is_finite() {
                        return Err(format!(
                            "sensitivity parameter '{}' has a non-finite value",
                            row.parameter
                        ));
                    }
                }
            }
            Self::ScalarMeasurements { values } => {
                if matches!(
                    analysis_type,
                    AnalysisType::PoleZero | AnalysisType::Sensitivity | AnalysisType::Tf
                ) {
                    return Err(format!(
                        "scalar result payload does not match analysis type {analysis_type:?}"
                    ));
                }
                for (name, value) in values {
                    require_non_empty(name, "scalar result name")?;
                    if !value.is_finite() {
                        return Err(format!("scalar result '{name}' is non-finite"));
                    }
                }
            }
            Self::TransferFunction {
                input_source,
                output_expression,
                input_quantity,
                output_quantity,
                input_unit,
                output_unit,
                normalization,
                gain,
                input_resistance,
                output_resistance,
                nominal_input,
                nominal_output,
                ..
            } => {
                if analysis_type != AnalysisType::Tf {
                    return Err(format!(
                        "transfer-function payload does not match analysis type {analysis_type:?}"
                    ));
                }
                require_non_empty(input_source, "transfer-function input source")?;
                if input_source.chars().any(char::is_whitespace) {
                    return Err("transfer-function input source contains whitespace".to_owned());
                }
                require_non_empty(output_expression, "transfer-function output expression")?;
                validate_transfer_function_output(output_expression, *output_quantity)?;

                if input_unit != input_quantity.canonical_unit() {
                    return Err(format!(
                        "transfer-function input unit '{input_unit}' does not match {input_quantity:?}"
                    ));
                }
                if output_unit != output_quantity.canonical_unit() {
                    return Err(format!(
                        "transfer-function output unit '{output_unit}' does not match {output_quantity:?}"
                    ));
                }
                if gain.is_none() && input_resistance.is_none() && output_resistance.is_none() {
                    return Err(
                        "transfer-function payload contains no requested scalar evidence"
                            .to_owned(),
                    );
                }
                for (label, scalar) in [
                    ("gain", gain.as_ref()),
                    ("input resistance", input_resistance.as_ref()),
                    ("output resistance", output_resistance.as_ref()),
                ] {
                    if let Some(scalar) = scalar {
                        scalar.validate(label)?;
                    }
                }

                let relative_gain = *normalization
                    == TransferFunctionNormalizationEvidence::RelativeToNominal
                    && gain.is_some();
                if relative_gain != nominal_input.is_some()
                    || relative_gain != nominal_output.is_some()
                {
                    return Err(
                        "transfer-function nominal values must be present exactly when a relative-normalized gain is retained"
                            .to_owned(),
                    );
                }
                for (label, value) in [
                    ("nominal input", nominal_input.as_ref()),
                    ("nominal output", nominal_output.as_ref()),
                ] {
                    if let Some(value) = value
                        && (!value.is_finite() || *value == 0.0)
                    {
                        return Err(format!(
                            "transfer-function {label} must be finite and nonzero"
                        ));
                    }
                }
            }
            Self::Reliability { devices } => {
                if analysis_type != AnalysisType::Reliability {
                    return Err(format!(
                        "reliability payload does not match analysis type {analysis_type:?}"
                    ));
                }
                if devices.is_empty() {
                    return Err("reliability payload contains no device evidence".to_owned());
                }
                let mut previous_device: Option<&str> = None;
                for device in devices {
                    require_non_empty(&device.device_id, "reliability device identity")?;
                    if previous_device.is_some_and(|previous| previous >= device.device_id.as_str())
                    {
                        return Err(
                            "reliability devices must have unique, strictly sorted identities"
                                .to_owned(),
                        );
                    }
                    previous_device = Some(&device.device_id);
                    for (label, value) in [
                        ("average gate stress", device.stress.average_gate_stress_v),
                        ("average drain stress", device.stress.average_drain_stress_v),
                        ("average temperature", device.stress.average_temperature_k),
                        ("stress duration", device.stress.duration_s),
                    ] {
                        if !value.is_finite() {
                            return Err(format!(
                                "reliability device '{}' has non-finite {label}",
                                device.device_id
                            ));
                        }
                    }
                    if device.stress.average_temperature_k <= 0.0 {
                        return Err(format!(
                            "reliability device '{}' has a non-positive absolute temperature",
                            device.device_id
                        ));
                    }
                    if device.stress.duration_s < 0.0 {
                        return Err(format!(
                            "reliability device '{}' has a negative stress duration",
                            device.device_id
                        ));
                    }
                    if device.checkpoints.is_empty() {
                        return Err(format!(
                            "reliability device '{}' has no lifetime checkpoints",
                            device.device_id
                        ));
                    }
                    let mut previous_years = None;
                    for checkpoint in &device.checkpoints {
                        if !checkpoint.years.is_finite() || checkpoint.years <= 0.0 {
                            return Err(format!(
                                "reliability device '{}' has an invalid lifetime checkpoint",
                                device.device_id
                            ));
                        }
                        if previous_years.is_some_and(|previous| previous >= checkpoint.years) {
                            return Err(format!(
                                "reliability device '{}' checkpoints must be unique and strictly increasing",
                                device.device_id
                            ));
                        }
                        previous_years = Some(checkpoint.years);
                        let shift = &checkpoint.shift;
                        if [
                            shift.threshold_voltage_shift_v,
                            shift.mobility_shift,
                            shift.drain_source_resistance_shift,
                        ]
                        .into_iter()
                        .any(|value| !value.is_finite())
                        {
                            return Err(format!(
                                "reliability device '{}' checkpoint '{}' years has a non-finite shift",
                                device.device_id, checkpoint.years
                            ));
                        }
                    }
                }
            }
            Self::Soa {
                evaluations,
                violations,
            } => {
                if analysis_type != AnalysisType::Soa {
                    return Err(format!(
                        "SOA payload does not match analysis type {analysis_type:?}"
                    ));
                }
                if evaluations.is_empty() {
                    return Err("SOA payload contains no evaluated-rule evidence".to_owned());
                }
                let mut previous_evaluation: Option<&SoaEvaluationEvidence> = None;
                for evaluation in evaluations {
                    require_non_empty(&evaluation.device_id, "SOA device identity")?;
                    require_non_empty(&evaluation.unit, "SOA rule unit")?;
                    require_non_empty(&evaluation.description, "SOA rule description")?;
                    for (label, value) in [
                        ("limit", evaluation.limit_value),
                        ("worst observed value", evaluation.worst_actual_value),
                        ("worst-point time", evaluation.worst_time_s),
                    ] {
                        if !value.is_finite() {
                            return Err(format!(
                                "SOA evaluation for '{}' has non-finite {label}",
                                evaluation.device_id
                            ));
                        }
                    }
                    if evaluation.limit_value <= 0.0 {
                        return Err(format!(
                            "SOA evaluation for '{}' has a non-positive limit",
                            evaluation.device_id
                        ));
                    }
                    if evaluation.worst_actual_value < 0.0
                        || evaluation.worst_time_s < 0.0
                        || evaluation.sample_count == 0
                    {
                        return Err(format!(
                            "SOA evaluation for '{}' has invalid sampling evidence",
                            evaluation.device_id
                        ));
                    }
                    let expected_verdict =
                        soa_rule_verdict(evaluation.worst_actual_value, evaluation.limit_value);
                    if evaluation.verdict != expected_verdict {
                        return Err(format!(
                            "SOA evaluation for '{}' has a verdict inconsistent with its worst value",
                            evaluation.device_id
                        ));
                    }
                    if previous_evaluation
                        .is_some_and(|previous| soa_evaluation_order(previous, evaluation).is_ge())
                    {
                        return Err(
                            "SOA evaluations must have unique canonical rule identities".to_owned()
                        );
                    }
                    previous_evaluation = Some(evaluation);
                }
                let mut previous: Option<&SoaViolationEvidence> = None;
                for violation in violations {
                    require_non_empty(&violation.device_id, "SOA device identity")?;
                    for (label, value) in [
                        ("limit", violation.limit_value),
                        ("observed value", violation.actual_value),
                        ("time", violation.time_s),
                    ] {
                        if !value.is_finite() {
                            return Err(format!(
                                "SOA violation for '{}' has non-finite {label}",
                                violation.device_id
                            ));
                        }
                    }
                    if violation.time_s < 0.0 {
                        return Err(format!(
                            "SOA violation for '{}' has a negative time",
                            violation.device_id
                        ));
                    }
                    if violation.limit_value <= 0.0 || violation.actual_value < 0.0 {
                        return Err(format!(
                            "SOA violation for '{}' has invalid magnitude evidence",
                            violation.device_id
                        ));
                    }
                    let expected_severity =
                        soa_violation_severity(violation.actual_value, violation.limit_value)
                            .ok_or_else(|| {
                                format!(
                                    "SOA event for '{}' does not meet the warning threshold",
                                    violation.device_id
                                )
                            })?;
                    if violation.severity != expected_severity {
                        return Err(format!(
                            "SOA event for '{}' has a severity inconsistent with its value",
                            violation.device_id
                        ));
                    }
                    if previous
                        .is_some_and(|previous| soa_violation_order(previous, violation).is_ge())
                    {
                        return Err(
                            "SOA violations must use unique canonical deterministic order"
                                .to_owned(),
                        );
                    }
                    previous = Some(violation);
                }
            }
            Self::TransientEvents {
                digital_traces,
                real_traces,
            } => {
                if analysis_type != AnalysisType::Transient {
                    return Err(format!(
                        "event payload does not match analysis type {analysis_type:?}"
                    ));
                }
                if digital_traces.is_empty() && real_traces.is_empty() {
                    return Err("event payload contains no retained event history".to_owned());
                }
                let mut seen = std::collections::BTreeSet::new();
                for trace in digital_traces {
                    require_non_empty(&trace.node_name, "event node identity")?;
                    if !seen.insert(trace.node_name.as_str()) {
                        return Err(format!(
                            "event node '{}' is retained more than once",
                            trace.node_name
                        ));
                    }
                    let times = trace.points.iter().map(|point| point.time_s);
                    validate_event_times(&trace.node_name, times)?;
                    // The typed decoder owns the encoding: a code it refuses
                    // is not a state this build can name, and asking it here
                    // leaves one spelling of that bound rather than a
                    // constant beside it that has to be kept in step.
                    if trace.points.iter().any(|point| {
                        rspice_core::xspice::DigitalValue::from_event_code(point.value_code)
                            .is_none()
                    }) {
                        return Err(format!(
                            "event node '{}' has a value outside the XSPICE 12-state encoding",
                            trace.node_name
                        ));
                    }
                }
                for trace in real_traces {
                    require_non_empty(&trace.node_name, "event node identity")?;
                    if !seen.insert(trace.node_name.as_str()) {
                        return Err(format!(
                            "event node '{}' is retained more than once",
                            trace.node_name
                        ));
                    }
                    let times = trace.points.iter().map(|point| point.time_s);
                    validate_event_times(&trace.node_name, times)?;
                    if trace.points.iter().any(|point| !point.value.is_finite()) {
                        return Err(format!(
                            "event node '{}' has a non-finite value",
                            trace.node_name
                        ));
                    }
                }
            }
        }
        Ok(())
    }

    #[must_use]
    pub fn has_data(&self) -> bool {
        match self {
            Self::OperatingPoint { .. }
            | Self::PoleZero { .. }
            | Self::PssFloquet { .. }
            | Self::Pstb { .. }
            | Self::Sensitivity { .. } => true,
            Self::ScalarMeasurements { values } => !values.is_empty(),
            Self::TransferFunction {
                gain,
                input_resistance,
                output_resistance,
                ..
            } => gain.is_some() || input_resistance.is_some() || output_resistance.is_some(),
            Self::Reliability { devices } => !devices.is_empty(),
            Self::Soa { evaluations, .. } => !evaluations.is_empty(),
            Self::TransientEvents {
                digital_traces,
                real_traces,
            } => !digital_traces.is_empty() || !real_traces.is_empty(),
        }
    }
}

mod result_payload;

pub(crate) use result_payload::ScalarEvidenceCandidate;
pub use result_payload::{AnalysisResult, PeriodicNoiseOutputQuantity};
use result_payload::{
    require_finite_values, require_non_empty, soa_evaluation_order, soa_rule_verdict,
    soa_violation_order, soa_violation_severity, strictly_increasing, validate_complex_values,
    validate_event_times, validate_pss_floquet_payload, validate_pstb_payload,
    validate_transfer_function_output,
};
