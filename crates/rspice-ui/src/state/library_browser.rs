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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum ViewType {
    /// Schematic view (circuit diagram)
    #[default]
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
    pub fn from_name(s: &str) -> Self {
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
        let view_type = ViewType::from_name(&name);
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
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
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

    /// Remove a view by name
    pub fn remove_view(&mut self, name: &str) -> bool {
        self.views.remove(name).is_some()
    }
}

// =============================================================================
// Library
// =============================================================================

/// A design library containing multiple cells
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
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
        self.cells
            .entry(name.to_string())
            .or_insert_with(|| Cell::new(name))
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

    /// Remove a cell by name
    pub fn remove_cell(&mut self, name: &str) -> bool {
        self.cells.remove(name).is_some()
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

    // =========================================================================
    // Primitives Library Initialization
    // =========================================================================

    /// Primitives library name constant
    pub const PRIMITIVES_LIBRARY: &'static str = "primitives";

    /// User library name constant
    pub const USER_LIBRARY: &'static str = "user";

    /// Create a LibraryManager with built-in primitives library pre-populated
    pub fn with_primitives() -> Self {
        let mut mgr = Self::new();
        mgr.create_primitives_library();
        mgr.create_user_library();
        mgr
    }

    /// Create the built-in primitives library with all standard SPICE components
    pub fn create_primitives_library(&mut self) {
        let mut lib = Library::new(Self::PRIMITIVES_LIBRARY);
        lib.read_only = true;
        lib.technology = "SPICE".to_string();

        // Helper to add a primitive cell with schematic view
        fn add_primitive(lib: &mut Library, name: &str, category: &str, description: &str) {
            let mut cell = Cell::new(name);
            cell.description = description.to_string();
            cell.category = category.to_string();
            cell.add_view(View::new("schematic", ViewType::Schematic));
            cell.add_view(View::new("symbol", ViewType::Symbol));
            lib.add_cell(cell);
        }

        // ===== PASSIVES =====
        add_primitive(&mut lib, "Resistor", "Passives", "Two-terminal resistor");
        add_primitive(&mut lib, "Capacitor", "Passives", "Two-terminal capacitor");
        add_primitive(&mut lib, "Inductor", "Passives", "Two-terminal inductor");
        add_primitive(
            &mut lib,
            "Transformer",
            "Passives",
            "Two-winding coupled transformer",
        );
        add_primitive(&mut lib, "Ground", "Passives", "Ground reference node");

        // ===== SOURCES =====
        add_primitive(&mut lib, "VSource DC", "Sources", "DC voltage source");
        add_primitive(&mut lib, "VSource AC", "Sources", "AC small-signal source");
        add_primitive(
            &mut lib,
            "VSource Pulse",
            "Sources",
            "Pulse waveform voltage source",
        );
        add_primitive(
            &mut lib,
            "VSource Sin",
            "Sources",
            "Sinusoidal voltage source",
        );
        add_primitive(
            &mut lib,
            "VSource PWL",
            "Sources",
            "Piecewise linear voltage source",
        );
        add_primitive(
            &mut lib,
            "VSource Exp",
            "Sources",
            "Exponential voltage source",
        );
        add_primitive(
            &mut lib,
            "VSource SFFM",
            "Sources",
            "Single-frequency FM voltage source",
        );
        add_primitive(&mut lib, "ISource DC", "Sources", "DC current source");
        add_primitive(
            &mut lib,
            "ISource AC",
            "Sources",
            "AC small-signal current source",
        );
        add_primitive(
            &mut lib,
            "ISource Pulse",
            "Sources",
            "Pulse waveform current source",
        );
        add_primitive(
            &mut lib,
            "ISource Sin",
            "Sources",
            "Sinusoidal current source",
        );
        add_primitive(
            &mut lib,
            "ISource PWL",
            "Sources",
            "Piecewise linear current source",
        );
        add_primitive(
            &mut lib,
            "ISource Exp",
            "Sources",
            "Exponential current source",
        );
        add_primitive(&mut lib, "ISource Noise", "Sources", "Noise current source");

        // ===== CONTROLLED SOURCES =====
        add_primitive(
            &mut lib,
            "VCVS",
            "Controlled Sources",
            "Voltage-controlled voltage source",
        );
        add_primitive(
            &mut lib,
            "VCCS",
            "Controlled Sources",
            "Voltage-controlled current source",
        );
        add_primitive(
            &mut lib,
            "CCVS",
            "Controlled Sources",
            "Current-controlled voltage source",
        );
        add_primitive(
            &mut lib,
            "CCCS",
            "Controlled Sources",
            "Current-controlled current source",
        );

        // ===== SEMICONDUCTORS =====
        add_primitive(&mut lib, "Diode", "Semiconductors", "PN junction diode");
        add_primitive(&mut lib, "NMOS", "Semiconductors", "N-channel MOSFET");
        add_primitive(&mut lib, "PMOS", "Semiconductors", "P-channel MOSFET");
        add_primitive(
            &mut lib,
            "NPN",
            "Semiconductors",
            "NPN bipolar junction transistor",
        );
        add_primitive(
            &mut lib,
            "PNP",
            "Semiconductors",
            "PNP bipolar junction transistor",
        );
        add_primitive(
            &mut lib,
            "NJFET",
            "Semiconductors",
            "N-channel junction FET",
        );
        add_primitive(
            &mut lib,
            "PJFET",
            "Semiconductors",
            "P-channel junction FET",
        );

        self.add_library(lib);
    }

    /// Create an empty user library for custom cells
    pub fn create_user_library(&mut self) {
        let lib = Library::new(Self::USER_LIBRARY);
        self.add_library(lib);
    }

    /// Check if a cell is a primitive (from the primitives library)
    pub fn is_primitive(&self, cell_name: &str) -> bool {
        self.get_library(Self::PRIMITIVES_LIBRARY)
            .and_then(|lib| lib.get_cell(cell_name))
            .is_some()
    }

    /// Get all cells in a category
    pub fn cells_in_category(&self, library: &str, category: &str) -> Vec<&Cell> {
        self.get_library(library)
            .map(|lib| {
                lib.cells
                    .values()
                    .filter(|c| c.category == category)
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Get unique categories from a library
    pub fn categories(&self, library: &str) -> Vec<String> {
        self.get_library(library)
            .map(|lib| {
                let mut cats: Vec<String> =
                    lib.cells.values().map(|c| c.category.clone()).collect();
                cats.sort();
                cats.dedup();
                cats
            })
            .unwrap_or_default()
    }
}

// =============================================================================
// Tests
// =============================================================================
