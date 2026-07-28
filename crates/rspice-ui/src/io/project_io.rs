//! Project file I/O.
//!
//! `.rspiceproj` stores the product-level workspace: project identity,
//! libraries/cells/views, open documents, schematic buffers, authoritative
//! simulation inputs, model-section bindings, and retained results. Individual
//! schematic export remains available through `.rsch`; project files are the
//! native professional workflow container.

mod results;

pub use results::*;

use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use sha2::{Digest as _, Sha256};

use crate::io::project_execution::ProjectExecutionContext;
use crate::product::{
    AnalysisInstanceId, ContentDigest, DatasetId, JobId, ModelSourceId, ObjectRevision, ProjectId,
    RunId, SimulationPlanId,
};
use crate::simulation::plan::AnalysisKind;
use crate::state::workspace::validate_cell_view_name_segment;
use crate::state::{
    AnalysisResult, AnalysisResultFamilyMetadata, AnalysisResultPayload, AnalysisResultProvenance,
    AnalysisResultSourceDomain, AnalysisType, CanonicalCellViewOwnerKey, CellViewRef, DcOpResult,
    ExecutionTarget, LibraryManager, NoiseContributorRow, NoiseSummary, OperatingPointValue,
    PreparedModelSourceIdentity, PreparedRunReceipt, PreparedRunTaskReceipt,
    PreparedSourceCheckReceipt, ProjectWorkspace, SavedOutputMaterializationStatus,
    SavedOutputReceipt, SimulationRun, SimulationRunLifecycle, SimulationRunProvenance,
    SimulationState, ViewType, WaveformData, canonical_cell_view_owner_key,
};

/// Presence-aware persisted field used at schema-era boundaries.
///
/// Serde's `Option<T>` intentionally treats an explicit JSON `null` exactly
/// like an absent field. That is unsafe for fields introduced by a later
/// schema because a caller could otherwise add the field as `null`, relabel
/// the container as an older schema, and have migration accept it as genuine
/// absence. This representation keeps all three wire states distinct.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum PersistedField<T> {
    #[default]
    Missing,
    Null,
    Value(T),
}

impl<T> PersistedField<T> {
    #[must_use]
    pub const fn is_missing(&self) -> bool {
        matches!(self, Self::Missing)
    }

    #[must_use]
    pub const fn is_null(&self) -> bool {
        matches!(self, Self::Null)
    }

    #[must_use]
    pub const fn is_present(&self) -> bool {
        !self.is_missing()
    }

    #[must_use]
    pub const fn as_ref(&self) -> Option<&T> {
        match self {
            Self::Value(value) => Some(value),
            Self::Missing | Self::Null => None,
        }
    }

    #[must_use]
    pub fn as_mut(&mut self) -> Option<&mut T> {
        match self {
            Self::Value(value) => Some(value),
            Self::Missing | Self::Null => None,
        }
    }

    #[must_use]
    pub fn into_value(self) -> Option<T> {
        match self {
            Self::Value(value) => Some(value),
            Self::Missing | Self::Null => None,
        }
    }
}

impl<T: Serialize> Serialize for PersistedField<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Value(value) => value.serialize(serializer),
            Self::Missing | Self::Null => serializer.serialize_none(),
        }
    }
}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for PersistedField<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Option::<T>::deserialize(deserializer).map(|value| match value {
            Some(value) => Self::Value(value),
            None => Self::Null,
        })
    }
}

#[derive(Debug)]
struct ValidatedLibraryView {
    reference: CellViewRef,
    view_type: ViewType,
}

#[derive(Debug, Default)]
struct ValidatedLibraryIndex {
    exact: HashMap<String, ValidatedLibraryView>,
    canonical: HashMap<CanonicalCellViewOwnerKey, String>,
}

impl ValidatedLibraryIndex {
    fn get_exact(&self, key: &str) -> Option<&ValidatedLibraryView> {
        self.exact.get(key)
    }

    fn get_owner(&self, reference: &CellViewRef) -> Option<&ValidatedLibraryView> {
        let key =
            canonical_cell_view_owner_key(&reference.library, &reference.cell, &reference.view);
        self.canonical
            .get(&key)
            .and_then(|exact| self.exact.get(exact))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectVersion {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

impl ProjectVersion {
    pub const fn current() -> Self {
        Self {
            major: 1,
            minor: 0,
            patch: 0,
        }
    }

    pub fn is_compatible(self) -> bool {
        self.major == Self::current().major
    }
}

impl Default for ProjectVersion {
    fn default() -> Self {
        Self::current()
    }
}

impl std::fmt::Display for ProjectVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectFile {
    pub version: ProjectVersion,
    pub workspace: ProjectWorkspace,
    pub libraries: LibraryManager,
    #[serde(default, skip_serializing_if = "ProjectSimulationResults::is_empty")]
    pub simulation_results: ProjectSimulationResults,
    /// Authoritative execution inputs. Absent only in projects written before
    /// project-owned simulation plans were introduced.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub execution_context: Option<ProjectExecutionContext>,
    #[serde(skip)]
    pub simulation_results_warning: Option<String>,
}

impl ProjectFile {
    pub fn new(workspace: ProjectWorkspace, libraries: LibraryManager) -> Self {
        Self {
            version: ProjectVersion::current(),
            workspace,
            libraries,
            simulation_results: ProjectSimulationResults::default(),
            execution_context: None,
            simulation_results_warning: None,
        }
    }

    pub fn new_with_simulation_results(
        workspace: ProjectWorkspace,
        libraries: LibraryManager,
        simulation_results: ProjectSimulationResults,
    ) -> Self {
        Self {
            version: ProjectVersion::current(),
            workspace,
            libraries,
            simulation_results,
            execution_context: None,
            simulation_results_warning: None,
        }
    }

    pub fn new_with_execution_context(
        mut workspace: ProjectWorkspace,
        libraries: LibraryManager,
        simulation_results: ProjectSimulationResults,
        execution_context: ProjectExecutionContext,
    ) -> Self {
        if let Some(plan) = &execution_context.simulation_plan.analysis_plan {
            workspace.migrate_active_plan_data(plan.id());
        }
        for plan in execution_context.simulation_plan.inactive_plans() {
            workspace.migrate_inactive_plan_data(plan.id());
        }
        Self {
            version: ProjectVersion::current(),
            workspace,
            libraries,
            simulation_results,
            execution_context: Some(execution_context),
            simulation_results_warning: None,
        }
    }

    pub fn validate(&self) -> Result<(), ProjectIoError> {
        if !self.version.is_compatible() {
            return Err(ProjectIoError::IncompatibleVersion {
                file_version: self.version.to_string(),
                app_version: ProjectVersion::current().to_string(),
            });
        }
        self.workspace
            .project
            .validate()
            .map_err(|error| ProjectIoError::InvalidData(error.to_string()))?;
        self.workspace
            .validate_simulation_configuration()
            .map_err(|error| ProjectIoError::InvalidData(error.to_string()))?;
        let view_index = self.validate_library_tree()?;
        self.validate_project_source_owners(&view_index)?;
        self.validate_workspace_references(&view_index)?;
        if let Some(context) = &self.execution_context {
            context.validate().map_err(|error| {
                ProjectIoError::InvalidData(format!("execution context is invalid: {error}"))
            })?;
        }
        if let Some(binding) = self.workspace.project.technology_binding() {
            let context = self.execution_context.as_ref().ok_or_else(|| {
                ProjectIoError::InvalidData(
                    "attached technology requires an authoritative execution context".to_owned(),
                )
            })?;
            context
                .validate_technology_binding(binding)
                .map_err(|error| {
                    ProjectIoError::InvalidData(format!(
                        "attached technology does not match the execution catalog: {error}"
                    ))
                })?;
        }
        self.validate_simulation_configuration_references()?;
        Ok(())
    }

    fn validate_simulation_configuration_references(&self) -> Result<(), ProjectIoError> {
        let setup = self
            .execution_context
            .as_ref()
            .map(|context| &context.simulation_plan);
        let find_plan = |plan_id: SimulationPlanId| {
            setup.and_then(|setup| {
                setup
                    .analysis_plan
                    .as_ref()
                    .filter(|plan| plan.id() == plan_id)
                    .or_else(|| {
                        setup
                            .inactive_plans()
                            .iter()
                            .find(|plan| plan.id() == plan_id)
                            .map(crate::workbench::app_state::StoredSimulationPlan::analysis_plan)
                    })
            })
        };

        for record in &self.workspace.simulation_plan_payloads {
            let plan_id = record.plan_id;
            let plan = find_plan(plan_id).ok_or_else(|| {
                ProjectIoError::InvalidData(format!(
                    "workspace.simulation_plan_payloads[{plan_id}] has no owning simulation plan"
                ))
            })?;
            for (index, variable) in record.payload.design_variables.iter().enumerate() {
                match &variable.scope {
                    crate::state::DesignVariableScope::SelectedCell { cell } => {
                        self.validate_library_reference(
                            &format!(
                                "workspace.simulation_plan_payloads[{plan_id}].design_variables[{index}].scope.cell"
                            ),
                            cell,
                        )?;
                    }
                    crate::state::DesignVariableScope::SelectedAnalysis { analysis_id } => {
                        if plan.instance(*analysis_id).is_none() {
                            return Err(ProjectIoError::InvalidData(format!(
                                "workspace.simulation_plan_payloads[{plan_id}].design_variables[{index}].scope.analysis_id references analysis {analysis_id}, which is absent from its owning plan"
                            )));
                        }
                    }
                    crate::state::DesignVariableScope::Testbench
                    | crate::state::DesignVariableScope::Project => {}
                }
            }

            for (index, output) in record.payload.saved_outputs.iter().enumerate() {
                if let crate::state::SavedOutputCompatibility::SelectedAnalysis { analysis_id } =
                    &output.compatible_analyses
                    && plan.instance(*analysis_id).is_none()
                {
                    return Err(ProjectIoError::InvalidData(format!(
                        "workspace.simulation_plan_payloads[{plan_id}].saved_outputs[{index}].compatible_analyses.analysis_id references analysis {analysis_id}, which is absent from its owning plan"
                    )));
                }
            }
            for (index, tolerance) in record.payload.regression_tolerances.iter().enumerate() {
                if tolerance.target.source_domain
                    == crate::state::AnalysisResultSourceDomain::SimulationPlan
                    && plan.instance(tolerance.target.source_instance_id).is_none()
                {
                    return Err(ProjectIoError::InvalidData(format!(
                        "workspace.simulation_plan_payloads[{plan_id}].regression_tolerances[{index}].target.source_instance_id references analysis {}, which is absent from its owning plan",
                        tolerance.target.source_instance_id
                    )));
                }
            }
        }

        if let Some(setup) = setup {
            let plan_ids = setup
                .analysis_plan
                .iter()
                .map(crate::simulation::plan::SimulationPlan::id)
                .chain(setup.inactive_plans().iter().map(|plan| plan.id()));
            for plan_id in plan_ids {
                if self.workspace.active_plan_data(plan_id).is_none() {
                    return Err(ProjectIoError::InvalidData(format!(
                        "simulation plan {plan_id} has no plan-owned configuration payload"
                    )));
                }
            }
        } else if !self.workspace.simulation_plan_payloads.is_empty() {
            return Err(ProjectIoError::InvalidData(
                "simulation plan payloads are present without an execution context".to_owned(),
            ));
        }
        Ok(())
    }

    fn validate_regression_baseline_eligibility(&self) -> Result<(), String> {
        for record in &self.workspace.simulation_plan_payloads {
            let Some(run_id) = record.payload.regression_baseline_run else {
                continue;
            };
            let run = self
                .simulation_results
                .runs
                .iter()
                .find(|run| run.run_id == Some(run_id))
                .ok_or_else(|| {
                    format!(
                        "simulation plan {} references regression baseline {run_id}, which is absent from retained result history",
                        record.plan_id
                    )
                })?;
            let receipt = run.prepared_receipt.as_ref().ok_or_else(|| {
                format!("regression baseline {run_id} has no current prepared-run authority")
            })?;
            if run.provenance_mode.as_ref() != Some(&ProjectRunProvenanceMode::PreparedTaskBound)
                || receipt.source_domain == AnalysisResultSourceDomain::LegacyUnclassified
            {
                return Err(format!(
                    "regression baseline {run_id} is legacy or unclassified"
                ));
            }
            if run.analyses.len() != receipt.tasks.len()
                || run.analyses.iter().any(|analysis| !analysis.success)
            {
                return Err(format!(
                    "regression baseline {run_id} is incomplete or unsuccessful"
                ));
            }
        }
        Ok(())
    }

    fn clear_regression_baseline_references(&mut self) -> usize {
        let mut cleared = 0;
        for record in &mut self.workspace.simulation_plan_payloads {
            if record.payload.regression_baseline_run.take().is_some() {
                cleared += 1;
            }
        }
        cleared
    }

    /// Validate the cross-document edge from retained results to the exact
    /// simulation-plan identity that produced them. Execution context and
    /// result history are independently well-formed documents, but a project
    /// may persist them only as one referentially closed engineering record.
    fn validate_result_plan_references(&self) -> Result<(), String> {
        Self::validate_result_plan_references_for(
            &self.simulation_results,
            self.execution_context
                .as_ref()
                .map(|context| &context.simulation_plan),
            self.workspace.project.revision(),
        )
    }

    pub(crate) fn validate_result_plan_references_for(
        simulation_results: &ProjectSimulationResults,
        simulation_plan: Option<&crate::workbench::app_state::SimSetupState>,
        project_revision: ObjectRevision,
    ) -> Result<(), String> {
        let has_plan_receipt = simulation_results.runs.iter().any(|run| {
            run.prepared_receipt.as_ref().is_some_and(|receipt| {
                receipt.source_domain == AnalysisResultSourceDomain::SimulationPlan
            })
        });
        let plan = if has_plan_receipt {
            Some(
                simulation_plan
                    .ok_or_else(|| {
                        "simulation-plan result history has no persisted simulation plan".to_owned()
                    })?
                    .stable_analysis_plan()?,
            )
        } else {
            None
        };
        let current = plan
            .map(|plan| {
                plan.instances()
                    .iter()
                    .map(|instance| (instance.id(), instance))
                    .collect::<HashMap<_, _>>()
            })
            .unwrap_or_default();
        let retired = plan
            .map(|plan| {
                plan.tombstones()
                    .iter()
                    .map(|tombstone| (tombstone.id(), tombstone))
                    .collect::<HashMap<_, _>>()
            })
            .unwrap_or_default();

        for (run_idx, run) in simulation_results.runs.iter().enumerate() {
            let Some(receipt) = run.prepared_receipt.as_ref() else {
                continue;
            };
            if receipt.project_revision > project_revision {
                return Err(format!(
                    "runs[{run_idx}].prepared_receipt.project_revision {} is newer than project revision {}",
                    receipt.project_revision.get(),
                    project_revision.get()
                ));
            }
            match receipt.source_domain {
                AnalysisResultSourceDomain::SimulationPlan => {
                    let plan = plan.expect("plan receipt precondition established above");
                    if receipt.simulation_plan_id != Some(plan.id()) {
                        return Err(format!(
                            "runs[{run_idx}].prepared_receipt.simulation_plan_id does not match persisted plan {}",
                            plan.id()
                        ));
                    }
                    let produced_task_ids = run
                        .analyses
                        .iter()
                        .filter_map(|analysis| analysis.provenance.as_ref())
                        .map(|provenance| provenance.source_instance_id)
                        .collect::<HashSet<_>>();
                    for (task_idx, task) in receipt.tasks.iter().enumerate() {
                        let prefix = format!("runs[{run_idx}].prepared_receipt.tasks[{task_idx}]");
                        let (kind, created_revision, latest_revision, is_retired) = if let Some(
                            instance,
                        ) =
                            current.get(&task.source_instance_id)
                        {
                            (
                                instance.kind(),
                                instance.created_revision(),
                                plan.revision(),
                                false,
                            )
                        } else if let Some(tombstone) = retired.get(&task.source_instance_id) {
                            if produced_task_ids.contains(&task.source_instance_id) {
                                let run_id = run.run_id.ok_or_else(|| {
                                        format!(
                                            "runs[{run_idx}] has prepared provenance but no stable run id"
                                        )
                                    })?;
                                if !tombstone.prior_run_ids().contains(&run_id) {
                                    return Err(format!(
                                        "{prefix}.source_instance_id identifies retired analysis {}, but run {run_id} is not retained by its tombstone",
                                        task.source_instance_id
                                    ));
                                }
                            }
                            (
                                tombstone.kind(),
                                tombstone.created_revision(),
                                tombstone.removed_revision(),
                                true,
                            )
                        } else {
                            return Err(format!(
                                "{prefix}.source_instance_id {} is absent from the persisted plan and its tombstones",
                                task.source_instance_id
                            ));
                        };
                        let outside_lifetime = task.source_revision < created_revision
                            || if is_retired {
                                task.source_revision >= latest_revision
                            } else {
                                task.source_revision > latest_revision
                            };
                        if outside_lifetime {
                            let interval = if is_retired {
                                format!("{}..{}", created_revision.get(), latest_revision.get())
                            } else {
                                format!("{}..={}", created_revision.get(), latest_revision.get())
                            };
                            return Err(format!(
                                "{prefix}.source_revision {} is outside the retained analysis revision interval {interval}",
                                task.source_revision.get()
                            ));
                        }
                        if task.analysis_kind_tag != analysis_kind_tag_for_plan_kind(kind) {
                            return Err(format!(
                                "{prefix}.analysis_kind_tag does not match persisted {kind} analysis"
                            ));
                        }
                    }
                }
                AnalysisResultSourceDomain::ManualDeck => {
                    if receipt.simulation_plan_id.is_some() {
                        return Err(format!(
                            "runs[{run_idx}].prepared_receipt manual source must not claim a simulation plan"
                        ));
                    }
                    let mut occurrences = HashMap::<u8, usize>::new();
                    for (task_idx, task) in receipt.tasks.iter().enumerate() {
                        let occurrence = occurrences.entry(task.analysis_kind_tag).or_default();
                        let expected =
                            crate::simulation::execution::manual_deck_analysis_instance_id_from_tag(
                                receipt.source_content_digest,
                                task.analysis_kind_tag,
                                *occurrence,
                            );
                        *occurrence += 1;
                        if task.source_instance_id != expected {
                            return Err(format!(
                                "runs[{run_idx}].prepared_receipt.tasks[{task_idx}].source_instance_id is not derived from the retained manual source identity"
                            ));
                        }
                        if task.source_revision != receipt.project_revision {
                            return Err(format!(
                                "runs[{run_idx}].prepared_receipt.tasks[{task_idx}].source_revision does not match the authenticated manual-run project revision"
                            ));
                        }
                    }
                }
                AnalysisResultSourceDomain::LegacyUnclassified => {
                    return Err(format!(
                        "runs[{run_idx}].prepared_receipt cannot use a legacy-unclassified source"
                    ));
                }
            }
        }
        Ok(())
    }

    fn validate_library_tree(&self) -> Result<ValidatedLibraryIndex, ProjectIoError> {
        let mut view_index = ValidatedLibraryIndex::default();
        let mut canonical_libraries = HashMap::new();
        let mut canonical_cells = HashMap::new();
        let mut first_invalid_name = None;
        let mut libraries = self.libraries.libraries_by_key().collect::<Vec<_>>();
        libraries.sort_by(|(left, _), (right, _)| left.cmp(right));

        for (library_key, library) in libraries {
            if library.name != library_key {
                return Err(ProjectIoError::InvalidData(format!(
                    "library map key '{library_key}' does not match embedded library name '{}'",
                    library.name
                )));
            }
            record_invalid_lcv_name(
                &mut first_invalid_name,
                &format!("library '{library_key}'"),
                library_key,
            );
            let canonical_library = canonical_cell_view_owner_key(library_key, "", "");
            if let Some(existing) =
                canonical_libraries.insert(canonical_library, library_key.to_owned())
            {
                return Err(ProjectIoError::InvalidData(format!(
                    "library tree contains canonical library identity collision between '{existing}' and '{library_key}'"
                )));
            }

            let mut cells = library.cells.iter().collect::<Vec<_>>();
            cells.sort_by(|(left, _), (right, _)| left.cmp(right));
            for (cell_key, cell) in cells {
                if cell.name != *cell_key {
                    return Err(ProjectIoError::InvalidData(format!(
                        "cell map key '{library_key}/{cell_key}' does not match embedded cell name '{}'",
                        cell.name
                    )));
                }
                record_invalid_lcv_name(
                    &mut first_invalid_name,
                    &format!("cell '{library_key}/{cell_key}'"),
                    cell_key,
                );
                let canonical_cell = canonical_cell_view_owner_key(library_key, cell_key, "");
                if let Some(existing) =
                    canonical_cells.insert(canonical_cell, format!("{library_key}/{cell_key}"))
                {
                    return Err(ProjectIoError::InvalidData(format!(
                        "library tree contains canonical cell identity collision between '{existing}' and '{library_key}/{cell_key}'"
                    )));
                }

                let mut views = cell.views.iter().collect::<Vec<_>>();
                views.sort_by(|(left, _), (right, _)| left.cmp(right));
                for (view_key, view) in views {
                    if view.name != *view_key {
                        return Err(ProjectIoError::InvalidData(format!(
                            "view map key '{library_key}/{cell_key}/{view_key}' does not match embedded view name '{}'",
                            view.name
                        )));
                    }
                    record_invalid_lcv_name(
                        &mut first_invalid_name,
                        &format!("view '{library_key}/{cell_key}/{view_key}'"),
                        view_key,
                    );

                    let reference = CellViewRef::new(library_key, cell_key, view_key);
                    let key = reference.key();
                    if let Some(existing) = view_index.exact.get(&key) {
                        return Err(ProjectIoError::InvalidData(format!(
                            "library tree generates duplicate cell-view key '{key}' for {} and {}; slash-delimited persisted keys must be injective",
                            format_lcv_segments(&existing.reference),
                            format_lcv_segments(&reference)
                        )));
                    }
                    let canonical = canonical_cell_view_owner_key(
                        &reference.library,
                        &reference.cell,
                        &reference.view,
                    );
                    if let Some(existing_key) = view_index.canonical.get(&canonical) {
                        let existing = view_index
                            .exact
                            .get(existing_key)
                            .expect("canonical library index points at an exact view");
                        return Err(ProjectIoError::InvalidData(format!(
                            "library tree contains canonical cell-view identity collision between {} and {}",
                            format_lcv_segments(&existing.reference),
                            format_lcv_segments(&reference)
                        )));
                    }
                    view_index.canonical.insert(canonical, key.clone());
                    view_index.exact.insert(
                        key,
                        ValidatedLibraryView {
                            reference,
                            view_type: view.view_type,
                        },
                    );
                }
            }
        }

        if let Some(message) = first_invalid_name {
            return Err(ProjectIoError::InvalidData(message));
        }
        Ok(view_index)
    }

    fn validate_project_source_owners(
        &self,
        view_index: &ValidatedLibraryIndex,
    ) -> Result<(), ProjectIoError> {
        let mut owned_veriloga_views = HashSet::new();
        for bundle in self.workspace.project_sources.iter_bundles() {
            let crate::state::ProjectSourceOwner::CellView { reference } = bundle.owner() else {
                continue;
            };
            let view = view_index.get_owner(reference).ok_or_else(|| {
                ProjectIoError::InvalidData(format!(
                    "project source bundle {} owns missing cell view '{}'",
                    bundle.id(),
                    reference.key()
                ))
            })?;
            if view.reference != *reference {
                return Err(ProjectIoError::InvalidData(format!(
                    "project source bundle {} owner '{}' does not match the canonical library/view identity '{}'",
                    bundle.id(),
                    reference.key(),
                    view.reference.key()
                )));
            }
            if view.view_type != ViewType::VerilogA {
                return Err(ProjectIoError::InvalidData(format!(
                    "project source bundle {} owns {} view '{}'; cell-owned Verilog-A source requires a Verilog-A view",
                    bundle.id(),
                    view.view_type.display_name(),
                    reference.key()
                )));
            }
            owned_veriloga_views.insert(canonical_cell_view_owner_key(
                &reference.library,
                &reference.cell,
                &reference.view,
            ));
        }
        for view in view_index.exact.values() {
            if view.view_type != ViewType::VerilogA {
                continue;
            }
            let owner_key = canonical_cell_view_owner_key(
                &view.reference.library,
                &view.reference.cell,
                &view.reference.view,
            );
            if !owned_veriloga_views.contains(&owner_key) {
                return Err(ProjectIoError::InvalidData(format!(
                    "Verilog-A cell view '{}' has no project source bundle; every Verilog-A view must own exactly one persisted source closure",
                    view.reference.key()
                )));
            }
        }
        Ok(())
    }

    fn validate_workspace_references(
        &self,
        view_index: &ValidatedLibraryIndex,
    ) -> Result<(), ProjectIoError> {
        let mut required_schematic_buffers = HashSet::new();

        validate_lcv_name(
            "workspace.project.root_library",
            &self.workspace.project.root_library,
        )?;
        validate_lcv_name(
            "workspace.project.top_cell",
            &self.workspace.project.top_cell,
        )?;
        let root_library = self
            .libraries
            .get_library(&self.workspace.project.root_library)
            .ok_or_else(|| {
                ProjectIoError::InvalidData(format!(
                    "workspace.project.root_library '{}' was not found",
                    self.workspace.project.root_library
                ))
            })?;
        if root_library
            .get_cell(&self.workspace.project.top_cell)
            .is_none()
        {
            return Err(ProjectIoError::InvalidData(format!(
                "workspace.project.top_cell '{}' was not found in root library '{}'",
                self.workspace.project.top_cell, self.workspace.project.root_library
            )));
        }

        for (index, configuration) in self
            .workspace
            .configuration_sets
            .configurations()
            .iter()
            .enumerate()
        {
            let view_type = self.validate_library_reference(
                &format!("workspace.configuration_sets.configurations[{index}].root"),
                configuration.root(),
            )?;
            if !matches!(view_type, ViewType::Schematic | ViewType::Testbench) {
                return Err(ProjectIoError::InvalidData(format!(
                    "workspace.configuration_sets.configurations[{index}].root references {} view '{}'; a simulation root must be a schematic or testbench",
                    view_type.display_name(),
                    configuration.root().key()
                )));
            }
            required_schematic_buffers.insert(configuration.root().key());
        }

        let active_view_type =
            self.validate_library_reference("workspace.active_view", &self.workspace.active_view)?;
        if !self
            .workspace
            .open_views
            .iter()
            .any(|open_view| open_view.reference == self.workspace.active_view)
        {
            return Err(ProjectIoError::InvalidData(format!(
                "workspace.active_view references '{}', but workspace.open_views does not contain it",
                self.workspace.active_key()
            )));
        }
        if project_view_requires_schematic_buffer(active_view_type) {
            required_schematic_buffers.insert(self.workspace.active_key());
        }
        let mut open_view_keys = HashSet::new();
        for (index, open_view) in self.workspace.open_views.iter().enumerate() {
            let open_key = open_view.reference.key();
            if !open_view_keys.insert(open_key.clone()) {
                return Err(ProjectIoError::InvalidData(format!(
                    "workspace.open_views contains duplicate cell-view key '{open_key}' at index {index}"
                )));
            }
            let library_view_type = self.validate_library_reference(
                &format!("workspace.open_views[{index}]"),
                &open_view.reference,
            )?;
            if library_view_type != open_view.view_type {
                return Err(ProjectIoError::InvalidData(format!(
                    "workspace.open_views[{index}] references '{}', but persisted view type '{}' does not match library view type '{}'",
                    open_view.reference.key(),
                    open_view.view_type.display_name(),
                    library_view_type.display_name()
                )));
            }
            if project_view_requires_schematic_buffer(open_view.view_type) {
                required_schematic_buffers.insert(open_view.reference.key());
            }
        }
        for (index, reference) in self.workspace.hierarchy_stack.iter().enumerate() {
            self.validate_library_reference(
                &format!("workspace.hierarchy_stack[{index}]"),
                reference,
            )?;
        }

        let mut schematic_buffer_keys = self
            .workspace
            .schematic_buffers
            .keys()
            .map(String::as_str)
            .collect::<Vec<_>>();
        schematic_buffer_keys.sort_unstable();
        for key in schematic_buffer_keys {
            let Some(schematic) = self.workspace.schematic_buffers.get(key) else {
                return Err(ProjectIoError::InvalidData(format!(
                    "workspace schematic buffer '{key}' disappeared during validation"
                )));
            };
            schematic.validated_revisions.validate().map_err(|error| {
                ProjectIoError::InvalidData(format!(
                    "workspace schematic buffer '{key}' has invalid validated revision history: {error}"
                ))
            })?;
            let Some(view) = view_index.get_exact(key) else {
                let message = if persisted_lcv_key_is_well_formed(key) {
                    format!(
                        "workspace schematic/testbench buffer '{key}' is orphaned because no library view owns that key"
                    )
                } else {
                    format!(
                        "workspace schematic/testbench buffer key '{key}' is malformed; expected exactly three valid library/cell/view segments"
                    )
                };
                return Err(ProjectIoError::InvalidData(message));
            };
            if !project_view_requires_schematic_buffer(view.view_type) {
                return Err(ProjectIoError::InvalidData(format!(
                    "workspace schematic buffer '{key}' targets {} view {}, which cannot own schematic/testbench data",
                    view.view_type.display_name(),
                    format_lcv_segments(&view.reference)
                )));
            }
        }

        let mut required_schematic_buffers =
            required_schematic_buffers.into_iter().collect::<Vec<_>>();
        required_schematic_buffers.sort_unstable();
        for key in required_schematic_buffers {
            if !self.workspace.schematic_buffers.contains_key(&key) {
                return Err(ProjectIoError::InvalidData(format!(
                    "workspace references schematic/testbench buffer '{key}', but no backing buffer was found"
                )));
            }
        }
        Ok(())
    }

    fn validate_library_reference(
        &self,
        context: &str,
        reference: &CellViewRef,
    ) -> Result<ViewType, ProjectIoError> {
        for (field, value) in [
            ("library", reference.library.as_str()),
            ("cell", reference.cell.as_str()),
            ("view", reference.view.as_str()),
        ] {
            validate_lcv_name(&format!("{context}.{field}"), value)?;
        }
        let key = reference.key();
        let library = self
            .libraries
            .get_library(&reference.library)
            .ok_or_else(|| {
                ProjectIoError::InvalidData(format!(
                    "{context} references '{key}', but library '{}' was not found",
                    reference.library
                ))
            })?;
        let cell = library.get_cell(&reference.cell).ok_or_else(|| {
            ProjectIoError::InvalidData(format!(
                "{context} references '{key}', but cell '{}' was not found in library '{}'",
                reference.cell, reference.library
            ))
        })?;
        let view = cell.get_view(&reference.view).ok_or_else(|| {
            ProjectIoError::InvalidData(format!(
                "{context} references '{key}', but view '{}' was not found in cell '{}'",
                reference.view, reference.cell
            ))
        })?;
        Ok(view.view_type)
    }
}

fn record_invalid_lcv_name(first_error: &mut Option<String>, context: &str, value: &str) {
    if first_error.is_none()
        && let Err(error) = validate_cell_view_name_segment(value)
    {
        *first_error = Some(format!(
            "persisted {context} name '{value}' violates the cell-view name contract: {error}"
        ));
    }
}

fn validate_lcv_name(context: &str, value: &str) -> Result<(), ProjectIoError> {
    validate_cell_view_name_segment(value).map_err(|error| {
        ProjectIoError::InvalidData(format!(
            "persisted {context} name '{value}' violates the cell-view name contract: {error}"
        ))
    })
}

fn persisted_lcv_key_is_well_formed(key: &str) -> bool {
    let mut segments = key.split('/');
    let Some(library) = segments.next() else {
        return false;
    };
    let Some(cell) = segments.next() else {
        return false;
    };
    let Some(view) = segments.next() else {
        return false;
    };
    segments.next().is_none()
        && [library, cell, view]
            .into_iter()
            .all(|segment| validate_cell_view_name_segment(segment).is_ok())
}

fn format_lcv_segments(reference: &CellViewRef) -> String {
    format!(
        "(library={:?}, cell={:?}, view={:?})",
        reference.library, reference.cell, reference.view
    )
}

fn project_view_requires_schematic_buffer(view_type: ViewType) -> bool {
    matches!(view_type, ViewType::Schematic | ViewType::Testbench)
}

const LEGACY_SIMULATION_RESULTS_SCHEMA_VERSION: u32 = 1;
const STABLE_DATASET_RESULTS_SCHEMA_VERSION: u32 = 2;
const PREPARED_PROVENANCE_RESULTS_SCHEMA_VERSION: u32 = 3;
const EXPLICIT_PROVENANCE_MODE_RESULTS_SCHEMA_VERSION: u32 = 4;
const SOURCE_DOMAIN_RESULTS_SCHEMA_VERSION: u32 = 5;
const EXECUTION_IDENTITY_RESULTS_SCHEMA_VERSION: u32 = 6;
const FAMILY_METADATA_RESULTS_SCHEMA_VERSION: u32 = 7;
const CONTENT_DIGEST_RESULTS_SCHEMA_VERSION: u32 = 8;
const TYPED_PAYLOAD_RESULTS_SCHEMA_VERSION: u32 = 9;
const RELIABILITY_SOA_RESULTS_SCHEMA_VERSION: u32 = 10;
const TRANSFER_FUNCTION_RESULTS_SCHEMA_VERSION: u32 = 11;
const PROJECT_SIMULATION_RESULTS_SCHEMA_VERSION: u32 = 12;

const LEGACY_PROJECT_ID_NAMESPACE: uuid::Uuid =
    uuid::Uuid::from_u128(0x63a2_4271_a7cb_5a5e_b8bb_e783_e768_daf0);
const LEGACY_RESULT_RUN_ID_NAMESPACE: uuid::Uuid =
    uuid::Uuid::from_u128(0xe515_12ea_10c0_58c8_8bd7_ea31_003f_f6cf);
const LEGACY_RESULT_DATASET_ID_NAMESPACE: uuid::Uuid =
    uuid::Uuid::from_u128(0xa697_7219_0a25_536d_8dde_4319_08e4_d0c7);

fn default_simulation_results_schema_version() -> u32 {
    // A present result-history object without a version predates the stable-ID
    // schema. New objects set the current version explicitly through
    // `Default`/`from_state`.
    LEGACY_SIMULATION_RESULTS_SCHEMA_VERSION
}

fn analysis_type_key(analysis_type: AnalysisType) -> &'static str {
    match analysis_type {
        AnalysisType::DcOp => "DcOp",
        AnalysisType::DcSweep => "DcSweep",
        AnalysisType::Ac => "Ac",
        AnalysisType::Disto => "Disto",
        AnalysisType::Transient => "Transient",
        AnalysisType::Noise => "Noise",
        AnalysisType::PoleZero => "PoleZero",
        AnalysisType::Tf => "Tf",
        AnalysisType::Sensitivity => "Sensitivity",
        AnalysisType::Pac => "Pac",
        AnalysisType::Pnoise => "Pnoise",
        AnalysisType::Pxf => "Pxf",
        AnalysisType::Pstb => "Pstb",
        AnalysisType::Stb => "Stb",
        AnalysisType::MonteCarlo => "MonteCarlo",
        AnalysisType::Parametric => "Parametric",
        AnalysisType::Corner => "Corner",
        AnalysisType::Reliability => "Reliability",
        AnalysisType::Optimization => "Optimization",
        AnalysisType::Soa => "Soa",
        AnalysisType::SParameter => "SParameter",
        AnalysisType::Envelope => "Envelope",
        AnalysisType::Fourier => "Fourier",
        AnalysisType::HarmonicBalance => "HarmonicBalance",
        AnalysisType::Pss => "Pss",
        AnalysisType::Qpss => "Qpss",
        AnalysisType::Hbsp => "Hbsp",
        AnalysisType::Hbnoise => "Hbnoise",
        AnalysisType::Psp => "Psp",
        AnalysisType::Qpac => "Qpac",
        AnalysisType::Qpnoise => "Qpnoise",
        AnalysisType::Qpxf => "Qpxf",
        AnalysisType::TransientNoise => "TransientNoise",
        AnalysisType::DcMismatch => "DcMismatch",
    }
}

fn analysis_type_from_key(key: &str) -> Option<AnalysisType> {
    match key {
        "DcOp" | ".op" => Some(AnalysisType::DcOp),
        "DcSweep" | ".dc" => Some(AnalysisType::DcSweep),
        "Ac" | ".ac" => Some(AnalysisType::Ac),
        "Disto" | ".disto" => Some(AnalysisType::Disto),
        "Transient" | ".tran" => Some(AnalysisType::Transient),
        "Noise" | ".noise" => Some(AnalysisType::Noise),
        "PoleZero" | ".pz" => Some(AnalysisType::PoleZero),
        "Tf" | ".tf" => Some(AnalysisType::Tf),
        "Sensitivity" | ".sens" => Some(AnalysisType::Sensitivity),
        "Pac" | ".pac" => Some(AnalysisType::Pac),
        "Pnoise" | ".pnoise" => Some(AnalysisType::Pnoise),
        "Pxf" | ".pxf" => Some(AnalysisType::Pxf),
        "Pstb" | ".pstb" => Some(AnalysisType::Pstb),
        "Stb" | ".stb" => Some(AnalysisType::Stb),
        "MonteCarlo" | ".mc" => Some(AnalysisType::MonteCarlo),
        "Parametric" | ".step" => Some(AnalysisType::Parametric),
        "Corner" => Some(AnalysisType::Corner),
        "Reliability" | ".reliability" => Some(AnalysisType::Reliability),
        "Optimization" | ".opt" => Some(AnalysisType::Optimization),
        "Soa" | ".soa" => Some(AnalysisType::Soa),
        "SParameter" | ".sp" => Some(AnalysisType::SParameter),
        "Envelope" | ".envlp" => Some(AnalysisType::Envelope),
        "Fourier" | ".four" => Some(AnalysisType::Fourier),
        "HarmonicBalance" | ".hb" => Some(AnalysisType::HarmonicBalance),
        "Pss" | ".pss" => Some(AnalysisType::Pss),
        "Qpss" | ".qpss" => Some(AnalysisType::Qpss),
        "Hbsp" | ".hbsp" => Some(AnalysisType::Hbsp),
        "Hbnoise" | ".hbnoise" => Some(AnalysisType::Hbnoise),
        "Psp" | ".psp" => Some(AnalysisType::Psp),
        "Qpac" | ".qpac" => Some(AnalysisType::Qpac),
        "Qpnoise" | ".qpnoise" => Some(AnalysisType::Qpnoise),
        "Qpxf" | ".qpxf" => Some(AnalysisType::Qpxf),
        "TransientNoise" | ".tnoise" => Some(AnalysisType::TransientNoise),
        "DcMismatch" | ".dcmatch" => Some(AnalysisType::DcMismatch),
        _ => None,
    }
}

const fn analysis_kind_tag_for_plan_kind(kind: AnalysisKind) -> u8 {
    match kind {
        AnalysisKind::OperatingPoint => 0,
        AnalysisKind::DcSweep => 1,
        AnalysisKind::Ac => 2,
        AnalysisKind::Disto => 4,
        AnalysisKind::Transient => 5,
        AnalysisKind::Noise => 6,
        AnalysisKind::Pss => 7,
        AnalysisKind::HarmonicBalance => 8,
        AnalysisKind::TransferFunction => 9,
        AnalysisKind::Sensitivity => 10,
        AnalysisKind::PoleZero => 11,
        AnalysisKind::Pac => 12,
        AnalysisKind::Pnoise => 13,
        AnalysisKind::Pxf => 14,
        AnalysisKind::Pstb => 15,
        AnalysisKind::Stb => 16,
        AnalysisKind::MonteCarlo => 17,
        AnalysisKind::Temperature => 18,
        AnalysisKind::Corner => 19,
        AnalysisKind::Reliability => 20,
        AnalysisKind::Optimization => 21,
        AnalysisKind::Soa => 22,
        AnalysisKind::SParameter => 23,
        AnalysisKind::Envelope => 24,
        AnalysisKind::Fourier => 25,
        AnalysisKind::Qpss => 26,
        AnalysisKind::Hbsp => 27,
        AnalysisKind::Hbnoise => 28,
        AnalysisKind::Psp => 29,
        AnalysisKind::Qpac => 30,
        AnalysisKind::Qpnoise => 31,
        AnalysisKind::Qpxf => 32,
        AnalysisKind::TransientNoise => 33,
        AnalysisKind::DcMismatch => 34,
    }
}

fn default_true() -> bool {
    true
}

fn require_finite(value: f64, field: &str) -> Result<(), String> {
    if value.is_finite() {
        Ok(())
    } else {
        Err(format!("{field} is not finite"))
    }
}

fn require_optional_finite(value: Option<f64>, field: &str) -> Result<(), String> {
    if let Some(value) = value {
        require_finite(value, field)?;
    }
    Ok(())
}

fn require_finite_values(values: &[f64], field: &str) -> Result<(), String> {
    for (idx, value) in values.iter().enumerate() {
        if !value.is_finite() {
            return Err(format!("{field}[{idx}] is not finite"));
        }
    }
    Ok(())
}

fn require_monotonic_non_decreasing(values: &[f64], field: &str) -> Result<(), String> {
    for (idx, pair) in values.windows(2).enumerate() {
        if pair[1] < pair[0] {
            return Err(format!(
                "{field} must be monotonic non-decreasing; sample {} ({}) is less than sample {} ({})",
                idx + 1,
                pair[1],
                idx,
                pair[0]
            ));
        }
    }
    Ok(())
}

fn require_static_label(value: &str, field: &str) -> Result<(), String> {
    if known_static_label(value).is_some() {
        Ok(())
    } else {
        Err(format!("{field} has unknown static label '{value}'"))
    }
}

fn intern_static_label(value: String) -> &'static str {
    known_static_label(&value).unwrap_or("unknown")
}

fn known_static_label(value: &str) -> Option<&'static str> {
    match value {
        "MOSFET" => "MOSFET",
        "BSIM3" => "BSIM3",
        "BSIM4" => "BSIM4",
        "BJT" => "BJT",
        "DIODE" => "DIODE",
        "JFET" => "JFET",
        "MESFET" => "MESFET",
        "id" => "id",
        "vgs" => "vgs",
        "vds" => "vds",
        "vbs" => "vbs",
        "vth" => "vth",
        "vdsat" => "vdsat",
        "gm" => "gm",
        "gds" => "gds",
        "gmb" => "gmb",
        "gmbs" => "gmbs",
        "ic" => "ic",
        "ib" => "ib",
        "vbe" => "vbe",
        "vce" => "vce",
        "beta" => "beta",
        "vd" => "vd",
        "gd" => "gd",
        "igs" => "igs",
        "igd" => "igd",
        "saturation" => "saturation",
        "linear" => "linear",
        "cutoff" => "cutoff",
        "triode" => "triode",
        "subthreshold" => "subthreshold",
        "active" => "active",
        "reverse" => "reverse",
        "forward" => "forward",
        "thermal" => "thermal",
        "flicker" => "flicker",
        "shot" => "shot",
        "burst" => "burst",
        "white" => "white",
        "table" => "table",
        _ => return None,
    }
    .into()
}

#[derive(Debug, Clone)]
pub enum ProjectIoError {
    Cancelled,
    NotFound(PathBuf),
    IncompatibleVersion {
        file_version: String,
        app_version: String,
    },
    ParseError(String),
    SerializeError(String),
    InvalidData(String),
    Io(String),
}

impl std::fmt::Display for ProjectIoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Cancelled => f.write_str("Operation cancelled"),
            Self::NotFound(path) => write!(f, "Project file not found: {}", path.display()),
            Self::IncompatibleVersion {
                file_version,
                app_version,
            } => write!(
                f,
                "Project version {} is not compatible with app version {}",
                file_version, app_version
            ),
            Self::ParseError(error) => write!(f, "Project parse error: {}", error),
            Self::SerializeError(error) => write!(f, "Project serialize error: {}", error),
            Self::InvalidData(error) => write!(f, "Project data error: {}", error),
            Self::Io(error) => write!(f, "Project I/O error: {}", error),
        }
    }
}

impl std::error::Error for ProjectIoError {}

impl From<std::io::Error> for ProjectIoError {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error.to_string())
    }
}

pub const PROJECT_FILTER: (&str, &[&str]) = ("RSpice Project", &["rspiceproj", "json"]);
/// Hard boundary for one project container. It prevents corrupt or hostile
/// files from causing unbounded allocation while retaining ample headroom for
/// large retained result histories.
pub const MAX_PROJECT_FILE_BYTES: u64 = 512 * 1024 * 1024;
const MAX_LEGACY_PROJECT_FILE_BYTES: u64 = 64 * 1024 * 1024;

#[cfg(not(target_arch = "wasm32"))]
pub fn show_open_project_dialog() -> Result<PathBuf, ProjectIoError> {
    rfd::FileDialog::new()
        .add_filter(PROJECT_FILTER.0, PROJECT_FILTER.1)
        .add_filter("All Files", &["*"])
        .set_title("Open RSpice Project")
        .pick_file()
        .ok_or(ProjectIoError::Cancelled)
}

#[cfg(not(target_arch = "wasm32"))]
pub fn show_save_project_dialog(default_name: Option<&str>) -> Result<PathBuf, ProjectIoError> {
    let mut dialog = rfd::FileDialog::new()
        .add_filter(PROJECT_FILTER.0, PROJECT_FILTER.1)
        .set_title("Save RSpice Project");

    dialog = dialog.set_file_name(default_name.unwrap_or("untitled.rspiceproj"));

    let mut path = dialog.save_file().ok_or(ProjectIoError::Cancelled)?;
    let has_extension = path
        .extension()
        .and_then(|ext| ext.to_str())
        .is_some_and(|ext| ext.eq_ignore_ascii_case("rspiceproj"));
    if !has_extension {
        path.set_extension("rspiceproj");
    }
    Ok(path)
}

#[cfg(target_arch = "wasm32")]
pub fn show_open_project_dialog() -> Result<PathBuf, ProjectIoError> {
    Err(ProjectIoError::Io(
        "Use the browser project import workflow for web file selection".to_string(),
    ))
}

#[cfg(target_arch = "wasm32")]
pub fn show_save_project_dialog(default_name: Option<&str>) -> Result<PathBuf, ProjectIoError> {
    Ok(suggested_project_save_path(default_name))
}

/// Legacy create-only project export.
///
/// Canonical Save/Save As operations must use the project lifecycle so they
/// retain exact persistence identity, overwrite authorization, and recovery
/// semantics. This compatibility entry point deliberately refuses to replace
/// an existing pathname; it cannot regain unconditional overwrite authority if
/// a production caller reappears.
#[deprecated(
    since = "0.1.0",
    note = "use the project lifecycle persistence API; this compatibility function is create-only"
)]
pub fn save_project_file(project: &ProjectFile, path: &Path) -> Result<(), ProjectIoError> {
    let contents = serialize_project_file(project)?;

    #[cfg(target_arch = "wasm32")]
    {
        crate::workbench::browser::download::download_text_file(path, &contents)
            .map_err(ProjectIoError::Io)?;
        Ok(())
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        crate::io::durable_file::compare_exchange_bytes(
            path,
            crate::io::durable_file::ExpectedContent::Missing,
            contents.as_bytes(),
        )
        .map_err(|error| {
            ProjectIoError::Io(format!(
                "legacy create-only project export could not publish '{}': {error}",
                path.display()
            ))
        })
    }
}

#[cfg(any(test, target_arch = "wasm32"))]
pub(crate) fn suggested_project_save_path(default_name: Option<&str>) -> PathBuf {
    let mut path = PathBuf::from(
        default_name
            .filter(|name| !name.trim().is_empty())
            .unwrap_or("untitled.rspiceproj"),
    );
    crate::workbench::workflows::file_actions::ensure_file_extension(&mut path, "rspiceproj");
    path
}

pub(crate) fn serialize_project_file(project: &ProjectFile) -> Result<String, ProjectIoError> {
    project.validate()?;
    project
        .simulation_results
        .validate()
        .map_err(ProjectIoError::InvalidData)?;
    project
        .validate_result_plan_references()
        .map_err(ProjectIoError::InvalidData)?;
    project
        .validate_regression_baseline_eligibility()
        .map_err(ProjectIoError::InvalidData)?;
    let mut contents = serde_json::to_string_pretty(project)
        .map_err(|error| ProjectIoError::SerializeError(error.to_string()))?;
    contents.push('\n');
    if contents.len() as u64 > MAX_PROJECT_FILE_BYTES {
        return Err(ProjectIoError::InvalidData(format!(
            "serialized project is {} bytes; the supported maximum is {MAX_PROJECT_FILE_BYTES} bytes",
            contents.len()
        )));
    }
    Ok(contents)
}

pub(crate) fn load_project_text(
    contents: &str,
    source_path: Option<&Path>,
) -> Result<ProjectFile, ProjectIoError> {
    validate_project_text_size(contents.len())?;
    let load_route = project_text_load_route(contents)?;
    let mut project: ProjectFile = match load_route {
        ProjectTextLoadRoute::Direct => serde_json::from_str(contents)
            .map_err(|error| ProjectIoError::ParseError(error.to_string()))?,
        ProjectTextLoadRoute::LegacyProjectIdInjection => {
            validate_legacy_project_text_size(contents.len())?;
            let mut value: serde_json::Value = serde_json::from_str(contents)
                .map_err(|error| ProjectIoError::ParseError(error.to_string()))?;
            if let Some(descriptor) = value
                .get_mut("workspace")
                .and_then(|workspace| workspace.get_mut("project"))
                .and_then(serde_json::Value::as_object_mut)
            {
                let legacy_id =
                    ProjectId::from_namespace(LEGACY_PROJECT_ID_NAMESPACE, contents.as_bytes());
                descriptor.insert(
                    "id".to_owned(),
                    serde_json::Value::String(legacy_id.to_string()),
                );
            }
            serde_json::from_value(value)
                .map_err(|error| ProjectIoError::ParseError(error.to_string()))?
        }
    };
    let project_id = project.workspace.project.id();
    if let Some(context) = &mut project.execution_context {
        context.migrate_to_current(project_id).map_err(|error| {
            ProjectIoError::InvalidData(format!("execution context migration failed: {error}"))
        })?;
        if let Some(plan) = &context.simulation_plan.analysis_plan {
            project.workspace.migrate_active_plan_data(plan.id());
        }
        for plan in context.simulation_plan.inactive_plans() {
            project.workspace.migrate_inactive_plan_data(plan.id());
        }
    }
    project.validate()?;
    let mut simulation_results_error = project
        .simulation_results
        .migrate_to_current(project_id)
        .err()
        .or_else(|| project.simulation_results.validate().err());
    if simulation_results_error.is_none() {
        simulation_results_error = project.validate_result_plan_references().err();
    }
    if let Some(error) = simulation_results_error {
        project.simulation_results = ProjectSimulationResults::default();
        let cleared = project.clear_regression_baseline_references();
        project.simulation_results_warning = Some(format!(
            "Simulation results were not restored because their persisted data is invalid: {error}. Cleared {cleared} regression baseline reference(s) that could no longer be authenticated."
        ));
    } else if let Err(error) = project.validate_regression_baseline_eligibility() {
        let cleared = project.clear_regression_baseline_references();
        project.simulation_results_warning = Some(format!(
            "Retained simulation results were restored, but {cleared} regression baseline reference(s) were cleared because the persisted selection is not eligible: {error}"
        ));
    }
    match source_path {
        Some(path) => project.workspace.project.set_path(path.to_path_buf()),
        None => project.workspace.project.path = None,
    }
    Ok(project)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ProjectTextLoadRoute {
    Direct,
    LegacyProjectIdInjection,
}

#[derive(Deserialize, Default)]
struct ProjectTextLoadProbe {
    #[serde(default)]
    workspace: PersistedField<ProjectTextWorkspaceProbe>,
}

#[derive(Deserialize, Default)]
struct ProjectTextWorkspaceProbe {
    #[serde(default)]
    project: PersistedField<ProjectTextDescriptorProbe>,
}

#[derive(Deserialize, Default)]
struct ProjectTextDescriptorProbe {
    #[serde(default)]
    schema_version: PersistedField<serde::de::IgnoredAny>,
    #[serde(default)]
    id: PersistedField<serde::de::IgnoredAny>,
}

fn project_text_load_route(contents: &str) -> Result<ProjectTextLoadRoute, ProjectIoError> {
    let probe: ProjectTextLoadProbe = serde_json::from_str(contents)
        .map_err(|error| ProjectIoError::ParseError(error.to_string()))?;
    let legacy = probe
        .workspace
        .as_ref()
        .and_then(|workspace| workspace.project.as_ref())
        .is_some_and(|descriptor| {
            descriptor.schema_version.is_missing() && descriptor.id.is_missing()
        });
    Ok(if legacy {
        ProjectTextLoadRoute::LegacyProjectIdInjection
    } else {
        ProjectTextLoadRoute::Direct
    })
}

fn validate_project_text_size(byte_len: usize) -> Result<(), ProjectIoError> {
    if byte_len as u64 > MAX_PROJECT_FILE_BYTES {
        return Err(ProjectIoError::InvalidData(format!(
            "project text is {} bytes; the supported maximum is {MAX_PROJECT_FILE_BYTES} bytes",
            byte_len
        )));
    }
    Ok(())
}

fn validate_legacy_project_text_size(byte_len: usize) -> Result<(), ProjectIoError> {
    if byte_len as u64 > MAX_LEGACY_PROJECT_FILE_BYTES {
        return Err(ProjectIoError::InvalidData(format!(
            "legacy project text is {} bytes; migration requiring identity injection is limited to {MAX_LEGACY_PROJECT_FILE_BYTES} bytes",
            byte_len
        )));
    }
    Ok(())
}

pub fn load_project_file(path: &Path) -> Result<ProjectFile, ProjectIoError> {
    load_project_file_with_digest(path).map(|(project, _)| project)
}

/// Read, hash, parse, migrate, and validate one immutable byte snapshot.
/// The returned digest is therefore the exact persistence identity accepted
/// by the caller, not a later re-read that could race an external editor.
pub(crate) fn load_project_file_with_digest(
    path: &Path,
) -> Result<(ProjectFile, ContentDigest), ProjectIoError> {
    if !path.exists() {
        return Err(ProjectIoError::NotFound(path.to_path_buf()));
    }

    let (bytes, digest) = read_project_bytes_and_digest(path)?;
    let contents = std::str::from_utf8(&bytes).map_err(|error| {
        ProjectIoError::ParseError(format!("project is not valid UTF-8: {error}"))
    })?;
    let project = load_project_text(contents, Some(path))?;
    Ok((project, digest))
}

/// Read one bounded byte snapshot and parse it only when it matches an exact
/// previously accepted persistence identity.
///
/// Session restoration uses this boundary so a file replaced at a remembered
/// pathname cannot become parser input, much less regain canonical Save
/// authority. A mismatch is an ordinary `Ok(None)` conflict; malformed bytes
/// are reported only when they carry the expected digest.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn load_project_file_with_expected_digest(
    path: &Path,
    expected: ContentDigest,
) -> Result<Option<ProjectFile>, ProjectIoError> {
    if !path.exists() {
        return Err(ProjectIoError::NotFound(path.to_path_buf()));
    }

    let (bytes, digest) = read_project_bytes_and_digest(path)?;
    if digest != expected {
        return Ok(None);
    }
    let contents = std::str::from_utf8(&bytes).map_err(|error| {
        ProjectIoError::ParseError(format!("project is not valid UTF-8: {error}"))
    })?;
    load_project_text(contents, Some(path)).map(Some)
}

fn read_project_bytes_and_digest(path: &Path) -> Result<(Vec<u8>, ContentDigest), ProjectIoError> {
    let file = File::open(path)?;
    let advertised = file.metadata()?.len();
    if advertised > MAX_PROJECT_FILE_BYTES {
        return Err(ProjectIoError::InvalidData(format!(
            "project is {advertised} bytes; the supported maximum is {MAX_PROJECT_FILE_BYTES} bytes"
        )));
    }
    let mut file = file;
    let mut bytes = Vec::with_capacity(advertised.min(8 * 1024 * 1024) as usize);
    let mut hasher = Sha256::new();
    let mut buffer = [0_u8; 64 * 1024];
    let mut total = 0_u64;
    loop {
        let read = file.read(&mut buffer)?;
        if read == 0 {
            break;
        }
        total = total.saturating_add(read as u64);
        if total > MAX_PROJECT_FILE_BYTES {
            return Err(ProjectIoError::InvalidData(format!(
                "project grew beyond the supported {MAX_PROJECT_FILE_BYTES} byte maximum while it was being read"
            )));
        }
        hasher.update(&buffer[..read]);
        bytes.extend_from_slice(&buffer[..read]);
    }
    Ok((bytes, ContentDigest::from_bytes(hasher.finalize().into())))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{
        AnalysisResult, AnalysisType, CellViewRef, OpenCellView, OperatingPointValue,
        PreparedRunReceipt, PreparedRunTaskReceipt, PreparedSourceCheckReceipt, SimulationRun,
        SimulationRunProvenance, SimulationState, WaveformData,
    };

    #[test]
    fn in_memory_project_text_is_size_checked_before_parsing() {
        assert!(validate_project_text_size(MAX_PROJECT_FILE_BYTES as usize).is_ok());
        let error = validate_project_text_size(MAX_PROJECT_FILE_BYTES as usize + 1)
            .expect_err("oversized project text is rejected");
        assert!(matches!(error, ProjectIoError::InvalidData(_)));
        assert!(error.to_string().contains("supported maximum"));
        assert!(validate_legacy_project_text_size(MAX_LEGACY_PROJECT_FILE_BYTES as usize).is_ok());
        let legacy_error =
            validate_legacy_project_text_size(MAX_LEGACY_PROJECT_FILE_BYTES as usize + 1)
                .expect_err("oversized legacy materialization is rejected");
        assert!(matches!(legacy_error, ProjectIoError::InvalidData(_)));
        assert!(legacy_error.to_string().contains("identity injection"));
    }

    #[test]
    fn current_project_text_routes_to_direct_deserialization() {
        let mut libraries = LibraryManager::with_primitives();
        let workspace = ProjectWorkspace::new_bootstrapped(&mut libraries);
        let project = ProjectFile::new(workspace, libraries);
        let json = serialize_project_file(&project).expect("current project serializes");

        assert_eq!(
            project_text_load_route(&json).expect("current route probes"),
            ProjectTextLoadRoute::Direct
        );
        assert_eq!(
            project_text_load_route(
                r#"{"workspace":{"project":{"schema_version":null,"id":null}}}"#
            )
            .expect("present null fields still probe"),
            ProjectTextLoadRoute::Direct,
            "legacy ID injection is permitted only when both keys are absent"
        );
    }

    fn seal_legacy_unattributed(run: &mut SimulationRun) {
        run.restore_provenance(SimulationRunProvenance::LegacyUnattributed)
            .expect("legacy fixture seals explicitly");
    }

    fn operating_point_payload_fixture() -> AnalysisResultPayload {
        AnalysisResultPayload::OperatingPoint {
            temperature_mode: crate::state::OperatingPointTemperatureEvidence::PvtRunSet,
            temperature_celsius: 27.0,
            initial_guess: crate::state::OperatingPointInitialGuessEvidence::Automatic,
            node_initialization:
                crate::state::OperatingPointNodeInitializationEvidence::UseIcAndNodeset,
            homotopy: crate::state::OperatingPointHomotopyEvidence::Adaptive,
            annotation: crate::state::OperatingPointAnnotationEvidence::VoltagesAndCurrents,
            device_detail: crate::state::OperatingPointDeviceDetailEvidence::SelectedAndViolations,
            save_device_op: crate::state::OperatingPointSaveDeviceEvidence::Enabled,
            accuracy: crate::state::OperatingPointAccuracyEvidence::Balanced,
            selected_devices: Vec::new(),
            violation_devices: Vec::new(),
            violation_source_content_digest: None,
            validated_startup_directives: 0,
            mna_node_names: vec!["out".to_owned()],
            mna_branch_names: Vec::new(),
            mna_solution: vec![1.0],
            effective_source_content_digest: Some(ContentDigest::from_bytes([0x70; 32])),
            run_point_index: 0,
            run_point_count: 1,
            run_point_process: crate::state::OperatingPointProcessEvidence::TT,
            run_point_supply_voltage: None,
            run_point_nominal_supply_voltage: None,
        }
    }

    fn clear_v6_execution_fields(results: &mut ProjectSimulationResults) {
        for run in &mut results.runs {
            run.job_id = None;
            run.execution_target = None;
            run.lifecycle = None;
            run.dataset_content_digest = PersistedField::Missing;
            for analysis in &mut run.analyses {
                analysis.result_data_digest = PersistedField::Missing;
            }
        }
    }

    fn clear_v6_execution_fields_json(results: &mut serde_json::Value) {
        for run in results["runs"]
            .as_array_mut()
            .expect("simulation result run array")
        {
            let run = run.as_object_mut().expect("simulation result run object");
            run.remove("job_id");
            run.remove("execution_target");
            run.remove("lifecycle");
            run.remove("dataset_content_digest");
            for analysis in run
                .get_mut("analyses")
                .and_then(serde_json::Value::as_array_mut)
                .expect("simulation analysis array")
            {
                analysis
                    .as_object_mut()
                    .expect("simulation analysis object")
                    .remove("result_data_digest");
            }
        }
    }

    fn seal_prepared_run(
        run: &mut SimulationRun,
        source_domain: AnalysisResultSourceDomain,
        simulation_plan_id: Option<SimulationPlanId>,
        project_revision: ObjectRevision,
        source_content_digest: ContentDigest,
        source_check_receipt: PreparedSourceCheckReceipt,
        analysis_kind_tags: &[u8],
    ) {
        assert_eq!(run.analyses.len(), analysis_kind_tags.len());
        let prepared_snapshot_digest = run
            .analyses
            .first()
            .and_then(|analysis| analysis.provenance.as_ref())
            .expect("prepared fixture has provenance")
            .prepared_snapshot_digest();
        let tasks = run
            .analyses
            .iter()
            .zip(analysis_kind_tags)
            .enumerate()
            .map(|(index, (analysis, kind_tag))| {
                let provenance = analysis.provenance.as_ref().expect("prepared provenance");
                PreparedRunTaskReceipt::new(
                    provenance.source_instance_id(),
                    provenance.source_revision(),
                    provenance.dependency_ids().to_vec(),
                    *kind_tag,
                    ContentDigest::from_bytes([0xc0_u8.wrapping_add(index as u8); 32]),
                )
                .expect("prepared task receipt")
            })
            .collect::<Vec<_>>();
        let receipt = PreparedRunReceipt::new(
            source_domain,
            simulation_plan_id,
            project_revision,
            prepared_snapshot_digest,
            source_content_digest,
            source_check_receipt,
            tasks,
        )
        .expect("prepared run receipt");
        run.restore_provenance(SimulationRunProvenance::Prepared(receipt))
            .expect("prepared fixture seals explicitly");
    }

    #[test]
    fn prepared_run_receipt_round_trip_retains_exact_project_model_sources() {
        let source_id = ModelSourceId::new();
        let identity = PreparedModelSourceIdentity::new(
            source_id,
            "nch_receipt",
            ObjectRevision::INITIAL,
            ContentDigest::from_bytes([0x51; 32]),
        )
        .unwrap();
        let task = PreparedRunTaskReceipt::new(
            AnalysisInstanceId::new(),
            ObjectRevision::INITIAL,
            Vec::new(),
            2,
            ContentDigest::from_bytes([0x52; 32]),
        )
        .unwrap();
        let receipt = PreparedRunReceipt::new_with_project_model_sources(
            AnalysisResultSourceDomain::SimulationPlan,
            Some(SimulationPlanId::new()),
            ObjectRevision::INITIAL,
            ContentDigest::from_bytes([0x53; 32]),
            ContentDigest::from_bytes([0x54; 32]),
            PreparedSourceCheckReceipt::SchematicDrc(ContentDigest::from_bytes([0x55; 32])),
            vec![identity],
            vec![task],
        )
        .unwrap();

        let wire = ProjectPreparedRunReceipt::from(&receipt);
        let encoded = serde_json::to_string(&wire).unwrap();
        let decoded = serde_json::from_str::<ProjectPreparedRunReceipt>(&encoded).unwrap();
        let restored = decoded.into_receipt().unwrap();

        assert_eq!(restored.project_model_sources().len(), 1);
        let restored_source = &restored.project_model_sources()[0];
        assert_eq!(restored_source.source_id(), source_id);
        assert_eq!(restored_source.model_name(), "nch_receipt");
        assert_eq!(
            restored_source.content_digest(),
            ContentDigest::from_bytes([0x51; 32])
        );
    }

    fn project_with_execution_context() -> ProjectFile {
        use crate::workbench::simulation_analysis_tabs::{TAB_AC, TAB_NOISE, TAB_TRANSIENT};
        use crate::simulation::dialog::{DampingStrategy, IntegrationMethod, MatrixSolver};
        use crate::state::model_library::{
            DeviceModel, ModelLibrary, ModelLibraryManager, ModelType,
        };

        let mut design_libraries = LibraryManager::with_primitives();
        let workspace = ProjectWorkspace::new_bootstrapped(&mut design_libraries);

        let mut setup = crate::workbench::app_state::SimSetupState::new();
        setup.enabled.extend([TAB_AC, TAB_NOISE]);
        setup.analysis_order = vec![TAB_NOISE, TAB_TRANSIENT, TAB_AC];
        setup.listed.extend([TAB_AC, TAB_NOISE]);
        setup
            .set_reference_pvt(crate::simulation::dialog::corner::ProcessCorner::FF, -40.0)
            .expect("fixture PVT is valid");
        setup.tran.stop = "25u".to_owned();
        setup.tran.step = "2n".to_owned();
        setup.tran.start = "1u".to_owned();
        setup.tran.max_step = "100n".to_owned();
        setup.tran.uic = true;
        setup.ac.fstart = "10".to_owned();
        setup.ac.fstop = "8G".to_owned();
        setup.ac.points = "77".to_owned();
        setup.ac.sweep = 1;
        setup.noise.output = "vout".to_owned();
        setup.noise.reference = "vref".to_owned();
        setup.noise.input = "VIN".to_owned();
        setup.noise.fstart = "10".to_owned();
        setup.noise.fstop = "5G".to_owned();
        setup.disto_f2_over_f1 = "0.91".to_owned();
        setup.options.reltol = 2e-4;
        setup.options.residual_reltol = 3e-4;
        setup.options.vntol = 4e-7;
        setup.options.abstol = 5e-13;
        setup.options.iabstol = 6e-13;
        setup.options.chgtol = 7e-15;
        setup.options.pivrel = 8e-4;
        setup.options.pivtol = 9e-14;
        setup.options.itl1 = 80;
        setup.options.itl2 = 120;
        setup.options.itl4 = 12;
        setup.options.gmin_stepping = false;
        setup.options.source_stepping = false;
        setup.options.pseudo_transient = false;
        setup.options.arc_length = true;
        setup.options.gmin = 2e-12;
        setup.options.damping = DampingStrategy::Combined;
        setup.options.method = IntegrationMethod::Gear2Only;
        setup.options.solver = MatrixSolver::SparseLu;
        setup.options.bypass_enabled = true;
        setup.options.bypass_reltol = 5e-4;
        setup.options.bypass_abstol = 5e-7;
        setup.options.min_timestep = 2e-15;
        setup.options.max_timestep = 2e-3;
        setup.options.timestep_factor = 10.0;
        setup.options.tnom = 25.0;
        setup.options.verbose = true;
        setup.options.save_internals = true;

        let mut model_manager = ModelLibraryManager::new();
        let mut model_library = ModelLibrary::new("fixture_models");
        model_library.pdk_name = "fixture_pdk".to_owned();
        model_library.technology_node = "90nm".to_owned();
        model_library.version = "2.1".to_owned();
        let mut model = DeviceModel::new("nch_fixture", ModelType::Nmos);
        model.add_parameter("kp", 1.25e-3);
        model_library.add_model(model);
        model_manager.add_library(model_library);

        // Exercise the deterministic singleton-to-instance migration while
        // retaining every legacy fixture edit above.
        setup.analysis_plan = None;
        setup
            .migrate_legacy_analysis_plan(workspace.project.id())
            .expect("legacy execution fixture migrates at the load boundary");
        let execution_context = crate::io::ProjectExecutionContext::from_state(
            workspace.project.id(),
            &setup,
            &model_manager,
        )
        .expect("execution fixture validates");
        ProjectFile::new_with_execution_context(
            workspace,
            design_libraries,
            ProjectSimulationResults::default(),
            execution_context,
        )
    }

    fn serialized_analysis_instance_mut<'a>(
        project: &'a mut serde_json::Value,
        kind: &str,
    ) -> &'a mut serde_json::Value {
        project["execution_context"]["simulation_plan"]["analysis_plan"]["instances"]
            .as_array_mut()
            .expect("stable analysis instances serialize as an array")
            .iter_mut()
            .find(|instance| instance["kind"] == kind)
            .unwrap_or_else(|| panic!("fixture contains {kind} analysis"))
    }

    fn cell_source_bundle(reference: CellViewRef) -> crate::state::ProjectSourceBundle {
        crate::state::ProjectSourceBundle::try_new(
            crate::state::ProjectSourceOwner::cell_view(reference),
            crate::state::ProjectSourceLanguage::VerilogA,
            "behavior.va",
            "module behavior(p, n); inout p, n; endmodule",
            std::iter::empty(),
            std::iter::empty(),
        )
        .expect("valid cell source bundle")
    }

    #[test]
    fn project_validation_requires_cell_source_owner_to_be_an_exact_veriloga_view() {
        let mut valid = project_with_execution_context();
        let reference = CellViewRef::new(
            valid.workspace.project.root_library.clone(),
            valid.workspace.project.top_cell.clone(),
            "behavior",
        );
        valid
            .libraries
            .get_library_mut(&reference.library)
            .and_then(|library| library.get_cell_mut(&reference.cell))
            .expect("top cell")
            .add_view(crate::state::View::new(
                reference.view.as_str(),
                ViewType::VerilogA,
            ));
        valid
            .workspace
            .project_sources
            .insert_bundle(cell_source_bundle(reference.clone()))
            .expect("unique source owner");
        valid.validate().expect("exact Verilog-A owner is valid");

        let mut missing_source = valid.clone();
        missing_source.workspace.project_sources = Default::default();
        assert!(
            missing_source
                .validate()
                .expect_err("a Verilog-A view without its source must fail")
                .to_string()
                .contains("has no project source bundle")
        );

        let mut canonical_alias = valid.clone();
        canonical_alias.workspace.project_sources = Default::default();
        canonical_alias
            .workspace
            .project_sources
            .insert_bundle(cell_source_bundle(CellViewRef::new(
                reference.library.to_uppercase(),
                reference.cell.to_uppercase(),
                reference.view.to_uppercase(),
            )))
            .expect("registry accepts one canonical owner until tree validation");
        assert!(
            canonical_alias
                .validate()
                .expect_err("canonical aliases must retain exact library-tree spelling")
                .to_string()
                .contains("does not match the canonical library/view identity")
        );

        let mut missing = valid.clone();
        missing.workspace.project_sources = Default::default();
        missing
            .workspace
            .project_sources
            .insert_bundle(cell_source_bundle(CellViewRef::new(
                &reference.library,
                "missing_cell",
                "behavior",
            )))
            .expect("registry permits unresolved owner until project validation");
        assert!(
            missing
                .validate()
                .expect_err("missing owner must fail")
                .to_string()
                .contains("owns missing cell view")
        );

        let mut wrong_type = valid;
        wrong_type.workspace.project_sources = Default::default();
        let schematic = CellViewRef::new(
            &wrong_type.workspace.project.root_library,
            &wrong_type.workspace.project.top_cell,
            crate::state::workspace::DEFAULT_SCHEMATIC_VIEW,
        );
        wrong_type
            .workspace
            .project_sources
            .insert_bundle(cell_source_bundle(schematic))
            .expect("registry validates owner shape");
        assert!(
            wrong_type
                .validate()
                .expect_err("schematic owner must fail")
                .to_string()
                .contains("requires a Verilog-A view")
        );
    }

    #[test]
    fn expected_digest_gate_rejects_replaced_bytes_before_parsing() {
        let path = std::env::temp_dir().join(format!(
            "rspice-expected-project-digest-{}-{}.rspiceproj",
            std::process::id(),
            uuid::Uuid::new_v4()
        ));
        let project = project_with_execution_context();
        let contents = serialize_project_file(&project).expect("serialize fixture");
        std::fs::write(&path, contents).expect("write accepted fixture");
        let (_, accepted) =
            load_project_file_with_digest(&path).expect("load accepted fixture identity");

        assert!(
            load_project_file_with_expected_digest(&path, accepted)
                .expect("matching project loads")
                .is_some()
        );

        std::fs::write(&path, b"replacement bytes are intentionally not JSON")
            .expect("replace fixture");
        assert!(
            load_project_file_with_expected_digest(&path, accepted)
                .expect("digest mismatch is rejected without parsing")
                .is_none()
        );

        std::fs::remove_file(path).expect("remove isolated fixture");
    }

    #[test]
    fn suggested_project_save_path_defaults_and_enforces_extension() {
        assert_eq!(
            suggested_project_save_path(None),
            PathBuf::from("untitled.rspiceproj")
        );
        assert_eq!(
            suggested_project_save_path(Some("amp")),
            PathBuf::from("amp.rspiceproj")
        );
        assert_eq!(
            suggested_project_save_path(Some("amp.rspiceproj")),
            PathBuf::from("amp.rspiceproj")
        );
    }

    #[test]
    fn project_file_serializes_to_versioned_json() {
        let mut libraries = LibraryManager::with_primitives();
        let workspace = ProjectWorkspace::new_bootstrapped(&mut libraries);
        let project = ProjectFile::new(workspace, libraries);

        let json = serialize_project_file(&project).expect("project serializes");

        assert!(json.contains("\"version\""));
        assert!(json.contains("\"workspace\""));
        assert!(json.contains("\"libraries\""));
        assert!(json.ends_with('\n'));
    }

    #[test]
    fn project_file_round_trips_configuration_execution_authority() {
        let mut libraries = LibraryManager::with_primitives();
        let mut workspace = ProjectWorkspace::new_bootstrapped(&mut libraries);
        workspace
            .configuration_sets
            .create(crate::state::ConfigurationSetDefinition {
                name: "Browser-qualified release".to_owned(),
                root: workspace.active_view.clone(),
                dut_path: "/top/XDUT".to_owned(),
                executable_view_policy: vec!["schematic".to_owned(), "spice".to_owned()],
                stop_views: vec!["spice".to_owned()],
                unresolved_policy: crate::state::UnresolvedBindingPolicy::BlockNetlist,
                black_box_policy:
                    crate::state::ConfigurationBlackBoxPolicy::MaterializedSourceBoundariesOnly,
                overrides: vec![crate::state::ConfigurationSetOverride {
                    instance_path: "/top/XDUT/*".to_owned(),
                    executable_views: vec!["spice".to_owned()],
                    stop_view: Some("spice".to_owned()),
                    model_section: Some("tt".to_owned()),
                    eligible_platforms: vec![crate::state::ConfigurationPlatform::Browser],
                }],
                model_profile: crate::state::ConfigurationModelProfile::ProjectRunSetSections,
                owner: "Verification".to_owned(),
            })
            .expect("configuration fixture");
        let expected = workspace.configuration_sets.clone();
        let project = ProjectFile::new(workspace, libraries);

        let json = serialize_project_file(&project).expect("configuration project serializes");
        let loaded = load_project_text(&json, None).expect("configuration project loads");

        assert_eq!(loaded.workspace.configuration_sets, expected);
    }

    #[test]
    fn project_file_round_trips_design_management_authority() {
        let mut libraries = LibraryManager::with_primitives();
        let mut workspace = ProjectWorkspace::new_bootstrapped(&mut libraries);
        let owner = workspace.active_view.key();
        workspace
            .design_management
            .bootstrap_for_cell_view(&owner, "Main", [11, 12])
            .expect("design-management fixture");
        let expected = workspace.design_management.clone();
        let project = ProjectFile::new(workspace, libraries);

        let json = serialize_project_file(&project).expect("design project serializes");
        let loaded = load_project_text(&json, None).expect("design project loads");

        assert_eq!(loaded.workspace.design_management, expected);
        assert_eq!(
            loaded
                .workspace
                .design_management
                .semantic_digest()
                .expect("loaded semantic digest"),
            expected
                .semantic_digest()
                .expect("expected semantic digest")
        );
    }

    #[test]
    fn project_file_rejects_unsupported_design_management_schema() {
        let mut libraries = LibraryManager::with_primitives();
        let mut workspace = ProjectWorkspace::new_bootstrapped(&mut libraries);
        workspace
            .design_management
            .bootstrap_for_cell_view(&workspace.active_view.key(), "Main", [11])
            .expect("design-management fixture");
        let project = ProjectFile::new(workspace, libraries);
        let mut value = serde_json::to_value(project).expect("project JSON value");
        value["workspace"]["design_management"]["schema_version"] = serde_json::Value::from(999);
        let json = serde_json::to_string(&value).expect("malformed project JSON");

        let error = load_project_text(&json, None).expect_err("unsupported schema is rejected");
        assert!(error.to_string().contains("schema 999 is unsupported"));
    }

    #[test]
    fn project_file_round_trips_project_owned_report_documents() {
        use crate::results::report_document::{
            ReportDocument, ReportEdit, ReportPageUpdatePolicy, ReportTemplate,
        };

        let mut libraries = LibraryManager::with_primitives();
        let mut workspace = ProjectWorkspace::new_bootstrapped(&mut libraries);
        let mut report =
            ReportDocument::new_with_template("Verification report", ReportTemplate::DesignReview)
                .expect("report document");
        report
            .transact(
                report.revision(),
                vec![ReportEdit::AddPage {
                    title: "PVT and yield".to_owned(),
                }],
                10,
            )
            .expect("add page");
        let page = report.pages()[0].clone();
        report
            .transact(
                report.revision(),
                vec![ReportEdit::SetPageUpdatePolicy {
                    page_id: page.id(),
                    expected_page_revision: page.revision(),
                    update_policy: ReportPageUpdatePolicy::FreezeSelectedRevision,
                }],
                11,
            )
            .expect("set page policy");
        workspace.report_documents.push(report.clone());
        workspace.report_documents_dirty = true;
        let project = ProjectFile::new(workspace, libraries);

        let json = serialize_project_file(&project).expect("project serializes");
        let restored = load_project_text(&json, None).expect("project reloads");

        assert_eq!(restored.workspace.report_documents, vec![report]);
        let restored_report = &restored.workspace.report_documents[0];
        assert_eq!(restored_report.revision_history().records().len(), 3);
        assert_eq!(
            restored_report
                .reconstruct_revision(restored_report.id(), ObjectRevision::INITIAL)
                .expect("initial report source is reconstructable")
                .title(),
            "Verification report"
        );
        assert!(
            restored_report
                .reconstruct_revision(restored_report.id(), ObjectRevision::INITIAL)
                .expect("initial report source is reconstructable")
                .pages()
                .is_empty()
        );
        assert!(!restored.workspace.report_documents_dirty);
    }

    #[test]
    fn project_execution_context_round_trips_every_persisted_input() {
        let project = project_with_execution_context();
        let expected = serde_json::to_value(
            project
                .execution_context
                .as_ref()
                .expect("fixture has execution context"),
        )
        .expect("context serializes");

        let json = serialize_project_file(&project).expect("project serializes");
        let loaded = load_project_text(&json, None).expect("project reloads");
        let actual = serde_json::to_value(
            loaded
                .execution_context
                .as_ref()
                .expect("execution context restored"),
        )
        .expect("restored context serializes");

        assert_eq!(actual, expected);
        let plan = &actual["simulation_plan"];
        for retired in [
            "enabled",
            "analysis_order",
            "listed",
            "op",
            "tran",
            "ac",
            "noise",
        ] {
            assert!(
                plan.get(retired).is_none(),
                "retired field {retired} leaked"
            );
        }
        let instances = plan["analysis_plan"]["instances"]
            .as_array()
            .expect("stable instances serialize");
        assert_eq!(instances[0]["kind"], "noise");
        assert_eq!(instances[1]["kind"], "tran");
        assert_eq!(instances[2]["kind"], "ac");
        assert!(
            instances[..3]
                .iter()
                .all(|instance| instance["enabled"] == true)
        );
        assert!(plan.get("options_draft").is_none());
        assert!(plan.get("options_open").is_none());
        assert_eq!(actual["model_libraries"][0]["name"], "fixture_models");
        assert!(actual["model_libraries"][0].get("expanded").is_none());
    }

    #[test]
    fn legacy_project_without_execution_context_remains_compatible() {
        let mut libraries = LibraryManager::with_primitives();
        let workspace = ProjectWorkspace::new_bootstrapped(&mut libraries);
        let project = ProjectFile::new(workspace, libraries);
        let json = serialize_project_file(&project).expect("legacy-compatible project serializes");

        let loaded = load_project_text(&json, None).expect("legacy project opens");

        assert!(loaded.execution_context.is_none());
    }

    #[test]
    fn unversioned_execution_context_migrates_to_sorted_legacy_order() {
        let project = project_with_execution_context();
        let json = serialize_project_file(&project).expect("project serializes");
        let mut value: serde_json::Value = serde_json::from_str(&json).expect("valid JSON");
        let context = value["execution_context"]
            .as_object_mut()
            .expect("execution object");
        context.remove("schema_version");
        let simulation_plan = context["simulation_plan"]
            .as_object_mut()
            .expect("simulation plan");
        simulation_plan.remove("analysis_plan");
        simulation_plan.insert("enabled".to_owned(), serde_json::json!([4, 1, 2]));

        let loaded = load_project_text(
            &serde_json::to_string(&value).expect("fixture serializes"),
            None,
        )
        .expect("legacy context migrates");
        let context = loaded.execution_context.expect("context retained");

        assert_eq!(
            context.schema_version,
            crate::io::PROJECT_EXECUTION_CONTEXT_SCHEMA_VERSION
        );
        let enabled = context
            .simulation_plan
            .stable_analysis_plan()
            .expect("legacy singleton plan migrates")
            .instances()
            .iter()
            .filter(|instance| instance.enabled())
            .map(|instance| instance.kind())
            .collect::<Vec<_>>();
        assert_eq!(
            enabled,
            vec![
                crate::simulation::plan::AnalysisKind::Transient,
                crate::simulation::plan::AnalysisKind::Ac,
                crate::simulation::plan::AnalysisKind::Noise,
            ]
        );
    }

    #[test]
    fn malformed_execution_context_is_never_silently_defaulted() {
        let project = project_with_execution_context();
        let json = serialize_project_file(&project).expect("project serializes");
        let valid: serde_json::Value = serde_json::from_str(&json).expect("valid JSON");

        let mut future = valid.clone();
        future["execution_context"]["schema_version"] = serde_json::json!(999);
        let error = load_project_text(&future.to_string(), None)
            .expect_err("future schema must fail")
            .to_string();
        assert!(error.contains("unsupported execution-context schema version 999"));

        let mut duplicate_identity = valid.clone();
        let instances = duplicate_identity["execution_context"]["simulation_plan"]["analysis_plan"]
            ["instances"]
            .as_array_mut()
            .expect("stable instances");
        let first_id = instances[0]["id"].clone();
        instances[1]["id"] = first_id;
        let error = load_project_text(&duplicate_identity.to_string(), None)
            .expect_err("duplicate stable identity must fail")
            .to_string();
        assert!(error.contains("appears more than once"), "{error}");

        let mut mismatched_draft = valid.clone();
        serialized_analysis_instance_mut(&mut mismatched_draft, "noise")["kind"] =
            serde_json::json!("ac");
        let error = load_project_text(&mismatched_draft.to_string(), None)
            .expect_err("declared kind and draft kind must agree")
            .to_string();
        assert!(error.contains("declared as ac"), "{error}");

        let mut unsupported = valid.clone();
        serialized_analysis_instance_mut(&mut unsupported, "tran")["kind"] =
            serde_json::json!("future-analysis");
        let error = load_project_text(&unsupported.to_string(), None)
            .expect_err("unsupported stable analysis kind must fail")
            .to_string();
        assert!(
            error.contains("unknown variant `future-analysis`"),
            "{error}"
        );

        let mut mismatched_pvt = valid.clone();
        mismatched_pvt["execution_context"]["simulation_plan"]["options"]["temp"] =
            serde_json::json!(125.0);
        let error = load_project_text(&mismatched_pvt.to_string(), None)
            .expect_err("conflicting execution temperatures must fail")
            .to_string();
        assert!(error.contains("disagrees with solver option temp"));

        let mut unknown_input = valid.clone();
        serialized_analysis_instance_mut(&mut unknown_input, "tran")["draft"]["draft"]["future_mode"] =
            serde_json::json!(true);
        let error = load_project_text(&unknown_input.to_string(), None)
            .expect_err("unknown execution input must not be ignored")
            .to_string();
        assert!(error.contains("unknown field `future_mode`"));

        let mut invalid_model = valid;
        invalid_model["execution_context"]["model_libraries"][0]["selected_corner"] =
            serde_json::json!("missing");
        let error = load_project_text(&invalid_model.to_string(), None)
            .expect_err("invalid model binding must fail")
            .to_string();
        assert!(error.contains("selected_corner 'missing' does not exist"));

        let mut invalid_digest =
            serde_json::from_str::<serde_json::Value>(&json).expect("valid JSON");
        let absolute_source = std::env::temp_dir().join("rspice-digest-shape.lib");
        invalid_digest["execution_context"]["model_libraries"][0]["root_path"] =
            serde_json::to_value(&absolute_source).expect("path serializes");
        invalid_digest["execution_context"]["model_libraries"][0]["source_closure"] = serde_json::json!([{
            "path": absolute_source,
            "digest": "not-a-sha-256-digest"
        }]);
        let error = load_project_text(&invalid_digest.to_string(), None)
            .expect_err("malformed digest must fail")
            .to_string();
        assert!(error.contains("SHA-256 digest must contain 64 hexadecimal characters"));

        let mut digest_without_source =
            serde_json::from_str::<serde_json::Value>(&json).expect("valid JSON");
        digest_without_source["execution_context"]["model_libraries"][0]["source_closure"] = serde_json::json!([{
            "path": std::env::temp_dir().join("rspice-orphan-pin.lib"),
            "digest": "00".repeat(32)
        }]);
        let error = load_project_text(&digest_without_source.to_string(), None)
            .expect_err("digest without source path must fail")
            .to_string();
        assert!(
            error.contains("source_authority built_in cannot own a root path or source closure"),
            "{error}"
        );
    }

    #[test]
    fn unfinished_analysis_drafts_are_project_data_not_file_corruption() {
        let project = project_with_execution_context();
        let json = serialize_project_file(&project).expect("project serializes");
        let mut value: serde_json::Value = serde_json::from_str(&json).expect("valid JSON");
        serialized_analysis_instance_mut(&mut value, "tran")["draft"]["draft"]["stop"] =
            serde_json::json!("unfinished(");
        serialized_analysis_instance_mut(&mut value, "mc")["draft"]["draft"]["seed"] =
            serde_json::json!("not-an-integer-yet");

        let loaded = load_project_text(&value.to_string(), None)
            .expect("draft syntax is validated by run preflight, not project loading");
        let plan = &loaded
            .execution_context
            .expect("context retained")
            .simulation_plan;

        let stable = plan.stable_analysis_plan().expect("stable plan restored");
        let transient = stable
            .instances()
            .iter()
            .find(|instance| instance.kind() == crate::simulation::plan::AnalysisKind::Transient)
            .expect("transient instance");
        let crate::simulation::plan::AnalysisDraft::Transient(transient) = transient.draft() else {
            panic!("transient instance owns transient draft");
        };
        assert_eq!(transient.stop, "unfinished(");
        let monte_carlo = stable
            .instances()
            .iter()
            .find(|instance| instance.kind() == crate::simulation::plan::AnalysisKind::MonteCarlo)
            .expect("Monte Carlo instance");
        let crate::simulation::plan::AnalysisDraft::MonteCarlo(monte_carlo) = monte_carlo.draft()
        else {
            panic!("Monte Carlo instance owns Monte Carlo draft");
        };
        assert_eq!(monte_carlo.seed, "not-an-integer-yet");
        assert!(
            plan.analysis_draft_validation_error(
                &crate::simulation::plan::AnalysisDraft::Transient(transient.clone(),)
            )
            .is_some()
        );
        assert!(
            plan.analysis_draft_validation_error(
                &crate::simulation::plan::AnalysisDraft::MonteCarlo(monte_carlo.clone()),
            )
            .is_some()
        );
    }

    #[test]
    fn project_file_round_trips_persisted_simulation_results() {
        let mut libraries = LibraryManager::with_primitives();
        let workspace = ProjectWorkspace::new_bootstrapped(&mut libraries);
        let mut simulation = SimulationState::default();
        let waveform = WaveformData::new(
            "|V(out)|",
            vec![1.0, 10.0, 100.0],
            vec![2.0, 3.0, 4.0],
            "#00aaff",
        )
        .with_complex_components("V(out)", vec![2.0, 3.0, 4.0], vec![0.1, 0.2, 0.3]);
        let mut run = SimulationRun::new(12);
        run.timestamp = 1234.5;
        run.label = "Run 12 (fixture)".to_string();
        run.mark_running().expect("fixture run starts");
        run.finish_lifecycle(SimulationRunLifecycle::Completed)
            .expect("fixture run completes");
        run.set_elapsed_time(0.125);
        run.add_analysis(
            AnalysisResult::new(7, AnalysisType::Ac, "AC fixture")
                .with_waveforms(vec![waveform])
                .with_dc_op(crate::state::DcOpResult {
                    node_voltages: vec![OperatingPointValue {
                        name: "V(out)".to_string(),
                        value: 1.25,
                        unit: "V".to_string(),
                    }],
                    branch_currents: Vec::new(),
                    power_dissipation: Vec::new(),
                })
                .with_measurements(vec![rspice_core::MeasureResult::success("gain", 3.0)]),
        );
        seal_legacy_unattributed(&mut run);
        let expected_run_id = run.run_id;
        let expected_dataset_id = run.dataset_id;
        simulation.runs = vec![run];
        simulation.next_run_id = 12;
        simulation.active_run_idx = Some(0);
        simulation.active_analysis_idx = Some(0);

        let project = ProjectFile::new_with_simulation_results(
            workspace,
            libraries,
            ProjectSimulationResults::from_state(&simulation),
        );
        let json = serialize_project_file(&project).expect("project serializes with results");

        assert!(json.contains("\"simulation_results\""));
        let loaded = load_project_text(&json, None).expect("project reloads");
        let restored = loaded
            .simulation_results
            .into_simulation_state()
            .expect("validated project results restore");

        assert_eq!(restored.run_count(), 1);
        assert_eq!(
            restored.active_run().map(|run| run.run_id),
            Some(expected_run_id)
        );
        assert_eq!(
            restored.active_run().map(|run| run.dataset_id),
            Some(expected_dataset_id)
        );
        assert_eq!(
            restored.active_run().expect("active run").label,
            "Run 12 (fixture)"
        );
        let restored_run = restored.active_run().expect("active run");
        assert!(restored_run.job_id.is_some());
        assert_eq!(
            restored_run.execution_target,
            Some(ExecutionTarget::current())
        );
        assert_eq!(restored_run.lifecycle, SimulationRunLifecycle::Completed);
        let analysis = restored.active_analysis().expect("active analysis");
        assert_eq!(analysis.id, 7);
        assert_eq!(analysis.analysis_type, AnalysisType::Ac);
        assert_eq!(analysis.measurements[0].name, "gain");
        assert_eq!(analysis.waveforms[0].complex.as_ref().unwrap().imag[2], 0.3);
        assert_eq!(restored.waveforms[0].name, "|V(out)|");

        let mut unversioned_value: serde_json::Value =
            serde_json::from_str(&json).expect("current project parses as JSON");
        let legacy_results = unversioned_value["simulation_results"]
            .as_object_mut()
            .expect("simulation result object");
        legacy_results.remove("schema_version");
        legacy_results.remove("active_run_stable_id");
        legacy_results.remove("active_dataset_id");
        legacy_results.remove("active_analysis_sequence");
        legacy_results.insert("active_run_id".to_owned(), serde_json::json!(12));
        legacy_results.insert("active_analysis_id".to_owned(), serde_json::json!(7));
        let legacy_run = legacy_results["runs"][0]
            .as_object_mut()
            .expect("legacy run object");
        legacy_run.remove("run_id");
        legacy_run.remove("dataset_id");
        legacy_run.remove("job_id");
        legacy_run.remove("execution_target");
        legacy_run.remove("lifecycle");
        legacy_run.remove("dataset_content_digest");
        for analysis in legacy_run["analyses"]
            .as_array_mut()
            .expect("legacy analysis array")
        {
            analysis
                .as_object_mut()
                .expect("legacy analysis object")
                .remove("result_data_digest");
        }
        legacy_run.remove("provenance_mode");
        let unversioned_json =
            serde_json::to_string(&unversioned_value).expect("unversioned project serializes");
        let unversioned =
            load_project_text(&unversioned_json, None).expect("unversioned project migrates");
        let migrated_run = &unversioned.simulation_results.runs[0];
        assert_eq!(
            unversioned.simulation_results.active_run_stable_id,
            migrated_run.run_id
        );
        assert_eq!(
            unversioned.simulation_results.active_dataset_id,
            migrated_run.dataset_id
        );
        assert_ne!(migrated_run.run_id, Some(expected_run_id));
        assert_ne!(migrated_run.dataset_id, Some(expected_dataset_id));
        assert_eq!(
            unversioned.simulation_results.active_analysis_sequence,
            Some(7)
        );
        assert_eq!(
            migrated_run.lifecycle,
            Some(SimulationRunLifecycle::LegacyUnknown)
        );
    }

    #[test]
    fn project_file_round_trips_exact_result_family_metadata_and_migrates_v6_absence() {
        let mut libraries = LibraryManager::with_primitives();
        let workspace = ProjectWorkspace::new_bootstrapped(&mut libraries);
        let metadata = crate::state::AnalysisResultFamilyMetadata::MonteCarlo {
            seed: 42,
            runs_requested: 3,
            runs_completed: 2,
            failures: 1,
            all_converged: false,
            variables: vec![crate::state::MonteCarloVariableMetadata {
                name: "V(out)".to_owned(),
                samples: vec![0.975, 1.025],
                mean: 1.0,
                std_dev: 0.025,
                min: 0.975,
                max: 1.025,
            }],
        };
        let mut run = SimulationRun::new(1);
        run.mark_running().expect("fixture run starts");
        run.finish_lifecycle(SimulationRunLifecycle::Completed)
            .expect("fixture run completes");
        run.add_analysis(
            AnalysisResult::new(1, AnalysisType::MonteCarlo, "MC")
                .with_family_metadata(metadata.clone()),
        );
        seal_legacy_unattributed(&mut run);
        let mut simulation = SimulationState::default();
        simulation.runs = vec![run];
        simulation.next_run_id = 1;
        simulation.active_run_idx = Some(0);
        simulation.active_analysis_idx = Some(0);

        let project = ProjectFile::new_with_simulation_results(
            workspace,
            libraries,
            ProjectSimulationResults::from_state(&simulation),
        );
        let json = serialize_project_file(&project).expect("family metadata serializes");
        let loaded = load_project_text(&json, None).expect("family metadata reloads");
        let restored = loaded
            .simulation_results
            .into_simulation_state()
            .expect("family metadata restores");
        assert_eq!(
            restored
                .active_analysis()
                .and_then(|analysis| analysis.family_metadata.as_ref()),
            Some(&metadata)
        );

        let mut v6: serde_json::Value = serde_json::from_str(&json).expect("project JSON");
        v6["simulation_results"]["schema_version"] =
            serde_json::Value::from(EXECUTION_IDENTITY_RESULTS_SCHEMA_VERSION);
        v6["simulation_results"]["runs"][0]
            .as_object_mut()
            .expect("run object")
            .remove("dataset_content_digest");
        v6["simulation_results"]["runs"][0]["analyses"][0]
            .as_object_mut()
            .expect("analysis object")
            .remove("family_metadata");
        v6["simulation_results"]["runs"][0]["analyses"][0]
            .as_object_mut()
            .expect("analysis object")
            .remove("result_data_digest");
        let migrated = load_project_text(&v6.to_string(), None).expect("v6 project migrates");
        assert_eq!(
            migrated.simulation_results.schema_version,
            PROJECT_SIMULATION_RESULTS_SCHEMA_VERSION
        );
        let migrated = migrated
            .simulation_results
            .into_simulation_state()
            .expect("migrated v6 results restore");
        assert!(
            migrated
                .active_analysis()
                .expect("migrated analysis")
                .family_metadata
                .is_none(),
            "legacy absence must remain explicit instead of being inferred from waveforms"
        );
    }

    #[test]
    fn retained_result_data_digests_round_trip_and_reject_sample_tampering() {
        let mut run = SimulationRun::new(2);
        run.mark_running().expect("fixture run starts");
        run.finish_lifecycle(SimulationRunLifecycle::Completed)
            .expect("fixture run completes");
        run.add_analysis(
            AnalysisResult::new(1, AnalysisType::Ac, "AC").with_waveforms(vec![
                WaveformData::new("V(out)", vec![1.0, 10.0], vec![0.25, 0.5], "#00aaff")
                    .with_complex_components("V(out)", vec![0.25, 0.5], vec![-0.75, -0.5]),
            ]),
        );
        seal_legacy_unattributed(&mut run);
        let expected_analysis_digest = run.analyses[0].result_data_digest();
        let expected_dataset_digest = run.dataset_content_digest();
        let mut simulation = SimulationState::default();
        simulation.runs = vec![run];
        simulation.next_run_id = 2;

        let persisted = ProjectSimulationResults::from_state(&simulation);
        assert_eq!(
            persisted.runs[0].analyses[0]
                .result_data_digest
                .as_ref()
                .copied(),
            Some(expected_analysis_digest)
        );
        assert_eq!(
            persisted.runs[0].dataset_content_digest.as_ref().copied(),
            Some(expected_dataset_digest)
        );

        let json = serde_json::to_string(&persisted).expect("result data serializes");
        let restored: ProjectSimulationResults =
            serde_json::from_str(&json).expect("result data deserializes");
        restored.validate().expect("retained digests validate");
        let restored_run = restored
            .into_simulation_state()
            .expect("retained result data restores")
            .runs
            .remove(0);
        assert_eq!(
            restored_run.analyses[0].result_data_digest(),
            expected_analysis_digest
        );
        assert_eq!(
            restored_run.dataset_content_digest(),
            expected_dataset_digest
        );

        let mut tampered: serde_json::Value =
            serde_json::from_str(&json).expect("result document JSON");
        tampered["runs"][0]["analyses"][0]["waveforms"][0]["complex"]["imag"][1] =
            serde_json::json!(-0.499_999_999_999_999_94_f64);
        let tampered: ProjectSimulationResults =
            serde_json::from_value(tampered).expect("tampered result remains structurally valid");
        assert!(
            tampered
                .validate()
                .expect_err("a changed complex sample invalidates its retained digest")
                .contains("result_data_digest does not match retained analysis content")
        );
    }

    #[test]
    fn typed_result_payloads_round_trip_and_reject_payload_tampering() {
        let mut run = SimulationRun::new(31);
        run.mark_running().expect("fixture run starts");
        run.finish_lifecycle(SimulationRunLifecycle::Completed)
            .expect("fixture run completes");
        run.add_analysis(
            AnalysisResult::new(1, AnalysisType::PoleZero, "PZ").with_result_payload(
                AnalysisResultPayload::PoleZero {
                    poles: vec![crate::state::ComplexResultValue {
                        real: -1.0,
                        imaginary: 2.0,
                    }],
                    zeros: vec![crate::state::ComplexResultValue {
                        real: -3.0,
                        imaginary: 0.0,
                    }],
                    gain: 4.0,
                },
            ),
        );
        run.add_analysis(
            AnalysisResult::new(2, AnalysisType::Sensitivity, "SENS").with_result_payload(
                AnalysisResultPayload::Sensitivity {
                    output: "V(out)".to_owned(),
                    result_mode: crate::state::SensitivityResultMode::Ac {
                        frequency_hz: 10_000.0,
                    },
                    rows: vec![crate::state::SensitivityResultRow {
                        parameter: "width".to_owned(),
                        raw: 2.0,
                        normalized: 0.5,
                    }],
                },
            ),
        );
        run.add_analysis(
            AnalysisResult::new(3, AnalysisType::Tf, "TF").with_result_payload(
                AnalysisResultPayload::TransferFunction {
                    input_source: "VIN".to_owned(),
                    output_expression: "V(OUT)".to_owned(),
                    input_quantity: crate::state::TransferFunctionQuantityEvidence::Voltage,
                    output_quantity: crate::state::TransferFunctionQuantityEvidence::Voltage,
                    input_unit: "V".to_owned(),
                    output_unit: "V".to_owned(),
                    normalization: crate::state::TransferFunctionNormalizationEvidence::None,
                    accuracy: crate::state::TransferFunctionAccuracyEvidence::Balanced,
                    gain: Some(crate::state::TransferFunctionScalarEvidence::Finite(10.0)),
                    input_resistance: Some(
                        crate::state::TransferFunctionScalarEvidence::PositiveInfinity,
                    ),
                    output_resistance: Some(crate::state::TransferFunctionScalarEvidence::Finite(
                        50.0,
                    )),
                    nominal_input: None,
                    nominal_output: None,
                },
            ),
        );
        run.add_analysis(
            AnalysisResult::new(4, AnalysisType::Reliability, "Reliability")
                .with_family_metadata(AnalysisResultFamilyMetadata::Reliability {
                    years: vec![10.0],
                })
                .with_result_payload(AnalysisResultPayload::Reliability {
                    devices: vec![crate::state::ReliabilityDeviceEvidence {
                        device_id: "M1".to_owned(),
                        stress: crate::state::ReliabilityStressEvidence {
                            average_gate_stress_v: 1.2,
                            average_drain_stress_v: 1.8,
                            average_temperature_k: 358.15,
                            duration_s: 3_600.0,
                        },
                        checkpoints: vec![crate::state::ReliabilityCheckpointEvidence {
                            years: 10.0,
                            shift: crate::state::ReliabilityShiftEvidence {
                                threshold_voltage_shift_v: 0.03,
                                mobility_shift: -0.004,
                                drain_source_resistance_shift: 0.0015,
                            },
                        }],
                    }],
                }),
        );
        run.add_analysis(
            AnalysisResult::new(5, AnalysisType::Soa, "SOA")
                .with_family_metadata(AnalysisResultFamilyMetadata::Soa {
                    time: vec![0.0, 1.0],
                })
                .with_result_payload(AnalysisResultPayload::Soa {
                    evaluations: vec![crate::state::SoaEvaluationEvidence {
                        device_id: "M1".to_owned(),
                        parameter: crate::state::SoaParameterEvidence::DrainSourceVoltage,
                        limit_value: 3.3,
                        worst_actual_value: 3.2,
                        worst_time_s: 1.0,
                        sample_count: 2,
                        unit: "V".to_owned(),
                        description: "Maximum drain-source voltage".to_owned(),
                        verdict: crate::state::SoaRuleVerdictEvidence::Warning,
                    }],
                    violations: vec![crate::state::SoaViolationEvidence {
                        device_id: "M1".to_owned(),
                        parameter: crate::state::SoaParameterEvidence::DrainSourceVoltage,
                        limit_value: 3.3,
                        actual_value: 3.2,
                        time_s: 1.0,
                        severity: crate::state::SoaViolationSeverityEvidence::Warning,
                    }],
                }),
        );
        run.add_analysis(
            AnalysisResult::new(6, AnalysisType::DcOp, "OP").with_result_payload(
                AnalysisResultPayload::OperatingPoint {
                    temperature_mode: crate::state::OperatingPointTemperatureEvidence::PvtRunSet,
                    temperature_celsius: 27.0,
                    initial_guess:
                        crate::state::OperatingPointInitialGuessEvidence::PreviousConverged,
                    node_initialization:
                        crate::state::OperatingPointNodeInitializationEvidence::UseIcAndNodeset,
                    homotopy: crate::state::OperatingPointHomotopyEvidence::Adaptive,
                    annotation: crate::state::OperatingPointAnnotationEvidence::VoltagesAndDeviceOp,
                    device_detail: crate::state::OperatingPointDeviceDetailEvidence::ViolationsOnly,
                    save_device_op: crate::state::OperatingPointSaveDeviceEvidence::FinalPointOnly,
                    accuracy: crate::state::OperatingPointAccuracyEvidence::Robust,
                    selected_devices: vec!["M1".to_owned()],
                    violation_devices: vec!["M1".to_owned()],
                    violation_source_content_digest: Some(
                        crate::product::ContentDigest::from_bytes([0x61; 32]),
                    ),
                    validated_startup_directives: 2,
                    mna_node_names: vec!["in".to_owned(), "out".to_owned()],
                    mna_branch_names: vec!["V1".to_owned()],
                    mna_solution: vec![1.0, 0.5, -0.5e-3],
                    effective_source_content_digest: Some(
                        crate::product::ContentDigest::from_bytes([0x62; 32]),
                    ),
                    run_point_index: 1,
                    run_point_count: 2,
                    run_point_process: crate::state::OperatingPointProcessEvidence::SS,
                    run_point_supply_voltage: Some(0.9),
                    run_point_nominal_supply_voltage: Some(1.0),
                },
            ),
        );
        seal_legacy_unattributed(&mut run);
        let dataset_id = run.dataset_id;
        let mut simulation = SimulationState::default();
        simulation.runs = vec![run];
        simulation.next_run_id = 31;
        simulation.active_run_idx = Some(0);
        simulation.active_analysis_idx = Some(1);

        let persisted = ProjectSimulationResults::from_state(&simulation);
        let json = serde_json::to_string(&persisted).expect("typed payloads serialize");
        let decoded: ProjectSimulationResults =
            serde_json::from_str(&json).expect("typed payloads deserialize");
        decoded.validate().expect("typed payload digests validate");
        let restored = decoded
            .into_simulation_state()
            .expect("typed payloads restore");
        assert_eq!(
            restored.active_run().map(|run| run.dataset_id),
            Some(dataset_id)
        );
        assert_eq!(
            restored.active_analysis().map(|analysis| analysis.id),
            Some(2)
        );
        assert_eq!(
            restored.runs[0].analyses[0].result_payload,
            simulation.runs[0].analyses[0].result_payload
        );
        assert_eq!(
            restored.runs[0].analyses[1].result_payload,
            simulation.runs[0].analyses[1].result_payload
        );
        assert_eq!(
            restored.runs[0].analyses[2].result_payload,
            simulation.runs[0].analyses[2].result_payload
        );
        assert!(matches!(
            restored.runs[0].analyses[2].result_payload.as_ref(),
            Some(AnalysisResultPayload::TransferFunction {
                input_resistance: Some(
                    crate::state::TransferFunctionScalarEvidence::PositiveInfinity
                ),
                ..
            })
        ));
        assert_eq!(
            restored.runs[0].analyses[3].result_payload,
            simulation.runs[0].analyses[3].result_payload
        );
        assert_eq!(
            restored.runs[0].analyses[4].result_payload,
            simulation.runs[0].analyses[4].result_payload
        );
        assert_eq!(
            restored.runs[0].analyses[5].result_payload,
            simulation.runs[0].analyses[5].result_payload
        );

        let mut tampered: serde_json::Value = serde_json::from_str(&json).expect("project JSON");
        tampered["runs"][0]["analyses"][0]["result_payload"]["gain"] =
            serde_json::json!(4.000_000_000_000_001_f64);
        let tampered: ProjectSimulationResults =
            serde_json::from_value(tampered).expect("tampered payload remains structural");
        assert!(
            tampered
                .validate()
                .expect_err("payload tampering invalidates the result digest")
                .contains("result_data_digest does not match retained analysis content")
        );

        let mut op_tampered: serde_json::Value = serde_json::from_str(&json).expect("project JSON");
        op_tampered["runs"][0]["analyses"][5]["result_payload"]["mna_solution"][1] =
            serde_json::json!(0.500_000_000_000_000_1_f64);
        let op_tampered: ProjectSimulationResults =
            serde_json::from_value(op_tampered).expect("tampered OP payload remains structural");
        assert!(
            op_tampered
                .validate()
                .expect_err("OP MNA tampering invalidates the result digest")
                .contains("result_data_digest does not match retained analysis content")
        );

        let mut reliability_tampered: serde_json::Value =
            serde_json::from_str(&json).expect("project JSON");
        reliability_tampered["runs"][0]["analyses"][3]["result_payload"]["devices"][0]["checkpoints"]
            [0]["shift"]["mobility_shift"] = serde_json::json!(-0.004_000_000_000_000_001_f64);
        let reliability_tampered: ProjectSimulationResults =
            serde_json::from_value(reliability_tampered)
                .expect("tampered reliability payload remains structural");
        assert!(
            reliability_tampered
                .validate()
                .expect_err("reliability field tampering invalidates the result digest")
                .contains("result_data_digest does not match retained analysis content")
        );

        let mut tf_tampered: serde_json::Value = serde_json::from_str(&json).expect("project JSON");
        tf_tampered["runs"][0]["analyses"][2]["result_payload"]["gain"]["value"] =
            serde_json::json!(10.000_000_000_000_002_f64);
        let tf_tampered: ProjectSimulationResults = serde_json::from_value(tf_tampered)
            .expect("tampered transfer-function payload remains structural");
        assert!(
            tf_tampered
                .validate()
                .expect_err("transfer-function field tampering invalidates the result digest")
                .contains("result_data_digest does not match retained analysis content")
        );

        let mut soa_tampered: serde_json::Value =
            serde_json::from_str(&json).expect("project JSON");
        soa_tampered["runs"][0]["analyses"][4]["result_payload"]["evaluations"][0]["description"] =
            serde_json::json!("Changed rule description");
        let soa_tampered: ProjectSimulationResults =
            serde_json::from_value(soa_tampered).expect("tampered SOA payload remains structural");
        assert!(
            soa_tampered
                .validate()
                .expect_err("SOA field tampering invalidates the result digest")
                .contains("result_data_digest does not match retained analysis content")
        );

        let mut null_payload: serde_json::Value =
            serde_json::from_str(&json).expect("project JSON");
        null_payload["runs"][0]["analyses"][0]["result_payload"] = serde_json::Value::Null;
        let null_payload: ProjectSimulationResults =
            serde_json::from_value(null_payload).expect("null remains presence-aware");
        assert!(
            null_payload
                .validate()
                .expect_err("current payload cannot be explicitly null")
                .contains("result_payload must not be null")
        );
    }

    #[test]
    fn schema_v8_digests_are_authenticated_before_v9_resealing() {
        let mut run = SimulationRun::new(32);
        run.mark_running().expect("fixture run starts");
        run.finish_lifecycle(SimulationRunLifecycle::Completed)
            .expect("fixture run completes");
        run.add_analysis(
            AnalysisResult::new(1, AnalysisType::Transient, "TRAN").with_waveforms(vec![
                WaveformData::new("V(out)", vec![0.0, 1.0], vec![0.0, 1.0], "#00aaff"),
            ]),
        );
        seal_legacy_unattributed(&mut run);
        let mut simulation = SimulationState::default();
        simulation.runs = vec![run];
        simulation.next_run_id = 32;
        let mut v8 = ProjectSimulationResults::from_state(&simulation);
        v8.schema_version = CONTENT_DIGEST_RESULTS_SCHEMA_VERSION;
        for analysis in &mut v8.runs[0].analyses {
            analysis.result_payload = PersistedField::Missing;
            analysis.result_data_digest = PersistedField::Value(
                analysis
                    .clone()
                    .into_analysis()
                    .expect("v8 analysis fixture")
                    .legacy_v1_result_data_digest(),
            );
        }
        v8.runs[0].dataset_content_digest = PersistedField::Value(
            v8.runs[0]
                .clone()
                .into_run()
                .expect("v8 run fixture")
                .legacy_v1_dataset_content_digest(),
        );

        let mut migrated = v8.clone();
        migrated
            .migrate_to_current(ProjectId::new())
            .expect("authentic v8 results migrate");
        assert_eq!(
            migrated.schema_version,
            PROJECT_SIMULATION_RESULTS_SCHEMA_VERSION
        );
        migrated
            .validate()
            .expect("resealed current results validate");
        assert_ne!(
            migrated.runs[0].dataset_content_digest, v8.runs[0].dataset_content_digest,
            "current results use the new canonical digest domain"
        );

        let mut tampered = v8.clone();
        tampered.runs[0].analyses[0].waveforms[0].y[1] = 1.000_000_000_000_000_2;
        assert!(
            tampered
                .migrate_to_current(ProjectId::new())
                .expect_err("v8 tampering is rejected before resealing")
                .contains("schema-v8 analysis 1 result data digest")
        );

        let mut injected = v8;
        injected.runs[0].analyses[0].result_payload =
            PersistedField::Value(AnalysisResultPayload::ScalarMeasurements {
                values: std::collections::BTreeMap::from([("gain".to_owned(), 1.0)]),
            });
        assert!(
            injected
                .migrate_to_current(ProjectId::new())
                .expect_err("schema-v8 cannot inject a v9 payload")
                .contains("typed result payload introduced by schema v9")
        );
    }

    #[test]
    fn schema_v9_digests_are_authenticated_before_current_resealing() {
        let mut run = SimulationRun::new(33);
        run.mark_running().expect("fixture run starts");
        run.finish_lifecycle(SimulationRunLifecycle::Completed)
            .expect("fixture run completes");
        run.add_analysis(
            AnalysisResult::new(1, AnalysisType::Disto, "DISTO").with_result_payload(
                AnalysisResultPayload::ScalarMeasurements {
                    values: std::collections::BTreeMap::from([("gain".to_owned(), 10.0)]),
                },
            ),
        );
        run.add_analysis(
            AnalysisResult::new(2, AnalysisType::Reliability, "Reliability").with_family_metadata(
                AnalysisResultFamilyMetadata::Reliability {
                    years: vec![1.0, 5.0, 10.0],
                },
            ),
        );
        seal_legacy_unattributed(&mut run);
        let mut simulation = SimulationState::default();
        simulation.runs = vec![run];
        simulation.next_run_id = 33;
        let mut v9 = ProjectSimulationResults::from_state(&simulation);
        v9.schema_version = TYPED_PAYLOAD_RESULTS_SCHEMA_VERSION;
        for analysis in &mut v9.runs[0].analyses {
            analysis.result_data_digest = PersistedField::Value(
                analysis
                    .clone()
                    .into_analysis()
                    .expect("v9 analysis fixture")
                    .legacy_v2_result_data_digest(),
            );
        }
        v9.runs[0].dataset_content_digest = PersistedField::Value(
            v9.runs[0]
                .clone()
                .into_run()
                .expect("v9 run fixture")
                .legacy_v2_dataset_content_digest(),
        );

        let mut migrated = v9.clone();
        migrated
            .migrate_to_current(ProjectId::new())
            .expect("authentic v9 results migrate");
        assert_eq!(
            migrated.schema_version,
            PROJECT_SIMULATION_RESULTS_SCHEMA_VERSION
        );
        migrated
            .validate()
            .expect("resealed current results validate");
        assert!(
            migrated.runs[0].analyses[1].result_payload.is_missing(),
            "migration preserves the absence of v10 reliability evidence"
        );
        assert_ne!(
            migrated.runs[0].dataset_content_digest, v9.runs[0].dataset_content_digest,
            "current results use the v4 canonical digest domain"
        );

        let mut tampered = v9.clone();
        let Some(AnalysisResultPayload::ScalarMeasurements { values }) =
            tampered.runs[0].analyses[0].result_payload.as_mut()
        else {
            panic!("schema-v9 scalar payload")
        };
        values.insert("gain".to_owned(), 10.000_000_000_000_002);
        assert!(
            tampered
                .migrate_to_current(ProjectId::new())
                .expect_err("v9 tampering is rejected before resealing")
                .contains("schema-v9 analysis 1 result data digest")
        );

        let mut injected = v9.clone();
        injected.runs[0].analyses[0].result_payload =
            PersistedField::Value(AnalysisResultPayload::Reliability {
                devices: vec![crate::state::ReliabilityDeviceEvidence {
                    device_id: "M1".to_owned(),
                    stress: crate::state::ReliabilityStressEvidence {
                        average_gate_stress_v: 1.0,
                        average_drain_stress_v: 1.0,
                        average_temperature_k: 300.0,
                        duration_s: 1.0,
                    },
                    checkpoints: Vec::new(),
                }],
            });
        assert!(
            injected
                .migrate_to_current(ProjectId::new())
                .expect_err("schema-v9 cannot inject v10 evidence")
                .contains("Reliability/SOA evidence introduced by schema v10")
        );

        let mut injected_op = v9;
        injected_op.runs[0].analyses[0].result_payload =
            PersistedField::Value(operating_point_payload_fixture());
        assert!(
            injected_op
                .migrate_to_current(ProjectId::new())
                .expect_err("schema-v9 cannot inject v12 operating-point evidence")
                .contains("operating-point evidence introduced by schema v12")
        );
    }

    #[test]
    fn schema_v10_digests_are_authenticated_before_v11_tf_resealing() {
        let mut run = SimulationRun::new(34);
        run.mark_running().expect("fixture run starts");
        run.finish_lifecycle(SimulationRunLifecycle::Completed)
            .expect("fixture run completes");
        run.add_analysis(
            AnalysisResult::new(1, AnalysisType::Reliability, "Reliability")
                .with_family_metadata(AnalysisResultFamilyMetadata::Reliability {
                    years: vec![10.0],
                })
                .with_result_payload(AnalysisResultPayload::Reliability {
                    devices: vec![crate::state::ReliabilityDeviceEvidence {
                        device_id: "M1".to_owned(),
                        stress: crate::state::ReliabilityStressEvidence {
                            average_gate_stress_v: 1.2,
                            average_drain_stress_v: 1.8,
                            average_temperature_k: 358.15,
                            duration_s: 3_600.0,
                        },
                        checkpoints: vec![crate::state::ReliabilityCheckpointEvidence {
                            years: 10.0,
                            shift: crate::state::ReliabilityShiftEvidence {
                                threshold_voltage_shift_v: 0.03,
                                mobility_shift: -0.004,
                                drain_source_resistance_shift: 0.0015,
                            },
                        }],
                    }],
                }),
        );
        run.add_analysis(AnalysisResult::new(2, AnalysisType::Tf, "TF"));
        seal_legacy_unattributed(&mut run);

        let mut simulation = SimulationState::default();
        simulation.runs = vec![run];
        simulation.next_run_id = 34;
        let mut v10 = ProjectSimulationResults::from_state(&simulation);
        v10.schema_version = RELIABILITY_SOA_RESULTS_SCHEMA_VERSION;
        for analysis in &mut v10.runs[0].analyses {
            analysis.result_data_digest = PersistedField::Value(
                analysis
                    .clone()
                    .into_analysis()
                    .expect("v10 analysis fixture")
                    .legacy_v3_result_data_digest(),
            );
        }
        v10.runs[0].dataset_content_digest = PersistedField::Value(
            v10.runs[0]
                .clone()
                .into_run()
                .expect("v10 run fixture")
                .legacy_v3_dataset_content_digest(),
        );

        let mut migrated = v10.clone();
        migrated
            .migrate_to_current(ProjectId::new())
            .expect("authentic v10 results migrate");
        assert_eq!(
            migrated.schema_version,
            PROJECT_SIMULATION_RESULTS_SCHEMA_VERSION
        );
        migrated.validate().expect("resealed v11 results validate");
        assert!(
            migrated.runs[0].analyses[1].result_payload.is_missing(),
            "migration preserves the absence of v11 transfer-function evidence"
        );
        assert_ne!(
            migrated.runs[0].dataset_content_digest, v10.runs[0].dataset_content_digest,
            "v11 uses the v4 canonical digest domain"
        );

        let mut tampered = v10.clone();
        let Some(AnalysisResultPayload::Reliability { devices }) =
            tampered.runs[0].analyses[0].result_payload.as_mut()
        else {
            panic!("schema-v10 reliability payload")
        };
        devices[0].stress.duration_s = 3_600.000_000_000_000_5;
        assert!(
            tampered
                .migrate_to_current(ProjectId::new())
                .expect_err("v10 tampering is rejected before resealing")
                .contains("schema-v10 analysis 1 result data digest")
        );

        let mut injected = v10.clone();
        injected.runs[0].analyses[1].result_payload =
            PersistedField::Value(AnalysisResultPayload::TransferFunction {
                input_source: "VIN".to_owned(),
                output_expression: "V(OUT)".to_owned(),
                input_quantity: crate::state::TransferFunctionQuantityEvidence::Voltage,
                output_quantity: crate::state::TransferFunctionQuantityEvidence::Voltage,
                input_unit: "V".to_owned(),
                output_unit: "V".to_owned(),
                normalization: crate::state::TransferFunctionNormalizationEvidence::None,
                accuracy: crate::state::TransferFunctionAccuracyEvidence::Balanced,
                gain: Some(crate::state::TransferFunctionScalarEvidence::Finite(0.5)),
                input_resistance: Some(
                    crate::state::TransferFunctionScalarEvidence::PositiveInfinity,
                ),
                output_resistance: Some(crate::state::TransferFunctionScalarEvidence::Finite(50.0)),
                nominal_input: None,
                nominal_output: None,
            });
        assert!(
            injected
                .migrate_to_current(ProjectId::new())
                .expect_err("schema-v10 cannot inject v11 transfer-function evidence")
                .contains("transfer-function evidence introduced by schema v11")
        );

        let mut injected_op = v10;
        injected_op.runs[0].analyses[0].result_payload =
            PersistedField::Value(operating_point_payload_fixture());
        assert!(
            injected_op
                .migrate_to_current(ProjectId::new())
                .expect_err("schema-v10 cannot inject v12 operating-point evidence")
                .contains("operating-point evidence introduced by schema v12")
        );
    }

    #[test]
    fn schema_v7_digest_migration_is_deterministic_and_rejects_anachronistic_fields() {
        let mut run = SimulationRun::new(3);
        run.mark_running().expect("fixture run starts");
        run.finish_lifecycle(SimulationRunLifecycle::Completed)
            .expect("fixture run completes");
        run.add_analysis(
            AnalysisResult::new(1, AnalysisType::Transient, "TRAN").with_waveforms(vec![
                WaveformData::new("V(out)", vec![0.0, 1.0], vec![0.0, 1.0], "#00aaff"),
            ]),
        );
        seal_legacy_unattributed(&mut run);
        let mut simulation = SimulationState::default();
        simulation.runs = vec![run];
        simulation.next_run_id = 3;
        let mut legacy = ProjectSimulationResults::from_state(&simulation);
        legacy.schema_version = FAMILY_METADATA_RESULTS_SCHEMA_VERSION;
        legacy.runs[0].dataset_content_digest = PersistedField::Missing;
        legacy.runs[0].analyses[0].result_data_digest = PersistedField::Missing;

        let mut first = legacy.clone();
        let mut second = legacy.clone();
        let project_id = ProjectId::new();
        first
            .migrate_to_current(project_id)
            .expect("first schema-v7 migration succeeds");
        second
            .migrate_to_current(project_id)
            .expect("identical schema-v7 migration succeeds");
        assert_eq!(
            first.runs[0].analyses[0].result_data_digest,
            second.runs[0].analyses[0].result_data_digest
        );
        assert_eq!(
            first.runs[0].dataset_content_digest,
            second.runs[0].dataset_content_digest
        );
        first.validate().expect("migrated digests validate");

        let mut injected_payload = legacy;
        injected_payload.runs[0].analyses[0].result_payload =
            PersistedField::Value(AnalysisResultPayload::ScalarMeasurements {
                values: std::collections::BTreeMap::from([("gain".to_owned(), 1.0)]),
            });
        let injected_before = injected_payload.clone();
        assert!(
            injected_payload
                .migrate_to_current(project_id)
                .expect_err("schema v7 cannot carry schema-v9 typed evidence")
                .contains("typed result payload introduced by schema v9")
        );
        assert_eq!(
            injected_payload, injected_before,
            "failed typed-payload migration is transactional"
        );

        let mut relabeled = first;
        relabeled.schema_version = FAMILY_METADATA_RESULTS_SCHEMA_VERSION;
        let before = relabeled.clone();
        assert!(
            relabeled
                .migrate_to_current(project_id)
                .expect_err("schema v7 cannot carry schema-v8 digest fields")
                .contains("introduced by schema v8")
        );
        assert_eq!(relabeled, before, "failed migration is transactional");
    }

    #[test]
    fn legacy_schema_field_gate_rejects_relabelled_typed_evidence() {
        let mut run = SimulationRun::new(4);
        run.add_analysis(AnalysisResult::new(
            1,
            AnalysisType::Reliability,
            "Reliability",
        ));
        seal_legacy_unattributed(&mut run);
        let mut persisted_run = ProjectSimulationRun::from(&run);
        persisted_run.analyses[0].result_data_digest = PersistedField::Missing;
        persisted_run.dataset_content_digest = PersistedField::Missing;
        persisted_run.analyses[0].result_payload =
            PersistedField::Value(AnalysisResultPayload::Reliability {
                devices: vec![crate::state::ReliabilityDeviceEvidence {
                    device_id: "M1".to_owned(),
                    stress: crate::state::ReliabilityStressEvidence {
                        average_gate_stress_v: 1.0,
                        average_drain_stress_v: 1.0,
                        average_temperature_k: 300.0,
                        duration_s: 1.0,
                    },
                    checkpoints: vec![crate::state::ReliabilityCheckpointEvidence {
                        years: 1.0,
                        shift: crate::state::ReliabilityShiftEvidence {
                            threshold_voltage_shift_v: 0.01,
                            mobility_shift: -0.001,
                            drain_source_resistance_shift: 0.0001,
                        },
                    }],
                }],
            });

        for source_schema in
            LEGACY_SIMULATION_RESULTS_SCHEMA_VERSION..=CONTENT_DIGEST_RESULTS_SCHEMA_VERSION
        {
            assert!(
                validate_result_fields_for_source_schema(&persisted_run, source_schema)
                    .expect_err("pre-v9 schema cannot carry a typed payload")
                    .contains("typed result payload introduced by schema v9")
            );
        }

        persisted_run.analyses[0].result_payload = PersistedField::Missing;
        persisted_run.analyses[0].family_metadata =
            Some(AnalysisResultFamilyMetadata::Reliability { years: vec![1.0] });
        for source_schema in
            LEGACY_SIMULATION_RESULTS_SCHEMA_VERSION..FAMILY_METADATA_RESULTS_SCHEMA_VERSION
        {
            assert!(
                validate_result_fields_for_source_schema(&persisted_run, source_schema)
                    .expect_err("pre-v7 schema cannot carry family metadata")
                    .contains("family metadata introduced by schema v7")
            );
        }
    }

    #[test]
    fn missing_persisted_lifecycle_restores_as_explicit_legacy_unknown() {
        let mut run = SimulationRun::new(13);
        seal_legacy_unattributed(&mut run);
        let expected_run_id = run.run_id;
        let mut persisted = ProjectSimulationRun::from(&run);
        persisted.job_id = None;
        persisted.execution_target = None;
        persisted.lifecycle = None;

        let restored = persisted.into_run().expect("legacy run restores");

        assert_eq!(restored.run_id, expected_run_id);
        assert_eq!(restored.job_id, None);
        assert_eq!(restored.execution_target, None);
        assert_eq!(restored.lifecycle, SimulationRunLifecycle::LegacyUnknown);
    }

    #[test]
    fn schema_v5_migrates_to_explicit_legacy_execution_state() {
        let mut run = SimulationRun::new(14);
        run.mark_running().expect("fixture run starts");
        run.finish_lifecycle(SimulationRunLifecycle::Completed)
            .expect("fixture run completes");
        seal_legacy_unattributed(&mut run);
        let mut simulation = SimulationState::default();
        simulation.runs = vec![run];
        simulation.next_run_id = 14;
        let mut persisted = ProjectSimulationResults::from_state(&simulation);
        persisted.schema_version = SOURCE_DOMAIN_RESULTS_SCHEMA_VERSION;
        clear_v6_execution_fields(&mut persisted);

        persisted
            .migrate_to_current(ProjectId::new())
            .expect("schema v5 migrates without inventing execution evidence");
        persisted.validate().expect("migrated schema validates");

        assert_eq!(
            persisted.schema_version,
            PROJECT_SIMULATION_RESULTS_SCHEMA_VERSION
        );
        let migrated = &persisted.runs[0];
        assert_eq!(
            migrated.lifecycle,
            Some(SimulationRunLifecycle::LegacyUnknown)
        );
        assert_eq!(migrated.job_id, None);
        assert_eq!(migrated.execution_target, None);
        assert!(migrated.success, "legacy outcome evidence is preserved");
    }

    #[test]
    fn current_schema_requires_coherent_lifecycle_and_execution_identity() {
        let mut run = SimulationRun::new(15);
        seal_legacy_unattributed(&mut run);
        let mut simulation = SimulationState::default();
        simulation.runs = vec![run];
        simulation.next_run_id = 15;
        let current = ProjectSimulationResults::from_state(&simulation);
        current
            .validate()
            .expect("current preparing snapshot is explicit");

        let mut missing_lifecycle = current.clone();
        missing_lifecycle.runs[0].lifecycle = None;
        assert!(
            missing_lifecycle
                .validate()
                .expect_err("schema v6 requires lifecycle evidence")
                .contains("lifecycle is required by simulation results schema v6")
        );

        let mut missing_job = current.clone();
        missing_job.runs[0].job_id = None;
        assert!(
            missing_job
                .validate()
                .expect_err("current execution requires job identity")
                .contains("job_id is required for a non-legacy lifecycle")
        );

        let mut legacy_with_target = current.clone();
        legacy_with_target.runs[0].lifecycle = Some(SimulationRunLifecycle::LegacyUnknown);
        assert!(
            legacy_with_target
                .validate()
                .expect_err("legacy execution cannot claim current identity")
                .contains("legacy_unknown but carries current execution job/target identity")
        );

        let mut false_completion = current;
        false_completion.runs[0].lifecycle = Some(SimulationRunLifecycle::Completed);
        assert!(
            false_completion
                .validate()
                .expect_err("completed outcome must be successful")
                .contains("completed but its success outcome is false")
        );
    }

    #[test]
    fn persisted_running_and_cancelling_runs_restore_as_interrupted() {
        for (sequence, cancelling) in [(16, false), (17, true)] {
            let mut run = SimulationRun::new(sequence);
            run.mark_running().expect("fixture run starts");
            if cancelling {
                run.mark_cancelling().expect("fixture cancellation starts");
            }
            seal_legacy_unattributed(&mut run);
            let expected_job_id = run.job_id;
            let expected_target = run.execution_target;
            let mut simulation = SimulationState::default();
            simulation.runs = vec![run];
            simulation.next_run_id = sequence;

            let persisted = ProjectSimulationResults::from_state(&simulation);
            assert!(!persisted.runs[0].success);
            persisted
                .validate()
                .expect("nonterminal snapshot validates");
            let restored = persisted
                .into_simulation_state()
                .expect("nonterminal snapshot restores fail-closed");
            let restored = &restored.runs[0];

            assert_eq!(restored.lifecycle, SimulationRunLifecycle::Interrupted);
            assert!(!restored.success);
            assert_eq!(restored.job_id, expected_job_id);
            assert_eq!(restored.execution_target, expected_target);
        }
    }

    #[test]
    fn prepared_result_provenance_round_trips_two_same_kind_analyses_exactly() {
        let first_id = AnalysisInstanceId::new();
        let second_id = AnalysisInstanceId::new();
        let snapshot = ContentDigest::from_bytes([0xa7; 32]);
        let revision = ObjectRevision::new(12).expect("fixture revision");
        let mut run = SimulationRun::new(21);
        run.add_analysis(
            AnalysisResult::new(1, AnalysisType::Ac, "AC low band").with_provenance(
                AnalysisResultProvenance::new(first_id, revision, snapshot, Vec::new())
                    .expect("first provenance"),
            ),
        );
        run.add_analysis(
            AnalysisResult::new(2, AnalysisType::Ac, "AC high band").with_provenance(
                AnalysisResultProvenance::new(second_id, revision, snapshot, vec![first_id])
                    .expect("second provenance"),
            ),
        );
        seal_prepared_run(
            &mut run,
            AnalysisResultSourceDomain::SimulationPlan,
            Some(SimulationPlanId::new()),
            ObjectRevision::INITIAL,
            ContentDigest::from_bytes([0xa6; 32]),
            PreparedSourceCheckReceipt::SchematicDrc(ContentDigest::from_bytes([0xa5; 32])),
            &[2, 2],
        );
        let mut simulation = SimulationState::default();
        simulation.runs = vec![run];
        simulation.next_run_id = 21;

        let persisted = ProjectSimulationResults::from_state(&simulation);
        let json = serde_json::to_string(&persisted).expect("results serialize");
        let decoded: ProjectSimulationResults =
            serde_json::from_str(&json).expect("results deserialize");
        let restored = decoded
            .into_simulation_state()
            .expect("provenance validates and restores");
        let restored_run = &restored.runs[0];

        assert_eq!(restored_run.analyses.len(), 2);
        assert_eq!(
            restored_run
                .find_analysis_by_source_instance(first_id)
                .expect("first exact source")
                .label,
            "AC low band"
        );
        let second = restored_run
            .find_analysis_by_source_instance(second_id)
            .expect("second exact source");
        let second_provenance = second.provenance.as_ref().expect("current provenance");
        assert_eq!(second.label, "AC high band");
        assert_eq!(second_provenance.source_revision(), revision);
        assert_eq!(second_provenance.prepared_snapshot_digest(), snapshot);
        assert_eq!(second_provenance.dependency_ids(), &[first_id]);
    }

    #[test]
    fn manual_deck_result_provenance_round_trips_without_a_simulation_plan() {
        let source_content_digest = ContentDigest::from_bytes([0x8c; 32]);
        let source_id = crate::simulation::execution::manual_deck_analysis_instance_id_from_tag(
            source_content_digest,
            5,
            0,
        );
        let snapshot = ContentDigest::from_bytes([0x8d; 32]);
        let mut run = SimulationRun::new(29);
        run.add_analysis(
            AnalysisResult::new(1, AnalysisType::Transient, "Manual TRAN").with_provenance(
                AnalysisResultProvenance::new_with_source_domain(
                    AnalysisResultSourceDomain::ManualDeck,
                    source_id,
                    ObjectRevision::INITIAL,
                    snapshot,
                    Vec::new(),
                )
                .expect("manual-deck provenance"),
            ),
        );
        seal_prepared_run(
            &mut run,
            AnalysisResultSourceDomain::ManualDeck,
            None,
            ObjectRevision::INITIAL,
            source_content_digest,
            PreparedSourceCheckReceipt::ManualSourceCheck(ContentDigest::from_bytes([0x8b; 32])),
            &[5],
        );
        let mut simulation = SimulationState::default();
        simulation.runs = vec![run];
        simulation.next_run_id = 29;

        let mut libraries = LibraryManager::with_primitives();
        let workspace = ProjectWorkspace::new_bootstrapped(&mut libraries);
        let project = ProjectFile::new_with_simulation_results(
            workspace,
            libraries,
            ProjectSimulationResults::from_state(&simulation),
        );

        let json = serialize_project_file(&project)
            .expect("manual-deck result does not require a simulation-plan owner");
        let loaded = load_project_text(&json, None).expect("manual-deck project reloads");
        let restored = loaded
            .simulation_results
            .into_simulation_state()
            .expect("manual-deck result history restores");
        let provenance = restored.runs[0].analyses[0]
            .provenance
            .as_ref()
            .expect("provenance retained");

        assert_eq!(
            provenance.source_domain(),
            AnalysisResultSourceDomain::ManualDeck
        );
        assert_eq!(provenance.source_instance_id(), source_id);
        assert_eq!(provenance.prepared_snapshot_digest(), snapshot);
    }

    #[test]
    fn schema_v4_provenance_migrates_without_guessing_its_source_domain() {
        let source_id = AnalysisInstanceId::new();
        let mut run = SimulationRun::new(30);
        run.add_analysis(
            AnalysisResult::new(1, AnalysisType::Ac, "Legacy prepared AC").with_provenance(
                AnalysisResultProvenance::new(
                    source_id,
                    ObjectRevision::INITIAL,
                    ContentDigest::from_bytes([0x94; 32]),
                    Vec::new(),
                )
                .expect("legacy prepared provenance fixture"),
            ),
        );
        seal_prepared_run(
            &mut run,
            AnalysisResultSourceDomain::SimulationPlan,
            Some(SimulationPlanId::new()),
            ObjectRevision::INITIAL,
            ContentDigest::from_bytes([0x93; 32]),
            PreparedSourceCheckReceipt::SchematicDrc(ContentDigest::from_bytes([0x92; 32])),
            &[2],
        );
        let mut simulation = SimulationState::default();
        simulation.runs = vec![run];
        simulation.next_run_id = 30;
        let mut persisted = ProjectSimulationResults::from_state(&simulation);
        persisted.schema_version = EXPLICIT_PROVENANCE_MODE_RESULTS_SCHEMA_VERSION;
        clear_v6_execution_fields(&mut persisted);
        persisted.runs[0].prepared_receipt = PersistedField::Missing;
        for provenance in persisted.runs[0]
            .analyses
            .iter_mut()
            .filter_map(|analysis| analysis.provenance.as_mut())
        {
            provenance.source_domain = PersistedField::Missing;
        }
        let v4_json = serde_json::to_string(&persisted).expect("schema-v4 fixture serializes");
        assert!(!v4_json.contains("source_domain"));
        let mut persisted: ProjectSimulationResults =
            serde_json::from_str(&v4_json).expect("schema-v4 fixture deserializes");

        persisted
            .migrate_to_current(ProjectId::new())
            .expect("schema v4 migrates");
        persisted.validate().expect("migrated schema validates");

        assert_eq!(
            persisted.runs[0].provenance_mode,
            PersistedField::Value(ProjectRunProvenanceMode::LegacyPreparedUnclassified)
        );
        assert_eq!(
            persisted.runs[0].analyses[0]
                .provenance
                .as_ref()
                .expect("provenance retained")
                .source_domain,
            PersistedField::Value(AnalysisResultSourceDomain::LegacyUnclassified)
        );
    }

    #[test]
    fn legacy_result_schema_truth_cannot_be_repaired_or_downgraded() {
        let source_id = AnalysisInstanceId::new();
        let mut run = SimulationRun::new(33);
        run.add_analysis(
            AnalysisResult::new(1, AnalysisType::Ac, "Prepared AC").with_provenance(
                AnalysisResultProvenance::new(
                    source_id,
                    ObjectRevision::INITIAL,
                    ContentDigest::from_bytes([0xa3; 32]),
                    Vec::new(),
                )
                .expect("prepared provenance fixture"),
            ),
        );
        seal_prepared_run(
            &mut run,
            AnalysisResultSourceDomain::SimulationPlan,
            Some(SimulationPlanId::new()),
            ObjectRevision::INITIAL,
            ContentDigest::from_bytes([0xa2; 32]),
            PreparedSourceCheckReceipt::SchematicDrc(ContentDigest::from_bytes([0xa1; 32])),
            &[2],
        );
        let mut simulation = SimulationState::default();
        simulation.runs = vec![run];
        simulation.next_run_id = 33;
        let mut v4 = ProjectSimulationResults::from_state(&simulation);
        v4.schema_version = EXPLICIT_PROVENANCE_MODE_RESULTS_SCHEMA_VERSION;
        clear_v6_execution_fields(&mut v4);
        v4.runs[0].prepared_receipt = PersistedField::Missing;
        v4.runs[0].provenance_mode =
            PersistedField::Value(ProjectRunProvenanceMode::PreparedTaskBound);
        v4.runs[0].analyses[0]
            .provenance
            .as_mut()
            .expect("provenance")
            .source_domain = PersistedField::Missing;

        let mut stripped = v4.clone();
        stripped.runs[0].analyses[0].provenance = None;
        assert!(
            stripped
                .migrate_to_current(ProjectId::new())
                .expect_err("v4 prepared mode cannot be laundered after provenance stripping")
                .contains("complete provenance is missing")
        );

        let mut contradictory = v4.clone();
        contradictory.runs[0].provenance_mode =
            PersistedField::Value(ProjectRunProvenanceMode::LegacyUnattributed);
        assert!(
            contradictory
                .migrate_to_current(ProjectId::new())
                .expect_err("v4 legacy mode cannot contain prepared provenance")
                .contains("legacy_unattributed")
        );

        let mut missing_mode = v4.clone();
        missing_mode.runs[0].provenance_mode = PersistedField::Missing;
        assert!(
            missing_mode
                .migrate_to_current(ProjectId::new())
                .expect_err("v4 mode is authoritative and required")
                .contains("provenance_mode is required")
        );

        let mut downgraded = v4.clone();
        downgraded.schema_version = PREPARED_PROVENANCE_RESULTS_SCHEMA_VERSION;
        downgraded.runs[0].provenance_mode = PersistedField::Missing;
        downgraded.runs[0].analyses[0]
            .provenance
            .as_mut()
            .expect("provenance")
            .source_domain = PersistedField::Value(AnalysisResultSourceDomain::SimulationPlan);
        assert!(
            downgraded
                .migrate_to_current(ProjectId::new())
                .expect_err("new source-domain data cannot masquerade as schema v3")
                .contains("introduced after schema v3")
        );

        for schema_version in [
            LEGACY_SIMULATION_RESULTS_SCHEMA_VERSION,
            STABLE_DATASET_RESULTS_SCHEMA_VERSION,
        ] {
            let mut impossible = v4.clone();
            impossible.schema_version = schema_version;
            impossible.runs[0].provenance_mode = PersistedField::Missing;
            impossible.runs[0].analyses[0]
                .provenance
                .as_mut()
                .expect("provenance")
                .source_domain = PersistedField::Missing;
            if schema_version == LEGACY_SIMULATION_RESULTS_SCHEMA_VERSION {
                impossible.runs[0].run_id = None;
                impossible.runs[0].dataset_id = None;
                impossible.active_run_stable_id = None;
                impossible.active_dataset_id = None;
                impossible.active_analysis_sequence = None;
            }
            assert!(
                impossible
                    .migrate_to_current(ProjectId::new())
                    .expect_err("v1/v2 cannot contain prepared provenance")
                    .contains("prepared provenance introduced after")
            );
        }
    }

    #[test]
    fn legacy_result_schema_rejects_present_or_null_later_era_fields() {
        let mut run = SimulationRun::new(34);
        run.add_analysis(
            AnalysisResult::new(1, AnalysisType::Ac, "Prepared AC").with_provenance(
                AnalysisResultProvenance::new(
                    AnalysisInstanceId::new(),
                    ObjectRevision::INITIAL,
                    ContentDigest::from_bytes([0xb3; 32]),
                    Vec::new(),
                )
                .expect("prepared provenance fixture"),
            ),
        );
        seal_prepared_run(
            &mut run,
            AnalysisResultSourceDomain::SimulationPlan,
            Some(SimulationPlanId::new()),
            ObjectRevision::INITIAL,
            ContentDigest::from_bytes([0xb2; 32]),
            PreparedSourceCheckReceipt::SchematicDrc(ContentDigest::from_bytes([0xb1; 32])),
            &[2],
        );
        let mut simulation = SimulationState::default();
        simulation.runs = vec![run];
        simulation.next_run_id = 34;
        let current = ProjectSimulationResults::from_state(&simulation);

        for schema_version in [
            LEGACY_SIMULATION_RESULTS_SCHEMA_VERSION,
            STABLE_DATASET_RESULTS_SCHEMA_VERSION,
            PREPARED_PROVENANCE_RESULTS_SCHEMA_VERSION,
            EXPLICIT_PROVENANCE_MODE_RESULTS_SCHEMA_VERSION,
        ] {
            let mut with_receipt = current.clone();
            with_receipt.schema_version = schema_version;
            clear_v6_execution_fields(&mut with_receipt);
            assert!(
                with_receipt
                    .migrate_to_current(ProjectId::new())
                    .expect_err("schemas v1-v4 cannot carry a v5 prepared receipt")
                    .contains("prepared run receipt introduced after")
            );

            let mut null_receipt_json =
                serde_json::to_value(&current).expect("current result document serializes");
            null_receipt_json["schema_version"] = serde_json::json!(schema_version);
            clear_v6_execution_fields_json(&mut null_receipt_json);
            null_receipt_json["runs"][0]["prepared_receipt"] = serde_json::Value::Null;
            let mut with_null_receipt: ProjectSimulationResults =
                serde_json::from_value(null_receipt_json)
                    .expect("explicit null receipt remains parseable evidence");
            assert!(with_null_receipt.runs[0].prepared_receipt.is_null());
            assert!(
                with_null_receipt
                    .migrate_to_current(ProjectId::new())
                    .expect_err("an explicit null receipt is still an anachronistic field")
                    .contains("prepared run receipt introduced after")
            );
        }

        let mut null_v3_mode_json =
            serde_json::to_value(&current).expect("current result document serializes");
        null_v3_mode_json["schema_version"] =
            serde_json::json!(PREPARED_PROVENANCE_RESULTS_SCHEMA_VERSION);
        clear_v6_execution_fields_json(&mut null_v3_mode_json);
        null_v3_mode_json["runs"][0]["provenance_mode"] = serde_json::Value::Null;
        let mut null_v3_mode: ProjectSimulationResults =
            serde_json::from_value(null_v3_mode_json).expect("null mode evidence deserializes");
        null_v3_mode.runs[0].prepared_receipt = PersistedField::Missing;
        assert!(null_v3_mode.runs[0].provenance_mode.is_null());
        null_v3_mode.runs[0].analyses[0]
            .provenance
            .as_mut()
            .expect("provenance")
            .source_domain = PersistedField::Missing;
        assert!(
            null_v3_mode
                .migrate_to_current(ProjectId::new())
                .expect_err("an explicit null mode is present in schema v3")
                .contains("provenance_mode introduced after schema v3")
        );

        let mut null_v4_mode = current.clone();
        null_v4_mode.schema_version = EXPLICIT_PROVENANCE_MODE_RESULTS_SCHEMA_VERSION;
        clear_v6_execution_fields(&mut null_v4_mode);
        null_v4_mode.runs[0].prepared_receipt = PersistedField::Missing;
        null_v4_mode.runs[0].provenance_mode = PersistedField::Null;
        null_v4_mode.runs[0].analyses[0]
            .provenance
            .as_mut()
            .expect("provenance")
            .source_domain = PersistedField::Missing;
        assert!(
            null_v4_mode
                .migrate_to_current(ProjectId::new())
                .expect_err("schema v4 requires a non-null authoritative mode")
                .contains("provenance_mode is required by schema v4")
        );

        let mut null_v4_source_json =
            serde_json::to_value(&current).expect("current result document serializes");
        null_v4_source_json["schema_version"] =
            serde_json::json!(EXPLICIT_PROVENANCE_MODE_RESULTS_SCHEMA_VERSION);
        clear_v6_execution_fields_json(&mut null_v4_source_json);
        null_v4_source_json["runs"][0]["analyses"][0]["provenance"]["source_domain"] =
            serde_json::Value::Null;
        let mut null_v4_source_domain: ProjectSimulationResults =
            serde_json::from_value(null_v4_source_json)
                .expect("null source-domain evidence deserializes");
        null_v4_source_domain.runs[0].prepared_receipt = PersistedField::Missing;
        assert!(
            null_v4_source_domain.runs[0].analyses[0]
                .provenance
                .as_ref()
                .expect("provenance")
                .source_domain
                .is_null()
        );
        assert!(
            null_v4_source_domain
                .migrate_to_current(ProjectId::new())
                .expect_err("an explicit null source domain is still a v5 field")
                .contains("source_domain was introduced after schema v4")
        );
    }

    #[test]
    fn failed_legacy_result_migration_is_transactional() {
        let mut run = SimulationRun::new(35);
        run.add_analysis(AnalysisResult::new(
            1,
            AnalysisType::Transient,
            "Legacy TRAN",
        ));
        seal_legacy_unattributed(&mut run);
        let mut simulation = SimulationState::default();
        simulation.runs = vec![run];
        simulation.next_run_id = 35;
        let mut legacy = ProjectSimulationResults::from_state(&simulation);
        legacy.schema_version = LEGACY_SIMULATION_RESULTS_SCHEMA_VERSION;
        clear_v6_execution_fields(&mut legacy);
        legacy.runs[0].run_id = None;
        legacy.runs[0].dataset_id = None;
        legacy.runs[0].provenance_mode = PersistedField::Missing;
        legacy.active_run_stable_id = None;
        legacy.active_dataset_id = None;
        legacy.active_analysis_sequence = None;
        legacy.active_run_id = Some(999);
        legacy.active_analysis_id = Some(1);
        let before = legacy.clone();

        let error = legacy
            .migrate_to_current(ProjectId::new())
            .expect_err("invalid late selection reference must abort migration");

        assert!(error.contains("run sequence 999 does not exist"));
        assert_eq!(legacy, before, "failed migration must not mutate any field");
    }

    #[test]
    fn schema_v1_result_identity_migration_is_reproducible() {
        let mut run = SimulationRun::new(18);
        run.add_analysis(AnalysisResult::new(
            1,
            AnalysisType::Transient,
            "Legacy TRAN",
        ));
        seal_legacy_unattributed(&mut run);
        let mut simulation = SimulationState::default();
        simulation.runs = vec![run];
        simulation.next_run_id = 18;
        let mut first = ProjectSimulationResults::from_state(&simulation);
        first.schema_version = LEGACY_SIMULATION_RESULTS_SCHEMA_VERSION;
        clear_v6_execution_fields(&mut first);
        first.runs[0].provenance_mode = PersistedField::Missing;
        first.runs[0].run_id = None;
        first.runs[0].dataset_id = None;
        first.active_run_stable_id = None;
        first.active_dataset_id = None;
        first.active_analysis_sequence = None;
        first.active_run_id = Some(18);
        first.active_analysis_id = Some(1);
        let mut second = first.clone();
        let mut other_project = first.clone();

        let project_id = ProjectId::new();
        first
            .migrate_to_current(project_id)
            .expect("first migration succeeds");
        second
            .migrate_to_current(project_id)
            .expect("identical migration succeeds");
        other_project
            .migrate_to_current(ProjectId::new())
            .expect("other project migration succeeds");

        assert_eq!(first.runs[0].run_id, second.runs[0].run_id);
        assert_eq!(first.runs[0].dataset_id, second.runs[0].dataset_id);
        assert_eq!(first.active_run_stable_id, second.active_run_stable_id);
        assert_eq!(first.active_dataset_id, second.active_dataset_id);
        assert_ne!(first.runs[0].run_id, other_project.runs[0].run_id);
        assert_ne!(first.runs[0].dataset_id, other_project.runs[0].dataset_id);
    }

    #[test]
    fn project_save_requires_result_provenance_to_be_closed_over_plan_or_tombstones() {
        let mut project = project_with_execution_context();
        let plan = project
            .execution_context
            .as_ref()
            .expect("execution context")
            .simulation_plan
            .stable_analysis_plan()
            .expect("stable plan");
        let source = plan
            .instances()
            .iter()
            .find(|instance| instance.kind() == AnalysisKind::Ac)
            .expect("AC fixture instance");
        let source_id = source.id();
        let source_revision = plan.revision();
        let snapshot = ContentDigest::from_bytes([0x5c; 32]);

        let mut run = SimulationRun::new(31);
        let run_id = run.run_id;
        run.add_analysis(
            AnalysisResult::new(1, AnalysisType::Ac, "AC").with_provenance(
                AnalysisResultProvenance::new(source_id, source_revision, snapshot, Vec::new())
                    .expect("prepared provenance"),
            ),
        );
        seal_prepared_run(
            &mut run,
            AnalysisResultSourceDomain::SimulationPlan,
            Some(plan.id()),
            project.workspace.project.revision(),
            ContentDigest::from_bytes([0x5b; 32]),
            PreparedSourceCheckReceipt::SchematicDrc(ContentDigest::from_bytes([0x5a; 32])),
            &[2],
        );
        let mut simulation = SimulationState::default();
        simulation.runs = vec![run];
        simulation.next_run_id = 31;
        project.simulation_results = ProjectSimulationResults::from_state(&simulation);

        serialize_project_file(&project).expect("current plan owns result source");

        let mut future_revision = project.clone();
        let future_source_revision =
            ObjectRevision::new(source_revision.get() + 1).expect("future fixture revision");
        future_revision.simulation_results.runs[0].analyses[0]
            .provenance
            .as_mut()
            .expect("provenance")
            .source_revision = future_source_revision;
        future_revision.simulation_results.runs[0]
            .prepared_receipt
            .as_mut()
            .expect("receipt")
            .tasks[0]
            .source_revision = future_source_revision;
        assert!(
            serialize_project_file(&future_revision)
                .expect_err("result revision beyond the retained plan must block save")
                .to_string()
                .contains("outside the retained analysis revision interval")
        );

        let mut missing = project.clone();
        let orphaned_source_id = AnalysisInstanceId::new();
        missing.simulation_results.runs[0].analyses[0]
            .provenance
            .as_mut()
            .expect("provenance")
            .source_instance_id = orphaned_source_id;
        missing.simulation_results.runs[0]
            .prepared_receipt
            .as_mut()
            .expect("receipt")
            .tasks[0]
            .source_instance_id = orphaned_source_id;
        assert!(
            serialize_project_file(&missing)
                .expect_err("orphaned prepared result must block save")
                .to_string()
                .contains("absent from the persisted plan and its tombstones")
        );

        let mut retained = project.clone();
        retained
            .execution_context
            .as_mut()
            .expect("execution context")
            .simulation_plan
            .stable_analysis_plan_mut()
            .expect("stable plan")
            .remove(source_id, vec![run_id])
            .expect("remove with exact retained run");
        serialize_project_file(&retained).expect("tombstone closes retained result reference");

        let removed_revision = retained
            .execution_context
            .as_ref()
            .expect("execution context")
            .simulation_plan
            .stable_analysis_plan()
            .expect("stable plan")
            .tombstones()
            .iter()
            .find(|tombstone| tombstone.id() == source_id)
            .expect("source tombstone")
            .removed_revision();
        let mut at_removal = retained.clone();
        at_removal.simulation_results.runs[0].analyses[0]
            .provenance
            .as_mut()
            .expect("provenance")
            .source_revision = removed_revision;
        at_removal.simulation_results.runs[0]
            .prepared_receipt
            .as_mut()
            .expect("receipt")
            .tasks[0]
            .source_revision = removed_revision;
        assert!(
            serialize_project_file(&at_removal)
                .expect_err("a result cannot be produced at the removal revision")
                .to_string()
                .contains("outside the retained analysis revision interval")
        );

        let mut unretained = project;
        unretained
            .execution_context
            .as_mut()
            .expect("execution context")
            .simulation_plan
            .stable_analysis_plan_mut()
            .expect("stable plan")
            .remove(source_id, Vec::new())
            .expect("remove without retained run");
        assert!(
            serialize_project_file(&unretained)
                .expect_err("tombstone must retain the exact historical run")
                .to_string()
                .contains("is not retained by its tombstone")
        );
    }

    #[test]
    fn project_save_rejects_result_revision_before_source_creation() {
        let mut project = project_with_execution_context();
        let (source_id, plan_id, created_revision) = {
            let plan = project
                .execution_context
                .as_mut()
                .expect("execution context")
                .simulation_plan
                .stable_analysis_plan_mut()
                .expect("stable plan");
            let (source_id, _) = plan
                .insert(AnalysisKind::Ac)
                .expect("independent AC source inserts");
            let source = plan.instance(source_id).expect("inserted source");
            (source_id, plan.id(), source.created_revision())
        };
        assert!(created_revision.get() > ObjectRevision::INITIAL.get());
        let before_creation = ObjectRevision::new(created_revision.get() - 1)
            .expect("revision immediately before creation exists");
        let snapshot = ContentDigest::from_bytes([0xc3; 32]);
        let mut run = SimulationRun::new(36);
        run.add_analysis(
            AnalysisResult::new(1, AnalysisType::Ac, "Premature AC").with_provenance(
                AnalysisResultProvenance::new(source_id, before_creation, snapshot, Vec::new())
                    .expect("prepared provenance"),
            ),
        );
        seal_prepared_run(
            &mut run,
            AnalysisResultSourceDomain::SimulationPlan,
            Some(plan_id),
            project.workspace.project.revision(),
            ContentDigest::from_bytes([0xc2; 32]),
            PreparedSourceCheckReceipt::SchematicDrc(ContentDigest::from_bytes([0xc1; 32])),
            &[2],
        );
        let mut simulation = SimulationState::default();
        simulation.runs = vec![run];
        simulation.next_run_id = 36;
        project.simulation_results = ProjectSimulationResults::from_state(&simulation);

        let error = serialize_project_file(&project)
            .expect_err("source cannot own results from before it existed")
            .to_string();
        assert!(
            error.contains("outside the retained analysis revision interval"),
            "{error}"
        );
    }

    #[test]
    fn v2_same_kind_results_migrate_without_guessing_source_identity() {
        let mut run = SimulationRun::new(22);
        run.add_analysis(AnalysisResult::new(1, AnalysisType::Ac, "AC low band"));
        run.add_analysis(AnalysisResult::new(2, AnalysisType::Ac, "AC high band"));
        seal_legacy_unattributed(&mut run);
        let mut simulation = SimulationState::default();
        simulation.runs = vec![run];
        simulation.next_run_id = 22;
        let mut persisted = ProjectSimulationResults::from_state(&simulation);
        persisted.schema_version = STABLE_DATASET_RESULTS_SCHEMA_VERSION;
        clear_v6_execution_fields(&mut persisted);
        persisted.runs[0].provenance_mode = PersistedField::Missing;

        persisted
            .migrate_to_current(ProjectId::new())
            .expect("v2 migrates");

        assert_eq!(
            persisted.schema_version,
            PROJECT_SIMULATION_RESULTS_SCHEMA_VERSION
        );
        assert!(
            persisted.runs[0]
                .analyses
                .iter()
                .all(|analysis| analysis.provenance.is_none())
        );
        let restored = persisted
            .into_simulation_state()
            .expect("legacy absence remains valid");
        assert!(
            restored.runs[0]
                .analyses
                .iter()
                .all(|analysis| analysis.provenance.is_none())
        );
    }

    #[test]
    fn prepared_result_provenance_validation_rejects_aliases_and_partial_history() {
        let first_id = AnalysisInstanceId::new();
        let second_id = AnalysisInstanceId::new();
        let snapshot = ContentDigest::from_bytes([0x61; 32]);
        let mut run = SimulationRun::new(23);
        for (sequence, source_id, label) in [(1, first_id, "AC one"), (2, second_id, "AC two")] {
            run.add_analysis(
                AnalysisResult::new(sequence, AnalysisType::Ac, label).with_provenance(
                    AnalysisResultProvenance::new(
                        source_id,
                        ObjectRevision::INITIAL,
                        snapshot,
                        Vec::new(),
                    )
                    .expect("fixture provenance"),
                ),
            );
        }
        seal_prepared_run(
            &mut run,
            AnalysisResultSourceDomain::SimulationPlan,
            Some(SimulationPlanId::new()),
            ObjectRevision::INITIAL,
            ContentDigest::from_bytes([0x60; 32]),
            PreparedSourceCheckReceipt::SchematicDrc(ContentDigest::from_bytes([0x5f; 32])),
            &[2, 2],
        );
        let mut simulation = SimulationState::default();
        simulation.runs = vec![run];
        simulation.next_run_id = 23;
        let baseline = ProjectSimulationResults::from_state(&simulation);

        let mut aliased = baseline.clone();
        aliased.runs[0].analyses[1]
            .provenance
            .as_mut()
            .expect("second provenance")
            .source_instance_id = first_id;
        assert!(
            aliased
                .validate()
                .expect_err("source aliases fail closed")
                .contains("duplicates prepared analysis instance")
        );

        let mut forward_dependency = baseline.clone();
        forward_dependency.runs[0].analyses[0]
            .provenance
            .as_mut()
            .expect("first provenance")
            .dependency_ids = vec![second_id];
        assert!(
            forward_dependency
                .validate()
                .expect_err("dependencies must follow frozen execution order")
                .contains("before that result appears")
        );

        let mut partial = baseline.clone();
        partial.runs[0].analyses[1].provenance = None;
        assert!(
            partial
                .validate()
                .expect_err("legacy/current mixing fails closed")
                .contains("is prepared_task_bound")
        );

        let mut stripped = baseline;
        for analysis in &mut stripped.runs[0].analyses {
            analysis.provenance = None;
        }
        assert!(
            stripped
                .validate()
                .expect_err("current-schema provenance cannot disappear wholesale")
                .contains("is prepared_task_bound")
        );
    }

    #[test]
    fn project_simulation_results_omits_empty_history_after_cleared_runs() {
        let mut simulation = SimulationState::default();
        simulation.start_run();
        simulation.clear_runs();

        let results = ProjectSimulationResults::from_state(&simulation);

        assert!(results.is_empty());
    }

    #[test]
    fn project_text_load_drops_invalid_simulation_results_without_rejecting_workspace() {
        let mut libraries = LibraryManager::with_primitives();
        let workspace = ProjectWorkspace::new_bootstrapped(&mut libraries);
        let mut simulation = SimulationState::default();
        let mut run = SimulationRun::new(1);
        run.add_analysis(AnalysisResult::new(1, AnalysisType::Transient, "TRAN"));
        simulation.runs = vec![run];
        simulation.next_run_id = 1;
        simulation.active_run_idx = Some(0);
        simulation.active_analysis_idx = Some(0);
        let project = ProjectFile::new_with_simulation_results(
            workspace,
            libraries,
            ProjectSimulationResults::from_state(&simulation),
        );
        let mut value = serde_json::to_value(project).expect("project converts to JSON");
        value["simulation_results"]["schema_version"] = serde_json::Value::from(999);
        let json = serde_json::to_string_pretty(&value).expect("fixture serializes");

        let loaded = load_project_text(&json, None).expect("workspace still loads");

        assert!(loaded.simulation_results.is_empty());
        assert!(
            loaded
                .simulation_results_warning
                .as_deref()
                .unwrap_or_default()
                .contains("unsupported simulation results schema version")
        );
    }

    #[test]
    fn project_load_clears_legacy_regression_baseline_after_result_migration() {
        let mut project = project_with_execution_context();
        let plan_id = project
            .execution_context
            .as_ref()
            .expect("execution context")
            .simulation_plan
            .stable_analysis_plan()
            .expect("stable plan")
            .id();
        let mut run = SimulationRun::new(71);
        run.add_analysis(AnalysisResult::new(1, AnalysisType::Ac, "legacy AC"));
        seal_legacy_unattributed(&mut run);
        let baseline_id = run.run_id;
        let mut simulation = SimulationState::default();
        simulation.runs = vec![run];
        simulation.next_run_id = 71;
        project.simulation_results = ProjectSimulationResults::from_state(&simulation);
        project
            .workspace
            .active_plan_data_mut(plan_id)
            .expect("active plan payload")
            .regression_baseline_run = Some(baseline_id);

        let json = serde_json::to_string_pretty(&project).expect("legacy baseline fixture");
        let loaded = load_project_text(&json, None).expect("project remains loadable");

        assert_eq!(loaded.simulation_results.runs.len(), 1);
        assert!(
            loaded
                .workspace
                .active_plan_data(plan_id)
                .expect("active plan payload")
                .regression_baseline_run
                .is_none()
        );
        assert!(
            loaded
                .simulation_results_warning
                .as_deref()
                .unwrap_or_default()
                .contains("not eligible")
        );
        serialize_project_file(&loaded).expect("cleaned project reserializes");
    }

    #[test]
    fn project_load_authenticates_v11_noise_and_preserves_eligible_regression_baseline() {
        let mut project = project_with_execution_context();
        let (plan_id, source_id, source_revision, dependencies) = {
            let plan = project
                .execution_context
                .as_ref()
                .expect("execution context")
                .simulation_plan
                .stable_analysis_plan()
                .expect("stable plan");
            let source = plan
                .instances()
                .iter()
                .find(|instance| instance.kind() == AnalysisKind::Noise)
                .expect("noise fixture instance");
            (
                plan.id(),
                source.id(),
                source.modified_revision(),
                source
                    .dependencies()
                    .iter()
                    .map(|dependency| dependency.target())
                    .collect::<Vec<_>>(),
            )
        };
        let snapshot = ContentDigest::from_bytes([0xd1; 32]);
        let summary = NoiseSummary {
            rows: vec![NoiseContributorRow {
                device: "R1".to_owned(),
                mechanism: "thermal".to_owned(),
                power: 2.5e-12,
                share_pct: 100.0,
            }],
            total_rms: Some(1.25e-6),
            input_rms: None,
            band: (1.0, 1.0e6),
        };
        let mut run = SimulationRun::new(72);
        run.mark_running().expect("fixture run starts");
        run.finish_lifecycle(SimulationRunLifecycle::Completed)
            .expect("fixture run completes");
        run.add_analysis(
            AnalysisResult::new(1, AnalysisType::Noise, "NOISE")
                .with_noise_summary(summary.clone())
                .with_provenance(
                    AnalysisResultProvenance::new(
                        source_id,
                        source_revision,
                        snapshot,
                        dependencies,
                    )
                    .expect("noise provenance"),
                ),
        );
        seal_prepared_run(
            &mut run,
            AnalysisResultSourceDomain::SimulationPlan,
            Some(plan_id),
            project.workspace.project.revision(),
            ContentDigest::from_bytes([0xd2; 32]),
            PreparedSourceCheckReceipt::SchematicDrc(ContentDigest::from_bytes([0xd3; 32])),
            &[analysis_kind_tag_for_plan_kind(AnalysisKind::Noise)],
        );
        let baseline_id = run.run_id;
        let legacy_analysis_digest = run.analyses[0].legacy_v4_result_data_digest();
        let legacy_dataset_digest = run.legacy_v4_dataset_content_digest();
        let mut simulation = SimulationState::default();
        simulation.runs = vec![run];
        simulation.next_run_id = 72;
        simulation.active_run_idx = Some(0);
        simulation.active_analysis_idx = Some(0);
        project.simulation_results = ProjectSimulationResults::from_state(&simulation);
        project
            .workspace
            .active_plan_data_mut(plan_id)
            .expect("active plan payload")
            .regression_baseline_run = Some(baseline_id);

        let current_json = serialize_project_file(&project).expect("current project serializes");
        let mut v11: serde_json::Value = serde_json::from_str(&current_json).expect("project JSON");
        v11["simulation_results"]["schema_version"] =
            serde_json::json!(TRANSFER_FUNCTION_RESULTS_SCHEMA_VERSION);
        v11["simulation_results"]["runs"][0]["analyses"][0]["result_data_digest"] =
            serde_json::to_value(legacy_analysis_digest).expect("legacy analysis digest");
        v11["simulation_results"]["runs"][0]["dataset_content_digest"] =
            serde_json::to_value(legacy_dataset_digest).expect("legacy dataset digest");
        v11["simulation_results"]["runs"][0]["analyses"][0]["noise_summary"]
            .as_object_mut()
            .expect("noise summary object")
            .remove("input_rms");

        let loaded = load_project_text(&v11.to_string(), None)
            .expect("authentic schema-v11 project remains loadable");

        assert_eq!(
            loaded.simulation_results.schema_version,
            PROJECT_SIMULATION_RESULTS_SCHEMA_VERSION
        );
        assert_eq!(loaded.simulation_results.runs.len(), 1);
        assert!(loaded.simulation_results_warning.is_none());
        assert_eq!(
            loaded
                .workspace
                .active_plan_data(plan_id)
                .expect("active plan payload")
                .regression_baseline_run,
            Some(baseline_id)
        );
        let restored = loaded.simulation_results.runs[0]
            .clone()
            .into_run()
            .expect("migrated result restores");
        assert_eq!(restored.analyses[0].noise_summary, Some(summary));
        assert_ne!(
            restored.analyses[0].result_data_digest(),
            legacy_analysis_digest,
            "v11 evidence must be resealed in the v12 digest domain"
        );
        serialize_project_file(&loaded).expect("migrated project reserializes");

        let mut tampered_v11 = v11;
        tampered_v11["simulation_results"]["runs"][0]["analyses"][0]["noise_summary"]["total_rms"] =
            serde_json::json!(1.250_000_000_000_000_3e-6_f64);
        let rejected = load_project_text(&tampered_v11.to_string(), None)
            .expect("a bad result digest must not reject unrelated project documents");
        assert!(rejected.simulation_results.is_empty());
        assert!(
            rejected
                .workspace
                .active_plan_data(plan_id)
                .expect("active plan payload")
                .regression_baseline_run
                .is_none()
        );
        assert!(
            rejected
                .simulation_results_warning
                .as_deref()
                .unwrap_or_default()
                .contains(
                    "schema-v11 analysis 1 result data digest does not match retained content"
                )
        );
    }

    #[test]
    fn project_load_clears_dangling_regression_baseline_without_rejecting_project() {
        let mut project = project_with_execution_context();
        let plan_id = project
            .execution_context
            .as_ref()
            .expect("execution context")
            .simulation_plan
            .stable_analysis_plan()
            .expect("stable plan")
            .id();
        project
            .workspace
            .active_plan_data_mut(plan_id)
            .expect("active plan payload")
            .regression_baseline_run = Some(crate::product::RunId::new());

        let json = serde_json::to_string_pretty(&project).expect("dangling baseline fixture");
        let loaded = load_project_text(&json, None).expect("project remains loadable");

        assert!(loaded.simulation_results.is_empty());
        assert!(
            loaded
                .workspace
                .active_plan_data(plan_id)
                .expect("active plan payload")
                .regression_baseline_run
                .is_none()
        );
        assert!(
            loaded
                .simulation_results_warning
                .as_deref()
                .unwrap_or_default()
                .contains("absent from retained result history")
        );
        serialize_project_file(&loaded).expect("cleaned project reserializes");
    }

    #[test]
    fn project_text_load_drops_unknown_analysis_type_results_without_parse_failure() {
        let mut libraries = LibraryManager::with_primitives();
        let workspace = ProjectWorkspace::new_bootstrapped(&mut libraries);
        let mut simulation = SimulationState::default();
        let mut run = SimulationRun::new(1);
        run.add_analysis(AnalysisResult::new(1, AnalysisType::Ac, "AC"));
        seal_legacy_unattributed(&mut run);
        simulation.runs = vec![run];
        simulation.next_run_id = 1;
        simulation.active_run_idx = Some(0);
        simulation.active_analysis_idx = Some(0);
        let project = ProjectFile::new_with_simulation_results(
            workspace,
            libraries,
            ProjectSimulationResults::from_state(&simulation),
        );
        let json = serialize_project_file(&project)
            .expect("project serializes")
            .replace(
                "\"analysis_type\": \"Ac\"",
                "\"analysis_type\": \"FutureAnalysis\"",
            );

        let loaded = load_project_text(&json, None).expect("workspace still loads");

        assert!(loaded.simulation_results.is_empty());
        assert!(
            loaded
                .simulation_results_warning
                .as_deref()
                .unwrap_or_default()
                .contains("unknown analysis type")
        );
    }

    #[test]
    fn project_results_restore_rejects_invalid_overlay_references() {
        let mut run_one = SimulationRun::new(1);
        let mut run_two = SimulationRun::new(2);
        seal_legacy_unattributed(&mut run_one);
        seal_legacy_unattributed(&mut run_two);
        let results = ProjectSimulationResults {
            schema_version: PROJECT_SIMULATION_RESULTS_SCHEMA_VERSION,
            runs: vec![
                ProjectSimulationRun::from(&run_one),
                ProjectSimulationRun::from(&run_two),
            ],
            next_run_id: 2,
            active_run_stable_id: Some(run_one.run_id),
            active_dataset_id: Some(run_one.dataset_id),
            active_analysis_sequence: None,
            overlay_dataset_ids: vec![
                run_two.dataset_id,
                run_two.dataset_id,
                run_one.dataset_id,
                DatasetId::new(),
            ],
            active_run_id: None,
            active_analysis_id: None,
            overlay_run_ids: Vec::new(),
        };

        let error = results
            .into_simulation_state()
            .expect_err("invalid overlay references fail closed");

        assert!(error.contains("duplicate overlay dataset id"));
    }

    #[test]
    fn project_results_validation_rejects_duplicate_run_ids() {
        let mut run_one = SimulationRun::new(1);
        let mut run_duplicate = SimulationRun::new(1);
        seal_legacy_unattributed(&mut run_one);
        seal_legacy_unattributed(&mut run_duplicate);
        let results = ProjectSimulationResults {
            schema_version: PROJECT_SIMULATION_RESULTS_SCHEMA_VERSION,
            runs: vec![
                ProjectSimulationRun::from(&run_one),
                ProjectSimulationRun::from(&run_duplicate),
            ],
            next_run_id: 1,
            active_run_stable_id: Some(run_one.run_id),
            active_dataset_id: Some(run_one.dataset_id),
            active_analysis_sequence: None,
            overlay_dataset_ids: Vec::new(),
            active_run_id: None,
            active_analysis_id: None,
            overlay_run_ids: Vec::new(),
        };

        let error = results.validate().expect_err("duplicate run ids fail");

        assert!(error.contains("duplicate simulation run id 1"));
    }

    #[test]
    fn project_results_v2_requires_unique_stable_run_and_dataset_ids() {
        let mut run_one = SimulationRun::new(1);
        run_one.add_analysis(AnalysisResult::new(1, AnalysisType::Transient, "TRAN one"));
        let mut run_two = SimulationRun::new(2);
        run_two.add_analysis(AnalysisResult::new(1, AnalysisType::Transient, "TRAN two"));
        seal_legacy_unattributed(&mut run_one);
        seal_legacy_unattributed(&mut run_two);
        let mut simulation = SimulationState::default();
        simulation.runs = vec![run_one, run_two];
        simulation.next_run_id = 2;

        let baseline = ProjectSimulationResults::from_state(&simulation);

        let mut missing_run_identity = baseline.clone();
        missing_run_identity.runs[0].run_id = None;
        let error = missing_run_identity
            .validate()
            .expect_err("schema v2 must not regenerate a missing run id");
        assert!(error.contains("run_id is required"));

        let mut missing_dataset_identity = baseline.clone();
        missing_dataset_identity.runs[0].dataset_id = None;
        let error = missing_dataset_identity
            .validate()
            .expect_err("schema v2 must not regenerate a missing dataset id");
        assert!(error.contains("dataset_id is required"));

        let mut duplicate_run_identity = baseline.clone();
        duplicate_run_identity.runs[1].run_id = duplicate_run_identity.runs[0].run_id;
        let error = duplicate_run_identity
            .validate()
            .expect_err("stable run ids are globally unique");
        assert!(error.contains("duplicate stable simulation run id"));

        let mut duplicate_dataset_identity = baseline;
        duplicate_dataset_identity.runs[1].dataset_id =
            duplicate_dataset_identity.runs[0].dataset_id;
        let error = duplicate_dataset_identity
            .validate()
            .expect_err("dataset ids are globally unique");
        assert!(error.contains("duplicate immutable dataset id"));
    }

    #[test]
    fn project_results_v2_rejects_cross_bound_selection_and_active_overlay() {
        let mut run_one = SimulationRun::new(1);
        run_one.add_analysis(AnalysisResult::new(3, AnalysisType::Transient, "TRAN one"));
        let mut run_two = SimulationRun::new(2);
        run_two.add_analysis(AnalysisResult::new(8, AnalysisType::Ac, "AC two"));
        seal_legacy_unattributed(&mut run_one);
        seal_legacy_unattributed(&mut run_two);
        let run_one_dataset_id = run_one.dataset_id;
        let run_two_dataset_id = run_two.dataset_id;
        let mut simulation = SimulationState::default();
        simulation.runs = vec![run_one, run_two];
        simulation.next_run_id = 2;
        simulation.active_run_idx = Some(0);
        simulation.active_analysis_idx = Some(0);
        let baseline = ProjectSimulationResults::from_state(&simulation);

        let mut cross_bound = baseline.clone();
        cross_bound.active_dataset_id = Some(run_two_dataset_id);
        let error = cross_bound
            .validate()
            .expect_err("a dataset cannot be rebound to a different active run");
        assert!(error.contains("does not belong to active run"));

        let mut missing_analysis = baseline.clone();
        missing_analysis.active_analysis_sequence = Some(999);
        let error = missing_analysis
            .validate()
            .expect_err("the selected analysis must belong to the active dataset");
        assert!(error.contains("does not exist in active dataset"));

        let mut active_overlay = baseline;
        active_overlay.overlay_dataset_ids = vec![run_one_dataset_id];
        let error = active_overlay
            .validate()
            .expect_err("the active dataset cannot also be an overlay");
        assert!(error.contains("cannot also be an overlay"));
    }

    #[test]
    fn project_text_migrates_v1_result_sequences_once_to_stable_identities() {
        let mut libraries = LibraryManager::with_primitives();
        let workspace = ProjectWorkspace::new_bootstrapped(&mut libraries);
        let mut run_one = SimulationRun::new(1);
        run_one.add_analysis(AnalysisResult::new(
            4,
            AnalysisType::Transient,
            "TRAN legacy one",
        ));
        let mut run_two = SimulationRun::new(2);
        run_two.add_analysis(AnalysisResult::new(9, AnalysisType::Ac, "AC legacy two"));
        let overlay_dataset_id = run_one.dataset_id;
        let mut simulation = SimulationState::default();
        simulation.runs = vec![run_one, run_two];
        simulation.next_run_id = 2;
        simulation.active_run_idx = Some(1);
        simulation.active_analysis_idx = Some(0);
        simulation.overlay_dataset_ids = vec![overlay_dataset_id];
        let project = ProjectFile::new_with_simulation_results(
            workspace,
            libraries,
            ProjectSimulationResults::from_state(&simulation),
        );
        let mut value = serde_json::to_value(project).expect("project converts to JSON");
        let results = value["simulation_results"]
            .as_object_mut()
            .expect("simulation result object");
        results.insert(
            "schema_version".to_owned(),
            serde_json::Value::from(LEGACY_SIMULATION_RESULTS_SCHEMA_VERSION),
        );
        results.remove("active_run_stable_id");
        results.remove("active_dataset_id");
        results.remove("active_analysis_sequence");
        results.remove("overlay_dataset_ids");
        results.insert("active_run_id".to_owned(), serde_json::Value::from(2));
        results.insert("active_analysis_id".to_owned(), serde_json::Value::from(9));
        results.insert("overlay_run_ids".to_owned(), serde_json::json!([1]));
        for run in results["runs"].as_array_mut().expect("legacy run array") {
            let run = run.as_object_mut().expect("legacy run object");
            run.remove("run_id");
            run.remove("dataset_id");
            run.remove("job_id");
            run.remove("execution_target");
            run.remove("lifecycle");
            run.remove("dataset_content_digest");
            for analysis in run["analyses"]
                .as_array_mut()
                .expect("legacy analysis array")
            {
                analysis
                    .as_object_mut()
                    .expect("legacy analysis object")
                    .remove("result_data_digest");
            }
            run.remove("provenance_mode");
        }

        let mut unversioned_value = value.clone();
        unversioned_value["simulation_results"]
            .as_object_mut()
            .expect("unversioned result object")
            .remove("schema_version");
        let unversioned_json = serde_json::to_string_pretty(&unversioned_value)
            .expect("unversioned legacy fixture serializes");
        let unversioned = load_project_text(&unversioned_json, None)
            .expect("unversioned legacy project migrates as v1");
        assert!(unversioned.simulation_results_warning.is_none());
        assert_eq!(
            unversioned.simulation_results.schema_version,
            PROJECT_SIMULATION_RESULTS_SCHEMA_VERSION
        );

        let legacy_json = serde_json::to_string_pretty(&value).expect("legacy fixture serializes");
        let migrated = load_project_text(&legacy_json, None).expect("legacy project migrates");

        assert!(migrated.simulation_results_warning.is_none());
        assert_eq!(
            migrated.simulation_results.schema_version,
            PROJECT_SIMULATION_RESULTS_SCHEMA_VERSION
        );
        assert!(
            migrated
                .simulation_results
                .runs
                .iter()
                .all(|run| run.run_id.is_some() && run.dataset_id.is_some())
        );
        let active_run_id = migrated.simulation_results.runs[1]
            .run_id
            .expect("migrated active run id");
        let active_dataset_id = migrated.simulation_results.runs[1]
            .dataset_id
            .expect("migrated active dataset id");
        let migrated_overlay_id = migrated.simulation_results.runs[0]
            .dataset_id
            .expect("migrated overlay id");
        assert_eq!(
            migrated.simulation_results.active_run_stable_id,
            Some(active_run_id)
        );
        assert_eq!(
            migrated.simulation_results.active_dataset_id,
            Some(active_dataset_id)
        );
        assert_eq!(
            migrated.simulation_results.active_analysis_sequence,
            Some(9)
        );
        assert_eq!(
            migrated.simulation_results.overlay_dataset_ids,
            vec![migrated_overlay_id]
        );
        assert!(migrated.simulation_results.active_run_id.is_none());
        assert!(migrated.simulation_results.active_analysis_id.is_none());
        assert!(migrated.simulation_results.overlay_run_ids.is_empty());

        let current_json = serialize_project_file(&migrated).expect("migration persists");
        let reloaded = load_project_text(&current_json, None).expect("migrated project reloads");
        assert_eq!(
            reloaded.simulation_results.active_run_stable_id,
            Some(active_run_id)
        );
        assert_eq!(
            reloaded.simulation_results.active_dataset_id,
            Some(active_dataset_id)
        );
        assert_eq!(
            reloaded.simulation_results.active_analysis_sequence,
            Some(9)
        );
        assert_eq!(
            reloaded.simulation_results.overlay_dataset_ids,
            vec![migrated_overlay_id]
        );
    }

    #[test]
    fn project_results_validation_rejects_duplicate_waveform_names_in_analysis() {
        let run_id = RunId::new();
        let dataset_id = DatasetId::new();
        let results = ProjectSimulationResults {
            schema_version: PROJECT_SIMULATION_RESULTS_SCHEMA_VERSION,
            runs: vec![ProjectSimulationRun {
                job_id: None,
                run_id: Some(run_id),
                dataset_id: Some(dataset_id),
                execution_target: None,
                lifecycle: Some(SimulationRunLifecycle::LegacyUnknown),
                id: 1,
                label: "Run 1".to_string(),
                timestamp: 1.0,
                analyses: vec![ProjectAnalysisResult {
                    id: 1,
                    analysis_type: "Transient".to_string(),
                    label: "TRAN".to_string(),
                    timestamp: 1.0,
                    result_data_digest: PersistedField::Missing,
                    waveforms: vec![
                        ProjectWaveformData {
                            name: "V(out)".to_string(),
                            x: vec![0.0],
                            y: vec![1.0],
                            color: "#00aaff".to_string(),
                            visible: true,
                            complex: None,
                        },
                        ProjectWaveformData {
                            name: "V(out)".to_string(),
                            x: vec![0.0],
                            y: vec![2.0],
                            color: "#ffaa00".to_string(),
                            visible: true,
                            complex: None,
                        },
                    ],
                    dc_op: None,
                    device_op: None,
                    noise_summary: None,
                    family_metadata: None,
                    result_payload: PersistedField::Missing,
                    measurements: Vec::new(),
                    saved_output_receipts: Vec::new(),
                    success: true,
                    error_message: None,
                    provenance: None,
                }],
                dataset_content_digest: PersistedField::Missing,
                provenance_mode: PersistedField::Value(
                    ProjectRunProvenanceMode::LegacyUnattributed,
                ),
                prepared_receipt: PersistedField::Missing,
                elapsed_time: 0.1,
                success: true,
            }],
            next_run_id: 1,
            active_run_stable_id: Some(run_id),
            active_dataset_id: Some(dataset_id),
            active_analysis_sequence: Some(1),
            overlay_dataset_ids: Vec::new(),
            active_run_id: None,
            active_analysis_id: None,
            overlay_run_ids: Vec::new(),
        };

        let error = results
            .validate()
            .expect_err("duplicate waveform names in an analysis fail");

        assert!(error.contains("runs[0].analyses[0]"));
        assert!(error.contains("duplicate waveform name 'V(out)'"));
    }

    #[test]
    fn project_results_validation_rejects_non_monotonic_waveform_x() {
        let run_id = RunId::new();
        let dataset_id = DatasetId::new();
        let results = ProjectSimulationResults {
            schema_version: PROJECT_SIMULATION_RESULTS_SCHEMA_VERSION,
            runs: vec![ProjectSimulationRun {
                job_id: None,
                run_id: Some(run_id),
                dataset_id: Some(dataset_id),
                execution_target: None,
                lifecycle: Some(SimulationRunLifecycle::LegacyUnknown),
                id: 1,
                label: "Run 1".to_string(),
                timestamp: 1.0,
                analyses: vec![ProjectAnalysisResult {
                    id: 1,
                    analysis_type: "Transient".to_string(),
                    label: "TRAN".to_string(),
                    timestamp: 1.0,
                    result_data_digest: PersistedField::Missing,
                    waveforms: vec![ProjectWaveformData {
                        name: "V(out)".to_string(),
                        x: vec![0.0, 2.0, 1.0, 3.0],
                        y: vec![0.0, 1.0, 2.0, 3.0],
                        color: "#00aaff".to_string(),
                        visible: true,
                        complex: None,
                    }],
                    dc_op: None,
                    device_op: None,
                    noise_summary: None,
                    family_metadata: None,
                    result_payload: PersistedField::Missing,
                    measurements: Vec::new(),
                    saved_output_receipts: Vec::new(),
                    success: true,
                    error_message: None,
                    provenance: None,
                }],
                dataset_content_digest: PersistedField::Missing,
                provenance_mode: PersistedField::Value(
                    ProjectRunProvenanceMode::LegacyUnattributed,
                ),
                prepared_receipt: PersistedField::Missing,
                elapsed_time: 0.1,
                success: true,
            }],
            next_run_id: 1,
            active_run_stable_id: Some(run_id),
            active_dataset_id: Some(dataset_id),
            active_analysis_sequence: Some(1),
            overlay_dataset_ids: Vec::new(),
            active_run_id: None,
            active_analysis_id: None,
            overlay_run_ids: Vec::new(),
        };

        let error = results
            .validate()
            .expect_err("non-monotonic waveform x data must fail");

        assert!(error.contains("runs[0].analyses[0].waveforms[0].x"));
        assert!(error.contains("monotonic"));
    }

    #[test]
    fn legacy_noise_total_rms_migrates_losslessly_to_optional_evidence() {
        let legacy = r#"{"rows":[],"total_rms":1.25e-6,"band":[1.0,1000.0]}"#;
        let restored: ProjectNoiseSummary =
            serde_json::from_str(legacy).expect("legacy noise summary decodes");
        assert_eq!(restored.total_rms, Some(1.25e-6));
        assert_eq!(restored.input_rms, None);
        assert_eq!(restored.into_noise_summary().total_rms, Some(1.25e-6));
    }

    #[test]
    fn project_results_preserve_core_noise_mechanism_labels() {
        let run_id = RunId::new();
        let dataset_id = DatasetId::new();
        let mut results = ProjectSimulationResults {
            schema_version: PROJECT_SIMULATION_RESULTS_SCHEMA_VERSION,
            runs: vec![ProjectSimulationRun {
                job_id: None,
                run_id: Some(run_id),
                dataset_id: Some(dataset_id),
                execution_target: None,
                lifecycle: Some(SimulationRunLifecycle::LegacyUnknown),
                id: 1,
                label: "Run 1".to_string(),
                timestamp: 1.0,
                analyses: vec![ProjectAnalysisResult {
                    id: 1,
                    analysis_type: "Noise".to_string(),
                    label: "NOISE".to_string(),
                    timestamp: 1.0,
                    result_data_digest: PersistedField::Missing,
                    waveforms: Vec::new(),
                    dc_op: None,
                    device_op: None,
                    noise_summary: Some(ProjectNoiseSummary {
                        rows: vec![
                            ProjectNoiseContributorRow {
                                device: "BNOISE1".to_string(),
                                mechanism: "white".to_string(),
                                power: 1.0e-18,
                                share_pct: 60.0,
                            },
                            ProjectNoiseContributorRow {
                                device: "ATABLE1".to_string(),
                                mechanism: "table".to_string(),
                                power: 2.0e-18,
                                share_pct: 40.0,
                            },
                        ],
                        total_rms: Some(1.0e-6),
                        input_rms: None,
                        band: (1.0, 1.0e6),
                    }),
                    family_metadata: None,
                    result_payload: PersistedField::Missing,
                    measurements: Vec::new(),
                    saved_output_receipts: Vec::new(),
                    success: true,
                    error_message: None,
                    provenance: None,
                }],
                dataset_content_digest: PersistedField::Missing,
                provenance_mode: PersistedField::Value(
                    ProjectRunProvenanceMode::LegacyUnattributed,
                ),
                prepared_receipt: PersistedField::Missing,
                elapsed_time: 0.1,
                success: true,
            }],
            next_run_id: 1,
            active_run_stable_id: Some(run_id),
            active_dataset_id: Some(dataset_id),
            active_analysis_sequence: Some(1),
            overlay_dataset_ids: Vec::new(),
            active_run_id: None,
            active_analysis_id: None,
            overlay_run_ids: Vec::new(),
        };

        seal_project_result_digests(&mut results.runs[0]).expect("fixture digests seal");

        results.validate().expect("core noise labels are valid");

        let restored = results
            .into_simulation_state()
            .expect("valid result history restores");
        let summary = restored
            .active_analysis()
            .and_then(|analysis| analysis.noise_summary.as_ref())
            .expect("noise summary restores");

        assert_eq!(summary.rows[0].mechanism, "white");
        assert_eq!(summary.rows[1].mechanism, "table");
    }

    #[test]
    fn project_text_load_updates_source_path_without_renaming_identity() {
        let mut libraries = LibraryManager::with_primitives();
        let mut workspace = ProjectWorkspace::new_bootstrapped(&mut libraries);
        workspace
            .project
            .set_path(PathBuf::from("stale-native-path.rspiceproj"));
        let project = ProjectFile::new(workspace, libraries);
        let json = serialize_project_file(&project).expect("project serializes");

        let loaded = load_project_text(&json, Some(Path::new("browser-import.rspiceproj")))
            .expect("project text loads");

        assert_eq!(
            loaded.workspace.project.path.as_deref(),
            Some(Path::new("browser-import.rspiceproj"))
        );
        assert_eq!(
            loaded.workspace.project.display_name(),
            "stale-native-path",
            "moving a project file must not silently rename its logical identity"
        );
    }

    #[test]
    fn project_text_load_without_source_path_clears_stale_file_identity() {
        let mut libraries = LibraryManager::with_primitives();
        let mut workspace = ProjectWorkspace::new_bootstrapped(&mut libraries);
        workspace
            .project
            .set_path(PathBuf::from("stale-native-path.rspiceproj"));
        let project = ProjectFile::new(workspace, libraries);
        let json = serialize_project_file(&project).expect("project serializes");

        let loaded = load_project_text(&json, None).expect("project text loads");

        assert!(loaded.workspace.project.path.is_none());
        assert_eq!(loaded.workspace.project.display_name(), "stale-native-path");
    }

    #[test]
    fn legacy_project_migration_assigns_stable_identity_metadata() {
        let project = project_with_execution_context();
        let mut value = serde_json::to_value(project).expect("project converts to JSON");
        let descriptor = value["workspace"]["project"]
            .as_object_mut()
            .expect("project descriptor is an object");
        descriptor.remove("id");
        descriptor.remove("schema_version");
        descriptor.remove("revision");
        value["workspace"]
            .as_object_mut()
            .expect("workspace is an object")
            .remove("simulation_plan_payloads");
        value["execution_context"]["schema_version"] = serde_json::Value::from(3_u32);
        let legacy_plan = value["execution_context"]["simulation_plan"]
            .as_object_mut()
            .expect("simulation plan is an object");
        legacy_plan.remove("analysis_plan");
        legacy_plan.insert("enabled".to_owned(), serde_json::json!([1]));
        legacy_plan.insert("analysis_order".to_owned(), serde_json::json!([1]));
        let legacy = serde_json::to_string_pretty(&value).expect("legacy fixture serializes");

        assert_eq!(
            project_text_load_route(&legacy).expect("legacy route probes"),
            ProjectTextLoadRoute::LegacyProjectIdInjection
        );

        let migrated = load_project_text(&legacy, None).expect("legacy project migrates");
        let replay =
            load_project_text(&legacy, None).expect("identical legacy bytes migrate again");

        assert!(!migrated.workspace.project.id().as_uuid().is_nil());
        assert_eq!(
            migrated.workspace.project.schema_version(),
            crate::state::PROJECT_DESCRIPTOR_SCHEMA_VERSION
        );
        assert_eq!(migrated.workspace.project.revision().get(), 1);
        assert_eq!(
            migrated.workspace.project.id(),
            replay.workspace.project.id()
        );
        let migrated_plan = migrated
            .execution_context
            .as_ref()
            .expect("migrated execution context")
            .simulation_plan
            .stable_analysis_plan()
            .expect("migrated stable plan");
        let replay_plan = replay
            .execution_context
            .as_ref()
            .expect("replayed execution context")
            .simulation_plan
            .stable_analysis_plan()
            .expect("replayed stable plan");
        assert_eq!(migrated_plan.id(), replay_plan.id());
        assert_eq!(
            migrated_plan
                .instances()
                .iter()
                .map(|instance| instance.id())
                .collect::<Vec<_>>(),
            replay_plan
                .instances()
                .iter()
                .map(|instance| instance.id())
                .collect::<Vec<_>>()
        );

        let migrated_json =
            serialize_project_file(&migrated).expect("migrated identity persists on save");
        assert_eq!(
            project_text_load_route(&migrated_json).expect("migrated route probes"),
            ProjectTextLoadRoute::Direct
        );
        let reloaded = load_project_text(&migrated_json, None).expect("migrated project reloads");
        assert_eq!(
            reloaded.workspace.project.id(),
            migrated.workspace.project.id()
        );
        assert_eq!(
            reloaded.workspace.project.revision(),
            migrated.workspace.project.revision()
        );
    }

    #[test]
    fn project_load_rejects_unsupported_descriptor_schema() {
        let mut libraries = LibraryManager::with_primitives();
        let workspace = ProjectWorkspace::new_bootstrapped(&mut libraries);
        let project = ProjectFile::new(workspace, libraries);
        let mut value = serde_json::to_value(project).expect("project converts to JSON");
        value["workspace"]["project"]["schema_version"] =
            serde_json::Value::from(crate::state::PROJECT_DESCRIPTOR_SCHEMA_VERSION + 1);
        let contents = serde_json::to_string_pretty(&value).expect("fixture serializes");

        let error = load_project_text(&contents, None)
            .expect_err("future project descriptor schema must fail closed");

        assert!(matches!(error, ProjectIoError::InvalidData(_)));
        assert!(error.to_string().contains("project schema version"));
    }

    #[test]
    fn project_load_rejects_missing_or_null_identity_on_a_versioned_descriptor() {
        let mut libraries = LibraryManager::with_primitives();
        let workspace = ProjectWorkspace::new_bootstrapped(&mut libraries);
        let project = ProjectFile::new(workspace, libraries);
        let value = serde_json::to_value(project).expect("project converts to JSON");

        let mut missing = value.clone();
        missing["workspace"]["project"]
            .as_object_mut()
            .expect("project descriptor object")
            .remove("id");
        let missing_error = load_project_text(&missing.to_string(), None)
            .expect_err("versioned descriptor cannot lose its identity");
        assert!(
            missing_error
                .to_string()
                .contains("missing its stable identity")
        );

        let mut null = value.clone();
        null["workspace"]["project"]["id"] = serde_json::Value::Null;
        let null_error = load_project_text(&null.to_string(), None)
            .expect_err("explicit null identity is never a legacy migration");
        assert!(
            null_error
                .to_string()
                .contains("must not be explicitly null")
        );

        let mut unversioned_null_id = value.clone();
        unversioned_null_id["workspace"]["project"]
            .as_object_mut()
            .expect("project descriptor object")
            .remove("schema_version");
        unversioned_null_id["workspace"]["project"]["id"] = serde_json::Value::Null;
        let unversioned_null_error = load_project_text(&unversioned_null_id.to_string(), None)
            .expect_err("unversioned explicit null identity is not legacy absence");
        assert!(
            unversioned_null_error
                .to_string()
                .contains("must not be explicitly null")
        );

        let mut null_schema = value;
        null_schema["workspace"]["project"]["schema_version"] = serde_json::Value::Null;
        let null_schema_error = load_project_text(&null_schema.to_string(), None)
            .expect_err("explicit null schema cannot trigger legacy migration");
        assert!(
            null_schema_error
                .to_string()
                .contains("schema version must not be explicitly null")
        );
    }

    #[test]
    fn project_text_load_rejects_missing_active_schematic_buffer() {
        let mut libraries = LibraryManager::with_primitives();
        let workspace = ProjectWorkspace::new_bootstrapped(&mut libraries);
        let mut project = ProjectFile::new(workspace, libraries);
        let active_key = project.workspace.active_key();
        project.workspace.schematic_buffers.remove(&active_key);
        let json = serde_json::to_string_pretty(&project).expect("corrupt fixture serializes");

        let err = load_project_text(&json, None)
            .expect_err("missing active schematic buffer must fail load");

        assert!(matches!(err, ProjectIoError::InvalidData(_)));
        assert!(
            err.to_string().contains(&active_key),
            "error should name the missing buffer key"
        );
    }

    #[test]
    fn project_text_load_rejects_workspace_references_missing_from_libraries() {
        let mut libraries = LibraryManager::with_primitives();
        let mut workspace = ProjectWorkspace::new_bootstrapped(&mut libraries);
        let ghost = CellViewRef::new("ghost", "amp", "schematic");
        let ghost_key = ghost.key();
        let schematic = workspace
            .schematic_buffers
            .values()
            .next()
            .cloned()
            .expect("default project has a schematic buffer");
        workspace.active_view = ghost.clone();
        workspace.open_views = vec![OpenCellView::new(ghost.clone(), ViewType::Schematic)];
        workspace.hierarchy_stack = vec![ghost.clone()];
        workspace.schematic_buffers.clear();
        workspace
            .schematic_buffers
            .insert(ghost_key.clone(), schematic);
        let project = ProjectFile::new(workspace, libraries);
        let json = serde_json::to_string_pretty(&project).expect("corrupt fixture serializes");

        let err = load_project_text(&json, None)
            .expect_err("workspace references absent from libraries must fail load");

        assert!(matches!(err, ProjectIoError::InvalidData(_)));
        assert!(
            err.to_string().contains(&ghost_key),
            "error should name the missing workspace reference"
        );
    }

    #[test]
    fn project_text_load_rejects_workspace_view_type_mismatch() {
        let mut libraries = LibraryManager::with_primitives();
        let mut workspace = ProjectWorkspace::new_bootstrapped(&mut libraries);
        let active = workspace.active_view.clone();
        workspace.open_views = vec![OpenCellView::new(active.clone(), ViewType::Symbol)];
        let active_key = active.key();
        let project = ProjectFile::new(workspace, libraries);
        let json = serde_json::to_string_pretty(&project).expect("corrupt fixture serializes");

        let err = load_project_text(&json, None)
            .expect_err("workspace view type mismatch must fail load");

        assert!(matches!(err, ProjectIoError::InvalidData(_)));
        assert!(
            err.to_string().contains(&active_key) && err.to_string().contains("view type"),
            "error should name the mismatched view reference"
        );
    }

    #[test]
    fn project_text_load_rejects_active_view_missing_from_open_views() {
        let mut libraries = LibraryManager::with_primitives();
        let mut workspace = ProjectWorkspace::new_bootstrapped(&mut libraries);
        assert!(libraries.create_view("user", "top", "symbol", ViewType::Symbol));
        let active = CellViewRef::new("user", "top", "symbol");
        workspace.active_view = active.clone();
        workspace.hierarchy_stack = vec![active.clone()];
        let project = ProjectFile::new(workspace, libraries);
        let json = serde_json::to_string_pretty(&project).expect("corrupt fixture serializes");

        let err = load_project_text(&json, None)
            .expect_err("active view absent from open views must fail load");

        assert!(matches!(err, ProjectIoError::InvalidData(_)));
        assert!(
            err.to_string().contains(&active.key()) && err.to_string().contains("open_views"),
            "error should name the missing active open view"
        );
    }

    #[test]
    fn project_text_load_rejects_library_tree_key_name_mismatch() {
        type ProjectJsonMutation = fn(&mut serde_json::Value, &CellViewRef);

        let cases: [(&str, ProjectJsonMutation); 3] = [
            (
                "library",
                |value: &mut serde_json::Value, active: &CellViewRef| {
                    value["libraries"]["libraries"][&active.library]["name"] =
                        serde_json::Value::String("ghost".to_string());
                },
            ),
            (
                "cell",
                |value: &mut serde_json::Value, active: &CellViewRef| {
                    value["libraries"]["libraries"][&active.library]["cells"][&active.cell]["name"] =
                        serde_json::Value::String("ghost".to_string());
                },
            ),
            (
                "view",
                |value: &mut serde_json::Value, active: &CellViewRef| {
                    value["libraries"]["libraries"][&active.library]["cells"][&active.cell]["views"]
                        [&active.view]["name"] = serde_json::Value::String("ghost".to_string());
                },
            ),
        ];

        for (case, mutate) in cases {
            let mut libraries = LibraryManager::with_primitives();
            let workspace = ProjectWorkspace::new_bootstrapped(&mut libraries);
            let active = workspace.active_view.clone();
            let project = ProjectFile::new(workspace, libraries);
            let mut value = serde_json::to_value(&project).expect("project converts to json value");
            mutate(&mut value, &active);
            let json = serde_json::to_string_pretty(&value).expect("corrupt fixture serializes");

            let err = load_project_text(&json, None)
                .expect_err("library tree key/name mismatches must fail load");

            assert!(matches!(err, ProjectIoError::InvalidData(_)));
            assert!(
                err.to_string().contains(case) && err.to_string().contains("map key"),
                "unexpected {case} mismatch error: {err}"
            );
        }
    }

    #[test]
    fn project_load_rejects_unicode_canonical_library_cell_and_view_collisions() {
        for scope in ["library", "cell", "view"] {
            let mut libraries = LibraryManager::with_primitives();
            let workspace = ProjectWorkspace::new_bootstrapped(&mut libraries);
            match scope {
                "library" => {
                    libraries.add_library(crate::state::Library::new("\u{c9}tage"));
                    libraries.add_library(crate::state::Library::new("\u{e9}TAGE"));
                }
                "cell" => {
                    let library = libraries.get_library_mut("user").expect("user library");
                    library.add_cell(crate::state::Cell::new("\u{c9}tage"));
                    library.add_cell(crate::state::Cell::new("\u{e9}TAGE"));
                }
                "view" => {
                    let cell = libraries
                        .get_library_mut("user")
                        .expect("user library")
                        .get_cell_mut("top")
                        .expect("top cell");
                    cell.add_view(crate::state::View::new("Mod\u{e8}le", ViewType::Symbol));
                    cell.add_view(crate::state::View::new("MOD\u{c8}LE", ViewType::Symbol));
                }
                _ => unreachable!(),
            }

            let project = ProjectFile::new(workspace, libraries);
            let json = serde_json::to_string_pretty(&project).expect("fixture serializes");
            let error = load_project_text(&json, None)
                .expect_err("canonical library identities must be unique");

            assert!(
                error.to_string().contains("canonical")
                    && error.to_string().contains(scope)
                    && error.to_string().contains("collision"),
                "unexpected {scope} collision error: {error}"
            );
        }
    }

    #[test]
    fn project_load_rejects_slash_alias_triples_before_key_lookup() {
        let mut libraries = LibraryManager::with_primitives();
        let workspace = ProjectWorkspace::new_bootstrapped(&mut libraries);

        let mut first_cell = crate::state::Cell::new("b");
        first_cell.add_view(crate::state::View::new("c/d", ViewType::Symbol));
        let mut first_library = crate::state::Library::new("a");
        first_library.add_cell(first_cell);
        libraries.add_library(first_library);

        let mut second_cell = crate::state::Cell::new("c");
        second_cell.add_view(crate::state::View::new("d", ViewType::Symbol));
        let mut second_library = crate::state::Library::new("a/b");
        second_library.add_cell(second_cell);
        libraries.add_library(second_library);

        let project = ProjectFile::new(workspace, libraries);
        let json = serde_json::to_string_pretty(&project).expect("alias fixture serializes");
        let error = load_project_text(&json, None)
            .expect_err("two distinct triples must not alias one generated key");

        assert!(matches!(error, ProjectIoError::InvalidData(_)));
        assert!(
            error
                .to_string()
                .contains("duplicate cell-view key 'a/b/c/d'")
                && error.to_string().contains("injective"),
            "unexpected alias error: {error}"
        );
    }

    #[test]
    fn project_load_rejects_lcv_names_outside_the_ui_contract() {
        let mut libraries = LibraryManager::with_primitives();
        let workspace = ProjectWorkspace::new_bootstrapped(&mut libraries);
        let mut invalid_cell = crate::state::Cell::new("bad-name");
        invalid_cell.add_view(crate::state::View::new("schematic", ViewType::Schematic));
        libraries
            .get_library_mut("user")
            .expect("default library")
            .add_cell(invalid_cell);
        let project = ProjectFile::new(workspace, libraries);
        let json = serde_json::to_string_pretty(&project).expect("invalid fixture serializes");

        let error = load_project_text(&json, None)
            .expect_err("persisted names outside the UI contract must fail closed");

        assert!(error.to_string().contains("bad-name"));
        assert!(error.to_string().contains("cell-view name contract"));
    }

    #[test]
    fn project_load_rejects_orphan_and_malformed_schematic_buffers() {
        for (key, expected) in [
            ("ghost/cell/schematic", "orphaned"),
            ("user/top/schematic/alias", "malformed"),
        ] {
            let mut libraries = LibraryManager::with_primitives();
            let mut workspace = ProjectWorkspace::new_bootstrapped(&mut libraries);
            let buffer = workspace
                .schematic_buffers
                .values()
                .next()
                .cloned()
                .expect("default schematic buffer");
            workspace.schematic_buffers.insert(key.to_owned(), buffer);
            let project = ProjectFile::new(workspace, libraries);
            let json = serde_json::to_string_pretty(&project).expect("buffer fixture serializes");

            let error = load_project_text(&json, None)
                .expect_err("unowned or malformed persisted buffers must fail closed");
            assert!(
                error.to_string().contains(key) && error.to_string().contains(expected),
                "unexpected buffer error for {key}: {error}"
            );
        }
    }

    #[test]
    fn project_load_rejects_schematic_buffer_bound_to_symbol_view() {
        let mut libraries = LibraryManager::with_primitives();
        let mut workspace = ProjectWorkspace::new_bootstrapped(&mut libraries);
        assert!(libraries.create_view("user", "top", "symbol", ViewType::Symbol));
        let buffer = workspace
            .schematic_buffers
            .values()
            .next()
            .cloned()
            .expect("default schematic buffer");
        workspace
            .schematic_buffers
            .insert("user/top/symbol".to_owned(), buffer);
        let project = ProjectFile::new(workspace, libraries);
        let json = serde_json::to_string_pretty(&project).expect("buffer fixture serializes");

        let error =
            load_project_text(&json, None).expect_err("symbol views cannot own schematic buffers");
        assert!(
            error.to_string().contains("user/top/symbol")
                && error.to_string().contains("cannot own")
        );
    }

    #[test]
    fn project_load_rejects_duplicate_open_view_keys() {
        let mut libraries = LibraryManager::with_primitives();
        let mut workspace = ProjectWorkspace::new_bootstrapped(&mut libraries);
        workspace.open_views.push(workspace.open_views[0].clone());
        let project = ProjectFile::new(workspace, libraries);
        let json = serde_json::to_string_pretty(&project).expect("duplicate fixture serializes");

        let error =
            load_project_text(&json, None).expect_err("duplicate open-view keys must fail closed");
        assert!(
            error.to_string().contains("duplicate cell-view key")
                && error.to_string().contains("workspace.open_views")
        );
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    #[allow(deprecated)]
    fn legacy_public_project_save_is_create_only_and_preserves_existing_bytes() {
        let root = std::env::temp_dir().join(format!(
            "rspice-legacy-create-only-project-{}",
            uuid::Uuid::new_v4()
        ));
        std::fs::create_dir_all(&root).expect("create isolated project test directory");
        let path = root.join("design.rspiceproj");
        std::fs::write(&path, "external project bytes").expect("write existing target");
        let mut libraries = LibraryManager::with_primitives();
        let workspace = ProjectWorkspace::new_bootstrapped(&mut libraries);
        let project = ProjectFile::new(workspace, libraries);

        let error = save_project_file(&project, &path)
            .expect_err("legacy public save must not overwrite an existing destination");

        assert!(error.to_string().contains("create-only"));
        assert_eq!(
            std::fs::read_to_string(&path).expect("read preserved destination"),
            "external project bytes"
        );
        std::fs::remove_dir_all(root).expect("remove isolated project test directory");
    }

    #[test]
    fn project_text_load_reports_parse_errors_without_filesystem() {
        let err = load_project_text("{not valid json", Some(Path::new("bad.rspiceproj")))
            .expect_err("invalid project text fails");

        assert!(matches!(err, ProjectIoError::ParseError(_)));
    }

    #[test]
    fn canonical_plan_kind_tags_cover_the_complete_manifest_without_collisions() {
        let tags = AnalysisKind::ALL
            .into_iter()
            .map(analysis_kind_tag_for_plan_kind)
            .collect::<std::collections::HashSet<_>>();
        assert_eq!(tags.len(), AnalysisKind::ALL.len());
        assert_eq!(analysis_kind_tag_for_plan_kind(AnalysisKind::Qpss), 26);
        assert_eq!(
            analysis_kind_tag_for_plan_kind(AnalysisKind::DcMismatch),
            34
        );
    }
}
