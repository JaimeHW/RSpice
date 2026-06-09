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

/// Project-level workspace state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectWorkspace {
    pub project: ProjectDescriptor,
    pub active_view: CellViewRef,
    pub open_views: Vec<OpenCellView>,
    pub hierarchy_stack: Vec<CellViewRef>,
    pub schematic_buffers: HashMap<String, SchematicState>,
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
            schematic_buffers,
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

        ensure_cell_view(
            libraries,
            &self.active_view.library,
            &self.active_view.cell,
            &self.active_view.view,
            ViewType::Schematic,
        );

        if self.open_views.is_empty() {
            self.open_views.push(OpenCellView::new(
                self.active_view.clone(),
                ViewType::Schematic,
            ));
        }
        if self.hierarchy_stack.is_empty() {
            self.hierarchy_stack.push(self.active_view.clone());
        }

        self.ensure_active_buffer();
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

    pub fn save_active_schematic(&mut self, schematic: &SchematicState) {
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
    }

    pub fn any_dirty(&self) -> bool {
        self.open_views.iter().any(|view| view.dirty)
            || self
                .schematic_buffers
                .values()
                .any(|schematic| schematic.is_dirty)
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
        if matches!(view_type, ViewType::Schematic | ViewType::Testbench) {
            self.schematic_buffers.entry(reference.key()).or_default();
        }
    }

    pub fn open_as_root(&mut self, reference: CellViewRef, view_type: ViewType) {
        self.open_view(reference.clone(), view_type);
        self.hierarchy_stack.clear();
        self.hierarchy_stack.push(reference);
    }

    pub fn enter_hierarchy(&mut self, reference: CellViewRef, view_type: ViewType) {
        self.open_view(reference.clone(), view_type);
        if self.hierarchy_stack.last() != Some(&reference) {
            self.hierarchy_stack.push(reference);
        }
    }

    pub fn focus_breadcrumb(&mut self, index: usize) -> Option<CellViewRef> {
        if index >= self.hierarchy_stack.len() {
            return None;
        }

        self.hierarchy_stack.truncate(index + 1);
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

    ensure_cell_view(
        libraries,
        DEFAULT_PROJECT_LIBRARY,
        DEFAULT_TOP_CELL,
        DEFAULT_SCHEMATIC_VIEW,
        ViewType::Schematic,
    );
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
