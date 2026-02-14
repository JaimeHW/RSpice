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
#[derive(Default)]
pub enum ModelType {
    /// NMOS transistor
    #[default]
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
            ModelType::Nmos => "NM",
            ModelType::Pmos => "PM",
            ModelType::Npn => "QN",
            ModelType::Pnp => "QP",
            ModelType::Resistor => "R",
            ModelType::Capacitor => "C",
            ModelType::Inductor => "L",
            ModelType::Diode => "D",
            ModelType::Varactor => "VAR",
            ModelType::Rf => "RF",
            ModelType::Esd => "ESD",
            ModelType::Other => "?",
        }
    }
}

/// SPICE model level/type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[derive(Default)]
pub enum ModelLevel {
    /// BSIM3 v3.3
    Bsim3v3,
    /// BSIM4
    #[default]
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
#[derive(Default)]
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

    // =========================================================================
    // PDK Integration
    // =========================================================================

    /// Load a library from a .lib file
    ///
    /// Parses the file using the rspice-core library parser and adds models
    /// to a new library entry.
    ///
    /// # Arguments
    /// * `path` - Path to the .lib file
    /// * `section` - Optional section/corner name to load (e.g., "tt", "ff")
    ///
    /// # Returns
    /// * Ok(library_name) - Name of the created library
    /// * Err(message) - Error description
    pub fn load_library_file(
        &mut self,
        path: impl AsRef<std::path::Path>,
        section: Option<&str>,
    ) -> Result<String, String> {
        use rspice_core::library::LibParser;

        let path = path.as_ref();
        let base_dir = path.parent().unwrap_or(std::path::Path::new("."));

        // Get library name from filename
        let lib_name = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unnamed")
            .to_string();

        // Create or get existing library
        let library = self.libraries.entry(lib_name.clone()).or_insert_with(|| {
            let mut lib = ModelLibrary::new(&lib_name);
            lib.root_path = Some(path.to_path_buf());
            lib
        });

        // Parse the file
        let mut parser = LibParser::new(base_dir);
        let result = parser.parse_file(path).map_err(|e| e.to_string())?;

        // Update corners from parsed sections
        for section_name in result.section_names() {
            let corner = ProcessCorner {
                name: section_name.to_string(),
                description: format!("Process corner from {}", lib_name),
                ..ProcessCorner::default()
            };
            library.corners.insert(corner.name.clone(), corner);
        }

        // Load models based on section selection
        if let Some(section_name) = section {
            if let Some(lib_section) = result.get_section(section_name) {
                for model in &lib_section.models {
                    let device_model = Self::convert_parsed_model(model, path);
                    library
                        .models
                        .insert(device_model.name.clone(), device_model);
                }
            } else {
                return Err(format!(
                    "Section '{}' not found. Available: {:?}",
                    section_name,
                    result.section_names()
                ));
            }
        } else {
            // Load all models
            for model in &result.top_level_models {
                let device_model = Self::convert_parsed_model(model, path);
                library
                    .models
                    .insert(device_model.name.clone(), device_model);
            }

            for section in &result.sections {
                for model in &section.models {
                    let device_model = Self::convert_parsed_model(model, path);
                    library
                        .models
                        .insert(device_model.name.clone(), device_model);
                }
            }
        }

        Ok(lib_name)
    }

    /// Convert a parsed model from the core library to UI DeviceModel
    fn convert_parsed_model(
        model: &rspice_core::library::ParsedModel,
        file_path: &std::path::Path,
    ) -> DeviceModel {
        let model_type = Self::convert_core_model_type(model.model_type);

        DeviceModel {
            name: model.name.clone(),
            model_type,
            level: ModelLevel::Unknown, // Could be enhanced to detect from parameters
            description: model.description.clone().unwrap_or_default(),
            l_min: model.lmin,
            l_max: model.lmax,
            w_min: model.wmin,
            w_max: model.wmax,
            vdd: None,
            vth0: None, // Could extract from parameters
            file_path: Some(file_path.to_path_buf()),
            parameters: model.parameters.clone(),
        }
    }

    /// Convert core ModelType to UI ModelType
    fn convert_core_model_type(core_type: rspice_core::library::ModelType) -> ModelType {
        use rspice_core::library::ModelType as CoreType;
        match core_type {
            CoreType::Nmos => ModelType::Nmos,
            CoreType::Pmos => ModelType::Pmos,
            CoreType::NpnBjt => ModelType::Npn,
            CoreType::PnpBjt => ModelType::Pnp,
            CoreType::Diode => ModelType::Diode,
            CoreType::Resistor => ModelType::Resistor,
            CoreType::Capacitor => ModelType::Capacitor,
            CoreType::Njfet | CoreType::Pjfet => ModelType::Other,
            CoreType::Other => ModelType::Other,
        }
    }

    /// Get available sections/corners from a .lib file without fully loading it
    pub fn peek_library_sections(path: impl AsRef<std::path::Path>) -> Result<Vec<String>, String> {
        rspice_core::library::LibraryManager::peek_lib_sections(path)
    }

    /// Load models from all discovered files in a PdkConfig
    ///
    /// Scans all discovered .lib and .scs files and adds them as libraries.
    ///
    /// # Arguments
    /// * `pdk_config` - The PDK configuration with discovered files
    ///
    /// # Returns
    /// * Ok(count) - Number of libraries loaded
    /// * Err(errors) - List of errors encountered during loading
    pub fn load_from_pdk_config(
        &mut self,
        pdk_config: &super::pdk_config::PdkConfig,
    ) -> Result<usize, Vec<String>> {
        let mut loaded = 0;
        let mut errors = Vec::new();

        for file in &pdk_config.discovered_files {
            // Only load .lib and .scs files
            if file.extension != "lib" && file.extension != "scs" {
                continue;
            }

            match self.load_library_file(&file.path, None) {
                Ok(_) => loaded += 1,
                Err(e) => errors.push(format!("{}: {}", file.path.display(), e)),
            }
        }

        if errors.is_empty() {
            Ok(loaded)
        } else {
            Err(errors)
        }
    }

    /// Populate with built-in models from the core engine
    ///
    /// Loads the embedded model libraries (diode.lib, mosfet.lib, etc.)
    /// into UI-accessible libraries.
    pub fn load_builtin_models(&mut self) {
        let core_manager = rspice_core::library::LibraryManager::new();

        // Create a built-in library for each model type
        for model_type in core_manager.available_types() {
            let models = core_manager.models_of_type(model_type);
            if models.is_empty() {
                continue;
            }

            let lib_name = model_type.display_name().to_string();
            let library = self
                .libraries
                .entry(lib_name.clone())
                .or_insert_with(|| ModelLibrary::new(&lib_name));

            for model in models {
                let device_model = DeviceModel {
                    name: model.name.clone(),
                    model_type: Self::convert_core_model_type(model.model_type),
                    level: ModelLevel::Unknown,
                    description: model.description.clone().unwrap_or_default(),
                    l_min: model.lmin,
                    l_max: model.lmax,
                    w_min: model.wmin,
                    w_max: model.wmax,
                    vdd: None,
                    vth0: None,
                    file_path: None,
                    parameters: HashMap::new(),
                };
                library
                    .models
                    .insert(device_model.name.clone(), device_model);
            }
        }
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests;
