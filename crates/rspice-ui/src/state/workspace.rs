//! Project workspace state.
//!
//! This module is the product-level design spine for RSpice Studio. It keeps
//! project identity, open Library/Cell/View documents, active hierarchy
//! breadcrumbs, and per-view schematic buffers together instead of letting the
//! shell, library browser, and single schematic buffer drift apart.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::state::{Cell, Library, LibraryManager, SchematicState, View, ViewType};

/// Default editable design library created for new projects.
pub const DEFAULT_PROJECT_LIBRARY: &str = "user";
/// Default top-level cell created for new projects.
pub const DEFAULT_TOP_CELL: &str = "top";
/// Default schematic view name.
pub const DEFAULT_SCHEMATIC_VIEW: &str = "schematic";

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

    pub fn display_path(&self) -> String {
        self.key()
    }
}

/// Metadata for the active RSpice project.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectDescriptor {
    pub name: String,
    pub path: Option<PathBuf>,
    pub root_library: String,
    pub top_cell: String,
    pub technology: Option<String>,
    pub description: String,
}

impl Default for ProjectDescriptor {
    fn default() -> Self {
        Self {
            name: "Untitled Project".to_string(),
            path: None,
            root_library: DEFAULT_PROJECT_LIBRARY.to_string(),
            top_cell: DEFAULT_TOP_CELL.to_string(),
            technology: None,
            description: String::new(),
        }
    }
}

impl ProjectDescriptor {
    pub fn display_name(&self) -> &str {
        if self.name.trim().is_empty() {
            "Untitled Project"
        } else {
            self.name.as_str()
        }
    }

    pub fn set_path(&mut self, path: PathBuf) {
        if let Some(stem) = path.file_stem().and_then(|s| s.to_str())
            && !stem.trim().is_empty()
        {
            self.name = stem.to_string();
        }
        self.path = Some(path);
    }

    pub fn directory(&self) -> Option<&Path> {
        self.path.as_deref().and_then(Path::parent)
    }
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
}
