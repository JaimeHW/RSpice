//! Hierarchical Design Support
//!
//! Implements a professional Library/Cell/View hierarchy for managing complex
//! designs with reusable blocks. This follows the Cadence Open Access model
//! used by industry-standard IC design tools.
//!
//! # Hierarchy Model
//!
//! ```text
//! Library (project or IP collection)
//!   └── Cell (reusable block)
//!         └── View (representation)
//!               ├── schematic (circuit design)
//!               ├── symbol (graphical representation)
//!               ├── layout (physical design)
//!               └── netlist (text-based circuit)
//! ```
//!
//! # Usage
//!
//! ```ignore
//! let mut library = Library::new("my_project");
//!
//! // Create an op-amp cell
//! let opamp = library.create_cell("opamp")
//!     .with_view(View::schematic(opamp_schematic))
//!     .with_view(View::symbol(opamp_symbol));
//!
//! // Instantiate in top-level
//! let instance = HierarchyInstance::new("U1", &opamp);
//! top_schematic.add_instance(instance);
//! ```

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::state::schematic::SchematicState;
use crate::state::Point;

// =============================================================================
// Library
// =============================================================================

/// A library is a collection of cells, typically representing a project or IP.
///
/// Libraries can be:
/// - Project libraries (user's own designs)
/// - Reference libraries (vendor IP, standard cells)
/// - Technology libraries (primitives for a process)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Library {
    /// Unique library name
    pub name: String,
    /// Library path on disk
    pub path: Option<String>,
    /// Cells in this library
    pub cells: HashMap<String, Cell>,
    /// Library type/category
    pub library_type: LibraryType,
    /// Library metadata
    pub metadata: LibraryMetadata,
    /// Whether library is read-only
    pub read_only: bool,
}

/// Library type classification
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum LibraryType {
    /// User's project library
    #[default]
    Project,
    /// Reference/IP library
    Reference,
    /// Technology primitives
    Technology,
    /// Standard cell library
    StdCell,
}

/// Library metadata
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct LibraryMetadata {
    /// Description
    pub description: String,
    /// Version string
    pub version: String,
    /// Author/vendor
    pub author: String,
    /// Creation timestamp
    pub created: String,
    /// Last modified timestamp
    pub modified: String,
    /// Technology/process name
    pub technology: Option<String>,
}

impl Library {
    /// Create a new library
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            path: None,
            cells: HashMap::new(),
            library_type: LibraryType::Project,
            metadata: LibraryMetadata::default(),
            read_only: false,
        }
    }

    /// Create library with path
    pub fn with_path(mut self, path: &str) -> Self {
        self.path = Some(path.to_string());
        self
    }

    /// Set library type
    pub fn with_type(mut self, lib_type: LibraryType) -> Self {
        self.library_type = lib_type;
        self
    }

    /// Create a new cell in this library
    pub fn create_cell(&mut self, name: &str) -> &mut Cell {
        let cell = Cell::new(name, &self.name);
        self.cells.insert(name.to_string(), cell);
        self.cells.get_mut(name).unwrap()
    }

    /// Get cell by name
    pub fn get_cell(&self, name: &str) -> Option<&Cell> {
        self.cells.get(name)
    }

    /// Get mutable cell by name
    pub fn get_cell_mut(&mut self, name: &str) -> Option<&mut Cell> {
        self.cells.get_mut(name)
    }

    /// List all cell names
    pub fn cell_names(&self) -> Vec<&str> {
        self.cells.keys().map(|s| s.as_str()).collect()
    }

    /// Number of cells
    pub fn cell_count(&self) -> usize {
        self.cells.len()
    }

    /// Check if cell exists
    pub fn has_cell(&self, name: &str) -> bool {
        self.cells.contains_key(name)
    }

    /// Delete a cell
    pub fn delete_cell(&mut self, name: &str) -> bool {
        self.cells.remove(name).is_some()
    }
}

// =============================================================================
// Cell
// =============================================================================

/// A cell is a reusable design unit containing multiple views.
///
/// Cells represent logical design blocks like op-amps, filters, or full ICs.
/// Each cell can have multiple views (schematic, symbol, layout, etc.).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cell {
    /// Cell name
    pub name: String,
    /// Parent library name
    pub library: String,
    /// Views in this cell
    pub views: HashMap<String, CellView>,
    /// Cell category/type
    pub category: CellCategory,
    /// Cell properties
    pub properties: HashMap<String, String>,
    /// Interface definition (pins)
    pub interface: CellInterface,
}

/// Cell category for organization
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum CellCategory {
    /// Analog circuit block
    #[default]
    Analog,
    /// Digital logic block
    Digital,
    /// Mixed-signal block
    MixedSignal,
    /// Power/IO cell
    PowerIO,
    /// Test structure
    Test,
    /// Top-level design
    Top,
}

/// Cell interface definition (ports/pins)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CellInterface {
    /// Interface pins
    pub pins: Vec<InterfacePin>,
}

/// Interface pin definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterfacePin {
    /// Pin name
    pub name: String,
    /// Pin direction
    pub direction: PinDirection,
    /// Pin type (signal, power, ground)
    pub pin_type: PinType,
    /// Bus width (1 for single, >1 for bus)
    pub width: usize,
}

/// Pin direction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum PinDirection {
    #[default]
    Input,
    Output,
    InOut,
}

/// Pin electrical type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum PinType {
    #[default]
    Signal,
    Power,
    Ground,
    Clock,
}

impl Cell {
    /// Create a new cell
    pub fn new(name: &str, library: &str) -> Self {
        Self {
            name: name.to_string(),
            library: library.to_string(),
            views: HashMap::new(),
            category: CellCategory::Analog,
            properties: HashMap::new(),
            interface: CellInterface::default(),
        }
    }

    /// Set category
    pub fn with_category(mut self, category: CellCategory) -> Self {
        self.category = category;
        self
    }

    /// Add a view to this cell
    pub fn add_view(&mut self, view: CellView) {
        self.views.insert(view.name.clone(), view);
    }

    /// Add view (builder pattern)
    pub fn with_view(mut self, view: CellView) -> Self {
        self.add_view(view);
        self
    }

    /// Get view by name
    pub fn get_view(&self, name: &str) -> Option<&CellView> {
        self.views.get(name)
    }

    /// Get mutable view
    pub fn get_view_mut(&mut self, name: &str) -> Option<&mut CellView> {
        self.views.get_mut(name)
    }

    /// Get schematic view (convenience)
    pub fn schematic(&self) -> Option<&CellView> {
        self.views.get("schematic")
    }

    /// Get symbol view (convenience)
    pub fn symbol(&self) -> Option<&CellView> {
        self.views.get("symbol")
    }

    /// List view names
    pub fn view_names(&self) -> Vec<&str> {
        self.views.keys().map(|s| s.as_str()).collect()
    }

    /// Add interface pin
    pub fn add_pin(&mut self, pin: InterfacePin) {
        self.interface.pins.push(pin);
    }

    /// Set a property
    pub fn set_property(&mut self, key: &str, value: &str) {
        self.properties.insert(key.to_string(), value.to_string());
    }

    /// Get full cell reference (library:cell)
    pub fn full_name(&self) -> String {
        format!("{}:{}", self.library, self.name)
    }
}

impl InterfacePin {
    /// Create a new interface pin
    pub fn new(name: &str, direction: PinDirection) -> Self {
        Self {
            name: name.to_string(),
            direction,
            pin_type: PinType::Signal,
            width: 1,
        }
    }

    /// Create input pin
    pub fn input(name: &str) -> Self {
        Self::new(name, PinDirection::Input)
    }

    /// Create output pin
    pub fn output(name: &str) -> Self {
        Self::new(name, PinDirection::Output)
    }

    /// Create inout pin
    pub fn inout(name: &str) -> Self {
        Self::new(name, PinDirection::InOut)
    }

    /// Set as power pin
    pub fn power(mut self) -> Self {
        self.pin_type = PinType::Power;
        self
    }

    /// Set as ground pin
    pub fn ground(mut self) -> Self {
        self.pin_type = PinType::Ground;
        self
    }

    /// Set bus width
    pub fn bus(mut self, width: usize) -> Self {
        self.width = width;
        self
    }
}

// =============================================================================
// View
// =============================================================================

/// A view represents one aspect of a cell (schematic, symbol, layout, etc.)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CellView {
    /// View name (e.g., "schematic", "symbol")
    pub name: String,
    /// View type
    pub view_type: ViewType,
    /// View content
    pub content: ViewContent,
    /// Last modified timestamp
    pub modified: String,
}

/// View type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum ViewType {
    /// Schematic (circuit design)
    #[default]
    Schematic,
    /// Symbol (graphical representation)
    Symbol,
    /// Netlist (text-based circuit)
    Netlist,
    /// Layout (physical design) - placeholder for future
    Layout,
    /// Documentation
    Documentation,
}

/// View content - the actual data for each view type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ViewContent {
    /// Schematic content
    Schematic(Box<SchematicState>),
    /// Symbol graphics content
    Symbol(SymbolContent),
    /// Netlist text content
    Netlist(String),
    /// Empty/placeholder
    Empty,
}

impl CellView {
    /// Create a schematic view
    pub fn schematic(schematic: SchematicState) -> Self {
        Self {
            name: "schematic".to_string(),
            view_type: ViewType::Schematic,
            content: ViewContent::Schematic(Box::new(schematic)),
            modified: String::new(),
        }
    }

    /// Create a symbol view
    pub fn symbol(symbol: SymbolContent) -> Self {
        Self {
            name: "symbol".to_string(),
            view_type: ViewType::Symbol,
            content: ViewContent::Symbol(symbol),
            modified: String::new(),
        }
    }

    /// Create a netlist view
    pub fn netlist(content: &str) -> Self {
        Self {
            name: "netlist".to_string(),
            view_type: ViewType::Netlist,
            content: ViewContent::Netlist(content.to_string()),
            modified: String::new(),
        }
    }

    /// Create named view
    pub fn named(name: &str, view_type: ViewType) -> Self {
        Self {
            name: name.to_string(),
            view_type,
            content: ViewContent::Empty,
            modified: String::new(),
        }
    }

    /// Create a placeholder symbol view (for primitive components)
    ///
    /// Primitive components use built-in SVG symbols from the schematic view,
    /// so this creates an empty symbol view as a placeholder.
    pub fn symbol_placeholder() -> Self {
        Self {
            name: "symbol".to_string(),
            view_type: ViewType::Symbol,
            content: ViewContent::Empty,
            modified: String::new(),
        }
    }
}

// =============================================================================
// Symbol Content
// =============================================================================

/// Symbol graphical content for cell representation
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SymbolContent {
    /// Symbol bounding box (width, height in grid units)
    pub bounds: (i32, i32),
    /// Symbol pins with positions
    pub pins: Vec<SymbolPin>,
    /// SVG graphic content or drawing primitives
    pub graphics: SymbolGraphics,
}

/// Symbol pin with position
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SymbolPin {
    /// Pin name (matches interface pin)
    pub name: String,
    /// Position relative to symbol origin (grid units)
    pub position: Point,
    /// Pin orientation (which way pin sticks out)
    pub orientation: PinOrientation,
}

/// Pin orientation on symbol
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum PinOrientation {
    #[default]
    Left,
    Right,
    Top,
    Bottom,
}

/// Symbol graphics representation
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SymbolGraphics {
    /// SVG content for the symbol body
    pub svg: String,
    /// Drawing primitives (alternative to SVG)
    pub primitives: Vec<DrawingPrimitive>,
}

/// Drawing primitive for symbol graphics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DrawingPrimitive {
    /// Rectangle
    Rect {
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    },
    /// Line
    Line { x1: i32, y1: i32, x2: i32, y2: i32 },
    /// Circle (or ellipse)
    Circle { cx: i32, cy: i32, r: i32 },
    /// Arc
    Arc {
        cx: i32,
        cy: i32,
        r: i32,
        start_angle: f64,
        end_angle: f64,
    },
    /// Polyline
    Polyline { points: Vec<(i32, i32)> },
    /// Text label
    Text {
        x: i32,
        y: i32,
        text: String,
        anchor: TextAnchor,
    },
}

/// Text anchor position
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum TextAnchor {
    #[default]
    Start,
    Middle,
    End,
}

impl SymbolContent {
    /// Create a simple rectangular symbol
    pub fn rectangle(width: i32, height: i32, pins: Vec<SymbolPin>) -> Self {
        let graphics = SymbolGraphics {
            svg: String::new(),
            primitives: vec![DrawingPrimitive::Rect {
                x: 0,
                y: 0,
                width,
                height,
            }],
        };

        Self {
            bounds: (width, height),
            pins,
            graphics,
        }
    }
}

impl SymbolPin {
    /// Create a new symbol pin
    pub fn new(name: &str, x: i32, y: i32, orientation: PinOrientation) -> Self {
        Self {
            name: name.to_string(),
            position: Point::new(x, y),
            orientation,
        }
    }

    /// Create left-side pin
    pub fn left(name: &str, y: i32) -> Self {
        Self::new(name, 0, y, PinOrientation::Left)
    }

    /// Create right-side pin
    pub fn right(name: &str, width: i32, y: i32) -> Self {
        Self::new(name, width, y, PinOrientation::Right)
    }
}

// =============================================================================
// Hierarchy Instance
// =============================================================================

/// An instance of a cell placed in a parent schematic
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HierarchyInstance {
    /// Instance name (must be unique in parent)
    pub instance_name: String,
    /// Cell reference (library:cell)
    pub cell_ref: CellReference,
    /// Position in parent schematic (grid units)
    pub position: Point,
    /// Rotation in degrees (0, 90, 180, 270)
    pub rotation: i32,
    /// Horizontal mirror
    pub mirror_x: bool,
    /// Instance properties (override cell defaults)
    pub properties: HashMap<String, String>,
    /// Net connections (pin_name -> net_name)
    pub connections: HashMap<String, String>,
}

/// Reference to a cell (for instances)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CellReference {
    /// Library name
    pub library: String,
    /// Cell name
    pub cell: String,
    /// View name (usually "symbol" for graphical, "schematic" for simulation)
    pub view: String,
}

impl CellReference {
    /// Create a cell reference
    pub fn new(library: &str, cell: &str, view: &str) -> Self {
        Self {
            library: library.to_string(),
            cell: cell.to_string(),
            view: view.to_string(),
        }
    }

    /// Create reference to schematic view
    pub fn schematic(library: &str, cell: &str) -> Self {
        Self::new(library, cell, "schematic")
    }

    /// Create reference to symbol view
    pub fn symbol(library: &str, cell: &str) -> Self {
        Self::new(library, cell, "symbol")
    }

    /// Get full path string
    pub fn full_path(&self) -> String {
        format!("{}:{}:{}", self.library, self.cell, self.view)
    }

    //-------------------------------------------------------------------------
    // Core Integration Methods
    //-------------------------------------------------------------------------

    /// Convert to a core HierarchyPath segment
    ///
    /// Creates a path segment representing this cell reference.
    /// The format is compatible with simulation hierarchy tracking.
    pub fn to_hierarchy_segment(&self) -> String {
        self.cell.clone()
    }

    /// Create from a cell name (for quick lookups)
    pub fn from_cell_name(cell: &str) -> Self {
        Self::new("", cell, "schematic")
    }
}

impl HierarchyInstance {
    /// Create a new hierarchy instance
    pub fn new(name: &str, cell_ref: CellReference, x: i32, y: i32) -> Self {
        Self {
            instance_name: name.to_string(),
            cell_ref,
            position: Point::new(x, y),
            rotation: 0,
            mirror_x: false,
            properties: HashMap::new(),
            connections: HashMap::new(),
        }
    }

    /// Set rotation
    pub fn with_rotation(mut self, degrees: i32) -> Self {
        self.rotation = degrees % 360;
        self
    }

    /// Set mirror
    pub fn with_mirror(mut self, mirror: bool) -> Self {
        self.mirror_x = mirror;
        self
    }

    /// Connect pin to net
    pub fn connect(&mut self, pin_name: &str, net_name: &str) {
        self.connections
            .insert(pin_name.to_string(), net_name.to_string());
    }

    /// Get net connected to pin
    pub fn get_connection(&self, pin_name: &str) -> Option<&str> {
        self.connections.get(pin_name).map(|s| s.as_str())
    }

    /// Set instance property
    pub fn set_property(&mut self, key: &str, value: &str) {
        self.properties.insert(key.to_string(), value.to_string());
    }

    //-------------------------------------------------------------------------
    // Core Integration Methods
    //-------------------------------------------------------------------------

    /// Get the full hierarchical path to this instance from a parent path
    ///
    /// Appends this instance name to the parent path to create the full
    /// simulation hierarchy path.
    ///
    /// # Arguments
    /// * `parent_path` - The hierarchical path to the parent cell
    ///
    /// # Returns
    /// Full path string in format "parent.instance_name"
    pub fn full_hierarchy_path(&self, parent_path: &str) -> String {
        if parent_path.is_empty() {
            self.instance_name.clone()
        } else {
            format!("{}.{}", parent_path, self.instance_name)
        }
    }

    /// Get parameter value with string key (for simulation)
    pub fn get_param(&self, key: &str) -> Option<&str> {
        self.properties.get(key).map(|s| s.as_str())
    }

    /// Get all properties as (name, value) pairs for ParamResolver
    pub fn param_pairs(&self) -> Vec<(String, f64)> {
        self.properties
            .iter()
            .filter_map(|(k, v)| v.parse::<f64>().ok().map(|val| (k.clone(), val)))
            .collect()
    }
}

// =============================================================================
// Hierarchy Manager
// =============================================================================

/// Manages all libraries and provides hierarchy navigation
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct HierarchyManager {
    /// All loaded libraries
    pub libraries: HashMap<String, Library>,
    /// Current navigation path (stack of cell references)
    pub navigation_stack: Vec<CellReference>,
}

impl HierarchyManager {
    /// Create new hierarchy manager
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a library
    pub fn add_library(&mut self, library: Library) {
        self.libraries.insert(library.name.clone(), library);
    }

    /// Get library by name
    pub fn get_library(&self, name: &str) -> Option<&Library> {
        self.libraries.get(name)
    }

    /// Get mutable library
    pub fn get_library_mut(&mut self, name: &str) -> Option<&mut Library> {
        self.libraries.get_mut(name)
    }

    /// Resolve cell reference to actual cell
    pub fn resolve_cell(&self, cell_ref: &CellReference) -> Option<&Cell> {
        self.libraries
            .get(&cell_ref.library)
            .and_then(|lib| lib.get_cell(&cell_ref.cell))
    }

    /// Push into hierarchy (descend into instance)
    pub fn push_into(&mut self, cell_ref: CellReference) {
        self.navigation_stack.push(cell_ref);
    }

    /// Pop out of hierarchy (return to parent)
    pub fn pop_out(&mut self) -> Option<CellReference> {
        self.navigation_stack.pop()
    }

    /// Get current cell reference
    pub fn current_cell(&self) -> Option<&CellReference> {
        self.navigation_stack.last()
    }

    /// Get hierarchy depth
    pub fn depth(&self) -> usize {
        self.navigation_stack.len()
    }

    /// Get breadcrumb path
    pub fn breadcrumb(&self) -> Vec<String> {
        self.navigation_stack
            .iter()
            .map(|r| format!("{}:{}", r.library, r.cell))
            .collect()
    }

    /// List all library names
    pub fn library_names(&self) -> Vec<&str> {
        self.libraries.keys().map(|s| s.as_str()).collect()
    }

    //-------------------------------------------------------------------------
    // Core Integration Methods
    //-------------------------------------------------------------------------

    /// Get the current simulation hierarchy path string
    ///
    /// Returns the dot-separated path of cell names from the navigation stack.
    /// This format is compatible with `rspice_core::HierarchyPath`.
    ///
    /// # Example
    /// Navigation stack [project:top, project:opamp, project:input_stage]
    /// returns "top.opamp.input_stage"
    pub fn current_hierarchy_path(&self) -> String {
        self.navigation_stack
            .iter()
            .map(|r| r.cell.as_str())
            .collect::<Vec<_>>()
            .join(".")
    }

    /// Get the current path as segments (cell names only)
    ///
    /// This is useful for creating a `rspice_core::HierarchyPath`.
    pub fn path_segments(&self) -> Vec<&str> {
        self.navigation_stack
            .iter()
            .map(|r| r.cell.as_str())
            .collect()
    }

    /// Check if we're at the top level
    pub fn is_top_level(&self) -> bool {
        self.navigation_stack.len() <= 1
    }

    /// Get the separator character for hierarchy paths
    pub fn hierarchy_separator(&self) -> char {
        '.'
    }

    //-------------------------------------------------------------------------
    // Primitives Library (Built-in Components)
    //-------------------------------------------------------------------------

    /// Name of the built-in primitives library
    pub const PRIMITIVES_LIBRARY: &'static str = "primitives";

    /// Create and add the built-in primitives library containing all basic components
    ///
    /// This populates a library with cells for resistors, capacitors, inductors,
    /// voltage/current sources, transistors, diodes, and controlled sources.
    /// Each cell has a `component_type` property that maps to `ComponentType`.
    pub fn create_primitives_library(&mut self) {
        use crate::state::ComponentType;

        let mut lib = Library::new(Self::PRIMITIVES_LIBRARY).with_type(LibraryType::Technology);
        lib.read_only = true;
        lib.metadata.description = "Built-in primitive components".to_string();
        lib.metadata.author = "RSpice".to_string();

        // Helper to create a primitive cell with component type and category
        fn add_primitive(
            lib: &mut Library,
            name: &str,
            comp_type: &str,
            description: &str,
            category: &str,
            shortcut: Option<&str>,
        ) {
            let mut cell = Cell::new(name, Library::PRIMITIVES_LIBRARY_NAME);
            cell.properties
                .insert("component_type".to_string(), comp_type.to_string());
            cell.properties
                .insert("description".to_string(), description.to_string());
            cell.properties
                .insert("category".to_string(), category.to_string());
            if let Some(key) = shortcut {
                cell.properties
                    .insert("shortcut".to_string(), key.to_string());
            }
            // Add a symbol view (placeholder for now)
            cell.add_view(CellView::symbol_placeholder());
            lib.cells.insert(name.to_string(), cell);
        }

        // ===== PASSIVES =====
        add_primitive(
            &mut lib,
            "Resistor",
            "Resistor",
            "Two-terminal resistor",
            "Passives",
            Some("R"),
        );
        add_primitive(
            &mut lib,
            "Capacitor",
            "Capacitor",
            "Two-terminal capacitor",
            "Passives",
            Some("C"),
        );
        add_primitive(
            &mut lib,
            "Inductor",
            "Inductor",
            "Two-terminal inductor",
            "Passives",
            Some("L"),
        );
        add_primitive(
            &mut lib,
            "Coupled Inductors",
            "CoupledInductor",
            "Mutual inductance coupling",
            "Passives",
            None,
        );

        // ===== SOURCES =====
        add_primitive(
            &mut lib,
            "Voltage Source (DC)",
            "VoltageSource",
            "DC voltage source",
            "Sources",
            Some("V"),
        );
        add_primitive(
            &mut lib,
            "Current Source (DC)",
            "CurrentSource",
            "DC current source",
            "Sources",
            Some("I"),
        );
        add_primitive(
            &mut lib,
            "Voltage Source (AC)",
            "VoltageSourceAc",
            "AC small-signal source",
            "Sources",
            None,
        );
        add_primitive(
            &mut lib,
            "Voltage Source (Pulse)",
            "VoltageSourcePulse",
            "Pulse waveform source",
            "Sources",
            None,
        );
        add_primitive(
            &mut lib,
            "Voltage Source (Sine)",
            "VoltageSourceSin",
            "Sinusoidal waveform source",
            "Sources",
            None,
        );

        // ===== CONTROLLED SOURCES =====
        add_primitive(
            &mut lib,
            "VCVS",
            "Vcvs",
            "Voltage-controlled voltage source",
            "Controlled Sources",
            None,
        );
        add_primitive(
            &mut lib,
            "VCCS",
            "Vccs",
            "Voltage-controlled current source",
            "Controlled Sources",
            None,
        );
        add_primitive(
            &mut lib,
            "CCVS",
            "Ccvs",
            "Current-controlled voltage source",
            "Controlled Sources",
            None,
        );
        add_primitive(
            &mut lib,
            "CCCS",
            "Cccs",
            "Current-controlled current source",
            "Controlled Sources",
            None,
        );

        // ===== SEMICONDUCTORS =====
        add_primitive(
            &mut lib,
            "Diode",
            "Diode",
            "PN junction diode",
            "Semiconductors",
            Some("D"),
        );
        add_primitive(
            &mut lib,
            "NPN BJT",
            "NpnBjt",
            "NPN bipolar junction transistor",
            "Semiconductors",
            Some("Q"),
        );
        add_primitive(
            &mut lib,
            "PNP BJT",
            "PnpBjt",
            "PNP bipolar junction transistor",
            "Semiconductors",
            None,
        );
        add_primitive(
            &mut lib,
            "NMOS Transistor",
            "Nmos",
            "N-channel MOSFET",
            "Semiconductors",
            Some("M"),
        );
        add_primitive(
            &mut lib,
            "PMOS Transistor",
            "Pmos",
            "P-channel MOSFET",
            "Semiconductors",
            None,
        );
        add_primitive(
            &mut lib,
            "N-JFET",
            "Njfet",
            "N-channel junction FET",
            "Semiconductors",
            None,
        );
        add_primitive(
            &mut lib,
            "P-JFET",
            "Pjfet",
            "P-channel junction FET",
            "Semiconductors",
            None,
        );

        // ===== SPECIAL =====
        add_primitive(
            &mut lib,
            "Ground",
            "Ground",
            "Ground reference node",
            "Special",
            Some("G"),
        );

        self.add_library(lib);
    }

    /// Check if a cell is a primitive component
    pub fn is_primitive(&self, library: &str, _cell: &str) -> bool {
        library == Self::PRIMITIVES_LIBRARY
    }

    /// Get the ComponentType for a primitive cell (if applicable)
    pub fn get_component_type(&self, library: &str, cell: &str) -> Option<String> {
        self.libraries
            .get(library)
            .and_then(|lib| lib.get_cell(cell))
            .and_then(|c| c.properties.get("component_type").cloned())
    }

    //-------------------------------------------------------------------------
    // Verilog-A Library (Imported Models)
    //-------------------------------------------------------------------------

    /// Name of the Verilog-A models library
    pub const VERILOGA_LIBRARY: &'static str = "veriloga";

    /// Create the Verilog-A models library if it doesn't exist
    pub fn ensure_veriloga_library(&mut self) {
        if !self.libraries.contains_key(Self::VERILOGA_LIBRARY) {
            let mut lib = Library::new(Self::VERILOGA_LIBRARY).with_type(LibraryType::Reference);
            lib.read_only = false;
            lib.metadata.description = "Compiled Verilog-A models".to_string();
            lib.metadata.author = "User Import".to_string();
            self.add_library(lib);
        }
    }

    /// Add a Verilog-A model to the veriloga library
    ///
    /// This creates a cell with the model's name and adds its terminals as pins.
    /// The model info is stored in cell properties for later retrieval.
    pub fn add_veriloga_model(
        &mut self,
        name: &str,
        terminals: &[String],
        parameters: &[(String, String)],
        source_path: Option<&str>,
    ) {
        // Ensure the library exists
        self.ensure_veriloga_library();

        // Create the cell
        let mut cell = Cell::new(name, Self::VERILOGA_LIBRARY);

        // Add terminals as interface pins
        for terminal in terminals.iter() {
            let pin = InterfacePin::inout(terminal);
            cell.add_pin(pin);
        }

        // Store category for Project Browser grouping
        cell.properties
            .insert("category".to_string(), "Verilog-A".to_string());
        cell.properties
            .insert("component_type".to_string(), "veriloga".to_string());

        // Store source path if provided
        if let Some(path) = source_path {
            cell.properties
                .insert("source_path".to_string(), path.to_string());
        }

        // Store parameter names (comma-separated for simplicity)
        let param_names: Vec<&str> = parameters.iter().map(|(n, _)| n.as_str()).collect();
        cell.properties
            .insert("parameters".to_string(), param_names.join(","));

        // Store default parameter values
        for (param_name, default_value) in parameters {
            cell.properties
                .insert(format!("param_{}", param_name), default_value.clone());
        }

        // Add to library
        if let Some(lib) = self.libraries.get_mut(Self::VERILOGA_LIBRARY) {
            lib.cells.insert(name.to_string(), cell);
        }
    }

    /// Check if a cell is a Verilog-A model
    pub fn is_veriloga_model(&self, library: &str, _cell: &str) -> bool {
        library == Self::VERILOGA_LIBRARY
    }

    /// Get list of all Verilog-A model names
    pub fn veriloga_model_names(&self) -> Vec<String> {
        self.libraries
            .get(Self::VERILOGA_LIBRARY)
            .map(|lib| lib.cells.keys().cloned().collect())
            .unwrap_or_default()
    }
}

impl Library {
    /// Constant for primitives library name (for use in inner functions)
    pub const PRIMITIVES_LIBRARY_NAME: &'static str = "primitives";
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_library_creation() {
        let mut lib = Library::new("my_project");
        lib.create_cell("opamp");
        lib.create_cell("comparator");

        assert_eq!(lib.name, "my_project");
        assert_eq!(lib.cell_count(), 2);
        assert!(lib.has_cell("opamp"));
    }

    #[test]
    fn test_cell_views() {
        let mut cell = Cell::new("opamp", "analog_lib");
        cell.add_view(CellView::netlist(".subckt opamp..."));

        assert_eq!(cell.name, "opamp");
        assert_eq!(cell.full_name(), "analog_lib:opamp");
        assert!(cell.get_view("netlist").is_some());
    }

    #[test]
    fn test_cell_interface() {
        let mut cell = Cell::new("buffer", "std_lib");
        cell.add_pin(InterfacePin::input("A"));
        cell.add_pin(InterfacePin::output("Y"));
        cell.add_pin(InterfacePin::input("VDD").power());
        cell.add_pin(InterfacePin::input("VSS").ground());

        assert_eq!(cell.interface.pins.len(), 4);
    }

    #[test]
    fn test_hierarchy_instance() {
        let cell_ref = CellReference::symbol("analog", "opamp");
        let mut inst = HierarchyInstance::new("U1", cell_ref, 100, 200);

        inst.connect("INP", "net1");
        inst.connect("INN", "net2");
        inst.connect("OUT", "vout");

        assert_eq!(inst.get_connection("INP"), Some("net1"));
        assert_eq!(inst.position, Point::new(100, 200));
    }

    #[test]
    fn test_hierarchy_navigation() {
        let mut mgr = HierarchyManager::new();

        let mut lib = Library::new("project");
        lib.create_cell("top");
        lib.create_cell("block_a");
        mgr.add_library(lib);

        // Navigate into hierarchy
        mgr.push_into(CellReference::schematic("project", "top"));
        assert_eq!(mgr.depth(), 1);

        mgr.push_into(CellReference::schematic("project", "block_a"));
        assert_eq!(mgr.depth(), 2);

        let breadcrumb = mgr.breadcrumb();
        assert_eq!(breadcrumb, vec!["project:top", "project:block_a"]);

        // Navigate back
        mgr.pop_out();
        assert_eq!(mgr.depth(), 1);
    }
}
