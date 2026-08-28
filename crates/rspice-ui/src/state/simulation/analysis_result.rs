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

/// The highest XSPICE event code a digital event may carry.
pub(crate) const MAX_DIGITAL_EVENT_CODE: u8 = 12;

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
                    if trace
                        .points
                        .iter()
                        .any(|point| point.value_code > MAX_DIGITAL_EVENT_CODE)
                    {
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

#[allow(clippy::too_many_arguments)]
fn validate_pss_floquet_payload(
    period_s: Option<f64>,
    fundamental_frequency_hz: Option<f64>,
    _iterations: Option<u64>,
    residual_norm: Option<f64>,
    multipliers: &[PssFloquetMultiplierEvidence],
    evidence: &FloquetSpectrumEvidence,
    orbit_kind: FloquetOrbitKindEvidence,
    trivial_multiplier_index: Option<u64>,
    verdict: FloquetStabilityVerdictEvidence,
) -> Result<(), String> {
    if matches!(evidence, FloquetSpectrumEvidence::LegacyUnknown)
        || orbit_kind == FloquetOrbitKindEvidence::LegacyUnknown
    {
        let exact_legacy_marker = period_s.is_none()
            && fundamental_frequency_hz.is_none()
            && _iterations.is_none()
            && residual_norm.is_none()
            && multipliers.is_empty()
            && matches!(evidence, FloquetSpectrumEvidence::LegacyUnknown)
            && orbit_kind == FloquetOrbitKindEvidence::LegacyUnknown
            && trivial_multiplier_index.is_none()
            && verdict == FloquetStabilityVerdictEvidence::Indeterminate;
        return exact_legacy_marker.then_some(()).ok_or_else(|| {
            "legacy PSS Floquet evidence is not an exact migration marker".to_owned()
        });
    }

    let (Some(period_s), Some(frequency_hz), Some(_), Some(residual_norm)) = (
        period_s,
        fundamental_frequency_hz,
        _iterations,
        residual_norm,
    ) else {
        return Err("current PSS Floquet payload is missing global metrics".to_owned());
    };
    if !period_s.is_finite()
        || period_s <= 0.0
        || !frequency_hz.is_finite()
        || frequency_hz <= 0.0
        || !same_retained_float(frequency_hz, 1.0 / period_s)
        || !residual_norm.is_finite()
        || residual_norm < 0.0
    {
        return Err("PSS Floquet period, frequency, or residual is invalid".to_owned());
    }

    let values = multipliers
        .iter()
        .map(|mode| mode.multiplier)
        .collect::<Vec<_>>();
    validate_complex_values(&values, "PSS Floquet multiplier")?;
    if !evidence.is_consistent_with_count(values.len()) {
        return Err(
            "PSS Floquet certificate does not cover the complete multiplier vector".to_owned(),
        );
    }
    if matches!(evidence, FloquetSpectrumEvidence::NoDynamicModes)
        && orbit_kind != FloquetOrbitKindEvidence::Driven
    {
        return Err(
            "a zero-order PSS Floquet spectrum must use the driven orbit policy".to_owned(),
        );
    }

    let expected_trivial = expected_trivial_floquet_index(&values, evidence, orbit_kind)?;
    if trivial_multiplier_index != expected_trivial {
        return Err("PSS autonomous phase-mode index is inconsistent with the spectrum".to_owned());
    }
    let expected_verdict = derive_floquet_verdict(
        &values,
        evidence,
        orbit_kind,
        trivial_multiplier_index,
        rspice_core::analysis::FLOQUET_UNIT_CIRCLE_BAND,
    )?;
    if verdict != expected_verdict {
        return Err("PSS stability verdict is inconsistent with its Floquet evidence".to_owned());
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn validate_pstb_payload(
    period_s: Option<f64>,
    fundamental_frequency_hz: Option<f64>,
    stability_threshold: Option<f64>,
    probe_instance: Option<&str>,
    detect_subharmonics: Option<bool>,
    modes: &[PstbFloquetModeEvidence],
    evidence: &FloquetSpectrumEvidence,
    orbit_kind: FloquetOrbitKindEvidence,
    trivial_multiplier_index: Option<u64>,
    verdict: FloquetStabilityVerdictEvidence,
    classification: PstbStabilityClassificationEvidence,
    min_stability_margin_db: Option<f64>,
    max_multiplier_magnitude: Option<f64>,
    num_unstable: Option<u64>,
    subharmonics: &[u64],
    converged: Option<bool>,
    iterations: Option<u64>,
) -> Result<(), String> {
    if matches!(evidence, FloquetSpectrumEvidence::LegacyUnknown)
        || orbit_kind == FloquetOrbitKindEvidence::LegacyUnknown
    {
        let exact_legacy_marker = period_s.is_none()
            && fundamental_frequency_hz.is_none()
            && stability_threshold.is_none()
            && probe_instance.is_none()
            && detect_subharmonics.is_none()
            && modes.is_empty()
            && matches!(evidence, FloquetSpectrumEvidence::LegacyUnknown)
            && orbit_kind == FloquetOrbitKindEvidence::LegacyUnknown
            && trivial_multiplier_index.is_none()
            && verdict == FloquetStabilityVerdictEvidence::Indeterminate
            && classification == PstbStabilityClassificationEvidence::Indeterminate
            && min_stability_margin_db.is_none()
            && max_multiplier_magnitude.is_none()
            && num_unstable.is_none()
            && subharmonics.is_empty()
            && converged.is_none()
            && iterations.is_none();
        return exact_legacy_marker
            .then_some(())
            .ok_or_else(|| "legacy PSTB evidence is not an exact migration marker".to_owned());
    }

    let (
        Some(period_s),
        Some(frequency_hz),
        Some(stability_threshold),
        Some(probe_instance),
        Some(detect_subharmonics),
        Some(max_multiplier_magnitude),
        Some(num_unstable),
        Some(true),
        Some(_),
    ) = (
        period_s,
        fundamental_frequency_hz,
        stability_threshold,
        probe_instance,
        detect_subharmonics,
        max_multiplier_magnitude,
        num_unstable,
        converged,
        iterations,
    )
    else {
        return Err(
            "current PSTB payload is missing provenance, convergence, or global metrics".to_owned(),
        );
    };
    if !period_s.is_finite()
        || period_s <= 0.0
        || !frequency_hz.is_finite()
        || frequency_hz <= 0.0
        || !same_retained_float(frequency_hz, 1.0 / period_s)
        || !stability_threshold.is_finite()
        || stability_threshold < 1.0
        || probe_instance.is_empty()
        || probe_instance.trim() != probe_instance
        || probe_instance
            .chars()
            .any(|character| character.is_control() || character.is_whitespace())
    {
        return Err(
            "PSTB period, frequency, stability boundary, or probe identity is invalid".to_owned(),
        );
    }
    if !matches!(
        evidence,
        FloquetSpectrumEvidence::NoDynamicModes | FloquetSpectrumEvidence::Qualified { .. }
    ) || !evidence.is_consistent_with_count(modes.len())
    {
        return Err("PSTB requires a complete current Floquet spectrum".to_owned());
    }
    if matches!(evidence, FloquetSpectrumEvidence::NoDynamicModes)
        && orbit_kind != FloquetOrbitKindEvidence::Driven
    {
        return Err("a zero-order PSTB spectrum must use the driven orbit policy".to_owned());
    }

    let values = modes.iter().map(|mode| mode.multiplier).collect::<Vec<_>>();
    validate_complex_values(&values, "PSTB Floquet multiplier")?;
    if !pstb_modes_are_canonically_sorted(modes) {
        return Err("PSTB Floquet modes are not in canonical sorted order".to_owned());
    }
    let expected_trivial = expected_trivial_floquet_index(&values, evidence, orbit_kind)?;
    if trivial_multiplier_index != expected_trivial {
        return Err(
            "PSTB autonomous phase-mode index is inconsistent with the spectrum".to_owned(),
        );
    }
    let trivial_index = trivial_multiplier_index
        .map(usize::try_from)
        .transpose()
        .map_err(|_| "PSTB phase-mode index does not fit this platform".to_owned())?;

    let mut expected_subharmonics = Vec::new();
    let mut expected_unstable_count = 0_u64;
    let mut expected_min_margin: Option<f64> = None;
    for (index, mode) in modes.iter().enumerate() {
        let value = complex_value(mode.multiplier);
        let magnitude = value.norm();
        if !magnitude.is_finite()
            || magnitude <= 0.0
            || !mode.exponent.real.is_finite()
            || !mode.exponent.imaginary.is_finite()
            || !mode.probe_participation.is_finite()
            || !(0.0..=1.0).contains(&mode.probe_participation)
        {
            return Err(format!(
                "PSTB Floquet mode {index} contains invalid numerical data"
            ));
        }
        let expected_exponent = value.ln() / period_s;
        if !same_retained_float(mode.exponent.real, expected_exponent.re)
            || !same_retained_float(mode.exponent.imaginary, expected_exponent.im)
        {
            return Err(format!(
                "PSTB Floquet mode {index} has an inconsistent exponent"
            ));
        }
        let expected_trivial_flag = trivial_index == Some(index);
        let expected_unstable = !expected_trivial_flag && magnitude > stability_threshold;
        if mode.is_trivial != expected_trivial_flag || mode.is_unstable != expected_unstable {
            return Err(format!(
                "PSTB Floquet mode {index} has inconsistent stability flags"
            ));
        }
        expected_unstable_count += u64::from(expected_unstable);

        if !expected_trivial_flag {
            let margin = -20.0 * magnitude.log10();
            if !margin.is_finite() {
                return Err(format!("PSTB Floquet mode {index} has a non-finite margin"));
            }
            expected_min_margin = Some(match expected_min_margin {
                Some(current) if current.total_cmp(&margin).is_le() => current,
                _ => margin,
            });
        }

        let detected_order = detect_subharmonics
            .then(|| detected_subharmonic_order(value))
            .flatten();
        if mode.subharmonic_order != detected_order {
            return Err(format!(
                "PSTB Floquet mode {index} has inconsistent subharmonic evidence"
            ));
        }
        if let Some(order) = detected_order {
            expected_subharmonics.push(order);
        }
    }

    let expected_max_magnitude = modes
        .first()
        .map_or(0.0, |mode| complex_value(mode.multiplier).norm());
    if !max_multiplier_magnitude.is_finite()
        || !same_retained_float(max_multiplier_magnitude, expected_max_magnitude)
        || num_unstable != expected_unstable_count
        || !same_optional_retained_float(min_stability_margin_db, expected_min_margin)
        || subharmonics != expected_subharmonics
    {
        return Err(
            "PSTB aggregate counts, margins, or subharmonics contradict the complete spectrum"
                .to_owned(),
        );
    }

    let expected_verdict = derive_floquet_verdict(
        &values,
        evidence,
        orbit_kind,
        trivial_multiplier_index,
        stability_threshold - 1.0,
    )?;
    if verdict != expected_verdict {
        return Err("PSTB stability verdict contradicts the complete Floquet spectrum".to_owned());
    }
    let expected_classification = classify_pstb_modes(modes, verdict, trivial_index)?;
    if classification != expected_classification {
        return Err(
            "PSTB rich stability classification contradicts the complete spectrum".to_owned(),
        );
    }
    Ok(())
}

fn expected_trivial_floquet_index(
    values: &[ComplexResultValue],
    evidence: &FloquetSpectrumEvidence,
    orbit_kind: FloquetOrbitKindEvidence,
) -> Result<Option<u64>, String> {
    match orbit_kind {
        FloquetOrbitKindEvidence::Driven => Ok(None),
        FloquetOrbitKindEvidence::Autonomous => {
            if !matches!(evidence, FloquetSpectrumEvidence::Qualified { .. }) {
                return Ok(None);
            }
            let values = values
                .iter()
                .copied()
                .map(complex_value)
                .collect::<Vec<_>>();
            rspice_core::analysis::select_autonomous_phase_mode(&values)
                .map(u64::try_from)
                .transpose()
                .map_err(|_| {
                    "Floquet phase-mode index does not fit the durable contract".to_owned()
                })
        }
        FloquetOrbitKindEvidence::LegacyUnknown => {
            Err("current Floquet evidence has an unknown orbit policy".to_owned())
        }
    }
}

fn derive_floquet_verdict(
    values: &[ComplexResultValue],
    evidence: &FloquetSpectrumEvidence,
    orbit_kind: FloquetOrbitKindEvidence,
    trivial_multiplier_index: Option<u64>,
    band: f64,
) -> Result<FloquetStabilityVerdictEvidence, String> {
    let values = values
        .iter()
        .copied()
        .map(complex_value)
        .collect::<Vec<_>>();
    let evidence = evidence
        .as_core()
        .ok_or_else(|| "Floquet certificate is not core-authentic".to_owned())?;
    let orbit_kind = match orbit_kind {
        FloquetOrbitKindEvidence::Driven => rspice_core::analysis::FloquetOrbitKind::Driven,
        FloquetOrbitKindEvidence::Autonomous => rspice_core::analysis::FloquetOrbitKind::Autonomous,
        FloquetOrbitKindEvidence::LegacyUnknown => {
            return Ok(FloquetStabilityVerdictEvidence::Indeterminate);
        }
    };
    let trivial_multiplier_index = trivial_multiplier_index
        .map(usize::try_from)
        .transpose()
        .map_err(|_| "Floquet phase-mode index does not fit this platform".to_owned())?;
    let verdict = match rspice_core::analysis::classify_floquet_stability(
        &values,
        &evidence,
        orbit_kind,
        trivial_multiplier_index,
        band,
    ) {
        rspice_core::analysis::FloquetStabilityVerdict::Stable => {
            FloquetStabilityVerdictEvidence::Stable
        }
        rspice_core::analysis::FloquetStabilityVerdict::Unstable => {
            FloquetStabilityVerdictEvidence::Unstable
        }
        rspice_core::analysis::FloquetStabilityVerdict::Marginal => {
            FloquetStabilityVerdictEvidence::Marginal
        }
        rspice_core::analysis::FloquetStabilityVerdict::Indeterminate => {
            FloquetStabilityVerdictEvidence::Indeterminate
        }
        // The durable schema must be deliberately revised before it can
        // authenticate a future core semantic state.
        _ => return Err("unsupported core Floquet stability verdict".to_owned()),
    };
    Ok(verdict)
}

fn classify_pstb_modes(
    modes: &[PstbFloquetModeEvidence],
    verdict: FloquetStabilityVerdictEvidence,
    trivial_index: Option<usize>,
) -> Result<PstbStabilityClassificationEvidence, String> {
    match verdict {
        FloquetStabilityVerdictEvidence::Stable => Ok(PstbStabilityClassificationEvidence::Stable),
        FloquetStabilityVerdictEvidence::Indeterminate => {
            Ok(PstbStabilityClassificationEvidence::Indeterminate)
        }
        FloquetStabilityVerdictEvidence::Unstable => {
            let dominant = modes.iter().find(|mode| mode.is_unstable).ok_or_else(|| {
                "PSTB unstable verdict has no mode outside the stability boundary".to_owned()
            })?;
            if dominant.multiplier.imaginary.abs() > 0.01 {
                Ok(PstbStabilityClassificationEvidence::UnstableComplex)
            } else {
                Ok(PstbStabilityClassificationEvidence::UnstableReal)
            }
        }
        FloquetStabilityVerdictEvidence::Marginal => {
            for (index, mode) in modes.iter().enumerate() {
                if trivial_index == Some(index) {
                    continue;
                }
                let value = complex_value(mode.multiplier);
                if (value + num_complex::Complex64::new(1.0, 0.0)).norm() < 0.01 {
                    return Ok(PstbStabilityClassificationEvidence::PeriodDoubling);
                }
                if (value - num_complex::Complex64::new(1.0, 0.0)).norm() < 0.01 {
                    return Ok(PstbStabilityClassificationEvidence::SaddleNode);
                }
                if (value.norm() - 1.0).abs() < 0.01 && value.im.abs() > 0.01 {
                    return Ok(PstbStabilityClassificationEvidence::NeimarkSacker);
                }
            }
            Ok(PstbStabilityClassificationEvidence::Marginal)
        }
    }
}

fn pstb_modes_are_canonically_sorted(modes: &[PstbFloquetModeEvidence]) -> bool {
    modes.windows(2).all(|pair| {
        let left = &pair[0].multiplier;
        let right = &pair[1].multiplier;
        complex_value(*right)
            .norm()
            .total_cmp(&complex_value(*left).norm())
            .then_with(|| left.real.total_cmp(&right.real))
            .then_with(|| left.imaginary.total_cmp(&right.imaginary))
            .is_le()
    })
}

fn detected_subharmonic_order(value: num_complex::Complex64) -> Option<u64> {
    if (value.norm() - 1.0).abs() > 0.01 {
        return None;
    }
    let angle = value.arg().abs();
    (2_u64..=8).find(|order| {
        let expected_angle = 2.0 * std::f64::consts::PI / *order as f64;
        (angle - expected_angle).abs() < 0.01
    })
}

fn complex_value(value: ComplexResultValue) -> num_complex::Complex64 {
    num_complex::Complex64::new(value.real, value.imaginary)
}

fn same_optional_retained_float(left: Option<f64>, right: Option<f64>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => same_retained_float(left, right),
        (None, None) => true,
        (Some(_), None) | (None, Some(_)) => false,
    }
}

/// An event history is a schedule: nonnegative, finite, and non-decreasing.
///
/// Non-decreasing, not strictly increasing. An event-driven solver settles a
/// node through several delta cycles at one physical time, and every one of
/// those transitions is a committed event with its own value. Their order is
/// the order they were committed in, which is the order they are stored in —
/// so a repeated timestamp is evidence, not corruption.
fn validate_event_times(node_name: &str, times: impl Iterator<Item = f64>) -> Result<(), String> {
    let mut previous: Option<f64> = None;
    let mut count = 0usize;
    for time in times {
        count += 1;
        if !time.is_finite() || time < 0.0 {
            return Err(format!(
                "event node '{node_name}' has an invalid event time"
            ));
        }
        if previous.is_some_and(|previous| previous > time) {
            return Err(format!(
                "event node '{node_name}' events must not move backwards in time"
            ));
        }
        previous = Some(time);
    }
    if count == 0 {
        return Err(format!("event node '{node_name}' retained no events"));
    }
    Ok(())
}

fn validate_transfer_function_output(
    expression: &str,
    expected_quantity: TransferFunctionQuantityEvidence,
) -> Result<(), String> {
    let trimmed = expression.trim();
    if expression != trimmed {
        return Err("transfer-function output contains surrounding whitespace".to_owned());
    }
    let expression = trimmed;
    let Some(open) = expression.find('(') else {
        return Err(
            "transfer-function output must use V(node), V(node,ref), or I(element)".to_owned(),
        );
    };
    if !expression.ends_with(')') || expression[open + 1..expression.len() - 1].contains(['(', ')'])
    {
        return Err("transfer-function output has unbalanced parentheses".to_owned());
    }
    let function = &expression[..open];
    let arguments = expression[open + 1..expression.len() - 1]
        .split(',')
        .collect::<Vec<_>>();
    if arguments.iter().any(|argument| {
        argument.is_empty()
            || *argument != argument.trim()
            || argument.chars().any(char::is_whitespace)
    }) {
        return Err("transfer-function output contains an invalid identifier".to_owned());
    }
    let quantity = if function.eq_ignore_ascii_case("V") && matches!(arguments.len(), 1 | 2) {
        TransferFunctionQuantityEvidence::Voltage
    } else if function.eq_ignore_ascii_case("I") && arguments.len() == 1 {
        TransferFunctionQuantityEvidence::Current
    } else {
        return Err(
            "transfer-function output must use V(node), V(node,ref), or I(element)".to_owned(),
        );
    };
    if quantity != expected_quantity {
        return Err("transfer-function output quantity contradicts its expression".to_owned());
    }
    Ok(())
}

fn soa_rule_verdict(actual: f64, limit: f64) -> SoaRuleVerdictEvidence {
    if actual > limit * 1.2 {
        SoaRuleVerdictEvidence::Critical
    } else if actual > limit {
        SoaRuleVerdictEvidence::Violation
    } else if actual > limit * 0.9 {
        SoaRuleVerdictEvidence::Warning
    } else {
        SoaRuleVerdictEvidence::Pass
    }
}

fn soa_violation_severity(actual: f64, limit: f64) -> Option<SoaViolationSeverityEvidence> {
    if actual > limit * 1.2 {
        Some(SoaViolationSeverityEvidence::Critical)
    } else if actual > limit {
        Some(SoaViolationSeverityEvidence::Violation)
    } else if actual > limit * 0.9 {
        Some(SoaViolationSeverityEvidence::Warning)
    } else {
        None
    }
}

fn soa_evaluation_order(
    left: &SoaEvaluationEvidence,
    right: &SoaEvaluationEvidence,
) -> std::cmp::Ordering {
    left.device_id
        .cmp(&right.device_id)
        .then_with(|| left.parameter.cmp(&right.parameter))
}

fn soa_violation_order(
    left: &SoaViolationEvidence,
    right: &SoaViolationEvidence,
) -> std::cmp::Ordering {
    left.device_id
        .cmp(&right.device_id)
        .then_with(|| left.time_s.total_cmp(&right.time_s))
        .then_with(|| left.parameter.cmp(&right.parameter))
        .then_with(|| left.severity.cmp(&right.severity))
        .then_with(|| left.limit_value.total_cmp(&right.limit_value))
        .then_with(|| left.actual_value.total_cmp(&right.actual_value))
}

fn validate_complex_values(values: &[ComplexResultValue], label: &str) -> Result<(), String> {
    for (index, value) in values.iter().enumerate() {
        if !value.real.is_finite() || !value.imaginary.is_finite() {
            return Err(format!("{label} {index} has a non-finite component"));
        }
    }
    Ok(())
}

/// Exact physical quantity retained for the primary periodic-noise trace.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PeriodicNoiseOutputQuantity {
    /// Output-referred voltage or current noise power spectral density.
    OutputNoisePowerSpectralDensity,
    /// Single-sideband phase noise L(f) in dBc/Hz.
    PhaseNoiseDbcPerHz,
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

fn strictly_increasing(values: &[f64]) -> bool {
    values
        .windows(2)
        .all(|pair| normalized_f64(pair[0]) < normalized_f64(pair[1]))
}

fn normalized_f64(value: f64) -> f64 {
    if value == 0.0 { 0.0 } else { value }
}

fn same_retained_float(left: f64, right: f64) -> bool {
    normalized_f64(left).to_bits() == normalized_f64(right).to_bits()
}

fn contains_retained_coordinate(sorted: &[f64], target: f64) -> bool {
    let target = normalized_f64(target);
    sorted
        .binary_search_by(|probe| normalized_f64(*probe).total_cmp(&target))
        .is_ok()
}

/// Single analysis result with metadata and waveforms.
///
/// This represents one analysis within a simulation run, containing
/// all the data needed to display results in the appropriate viewer.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct ScalarEvidenceCandidate {
    pub value: Option<f64>,
    pub passed: bool,
}

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
    /// Exact analysis-native evidence such as pole/zero roots, sensitivity
    /// rows, or scalar-only results. This is immutable retained data, not a
    /// viewer cache.
    pub result_payload: Option<AnalysisResultPayload>,
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
    /// Exact prepared-task identity. Missing only for migrated legacy result
    /// history that was written before source instance IDs existed.
    pub provenance: Option<AnalysisResultProvenance>,
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

    #[must_use]
    pub fn is_live_partial(&self) -> bool {
        !self.success && self.error_message.as_deref() == Some(LIVE_TRANSIENT_PARTIAL_MESSAGE)
    }

    /// Exact prepared-task provenance for current retained results.
    #[must_use]
    pub fn provenance(&self) -> Option<&AnalysisResultProvenance> {
        self.provenance.as_ref()
    }

    /// Exact scalar evidence exposed to specification and result-document
    /// consumers. Explicit `.MEAS` results take precedence over a same-named
    /// analysis-native scalar so one execution cannot be counted twice.
    pub(crate) fn scalar_evidence(&self, name: &str) -> Vec<ScalarEvidenceCandidate> {
        let name = name.trim();
        if name.is_empty() {
            return Vec::new();
        }

        let measurements = self
            .measurements
            .iter()
            .filter(|measurement| measurement.name.eq_ignore_ascii_case(name))
            .map(|measurement| ScalarEvidenceCandidate {
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
            .into_iter()
            .collect()
    }

    /// Canonical discoverable scalar names for the active retained dataset.
    /// These are evidence keys, not synthesized stability booleans.
    pub(crate) fn scalar_evidence_names(&self) -> Vec<String> {
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
            result_payload: None,
            measurements: Vec::new(),
            saved_output_receipts: Vec::new(),
            success: true,
            error_message: None,
            failure_attribution: None,
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
            result_payload: None,
            measurements: Vec::new(),
            saved_output_receipts: Vec::new(),
            success: false,
            error_message: Some(error.into()),
            failure_attribution: None,
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

    /// Validate relationships between independently versioned retained fields.
    /// Historical analyses may legitimately lack a newer payload; when both
    /// fields exist they must describe one coherent execution.
    pub fn validate_retained_evidence(&self) -> Result<(), String> {
        let mut waveform_names = HashSet::with_capacity(self.waveforms.len());
        for waveform in &self.waveforms {
            let name = waveform.name.trim();
            if name.is_empty() || waveform.name.chars().any(char::is_control) {
                return Err("retained waveform requires a non-empty control-free name".to_owned());
            }
            if !waveform_names.insert(waveform.name.as_str()) {
                return Err(format!(
                    "retained waveform name '{}' is duplicated",
                    waveform.name
                ));
            }
            if waveform.x.len() != waveform.y.len() {
                return Err(format!(
                    "retained waveform '{}' has {} coordinates but {} values",
                    waveform.name,
                    waveform.x.len(),
                    waveform.y.len()
                ));
            }
            if waveform
                .x
                .iter()
                .chain(waveform.y.iter())
                .any(|value| !value.is_finite())
            {
                return Err(format!(
                    "retained waveform '{}' contains a non-finite coordinate or value",
                    waveform.name
                ));
            }
            if waveform
                .unit
                .as_ref()
                .is_some_and(|unit| unit.trim().is_empty() || unit.chars().any(char::is_control))
            {
                return Err(format!(
                    "retained waveform '{}' has an invalid engineering unit",
                    waveform.name
                ));
            }
            if let Some(complex) = &waveform.complex {
                if complex.source_name.trim().is_empty()
                    || complex.source_name.chars().any(char::is_control)
                {
                    return Err(format!(
                        "retained waveform '{}' has an invalid complex-source name",
                        waveform.name
                    ));
                }
                if complex.real.len() != waveform.x.len() || complex.imag.len() != waveform.x.len()
                {
                    return Err(format!(
                        "retained waveform '{}' complex components do not match its {} coordinates",
                        waveform.name,
                        waveform.x.len()
                    ));
                }
                if complex
                    .real
                    .iter()
                    .chain(complex.imag.iter())
                    .any(|value| !value.is_finite())
                {
                    return Err(format!(
                        "retained waveform '{}' contains a non-finite complex component",
                        waveform.name
                    ));
                }
            }
        }

        let valid_text =
            |text: &str| !text.trim().is_empty() && !text.chars().any(char::is_control);
        if let Some(dc_op) = &self.dc_op {
            for (group, values) in [
                ("node voltage", dc_op.node_voltages.as_slice()),
                ("branch current", dc_op.branch_currents.as_slice()),
                ("device power", dc_op.power_dissipation.as_slice()),
            ] {
                let mut names = HashSet::with_capacity(values.len());
                for value in values {
                    if !valid_text(&value.name) || !valid_text(&value.unit) {
                        return Err(format!(
                            "retained {group} requires a valid canonical name and engineering unit"
                        ));
                    }
                    if !names.insert(value.name.as_str()) {
                        return Err(format!("retained {group} '{}' is duplicated", value.name));
                    }
                    if !value.value.is_finite() {
                        return Err(format!(
                            "retained {group} '{}' contains a non-finite value",
                            value.name
                        ));
                    }
                }
            }
        }
        if let Some(report) = &self.device_op {
            if !report.labels_resolve() {
                return Err("retained device operating-point labels do not resolve".to_owned());
            }
            let mut devices = HashSet::with_capacity(report.entries.len());
            for entry in &report.entries {
                if !valid_text(&entry.name) || !devices.insert(entry.name.as_str()) {
                    return Err(format!(
                        "retained device operating-point identity '{}' is invalid or duplicated",
                        entry.name
                    ));
                }
                if entry.params.iter().any(|(_, value)| !value.is_finite()) {
                    return Err(format!(
                        "retained device operating-point entry '{}' contains a non-finite value",
                        entry.name
                    ));
                }
            }
        }
        if let Some(noise) = &self.noise_summary {
            if !noise.band.0.is_finite()
                || !noise.band.1.is_finite()
                || noise.band.0 < 0.0
                || noise.band.1 < noise.band.0
                || noise
                    .total_rms
                    .is_some_and(|value| !value.is_finite() || value < 0.0)
                || noise
                    .input_rms
                    .is_some_and(|value| !value.is_finite() || value < 0.0)
            {
                return Err("retained noise summary has an invalid band or RMS total".to_owned());
            }
            let mut contributors = HashSet::with_capacity(noise.rows.len());
            for row in &noise.rows {
                if !valid_text(&row.device)
                    || !valid_text(&row.mechanism)
                    || !row.power.is_finite()
                    || row.power < 0.0
                    || !row.share_pct.is_finite()
                    || !(0.0..=100.0).contains(&row.share_pct)
                    || !contributors.insert((row.device.as_str(), row.mechanism.as_str()))
                {
                    return Err(format!(
                        "retained noise contribution '{} / {}' is invalid or duplicated",
                        row.device, row.mechanism
                    ));
                }
            }
        }
        let mut measurement_names = HashSet::with_capacity(self.measurements.len());
        for measurement in &self.measurements {
            if !valid_text(&measurement.name)
                || !measurement_names.insert(measurement.name.as_str())
            {
                return Err(format!(
                    "retained measurement identity '{}' is invalid or duplicated",
                    measurement.name
                ));
            }
            if [
                measurement.value,
                measurement.expected,
                measurement.tolerance,
                measurement.event_axis,
            ]
            .into_iter()
            .flatten()
            .any(|value| !value.is_finite())
                || measurement.tolerance.is_some_and(|value| value < 0.0)
                || (measurement.passed
                    && (measurement.value.is_none() || measurement.error.is_some()))
            {
                return Err(format!(
                    "retained measurement '{}' has contradictory or non-finite evidence",
                    measurement.name
                ));
            }
        }

        if let Some(metadata) = &self.family_metadata {
            metadata.validate_for(self.analysis_type)?;
        }
        if let Some(payload) = &self.result_payload {
            payload.validate_for(self.analysis_type)?;
        }

        match (&self.family_metadata, &self.result_payload) {
            (None, Some(AnalysisResultPayload::Reliability { .. })) => {
                return Err("reliability payload is missing its retained lifetime axis".to_owned());
            }
            (None, Some(AnalysisResultPayload::Soa { .. })) => {
                return Err("SOA payload is missing its retained time axis".to_owned());
            }
            (
                Some(AnalysisResultFamilyMetadata::Reliability { years }),
                Some(AnalysisResultPayload::Reliability { devices }),
            ) => {
                for device in devices {
                    if device.checkpoints.len() != years.len()
                        || !device
                            .checkpoints
                            .iter()
                            .zip(years)
                            .all(|(checkpoint, years)| {
                                same_retained_float(checkpoint.years, *years)
                            })
                    {
                        return Err(format!(
                            "reliability device '{}' checkpoints do not match the retained lifetime axis",
                            device.device_id
                        ));
                    }
                }
            }
            (
                Some(AnalysisResultFamilyMetadata::Soa { time }),
                Some(AnalysisResultPayload::Soa {
                    evaluations,
                    violations,
                }),
            ) => {
                let rules = evaluations
                    .iter()
                    .map(|evaluation| {
                        (
                            (evaluation.device_id.as_str(), evaluation.parameter),
                            evaluation,
                        )
                    })
                    .collect::<std::collections::BTreeMap<_, _>>();
                let mut exact_worst_events = std::collections::BTreeSet::new();
                for violation in violations {
                    let key = (violation.device_id.as_str(), violation.parameter);
                    let evaluation = rules.get(&key).ok_or_else(|| {
                        format!(
                            "SOA event for '{}' has no matching evaluated rule",
                            violation.device_id
                        )
                    })?;
                    if !same_retained_float(violation.limit_value, evaluation.limit_value) {
                        return Err(format!(
                            "SOA event for '{}' contradicts its evaluated rule limit",
                            violation.device_id
                        ));
                    }
                    if violation.actual_value > evaluation.worst_actual_value {
                        return Err(format!(
                            "SOA event for '{}' exceeds its retained worst point",
                            violation.device_id
                        ));
                    }
                    if !contains_retained_coordinate(time, violation.time_s) {
                        return Err(format!(
                            "SOA event for '{}' does not reference an exact retained sample",
                            violation.device_id
                        ));
                    }
                    let expected_severity = match evaluation.verdict {
                        SoaRuleVerdictEvidence::Pass => None,
                        SoaRuleVerdictEvidence::Warning => {
                            Some(SoaViolationSeverityEvidence::Warning)
                        }
                        SoaRuleVerdictEvidence::Violation => {
                            Some(SoaViolationSeverityEvidence::Violation)
                        }
                        SoaRuleVerdictEvidence::Critical => {
                            Some(SoaViolationSeverityEvidence::Critical)
                        }
                    };
                    if expected_severity == Some(violation.severity)
                        && same_retained_float(
                            violation.actual_value,
                            evaluation.worst_actual_value,
                        )
                        && same_retained_float(violation.time_s, evaluation.worst_time_s)
                    {
                        exact_worst_events.insert(key);
                    }
                }
                for evaluation in evaluations {
                    let retained_sample_count = u64::try_from(time.len())
                        .map_err(|_| "SOA time axis exceeds the retained count range".to_owned())?;
                    if evaluation.sample_count != retained_sample_count {
                        return Err(format!(
                            "SOA evaluation for '{}' covers {} samples but the retained run has {}",
                            evaluation.device_id,
                            evaluation.sample_count,
                            time.len()
                        ));
                    }
                    if !contains_retained_coordinate(time, evaluation.worst_time_s) {
                        return Err(format!(
                            "SOA evaluation for '{}' does not reference an exact retained sample",
                            evaluation.device_id
                        ));
                    }
                    let key = (evaluation.device_id.as_str(), evaluation.parameter);
                    if evaluation.verdict != SoaRuleVerdictEvidence::Pass
                        && !exact_worst_events.contains(&key)
                    {
                        return Err(format!(
                            "SOA evaluation for '{}' has no exact event at its worst point",
                            evaluation.device_id
                        ));
                    }
                }
            }
            (Some(AnalysisResultFamilyMetadata::Reliability { .. }), Some(payload))
                if !matches!(payload, AnalysisResultPayload::Reliability { .. }) =>
            {
                return Err("reliability metadata has a mismatched retained payload".to_owned());
            }
            (Some(AnalysisResultFamilyMetadata::Soa { .. }), Some(payload))
                if !matches!(payload, AnalysisResultPayload::Soa { .. }) =>
            {
                return Err("SOA metadata has a mismatched retained payload".to_owned());
            }
            _ => {}
        }
        Ok(())
    }
}

#[cfg(test)]
mod retained_payload_tests {
    use super::*;

    fn floquet_certificate(problem_order: u64) -> FloquetSpectrumCertificateEvidence {
        FloquetSpectrumCertificateEvidence {
            problem_order,
            max_backward_error: 0.0,
            qualification_tolerance:
                FloquetSpectrumCertificateEvidence::canonical_qualification_tolerance(problem_order)
                    .unwrap(),
        }
    }

    fn stable_pstb_payload() -> AnalysisResultPayload {
        let multiplier = ComplexResultValue {
            real: 0.5,
            imaginary: 0.0,
        };
        let exponent = complex_value(multiplier).ln();
        AnalysisResultPayload::Pstb {
            period_s: Some(1.0),
            fundamental_frequency_hz: Some(1.0),
            stability_threshold: Some(1.0),
            probe_instance: Some("LPROBE".to_owned()),
            detect_subharmonics: Some(false),
            modes: vec![PstbFloquetModeEvidence {
                multiplier,
                exponent: ComplexResultValue {
                    real: exponent.re,
                    imaginary: exponent.im,
                },
                probe_participation: 0.25,
                is_unstable: false,
                is_trivial: false,
                subharmonic_order: None,
            }],
            floquet_evidence: FloquetSpectrumEvidence::Qualified {
                certificate: floquet_certificate(1),
            },
            orbit_kind: FloquetOrbitKindEvidence::Driven,
            trivial_multiplier_index: None,
            stability_verdict: FloquetStabilityVerdictEvidence::Stable,
            stability_classification: PstbStabilityClassificationEvidence::Stable,
            min_stability_margin_db: Some(-20.0 * 0.5_f64.log10()),
            max_multiplier_magnitude: Some(0.5),
            num_unstable: Some(0),
            subharmonics: Vec::new(),
            converged: Some(true),
            iterations: Some(0),
        }
    }

    #[test]
    fn pss_floquet_payload_requires_core_authentic_complete_evidence() {
        let valid = AnalysisResultPayload::PssFloquet {
            period_s: Some(2.0),
            fundamental_frequency_hz: Some(0.5),
            iterations: Some(4),
            residual_norm: Some(1.0e-12),
            multipliers: vec![PssFloquetMultiplierEvidence {
                multiplier: ComplexResultValue {
                    real: 0.5,
                    imaginary: 0.0,
                },
            }],
            floquet_evidence: FloquetSpectrumEvidence::Qualified {
                certificate: floquet_certificate(1),
            },
            orbit_kind: FloquetOrbitKindEvidence::Driven,
            trivial_multiplier_index: None,
            stability_verdict: FloquetStabilityVerdictEvidence::Stable,
        };
        assert!(valid.validate_for(AnalysisType::Pss).is_ok());
        assert!(valid.validate_for(AnalysisType::Pstb).is_err());

        let mut inflated = valid.clone();
        let AnalysisResultPayload::PssFloquet {
            floquet_evidence: FloquetSpectrumEvidence::Qualified { certificate },
            ..
        } = &mut inflated
        else {
            unreachable!()
        };
        certificate.qualification_tolerance *= 2.0;
        assert!(inflated.validate_for(AnalysisType::Pss).is_err());

        let legacy = AnalysisResultPayload::legacy_periodic_marker(AnalysisType::Pss).unwrap();
        assert!(legacy.validate_for(AnalysisType::Pss).is_ok());
        let mut forged_legacy = legacy;
        let AnalysisResultPayload::PssFloquet { period_s, .. } = &mut forged_legacy else {
            unreachable!()
        };
        *period_s = Some(1.0);
        assert!(forged_legacy.validate_for(AnalysisType::Pss).is_err());
    }

    #[test]
    fn pstb_payload_recomputes_mode_flags_counts_metrics_and_classification() {
        let valid = stable_pstb_payload();
        assert!(valid.validate_for(AnalysisType::Pstb).is_ok());

        let mutations: [fn(&mut AnalysisResultPayload); 3] = [
            |payload: &mut AnalysisResultPayload| {
                let AnalysisResultPayload::Pstb { num_unstable, .. } = payload else {
                    unreachable!()
                };
                *num_unstable = Some(1);
            },
            |payload: &mut AnalysisResultPayload| {
                let AnalysisResultPayload::Pstb {
                    stability_classification,
                    ..
                } = payload
                else {
                    unreachable!()
                };
                *stability_classification = PstbStabilityClassificationEvidence::UnstableReal;
            },
            |payload: &mut AnalysisResultPayload| {
                let AnalysisResultPayload::Pstb { probe_instance, .. } = payload else {
                    unreachable!()
                };
                *probe_instance = Some(" LPROBE".to_owned());
            },
        ];
        for mutate in mutations {
            let mut tampered = valid.clone();
            mutate(&mut tampered);
            assert!(tampered.validate_for(AnalysisType::Pstb).is_err());
        }
    }

    #[test]
    fn pstb_zero_dynamic_modes_and_single_autonomous_phase_are_json_safe() {
        let zero_order = AnalysisResultPayload::Pstb {
            period_s: Some(1.0),
            fundamental_frequency_hz: Some(1.0),
            stability_threshold: Some(1.0),
            probe_instance: Some("LPROBE".to_owned()),
            detect_subharmonics: Some(false),
            modes: Vec::new(),
            floquet_evidence: FloquetSpectrumEvidence::NoDynamicModes,
            orbit_kind: FloquetOrbitKindEvidence::Driven,
            trivial_multiplier_index: None,
            stability_verdict: FloquetStabilityVerdictEvidence::Stable,
            stability_classification: PstbStabilityClassificationEvidence::Stable,
            min_stability_margin_db: None,
            max_multiplier_magnitude: Some(0.0),
            num_unstable: Some(0),
            subharmonics: Vec::new(),
            converged: Some(true),
            iterations: Some(0),
        };
        assert!(zero_order.validate_for(AnalysisType::Pstb).is_ok());
        serde_json::to_string(&zero_order).expect("zero-order PSTB payload is strict-JSON safe");

        let multiplier = ComplexResultValue {
            real: 1.0 + 0.5 * rspice_core::analysis::FLOQUET_UNIT_CIRCLE_BAND,
            imaginary: 0.0,
        };
        let exponent = complex_value(multiplier).ln();
        let autonomous = AnalysisResultPayload::Pstb {
            period_s: Some(1.0),
            fundamental_frequency_hz: Some(1.0),
            stability_threshold: Some(1.0),
            probe_instance: Some("LPROBE".to_owned()),
            detect_subharmonics: Some(false),
            modes: vec![PstbFloquetModeEvidence {
                multiplier,
                exponent: ComplexResultValue {
                    real: exponent.re,
                    imaginary: exponent.im,
                },
                probe_participation: 1.0,
                is_unstable: false,
                is_trivial: true,
                subharmonic_order: None,
            }],
            floquet_evidence: FloquetSpectrumEvidence::Qualified {
                certificate: floquet_certificate(1),
            },
            orbit_kind: FloquetOrbitKindEvidence::Autonomous,
            trivial_multiplier_index: Some(0),
            stability_verdict: FloquetStabilityVerdictEvidence::Stable,
            stability_classification: PstbStabilityClassificationEvidence::Stable,
            min_stability_margin_db: None,
            max_multiplier_magnitude: Some(complex_value(multiplier).norm()),
            num_unstable: Some(0),
            subharmonics: Vec::new(),
            converged: Some(true),
            iterations: Some(0),
        };
        assert!(autonomous.validate_for(AnalysisType::Pstb).is_ok());
    }

    #[test]
    fn durable_floquet_scalars_are_central_and_never_fabricate_a_stability_bool() {
        let result = AnalysisResult::new(1, AnalysisType::Pstb, "PSTB")
            .with_result_payload(stable_pstb_payload());
        let scalar = |name: &str| {
            let candidates = result.scalar_evidence(name);
            assert_eq!(candidates.len(), 1, "missing scalar evidence {name}");
            assert!(candidates[0].passed);
            candidates[0].value.unwrap()
        };

        assert_eq!(scalar("pstb_period"), 1.0);
        assert_eq!(scalar("PSTB_FUNDAMENTAL_FREQUENCY"), 1.0);
        assert_eq!(scalar("pstb_mode_count"), 1.0);
        assert_eq!(scalar("pstb_unstable_mode_count"), 0.0);
        assert_eq!(scalar("pstb_max_multiplier_magnitude"), 0.5);
        assert_eq!(
            scalar("pstb_min_stability_margin_db"),
            -20.0 * 0.5_f64.log10()
        );
        assert_eq!(
            scalar("pstb.min_stability_margin_db"),
            -20.0 * 0.5_f64.log10(),
            "the established dotted runtime spelling remains compatible"
        );
        assert_eq!(
            result.scalar_evidence_names(),
            [
                "pstb_period",
                "pstb_fundamental_frequency",
                "pstb_mode_count",
                "pstb_unstable_mode_count",
                "pstb_max_multiplier_magnitude",
                "pstb_min_stability_margin_db",
            ]
        );
        assert!(result.scalar_evidence("pstb.is_stable").is_empty());
        assert!(
            !result
                .scalar_evidence_names()
                .iter()
                .any(|name| name == "pstb.is_stable" || name == "pstb.stability")
        );

        let legacy = AnalysisResult::new(2, AnalysisType::Pstb, "legacy").with_result_payload(
            AnalysisResultPayload::legacy_periodic_marker(AnalysisType::Pstb).unwrap(),
        );
        assert!(legacy.scalar_evidence("pstb.mode_count").is_empty());
        assert!(legacy.scalar_evidence_names().is_empty());
    }

    #[test]
    fn zero_order_pss_exposes_an_authenticated_zero_mode_count() {
        let payload = AnalysisResultPayload::PssFloquet {
            period_s: Some(2.0),
            fundamental_frequency_hz: Some(0.5),
            iterations: Some(2),
            residual_norm: Some(1.0e-12),
            multipliers: Vec::new(),
            floquet_evidence: FloquetSpectrumEvidence::NoDynamicModes,
            orbit_kind: FloquetOrbitKindEvidence::Driven,
            trivial_multiplier_index: None,
            stability_verdict: FloquetStabilityVerdictEvidence::Stable,
        };
        let result = AnalysisResult::new(1, AnalysisType::Pss, "PSS").with_result_payload(payload);

        assert_eq!(result.scalar_evidence("pss_period")[0].value, Some(2.0));
        assert_eq!(
            result.scalar_evidence("pss_fundamental_frequency")[0].value,
            Some(0.5)
        );
        assert_eq!(result.scalar_evidence("pss_mode_count")[0].value, Some(0.0));
        assert_eq!(
            result.scalar_evidence("pss.mode_count")[0].value,
            Some(0.0),
            "the established dotted runtime spelling remains compatible"
        );
        assert_eq!(
            result.scalar_evidence_names(),
            ["pss_period", "pss_fundamental_frequency", "pss_mode_count"]
        );
        assert!(result.scalar_evidence("pss.is_stable").is_empty());
    }

    #[test]
    fn retained_waveforms_require_exact_finite_aligned_unique_evidence() {
        let valid = AnalysisResult::new(1, AnalysisType::Transient, "TRAN").with_waveforms(vec![
            WaveformData::new("V(out)", vec![0.0, 1.0], vec![0.25, 0.5], "#00aaff")
                .with_unit("V")
                .with_complex_components("V(out)", vec![0.25, 0.5], vec![0.0, -0.125]),
        ]);
        assert!(valid.validate_retained_evidence().is_ok());

        let misaligned =
            AnalysisResult::new(1, AnalysisType::Transient, "TRAN").with_waveforms(vec![
                WaveformData::new("V(out)", vec![0.0, 1.0], vec![0.25], "#00aaff"),
            ]);
        assert!(
            misaligned
                .validate_retained_evidence()
                .expect_err("misaligned retained samples must fail closed")
                .contains("coordinates")
        );

        let non_finite =
            AnalysisResult::new(1, AnalysisType::Transient, "TRAN").with_waveforms(vec![
                WaveformData::new("V(out)", vec![0.0, 1.0], vec![0.25, f64::NAN], "#00aaff"),
            ]);
        assert!(
            non_finite
                .validate_retained_evidence()
                .expect_err("non-finite retained samples must fail closed")
                .contains("non-finite")
        );

        let duplicated =
            AnalysisResult::new(1, AnalysisType::Transient, "TRAN").with_waveforms(vec![
                WaveformData::new("V(out)", vec![0.0], vec![0.25], "#00aaff"),
                WaveformData::new("V(out)", vec![0.0], vec![0.5], "#ffaa00"),
            ]);
        assert!(
            duplicated
                .validate_retained_evidence()
                .expect_err("duplicate retained identities must fail closed")
                .contains("duplicated")
        );

        let bad_complex = AnalysisResult::new(1, AnalysisType::Ac, "AC").with_waveforms(vec![
            WaveformData::new("V(out)", vec![1.0, 10.0], vec![0.25, 0.5], "#00aaff")
                .with_complex_components("V(out)", vec![0.25], vec![0.0, -0.125]),
        ]);
        assert!(
            bad_complex
                .validate_retained_evidence()
                .expect_err("misaligned complex evidence must fail closed")
                .contains("complex components")
        );
    }

    #[test]
    fn retained_scalar_and_operating_point_evidence_is_finite_and_unambiguous() {
        let invalid_op = AnalysisResult::new(1, AnalysisType::DcOp, "OP").with_dc_op(DcOpResult {
            node_voltages: vec![OperatingPointValue {
                name: "V(out)".to_owned(),
                value: f64::INFINITY,
                unit: "V".to_owned(),
            }],
            ..DcOpResult::default()
        });
        assert!(
            invalid_op
                .validate_retained_evidence()
                .expect_err("non-finite OP evidence must fail closed")
                .contains("non-finite")
        );

        let invalid_measurement = AnalysisResult::new(1, AnalysisType::Transient, "TRAN")
            .with_measurements(vec![rspice_core::MeasureResult::success("gain", f64::NAN)]);
        assert!(
            invalid_measurement
                .validate_retained_evidence()
                .expect_err("non-finite measurement evidence must fail closed")
                .contains("measurement")
        );
    }

    #[test]
    fn summary_only_noise_retains_integrated_totals_without_contributor_rows() {
        let summary = NoiseSummary {
            rows: Vec::new(),
            total_rms: Some(2.5e-6),
            input_rms: Some(1.25e-6),
            band: (10.0, 1.0e6),
        };
        let result = AnalysisResult::new(1, AnalysisType::Noise, "NOISE")
            .with_noise_summary(summary.clone());

        assert_eq!(result.noise_summary, Some(summary));
    }

    fn transfer_function_payload() -> AnalysisResultPayload {
        AnalysisResultPayload::TransferFunction {
            input_source: "VIN".to_owned(),
            output_expression: "V(OUT,REF)".to_owned(),
            input_quantity: TransferFunctionQuantityEvidence::Voltage,
            output_quantity: TransferFunctionQuantityEvidence::Voltage,
            input_unit: "V".to_owned(),
            output_unit: "V".to_owned(),
            normalization: TransferFunctionNormalizationEvidence::None,
            accuracy: TransferFunctionAccuracyEvidence::Balanced,
            gain: Some(TransferFunctionScalarEvidence::Finite(0.5)),
            input_resistance: Some(TransferFunctionScalarEvidence::PositiveInfinity),
            output_resistance: Some(TransferFunctionScalarEvidence::Finite(-25.0)),
            nominal_input: None,
            nominal_output: None,
        }
    }

    #[test]
    fn pole_zero_payload_requires_matching_type_and_finite_values() {
        let payload = AnalysisResultPayload::PoleZero {
            poles: vec![ComplexResultValue {
                real: -1.0,
                imaginary: 2.0,
            }],
            zeros: Vec::new(),
            pole_evidence: PoleZeroRootSetEvidence::LegacyUnknown,
            zero_evidence: PoleZeroRootSetEvidence::LegacyUnknown,
            gain: Some(1.0),
        };
        assert!(payload.validate_for(AnalysisType::PoleZero).is_ok());
        assert!(payload.validate_for(AnalysisType::Ac).is_err());

        let unavailable_gain = AnalysisResultPayload::PoleZero {
            poles: vec![ComplexResultValue {
                real: -1.0,
                imaginary: 2.0,
            }],
            zeros: Vec::new(),
            pole_evidence: PoleZeroRootSetEvidence::LegacyUnknown,
            zero_evidence: PoleZeroRootSetEvidence::LegacyUnknown,
            gain: None,
        };
        assert!(
            unavailable_gain
                .validate_for(AnalysisType::PoleZero)
                .is_ok()
        );

        let invalid = AnalysisResultPayload::PoleZero {
            poles: vec![ComplexResultValue {
                real: f64::INFINITY,
                imaginary: 0.0,
            }],
            zeros: Vec::new(),
            pole_evidence: PoleZeroRootSetEvidence::LegacyUnknown,
            zero_evidence: PoleZeroRootSetEvidence::LegacyUnknown,
            gain: Some(1.0),
        };
        assert!(invalid.validate_for(AnalysisType::PoleZero).is_err());

        let invalid_gain = AnalysisResultPayload::PoleZero {
            poles: Vec::new(),
            zeros: Vec::new(),
            pole_evidence: PoleZeroRootSetEvidence::LegacyUnknown,
            zero_evidence: PoleZeroRootSetEvidence::LegacyUnknown,
            gain: Some(f64::INFINITY),
        };
        assert!(invalid_gain.validate_for(AnalysisType::PoleZero).is_err());

        let invalid_tolerance = AnalysisResultPayload::PoleZero {
            poles: vec![ComplexResultValue {
                real: -1.0,
                imaginary: 0.0,
            }],
            zeros: Vec::new(),
            pole_evidence: PoleZeroRootSetEvidence::Qualified {
                certificate: PoleZeroSpectrumCertificate {
                    problem_order: 1,
                    infinite_count: 0,
                    max_backward_error: 0.0,
                    qualification_tolerance: 2.0
                        * PoleZeroSpectrumCertificate::canonical_qualification_tolerance(1)
                            .unwrap(),
                },
            },
            zero_evidence: PoleZeroRootSetEvidence::NotRequested,
            gain: Some(1.0),
        };
        assert!(
            invalid_tolerance
                .validate_for(AnalysisType::PoleZero)
                .expect_err("inflated qualification tolerance is not core-authentic")
                .contains("pole evidence")
        );
    }

    #[test]
    fn pole_zero_payload_deserializes_legacy_numeric_and_missing_gain() {
        let legacy: AnalysisResultPayload =
            serde_json::from_str(r#"{"kind":"pole_zero","poles":[],"zeros":[],"gain":4.25}"#)
                .expect("legacy numeric pole-zero gain deserializes");
        assert!(matches!(
            legacy,
            AnalysisResultPayload::PoleZero {
                gain: Some(4.25),
                pole_evidence: PoleZeroRootSetEvidence::LegacyUnknown,
                zero_evidence: PoleZeroRootSetEvidence::LegacyUnknown,
                ..
            }
        ));

        let missing: AnalysisResultPayload =
            serde_json::from_str(r#"{"kind":"pole_zero","poles":[],"zeros":[]}"#)
                .expect("missing pole-zero gain deserializes as unavailable");
        assert!(matches!(
            missing,
            AnalysisResultPayload::PoleZero {
                gain: None,
                pole_evidence: PoleZeroRootSetEvidence::LegacyUnknown,
                zero_evidence: PoleZeroRootSetEvidence::LegacyUnknown,
                ..
            }
        ));
    }

    #[test]
    fn sensitivity_payload_requires_canonical_unique_rows_and_valid_basis() {
        let valid = AnalysisResultPayload::Sensitivity {
            output: "V(out)".to_owned(),
            result_mode: SensitivityResultMode::Ac {
                frequency_hz: 1_000.0,
            },
            rows: vec![
                SensitivityResultRow {
                    parameter: "length".to_owned(),
                    raw: -1.0,
                    normalized: -0.25,
                },
                SensitivityResultRow {
                    parameter: "width".to_owned(),
                    raw: 2.0,
                    normalized: 0.5,
                },
            ],
        };
        assert!(valid.validate_for(AnalysisType::Sensitivity).is_ok());

        let duplicate = AnalysisResultPayload::Sensitivity {
            output: "V(out)".to_owned(),
            result_mode: SensitivityResultMode::Dc,
            rows: vec![
                SensitivityResultRow {
                    parameter: "width".to_owned(),
                    raw: 1.0,
                    normalized: 1.0,
                },
                SensitivityResultRow {
                    parameter: "width".to_owned(),
                    raw: 2.0,
                    normalized: 2.0,
                },
            ],
        };
        assert!(duplicate.validate_for(AnalysisType::Sensitivity).is_err());

        let invalid_frequency = AnalysisResultPayload::Sensitivity {
            output: "V(out)".to_owned(),
            result_mode: SensitivityResultMode::Ac { frequency_hz: 0.0 },
            rows: Vec::new(),
        };
        assert!(
            invalid_frequency
                .validate_for(AnalysisType::Sensitivity)
                .is_err()
        );
    }

    #[test]
    fn transfer_function_payload_is_typed_non_finite_safe_and_analysis_specific() {
        let payload = transfer_function_payload();
        assert!(payload.validate_for(AnalysisType::Tf).is_ok());
        assert!(payload.validate_for(AnalysisType::Ac).is_err());
        assert!(payload.has_data());

        let encoded = serde_json::to_string(&payload).expect("TF payload serializes");
        assert!(encoded.contains("positive_infinity"));
        assert!(!encoded.contains("Infinity"));
        let decoded: AnalysisResultPayload =
            serde_json::from_str(&encoded).expect("TF payload deserializes");
        assert_eq!(decoded, payload);

        assert_eq!(
            TransferFunctionScalarEvidence::from_f64(f64::INFINITY),
            Some(TransferFunctionScalarEvidence::PositiveInfinity)
        );
        assert_eq!(
            TransferFunctionScalarEvidence::from_f64(f64::NEG_INFINITY),
            Some(TransferFunctionScalarEvidence::NegativeInfinity)
        );
        assert_eq!(TransferFunctionScalarEvidence::from_f64(f64::NAN), None);

        let scalar = AnalysisResultPayload::ScalarMeasurements {
            values: BTreeMap::from([("gain".to_owned(), 0.5)]),
        };
        assert!(scalar.validate_for(AnalysisType::Tf).is_err());
    }

    #[test]
    fn transfer_function_payload_rejects_contradictory_or_malformed_evidence() {
        let mut invalid_finite = transfer_function_payload();
        let AnalysisResultPayload::TransferFunction { gain, .. } = &mut invalid_finite else {
            unreachable!()
        };
        *gain = Some(TransferFunctionScalarEvidence::Finite(f64::INFINITY));
        assert!(
            invalid_finite
                .validate_for(AnalysisType::Tf)
                .expect_err("infinity cannot use the finite classification")
                .contains("finite classification")
        );

        let mut wrong_unit = transfer_function_payload();
        let AnalysisResultPayload::TransferFunction { input_unit, .. } = &mut wrong_unit else {
            unreachable!()
        };
        *input_unit = "A".to_owned();
        assert!(wrong_unit.validate_for(AnalysisType::Tf).is_err());

        let mut wrong_quantity = transfer_function_payload();
        let AnalysisResultPayload::TransferFunction {
            output_quantity, ..
        } = &mut wrong_quantity
        else {
            unreachable!()
        };
        *output_quantity = TransferFunctionQuantityEvidence::Current;
        assert!(wrong_quantity.validate_for(AnalysisType::Tf).is_err());

        for expression in [" V(OUT,REF)", "V (OUT,REF)", "V( OUT,REF)"] {
            let mut malformed = transfer_function_payload();
            let AnalysisResultPayload::TransferFunction {
                output_expression, ..
            } = &mut malformed
            else {
                unreachable!()
            };
            *output_expression = expression.to_owned();
            assert!(malformed.validate_for(AnalysisType::Tf).is_err());
        }

        let mut empty = transfer_function_payload();
        let AnalysisResultPayload::TransferFunction {
            gain,
            input_resistance,
            output_resistance,
            ..
        } = &mut empty
        else {
            unreachable!()
        };
        *gain = None;
        *input_resistance = None;
        *output_resistance = None;
        assert!(empty.validate_for(AnalysisType::Tf).is_err());
    }

    #[test]
    fn relative_transfer_function_gain_requires_exact_nonzero_nominals() {
        let mut relative = transfer_function_payload();
        let AnalysisResultPayload::TransferFunction {
            normalization,
            nominal_input,
            nominal_output,
            ..
        } = &mut relative
        else {
            unreachable!()
        };
        *normalization = TransferFunctionNormalizationEvidence::RelativeToNominal;
        *nominal_input = Some(1.0);
        *nominal_output = Some(0.5);
        assert!(relative.validate_for(AnalysisType::Tf).is_ok());

        let mut missing = relative.clone();
        let AnalysisResultPayload::TransferFunction { nominal_output, .. } = &mut missing else {
            unreachable!()
        };
        *nominal_output = None;
        assert!(missing.validate_for(AnalysisType::Tf).is_err());

        let mut zero = relative;
        let AnalysisResultPayload::TransferFunction { nominal_input, .. } = &mut zero else {
            unreachable!()
        };
        *nominal_input = Some(0.0);
        assert!(zero.validate_for(AnalysisType::Tf).is_err());
    }

    #[test]
    fn reliability_payload_requires_canonical_devices_and_exact_lifetime_coverage() {
        let device = ReliabilityDeviceEvidence {
            device_id: "M1".to_owned(),
            stress: ReliabilityStressEvidence {
                average_gate_stress_v: 1.2,
                average_drain_stress_v: 1.8,
                average_temperature_k: 358.15,
                duration_s: 3_600.0,
            },
            checkpoints: vec![
                ReliabilityCheckpointEvidence {
                    years: 1.0,
                    shift: ReliabilityShiftEvidence {
                        threshold_voltage_shift_v: 0.01,
                        mobility_shift: -0.001,
                        drain_source_resistance_shift: 0.0005,
                    },
                },
                ReliabilityCheckpointEvidence {
                    years: 10.0,
                    shift: ReliabilityShiftEvidence {
                        threshold_voltage_shift_v: 0.03,
                        mobility_shift: -0.004,
                        drain_source_resistance_shift: 0.0015,
                    },
                },
            ],
        };
        let valid = AnalysisResult::new(1, AnalysisType::Reliability, "Reliability")
            .with_family_metadata(AnalysisResultFamilyMetadata::Reliability {
                years: vec![1.0, 10.0],
            })
            .with_result_payload(AnalysisResultPayload::Reliability {
                devices: vec![device.clone()],
            });
        assert!(valid.validate_retained_evidence().is_ok());

        let payload_without_axis = AnalysisResult::new(1, AnalysisType::Reliability, "Reliability")
            .with_result_payload(AnalysisResultPayload::Reliability {
                devices: vec![device.clone()],
            });
        assert!(
            payload_without_axis
                .validate_retained_evidence()
                .expect_err("reliability payload requires its lifetime axis")
                .contains("missing its retained lifetime axis")
        );

        let incomplete = AnalysisResult::new(1, AnalysisType::Reliability, "Reliability")
            .with_family_metadata(AnalysisResultFamilyMetadata::Reliability {
                years: vec![1.0, 5.0, 10.0],
            })
            .with_result_payload(AnalysisResultPayload::Reliability {
                devices: vec![device],
            });
        assert!(
            incomplete
                .validate_retained_evidence()
                .expect_err("missing lifetime evidence is rejected")
                .contains("do not match")
        );
    }

    #[test]
    fn soa_payload_requires_complete_rule_coverage_consistent_events_and_axis() {
        let evaluation = SoaEvaluationEvidence {
            device_id: "M1".to_owned(),
            parameter: SoaParameterEvidence::DrainSourceVoltage,
            limit_value: 3.3,
            worst_actual_value: 3.2,
            worst_time_s: 1.0,
            sample_count: 2,
            unit: "V".to_owned(),
            description: "Maximum drain-source voltage".to_owned(),
            verdict: SoaRuleVerdictEvidence::Warning,
        };
        let event = SoaViolationEvidence {
            device_id: "M1".to_owned(),
            parameter: SoaParameterEvidence::DrainSourceVoltage,
            limit_value: 3.3,
            actual_value: 3.2,
            time_s: 1.0,
            severity: SoaViolationSeverityEvidence::Warning,
        };
        let valid = AnalysisResult::new(1, AnalysisType::Soa, "SOA")
            .with_family_metadata(AnalysisResultFamilyMetadata::Soa {
                time: vec![0.0, 1.0],
            })
            .with_result_payload(AnalysisResultPayload::Soa {
                evaluations: vec![evaluation.clone()],
                violations: vec![event.clone()],
            });
        assert!(valid.validate_retained_evidence().is_ok());

        let payload_without_axis = AnalysisResult::new(1, AnalysisType::Soa, "SOA")
            .with_result_payload(AnalysisResultPayload::Soa {
                evaluations: vec![evaluation.clone()],
                violations: vec![event.clone()],
            });
        assert!(
            payload_without_axis
                .validate_retained_evidence()
                .expect_err("SOA payload requires its time axis")
                .contains("missing its retained time axis")
        );

        let mut invalid_event = event.clone();
        invalid_event.severity = SoaViolationSeverityEvidence::Critical;
        assert!(
            AnalysisResultPayload::Soa {
                evaluations: vec![evaluation.clone()],
                violations: vec![invalid_event],
            }
            .validate_for(AnalysisType::Soa)
            .expect_err("contradictory event severity is rejected")
            .contains("severity")
        );

        let contradictory_limit = AnalysisResult::new(1, AnalysisType::Soa, "SOA")
            .with_family_metadata(AnalysisResultFamilyMetadata::Soa {
                time: vec![0.0, 1.0],
            })
            .with_result_payload(AnalysisResultPayload::Soa {
                evaluations: vec![evaluation.clone()],
                violations: vec![SoaViolationEvidence {
                    limit_value: 3.4,
                    ..event.clone()
                }],
            });
        assert!(
            contradictory_limit
                .validate_retained_evidence()
                .expect_err("event rule limit must be exact")
                .contains("contradicts its evaluated rule limit")
        );

        let missing_worst_event = AnalysisResult::new(1, AnalysisType::Soa, "SOA")
            .with_family_metadata(AnalysisResultFamilyMetadata::Soa {
                time: vec![0.0, 1.0],
            })
            .with_result_payload(AnalysisResultPayload::Soa {
                evaluations: vec![evaluation.clone()],
                violations: Vec::new(),
            });
        assert!(
            missing_worst_event
                .validate_retained_evidence()
                .expect_err("non-pass verdict requires exact worst event")
                .contains("no exact event at its worst point")
        );

        let incomplete = AnalysisResult::new(1, AnalysisType::Soa, "SOA")
            .with_family_metadata(AnalysisResultFamilyMetadata::Soa {
                time: vec![0.0, 0.5, 1.0],
            })
            .with_result_payload(AnalysisResultPayload::Soa {
                evaluations: vec![evaluation],
                violations: Vec::new(),
            });
        assert!(
            incomplete
                .validate_retained_evidence()
                .expect_err("incomplete sample coverage is rejected")
                .contains("covers 2 samples")
        );
    }

    #[test]
    fn reliability_and_soa_axes_are_canonical_engineering_coordinates() {
        for years in [Vec::new(), vec![0.0], vec![10.0, 1.0], vec![1.0, 1.0]] {
            assert!(
                AnalysisResultFamilyMetadata::Reliability { years }
                    .validate_for(AnalysisType::Reliability)
                    .is_err()
            );
        }
        for time in [Vec::new(), vec![-1.0, 0.0], vec![0.0, 0.0], vec![1.0, 0.0]] {
            assert!(
                AnalysisResultFamilyMetadata::Soa { time }
                    .validate_for(AnalysisType::Soa)
                    .is_err()
            );
        }
        assert!(
            AnalysisResultFamilyMetadata::Reliability {
                years: vec![1.0, 5.0, 10.0],
            }
            .validate_for(AnalysisType::Reliability)
            .is_ok()
        );
        assert!(
            AnalysisResultFamilyMetadata::Soa {
                time: vec![-0.0, 1.0e-9, 2.0e-9],
            }
            .validate_for(AnalysisType::Soa)
            .is_ok()
        );
    }
}
