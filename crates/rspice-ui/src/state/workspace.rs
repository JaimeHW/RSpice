//! Project workspace state.
//!
//! This module is the product-level design spine for RSpice Studio. It keeps
//! project identity, open Library/Cell/View documents, active hierarchy
//! breadcrumbs, and per-view schematic buffers together instead of letting the
//! workbench, library browser, and single schematic buffer drift apart.

use std::collections::{BTreeMap, HashMap, HashSet};
use std::path::{Path, PathBuf};

mod design_intent;
mod plan_data;
mod project_descriptor;
mod hierarchy;
mod materialize;
mod open_documents;
mod saved_output;

pub(crate) use saved_output::validate_raw_probe;
pub use design_intent::*;
pub use project_descriptor::*;
pub use hierarchy::*;
pub use materialize::*;

pub use saved_output::{
    SavedOutput, SavedOutputCompatibility, SavedOutputKind, SavedOutputPolicy,
    SavedOutputPrecision, SavedOutputStreaming,
};
use saved_output::{
    deserialize_or_migrate_identity, missing_identity_sentinel, parse_design_quantity,
    validate_bounded_text, validate_parameter_name, validate_single_line_expression,
};

use serde::{Deserialize, Deserializer, Serialize, de::Error as _};
use sha2::Digest as _;
use unicode_segmentation::UnicodeSegmentation;
use uuid::Uuid;

use crate::product::{
    AnalysisInstanceId, ContentDigest, DesignVariableId, ObjectRevision, ProjectId,
    ResultDocumentId, RevisionError, RunId, SavedOutputId, SimulationPlanId,
};
use crate::state::{
    AnalysisResultSourceDomain, Cell, ComponentType, Library, LibraryCellInstance, LibraryManager,
    SchematicState, View, ViewType,
};

/// Default editable design library created for new projects.
pub const DEFAULT_PROJECT_LIBRARY: &str = "user";
/// Default top-level cell created for new projects.
pub const DEFAULT_TOP_CELL: &str = "top";
/// Default schematic view name.
pub const DEFAULT_SCHEMATIC_VIEW: &str = "schematic";
/// Persisted schema for project identity metadata.
pub const PROJECT_DESCRIPTOR_SCHEMA_VERSION: u16 = 1;
/// Persisted schema for an exact project-owned technology binding.
pub const PROJECT_TECHNOLOGY_BINDING_SCHEMA_VERSION: u16 = 1;

/// Maximum legal hierarchy depth. This is deliberately generous for real
/// designs while placing a deterministic bound on corrupt or hostile project
/// data before it reaches netlisting.
const MAX_HIERARCHY_RESOLUTION_DEPTH: usize = 128;
/// Maximum number of expanded instances accepted by the configuration
/// resolver. The table remains grouped by master, but the receipt count is an
/// exact expanded-instance count up to this defensive product limit.
const MAX_HIERARCHY_RESOLUTION_INSTANCES: usize = 1_000_000;

/// Versioned identity domain for legacy session descriptors that predate a
/// persisted [`ProjectId`]. Project-file migration derives its ID from the
/// complete source bytes before deserialization; this namespace is reserved
/// for standalone/session descriptor migration.
const LEGACY_PROJECT_DESCRIPTOR_ID_NAMESPACE: Uuid =
    Uuid::from_u128(0xd59a_680f_c781_5f1a_a69f_9a67_64bb_32ac);

/// Validate one persisted library, cell, or view name.
///
/// The slash-delimited workspace key format is unambiguous only while every
/// segment follows the same contract enforced by the library dialogs: a
/// non-empty sequence of Unicode letters/numbers and underscores. Persisted
/// data is validated against this boundary before any generated key is used.
pub fn validate_cell_view_name_segment(value: &str) -> Result<(), CellViewNameError> {
    if value.is_empty() {
        return Err(CellViewNameError::Empty);
    }
    if let Some(character) = value
        .chars()
        .find(|character| !character.is_alphanumeric() && *character != '_')
    {
        return Err(CellViewNameError::UnsupportedCharacter(character));
    }
    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum CellViewNameError {
    #[error("must not be empty")]
    Empty,
    #[error(
        "contains unsupported character {0:?}; only letters, numbers, and underscores are allowed"
    )]
    UnsupportedCharacter(char),
}

fn default_project_name() -> String {
    "Untitled Project".to_owned()
}

/// Presence-aware project identity used only while decoding persisted data.
///
/// `Option<T>` intentionally maps both a missing field (through `default`) and
/// an explicit JSON `null` to `None`. Those states have different security
/// semantics for project identity: only a genuinely missing field from an
/// unversioned legacy descriptor may be migrated.
#[derive(Debug, Default)]
enum DeserializedProjectId {
    #[default]
    Missing,
    Null,
    Value(ProjectId),
}

#[derive(Debug, Default)]
enum DeserializedProjectSchemaVersion {
    #[default]
    Missing,
    Null,
    Value(u16),
}

impl<'de> Deserialize<'de> for DeserializedProjectSchemaVersion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = serde_json::Value::deserialize(deserializer)?;
        if value.is_null() {
            Ok(Self::Null)
        } else {
            serde_json::from_value(value)
                .map(Self::Value)
                .map_err(D::Error::custom)
        }
    }
}

impl<'de> Deserialize<'de> for DeserializedProjectId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = serde_json::Value::deserialize(deserializer)?;
        if value.is_null() {
            Ok(Self::Null)
        } else {
            serde_json::from_value(value)
                .map(Self::Value)
                .map_err(D::Error::custom)
        }
    }
}

/// A stable reference to one Library/Cell/View document.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CellViewRef {
    pub library: String,
    pub cell: String,
    pub view: String,
}

impl CellViewRef {
    pub fn new(
        library: impl Into<String>,
        cell: impl Into<String>,
        view: impl Into<String>,
    ) -> Self {
        Self {
            library: library.into(),
            cell: cell.into(),
            view: view.into(),
        }
    }

    pub fn default_top() -> Self {
        Self::new(
            DEFAULT_PROJECT_LIBRARY,
            DEFAULT_TOP_CELL,
            DEFAULT_SCHEMATIC_VIEW,
        )
    }

    pub fn key(&self) -> String {
        format!("{}/{}/{}", self.library, self.cell, self.view)
    }

    /// Validate every segment before this reference participates in a
    /// persisted slash-delimited key.
    pub fn validate_name_segments(&self) -> Result<(), CellViewNameError> {
        validate_cell_view_name_segment(&self.library)?;
        validate_cell_view_name_segment(&self.cell)?;
        validate_cell_view_name_segment(&self.view)
    }

    pub fn display_path(&self) -> String {
        self.key()
    }
}


/// One immutable, exact-path executable binding consumed by hierarchical
/// netlist generation.  The placed schematic binding is deliberately not
/// retained as execution authority: `materialized_binding` is rebuilt from
/// the resolved Library/Cell/View and its authoritative view metadata.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfigurationExecutionBinding {
    instance_path: String,
    resolved_reference: CellViewRef,
    resolved_view_type: ViewType,
    materialized_binding: Option<LibraryCellInstance>,
    model_section: Option<String>,
    stop_boundary: bool,
    project_veriloga: Option<ConfigurationVerilogABinding>,
}

/// Exact project-owned behavioral source selected for one configuration
/// binding. This is derived from the active configuration and source registry;
/// it is never accepted from placed-instance or filesystem metadata.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfigurationVerilogABinding {
    source_bundle_id: ProjectSourceId,
    source_closure_digest: ContentDigest,
    selected_module: String,
    source_key: String,
    netlist_alias: String,
}

impl ConfigurationVerilogABinding {
    pub const fn source_bundle_id(&self) -> ProjectSourceId {
        self.source_bundle_id
    }

    pub const fn source_closure_digest(&self) -> ContentDigest {
        self.source_closure_digest
    }

    pub fn selected_module(&self) -> &str {
        &self.selected_module
    }

    pub fn source_key(&self) -> &str {
        &self.source_key
    }

    pub fn netlist_alias(&self) -> &str {
        &self.netlist_alias
    }
}

impl ConfigurationExecutionBinding {
    pub fn instance_path(&self) -> &str {
        &self.instance_path
    }

    pub const fn resolved_reference(&self) -> &CellViewRef {
        &self.resolved_reference
    }

    pub const fn resolved_view_type(&self) -> ViewType {
        self.resolved_view_type
    }

    pub const fn materialized_binding(&self) -> Option<&LibraryCellInstance> {
        self.materialized_binding.as_ref()
    }

    pub fn model_section(&self) -> Option<&str> {
        self.model_section.as_deref()
    }

    pub const fn stop_boundary(&self) -> bool {
        self.stop_boundary
    }

    pub const fn project_veriloga(&self) -> Option<&ConfigurationVerilogABinding> {
        self.project_veriloga.as_ref()
    }
}

/// Frozen per-instance hierarchy authority for one active configuration-set
/// revision. Keys are canonicalized exact instance paths; values retain the
/// display spelling for receipts and diagnostics.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfigurationExecutionPlan {
    root: CellViewRef,
    bindings: BTreeMap<String, ConfigurationExecutionBinding>,
    configuration_id: crate::state::ConfigurationSetId,
    configuration_revision: u64,
    configuration_digest: ContentDigest,
}

impl ConfigurationExecutionPlan {
    pub const fn root(&self) -> &CellViewRef {
        &self.root
    }

    pub fn binding(&self, instance_path: &str) -> Option<&ConfigurationExecutionBinding> {
        self.bindings.get(&instance_path.to_ascii_lowercase())
    }

    pub fn bindings(&self) -> impl ExactSizeIterator<Item = &ConfigurationExecutionBinding> {
        self.bindings.values()
    }

    pub const fn configuration_id(&self) -> crate::state::ConfigurationSetId {
        self.configuration_id
    }

    pub const fn configuration_revision(&self) -> u64 {
        self.configuration_revision
    }

    pub const fn configuration_digest(&self) -> ContentDigest {
        self.configuration_digest
    }
}

/// Owned live-buffer projection paired with its frozen configuration plan.
/// Holding both in one value prevents a caller from resolving one hierarchy
/// and accidentally netlisting a different editor buffer.
#[derive(Debug, Clone)]
pub struct ConfigurationExecutionProjection {
    root: CellViewRef,
    schematic_buffers: HashMap<String, SchematicState>,
    plan: Option<ConfigurationExecutionPlan>,
    connectivity: crate::state::ConnectivityContract,
}

impl ConfigurationExecutionProjection {
    pub const fn root(&self) -> &CellViewRef {
        &self.root
    }

    pub fn root_schematic(&self) -> Option<&SchematicState> {
        self.schematic_buffers
            .iter()
            .find(|(key, _)| key.eq_ignore_ascii_case(&self.root.key()))
            .map(|(_, schematic)| schematic)
    }

    pub const fn schematic_buffers(&self) -> &HashMap<String, SchematicState> {
        &self.schematic_buffers
    }

    pub const fn plan(&self) -> Option<&ConfigurationExecutionPlan> {
        self.plan.as_ref()
    }

    pub const fn connectivity(&self) -> &crate::state::ConnectivityContract {
        &self.connectivity
    }
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ConfigurationExecutionPlanError {
    #[error("configuration hierarchy is unresolved: {0}")]
    Unresolved(String),
    #[error("configuration root {0} has no materialized schematic buffer")]
    MissingRoot(String),
    #[error("design-management projection is invalid: {0}")]
    DesignManagement(String),
}


/// One open view tab in the workspace.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenCellView {
    pub reference: CellViewRef,
    pub view_type: ViewType,
    pub dirty: bool,
}

impl OpenCellView {
    pub fn new(reference: CellViewRef, view_type: ViewType) -> Self {
        Self {
            reference,
            view_type,
            dirty: false,
        }
    }
}

fn is_schematic_like(view_type: ViewType) -> bool {
    matches!(view_type, ViewType::Schematic | ViewType::Testbench)
}

fn library_view_type(libraries: &LibraryManager, reference: &CellViewRef) -> Option<ViewType> {
    libraries
        .get_library(&reference.library)
        .and_then(|library| library.get_cell(&reference.cell))
        .and_then(|cell| cell.get_view(&reference.view))
        .map(|view| view.view_type)
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum SimulationConfigurationError {
    #[error("project configuration-set catalog is invalid: {message}")]
    InvalidConfigurationSetCatalog { message: String },
    #[error("project design-management catalog is invalid: {message}")]
    InvalidDesignManagementCatalog { message: String },
    #[error("project connectivity contract is invalid: {message}")]
    InvalidConnectivityContract { message: String },
    #[error("project-owned netlist document is invalid: {message}")]
    InvalidNetlistDocumentProjection { message: String },
    #[error("project-owned Code source registry is invalid: {message}")]
    InvalidProjectSourceRegistry { message: String },
    #[error("project plot export preset catalog has invalid ownership: {message}")]
    InvalidPlotExportPresetOwnership { message: String },
    #[error("project hardcopy source-set catalog is invalid: {message}")]
    InvalidHardcopySourceSetCatalog { message: String },
    #[error("report_documents[{index}] is invalid: {message}")]
    InvalidReportDocument { index: usize, message: String },
    #[error("report document identity {document_id} is duplicated")]
    DuplicateReportDocumentIdentity { document_id: ResultDocumentId },
    #[error("simulation_plan_payloads contains duplicate owner {plan_id}")]
    DuplicatePlanPayload { plan_id: SimulationPlanId },
    #[error("simulation_plan_payloads[{plan_id}].design_variables[{index}] is invalid: {message}")]
    InvalidDesignVariable {
        plan_id: SimulationPlanId,
        index: usize,
        message: String,
    },
    #[error(
        "simulation_plan_payloads[{plan_id}].design_variables[{index}] duplicates the case-insensitive name of design_variables[{first_index}]"
    )]
    DuplicateDesignVariableName {
        plan_id: SimulationPlanId,
        index: usize,
        first_index: usize,
    },
    #[error("design variable identity {id} is reused by plans {first_plan_id} and {plan_id}")]
    DuplicateDesignVariableIdentity {
        id: DesignVariableId,
        first_plan_id: SimulationPlanId,
        plan_id: SimulationPlanId,
    },
    #[error("simulation_plan_payloads[{plan_id}].saved_outputs[{index}] is invalid: {message}")]
    InvalidSavedOutput {
        plan_id: SimulationPlanId,
        index: usize,
        message: String,
    },
    #[error(
        "simulation_plan_payloads[{plan_id}].saved_outputs[{index}] duplicates the case-insensitive name of saved_outputs[{first_index}]"
    )]
    DuplicateSavedOutputName {
        plan_id: SimulationPlanId,
        index: usize,
        first_index: usize,
    },
    #[error("saved output identity {id} is reused by plans {first_plan_id} and {plan_id}")]
    DuplicateSavedOutputIdentity {
        id: SavedOutputId,
        first_plan_id: SimulationPlanId,
        plan_id: SimulationPlanId,
    },
    #[error("simulation_plan_payloads[{plan_id}].specs[{index}] is invalid: {message}")]
    InvalidSpecification {
        plan_id: SimulationPlanId,
        index: usize,
        message: String,
    },
    #[error(
        "simulation_plan_payloads[{plan_id}].specs[{index}] duplicates the case-insensitive measurement of specs[{first_index}]"
    )]
    DuplicateSpecification {
        plan_id: SimulationPlanId,
        index: usize,
        first_index: usize,
    },
    #[error(
        "simulation_plan_payloads[{plan_id}].regression_tolerances[{index}] is invalid: {message}"
    )]
    InvalidRegressionTolerance {
        plan_id: SimulationPlanId,
        index: usize,
        message: String,
    },
    #[error(
        "simulation_plan_payloads[{plan_id}].regression_tolerances[{index}] duplicates target owned by entry {first_index}"
    )]
    DuplicateRegressionTolerance {
        plan_id: SimulationPlanId,
        index: usize,
        first_index: usize,
    },
    #[error("simulation plan {plan_id} already owns a design variable named '{name}'")]
    DesignVariableNameConflict {
        plan_id: SimulationPlanId,
        name: String,
    },
    #[error("simulation plan {plan_id} has no design variable with identity {variable_id}")]
    DesignVariableNotFound {
        plan_id: SimulationPlanId,
        variable_id: DesignVariableId,
    },
    #[error(
        "design variable {variable_id} in simulation plan {plan_id} could not advance its revision: {source}"
    )]
    DesignVariableRevision {
        plan_id: SimulationPlanId,
        variable_id: DesignVariableId,
        #[source]
        source: RevisionError,
    },
    #[error(
        "design variable {variable_id} is repeated in one update transaction for simulation plan {plan_id}"
    )]
    DuplicateDesignVariableUpdate {
        plan_id: SimulationPlanId,
        variable_id: DesignVariableId,
    },
    #[error("simulation plan {plan_id} already owns a saved output named '{name}'")]
    SavedOutputNameConflict {
        plan_id: SimulationPlanId,
        name: String,
    },
    #[error("simulation plan {plan_id} has no configuration payload")]
    PlanPayloadMissing { plan_id: SimulationPlanId },
    #[error("simulation plan {plan_id} already has a configuration payload")]
    PlanPayloadAlreadyExists { plan_id: SimulationPlanId },
    #[error("cloned plan payload has no destination mapping for source analysis {analysis_id}")]
    MissingClonedAnalysisMapping { analysis_id: AnalysisInstanceId },
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ProjectConfigurationMutationError {
    #[error("configuration-set catalog is invalid: {0}")]
    InvalidCatalog(#[from] crate::state::ConfigurationSetError),
    #[error("design-management catalog is invalid: {message}")]
    InvalidDesignManagementCatalog { message: String },
    #[error("configuration '{configuration}' root {root} is not a schematic or testbench view")]
    UnsupportedRootView { configuration: String, root: String },
    #[error("configuration '{configuration}' root {root} has no authoritative schematic buffer")]
    MissingRootBuffer { configuration: String, root: String },
    #[error("project revision could not advance: {0}")]
    ProjectRevision(#[from] RevisionError),
    #[error("configuration-set transaction has no semantic changes")]
    NoChanges,
}

pub const MAX_PROJECT_HARDCOPY_SOURCE_SETS: usize = 64;

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum HardcopySourceSetPersistenceError {
    #[error("hardcopy source set is invalid: {message}")]
    Invalid { message: String },
    #[error(
        "project hardcopy source-set catalog is full ({MAX_PROJECT_HARDCOPY_SOURCE_SETS} sets)"
    )]
    CatalogFull,
    #[error("hardcopy source-set name '{name}' is already owned by another retained set")]
    DuplicateName { name: String },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RegressionComparisonMethod {
    AbsoluteRelativeEnvelope,
    PointwiseRelative,
}

impl RegressionComparisonMethod {
    pub const ALL: [Self; 2] = [Self::AbsoluteRelativeEnvelope, Self::PointwiseRelative];

    pub const fn label(self) -> &'static str {
        match self {
            Self::AbsoluteRelativeEnvelope => "Absolute + relative envelope",
            Self::PointwiseRelative => "Pointwise relative",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RegressionTargetKind {
    Measurement,
    Waveform,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RegressionTargetSelector {
    pub source_domain: AnalysisResultSourceDomain,
    pub source_instance_id: AnalysisInstanceId,
    pub kind: RegressionTargetKind,
    pub name: String,
    pub occurrence: u32,
}

impl RegressionTargetSelector {
    fn validate(&self) -> Result<(), String> {
        if self.source_domain == AnalysisResultSourceDomain::LegacyUnclassified {
            return Err(
                "legacy-unclassified result sources cannot own regression policy".to_owned(),
            );
        }
        if self.name.trim().is_empty() {
            return Err("target name must not be empty".to_owned());
        }
        if self.name != self.name.trim() {
            return Err("target name must not have surrounding whitespace".to_owned());
        }
        if self.name.chars().any(char::is_control) {
            return Err("target name must not contain control characters".to_owned());
        }
        if self.name.graphemes(true).count() > 256 {
            return Err("target name exceeds 256 grapheme clusters".to_owned());
        }
        Ok(())
    }

    fn cloned_for_new_plan(
        &self,
        analysis_identity_map: &HashMap<AnalysisInstanceId, AnalysisInstanceId>,
    ) -> Result<Self, AnalysisInstanceId> {
        let mut cloned = self.clone();
        if self.source_domain == AnalysisResultSourceDomain::SimulationPlan {
            cloned.source_instance_id = analysis_identity_map
                .get(&self.source_instance_id)
                .copied()
                .ok_or(self.source_instance_id)?;
        }
        Ok(cloned)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RegressionComparisonWindow {
    pub start: f64,
    pub end: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RegressionToleranceRule {
    pub target: RegressionTargetSelector,
    pub method: RegressionComparisonMethod,
    /// Absolute value-domain tolerance in the target's retained base unit.
    pub absolute_tolerance: f64,
    /// Relative tolerance as a fraction (`0.005` = `0.5%`).
    pub relative_tolerance: f64,
    /// Maximum horizontal displacement in the waveform X-axis base unit.
    pub time_skew_allowance: f64,
    /// Optional inclusive X-axis comparison window. Measurements use `None`.
    pub comparison_window: Option<RegressionComparisonWindow>,
}

impl RegressionToleranceRule {
    pub fn validate(&self) -> Result<(), String> {
        self.target.validate()?;
        for (label, value) in [
            ("absolute tolerance", self.absolute_tolerance),
            ("relative tolerance", self.relative_tolerance),
            ("time-skew allowance", self.time_skew_allowance),
        ] {
            if !value.is_finite() || value < 0.0 {
                return Err(format!("{label} must be finite and nonnegative"));
            }
        }
        if self.target.kind == RegressionTargetKind::Measurement
            && (self.time_skew_allowance != 0.0 || self.comparison_window.is_some())
        {
            return Err(
                "measurement targets cannot define time skew or a comparison window".to_owned(),
            );
        }
        if let Some(window) = self.comparison_window {
            if !window.start.is_finite() || !window.end.is_finite() {
                return Err("comparison-window bounds must be finite".to_owned());
            }
            if window.start > window.end {
                return Err("comparison-window start must not exceed its end".to_owned());
            }
        }
        Ok(())
    }

    fn cloned_for_new_plan(
        &self,
        analysis_identity_map: &HashMap<AnalysisInstanceId, AnalysisInstanceId>,
    ) -> Result<Self, AnalysisInstanceId> {
        let mut cloned = self.clone();
        cloned.target = self.target.cloned_for_new_plan(analysis_identity_map)?;
        Ok(cloned)
    }
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SimulationPlanPayload {
    #[serde(default)]
    pub design_variables: Vec<DesignVariable>,
    #[serde(default)]
    pub saved_outputs: Vec<SavedOutput>,
    #[serde(default)]
    pub specs: Vec<SpecEntry>,
    #[serde(default)]
    pub regression_baseline_run: Option<RunId>,
    #[serde(default)]
    pub regression_tolerances: Vec<RegressionToleranceRule>,
}

/// Vec-backed because product UUID wrappers intentionally do not define an
/// ordering. Validation guarantees unique owners; lifecycle hashing sorts a
/// canonical projection by UUID bytes.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SimulationPlanPayloadRecord {
    pub plan_id: SimulationPlanId,
    pub payload: SimulationPlanPayload,
}

/// Mockup-specified ownership strategy for a project-owned SPICE artifact.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub enum OwnedNetlistEditStrategy {
    #[default]
    OwnedSource,
    ParameterOptionOverride,
    IncludeOrderOverride,
    AnalysisOnlyDeck,
}

impl OwnedNetlistEditStrategy {
    pub const ALL: [Self; 4] = [
        Self::OwnedSource,
        Self::ParameterOptionOverride,
        Self::IncludeOrderOverride,
        Self::AnalysisOnlyDeck,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::OwnedSource => "Owned source derived from generated output",
            Self::ParameterOptionOverride => "Parameter and option override",
            Self::IncludeOrderOverride => "Include-order override",
            Self::AnalysisOnlyDeck => "Analysis-only deck",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OwnedNetlistSaveRecord {
    pub document_revision: u64,
    pub content_digest: crate::product::ContentDigest,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OwnedNetlistDescriptor {
    pub artifact_name: String,
    pub strategy: OwnedNetlistEditStrategy,
    #[serde(default)]
    pub save_history: Vec<OwnedNetlistSaveRecord>,
}

// The source bundle API, re-exported from its historical workspace path. This
// block used to carry the whole `project_sources` surface "so downstream
// integrations keep compiling" -- there are no downstream integrations; the
// crate is the application. What is left is what `state::workspace` callers
// actually name through this path.
pub use super::project_sources::{
    MAX_PROJECT_CODE_SOURCE_BYTES, ProjectSourceBundle, ProjectSourceDocument, ProjectSourceError,
    ProjectSourceId, ProjectSourceLanguage, ProjectSourceOwner, ProjectSourceRegistry,
    ProjectSourceValidationIdentity,
};

/// Project-level workspace state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectWorkspace {
    pub project: ProjectDescriptor,
    /// Project-owned hierarchy/view-resolution authority. Empty catalogs
    /// preserve legacy deterministic resolution; once populated, the active
    /// configuration is the exact authority used by preflight and netlisting.
    #[serde(default)]
    pub configuration_sets: crate::state::ConfigurationSetCatalog,
    /// Project-owned schematic sheet, assembly variant, annotation, and
    /// hierarchy-audit authority. The catalog is deliberately separate from
    /// simulation configuration sets: it describes design identity, while a
    /// configuration set describes how that identity is executed.
    #[serde(default)]
    pub design_management: crate::state::DesignManagementCatalog,
    /// Project-owned bundle mapping and global-net policy. Older projects
    /// migrate to strict fail-closed defaults instead of inheriting UI state.
    #[serde(default)]
    pub connectivity: crate::state::ConnectivityContract,
    pub active_view: CellViewRef,
    pub open_views: Vec<OpenCellView>,
    pub hierarchy_stack: Vec<CellViewRef>,
    /// Instance names descended through, aligned with
    /// `hierarchy_stack[1..]`: entry N-1 is the instance whose master is
    /// `hierarchy_stack[N]`. Older saves default to empty; rendering
    /// falls back to cell names per entry.
    #[serde(default)]
    pub hierarchy_instances: Vec<String>,
    pub schematic_buffers: HashMap<String, SchematicState>,
    /// Measurement specifications for the results specs matrix. Project
    /// design intent, so it persists with the workspace.
    #[serde(default)]
    pub specs: Vec<SpecEntry>,
    /// Plan-owned variables, output contracts, and specifications. Projects
    /// predating this feature migrate the active legacy `specs` projection
    /// into one record after execution-context migration.
    #[serde(default)]
    pub simulation_plan_payloads: Vec<SimulationPlanPayloadRecord>,
    /// Project-owned, versioned publication profiles for result plots.
    /// Personal profiles are owned by serialized user preferences; an
    /// organization profile requires a connected organization authority.
    #[serde(default)]
    pub plot_export_presets: crate::results::plot_export_preset::PlotExportPresetCatalog,
    /// Versioned, per-document Page Setup contracts used by schematic,
    /// symbol, result, and report hardcopy workflows. Publication artifacts
    /// and transient preview state are intentionally not persisted here.
    #[serde(default)]
    pub hardcopy_setups: crate::hardcopy::HardcopySetupStore,
    /// Reusable print-mapping sets owned by this project. Personal portable
    /// presets are persisted by `UserPreferences`; document mappings remain
    /// embedded in `hardcopy_setups` for reproducible publication.
    #[serde(default)]
    pub project_print_mappings: crate::hardcopy::PrintMappingPresetCatalog,
    /// Project-owned named engineering-table views. Working and personal
    /// views are device preferences; only explicitly project-scoped views
    /// participate in project revisioning and collaboration.
    #[serde(default)]
    pub engineering_table_views: crate::state::EngineeringTableViewStore,
    /// Ordered, exact source aggregates used by all-sheets/all-panes and
    /// named print-set publication. Every member pins its document revision
    /// and content digest; stale members fail closed when resolved.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    hardcopy_source_sets: Vec<crate::hardcopy::sources::HardcopySourceSet>,
    /// Project-owned, versioned engineering report sources. Rendered review
    /// artifacts are derived from these documents and are never represented
    /// here unless a publication writer has produced and verified them.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub report_documents: Vec<crate::results::report_document::ReportDocument>,
    /// Manually edited netlist source. When set, simulations run this
    /// deck instead of regenerating from the schematic (text-first mode);
    /// `None` means the netlist view shows the generated artifact.
    #[serde(default)]
    pub netlist_source: Option<String>,
    /// Canonical owned-source identity, provenance, generated base, sealed
    /// dependency metadata, revision, and validation evidence. The legacy
    /// `netlist_source` projection remains for backwards compatibility and
    /// must exactly match this document when both are present.
    #[serde(default)]
    pub netlist_document: Option<crate::state::NetlistDocument>,
    /// Ownership-dialog selection for the project-owned source artifact.
    #[serde(default)]
    pub netlist_descriptor: Option<OwnedNetlistDescriptor>,
    /// Project-owned source documents shown by the Verilog-A and Automation
    /// pages of the Code workspace. Older projects intentionally restore an
    /// empty registry rather than receiving demonstration content.
    #[serde(default, skip_serializing_if = "ProjectSourceRegistry::is_empty")]
    pub project_sources: ProjectSourceRegistry,
    /// Native filesystem origin for `netlist_source`, used to resolve relative
    /// `.include`/`.lib` paths for imported decks. Edits retain this origin:
    /// changing document bytes does not change the directory against which its
    /// authored relative dependencies resolve. Browser imports have no native
    /// path authority and therefore leave it absent.
    #[serde(default)]
    pub netlist_source_path: Option<PathBuf>,
    /// Runtime dirty bit for `netlist_source`; skipped because dirty state is
    /// session-local while the source itself is persisted with the project.
    #[serde(default, skip)]
    pub netlist_source_dirty: bool,
    /// Runtime dirty state for `project_sources`. Source bytes and validation
    /// identities persist; dirty state is derived against the accepted project
    /// and remains session-local.
    #[serde(default, skip)]
    pub project_sources_dirty: bool,
    /// Runtime dirty state for project-owned metadata such as an exact
    /// technology attachment. The binding itself persists in `project`.
    #[serde(default, skip)]
    #[doc(hidden)]
    pub project_metadata_dirty: bool,
    /// Runtime dirty projection for project-owned report sources. The report
    /// documents themselves persist; accepted-baseline comparison remains the
    /// canonical save/revert authority.
    #[serde(default, skip)]
    pub report_documents_dirty: bool,
    /// Runtime dirty projection for committed per-document page setups.
    #[serde(default, skip)]
    pub hardcopy_setups_dirty: bool,
    /// Runtime dirty projection for reusable project-owned print mappings.
    #[serde(default, skip)]
    pub project_print_mappings_dirty: bool,
    /// Runtime dirty projection for project-owned hardcopy source sets.
    #[serde(default, skip)]
    hardcopy_source_sets_dirty: bool,
}

impl Default for ProjectWorkspace {
    fn default() -> Self {
        let active_view = CellViewRef::default_top();
        let mut schematic_buffers = HashMap::new();
        schematic_buffers.insert(active_view.key(), SchematicState::default());

        Self {
            project: ProjectDescriptor::default(),
            configuration_sets: crate::state::ConfigurationSetCatalog::default(),
            design_management: crate::state::DesignManagementCatalog::default(),
            connectivity: crate::state::ConnectivityContract::default(),
            active_view: active_view.clone(),
            open_views: vec![OpenCellView::new(active_view.clone(), ViewType::Schematic)],
            hierarchy_stack: vec![active_view],
            hierarchy_instances: Vec::new(),
            schematic_buffers,
            specs: Vec::new(),
            simulation_plan_payloads: Vec::new(),
            plot_export_presets:
                crate::results::plot_export_preset::PlotExportPresetCatalog::default(),
            hardcopy_setups: crate::hardcopy::HardcopySetupStore::default(),
            project_print_mappings: crate::hardcopy::PrintMappingPresetCatalog::new(
                crate::hardcopy::PrintMappingCatalogOwner::Project,
            ),
            engineering_table_views: crate::state::EngineeringTableViewStore::default(),
            hardcopy_source_sets: Vec::new(),
            report_documents: Vec::new(),
            netlist_source: None,
            netlist_document: None,
            netlist_descriptor: None,
            project_sources: ProjectSourceRegistry::default(),
            netlist_source_path: None,
            netlist_source_dirty: false,
            project_sources_dirty: false,
            project_metadata_dirty: false,
            report_documents_dirty: false,
            hardcopy_setups_dirty: false,
            project_print_mappings_dirty: false,
            hardcopy_source_sets_dirty: false,
        }
    }
}

fn validate_hardcopy_source_set_catalog(
    source_sets: &[crate::hardcopy::sources::HardcopySourceSet],
) -> Result<(), HardcopySourceSetPersistenceError> {
    if source_sets.len() > MAX_PROJECT_HARDCOPY_SOURCE_SETS {
        return Err(HardcopySourceSetPersistenceError::CatalogFull);
    }
    let mut source_keys = std::collections::HashSet::with_capacity(source_sets.len());
    let mut folded_names = std::collections::HashSet::with_capacity(source_sets.len());
    for source_set in source_sets {
        source_set
            .validate()
            .map_err(|error| HardcopySourceSetPersistenceError::Invalid {
                message: error.to_string(),
            })?;
        if !source_keys.insert(source_set.source_key()) {
            return Err(HardcopySourceSetPersistenceError::Invalid {
                message: format!("source identity {} is duplicated", source_set.source_key()),
            });
        }
        if !folded_names.insert(source_set.name().to_lowercase()) {
            return Err(HardcopySourceSetPersistenceError::DuplicateName {
                name: source_set.name().to_owned(),
            });
        }
    }
    Ok(())
}

fn validate_connectivity_contract_references(
    workspace: &ProjectWorkspace,
) -> Result<(), SimulationConfigurationError> {
    let mut nets_by_view = HashMap::<&str, HashSet<String>>::new();
    for (view_key, schematic) in &workspace.schematic_buffers {
        nets_by_view.insert(
            view_key.as_str(),
            crate::simulation::netlist_gen::design_nets(schematic)
                .into_iter()
                .map(|net| net.name)
                .collect(),
        );
    }
    for (bundle_index, bundle) in workspace.connectivity.named_bundles.iter().enumerate() {
        for (member_index, member) in bundle.members.iter().enumerate() {
            let Some(nets) = nets_by_view.get(member.target.view_key.as_str()) else {
                return Err(SimulationConfigurationError::InvalidConnectivityContract {
                    message: format!(
                        "named_bundles[{bundle_index}].members[{member_index}] references missing schematic view '{}'",
                        member.target.view_key
                    ),
                });
            };
            if !nets.contains(&member.target.net_name) {
                return Err(SimulationConfigurationError::InvalidConnectivityContract {
                    message: format!(
                        "named_bundles[{bundle_index}].members[{member_index}] references missing exact net '{}::{}'",
                        member.target.view_key, member.target.net_name
                    ),
                });
            }
        }
    }
    Ok(())
}

impl ProjectWorkspace {
    /// Validate the persisted simulation configuration without requiring any
    /// runtime editor state. Cross-document targets are validated by project
    /// I/O once the library tree and simulation plan are available.
    pub fn validate_simulation_configuration(&self) -> Result<(), SimulationConfigurationError> {
        self.configuration_sets.validate().map_err(|error| {
            SimulationConfigurationError::InvalidConfigurationSetCatalog {
                message: error.to_string(),
            }
        })?;
        self.design_management.validate().map_err(|error| {
            SimulationConfigurationError::InvalidDesignManagementCatalog {
                message: error.to_string(),
            }
        })?;
        self.connectivity.validate().map_err(|message| {
            SimulationConfigurationError::InvalidConnectivityContract { message }
        })?;
        validate_connectivity_contract_references(self)?;
        self.plot_export_presets
            .validate_ownership_scope(
                crate::results::plot_export_preset::PlotExportPresetScope::Project,
            )
            .map_err(
                |error| SimulationConfigurationError::InvalidPlotExportPresetOwnership {
                    message: error.to_string(),
                },
            )?;
        validate_hardcopy_source_set_catalog(&self.hardcopy_source_sets).map_err(|error| {
            SimulationConfigurationError::InvalidHardcopySourceSetCatalog {
                message: error.to_string(),
            }
        })?;
        let mut report_document_ids = std::collections::HashSet::new();
        for (index, document) in self.report_documents.iter().enumerate() {
            document.validate().map_err(|error| {
                SimulationConfigurationError::InvalidReportDocument {
                    index,
                    message: error.to_string(),
                }
            })?;
            if !report_document_ids.insert(document.id()) {
                return Err(
                    SimulationConfigurationError::DuplicateReportDocumentIdentity {
                        document_id: document.id(),
                    },
                );
            }
        }
        self.project_sources.validate().map_err(|error| {
            SimulationConfigurationError::InvalidProjectSourceRegistry {
                message: error.to_string(),
            }
        })?;
        if let Some(document) = &self.netlist_document {
            if document.ownership()
                == crate::state::DocumentOwnership::Generated
            {
                return Err(
                    SimulationConfigurationError::InvalidNetlistDocumentProjection {
                        message: "project-owned netlist document cannot have generated ownership"
                            .to_owned(),
                    },
                );
            }
            if self.netlist_source.as_deref() != Some(document.source()) {
                return Err(
                    SimulationConfigurationError::InvalidNetlistDocumentProjection {
                        message: "canonical document bytes differ from netlist_source".to_owned(),
                    },
                );
            }
            let descriptor = self.netlist_descriptor.as_ref().ok_or_else(|| {
                SimulationConfigurationError::InvalidNetlistDocumentProjection {
                    message: "canonical document has no owned-artifact descriptor".to_owned(),
                }
            })?;
            let name = descriptor.artifact_name.trim();
            if name.is_empty()
                || name != descriptor.artifact_name
                || name.chars().any(char::is_control)
                || name.contains('/')
                || name.contains('\\')
            {
                return Err(
                    SimulationConfigurationError::InvalidNetlistDocumentProjection {
                        message: "owned artifact name must be one trimmed file name".to_owned(),
                    },
                );
            }
            let mut previous_revision = 0_u64;
            for record in &descriptor.save_history {
                if record.document_revision == 0
                    || record.document_revision <= previous_revision
                    || record.document_revision > document.revision().get()
                    || record.message.trim().is_empty()
                    || record.message != record.message.trim()
                    || record.message.chars().any(char::is_control)
                {
                    return Err(
                        SimulationConfigurationError::InvalidNetlistDocumentProjection {
                            message: "owned source save history is not strictly revision ordered or has an invalid message".to_owned(),
                        },
                    );
                }
                previous_revision = record.document_revision;
            }
        } else if self.netlist_descriptor.is_some() {
            return Err(
                SimulationConfigurationError::InvalidNetlistDocumentProjection {
                    message: "owned-artifact descriptor has no canonical document".to_owned(),
                },
            );
        }

        let mut plan_ids = HashMap::<SimulationPlanId, usize>::new();
        let mut variable_ids = HashMap::<DesignVariableId, SimulationPlanId>::new();
        let mut output_ids = HashMap::<SavedOutputId, SimulationPlanId>::new();
        for (record_index, record) in self.simulation_plan_payloads.iter().enumerate() {
            let plan_id = record.plan_id;
            if plan_ids.insert(plan_id, record_index).is_some() {
                return Err(SimulationConfigurationError::DuplicatePlanPayload { plan_id });
            }

            let mut variable_names = HashMap::<String, usize>::new();
            for (index, variable) in record.payload.design_variables.iter().enumerate() {
                variable.validate().map_err(|message| {
                    SimulationConfigurationError::InvalidDesignVariable {
                        plan_id,
                        index,
                        message,
                    }
                })?;
                if let Some(first_plan_id) = variable_ids.insert(variable.id, plan_id) {
                    return Err(
                        SimulationConfigurationError::DuplicateDesignVariableIdentity {
                            id: variable.id,
                            first_plan_id,
                            plan_id,
                        },
                    );
                }
                let canonical = variable.name.to_ascii_lowercase();
                if let Some(first_index) = variable_names.insert(canonical, index) {
                    return Err(SimulationConfigurationError::DuplicateDesignVariableName {
                        plan_id,
                        index,
                        first_index,
                    });
                }
            }

            let mut output_names = HashMap::<String, usize>::new();
            for (index, output) in record.payload.saved_outputs.iter().enumerate() {
                output.validate().map_err(|message| {
                    SimulationConfigurationError::InvalidSavedOutput {
                        plan_id,
                        index,
                        message,
                    }
                })?;
                if let Some(first_plan_id) = output_ids.insert(output.id, plan_id) {
                    return Err(SimulationConfigurationError::DuplicateSavedOutputIdentity {
                        id: output.id,
                        first_plan_id,
                        plan_id,
                    });
                }
                let canonical = output.name.to_lowercase();
                if let Some(first_index) = output_names.insert(canonical, index) {
                    return Err(SimulationConfigurationError::DuplicateSavedOutputName {
                        plan_id,
                        index,
                        first_index,
                    });
                }
            }

            let mut specification_names = HashMap::<String, usize>::new();
            for (index, specification) in record.payload.specs.iter().enumerate() {
                specification.validate().map_err(|message| {
                    SimulationConfigurationError::InvalidSpecification {
                        plan_id,
                        index,
                        message,
                    }
                })?;
                let canonical = specification.measurement.to_ascii_lowercase();
                if let Some(first_index) = specification_names.insert(canonical, index) {
                    return Err(SimulationConfigurationError::DuplicateSpecification {
                        plan_id,
                        index,
                        first_index,
                    });
                }
            }

            let mut regression_targets = Vec::<&RegressionTargetSelector>::new();
            for (index, tolerance) in record.payload.regression_tolerances.iter().enumerate() {
                tolerance.validate().map_err(|message| {
                    SimulationConfigurationError::InvalidRegressionTolerance {
                        plan_id,
                        index,
                        message,
                    }
                })?;
                if let Some(first_index) = regression_targets
                    .iter()
                    .position(|target| **target == tolerance.target)
                {
                    return Err(SimulationConfigurationError::DuplicateRegressionTolerance {
                        plan_id,
                        index,
                        first_index,
                    });
                }
                regression_targets.push(&tolerance.target);
            }
        }
        Ok(())
    }

}


#[cfg(test)]
mod tests;
