use super::{Cell, Library, LibraryManager, View, ViewType};

impl LibraryManager {
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
