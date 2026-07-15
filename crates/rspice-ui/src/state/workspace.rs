//! Project workspace state.
//!
//! This module is the product-level design spine for RSpice Studio. It keeps
//! project identity, open Library/Cell/View documents, active hierarchy
//! breadcrumbs, and per-view schematic buffers together instead of letting the
//! workbench, library browser, and single schematic buffer drift apart.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use unicode_segmentation::UnicodeSegmentation;

use crate::product::{ObjectRevision, ProjectId, RevisionError};
use crate::state::{Cell, Library, LibraryManager, SchematicState, View, ViewType};

/// Default editable design library created for new projects.
pub const DEFAULT_PROJECT_LIBRARY: &str = "user";
/// Default top-level cell created for new projects.
pub const DEFAULT_TOP_CELL: &str = "top";
/// Default schematic view name.
pub const DEFAULT_SCHEMATIC_VIEW: &str = "schematic";
/// Persisted schema for project identity metadata.
pub const PROJECT_DESCRIPTOR_SCHEMA_VERSION: u16 = 1;

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

const fn default_project_descriptor_schema_version() -> u16 {
    PROJECT_DESCRIPTOR_SCHEMA_VERSION
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

/// Metadata for the active RSpice project.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectDescriptor {
    /// Stable product identity. Legacy project files receive an ID exactly
    /// once during deserialization and retain it on every later save.
    #[serde(default)]
    id: ProjectId,
    /// Schema of this object, independent of the outer project container.
    #[serde(default = "default_project_descriptor_schema_version")]
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

impl ProjectWorkspace {
    /// Create a new default project and ensure its editable top cell exists in
    /// the shared library manager.
    pub fn new_bootstrapped(libraries: &mut LibraryManager) -> Self {
        let mut workspace = Self::default();
        workspace.ensure_library_model(libraries);
        workspace
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

    fn reference(cell: &str) -> CellViewRef {
        CellViewRef::new("work", cell, "schematic")
    }

    fn symbol_reference(cell: &str) -> CellViewRef {
        CellViewRef::new("work", cell, "symbol")
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
