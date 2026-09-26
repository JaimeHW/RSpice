//! What a run was actually executed against.
//!
//! Identifies the exact model sources a run consumed, by digest rather than
//! by path. Re-running a design whose libraries changed underneath is a
//! different run, and the receipt is what makes that detectable.

use std::collections::HashSet;

use crate::specification::{PreparedSpecification, PreparedSpecificationPolicy};
use rspice_app_types::hierarchy_path::InstancePath;
use rspice_app_types::product::{
    AnalysisInstanceId, ContentDigest, DerivedAnalysisIdentity, ModelSourceId, ObjectRevision,
    SimulationPlanId,
};
use rspice_design_model::cell_view::CellViewRef;

use crate::analysis_result::AnalysisResult;
use crate::analysis_tag::CanonicalAnalysisKind;
use crate::analysis_type::AnalysisType;
use crate::provenance::AnalysisResultSourceDomain;
use crate::waveform::RetainedWaveform;

/// Whether the exact model revision a run consumed had cleared its
/// qualification gate at the moment the run was prepared.
///
/// Recorded per model rather than derived later, because the qualification
/// state is editable and the run is not: a model released after a run finished
/// does not retroactively qualify that run's results, and a release withdrawn
/// afterwards does not retroactively disqualify them. What the receipt says is
/// what was true when the deck was sealed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PreparedModelQualification {
    /// An immutable release covers this exact source revision and digest.
    Released,
    /// No release covers this exact revision. It may never have been
    /// qualified, or it may have been edited since the release that covered it
    /// — the receipt does not distinguish them, because neither is sign-off.
    Unqualified,
    /// The run predates recorded qualification state. Legacy history is not
    /// evidence of qualification and is not evidence against it.
    #[default]
    Unrecorded,
}

impl PreparedModelQualification {
    /// Whether this model bars the run from carrying a sign-off.
    ///
    /// Only an outright `Unqualified` model does. An unrecorded state is a gap
    /// in the record, not a finding, and stamping historical results as
    /// unqualified would put a claim on them that was never assessed.
    #[must_use]
    pub const fn blocks_sign_off(self) -> bool {
        matches!(self, Self::Unqualified)
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
    qualification: PreparedModelQualification,
}

impl PreparedModelSourceIdentity {
    pub fn new(
        source_id: ModelSourceId,
        model_name: impl Into<String>,
        revision: ObjectRevision,
        content_digest: ContentDigest,
        qualification: PreparedModelQualification,
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
            qualification,
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

    #[must_use]
    pub const fn qualification(&self) -> PreparedModelQualification {
        self.qualification
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
}

/// Authenticated identity and graph position of one ordered prepared task.
///
/// A task that the plan authored keeps the instance identity it was authored
/// with. A task the run expanded — one point of a declared space, the spectrum
/// companion of a PSS — has no authored identity to keep and instead states
/// the [`DerivedAnalysisIdentity`] it was minted from. That record is what lets
/// a restored receipt be authenticated against the plan: its identity is
/// re-derived from an authored instance rather than looked up as one.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreparedRunTaskReceipt {
    instance_id: AnalysisInstanceId,
    derived_from: Option<DerivedAnalysisIdentity>,
    source_revision: ObjectRevision,
    dependencies: Vec<AnalysisInstanceId>,
    analysis_kind_tag: u8,
    config_digest: ContentDigest,
}

impl PreparedRunTaskReceipt {
    pub fn new(
        instance_id: AnalysisInstanceId,
        source_revision: ObjectRevision,
        dependencies: Vec<AnalysisInstanceId>,
        analysis_kind_tag: u8,
        config_digest: ContentDigest,
    ) -> Result<Self, String> {
        Self::build(
            instance_id,
            None,
            source_revision,
            dependencies,
            analysis_kind_tag,
            config_digest,
        )
    }

    /// A task whose identity the run derived rather than the plan authored.
    ///
    /// The identity is taken from the record instead of being supplied beside
    /// it, so a receipt can never state a derived identity that its own
    /// derivation does not produce.
    pub fn new_derived(
        derived_from: DerivedAnalysisIdentity,
        source_revision: ObjectRevision,
        dependencies: Vec<AnalysisInstanceId>,
        analysis_kind_tag: u8,
        config_digest: ContentDigest,
    ) -> Result<Self, String> {
        Self::build(
            derived_from.instance_id(),
            Some(derived_from),
            source_revision,
            dependencies,
            analysis_kind_tag,
            config_digest,
        )
    }

    fn build(
        instance_id: AnalysisInstanceId,
        derived_from: Option<DerivedAnalysisIdentity>,
        source_revision: ObjectRevision,
        dependencies: Vec<AnalysisInstanceId>,
        analysis_kind_tag: u8,
        config_digest: ContentDigest,
    ) -> Result<Self, String> {
        // Canonical analysis tags are a closed execution protocol. Reject an
        // unknown persisted tag instead of allowing it to masquerade as a
        // task produced by this binary. The accepted set is not written here:
        // it is exactly what `CanonicalAnalysisKind` defines, which is also
        // what dispatch stamps, so the two cannot drift apart again.
        if CanonicalAnalysisKind::from_tag(analysis_kind_tag).is_none() {
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

        if derived_from
            .as_ref()
            .is_some_and(|derived| derived.instance_id() != instance_id)
        {
            return Err(format!(
                "prepared task {instance_id} is not the identity its derivation produces"
            ));
        }

        Ok(Self {
            instance_id,
            derived_from,
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

    /// The authored instance and role path this task's identity was derived
    /// from, absent for a task the plan authored directly.
    #[must_use]
    pub const fn derived_from(&self) -> Option<&DerivedAnalysisIdentity> {
        self.derived_from.as_ref()
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

    /// The canonical kind this task's tag names.
    #[must_use]
    pub const fn canonical_kind(&self) -> CanonicalAnalysisKind {
        match CanonicalAnalysisKind::from_tag(self.analysis_kind_tag) {
            Some(kind) => kind,
            // `new` is the only constructor and refuses every tag outside the
            // protocol, so an unresolvable tag cannot reach a built receipt.
            None => unreachable!(),
        }
    }

    /// Result-family identity encoded by the canonical execution tag.
    #[must_use]
    pub const fn result_analysis_type(&self) -> AnalysisType {
        self.canonical_kind().result_analysis_type()
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
    pub fn new(
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

/// Input facts admitted by the validated prepared-run receipt constructor.
/// This input is not itself a receipt and grants no execution authority.
#[derive(Debug)]
pub struct PreparedRunReceiptInput {
    pub source_domain: AnalysisResultSourceDomain,
    pub simulation_plan_id: Option<SimulationPlanId>,
    pub project_revision: ObjectRevision,
    pub prepared_snapshot_digest: ContentDigest,
    pub source_content_digest: ContentDigest,
    pub source_check_receipt: PreparedSourceCheckReceipt,
    pub project_model_sources: Vec<PreparedModelSourceIdentity>,
    pub specifications: Vec<PreparedSpecification>,
    pub specification_policy: PreparedSpecificationPolicy,
    pub tasks: Vec<PreparedRunTaskReceipt>,
}

impl PreparedRunReceipt {
    pub fn new(input: PreparedRunReceiptInput) -> Result<Self, String> {
        let PreparedRunReceiptInput {
            source_domain,
            simulation_plan_id,
            project_revision,
            prepared_snapshot_digest,
            source_content_digest,
            source_check_receipt,
            mut project_model_sources,
            specifications,
            specification_policy,
            tasks,
        } = input;
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
    pub fn with_hierarchy_map(
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

    /// The project-owned models this run consumed that had not cleared their
    /// qualification gate when it was prepared.
    ///
    /// A non-empty answer does not make the results wrong and must never stop
    /// anyone reading them — an unqualified model is exactly what an engineer
    /// characterizing a new device is supposed to be simulating with. It makes
    /// them unfit to be *cited* as sign-off, so every surface that presents
    /// them as evidence says so, and says which model.
    #[must_use]
    pub fn unqualified_model_sources(&self) -> Vec<&PreparedModelSourceIdentity> {
        self.project_model_sources
            .iter()
            .filter(|identity| identity.qualification().blocks_sign_off())
            .collect()
    }

    /// The preview-engine analyses this run executed.
    ///
    /// The second thing that disqualifies a run from sign-off, and it is a
    /// property of the *engine* rather than of the design: a preview solver
    /// ships connected and produces real numbers, but has not cleared the
    /// evidence bar a production kind has. The tag on each authenticated task
    /// is what survives into a sealed receipt, so it is what this reads.
    #[must_use]
    pub fn preview_engine_kinds(&self) -> Vec<CanonicalAnalysisKind> {
        let mut kinds = Vec::new();
        for task in &self.tasks {
            let kind = task.canonical_kind();
            if kind.availability().blocks_sign_off() && !kinds.contains(&kind) {
                kinds.push(kind);
            }
        }
        kinds
    }

    /// Why this run may not be cited as sign-off evidence, naming the objects.
    ///
    /// The one predicate, and it answers with its reason rather than with a
    /// bare verdict. Two things disqualify a run and both are stamped on the
    /// receipt: a project model that had not cleared its qualification gate
    /// when the run was prepared, and an analysis whose engine is still
    /// preview. Verify's sign-off tile, the Results manifest, the Results specs
    /// table and the requirements page all read this rather than re-deriving
    /// half of it, because a surface that checks only the models calls a
    /// preview run eligible while one that checks only the kind calls a
    /// qualified one ineligible — and a stamp reading "NOT SIGN-OFF" with
    /// nothing after it is what a surface says when it knows the answer and not
    /// the cause.
    ///
    /// There was an `is_sign_off_eligible` beside this returning
    /// `self.sign_off_blocker().is_none()`. Every surface that reached for it
    /// wanted the reason too, so each either asked twice or asked this and
    /// discarded half the answer.
    #[must_use]
    pub fn sign_off_blocker(&self) -> Option<String> {
        let mut reasons = Vec::new();
        let unqualified = self.unqualified_model_sources();
        if !unqualified.is_empty() {
            let named = unqualified
                .iter()
                .take(3)
                .map(|identity| identity.model_name())
                .collect::<Vec<_>>()
                .join(", ");
            reasons.push(if unqualified.len() > 3 {
                format!("{named} and {} more unqualified", unqualified.len() - 3)
            } else {
                format!("{named} unqualified at run time")
            });
        }
        let preview = self.preview_engine_kinds();
        if !preview.is_empty() {
            let named = preview
                .iter()
                .map(|kind| kind.result_analysis_type().display_name())
                .collect::<Vec<_>>()
                .join(", ");
            reasons.push(format!("{named} on a preview engine"));
        }
        (!reasons.is_empty()).then(|| reasons.join(" \u{00b7} "))
    }

    /// This run's sign-off standing, in the words every surface states it in.
    ///
    /// [`Self::sign_off_blocker`] answers `None` for a run that qualifies, and
    /// each surface was left to invent a sentence for that `None`. They
    /// invented opposite ones: Verify's tile stamped `Eligible` while the
    /// Results manifest, for the same receipt, printed `unavailable · no
    /// retained sign-off qualification`. Two surfaces of one workbench
    /// disagreeing about whether a dataset may be cited as evidence is a worse
    /// fault than either sentence on its own.
    #[must_use]
    pub fn sign_off_standing(&self) -> SignOffStanding {
        self.sign_off_blocker()
            .map_or(SignOffStanding::Eligible, SignOffStanding::Blocked)
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

    pub fn validate_result_prefix<'a, W: AsRef<RetainedWaveform> + 'a>(
        &self,
        analyses: impl ExactSizeIterator<Item = &'a AnalysisResult<W>>,
    ) -> Result<(), String> {
        if analyses.len() > self.tasks.len() {
            return Err(format!(
                "prepared run contains {} results for only {} authenticated tasks",
                analyses.len(),
                self.tasks.len()
            ));
        }

        for (index, (analysis, task)) in analyses.zip(&self.tasks).enumerate() {
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

/// Whether a retained run may be cited as sign-off evidence, and why not.
///
/// Both cases carry their words here, beside each other, so a surface cannot
/// author a third spelling of either. Verify's tile leads with
/// [`Self::verdict`] and explains with [`Self::cause`]; the Results manifest's
/// qualification cell takes [`Self::qualification`], which is the same two
/// facts in that ledger's own grammar.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SignOffStanding {
    /// Nothing on the receipt disqualifies the run.
    Eligible,
    /// Why it may not be cited, naming the objects — see
    /// [`PreparedRunReceipt::sign_off_blocker`].
    Blocked(String),
}

impl SignOffStanding {
    /// The verdict a status line leads with.
    #[must_use]
    pub const fn verdict(&self) -> &'static str {
        match self {
            Self::Eligible => "Eligible",
            Self::Blocked(_) => "Not sign-off",
        }
    }

    /// The sentence under that verdict: what disqualifies the run, or what
    /// being eligible means.
    #[must_use]
    pub fn cause(&self) -> String {
        match self {
            Self::Eligible => "Every project model this run consumed was released, and every \
                               analysis it ran is production"
                .to_owned(),
            Self::Blocked(blocker) => blocker.clone(),
        }
    }

    /// The same standing as a qualification cell states it.
    #[must_use]
    pub fn qualification(&self) -> String {
        match self {
            Self::Eligible => {
                "eligible \u{00b7} every model released \u{00b7} every analysis production"
                    .to_owned()
            }
            Self::Blocked(blocker) => format!("blocked \u{00b7} {blocker} \u{00b7} non-sign-off"),
        }
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
mod canonical_tag_tests;
