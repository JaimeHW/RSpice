use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::{Cell, CellReference, InterfacePin, Library, LibraryType};

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
        let mut lib = Library::new(Self::PRIMITIVES_LIBRARY).with_type(LibraryType::Technology);
        lib.read_only = true;
        lib.metadata.description = "Built-in primitive components".to_string();
        lib.metadata.author = "RSpice".to_string();

        fn primitive_interface_pins(comp_type: &str) -> Vec<InterfacePin> {
            match comp_type {
                "Resistor" | "Capacitor" | "Inductor" | "VoltageSource" | "CurrentSource"
                | "VoltageSourceAc" | "VoltageSourcePulse" | "VoltageSourceSin" | "Diode" => {
                    vec![InterfacePin::inout("p"), InterfacePin::inout("n")]
                }
                "CoupledInductor" => vec![
                    InterfacePin::inout("p1"),
                    InterfacePin::inout("n1"),
                    InterfacePin::inout("p2"),
                    InterfacePin::inout("n2"),
                ],
                "Vcvs" | "Vccs" | "Ccvs" | "Cccs" => vec![
                    InterfacePin::inout("outp"),
                    InterfacePin::inout("outn"),
                    InterfacePin::input("ctrlp"),
                    InterfacePin::input("ctrln"),
                ],
                "NpnBjt" | "PnpBjt" => vec![
                    InterfacePin::inout("c"),
                    InterfacePin::input("b"),
                    InterfacePin::inout("e"),
                ],
                "Nmos" | "Pmos" | "Njfet" | "Pjfet" => vec![
                    InterfacePin::inout("d"),
                    InterfacePin::input("g"),
                    InterfacePin::inout("s"),
                    InterfacePin::inout("b"),
                ],
                "Ground" => vec![InterfacePin::input("0").ground()],
                _ => vec![InterfacePin::inout("p"), InterfacePin::inout("n")],
            }
        }

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
            for pin in primitive_interface_pins(comp_type) {
                cell.add_pin(pin);
            }
            cell.ensure_symbol_view();
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
        cell.ensure_symbol_view();

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
