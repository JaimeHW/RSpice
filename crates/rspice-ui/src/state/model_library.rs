//! Model Library Browser
//!
//! PDK model file navigation with corner/process selection.
//!
//! # Architecture
//!
//! Matches Cadence's model library management:
//! - **Model Library**: Collection of device models (e.g., `tsmc180.lib`)
//! - **Section/Corner**: Process corner within library (tt, ff, ss, etc.)
//! - **Model**: Individual device model (nmos, pmos, npn, etc.)
//!
//! # Example Structure
//!
//! ```text
//! tsmc180_1p8v/
//! ├── models.lib (main include file)
//! ├── nmos/
//! │   ├── nmos_3p3.va
//! │   └── nmos_1p8.va
//! └── corners/
//!     ├── tt.corner
//!     ├── ff.corner
//!     └── ss.corner
//! ```

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

// =============================================================================
// Model Types
// =============================================================================

/// Type/category of device model
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ModelType {
    /// NMOS transistor
    Nmos,
    /// PMOS transistor
    Pmos,
    /// NPN bipolar
    Npn,
    /// PNP bipolar
    Pnp,
    /// Resistor
    Resistor,
    /// Capacitor
    Capacitor,
    /// Inductor
    Inductor,
    /// Diode
    Diode,
    /// Varactor
    Varactor,
    /// RF device
    Rf,
    /// ESD protection
    Esd,
    /// Custom/other
    Other,
}

impl Default for ModelType {
    fn default() -> Self {
        ModelType::Nmos
    }
}

impl ModelType {
    /// Display name
    pub fn display_name(&self) -> &'static str {
        match self {
            ModelType::Nmos => "NMOS",
            ModelType::Pmos => "PMOS",
            ModelType::Npn => "NPN",
            ModelType::Pnp => "PNP",
            ModelType::Resistor => "Resistor",
            ModelType::Capacitor => "Capacitor",
            ModelType::Inductor => "Inductor",
            ModelType::Diode => "Diode",
            ModelType::Varactor => "Varactor",
            ModelType::Rf => "RF",
            ModelType::Esd => "ESD",
            ModelType::Other => "Other",
        }
    }

    /// Parse from string
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "nmos" | "nch" | "n" => ModelType::Nmos,
            "pmos" | "pch" | "p" => ModelType::Pmos,
            "npn" => ModelType::Npn,
            "pnp" => ModelType::Pnp,
            "r" | "res" | "resistor" => ModelType::Resistor,
            "c" | "cap" | "capacitor" => ModelType::Capacitor,
            "l" | "ind" | "inductor" => ModelType::Inductor,
            "d" | "diode" => ModelType::Diode,
            "var" | "varactor" => ModelType::Varactor,
            "rf" => ModelType::Rf,
            "esd" => ModelType::Esd,
            _ => ModelType::Other,
        }
    }

    /// Icon for UI
    pub fn icon(&self) -> &'static str {
        match self {
            ModelType::Nmos | ModelType::Pmos => "🔲",
            ModelType::Npn | ModelType::Pnp => "◉",
            ModelType::Resistor => "⏫",
            ModelType::Capacitor => "⏸️",
            ModelType::Inductor => "🔄",
            ModelType::Diode => "▶️",
            ModelType::Varactor => "◇",
            ModelType::Rf => "📶",
            ModelType::Esd => "⚡",
            ModelType::Other => "❓",
        }
    }
}

/// SPICE model level/type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ModelLevel {
    /// BSIM3 v3.3
    Bsim3v3,
    /// BSIM4
    Bsim4,
    /// BSIM CMG (FinFET)
    BsimCmg,
    /// PSP
    Psp,
    /// EKV
    Ekv,
    /// Verilog-A compact model
    VerilogA,
    /// SPICE Level 1
    SpiceLevel1,
    /// SPICE Level 3
    SpiceLevel3,
    /// Unknown/custom
    Unknown,
}

impl Default for ModelLevel {
    fn default() -> Self {
        ModelLevel::Bsim4
    }
}

impl ModelLevel {
    /// Display name
    pub fn display_name(&self) -> &'static str {
        match self {
            ModelLevel::Bsim3v3 => "BSIM3v3",
            ModelLevel::Bsim4 => "BSIM4",
            ModelLevel::BsimCmg => "BSIM-CMG",
            ModelLevel::Psp => "PSP",
            ModelLevel::Ekv => "EKV",
            ModelLevel::VerilogA => "Verilog-A",
            ModelLevel::SpiceLevel1 => "SPICE L1",
            ModelLevel::SpiceLevel3 => "SPICE L3",
            ModelLevel::Unknown => "Unknown",
        }
    }
}

// =============================================================================
// Model Definition
// =============================================================================

/// A single device model definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceModel {
    /// Model name (e.g., "nmos_3p3_svt")
    pub name: String,
    /// Model type
    pub model_type: ModelType,
    /// Model level
    pub level: ModelLevel,
    /// Description
    pub description: String,
    /// Minimum channel length (for MOS)
    pub l_min: Option<f64>,
    /// Maximum channel length
    pub l_max: Option<f64>,
    /// Minimum channel width
    pub w_min: Option<f64>,
    /// Maximum channel width
    pub w_max: Option<f64>,
    /// Operating voltage (Vdd)
    pub vdd: Option<f64>,
    /// Threshold voltage (typical)
    pub vth0: Option<f64>,
    /// Model file path
    pub file_path: Option<PathBuf>,
    /// Model parameters (key-value)
    pub parameters: HashMap<String, f64>,
}

impl Default for DeviceModel {
    fn default() -> Self {
        Self {
            name: String::new(),
            model_type: ModelType::Nmos,
            level: ModelLevel::Bsim4,
            description: String::new(),
            l_min: None,
            l_max: None,
            w_min: None,
            w_max: None,
            vdd: None,
            vth0: None,
            file_path: None,
            parameters: HashMap::new(),
        }
    }
}

impl DeviceModel {
    /// Create a new model
    pub fn new(name: impl Into<String>, model_type: ModelType) -> Self {
        Self {
            name: name.into(),
            model_type,
            ..Default::default()
        }
    }

    /// Set level
    pub fn with_level(mut self, level: ModelLevel) -> Self {
        self.level = level;
        self
    }

    /// Set geometry limits
    pub fn with_geometry(mut self, l_min: f64, l_max: f64, w_min: f64, w_max: f64) -> Self {
        self.l_min = Some(l_min);
        self.l_max = Some(l_max);
        self.w_min = Some(w_min);
        self.w_max = Some(w_max);
        self
    }

    /// Set operating voltage
    pub fn with_vdd(mut self, vdd: f64) -> Self {
        self.vdd = Some(vdd);
        self
    }

    /// Add a parameter
    pub fn add_parameter(&mut self, name: &str, value: f64) {
        self.parameters.insert(name.to_string(), value);
    }

    /// Check if geometry is within model limits
    pub fn check_geometry(&self, l: f64, w: f64) -> bool {
        let l_ok = match (self.l_min, self.l_max) {
            (Some(min), Some(max)) => l >= min && l <= max,
            (Some(min), None) => l >= min,
            (None, Some(max)) => l <= max,
            (None, None) => true,
        };
        let w_ok = match (self.w_min, self.w_max) {
            (Some(min), Some(max)) => w >= min && w <= max,
            (Some(min), None) => w >= min,
            (None, Some(max)) => w <= max,
            (None, None) => true,
        };
        l_ok && w_ok
    }
}

// =============================================================================
// Process Corner
// =============================================================================

/// A process corner definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessCorner {
    /// Corner name (e.g., "tt", "ff", "ss")
    pub name: String,
    /// Description
    pub description: String,
    /// NMOS corner (typical, fast, slow)
    pub nmos_corner: String,
    /// PMOS corner
    pub pmos_corner: String,
    /// Temperature
    pub temperature: f64,
    /// Supply voltage adjustment factor
    pub vdd_factor: f64,
    /// Corner file path
    pub file_path: Option<PathBuf>,
    /// Whether this is the default/typical corner
    pub is_default: bool,
}

impl Default for ProcessCorner {
    fn default() -> Self {
        Self {
            name: "tt".to_string(),
            description: "Typical-Typical".to_string(),
            nmos_corner: "typical".to_string(),
            pmos_corner: "typical".to_string(),
            temperature: 27.0,
            vdd_factor: 1.0,
            file_path: None,
            is_default: true,
        }
    }
}

impl ProcessCorner {
    /// Create a new corner
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            is_default: false,
            ..Default::default()
        }
    }

    /// Standard corners for a PDK
    pub fn standard_corners() -> Vec<ProcessCorner> {
        vec![
            ProcessCorner {
                name: "tt".to_string(),
                description: "Typical-Typical".to_string(),
                nmos_corner: "typical".to_string(),
                pmos_corner: "typical".to_string(),
                temperature: 27.0,
                vdd_factor: 1.0,
                is_default: true,
                ..Default::default()
            },
            ProcessCorner {
                name: "ff".to_string(),
                description: "Fast-Fast".to_string(),
                nmos_corner: "fast".to_string(),
                pmos_corner: "fast".to_string(),
                temperature: -40.0,
                vdd_factor: 1.1,
                file_path: None,
                is_default: false,
            },
            ProcessCorner {
                name: "ss".to_string(),
                description: "Slow-Slow".to_string(),
                nmos_corner: "slow".to_string(),
                pmos_corner: "slow".to_string(),
                temperature: 125.0,
                vdd_factor: 0.9,
                file_path: None,
                is_default: false,
            },
            ProcessCorner {
                name: "sf".to_string(),
                description: "Slow-Fast".to_string(),
                nmos_corner: "slow".to_string(),
                pmos_corner: "fast".to_string(),
                temperature: 27.0,
                vdd_factor: 1.0,
                file_path: None,
                is_default: false,
            },
            ProcessCorner {
                name: "fs".to_string(),
                description: "Fast-Slow".to_string(),
                nmos_corner: "fast".to_string(),
                pmos_corner: "slow".to_string(),
                temperature: 27.0,
                vdd_factor: 1.0,
                file_path: None,
                is_default: false,
            },
        ]
    }
}

// =============================================================================
// Model Library
// =============================================================================

/// A PDK model library
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelLibrary {
    /// Library name (e.g., "tsmc180_1p8v")
    pub name: String,
    /// PDK name
    pub pdk_name: String,
    /// Technology node (e.g., "180nm", "65nm")
    pub technology_node: String,
    /// Root path on disk
    pub root_path: Option<PathBuf>,
    /// Device models
    pub models: HashMap<String, DeviceModel>,
    /// Process corners
    pub corners: HashMap<String, ProcessCorner>,
    /// Currently selected corner
    pub selected_corner: Option<String>,
    /// Version string
    pub version: String,
    /// Is expanded in browser
    pub expanded: bool,
}

impl Default for ModelLibrary {
    fn default() -> Self {
        Self {
            name: String::new(),
            pdk_name: String::new(),
            technology_node: String::new(),
            root_path: None,
            models: HashMap::new(),
            corners: HashMap::new(),
            selected_corner: None,
            version: String::new(),
            expanded: false,
        }
    }
}

impl ModelLibrary {
    /// Create a new library
    pub fn new(name: impl Into<String>) -> Self {
        let mut lib = Self {
            name: name.into(),
            ..Default::default()
        };
        // Add standard corners by default
        for corner in ProcessCorner::standard_corners() {
            if corner.is_default {
                lib.selected_corner = Some(corner.name.clone());
            }
            lib.corners.insert(corner.name.clone(), corner);
        }
        lib
    }

    /// Set technology
    pub fn with_technology(mut self, pdk: impl Into<String>, node: impl Into<String>) -> Self {
        self.pdk_name = pdk.into();
        self.technology_node = node.into();
        self
    }

    /// Add a model
    pub fn add_model(&mut self, model: DeviceModel) {
        self.models.insert(model.name.clone(), model);
    }

    /// Get a model by name
    pub fn get_model(&self, name: &str) -> Option<&DeviceModel> {
        self.models.get(name)
    }

    /// Get models by type
    pub fn models_by_type(&self, model_type: ModelType) -> Vec<&DeviceModel> {
        self.models
            .values()
            .filter(|m| m.model_type == model_type)
            .collect()
    }

    /// Select a corner
    pub fn select_corner(&mut self, name: &str) -> bool {
        if self.corners.contains_key(name) {
            self.selected_corner = Some(name.to_string());
            true
        } else {
            false
        }
    }

    /// Get the selected corner
    pub fn current_corner(&self) -> Option<&ProcessCorner> {
        self.selected_corner
            .as_ref()
            .and_then(|name| self.corners.get(name))
    }

    /// Get model count
    pub fn model_count(&self) -> usize {
        self.models.len()
    }

    /// Get corner count
    pub fn corner_count(&self) -> usize {
        self.corners.len()
    }
}

// =============================================================================
// Model Library Manager
// =============================================================================

/// Manager for all model libraries
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ModelLibraryManager {
    /// All libraries
    libraries: HashMap<String, ModelLibrary>,
    /// Currently selected library
    pub selected_library: Option<String>,
    /// Search filter
    pub filter_text: String,
    /// Filter by model type
    pub filter_type: Option<ModelType>,
}

impl ModelLibraryManager {
    /// Create a new manager
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a library
    pub fn add_library(&mut self, library: ModelLibrary) {
        self.libraries.insert(library.name.clone(), library);
    }

    /// Remove a library
    pub fn remove_library(&mut self, name: &str) -> Option<ModelLibrary> {
        self.libraries.remove(name)
    }

    /// Get a library
    pub fn get_library(&self, name: &str) -> Option<&ModelLibrary> {
        self.libraries.get(name)
    }

    /// Get mutable library
    pub fn get_library_mut(&mut self, name: &str) -> Option<&mut ModelLibrary> {
        self.libraries.get_mut(name)
    }

    /// Select a library
    pub fn select_library(&mut self, name: &str) {
        if self.libraries.contains_key(name) {
            self.selected_library = Some(name.to_string());
        }
    }

    /// Get current library
    pub fn current_library(&self) -> Option<&ModelLibrary> {
        self.selected_library
            .as_ref()
            .and_then(|name| self.libraries.get(name))
    }

    /// Search for models by name
    pub fn search_models(&self, pattern: &str) -> Vec<(&ModelLibrary, &DeviceModel)> {
        let pattern_lower = pattern.to_lowercase();
        let mut results = Vec::new();

        for lib in self.libraries.values() {
            for model in lib.models.values() {
                if model.name.to_lowercase().contains(&pattern_lower)
                    || model.description.to_lowercase().contains(&pattern_lower)
                {
                    if let Some(filter_type) = self.filter_type {
                        if model.model_type == filter_type {
                            results.push((lib, model));
                        }
                    } else {
                        results.push((lib, model));
                    }
                }
            }
        }

        results
    }

    /// Get libraries sorted by name
    pub fn libraries_sorted(&self) -> Vec<&ModelLibrary> {
        let mut libs: Vec<_> = self.libraries.values().collect();
        libs.sort_by(|a, b| a.name.cmp(&b.name));
        libs
    }

    /// Total library count
    pub fn library_count(&self) -> usize {
        self.libraries.len()
    }

    /// Total model count across all libraries
    pub fn total_model_count(&self) -> usize {
        self.libraries.values().map(|l| l.model_count()).sum()
    }

    /// Clear all
    pub fn clear(&mut self) {
        self.libraries.clear();
        self.selected_library = None;
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // ModelType Tests
    // =========================================================================

    #[test]
    fn test_model_type_from_str() {
        assert_eq!(ModelType::from_str("nmos"), ModelType::Nmos);
        assert_eq!(ModelType::from_str("PMOS"), ModelType::Pmos);
        assert_eq!(ModelType::from_str("nch"), ModelType::Nmos);
        assert_eq!(ModelType::from_str("pch"), ModelType::Pmos);
        assert_eq!(ModelType::from_str("resistor"), ModelType::Resistor);
        assert_eq!(ModelType::from_str("unknown"), ModelType::Other);
    }

    #[test]
    fn test_model_type_display() {
        assert_eq!(ModelType::Nmos.display_name(), "NMOS");
        assert_eq!(ModelType::Capacitor.display_name(), "Capacitor");
    }

    // =========================================================================
    // DeviceModel Tests
    // =========================================================================

    #[test]
    fn test_device_model_creation() {
        let model = DeviceModel::new("nmos_svt", ModelType::Nmos);
        assert_eq!(model.name, "nmos_svt");
        assert_eq!(model.model_type, ModelType::Nmos);
    }

    #[test]
    fn test_device_model_with_geometry() {
        let model = DeviceModel::new("nmos_svt", ModelType::Nmos)
            .with_geometry(60e-9, 10e-6, 120e-9, 100e-6);

        assert_eq!(model.l_min, Some(60e-9));
        assert_eq!(model.w_max, Some(100e-6));
    }

    #[test]
    fn test_device_model_geometry_check() {
        let model =
            DeviceModel::new("nmos", ModelType::Nmos).with_geometry(60e-9, 10e-6, 120e-9, 100e-6);

        assert!(model.check_geometry(100e-9, 1e-6)); // Within limits
        assert!(!model.check_geometry(50e-9, 1e-6)); // L too small
        assert!(!model.check_geometry(100e-9, 200e-6)); // W too large
    }

    #[test]
    fn test_device_model_parameters() {
        let mut model = DeviceModel::new("nmos", ModelType::Nmos);
        model.add_parameter("vth0", 0.4);
        model.add_parameter("tox", 2e-9);

        assert_eq!(model.parameters.get("vth0"), Some(&0.4));
        assert_eq!(model.parameters.get("tox"), Some(&2e-9));
    }

    // =========================================================================
    // ProcessCorner Tests
    // =========================================================================

    #[test]
    fn test_process_corner_creation() {
        let corner = ProcessCorner::new("ff");
        assert_eq!(corner.name, "ff");
        assert!(!corner.is_default);
    }

    #[test]
    fn test_standard_corners() {
        let corners = ProcessCorner::standard_corners();
        assert_eq!(corners.len(), 5);

        let tt = corners.iter().find(|c| c.name == "tt").unwrap();
        assert!(tt.is_default);
        assert_eq!(tt.temperature, 27.0);

        let ff = corners.iter().find(|c| c.name == "ff").unwrap();
        assert_eq!(ff.temperature, -40.0);
        assert_eq!(ff.vdd_factor, 1.1);

        let ss = corners.iter().find(|c| c.name == "ss").unwrap();
        assert_eq!(ss.temperature, 125.0);
    }

    // =========================================================================
    // ModelLibrary Tests
    // =========================================================================

    #[test]
    fn test_model_library_creation() {
        let lib = ModelLibrary::new("tsmc180");
        assert_eq!(lib.name, "tsmc180");
        assert_eq!(lib.corner_count(), 5); // Standard corners added
        assert_eq!(lib.selected_corner, Some("tt".to_string()));
    }

    #[test]
    fn test_model_library_add_model() {
        let mut lib = ModelLibrary::new("test");
        lib.add_model(DeviceModel::new("nmos_svt", ModelType::Nmos));
        lib.add_model(DeviceModel::new("pmos_svt", ModelType::Pmos));

        assert_eq!(lib.model_count(), 2);
        assert!(lib.get_model("nmos_svt").is_some());
    }

    #[test]
    fn test_model_library_models_by_type() {
        let mut lib = ModelLibrary::new("test");
        lib.add_model(DeviceModel::new("nmos_svt", ModelType::Nmos));
        lib.add_model(DeviceModel::new("nmos_hvt", ModelType::Nmos));
        lib.add_model(DeviceModel::new("pmos_svt", ModelType::Pmos));

        let nmos_models = lib.models_by_type(ModelType::Nmos);
        assert_eq!(nmos_models.len(), 2);

        let pmos_models = lib.models_by_type(ModelType::Pmos);
        assert_eq!(pmos_models.len(), 1);
    }

    #[test]
    fn test_model_library_select_corner() {
        let mut lib = ModelLibrary::new("test");

        assert!(lib.select_corner("ff"));
        assert_eq!(lib.selected_corner, Some("ff".to_string()));

        assert!(!lib.select_corner("nonexistent"));
        assert_eq!(lib.selected_corner, Some("ff".to_string())); // Unchanged
    }

    #[test]
    fn test_model_library_current_corner() {
        let lib = ModelLibrary::new("test");
        let corner = lib.current_corner().unwrap();
        assert_eq!(corner.name, "tt");
    }

    #[test]
    fn test_model_library_with_technology() {
        let lib = ModelLibrary::new("tsmc180").with_technology("TSMC", "180nm");

        assert_eq!(lib.pdk_name, "TSMC");
        assert_eq!(lib.technology_node, "180nm");
    }

    // =========================================================================
    // ModelLibraryManager Tests
    // =========================================================================

    #[test]
    fn test_manager_creation() {
        let mgr = ModelLibraryManager::new();
        assert_eq!(mgr.library_count(), 0);
    }

    #[test]
    fn test_manager_add_library() {
        let mut mgr = ModelLibraryManager::new();
        mgr.add_library(ModelLibrary::new("lib1"));
        mgr.add_library(ModelLibrary::new("lib2"));

        assert_eq!(mgr.library_count(), 2);
    }

    #[test]
    fn test_manager_select_library() {
        let mut mgr = ModelLibraryManager::new();
        mgr.add_library(ModelLibrary::new("lib1"));

        mgr.select_library("lib1");
        assert!(mgr.current_library().is_some());
        assert_eq!(mgr.current_library().unwrap().name, "lib1");
    }

    #[test]
    fn test_manager_search_models() {
        let mut mgr = ModelLibraryManager::new();
        let mut lib = ModelLibrary::new("lib1");
        lib.add_model(DeviceModel::new("nmos_svt", ModelType::Nmos));
        lib.add_model(DeviceModel::new("nmos_hvt", ModelType::Nmos));
        lib.add_model(DeviceModel::new("pmos_svt", ModelType::Pmos));
        mgr.add_library(lib);

        let results = mgr.search_models("svt");
        assert_eq!(results.len(), 2);

        let results = mgr.search_models("nmos");
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_manager_search_with_type_filter() {
        let mut mgr = ModelLibraryManager::new();
        let mut lib = ModelLibrary::new("lib1");
        lib.add_model(DeviceModel::new("nmos_svt", ModelType::Nmos));
        lib.add_model(DeviceModel::new("pmos_svt", ModelType::Pmos));
        mgr.add_library(lib);

        mgr.filter_type = Some(ModelType::Nmos);
        let results = mgr.search_models("svt");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].1.model_type, ModelType::Nmos);
    }

    #[test]
    fn test_manager_total_model_count() {
        let mut mgr = ModelLibraryManager::new();

        let mut lib1 = ModelLibrary::new("lib1");
        lib1.add_model(DeviceModel::new("m1", ModelType::Nmos));
        lib1.add_model(DeviceModel::new("m2", ModelType::Pmos));
        mgr.add_library(lib1);

        let mut lib2 = ModelLibrary::new("lib2");
        lib2.add_model(DeviceModel::new("m3", ModelType::Resistor));
        mgr.add_library(lib2);

        assert_eq!(mgr.total_model_count(), 3);
    }

    #[test]
    fn test_manager_libraries_sorted() {
        let mut mgr = ModelLibraryManager::new();
        mgr.add_library(ModelLibrary::new("zebra"));
        mgr.add_library(ModelLibrary::new("alpha"));

        let sorted = mgr.libraries_sorted();
        assert_eq!(sorted[0].name, "alpha");
        assert_eq!(sorted[1].name, "zebra");
    }

    #[test]
    fn test_manager_clear() {
        let mut mgr = ModelLibraryManager::new();
        mgr.add_library(ModelLibrary::new("lib1"));
        mgr.select_library("lib1");

        mgr.clear();
        assert_eq!(mgr.library_count(), 0);
        assert!(mgr.selected_library.is_none());
    }
}
