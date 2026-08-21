//! What a persisted run says about the authority it executed under.
//!
//! A result payload says what came out; these records say what it came out
//! of — the provenance mode, the consumed prepared snapshot, and the deck's
//! occurrence map. They are the half of the results document that a later
//! reader authenticates against, so every field here is admitted only for the
//! schema era that sealed it and none is ever reconstructed for a run that
//! predates it.

use super::*;

use crate::state::{HierarchyMapRow, PreparedModelQualification};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProjectRunProvenanceMode {
    /// Missing only while reading schemas older than v4. Current-schema
    /// validation rejects this state.
    #[default]
    Unspecified,
    /// Results written before prepared analysis-instance provenance existed.
    LegacyUnattributed,
    /// Prepared-task identity was stored before its source domain was
    /// persisted. No plan/manual origin is inferred during migration.
    LegacyPreparedUnclassified,
    /// Every result is bound to one authenticated prepared task.
    PreparedTaskBound,
}

pub(super) fn set_legacy_unclassified_source_domains(run: &mut ProjectSimulationRun) {
    for provenance in run
        .analyses
        .iter_mut()
        .filter_map(|analysis| analysis.provenance.as_mut())
    {
        provenance.source_domain =
            PersistedField::Value(AnalysisResultSourceDomain::LegacyUnclassified);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "digest", rename_all = "snake_case")]
pub enum ProjectPreparedSourceCheckReceipt {
    SchematicDrc(ContentDigest),
    ManualSourceCheck(ContentDigest),
}

impl From<PreparedSourceCheckReceipt> for ProjectPreparedSourceCheckReceipt {
    fn from(receipt: PreparedSourceCheckReceipt) -> Self {
        match receipt {
            PreparedSourceCheckReceipt::SchematicDrc(digest) => Self::SchematicDrc(digest),
            PreparedSourceCheckReceipt::ManualSourceCheck(digest) => {
                Self::ManualSourceCheck(digest)
            }
        }
    }
}

impl From<ProjectPreparedSourceCheckReceipt> for PreparedSourceCheckReceipt {
    fn from(receipt: ProjectPreparedSourceCheckReceipt) -> Self {
        match receipt {
            ProjectPreparedSourceCheckReceipt::SchematicDrc(digest) => Self::SchematicDrc(digest),
            ProjectPreparedSourceCheckReceipt::ManualSourceCheck(digest) => {
                Self::ManualSourceCheck(digest)
            }
        }
    }
}

/// The authored plan instance a derived task's identity was minted from, and
/// the ordered roles that produced it.
///
/// Persisted rather than recomputed because the material a point identity is
/// minted from — which point of which declared space, at which condition — is
/// a fact about the run that happened, not about the plan as it stands now.
/// Re-expanding today's declaration to authenticate a historical receipt would
/// refuse every run whose declaration has since been edited.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProjectDerivedAnalysisIdentity {
    /// The plan instance the derivation roots at.
    pub authored_instance_id: AnalysisInstanceId,
    /// Derivation steps, outermost last.
    pub roles: Vec<String>,
}

impl From<&DerivedAnalysisIdentity> for ProjectDerivedAnalysisIdentity {
    fn from(derived: &DerivedAnalysisIdentity) -> Self {
        Self {
            authored_instance_id: derived.authored(),
            roles: derived.roles().to_vec(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProjectPreparedRunTaskReceipt {
    pub source_instance_id: AnalysisInstanceId,
    /// Present exactly when the run expanded this task rather than the plan
    /// authoring it. Absent for every receipt written before derived task
    /// identities could be persisted at all — those runs had none, because a
    /// project holding one could not be saved.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub derived_from: Option<ProjectDerivedAnalysisIdentity>,
    pub source_revision: ObjectRevision,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dependency_ids: Vec<AnalysisInstanceId>,
    pub analysis_kind_tag: u8,
    pub config_digest: ContentDigest,
}

impl ProjectPreparedRunTaskReceipt {
    pub(in crate::io::project_io) fn into_receipt(self) -> Result<PreparedRunTaskReceipt, String> {
        let Some(derived) = self.derived_from else {
            return PreparedRunTaskReceipt::new(
                self.source_instance_id,
                self.source_revision,
                self.dependency_ids,
                self.analysis_kind_tag,
                self.config_digest,
            );
        };
        let derived =
            DerivedAnalysisIdentity::from_roles(derived.authored_instance_id, derived.roles)?;
        if derived.instance_id() != self.source_instance_id {
            return Err(format!(
                "prepared task {} does not re-derive from analysis {}",
                self.source_instance_id,
                derived.authored()
            ));
        }
        PreparedRunTaskReceipt::new_derived(
            derived,
            self.source_revision,
            self.dependency_ids,
            self.analysis_kind_tag,
            self.config_digest,
        )
    }
}

impl From<&PreparedRunTaskReceipt> for ProjectPreparedRunTaskReceipt {
    fn from(receipt: &PreparedRunTaskReceipt) -> Self {
        Self {
            source_instance_id: receipt.instance_id(),
            derived_from: receipt
                .derived_from()
                .map(ProjectDerivedAnalysisIdentity::from),
            source_revision: receipt.source_revision(),
            dependency_ids: receipt.dependencies().to_vec(),
            analysis_kind_tag: receipt.analysis_kind_tag(),
            config_digest: receipt.config_digest(),
        }
    }
}

/// How a persisted receipt spells the qualification gate a model had cleared.
///
/// Its own type rather than the state enum's serde derive, so the persisted
/// spelling is stated here where the file format lives and cannot be renamed by
/// an unrelated refactor of the state vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProjectPreparedModelQualification {
    Released,
    Unqualified,
    /// What every receipt written before the gate was recorded restores as.
    #[default]
    Unrecorded,
}

impl From<PreparedModelQualification> for ProjectPreparedModelQualification {
    fn from(value: PreparedModelQualification) -> Self {
        match value {
            PreparedModelQualification::Released => Self::Released,
            PreparedModelQualification::Unqualified => Self::Unqualified,
            PreparedModelQualification::Unrecorded => Self::Unrecorded,
        }
    }
}

impl From<ProjectPreparedModelQualification> for PreparedModelQualification {
    fn from(value: ProjectPreparedModelQualification) -> Self {
        match value {
            ProjectPreparedModelQualification::Released => Self::Released,
            ProjectPreparedModelQualification::Unqualified => Self::Unqualified,
            ProjectPreparedModelQualification::Unrecorded => Self::Unrecorded,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProjectPreparedModelSourceIdentity {
    pub source_id: ModelSourceId,
    pub model_name: String,
    pub revision: ObjectRevision,
    pub content_digest: ContentDigest,
    /// Defaulted so a project written before runs recorded the gate restores
    /// as `Unrecorded` — a gap in the record, never a claim that the model was
    /// qualified and never a claim that it was not.
    #[serde(default)]
    pub qualification: ProjectPreparedModelQualification,
}

impl ProjectPreparedModelSourceIdentity {
    fn into_identity(self) -> Result<PreparedModelSourceIdentity, String> {
        PreparedModelSourceIdentity::new(
            self.source_id,
            self.model_name,
            self.revision,
            self.content_digest,
            self.qualification.into(),
        )
    }
}

impl From<&PreparedModelSourceIdentity> for ProjectPreparedModelSourceIdentity {
    fn from(identity: &PreparedModelSourceIdentity) -> Self {
        Self {
            source_id: identity.source_id(),
            model_name: identity.model_name().to_owned(),
            revision: identity.revision(),
            content_digest: identity.content_digest(),
            qualification: identity.qualification().into(),
        }
    }
}

/// One occurrence of the executed design, as the deck named it.
///
/// The occurrence and the master are plain text because a receipt is a
/// historical record of a run that already happened: the file keeps the
/// rendering, and the reader parses it back through today's path type. That is
/// the same contract the persisted configuration sets use, and it is what lets
/// a project written years ago still say what it ran.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProjectHierarchyMapRow {
    /// Canonical rendering of the instantiation, `/X1/X2`.
    pub occurrence: String,
    /// `.subckt` name the deck emitted for it.
    pub master: String,
    /// Uppercased engine spelling of the occurrence scope, `X1.X2`. Absent
    /// when the occurrence has no engine spelling at all.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub engine_prefix: String,
    /// Cell view the master was generated from.
    pub master_reference: CellViewRef,
}

impl ProjectHierarchyMapRow {
    fn into_row(self) -> Result<HierarchyMapRow, String> {
        let Self {
            occurrence,
            master,
            engine_prefix,
            master_reference,
        } = self;
        HierarchyMapRow::new(&occurrence, master, engine_prefix, master_reference)
    }
}

impl From<&HierarchyMapRow> for ProjectHierarchyMapRow {
    fn from(row: &HierarchyMapRow) -> Self {
        Self {
            occurrence: row.occurrence().to_owned(),
            master: row.master().to_owned(),
            engine_prefix: row.engine_prefix().to_owned(),
            master_reference: row.master_reference().clone(),
        }
    }
}

fn spec_entries_bitwise_equal(left: &SpecEntry, right: &SpecEntry) -> bool {
    left.measurement == right.measurement
        && left.expression == right.expression
        && left.min.map(f64::to_bits) == right.min.map(f64::to_bits)
        && left.max.map(f64::to_bits) == right.max.map(f64::to_bits)
        && left.unit == right.unit
        && left.scope == right.scope
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProjectPreparedRunReceipt {
    pub source_domain: AnalysisResultSourceDomain,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub simulation_plan_id: Option<SimulationPlanId>,
    pub project_revision: ObjectRevision,
    pub prepared_snapshot_digest: ContentDigest,
    pub source_content_digest: ContentDigest,
    pub source_check_receipt: ProjectPreparedSourceCheckReceipt,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub project_model_sources: Vec<ProjectPreparedModelSourceIdentity>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub specifications: Vec<SpecEntry>,
    /// Governed records corresponding one-for-one with `specifications`.
    /// Empty is retained for receipts written before schema v14.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub specification_definitions: Vec<SpecificationDefinition>,
    /// Presence-aware because schema v14 requires even the default policy to
    /// be sealed explicitly; `null` must not masquerade as legacy absence.
    #[serde(default, skip_serializing_if = "PersistedField::is_missing")]
    pub specification_policy: PersistedField<SpecificationPolicy>,
    pub tasks: Vec<ProjectPreparedRunTaskReceipt>,
    /// The deck's occurrence map, sealed by schema v15. Empty both for a manual
    /// deck, which has no hierarchy, and for every receipt written earlier;
    /// migration never invents rows for a run that executed without one.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub hierarchy_map: Vec<ProjectHierarchyMapRow>,
}

impl ProjectPreparedRunReceipt {
    pub(in crate::io::project_io) fn into_receipt(self) -> Result<PreparedRunReceipt, String> {
        let project_model_sources = self
            .project_model_sources
            .into_iter()
            .map(ProjectPreparedModelSourceIdentity::into_identity)
            .collect::<Result<Vec<_>, _>>()?;
        let tasks = self
            .tasks
            .into_iter()
            .map(ProjectPreparedRunTaskReceipt::into_receipt)
            .collect::<Result<Vec<_>, _>>()?;
        let projected_specifications = self.specifications;
        let specifications = if self.specification_definitions.is_empty() {
            projected_specifications
                .into_iter()
                .map(PreparedSpecification::new)
                .collect::<Result<Vec<_>, _>>()?
        } else {
            if self.specification_definitions.len() != projected_specifications.len() {
                return Err(
                    "prepared-run governed specification count does not match its scalar projection"
                        .to_owned(),
                );
            }
            self.specification_definitions
                .into_iter()
                .zip(projected_specifications)
                .map(|(definition, projection)| {
                    if !spec_entries_bitwise_equal(&definition.projected_entry(), &projection) {
                        return Err(format!(
                            "governed specification '{}' does not match its sealed scalar projection",
                            definition.requirement_key
                        ));
                    }
                    PreparedSpecification::from_definition(definition)
                })
                .collect::<Result<Vec<_>, _>>()?
        };
        let hierarchy_map = self
            .hierarchy_map
            .into_iter()
            .map(ProjectHierarchyMapRow::into_row)
            .collect::<Result<Vec<_>, _>>()?;
        let specification_policy = match self.specification_policy {
            PersistedField::Value(policy) => PreparedSpecificationPolicy::new(policy)?,
            PersistedField::Missing => {
                return Err("prepared-run receipt is missing specification_policy".to_owned());
            }
            PersistedField::Null => {
                return Err("prepared-run receipt specification_policy cannot be null".to_owned());
            }
        };
        PreparedRunReceipt::new_with_project_model_sources_specifications_and_policy(
            self.source_domain,
            self.simulation_plan_id,
            self.project_revision,
            self.prepared_snapshot_digest,
            self.source_content_digest,
            self.source_check_receipt.into(),
            project_model_sources,
            specifications,
            specification_policy,
            tasks,
        )
        .and_then(|receipt| receipt.with_hierarchy_map(hierarchy_map))
    }

    pub(super) fn validate(&self) -> Result<(), String> {
        self.clone().into_receipt().map(|_| ())
    }
}

impl From<&PreparedRunReceipt> for ProjectPreparedRunReceipt {
    fn from(receipt: &PreparedRunReceipt) -> Self {
        Self {
            source_domain: receipt.source_domain(),
            simulation_plan_id: receipt.simulation_plan_id(),
            project_revision: receipt.project_revision(),
            prepared_snapshot_digest: receipt.prepared_snapshot_digest(),
            source_content_digest: receipt.source_content_digest(),
            source_check_receipt: receipt.source_check_receipt().into(),
            project_model_sources: receipt
                .project_model_sources()
                .iter()
                .map(ProjectPreparedModelSourceIdentity::from)
                .collect(),
            specifications: receipt
                .specifications()
                .iter()
                .map(|specification| specification.entry().clone())
                .collect(),
            specification_definitions: receipt
                .specifications()
                .iter()
                .filter_map(|specification| specification.definition().cloned())
                .collect(),
            specification_policy: PersistedField::Value(
                receipt.specification_policy().policy().clone(),
            ),
            tasks: receipt
                .tasks()
                .iter()
                .map(ProjectPreparedRunTaskReceipt::from)
                .collect(),
            hierarchy_map: receipt
                .hierarchy_map()
                .iter()
                .map(ProjectHierarchyMapRow::from)
                .collect(),
        }
    }
}

/// Schema v14 sealed governed specification records and an explicit plan-wide
/// policy onto the receipt. An earlier receipt carries neither, and the default
/// policy it is given is the one those runs already evaluated under; a receipt
/// that carries either field early is a rewritten file, not an old one.
pub(super) fn migrate_legacy_specification_receipts(
    results: &mut ProjectSimulationResults,
    source_schema: u32,
) -> Result<(), String> {
    if source_schema >= GOVERNED_SPECIFICATION_RESULTS_SCHEMA_VERSION {
        return Ok(());
    }
    for (run_index, run) in results.runs.iter_mut().enumerate() {
        let PersistedField::Value(receipt) = &mut run.prepared_receipt else {
            continue;
        };
        if !receipt.specification_definitions.is_empty()
            || receipt.specification_policy.is_present()
        {
            return Err(format!(
                "schema-v{source_schema} runs[{run_index}].prepared_receipt contains governed specification fields introduced by schema v14"
            ));
        }
        receipt.specification_policy = PersistedField::Value(SpecificationPolicy::default());
    }
    Ok(())
}

/// Schema v15 sealed the deck's occurrence map onto the receipt.
///
/// Nothing is added here, and that is the whole rule: a run recorded earlier
/// executed without a map, so its map stays empty and its reader falls back to
/// raw engine names, exactly as that run always did. Reconstructing rows from
/// today's design would attribute a deck to a run that never emitted it. A file
/// that claims an older schema while carrying a map is therefore refused rather
/// than trusted.
/// A derived task identity could not be persisted before schema v15 either.
///
/// Not because the field is new to the format — it is additive within v15 —
/// but because a project holding a run with derived task identities could not
/// be written at all: the plan-closure check refused every one of them. So a
/// file claiming an older schema while carrying a derivation record is a
/// rewritten file, and is refused rather than trusted.
pub(super) fn reject_derived_task_identities_before_schema_v15(
    results: &ProjectSimulationResults,
    source_schema: u32,
) -> Result<(), String> {
    if source_schema >= PROJECT_SIMULATION_RESULTS_SCHEMA_VERSION {
        return Ok(());
    }
    for (run_index, run) in results.runs.iter().enumerate() {
        let PersistedField::Value(receipt) = &run.prepared_receipt else {
            continue;
        };
        if let Some(task_index) = receipt
            .tasks
            .iter()
            .position(|task| task.derived_from.is_some())
        {
            return Err(format!(
                "schema-v{source_schema} runs[{run_index}].prepared_receipt.tasks[{task_index}] carries a derived task identity that schema could not persist"
            ));
        }
    }
    Ok(())
}

pub(super) fn reject_hierarchy_maps_before_schema_v15(
    results: &ProjectSimulationResults,
    source_schema: u32,
) -> Result<(), String> {
    if source_schema >= PROJECT_SIMULATION_RESULTS_SCHEMA_VERSION {
        return Ok(());
    }
    for (run_index, run) in results.runs.iter().enumerate() {
        let PersistedField::Value(receipt) = &run.prepared_receipt else {
            continue;
        };
        if !receipt.hierarchy_map.is_empty() {
            return Err(format!(
                "schema-v{source_schema} runs[{run_index}].prepared_receipt contains a hierarchy map introduced by schema v15"
            ));
        }
    }
    Ok(())
}

#[cfg(test)]
mod qualification_gate_tests {
    use super::*;

    /// A receipt written before runs recorded the gate must still load, and
    /// must load as unrecorded.
    ///
    /// Unrecorded is not "qualified": a project from before the gate existed
    /// says nothing about whether its models were released, and answering
    /// either way would invent a finding. It is also not "unqualified", which
    /// would stamp every historical result in every existing project as unfit
    /// for sign-off on the strength of a schema change.
    #[test]
    fn a_model_source_persisted_before_the_gate_restores_as_unrecorded() {
        let legacy = serde_json::json!({
            "source_id": ModelSourceId::new(),
            "model_name": "nch_legacy",
            "revision": ObjectRevision::INITIAL,
            "content_digest": ContentDigest::from_bytes([0x2a; 32]),
        });

        let decoded: ProjectPreparedModelSourceIdentity =
            serde_json::from_value(legacy).expect("a pre-gate model source still loads");

        assert_eq!(
            decoded.qualification,
            ProjectPreparedModelQualification::Unrecorded
        );

        let identity = decoded.into_identity().expect("valid restored identity");
        assert_eq!(
            identity.qualification(),
            PreparedModelQualification::Unrecorded
        );
        assert!(
            !identity.qualification().blocks_sign_off(),
            "a gap in the record is not a finding against the run"
        );
    }

    #[test]
    fn every_gate_state_survives_the_persisted_spelling() {
        for state in [
            PreparedModelQualification::Released,
            PreparedModelQualification::Unqualified,
            PreparedModelQualification::Unrecorded,
        ] {
            let persisted = ProjectPreparedModelQualification::from(state);
            let encoded = serde_json::to_string(&persisted).expect("gate state serializes");
            let decoded: ProjectPreparedModelQualification =
                serde_json::from_str(&encoded).expect("gate state round-trips");
            assert_eq!(PreparedModelQualification::from(decoded), state);
        }
    }
}
