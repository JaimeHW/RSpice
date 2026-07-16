//! Project workspace state.
//!
//! This module is the product-level design spine for RSpice Studio. It keeps
//! project identity, open Library/Cell/View documents, active hierarchy
//! breadcrumbs, and per-view schematic buffers together instead of letting the
//! workbench, library browser, and single schematic buffer drift apart.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Deserializer, Serialize, de::Error as _};
use unicode_segmentation::UnicodeSegmentation;
use uuid::Uuid;

use crate::product::{ObjectRevision, ProjectId, RevisionError};
use crate::state::{
    Cell, ComponentType, Library, LibraryCellInstance, LibraryManager, SchematicState, View,
    ViewType,
};

/// Default editable design library created for new projects.
pub const DEFAULT_PROJECT_LIBRARY: &str = "user";
/// Default top-level cell created for new projects.
pub const DEFAULT_TOP_CELL: &str = "top";
/// Default schematic view name.
pub const DEFAULT_SCHEMATIC_VIEW: &str = "schematic";
/// Persisted schema for project identity metadata.
pub const PROJECT_DESCRIPTOR_SCHEMA_VERSION: u16 = 1;

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

/// Resolution state for one grouped library/cell/view binding in the complete
/// testbench hierarchy.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HierarchyBindingStatus {
    Resolved,
    Modified,
    Unresolved,
    Recursive,
    DepthLimit,
    InstanceLimit,
}

impl HierarchyBindingStatus {
    pub fn label(self) -> &'static str {
        match self {
            Self::Resolved => "resolved",
            Self::Modified => "modified",
            Self::Unresolved => "unresolved",
            Self::Recursive => "recursive",
            Self::DepthLimit => "depth limit",
            Self::InstanceLimit => "instance limit",
        }
    }

    pub fn is_resolved(self) -> bool {
        matches!(self, Self::Resolved | Self::Modified)
    }

    pub fn is_modified(self) -> bool {
        self == Self::Modified
    }

    fn severity(self) -> u8 {
        match self {
            Self::Resolved => 0,
            Self::Modified => 1,
            Self::Unresolved => 2,
            Self::Recursive => 3,
            Self::DepthLimit => 4,
            Self::InstanceLimit => 5,
        }
    }
}

/// One row in the resolved hierarchy-binding manifest. Repeated masters are
/// grouped while `instance_count` retains their exact expanded multiplicity.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedHierarchyBinding {
    pub reference: CellViewRef,
    pub purpose: String,
    pub view_search_order: Vec<String>,
    pub stop_view: Option<String>,
    pub model_section: String,
    pub status: HierarchyBindingStatus,
    pub instance_count: usize,
    pub diagnostic: Option<String>,
}

/// Immutable resolution receipt for the project configuration surface and
/// preflight diagnostics.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HierarchyResolution {
    pub bindings: Vec<ResolvedHierarchyBinding>,
    pub total_instances: usize,
    pub resolved_instances: usize,
}

impl HierarchyResolution {
    pub fn unresolved_instances(&self) -> usize {
        self.total_instances.saturating_sub(self.resolved_instances)
    }

    pub fn is_valid(&self) -> bool {
        self.unresolved_instances() == 0
    }
}

/// Metadata for the active RSpice project.
#[derive(Debug, Clone, Serialize)]
pub struct ProjectDescriptor {
    /// Stable product identity. Legacy project files receive an ID exactly
    /// once during deserialization and retain it on every later save.
    id: ProjectId,
    /// Schema of this object, independent of the outer project container.
    schema_version: u16,
    /// Monotonic logical revision. Runtime-only path changes do not alter it.
    #[serde(default)]
    revision: ObjectRevision,
    #[serde(default = "default_project_name")]
    name: String,
    pub path: Option<PathBuf>,
    pub root_library: String,
    pub top_cell: String,
    pub technology: Option<String>,
    pub description: String,
}

impl<'de> Deserialize<'de> for ProjectDescriptor {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct ProjectDescriptorDe {
            #[serde(default)]
            id: DeserializedProjectId,
            #[serde(default)]
            schema_version: DeserializedProjectSchemaVersion,
            #[serde(default)]
            revision: ObjectRevision,
            #[serde(default = "default_project_name")]
            name: String,
            path: Option<PathBuf>,
            root_library: String,
            top_cell: String,
            technology: Option<String>,
            description: String,
        }

        let descriptor = ProjectDescriptorDe::deserialize(deserializer)?;
        if matches!(
            &descriptor.schema_version,
            DeserializedProjectSchemaVersion::Null
        ) {
            return Err(D::Error::custom(
                "project schema version must not be explicitly null",
            ));
        }
        let has_schema_version = matches!(
            &descriptor.schema_version,
            DeserializedProjectSchemaVersion::Value(_)
        );
        let id = match descriptor.id {
            DeserializedProjectId::Null => {
                return Err(D::Error::custom(
                    "project identity must not be explicitly null",
                ));
            }
            DeserializedProjectId::Value(id) => id,
            DeserializedProjectId::Missing if has_schema_version => {
                return Err(D::Error::custom(
                    "versioned project descriptor is missing its stable identity",
                ));
            }
            DeserializedProjectId::Missing => {
                // Serialize the complete identity-bearing descriptor state in
                // a fixed field order. Project/session containers inject an ID
                // scoped to their complete artifact before reaching this
                // fallback; this path covers standalone genuine-legacy
                // descriptors without inventing random identity.
                let material = serde_json::to_vec(&(
                    descriptor.revision,
                    &descriptor.name,
                    &descriptor.path,
                    &descriptor.root_library,
                    &descriptor.top_cell,
                    &descriptor.technology,
                    &descriptor.description,
                ))
                .map_err(D::Error::custom)?;
                ProjectId::from_namespace(LEGACY_PROJECT_DESCRIPTOR_ID_NAMESPACE, &material)
            }
        };

        Ok(Self {
            id,
            schema_version: match descriptor.schema_version {
                DeserializedProjectSchemaVersion::Value(version) => version,
                DeserializedProjectSchemaVersion::Missing => PROJECT_DESCRIPTOR_SCHEMA_VERSION,
                DeserializedProjectSchemaVersion::Null => {
                    unreachable!("explicitly null project schema rejected above")
                }
            },
            revision: descriptor.revision,
            name: descriptor.name,
            path: descriptor.path,
            root_library: descriptor.root_library,
            top_cell: descriptor.top_cell,
            technology: descriptor.technology,
            description: descriptor.description,
        })
    }
}

impl Default for ProjectDescriptor {
    fn default() -> Self {
        Self {
            id: ProjectId::new(),
            schema_version: PROJECT_DESCRIPTOR_SCHEMA_VERSION,
            revision: ObjectRevision::INITIAL,
            name: default_project_name(),
            path: None,
            root_library: DEFAULT_PROJECT_LIBRARY.to_string(),
            top_cell: DEFAULT_TOP_CELL.to_string(),
            technology: None,
            description: String::new(),
        }
    }
}

impl ProjectDescriptor {
    #[must_use]
    pub const fn id(&self) -> ProjectId {
        self.id
    }

    #[must_use]
    pub const fn schema_version(&self) -> u16 {
        self.schema_version
    }

    #[must_use]
    pub const fn revision(&self) -> ObjectRevision {
        self.revision
    }

    #[must_use]
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn display_name(&self) -> &str {
        if self.name.trim().is_empty() {
            "Untitled Project"
        } else {
            self.name.as_str()
        }
    }

    /// Validate the persisted identity boundary before saving or accepting a
    /// project file. Remote uniqueness and authority checks remain service
    /// responsibilities because they require the selected project location.
    pub fn validate(&self) -> Result<(), ProjectDescriptorError> {
        if self.schema_version != PROJECT_DESCRIPTOR_SCHEMA_VERSION {
            return Err(ProjectDescriptorError::UnsupportedSchema {
                found: self.schema_version,
                supported: PROJECT_DESCRIPTOR_SCHEMA_VERSION,
            });
        }
        if self.id.as_uuid().is_nil() {
            return Err(ProjectDescriptorError::NilIdentity);
        }
        Self::validate_name(&self.name)?;
        if self.root_library.trim().is_empty() {
            return Err(ProjectDescriptorError::RequiredField("root_library"));
        }
        if self.top_cell.trim().is_empty() {
            return Err(ProjectDescriptorError::RequiredField("top_cell"));
        }
        Ok(())
    }

    /// Validate the local portion of `project.name` from the frozen field
    /// contract. The exact Unicode scalar sequence is retained; no case or
    /// normalization folding is performed.
    pub fn validate_name(value: &str) -> Result<(), ProjectDescriptorError> {
        if value != value.trim() {
            return Err(ProjectDescriptorError::SurroundingWhitespace);
        }
        let grapheme_count = value.graphemes(true).count();
        if grapheme_count == 0 {
            return Err(ProjectDescriptorError::EmptyName);
        }
        if grapheme_count > 120 {
            return Err(ProjectDescriptorError::NameTooLong { grapheme_count });
        }
        if let Some(character) = value.chars().find(|character| character.is_control()) {
            return Err(ProjectDescriptorError::ControlCharacter(character));
        }
        if let Some(separator) = value
            .chars()
            .find(|character| matches!(character, '/' | '\\'))
        {
            return Err(ProjectDescriptorError::PathSeparator(separator));
        }
        Ok(())
    }

    /// Rename as one fail-closed logical transaction. A rejected name leaves
    /// both the value and revision unchanged.
    pub fn rename(
        &mut self,
        name: impl Into<String>,
    ) -> Result<ObjectRevision, ProjectDescriptorError> {
        let name = name.into();
        Self::validate_name(&name)?;
        if self.name == name {
            return Ok(self.revision);
        }
        let next_revision = self.revision.next()?;
        self.name = name;
        self.revision = next_revision;
        Ok(next_revision)
    }

    pub fn set_path(&mut self, path: PathBuf) {
        // The first save may supply a useful name for an otherwise untitled
        // project. Reopening or moving a named project must not silently
        // rename it or change its logical revision.
        if self.path.is_none()
            && self.name == default_project_name()
            && let Some(stem) = path.file_stem().and_then(|s| s.to_str())
            && Self::validate_name(stem).is_ok()
        {
            // Validation above and an available next revision make this
            // infallible for every practical project lifetime. If the
            // revision space is exhausted, retaining the old name is safer.
            let _ = self.rename(stem.to_owned());
        }
        self.path = Some(path);
    }

    /// Create the descriptor for an independent project copy.
    ///
    /// A project copy is not a rename or move of the source project: it owns
    /// a fresh stable identity and starts a new revision history at the
    /// selected location.  The receiver is borrowed, so this operation cannot
    /// accidentally rebind or otherwise mutate the source project.
    #[must_use]
    pub fn fork_copy_at(&self, path: PathBuf) -> Self {
        let mut copy = self.clone();
        copy.id = ProjectId::new();
        copy.revision = ObjectRevision::INITIAL;
        copy.path = Some(path);
        copy
    }

    pub fn directory(&self) -> Option<&Path> {
        self.path.as_deref().and_then(Path::parent)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ProjectDescriptorError {
    #[error("project schema version {found} is unsupported; this build supports {supported}")]
    UnsupportedSchema { found: u16, supported: u16 },
    #[error("project identity must not be the nil UUID")]
    NilIdentity,
    #[error("project name must contain at least one grapheme cluster")]
    EmptyName,
    #[error("project name contains {grapheme_count} grapheme clusters; the maximum is 120")]
    NameTooLong { grapheme_count: usize },
    #[error("project name must not begin or end with whitespace")]
    SurroundingWhitespace,
    #[error("project name contains control character {0:?}")]
    ControlCharacter(char),
    #[error("project name contains path separator {0:?}")]
    PathSeparator(char),
    #[error("project field {0} is required")]
    RequiredField(&'static str),
    #[error(transparent)]
    Revision(#[from] RevisionError),
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

/// One specification bound for a `.MEAS` result — a row of the specs
/// matrix. At least one of `min`/`max` is normally set; a spec with
/// neither still pins the measurement as a tracked row (value-only).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpecEntry {
    /// `.MEAS` result name this spec bounds (case-insensitive match).
    pub measurement: String,
    /// Lower bound (pass when value ≥ min).
    pub min: Option<f64>,
    /// Upper bound (pass when value ≤ max).
    pub max: Option<f64>,
    /// Display unit, purely cosmetic (e.g. "V", "s", "dB").
    pub unit: String,
}

impl SpecEntry {
    /// Spec verdict for one measured value.
    pub fn passes(&self, value: f64) -> bool {
        self.min.is_none_or(|min| value >= min) && self.max.is_none_or(|max| value <= max)
    }

    /// Violation magnitude (how far outside the bounds), 0 when passing.
    pub fn violation(&self, value: f64) -> f64 {
        let below = self.min.map_or(0.0, |min| (min - value).max(0.0));
        let above = self.max.map_or(0.0, |max| (value - max).max(0.0));
        below.max(above)
    }
}

/// Project-level workspace state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectWorkspace {
    pub project: ProjectDescriptor,
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
    /// Manually edited netlist source. When set, simulations run this
    /// deck instead of regenerating from the schematic (text-first mode);
    /// `None` means the netlist view shows the generated artifact.
    #[serde(default)]
    pub netlist_source: Option<String>,
    /// Native filesystem origin for `netlist_source`, used to resolve relative
    /// `.include`/`.lib` paths for imported decks. Browser imports and direct
    /// editor edits clear this because they do not have a reliable file base.
    #[serde(default)]
    pub netlist_source_path: Option<PathBuf>,
    /// Runtime dirty bit for `netlist_source`; skipped because dirty state is
    /// session-local while the source itself is persisted with the project.
    #[serde(default, skip)]
    pub netlist_source_dirty: bool,
}

impl Default for ProjectWorkspace {
    fn default() -> Self {
        let active_view = CellViewRef::default_top();
        let mut schematic_buffers = HashMap::new();
        schematic_buffers.insert(active_view.key(), SchematicState::default());

        Self {
            project: ProjectDescriptor::default(),
            active_view: active_view.clone(),
            open_views: vec![OpenCellView::new(active_view.clone(), ViewType::Schematic)],
            hierarchy_stack: vec![active_view],
            hierarchy_instances: Vec::new(),
            schematic_buffers,
            specs: Vec::new(),
            netlist_source: None,
            netlist_source_path: None,
            netlist_source_dirty: false,
        }
    }
}

struct HierarchyResolver<'a> {
    workspace: &'a ProjectWorkspace,
    libraries: &'a LibraryManager,
    active_overlay: Option<(&'a CellViewRef, &'a SchematicState)>,
    rows: Vec<ResolvedHierarchyBinding>,
    row_indices: HashMap<String, usize>,
    total_instances: usize,
    resolved_instances: usize,
}

#[derive(Clone, Copy)]
struct HierarchyMaster<'a> {
    schematic: Option<&'a SchematicState>,
    view_type: Option<ViewType>,
    view_modified: bool,
    library_read_only: bool,
    library_has_technology: bool,
}

impl<'a> HierarchyResolver<'a> {
    fn new(
        workspace: &'a ProjectWorkspace,
        libraries: &'a LibraryManager,
        active_overlay: Option<(&'a CellViewRef, &'a SchematicState)>,
    ) -> Self {
        Self {
            workspace,
            libraries,
            active_overlay,
            rows: Vec::new(),
            row_indices: HashMap::new(),
            total_instances: 0,
            resolved_instances: 0,
        }
    }

    fn resolve(mut self) -> HierarchyResolution {
        let root = CellViewRef::new(
            &self.workspace.project.root_library,
            &self.workspace.project.top_cell,
            DEFAULT_SCHEMATIC_VIEW,
        );
        let mut ancestors = Vec::new();
        self.resolve_reference(root, None, 0, true, &mut ancestors);
        HierarchyResolution {
            bindings: self.rows,
            total_instances: self.total_instances,
            resolved_instances: self.resolved_instances,
        }
    }

    fn resolve_reference(
        &mut self,
        requested: CellViewRef,
        binding: Option<&LibraryCellInstance>,
        depth: usize,
        is_root: bool,
        ancestors: &mut Vec<String>,
    ) {
        if self.total_instances >= MAX_HIERARCHY_RESOLUTION_INSTANCES {
            let mut row = self.binding_row(
                requested,
                binding,
                depth,
                is_root,
                HierarchyBindingStatus::InstanceLimit,
                Some(format!(
                    "hierarchy exceeds the supported limit of {MAX_HIERARCHY_RESOLUTION_INSTANCES} expanded instances"
                )),
            );
            row.instance_count = 1;
            self.total_instances = self.total_instances.saturating_add(1);
            self.upsert(row);
            return;
        }
        self.total_instances += 1;

        if depth > MAX_HIERARCHY_RESOLUTION_DEPTH {
            let row = self.binding_row(
                requested,
                binding,
                depth,
                is_root,
                HierarchyBindingStatus::DepthLimit,
                Some(format!(
                    "hierarchy exceeds the supported depth of {MAX_HIERARCHY_RESOLUTION_DEPTH}"
                )),
            );
            self.upsert(row);
            return;
        }

        let search_order = hierarchy_view_search_order(&requested.view, is_root);
        let source_binding_error = binding.and_then(|binding| {
            binding
                .source_path
                .as_ref()
                .and_then(|_| self.validate_source_binding(binding).err())
        });
        let master = if source_binding_error.is_none() {
            self.resolve_master(&requested, binding, &search_order)
        } else {
            None
        };
        let resolved_reference = master
            .as_ref()
            .and_then(|(_, reference)| reference.clone())
            .unwrap_or_else(|| requested.clone());
        let identity = hierarchy_identity(&resolved_reference);

        if ancestors.iter().any(|ancestor| ancestor == &identity) {
            let chain = ancestors
                .iter()
                .chain(std::iter::once(&identity))
                .cloned()
                .collect::<Vec<_>>()
                .join(" → ");
            let row = self.binding_row_with_master(
                resolved_reference,
                binding,
                depth,
                is_root,
                master.map(|(master, _)| master),
                HierarchyBindingStatus::Recursive,
                Some(format!("recursive hierarchy: {chain}")),
            );
            self.upsert(row);
            return;
        }

        let (master, status, diagnostic) = match master {
            Some((master, _)) => {
                let modified = master.view_modified
                    || master.schematic.is_some_and(|schematic| schematic.is_dirty);
                (
                    Some(master),
                    if modified {
                        HierarchyBindingStatus::Modified
                    } else {
                        HierarchyBindingStatus::Resolved
                    },
                    None,
                )
            }
            None => (
                None,
                HierarchyBindingStatus::Unresolved,
                source_binding_error.or_else(|| {
                    Some(format!(
                        "no executable master resolved for {} using {}",
                        hierarchy_display_path(&requested),
                        search_order.join(" → ")
                    ))
                }),
            ),
        };

        let row = self.binding_row_with_master(
            resolved_reference,
            binding,
            depth,
            is_root,
            master,
            status,
            diagnostic,
        );
        self.upsert(row);
        if status.is_resolved() {
            self.resolved_instances += 1;
        }

        let Some(schematic) = master.and_then(|master| master.schematic) else {
            return;
        };
        let children = schematic
            .components
            .iter()
            .filter(|component| component.kind == ComponentType::CellInstance)
            .filter_map(|component| component.library_cell.clone())
            .collect::<Vec<_>>();
        if children.is_empty() {
            return;
        }

        ancestors.push(identity);
        for child in &children {
            let requested_view = if child.view.eq_ignore_ascii_case("symbol") {
                DEFAULT_SCHEMATIC_VIEW
            } else {
                child.view.as_str()
            };
            self.resolve_reference(
                CellViewRef::new(&child.library, &child.cell, requested_view),
                Some(child),
                depth + 1,
                false,
                ancestors,
            );
        }
        ancestors.pop();
    }

    fn binding_row(
        &self,
        reference: CellViewRef,
        binding: Option<&LibraryCellInstance>,
        depth: usize,
        is_root: bool,
        status: HierarchyBindingStatus,
        diagnostic: Option<String>,
    ) -> ResolvedHierarchyBinding {
        self.binding_row_with_master(reference, binding, depth, is_root, None, status, diagnostic)
    }

    #[allow(clippy::too_many_arguments)]
    fn binding_row_with_master(
        &self,
        reference: CellViewRef,
        binding: Option<&LibraryCellInstance>,
        depth: usize,
        is_root: bool,
        master: Option<HierarchyMaster<'_>>,
        status: HierarchyBindingStatus,
        diagnostic: Option<String>,
    ) -> ResolvedHierarchyBinding {
        let search_order = hierarchy_view_search_order(&reference.view, is_root);
        let source_bound = binding
            .and_then(|value| value.source_path.as_ref())
            .is_some();
        let terminal_view = master
            .and_then(|value| value.view_type)
            .filter(|view_type| hierarchy_stop_view(*view_type));
        let purpose = if is_root {
            "testbench root"
        } else if source_bound || terminal_view.is_some() {
            "macro-model"
        } else if master
            .is_some_and(|value| value.library_read_only && value.library_has_technology)
        {
            "foundry devices"
        } else if depth == 1 {
            "design under test"
        } else {
            "hierarchical cell"
        };
        let stop_view = if is_root {
            None
        } else {
            terminal_view
                .map(|view_type| view_type.display_name().to_owned())
                .or_else(|| {
                    search_order
                        .iter()
                        .rev()
                        .find(|view| hierarchy_stop_view(ViewType::from_name(view)))
                        .cloned()
                })
        };
        ResolvedHierarchyBinding {
            model_section: hierarchy_model_section(self.libraries, &reference, binding),
            reference,
            purpose: purpose.to_owned(),
            view_search_order: search_order,
            stop_view,
            status,
            instance_count: 1,
            diagnostic,
        }
    }

    fn resolve_master(
        &self,
        requested: &CellViewRef,
        binding: Option<&LibraryCellInstance>,
        search_order: &[String],
    ) -> Option<(HierarchyMaster<'a>, Option<CellViewRef>)> {
        let library = find_library(self.libraries, &requested.library);
        let cell = library.and_then(|library| find_cell(library, &requested.cell));
        let source_bound = binding
            .and_then(|value| value.source_path.as_ref())
            .is_some();

        if source_bound {
            let view = cell.and_then(|cell| find_view(cell, &requested.view))?;
            return Some((
                HierarchyMaster {
                    schematic: None,
                    view_type: Some(view.view_type),
                    view_modified: view.modified,
                    library_read_only: library.is_some_and(|library| library.read_only),
                    library_has_technology: library
                        .is_some_and(|library| !library.technology.trim().is_empty()),
                },
                Some(requested.clone()),
            ));
        }

        for candidate in search_order {
            let reference = CellViewRef::new(&requested.library, &requested.cell, candidate);
            // A buffer without an authoritative library/cell/view identity is
            // an orphan, not an executable master. Corrupt or partially
            // restored workspaces must fail closed.
            let Some(view) = cell.and_then(|cell| find_view(cell, candidate)) else {
                continue;
            };
            let view_type = view.view_type;
            if matches!(view_type, ViewType::Schematic | ViewType::Testbench) {
                if let Some(schematic) = self.find_schematic(&reference) {
                    return Some((
                        HierarchyMaster {
                            schematic: Some(schematic),
                            view_type: Some(view_type),
                            view_modified: view.modified,
                            library_read_only: library.is_some_and(|library| library.read_only),
                            library_has_technology: library
                                .is_some_and(|library| !library.technology.trim().is_empty()),
                        },
                        Some(reference),
                    ));
                }
                continue;
            }
            // External executable views are owned by the placed binding's
            // source path. A library view alone is not enough: accepting it
            // here would claim a closure the netlist generator cannot emit.
        }
        None
    }

    fn find_schematic(&self, reference: &CellViewRef) -> Option<&'a SchematicState> {
        if let Some((overlay_reference, schematic)) = self.active_overlay
            && overlay_reference
                .key()
                .eq_ignore_ascii_case(&reference.key())
        {
            return Some(schematic);
        }
        find_schematic(self.workspace, reference)
    }

    fn validate_source_binding(&self, binding: &LibraryCellInstance) -> Result<(), String> {
        let Some(library) = find_library(self.libraries, &binding.library) else {
            return Err(format!(
                "source-backed binding {}/{} has no authoritative library",
                binding.library, binding.cell
            ));
        };
        let Some(cell) = find_cell(library, &binding.cell) else {
            return Err(format!(
                "source-backed binding {}/{} has no authoritative cell",
                binding.library, binding.cell
            ));
        };
        let Some(view) = find_view(cell, &binding.view) else {
            return Err(format!(
                "source-backed binding {}/{}/{} has no authoritative view",
                binding.library, binding.cell, binding.view
            ));
        };
        if !matches!(
            view.view_type,
            ViewType::Spice | ViewType::VerilogA | ViewType::Verilog | ViewType::Extracted
        ) {
            return Err(format!(
                "source-backed binding {}/{}/{} is not an executable source view",
                binding.library, binding.cell, binding.view
            ));
        }
        if binding.terminal_order.is_empty() {
            return Err(format!(
                "source-backed binding {}/{}/{} has no validated terminal contract",
                binding.library, binding.cell, binding.view
            ));
        }
        let source_path = binding
            .source_path
            .as_deref()
            .expect("validated only for source-backed bindings");
        if !source_path.is_absolute() {
            return Err(format!(
                "source-backed binding {}/{}/{} does not have an absolute source identity",
                binding.library, binding.cell, binding.view
            ));
        }
        let authoritative_path = view
            .file_path
            .as_deref()
            .or_else(|| metadata_source_path(&view.metadata))
            .or_else(|| metadata_source_path(&cell.metadata));
        let Some(authoritative_path) = authoritative_path else {
            return Err(format!(
                "source-backed binding {}/{}/{} has no authoritative source identity",
                binding.library, binding.cell, binding.view
            ));
        };
        if !source_paths_match(source_path, authoritative_path) {
            return Err(format!(
                "source-backed binding {}/{}/{} conflicts with the authoritative source path",
                binding.library, binding.cell, binding.view
            ));
        }
        validate_source_file(source_path, view.view_type, binding)
    }

    fn upsert(&mut self, row: ResolvedHierarchyBinding) {
        let key = row.reference.key().to_ascii_lowercase();
        if let Some(index) = self.row_indices.get(&key).copied() {
            let existing = &mut self.rows[index];
            existing.instance_count = existing.instance_count.saturating_add(row.instance_count);
            if row.status.severity() > existing.status.severity() {
                existing.status = row.status;
                existing.diagnostic = row.diagnostic;
            }
            return;
        }
        self.row_indices.insert(key, self.rows.len());
        self.rows.push(row);
    }
}

fn find_library<'a>(libraries: &'a LibraryManager, name: &str) -> Option<&'a Library> {
    libraries
        .libraries_by_key()
        .find(|(key, library)| {
            key.eq_ignore_ascii_case(name) || library.name.eq_ignore_ascii_case(name)
        })
        .map(|(_, library)| library)
}

fn find_cell<'a>(library: &'a Library, name: &str) -> Option<&'a Cell> {
    library
        .cells
        .iter()
        .find(|(key, cell)| key.eq_ignore_ascii_case(name) || cell.name.eq_ignore_ascii_case(name))
        .map(|(_, cell)| cell)
}

fn find_view<'a>(cell: &'a Cell, name: &str) -> Option<&'a View> {
    cell.views
        .iter()
        .find(|(key, view)| key.eq_ignore_ascii_case(name) || view.name.eq_ignore_ascii_case(name))
        .map(|(_, view)| view)
}

fn find_schematic<'a>(
    workspace: &'a ProjectWorkspace,
    reference: &CellViewRef,
) -> Option<&'a SchematicState> {
    workspace
        .schematic_buffers
        .iter()
        .find(|(key, _)| key.eq_ignore_ascii_case(&reference.key()))
        .map(|(_, schematic)| schematic)
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

#[cfg(not(target_arch = "wasm32"))]
fn validate_source_file(
    source_path: &Path,
    view_type: ViewType,
    binding: &LibraryCellInstance,
) -> Result<(), String> {
    let source = std::fs::read_to_string(source_path).map_err(|error| {
        format!(
            "source-backed binding {}/{}/{} cannot read {}: {error}",
            binding.library,
            binding.cell,
            binding.view,
            source_path.display()
        )
    })?;
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
            tokens
                .next()
                .is_some_and(|token| token.eq_ignore_ascii_case(".subckt"))
                && tokens
                    .next()
                    .is_some_and(|token| token.eq_ignore_ascii_case(master))
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
    format!(
        "{}/{}",
        reference.library.to_ascii_lowercase(),
        reference.cell.to_ascii_lowercase()
    )
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

impl ProjectWorkspace {
    /// Create a new default project and ensure its editable top cell exists in
    /// the shared library manager.
    pub fn new_bootstrapped(libraries: &mut LibraryManager) -> Self {
        let mut workspace = Self::default();
        workspace.ensure_library_model(libraries);
        workspace
    }

    /// Resolve the complete executable library/cell/view closure rooted at the
    /// project testbench. Open tabs are intentionally irrelevant: this receipt
    /// follows placed hierarchical instances and the same schematic/source
    /// ownership used by netlisting.
    pub fn resolve_hierarchy(&self, libraries: &LibraryManager) -> HierarchyResolution {
        HierarchyResolver::new(self, libraries, None).resolve()
    }

    /// Resolve the hierarchy while projecting the live editor buffer over its
    /// persisted workspace copy. Rendering and validation use this form so a
    /// just-placed instance cannot disappear from the receipt until save or a
    /// view switch.
    pub fn resolve_hierarchy_with_active<'a>(
        &'a self,
        libraries: &'a LibraryManager,
        active_reference: &'a CellViewRef,
        active_schematic: &'a SchematicState,
    ) -> HierarchyResolution {
        HierarchyResolver::new(self, libraries, Some((active_reference, active_schematic)))
            .resolve()
    }

    /// Ensure the workspace's top library/cell/view exists in the library tree.
    pub fn ensure_library_model(&mut self, libraries: &mut LibraryManager) {
        ensure_project_library(libraries);

        if self.active_view.library.is_empty() {
            self.active_view.library = self.project.root_library.clone();
        }
        if self.active_view.cell.is_empty() {
            self.active_view.cell = self.project.top_cell.clone();
        }
        if self.active_view.view.is_empty() {
            self.active_view.view = DEFAULT_SCHEMATIC_VIEW.to_string();
        }

        let active_view_type = self
            .open_views
            .iter()
            .find(|open| open.reference == self.active_view)
            .map(|open| open.view_type)
            .or_else(|| library_view_type(libraries, &self.active_view))
            .unwrap_or(ViewType::Schematic);

        ensure_cell_view(
            libraries,
            &self.active_view.library,
            &self.active_view.cell,
            &self.active_view.view,
            active_view_type,
        );

        if self.open_views.is_empty() {
            self.open_views.push(OpenCellView::new(
                self.active_view.clone(),
                active_view_type,
            ));
        }
        if self.hierarchy_stack.is_empty() {
            self.hierarchy_stack.push(self.active_view.clone());
        }

        if is_schematic_like(active_view_type) {
            self.ensure_active_buffer();
        }
        libraries.select_view(
            &self.active_view.library,
            &self.active_view.cell,
            &self.active_view.view,
        );
    }

    pub fn active_key(&self) -> String {
        self.active_view.key()
    }

    pub fn active_display_path(&self) -> String {
        self.active_view.display_path()
    }

    pub fn active_view_type(&self) -> ViewType {
        self.open_views
            .iter()
            .find(|open| open.reference == self.active_view)
            .map(|open| open.view_type)
            .unwrap_or(ViewType::Schematic)
    }

    pub fn ensure_active_buffer(&mut self) {
        let key = self.active_key();
        self.schematic_buffers.entry(key).or_default();
    }

    pub fn active_schematic(&self) -> Option<&SchematicState> {
        self.schematic_buffers.get(&self.active_key())
    }

    pub fn active_schematic_reference(&self) -> CellViewRef {
        if self.active_view_type() == ViewType::Symbol {
            return CellViewRef::new(
                &self.active_view.library,
                &self.active_view.cell,
                DEFAULT_SCHEMATIC_VIEW,
            );
        }
        self.active_view.clone()
    }

    pub fn active_context_schematic(&self) -> Option<&SchematicState> {
        let reference = self.active_schematic_reference();
        self.schematic_buffers.get(&reference.key())
    }

    pub fn save_active_schematic(&mut self, schematic: &SchematicState) {
        if !is_schematic_like(self.active_view_type()) {
            return;
        }
        let key = self.active_key();
        self.schematic_buffers.insert(key, schematic.clone());
        self.set_active_dirty(schematic.is_dirty);
    }

    pub fn mark_all_clean(&mut self) {
        for view in &mut self.open_views {
            view.dirty = false;
        }
        for schematic in self.schematic_buffers.values_mut() {
            schematic.is_dirty = false;
        }
        self.netlist_source_dirty = false;
    }

    pub fn any_dirty(&self) -> bool {
        self.open_views.iter().any(|view| view.dirty)
            || self
                .schematic_buffers
                .values()
                .any(|schematic| schematic.is_dirty)
            || self.netlist_source_dirty
    }

    pub fn set_netlist_source_dirty(&mut self, dirty: bool) {
        self.netlist_source_dirty = dirty;
    }

    /// Whether the Netlist workspace owns an editable source deck.
    ///
    /// A missing source is intentional: in that state the editor is showing a
    /// generated schematic artifact and must never promote edits implicitly.
    pub fn has_editable_netlist_source(&self) -> bool {
        self.netlist_source.is_some()
    }

    /// Create a project-owned source deck from the current generated artifact.
    ///
    /// This is the one ownership transition used by the explicit Netlist
    /// workspace "Make editable copy" action. Creating the source changes the
    /// persisted project, so it participates in the ordinary project dirty and
    /// save lifecycle on both native and browser targets.
    pub fn make_netlist_editable_copy(&mut self, generated: &str) -> bool {
        if self.netlist_source.is_some() {
            return false;
        }

        self.netlist_source = Some(generated.to_owned());
        self.netlist_source_path = None;
        self.netlist_source_dirty = true;
        true
    }

    /// Replace an existing project-owned source deck.
    ///
    /// Returns `false` for generated artifacts instead of silently creating an
    /// editable source. That guard makes editor, completion, and tuner writes
    /// safe even if a caller accidentally reaches a mutation path while the
    /// generated document is active.
    pub fn replace_editable_netlist_source(&mut self, source: String) -> bool {
        let Some(owned_source) = self.netlist_source.as_mut() else {
            return false;
        };

        if *owned_source == source {
            return false;
        }

        *owned_source = source;
        self.netlist_source_path = None;
        self.netlist_source_dirty = true;
        true
    }

    /// Remove the project-owned source and return to schematic-generated output.
    ///
    /// Removing persisted source ownership is itself a project modification;
    /// the dirty bit remains set until an actual project save succeeds.
    pub fn return_to_generated_netlist(&mut self) -> bool {
        if self.netlist_source.take().is_none() {
            return false;
        }

        self.netlist_source_path = None;
        self.netlist_source_dirty = true;
        true
    }

    pub fn open_view(&mut self, reference: CellViewRef, view_type: ViewType) {
        self.active_view = reference.clone();
        if !self
            .open_views
            .iter()
            .any(|open| open.reference == reference)
        {
            self.open_views
                .push(OpenCellView::new(reference.clone(), view_type));
        }
        if is_schematic_like(view_type) {
            self.schematic_buffers.entry(reference.key()).or_default();
        }
    }

    pub fn open_as_root(&mut self, reference: CellViewRef, view_type: ViewType) {
        self.open_view(reference.clone(), view_type);
        self.hierarchy_stack.clear();
        self.hierarchy_instances.clear();
        self.hierarchy_stack.push(reference);
    }

    pub fn enter_hierarchy(&mut self, reference: CellViewRef, view_type: ViewType) {
        // No instance context (menu/browser entry): the cell name is the
        // best available occurrence label.
        let label = reference.cell.clone();
        self.descend_into(label, reference, view_type);
    }

    /// Descend into `instance`, opening its master `reference`. The
    /// occurrence path (TOP, X1, XB, ...) records the instance name.
    pub fn descend_into(&mut self, instance: String, reference: CellViewRef, view_type: ViewType) {
        self.open_view(reference.clone(), view_type);
        if self.hierarchy_stack.last() != Some(&reference) {
            self.hierarchy_stack.push(reference);
            self.hierarchy_instances.push(instance);
            self.align_occurrences();
        }
    }

    /// Keep `hierarchy_instances` exactly one shorter than the stack —
    /// older saves and external truncation re-label from cell names.
    fn align_occurrences(&mut self) {
        let want = self.hierarchy_stack.len().saturating_sub(1);
        self.hierarchy_instances.truncate(want);
        while self.hierarchy_instances.len() < want {
            let index = self.hierarchy_instances.len() + 1;
            self.hierarchy_instances
                .push(self.hierarchy_stack[index].cell.clone());
        }
    }

    /// Display labels for the occurrence path: the root cell, then the
    /// instance descended through at each level.
    pub fn occurrence_labels(&self) -> Vec<String> {
        self.hierarchy_stack
            .iter()
            .enumerate()
            .map(|(index, reference)| {
                if index == 0 {
                    reference.cell.clone()
                } else {
                    self.hierarchy_instances
                        .get(index - 1)
                        .cloned()
                        .unwrap_or_else(|| reference.cell.clone())
                }
            })
            .collect()
    }

    /// Pop one hierarchy level (the U gesture). Returns the new focus.
    pub fn ascend_one(&mut self) -> Option<CellViewRef> {
        let len = self.hierarchy_stack.len();
        if len < 2 {
            return None;
        }
        self.focus_breadcrumb(len - 2)
    }

    pub fn focus_breadcrumb(&mut self, index: usize) -> Option<CellViewRef> {
        if index >= self.hierarchy_stack.len() {
            return None;
        }

        self.hierarchy_stack.truncate(index + 1);
        self.align_occurrences();
        let reference = self.hierarchy_stack[index].clone();
        self.open_view(reference.clone(), ViewType::Schematic);
        Some(reference)
    }

    pub fn close_view(&mut self, reference: &CellViewRef) {
        if self.open_views.len() <= 1 {
            return;
        }

        self.open_views.retain(|open| &open.reference != reference);
        if &self.active_view == reference
            && let Some(next) = self.open_views.last().cloned()
        {
            self.active_view = next.reference;
        }
    }

    pub fn set_active_dirty(&mut self, dirty: bool) {
        if let Some(open) = self
            .open_views
            .iter_mut()
            .find(|open| open.reference == self.active_view)
        {
            open.dirty = dirty;
        }
    }
}

/// Ensure the default editable project library exists.
pub fn ensure_project_library(libraries: &mut LibraryManager) {
    if libraries.get_library(DEFAULT_PROJECT_LIBRARY).is_none() {
        let mut library = Library::new(DEFAULT_PROJECT_LIBRARY);
        library
            .metadata
            .insert("role".to_string(), "project".to_string());
        library.metadata.insert(
            "description".to_string(),
            "Project design library".to_string(),
        );
        libraries.add_library(library);
    }
}

/// Ensure a cell view exists in the library manager.
pub fn ensure_cell_view(
    libraries: &mut LibraryManager,
    library_name: &str,
    cell_name: &str,
    view_name: &str,
    view_type: ViewType,
) {
    if libraries.get_library(library_name).is_none() {
        libraries.add_library(Library::new(library_name));
    }

    if let Some(library) = libraries.get_library_mut(library_name) {
        if library.get_cell(cell_name).is_none() {
            let mut cell = Cell::new(cell_name);
            cell.description = "Top-level design cell".to_string();
            cell.add_view(View::new(view_name, view_type));
            library.add_cell(cell);
            return;
        }

        if let Some(cell) = library.get_cell_mut(cell_name)
            && cell.get_view(view_name).is_none()
        {
            cell.add_view(View::new(view_name, view_type));
        }
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
}
