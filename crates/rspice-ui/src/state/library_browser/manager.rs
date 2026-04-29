use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::{Cell, Library, View, ViewType};

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
}

impl LibraryManager {
    /// Create a new library manager
    pub fn new() -> Self {
        Self {
            show_read_only: true,
            ..Default::default()
        }
    }

    /// Add a library
    pub fn add_library(&mut self, library: Library) {
        self.libraries.insert(library.name.clone(), library);
    }

    /// Remove a library
    pub fn remove_library(&mut self, name: &str) -> Option<Library> {
        self.libraries.remove(name)
    }

    /// Get a library by name
    pub fn get_library(&self, name: &str) -> Option<&Library> {
        self.libraries.get(name)
    }

    /// Get mutable library by name
    pub fn get_library_mut(&mut self, name: &str) -> Option<&mut Library> {
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
            if !lib.cells.contains_key(cell_name) {
                lib.add_cell(Cell::new(cell_name));
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
                && !c.views.contains_key(view_name)
            {
                c.add_view(View::new(view_name, view_type));
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
    }
}
