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
mod open_documents;
mod saved_output;

pub(crate) use saved_output::validate_raw_probe;
pub use design_intent::*;
pub use project_descriptor::*;
pub use hierarchy::*;

pub use saved_output::{
    SavedOutput, SavedOutputCompatibility, SavedOutputCompatibilityKind, SavedOutputKind,
    SavedOutputPolicy, SavedOutputPrecision, SavedOutputStreaming,
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

// Re-export the source bundle API from its historical workspace path so
// downstream integrations keep compiling while the implementation remains an
// independently testable state subsystem.
pub use super::project_sources::{
    MAX_PROJECT_CODE_SOURCE_BYTES, MAX_PROJECT_SOURCE_BUNDLE_BYTES,
    MAX_PROJECT_SOURCE_DEPENDENCIES, MAX_PROJECT_SOURCE_DEPENDENCY_DEPTH, MAX_PROJECT_SOURCE_FILES,
    MAX_PROJECT_SOURCE_LOGICAL_PATH_BYTES, PROJECT_SOURCE_REGISTRY_SCHEMA_VERSION,
    ProjectSourceBundle, ProjectSourceDependency, ProjectSourceDocument, ProjectSourceError,
    ProjectSourceFile, ProjectSourceId, ProjectSourceIdError, ProjectSourceIdParseError,
    ProjectSourceLanguage, ProjectSourceOwner, ProjectSourceRegistry,
    ProjectSourceValidationIdentity, project_veriloga_bundle_alias,
    project_veriloga_bundle_source_key,
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

fn is_project_virtual_source_path(path: &Path) -> bool {
    path.to_str()
        .is_some_and(|value| value.starts_with("__rspice_project__/"))
}

fn translated_point(
    point: crate::state::Point,
    delta: crate::state::Point,
) -> Result<crate::state::Point, crate::state::DesignManagementError> {
    Ok(crate::state::Point::new(
        point
            .x
            .checked_add(delta.x)
            .ok_or(crate::state::DesignManagementError::NumericRange(
                "materialized sheet x coordinate",
            ))?,
        point
            .y
            .checked_add(delta.y)
            .ok_or(crate::state::DesignManagementError::NumericRange(
                "materialized sheet y coordinate",
            ))?,
    ))
}

/// Resolve one typed cross-sheet endpoint against the authored topology and
/// then project it into the endpoint sheet's execution namespace. A wire
/// point must still lie on its retained conductor; a component terminal must
/// still own a canonical wire connection. Stale contracts fail before DRC or
/// netlisting rather than silently connecting a label to a component origin.
fn projected_cross_sheet_anchor(
    source: &SchematicState,
    projected: &SchematicState,
    endpoint: &crate::state::CrossSheetPortEndpoint,
    delta: crate::state::Point,
) -> Result<crate::state::Point, crate::state::DesignManagementError> {
    let authored_point = match &endpoint.anchor {
        crate::state::CrossSheetPortAnchor::WirePoint { wire_id, point } => {
            let wire = source
                .wires
                .iter()
                .find(|wire| wire.id == *wire_id)
                .ok_or_else(|| crate::state::DesignManagementError::MissingReference {
                    domain: "cross-sheet wire anchor",
                    identity: wire_id.to_string(),
                })?;
            if !wire.contains_point(*point) {
                return Err(crate::state::DesignManagementError::MissingReference {
                    domain: "cross-sheet wire anchor point",
                    identity: format!("{}@{},{}", wire_id, point.x, point.y),
                });
            }
            *point
        }
        crate::state::CrossSheetPortAnchor::ComponentTerminal {
            component_id,
            terminal_name,
        } => {
            if !source
                .components
                .iter()
                .any(|component| component.id == *component_id)
            {
                return Err(crate::state::DesignManagementError::MissingReference {
                    domain: "cross-sheet component anchor",
                    identity: component_id.to_string(),
                });
            }
            let connection = source
                .connections
                .iter()
                .find(|connection| {
                    connection.component_id == *component_id
                        && connection.terminal_name.eq_ignore_ascii_case(terminal_name)
                })
                .ok_or_else(|| crate::state::DesignManagementError::MissingReference {
                    domain: "cross-sheet component terminal connection",
                    identity: format!("{}:{}", component_id, terminal_name),
                })?;
            source
                .wires
                .iter()
                .find(|wire| wire.id == connection.wire_id)
                .and_then(|wire| wire.points.get(connection.point_index))
                .copied()
                .ok_or_else(|| crate::state::DesignManagementError::MissingReference {
                    domain: "cross-sheet component terminal wire point",
                    identity: format!("{}:{}", connection.wire_id, connection.point_index),
                })?
        }
    };
    let anchor = translated_point(authored_point, delta)?;
    match &endpoint.anchor {
        crate::state::CrossSheetPortAnchor::WirePoint { wire_id, .. } => {
            if !projected
                .wires
                .iter()
                .any(|wire| wire.id == *wire_id && wire.contains_point(anchor))
            {
                return Err(crate::state::DesignManagementError::MissingReference {
                    domain: "projected cross-sheet wire anchor",
                    identity: wire_id.to_string(),
                });
            }
        }
        crate::state::CrossSheetPortAnchor::ComponentTerminal { component_id, .. } => {
            if !projected
                .components
                .iter()
                .any(|component| component.id == *component_id)
            {
                return Err(crate::state::DesignManagementError::MissingReference {
                    domain: "projected cross-sheet component anchor",
                    identity: component_id.to_string(),
                });
            }
        }
    }
    Ok(anchor)
}

fn materialize_schematic_binding(
    placed: &LibraryCellInstance,
    reference: &CellViewRef,
    schematic: &SchematicState,
) -> Result<LibraryCellInstance, String> {
    let ports = schematic.interface_ports();
    let authoritative = ports
        .iter()
        .map(|port| port.name.as_str())
        .collect::<Vec<_>>();
    if !placed.terminal_order.is_empty()
        && !same_terminal_contract(&placed.terminal_order, &authoritative)
    {
        return Err(format!(
            "placed interface for {}/{} is incompatible with authoritative schematic view '{}'",
            placed.library, placed.cell, reference.view
        ));
    }
    let mut materialized =
        LibraryCellInstance::new(&reference.library, &reference.cell, &reference.view);
    materialized.bind_interface(&ports);
    Ok(materialized)
}

fn materialize_authoritative_source_binding(
    placed: &LibraryCellInstance,
    library: &Library,
    cell: &Cell,
    view: &View,
    workspace: &ProjectWorkspace,
    libraries: &LibraryManager,
) -> Result<LibraryCellInstance, String> {
    let reference = CellViewRef::new(&library.name, &cell.name, &view.name);
    let project_veriloga = (view.view_type == ViewType::VerilogA)
        .then(|| project_veriloga_binding_for_view(workspace, libraries, &reference))
        .transpose()?;
    let source_path = if let Some(binding) = project_veriloga.as_ref() {
        PathBuf::from(binding.source_key())
    } else {
        view.file_path
            .clone()
            .or_else(|| metadata_source_path(&view.metadata).map(Path::to_path_buf))
            .or_else(|| metadata_source_path(&cell.metadata).map(Path::to_path_buf))
            .ok_or_else(|| {
                format!(
                    "authoritative source view {}/{}/{} has no source identity",
                    library.name, cell.name, view.name
                )
            })?
    };
    let terminal_order = metadata_terminal_names(&view.metadata)
        .or_else(|| metadata_terminal_names(&cell.metadata))
        .ok_or_else(|| {
            format!(
                "authoritative source view {}/{}/{} has no terminal contract",
                library.name, cell.name, view.name
            )
        })?;
    if !placed.terminal_order.is_empty()
        && !same_terminal_contract(
            &placed.terminal_order,
            &terminal_order
                .iter()
                .map(String::as_str)
                .collect::<Vec<_>>(),
        )
    {
        return Err(format!(
            "placed interface for {}/{} is incompatible with authoritative source view '{}'",
            placed.library, placed.cell, view.name
        ));
    }
    let mut materialized = LibraryCellInstance::new(&library.name, &cell.name, &view.name);
    materialized.source_path = Some(source_path);
    materialized.module_name = project_veriloga
        .as_ref()
        .map(|binding| binding.netlist_alias().to_owned())
        .or_else(|| {
            view.metadata
                .get("veriloga.module")
                .or_else(|| view.metadata.get("netlist.module"))
                .or_else(|| cell.metadata.get("veriloga.module"))
                .or_else(|| cell.metadata.get("netlist.module"))
                .cloned()
        });
    materialized.netlist_template = metadata_value(
        [&view.metadata, &cell.metadata],
        &["netlist.template", "netlist_template"],
    );
    materialized.model_section = metadata_value(
        [&view.metadata, &cell.metadata],
        &["netlist.section", "model.section"],
    );
    materialized.reference_prefix = metadata_value(
        [&view.metadata, &cell.metadata],
        &["reference.prefix", "reference_prefix"],
    );
    materialized.parameter_order = metadata_terminal_names_for_keys(
        [&view.metadata, &cell.metadata],
        &["netlist.parameter_order"],
    )
    .unwrap_or_default();
    let ports = terminal_order
        .into_iter()
        .map(|name| crate::state::PortSpec {
            name,
            direction: crate::state::PortDirection::InOut,
        })
        .collect::<Vec<_>>();
    materialized.bind_interface(&ports);
    Ok(materialized)
}

pub(crate) fn project_veriloga_binding_for_view(
    workspace: &ProjectWorkspace,
    libraries: &LibraryManager,
    reference: &CellViewRef,
) -> Result<ConfigurationVerilogABinding, String> {
    let library = find_library(libraries, &reference.library).ok_or_else(|| {
        format!(
            "project Verilog-A source owner {} has no authoritative library",
            reference.display_path()
        )
    })?;
    let cell = find_cell(library, &reference.cell).ok_or_else(|| {
        format!(
            "project Verilog-A source owner {} has no authoritative cell",
            reference.display_path()
        )
    })?;
    let view = find_view(cell, &reference.view).ok_or_else(|| {
        format!(
            "project Verilog-A source owner {} has no authoritative view",
            reference.display_path()
        )
    })?;
    if view.view_type != ViewType::VerilogA {
        return Err(format!(
            "project source owner {} is not a Verilog-A view",
            reference.display_path()
        ));
    }
    let owner = ProjectSourceOwner::cell_view(reference.clone());
    let bundle = workspace
        .project_sources
        .bundle_for_owner(&owner)
        .ok_or_else(|| {
            format!(
                "Verilog-A view {} has no project-owned source bundle",
                reference.display_path()
            )
        })?;
    let selected_module = view
        .metadata
        .get("veriloga.module")
        .or_else(|| view.metadata.get("netlist.module"))
        .or_else(|| cell.metadata.get("veriloga.module"))
        .or_else(|| cell.metadata.get("netlist.module"))
        .map(|value| value.trim())
        .filter(|value| !value.is_empty())
        .ok_or_else(|| {
            format!(
                "Verilog-A view {} has no explicit module binding",
                reference.display_path()
            )
        })?
        .to_owned();
    let source_key = super::project_sources::project_veriloga_bundle_source_key(
        workspace.project.id(),
        bundle,
        &selected_module,
    )
    .map_err(|error| error.to_string())?;
    let netlist_alias =
        super::project_sources::project_veriloga_bundle_alias(bundle, &selected_module)
            .map_err(|error| error.to_string())?;
    Ok(ConfigurationVerilogABinding {
        source_bundle_id: bundle.id(),
        source_closure_digest: bundle.closure_digest(),
        selected_module,
        source_key,
        netlist_alias,
    })
}

fn metadata_terminal_names(metadata: &HashMap<String, String>) -> Option<Vec<String>> {
    let encoded = metadata
        .get("netlist.ports")
        .or_else(|| metadata.get("netlist.terminals"))
        .or_else(|| metadata.get("veriloga.ports"))?;
    let names = serde_json::from_str::<Vec<String>>(encoded).unwrap_or_else(|_| {
        encoded
            .split([',', ' ', '\t', '\n'])
            .filter_map(|name| {
                let name = name.trim();
                (!name.is_empty()).then(|| name.to_owned())
            })
            .collect()
    });
    (!names.is_empty()).then_some(names)
}

fn metadata_value<const N: usize>(
    maps: [&HashMap<String, String>; N],
    keys: &[&str],
) -> Option<String> {
    maps.into_iter()
        .find_map(|metadata| {
            keys.iter()
                .find_map(|key| metadata.get(*key))
                .map(|value| value.trim())
                .filter(|value| !value.is_empty())
        })
        .map(str::to_owned)
}

fn metadata_terminal_names_for_keys<const N: usize>(
    maps: [&HashMap<String, String>; N],
    keys: &[&str],
) -> Option<Vec<String>> {
    let encoded = maps
        .into_iter()
        .find_map(|metadata| keys.iter().find_map(|key| metadata.get(*key)))?;
    let values = serde_json::from_str::<Vec<String>>(encoded).unwrap_or_else(|_| {
        encoded
            .split([',', ' ', '\t', '\n'])
            .filter_map(|value| {
                let value = value.trim();
                (!value.is_empty()).then(|| value.to_owned())
            })
            .collect()
    });
    (!values.is_empty()).then_some(values)
}

fn same_terminal_contract(placed: &[String], authoritative: &[&str]) -> bool {
    placed.len() == authoritative.len()
        && placed
            .iter()
            .zip(authoritative)
            .all(|(left, right)| left.eq_ignore_ascii_case(right))
}

fn configured_subcircuit_name(reference: &CellViewRef, instance_path: &str) -> String {
    let stem = reference
        .cell
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() || character == '_' {
                character
            } else {
                '_'
            }
        })
        .collect::<String>();
    let digest = sha2::Sha256::digest(
        format!(
            "{}|{}",
            reference.key().to_ascii_lowercase(),
            instance_path.to_ascii_lowercase()
        )
        .as_bytes(),
    );
    let suffix = digest[..6]
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect::<String>();
    format!("{stem}__cfg_{suffix}")
}

fn metadata_source_path(metadata: &HashMap<String, String>) -> Option<&Path> {
    metadata
        .get("netlist.source_path")
        .or_else(|| metadata.get("veriloga.source_path"))
        .filter(|path| !path.trim().is_empty())
        .map(Path::new)
}

fn source_paths_match(left: &Path, right: &Path) -> bool {
    if left == right {
        return true;
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        match (std::fs::canonicalize(left), std::fs::canonicalize(right)) {
            (Ok(left), Ok(right)) => left == right,
            _ => false,
        }
    }
    #[cfg(target_arch = "wasm32")]
    false
}

fn configured_source_identity(path: &Path) -> String {
    #[cfg(not(target_arch = "wasm32"))]
    let path = std::fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf());
    #[cfg(target_arch = "wasm32")]
    let path = path.to_path_buf();
    path.to_string_lossy()
        .replace('\\', "/")
        .to_ascii_lowercase()
}

#[cfg(not(target_arch = "wasm32"))]
fn validate_source_file(
    source_path: &Path,
    view_type: ViewType,
    binding: &LibraryCellInstance,
) -> Result<(), String> {
    let source = if let Some(section) = binding.model_section.as_deref() {
        let mut processor = rspice_core::netlist::IncludeProcessor::new(source_path);
        processor
            .process_lib(&source_path.to_string_lossy(), Some(section))
            .map_err(|error| {
                format!(
                    "source-backed binding {}/{}/{} cannot resolve model section '{}' from {}: {error}",
                    binding.library,
                    binding.cell,
                    binding.view,
                    section,
                    source_path.display()
                )
            })?
    } else {
        std::fs::read_to_string(source_path).map_err(|error| {
            format!(
                "source-backed binding {}/{}/{} cannot read {}: {error}",
                binding.library,
                binding.cell,
                binding.view,
                source_path.display()
            )
        })?
    };
    let master = binding
        .module_name
        .as_deref()
        .filter(|name| !name.trim().is_empty())
        .unwrap_or(&binding.cell);
    let declaration_found = match view_type {
        ViewType::Verilog | ViewType::VerilogA => source.lines().any(|line| {
            let code = line.split("//").next().unwrap_or_default();
            let mut tokens = code
                .split(|character: char| !character.is_alphanumeric() && character != '_')
                .filter(|token| !token.is_empty());
            tokens.any(|token| token.eq_ignore_ascii_case("module"))
                && tokens.any(|token| token.eq_ignore_ascii_case(master))
        }),
        ViewType::Spice | ViewType::Extracted => source.lines().any(|line| {
            let mut tokens = line.split_ascii_whitespace();
            let directive = tokens.next();
            let declared_name = tokens.next();
            let subcircuit_matches = directive
                .is_some_and(|token| token.eq_ignore_ascii_case(".subckt"))
                && declared_name.is_some_and(|token| token.eq_ignore_ascii_case(master));
            let model_matches = binding.netlist_template.is_some()
                && directive.is_some_and(|token| token.eq_ignore_ascii_case(".model"))
                && declared_name.is_some_and(|token| token.eq_ignore_ascii_case(master));
            subcircuit_matches || model_matches
        }),
        _ => false,
    };
    if declaration_found {
        Ok(())
    } else {
        Err(format!(
            "source-backed binding {}/{}/{} does not declare executable master {master}",
            binding.library, binding.cell, binding.view
        ))
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn validate_configured_model_section(source_path: &Path, section: &str) -> Result<(), String> {
    let mut processor = rspice_core::netlist::IncludeProcessor::new(source_path);
    processor
        .process_lib(&source_path.to_string_lossy(), Some(section))
        .map(|_| ())
        .map_err(|error| error.to_string())
}

#[cfg(target_arch = "wasm32")]
fn validate_configured_model_section(_source_path: &Path, _section: &str) -> Result<(), String> {
    Err("filesystem-backed model sections are unavailable in this browser session".to_owned())
}

#[cfg(target_arch = "wasm32")]
fn validate_source_file(
    _source_path: &Path,
    _view_type: ViewType,
    binding: &LibraryCellInstance,
) -> Result<(), String> {
    Err(format!(
        "source-backed binding {}/{}/{} references a desktop path unavailable in this browser session",
        binding.library, binding.cell, binding.view
    ))
}

fn hierarchy_identity(reference: &CellViewRef) -> String {
    reference.key().to_ascii_lowercase()
}

fn hierarchy_display_path(reference: &CellViewRef) -> String {
    format!("{}/{}", reference.library, reference.cell)
}

fn hierarchy_view_search_order(requested: &str, is_root: bool) -> Vec<String> {
    let requested = if requested.eq_ignore_ascii_case("symbol") {
        DEFAULT_SCHEMATIC_VIEW
    } else {
        requested
    };
    let mut order = vec![requested.to_owned()];
    match ViewType::from_name(requested) {
        ViewType::Schematic | ViewType::Testbench if !is_root => {
            order.push("extracted".to_owned());
            order.push("spice".to_owned());
        }
        ViewType::Schematic | ViewType::Testbench => order.push("spice".to_owned()),
        ViewType::Extracted | ViewType::Verilog | ViewType::VerilogA => {
            order.push("spice".to_owned());
        }
        ViewType::Spice => {}
        _ => order.push("spice".to_owned()),
    }
    order.dedup_by(|left, right| left.eq_ignore_ascii_case(right));
    order
}

fn deduplicate_view_order(order: &mut Vec<String>) {
    let mut seen = HashSet::with_capacity(order.len());
    order.retain(|view| seen.insert(view.to_ascii_lowercase()));
}

fn selected_configuration_override<'a>(
    overrides: &'a [crate::state::ConfigurationSetOverride],
    instance_path: &str,
) -> Option<&'a crate::state::ConfigurationSetOverride> {
    overrides
        .iter()
        .filter(|scoped| instance_path_pattern_matches(&scoped.instance_path, instance_path))
        .max_by_key(|scoped| instance_path_pattern_specificity(&scoped.instance_path))
}

fn instance_path_pattern_matches(pattern: &str, instance_path: &str) -> bool {
    let pattern = pattern.trim_start_matches('/').split('/');
    let instance = instance_path.trim_start_matches('/').split('/');
    let pattern = pattern.collect::<Vec<_>>();
    let instance = instance.collect::<Vec<_>>();
    pattern.len() == instance.len()
        && pattern
            .iter()
            .zip(instance)
            .all(|(expected, actual)| *expected == "*" || expected.eq_ignore_ascii_case(actual))
}

fn instance_path_pattern_specificity(pattern: &str) -> usize {
    pattern
        .trim_start_matches('/')
        .split('/')
        .filter(|segment| *segment != "*")
        .count()
}

fn instance_path_patterns_overlap(left: &str, right: &str) -> bool {
    let left = left.trim_start_matches('/').split('/').collect::<Vec<_>>();
    let right = right.trim_start_matches('/').split('/').collect::<Vec<_>>();
    left.len() == right.len()
        && left
            .iter()
            .zip(right)
            .all(|(left, right)| *left == "*" || right == "*" || left.eq_ignore_ascii_case(right))
}

fn validate_override_pattern_authority(
    configuration: &crate::state::ConfigurationSet,
) -> Result<(), String> {
    for (index, left) in configuration.overrides().iter().enumerate() {
        for right in configuration.overrides().iter().skip(index + 1) {
            if instance_path_pattern_specificity(&left.instance_path)
                == instance_path_pattern_specificity(&right.instance_path)
                && instance_path_patterns_overlap(&left.instance_path, &right.instance_path)
            {
                return Err(format!(
                    "configuration overrides '{}' and '{}' overlap with equal specificity",
                    left.instance_path, right.instance_path
                ));
            }
        }
    }
    Ok(())
}

fn hierarchy_stop_view(view_type: ViewType) -> bool {
    matches!(
        view_type,
        ViewType::Spice | ViewType::Verilog | ViewType::VerilogA | ViewType::Extracted
    )
}

fn hierarchy_model_section(
    libraries: &LibraryManager,
    reference: &CellViewRef,
    binding: Option<&LibraryCellInstance>,
) -> String {
    let library = find_library(libraries, &reference.library);
    let cell = library.and_then(|library| find_cell(library, &reference.cell));
    let view = cell.and_then(|cell| find_view(cell, &reference.view));
    for metadata in [
        view.map(|view| &view.metadata),
        cell.map(|cell| &cell.metadata),
        library.map(|library| &library.metadata),
    ]
    .into_iter()
    .flatten()
    {
        for key in ["model_sections", "model_section", "sections", "section"] {
            if let Some(value) = metadata.get(key).filter(|value| !value.trim().is_empty()) {
                return value.clone();
            }
        }
    }
    if binding
        .and_then(|value| value.source_path.as_ref())
        .is_some()
    {
        "source-defined".to_owned()
    } else {
        "inherit PVT".to_owned()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::Point;

    fn reference(cell: &str) -> CellViewRef {
        CellViewRef::new("work", cell, "schematic")
    }

    fn symbol_reference(cell: &str) -> CellViewRef {
        CellViewRef::new("work", cell, "symbol")
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn model_bound_source_validation_resolves_the_selected_lib_section() {
        let path = std::env::temp_dir().join(format!(
            "rspice-model-bound-section-{}.lib",
            uuid::Uuid::new_v4()
        ));
        std::fs::write(
            &path,
            ".lib TT\n.model nmos_18 nmos level=1\n.endl TT\n.lib FF\n.model nmos_18_fast nmos level=1\n.endl FF\n",
        )
        .expect("write sectioned model fixture");
        let mut binding = LibraryCellInstance::new("models", "nmos_18", "spice");
        binding.module_name = Some("nmos_18".to_owned());
        binding.netlist_template = Some("M{name} {nodes} {model} {params}".to_owned());
        binding.model_section = Some("TT".to_owned());

        validate_source_file(&path, ViewType::Spice, &binding)
            .expect("selected section declares the executable model");
        binding.model_section = Some("FF".to_owned());
        assert!(validate_source_file(&path, ViewType::Spice, &binding).is_err());

        std::fs::remove_file(path).expect("remove sectioned model fixture");
    }

    #[test]
    fn configuration_override_patterns_use_most_specific_segment_match() {
        let overrides = vec![
            crate::state::ConfigurationSetOverride {
                instance_path: "/top/*".to_owned(),
                executable_views: vec!["spice".to_owned()],
                stop_view: Some("spice".to_owned()),
                model_section: None,
                eligible_platforms: crate::state::ConfigurationPlatform::ALL.to_vec(),
            },
            crate::state::ConfigurationSetOverride {
                instance_path: "/top/Xcritical".to_owned(),
                executable_views: vec!["schematic".to_owned()],
                stop_view: None,
                model_section: None,
                eligible_platforms: crate::state::ConfigurationPlatform::ALL.to_vec(),
            },
        ];
        let selected = selected_configuration_override(&overrides, "/top/xCRITICAL")
            .expect("specific override matches");
        assert_eq!(selected.instance_path, "/top/Xcritical");
        let wildcard = selected_configuration_override(&overrides, "/top/Xother")
            .expect("wildcard override matches");
        assert_eq!(wildcard.instance_path, "/top/*");
    }

    #[test]
    fn equal_specificity_pattern_overlap_is_detectable() {
        assert!(instance_path_patterns_overlap("/top/*/X1", "/top/I0/*"));
        assert_eq!(
            instance_path_pattern_specificity("/top/*/X1"),
            instance_path_pattern_specificity("/top/I0/*")
        );
        assert!(!instance_path_patterns_overlap("/top/I0/X1", "/top/I1/X1"));
    }

    fn resistance_variable(
        name: &str,
        expression: &str,
        scope: DesignVariableScope,
    ) -> DesignVariable {
        DesignVariable::new(
            name,
            expression,
            DesignVariableQuantity::Resistance,
            scope,
            "fixture",
            Some(DesignVariableRange {
                minimum: "1 kohm".to_owned(),
                maximum: "1 Mohm".to_owned(),
            }),
            DesignVariableSweepEligibility::NestedSweepAndOptimization,
            DesignVariableOverridePolicy::ExplicitTestLocalOverride,
        )
        .expect("fixture variable is valid")
    }

    fn raw_output(
        name: &str,
        expression: &str,
        compatibility: SavedOutputCompatibility,
    ) -> SavedOutput {
        SavedOutput::new(
            SavedOutputKind::RawVoltageOrCurrent,
            name,
            expression,
            compatibility,
            SavedOutputPolicy::EveryAcceptedPoint,
            SavedOutputPrecision::FullSourcePrecision,
            SavedOutputStreaming::LivePlotAdaptiveDisplayDecimation,
        )
        .expect("fixture output is valid")
    }

    #[test]
    fn typed_design_variable_enforces_units_range_and_canonical_netlist_value() {
        let variable = resistance_variable("RLOAD", "10 kohm", DesignVariableScope::Project);
        assert_eq!(variable.resolved_value_si().unwrap(), 10_000.0);
        assert_eq!(
            variable.netlist_statement(),
            ".param RLOAD=1.00000000000000000e4"
        );

        let mut wrong_unit = variable.clone();
        wrong_unit.expression = "10 V".to_owned();
        assert!(wrong_unit.validate().unwrap_err().contains("resistance"));

        let mut outside = variable;
        outside.expression = "2 Mohm".to_owned();
        assert!(outside.validate().unwrap_err().contains("outside"));
    }

    #[test]
    fn design_variable_expression_update_preserves_identity_and_metadata() {
        let plan_id = SimulationPlanId::new();
        let mut workspace = ProjectWorkspace::default();
        let original = resistance_variable("RLOAD", "10 kohm", DesignVariableScope::Project);
        let variable_id = original.id;
        workspace
            .add_design_variable(plan_id, original.clone())
            .expect("fixture variable is accepted");

        workspace
            .update_design_variable_expression(plan_id, variable_id, "22 kohm")
            .expect("valid expression update commits");

        let updated = &workspace
            .active_plan_data(plan_id)
            .expect("plan payload remains present")
            .design_variables[0];
        assert_eq!(updated.id, original.id);
        assert_eq!(updated.name, original.name);
        assert_eq!(updated.expression, "22 kohm");
        assert_eq!(updated.quantity, original.quantity);
        assert_eq!(updated.scope, original.scope);
        assert_eq!(updated.description, original.description);
        assert_eq!(updated.allowed_range, original.allowed_range);
        assert_eq!(updated.sweep_eligibility, original.sweep_eligibility);
        assert_eq!(updated.override_policy, original.override_policy);
    }

    #[test]
    fn out_of_range_design_variable_update_is_rejected_atomically() {
        let plan_id = SimulationPlanId::new();
        let mut workspace = ProjectWorkspace::default();
        let variable = resistance_variable("RLOAD", "10 kohm", DesignVariableScope::Project);
        let variable_id = variable.id;
        workspace
            .add_design_variable(plan_id, variable)
            .expect("fixture variable is accepted");
        let before = serde_json::to_value(&workspace).expect("workspace serializes");

        let error = workspace
            .update_design_variable_expression(plan_id, variable_id, "2 Mohm")
            .expect_err("out-of-range expression must be rejected");

        assert!(matches!(
            error,
            SimulationConfigurationError::InvalidDesignVariable { message, .. }
                if message.contains("outside the inclusive allowed range")
        ));
        assert_eq!(
            serde_json::to_value(&workspace).expect("workspace still serializes"),
            before
        );
    }

    #[test]
    fn design_variable_update_rejects_a_missing_stable_identity() {
        let plan_id = SimulationPlanId::new();
        let mut workspace = ProjectWorkspace::default();
        let variable = resistance_variable("RLOAD", "10 kohm", DesignVariableScope::Project);
        workspace
            .add_design_variable(plan_id, variable)
            .expect("fixture variable is accepted");
        let missing_id = DesignVariableId::new();
        let before = serde_json::to_value(&workspace).expect("workspace serializes");

        assert_eq!(
            workspace.update_design_variable_expression(plan_id, missing_id, "22 kohm"),
            Err(SimulationConfigurationError::DesignVariableNotFound {
                plan_id,
                variable_id: missing_id,
            })
        );
        assert_eq!(
            serde_json::to_value(&workspace).expect("workspace still serializes"),
            before
        );
    }

    #[test]
    fn committed_design_variable_update_advances_revision_once() {
        let plan_id = SimulationPlanId::new();
        let mut workspace = ProjectWorkspace::default();
        let variable = resistance_variable("RLOAD", "10 kohm", DesignVariableScope::Project);
        let variable_id = variable.id;
        let initial_revision = variable.revision;
        workspace
            .add_design_variable(plan_id, variable)
            .expect("fixture variable is accepted");

        let committed_revision = workspace
            .update_design_variable_expression(plan_id, variable_id, "22 kohm")
            .expect("valid expression update commits");

        assert_eq!(committed_revision.get(), initial_revision.get() + 1);
        assert_eq!(
            workspace
                .active_plan_data(plan_id)
                .expect("plan payload remains present")
                .design_variables[0]
                .revision,
            committed_revision
        );
    }

    #[test]
    fn bulk_design_variable_update_is_all_or_nothing() {
        let plan_id = SimulationPlanId::new();
        let mut workspace = ProjectWorkspace::default();
        let first = resistance_variable("RLOAD", "10 kohm", DesignVariableScope::Project);
        let second = resistance_variable("RBIAS", "15 kohm", DesignVariableScope::Project);
        let updates = vec![
            (first.id, "22 kohm".to_owned()),
            (second.id, "2 Mohm".to_owned()),
        ];
        workspace
            .add_design_variable(plan_id, first)
            .expect("first fixture variable is accepted");
        workspace
            .add_design_variable(plan_id, second)
            .expect("second fixture variable is accepted");
        let before = serde_json::to_value(&workspace).expect("workspace serializes");

        assert!(matches!(
            workspace.update_design_variable_expressions(plan_id, &updates),
            Err(SimulationConfigurationError::InvalidDesignVariable { index: 1, .. })
        ));
        assert_eq!(
            serde_json::to_value(&workspace).expect("workspace still serializes"),
            before
        );
    }

    #[test]
    fn bulk_design_variable_update_rejects_duplicate_identities_atomically() {
        let plan_id = SimulationPlanId::new();
        let mut workspace = ProjectWorkspace::default();
        let variable = resistance_variable("RLOAD", "10 kohm", DesignVariableScope::Project);
        let variable_id = variable.id;
        workspace
            .add_design_variable(plan_id, variable)
            .expect("fixture variable is accepted");
        let before = serde_json::to_value(&workspace).expect("workspace serializes");
        let updates = vec![
            (variable_id, "22 kohm".to_owned()),
            (variable_id, "47 kohm".to_owned()),
        ];

        assert_eq!(
            workspace.update_design_variable_expressions(plan_id, &updates),
            Err(
                SimulationConfigurationError::DuplicateDesignVariableUpdate {
                    plan_id,
                    variable_id,
                }
            )
        );
        assert_eq!(
            serde_json::to_value(&workspace).expect("workspace still serializes"),
            before
        );
    }

    #[test]
    fn saved_output_validation_is_kind_specific() {
        assert!(
            raw_output(
                "VOUT",
                "V(out)",
                SavedOutputCompatibility::AllCompatibleAnalyses
            )
            .validate()
            .is_ok()
        );
        let invalid = SavedOutput::new(
            SavedOutputKind::RawVoltageOrCurrent,
            "gain",
            "V(out) / V(in)",
            SavedOutputCompatibility::AllCompatibleAnalyses,
            SavedOutputPolicy::EveryAcceptedPoint,
            SavedOutputPrecision::FullSourcePrecision,
            SavedOutputStreaming::StoreOnly,
        );
        assert!(invalid.unwrap_err().contains("raw output"));
        let derived = SavedOutput::new(
            SavedOutputKind::DerivedExpression,
            "gain",
            "V(out) / V(in)",
            SavedOutputCompatibility::AllCompatibleAnalyses,
            SavedOutputPolicy::OnDemandFromRetainedState,
            SavedOutputPrecision::FullSourcePrecision,
            SavedOutputStreaming::StoreOnly,
        )
        .expect("calculator expression is valid");
        assert_eq!(derived.inferred_unit(), "resolved from expression");
    }

    #[test]
    fn missing_row_identity_migrates_deterministically_and_null_is_rejected() {
        let variable = resistance_variable("RLOAD", "10 kohm", DesignVariableScope::Project);
        let mut value = serde_json::to_value(variable).unwrap();
        value.as_object_mut().unwrap().remove("id");
        let first: DesignVariable = serde_json::from_value(value.clone()).unwrap();
        let second: DesignVariable = serde_json::from_value(value.clone()).unwrap();
        assert_eq!(first.id, second.id);

        value
            .as_object_mut()
            .unwrap()
            .insert("id".to_owned(), serde_json::Value::Null);
        assert!(
            serde_json::from_value::<DesignVariable>(value)
                .unwrap_err()
                .to_string()
                .contains("must not be null")
        );
    }

    #[test]
    fn plan_payload_clone_refreshes_row_ids_and_analysis_references() {
        let source_plan_id = SimulationPlanId::new();
        let cloned_plan_id = SimulationPlanId::new();
        let source_analysis = AnalysisInstanceId::new();
        let cloned_analysis = AnalysisInstanceId::new();
        let mut workspace = ProjectWorkspace::default();
        let variable = resistance_variable(
            "RLOAD",
            "10 kohm",
            DesignVariableScope::SelectedAnalysis {
                analysis_id: source_analysis,
            },
        );
        let output = raw_output(
            "VOUT",
            "V(out)",
            SavedOutputCompatibility::SelectedAnalysis {
                analysis_id: source_analysis,
            },
        );
        let variable_id = variable.id;
        let output_id = output.id;
        let regression_rule = RegressionToleranceRule {
            target: RegressionTargetSelector {
                source_domain: AnalysisResultSourceDomain::SimulationPlan,
                source_instance_id: source_analysis,
                kind: RegressionTargetKind::Waveform,
                name: "v(out)".to_owned(),
                occurrence: 0,
            },
            method: RegressionComparisonMethod::AbsoluteRelativeEnvelope,
            absolute_tolerance: 0.01,
            relative_tolerance: 0.005,
            time_skew_allowance: 20e-6,
            comparison_window: Some(RegressionComparisonWindow {
                start: 0.0,
                end: 20e-3,
            }),
        };
        workspace
            .simulation_plan_payloads
            .push(SimulationPlanPayloadRecord {
                plan_id: source_plan_id,
                payload: SimulationPlanPayload {
                    design_variables: vec![variable],
                    saved_outputs: vec![output],
                    regression_baseline_run: Some(RunId::new()),
                    regression_tolerances: vec![regression_rule],
                    ..SimulationPlanPayload::default()
                },
            });

        workspace
            .clone_plan_data(
                source_plan_id,
                cloned_plan_id,
                true,
                true,
                &[(source_analysis, cloned_analysis)],
            )
            .unwrap();
        let cloned = workspace.active_plan_data(cloned_plan_id).unwrap();
        assert_ne!(cloned.design_variables[0].id, variable_id);
        assert_ne!(cloned.saved_outputs[0].id, output_id);
        assert!(matches!(
            cloned.design_variables[0].scope,
            DesignVariableScope::SelectedAnalysis { analysis_id }
                if analysis_id == cloned_analysis
        ));
        assert_eq!(cloned.regression_tolerances.len(), 1);
        assert_eq!(
            cloned.regression_tolerances[0].target.source_instance_id,
            cloned_analysis
        );
        assert_eq!(
            cloned.regression_tolerances[0].comparison_window,
            Some(RegressionComparisonWindow {
                start: 0.0,
                end: 20e-3,
            })
        );
        assert!(matches!(
            cloned.saved_outputs[0].compatible_analyses,
            SavedOutputCompatibility::SelectedAnalysis { analysis_id }
                if analysis_id == cloned_analysis
        ));

        workspace
            .active_plan_data_mut(cloned_plan_id)
            .unwrap()
            .design_variables[0]
            .expression = "20 kohm".to_owned();
        assert_eq!(
            workspace
                .active_plan_data(source_plan_id)
                .unwrap()
                .design_variables[0]
                .expression,
            "10 kohm"
        );
        workspace.validate_simulation_configuration().unwrap();
    }

    #[test]
    fn regression_tolerance_contract_round_trips_and_rejects_invalid_windows() {
        let plan_id = SimulationPlanId::new();
        let mut workspace = ProjectWorkspace::default();
        let rule = RegressionToleranceRule {
            target: RegressionTargetSelector {
                source_domain: AnalysisResultSourceDomain::ManualDeck,
                source_instance_id: AnalysisInstanceId::new(),
                kind: RegressionTargetKind::Waveform,
                name: "v(out)".to_owned(),
                occurrence: 0,
            },
            method: RegressionComparisonMethod::PointwiseRelative,
            absolute_tolerance: 1e-3,
            relative_tolerance: 0.02,
            time_skew_allowance: 1e-6,
            comparison_window: Some(RegressionComparisonWindow {
                start: 0.0,
                end: 1e-3,
            }),
        };
        workspace
            .ensure_active_plan_data(plan_id)
            .regression_tolerances = vec![rule.clone()];
        workspace.validate_simulation_configuration().unwrap();

        let json = serde_json::to_string(&workspace).unwrap();
        let restored: ProjectWorkspace = serde_json::from_str(&json).unwrap();
        assert_eq!(
            restored
                .active_plan_data(plan_id)
                .unwrap()
                .regression_tolerances,
            vec![rule]
        );

        let mut invalid = restored;
        invalid
            .active_plan_data_mut(plan_id)
            .unwrap()
            .regression_tolerances[0]
            .comparison_window = Some(RegressionComparisonWindow {
            start: 2.0,
            end: 1.0,
        });
        assert!(matches!(
            invalid.validate_simulation_configuration(),
            Err(SimulationConfigurationError::InvalidRegressionTolerance { .. })
        ));

        let mut invalid_name = workspace;
        invalid_name
            .active_plan_data_mut(plan_id)
            .unwrap()
            .regression_tolerances[0]
            .target
            .name = "v(out)\u{1}".to_owned();
        assert!(matches!(
            invalid_name.validate_simulation_configuration(),
            Err(SimulationConfigurationError::InvalidRegressionTolerance { .. })
        ));
    }

    fn add_schematic_master(
        libraries: &mut LibraryManager,
        workspace: &mut ProjectWorkspace,
        library_name: &str,
        cell_name: &str,
        schematic: SchematicState,
    ) {
        if libraries.get_library(library_name).is_none() {
            libraries.add_library(Library::new(library_name));
        }
        let library = libraries
            .get_library_mut(library_name)
            .expect("library exists");
        let cell = library.get_or_create_cell(cell_name);
        if cell.get_view("schematic").is_none() {
            cell.add_view(View::new("schematic", ViewType::Schematic));
        }
        workspace.schematic_buffers.insert(
            CellViewRef::new(library_name, cell_name, "schematic").key(),
            schematic,
        );
    }

    fn instance(library: &str, cell: &str) -> LibraryCellInstance {
        LibraryCellInstance::new(library, cell, "schematic")
    }

    #[test]
    fn hierarchy_resolution_follows_instances_not_open_tabs() {
        let mut workspace = ProjectWorkspace::default();
        workspace.open_views.push(OpenCellView::new(
            CellViewRef::new("unrelated", "open_tab", "schematic"),
            ViewType::Schematic,
        ));
        let mut libraries = LibraryManager::default();
        workspace.ensure_library_model(&mut libraries);

        let resolution = workspace.resolve_hierarchy(&libraries);

        assert_eq!(resolution.total_instances, 1);
        assert_eq!(resolution.resolved_instances, 1);
        assert_eq!(resolution.bindings.len(), 1);
        assert_eq!(resolution.bindings[0].purpose, "testbench root");
        assert_eq!(resolution.bindings[0].reference.cell, "top");
    }

    #[test]
    fn hierarchy_resolution_counts_transitive_repeated_instances() {
        let mut workspace = ProjectWorkspace::default();
        let mut libraries = LibraryManager::default();
        workspace.ensure_library_model(&mut libraries);

        let top = workspace
            .schematic_buffers
            .get_mut(&CellViewRef::default_top().key())
            .expect("top buffer");
        top.add_library_cell_component(Point::new(20, 20), instance("work", "amp"));
        top.add_library_cell_component(Point::new(80, 20), instance("work", "amp"));

        let mut amp = SchematicState::default();
        amp.add_library_cell_component(Point::new(40, 40), instance("work", "bias"));
        add_schematic_master(&mut libraries, &mut workspace, "work", "amp", amp);
        add_schematic_master(
            &mut libraries,
            &mut workspace,
            "work",
            "bias",
            SchematicState::default(),
        );

        let resolution = workspace.resolve_hierarchy(&libraries);

        assert!(resolution.is_valid());
        assert_eq!(resolution.total_instances, 5);
        assert_eq!(resolution.resolved_instances, 5);
        assert_eq!(resolution.bindings.len(), 3);
        let amp = resolution
            .bindings
            .iter()
            .find(|row| row.reference.cell == "amp")
            .expect("amp row");
        assert_eq!(amp.instance_count, 2);
        assert_eq!(amp.purpose, "design under test");
        assert_eq!(
            amp.view_search_order
                .iter()
                .map(String::as_str)
                .collect::<Vec<_>>(),
            ["schematic", "extracted", "spice"]
        );
        assert_eq!(amp.stop_view.as_deref(), Some("spice"));
        let bias = resolution
            .bindings
            .iter()
            .find(|row| row.reference.cell == "bias")
            .expect("bias row");
        assert_eq!(bias.instance_count, 2);
        assert_eq!(bias.purpose, "hierarchical cell");
    }

    #[test]
    fn active_configuration_drives_exact_path_resolution_and_receipt_identity() {
        let mut workspace = ProjectWorkspace::default();
        let mut libraries = LibraryManager::default();
        workspace.ensure_library_model(&mut libraries);
        let top = workspace
            .schematic_buffers
            .get_mut(&CellViewRef::default_top().key())
            .expect("top buffer");
        top.add_library_cell_component(Point::new(20, 20), instance("work", "amp"));
        top.add_library_cell_component(Point::new(80, 20), instance("work", "amp"));
        add_schematic_master(
            &mut libraries,
            &mut workspace,
            "work",
            "amp",
            SchematicState::default(),
        );

        let id = workspace
            .configuration_sets
            .create(crate::state::ConfigurationSetDefinition {
                name: "Lab characterization".to_owned(),
                root: CellViewRef::default_top(),
                dut_path: "/top/X1".to_owned(),
                executable_view_policy: vec!["schematic".to_owned(), "spice".to_owned()],
                stop_views: vec!["spice".to_owned()],
                unresolved_policy: crate::state::UnresolvedBindingPolicy::BlockNetlist,
                black_box_policy:
                    crate::state::ConfigurationBlackBoxPolicy::MaterializedSourceBoundariesOnly,
                overrides: vec![crate::state::ConfigurationSetOverride {
                    instance_path: "/top/X2".to_owned(),
                    executable_views: vec!["spice".to_owned()],
                    stop_view: Some("spice".to_owned()),
                    model_section: Some("tt".to_owned()),
                    eligible_platforms: crate::state::ConfigurationPlatform::ALL.to_vec(),
                }],
                model_profile: crate::state::ConfigurationModelProfile::ProjectRunSetSections,
                owner: "Analog design".to_owned(),
            })
            .expect("create configuration");

        let resolution = workspace.resolve_hierarchy(&libraries);

        assert_eq!(resolution.configuration_id, Some(id));
        assert_eq!(resolution.configuration_revision, Some(1));
        assert_eq!(
            resolution.configuration_digest,
            workspace
                .configuration_sets
                .find(id)
                .map(|configuration| configuration.semantic_digest())
        );
        assert_eq!(resolution.total_instances, 3);
        assert_eq!(resolution.resolved_instances, 2);
        assert_eq!(resolution.unresolved_instances(), 1);
        let configured = resolution
            .bindings
            .iter()
            .find(|binding| binding.instance_paths == ["/top/X2"])
            .expect("exact overridden instance row");
        assert_eq!(configured.view_search_order, ["spice"]);
        assert_eq!(configured.model_section, "tt");
        assert_eq!(configured.status, HierarchyBindingStatus::Unresolved);
        assert!(resolution.bindings.iter().any(|binding| {
            binding.instance_paths.iter().any(|path| path == "/top/X1")
                && binding.status.is_resolved()
        }));
    }

    #[test]
    fn active_configuration_rejects_missing_dut_and_override_paths() {
        let mut workspace = ProjectWorkspace::default();
        let mut libraries = LibraryManager::default();
        workspace.ensure_library_model(&mut libraries);
        workspace
            .configuration_sets
            .create(crate::state::ConfigurationSetDefinition {
                name: "Missing bindings".to_owned(),
                root: CellViewRef::default_top(),
                dut_path: "/top/XMISSING".to_owned(),
                executable_view_policy: vec!["schematic".to_owned(), "spice".to_owned()],
                stop_views: vec!["spice".to_owned()],
                unresolved_policy: crate::state::UnresolvedBindingPolicy::BlockNetlist,
                black_box_policy:
                    crate::state::ConfigurationBlackBoxPolicy::MaterializedSourceBoundariesOnly,
                overrides: vec![crate::state::ConfigurationSetOverride {
                    instance_path: "/top/XOTHER".to_owned(),
                    executable_views: vec!["schematic".to_owned()],
                    stop_view: None,
                    model_section: None,
                    eligible_platforms: crate::state::ConfigurationPlatform::ALL.to_vec(),
                }],
                model_profile: crate::state::ConfigurationModelProfile::ProjectRunSetSections,
                owner: "Local project".to_owned(),
            })
            .expect("create configuration");

        let resolution = workspace.resolve_hierarchy(&libraries);

        assert_eq!(resolution.total_instances, 3);
        assert_eq!(resolution.resolved_instances, 1);
        assert_eq!(resolution.unresolved_instances(), 2);
        assert!(resolution.bindings.iter().any(|binding| {
            binding.diagnostic.as_deref().is_some_and(|diagnostic| {
                diagnostic.contains("configured DUT path /top/XMISSING does not exist")
            })
        }));
        assert!(resolution.bindings.iter().any(|binding| {
            binding.diagnostic.as_deref().is_some_and(|diagnostic| {
                diagnostic.contains("scoped configuration override /top/XOTHER does not exist")
            })
        }));
    }

    #[test]
    fn reviewed_fallback_is_resolved_and_retained_in_the_hierarchy_receipt() {
        let mut workspace = ProjectWorkspace::default();
        let mut libraries = LibraryManager::default();
        workspace.ensure_library_model(&mut libraries);
        let top = workspace
            .schematic_buffers
            .get_mut(&CellViewRef::default_top().key())
            .expect("top buffer");
        top.add_library_cell_component(Point::new(20, 20), instance("work", "amp"));
        add_schematic_master(
            &mut libraries,
            &mut workspace,
            "work",
            "amp",
            SchematicState::default(),
        );
        workspace
            .configuration_sets
            .create(crate::state::ConfigurationSetDefinition {
                name: "Reviewed fallback".to_owned(),
                root: CellViewRef::default_top(),
                dut_path: "/top/X1".to_owned(),
                executable_view_policy: vec!["spice".to_owned()],
                stop_views: vec!["spice".to_owned()],
                unresolved_policy:
                    crate::state::UnresolvedBindingPolicy::ExplicitFallbackWithReview,
                black_box_policy:
                    crate::state::ConfigurationBlackBoxPolicy::MaterializedSourceBoundariesOnly,
                overrides: Vec::new(),
                model_profile: crate::state::ConfigurationModelProfile::ProjectRunSetSections,
                owner: "Local project".to_owned(),
            })
            .expect("create configuration");

        let resolution = workspace.resolve_hierarchy(&libraries);
        let fallback = resolution
            .bindings
            .iter()
            .find(|binding| binding.instance_paths == ["/top/X1"])
            .expect("child binding");

        assert!(fallback.status.is_resolved());
        assert!(fallback.used_review_fallback);
        assert_eq!(fallback.reference.view, "schematic");
        assert_eq!(
            fallback.view_search_order,
            ["spice", "schematic", "extracted"]
        );
    }

    #[test]
    fn configuration_catalog_replacement_advances_project_revision_atomically() {
        let mut workspace = ProjectWorkspace::default();
        let original_revision = workspace.project.revision;
        let mut candidate = workspace.configuration_sets.clone();
        candidate
            .create(crate::state::ConfigurationSetDefinition {
                name: "Release".to_owned(),
                root: CellViewRef::default_top(),
                dut_path: "/top/X1".to_owned(),
                executable_view_policy: vec!["schematic".to_owned()],
                stop_views: Vec::new(),
                unresolved_policy: crate::state::UnresolvedBindingPolicy::BlockNetlist,
                black_box_policy:
                    crate::state::ConfigurationBlackBoxPolicy::MaterializedSourceBoundariesOnly,
                overrides: Vec::new(),
                model_profile: crate::state::ConfigurationModelProfile::ProjectRunSetSections,
                owner: "Local project".to_owned(),
            })
            .expect("candidate configuration");

        let committed_revision = workspace
            .replace_configuration_sets(candidate.clone())
            .expect("publish configuration catalog");
        assert_eq!(workspace.project.revision, committed_revision);
        assert_ne!(workspace.project.revision, original_revision);
        assert_eq!(workspace.configuration_sets, candidate);
        assert!(workspace.project_metadata_dirty);

        let committed = workspace.clone();
        assert_eq!(
            workspace.replace_configuration_sets(candidate),
            Err(ProjectConfigurationMutationError::NoChanges)
        );
        assert_eq!(workspace.project.revision, committed.project.revision);
        assert_eq!(workspace.configuration_sets, committed.configuration_sets);
    }

    #[test]
    fn configuration_catalog_replacement_rejects_unmaterialized_roots_atomically() {
        let mut workspace = ProjectWorkspace::default();
        let before = workspace.clone();
        let mut candidate = crate::state::ConfigurationSetCatalog::default();
        candidate
            .create(crate::state::ConfigurationSetDefinition {
                name: "Missing root".to_owned(),
                root: CellViewRef::new("user", "missing", "schematic"),
                dut_path: "/top/X1".to_owned(),
                executable_view_policy: vec!["schematic".to_owned()],
                stop_views: Vec::new(),
                unresolved_policy: crate::state::UnresolvedBindingPolicy::BlockNetlist,
                black_box_policy:
                    crate::state::ConfigurationBlackBoxPolicy::MaterializedSourceBoundariesOnly,
                overrides: Vec::new(),
                model_profile: crate::state::ConfigurationModelProfile::ProjectRunSetSections,
                owner: "Local project".to_owned(),
            })
            .expect("structurally valid candidate");

        assert!(matches!(
            workspace.replace_configuration_sets(candidate),
            Err(ProjectConfigurationMutationError::MissingRootBuffer { .. })
        ));
        assert_eq!(workspace.project.revision, before.project.revision);
        assert_eq!(workspace.configuration_sets, before.configuration_sets);
        assert_eq!(
            workspace.project_metadata_dirty,
            before.project_metadata_dirty
        );
    }

    #[test]
    fn design_management_projection_namespaces_sheets_and_materializes_explicit_ports() {
        use crate::state::{
            CrossSheetDiscipline, CrossSheetPortAnchor, CrossSheetPortDefinition,
            CrossSheetPortDirection, CrossSheetPortEndpoint, CrossSheetSignalType,
            MoveBoundaryResolution, MoveSelectionRequest, SheetDefinition, SheetPortPolicy,
            SheetTemplate,
        };

        let mut workspace = ProjectWorkspace::default();
        let key = CellViewRef::default_top().key();
        let mut schematic = SchematicState::default();
        let first = schematic
            .add_wire(vec![Point::origin(), Point::new(10, 0)])
            .expect("first wire");
        let second = schematic
            .add_wire(vec![Point::origin(), Point::new(0, 10)])
            .expect("second wire");
        let component = schematic.add_component(ComponentType::Resistor, Point::new(20, 0));
        let terminal_name = schematic
            .components
            .iter()
            .find(|candidate| candidate.id == component)
            .expect("component")
            .terminal_positions_resolved(None)
            .into_iter()
            .find(|(_, point)| *point == Point::origin())
            .map(|(name, _)| name)
            .expect("terminal at the second-wire anchor");
        schematic
            .connections
            .push(crate::state::WireConnection::new(
                second,
                0,
                component,
                terminal_name.clone(),
            ));
        let source_sheet = workspace
            .design_management
            .bootstrap_for_cell_view(&key, "Input", [first, second, component])
            .expect("bootstrap sheet ownership");
        let catalog = workspace
            .design_management
            .sheet_catalog_mut(&key)
            .expect("sheet catalog");
        let destination_sheet = catalog
            .create_sheet(
                SheetDefinition {
                    name: "Output".to_owned(),
                    template: SheetTemplate::AnalogSchematic,
                    port_policy: SheetPortPolicy::TypedOffSheetPorts,
                    explicit_page_number: Some(2),
                },
                Some(source_sheet),
            )
            .expect("second sheet");
        catalog
            .move_selection(MoveSelectionRequest {
                expected_catalog_revision: catalog.revision(),
                object_ids: vec![second, component],
                destination_sheet_id: destination_sheet,
                boundary_resolution: MoveBoundaryResolution::ExplicitPorts {
                    ports: vec![CrossSheetPortDefinition {
                        net_name: "BIAS".to_owned(),
                        first: CrossSheetPortEndpoint {
                            sheet_id: source_sheet,
                            anchor: CrossSheetPortAnchor::WirePoint {
                                wire_id: first,
                                point: Point::origin(),
                            },
                        },
                        second: CrossSheetPortEndpoint {
                            sheet_id: destination_sheet,
                            anchor: CrossSheetPortAnchor::ComponentTerminal {
                                component_id: component,
                                terminal_name,
                            },
                        },
                        direction: CrossSheetPortDirection::Output,
                        signal_type: CrossSheetSignalType::Analog,
                        discipline: CrossSheetDiscipline::Electrical,
                    }],
                },
            })
            .expect("move with explicit boundary contract");

        let projected = workspace
            .materialize_design_management_schematic(&key, &schematic)
            .expect("materialize governed design");
        let first_position = projected
            .wires
            .iter()
            .find(|wire| wire.id == first)
            .and_then(|wire| wire.points.first())
            .copied()
            .expect("first wire");
        let second_position = projected
            .wires
            .iter()
            .find(|wire| wire.id == second)
            .and_then(|wire| wire.points.first())
            .copied()
            .expect("second wire");
        assert_ne!(first_position, second_position);
        assert_eq!(first_position, Point::origin());
        assert_eq!(second_position, Point::new(1_000_000, 0));

        let mut port_positions = projected
            .net_labels
            .iter()
            .filter(|label| label.name == "BIAS")
            .map(|label| label.pos)
            .collect::<Vec<_>>();
        port_positions.sort_by_key(|point| point.x);
        assert_eq!(port_positions, [first_position, second_position]);
    }

    #[test]
    fn design_management_projection_applies_active_variant_and_annotation() {
        use std::collections::BTreeMap;

        use crate::state::{
            AnnotationObject, AnnotationPosition, AssemblyVariantDraft, ComponentSubstitution,
            ProtectedReferencePolicy, RenumberOrder, RenumberRequest, RenumberScope,
            SchematicObjectKey, VariantInheritance, VariantObjectOverride,
            VariantQualificationPlan, VariantQualificationState,
        };

        let mut workspace = ProjectWorkspace::default();
        let key = CellViewRef::default_top().key();
        let mut schematic = SchematicState::default();
        let substituted = schematic.add_component(ComponentType::Resistor, Point::new(10, 10));
        let omitted = schematic.add_component(ComponentType::Capacitor, Point::new(20, 10));
        let variant = workspace
            .design_management
            .variants_mut()
            .create(AssemblyVariantDraft {
                name: "Automotive".to_owned(),
                parent_id: None,
                inheritance: VariantInheritance::OverrideChangedObjectsOnly,
                qualification_plan: VariantQualificationPlan::InvalidateAffectedTests,
                overrides: BTreeMap::from([
                    (
                        SchematicObjectKey::new(&key, substituted)
                            .expect("scoped substituted identity"),
                        VariantObjectOverride::Substitute {
                            replacement: ComponentSubstitution {
                                library: "qualified".to_owned(),
                                cell: "resistor_aecq".to_owned(),
                                view: "schematic".to_owned(),
                                value_override: Some("2 kohm".to_owned()),
                                model_section: Some("automotive".to_owned()),
                                port_equivalence_digest: Some(ContentDigest::from_bytes([9; 32])),
                                qualification: VariantQualificationState::Current,
                            },
                        },
                    ),
                    (
                        SchematicObjectKey::new(&key, omitted).expect("scoped omitted identity"),
                        VariantObjectOverride::DoNotPopulate {
                            approval_reference: "ECO-104".to_owned(),
                        },
                    ),
                ]),
            })
            .expect("create governed variant");
        workspace
            .design_management
            .variants_mut()
            .set_active(variant)
            .expect("activate variant");

        let request = RenumberRequest {
            scope: RenumberScope::WholeProject,
            order: RenumberOrder::HierarchyThenCoordinates,
            protected_references: ProtectedReferencePolicy::RetainLockedAndExternalIds,
            protected_reviewed: false,
            objects: vec![AnnotationObject {
                object: SchematicObjectKey::new(&key, substituted)
                    .expect("scoped annotation identity"),
                current_reference: "R42".to_owned(),
                device_family: "R".to_owned(),
                sheet_id: None,
                hierarchy_path: "/top".to_owned(),
                position: AnnotationPosition { x: 10, y: 10 },
                connectivity_order: Some(1),
                locked: false,
                external: false,
                imported: false,
            }],
        };
        let preview = workspace
            .design_management
            .annotation()
            .preview_renumbering(&request)
            .expect("preview annotation");
        workspace
            .design_management
            .annotation_mut()
            .commit_renumbering(&preview, &request)
            .expect("commit annotation receipt");

        let projected = workspace
            .materialize_design_management_schematic(&key, &schematic)
            .expect("materialize variant and annotation");
        assert!(
            projected
                .components
                .iter()
                .all(|component| component.id != omitted)
        );
        assert!(
            projected
                .connections
                .iter()
                .all(|connection| connection.component_id != omitted)
        );
        let component = projected
            .components
            .iter()
            .find(|component| component.id == substituted)
            .expect("substituted component");
        let binding = component
            .library_cell
            .as_ref()
            .expect("qualified cell binding");
        assert_eq!(binding.library, "qualified");
        assert_eq!(binding.cell, "resistor_aecq");
        assert_eq!(component.value, "2 kohm");
        assert!(component.params.contains("model_section=automotive"));
        assert_eq!(component.name, "R1");
    }

    #[test]
    fn hierarchy_resolution_reports_unbound_and_recursive_masters() {
        let mut workspace = ProjectWorkspace::default();
        let mut libraries = LibraryManager::default();
        workspace.ensure_library_model(&mut libraries);
        workspace
            .schematic_buffers
            .get_mut(&CellViewRef::default_top().key())
            .expect("top buffer")
            .add_library_cell_component(Point::new(20, 20), instance("missing", "unbound"));

        let unresolved = workspace.resolve_hierarchy(&libraries);
        assert_eq!(unresolved.total_instances, 2);
        assert_eq!(unresolved.resolved_instances, 1);
        assert_eq!(unresolved.unresolved_instances(), 1);
        assert_eq!(
            unresolved.bindings[1].status,
            HierarchyBindingStatus::Unresolved
        );
        assert!(unresolved.bindings[1].diagnostic.is_some());

        let top = workspace
            .schematic_buffers
            .get_mut(&CellViewRef::default_top().key())
            .expect("top buffer");
        top.components.clear();
        top.add_library_cell_component(Point::new(20, 20), instance("work", "loop"));
        let mut loop_master = SchematicState::default();
        loop_master.add_library_cell_component(Point::new(20, 20), instance("work", "loop"));
        add_schematic_master(&mut libraries, &mut workspace, "work", "loop", loop_master);

        let recursive = workspace.resolve_hierarchy(&libraries);
        assert_eq!(recursive.total_instances, 3);
        assert_eq!(recursive.resolved_instances, 2);
        let loop_row = recursive
            .bindings
            .iter()
            .find(|row| row.reference.cell == "loop")
            .expect("loop row");
        assert_eq!(loop_row.instance_count, 2);
        assert_eq!(loop_row.status, HierarchyBindingStatus::Recursive);
        assert!(
            loop_row
                .diagnostic
                .as_deref()
                .is_some_and(|diagnostic| diagnostic.contains("work/loop → work/loop"))
        );
    }

    #[test]
    fn hierarchy_resolution_projects_unsaved_active_topology() {
        let mut workspace = ProjectWorkspace::default();
        let mut libraries = LibraryManager::default();
        workspace.ensure_library_model(&mut libraries);
        let mut live = workspace
            .schematic_buffers
            .get(&CellViewRef::default_top().key())
            .expect("top buffer")
            .clone();
        live.add_library_cell_component(Point::new(20, 20), instance("missing", "live_child"));

        let persisted = workspace.resolve_hierarchy(&libraries);
        let projected =
            workspace.resolve_hierarchy_with_active(&libraries, &workspace.active_view, &live);

        assert_eq!(persisted.total_instances, 1);
        assert_eq!(projected.total_instances, 2);
        assert_eq!(projected.unresolved_instances(), 1);
        assert!(
            projected
                .bindings
                .iter()
                .any(|binding| binding.reference.cell == "live_child"
                    && binding.status == HierarchyBindingStatus::Unresolved)
        );
    }

    #[test]
    fn hierarchy_resolution_rejects_orphan_schematic_buffers() {
        let mut workspace = ProjectWorkspace::default();
        let mut libraries = LibraryManager::default();
        workspace.ensure_library_model(&mut libraries);
        workspace
            .schematic_buffers
            .get_mut(&CellViewRef::default_top().key())
            .expect("top buffer")
            .add_library_cell_component(Point::new(20, 20), instance("orphan", "amp"));
        workspace.schematic_buffers.insert(
            CellViewRef::new("orphan", "amp", "schematic").key(),
            SchematicState::default(),
        );

        let resolution = workspace.resolve_hierarchy(&libraries);

        assert_eq!(resolution.unresolved_instances(), 1);
        assert!(
            resolution
                .bindings
                .iter()
                .any(|binding| binding.reference.cell == "amp"
                    && binding.status == HierarchyBindingStatus::Unresolved)
        );
    }

    #[test]
    fn configuration_veriloga_binding_uses_exact_project_bundle_on_all_targets() {
        let mut workspace = ProjectWorkspace::default();
        let mut libraries = LibraryManager::default();
        workspace.ensure_library_model(&mut libraries);
        let reference = CellViewRef::new("models", "amp", "veriloga");
        let mut view = View::new("veriloga", ViewType::VerilogA);
        view.metadata
            .insert("veriloga.module".to_owned(), "project_amp".to_owned());
        view.metadata
            .insert("veriloga.ports".to_owned(), r#"["in","out"]"#.to_owned());
        let mut cell = Cell::new("amp");
        cell.add_view(view);
        let mut library = Library::new("models");
        library.add_cell(cell);
        libraries.add_library(library);

        let bundle = ProjectSourceBundle::try_new(
            ProjectSourceOwner::cell_view(reference.clone()),
            ProjectSourceLanguage::VerilogA,
            "models/amp.va",
            "module project_amp(input in, output out); electrical in, out; analog V(out) <+ V(in); endmodule\n",
            [],
            [],
        )
        .expect("valid project source bundle");
        let bundle_id = bundle.id();
        workspace
            .project_sources
            .insert_bundle(bundle)
            .expect("attach project source bundle");

        let mut placed = LibraryCellInstance::new("models", "amp", "schematic");
        placed.terminal_order = vec!["in".to_owned(), "out".to_owned()];
        workspace
            .schematic_buffers
            .get_mut(&CellViewRef::default_top().key())
            .expect("top buffer")
            .add_library_cell_component(Point::new(20, 20), placed);
        workspace
            .configuration_sets
            .create(crate::state::ConfigurationSetDefinition {
                name: "Mixed-signal".to_owned(),
                root: CellViewRef::default_top(),
                dut_path: "/top/X1".to_owned(),
                executable_view_policy: vec!["veriloga".to_owned()],
                stop_views: vec!["veriloga".to_owned()],
                unresolved_policy: crate::state::UnresolvedBindingPolicy::BlockNetlist,
                black_box_policy:
                    crate::state::ConfigurationBlackBoxPolicy::MaterializedSourceBoundariesOnly,
                overrides: Vec::new(),
                model_profile: crate::state::ConfigurationModelProfile::ProjectRunSetSections,
                owner: "Mixed-signal design".to_owned(),
            })
            .expect("create mixed-signal configuration");

        let active = workspace
            .active_schematic()
            .expect("active schematic")
            .clone();
        let projection = workspace
            .configuration_execution_projection(&libraries, &CellViewRef::default_top(), &active)
            .expect("resolve project-owned Verilog-A binding");
        let execution = projection
            .plan()
            .and_then(|plan| plan.binding("/top/X1"))
            .expect("exact execution binding");
        let behavioral = execution
            .project_veriloga()
            .expect("project Verilog-A contract");
        assert_eq!(behavioral.source_bundle_id(), bundle_id);
        assert_eq!(behavioral.selected_module(), "project_amp");
        assert!(behavioral.source_key().starts_with("__rspice_project__/"));
        assert_eq!(
            execution
                .materialized_binding()
                .and_then(|binding| binding.source_path.as_deref()),
            Some(Path::new(behavioral.source_key()))
        );
        assert_eq!(
            execution
                .materialized_binding()
                .and_then(|binding| binding.module_name.as_deref()),
            Some(behavioral.netlist_alias())
        );
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn hierarchy_resolution_rejects_missing_and_conflicting_source_bindings() {
        let mut workspace = ProjectWorkspace::default();
        let mut libraries = LibraryManager::default();
        workspace.ensure_library_model(&mut libraries);
        let base = std::env::temp_dir().join(format!("rspice-hierarchy-{}", Uuid::new_v4()));
        let authoritative = base.join("amp.cir");
        let conflicting = base.join("other.cir");
        std::fs::create_dir_all(&base).expect("create source fixture directory");
        std::fs::write(&authoritative, ".subckt amp in out\n.ends amp\n")
            .expect("write authoritative source");
        std::fs::write(&conflicting, ".subckt amp in out\n.ends amp\n")
            .expect("write conflicting source");

        let missing_path = base.join("missing.cir");
        let mut library = Library::new("models");
        let mut cell = Cell::new("amp");
        cell.add_view(View::new("spice", ViewType::Spice).with_path(missing_path.clone()));
        library.add_cell(cell);
        libraries.add_library(library);

        let mut binding = LibraryCellInstance::new("models", "amp", "spice");
        binding.terminal_order = vec!["in".to_owned(), "out".to_owned()];
        binding.source_path = Some(missing_path);
        workspace
            .schematic_buffers
            .get_mut(&CellViewRef::default_top().key())
            .expect("top buffer")
            .add_library_cell_component(Point::new(20, 20), binding.clone());

        let missing = workspace.resolve_hierarchy(&libraries);
        assert_eq!(missing.unresolved_instances(), 1);
        assert!(missing.bindings.iter().any(|row| {
            row.diagnostic
                .as_deref()
                .is_some_and(|diagnostic| diagnostic.contains("cannot read"))
        }));

        libraries
            .get_library_mut("models")
            .and_then(|library| library.get_cell_mut("amp"))
            .and_then(|cell| cell.get_view_mut("spice"))
            .expect("authoritative source view")
            .file_path = Some(authoritative);
        binding.source_path = Some(conflicting);
        workspace
            .schematic_buffers
            .get_mut(&CellViewRef::default_top().key())
            .expect("top buffer")
            .components
            .last_mut()
            .expect("source-backed instance")
            .library_cell = Some(binding);
        let conflicting = workspace.resolve_hierarchy(&libraries);
        assert_eq!(conflicting.unresolved_instances(), 1);
        assert!(conflicting.bindings.iter().any(|row| {
            row.diagnostic
                .as_deref()
                .is_some_and(|diagnostic| diagnostic.contains("conflicts"))
        }));

        std::fs::remove_dir_all(base).expect("remove source fixture directory");
    }

    #[test]
    fn descend_records_the_instance_names() {
        let mut workspace = ProjectWorkspace::default();
        workspace.open_as_root(reference("tb_ota"), ViewType::Schematic);
        workspace.descend_into("X1".into(), reference("ota_5t"), ViewType::Schematic);
        workspace.descend_into("XB".into(), reference("bias_2t"), ViewType::Schematic);

        assert_eq!(workspace.occurrence_labels(), ["tb_ota", "X1", "XB"]);
        assert_eq!(workspace.active_view.cell, "bias_2t");
    }

    #[test]
    fn breadcrumb_focus_truncates_the_occurrence_path() {
        let mut workspace = ProjectWorkspace::default();
        workspace.open_as_root(reference("tb_ota"), ViewType::Schematic);
        workspace.descend_into("X1".into(), reference("ota_5t"), ViewType::Schematic);
        workspace.descend_into("XB".into(), reference("bias_2t"), ViewType::Schematic);

        workspace.focus_breadcrumb(1);
        assert_eq!(workspace.occurrence_labels(), ["tb_ota", "X1"]);
        assert_eq!(workspace.active_view.cell, "ota_5t");

        workspace.ascend_one();
        assert_eq!(workspace.occurrence_labels(), ["tb_ota"]);
        assert_eq!(workspace.active_view.cell, "tb_ota");
        // At the root, ascending is a no-op.
        assert!(workspace.ascend_one().is_none());
    }

    #[test]
    fn legacy_stacks_fall_back_to_cell_names() {
        let mut workspace = ProjectWorkspace::default();
        workspace.open_as_root(reference("tb_ota"), ViewType::Schematic);
        // Simulate an older save: stack grew without instance labels.
        workspace.hierarchy_stack.push(reference("ota_5t"));
        assert_eq!(workspace.occurrence_labels(), ["tb_ota", "ota_5t"]);
    }

    #[test]
    fn symbol_active_view_does_not_allocate_schematic_buffer() {
        let reference = symbol_reference("ota_5t");
        let mut workspace = ProjectWorkspace {
            active_view: reference.clone(),
            open_views: vec![OpenCellView::new(reference.clone(), ViewType::Symbol)],
            hierarchy_stack: vec![reference.clone()],
            schematic_buffers: HashMap::new(),
            ..ProjectWorkspace::default()
        };
        let mut libraries = LibraryManager::default();
        let mut library = Library::new("work");
        let mut cell = Cell::new("ota_5t");
        cell.add_view(View::new("symbol", ViewType::Symbol));
        library.add_cell(cell);
        libraries.add_library(library);

        workspace.ensure_library_model(&mut libraries);

        assert_eq!(workspace.active_view_type(), ViewType::Symbol);
        assert!(
            !workspace.schematic_buffers.contains_key(&reference.key()),
            "symbol views must not be backed by stale schematic buffers"
        );
        let symbol_view = libraries
            .get_library("work")
            .and_then(|library| library.get_cell("ota_5t"))
            .and_then(|cell| cell.get_view("symbol"))
            .expect("symbol view still exists");
        assert_eq!(symbol_view.view_type, ViewType::Symbol);
    }

    #[test]
    fn saving_while_symbol_active_does_not_create_symbol_schematic_buffer() {
        let reference = symbol_reference("ota_5t");
        let mut workspace = ProjectWorkspace {
            active_view: reference.clone(),
            open_views: vec![OpenCellView::new(reference.clone(), ViewType::Symbol)],
            hierarchy_stack: vec![reference.clone()],
            schematic_buffers: HashMap::new(),
            ..ProjectWorkspace::default()
        };

        workspace.save_active_schematic(&SchematicState::default());

        assert!(
            !workspace.schematic_buffers.contains_key(&reference.key()),
            "session restore/save paths must not persist default schematics under symbol views"
        );
    }

    #[test]
    fn project_identity_is_stable_and_rename_is_atomic() {
        let mut project = ProjectDescriptor::default();
        let id = project.id();
        let initial_revision = project.revision();

        let renamed_revision = project
            .rename("Precision ΔΣ ADC")
            .expect("valid Unicode name");

        assert_eq!(project.id(), id);
        assert_eq!(project.name(), "Precision ΔΣ ADC");
        assert_eq!(renamed_revision.get(), initial_revision.get() + 1);
        assert_eq!(
            project.rename("Precision ΔΣ ADC").expect("no-op rename"),
            renamed_revision
        );

        let rejected = project.rename("bad/name");
        assert!(matches!(
            rejected,
            Err(ProjectDescriptorError::PathSeparator('/'))
        ));
        assert_eq!(project.name(), "Precision ΔΣ ADC");
        assert_eq!(project.revision(), renamed_revision);
        assert_eq!(project.id(), id);
    }

    #[test]
    fn legacy_project_descriptor_identity_migration_is_deterministic() {
        let original = ProjectDescriptor::default();
        let mut legacy = serde_json::to_value(&original).expect("descriptor serializes");
        legacy
            .as_object_mut()
            .expect("descriptor is an object")
            .remove("id");
        legacy
            .as_object_mut()
            .expect("descriptor is an object")
            .remove("schema_version");
        legacy
            .as_object_mut()
            .expect("descriptor is an object")
            .remove("revision");
        let legacy_json = serde_json::to_string(&legacy).expect("legacy descriptor serializes");

        let first: ProjectDescriptor =
            serde_json::from_str(&legacy_json).expect("legacy descriptor restores");
        let second: ProjectDescriptor =
            serde_json::from_str(&legacy_json).expect("legacy descriptor restores again");

        assert_eq!(first.id(), second.id());
        assert!(!first.id().as_uuid().is_nil());
        assert_ne!(first.id(), original.id());

        let persisted = serde_json::to_value(&first).expect("migrated descriptor serializes");
        assert_eq!(
            persisted.get("id"),
            Some(&serde_json::to_value(first.id()).expect("identity serializes"))
        );
    }

    #[test]
    fn versioned_or_explicitly_null_project_identity_and_schema_are_rejected() {
        let project = ProjectDescriptor::default();
        let mut missing = serde_json::to_value(&project).expect("descriptor serializes");
        missing
            .as_object_mut()
            .expect("descriptor object")
            .remove("id");
        let missing_error = serde_json::from_value::<ProjectDescriptor>(missing)
            .expect_err("versioned descriptor must retain identity");
        assert!(
            missing_error
                .to_string()
                .contains("missing its stable identity")
        );

        let mut null = serde_json::to_value(&project).expect("descriptor serializes");
        null["id"] = serde_json::Value::Null;
        let null_error = serde_json::from_value::<ProjectDescriptor>(null)
            .expect_err("explicit null identity is not legacy absence");
        assert!(
            null_error
                .to_string()
                .contains("must not be explicitly null")
        );

        let mut unversioned_null = serde_json::to_value(&project).expect("descriptor serializes");
        unversioned_null
            .as_object_mut()
            .expect("descriptor object")
            .remove("schema_version");
        unversioned_null["id"] = serde_json::Value::Null;
        let unversioned_null_error = serde_json::from_value::<ProjectDescriptor>(unversioned_null)
            .expect_err("unversioned explicit null is not genuine legacy absence");
        assert!(
            unversioned_null_error
                .to_string()
                .contains("must not be explicitly null")
        );

        let mut null_schema = serde_json::to_value(&project).expect("descriptor serializes");
        null_schema["schema_version"] = serde_json::Value::Null;
        let null_schema_error = serde_json::from_value::<ProjectDescriptor>(null_schema)
            .expect_err("explicit null schema is not an unversioned descriptor");
        assert!(
            null_schema_error
                .to_string()
                .contains("schema version must not be explicitly null")
        );
    }

    #[test]
    fn project_name_contract_counts_graphemes_and_rejects_unsafe_text() {
        let family = "👨‍👩‍👧‍👦";
        assert!(ProjectDescriptor::validate_name(&family.repeat(120)).is_ok());
        assert!(matches!(
            ProjectDescriptor::validate_name(&family.repeat(121)),
            Err(ProjectDescriptorError::NameTooLong {
                grapheme_count: 121
            })
        ));
        assert!(matches!(
            ProjectDescriptor::validate_name(" leading"),
            Err(ProjectDescriptorError::SurroundingWhitespace)
        ));
        assert!(matches!(
            ProjectDescriptor::validate_name("line\nfeed"),
            Err(ProjectDescriptorError::ControlCharacter('\n'))
        ));
        assert!(matches!(
            ProjectDescriptor::validate_name("path\\name"),
            Err(ProjectDescriptorError::PathSeparator('\\'))
        ));
    }

    #[test]
    fn cell_view_name_contract_keeps_slash_delimited_keys_injective() {
        for valid in ["user", "bandgap_2", "ΔΣ"] {
            assert!(validate_cell_view_name_segment(valid).is_ok(), "{valid}");
        }
        assert_eq!(
            validate_cell_view_name_segment(""),
            Err(CellViewNameError::Empty)
        );
        assert_eq!(
            validate_cell_view_name_segment("bad/name"),
            Err(CellViewNameError::UnsupportedCharacter('/'))
        );
        assert_eq!(
            validate_cell_view_name_segment("has space"),
            Err(CellViewNameError::UnsupportedCharacter(' '))
        );
    }

    #[test]
    fn changing_source_path_does_not_rename_an_existing_project() {
        let mut project = ProjectDescriptor::default();
        project.set_path(PathBuf::from("first-save.rspiceproj"));
        let revision = project.revision();

        assert_eq!(project.name(), "first-save");
        project.set_path(PathBuf::from("moved-copy.rspiceproj"));

        assert_eq!(project.name(), "first-save");
        assert_eq!(project.revision(), revision);
        assert_eq!(
            project.path.as_deref(),
            Some(Path::new("moved-copy.rspiceproj"))
        );
    }

    #[test]
    fn project_copy_has_independent_identity_without_rebinding_source() {
        let mut source = ProjectDescriptor::default();
        source
            .rename("Precision reference")
            .expect("source name is valid");
        source.set_path(PathBuf::from("source.rspiceproj"));
        let source_id = source.id();
        let source_revision = source.revision();
        let source_path = source.path.clone();

        let copy = source.fork_copy_at(PathBuf::from("copy.rspiceproj"));

        assert_ne!(copy.id(), source_id);
        assert_eq!(copy.revision(), ObjectRevision::INITIAL);
        assert_eq!(copy.name(), source.name());
        assert_eq!(copy.path.as_deref(), Some(Path::new("copy.rspiceproj")));
        assert_eq!(source.id(), source_id);
        assert_eq!(source.revision(), source_revision);
        assert_eq!(source.path, source_path);
    }

    #[test]
    fn generated_netlist_cannot_be_promoted_by_an_editor_write() {
        let mut workspace = ProjectWorkspace::default();

        assert!(!workspace.replace_editable_netlist_source("edited\n.end\n".to_owned()));
        assert!(workspace.netlist_source.is_none());
        assert!(!workspace.netlist_source_dirty);
        assert!(!workspace.any_dirty());
    }

    #[test]
    fn explicit_editable_copy_enters_project_dirty_lifecycle() {
        let mut workspace = ProjectWorkspace::default();
        workspace.netlist_source_path = Some(PathBuf::from("generated.sp"));

        assert!(workspace.make_netlist_editable_copy("generated\n.op\n.end\n"));
        assert_eq!(
            workspace.netlist_source.as_deref(),
            Some("generated\n.op\n.end\n")
        );
        assert!(workspace.netlist_source_path.is_none());
        assert!(workspace.netlist_source_dirty);
        assert!(workspace.any_dirty());

        workspace.mark_all_clean();
        assert!(workspace.has_editable_netlist_source());
        assert!(!workspace.netlist_source_dirty);
        assert!(!workspace.any_dirty());
    }

    #[test]
    fn editable_copy_does_not_overwrite_existing_owned_source() {
        let mut workspace = ProjectWorkspace::default();
        workspace.netlist_source = Some("owned\n.end\n".to_owned());
        workspace.netlist_source_path = Some(PathBuf::from("owned.cir"));

        assert!(!workspace.make_netlist_editable_copy("generated\n.end\n"));
        assert_eq!(workspace.netlist_source.as_deref(), Some("owned\n.end\n"));
        assert_eq!(
            workspace.netlist_source_path.as_deref(),
            Some(Path::new("owned.cir"))
        );
        assert!(!workspace.netlist_source_dirty);
    }

    #[test]
    fn editing_imported_source_preserves_its_dependency_origin() {
        let mut workspace = ProjectWorkspace::default();
        workspace.netlist_source = Some("owned\n.end\n".to_owned());
        workspace.netlist_source_path = Some(PathBuf::from("decks/owned.cir"));

        assert!(workspace.replace_editable_netlist_source("edited\n.end\n".to_owned()));
        assert_eq!(
            workspace.netlist_source_path.as_deref(),
            Some(Path::new("decks/owned.cir"))
        );
        assert!(workspace.netlist_source_dirty);
    }

    #[test]
    fn returning_to_generated_output_is_saved_as_a_project_change() {
        let mut workspace = ProjectWorkspace::default();
        workspace.netlist_source = Some("owned\n.end\n".to_owned());
        workspace.netlist_source_path = Some(PathBuf::from("owned.cir"));

        assert!(workspace.return_to_generated_netlist());
        assert!(workspace.netlist_source.is_none());
        assert!(workspace.netlist_source_path.is_none());
        assert!(workspace.netlist_source_dirty);
        assert!(workspace.any_dirty());
        assert!(!workspace.return_to_generated_netlist());
    }

    fn technology_binding_fixture() -> ProjectTechnologyBinding {
        let root = PathBuf::from(r"C:\qualified-pdk\models.lib");
        ProjectTechnologyBinding {
            schema_version: PROJECT_TECHNOLOGY_BINDING_SCHEMA_VERSION,
            package_name: "Qualified analog models".to_owned(),
            package_version: Some("2026.07".to_owned()),
            technology_node: Some("180 nm".to_owned()),
            model_library: "qualified_analog".to_owned(),
            root_source: root.clone(),
            source_closure: vec![crate::state::model_library::ModelSourcePin {
                path: root,
                digest: crate::product::ContentDigest::from_bytes([0x4a; 32]),
            }],
            source_edges: Vec::new(),
            model_count: 14,
            process_sections: vec!["ff".to_owned(), "ss".to_owned(), "tt".to_owned()],
        }
    }

    #[test]
    fn technology_attachment_is_atomic_revisioned_and_idempotent() {
        let mut project = ProjectDescriptor::default();
        let initial_revision = project.revision();
        let binding = technology_binding_fixture();

        let committed = project
            .attach_technology(binding.clone())
            .expect("valid binding commits");
        assert_eq!(committed.get(), initial_revision.get() + 1);
        assert_eq!(project.technology_binding(), Some(&binding));
        assert_eq!(
            project.technology.as_deref(),
            Some(binding.display_label().as_str())
        );
        assert_eq!(
            project
                .attach_technology(binding)
                .expect("identical binding is a no-op"),
            committed
        );

        let mut rejected = technology_binding_fixture();
        rejected.model_count = 0;
        let before = project.clone();
        assert!(matches!(
            project.attach_technology(rejected),
            Err(ProjectDescriptorError::Technology(
                TechnologyBindingError::NoModels
            ))
        ));
        assert_eq!(project.revision(), before.revision());
        assert_eq!(project.technology, before.technology);
        assert_eq!(project.technology_binding(), before.technology_binding());
    }

    #[test]
    fn attached_technology_detects_exact_catalog_drift() {
        let root = PathBuf::from(r"C:\qualified-pdk\models.lib");
        let bytes = b".model nch nmos level=1\n".to_vec();
        let digest = crate::product::ContentDigest::from_bytes(sha2::Sha256::digest(&bytes).into());
        let mut library = crate::state::model_library::ModelLibrary::new("qualified_analog")
            .with_technology("Qualified analog models", "180 nm");
        library.version = "2026.07".to_owned();
        library.root_path = Some(root.clone());
        library.source_closure = vec![crate::state::model_library::ModelSourcePin {
            path: root.clone(),
            digest,
        }];
        library.source_contents =
            vec![crate::state::model_library::ModelSourceContent { path: root, bytes }];
        library.add_model(crate::state::model_library::DeviceModel::new(
            "nch",
            crate::state::model_library::ModelType::Nmos,
        ));
        let binding = ProjectTechnologyBinding::from_model_library(&library)
            .expect("exact retained source is attachable");
        binding
            .validate_model_library(&library)
            .expect("unchanged catalog matches");

        library.version = "2026.08".to_owned();
        assert!(matches!(
            binding.validate_model_library(&library),
            Err(TechnologyBindingError::CatalogDrift { .. })
        ));
    }

    #[test]
    fn technology_binding_persists_while_runtime_dirty_state_resets() {
        let mut workspace = ProjectWorkspace::default();
        let binding = technology_binding_fixture();
        workspace
            .attach_technology(binding.clone())
            .expect("valid binding commits");
        assert!(workspace.any_dirty());

        let bytes = serde_json::to_vec(&workspace).expect("workspace serializes");
        let restored: ProjectWorkspace =
            serde_json::from_slice(&bytes).expect("workspace restores");

        assert_eq!(restored.project.technology_binding(), Some(&binding));
        restored
            .project
            .validate()
            .expect("restored binding validates");
        assert!(!restored.any_dirty());
    }

    #[test]
    fn hardcopy_page_setup_persists_and_uses_project_dirty_lifecycle() {
        use crate::hardcopy::{
            ActiveHardcopySource, HardcopyDocumentId, HardcopyDocumentKind, HardcopyScope,
            HardcopySetup, SetupSaveDisposition,
        };

        let source = ActiveHardcopySource::try_new(
            HardcopyDocumentId::try_from_uuid(uuid::Uuid::from_u128(0x4852_4450_5901))
                .expect("stable fixture identity"),
            crate::product::ObjectRevision::INITIAL,
            crate::product::ContentDigest::from_bytes([0x48; 32]),
            "top / schematic",
            HardcopyDocumentKind::SchematicOrSymbol,
            HardcopyScope::CurrentSheet,
        )
        .expect("valid hardcopy source");
        let mut workspace = ProjectWorkspace::default();

        let first = workspace
            .save_hardcopy_setup(&source, HardcopySetup::default())
            .expect("page setup commits");
        assert_eq!(first.disposition(), SetupSaveDisposition::Inserted);
        assert!(workspace.hardcopy_setups_dirty);
        assert!(workspace.any_dirty());

        let bytes = serde_json::to_vec(&workspace).expect("workspace serializes");
        let mut restored: ProjectWorkspace =
            serde_json::from_slice(&bytes).expect("workspace restores");
        assert_eq!(restored.hardcopy_setups.len(), 1);
        assert!(!restored.hardcopy_setups_dirty);
        assert!(!restored.any_dirty());

        let unchanged = restored
            .save_hardcopy_setup(&source, HardcopySetup::default())
            .expect("identical setup is accepted");
        assert_eq!(unchanged.disposition(), SetupSaveDisposition::Unchanged);
        assert!(!restored.hardcopy_setups_dirty);
        assert!(!restored.any_dirty());
    }

    #[test]
    fn project_print_mapping_routes_through_project_dirty_lifecycle() {
        let mapping = crate::hardcopy::PrintMappingTable::try_new(
            crate::hardcopy::PrintMappingSaveScope::ProjectPrintSet(
                "documentation".to_owned(),
            ),
            Vec::new(),
        )
        .unwrap();
        let mut workspace = ProjectWorkspace::default();
        let receipt = workspace
            .save_project_print_mapping(mapping.clone())
            .unwrap();
        assert_eq!(
            receipt.disposition(),
            crate::hardcopy::PrintMappingSaveDisposition::Created
        );
        assert!(workspace.project_print_mappings_dirty);
        assert!(workspace.any_dirty());

        let bytes = serde_json::to_vec(&workspace).unwrap();
        let mut restored: ProjectWorkspace = serde_json::from_slice(&bytes).unwrap();
        assert!(
            restored
                .project_print_mappings
                .get("documentation")
                .is_some()
        );
        assert!(!restored.any_dirty());

        let unchanged = restored.save_project_print_mapping(mapping).unwrap();
        assert_eq!(
            unchanged.disposition(),
            crate::hardcopy::PrintMappingSaveDisposition::Unchanged
        );
        assert!(!restored.any_dirty());
    }

    #[test]
    fn hardcopy_source_sets_persist_validate_and_use_project_dirty_lifecycle() {
        use crate::hardcopy::{HardcopyDocumentId, HardcopyDocumentKind, HardcopyScope};
        use crate::hardcopy::sources::{HardcopySourceSet, HardcopySourceSetMember};

        let member_id =
            HardcopyDocumentId::try_from_uuid(uuid::Uuid::from_u128(0x4853_4d45_4d42_4552))
                .unwrap();
        let set_id =
            HardcopyDocumentId::try_from_uuid(uuid::Uuid::from_u128(0x4853_5345_5449_4431))
                .unwrap();
        let member = HardcopySourceSetMember::try_new(
            "project:test:sheet:1",
            "Sheet 1",
            member_id,
            crate::product::ObjectRevision::INITIAL,
            crate::product::ContentDigest::from_bytes([0x51; 32]),
            HardcopyScope::CurrentSheet,
        )
        .unwrap();
        let source_set = HardcopySourceSet::try_new(
            set_id,
            crate::product::ObjectRevision::INITIAL,
            "Review set",
            HardcopyDocumentKind::SchematicOrSymbol,
            HardcopyScope::NamedPrintSet("Review set".to_owned()),
            vec![member],
        )
        .unwrap();
        let source_key = source_set.source_key();
        let mut workspace = ProjectWorkspace::default();

        assert!(workspace.save_hardcopy_source_set(source_set).unwrap());
        assert!(!workspace.hardcopy_source_sets().is_empty());
        assert!(workspace.hardcopy_source_set(&source_key).is_some());
        assert!(workspace.any_dirty());

        let bytes = serde_json::to_vec(&workspace).unwrap();
        let mut restored: ProjectWorkspace = serde_json::from_slice(&bytes).unwrap();
        restored.validate_simulation_configuration().unwrap();
        assert_eq!(restored.hardcopy_source_sets().len(), 1);
        assert!(!restored.any_dirty());
        assert!(restored.remove_hardcopy_source_set(&source_key));
        assert!(restored.hardcopy_source_sets().is_empty());
        assert!(restored.any_dirty());
    }

    #[test]
    fn hardcopy_source_set_catalog_rejects_case_folded_duplicate_names() {
        use crate::hardcopy::{HardcopyDocumentId, HardcopyDocumentKind, HardcopyScope};
        use crate::hardcopy::sources::{HardcopySourceSet, HardcopySourceSetMember};

        let build_set = |seed: u128, name: &str| {
            let member_id = HardcopyDocumentId::try_from_uuid(uuid::Uuid::from_u128(seed)).unwrap();
            let set_id =
                HardcopyDocumentId::try_from_uuid(uuid::Uuid::from_u128(seed + 0x1000)).unwrap();
            let member = HardcopySourceSetMember::try_new(
                format!("project:test:sheet:{seed}"),
                format!("Sheet {seed}"),
                member_id,
                crate::product::ObjectRevision::INITIAL,
                crate::product::ContentDigest::from_bytes([(seed & 0xff) as u8; 32]),
                HardcopyScope::CurrentSheet,
            )
            .unwrap();
            HardcopySourceSet::try_new(
                set_id,
                crate::product::ObjectRevision::INITIAL,
                name,
                HardcopyDocumentKind::SchematicOrSymbol,
                HardcopyScope::NamedPrintSet(name.to_owned()),
                vec![member],
            )
            .unwrap()
        };
        let mut workspace = ProjectWorkspace::default();
        workspace
            .save_hardcopy_source_set(build_set(0x5100, "Tapeout"))
            .unwrap();
        let error = workspace
            .save_hardcopy_source_set(build_set(0x5200, "tapeout"))
            .unwrap_err();
        assert!(matches!(
            error,
            HardcopySourceSetPersistenceError::DuplicateName { .. }
        ));
        assert_eq!(workspace.hardcopy_source_sets().len(), 1);
    }

    #[test]
    fn corrupted_persisted_technology_contract_fails_project_validation() {
        let mut project = ProjectDescriptor::default();
        project
            .attach_technology(technology_binding_fixture())
            .expect("fixture binding commits");
        let mut encoded = serde_json::to_value(&project).expect("descriptor serializes");
        encoded["technology_binding"]["root_source"] =
            serde_json::Value::String("relative/models.lib".to_owned());
        let restored: ProjectDescriptor =
            serde_json::from_value(encoded).expect("descriptor shape restores");

        assert!(matches!(
            restored.validate(),
            Err(ProjectDescriptorError::Technology(
                TechnologyBindingError::NonAbsoluteSource(_)
            ))
        ));
    }

    #[test]
    fn legacy_workspaces_restore_with_no_project_source_examples() {
        let mut value = serde_json::to_value(ProjectWorkspace::default()).unwrap();
        value.as_object_mut().unwrap().remove("project_sources");

        let restored: ProjectWorkspace = serde_json::from_value(value).unwrap();

        assert!(restored.project_sources.is_empty());
        assert!(!restored.project_sources_dirty);
    }

    #[test]
    fn only_bootstrapped_projects_receive_exact_mockup_sources() {
        let mut libraries = LibraryManager::default();
        let workspace = ProjectWorkspace::new_bootstrapped(&mut libraries);
        let verilog_a = workspace
            .project_sources
            .get(ProjectSourceLanguage::VerilogA)
            .unwrap();
        let automation = workspace
            .project_sources
            .get(ProjectSourceLanguage::RSpiceAutomation)
            .unwrap();

        assert_eq!(verilog_a.file_name(), "sensor_bridge.va");
        assert_eq!(
            verilog_a.content(),
            "`include \"constants.vams\"\nmodule sensor_bridge(out, inp, inn);\n  parameter real gain = 100.0 from (0:inf);\n  analog V(out) <+ gain * (V(inp)-V(inn));\nendmodule"
        );
        assert_eq!(automation.file_name(), "characterize.rspice");
        assert_eq!(
            automation.content(),
            "plan = project.plan(\"Lab characterization\")\nrun = plan.with_corners(\"all\").execute(target=\"local\")\nrun.require(specs=\"release\")\nrun.compare(baseline=\"main\", waveforms=True)\nrun.export([\"junit\", \"summary.json\", \"report.pdf\"])",
        );
        assert!(!workspace.any_dirty());
        assert!(ProjectWorkspace::default().project_sources.is_empty());
    }

    #[test]
    fn file_new_bootstrap_is_empty_but_keeps_a_valid_project_hierarchy() {
        let mut libraries = LibraryManager::default();
        let workspace = ProjectWorkspace::new_empty_bootstrapped(&mut libraries);

        assert!(workspace.project_sources.is_empty());
        assert!(!workspace.project_sources_dirty);
        assert!(
            libraries
                .get_library(&workspace.active_view.library)
                .and_then(|library| library.get_cell(&workspace.active_view.cell))
                .and_then(|cell| cell.get_view(&workspace.active_view.view))
                .is_some()
        );
    }

    #[test]
    fn project_source_names_are_portable_and_extensions_are_case_insensitive() {
        assert!(
            ProjectSourceDocument::try_new(
                "MODEL.VA",
                ProjectSourceLanguage::VerilogA,
                "module model; endmodule",
            )
            .is_ok()
        );
        assert!(matches!(
            ProjectSourceDocument::try_new(
                "bad\"name.va",
                ProjectSourceLanguage::VerilogA,
                "module model; endmodule",
            ),
            Err(ProjectSourceError::InvalidFileNameCharacters { .. })
        ));
        assert!(matches!(
            ProjectSourceDocument::try_new(
                "COM1.va",
                ProjectSourceLanguage::VerilogA,
                "module model; endmodule",
            ),
            Err(ProjectSourceError::ReservedFileName { .. })
        ));
    }

    #[test]
    fn project_source_payload_limit_is_enforced_before_compilation() {
        let oversized = "x".repeat(MAX_PROJECT_CODE_SOURCE_BYTES + 1);
        assert!(matches!(
            ProjectSourceDocument::try_new(
                "oversized.va",
                ProjectSourceLanguage::VerilogA,
                oversized,
            ),
            Err(ProjectSourceError::SourceTooLarge {
                bytes,
                limit: MAX_PROJECT_CODE_SOURCE_BYTES,
                ..
            }) if bytes == MAX_PROJECT_CODE_SOURCE_BYTES + 1
        ));
    }

    #[test]
    fn source_edits_preserve_exact_utf8_and_invalidate_validation_identity() {
        let mut registry =
            ProjectSourceRegistry::try_from_documents([ProjectSourceDocument::try_new(
                "sensor_bridge.va",
                ProjectSourceLanguage::VerilogA,
                "module sensor_bridge; endmodule\r\n",
            )
            .unwrap()])
            .unwrap();
        let first_identity = registry
            .mark_validated(ProjectSourceLanguage::VerilogA)
            .unwrap();
        assert!(
            registry
                .get(ProjectSourceLanguage::VerilogA)
                .unwrap()
                .validation_is_current()
        );

        let source = "module sensor_bridge; // Δ温度\nendmodule\n".to_owned();
        assert!(
            registry
                .replace_content(ProjectSourceLanguage::VerilogA, source.clone())
                .unwrap()
        );
        let edited = registry.get(ProjectSourceLanguage::VerilogA).unwrap();
        assert_eq!(edited.content(), source);
        assert_eq!(edited.revision().get(), 2);
        assert!(edited.validated_identity().is_none());
        assert_ne!(edited.content_digest(), first_identity.content_digest());
        let edited_revision = edited.revision();
        assert!(
            !registry
                .replace_content(ProjectSourceLanguage::VerilogA, source)
                .unwrap()
        );
        assert_eq!(
            registry
                .get(ProjectSourceLanguage::VerilogA)
                .unwrap()
                .revision(),
            edited_revision
        );
    }

    #[test]
    fn imported_source_replacement_is_monotonic_validated_and_atomic() {
        let mut registry =
            ProjectSourceRegistry::try_from_documents([ProjectSourceDocument::try_new(
                "first.va",
                ProjectSourceLanguage::VerilogA,
                "module first; endmodule\n",
            )
            .unwrap()])
            .unwrap();
        registry
            .mark_validated(ProjectSourceLanguage::VerilogA)
            .unwrap();

        assert!(
            registry
                .replace_imported(
                    ProjectSourceLanguage::VerilogA,
                    "second.va".to_owned(),
                    "module second; endmodule\r\n".to_owned(),
                )
                .unwrap()
        );
        let imported = registry.get(ProjectSourceLanguage::VerilogA).unwrap();
        assert_eq!(imported.file_name(), "second.va");
        assert_eq!(imported.content(), "module second; endmodule\r\n");
        assert_eq!(imported.revision().get(), 2);
        assert!(imported.validated_identity().is_none());

        let before = registry.clone();
        assert!(matches!(
            registry.replace_imported(
                ProjectSourceLanguage::VerilogA,
                "wrong.txt".to_owned(),
                "module wrong; endmodule\n".to_owned(),
            ),
            Err(ProjectSourceError::InvalidFileNameExtension { .. })
        ));
        assert_eq!(registry, before);
    }

    #[test]
    fn workspace_source_dirty_state_tracks_edits_validation_and_cleaning() {
        let mut libraries = LibraryManager::default();
        let mut workspace = ProjectWorkspace::new_bootstrapped(&mut libraries);

        workspace
            .replace_project_source(
                ProjectSourceLanguage::RSpiceAutomation,
                "plan = project.plan(\"Unicode Δ\")".to_owned(),
            )
            .unwrap();
        assert!(workspace.project_sources_dirty);
        assert!(workspace.any_dirty());
        workspace.mark_project_sources_clean();
        assert!(!workspace.any_dirty());

        let identity = workspace
            .mark_project_source_validated(ProjectSourceLanguage::RSpiceAutomation)
            .unwrap();
        assert!(workspace.project_sources_dirty);
        assert_eq!(
            workspace
                .project_sources
                .get(ProjectSourceLanguage::RSpiceAutomation)
                .unwrap()
                .validated_identity(),
            Some(identity)
        );
        workspace.mark_all_clean();
        assert!(!workspace.any_dirty());

        let repeated = workspace
            .mark_project_source_validated(ProjectSourceLanguage::RSpiceAutomation)
            .unwrap();
        assert_eq!(repeated, identity);
        assert!(!workspace.any_dirty());
    }

    #[test]
    fn project_source_validation_rejects_mismatched_slots_and_stale_evidence() {
        let document = ProjectSourceDocument::try_new(
            "sensor_bridge.va",
            ProjectSourceLanguage::VerilogA,
            "module sensor_bridge; endmodule",
        )
        .unwrap();
        let mut registry = ProjectSourceRegistry::try_from_documents([document]).unwrap();
        registry
            .mark_validated(ProjectSourceLanguage::VerilogA)
            .unwrap();
        let mut value = serde_json::to_value(&registry).unwrap();
        value["bundles"][0]["root"]["content"] = serde_json::Value::String("changed".to_owned());
        assert!(serde_json::from_value::<ProjectSourceRegistry>(value).is_err());

        let root = serde_json::to_value(
            registry
                .get(ProjectSourceLanguage::VerilogA)
                .expect("fixture root exists"),
        )
        .unwrap();
        let mut legacy = serde_json::json!({ "verilog_a": root });
        legacy["verilog_a"]["language"] = serde_json::Value::String("rspice-automation".to_owned());
        assert!(serde_json::from_value::<ProjectSourceRegistry>(legacy).is_err());
    }
}
