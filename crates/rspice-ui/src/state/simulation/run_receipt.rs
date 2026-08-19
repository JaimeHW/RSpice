//! What a run was actually executed against.
//!
//! Identifies the exact model sources a run consumed, by digest rather than
//! by path. Re-running a design whose libraries changed underneath is a
//! different run, and the receipt is what makes that detectable.

use std::collections::HashSet;

use crate::product::{
    AnalysisInstanceId, ContentDigest, ModelSourceId, ObjectRevision, SimulationPlanId,
};
use crate::state::{
    CellViewRef, InstancePath, SpecEntry, SpecificationDefinition, SpecificationPolicy,
};

use super::{AnalysisResult, AnalysisResultSourceDomain, AnalysisType};

/// Immutable specification definition sealed into one prepared run.
///
/// The wrapper keeps the exact authored row while enforcing the invariants
/// that make bitwise equality sound. Historical result readers can therefore
/// use the run's own requirement instead of the currently edited plan.
#[derive(Debug, Clone)]
pub struct PreparedSpecification {
    entry: SpecEntry,
    definition: Option<SpecificationDefinition>,
}

impl PartialEq for PreparedSpecification {
    fn eq(&self, other: &Self) -> bool {
        self.entry.measurement == other.entry.measurement
            && self.entry.expression == other.entry.expression
            && self.entry.min.map(f64::to_bits) == other.entry.min.map(f64::to_bits)
            && self.entry.max.map(f64::to_bits) == other.entry.max.map(f64::to_bits)
            && self.entry.unit == other.entry.unit
            && self.entry.scope == other.entry.scope
            && match (&self.definition, &other.definition) {
                (None, None) => true,
                (Some(left), Some(right)) => left.bitwise_eq(right),
                (None, Some(_)) | (Some(_), None) => false,
            }
    }
}

impl Eq for PreparedSpecification {}

impl PreparedSpecification {
    pub(crate) fn new(entry: SpecEntry) -> Result<Self, String> {
        entry.validate()?;
        Ok(Self {
            entry,
            definition: None,
        })
    }

    pub(crate) fn from_definition(definition: SpecificationDefinition) -> Result<Self, String> {
        definition.validate()?;
        let entry = definition.projected_entry();
        entry.validate()?;
        Ok(Self {
            entry,
            definition: Some(definition),
        })
    }

    #[must_use]
    pub fn entry(&self) -> &SpecEntry {
        &self.entry
    }

    #[must_use]
    pub fn definition(&self) -> Option<&SpecificationDefinition> {
        self.definition.as_ref()
    }
}

/// Validated plan-wide requirement-evaluation policy sealed into a run.
///
/// The authored policy contains a floating-point yield threshold, so this
/// wrapper supplies bitwise equality only after validation has rejected
/// non-finite values. That keeps durable receipt equality reflexive and exact.
#[derive(Debug, Clone, Default)]
pub struct PreparedSpecificationPolicy {
    policy: SpecificationPolicy,
}

impl PartialEq for PreparedSpecificationPolicy {
    fn eq(&self, other: &Self) -> bool {
        self.policy.bitwise_eq(&other.policy)
    }
}

impl Eq for PreparedSpecificationPolicy {}

impl PreparedSpecificationPolicy {
    pub(crate) fn new(policy: SpecificationPolicy) -> Result<Self, String> {
        policy.validate()?;
        Ok(Self { policy })
    }

    #[must_use]
    pub fn policy(&self) -> &SpecificationPolicy {
        &self.policy
    }
}

/// Exact project-owned model definition admitted to one prepared run.
///
/// Historical receipts legitimately carry no identities and therefore cannot
/// authorize model-correlation evidence for an exact model revision.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreparedModelSourceIdentity {
    source_id: ModelSourceId,
    model_name: String,
    revision: ObjectRevision,
    content_digest: ContentDigest,
}

impl PreparedModelSourceIdentity {
    pub(crate) fn new(
        source_id: ModelSourceId,
        model_name: impl Into<String>,
        revision: ObjectRevision,
        content_digest: ContentDigest,
    ) -> Result<Self, String> {
        let model_name = model_name.into();
        if model_name.trim().is_empty() {
            return Err("prepared model-source identity requires a model name".to_owned());
        }
        Ok(Self {
            source_id,
            model_name,
            revision,
            content_digest,
        })
    }

    #[must_use]
    pub const fn source_id(&self) -> ModelSourceId {
        self.source_id
    }

    #[must_use]
    pub fn model_name(&self) -> &str {
        &self.model_name
    }

    #[must_use]
    pub const fn revision(&self) -> ObjectRevision {
        self.revision
    }

    #[must_use]
    pub const fn content_digest(&self) -> ContentDigest {
        self.content_digest
    }
}

/// Typed source-check authority captured by immutable run preparation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PreparedSourceCheckReceipt {
    SchematicDrc(ContentDigest),
    ManualSourceCheck(ContentDigest),
}

impl PreparedSourceCheckReceipt {
    #[must_use]
    pub const fn digest(self) -> ContentDigest {
        match self {
            Self::SchematicDrc(digest) | Self::ManualSourceCheck(digest) => digest,
        }
    }

    #[must_use]
    pub const fn is_schematic_drc(self) -> bool {
        matches!(self, Self::SchematicDrc(_))
    }

    #[must_use]
    #[cfg(test)]
    pub const fn is_manual_source_check(self) -> bool {
        matches!(self, Self::ManualSourceCheck(_))
    }
}

/// Authenticated identity and graph position of one ordered prepared task.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreparedRunTaskReceipt {
    instance_id: AnalysisInstanceId,
    source_revision: ObjectRevision,
    dependencies: Vec<AnalysisInstanceId>,
    analysis_kind_tag: u8,
    config_digest: ContentDigest,
}

impl PreparedRunTaskReceipt {
    pub(crate) fn new(
        instance_id: AnalysisInstanceId,
        source_revision: ObjectRevision,
        dependencies: Vec<AnalysisInstanceId>,
        analysis_kind_tag: u8,
        config_digest: ContentDigest,
    ) -> Result<Self, String> {
        // Canonical analysis tags are a closed execution protocol. Reject an
        // unknown persisted tag instead of allowing it to masquerade as a
        // task produced by this binary.
        if analysis_kind_tag > 25 {
            return Err(format!(
                "prepared task {instance_id} has unknown analysis kind tag {analysis_kind_tag}"
            ));
        }

        let mut unique_dependencies = HashSet::with_capacity(dependencies.len());
        for dependency in &dependencies {
            if *dependency == instance_id {
                return Err(format!(
                    "prepared task {instance_id} cannot depend on itself"
                ));
            }
            if !unique_dependencies.insert(*dependency) {
                return Err(format!(
                    "prepared task {instance_id} repeats dependency {dependency}"
                ));
            }
        }

        Ok(Self {
            instance_id,
            source_revision,
            dependencies,
            analysis_kind_tag,
            config_digest,
        })
    }

    #[must_use]
    pub const fn instance_id(&self) -> AnalysisInstanceId {
        self.instance_id
    }

    #[must_use]
    pub const fn source_revision(&self) -> ObjectRevision {
        self.source_revision
    }

    #[must_use]
    pub fn dependencies(&self) -> &[AnalysisInstanceId] {
        &self.dependencies
    }

    #[must_use]
    pub const fn analysis_kind_tag(&self) -> u8 {
        self.analysis_kind_tag
    }

    #[must_use]
    pub const fn config_digest(&self) -> ContentDigest {
        self.config_digest
    }

    /// Result-family identity encoded by the canonical execution tag.
    #[must_use]
    pub const fn result_analysis_type(&self) -> AnalysisType {
        match self.analysis_kind_tag {
            0 => AnalysisType::DcOp,
            1 => AnalysisType::DcSweep,
            2 | 3 => AnalysisType::Ac,
            4 => AnalysisType::Disto,
            5 => AnalysisType::Transient,
            6 => AnalysisType::Noise,
            7 => AnalysisType::Pss,
            8 => AnalysisType::HarmonicBalance,
            9 => AnalysisType::Tf,
            10 => AnalysisType::Sensitivity,
            11 => AnalysisType::PoleZero,
            12 => AnalysisType::Pac,
            13 => AnalysisType::Pnoise,
            14 => AnalysisType::Pxf,
            15 => AnalysisType::Pstb,
            16 => AnalysisType::Stb,
            17 => AnalysisType::MonteCarlo,
            18 => AnalysisType::Parametric,
            19 => AnalysisType::Corner,
            20 => AnalysisType::Reliability,
            21 => AnalysisType::Optimization,
            22 => AnalysisType::Soa,
            23 => AnalysisType::SParameter,
            24 => AnalysisType::Envelope,
            25 => AnalysisType::Fourier,
            _ => unreachable!(),
        }
    }
}

/// One occurrence of the executed design, as the emitted deck named it.
///
/// The occurrence is retained in its rendered spelling rather than as a parsed
/// path because a receipt is a historical record: it must stay readable by a
/// build whose path type has moved on. The spelling is canonicalized and
/// checked once, here, so every stored row parses back for the reader.
///
/// `engine_prefix` is the uppercased engine scope of the same occurrence, and
/// that correspondence is the condition under which an engine name can be
/// reversed back to a design occurrence. It is verified rather than trusted.
/// It is empty only when the occurrence has no engine spelling at all, which
/// is a row the reverse map skips rather than a row it may guess at.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HierarchyMapRow {
    occurrence: String,
    master: String,
    engine_prefix: String,
    master_reference: CellViewRef,
}

impl HierarchyMapRow {
    pub(crate) fn new(
        occurrence: &str,
        master: impl Into<String>,
        engine_prefix: impl Into<String>,
        master_reference: CellViewRef,
    ) -> Result<Self, String> {
        let path = InstancePath::parse(occurrence).map_err(|error| {
            format!("hierarchy map row '{occurrence}' is not an instance path: {error}")
        })?;
        if path.is_root() {
            return Err("hierarchy map row cannot name the design root".to_owned());
        }
        let master = master.into();
        if master.trim().is_empty() {
            return Err(format!("hierarchy map row {path} has no master name"));
        }
        let engine_prefix = engine_prefix.into();
        if !engine_prefix.is_empty() {
            let derived = path
                .to_engine_name()
                .map(|scope| scope.to_ascii_uppercase())
                .map_err(|error| {
                    format!("hierarchy map row {path} has no engine scope: {error}")
                })?;
            if engine_prefix != derived {
                return Err(format!(
                    "hierarchy map row {path} carries engine prefix '{engine_prefix}' rather than '{derived}'"
                ));
            }
        }
        Ok(Self {
            occurrence: path.to_string(),
            master,
            engine_prefix,
            master_reference,
        })
    }

    #[must_use]
    pub fn occurrence(&self) -> &str {
        &self.occurrence
    }

    #[must_use]
    pub fn master(&self) -> &str {
        &self.master
    }

    #[must_use]
    pub fn engine_prefix(&self) -> &str {
        &self.engine_prefix
    }

    #[must_use]
    pub fn master_reference(&self) -> &CellViewRef {
        &self.master_reference
    }
}

/// Durable authority for a run created from one consumed prepared snapshot.
///
/// The complete task graph remains present even when execution is aborted or
/// fails partway through the queue. Result history is consequently an ordered
/// prefix of `tasks`, never the source from which run classification is
/// inferred.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreparedRunReceipt {
    source_domain: AnalysisResultSourceDomain,
    simulation_plan_id: Option<SimulationPlanId>,
    project_revision: ObjectRevision,
    prepared_snapshot_digest: ContentDigest,
    source_content_digest: ContentDigest,
    source_check_receipt: PreparedSourceCheckReceipt,
    project_model_sources: Vec<PreparedModelSourceIdentity>,
    specifications: Vec<PreparedSpecification>,
    specification_policy: PreparedSpecificationPolicy,
    tasks: Vec<PreparedRunTaskReceipt>,
    hierarchy_map: Vec<HierarchyMapRow>,
}

impl PreparedRunReceipt {
    #[cfg(test)]
    pub(crate) fn new(
        source_domain: AnalysisResultSourceDomain,
        simulation_plan_id: Option<SimulationPlanId>,
        project_revision: ObjectRevision,
        prepared_snapshot_digest: ContentDigest,
        source_content_digest: ContentDigest,
        source_check_receipt: PreparedSourceCheckReceipt,
        tasks: Vec<PreparedRunTaskReceipt>,
    ) -> Result<Self, String> {
        Self::new_with_project_model_sources(
            source_domain,
            simulation_plan_id,
            project_revision,
            prepared_snapshot_digest,
            source_content_digest,
            source_check_receipt,
            Vec::new(),
            tasks,
        )
    }

    #[cfg(test)]
    pub(crate) fn new_with_project_model_sources(
        source_domain: AnalysisResultSourceDomain,
        simulation_plan_id: Option<SimulationPlanId>,
        project_revision: ObjectRevision,
        prepared_snapshot_digest: ContentDigest,
        source_content_digest: ContentDigest,
        source_check_receipt: PreparedSourceCheckReceipt,
        project_model_sources: Vec<PreparedModelSourceIdentity>,
        tasks: Vec<PreparedRunTaskReceipt>,
    ) -> Result<Self, String> {
        Self::new_with_project_model_sources_and_specifications(
            source_domain,
            simulation_plan_id,
            project_revision,
            prepared_snapshot_digest,
            source_content_digest,
            source_check_receipt,
            project_model_sources,
            Vec::new(),
            tasks,
        )
    }

    #[cfg(test)]
    pub(crate) fn new_with_project_model_sources_and_specifications(
        source_domain: AnalysisResultSourceDomain,
        simulation_plan_id: Option<SimulationPlanId>,
        project_revision: ObjectRevision,
        prepared_snapshot_digest: ContentDigest,
        source_content_digest: ContentDigest,
        source_check_receipt: PreparedSourceCheckReceipt,
        project_model_sources: Vec<PreparedModelSourceIdentity>,
        specifications: Vec<PreparedSpecification>,
        tasks: Vec<PreparedRunTaskReceipt>,
    ) -> Result<Self, String> {
        Self::new_with_project_model_sources_specifications_and_policy(
            source_domain,
            simulation_plan_id,
            project_revision,
            prepared_snapshot_digest,
            source_content_digest,
            source_check_receipt,
            project_model_sources,
            specifications,
            PreparedSpecificationPolicy::default(),
            tasks,
        )
    }

    pub(crate) fn new_with_project_model_sources_specifications_and_policy(
        source_domain: AnalysisResultSourceDomain,
        simulation_plan_id: Option<SimulationPlanId>,
        project_revision: ObjectRevision,
        prepared_snapshot_digest: ContentDigest,
        source_content_digest: ContentDigest,
        source_check_receipt: PreparedSourceCheckReceipt,
        mut project_model_sources: Vec<PreparedModelSourceIdentity>,
        specifications: Vec<PreparedSpecification>,
        specification_policy: PreparedSpecificationPolicy,
        tasks: Vec<PreparedRunTaskReceipt>,
    ) -> Result<Self, String> {
        match (source_domain, simulation_plan_id, source_check_receipt) {
            (
                AnalysisResultSourceDomain::SimulationPlan,
                Some(_),
                PreparedSourceCheckReceipt::SchematicDrc(_),
            )
            | (
                AnalysisResultSourceDomain::ManualDeck,
                None,
                PreparedSourceCheckReceipt::ManualSourceCheck(_),
            ) => {}
            (AnalysisResultSourceDomain::LegacyUnclassified, _, _) => {
                return Err(
                    "current prepared-run receipt cannot use a legacy-unclassified source domain"
                        .to_owned(),
                );
            }
            _ => {
                return Err(
                    "prepared-run source domain, simulation plan, and source-check receipt disagree"
                        .to_owned(),
                );
            }
        }
        if tasks.is_empty() {
            return Err("prepared-run receipt must contain at least one task".to_owned());
        }
        let source_revision = tasks[0].source_revision;
        if tasks
            .iter()
            .any(|task| task.source_revision != source_revision)
        {
            return Err(
                "prepared-run receipt cannot mix task source revisions in one frozen run"
                    .to_owned(),
            );
        }
        project_model_sources.sort_by(|left, right| {
            left.source_id
                .as_uuid()
                .cmp(&right.source_id.as_uuid())
                .then_with(|| {
                    left.model_name
                        .to_ascii_lowercase()
                        .cmp(&right.model_name.to_ascii_lowercase())
                })
                .then_with(|| left.revision.cmp(&right.revision))
                .then_with(|| left.content_digest.cmp(&right.content_digest))
        });
        if project_model_sources
            .windows(2)
            .any(|pair| pair[0] == pair[1])
        {
            return Err(
                "prepared-run receipt repeats an exact project model-source identity".to_owned(),
            );
        }
        let mut semantic_model_keys = HashSet::with_capacity(project_model_sources.len());
        for model in &project_model_sources {
            let key = (
                model.source_id,
                model.model_name.to_ascii_lowercase(),
                model.revision,
            );
            if !semantic_model_keys.insert(key) {
                return Err(format!(
                    "prepared-run receipt carries conflicting content for project model '{}'",
                    model.model_name
                ));
            }
        }

        let mut prior_tasks = HashSet::with_capacity(tasks.len());
        for task in &tasks {
            if prior_tasks.contains(&task.instance_id) {
                return Err(format!(
                    "prepared-run receipt repeats task instance {}",
                    task.instance_id
                ));
            }
            for dependency in &task.dependencies {
                if !prior_tasks.contains(dependency) {
                    return Err(format!(
                        "prepared task {} dependency {} must appear earlier in receipt order",
                        task.instance_id, dependency
                    ));
                }
            }
            prior_tasks.insert(task.instance_id);
        }

        let mut specification_names = HashSet::with_capacity(specifications.len());
        let governed_specification_count = specifications
            .iter()
            .filter(|specification| specification.definition().is_some())
            .count();
        if governed_specification_count != 0 && governed_specification_count != specifications.len()
        {
            return Err(
                "prepared-run receipt cannot mix governed and legacy specification rows".to_owned(),
            );
        }
        for specification in &specifications {
            specification.entry().validate()?;
            if !specification_names.insert(specification.entry().measurement.to_ascii_lowercase()) {
                return Err(format!(
                    "prepared-run receipt repeats specification measurement '{}'",
                    specification.entry().measurement
                ));
            }
        }

        Ok(Self {
            source_domain,
            simulation_plan_id,
            project_revision,
            prepared_snapshot_digest,
            source_content_digest,
            source_check_receipt,
            project_model_sources,
            specifications,
            specification_policy,
            tasks,
            hierarchy_map: Vec::new(),
        })
    }

    /// Seals the deck's occurrence map onto an already-validated receipt.
    ///
    /// Separate from construction because an absent map is a first-class
    /// state, not a receipt to repair: a manual deck has no hierarchy, and a
    /// run restored from a project file written before the map existed carries
    /// none. Neither is ever reconstructed — the reader falls back to raw
    /// engine names, which is what those runs always did.
    pub(crate) fn with_hierarchy_map(
        mut self,
        hierarchy_map: Vec<HierarchyMapRow>,
    ) -> Result<Self, String> {
        let mut occurrences = HashSet::with_capacity(hierarchy_map.len());
        for row in &hierarchy_map {
            let occurrence = InstancePath::parse(&row.occurrence).map_err(|error| {
                format!(
                    "prepared-run receipt hierarchy map row '{}' is not an instance path: {error}",
                    row.occurrence
                )
            })?;
            if !occurrences.insert(occurrence.fold_key()) {
                return Err(format!(
                    "prepared-run receipt repeats hierarchy map occurrence {occurrence}"
                ));
            }
        }
        self.hierarchy_map = hierarchy_map;
        Ok(self)
    }

    #[must_use]
    pub const fn source_domain(&self) -> AnalysisResultSourceDomain {
        self.source_domain
    }

    #[must_use]
    pub const fn simulation_plan_id(&self) -> Option<SimulationPlanId> {
        self.simulation_plan_id
    }

    #[must_use]
    pub const fn project_revision(&self) -> ObjectRevision {
        self.project_revision
    }

    #[must_use]
    pub const fn prepared_snapshot_digest(&self) -> ContentDigest {
        self.prepared_snapshot_digest
    }

    #[must_use]
    pub const fn source_content_digest(&self) -> ContentDigest {
        self.source_content_digest
    }

    #[must_use]
    pub const fn source_check_receipt(&self) -> PreparedSourceCheckReceipt {
        self.source_check_receipt
    }

    #[must_use]
    pub fn project_model_sources(&self) -> &[PreparedModelSourceIdentity] {
        &self.project_model_sources
    }

    /// Exact specification rows that were in force when this run was sealed.
    #[must_use]
    pub fn specifications(&self) -> &[PreparedSpecification] {
        &self.specifications
    }

    /// Exact plan-wide requirement policy in force for this run.
    #[must_use]
    pub fn specification_policy(&self) -> &PreparedSpecificationPolicy {
        &self.specification_policy
    }

    #[must_use]
    pub fn tasks(&self) -> &[PreparedRunTaskReceipt] {
        &self.tasks
    }

    /// The occurrence map of the deck this run executed, empty for a manual
    /// deck and for every run recorded before the map was sealed.
    #[must_use]
    pub fn hierarchy_map(&self) -> &[HierarchyMapRow] {
        &self.hierarchy_map
    }

    pub(crate) fn validate_result_prefix(&self, analyses: &[AnalysisResult]) -> Result<(), String> {
        if analyses.len() > self.tasks.len() {
            return Err(format!(
                "prepared run contains {} results for only {} authenticated tasks",
                analyses.len(),
                self.tasks.len()
            ));
        }

        for (index, (analysis, task)) in analyses.iter().zip(&self.tasks).enumerate() {
            let provenance = analysis.provenance.as_ref().ok_or_else(|| {
                format!(
                    "prepared run result {index} for task {} has no analysis provenance",
                    task.instance_id
                )
            })?;
            if provenance.source_domain() != self.source_domain
                || provenance.source_instance_id() != task.instance_id
                || provenance.source_revision() != task.source_revision
                || provenance.prepared_snapshot_digest() != self.prepared_snapshot_digest
                || provenance.dependency_ids() != task.dependencies
                || analysis.analysis_type != task.result_analysis_type()
            {
                return Err(format!(
                    "prepared run result {index} does not match authenticated task {}",
                    task.instance_id
                ));
            }
        }
        Ok(())
    }
}

/// Authoritative provenance classification retained by a simulation run.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SimulationRunProvenance {
    /// Result history predates prepared analysis-instance identity entirely.
    LegacyUnattributed,
    /// Historical results carry prepared IDs but predate source-domain and
    /// complete run-receipt persistence.
    LegacyPreparedUnclassified,
    /// Current run authenticated by an exact consumed prepared snapshot.
    Prepared(Box<PreparedRunReceipt>),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{AnalysisResultProvenance, SimulationRun};

    fn digest(byte: u8) -> ContentDigest {
        ContentDigest::from_bytes([byte; 32])
    }

    fn task(
        id: AnalysisInstanceId,
        revision: ObjectRevision,
        dependencies: Vec<AnalysisInstanceId>,
        tag: u8,
        config_byte: u8,
    ) -> PreparedRunTaskReceipt {
        PreparedRunTaskReceipt::new(id, revision, dependencies, tag, digest(config_byte))
            .expect("valid task receipt")
    }

    fn plan_receipt(tasks: Vec<PreparedRunTaskReceipt>) -> PreparedRunReceipt {
        PreparedRunReceipt::new(
            AnalysisResultSourceDomain::SimulationPlan,
            Some(SimulationPlanId::new()),
            ObjectRevision::INITIAL,
            digest(0x31),
            digest(0x32),
            PreparedSourceCheckReceipt::SchematicDrc(digest(0x33)),
            tasks,
        )
        .expect("valid plan receipt")
    }

    #[test]
    fn prepared_receipt_retains_typed_project_model_identity() {
        let source_id = ModelSourceId::new();
        let identity = PreparedModelSourceIdentity::new(
            source_id,
            "precision_nmos",
            ObjectRevision::INITIAL,
            digest(0x44),
        )
        .expect("valid project model identity");
        let receipt = PreparedRunReceipt::new_with_project_model_sources(
            AnalysisResultSourceDomain::SimulationPlan,
            Some(SimulationPlanId::new()),
            ObjectRevision::INITIAL,
            digest(0x31),
            digest(0x32),
            PreparedSourceCheckReceipt::SchematicDrc(digest(0x33)),
            vec![identity],
            vec![task(
                AnalysisInstanceId::new(),
                ObjectRevision::INITIAL,
                Vec::new(),
                0,
                0x45,
            )],
        )
        .expect("model-bound prepared receipt");

        let retained = &receipt.project_model_sources()[0];
        assert_eq!(retained.source_id(), source_id);
        assert_eq!(retained.model_name(), "precision_nmos");
        assert_eq!(retained.revision(), ObjectRevision::INITIAL);
        assert_eq!(retained.content_digest(), digest(0x44));
    }

    fn result(
        id: AnalysisInstanceId,
        revision: ObjectRevision,
        snapshot: ContentDigest,
        dependencies: Vec<AnalysisInstanceId>,
        analysis_type: AnalysisType,
    ) -> AnalysisResult {
        AnalysisResult::new(1, analysis_type, "result").with_provenance(
            AnalysisResultProvenance::new_with_source_domain(
                AnalysisResultSourceDomain::SimulationPlan,
                id,
                revision,
                snapshot,
                dependencies,
            )
            .expect("valid result provenance"),
        )
    }

    fn amp_reference() -> CellViewRef {
        CellViewRef::new("user", "amp", "schematic")
    }

    #[test]
    fn a_hierarchy_map_row_must_agree_with_the_engine_scope_it_claims() {
        let row = HierarchyMapRow::new("/X1/X2", "amp_1", "X1.X2", amp_reference())
            .expect("a derived engine prefix is accepted");
        assert_eq!(row.occurrence(), "/X1/X2");
        assert_eq!(row.master(), "amp_1");
        assert_eq!(row.engine_prefix(), "X1.X2");
        assert_eq!(row.master_reference(), &amp_reference());

        assert!(
            HierarchyMapRow::new("/X1/X2", "amp_1", "X1.X9", amp_reference())
                .unwrap_err()
                .contains("rather than 'X1.X2'")
        );
        assert!(
            HierarchyMapRow::new("/X1", "", "X1", amp_reference())
                .unwrap_err()
                .contains("no master name")
        );
        assert!(
            HierarchyMapRow::new("/", "amp_1", "", amp_reference())
                .unwrap_err()
                .contains("design root")
        );
        assert!(
            HierarchyMapRow::new("/X1/", "amp_1", "", amp_reference())
                .unwrap_err()
                .contains("not an instance path")
        );
    }

    #[test]
    fn a_sealed_hierarchy_map_names_each_occurrence_once() {
        let receipt = plan_receipt(vec![task(
            AnalysisInstanceId::new(),
            ObjectRevision::INITIAL,
            Vec::new(),
            0,
            0x51,
        )]);
        assert!(
            receipt.hierarchy_map().is_empty(),
            "a receipt seals no map until one is carried onto it"
        );

        let sealed = receipt
            .clone()
            .with_hierarchy_map(vec![
                HierarchyMapRow::new("/X1", "amp_1", "X1", amp_reference())
                    .expect("outer occurrence"),
                HierarchyMapRow::new("/X1/X2", "amp_2", "X1.X2", amp_reference())
                    .expect("inner occurrence"),
            ])
            .expect("distinct occurrences seal");
        assert_eq!(
            sealed
                .hierarchy_map()
                .iter()
                .map(HierarchyMapRow::engine_prefix)
                .collect::<Vec<_>>(),
            vec!["X1", "X1.X2"]
        );

        // Two rows for one instance would make the reverse map ambiguous, and
        // the case fold is what decides that they are one instance.
        let error = receipt
            .with_hierarchy_map(vec![
                HierarchyMapRow::new("/X1", "amp_1", "X1", amp_reference()).expect("first row"),
                HierarchyMapRow::new("/x1", "amp_2", "X1", amp_reference()).expect("second row"),
            ])
            .expect_err("one occurrence cannot name two masters");
        assert!(
            error.contains("repeats hierarchy map occurrence"),
            "{error}"
        );
    }

    #[test]
    fn fresh_run_is_unsealed_and_cannot_validate_as_legacy() {
        let run = SimulationRun::new(1);

        assert!(run.provenance().is_none());
        assert!(run.validate_provenance().unwrap_err().contains("unsealed"));
    }

    #[test]
    fn migration_must_restore_an_explicit_legacy_classification() {
        let mut unattributed = SimulationRun::new(2);
        unattributed.add_analysis(AnalysisResult::new(1, AnalysisType::Ac, "legacy"));
        unattributed
            .restore_provenance(SimulationRunProvenance::LegacyUnattributed)
            .expect("unattributed legacy history restores explicitly");
        assert!(matches!(
            unattributed.provenance(),
            Some(SimulationRunProvenance::LegacyUnattributed)
        ));

        let mut unclassified = SimulationRun::new(3);
        unclassified.add_analysis(
            AnalysisResult::new(1, AnalysisType::Ac, "legacy prepared").with_provenance(
                AnalysisResultProvenance::new_with_source_domain(
                    AnalysisResultSourceDomain::LegacyUnclassified,
                    AnalysisInstanceId::new(),
                    ObjectRevision::INITIAL,
                    digest(0x21),
                    Vec::new(),
                )
                .expect("legacy prepared provenance"),
            ),
        );
        unclassified
            .restore_provenance(SimulationRunProvenance::LegacyPreparedUnclassified)
            .expect("unclassified prepared history restores explicitly");
        assert!(matches!(
            unclassified.provenance(),
            Some(SimulationRunProvenance::LegacyPreparedUnclassified)
        ));
    }

    #[test]
    fn receipt_rejects_mixed_task_revisions() {
        let first = AnalysisInstanceId::new();
        let second = AnalysisInstanceId::new();
        let tasks = vec![
            task(first, ObjectRevision::INITIAL, Vec::new(), 0, 1),
            task(
                second,
                ObjectRevision::new(2).expect("revision two"),
                Vec::new(),
                5,
                2,
            ),
        ];

        let error = PreparedRunReceipt::new(
            AnalysisResultSourceDomain::SimulationPlan,
            Some(SimulationPlanId::new()),
            ObjectRevision::INITIAL,
            digest(3),
            digest(4),
            PreparedSourceCheckReceipt::SchematicDrc(digest(5)),
            tasks,
        )
        .expect_err("mixed task revisions must fail");

        assert!(error.contains("mix task source revisions"));
    }

    #[test]
    fn receipt_rejects_source_domain_plan_and_check_mismatches() {
        let make_tasks = || {
            vec![task(
                AnalysisInstanceId::new(),
                ObjectRevision::INITIAL,
                Vec::new(),
                0,
                9,
            )]
        };
        let plan_id = SimulationPlanId::new();
        let build = |domain, plan, check, tasks| {
            PreparedRunReceipt::new(
                domain,
                plan,
                ObjectRevision::INITIAL,
                digest(6),
                digest(7),
                check,
                tasks,
            )
        };

        assert!(
            build(
                AnalysisResultSourceDomain::SimulationPlan,
                None,
                PreparedSourceCheckReceipt::SchematicDrc(digest(8)),
                make_tasks(),
            )
            .is_err()
        );
        assert!(
            build(
                AnalysisResultSourceDomain::SimulationPlan,
                Some(plan_id),
                PreparedSourceCheckReceipt::ManualSourceCheck(digest(8)),
                make_tasks(),
            )
            .is_err()
        );
        assert!(
            build(
                AnalysisResultSourceDomain::ManualDeck,
                Some(plan_id),
                PreparedSourceCheckReceipt::ManualSourceCheck(digest(8)),
                make_tasks(),
            )
            .is_err()
        );
        assert!(
            build(
                AnalysisResultSourceDomain::ManualDeck,
                None,
                PreparedSourceCheckReceipt::SchematicDrc(digest(8)),
                make_tasks(),
            )
            .is_err()
        );
        assert!(
            build(
                AnalysisResultSourceDomain::LegacyUnclassified,
                None,
                PreparedSourceCheckReceipt::ManualSourceCheck(digest(8)),
                make_tasks(),
            )
            .is_err()
        );
    }

    #[test]
    fn prepared_result_history_accepts_exact_abort_prefix_and_rejects_identity_rewrites() {
        let first = AnalysisInstanceId::new();
        let second = AnalysisInstanceId::new();
        let revision = ObjectRevision::INITIAL;
        let receipt = plan_receipt(vec![
            task(first, revision, Vec::new(), 2, 0x41),
            task(second, revision, vec![first], 5, 0x42),
        ]);
        let snapshot = receipt.prepared_snapshot_digest();
        let first_result = result(first, revision, snapshot, Vec::new(), AnalysisType::Ac);
        let second_result = result(
            second,
            revision,
            snapshot,
            vec![first],
            AnalysisType::Transient,
        );

        receipt
            .validate_result_prefix(std::slice::from_ref(&first_result))
            .expect("authenticated abort prefix is valid");
        receipt
            .validate_result_prefix(&[first_result.clone(), second_result.clone()])
            .expect("complete authenticated result sequence is valid");

        assert!(
            receipt
                .validate_result_prefix(&[second_result.clone(), first_result.clone()])
                .is_err(),
            "result reorder must fail"
        );

        let mut changed_type = first_result.clone();
        changed_type.analysis_type = AnalysisType::Transient;
        assert!(receipt.validate_result_prefix(&[changed_type]).is_err());

        let changed_domain = AnalysisResult::new(1, AnalysisType::Ac, "result").with_provenance(
            AnalysisResultProvenance::new_with_source_domain(
                AnalysisResultSourceDomain::ManualDeck,
                first,
                revision,
                snapshot,
                Vec::new(),
            )
            .expect("changed domain provenance"),
        );
        assert!(receipt.validate_result_prefix(&[changed_domain]).is_err());

        let changed_id = result(
            AnalysisInstanceId::new(),
            revision,
            snapshot,
            Vec::new(),
            AnalysisType::Ac,
        );
        assert!(receipt.validate_result_prefix(&[changed_id]).is_err());

        let changed_revision = result(
            first,
            ObjectRevision::new(2).expect("revision two"),
            snapshot,
            Vec::new(),
            AnalysisType::Ac,
        );
        assert!(receipt.validate_result_prefix(&[changed_revision]).is_err());

        let changed_snapshot = result(first, revision, digest(0x99), Vec::new(), AnalysisType::Ac);
        assert!(receipt.validate_result_prefix(&[changed_snapshot]).is_err());

        let changed_dependencies = result(
            second,
            revision,
            snapshot,
            Vec::new(),
            AnalysisType::Transient,
        );
        assert!(
            receipt
                .validate_result_prefix(&[first_result, changed_dependencies])
                .is_err()
        );
    }
}
