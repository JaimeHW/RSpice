//! Workspace editor sessions and application adapters over project-owned content.

use std::collections::{BTreeMap, HashMap, HashSet};
use std::path::Path;

mod annotation_restore;
mod capture_group;
mod design_intent;
mod design_projection;
mod hierarchy;
use rspice_design::hierarchy as hierarchy_resolver;
mod materialize;
mod open_documents;
mod reference_changes;
mod reference_preparation;
mod schematic_buffers;

// The two functions are renamed on export: bare `normalize` and
// `collation_key` say nothing about what they normalize outside their module,
// and analysis names have functions by exactly those names.
pub use capture_group::{
    CaptureGroup, CaptureGroupMembership, CaptureGroupRule, MembershipMove, UNGROUPED_NAME,
    collation_key as capture_group_collation_key, group_namer,
    normalize_name as normalize_capture_group_name,
};
pub use design_intent::*;
pub use design_projection::*;
pub use hierarchy::*;
pub(crate) use reference_changes::{PreparedReferences, ReferenceChanges};
pub(crate) use reference_preparation::{SchematicReferenceTransaction, reference_from_key};
pub use rspice_design::library::ProjectLibraryMutation;
pub use rspice_design::occurrence::{DocumentOccurrence, OccurrencePrune};
pub use rspice_design::owned_netlist::{
    NetlistExecutionProfile, NetlistLineEnding, NetlistSourceDialect, NetlistTextEncoding,
    OwnedNetlistDescriptor, OwnedNetlistEditStrategy, OwnedNetlistIncludeDescriptor,
    OwnedNetlistSaveRecord, RetainedOwnedNetlistDeck, validate_owned_netlist_artifact_path,
};
pub use rspice_project::*;
pub(crate) use rspice_simulation_contract::saved_output::{
    device_current_probe, saved_output_references,
};
// The glob is crate-private: `materialize` is `pub(super)` throughout except
// the one binding lookup two workbench surfaces reach by path and the metadata lookup the
// Models & PDKs symbol-contract table reads a declared family with.
use materialize::*;
pub(crate) use materialize::{metadata_value, project_veriloga_binding_for_view};

pub use rspice_simulation_contract::saved_output::{
    ComplexExpressionPolicy, OutputSelectionMode, SavedOutput, SavedOutputCompatibility,
    SavedOutputDisplayIntent, SavedOutputKind, SavedOutputOrigin, SavedOutputPolicy,
    SavedOutputPrecision, SavedOutputStreaming,
};
use serde::{Deserialize, Serialize};

#[cfg(test)]
use crate::product::{AnalysisInstanceId, ContentDigest, DesignVariableId, SimulationPlanId};
use crate::state::schematic::SchematicEditorRef;
#[cfg(test)]
use crate::state::{Cell, View};
use crate::state::{Library, LibraryCellInstance, LibraryManager, SchematicState, ViewType};
#[cfg(test)]
use rspice_app_types::hierarchy_path::InstancePath;
#[cfg(test)]
use rspice_design::schematic::component_type::ComponentType;
#[cfg(test)]
use std::path::PathBuf;
#[cfg(test)]
use uuid::Uuid;

pub use rspice_design_model::cell_view::{
    CellViewRef, DEFAULT_PROJECT_LIBRARY, DEFAULT_SCHEMATIC_VIEW, DEFAULT_TOP_CELL,
    validate_cell_view_name_segment,
};

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

pub use rspice_simulation_contract::regression_policy::{
    RegressionComparisonMethod, RegressionComparisonWindow, RegressionTargetKind,
    RegressionTargetSelector, RegressionToleranceRule,
};

pub use rspice_simulation_contract::plan_payload::{
    SimulationPlanPayload, SimulationPlanPayloadRecord,
};

// The source bundle API, re-exported from its historical workspace path. This
// block used to carry the whole `project_sources` surface "so downstream
// integrations keep compiling" -- there are no downstream integrations; the
// crate is the application. What is left is what `state::workspace` callers
// actually name through this path.
#[cfg(test)]
pub use super::project_sources::{MAX_PROJECT_CODE_SOURCE_BYTES, ProjectSourceBundle};
#[cfg(test)]
use super::project_sources::{ProjectSourceDocument, ProjectSourceError};
pub use super::project_sources::{ProjectSourceLanguage, ProjectSourceRegistry};

/// Application editor sessions over the project-owned workspace.
#[derive(Debug, Clone)]
pub struct ProjectWorkspace {
    pub content: rspice_project::ProjectWorkspace,
    annotation_restoration_error: Option<String>,
    pub(crate) schematic_sessions: HashMap<String, crate::state::schematic::SchematicSession>,
    design_projection_cache: rspice_design::projection::DesignProjectionCache,
}

impl Serialize for ProjectWorkspace {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.content.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for ProjectWorkspace {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(Self {
            content: Deserialize::deserialize(deserializer)?,
            annotation_restoration_error: None,
            schematic_sessions: HashMap::new(),
            design_projection_cache: Default::default(),
        })
    }
}

impl Default for ProjectWorkspace {
    fn default() -> Self {
        let content = rspice_project::ProjectWorkspace::default();
        let schematic_sessions =
            HashMap::from([(content.active_view.key(), SchematicState::default().session)]);
        Self {
            content,
            annotation_restoration_error: None,
            schematic_sessions,
            design_projection_cache: Default::default(),
        }
    }
}

#[cfg(test)]
mod tests;
