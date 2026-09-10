//! Project workspace state.
//!
//! This module is the product-level design spine for RSpice Studio. It keeps
//! project identity, open Library/Cell/View documents, active hierarchy
//! breadcrumbs, and per-view schematic buffers together instead of letting the
//! workbench, library browser, and single schematic buffer drift apart.

use std::collections::{BTreeMap, HashMap, HashSet};
use std::path::{Path, PathBuf};

mod capture_group;
mod design_intent;
mod design_projection;
mod document_occurrence;
mod hierarchy;
mod materialize;
mod netlist_profile;
#[cfg(test)]
mod netlist_profile_tests;
mod open_documents;
mod plan_data;
mod project_descriptor;
mod project_library_publication;
mod reference_changes;
mod reference_preparation;
mod saved_output;

// The two functions are renamed on export: bare `normalize` and
// `collation_key` say nothing about what they normalize outside their module,
// and analysis names have functions by exactly those names.
pub use capture_group::{
    CaptureGroup, CaptureGroupError, CaptureGroupMembership, CaptureGroupRule, MembershipMove,
    UNGROUPED_NAME, collation_key as capture_group_collation_key, group_namer,
    normalize_name as normalize_capture_group_name,
};
pub use design_intent::*;
pub use design_projection::*;
pub use document_occurrence::*;
pub use hierarchy::*;
pub use netlist_profile::NetlistExecutionProfile;
pub use project_descriptor::*;
pub use project_library_publication::*;
pub(crate) use reference_changes::{PreparedReferences, ReferenceChanges};
pub(crate) use reference_preparation::{SchematicReferenceTransaction, reference_from_key};
pub(crate) use saved_output::{raw_probe_unit, saved_output_references, validate_raw_probe};
// The glob is crate-private: `materialize` is `pub(super)` throughout except
// the one binding lookup two workbench surfaces reach by path, the terminal
// contract netlist generation compares against, and the metadata lookup the
// Models & PDKs symbol-contract table reads a declared family with.
use materialize::*;
pub(crate) use materialize::{
    metadata_value, project_veriloga_binding_for_view, same_terminal_contract,
};

pub use saved_output::{
    ComplexExpressionPolicy, OutputSelectionMode, SavedOutput, SavedOutputCompatibility,
    SavedOutputDisplayIntent, SavedOutputKind, SavedOutputOrigin, SavedOutputPolicy,
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
    AnalysisInstanceId, CaptureGroupId, ContentDigest, DesignVariableId, ObjectRevision, ProjectId,
    ResultDocumentId, RevisionError, RunId, SavedOutputId, SimulationPlanId, SpecificationId,
};
use crate::state::{
    AnalysisResultPvtPoint, AnalysisResultSourceDomain, Cell, ComponentType, InstancePath, Library,
    LibraryCellInstance, LibraryManager, SchematicState, View, ViewType,
    validate_builtin_xspice_binding, validate_generated_veriloga_binding,
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
/// Defensive bound on project-owned result documents. Documents themselves
/// carry independent limits for panes, traces, retained samples, and history.
pub const MAX_PROJECT_VISUALIZATION_DOCUMENTS: usize = 1_024;
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
    instance_path: InstancePath,
    resolved_reference: CellViewRef,
    resolved_view_type: ViewType,
    materialized_binding: Option<LibraryCellInstance>,
    model_section: Option<String>,
    stop_boundary: bool,
    project_veriloga: Option<ConfigurationVerilogABinding>,
    /// Digest over everything this occurrence and its whole descendant subtree
    /// resolve to. Two occurrences whose subtrees resolve identically carry
    /// equal digests and therefore share one emitted master; two that differ
    /// anywhere below them do not.
    binding_closure_digest: ContentDigest,
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
    pub const fn instance_path(&self) -> &InstancePath {
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

    /// Digest over this occurrence's whole resolved subtree. Two occurrences
    /// carry the same digest exactly when they instantiate the same master —
    /// which is what [`ConfigurationExecutionPlan::occurrence_master`] answers
    /// without re-deriving it.
    pub const fn binding_closure_digest(&self) -> ContentDigest {
        self.binding_closure_digest
    }
}

/// One open view tab in the workspace.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenCellView {
    pub reference: CellViewRef,
    pub view_type: ViewType,
    pub dirty: bool,
    /// The exact occurrence this document is editing. Its terminal master is
    /// always `reference`; saves written before documents owned an occurrence
    /// deserialize unrooted and are rooted at `reference` on load.
    #[serde(default)]
    pub occurrence: DocumentOccurrence,
    /// Whether this document was opened as a read-only hierarchy reference.
    /// The marking belongs to the tab, so returning to it later still refuses
    /// writes.
    #[serde(default)]
    pub read_only_reference: bool,
}

impl OpenCellView {
    pub fn new(reference: CellViewRef, view_type: ViewType) -> Self {
        Self {
            occurrence: DocumentOccurrence::rooted(reference.clone()),
            reference,
            view_type,
            dirty: false,
            read_only_reference: false,
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
    #[error("project hardcopy source-set catalog is invalid: {message}")]
    InvalidHardcopySourceSetCatalog { message: String },
    #[error("project hardcopy receipt ledger is invalid: {message}")]
    InvalidHardcopyReceiptLedger { message: String },
    #[error("project signed-PDK callback receipt ledger is invalid: {message}")]
    InvalidPdkCallbackReceiptLedger { message: String },
    #[error("project physical-layout document catalog is invalid: {message}")]
    InvalidPhysicalLayoutCatalog { message: String },
    #[error("report_documents[{index}] is invalid: {message}")]
    InvalidReportDocument { index: usize, message: String },
    #[error("report document identity {document_id} is duplicated")]
    DuplicateReportDocumentIdentity { document_id: ResultDocumentId },
    #[error("visualization_documents[{index}] is invalid: {message}")]
    InvalidVisualizationDocument { index: usize, message: String },
    #[error("visualization document identity {document_id} is duplicated")]
    DuplicateVisualizationDocumentIdentity { document_id: ResultDocumentId },
    #[error(
        "visualization document title {title:?} is duplicated by entries {first_index} and {index}"
    )]
    DuplicateVisualizationDocumentTitle {
        title: String,
        first_index: usize,
        index: usize,
    },
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
    /// Every way a capture group can be refused, as that module states them.
    #[error("simulation plan {plan_id}: {source}")]
    CaptureGroup {
        plan_id: SimulationPlanId,
        #[source]
        source: CaptureGroupError,
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
        "simulation_plan_payloads[{plan_id}].specification_definitions[{index}] is invalid: {message}"
    )]
    InvalidSpecificationDefinition {
        plan_id: SimulationPlanId,
        index: usize,
        message: String,
    },
    #[error("specification identity {id} is reused by plans {first_plan_id} and {plan_id}")]
    DuplicateSpecificationIdentity {
        id: SpecificationId,
        first_plan_id: SimulationPlanId,
        plan_id: SimulationPlanId,
    },
    #[error(
        "simulation_plan_payloads[{plan_id}].specification_definitions[{index}] duplicates the case-insensitive requirement key of entry {first_index}"
    )]
    DuplicateSpecificationRequirementKey {
        plan_id: SimulationPlanId,
        index: usize,
        first_index: usize,
    },
    #[error("simulation_plan_payloads[{plan_id}].specification_policy is invalid: {message}")]
    InvalidSpecificationPolicy {
        plan_id: SimulationPlanId,
        message: String,
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
    #[error("simulation plan {plan_id} has no saved output with identity {output_id}")]
    SavedOutputNotFound {
        plan_id: SimulationPlanId,
        output_id: SavedOutputId,
    },
    #[error(
        "saved output {output_id} in simulation plan {plan_id} could not advance its revision: {source}"
    )]
    SavedOutputRevision {
        plan_id: SimulationPlanId,
        output_id: SavedOutputId,
        #[source]
        source: RevisionError,
    },
    #[error("simulation plan {plan_id} has no configuration payload")]
    PlanPayloadMissing { plan_id: SimulationPlanId },
    #[error("simulation plan {plan_id} already has a configuration payload")]
    PlanPayloadAlreadyExists { plan_id: SimulationPlanId },
    #[error("cloned plan payload has no destination mapping for source analysis {analysis_id}")]
    MissingClonedAnalysisMapping { analysis_id: AnalysisInstanceId },
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum VisualizationDocumentPersistenceError {
    #[error("the project already contains the visualization document identity {document_id}")]
    DuplicateIdentity { document_id: ResultDocumentId },
    #[error("the project already contains a result document named {title:?}")]
    DuplicateTitle { title: String },
    #[error(
        "the project already contains the supported limit of {MAX_PROJECT_VISUALIZATION_DOCUMENTS} result documents"
    )]
    CatalogFull,
    #[error("the visualization document is invalid: {message}")]
    Invalid { message: String },
    #[error("the project does not contain visualization document {document_id}")]
    NotFound { document_id: ResultDocumentId },
    #[error("visualization document {document_id} transaction failed: {message}")]
    Transaction {
        document_id: ResultDocumentId,
        message: String,
    },
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

pub(crate) struct PreparedPhysicalLayoutCatalog {
    documents: BTreeMap<String, crate::state::PhysicalLayoutDocument>,
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
    /// Named capture policies over the saved outputs. Empty is the state every
    /// project written before this model loads in, and it means exactly one
    /// thing: every output belongs to the synthesized fallback group, which
    /// overrides nothing. So an old project's forecast and its execution are
    /// unchanged by the field's arrival.
    #[serde(default)]
    pub capture_groups: Vec<CaptureGroup>,
    #[serde(default)]
    pub specs: Vec<SpecEntry>,
    /// Governed specification records. Empty means the project predates this
    /// model and is deterministically migrated from `specs` on first access.
    #[serde(default)]
    pub specification_definitions: Vec<SpecificationDefinition>,
    #[serde(default)]
    pub specification_policy: SpecificationPolicy,
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
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OwnedNetlistSaveRecord {
    pub document_revision: u64,
    pub content_digest: crate::product::ContentDigest,
    pub message: String,
}

pub const MAX_OWNED_NETLIST_HISTORY_REVISIONS: usize = 64;
pub const MAX_OWNED_NETLIST_HISTORY_BYTES: usize = 16 * 1024 * 1024;

/// Content-complete, bounded revision evidence for a project-owned deck.
/// Dependency bytes are retained with the authored root so compare, revert,
/// recovery, and merge never have to reopen an ambient filesystem path.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OwnedNetlistRevisionSnapshot {
    pub document_revision: u64,
    pub content_digest: crate::product::ContentDigest,
    pub source: String,
    #[serde(default)]
    pub dependencies: Vec<crate::state::DependencyMetadata>,
    /// Exact include-document ownership catalog at this revision. Restoring a
    /// root snapshot restores ownership and dependency bytes together so no
    /// stale editable authority can survive across history boundaries.
    #[serde(default)]
    pub owned_includes: Vec<OwnedNetlistIncludeDescriptor>,
    pub message: String,
    #[serde(default)]
    pub source_encoding: NetlistTextEncoding,
    #[serde(default)]
    pub source_line_ending: NetlistLineEnding,
}

/// Stable project ownership for one dependency document retained by an owned
/// netlist. The source bytes remain canonical in `NetlistDocument` so
/// execution, editor, history, and archive export cannot diverge.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OwnedNetlistIncludeDescriptor {
    pub document_id: Uuid,
    pub logical_identity: String,
    pub display_name: String,
    pub revision: u64,
    pub content_digest: crate::product::ContentDigest,
}

impl OwnedNetlistIncludeDescriptor {
    pub fn try_new(dependency: &crate::state::DependencyMetadata) -> Result<Self, String> {
        let source = dependency.source().ok_or_else(|| {
            "Only a resolved dependency can be copied into the project.".to_owned()
        })?;
        let value = Self {
            document_id: Uuid::new_v4(),
            logical_identity: dependency.locator().logical_identity().to_owned(),
            display_name: dependency.locator().display_name().to_owned(),
            revision: 1,
            content_digest: crate::state::content_digest(source),
        };
        value.validate()?;
        Ok(value)
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.document_id.is_nil() {
            return Err("owned include document identity cannot be nil".to_owned());
        }
        if self.logical_identity.trim().is_empty()
            || self.logical_identity != self.logical_identity.trim()
            || self.logical_identity.len() > 4_096
            || self.logical_identity.chars().any(char::is_control)
        {
            return Err("owned include logical identity is invalid".to_owned());
        }
        if self.display_name.trim().is_empty()
            || self.display_name != self.display_name.trim()
            || self.display_name.len() > 4_096
            || self.display_name.chars().any(char::is_control)
            || self.revision == 0
        {
            return Err("owned include display name or revision is invalid".to_owned());
        }
        Ok(())
    }
}

impl OwnedNetlistRevisionSnapshot {
    pub fn from_document(
        document: &crate::state::NetlistDocument,
        message: impl Into<String>,
        source_encoding: NetlistTextEncoding,
        source_line_ending: NetlistLineEnding,
    ) -> Result<Self, String> {
        let snapshot = Self {
            document_revision: document.revision().get(),
            content_digest: document.content_digest(),
            source: document.source().to_owned(),
            dependencies: document.dependencies().to_vec(),
            owned_includes: Vec::new(),
            message: message.into(),
            source_encoding,
            source_line_ending,
        };
        snapshot.validate()?;
        Ok(snapshot)
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.document_revision == 0 {
            return Err("owned netlist history revision must be non-zero".to_owned());
        }
        if self.content_digest != crate::state::content_digest(&self.source) {
            return Err("owned netlist history digest does not identify its source".to_owned());
        }
        if self.message.trim().is_empty()
            || self.message != self.message.trim()
            || self.message.chars().count() > 240
            || self.message.chars().any(char::is_control)
        {
            return Err("owned netlist history message is invalid".to_owned());
        }
        let mut document_ids = HashSet::new();
        let mut identities = HashSet::new();
        for include in &self.owned_includes {
            include.validate()?;
            if !document_ids.insert(include.document_id)
                || !identities.insert(include.logical_identity.as_str())
            {
                return Err("owned netlist history repeats an include identity".to_owned());
            }
            let dependency = self
                .dependencies
                .iter()
                .find(|dependency| {
                    dependency.locator().logical_identity() == include.logical_identity
                })
                .ok_or_else(|| {
                    format!(
                        "owned netlist history include '{}' is absent from its dependency closure",
                        include.logical_identity
                    )
                })?;
            let source = dependency.source().ok_or_else(|| {
                format!(
                    "owned netlist history include '{}' has no retained source",
                    include.logical_identity
                )
            })?;
            if include.content_digest != crate::state::content_digest(source) {
                return Err(format!(
                    "owned netlist history include '{}' digest does not identify its retained source",
                    include.logical_identity
                ));
            }
        }
        Ok(())
    }

    #[must_use]
    pub fn retained_bytes(&self) -> usize {
        self.source.len()
            + self.message.len()
            + self
                .dependencies
                .iter()
                .filter_map(crate::state::DependencyMetadata::source_bytes)
                .map(<[u8]>::len)
                .sum::<usize>()
            + self
                .owned_includes
                .iter()
                .map(|include| include.logical_identity.len() + include.display_name.len() + 80)
                .sum::<usize>()
    }
}

/// Encoding used by a project-owned netlist at its durable file boundary.
///
/// The editor and parser operate on Rust UTF-8 strings, but an imported deck
/// can legitimately be UTF-8 with a BOM, UTF-16, or legacy ISO-8859-1. The
/// encoding is therefore retained as project metadata and reapplied on Save;
/// RSpice never silently converts an imported source file.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub enum NetlistTextEncoding {
    #[default]
    Utf8,
    Utf8Bom,
    Utf16LeBom,
    Utf16BeBom,
    Latin1,
}

impl NetlistTextEncoding {
    pub const ALL: [Self; 5] = [
        Self::Utf8,
        Self::Utf8Bom,
        Self::Utf16LeBom,
        Self::Utf16BeBom,
        Self::Latin1,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Utf8 => "UTF-8",
            Self::Utf8Bom => "UTF-8 with BOM",
            Self::Utf16LeBom => "UTF-16 LE with BOM",
            Self::Utf16BeBom => "UTF-16 BE with BOM",
            Self::Latin1 => "ISO-8859-1",
        }
    }

    pub fn encode(self, source: &str) -> Result<Vec<u8>, String> {
        match self {
            Self::Utf8 => Ok(source.as_bytes().to_vec()),
            Self::Utf8Bom => {
                let mut bytes = Vec::with_capacity(source.len().saturating_add(3));
                bytes.extend_from_slice(&[0xef, 0xbb, 0xbf]);
                bytes.extend_from_slice(source.as_bytes());
                Ok(bytes)
            }
            Self::Utf16LeBom | Self::Utf16BeBom => {
                let mut bytes =
                    Vec::with_capacity(source.len().saturating_mul(2).saturating_add(2));
                bytes.extend_from_slice(if self == Self::Utf16LeBom {
                    &[0xff, 0xfe]
                } else {
                    &[0xfe, 0xff]
                });
                for unit in source.encode_utf16() {
                    let encoded = if self == Self::Utf16LeBom {
                        unit.to_le_bytes()
                    } else {
                        unit.to_be_bytes()
                    };
                    bytes.extend_from_slice(&encoded);
                }
                Ok(bytes)
            }
            Self::Latin1 => {
                let mut bytes = Vec::with_capacity(source.chars().count());
                for (character_index, character) in source.chars().enumerate() {
                    let value = u32::from(character);
                    if value > u32::from(u8::MAX) {
                        return Err(format!(
                            "character {} (U+{value:04X}) cannot be represented in ISO-8859-1; use Save As with UTF-8 or remove the character",
                            character_index + 1
                        ));
                    }
                    bytes.push(value as u8);
                }
                Ok(bytes)
            }
        }
    }
}

/// Line-ending form observed in the imported source. Source text retains its
/// exact separators; this value is durable evidence for the editor status and
/// import review rather than a request to rewrite the deck.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub enum NetlistLineEnding {
    #[default]
    None,
    Lf,
    Crlf,
    Cr,
    Mixed,
}

impl NetlistLineEnding {
    pub const fn label(self) -> &'static str {
        match self {
            Self::None => "no line terminators",
            Self::Lf => "LF",
            Self::Crlf => "CRLF",
            Self::Cr => "CR",
            Self::Mixed => "mixed",
        }
    }

    pub fn detect(source: &str) -> Self {
        let bytes = source.as_bytes();
        let mut lf = 0usize;
        let mut crlf = 0usize;
        let mut cr = 0usize;
        let mut index = 0usize;
        while index < bytes.len() {
            match bytes[index] {
                b'\r' if bytes.get(index + 1) == Some(&b'\n') => {
                    crlf += 1;
                    index += 2;
                }
                b'\r' => {
                    cr += 1;
                    index += 1;
                }
                b'\n' => {
                    lf += 1;
                    index += 1;
                }
                _ => index += 1,
            }
        }
        match (lf > 0, crlf > 0, cr > 0) {
            (false, false, false) => Self::None,
            (true, false, false) => Self::Lf,
            (false, true, false) => Self::Crlf,
            (false, false, true) => Self::Cr,
            _ => Self::Mixed,
        }
    }
}

/// Declared source dialect retained with an imported owned deck. Detection is
/// advisory; a non-native dialect requires an explicit compatibility review
/// before the staged import can commit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub enum NetlistSourceDialect {
    #[default]
    RSpice,
    Spice3Ngspice,
    Hspice,
    Pspice,
    Spectre,
    Ads,
    Unknown,
}

impl NetlistSourceDialect {
    pub const ALL: [Self; 7] = [
        Self::RSpice,
        Self::Spice3Ngspice,
        Self::Hspice,
        Self::Pspice,
        Self::Spectre,
        Self::Ads,
        Self::Unknown,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::RSpice => "RSpice canonical SPICE",
            Self::Spice3Ngspice => "SPICE3 / ngspice",
            Self::Hspice => "HSPICE",
            Self::Pspice => "PSpice",
            Self::Spectre => "Cadence Spectre",
            Self::Ads => "Keysight ADS netlist",
            Self::Unknown => "Unknown SPICE-family dialect",
        }
    }

    pub const fn requires_compatibility_review(self) -> bool {
        !matches!(self, Self::RSpice)
    }

    /// Exact executable semantics qualified for this source dialect.
    ///
    /// This is intentionally fallible. A display label or a completed review
    /// must never manufacture an execution adapter for a vendor dialect.
    pub const fn execution_profile(self) -> Option<NetlistExecutionProfile> {
        match self {
            Self::RSpice => Some(NetlistExecutionProfile::RSpiceCanonicalV1),
            Self::Spice3Ngspice => Some(NetlistExecutionProfile::Spice3NgspiceV2),
            Self::Hspice => Some(NetlistExecutionProfile::HspiceDeclarativeV1),
            Self::Pspice => Some(NetlistExecutionProfile::PspiceDeclarativeV2),
            Self::Spectre => Some(NetlistExecutionProfile::SpectreSpiceV1),
            Self::Ads => Some(NetlistExecutionProfile::AdsSpiceExportV1),
            Self::Unknown => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OwnedNetlistDescriptor {
    /// Stable project identity for this top-level deck. This identity follows
    /// the deck across logical rename/move operations and remains distinct when
    /// a deck is duplicated.
    #[serde(default)]
    pub deck_id: Uuid,
    pub artifact_name: String,
    pub strategy: OwnedNetlistEditStrategy,
    #[serde(default)]
    pub source_encoding: NetlistTextEncoding,
    #[serde(default)]
    pub source_line_ending: NetlistLineEnding,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub imported_dialect: Option<NetlistSourceDialect>,
    #[serde(default)]
    pub compatibility_reviewed: bool,
    /// Exact reviewed execution semantics. Legacy canonical projects may omit
    /// this field; a non-canonical dialect without an exact profile is always
    /// rejected at preflight and must be reviewed again.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub execution_profile: Option<NetlistExecutionProfile>,
    /// SHA-256 of the last imported or successfully published raw file. This
    /// is the compare-and-exchange baseline used by ordinary Save so an
    /// external edit is never overwritten silently.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub external_file_sha256: Option<[u8; 32]>,
    #[serde(default)]
    pub save_history: Vec<OwnedNetlistSaveRecord>,
    /// Exact project-persisted source/dependency snapshots. This is separate
    /// from the compact save ledger because recovery may retain an unsaved
    /// pre-restore working revision as well as externally published states.
    #[serde(default)]
    pub revision_history: Vec<OwnedNetlistRevisionSnapshot>,
    /// Explicit copy-to-project ownership for dependency documents. Any
    /// retained dependency absent from this list remains find-only/read-only.
    #[serde(default)]
    pub owned_includes: Vec<OwnedNetlistIncludeDescriptor>,
}

/// One inactive, project-owned top-level deck. The active deck continues to
/// use the long-standing `netlist_*` workspace fields as the single execution
/// authority; this catalog retains complete inactive documents so switching
/// decks is an atomic swap rather than a lossy import.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RetainedOwnedNetlistDeck {
    pub descriptor: OwnedNetlistDescriptor,
    pub document: crate::state::NetlistDocument,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_path: Option<PathBuf>,
}

impl OwnedNetlistDescriptor {
    /// Whether this source is intentionally retained but cannot execute until
    /// its exact current bytes receive a versioned compatibility receipt.
    #[must_use]
    pub fn execution_profile_review_required(&self) -> bool {
        let dialect = self
            .imported_dialect
            .unwrap_or(NetlistSourceDialect::RSpice);
        dialect.requires_compatibility_review()
            && (!self.compatibility_reviewed
                || self.execution_profile != dialect.execution_profile()
                || self.execution_profile.is_none())
    }

    #[must_use]
    pub fn owned_include(&self, logical_identity: &str) -> Option<&OwnedNetlistIncludeDescriptor> {
        self.owned_includes
            .iter()
            .find(|include| include.logical_identity == logical_identity)
    }

    pub fn retain_revision(
        &mut self,
        document: &crate::state::NetlistDocument,
        message: impl Into<String>,
    ) -> Result<(), String> {
        let mut snapshot = OwnedNetlistRevisionSnapshot::from_document(
            document,
            message,
            self.source_encoding,
            self.source_line_ending,
        )?;
        snapshot.owned_includes.clone_from(&self.owned_includes);
        snapshot.validate()?;
        if let Some(last) = self.revision_history.last() {
            if last.document_revision == snapshot.document_revision
                && last.content_digest == snapshot.content_digest
            {
                return Ok(());
            }
            if last.document_revision >= snapshot.document_revision {
                return Err(
                    "owned netlist history cannot append a non-monotonic revision".to_owned(),
                );
            }
        }
        if snapshot.retained_bytes() > MAX_OWNED_NETLIST_HISTORY_BYTES {
            return Err("owned netlist revision exceeds the bounded history size".to_owned());
        }
        let mut next = self.revision_history.clone();
        next.push(snapshot);
        let mut retained_bytes = next
            .iter()
            .map(OwnedNetlistRevisionSnapshot::retained_bytes)
            .sum::<usize>();
        while next.len() > MAX_OWNED_NETLIST_HISTORY_REVISIONS
            || (retained_bytes > MAX_OWNED_NETLIST_HISTORY_BYTES && next.len() > 1)
        {
            retained_bytes = retained_bytes.saturating_sub(next[0].retained_bytes());
            next.remove(0);
        }
        self.revision_history = next;
        Ok(())
    }
}

/// Validate one portable, project-relative top-deck path. Forward slashes are
/// the persisted separator on every platform; native publication paths remain
/// separate in `netlist_source_path`.
pub fn validate_owned_netlist_artifact_path(path: &str) -> Result<(), String> {
    if path.is_empty()
        || path != path.trim()
        || path.len() > 4_096
        || path.chars().any(char::is_control)
        || path.contains('\\')
        || path.starts_with('/')
        || path.ends_with('/')
        || path
            .split('/')
            .any(|part| part.is_empty() || matches!(part, "." | ".."))
        || path.contains(':')
    {
        return Err(
            "owned top-deck path must be a trimmed, portable project-relative path".to_owned(),
        );
    }
    Ok(())
}

fn validate_owned_netlist_projection(
    document: &crate::state::NetlistDocument,
    descriptor: &OwnedNetlistDescriptor,
    source: &str,
) -> Result<(), String> {
    if descriptor.deck_id.is_nil() {
        return Err("owned top-deck identity cannot be nil".to_owned());
    }
    validate_owned_netlist_artifact_path(&descriptor.artifact_name)?;
    if document.ownership() == crate::state::DocumentOwnership::Generated {
        return Err("project-owned netlist document cannot have generated ownership".to_owned());
    }
    if document.source() != source {
        return Err("canonical document bytes differ from the owned source projection".to_owned());
    }

    let declared_dialect = descriptor
        .imported_dialect
        .unwrap_or(NetlistSourceDialect::RSpice);
    let expected_profile = declared_dialect.execution_profile();
    if descriptor.execution_profile.is_some() && descriptor.execution_profile != expected_profile {
        return Err(format!(
            "owned netlist dialect {} does not match its recorded execution profile",
            declared_dialect.label()
        ));
    }
    if declared_dialect.requires_compatibility_review() {
        let reviewed = descriptor.compatibility_reviewed
            && descriptor.execution_profile == expected_profile
            && expected_profile.is_some();
        let quarantined =
            !descriptor.compatibility_reviewed && descriptor.execution_profile.is_none();
        if !reviewed && !quarantined {
            return Err(format!(
                "owned non-canonical netlist dialect {} has neither an exact reviewed executable profile nor a fail-closed quarantine",
                declared_dialect.label()
            ));
        }
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
                "owned source save history is not strictly revision ordered or has an invalid message"
                    .to_owned(),
            );
        }
        previous_revision = record.document_revision;
    }
    if descriptor.revision_history.len() > MAX_OWNED_NETLIST_HISTORY_REVISIONS {
        return Err("owned source revision history exceeds its bounded entry limit".to_owned());
    }
    previous_revision = 0;
    let mut retained_bytes = 0_usize;
    for snapshot in &descriptor.revision_history {
        snapshot.validate()?;
        if snapshot.document_revision <= previous_revision
            || snapshot.document_revision > document.revision().get()
        {
            return Err(
                "owned source revision history is not strictly revision ordered".to_owned(),
            );
        }
        retained_bytes = retained_bytes
            .checked_add(snapshot.retained_bytes())
            .ok_or_else(|| "owned source revision history size overflowed".to_owned())?;
        previous_revision = snapshot.document_revision;
    }
    if retained_bytes > MAX_OWNED_NETLIST_HISTORY_BYTES {
        return Err("owned source revision history exceeds its bounded byte limit".to_owned());
    }
    if descriptor.owned_includes.len() > crate::state::MAX_PROJECT_SOURCE_FILES {
        return Err("owned include catalog exceeds the project file limit".to_owned());
    }
    let mut include_ids = HashSet::new();
    let mut include_identities = HashSet::new();
    for include in &descriptor.owned_includes {
        include.validate()?;
        if !include_ids.insert(include.document_id)
            || !include_identities.insert(include.logical_identity.as_str())
        {
            return Err("owned include identities must be unique".to_owned());
        }
        let dependency = document
            .dependencies()
            .iter()
            .find(|dependency| dependency.locator().logical_identity() == include.logical_identity)
            .ok_or_else(|| {
                format!(
                    "owned include '{}' is absent from the canonical dependency closure",
                    include.logical_identity
                )
            })?;
        let dependency_source = dependency.source().ok_or_else(|| {
            format!(
                "owned include '{}' has no retained source bytes",
                include.logical_identity
            )
        })?;
        if include.content_digest != crate::state::content_digest(dependency_source) {
            return Err(format!(
                "owned include '{}' digest does not identify its retained bytes",
                include.logical_identity
            ));
        }
    }
    Ok(())
}

// The source bundle API, re-exported from its historical workspace path. This
// block used to carry the whole `project_sources` surface "so downstream
// integrations keep compiling" -- there are no downstream integrations; the
// crate is the application. What is left is what `state::workspace` callers
// actually name through this path.
#[cfg(test)]
pub use super::project_sources::{MAX_PROJECT_CODE_SOURCE_BYTES, ProjectSourceBundle};
pub use super::project_sources::{
    ProjectSourceDocument, ProjectSourceError, ProjectSourceId, ProjectSourceLanguage,
    ProjectSourceOwner, ProjectSourceRegistry, ProjectSourceValidationIdentity,
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
    /// Project-owned bus-width and global-net policy. Older projects migrate
    /// to strict fail-closed defaults instead of inheriting UI state.
    #[serde(default)]
    pub connectivity: crate::state::ConnectivityContract,
    pub active_view: CellViewRef,
    pub open_views: Vec<OpenCellView>,
    /// Masters on the active document's occurrence, outermost first — a
    /// runtime projection of `open_views[active].occurrence`, never the place
    /// it is stored. Deserialized so a save written before documents owned
    /// their occurrence can be folded onto the document it described, and
    /// never serialized back.
    #[serde(default, skip_serializing)]
    pub hierarchy_stack: Vec<CellViewRef>,
    /// Instance names on the same occurrence, aligned with
    /// `hierarchy_stack[1..]`, and a projection for the same reason.
    #[serde(default, skip_serializing)]
    pub hierarchy_instances: Vec<String>,
    pub schematic_buffers: HashMap<String, SchematicState>,
    /// Last design projection handed out, retained while every input it was
    /// derived from stands still. Derived state, so a restored project
    /// rebuilds it on first demand instead of trusting a persisted copy.
    #[serde(skip)]
    design_projection_cache: std::cell::RefCell<Option<std::sync::Arc<DesignProjection>>>,
    /// Design-management materialization of each cell view, retained per
    /// source document so one edit re-materializes one cell view rather than
    /// the whole design. Derived state for the same reason.
    #[serde(skip)]
    materialized_buffers:
        std::cell::RefCell<HashMap<String, (BufferMemoKey, std::sync::Arc<SchematicState>)>>,
    /// Measurement specifications for the results specs matrix. Project
    /// design intent, so it persists with the workspace.
    #[serde(default)]
    pub specs: Vec<SpecEntry>,
    /// Plan-owned variables, output contracts, and specifications. Projects
    /// predating this feature migrate the active legacy `specs` projection
    /// into one record after execution-context migration.
    #[serde(default)]
    pub simulation_plan_payloads: Vec<SimulationPlanPayloadRecord>,
    /// Authoritative physical geometry for layout cell views, keyed by the
    /// exact `library/cell/view` owner. Documents use integral PDK database
    /// units and expected-revision transactions; no schematic or display
    /// geometry is projected into this store.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    physical_layout_documents: BTreeMap<String, crate::state::PhysicalLayoutDocument>,
    /// Project-owned, append-only evidence for exact signed-PDK callback
    /// executions. Each entry retains canonical inputs, derived metadata,
    /// package/runtime provenance, active-plan identity, and project revision
    /// authority; callback output is never accepted as ambient mutable state.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pdk_callback_receipts: Vec<crate::state::pdk_config::ProjectPdkCallbackReceipt>,
    /// Versioned, per-document Page Setup contracts used by schematic,
    /// symbol, result, and report hardcopy workflows. Publication artifacts
    /// and transient preview state are intentionally not persisted here.
    #[serde(default)]
    pub hardcopy_setups: crate::hardcopy::HardcopySetupStore,
    /// Bounded, digest-sealed outcome history for print and export
    /// publications. Failures and cancellations are retained alongside
    /// successful artifacts so project evidence never implies more than the
    /// platform actually accepted.
    #[serde(default)]
    pub hardcopy_receipts: crate::hardcopy::HardcopyReceiptLedger,
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
    /// Project-owned, dataset-bound result documents. Immutable solver
    /// datasets remain owned by result history; each visualization document
    /// pins exact dataset digests and owns only its versioned presentation
    /// graph.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub visualization_documents: Vec<crate::results::visualization_document::VisualizationDocument>,
    /// Reusable stimulus definitions this project owns. Placed sources adopt
    /// them by copy and keep a provenance receipt, so nothing here is read
    /// during netlist generation or execution.
    #[serde(
        default,
        skip_serializing_if = "crate::state::StimulusLibrary::is_empty"
    )]
    pub stimulus_library: crate::state::StimulusLibrary,
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
    /// Complete inactive top-level source decks. Only the active deck is
    /// projected into `netlist_source`/`netlist_document` and may execute.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub retained_netlist_decks: Vec<RetainedOwnedNetlistDeck>,
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
    /// Runtime dirty projection for project-owned result documents.
    #[serde(default, skip)]
    pub visualization_documents_dirty: bool,
    /// Runtime dirty projection for committed per-document page setups.
    #[serde(default, skip)]
    pub hardcopy_setups_dirty: bool,
    /// Runtime dirty projection for the durable hardcopy outcome ledger.
    #[serde(default, skip)]
    pub hardcopy_receipts_dirty: bool,
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
            design_projection_cache: std::cell::RefCell::new(None),
            materialized_buffers: std::cell::RefCell::new(HashMap::new()),
            specs: Vec::new(),
            simulation_plan_payloads: Vec::new(),
            physical_layout_documents: BTreeMap::new(),
            pdk_callback_receipts: Vec::new(),
            hardcopy_setups: crate::hardcopy::HardcopySetupStore::default(),
            hardcopy_receipts: crate::hardcopy::HardcopyReceiptLedger::default(),
            project_print_mappings: crate::hardcopy::PrintMappingPresetCatalog::new(
                crate::hardcopy::PrintMappingCatalogOwner::Project,
            ),
            engineering_table_views: crate::state::EngineeringTableViewStore::default(),
            hardcopy_source_sets: Vec::new(),
            report_documents: Vec::new(),
            visualization_documents: Vec::new(),
            stimulus_library: crate::state::StimulusLibrary::default(),
            netlist_source: None,
            netlist_document: None,
            netlist_descriptor: None,
            retained_netlist_decks: Vec::new(),
            project_sources: ProjectSourceRegistry::default(),
            netlist_source_path: None,
            netlist_source_dirty: false,
            project_sources_dirty: false,
            project_metadata_dirty: false,
            report_documents_dirty: false,
            visualization_documents_dirty: false,
            hardcopy_setups_dirty: false,
            hardcopy_receipts_dirty: false,
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

type RenamedLayoutDocument = Result<super::PhysicalLayoutDocument, super::LayoutDocumentError>;

fn validate_physical_layout_document_catalog(
    documents: &BTreeMap<String, crate::state::PhysicalLayoutDocument>,
) -> Result<(), crate::state::LayoutDocumentError> {
    for (key, document) in documents {
        document.validate()?;
        if document.owner().key() != *key {
            return Err(crate::state::LayoutDocumentError::Invalid {
                path: format!("physical_layout_documents[{key}].owner"),
                message: format!(
                    "document owner '{}' does not match its exact catalog key",
                    document.owner().key()
                ),
            });
        }
    }
    Ok(())
}

impl ProjectWorkspace {
    /// Assign deterministic stable identities to legacy top decks that predate
    /// the multi-deck catalog. Migration uses only already-persisted project,
    /// document, and logical-path identities, so loading identical bytes on a
    /// second machine produces the same project state.
    pub fn migrate_owned_netlist_deck_ids(&mut self) {
        let namespace = self.project.id().as_uuid();
        if let (Some(descriptor), Some(document)) =
            (&mut self.netlist_descriptor, &self.netlist_document)
            && descriptor.deck_id.is_nil()
        {
            let name = format!(
                "rspice/top-deck/v1/{}/{}",
                document.id().as_uuid(),
                descriptor.artifact_name
            );
            descriptor.deck_id = Uuid::new_v5(&namespace, name.as_bytes());
        }
        for (index, deck) in self.retained_netlist_decks.iter_mut().enumerate() {
            if deck.descriptor.deck_id.is_nil() {
                let name = format!(
                    "rspice/retained-top-deck/v1/{index}/{}/{}",
                    deck.document.id().as_uuid(),
                    deck.descriptor.artifact_name
                );
                deck.descriptor.deck_id = Uuid::new_v5(&namespace, name.as_bytes());
            }
        }
    }

    pub fn physical_layout_documents(
        &self,
    ) -> &BTreeMap<String, crate::state::PhysicalLayoutDocument> {
        &self.physical_layout_documents
    }

    pub fn physical_layout_document(
        &self,
        owner: &CellViewRef,
    ) -> Option<&crate::state::PhysicalLayoutDocument> {
        self.physical_layout_documents.get(&owner.key())
    }

    pub fn commit_physical_layout_document(
        &mut self,
        document: crate::state::PhysicalLayoutDocument,
    ) -> Result<Option<crate::state::PhysicalLayoutDocument>, crate::state::LayoutDocumentError>
    {
        document.validate()?;
        let key = document.owner().key();
        let mut candidate = self.physical_layout_documents.clone();
        let previous = candidate.insert(key, document);
        validate_physical_layout_document_catalog(&candidate)?;
        self.physical_layout_documents = candidate;
        Ok(previous)
    }

    pub(crate) fn prepare_insert_physical_layout_document(
        &self,
        document: crate::state::PhysicalLayoutDocument,
    ) -> Result<PreparedPhysicalLayoutCatalog, crate::state::LayoutDocumentError> {
        document.validate()?;
        let key = document.owner().key();
        let mut candidate = self.physical_layout_documents.clone();
        if candidate.insert(key.clone(), document).is_some() {
            return Err(crate::state::LayoutDocumentError::DuplicateObject {
                kind: "physical-layout document",
                id: key,
            });
        }
        validate_physical_layout_document_catalog(&candidate)?;
        Ok(PreparedPhysicalLayoutCatalog {
            documents: candidate,
        })
    }

    pub(crate) fn synchronize_physical_layout_document_from(
        &mut self,
        owner: &CellViewRef,
        source: &Self,
    ) -> Result<bool, crate::state::LayoutDocumentError> {
        let key = owner.key();
        let incoming = source.physical_layout_documents.get(&key).cloned();
        if self.physical_layout_documents.get(&key) == incoming.as_ref() {
            return Ok(false);
        }
        let mut candidate = self.physical_layout_documents.clone();
        match incoming {
            Some(document) => {
                candidate.insert(key, document);
            }
            None => {
                candidate.remove(&key);
            }
        }
        validate_physical_layout_document_catalog(&candidate)?;
        self.physical_layout_documents = candidate;
        Ok(true)
    }

    pub(crate) fn remove_physical_layout_document(&mut self, owner: &CellViewRef) -> bool {
        self.physical_layout_documents
            .remove(&owner.key())
            .is_some()
    }

    pub(crate) fn prepare_copy_physical_layout_cell_documents(
        &self,
        source_library: &str,
        source_cell: &str,
        target_library: &str,
        target_cell: &str,
    ) -> Result<PreparedPhysicalLayoutCatalog, crate::state::LayoutDocumentError> {
        let copies = self
            .physical_layout_documents
            .values()
            .filter(|document| {
                document.owner().library == source_library && document.owner().cell == source_cell
            })
            .map(|document| {
                document.copy_for_cell(source_library, source_cell, target_library, target_cell)
            })
            .collect::<Result<Vec<_>, _>>()?;
        let mut candidate = self.physical_layout_documents.clone();
        for document in &copies {
            let key = document.owner().key();
            if candidate.contains_key(&key) {
                return Err(crate::state::LayoutDocumentError::DuplicateObject {
                    kind: "physical-layout document",
                    id: key,
                });
            }
            candidate.insert(key, document.clone());
        }
        validate_physical_layout_document_catalog(&candidate)?;
        Ok(PreparedPhysicalLayoutCatalog {
            documents: candidate,
        })
    }

    pub(crate) fn prepare_rename_physical_layout_cell_documents(
        &self,
        library: &str,
        source_cell: &str,
        target_cell: &str,
    ) -> Result<PreparedPhysicalLayoutCatalog, crate::state::LayoutDocumentError> {
        self.prepare_renamed_layout_catalog(|document| {
            document.rename_cell_references(library, source_cell, target_cell)
        })
    }

    pub(crate) fn prepare_rename_physical_layout_library_documents(
        &self,
        source_library: &str,
        target_library: &str,
    ) -> Result<PreparedPhysicalLayoutCatalog, crate::state::LayoutDocumentError> {
        self.prepare_renamed_layout_catalog(|document| {
            document.rename_library_references(source_library, target_library)
        })
    }

    pub(crate) fn prepare_rename_physical_layout_view_documents(
        &self,
        library: &str,
        cell: &str,
        from_view: &str,
        to_view: &str,
    ) -> Result<PreparedPhysicalLayoutCatalog, crate::state::LayoutDocumentError> {
        self.prepare_renamed_layout_catalog(|document| {
            document.rename_view_references(library, cell, from_view, to_view)
        })
    }

    /// Rebuild the whole catalog through one rename, re-keyed by each owner.
    /// Every document is offered the rename, not only those the renamed scope
    /// owns: a layout elsewhere may place a master from it and must follow.
    fn prepare_renamed_layout_catalog(
        &self,
        rename: impl Fn(&crate::state::PhysicalLayoutDocument) -> RenamedLayoutDocument,
    ) -> Result<PreparedPhysicalLayoutCatalog, crate::state::LayoutDocumentError> {
        let mut candidate = BTreeMap::new();
        for document in self.physical_layout_documents.values() {
            let document = rename(document)?;
            let key = document.owner().key();
            if candidate.insert(key.clone(), document).is_some() {
                return Err(crate::state::LayoutDocumentError::DuplicateObject {
                    kind: "physical-layout document",
                    id: key,
                });
            }
        }
        validate_physical_layout_document_catalog(&candidate)?;
        Ok(PreparedPhysicalLayoutCatalog {
            documents: candidate,
        })
    }

    pub(crate) fn commit_prepared_physical_layout_catalog(
        &mut self,
        prepared: PreparedPhysicalLayoutCatalog,
    ) {
        self.physical_layout_documents = prepared.documents;
    }

    pub fn validate_physical_layout_documents(&self) -> Result<(), String> {
        validate_physical_layout_document_catalog(&self.physical_layout_documents)
            .map_err(|error| error.to_string())
    }

    #[must_use]
    pub fn pdk_callback_receipts(&self) -> &[crate::state::pdk_config::ProjectPdkCallbackReceipt] {
        &self.pdk_callback_receipts
    }

    /// Replace the project-owned callback ledger while applying an already
    /// validated lifecycle snapshot. The candidate is validated against this
    /// workspace's project identity and revision before any live state changes.
    pub(crate) fn replace_pdk_callback_receipts_for_lifecycle(
        &mut self,
        receipts: Vec<crate::state::pdk_config::ProjectPdkCallbackReceipt>,
    ) -> Result<(), String> {
        let mut candidate = self.clone();
        candidate.pdk_callback_receipts = receipts;
        candidate.validate_pdk_callback_receipts()?;
        self.pdk_callback_receipts = candidate.pdk_callback_receipts;
        Ok(())
    }

    pub fn validate_pdk_callback_receipts(&self) -> Result<(), String> {
        use crate::state::pdk_config::MAX_PROJECT_PDK_CALLBACK_RECEIPTS;

        if self.pdk_callback_receipts.len() > MAX_PROJECT_PDK_CALLBACK_RECEIPTS {
            return Err(format!(
                "receipt count exceeds {MAX_PROJECT_PDK_CALLBACK_RECEIPTS}"
            ));
        }
        let mut previous_digest = None;
        let mut previous_to_revision = None;
        for (index, receipt) in self.pdk_callback_receipts.iter().enumerate() {
            receipt
                .validate()
                .map_err(|error| format!("receipt[{index}] is invalid: {error}"))?;
            let expected_sequence = u64::try_from(index)
                .ok()
                .and_then(|value| value.checked_add(1))
                .ok_or_else(|| "receipt sequence overflow".to_owned())?;
            if receipt.sequence != expected_sequence {
                return Err(format!(
                    "receipt[{index}] has sequence {}, expected {expected_sequence}",
                    receipt.sequence
                ));
            }
            if receipt.project_id != self.project.id() {
                return Err(format!(
                    "receipt[{index}] belongs to project {}, not {}",
                    receipt.project_id,
                    self.project.id()
                ));
            }
            if receipt.previous_receipt_digest != previous_digest {
                return Err(format!(
                    "receipt[{index}] does not continue the callback receipt hash chain"
                ));
            }
            if previous_to_revision.is_some_and(|previous: ObjectRevision| {
                receipt.from_project_revision.get() < previous.get()
            }) {
                return Err(format!(
                    "receipt[{index}] predates the previous callback project revision"
                ));
            }
            if receipt.to_project_revision.get() > self.project.revision().get() {
                return Err(format!(
                    "receipt[{index}] claims future project revision {}",
                    receipt.to_project_revision.get()
                ));
            }
            previous_digest = Some(receipt.receipt_digest);
            previous_to_revision = Some(receipt.to_project_revision);
        }
        Ok(())
    }

    pub(crate) fn commit_pdk_callback_execution(
        &mut self,
        plan_id: SimulationPlanId,
        plan_revision: ObjectRevision,
        authority: &crate::state::pdk_config::PdkAdministrativeAuthority,
        reason: &str,
        input: crate::state::pdk_config::PdkCallbackExecutionInput,
        execution: crate::state::pdk_config::PdkCallbackExecutionReceipt,
    ) -> Result<
        crate::state::pdk_config::ProjectPdkCallbackReceipt,
        crate::state::pdk_config::PdkCallbackError,
    > {
        use crate::state::pdk_config::{
            MAX_PROJECT_PDK_CALLBACK_RECEIPTS, PdkCallbackError, ProjectPdkCallbackReceipt,
        };

        self.validate_pdk_callback_receipts()
            .map_err(PdkCallbackError::ProjectTransaction)?;
        if self.pdk_callback_receipts.len() >= MAX_PROJECT_PDK_CALLBACK_RECEIPTS {
            return Err(PdkCallbackError::ProjectTransaction(format!(
                "callback receipt ledger is limited to {MAX_PROJECT_PDK_CALLBACK_RECEIPTS} entries"
            )));
        }
        let from_project_revision = self.project.revision();
        let to_project_revision = self
            .project
            .next_revision()
            .map_err(|error| PdkCallbackError::ProjectTransaction(error.to_string()))?;
        let sequence = u64::try_from(self.pdk_callback_receipts.len())
            .ok()
            .and_then(|value| value.checked_add(1))
            .ok_or_else(|| {
                PdkCallbackError::ProjectTransaction(
                    "callback receipt sequence is exhausted".to_owned(),
                )
            })?;
        let receipt = ProjectPdkCallbackReceipt::issue(
            sequence,
            self.project.id(),
            from_project_revision,
            to_project_revision,
            plan_id,
            plan_revision,
            authority.actor_id.trim().to_owned(),
            authority.authority_id.trim().to_owned(),
            reason.trim().to_owned(),
            input,
            execution,
            self.pdk_callback_receipts
                .last()
                .map(|receipt| receipt.receipt_digest),
        )?;

        let mut candidate = self.clone();
        let committed_revision = candidate
            .project
            .advance_revision()
            .map_err(|error| PdkCallbackError::ProjectTransaction(error.to_string()))?;
        if committed_revision != to_project_revision {
            return Err(PdkCallbackError::ProjectTransaction(
                "preflighted project revision changed before callback receipt commit".to_owned(),
            ));
        }
        candidate.pdk_callback_receipts.push(receipt.clone());
        candidate
            .validate_pdk_callback_receipts()
            .map_err(PdkCallbackError::ProjectTransaction)?;
        *self = candidate;
        Ok(receipt)
    }

    /// Commit a new visualization document into the project authority.
    ///
    /// Validation and every duplicate check run before the vector changes, so
    /// a failed dialog commit cannot leave a partial document or dirty bit.
    pub fn insert_visualization_document(
        &mut self,
        document: crate::results::visualization_document::VisualizationDocument,
    ) -> Result<ResultDocumentId, VisualizationDocumentPersistenceError> {
        if self.visualization_documents.len() >= MAX_PROJECT_VISUALIZATION_DOCUMENTS {
            return Err(VisualizationDocumentPersistenceError::CatalogFull);
        }
        document.content_digest().map_err(|error| {
            VisualizationDocumentPersistenceError::Invalid {
                message: error.to_string(),
            }
        })?;
        if self
            .visualization_documents
            .iter()
            .any(|candidate| candidate.id() == document.id())
        {
            return Err(VisualizationDocumentPersistenceError::DuplicateIdentity {
                document_id: document.id(),
            });
        }
        if self
            .visualization_documents
            .iter()
            .any(|candidate| candidate.title().eq_ignore_ascii_case(document.title()))
        {
            return Err(VisualizationDocumentPersistenceError::DuplicateTitle {
                title: document.title().to_owned(),
            });
        }
        let document_id = document.id();
        self.visualization_documents.push(document);
        self.visualization_documents_dirty = true;
        Ok(document_id)
    }

    #[must_use]
    pub fn visualization_document(
        &self,
        document_id: ResultDocumentId,
    ) -> Option<&crate::results::visualization_document::VisualizationDocument> {
        self.visualization_documents
            .iter()
            .find(|document| document.id() == document_id)
    }

    /// Apply a revision-checked transaction to one project-owned result
    /// document and mark the result-document catalog dirty only after the
    /// document commits successfully.
    pub fn transact_visualization_document(
        &mut self,
        document_id: ResultDocumentId,
        expected_revision: crate::product::ObjectRevision,
        edits: Vec<crate::results::visualization_document::DocumentEdit>,
    ) -> Result<
        crate::results::visualization_document::VisualizationTransactionReceipt,
        VisualizationDocumentPersistenceError,
    > {
        let document = self
            .visualization_documents
            .iter_mut()
            .find(|document| document.id() == document_id)
            .ok_or(VisualizationDocumentPersistenceError::NotFound { document_id })?;
        let receipt = document
            .transact(expected_revision, edits)
            .map_err(|error| VisualizationDocumentPersistenceError::Transaction {
                document_id,
                message: error.to_string(),
            })?;
        self.visualization_documents_dirty = true;
        Ok(receipt)
    }

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
        validate_hardcopy_source_set_catalog(&self.hardcopy_source_sets).map_err(|error| {
            SimulationConfigurationError::InvalidHardcopySourceSetCatalog {
                message: error.to_string(),
            }
        })?;
        self.hardcopy_receipts.validate().map_err(|error| {
            SimulationConfigurationError::InvalidHardcopyReceiptLedger {
                message: error.to_string(),
            }
        })?;
        self.validate_pdk_callback_receipts().map_err(|message| {
            SimulationConfigurationError::InvalidPdkCallbackReceiptLedger { message }
        })?;
        self.validate_physical_layout_documents()
            .map_err(
                |message| SimulationConfigurationError::InvalidPhysicalLayoutCatalog { message },
            )?;
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
        let mut visualization_document_ids = std::collections::HashSet::new();
        let mut visualization_document_titles = std::collections::HashMap::<String, usize>::new();
        for (index, document) in self.visualization_documents.iter().enumerate() {
            document.content_digest().map_err(|error| {
                SimulationConfigurationError::InvalidVisualizationDocument {
                    index,
                    message: error.to_string(),
                }
            })?;
            if !visualization_document_ids.insert(document.id()) {
                return Err(
                    SimulationConfigurationError::DuplicateVisualizationDocumentIdentity {
                        document_id: document.id(),
                    },
                );
            }
            let folded_title = document.title().to_lowercase();
            if let Some(first_index) = visualization_document_titles.insert(folded_title, index) {
                return Err(
                    SimulationConfigurationError::DuplicateVisualizationDocumentTitle {
                        title: document.title().to_owned(),
                        first_index,
                        index,
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
            if document.ownership() == crate::state::DocumentOwnership::Generated {
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
            if descriptor.deck_id.is_nil() {
                return Err(
                    SimulationConfigurationError::InvalidNetlistDocumentProjection {
                        message: "owned top-deck identity cannot be nil".to_owned(),
                    },
                );
            }
            validate_owned_netlist_artifact_path(&descriptor.artifact_name).map_err(|message| {
                SimulationConfigurationError::InvalidNetlistDocumentProjection { message }
            })?;
            let declared_dialect = descriptor
                .imported_dialect
                .unwrap_or(NetlistSourceDialect::RSpice);
            let expected_profile = declared_dialect.execution_profile();
            if descriptor.execution_profile.is_some()
                && descriptor.execution_profile != expected_profile
            {
                return Err(
                    SimulationConfigurationError::InvalidNetlistDocumentProjection {
                        message: format!(
                            "owned netlist dialect {} does not match its recorded execution profile",
                            declared_dialect.label()
                        ),
                    },
                );
            }
            if declared_dialect.requires_compatibility_review() {
                let reviewed = descriptor.compatibility_reviewed
                    && descriptor.execution_profile == expected_profile
                    && expected_profile.is_some();
                let quarantined =
                    !descriptor.compatibility_reviewed && descriptor.execution_profile.is_none();
                if !reviewed && !quarantined {
                    return Err(
                        SimulationConfigurationError::InvalidNetlistDocumentProjection {
                            message: format!(
                                "owned non-canonical netlist dialect {} has neither an exact reviewed executable profile nor a fail-closed quarantine",
                                declared_dialect.label()
                            ),
                        },
                    );
                }
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
            if descriptor.revision_history.len() > MAX_OWNED_NETLIST_HISTORY_REVISIONS {
                return Err(
                    SimulationConfigurationError::InvalidNetlistDocumentProjection {
                        message: "owned source revision history exceeds its bounded entry limit"
                            .to_owned(),
                    },
                );
            }
            let mut previous_revision = 0_u64;
            let mut retained_bytes = 0_usize;
            for snapshot in &descriptor.revision_history {
                snapshot.validate().map_err(|message| {
                    SimulationConfigurationError::InvalidNetlistDocumentProjection { message }
                })?;
                if snapshot.document_revision <= previous_revision
                    || snapshot.document_revision > document.revision().get()
                {
                    return Err(
                        SimulationConfigurationError::InvalidNetlistDocumentProjection {
                            message:
                                "owned source revision history is not strictly revision ordered"
                                    .to_owned(),
                        },
                    );
                }
                retained_bytes = retained_bytes
                    .checked_add(snapshot.retained_bytes())
                    .ok_or_else(|| {
                        SimulationConfigurationError::InvalidNetlistDocumentProjection {
                            message: "owned source revision history size overflowed".to_owned(),
                        }
                    })?;
                previous_revision = snapshot.document_revision;
            }
            if retained_bytes > MAX_OWNED_NETLIST_HISTORY_BYTES {
                return Err(
                    SimulationConfigurationError::InvalidNetlistDocumentProjection {
                        message: "owned source revision history exceeds its bounded byte limit"
                            .to_owned(),
                    },
                );
            }
            if descriptor.owned_includes.len() > crate::state::MAX_PROJECT_SOURCE_FILES {
                return Err(
                    SimulationConfigurationError::InvalidNetlistDocumentProjection {
                        message: "owned include catalog exceeds the project file limit".to_owned(),
                    },
                );
            }
            let mut include_ids = HashSet::new();
            let mut include_identities = HashSet::new();
            for include in &descriptor.owned_includes {
                include.validate().map_err(|message| {
                    SimulationConfigurationError::InvalidNetlistDocumentProjection { message }
                })?;
                if !include_ids.insert(include.document_id)
                    || !include_identities.insert(include.logical_identity.as_str())
                {
                    return Err(
                        SimulationConfigurationError::InvalidNetlistDocumentProjection {
                            message: "owned include identities must be unique".to_owned(),
                        },
                    );
                }
                let dependency = document
                    .dependencies()
                    .iter()
                    .find(|dependency| {
                        dependency.locator().logical_identity() == include.logical_identity
                    })
                    .ok_or_else(|| {
                        SimulationConfigurationError::InvalidNetlistDocumentProjection {
                            message: format!(
                                "owned include '{}' is absent from the canonical dependency closure",
                                include.logical_identity
                            ),
                        }
                    })?;
                let source = dependency.source().ok_or_else(|| {
                    SimulationConfigurationError::InvalidNetlistDocumentProjection {
                        message: format!(
                            "owned include '{}' has no retained source bytes",
                            include.logical_identity
                        ),
                    }
                })?;
                if include.content_digest != crate::state::content_digest(source) {
                    return Err(
                        SimulationConfigurationError::InvalidNetlistDocumentProjection {
                            message: format!(
                                "owned include '{}' digest does not identify its retained bytes",
                                include.logical_identity
                            ),
                        },
                    );
                }
            }
        } else if self.netlist_descriptor.is_some() {
            return Err(
                SimulationConfigurationError::InvalidNetlistDocumentProjection {
                    message: "owned-artifact descriptor has no canonical document".to_owned(),
                },
            );
        }

        if self.retained_netlist_decks.len() > crate::state::MAX_PROJECT_SOURCE_FILES {
            return Err(
                SimulationConfigurationError::InvalidNetlistDocumentProjection {
                    message: "retained top-deck catalog exceeds the project file limit".to_owned(),
                },
            );
        }
        let mut deck_ids = HashSet::new();
        let mut deck_paths = HashSet::new();
        if let Some(descriptor) = &self.netlist_descriptor {
            deck_ids.insert(descriptor.deck_id);
            deck_paths.insert(descriptor.artifact_name.to_ascii_lowercase());
        }
        for deck in &self.retained_netlist_decks {
            validate_owned_netlist_projection(
                &deck.document,
                &deck.descriptor,
                deck.document.source(),
            )
            .map_err(|message| {
                SimulationConfigurationError::InvalidNetlistDocumentProjection { message }
            })?;
            if !deck_ids.insert(deck.descriptor.deck_id)
                || !deck_paths.insert(deck.descriptor.artifact_name.to_ascii_lowercase())
            {
                return Err(
                    SimulationConfigurationError::InvalidNetlistDocumentProjection {
                        message: "top-deck identities and logical paths must be unique".to_owned(),
                    },
                );
            }
        }

        let mut plan_ids = HashMap::<SimulationPlanId, usize>::new();
        let mut variable_ids = HashMap::<DesignVariableId, SimulationPlanId>::new();
        let mut output_ids = HashMap::<SavedOutputId, SimulationPlanId>::new();
        let mut capture_group_ids = HashMap::<CaptureGroupId, SimulationPlanId>::new();
        let mut specification_ids = HashMap::<SpecificationId, SimulationPlanId>::new();
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

            capture_group::validate_plan_groups(
                plan_id,
                &record.payload.capture_groups,
                &mut capture_group_ids,
            )
            .map_err(|source| SimulationConfigurationError::CaptureGroup { plan_id, source })?;

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

            record
                .payload
                .specification_policy
                .validate()
                .map_err(
                    |message| SimulationConfigurationError::InvalidSpecificationPolicy {
                        plan_id,
                        message,
                    },
                )?;
            if !record.payload.specification_definitions.is_empty() {
                if record.payload.specification_definitions.len() != record.payload.specs.len() {
                    return Err(
                        SimulationConfigurationError::InvalidSpecificationDefinition {
                            plan_id,
                            index: record.payload.specification_definitions.len(),
                            message: format!(
                                "governed definition count {} does not match scalar projection count {}",
                                record.payload.specification_definitions.len(),
                                record.payload.specs.len()
                            ),
                        },
                    );
                }
                let mut requirement_keys = HashMap::<String, usize>::new();
                let mut governed_measurements = HashMap::<String, usize>::new();
                for (index, definition) in
                    record.payload.specification_definitions.iter().enumerate()
                {
                    definition.validate().map_err(|message| {
                        SimulationConfigurationError::InvalidSpecificationDefinition {
                            plan_id,
                            index,
                            message,
                        }
                    })?;
                    if let Some(first_plan_id) = specification_ids.insert(definition.id, plan_id) {
                        return Err(
                            SimulationConfigurationError::DuplicateSpecificationIdentity {
                                id: definition.id,
                                first_plan_id,
                                plan_id,
                            },
                        );
                    }
                    let requirement_key = definition.requirement_key.to_ascii_lowercase();
                    if let Some(first_index) = requirement_keys.insert(requirement_key, index) {
                        return Err(
                            SimulationConfigurationError::DuplicateSpecificationRequirementKey {
                                plan_id,
                                index,
                                first_index,
                            },
                        );
                    }
                    let measurement = definition.measurement.to_ascii_lowercase();
                    if let Some(first_index) = governed_measurements.insert(measurement, index) {
                        return Err(SimulationConfigurationError::DuplicateSpecification {
                            plan_id,
                            index,
                            first_index,
                        });
                    }
                    if definition.projected_entry() != record.payload.specs[index] {
                        return Err(
                            SimulationConfigurationError::InvalidSpecificationDefinition {
                                plan_id,
                                index,
                                message: "governed definition disagrees with its scalar execution projection"
                                    .to_owned(),
                            },
                        );
                    }
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
