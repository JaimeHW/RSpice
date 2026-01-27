//! Library/Cell/View Browser
//!
//! Cadence-style hierarchical design library management system.
//!
//! # Architecture
//!
//! The Library/Cell/View (LCV) hierarchy mirrors Cadence Virtuoso:
//! - **Library**: A collection of design cells (e.g., `my_designs`)
//! - **Cell**: A single design unit (e.g., `opamp`, `bandgap`)
//! - **View**: A particular representation (e.g., `schematic`, `symbol`, `layout`)
//!
//! # Example Hierarchy
//!
//! ```text
//! my_library/
//! ├── opamp/
//! │   ├── schematic
//! │   ├── symbol
//! │   └── testbench
//! ├── bandgap/
//! │   ├── schematic
//! │   └── symbol
//! └── resistor/
//!     └── symbol
//! ```

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

// =============================================================================
// View Types
// =============================================================================

/// Standard view types matching Cadence conventions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ViewType {
    /// Schematic view (circuit diagram)
    Schematic,
    /// Symbol view (block symbol for hierarchical use)
    Symbol,
    /// Layout view (physical design)
    Layout,
    /// Testbench view (simulation setup)
    Testbench,
    /// Verilog behavioral model
    Verilog,
    /// VerilogA behavioral model for SPICE
    VerilogA,
    /// SPICE netlist
    Spice,
    /// Documentation
    Document,
    /// Extracted view (post-layout)
    Extracted,
    /// Abstract view (timing/power)
    Abstract,
    /// Configuration view
    Config,
    /// Custom/user-defined view
    Custom,
}

impl Default for ViewType {
    fn default() -> Self {
        ViewType::Schematic
    }
}

impl ViewType {
    /// Display name for view type
    pub fn display_name(&self) -> &'static str {
        match self {
            ViewType::Schematic => "schematic",
            ViewType::Symbol => "symbol",
            ViewType::Layout => "layout",
            ViewType::Testbench => "testbench",
            ViewType::Verilog => "verilog",
            ViewType::VerilogA => "veriloga",
            ViewType::Spice => "spice",
            ViewType::Document => "doc",
            ViewType::Extracted => "extracted",
            ViewType::Abstract => "abstract",
            ViewType::Config => "config",
            ViewType::Custom => "custom",
        }
    }

    /// Parse view type from string
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "schematic" | "sch" => ViewType::Schematic,
            "symbol" | "sym" => ViewType::Symbol,
            "layout" | "lay" => ViewType::Layout,
            "testbench" | "tb" => ViewType::Testbench,
            "verilog" | "v" => ViewType::Verilog,
            "veriloga" | "va" => ViewType::VerilogA,
            "spice" | "sp" => ViewType::Spice,
            "doc" | "document" => ViewType::Document,
            "extracted" | "ext" => ViewType::Extracted,
            "abstract" | "abs" => ViewType::Abstract,
            "config" | "cfg" => ViewType::Config,
            _ => ViewType::Custom,
        }
    }

    /// Icon for this view type
    pub fn icon(&self) -> &'static str {
        match self {
            ViewType::Schematic => "📋",
            ViewType::Symbol => "🔲",
            ViewType::Layout => "🗺️",
            ViewType::Testbench => "🧪",
            ViewType::Verilog => "📝",
            ViewType::VerilogA => "📝",
            ViewType::Spice => "⚡",
            ViewType::Document => "📄",
            ViewType::Extracted => "🔍",
            ViewType::Abstract => "📊",
            ViewType::Config => "⚙️",
            ViewType::Custom => "📁",
        }
    }

    /// All standard view types
    pub const ALL: [ViewType; 12] = [
        ViewType::Schematic,
        ViewType::Symbol,
        ViewType::Layout,
        ViewType::Testbench,
        ViewType::Verilog,
        ViewType::VerilogA,
        ViewType::Spice,
        ViewType::Document,
        ViewType::Extracted,
        ViewType::Abstract,
        ViewType::Config,
        ViewType::Custom,
    ];
}

// =============================================================================
// View
// =============================================================================

/// A single view within a cell
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct View {
    /// View name (e.g., "schematic", "symbol")
    pub name: String,
    /// View type
    pub view_type: ViewType,
    /// Path to view file on disk
    pub file_path: Option<PathBuf>,
    /// Last modified timestamp (Unix epoch)
    pub modified_time: Option<u64>,
    /// Whether view has unsaved changes
    pub modified: bool,
    /// Whether view is currently open
    pub is_open: bool,
    /// View-specific metadata
    pub metadata: HashMap<String, String>,
}

impl Default for View {
    fn default() -> Self {
        Self {
            name: String::new(),
            view_type: ViewType::Schematic,
            file_path: None,
            modified_time: None,
            modified: false,
            is_open: false,
            metadata: HashMap::new(),
        }
    }
}

impl View {
    /// Create a new view
    pub fn new(name: impl Into<String>, view_type: ViewType) -> Self {
        Self {
            name: name.into(),
            view_type,
            ..Default::default()
        }
    }

    /// Create from file path
    pub fn from_path(path: PathBuf) -> Self {
        let name = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown")
            .to_string();
        let view_type = ViewType::from_str(&name);
        Self {
            name,
            view_type,
            file_path: Some(path),
            ..Default::default()
        }
    }

    /// Set file path
    pub fn with_path(mut self, path: PathBuf) -> Self {
        self.file_path = Some(path);
        self
    }
}

// =============================================================================
// Cell
// =============================================================================

/// A design cell containing multiple views
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cell {
    /// Cell name (e.g., "opamp", "bandgap")
    pub name: String,
    /// Views within this cell
    pub views: HashMap<String, View>,
    /// Cell description/documentation
    pub description: String,
    /// Cell category (e.g., "analog", "digital", "mixed")
    pub category: String,
    /// Whether cell is expanded in browser
    pub expanded: bool,
    /// Cell-level metadata
    pub metadata: HashMap<String, String>,
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            name: String::new(),
            views: HashMap::new(),
            description: String::new(),
            category: String::new(),
            expanded: false,
            metadata: HashMap::new(),
        }
    }
}

impl Cell {
    /// Create a new cell
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            ..Default::default()
        }
    }

    /// Add a view to this cell
    pub fn add_view(&mut self, view: View) {
        self.views.insert(view.name.clone(), view);
    }

    /// Get a view by name
    pub fn get_view(&self, name: &str) -> Option<&View> {
        self.views.get(name)
    }

    /// Get mutable view by name
    pub fn get_view_mut(&mut self, name: &str) -> Option<&mut View> {
        self.views.get_mut(name)
    }

    /// Check if cell has a specific view type
    pub fn has_view_type(&self, view_type: ViewType) -> bool {
        self.views.values().any(|v| v.view_type == view_type)
    }

    /// Get views sorted by name
    pub fn views_sorted(&self) -> Vec<&View> {
        let mut views: Vec<_> = self.views.values().collect();
        views.sort_by(|a, b| a.name.cmp(&b.name));
        views
    }

    /// Count of views
    pub fn view_count(&self) -> usize {
        self.views.len()
    }
}

// =============================================================================
// Library
// =============================================================================

/// A design library containing multiple cells
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Library {
    /// Library name (e.g., "my_designs", "analog_lib")
    pub name: String,
    /// Path to library directory on disk
    pub path: Option<PathBuf>,
    /// Cells within this library
    pub cells: HashMap<String, Cell>,
    /// Library technology/PDK name
    pub technology: String,
    /// Whether this is a read-only library
    pub read_only: bool,
    /// Whether library is expanded in browser
    pub expanded: bool,
    /// Library-level metadata
    pub metadata: HashMap<String, String>,
}

impl Default for Library {
    fn default() -> Self {
        Self {
            name: String::new(),
            path: None,
            cells: HashMap::new(),
            technology: String::new(),
            read_only: false,
            expanded: false,
            metadata: HashMap::new(),
        }
    }
}

impl Library {
    /// Create a new library
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            ..Default::default()
        }
    }

    /// Create a library from a directory path
    pub fn from_path(path: PathBuf) -> Self {
        let name = path
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("unnamed")
            .to_string();
        Self {
            name,
            path: Some(path),
            ..Default::default()
        }
    }

    /// Set path
    pub fn with_path(mut self, path: PathBuf) -> Self {
        self.path = Some(path);
        self
    }

    /// Set technology
    pub fn with_technology(mut self, tech: impl Into<String>) -> Self {
        self.technology = tech.into();
        self
    }

    /// Add a cell to this library
    pub fn add_cell(&mut self, cell: Cell) {
        self.cells.insert(cell.name.clone(), cell);
    }

    /// Get a cell by name
    pub fn get_cell(&self, name: &str) -> Option<&Cell> {
        self.cells.get(name)
    }

    /// Get mutable cell by name
    pub fn get_cell_mut(&mut self, name: &str) -> Option<&mut Cell> {
        self.cells.get_mut(name)
    }

    /// Get or create cell
    pub fn get_or_create_cell(&mut self, name: &str) -> &mut Cell {
        if !self.cells.contains_key(name) {
            self.cells.insert(name.to_string(), Cell::new(name));
        }
        self.cells.get_mut(name).unwrap()
    }

    /// Get cells sorted by name
    pub fn cells_sorted(&self) -> Vec<&Cell> {
        let mut cells: Vec<_> = self.cells.values().collect();
        cells.sort_by(|a, b| a.name.cmp(&b.name));
        cells
    }

    /// Count of cells
    pub fn cell_count(&self) -> usize {
        self.cells.len()
    }

    /// Total view count across all cells
    pub fn total_view_count(&self) -> usize {
        self.cells.values().map(|c| c.view_count()).sum()
    }
}

// =============================================================================
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
        if let Some(lib) = self.libraries.get(library) {
            if lib.cells.contains_key(cell) {
                self.selected_library = Some(library.to_string());
                self.selected_cell = Some(cell.to_string());
                self.selected_view = None;
            }
        }
    }

    /// Select a view
    pub fn select_view(&mut self, library: &str, cell: &str, view: &str) {
        if let Some(lib) = self.libraries.get(library) {
            if let Some(c) = lib.cells.get(cell) {
                if c.views.contains_key(view) {
                    self.selected_library = Some(library.to_string());
                    self.selected_cell = Some(cell.to_string());
                    self.selected_view = Some(view.to_string());
                }
            }
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
            if let Some(c) = lib.cells.get_mut(cell) {
                if !c.views.contains_key(view_name) {
                    c.add_view(View::new(view_name, view_type));
                    return true;
                }
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

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // ViewType Tests
    // =========================================================================

    #[test]
    fn test_view_type_default() {
        let vt = ViewType::default();
        assert_eq!(vt, ViewType::Schematic);
    }

    #[test]
    fn test_view_type_from_str() {
        assert_eq!(ViewType::from_str("schematic"), ViewType::Schematic);
        assert_eq!(ViewType::from_str("sch"), ViewType::Schematic);
        assert_eq!(ViewType::from_str("symbol"), ViewType::Symbol);
        assert_eq!(ViewType::from_str("layout"), ViewType::Layout);
        assert_eq!(ViewType::from_str("testbench"), ViewType::Testbench);
        assert_eq!(ViewType::from_str("veriloga"), ViewType::VerilogA);
        assert_eq!(ViewType::from_str("unknown"), ViewType::Custom);
    }

    #[test]
    fn test_view_type_display_name() {
        assert_eq!(ViewType::Schematic.display_name(), "schematic");
        assert_eq!(ViewType::Symbol.display_name(), "symbol");
        assert_eq!(ViewType::Testbench.display_name(), "testbench");
    }

    #[test]
    fn test_view_type_icon() {
        assert_eq!(ViewType::Schematic.icon(), "📋");
        assert_eq!(ViewType::Symbol.icon(), "🔲");
        assert_eq!(ViewType::Layout.icon(), "🗺️");
    }

    #[test]
    fn test_view_type_all() {
        assert_eq!(ViewType::ALL.len(), 12);
        assert!(ViewType::ALL.contains(&ViewType::Schematic));
        assert!(ViewType::ALL.contains(&ViewType::Layout));
    }

    // =========================================================================
    // View Tests
    // =========================================================================

    #[test]
    fn test_view_creation() {
        let view = View::new("schematic", ViewType::Schematic);
        assert_eq!(view.name, "schematic");
        assert_eq!(view.view_type, ViewType::Schematic);
        assert!(!view.modified);
        assert!(!view.is_open);
    }

    #[test]
    fn test_view_from_path() {
        let path = PathBuf::from("/designs/opamp/schematic.sch");
        let view = View::from_path(path.clone());
        assert_eq!(view.file_path, Some(path));
    }

    #[test]
    fn test_view_with_path() {
        let path = PathBuf::from("/test/path");
        let view = View::new("test", ViewType::Symbol).with_path(path.clone());
        assert_eq!(view.file_path, Some(path));
    }

    // =========================================================================
    // Cell Tests
    // =========================================================================

    #[test]
    fn test_cell_creation() {
        let cell = Cell::new("opamp");
        assert_eq!(cell.name, "opamp");
        assert!(cell.views.is_empty());
    }

    #[test]
    fn test_cell_add_view() {
        let mut cell = Cell::new("opamp");
        cell.add_view(View::new("schematic", ViewType::Schematic));
        cell.add_view(View::new("symbol", ViewType::Symbol));

        assert_eq!(cell.view_count(), 2);
        assert!(cell.get_view("schematic").is_some());
        assert!(cell.get_view("symbol").is_some());
    }

    #[test]
    fn test_cell_has_view_type() {
        let mut cell = Cell::new("opamp");
        cell.add_view(View::new("schematic", ViewType::Schematic));

        assert!(cell.has_view_type(ViewType::Schematic));
        assert!(!cell.has_view_type(ViewType::Symbol));
    }

    #[test]
    fn test_cell_views_sorted() {
        let mut cell = Cell::new("test");
        cell.add_view(View::new("symbol", ViewType::Symbol));
        cell.add_view(View::new("schematic", ViewType::Schematic));
        cell.add_view(View::new("layout", ViewType::Layout));

        let sorted = cell.views_sorted();
        assert_eq!(sorted[0].name, "layout");
        assert_eq!(sorted[1].name, "schematic");
        assert_eq!(sorted[2].name, "symbol");
    }

    // =========================================================================
    // Library Tests
    // =========================================================================

    #[test]
    fn test_library_creation() {
        let lib = Library::new("my_designs");
        assert_eq!(lib.name, "my_designs");
        assert!(lib.cells.is_empty());
        assert!(!lib.read_only);
    }

    #[test]
    fn test_library_from_path() {
        let path = PathBuf::from("/home/user/designs/my_lib");
        let lib = Library::from_path(path.clone());
        assert_eq!(lib.name, "my_lib");
        assert_eq!(lib.path, Some(path));
    }

    #[test]
    fn test_library_add_cell() {
        let mut lib = Library::new("test_lib");
        lib.add_cell(Cell::new("opamp"));
        lib.add_cell(Cell::new("bandgap"));

        assert_eq!(lib.cell_count(), 2);
        assert!(lib.get_cell("opamp").is_some());
        assert!(lib.get_cell("bandgap").is_some());
    }

    #[test]
    fn test_library_get_or_create_cell() {
        let mut lib = Library::new("test_lib");

        // First call creates
        let cell1 = lib.get_or_create_cell("new_cell");
        assert_eq!(cell1.name, "new_cell");

        // Second call returns existing
        let _ = lib.get_or_create_cell("new_cell");
        assert_eq!(lib.cell_count(), 1);
    }

    #[test]
    fn test_library_cells_sorted() {
        let mut lib = Library::new("test");
        lib.add_cell(Cell::new("zebra"));
        lib.add_cell(Cell::new("alpha"));
        lib.add_cell(Cell::new("middle"));

        let sorted = lib.cells_sorted();
        assert_eq!(sorted[0].name, "alpha");
        assert_eq!(sorted[1].name, "middle");
        assert_eq!(sorted[2].name, "zebra");
    }

    #[test]
    fn test_library_total_view_count() {
        let mut lib = Library::new("test");

        let mut cell1 = Cell::new("cell1");
        cell1.add_view(View::new("sch", ViewType::Schematic));
        cell1.add_view(View::new("sym", ViewType::Symbol));
        lib.add_cell(cell1);

        let mut cell2 = Cell::new("cell2");
        cell2.add_view(View::new("sch", ViewType::Schematic));
        lib.add_cell(cell2);

        assert_eq!(lib.total_view_count(), 3);
    }

    #[test]
    fn test_library_with_technology() {
        let lib = Library::new("test").with_technology("tsmc180nm");
        assert_eq!(lib.technology, "tsmc180nm");
    }

    // =========================================================================
    // LibraryManager Tests
    // =========================================================================

    #[test]
    fn test_manager_creation() {
        let mgr = LibraryManager::new();
        assert_eq!(mgr.library_count(), 0);
        assert!(mgr.show_read_only);
    }

    #[test]
    fn test_manager_add_library() {
        let mut mgr = LibraryManager::new();
        mgr.add_library(Library::new("lib1"));
        mgr.add_library(Library::new("lib2"));

        assert_eq!(mgr.library_count(), 2);
        assert!(mgr.get_library("lib1").is_some());
    }

    #[test]
    fn test_manager_remove_library() {
        let mut mgr = LibraryManager::new();
        mgr.add_library(Library::new("lib1"));

        let removed = mgr.remove_library("lib1");
        assert!(removed.is_some());
        assert_eq!(mgr.library_count(), 0);
    }

    #[test]
    fn test_manager_select_library() {
        let mut mgr = LibraryManager::new();
        mgr.add_library(Library::new("lib1"));

        mgr.select_library("lib1");
        assert_eq!(mgr.selected_library, Some("lib1".to_string()));
        assert!(mgr.current_library().is_some());
    }

    #[test]
    fn test_manager_select_cell() {
        let mut mgr = LibraryManager::new();
        let mut lib = Library::new("lib1");
        lib.add_cell(Cell::new("cell1"));
        mgr.add_library(lib);

        mgr.select_cell("lib1", "cell1");
        assert_eq!(mgr.selected_library, Some("lib1".to_string()));
        assert_eq!(mgr.selected_cell, Some("cell1".to_string()));
        assert!(mgr.current_cell().is_some());
    }

    #[test]
    fn test_manager_select_view() {
        let mut mgr = LibraryManager::new();
        let mut lib = Library::new("lib1");
        let mut cell = Cell::new("cell1");
        cell.add_view(View::new("schematic", ViewType::Schematic));
        lib.add_cell(cell);
        mgr.add_library(lib);

        mgr.select_view("lib1", "cell1", "schematic");
        assert!(mgr.current_view().is_some());
        assert_eq!(
            mgr.selected_lcv_path(),
            Some("lib1/cell1/schematic".to_string())
        );
    }

    #[test]
    fn test_manager_select_from_path() {
        let mut mgr = LibraryManager::new();
        let mut lib = Library::new("lib1");
        let mut cell = Cell::new("cell1");
        cell.add_view(View::new("schematic", ViewType::Schematic));
        lib.add_cell(cell);
        mgr.add_library(lib);

        mgr.select_from_path("lib1/cell1/schematic");
        assert_eq!(mgr.selected_view, Some("schematic".to_string()));
    }

    #[test]
    fn test_manager_create_cell() {
        let mut mgr = LibraryManager::new();
        mgr.add_library(Library::new("lib1"));

        assert!(mgr.create_cell("lib1", "new_cell"));
        assert!(mgr
            .get_library("lib1")
            .unwrap()
            .get_cell("new_cell")
            .is_some());
    }

    #[test]
    fn test_manager_create_cell_read_only() {
        let mut mgr = LibraryManager::new();
        let mut lib = Library::new("lib1");
        lib.read_only = true;
        mgr.add_library(lib);

        assert!(!mgr.create_cell("lib1", "new_cell"));
    }

    #[test]
    fn test_manager_create_view() {
        let mut mgr = LibraryManager::new();
        let mut lib = Library::new("lib1");
        lib.add_cell(Cell::new("cell1"));
        mgr.add_library(lib);

        assert!(mgr.create_view("lib1", "cell1", "schematic", ViewType::Schematic));
        let cell = mgr.get_library("lib1").unwrap().get_cell("cell1").unwrap();
        assert!(cell.get_view("schematic").is_some());
    }

    #[test]
    fn test_manager_search_cells() {
        let mut mgr = LibraryManager::new();
        let mut lib = Library::new("lib1");

        let mut cell1 = Cell::new("opamp");
        cell1.description = "Operational Amplifier".to_string();
        lib.add_cell(cell1);

        let mut cell2 = Cell::new("bandgap");
        cell2.description = "Bandgap Reference".to_string();
        lib.add_cell(cell2);

        mgr.add_library(lib);

        let results = mgr.search_cells("amp");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].1.name, "opamp");

        let results = mgr.search_cells("reference");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].1.name, "bandgap");
    }

    #[test]
    fn test_manager_open_views() {
        let mut mgr = LibraryManager::new();
        let mut lib = Library::new("lib1");
        let mut cell = Cell::new("cell1");

        let mut view1 = View::new("sch", ViewType::Schematic);
        view1.is_open = true;
        cell.add_view(view1);

        let mut view2 = View::new("sym", ViewType::Symbol);
        view2.is_open = false;
        cell.add_view(view2);

        lib.add_cell(cell);
        mgr.add_library(lib);

        let open = mgr.open_views();
        assert_eq!(open.len(), 1);
        assert_eq!(open[0].2.name, "sch");
    }

    #[test]
    fn test_manager_counts() {
        let mut mgr = LibraryManager::new();

        let mut lib1 = Library::new("lib1");
        let mut cell1 = Cell::new("cell1");
        cell1.add_view(View::new("sch", ViewType::Schematic));
        cell1.add_view(View::new("sym", ViewType::Symbol));
        lib1.add_cell(cell1);
        mgr.add_library(lib1);

        let mut lib2 = Library::new("lib2");
        let mut cell2 = Cell::new("cell2");
        cell2.add_view(View::new("sch", ViewType::Schematic));
        lib2.add_cell(cell2);
        mgr.add_library(lib2);

        assert_eq!(mgr.library_count(), 2);
        assert_eq!(mgr.total_cell_count(), 2);
        assert_eq!(mgr.total_view_count(), 3);
    }

    #[test]
    fn test_manager_libraries_sorted() {
        let mut mgr = LibraryManager::new();
        mgr.add_library(Library::new("zebra_lib"));
        mgr.add_library(Library::new("alpha_lib"));

        let sorted = mgr.libraries_sorted();
        assert_eq!(sorted[0].name, "alpha_lib");
        assert_eq!(sorted[1].name, "zebra_lib");
    }

    #[test]
    fn test_manager_filter_read_only() {
        let mut mgr = LibraryManager::new();
        mgr.add_library(Library::new("editable"));

        let mut ro_lib = Library::new("read_only");
        ro_lib.read_only = true;
        mgr.add_library(ro_lib);

        assert_eq!(mgr.libraries_sorted().len(), 2);

        mgr.show_read_only = false;
        assert_eq!(mgr.libraries_sorted().len(), 1);
    }

    #[test]
    fn test_manager_clear() {
        let mut mgr = LibraryManager::new();
        mgr.add_library(Library::new("lib1"));
        mgr.select_library("lib1");

        mgr.clear();
        assert_eq!(mgr.library_count(), 0);
        assert!(mgr.selected_library.is_none());
    }
}
