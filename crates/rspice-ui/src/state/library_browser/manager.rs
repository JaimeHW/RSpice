//! The library/cell/view hierarchy.
//!
//! The three-column navigation model every commercial EDA tool uses, and the
//! resolution rules behind it: a cell is addressed by library and name, and
//! a view by all three, so the same cell name in two libraries is two cells.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::{Cell, Library, View, ViewType};
use crate::state::canonical_cell_view_owner_key;

// Library Manager
// =============================================================================

/// Central manager for all libraries
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LibraryManager {
    /// All libraries indexed by name
    libraries: HashMap<String, Library>,
    /// Currently selected library name
    pub selected_library: Option<String>,
    /// Currently selected cell name
    pub selected_cell: Option<String>,
    /// Currently selected view name
    pub selected_view: Option<String>,
    /// Search/filter text
    pub filter_text: String,
    /// Whether to show read-only libraries
    pub show_read_only: bool,
    /// Governed persisted catalog revision. Engineering mutations advance it;
    /// runtime presentation projections and persistence sanitization do not.
    /// `get_library_mut` advances pessimistically because callers receive
    /// unrestricted access to persisted engineering content.
    #[serde(default)]
    revision: u64,
    /// Column the keyboard owns in the browser (↑↓ move, ←→ hop) —
    /// runtime UI state, follows mouse clicks too.
    #[serde(skip)]
    pub nav_column: NavColumn,
    /// Scroll the keyboard column's selected row into view this frame
    /// (set by a keyboard move, cleared after rendering).
    #[serde(skip)]
    pub nav_scroll: bool,
}

/// Which browser column keyboard navigation acts on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum NavColumn {
    /// The library list.
    #[default]
    Library,
    /// The cell list.
    Cell,
    /// The view list.
    View,
}

impl LibraryManager {
    /// Create a new library manager
    pub fn new() -> Self {
        Self {
            show_read_only: true,
            ..Default::default()
        }
    }

    /// Content revision, for caches over library listings.
    pub fn revision(&self) -> u64 {
        self.revision
    }

    /// Add a library
    pub fn add_library(&mut self, library: Library) {
        self.libraries.insert(library.name.clone(), library);
        self.revision = self.revision.wrapping_add(1);
    }

    /// Remove a library
    pub fn remove_library(&mut self, name: &str) -> Option<Library> {
        self.revision = self.revision.wrapping_add(1);
        self.libraries.remove(name)
    }

    /// Get a library by name
    pub fn get_library(&self, name: &str) -> Option<&Library> {
        self.libraries.get(name)
    }

    /// Iterate libraries with their canonical map keys.
    pub fn libraries_by_key(&self) -> impl Iterator<Item = (&str, &Library)> {
        self.libraries
            .iter()
            .map(|(key, library)| (key.as_str(), library))
    }

    /// Get mutable library by name (assumes mutation: bumps the revision)
    pub fn get_library_mut(&mut self, name: &str) -> Option<&mut Library> {
        self.revision = self.revision.wrapping_add(1);
        self.libraries.get_mut(name)
    }

    /// Get libraries sorted by name
    pub fn libraries_sorted(&self) -> Vec<&Library> {
        let mut libs: Vec<_> = self.libraries.values().collect();
        libs.sort_by(|a, b| a.name.cmp(&b.name));
        if !self.show_read_only {
            libs.retain(|l| !l.read_only);
        }
        libs
    }

    /// Get the currently selected library
    pub fn current_library(&self) -> Option<&Library> {
        self.selected_library
            .as_ref()
            .and_then(|name| self.libraries.get(name))
    }

    /// Get the currently selected cell
    pub fn current_cell(&self) -> Option<&Cell> {
        self.current_library().and_then(|lib| {
            self.selected_cell
                .as_ref()
                .and_then(|name| lib.cells.get(name))
        })
    }

    /// Get the currently selected view
    pub fn current_view(&self) -> Option<&View> {
        self.current_cell().and_then(|cell| {
            self.selected_view
                .as_ref()
                .and_then(|name| cell.views.get(name))
        })
    }

    /// Select a library
    pub fn select_library(&mut self, name: &str) {
        if self.libraries.contains_key(name) {
            self.selected_library = Some(name.to_string());
            self.selected_cell = None;
            self.selected_view = None;
        }
    }

    /// Select a cell
    pub fn select_cell(&mut self, library: &str, cell: &str) {
        if let Some(lib) = self.libraries.get(library)
            && lib.cells.contains_key(cell)
        {
            self.selected_library = Some(library.to_string());
            self.selected_cell = Some(cell.to_string());
            self.selected_view = None;
        }
    }

    /// Select a view
    pub fn select_view(&mut self, library: &str, cell: &str, view: &str) {
        if let Some(lib) = self.libraries.get(library)
            && let Some(c) = lib.cells.get(cell)
            && c.views.contains_key(view)
        {
            self.selected_library = Some(library.to_string());
            self.selected_cell = Some(cell.to_string());
            self.selected_view = Some(view.to_string());
        }
    }

    /// Get the LCV path string (library/cell/view)
    pub fn selected_lcv_path(&self) -> Option<String> {
        match (
            &self.selected_library,
            &self.selected_cell,
            &self.selected_view,
        ) {
            (Some(lib), Some(cell), Some(view)) => Some(format!("{}/{}/{}", lib, cell, view)),
            (Some(lib), Some(cell), None) => Some(format!("{}/{}", lib, cell)),
            (Some(lib), None, None) => Some(lib.clone()),
            _ => None,
        }
    }

    /// Parse an LCV path and select it
    pub fn select_from_path(&mut self, path: &str) {
        let parts: Vec<&str> = path.split('/').collect();
        match parts.len() {
            1 => self.select_library(parts[0]),
            2 => self.select_cell(parts[0], parts[1]),
            3 => self.select_view(parts[0], parts[1], parts[2]),
            _ => {}
        }
    }

    /// Create a new cell in a library
    pub fn create_cell(&mut self, library: &str, cell_name: &str) -> bool {
        if let Some(lib) = self.libraries.get_mut(library) {
            if lib.read_only {
                return false;
            }
            let requested = canonical_cell_view_owner_key(library, cell_name, "");
            if !lib
                .cells
                .values()
                .any(|cell| canonical_cell_view_owner_key(library, &cell.name, "") == requested)
            {
                lib.add_cell(Cell::new(cell_name));
                self.revision = self.revision.wrapping_add(1);
                return true;
            }
        }
        false
    }

    /// Create a new view in a cell
    pub fn create_view(
        &mut self,
        library: &str,
        cell: &str,
        view_name: &str,
        view_type: ViewType,
    ) -> bool {
        if let Some(lib) = self.libraries.get_mut(library) {
            if lib.read_only {
                return false;
            }
            if let Some(c) = lib.cells.get_mut(cell)
                && {
                    let requested = canonical_cell_view_owner_key(library, cell, view_name);
                    !c.views.values().any(|view| {
                        canonical_cell_view_owner_key(library, cell, &view.name) == requested
                    })
                }
            {
                c.add_view(View::new(view_name, view_type));
                self.revision = self.revision.wrapping_add(1);
                return true;
            }
        }
        false
    }

    /// Search for cells matching a pattern
    pub fn search_cells(&self, pattern: &str) -> Vec<(&Library, &Cell)> {
        let pattern_lower = pattern.to_lowercase();
        let mut results = Vec::new();

        for lib in self.libraries.values() {
            for cell in lib.cells.values() {
                if cell.name.to_lowercase().contains(&pattern_lower)
                    || cell.description.to_lowercase().contains(&pattern_lower)
                {
                    results.push((lib, cell));
                }
            }
        }

        results
    }

    /// Get all open views across all libraries
    pub fn open_views(&self) -> Vec<(&Library, &Cell, &View)> {
        let mut results = Vec::new();

        for lib in self.libraries.values() {
            for cell in lib.cells.values() {
                for view in cell.views.values() {
                    if view.is_open {
                        results.push((lib, cell, view));
                    }
                }
            }
        }

        results
    }

    /// Count total libraries
    pub fn library_count(&self) -> usize {
        self.libraries.len()
    }

    /// Count total cells across all libraries
    pub fn total_cell_count(&self) -> usize {
        self.libraries.values().map(|l| l.cell_count()).sum()
    }

    /// Count total views across all libraries
    pub fn total_view_count(&self) -> usize {
        self.libraries.values().map(|l| l.total_view_count()).sum()
    }

    /// Clear all libraries
    pub fn clear(&mut self) {
        self.libraries.clear();
        self.selected_library = None;
        self.selected_cell = None;
        self.selected_view = None;
        self.revision = self.revision.wrapping_add(1);
    }

    /// Replace the complete governed library catalog from a validated
    /// publication snapshot while retaining this session's presentation
    /// preferences and advancing, never rewinding, its content revision.
    pub(crate) fn replace_catalog_from_snapshot(&mut self, snapshot: &Self) -> Result<u64, String> {
        let revision = self
            .revision
            .checked_add(1)
            .ok_or_else(|| "project library content revision is exhausted".to_owned())?;
        self.libraries = snapshot.libraries.clone();
        self.selected_library = snapshot.selected_library.clone();
        self.selected_cell = snapshot.selected_cell.clone();
        self.selected_view = snapshot.selected_view.clone();
        self.revision = revision;
        Ok(revision)
    }

    /// Clear runtime dirty markers without claiming an engineering catalog
    /// mutation. Save acceptance must not advance the persisted content
    /// revision merely because editor presentation state became clean.
    pub(crate) fn mark_all_views_clean_runtime(&mut self) {
        for library in self.libraries.values_mut() {
            for cell in library.cells.values_mut() {
                for view in cell.views.values_mut() {
                    view.modified = false;
                }
            }
        }
    }

    /// Clear one view's runtime dirty marker without advancing the governed
    /// library revision.
    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) fn mark_view_clean_runtime(
        &mut self,
        library: &str,
        cell: &str,
        view: &str,
    ) -> bool {
        self.set_view_modified_runtime(library, cell, view, false)
    }

    /// Project one document-registry dirty bit into presentation state
    /// without claiming a library content mutation.
    pub(crate) fn set_view_modified_runtime(
        &mut self,
        library: &str,
        cell: &str,
        view: &str,
        modified: bool,
    ) -> bool {
        let Some(view) = self
            .libraries
            .get_mut(library)
            .and_then(|library| library.cells.get_mut(cell))
            .and_then(|cell| cell.views.get_mut(view))
        else {
            return false;
        };
        view.modified = modified;
        true
    }

    /// Strip runtime-only view presentation from a serialization clone while
    /// preserving the exact governed catalog revision.
    pub(crate) fn sanitize_views_for_persistence(&mut self) {
        self.mark_all_views_clean_runtime();
        for library in self.libraries.values_mut() {
            for cell in library.cells.values_mut() {
                for view in cell.views.values_mut() {
                    view.is_open = false;
                    view.file_path = None;
                    view.modified_time = None;
                }
            }
        }
    }

    /// Overlay one cell-view document from an already-snapshotted working
    /// catalog without manufacturing a new revision while assembling a
    /// partial-save candidate.
    ///
    /// The source snapshot is the authority for the observed revision. Parent
    /// library/cell records are copied only when the document does not yet
    /// exist in the accepted target. A generated symbol owned by a schematic
    /// document is synchronized as part of the same atomic overlay.
    pub(crate) fn overlay_cell_view_document_from_snapshot(
        &mut self,
        source: &Self,
        library_name: &str,
        cell_name: &str,
        view_name: &str,
    ) -> Result<(), String> {
        if self.revision > source.revision {
            return Err(
                "accepted library revision is newer than the working document snapshot".to_owned(),
            );
        }

        let source_library = source
            .libraries
            .get(library_name)
            .ok_or_else(|| format!("missing library '{library_name}'"))?;
        let source_cell = source_library
            .cells
            .get(cell_name)
            .ok_or_else(|| format!("missing cell '{cell_name}'"))?;
        let source_view = source_cell
            .views
            .get(view_name)
            .ok_or_else(|| format!("missing view '{view_name}'"))?
            .clone();
        let generated_symbol = view_name.eq_ignore_ascii_case("schematic").then(|| {
            source_cell
                .views
                .get("symbol")
                .filter(|view| view.metadata.contains_key("generated"))
                .cloned()
        });

        if !self.libraries.contains_key(library_name) {
            let mut library = source_library.clone();
            library.cells.clear();
            self.libraries.insert(library_name.to_owned(), library);
        }
        let target_library = self
            .libraries
            .get_mut(library_name)
            .expect("library was inserted above");
        if !target_library.cells.contains_key(cell_name) {
            let mut cell = source_cell.clone();
            cell.views.clear();
            target_library.cells.insert(cell_name.to_owned(), cell);
        }
        let target_cell = target_library
            .cells
            .get_mut(cell_name)
            .expect("cell was inserted above");
        target_cell.views.insert(view_name.to_owned(), source_view);

        if let Some(generated_symbol) = generated_symbol {
            match generated_symbol {
                Some(symbol) => {
                    target_cell.views.insert("symbol".to_owned(), symbol);
                }
                None if target_cell
                    .views
                    .get("symbol")
                    .is_some_and(|view| view.metadata.contains_key("generated")) =>
                {
                    target_cell.views.remove("symbol");
                }
                None => {}
            }
        }

        self.revision = source.revision;
        Ok(())
    }
}
