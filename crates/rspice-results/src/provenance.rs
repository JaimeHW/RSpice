//! Exact retained identity of a prepared result and its attributed PVT point.

use std::collections::HashSet;

use rspice_app_types::product::{AnalysisInstanceId, ContentDigest, ObjectRevision};

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
/// record. `AnalysisResult::provenance == None` is retained for imported data
/// and result history from formats that predate prepared-task identities; callers must never infer an identity from analysis kind or
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
