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
    AnalysisInstanceId, ContentDigest, DatasetId, DerivedAnalysisIdentity, JobId, ModelSourceId,
    ObjectRevision, ProjectId, RunId, SimulationPlanId,
};
use crate::simulation::plan::AnalysisKind;
use crate::state::workspace::validate_cell_view_name_segment;
use crate::state::{
    AnalysisResult, AnalysisResultFamilyMetadata, AnalysisResultPayload, AnalysisResultProvenance,
    AnalysisResultPvtPoint, AnalysisResultSourceDomain, AnalysisType, CanonicalCellViewOwnerKey,
    CellViewRef, ConfigurationSet, DcOpResult, ExecutionTarget, InstancePath, InstancePathPattern,
    LibraryManager, NoiseContributorRow, NoiseSummary, OperatingPointValue, PatternSegment,
    PreparedModelSourceIdentity, PreparedRunReceipt, PreparedRunTaskReceipt,
    PreparedSourceCheckReceipt, PreparedSpecification, PreparedSpecificationPolicy,
    ProjectWorkspace, RunRetention, SavedOutputMaterializationStatus, SavedOutputReceipt,
    SimulationRun, SimulationRunLifecycle, SimulationRunProvenance, SimulationState, SpecEntry,
    SpecificationDefinition, SpecificationPolicy, SpecificationVerdict, ViewType, WaveformData,
    canonical_cell_view_owner_key,
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
    #[cfg(test)]
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

fn validate_physical_layout_hierarchy(
    hierarchy: &HashMap<String, Vec<String>>,
) -> Result<(), ProjectIoError> {
    let mut colors = HashMap::<String, u8>::with_capacity(hierarchy.len());
    for root in hierarchy.keys() {
        if colors.get(root).copied().unwrap_or(0) != 0 {
            continue;
        }
        colors.insert(root.clone(), 1);
        let mut stack = vec![(root.clone(), 0_usize)];
        while let Some((owner, next_index)) = stack.pop() {
            let masters = hierarchy.get(&owner).ok_or_else(|| {
                ProjectIoError::InvalidData(format!(
                    "physical-layout hierarchy owner {owner} has no authoritative document"
                ))
            })?;
            if next_index >= masters.len() {
                colors.insert(owner.clone(), 2);
                continue;
            }
            let master = masters[next_index].clone();
            stack.push((owner, next_index + 1));
            match colors.get(&master).copied().unwrap_or(0) {
                0 => {
                    if !hierarchy.contains_key(&master) {
                        return Err(ProjectIoError::InvalidData(format!(
                            "physical-layout hierarchy master {master} has no authoritative document"
                        )));
                    }
                    colors.insert(master.clone(), 1);
                    stack.push((master, 0));
                }
                1 => {
                    return Err(ProjectIoError::InvalidData(format!(
                        "physical-layout hierarchy contains a recursive cycle through {master}"
                    )));
                }
                _ => {}
            }
        }
    }
    Ok(())
}

/// How many repaired identities a load warning spells out before it says
/// "and N more". A warning nobody finishes reading has repaired nothing.
const MAX_NAMED_LOAD_REPAIRS: usize = 8;

/// The first edge that closes a loop in the instantiation graph, as
/// `(owner, master)` positions into the same node list the edges index.
fn first_instantiation_back_edge(edges: &[Vec<usize>]) -> Option<(usize, usize)> {
    const UNVISITED: u8 = 0;
    const ON_PATH: u8 = 1;
    const SETTLED: u8 = 2;

    let mut color = vec![UNVISITED; edges.len()];
    for start in 0..edges.len() {
        if color[start] != UNVISITED {
            continue;
        }
        color[start] = ON_PATH;
        let mut stack = vec![(start, 0_usize)];
        while let Some((owner, next_index)) = stack.pop() {
            let Some(master) = edges[owner].get(next_index).copied() else {
                color[owner] = SETTLED;
                continue;
            };
            stack.push((owner, next_index + 1));
            match color[master] {
                UNVISITED => {
                    color[master] = ON_PATH;
                    stack.push((master, 0));
                }
                ON_PATH => return Some((owner, master)),
                _ => {}
            }
        }
    }
    None
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
    /// Markers the reader placed on the result plots.
    ///
    /// They are annotations *about* the retained datasets, not part of them,
    /// so they live beside the result document rather than inside it: a
    /// marker must never alter a result's provenance digest.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub result_markers: Vec<ProjectResultMarker>,
    /// Waveform panes the reader put on a logarithmic Y axis.
    ///
    /// The same kind of fact as a marker and kept the same way: a decision
    /// about how to read a retained dataset, living beside the result
    /// document rather than inside it so it cannot alter a provenance digest.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub result_log_y_panes: Vec<ProjectResultLogYPane>,
    /// Stable, project-owned calculated waveform traces grouped by immutable
    /// dataset and analysis identity.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub(crate) result_expression_groups: Vec<ProjectResultExpressionGroup>,
    /// Authoritative execution inputs. Absent only in projects written before
    /// project-owned simulation plans were introduced.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub execution_context: Option<ProjectExecutionContext>,
    #[serde(skip)]
    pub simulation_results_warning: Option<String>,
    /// What opening this project had to repair before it could be worked in:
    /// what migrating the persisted workspace forward changed, and what the
    /// hierarchical placements had to give up to address anything. Like the
    /// results warning it describes this load rather than the project, so it
    /// is never written back.
    #[serde(skip)]
    pub workspace_migration_warning: Option<String>,
}

/// One retained result marker, as written to the project.
///
/// The identities are the workspace's own dataset-bound presentation keys, so
/// a marker re-attaches to the same analysis and trace after a reload however
/// the retained analyses were reordered — and silently drops if the dataset
/// it annotated is no longer in the project.
pub type ProjectResultMarker = crate::workbench::documents::result_document::ResultMarker;
pub type ProjectResultLogYPane =
    crate::workbench::documents::result_document::WavePanePresentationKey;

const MAX_PROJECT_RESULT_EXPRESSION_GROUPS: usize = 4_096;
const MAX_PROJECT_RESULT_EXPRESSIONS_TOTAL: usize = 16_384;
const MAX_PROJECT_RESULT_EXPRESSIONS_PER_ANALYSIS: usize = 512;
const MAX_PROJECT_RESULT_EXPRESSION_BYTES: usize = 4_096;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct ProjectResultExpressionGroup {
    pub analysis: crate::workbench::documents::result_document::AnalysisPresentationKey,
    pub traces: Vec<crate::workbench::documents::result_document::ExprTrace>,
}

impl ProjectFile {
    #[cfg(test)]
    pub fn new(workspace: ProjectWorkspace, libraries: LibraryManager) -> Self {
        Self {
            version: ProjectVersion::current(),
            workspace,
            libraries,
            simulation_results: ProjectSimulationResults::default(),
            result_markers: Vec::new(),
            result_log_y_panes: Vec::new(),
            result_expression_groups: Vec::new(),
            execution_context: None,
            simulation_results_warning: None,
            workspace_migration_warning: None,
        }
    }

    #[cfg(test)]
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
            result_markers: Vec::new(),
            result_log_y_panes: Vec::new(),
            result_expression_groups: Vec::new(),
            execution_context: None,
            simulation_results_warning: None,
            workspace_migration_warning: None,
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
            result_markers: Vec::new(),
            result_log_y_panes: Vec::new(),
            result_expression_groups: Vec::new(),
            execution_context: Some(execution_context),
            simulation_results_warning: None,
            workspace_migration_warning: None,
        }
    }

    /// Attach the reader's plot markers to a snapshot.
    #[must_use]
    pub fn with_result_markers(mut self, markers: Vec<ProjectResultMarker>) -> Self {
        self.result_markers = markers;
        self
    }

    /// Attach the logarithmic-axis pane choices to a snapshot.
    #[must_use]
    pub fn with_result_log_y_panes(mut self, panes: Vec<ProjectResultLogYPane>) -> Self {
        self.result_log_y_panes = panes;
        self
    }

    /// Attach stable calculated waveform traces to a project snapshot.
    #[must_use]
    pub(crate) fn with_result_expression_groups(
        mut self,
        groups: Vec<ProjectResultExpressionGroup>,
    ) -> Self {
        self.result_expression_groups = groups;
        self
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
        self.validate_result_expression_groups()?;
        let view_index = self.validate_library_tree()?;
        self.validate_project_source_owners(&view_index)?;
        self.validate_workspace_references(&view_index)?;
        self.validate_physical_layout_references(&view_index)?;
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

    fn validate_result_expression_groups(&self) -> Result<(), ProjectIoError> {
        if self.result_expression_groups.len() > MAX_PROJECT_RESULT_EXPRESSION_GROUPS {
            return Err(ProjectIoError::InvalidData(format!(
                "result expression group count {} exceeds the supported limit of {MAX_PROJECT_RESULT_EXPRESSION_GROUPS}",
                self.result_expression_groups.len()
            )));
        }
        let mut analyses = HashSet::new();
        let mut total = 0_usize;
        for group in &self.result_expression_groups {
            if !analyses.insert(group.analysis) {
                return Err(ProjectIoError::InvalidData(
                    "result expression groups contain a duplicate stable analysis identity"
                        .to_owned(),
                ));
            }
            if group.traces.is_empty()
                || group.traces.len() > MAX_PROJECT_RESULT_EXPRESSIONS_PER_ANALYSIS
            {
                return Err(ProjectIoError::InvalidData(format!(
                    "each result expression group must contain 1 to {MAX_PROJECT_RESULT_EXPRESSIONS_PER_ANALYSIS} traces"
                )));
            }
            total = total.saturating_add(group.traces.len());
            if total > MAX_PROJECT_RESULT_EXPRESSIONS_TOTAL {
                return Err(ProjectIoError::InvalidData(format!(
                    "result expression count exceeds the supported limit of {MAX_PROJECT_RESULT_EXPRESSIONS_TOTAL}"
                )));
            }
            let mut expressions = HashSet::new();
            for trace in &group.traces {
                if trace.text.is_empty()
                    || trace.text.len() > MAX_PROJECT_RESULT_EXPRESSION_BYTES
                    || trace.text.chars().any(char::is_control)
                {
                    return Err(ProjectIoError::InvalidData(format!(
                        "result expressions must contain 1 to {MAX_PROJECT_RESULT_EXPRESSION_BYTES} non-control UTF-8 bytes"
                    )));
                }
                if !expressions.insert(trace.text.as_str()) {
                    return Err(ProjectIoError::InvalidData(
                        "a result expression group contains duplicate expression text".to_owned(),
                    ));
                }
            }
        }
        Ok(())
    }

    /// Bring every hierarchical placement in the loaded workspace back to
    /// something that addresses the project it arrived in, and say what that
    /// cost. Never refuses: this returns repairs, not errors.
    ///
    /// It asks the same two questions of schematic hierarchy that
    /// [`Self::validate_physical_layout_references`] asks of layout — does
    /// every instance name a master that exists, and does the graph close on
    /// itself — and answers them the opposite way, because the two documents
    /// are written by different hands. A layout catalog is only ever written
    /// by this application, so a dangling master there is corruption. A
    /// schematic placement outlives the master it names by design: deleting a
    /// cell leaves its placements drawn, exactly so a reader can see what was
    /// lost. Refusing the load would mean a project that opened yesterday
    /// cannot be opened today.
    ///
    /// So a missing master leaves its placements unresolved, an instantiation
    /// loop has the edge that closes it cut, and both are reported to the
    /// reader rather than applied behind their back.
    fn validate_schematic_instance_masters(&mut self) -> Vec<String> {
        let mut repairs = Vec::new();
        let mut missing = Vec::new();
        for schematic in self.workspace.schematic_buffers.values_mut() {
            missing.extend(schematic.revalidate_instance_bindings(&self.libraries));
        }
        missing.sort_unstable();
        missing.dedup();
        if !missing.is_empty() {
            let named = missing
                .iter()
                .take(MAX_NAMED_LOAD_REPAIRS)
                .cloned()
                .collect::<Vec<_>>()
                .join(", ");
            let rest = missing.len().saturating_sub(MAX_NAMED_LOAD_REPAIRS);
            let tail = if rest > 0 {
                format!(" and {rest} more")
            } else {
                String::new()
            };
            repairs.push(format!(
                "Instances in this project name masters it no longer holds ({named}{tail}). They were opened unresolved, so nothing netlists them until those masters exist again."
            ));
        }
        repairs.extend(self.cut_schematic_instantiation_cycles());
        repairs
    }

    /// Cut every edge that makes the schematic hierarchy close on itself, and
    /// name both ends of each one. Cutting one edge can expose another, so
    /// this runs until the graph is acyclic.
    fn cut_schematic_instantiation_cycles(&mut self) -> Vec<String> {
        // Sorted so which edge a loop loses is a property of the project, not
        // of a hash order that changes between runs.
        let mut owners = self
            .workspace
            .schematic_buffers
            .keys()
            .cloned()
            .collect::<Vec<_>>();
        owners.sort_unstable();
        let index = owners
            .iter()
            .enumerate()
            .map(|(position, key)| (key.clone(), position))
            .collect::<HashMap<_, _>>();
        let mut edges = owners
            .iter()
            .map(|key| {
                let mut masters = self.workspace.schematic_buffers[key]
                    .components
                    .iter()
                    .filter_map(|component| component.library_cell.as_ref())
                    .filter_map(|binding| {
                        index
                            .get(
                                &CellViewRef::new(&binding.library, &binding.cell, &binding.view)
                                    .key(),
                            )
                            .copied()
                    })
                    .collect::<Vec<_>>();
                masters.sort_unstable();
                masters.dedup();
                masters
            })
            .collect::<Vec<_>>();

        let mut repairs = Vec::new();
        while let Some((owner, master)) = first_instantiation_back_edge(&edges) {
            edges[owner].retain(|candidate| *candidate != master);
            let master_key = owners[master].clone();
            let schematic = self
                .workspace
                .schematic_buffers
                .get_mut(&owners[owner])
                .expect("the buffer the cycle graph was built from remains present");
            schematic.components.retain(|component| {
                component.library_cell.as_ref().is_none_or(|binding| {
                    CellViewRef::new(&binding.library, &binding.cell, &binding.view).key()
                        != master_key
                })
            });
            schematic.recalculate_runtime_state();
            schematic.bump_topology_version();
            repairs.push(format!(
                "'{}' instantiated '{master_key}', which reaches back to it. That placement was removed so the hierarchy can be walked.",
                owners[owner]
            ));
        }
        repairs
    }

    fn validate_physical_layout_references(
        &self,
        view_index: &ValidatedLibraryIndex,
    ) -> Result<(), ProjectIoError> {
        let mut hierarchy = HashMap::<String, Vec<String>>::new();
        let signed_pin = self
            .workspace
            .project
            .technology_binding()
            .and_then(crate::state::ProjectTechnologyBinding::signed_package);
        for (key, document) in self.workspace.physical_layout_documents() {
            let mut masters = Vec::with_capacity(document.instances().len());
            let owner = view_index.get_exact(key).ok_or_else(|| {
                ProjectIoError::InvalidData(format!(
                    "workspace.physical_layout_documents[{key}] has no exact owning library view"
                ))
            })?;
            if &owner.reference != document.owner() || owner.view_type != ViewType::Layout {
                return Err(ProjectIoError::InvalidData(format!(
                    "workspace.physical_layout_documents[{key}] must be owned by that exact Layout view"
                )));
            }
            let signed_pin = signed_pin.ok_or_else(|| {
                ProjectIoError::InvalidData(format!(
                    "workspace.physical_layout_documents[{key}] requires an exact signed project PDK pin"
                ))
            })?;
            let technology = document.technology();
            if technology.package_id() != signed_pin.package_id()
                || technology.revision() != signed_pin.revision()
                || technology.manifest_digest() != signed_pin.manifest_digest()
                || technology.archive_digest() != signed_pin.archive_digest()
                || technology.stack_id() != signed_pin.stack_name()
            {
                return Err(ProjectIoError::InvalidData(format!(
                    "workspace.physical_layout_documents[{key}] technology binding does not match the project's exact signed PDK pin"
                )));
            }
            for (instance_id, instance) in document.instances() {
                let master_key = instance.master.key();
                let master = view_index.get_exact(&master_key).ok_or_else(|| {
                    ProjectIoError::InvalidData(format!(
                        "workspace.physical_layout_documents[{key}].instances[{instance_id}] references missing exact master {master_key}"
                    ))
                })?;
                if master.reference != instance.master || master.view_type != ViewType::Layout {
                    return Err(ProjectIoError::InvalidData(format!(
                        "workspace.physical_layout_documents[{key}].instances[{instance_id}] master {master_key} is not that exact Layout view"
                    )));
                }
                if self
                    .workspace
                    .physical_layout_document(&instance.master)
                    .is_none()
                {
                    return Err(ProjectIoError::InvalidData(format!(
                        "workspace.physical_layout_documents[{key}].instances[{instance_id}] master {master_key} has no authoritative physical-layout document"
                    )));
                }
                masters.push(master_key);
            }
            hierarchy.insert(key.clone(), masters);
        }
        validate_physical_layout_hierarchy(&hierarchy)?;
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
            for (index, specification) in
                record.payload.specification_definitions.iter().enumerate()
            {
                if let Some(analysis_id) = specification.producing_analysis
                    && plan.instance(analysis_id).is_none()
                {
                    return Err(ProjectIoError::InvalidData(format!(
                        "workspace.simulation_plan_payloads[{plan_id}].specification_definitions[{index}].producing_analysis references analysis {analysis_id}, which is absent from its owning plan"
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
                if self.workspace.plan_data(plan_id).is_none() {
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
            if receipt.source_domain != AnalysisResultSourceDomain::SimulationPlan
                || receipt.simulation_plan_id != Some(record.plan_id)
            {
                return Err(format!(
                    "regression baseline {run_id} does not belong to simulation plan {}",
                    record.plan_id
                ));
            }
            if run.analyses.len() != receipt.tasks.len()
                || run.analyses.iter().any(|analysis| !analysis.success)
            {
                return Err(format!(
                    "regression baseline {run_id} is incomplete or unsuccessful"
                ));
            }
            if !run.retention.is_pinned() {
                return Err(format!(
                    "regression baseline {run_id} is not pinned against retention pruning"
                ));
            }
        }
        Ok(())
    }

    /// Repair projects saved by builds that persisted a regression-baseline
    /// reference without also pinning its run. The reference already names an
    /// authenticated immutable dataset; adding its required retention class
    /// restores the invariant without changing engineering evidence.
    fn pin_regression_baseline_references(&mut self) -> usize {
        let baseline_ids = self
            .workspace
            .simulation_plan_payloads
            .iter()
            .filter_map(|record| record.payload.regression_baseline_run)
            .collect::<std::collections::HashSet<_>>();
        let mut repaired = 0usize;
        for run in &mut self.simulation_results.runs {
            if run
                .run_id
                .is_some_and(|run_id| baseline_ids.contains(&run_id))
                && !run.retention.is_pinned()
            {
                run.retention = crate::state::RunRetention::GoldenBaseline;
                repaired += 1;
            }
        }
        repaired
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
    ///
    /// Closure is over *authored* instances, and a run may dispatch more tasks
    /// than the plan authored — see [`authored_instance_for_task`] for the one
    /// rule that closes a derived task over the instance it derived from.
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
        let plans = if has_plan_receipt {
            Some(simulation_plan.ok_or_else(|| {
                "simulation-plan result history has no persisted simulation plan".to_owned()
            })?)
        } else {
            None
        };

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
                    let plan_id = receipt.simulation_plan_id.ok_or_else(|| {
                        format!(
                            "runs[{run_idx}].prepared_receipt has simulation-plan provenance without a plan identity"
                        )
                    })?;
                    let plans = plans.expect("plan receipt precondition established above");
                    let plan = plans
                        .stable_analysis_plan()
                        .ok()
                        .filter(|plan| plan.id() == plan_id)
                        .or_else(|| {
                            plans
                                .inactive_plans()
                                .iter()
                                .find(|stored| stored.id() == plan_id)
                                .map(|stored| stored.analysis_plan())
                        })
                        .ok_or_else(|| {
                            format!(
                                "runs[{run_idx}].prepared_receipt.simulation_plan_id {plan_id} is absent from the persisted plan catalog"
                            )
                        })?;
                    let current = plan
                        .instances()
                        .iter()
                        .map(|instance| (instance.id(), instance))
                        .collect::<HashMap<_, _>>();
                    let retired = plan
                        .tombstones()
                        .iter()
                        .map(|tombstone| (tombstone.id(), tombstone))
                        .collect::<HashMap<_, _>>();
                    let produced_task_ids = run
                        .analyses
                        .iter()
                        .filter_map(|analysis| analysis.provenance.as_ref())
                        .map(|provenance| provenance.source_instance_id)
                        .collect::<HashSet<_>>();
                    for (task_idx, task) in receipt.tasks.iter().enumerate() {
                        let prefix = format!("runs[{run_idx}].prepared_receipt.tasks[{task_idx}]");
                        let authored_id = authored_instance_for_task(&prefix, task)?;
                        let (kind, created_revision, latest_revision, is_retired) = if let Some(
                            instance,
                        ) =
                            current.get(&authored_id)
                        {
                            (
                                instance.kind(),
                                instance.created_revision(),
                                plan.revision(),
                                false,
                            )
                        } else if let Some(tombstone) = retired.get(&authored_id) {
                            if produced_task_ids.contains(&task.source_instance_id) {
                                let run_id = run.run_id.ok_or_else(|| {
                                        format!(
                                            "runs[{run_idx}] has prepared provenance but no stable run id"
                                        )
                                    })?;
                                if !tombstone.prior_run_ids().contains(&run_id) {
                                    return Err(format!(
                                        "{prefix}.source_instance_id identifies retired analysis {authored_id}, but run {run_id} is not retained by its tombstone"
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
                                "{prefix}.source_instance_id {authored_id} is absent from the persisted plan and its tombstones"
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
                        // A derived task runs an analysis the plan never
                        // authored — a declared space's points run its base
                        // analysis, and a PSS's spectrum is a different kind
                        // entirely — so only an authored task's tag can be
                        // held to the authored kind. Binding the tag into the
                        // derivation material instead would authenticate
                        // nothing: the derivation is public and a forged record
                        // chooses its own role and tag together. What keeps a
                        // derived tag honest is the closed canonical protocol
                        // the receipt layer enforces, and the positional
                        // result-prefix check that pairs each retained result
                        // with the task whose family it claims.
                        if task.derived_from.is_none()
                            && task.analysis_kind_tag != analysis_kind_tag_for_plan_kind(kind)
                        {
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
                        // A manual deck authors no plan instance, so there is
                        // nothing for a task of one to be derived from: its
                        // identity comes from the expanded source itself.
                        if task.derived_from.is_some() {
                            return Err(format!(
                                "runs[{run_idx}].prepared_receipt.tasks[{task_idx}] is a manual-deck task and cannot derive from a plan instance"
                            ));
                        }
                        let occurrence = occurrences.entry(task.analysis_kind_tag).or_default();
                        let expected = crate::product::manual_deck_analysis_instance_id_from_tag(
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
            validate_configured_scopes(index, configuration)?;
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
            // Every master the document's occurrence passes through is a
            // persisted reference in its own right, and an occurrence that
            // names a cell the libraries no longer hold addresses nothing.
            for (level, master) in open_view.occurrence.masters().enumerate() {
                self.validate_library_reference(
                    &format!("workspace.open_views[{index}].occurrence[{level}]"),
                    master,
                )?;
            }
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

/// Refuse a project whose configured scopes could never be netlisted.
///
/// The catalog rewrites the pre-implicit-root spelling while it deserializes,
/// so a path that still fails to parse here is corrupt rather than old. Beyond
/// parsing, a scope that names an exact instance has to be one the engine can
/// address: engine node names are ASCII and the design root is not an instance,
/// so a concrete scope that fails either test would only be discovered at
/// netlist time — by which point the project is already open and the user is
/// several edits past the file that broke it.
fn validate_configured_scopes(
    index: usize,
    configuration: &ConfigurationSet,
) -> Result<(), ProjectIoError> {
    let field = format!("workspace.configuration_sets.configurations[{index}]");
    let dut = InstancePath::parse(configuration.dut_path()).map_err(|error| {
        ProjectIoError::InvalidData(format!(
            "{field}.dut_path '{}' is not a hierarchical instance path: {error}",
            configuration.dut_path()
        ))
    })?;
    if !dut.is_root() {
        dut.to_engine_name().map_err(|error| {
            ProjectIoError::InvalidData(format!(
                "{field}.dut_path '{dut}' cannot be netlisted: {error}"
            ))
        })?;
    }

    for (scoped_index, scoped) in configuration.overrides().iter().enumerate() {
        let scope = format!("{field}.overrides[{scoped_index}].instance_path");
        let pattern = InstancePathPattern::parse(&scoped.instance_path).map_err(|error| {
            ProjectIoError::InvalidData(format!(
                "{scope} '{}' is not a hierarchy scope: {error}",
                scoped.instance_path
            ))
        })?;
        if pattern.specificity() != pattern.depth() {
            continue;
        }
        if pattern.is_root() {
            return Err(ProjectIoError::InvalidData(format!(
                "{scope} names the design root; an override must name at least one instance"
            )));
        }
        let mut instance = InstancePath::root();
        for segment in pattern.segments() {
            let PatternSegment::Name(name) = segment else {
                continue;
            };
            instance = instance.child(name).map_err(|error| {
                ProjectIoError::InvalidData(format!(
                    "{scope} '{pattern}' is not a hierarchical instance path: {error}"
                ))
            })?;
        }
        instance.to_engine_name().map_err(|error| {
            ProjectIoError::InvalidData(format!("{scope} '{pattern}' cannot be netlisted: {error}"))
        })?;
    }
    Ok(())
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
const OPERATING_POINT_RESULTS_SCHEMA_VERSION: u32 = 12;
const WAVEFORM_UNIT_RESULTS_SCHEMA_VERSION: u32 = 13;
const GOVERNED_SPECIFICATION_RESULTS_SCHEMA_VERSION: u32 = 14;
const PROJECT_SIMULATION_RESULTS_SCHEMA_VERSION: u32 = 15;

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

/// The tag a persisted receipt task must carry for a plan analysis of `kind`.
///
/// Derived from the canonical tag table, not restated: this check exists to
/// catch a persisted receipt whose task claims the wrong analysis, and a
/// second list of numbers here would instead catch the day dispatch learned a
/// tag this file had not been told about.
const fn analysis_kind_tag_for_plan_kind(kind: AnalysisKind) -> u8 {
    kind.canonical_kind().tag()
}

/// The authored plan instance a persisted receipt task must be closed over.
///
/// This is the whole rule that lets an expanded run be persisted without
/// weakening what the closure check exists to stop. A task the plan authored
/// answers for itself. A task the run derived answers for the instance its
/// derivation roots at, and only if that derivation actually produces the
/// identity the task claims — recomputed here through the same fold that
/// minted it. The caller then applies every remaining check to the authored
/// instance, so a derived task is admitted on exactly the authority its
/// producer has and never on its own say-so.
///
/// The anti-forgery property therefore survives transitively: a receipt can
/// only name identities that root at instances the plan authored, and a forged
/// derivation record buys nothing an honest expansion of the same producer
/// could not have minted.
fn authored_instance_for_task(
    prefix: &str,
    task: &ProjectPreparedRunTaskReceipt,
) -> Result<AnalysisInstanceId, String> {
    let Some(derived) = task.derived_from.as_ref() else {
        return Ok(task.source_instance_id);
    };
    let path =
        DerivedAnalysisIdentity::from_roles(derived.authored_instance_id, derived.roles.clone())
            .map_err(|error| format!("{prefix}.derived_from is not a derivation path: {error}"))?;
    if path.instance_id() != task.source_instance_id {
        return Err(format!(
            "{prefix}.source_instance_id {} is not the identity its derivation from analysis {} produces",
            task.source_instance_id, derived.authored_instance_id
        ));
    }
    Ok(derived.authored_instance_id)
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

/// A noise mechanism is checked for shape rather than membership.
///
/// It is not a static label: the summary keeps it as owned text on both sides
/// of the file, so nothing is interned and no vocabulary has to cover it. Nor
/// could one — half the mechanisms are composed by the Verilog-A code
/// generator out of a model's own node and label names, and no build can list
/// what a future catalog will contain. The engine owns the shape those names
/// keep, so it is asked.
fn require_noise_mechanism(value: &str, field: &str) -> Result<(), String> {
    if rspice_core::analysis::is_persistable_noise_mechanism(value) {
        Ok(())
    } else {
        Err(format!(
            "{field} is not a noise mechanism: '{}'",
            value.chars().take(64).collect::<String>()
        ))
    }
}

/// Intern a persisted label for restoration.
///
/// Restoration is only ever reached through a document that already validated,
/// and validation rejects a label this predicate cannot resolve, so the
/// placeholder is what makes the function total rather than a decision to
/// accept unrecognized text.
fn intern_static_label(value: String) -> &'static str {
    known_static_label(&value).unwrap_or("unknown")
}

/// Labels a persisted result may carry, interned back to the `&'static str`
/// the running build uses.
///
/// The vocabulary belongs to the engine, which is the only thing that knows
/// what its device families report; asking it keeps the reader in step with
/// the emitter instead of restating its list here.
fn known_static_label(value: &str) -> Option<&'static str> {
    rspice_core::circuit::resolve_op_label(value)
}

#[derive(Debug, Clone)]
pub enum ProjectIoError {
    #[cfg(not(target_arch = "wasm32"))]
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
            #[cfg(not(target_arch = "wasm32"))]
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
#[cfg(test)]
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

#[cfg(test)]
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
    project.workspace.migrate_owned_netlist_deck_ids();
    let mut load_repairs = Vec::new();
    load_repairs.extend(project.workspace.migrate_document_occurrences());
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
    load_repairs.extend(project.validate_schematic_instance_masters());
    project.workspace_migration_warning =
        (!load_repairs.is_empty()).then(|| load_repairs.join(" "));
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
    } else {
        project.pin_regression_baseline_references();
        if let Err(error) = project.validate_regression_baseline_eligibility() {
            let cleared = project.clear_regression_baseline_references();
            project.simulation_results_warning = Some(format!(
                "Retained simulation results were restored, but {cleared} regression baseline reference(s) were cleared because the persisted selection is not eligible: {error}"
            ));
        }
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
mod tests;
