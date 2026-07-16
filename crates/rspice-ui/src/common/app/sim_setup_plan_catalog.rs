//! Persisted simulation-plan catalog and clone/switch transactions.
//!
//! `SimSetupState` retains the active plan in its historical fields so all
//! existing execution consumers remain source-compatible. Inactive plans are
//! stored as complete, typed records. Catalog mutations are staged on a clone
//! of the setup and commit only after every name, identity, plan graph, and
//! runner-ownership invariant has been checked.

use std::collections::HashSet;
use std::fmt;

use serde::{Deserialize, Deserializer, Serialize};

use crate::product::{AnalysisInstanceId, ObjectRevision, SimulationPlanId};
use crate::simulation::dialog::SimulationOptions;
use crate::simulation::plan::{AnalysisPlanError, SimulationPlan};

use super::{ReferencePvtPoint, SimSetupState};

const DEFAULT_PLAN_NAME: &str = "Lab characterization";
const MAX_PLAN_NAME_CHARACTERS: usize = 96;

/// Validated, user-visible simulation-plan name.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
#[serde(transparent)]
pub struct SimulationPlanName(String);

impl SimulationPlanName {
    /// Validate and normalize a user-entered plan name.
    pub fn new(value: impl Into<String>) -> Result<Self, SimulationPlanCatalogError> {
        let value = value.into();
        let normalized = value.trim();
        if normalized.is_empty() {
            return Err(SimulationPlanCatalogError::InvalidName(
                "Plan name cannot be empty.".to_owned(),
            ));
        }
        if normalized.chars().count() > MAX_PLAN_NAME_CHARACTERS {
            return Err(SimulationPlanCatalogError::InvalidName(format!(
                "Plan name cannot exceed {MAX_PLAN_NAME_CHARACTERS} characters."
            )));
        }
        if normalized.chars().any(char::is_control) {
            return Err(SimulationPlanCatalogError::InvalidName(
                "Plan name cannot contain control characters.".to_owned(),
            ));
        }
        Ok(Self(normalized.to_owned()))
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    fn uniqueness_key(&self) -> String {
        self.0.to_lowercase()
    }
}

impl Default for SimulationPlanName {
    fn default() -> Self {
        Self(DEFAULT_PLAN_NAME.to_owned())
    }
}

impl fmt::Display for SimulationPlanName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl<'de> Deserialize<'de> for SimulationPlanName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        Self::new(value).map_err(serde::de::Error::custom)
    }
}

/// Durable source lineage for one working simulation plan.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SimulationPlanLineage {
    #[serde(default)]
    source_plan_id: Option<SimulationPlanId>,
    #[serde(default)]
    source_revision: Option<ObjectRevision>,
    #[serde(default)]
    clone_contents: Option<SimulationPlanCloneOptions>,
}

impl SimulationPlanLineage {
    #[must_use]
    pub const fn root() -> Self {
        Self {
            source_plan_id: None,
            source_revision: None,
            clone_contents: None,
        }
    }

    #[must_use]
    pub const fn cloned_from(
        source_plan_id: SimulationPlanId,
        source_revision: ObjectRevision,
    ) -> Self {
        Self::cloned_from_with_contents(
            source_plan_id,
            source_revision,
            SimulationPlanCloneOptions::ALL_PLAN_CONTENTS,
        )
    }

    #[must_use]
    pub const fn cloned_from_with_contents(
        source_plan_id: SimulationPlanId,
        source_revision: ObjectRevision,
        clone_contents: SimulationPlanCloneOptions,
    ) -> Self {
        Self {
            source_plan_id: Some(source_plan_id),
            source_revision: Some(source_revision),
            clone_contents: Some(clone_contents),
        }
    }

    #[must_use]
    pub const fn source_plan_id(self) -> Option<SimulationPlanId> {
        self.source_plan_id
    }

    #[must_use]
    pub const fn source_revision(self) -> Option<ObjectRevision> {
        self.source_revision
    }

    #[must_use]
    pub const fn clone_contents(self) -> Option<SimulationPlanCloneOptions> {
        self.clone_contents
    }

    fn is_valid(self) -> bool {
        self.source_plan_id.is_some() == self.source_revision.is_some()
            && self.source_plan_id.is_some() == self.clone_contents.is_some()
    }
}

/// Complete persisted state for an inactive named plan.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StoredSimulationPlan {
    name: SimulationPlanName,
    lineage: SimulationPlanLineage,
    analysis_plan: SimulationPlan,
    reference_pvt: ReferencePvtPoint,
    options: SimulationOptions,
}

impl StoredSimulationPlan {
    #[must_use]
    pub fn name(&self) -> &SimulationPlanName {
        &self.name
    }

    #[must_use]
    pub const fn id(&self) -> SimulationPlanId {
        self.analysis_plan.id()
    }

    #[must_use]
    pub const fn revision(&self) -> ObjectRevision {
        self.analysis_plan.revision()
    }

    #[must_use]
    pub const fn lineage(&self) -> SimulationPlanLineage {
        self.lineage
    }

    #[must_use]
    pub const fn analysis_plan(&self) -> &SimulationPlan {
        &self.analysis_plan
    }

    #[must_use]
    pub const fn reference_pvt(&self) -> ReferencePvtPoint {
        self.reference_pvt
    }

    #[must_use]
    pub const fn options(&self) -> &SimulationOptions {
        &self.options
    }
}

/// Content domains copied into a newly cloned plan.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SimulationPlanCloneOptions {
    pub copy_analyses: bool,
    pub copy_advanced_options: bool,
    pub copy_variables_outputs_and_specifications: bool,
    pub copy_pvt_and_model_bindings: bool,
    pub copy_regression_baseline_ownership: bool,
}

impl Default for SimulationPlanCloneOptions {
    fn default() -> Self {
        Self::ALL_PLAN_CONTENTS
    }
}

impl SimulationPlanCloneOptions {
    /// Exact checked state shown by the mockup's initial clone workflow.
    pub const ALL_PLAN_CONTENTS: Self = Self {
        copy_analyses: true,
        copy_advanced_options: true,
        copy_variables_outputs_and_specifications: true,
        copy_pvt_and_model_bindings: true,
        copy_regression_baseline_ownership: false,
    };
}

/// Committed clone transaction used by adjacent project-owned domains to
/// copy their payloads under the exact same source and destination IDs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimulationPlanCloneOutcome {
    pub source_plan_id: SimulationPlanId,
    pub source_revision: ObjectRevision,
    pub cloned_plan_id: SimulationPlanId,
    pub contents: SimulationPlanCloneOptions,
    /// Exact source-to-clone analysis mapping for plan-owned payloads whose
    /// scope targets a selected analysis instance.
    pub analysis_identity_map: Vec<(AnalysisInstanceId, AnalysisInstanceId)>,
}

impl SimulationPlanCloneOutcome {
    #[must_use]
    pub fn remap_analysis_id(&self, source: AnalysisInstanceId) -> Option<AnalysisInstanceId> {
        self.analysis_identity_map
            .iter()
            .find_map(|(candidate, cloned)| (*candidate == source).then_some(*cloned))
    }
}

/// Atomic named-plan catalog operation failure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SimulationPlanCatalogError {
    InvalidName(String),
    DuplicateName(String),
    ActivePlanUnavailable,
    PlanNotFound(SimulationPlanId),
    PlanExecuting(SimulationPlanId),
    InvalidLineage(SimulationPlanId),
    DuplicatePlanIdentity(SimulationPlanId),
    InvalidPlan(AnalysisPlanError),
}

impl fmt::Display for SimulationPlanCatalogError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidName(reason) => formatter.write_str(reason),
            Self::DuplicateName(name) => {
                write!(
                    formatter,
                    "A simulation plan named '{name}' already exists."
                )
            }
            Self::ActivePlanUnavailable => formatter
                .write_str("The active simulation plan has not been migrated to stable identity."),
            Self::PlanNotFound(id) => write!(formatter, "Simulation plan {id} does not exist."),
            Self::PlanExecuting(id) => write!(
                formatter,
                "Simulation plan {id} owns queued or executing work and cannot be replaced."
            ),
            Self::InvalidLineage(id) => write!(
                formatter,
                "Simulation plan {id} has incomplete clone-lineage metadata."
            ),
            Self::DuplicatePlanIdentity(id) => write!(
                formatter,
                "Simulation plan identity {id} appears more than once in the project."
            ),
            Self::InvalidPlan(error) => error.fmt(formatter),
        }
    }
}

impl std::error::Error for SimulationPlanCatalogError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::InvalidPlan(error) => Some(error),
            _ => None,
        }
    }
}

impl From<AnalysisPlanError> for SimulationPlanCatalogError {
    fn from(error: AnalysisPlanError) -> Self {
        Self::InvalidPlan(error)
    }
}

impl SimSetupState {
    #[must_use]
    pub fn active_plan_name(&self) -> &SimulationPlanName {
        &self.active_plan_name
    }

    #[must_use]
    pub const fn active_plan_lineage(&self) -> SimulationPlanLineage {
        self.active_plan_lineage
    }

    #[must_use]
    pub fn inactive_plans(&self) -> &[StoredSimulationPlan] {
        &self.inactive_plans
    }

    #[must_use]
    pub fn plan_count(&self) -> usize {
        usize::from(self.analysis_plan.is_some()) + self.inactive_plans.len()
    }

    /// Clone the active plan and activate the clone in one atomic operation.
    ///
    /// The source becomes an inactive named plan. Result data is not part of
    /// `SimSetupState`, so no result dataset or manifest can be duplicated by
    /// this transaction.
    pub fn clone_active_plan(
        &mut self,
        new_name: impl Into<String>,
        contents: SimulationPlanCloneOptions,
    ) -> Result<SimulationPlanCloneOutcome, SimulationPlanCatalogError> {
        let new_name = SimulationPlanName::new(new_name)?;
        self.ensure_plan_name_available(&new_name, None)?;
        self.validate_plan_catalog()?;

        let source = self
            .analysis_plan
            .as_ref()
            .ok_or(SimulationPlanCatalogError::ActivePlanUnavailable)?;
        if source.has_executing_instances() {
            return Err(SimulationPlanCatalogError::PlanExecuting(source.id()));
        }

        let existing_ids = self.plan_identities();
        let cloned_plan = loop {
            let candidate = if contents.copy_analyses {
                source.clone_as_new()?
            } else {
                SimulationPlan::empty()
            };
            if !existing_ids.contains(&candidate.id()) {
                break candidate;
            }
        };
        let source_id = source.id();
        let source_revision = source.revision();
        let cloned_id = cloned_plan.id();
        let analysis_identity_map = source
            .instances()
            .iter()
            .zip(cloned_plan.instances())
            .map(|(source, cloned)| (source.id(), cloned.id()))
            .collect();
        let cloned_lineage =
            SimulationPlanLineage::cloned_from_with_contents(source_id, source_revision, contents);
        let cloned_reference_pvt = if contents.copy_pvt_and_model_bindings {
            self.reference_pvt
        } else {
            ReferencePvtPoint::default()
        };
        let mut cloned_options = if contents.copy_advanced_options {
            self.options.clone()
        } else {
            SimulationOptions::default()
        };
        // Reference temperature is owned by the PVT domain and must agree
        // with the exact value the solver consumes, regardless of whether the
        // rest of the advanced options were copied.
        cloned_options.temp = cloned_reference_pvt.temperature_celsius;

        let mut candidate = self.clone();
        let source_plan = candidate
            .analysis_plan
            .take()
            .ok_or(SimulationPlanCatalogError::ActivePlanUnavailable)?;
        candidate.inactive_plans.push(StoredSimulationPlan {
            name: candidate.active_plan_name.clone(),
            lineage: candidate.active_plan_lineage,
            analysis_plan: source_plan,
            reference_pvt: candidate.reference_pvt,
            options: candidate.options.clone(),
        });
        candidate.active_plan_name = new_name;
        candidate.active_plan_lineage = cloned_lineage;
        candidate.analysis_plan = Some(cloned_plan);
        candidate.reference_pvt = cloned_reference_pvt;
        candidate.options = cloned_options;
        candidate.reset_plan_editor_transients();
        candidate.refresh_legacy_analysis_projections();
        candidate.validate_plan_catalog()?;
        *self = candidate;
        Ok(SimulationPlanCloneOutcome {
            source_plan_id: source_id,
            source_revision,
            cloned_plan_id: cloned_id,
            contents,
            analysis_identity_map,
        })
    }

    /// Activate an existing plan without changing either plan's durable
    /// identity or revision. The replaced active plan is retained in the same
    /// catalog slot, preserving deterministic presentation order.
    pub fn activate_plan(
        &mut self,
        id: SimulationPlanId,
    ) -> Result<(), SimulationPlanCatalogError> {
        self.validate_plan_catalog()?;
        let active = self
            .analysis_plan
            .as_ref()
            .ok_or(SimulationPlanCatalogError::ActivePlanUnavailable)?;
        if active.id() == id {
            return Ok(());
        }
        if active.has_executing_instances() {
            return Err(SimulationPlanCatalogError::PlanExecuting(active.id()));
        }
        let index = self
            .inactive_plans
            .iter()
            .position(|plan| plan.id() == id)
            .ok_or(SimulationPlanCatalogError::PlanNotFound(id))?;
        if self.inactive_plans[index]
            .analysis_plan
            .has_executing_instances()
        {
            return Err(SimulationPlanCatalogError::PlanExecuting(id));
        }

        let mut candidate = self.clone();
        let target = candidate.inactive_plans.remove(index);
        let current = StoredSimulationPlan {
            name: candidate.active_plan_name.clone(),
            lineage: candidate.active_plan_lineage,
            analysis_plan: candidate
                .analysis_plan
                .take()
                .ok_or(SimulationPlanCatalogError::ActivePlanUnavailable)?,
            reference_pvt: candidate.reference_pvt,
            options: candidate.options.clone(),
        };
        candidate.inactive_plans.insert(index, current);
        candidate.active_plan_name = target.name;
        candidate.active_plan_lineage = target.lineage;
        candidate.analysis_plan = Some(target.analysis_plan);
        candidate.reference_pvt = target.reference_pvt;
        candidate.options = target.options;
        candidate.reset_plan_editor_transients();
        candidate.refresh_legacy_analysis_projections();
        candidate.validate_plan_catalog()?;
        *self = candidate;
        Ok(())
    }

    /// Rename the active plan while enforcing project-wide uniqueness.
    pub fn rename_active_plan(
        &mut self,
        new_name: impl Into<String>,
    ) -> Result<(), SimulationPlanCatalogError> {
        let id = self
            .analysis_plan
            .as_ref()
            .ok_or(SimulationPlanCatalogError::ActivePlanUnavailable)?
            .id();
        let new_name = SimulationPlanName::new(new_name)?;
        self.ensure_plan_name_available(&new_name, Some(id))?;
        self.active_plan_name = new_name;
        Ok(())
    }

    /// Rename any retained inactive plan while enforcing project-wide
    /// uniqueness. Pass the active plan identity to rename the active plan.
    pub fn rename_plan(
        &mut self,
        id: SimulationPlanId,
        new_name: impl Into<String>,
    ) -> Result<(), SimulationPlanCatalogError> {
        if self.analysis_plan.as_ref().map(SimulationPlan::id) == Some(id) {
            return self.rename_active_plan(new_name);
        }
        let new_name = SimulationPlanName::new(new_name)?;
        self.ensure_plan_name_available(&new_name, Some(id))?;
        let plan = self
            .inactive_plans
            .iter_mut()
            .find(|plan| plan.id() == id)
            .ok_or(SimulationPlanCatalogError::PlanNotFound(id))?;
        plan.name = new_name;
        Ok(())
    }

    /// Advance the active plan revision for a committed variables, outputs,
    /// specifications, PVT, model-binding, or other plan-owned configuration
    /// change. This invalidates revision-pinned preflight evidence without
    /// pretending an analysis instance was edited.
    pub fn commit_active_plan_configuration_change(
        &mut self,
        detail: impl Into<String>,
    ) -> Result<
        crate::simulation::plan::SimulationPlanConfigurationReceipt,
        SimulationPlanCatalogError,
    > {
        self.validate_plan_catalog()?;
        let plan = self
            .analysis_plan
            .as_mut()
            .ok_or(SimulationPlanCatalogError::ActivePlanUnavailable)?;
        let receipt = plan.commit_configuration_change(detail)?;
        Ok(receipt)
    }

    /// Validate names, identities, lineage, and every active/inactive analysis
    /// graph. Editable incompleteness is permitted; structural corruption is
    /// rejected.
    pub(crate) fn validate_plan_catalog(&self) -> Result<(), SimulationPlanCatalogError> {
        let active = self
            .analysis_plan
            .as_ref()
            .ok_or(SimulationPlanCatalogError::ActivePlanUnavailable)?;
        active.validate_structure()?;

        let mut names = HashSet::with_capacity(self.plan_count());
        names.insert(self.active_plan_name.uniqueness_key());
        let mut ids = HashSet::with_capacity(self.plan_count());
        ids.insert(active.id());
        if !self.active_plan_lineage.is_valid() {
            return Err(SimulationPlanCatalogError::InvalidLineage(active.id()));
        }

        for stored in &self.inactive_plans {
            if !names.insert(stored.name.uniqueness_key()) {
                return Err(SimulationPlanCatalogError::DuplicateName(
                    stored.name.to_string(),
                ));
            }
            if !ids.insert(stored.id()) {
                return Err(SimulationPlanCatalogError::DuplicatePlanIdentity(
                    stored.id(),
                ));
            }
            if !stored.lineage.is_valid() {
                return Err(SimulationPlanCatalogError::InvalidLineage(stored.id()));
            }
            stored.analysis_plan.validate_structure()?;
        }
        Ok(())
    }

    pub(crate) fn prepare_plan_catalog_after_restore(&mut self) {
        for plan in &mut self.inactive_plans {
            plan.analysis_plan.prepare_after_restore();
        }
    }

    fn plan_identities(&self) -> HashSet<SimulationPlanId> {
        self.analysis_plan
            .iter()
            .map(SimulationPlan::id)
            .chain(self.inactive_plans.iter().map(StoredSimulationPlan::id))
            .collect()
    }

    fn ensure_plan_name_available(
        &self,
        name: &SimulationPlanName,
        except_id: Option<SimulationPlanId>,
    ) -> Result<(), SimulationPlanCatalogError> {
        let key = name.uniqueness_key();
        let active_conflicts = self
            .analysis_plan
            .as_ref()
            .is_some_and(|plan| Some(plan.id()) != except_id)
            && self.active_plan_name.uniqueness_key() == key;
        let inactive_conflicts = self
            .inactive_plans
            .iter()
            .any(|plan| Some(plan.id()) != except_id && plan.name.uniqueness_key() == key);
        if active_conflicts || inactive_conflicts {
            Err(SimulationPlanCatalogError::DuplicateName(name.to_string()))
        } else {
            Ok(())
        }
    }

    fn reset_plan_editor_transients(&mut self) {
        self.options_draft =
            crate::simulation::dialog::OptionsDialogState::from_options(&self.options);
        self.options_errors.clear();
        self.options_open = false;
        self.palette_open = false;
        self.palette_query.clear();
        self.palette_active = 0;
        self.palette_scroll_to_active = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::simulation::plan::{AnalysisKind, AnalysisLifecycleState};

    #[test]
    fn plan_names_are_trimmed_validated_and_case_insensitively_unique() {
        let name = SimulationPlanName::new("  Post-layout sweep  ").expect("name is valid");
        assert_eq!(name.as_str(), "Post-layout sweep");
        assert!(SimulationPlanName::new("\n").is_err());
        assert!(SimulationPlanName::new("a".repeat(97)).is_err());

        let mut setup = SimSetupState::new();
        let error = setup
            .clone_active_plan(
                "LAB CHARACTERIZATION",
                SimulationPlanCloneOptions::default(),
            )
            .expect_err("names are unique without ASCII-case ambiguity");
        assert!(matches!(
            error,
            SimulationPlanCatalogError::DuplicateName(_)
        ));
    }

    #[test]
    fn cloning_activates_a_fresh_plan_and_retains_an_independent_source() {
        let mut setup = SimSetupState::new();
        let source_id = setup.analysis_plan.as_ref().unwrap().id();
        let source_instance_id = setup.analysis_plan.as_ref().unwrap().instances()[0].id();
        setup.options.reltol = 2.5e-5;
        setup
            .analysis_plan
            .as_mut()
            .unwrap()
            .edit(source_instance_id, |draft| {
                let crate::simulation::plan::AnalysisDraft::Transient(draft) = draft else {
                    panic!("expected transient");
                };
                draft.stop = "42u".to_owned();
            })
            .unwrap();

        let clone = setup
            .clone_active_plan(
                "Lab characterization · variant",
                SimulationPlanCloneOptions::default(),
            )
            .expect("clone commits");
        let clone_id = clone.cloned_plan_id;

        assert_eq!(setup.plan_count(), 2);
        assert_eq!(
            setup.active_plan_name().as_str(),
            "Lab characterization · variant"
        );
        assert_eq!(setup.analysis_plan.as_ref().unwrap().id(), clone_id);
        assert_ne!(clone_id, source_id);
        assert_eq!(setup.options.reltol, 2.5e-5);
        assert_eq!(clone.source_plan_id, source_id);
        assert_eq!(clone.contents, SimulationPlanCloneOptions::default());
        assert_eq!(
            clone.analysis_identity_map,
            vec![(source_instance_id, clone_instance_id(&setup))]
        );
        assert_eq!(
            setup.active_plan_lineage(),
            SimulationPlanLineage::cloned_from(source_id, setup.inactive_plans()[0].revision())
        );
        let clone_instance = &setup.analysis_plan.as_ref().unwrap().instances()[0];
        assert_ne!(clone_instance.id(), source_instance_id);
        assert_eq!(clone_instance.lifecycle(), AnalysisLifecycleState::Draft);
        assert!(setup.analysis_plan.as_ref().unwrap().receipts().is_empty());
        assert!(
            setup
                .analysis_plan
                .as_ref()
                .unwrap()
                .tombstones()
                .is_empty()
        );
        assert_eq!(setup.inactive_plans()[0].id(), source_id);

        setup
            .activate_plan(source_id)
            .expect("source can be reactivated");
        assert_eq!(setup.active_plan_name().as_str(), DEFAULT_PLAN_NAME);
        assert_eq!(setup.analysis_plan.as_ref().unwrap().id(), source_id);
        assert_eq!(
            setup.analysis_plan.as_ref().unwrap().instances()[0].id(),
            source_instance_id
        );
        assert_eq!(setup.inactive_plans()[0].id(), clone_id);
    }

    #[test]
    fn clone_content_flags_create_an_empty_plan_with_default_advanced_options() {
        let mut setup = SimSetupState::new();
        setup
            .set_reference_pvt(crate::simulation::dialog::corner::ProcessCorner::SS, 125.0)
            .unwrap();
        setup.options.reltol = 9.0e-7;

        let contents = SimulationPlanCloneOptions {
            copy_analyses: false,
            copy_advanced_options: false,
            ..SimulationPlanCloneOptions::default()
        };
        let outcome = setup
            .clone_active_plan("Empty diagnostic", contents)
            .expect("empty clone commits");

        assert!(setup.analysis_plan.as_ref().unwrap().instances().is_empty());
        assert_eq!(setup.options.reltol, SimulationOptions::default().reltol);
        assert_eq!(setup.options.temp, 125.0);
        assert_eq!(
            setup.reference_pvt.process,
            crate::simulation::dialog::corner::ProcessCorner::SS
        );
        assert_eq!(setup.reference_pvt.temperature_celsius, 125.0);
        assert_eq!(outcome.contents, contents);
        assert!(outcome.analysis_identity_map.is_empty());
        assert_eq!(setup.active_plan_lineage().clone_contents(), Some(contents));
        assert_eq!(setup.enabled_analysis_instance_count(), 0);
    }

    fn clone_instance_id(setup: &SimSetupState) -> AnalysisInstanceId {
        setup.analysis_plan.as_ref().unwrap().instances()[0].id()
    }

    #[test]
    fn plan_switch_is_atomic_when_the_target_is_missing() {
        let mut setup = SimSetupState::new();
        let before = serde_json::to_value(&setup).unwrap();
        assert!(matches!(
            setup.activate_plan(SimulationPlanId::new()),
            Err(SimulationPlanCatalogError::PlanNotFound(_))
        ));
        assert_eq!(serde_json::to_value(&setup).unwrap(), before);
    }

    #[test]
    fn stored_plan_round_trip_preserves_catalog_identity_and_lineage() {
        let mut setup = SimSetupState::new();
        let source = setup.analysis_plan.as_ref().unwrap().id();
        let clone = setup
            .clone_active_plan("AC characterization", SimulationPlanCloneOptions::default())
            .unwrap()
            .cloned_plan_id;
        setup
            .analysis_plan
            .as_mut()
            .unwrap()
            .insert(AnalysisKind::OperatingPoint)
            .unwrap();

        let json = serde_json::to_string(&setup).unwrap();
        let mut restored: SimSetupState = serde_json::from_str(&json).unwrap();
        restored.prepare_after_restore();

        assert_eq!(restored.analysis_plan.as_ref().unwrap().id(), clone);
        assert_eq!(restored.inactive_plans()[0].id(), source);
        assert_eq!(
            restored.active_plan_lineage().source_plan_id(),
            Some(source)
        );
        restored.validate_plan_catalog().unwrap();
    }
}
