//! Project file I/O.
//!
//! `.rspiceproj` stores the product-level workspace: project identity,
//! libraries/cells/views, open documents, schematic buffers, authoritative
//! simulation inputs, model-section bindings, and retained results. Individual
//! schematic export remains available through `.rsch`; project files are the
//! native professional workflow container.

use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use sha2::{Digest as _, Sha256};

use crate::io::project_execution::ProjectExecutionContext;
use crate::product::{
    AnalysisInstanceId, ContentDigest, DatasetId, JobId, ObjectRevision, ProjectId, RunId,
    SimulationPlanId,
};
use crate::simulation::plan::AnalysisKind;
use crate::state::workspace::validate_cell_view_name_segment;
use crate::state::{
    AnalysisResult, AnalysisResultFamilyMetadata, AnalysisResultPayload, AnalysisResultProvenance,
    AnalysisResultSourceDomain, AnalysisType, CellViewRef, DcOpResult, ExecutionTarget,
    LibraryManager, NoiseContributorRow, NoiseSummary, OperatingPointValue, PreparedRunReceipt,
    PreparedRunTaskReceipt, PreparedSourceCheckReceipt, ProjectWorkspace,
    SavedOutputMaterializationStatus, SavedOutputReceipt, SimulationRun, SimulationRunLifecycle,
    SimulationRunProvenance, SimulationState, ViewType, WaveformData,
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
                            .map(crate::common::app::StoredSimulationPlan::analysis_plan)
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
        simulation_plan: Option<&crate::common::app::SimSetupState>,
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

    fn validate_library_tree(
        &self,
    ) -> Result<HashMap<String, ValidatedLibraryView>, ProjectIoError> {
        let mut view_index: HashMap<String, ValidatedLibraryView> = HashMap::new();
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
                    if let Some(existing) = view_index.get(&key) {
                        return Err(ProjectIoError::InvalidData(format!(
                            "library tree generates duplicate cell-view key '{key}' for {} and {}; slash-delimited persisted keys must be injective",
                            format_lcv_segments(&existing.reference),
                            format_lcv_segments(&reference)
                        )));
                    }
                    view_index.insert(
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

    fn validate_workspace_references(
        &self,
        view_index: &HashMap<String, ValidatedLibraryView>,
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
            let Some(view) = view_index.get(key) else {
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
const PROJECT_SIMULATION_RESULTS_SCHEMA_VERSION: u32 = 9;

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

/// Stable project-file representation of result history.
///
/// This is intentionally separate from `SimulationState`: project files should
/// persist user-visible result data, not transient runner flags, progress text,
/// or UI trigger bits.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectSimulationResults {
    #[serde(default = "default_simulation_results_schema_version")]
    pub schema_version: u32,
    #[serde(default)]
    pub runs: Vec<ProjectSimulationRun>,
    #[serde(default)]
    pub next_run_id: u64,
    /// Stable v2 selection identity.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active_run_stable_id: Option<RunId>,
    /// Stable v2 selected immutable dataset identity.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active_dataset_id: Option<DatasetId>,
    /// Run-local analysis sequence within the selected immutable dataset.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active_analysis_sequence: Option<u64>,
    /// Stable v2 dataset overlay identities.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub overlay_dataset_ids: Vec<DatasetId>,
    /// Legacy v1 display-sequence selection; consumed during migration and
    /// never written by a current project.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active_run_id: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active_analysis_id: Option<u64>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub overlay_run_ids: Vec<u64>,
}

impl Default for ProjectSimulationResults {
    fn default() -> Self {
        Self {
            schema_version: PROJECT_SIMULATION_RESULTS_SCHEMA_VERSION,
            runs: Vec::new(),
            next_run_id: 0,
            active_run_stable_id: None,
            active_dataset_id: None,
            active_analysis_sequence: None,
            overlay_dataset_ids: Vec::new(),
            active_run_id: None,
            active_analysis_id: None,
            overlay_run_ids: Vec::new(),
        }
    }
}

impl ProjectSimulationResults {
    pub fn is_empty(&self) -> bool {
        self.runs.is_empty()
            && self.next_run_id == 0
            && self.active_run_stable_id.is_none()
            && self.active_dataset_id.is_none()
            && self.active_analysis_sequence.is_none()
            && self.overlay_dataset_ids.is_empty()
            && self.active_run_id.is_none()
            && self.active_analysis_id.is_none()
            && self.overlay_run_ids.is_empty()
    }

    pub fn from_state(state: &SimulationState) -> Self {
        if state.runs.is_empty() {
            return Self::default();
        }

        let runs: Vec<_> = state.runs.iter().map(ProjectSimulationRun::from).collect();
        let max_run_id = state.runs.iter().map(|run| run.id).max().unwrap_or(0);
        Self {
            schema_version: PROJECT_SIMULATION_RESULTS_SCHEMA_VERSION,
            runs,
            next_run_id: state.next_run_id.max(max_run_id),
            active_run_stable_id: state.active_run().map(|run| run.run_id),
            active_dataset_id: state.active_run().map(|run| run.dataset_id),
            active_analysis_sequence: state.active_analysis().map(|analysis| analysis.id),
            overlay_dataset_ids: state.overlay_dataset_ids.clone(),
            active_run_id: None,
            active_analysis_id: None,
            overlay_run_ids: Vec::new(),
        }
    }

    pub fn into_simulation_state(self) -> Result<SimulationState, String> {
        let mut state = SimulationState::default();
        self.apply_to_state(&mut state)?;
        Ok(state)
    }

    /// Apply an already-current, validated result document. Legacy migration
    /// requires the owning [`ProjectId`] and must be completed explicitly at
    /// the project/session boundary before this method is called.
    pub fn apply_to_state(self, state: &mut SimulationState) -> Result<(), String> {
        self.validate()?;
        let runs = self
            .runs
            .into_iter()
            .map(ProjectSimulationRun::into_run)
            .collect::<Result<Vec<_>, _>>()?;
        state.restore_run_history(
            runs,
            self.next_run_id,
            self.active_run_stable_id,
            self.active_dataset_id,
            self.active_analysis_sequence,
            self.overlay_dataset_ids,
        );
        Ok(())
    }

    /// Upgrade historical result schemas without fabricating analysis-source
    /// identity. V1 display-sequence references are converted to stable run and
    /// dataset IDs. V1/v2 analyses retain `provenance: None`; schema v5 records
    /// that fact explicitly per run so provenance cannot disappear from a
    /// current prepared-task result history without validation failing. Runs
    /// written before v6 become explicitly `LegacyUnknown`: execution identity
    /// and lifecycle are never inferred from historical result payloads. V6
    /// analyses migrate with `family_metadata: None`; metadata absent from an
    /// historical payload is never reconstructed from display waveforms.
    /// Result schemas through v7 acquire canonical result-data and dataset
    /// digests from the exact retained values during migration; no samples or
    /// analysis evidence are reconstructed. Schema v8 digests are verified
    /// with their original encoding before payload absence is migrated and the
    /// result is resealed with the current encoding.
    pub(crate) fn migrate_to_current(&mut self, project_id: ProjectId) -> Result<(), String> {
        if self.schema_version == PROJECT_SIMULATION_RESULTS_SCHEMA_VERSION {
            return Ok(());
        }
        let mut candidate = self.clone();
        candidate.migrate_to_current_in_place(project_id)?;
        *self = candidate;
        Ok(())
    }

    fn migrate_to_current_in_place(&mut self, project_id: ProjectId) -> Result<(), String> {
        let source_schema = self.schema_version;
        if source_schema == CONTENT_DIGEST_RESULTS_SCHEMA_VERSION {
            for run in &mut self.runs {
                validate_v8_result_digests(run)?;
                if let Some(analysis) = run
                    .analyses
                    .iter()
                    .find(|analysis| analysis.result_payload.is_present())
                {
                    return Err(format!(
                        "schema-v8 analysis {} contains a typed result payload introduced by schema v9",
                        analysis.id
                    ));
                }
                seal_project_result_digests(run)?;
            }
            self.schema_version = PROJECT_SIMULATION_RESULTS_SCHEMA_VERSION;
            return self.validate();
        }
        if matches!(
            source_schema,
            EXECUTION_IDENTITY_RESULTS_SCHEMA_VERSION | FAMILY_METADATA_RESULTS_SCHEMA_VERSION
        ) {
            for run in &mut self.runs {
                require_legacy_result_digest_absence(run, source_schema)?;
                seal_project_result_digests(run)?;
            }
            self.schema_version = PROJECT_SIMULATION_RESULTS_SCHEMA_VERSION;
            return self.validate();
        }
        let migrate_v1_references = match source_schema {
            PROJECT_SIMULATION_RESULTS_SCHEMA_VERSION => return Ok(()),
            LEGACY_SIMULATION_RESULTS_SCHEMA_VERSION => true,
            STABLE_DATASET_RESULTS_SCHEMA_VERSION
            | PREPARED_PROVENANCE_RESULTS_SCHEMA_VERSION
            | EXPLICIT_PROVENANCE_MODE_RESULTS_SCHEMA_VERSION
            | SOURCE_DOMAIN_RESULTS_SCHEMA_VERSION => false,
            unsupported => {
                return Err(format!(
                    "unsupported simulation results schema version {unsupported}"
                ));
            }
        };

        self.validate_legacy_schema_shape(source_schema)?;

        if migrate_v1_references {
            for (run_idx, run) in self.runs.iter_mut().enumerate() {
                let identity = serde_json::to_vec(&(
                    project_id,
                    run_idx,
                    run.id,
                    &run.label,
                    run.timestamp.to_bits(),
                    &run.analyses,
                    run.elapsed_time.to_bits(),
                    run.success,
                ))
                .map_err(|error| {
                    format!(
                        "legacy simulation run sequence {} could not be assigned a reproducible identity: {error}",
                        run.id
                    )
                })?;
                if run.run_id.is_none() {
                    run.run_id = Some(RunId::from_namespace(
                        LEGACY_RESULT_RUN_ID_NAMESPACE,
                        &identity,
                    ));
                }
                if run.dataset_id.is_none() {
                    run.dataset_id = Some(DatasetId::from_namespace(
                        LEGACY_RESULT_DATASET_ID_NAMESPACE,
                        &identity,
                    ));
                }
            }

            if let Some(run_sequence) = self.active_run_id {
                let active_run = self
                    .runs
                    .iter()
                    .find(|run| run.id == run_sequence)
                    .ok_or_else(|| {
                        format!(
                            "legacy active simulation run sequence {run_sequence} does not exist"
                        )
                    })?;
                self.active_run_stable_id = Some(active_run.run_id.ok_or_else(|| {
                    format!(
                        "legacy simulation run sequence {} has no stable id",
                        active_run.id
                    )
                })?);
                self.active_dataset_id = Some(active_run.dataset_id.ok_or_else(|| {
                    format!(
                        "legacy simulation run sequence {} has no dataset id",
                        active_run.id
                    )
                })?);
                self.active_analysis_sequence = if let Some(sequence) = self.active_analysis_id {
                    if !active_run
                        .analyses
                        .iter()
                        .any(|analysis| analysis.id == sequence)
                    {
                        return Err(format!(
                            "legacy active analysis sequence {sequence} does not exist in run {}",
                            active_run.id
                        ));
                    }
                    Some(sequence)
                } else {
                    None
                };
            } else if self.active_analysis_id.is_some() {
                return Err("legacy active analysis has no active simulation run".to_owned());
            }

            if !self.overlay_run_ids.is_empty() {
                self.overlay_dataset_ids.clear();
                for sequence in &self.overlay_run_ids {
                    let dataset_id = self
                        .runs
                        .iter()
                        .find(|run| run.id == *sequence)
                        .map(|run| {
                            run.dataset_id.ok_or_else(|| {
                                format!("legacy overlay run sequence {sequence} has no dataset id")
                            })
                        })
                        .transpose()?
                        .ok_or_else(|| {
                            format!("legacy overlay run sequence {sequence} does not exist")
                        })?;
                    if Some(dataset_id) != self.active_dataset_id
                        && !self.overlay_dataset_ids.contains(&dataset_id)
                    {
                        self.overlay_dataset_ids.push(dataset_id);
                    }
                }
            }
        }

        self.active_run_id = None;
        self.active_analysis_id = None;
        self.overlay_run_ids.clear();

        for (run_idx, run) in self.runs.iter_mut().enumerate() {
            let provenance_count = run
                .analyses
                .iter()
                .filter(|analysis| analysis.provenance.is_some())
                .count();
            let migrated_mode = match source_schema {
                LEGACY_SIMULATION_RESULTS_SCHEMA_VERSION
                | STABLE_DATASET_RESULTS_SCHEMA_VERSION => {
                    ProjectRunProvenanceMode::LegacyUnattributed
                }
                PREPARED_PROVENANCE_RESULTS_SCHEMA_VERSION => {
                    if provenance_count == 0 {
                        ProjectRunProvenanceMode::LegacyUnattributed
                    } else if provenance_count == run.analyses.len() {
                        set_legacy_unclassified_source_domains(run);
                        ProjectRunProvenanceMode::LegacyPreparedUnclassified
                    } else {
                        return Err(format!(
                            "runs[{run_idx}] mixes schema-v3 analyses with and without prepared-task provenance"
                        ));
                    }
                }
                EXPLICIT_PROVENANCE_MODE_RESULTS_SCHEMA_VERSION => {
                    match run.provenance_mode.as_ref().copied() {
                        Some(ProjectRunProvenanceMode::LegacyUnattributed)
                            if provenance_count == 0 =>
                        {
                            ProjectRunProvenanceMode::LegacyUnattributed
                        }
                        Some(ProjectRunProvenanceMode::PreparedTaskBound)
                            if provenance_count == run.analyses.len() && provenance_count != 0 =>
                        {
                            set_legacy_unclassified_source_domains(run);
                            ProjectRunProvenanceMode::LegacyPreparedUnclassified
                        }
                        Some(ProjectRunProvenanceMode::LegacyUnattributed) => {
                            return Err(format!(
                                "runs[{run_idx}] is schema-v4 legacy_unattributed but contains prepared-task provenance"
                            ));
                        }
                        Some(ProjectRunProvenanceMode::PreparedTaskBound) => {
                            return Err(format!(
                                "runs[{run_idx}] is schema-v4 prepared_task_bound but its complete provenance is missing"
                            ));
                        }
                        Some(ProjectRunProvenanceMode::Unspecified) | None => {
                            return Err(format!(
                                "runs[{run_idx}].provenance_mode is missing from schema-v4 simulation results"
                            ));
                        }
                        Some(ProjectRunProvenanceMode::LegacyPreparedUnclassified) => {
                            return Err(format!(
                                "runs[{run_idx}] uses a provenance mode introduced after schema v4"
                            ));
                        }
                    }
                }
                SOURCE_DOMAIN_RESULTS_SCHEMA_VERSION => run
                    .provenance_mode
                    .as_ref()
                    .copied()
                    .ok_or_else(|| {
                        format!(
                            "runs[{run_idx}].provenance_mode is missing from schema-v5 simulation results"
                        )
                    })?,
                _ => unreachable!("supported legacy result schema matched above"),
            };
            run.provenance_mode = PersistedField::Value(migrated_mode);
            run.lifecycle = Some(SimulationRunLifecycle::LegacyUnknown);
            seal_project_result_digests(run)?;
            run.validate(run_idx)?;
        }
        self.schema_version = PROJECT_SIMULATION_RESULTS_SCHEMA_VERSION;
        Ok(())
    }

    fn validate_legacy_schema_shape(&self, source_schema: u32) -> Result<(), String> {
        let stable_ids_required = source_schema >= STABLE_DATASET_RESULTS_SCHEMA_VERSION;
        let legacy_sequence_refs_allowed =
            source_schema == LEGACY_SIMULATION_RESULTS_SCHEMA_VERSION;
        if legacy_sequence_refs_allowed {
            if self.active_run_stable_id.is_some()
                || self.active_dataset_id.is_some()
                || self.active_analysis_sequence.is_some()
                || !self.overlay_dataset_ids.is_empty()
            {
                return Err(
                    "schema-v1 simulation results contain stable references introduced by schema v2"
                        .to_owned(),
                );
            }
        } else if self.active_run_id.is_some()
            || self.active_analysis_id.is_some()
            || !self.overlay_run_ids.is_empty()
        {
            return Err(format!(
                "schema-v{source_schema} simulation results retain schema-v1 sequence references"
            ));
        }

        for (run_idx, run) in self.runs.iter().enumerate() {
            require_legacy_result_digest_absence(run, source_schema)?;
            if run.job_id.is_some() || run.execution_target.is_some() || run.lifecycle.is_some() {
                return Err(format!(
                    "runs[{run_idx}] contains execution identity or lifecycle introduced after schema v{source_schema}"
                ));
            }
            if source_schema < SOURCE_DOMAIN_RESULTS_SCHEMA_VERSION
                && run.prepared_receipt.is_present()
            {
                return Err(format!(
                    "runs[{run_idx}] contains a prepared run receipt introduced after schema v{source_schema}"
                ));
            }
            if stable_ids_required {
                if run.run_id.is_none() || run.dataset_id.is_none() {
                    return Err(format!(
                        "runs[{run_idx}] is missing stable run/dataset identity required by schema v{source_schema}"
                    ));
                }
            } else if run.run_id.is_some() || run.dataset_id.is_some() {
                return Err(format!(
                    "runs[{run_idx}] contains stable identity introduced after schema v1"
                ));
            }

            match source_schema {
                LEGACY_SIMULATION_RESULTS_SCHEMA_VERSION
                | STABLE_DATASET_RESULTS_SCHEMA_VERSION => {
                    if run.provenance_mode.is_present() {
                        return Err(format!(
                            "runs[{run_idx}] contains provenance_mode introduced after schema v{source_schema}"
                        ));
                    }
                    if run
                        .analyses
                        .iter()
                        .any(|analysis| analysis.provenance.is_some())
                    {
                        return Err(format!(
                            "runs[{run_idx}] contains prepared provenance introduced after schema v{source_schema}"
                        ));
                    }
                }
                PREPARED_PROVENANCE_RESULTS_SCHEMA_VERSION => {
                    if run.provenance_mode.is_present() {
                        return Err(format!(
                            "runs[{run_idx}] contains provenance_mode introduced after schema v3"
                        ));
                    }
                }
                EXPLICIT_PROVENANCE_MODE_RESULTS_SCHEMA_VERSION => {
                    if run.provenance_mode.as_ref().is_none() {
                        return Err(format!(
                            "runs[{run_idx}].provenance_mode is required by schema v4"
                        ));
                    }
                }
                SOURCE_DOMAIN_RESULTS_SCHEMA_VERSION => {
                    if run.provenance_mode.as_ref().is_none() {
                        return Err(format!(
                            "runs[{run_idx}].provenance_mode is required by schema v5"
                        ));
                    }
                }
                _ => unreachable!("supported legacy result schema matched above"),
            }

            for (analysis_idx, analysis) in run.analyses.iter().enumerate() {
                if let Some(provenance) = &analysis.provenance {
                    if source_schema < SOURCE_DOMAIN_RESULTS_SCHEMA_VERSION
                        && provenance.source_domain.is_present()
                    {
                        return Err(format!(
                            "runs[{run_idx}].analyses[{analysis_idx}].provenance.source_domain was introduced after schema v{source_schema}"
                        ));
                    }
                    if source_schema == SOURCE_DOMAIN_RESULTS_SCHEMA_VERSION
                        && provenance.source_domain.as_ref().is_none()
                    {
                        return Err(format!(
                            "runs[{run_idx}].analyses[{analysis_idx}].provenance.source_domain is required by schema v5"
                        ));
                    }
                }
            }
        }
        Ok(())
    }

    pub(crate) fn validate(&self) -> Result<(), String> {
        if self.schema_version != PROJECT_SIMULATION_RESULTS_SCHEMA_VERSION {
            return Err(format!(
                "unsupported simulation results schema version {}",
                self.schema_version
            ));
        }
        if self.active_run_id.is_some()
            || self.active_analysis_id.is_some()
            || !self.overlay_run_ids.is_empty()
        {
            return Err("current simulation results retain unmigrated v1 references".to_owned());
        }

        let mut run_sequences = HashSet::new();
        let mut job_ids = HashSet::new();
        let mut run_ids = HashSet::new();
        let mut dataset_ids = HashSet::new();
        for (run_idx, run) in self.runs.iter().enumerate() {
            if run.id == 0 {
                return Err(format!("runs[{run_idx}].id must be greater than zero"));
            }
            if !run_sequences.insert(run.id) {
                return Err(format!("duplicate simulation run id {}", run.id));
            }
            if let Some(job_id) = run.job_id
                && !job_ids.insert(job_id)
            {
                return Err(format!("duplicate stable simulation job id {job_id}"));
            }
            let run_id = run.run_id.ok_or_else(|| {
                format!("runs[{run_idx}].run_id is required by simulation results schema v2")
            })?;
            if !run_ids.insert(run_id) {
                return Err(format!("duplicate stable simulation run id {run_id}"));
            }
            let dataset_id = run.dataset_id.ok_or_else(|| {
                format!("runs[{run_idx}].dataset_id is required by simulation results schema v2")
            })?;
            if !dataset_ids.insert(dataset_id) {
                return Err(format!("duplicate immutable dataset id {dataset_id}"));
            }
            run.validate(run_idx)?;
        }
        if let Some(active_run_id) = self.active_run_stable_id
            && !run_ids.contains(&active_run_id)
        {
            return Err(format!(
                "active simulation run id {} does not exist in persisted history",
                active_run_id
            ));
        }
        match (self.active_run_stable_id, self.active_dataset_id) {
            (Some(active_run_id), Some(active_dataset_id)) => {
                let Some(active_run) = self
                    .runs
                    .iter()
                    .find(|run| run.run_id == Some(active_run_id))
                else {
                    return Err(format!(
                        "active simulation run id {} does not exist in persisted history",
                        active_run_id
                    ));
                };
                if active_run.dataset_id != Some(active_dataset_id) {
                    return Err(format!(
                        "active dataset id {} does not belong to active run {}",
                        active_dataset_id, active_run_id
                    ));
                }
                if let Some(sequence) = self.active_analysis_sequence
                    && !active_run
                        .analyses
                        .iter()
                        .any(|analysis| analysis.id == sequence)
                {
                    return Err(format!(
                        "active analysis sequence {} does not exist in active dataset {}",
                        sequence, active_dataset_id
                    ));
                }
            }
            (Some(active_run_id), None) => {
                return Err(format!(
                    "active simulation run id {} has no active dataset id",
                    active_run_id
                ));
            }
            (None, Some(active_dataset_id)) => {
                return Err(format!(
                    "active dataset id {} has no active run id",
                    active_dataset_id
                ));
            }
            (None, None) if self.active_analysis_sequence.is_some() => {
                return Err("active analysis sequence has no active dataset".to_owned());
            }
            (None, None) => {}
        }
        let mut overlay_ids = HashSet::new();
        for overlay_id in &self.overlay_dataset_ids {
            if !dataset_ids.contains(overlay_id) {
                return Err(format!(
                    "overlay dataset id {} does not exist in persisted history",
                    overlay_id
                ));
            }
            if !overlay_ids.insert(*overlay_id) {
                return Err(format!("duplicate overlay dataset id {}", overlay_id));
            }
            if Some(*overlay_id) == self.active_dataset_id {
                return Err(format!(
                    "active dataset id {} cannot also be an overlay",
                    overlay_id
                ));
            }
        }
        Ok(())
    }
}

fn require_legacy_result_digest_absence(
    run: &ProjectSimulationRun,
    source_schema: u32,
) -> Result<(), String> {
    if run.dataset_content_digest.is_present() {
        return Err(format!(
            "schema-v{source_schema} simulation run {} contains a dataset content digest introduced by schema v8",
            run.id
        ));
    }
    if let Some(analysis) = run
        .analyses
        .iter()
        .find(|analysis| analysis.result_data_digest.is_present())
    {
        return Err(format!(
            "schema-v{source_schema} analysis {} contains a result data digest introduced by schema v8",
            analysis.id
        ));
    }
    Ok(())
}

/// Authenticate a schema-v8 run with the exact digest encoding that wrote it.
/// This must run before any v9 fields are introduced or digests are resealed.
fn validate_v8_result_digests(run: &ProjectSimulationRun) -> Result<(), String> {
    for analysis in &run.analyses {
        if analysis.result_payload.is_present() {
            return Err(format!(
                "schema-v8 analysis {} contains a typed result payload introduced by schema v9",
                analysis.id
            ));
        }
        let retained = analysis
            .result_data_digest
            .as_ref()
            .copied()
            .ok_or_else(|| {
                format!(
                    "schema-v8 analysis {} is missing its result data digest",
                    analysis.id
                )
            })?;
        let computed = analysis
            .clone()
            .into_analysis()?
            .legacy_v1_result_data_digest();
        if retained != computed {
            return Err(format!(
                "schema-v8 analysis {} result data digest does not match retained content",
                analysis.id
            ));
        }
    }

    let retained = run
        .dataset_content_digest
        .as_ref()
        .copied()
        .ok_or_else(|| {
            format!(
                "schema-v8 simulation run {} is missing its dataset content digest",
                run.id
            )
        })?;
    let computed = run.clone().into_run()?.legacy_v1_dataset_content_digest();
    if retained != computed {
        return Err(format!(
            "schema-v8 simulation run {} dataset content digest does not match retained content",
            run.id
        ));
    }
    Ok(())
}

fn seal_project_result_digests(run: &mut ProjectSimulationRun) -> Result<(), String> {
    for analysis in &mut run.analyses {
        let digest = analysis.clone().into_analysis()?.result_data_digest();
        analysis.result_data_digest = PersistedField::Value(digest);
    }
    let digest = run.clone().into_run()?.dataset_content_digest();
    run.dataset_content_digest = PersistedField::Value(digest);
    Ok(())
}

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

fn set_legacy_unclassified_source_domains(run: &mut ProjectSimulationRun) {
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProjectPreparedRunTaskReceipt {
    pub source_instance_id: AnalysisInstanceId,
    pub source_revision: ObjectRevision,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dependency_ids: Vec<AnalysisInstanceId>,
    pub analysis_kind_tag: u8,
    pub config_digest: ContentDigest,
}

impl ProjectPreparedRunTaskReceipt {
    fn into_receipt(self) -> Result<PreparedRunTaskReceipt, String> {
        PreparedRunTaskReceipt::new(
            self.source_instance_id,
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
            source_revision: receipt.source_revision(),
            dependency_ids: receipt.dependencies().to_vec(),
            analysis_kind_tag: receipt.analysis_kind_tag(),
            config_digest: receipt.config_digest(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProjectPreparedRunReceipt {
    pub source_domain: AnalysisResultSourceDomain,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub simulation_plan_id: Option<SimulationPlanId>,
    pub project_revision: ObjectRevision,
    pub prepared_snapshot_digest: ContentDigest,
    pub source_content_digest: ContentDigest,
    pub source_check_receipt: ProjectPreparedSourceCheckReceipt,
    pub tasks: Vec<ProjectPreparedRunTaskReceipt>,
}

impl ProjectPreparedRunReceipt {
    fn into_receipt(self) -> Result<PreparedRunReceipt, String> {
        let tasks = self
            .tasks
            .into_iter()
            .map(ProjectPreparedRunTaskReceipt::into_receipt)
            .collect::<Result<Vec<_>, _>>()?;
        PreparedRunReceipt::new(
            self.source_domain,
            self.simulation_plan_id,
            self.project_revision,
            self.prepared_snapshot_digest,
            self.source_content_digest,
            self.source_check_receipt.into(),
            tasks,
        )
    }

    fn validate(&self) -> Result<(), String> {
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
            tasks: receipt
                .tasks()
                .iter()
                .map(ProjectPreparedRunTaskReceipt::from)
                .collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectSimulationRun {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_id: Option<JobId>,
    #[serde(default)]
    pub run_id: Option<RunId>,
    #[serde(default)]
    pub dataset_id: Option<DatasetId>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub execution_target: Option<ExecutionTarget>,
    /// Required by schema v6. Historical schemas are migrated to an explicit
    /// `LegacyUnknown` value without outcome inference.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lifecycle: Option<SimulationRunLifecycle>,
    /// Display sequence retained for labels and v1 migration.
    pub id: u64,
    pub label: String,
    pub timestamp: f64,
    #[serde(default)]
    pub analyses: Vec<ProjectAnalysisResult>,
    /// Canonical identity of the ordered retained analysis content. Schemas
    /// through v7 omitted this field and acquire it during migration.
    #[serde(default, skip_serializing_if = "PersistedField::is_missing")]
    pub dataset_content_digest: PersistedField<ContentDigest>,
    #[serde(default, skip_serializing_if = "PersistedField::is_missing")]
    pub provenance_mode: PersistedField<ProjectRunProvenanceMode>,
    #[serde(default, skip_serializing_if = "PersistedField::is_missing")]
    pub prepared_receipt: PersistedField<ProjectPreparedRunReceipt>,
    #[serde(default)]
    pub elapsed_time: f64,
    #[serde(default = "default_true")]
    pub success: bool,
}

impl ProjectSimulationRun {
    fn into_run(self) -> Result<SimulationRun, String> {
        let run_id = self
            .run_id
            .ok_or_else(|| format!("simulation run sequence {} has no stable id", self.id))?;
        let dataset_id = self
            .dataset_id
            .ok_or_else(|| format!("simulation run sequence {} has no dataset id", self.id))?;
        let analyses = self
            .analyses
            .into_iter()
            .map(ProjectAnalysisResult::into_analysis)
            .collect::<Result<Vec<_>, _>>()?;
        if self.dataset_content_digest.is_null() {
            return Err(format!(
                "simulation run sequence {} has an explicitly null dataset content digest",
                self.id
            ));
        }
        if self.provenance_mode.is_null() {
            return Err(format!(
                "simulation run sequence {} has an explicitly null provenance mode",
                self.id
            ));
        }
        if self.prepared_receipt.is_null() {
            return Err(format!(
                "simulation run sequence {} has an explicitly null prepared receipt",
                self.id
            ));
        }
        let provenance = match self.provenance_mode.into_value() {
            Some(ProjectRunProvenanceMode::LegacyUnattributed) => {
                if self.prepared_receipt.is_present() {
                    return Err(format!(
                        "simulation run sequence {} is legacy-unattributed but carries a prepared receipt",
                        self.id
                    ));
                }
                SimulationRunProvenance::LegacyUnattributed
            }
            Some(ProjectRunProvenanceMode::LegacyPreparedUnclassified) => {
                if self.prepared_receipt.is_present() {
                    return Err(format!(
                        "simulation run sequence {} is legacy prepared-unclassified but carries a current prepared receipt",
                        self.id
                    ));
                }
                SimulationRunProvenance::LegacyPreparedUnclassified
            }
            Some(ProjectRunProvenanceMode::PreparedTaskBound) => SimulationRunProvenance::Prepared(
                self.prepared_receipt
                    .into_value()
                    .ok_or_else(|| {
                        format!(
                            "simulation run sequence {} is prepared-task-bound but has no receipt",
                            self.id
                        )
                    })?
                    .into_receipt()?,
            ),
            Some(ProjectRunProvenanceMode::Unspecified) | None => {
                return Err(format!(
                    "simulation run sequence {} has no authoritative provenance mode",
                    self.id
                ));
            }
        };
        let mut run = SimulationRun::new(self.id);
        run.job_id = self.job_id;
        run.run_id = run_id;
        run.dataset_id = dataset_id;
        run.execution_target = self.execution_target;
        let restored_lifecycle = self
            .lifecycle
            .unwrap_or(SimulationRunLifecycle::LegacyUnknown);
        run.lifecycle = match restored_lifecycle {
            SimulationRunLifecycle::Preparing
            | SimulationRunLifecycle::Running
            | SimulationRunLifecycle::Cancelling => SimulationRunLifecycle::Interrupted,
            lifecycle => lifecycle,
        };
        run.label = self.label;
        run.timestamp = self.timestamp;
        run.analyses = analyses;
        run.elapsed_time = self.elapsed_time;
        run.success = if matches!(
            restored_lifecycle,
            SimulationRunLifecycle::Preparing
                | SimulationRunLifecycle::Running
                | SimulationRunLifecycle::Cancelling
        ) {
            false
        } else {
            self.success
        };
        run.restore_provenance(provenance)?;
        Ok(run)
    }

    fn validate(&self, run_idx: usize) -> Result<(), String> {
        require_finite(self.timestamp, &format!("runs[{run_idx}].timestamp"))?;
        require_finite(self.elapsed_time, &format!("runs[{run_idx}].elapsed_time"))?;
        let lifecycle = self.lifecycle.ok_or_else(|| {
            format!("runs[{run_idx}].lifecycle is required by simulation results schema v6")
        })?;
        match lifecycle {
            SimulationRunLifecycle::LegacyUnknown => {
                if self.job_id.is_some() || self.execution_target.is_some() {
                    return Err(format!(
                        "runs[{run_idx}] is legacy_unknown but carries current execution job/target identity"
                    ));
                }
            }
            _ => {
                if self.job_id.is_none() {
                    return Err(format!(
                        "runs[{run_idx}].job_id is required for a non-legacy lifecycle"
                    ));
                }
                if self.execution_target.is_none() {
                    return Err(format!(
                        "runs[{run_idx}].execution_target is required for a non-legacy lifecycle"
                    ));
                }
            }
        }
        match lifecycle {
            SimulationRunLifecycle::Completed if !self.success => {
                return Err(format!(
                    "runs[{run_idx}] is completed but its success outcome is false"
                ));
            }
            SimulationRunLifecycle::Preparing
            | SimulationRunLifecycle::Running
            | SimulationRunLifecycle::Cancelling
            | SimulationRunLifecycle::Failed
            | SimulationRunLifecycle::Aborted
            | SimulationRunLifecycle::Interrupted
                if self.success =>
            {
                return Err(format!(
                    "runs[{run_idx}] lifecycle {lifecycle:?} cannot carry a successful outcome"
                ));
            }
            SimulationRunLifecycle::LegacyUnknown
            | SimulationRunLifecycle::Preparing
            | SimulationRunLifecycle::Running
            | SimulationRunLifecycle::Cancelling
            | SimulationRunLifecycle::Completed
            | SimulationRunLifecycle::Failed
            | SimulationRunLifecycle::Aborted
            | SimulationRunLifecycle::Interrupted => {}
        }
        let mut analysis_ids = HashSet::new();
        let provenance_count = self
            .analyses
            .iter()
            .filter(|analysis| analysis.provenance.is_some())
            .count();
        let legacy_domain_count = self
            .analyses
            .iter()
            .filter_map(|analysis| analysis.provenance.as_ref())
            .filter(|provenance| {
                provenance.source_domain.as_ref()
                    == Some(&AnalysisResultSourceDomain::LegacyUnclassified)
            })
            .count();
        if self.provenance_mode.is_null() {
            return Err(format!(
                "runs[{run_idx}].provenance_mode must not be explicitly null"
            ));
        }
        if self.prepared_receipt.is_null() {
            return Err(format!(
                "runs[{run_idx}].prepared_receipt must not be explicitly null"
            ));
        }
        match self.provenance_mode.as_ref().copied() {
            None | Some(ProjectRunProvenanceMode::Unspecified) => {
                return Err(format!(
                    "runs[{run_idx}].provenance_mode is missing from current simulation results"
                ));
            }
            Some(ProjectRunProvenanceMode::LegacyUnattributed) if provenance_count != 0 => {
                return Err(format!(
                    "runs[{run_idx}] is marked legacy_unattributed but contains prepared-task provenance"
                ));
            }
            Some(ProjectRunProvenanceMode::LegacyPreparedUnclassified)
                if provenance_count != self.analyses.len()
                    || legacy_domain_count != self.analyses.len() =>
            {
                return Err(format!(
                    "runs[{run_idx}] is legacy_prepared_unclassified but its prepared source domains are not uniformly legacy-unclassified"
                ));
            }
            Some(ProjectRunProvenanceMode::PreparedTaskBound)
                if provenance_count != self.analyses.len() || legacy_domain_count != 0 =>
            {
                return Err(format!(
                    "runs[{run_idx}] is prepared_task_bound but {} of {} analyses lack provenance",
                    self.analyses.len() - provenance_count,
                    self.analyses.len()
                ));
            }
            Some(ProjectRunProvenanceMode::LegacyUnattributed)
            | Some(ProjectRunProvenanceMode::LegacyPreparedUnclassified)
            | Some(ProjectRunProvenanceMode::PreparedTaskBound) => {}
        }
        match (
            self.provenance_mode.as_ref().copied(),
            self.prepared_receipt.as_ref(),
        ) {
            (Some(ProjectRunProvenanceMode::PreparedTaskBound), Some(receipt)) => {
                receipt.validate()?;
            }
            (Some(ProjectRunProvenanceMode::PreparedTaskBound), None) => {
                return Err(format!(
                    "runs[{run_idx}] is prepared_task_bound but has no authenticated run receipt"
                ));
            }
            (
                Some(
                    ProjectRunProvenanceMode::LegacyUnattributed
                    | ProjectRunProvenanceMode::LegacyPreparedUnclassified,
                ),
                Some(_),
            ) => {
                return Err(format!(
                    "runs[{run_idx}] is legacy but carries a current prepared run receipt"
                ));
            }
            _ => {}
        }

        let mut source_instance_ids = HashSet::new();
        let mut prepared_snapshot_digest = None;
        let mut source_revision = None;
        let mut source_domain = None;
        for (analysis_idx, analysis) in self.analyses.iter().enumerate() {
            if analysis.id == 0 {
                return Err(format!(
                    "runs[{run_idx}].analyses[{analysis_idx}].id must be greater than zero"
                ));
            }
            if !analysis_ids.insert(analysis.id) {
                return Err(format!(
                    "duplicate analysis id {} in runs[{run_idx}]",
                    analysis.id
                ));
            }
            analysis.validate(run_idx, analysis_idx)?;

            if let Some(provenance) = &analysis.provenance {
                let prefix = format!("runs[{run_idx}].analyses[{analysis_idx}].provenance");
                if !source_instance_ids.insert(provenance.source_instance_id) {
                    return Err(format!(
                        "{prefix}.source_instance_id duplicates prepared analysis instance {}",
                        provenance.source_instance_id
                    ));
                }
                match source_domain {
                    Some(expected) if Some(&expected) != provenance.source_domain.as_ref() => {
                        return Err(format!(
                            "{prefix}.source_domain mixes prepared source domains within one run"
                        ));
                    }
                    None => {
                        source_domain =
                            Some(*provenance.source_domain.as_ref().ok_or_else(|| {
                                format!("{prefix}.source_domain is required by schema v5")
                            })?)
                    }
                    Some(_) => {}
                }
                match prepared_snapshot_digest {
                    Some(expected) if expected != provenance.prepared_snapshot_digest => {
                        return Err(format!(
                            "{prefix}.prepared_snapshot_digest does not match the run's frozen snapshot"
                        ));
                    }
                    None => prepared_snapshot_digest = Some(provenance.prepared_snapshot_digest),
                    Some(_) => {}
                }
                match source_revision {
                    Some(expected) if expected != provenance.source_revision => {
                        return Err(format!(
                            "{prefix}.source_revision does not match the run's frozen plan revision"
                        ));
                    }
                    None => source_revision = Some(provenance.source_revision),
                    Some(_) => {}
                }
                for dependency_id in &provenance.dependency_ids {
                    if !source_instance_ids.contains(dependency_id) {
                        return Err(format!(
                            "{prefix}.dependency_ids references {dependency_id} before that result appears in the frozen execution order"
                        ));
                    }
                }
            }
        }
        if let Some(receipt) = self.prepared_receipt.as_ref() {
            let receipt = receipt.clone().into_receipt()?;
            let analyses = self
                .analyses
                .iter()
                .cloned()
                .map(ProjectAnalysisResult::into_analysis)
                .collect::<Result<Vec<_>, _>>()?;
            receipt.validate_result_prefix(&analyses)?;
        }
        let retained_digest = self.dataset_content_digest.as_ref().copied().ok_or_else(|| {
            format!("runs[{run_idx}].dataset_content_digest is required by simulation results schema v9")
        })?;
        let computed_digest = self
            .clone()
            .into_run()
            .map_err(|error| {
                format!("runs[{run_idx}] cannot compute its dataset content digest: {error}")
            })?
            .dataset_content_digest();
        if retained_digest != computed_digest {
            return Err(format!(
                "runs[{run_idx}].dataset_content_digest does not match retained analysis content"
            ));
        }
        Ok(())
    }
}

impl From<&SimulationRun> for ProjectSimulationRun {
    fn from(run: &SimulationRun) -> Self {
        let (provenance_mode, prepared_receipt) = match run.provenance() {
            None => (PersistedField::Missing, PersistedField::Missing),
            Some(SimulationRunProvenance::LegacyUnattributed) => (
                PersistedField::Value(ProjectRunProvenanceMode::LegacyUnattributed),
                PersistedField::Missing,
            ),
            Some(SimulationRunProvenance::LegacyPreparedUnclassified) => (
                PersistedField::Value(ProjectRunProvenanceMode::LegacyPreparedUnclassified),
                PersistedField::Missing,
            ),
            Some(SimulationRunProvenance::Prepared(receipt)) => (
                PersistedField::Value(ProjectRunProvenanceMode::PreparedTaskBound),
                PersistedField::Value(ProjectPreparedRunReceipt::from(receipt)),
            ),
        };
        let success = if matches!(
            run.lifecycle,
            SimulationRunLifecycle::Preparing
                | SimulationRunLifecycle::Running
                | SimulationRunLifecycle::Cancelling
        ) {
            false
        } else {
            run.success
        };
        Self {
            job_id: run.job_id,
            run_id: Some(run.run_id),
            dataset_id: Some(run.dataset_id),
            execution_target: run.execution_target,
            lifecycle: Some(run.lifecycle),
            id: run.id,
            label: run.label.clone(),
            timestamp: run.timestamp,
            analyses: run
                .analyses
                .iter()
                .map(ProjectAnalysisResult::from)
                .collect(),
            dataset_content_digest: PersistedField::Value(run.dataset_content_digest()),
            provenance_mode,
            prepared_receipt,
            elapsed_time: run.elapsed_time,
            success,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectAnalysisResult {
    /// Run-local display sequence retained for labels and v1 migration.
    pub id: u64,
    pub analysis_type: String,
    pub label: String,
    pub timestamp: f64,
    /// Canonical identity of authoritative result samples and evidence.
    /// Schemas through v7 omitted this field and acquire it during migration.
    #[serde(default, skip_serializing_if = "PersistedField::is_missing")]
    pub result_data_digest: PersistedField<ContentDigest>,
    #[serde(default)]
    pub waveforms: Vec<ProjectWaveformData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dc_op: Option<ProjectDcOpResult>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub device_op: Option<ProjectDeviceOpReport>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub noise_summary: Option<ProjectNoiseSummary>,
    /// Exact source metadata for multi-run and advanced analysis families.
    /// Result schemas through v6 omitted this field and migrate to `None`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub family_metadata: Option<AnalysisResultFamilyMetadata>,
    /// Exact analysis-native result evidence. Schemas through v8 omitted this
    /// field; presence-aware persistence prevents a relabeled legacy document
    /// from injecting current-schema evidence.
    #[serde(default, skip_serializing_if = "PersistedField::is_missing")]
    pub result_payload: PersistedField<AnalysisResultPayload>,
    #[serde(default)]
    pub measurements: Vec<ProjectMeasurement>,
    /// Authenticated outcomes for the immutable saved-output contracts that
    /// applied to this analysis. Older project files legitimately omit it.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub saved_output_receipts: Vec<SavedOutputReceipt>,
    #[serde(default = "default_true")]
    pub success: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// Complete source identity for prepared-task results. `None` is retained only
    /// when loading result history written by v1/v2 projects.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provenance: Option<ProjectAnalysisResultProvenance>,
}

/// Project-file representation of one frozen prepared analysis task.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProjectAnalysisResultProvenance {
    #[serde(default, skip_serializing_if = "PersistedField::is_missing")]
    pub source_domain: PersistedField<AnalysisResultSourceDomain>,
    pub source_instance_id: AnalysisInstanceId,
    pub source_revision: ObjectRevision,
    pub prepared_snapshot_digest: ContentDigest,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dependency_ids: Vec<AnalysisInstanceId>,
}

impl ProjectAnalysisResultProvenance {
    fn into_provenance(self) -> Result<AnalysisResultProvenance, String> {
        AnalysisResultProvenance::new_with_source_domain(
            self.source_domain
                .into_value()
                .ok_or_else(|| "prepared result source_domain is missing or null".to_owned())?,
            self.source_instance_id,
            self.source_revision,
            self.prepared_snapshot_digest,
            self.dependency_ids,
        )
    }

    fn validate(&self) -> Result<(), String> {
        AnalysisResultProvenance::new_with_source_domain(
            self.source_domain
                .as_ref()
                .copied()
                .ok_or_else(|| "prepared result source_domain is missing or null".to_owned())?,
            self.source_instance_id,
            self.source_revision,
            self.prepared_snapshot_digest,
            self.dependency_ids.clone(),
        )
        .map(|_| ())
    }
}

impl From<&AnalysisResultProvenance> for ProjectAnalysisResultProvenance {
    fn from(provenance: &AnalysisResultProvenance) -> Self {
        Self {
            source_domain: PersistedField::Value(provenance.source_domain()),
            source_instance_id: provenance.source_instance_id(),
            source_revision: provenance.source_revision(),
            prepared_snapshot_digest: provenance.prepared_snapshot_digest(),
            dependency_ids: provenance.dependency_ids().to_vec(),
        }
    }
}

impl ProjectAnalysisResult {
    fn into_analysis(self) -> Result<AnalysisResult, String> {
        if self.result_data_digest.is_null() {
            return Err(format!(
                "analysis sequence {} has an explicitly null result data digest",
                self.id
            ));
        }
        if self.result_payload.is_null() {
            return Err(format!(
                "analysis sequence {} has an explicitly null result payload",
                self.id
            ));
        }
        let analysis_type = analysis_type_from_key(&self.analysis_type)
            .ok_or_else(|| format!("unknown persisted analysis type '{}'", self.analysis_type))?;
        let provenance = self
            .provenance
            .map(ProjectAnalysisResultProvenance::into_provenance)
            .transpose()?;
        let mut analysis = AnalysisResult {
            id: self.id,
            analysis_type,
            label: self.label,
            timestamp: self.timestamp,
            waveforms: self
                .waveforms
                .into_iter()
                .map(ProjectWaveformData::into_waveform)
                .collect(),
            dc_op: self.dc_op.map(ProjectDcOpResult::into_dc_op),
            device_op: self.device_op.map(ProjectDeviceOpReport::into_report),
            noise_summary: self
                .noise_summary
                .map(ProjectNoiseSummary::into_noise_summary),
            family_metadata: self.family_metadata,
            result_payload: self.result_payload.into_value(),
            measurements: self
                .measurements
                .into_iter()
                .map(ProjectMeasurement::into_measurement)
                .collect(),
            saved_output_receipts: self.saved_output_receipts,
            success: self.success,
            error_message: self.error_message,
            provenance,
        };
        for receipt in &analysis.saved_output_receipts {
            let SavedOutputMaterializationStatus::Materialized { waveform_name, .. } =
                &receipt.status
            else {
                continue;
            };
            if (receipt.stored_precision
                == crate::state::SavedOutputPrecision::DisplayCacheWithFullSourcePrecision
                || receipt.streaming
                    == crate::state::SavedOutputStreaming::LivePlotAdaptiveDisplayDecimation)
                && let Some(waveform) = analysis
                    .waveforms
                    .iter_mut()
                    .find(|waveform| waveform.name == *waveform_name)
            {
                waveform
                    .rebuild_display_cache(crate::state::DEFAULT_DISPLAY_WAVEFORM_CACHE_SAMPLES);
            }
        }
        Ok(analysis)
    }

    fn validate(&self, run_idx: usize, analysis_idx: usize) -> Result<(), String> {
        let prefix = format!("runs[{run_idx}].analyses[{analysis_idx}]");
        if analysis_type_from_key(&self.analysis_type).is_none() {
            return Err(format!(
                "{prefix}.analysis_type has unknown analysis type '{}'",
                self.analysis_type
            ));
        }
        require_finite(self.timestamp, &format!("{prefix}.timestamp"))?;
        let mut waveform_names = HashSet::new();
        for (waveform_idx, waveform) in self.waveforms.iter().enumerate() {
            if !waveform_names.insert(waveform.name.as_str()) {
                return Err(format!(
                    "{prefix} has duplicate waveform name '{}'",
                    waveform.name
                ));
            }
            waveform.validate(&format!("{prefix}.waveforms[{waveform_idx}]"))?;
        }
        if let Some(dc_op) = &self.dc_op {
            dc_op.validate(&format!("{prefix}.dc_op"))?;
        }
        if let Some(device_op) = &self.device_op {
            device_op.validate(&format!("{prefix}.device_op"))?;
        }
        if let Some(noise_summary) = &self.noise_summary {
            noise_summary.validate(&format!("{prefix}.noise_summary"))?;
        }
        if let Some(metadata) = &self.family_metadata {
            metadata
                .validate_for(
                    analysis_type_from_key(&self.analysis_type)
                        .expect("analysis type was checked above"),
                )
                .map_err(|error| format!("{prefix}.family_metadata is invalid: {error}"))?;
        }
        if self.result_payload.is_null() {
            return Err(format!("{prefix}.result_payload must not be null"));
        }
        if let Some(payload) = self.result_payload.as_ref() {
            payload
                .validate_for(
                    analysis_type_from_key(&self.analysis_type)
                        .expect("analysis type was checked above"),
                )
                .map_err(|error| format!("{prefix}.result_payload is invalid: {error}"))?;
            if !self.success {
                return Err(format!(
                    "{prefix}.result_payload is not permitted on a failed analysis"
                ));
            }
        }
        for (measurement_idx, measurement) in self.measurements.iter().enumerate() {
            measurement.validate(&format!("{prefix}.measurements[{measurement_idx}]"))?;
        }
        let mut receipt_ids = HashSet::new();
        let mut receipt_digests = HashSet::new();
        for (receipt_idx, receipt) in self.saved_output_receipts.iter().enumerate() {
            let receipt_prefix = format!("{prefix}.saved_output_receipts[{receipt_idx}]");
            if !receipt_ids.insert(receipt.output_id) {
                return Err(format!(
                    "{prefix} has duplicate saved-output identity {}",
                    receipt.output_id
                ));
            }
            if !receipt_digests.insert(receipt.contract_digest) {
                return Err(format!(
                    "{prefix} has duplicate saved-output contract digest {}",
                    receipt.contract_digest
                ));
            }
            if receipt.name.trim().is_empty() || receipt.source_expression.trim().is_empty() {
                return Err(format!(
                    "{receipt_prefix} has an empty name or source expression"
                ));
            }
            if let Some(provenance) = &self.provenance
                && receipt.analysis_id != provenance.source_instance_id
            {
                return Err(format!(
                    "{receipt_prefix} analysis identity does not match result provenance"
                ));
            }
            match &receipt.status {
                SavedOutputMaterializationStatus::Materialized {
                    waveform_name,
                    sample_count,
                } => {
                    if self.success
                        && receipt.save_policy
                            == crate::state::SavedOutputPolicy::FailureDiagnosticsOnly
                    {
                        return Err(format!(
                            "{receipt_prefix} materializes failure-only data on a successful analysis"
                        ));
                    }
                    let waveform = self
                        .waveforms
                        .iter()
                        .find(|waveform| waveform.name == *waveform_name)
                        .ok_or_else(|| {
                            format!(
                                "{receipt_prefix} names absent materialized waveform '{waveform_name}'"
                            )
                        })?;
                    if usize::try_from(*sample_count).ok() != Some(waveform.x.len()) {
                        return Err(format!(
                            "{receipt_prefix} sample count does not match waveform '{waveform_name}'"
                        ));
                    }
                }
                SavedOutputMaterializationStatus::SuppressedOnSuccess if !self.success => {
                    return Err(format!(
                        "{receipt_prefix} suppresses failure diagnostics on a failed analysis"
                    ));
                }
                SavedOutputMaterializationStatus::SuppressedOnSuccess
                    if receipt.save_policy
                        != crate::state::SavedOutputPolicy::FailureDiagnosticsOnly =>
                {
                    return Err(format!(
                        "{receipt_prefix} uses success suppression for a non-diagnostic save policy"
                    ));
                }
                SavedOutputMaterializationStatus::Deferred
                    if receipt.save_policy
                        != crate::state::SavedOutputPolicy::OnDemandFromRetainedState =>
                {
                    return Err(format!(
                        "{receipt_prefix} defers a save policy that requires immediate materialization"
                    ));
                }
                SavedOutputMaterializationStatus::Unavailable { reason }
                    if reason.trim().is_empty() =>
                {
                    return Err(format!(
                        "{receipt_prefix} has an empty unavailability reason"
                    ));
                }
                SavedOutputMaterializationStatus::Deferred
                | SavedOutputMaterializationStatus::SuppressedOnSuccess
                | SavedOutputMaterializationStatus::Unavailable { .. } => {}
            }
        }
        if let Some(provenance) = &self.provenance {
            provenance
                .validate()
                .map_err(|error| format!("{prefix}.provenance is invalid: {error}"))?;
        }
        let retained_digest = self.result_data_digest.as_ref().copied().ok_or_else(|| {
            format!("{prefix}.result_data_digest is required by simulation results schema v9")
        })?;
        let computed_digest = self
            .clone()
            .into_analysis()
            .map_err(|error| format!("{prefix} cannot compute its result data digest: {error}"))?
            .result_data_digest();
        if retained_digest != computed_digest {
            return Err(format!(
                "{prefix}.result_data_digest does not match retained analysis content"
            ));
        }
        Ok(())
    }
}

impl From<&AnalysisResult> for ProjectAnalysisResult {
    fn from(analysis: &AnalysisResult) -> Self {
        Self {
            id: analysis.id,
            analysis_type: analysis_type_key(analysis.analysis_type).to_string(),
            label: analysis.label.clone(),
            timestamp: analysis.timestamp,
            result_data_digest: PersistedField::Value(analysis.result_data_digest()),
            waveforms: analysis
                .waveforms
                .iter()
                .map(ProjectWaveformData::from)
                .collect(),
            dc_op: analysis.dc_op.as_ref().map(ProjectDcOpResult::from),
            device_op: analysis.device_op.as_ref().map(ProjectDeviceOpReport::from),
            noise_summary: analysis
                .noise_summary
                .as_ref()
                .map(ProjectNoiseSummary::from),
            family_metadata: analysis.family_metadata.clone(),
            result_payload: analysis
                .result_payload
                .clone()
                .map_or(PersistedField::Missing, PersistedField::Value),
            measurements: analysis
                .measurements
                .iter()
                .map(ProjectMeasurement::from)
                .collect(),
            saved_output_receipts: analysis.saved_output_receipts.clone(),
            success: analysis.success,
            error_message: analysis.error_message.clone(),
            provenance: analysis
                .provenance
                .as_ref()
                .map(ProjectAnalysisResultProvenance::from),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectWaveformData {
    pub name: String,
    pub x: Vec<f64>,
    pub y: Vec<f64>,
    pub color: String,
    #[serde(default = "default_true")]
    pub visible: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub complex: Option<ProjectComplexWaveformComponents>,
}

impl ProjectWaveformData {
    fn into_waveform(self) -> WaveformData {
        let mut waveform = WaveformData::new(self.name, self.x, self.y, self.color);
        waveform.visible = self.visible;
        if let Some(complex) = self.complex {
            waveform =
                waveform.with_complex_components(complex.source_name, complex.real, complex.imag);
        }
        waveform
    }

    fn validate(&self, prefix: &str) -> Result<(), String> {
        if self.x.len() != self.y.len() {
            return Err(format!(
                "{prefix} has mismatched x/y sample counts ({} vs {})",
                self.x.len(),
                self.y.len()
            ));
        }
        require_finite_values(&self.x, &format!("{prefix}.x"))?;
        require_monotonic_non_decreasing(&self.x, &format!("{prefix}.x"))?;
        require_finite_values(&self.y, &format!("{prefix}.y"))?;
        if let Some(complex) = &self.complex {
            complex.validate(prefix, self.y.len())?;
        }
        Ok(())
    }
}

impl From<&WaveformData> for ProjectWaveformData {
    fn from(waveform: &WaveformData) -> Self {
        Self {
            name: waveform.name.clone(),
            x: waveform.x.iter().copied().collect(),
            y: waveform.y.iter().copied().collect(),
            color: waveform.color.clone(),
            visible: waveform.visible,
            complex: waveform
                .complex
                .as_ref()
                .map(|complex| ProjectComplexWaveformComponents {
                    source_name: complex.source_name.clone(),
                    real: complex.real.iter().copied().collect(),
                    imag: complex.imag.iter().copied().collect(),
                }),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectComplexWaveformComponents {
    pub source_name: String,
    pub real: Vec<f64>,
    pub imag: Vec<f64>,
}

impl ProjectComplexWaveformComponents {
    fn validate(&self, prefix: &str, expected_len: usize) -> Result<(), String> {
        if self.real.len() != self.imag.len() || self.real.len() != expected_len {
            return Err(format!(
                "{prefix}.complex has mismatched real/imag/display sample counts"
            ));
        }
        require_finite_values(&self.real, &format!("{prefix}.complex.real"))?;
        require_finite_values(&self.imag, &format!("{prefix}.complex.imag"))?;
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ProjectDcOpResult {
    #[serde(default)]
    pub node_voltages: Vec<ProjectOperatingPointValue>,
    #[serde(default)]
    pub branch_currents: Vec<ProjectOperatingPointValue>,
    #[serde(default)]
    pub power_dissipation: Vec<ProjectOperatingPointValue>,
}

impl ProjectDcOpResult {
    fn into_dc_op(self) -> DcOpResult {
        DcOpResult {
            node_voltages: self
                .node_voltages
                .into_iter()
                .map(ProjectOperatingPointValue::into_op_value)
                .collect(),
            branch_currents: self
                .branch_currents
                .into_iter()
                .map(ProjectOperatingPointValue::into_op_value)
                .collect(),
            power_dissipation: self
                .power_dissipation
                .into_iter()
                .map(ProjectOperatingPointValue::into_op_value)
                .collect(),
        }
    }

    fn validate(&self, prefix: &str) -> Result<(), String> {
        for (idx, value) in self.node_voltages.iter().enumerate() {
            value.validate(&format!("{prefix}.node_voltages[{idx}]"))?;
        }
        for (idx, value) in self.branch_currents.iter().enumerate() {
            value.validate(&format!("{prefix}.branch_currents[{idx}]"))?;
        }
        for (idx, value) in self.power_dissipation.iter().enumerate() {
            value.validate(&format!("{prefix}.power_dissipation[{idx}]"))?;
        }
        Ok(())
    }
}

impl From<&DcOpResult> for ProjectDcOpResult {
    fn from(dc_op: &DcOpResult) -> Self {
        Self {
            node_voltages: dc_op
                .node_voltages
                .iter()
                .map(ProjectOperatingPointValue::from)
                .collect(),
            branch_currents: dc_op
                .branch_currents
                .iter()
                .map(ProjectOperatingPointValue::from)
                .collect(),
            power_dissipation: dc_op
                .power_dissipation
                .iter()
                .map(ProjectOperatingPointValue::from)
                .collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectOperatingPointValue {
    pub name: String,
    pub value: f64,
    pub unit: String,
}

impl ProjectOperatingPointValue {
    fn into_op_value(self) -> OperatingPointValue {
        OperatingPointValue {
            name: self.name,
            value: self.value,
            unit: self.unit,
        }
    }

    fn validate(&self, prefix: &str) -> Result<(), String> {
        require_finite(self.value, &format!("{prefix}.value"))
    }
}

impl From<&OperatingPointValue> for ProjectOperatingPointValue {
    fn from(value: &OperatingPointValue) -> Self {
        Self {
            name: value.name.clone(),
            value: value.value,
            unit: value.unit.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectNoiseSummary {
    #[serde(default)]
    pub rows: Vec<ProjectNoiseContributorRow>,
    pub total_rms: f64,
    pub band: (f64, f64),
}

impl ProjectNoiseSummary {
    fn into_noise_summary(self) -> NoiseSummary {
        NoiseSummary {
            rows: self
                .rows
                .into_iter()
                .map(ProjectNoiseContributorRow::into_row)
                .collect(),
            total_rms: self.total_rms,
            band: self.band,
        }
    }

    fn validate(&self, prefix: &str) -> Result<(), String> {
        require_finite(self.total_rms, &format!("{prefix}.total_rms"))?;
        require_finite(self.band.0, &format!("{prefix}.band.0"))?;
        require_finite(self.band.1, &format!("{prefix}.band.1"))?;
        for (idx, row) in self.rows.iter().enumerate() {
            row.validate(&format!("{prefix}.rows[{idx}]"))?;
        }
        Ok(())
    }
}

impl From<&NoiseSummary> for ProjectNoiseSummary {
    fn from(summary: &NoiseSummary) -> Self {
        Self {
            rows: summary
                .rows
                .iter()
                .map(ProjectNoiseContributorRow::from)
                .collect(),
            total_rms: summary.total_rms,
            band: summary.band,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectNoiseContributorRow {
    pub device: String,
    pub mechanism: String,
    pub power: f64,
    pub share_pct: f64,
}

impl ProjectNoiseContributorRow {
    fn into_row(self) -> NoiseContributorRow {
        NoiseContributorRow {
            device: self.device,
            mechanism: intern_static_label(self.mechanism),
            power: self.power,
            share_pct: self.share_pct,
        }
    }

    fn validate(&self, prefix: &str) -> Result<(), String> {
        require_static_label(&self.mechanism, &format!("{prefix}.mechanism"))?;
        require_finite(self.power, &format!("{prefix}.power"))?;
        require_finite(self.share_pct, &format!("{prefix}.share_pct"))
    }
}

impl From<&NoiseContributorRow> for ProjectNoiseContributorRow {
    fn from(row: &NoiseContributorRow) -> Self {
        Self {
            device: row.device.clone(),
            mechanism: row.mechanism.to_string(),
            power: row.power,
            share_pct: row.share_pct,
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ProjectDeviceOpReport {
    #[serde(default)]
    pub entries: Vec<ProjectDeviceOpEntry>,
}

impl ProjectDeviceOpReport {
    fn into_report(self) -> rspice_core::circuit::DeviceOpReport {
        rspice_core::circuit::DeviceOpReport {
            entries: self
                .entries
                .into_iter()
                .map(ProjectDeviceOpEntry::into_entry)
                .collect(),
        }
    }

    fn validate(&self, prefix: &str) -> Result<(), String> {
        for (idx, entry) in self.entries.iter().enumerate() {
            entry.validate(&format!("{prefix}.entries[{idx}]"))?;
        }
        Ok(())
    }
}

impl From<&rspice_core::circuit::DeviceOpReport> for ProjectDeviceOpReport {
    fn from(report: &rspice_core::circuit::DeviceOpReport) -> Self {
        Self {
            entries: report
                .entries
                .iter()
                .map(ProjectDeviceOpEntry::from)
                .collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectDeviceOpEntry {
    pub name: String,
    pub device_kind: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(default)]
    pub params: Vec<ProjectNamedValue>,
}

impl ProjectDeviceOpEntry {
    fn into_entry(self) -> rspice_core::circuit::DeviceOpEntry {
        rspice_core::circuit::DeviceOpEntry {
            name: self.name,
            device_kind: intern_static_label(self.device_kind),
            region: self.region.map(intern_static_label),
            params: self
                .params
                .into_iter()
                .map(|param| (intern_static_label(param.name), param.value))
                .collect(),
        }
    }

    fn validate(&self, prefix: &str) -> Result<(), String> {
        require_static_label(&self.device_kind, &format!("{prefix}.device_kind"))?;
        if let Some(region) = &self.region {
            require_static_label(region, &format!("{prefix}.region"))?;
        }
        for (idx, param) in self.params.iter().enumerate() {
            param.validate(&format!("{prefix}.params[{idx}]"))?;
        }
        Ok(())
    }
}

impl From<&rspice_core::circuit::DeviceOpEntry> for ProjectDeviceOpEntry {
    fn from(entry: &rspice_core::circuit::DeviceOpEntry) -> Self {
        Self {
            name: entry.name.clone(),
            device_kind: entry.device_kind.to_string(),
            region: entry.region.map(str::to_string),
            params: entry
                .params
                .iter()
                .map(|(name, value)| ProjectNamedValue {
                    name: (*name).to_string(),
                    value: *value,
                })
                .collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectNamedValue {
    pub name: String,
    pub value: f64,
}

impl ProjectNamedValue {
    fn validate(&self, prefix: &str) -> Result<(), String> {
        require_static_label(&self.name, &format!("{prefix}.name"))?;
        require_finite(self.value, &format!("{prefix}.value"))
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectMeasurement {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    pub passed: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expected: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tolerance: Option<f64>,
}

impl ProjectMeasurement {
    fn into_measurement(self) -> rspice_core::MeasureResult {
        rspice_core::MeasureResult {
            name: self.name,
            value: self.value,
            error: self.error,
            passed: self.passed,
            expected: self.expected,
            tolerance: self.tolerance,
        }
    }

    fn validate(&self, prefix: &str) -> Result<(), String> {
        require_optional_finite(self.value, &format!("{prefix}.value"))?;
        require_optional_finite(self.expected, &format!("{prefix}.expected"))?;
        require_optional_finite(self.tolerance, &format!("{prefix}.tolerance"))
    }
}

impl From<&rspice_core::MeasureResult> for ProjectMeasurement {
    fn from(measurement: &rspice_core::MeasureResult) -> Self {
        Self {
            name: measurement.name.clone(),
            value: measurement.value,
            error: measurement.error.clone(),
            passed: measurement.passed,
            expected: measurement.expected,
            tolerance: measurement.tolerance,
        }
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
        crate::common::browser_download::download_text_file(path, &contents)
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
    crate::common::file_actions::ensure_file_extension(&mut path, "rspiceproj");
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
    let mut value: serde_json::Value =
        serde_json::from_str(contents).map_err(|e| ProjectIoError::ParseError(e.to_string()))?;
    if let Some(descriptor) = value
        .get_mut("workspace")
        .and_then(|workspace| workspace.get_mut("project"))
        .and_then(serde_json::Value::as_object_mut)
        && !descriptor.contains_key("schema_version")
        && !descriptor.contains_key("id")
    {
        let legacy_id = ProjectId::from_namespace(LEGACY_PROJECT_ID_NAMESPACE, contents.as_bytes());
        descriptor.insert(
            "id".to_owned(),
            serde_json::Value::String(legacy_id.to_string()),
        );
    }
    let mut project: ProjectFile =
        serde_json::from_value(value).map_err(|e| ProjectIoError::ParseError(e.to_string()))?;
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

    fn seal_legacy_unattributed(run: &mut SimulationRun) {
        run.restore_provenance(SimulationRunProvenance::LegacyUnattributed)
            .expect("legacy fixture seals explicitly");
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

    fn project_with_execution_context() -> ProjectFile {
        use crate::common::simulation_analysis_tabs::{TAB_AC, TAB_NOISE, TAB_TRANSIENT};
        use crate::simulation::dialog::{DampingStrategy, IntegrationMethod, MatrixSolver};
        use crate::state::model_library::{
            DeviceModel, ModelLibrary, ModelLibraryManager, ModelType,
        };

        let mut design_libraries = LibraryManager::with_primitives();
        let workspace = ProjectWorkspace::new_bootstrapped(&mut design_libraries);

        let mut setup = crate::common::app::SimSetupState::new();
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
        assert!(error.contains("source_closure cannot exist without an external root_path"));
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
                AnalysisResultPayload::ScalarMeasurements {
                    values: std::collections::BTreeMap::from([("gain".to_owned(), 10.0)]),
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
        migrated.validate().expect("resealed v9 results validate");
        assert_ne!(
            migrated.runs[0].dataset_content_digest, v8.runs[0].dataset_content_digest,
            "v9 uses the new canonical digest domain"
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
        let mut second = legacy;
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
                        total_rms: 1.0e-6,
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
