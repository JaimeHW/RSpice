//! Library Manager
//!
//! Central registry for embedded component libraries. Parses library files
//! at initialization and provides query APIs for available models and subcircuits.

use std::collections::HashMap;

use super::parser::parse_library_content;

//=============================================================================
// Embedded Library Content
//=============================================================================

/// Embedded diode library
const DIODE_LIB: &str = include_str!("../../models/spice/diode.lib");

/// Embedded MOSFET library
const MOSFET_LIB: &str = include_str!("../../models/spice/mosfet.lib");

/// Embedded OpAmp library
const OPAMP_LIB: &str = include_str!("../../models/spice/opamp.lib");

/// Embedded transistor library
const TRANSISTOR_LIB: &str = include_str!("../../models/spice/transistor.lib");

//=============================================================================
// Model Types
//=============================================================================

/// Type of SPICE model
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ModelType {
    /// Diode (D)
    Diode,
    /// NPN Bipolar Junction Transistor
    NpnBjt,
    /// PNP Bipolar Junction Transistor
    PnpBjt,
    /// N-channel MOSFET
    Nmos,
    /// P-channel MOSFET
    Pmos,
    /// N-channel JFET
    Njfet,
    /// P-channel JFET
    Pjfet,
    /// Resistor model
    Resistor,
    /// Capacitor model
    Capacitor,
    /// Unknown/other
    Other,
}

impl ModelType {
    /// Parse model type from SPICE model type string
    pub fn from_spice_type(s: &str) -> Self {
        match s.to_uppercase().as_str() {
            "D" => ModelType::Diode,
            "NPN" => ModelType::NpnBjt,
            "PNP" => ModelType::PnpBjt,
            "NMOS" => ModelType::Nmos,
            "PMOS" => ModelType::Pmos,
            "NJF" => ModelType::Njfet,
            "PJF" => ModelType::Pjfet,
            "R" => ModelType::Resistor,
            "C" => ModelType::Capacitor,
            _ => ModelType::Other,
        }
    }

    /// Get display name for the model type
    pub fn display_name(&self) -> &'static str {
        match self {
            ModelType::Diode => "Diodes",
            ModelType::NpnBjt => "NPN Transistors",
            ModelType::PnpBjt => "PNP Transistors",
            ModelType::Nmos => "N-MOSFETs",
            ModelType::Pmos => "P-MOSFETs",
            ModelType::Njfet => "N-JFETs",
            ModelType::Pjfet => "P-JFETs",
            ModelType::Resistor => "Resistor Models",
            ModelType::Capacitor => "Capacitor Models",
            ModelType::Other => "Other",
        }
    }

    /// Get SPICE letter prefix for this model type
    pub fn spice_prefix(&self) -> &'static str {
        match self {
            ModelType::Diode => "D",
            ModelType::NpnBjt | ModelType::PnpBjt => "Q",
            ModelType::Nmos | ModelType::Pmos => "M",
            ModelType::Njfet | ModelType::Pjfet => "J",
            ModelType::Resistor => "R",
            ModelType::Capacitor => "C",
            ModelType::Other => "X",
        }
    }
}

//=============================================================================
// Model Definition
//=============================================================================

/// Definition of a SPICE model from a library
#[derive(Debug, Clone)]
pub struct ModelDefinition {
    /// Model name (e.g., "1N4148")
    pub name: String,
    /// Model type (Diode, NPN, NMOS, etc.)
    pub model_type: ModelType,
    /// Source library file name
    pub library: &'static str,
    /// Description extracted from comments
    pub description: Option<String>,
    /// Number of terminals
    pub terminals: usize,

    //=========================================================================
    // Model Binning Parameters (for PDK geometry-based selection)
    //=========================================================================
    /// Minimum channel length for this bin (meters)
    pub lmin: Option<f64>,
    /// Maximum channel length for this bin (meters)
    pub lmax: Option<f64>,
    /// Minimum channel width for this bin (meters)
    pub wmin: Option<f64>,
    /// Maximum channel width for this bin (meters)
    pub wmax: Option<f64>,
    /// Bin name prefix (e.g., "nch" from "nch.1")
    pub bin_prefix: Option<String>,
}

impl ModelDefinition {
    /// Create a new model definition
    pub fn new(name: String, model_type: ModelType, library: &'static str) -> Self {
        let terminals = match model_type {
            ModelType::Diode => 2,
            ModelType::NpnBjt | ModelType::PnpBjt => 3,
            ModelType::Nmos | ModelType::Pmos => 4,
            ModelType::Njfet | ModelType::Pjfet => 3,
            _ => 2,
        };

        Self {
            name,
            model_type,
            library,
            description: None,
            terminals,
            lmin: None,
            lmax: None,
            wmin: None,
            wmax: None,
            bin_prefix: None,
        }
    }

    /// Set description
    pub fn with_description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }

    /// Set binning parameters
    pub fn with_binning(
        mut self,
        lmin: Option<f64>,
        lmax: Option<f64>,
        wmin: Option<f64>,
        wmax: Option<f64>,
    ) -> Self {
        self.lmin = lmin;
        self.lmax = lmax;
        self.wmin = wmin;
        self.wmax = wmax;
        self
    }

    /// Set bin prefix
    pub fn with_bin_prefix(mut self, prefix: impl Into<String>) -> Self {
        self.bin_prefix = Some(prefix.into());
        self
    }

    /// Check if this model matches the given geometry (W, L)
    /// Returns true if the geometry falls within this model's bin range
    pub fn matches_geometry(&self, width: f64, length: f64) -> bool {
        // Check length bounds
        if let Some(lmin) = self.lmin
            && length < lmin
        {
            return false;
        }
        if let Some(lmax) = self.lmax
            && length > lmax
        {
            return false;
        }

        // Check width bounds
        if let Some(wmin) = self.wmin
            && width < wmin
        {
            return false;
        }
        if let Some(wmax) = self.wmax
            && width > wmax
        {
            return false;
        }

        true
    }

    /// Check if this model has any binning constraints
    pub fn has_binning(&self) -> bool {
        self.lmin.is_some() || self.lmax.is_some() || self.wmin.is_some() || self.wmax.is_some()
    }
}

//=============================================================================
// Subcircuit Definition
//=============================================================================

/// Definition of a SPICE subcircuit from a library
#[derive(Debug, Clone)]
pub struct SubcircuitDefinition {
    /// Subcircuit name (e.g., "LM741")
    pub name: String,
    /// Pin names in order
    pub pins: Vec<String>,
    /// Source library file name
    pub library: &'static str,
    /// Description extracted from comments
    pub description: Option<String>,
}

impl SubcircuitDefinition {
    /// Create a new subcircuit definition
    pub fn new(name: String, pins: Vec<String>, library: &'static str) -> Self {
        Self {
            name,
            pins,
            library,
            description: None,
        }
    }

    /// Set description
    pub fn with_description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }

    /// Get number of pins
    pub fn pin_count(&self) -> usize {
        self.pins.len()
    }
}

//=============================================================================
// Library Manager
//=============================================================================

/// Main library manager for embedded SPICE component libraries
///
/// Provides access to all models and subcircuits available in the embedded libraries.
/// Parsed once at initialization for efficient queries.
#[derive(Debug)]
pub struct LibraryManager {
    /// All parsed model definitions
    models: HashMap<String, ModelDefinition>,
    /// All parsed subcircuit definitions
    subcircuits: HashMap<String, SubcircuitDefinition>,
    /// Models grouped by type for efficient category queries
    models_by_type: HashMap<ModelType, Vec<String>>,
}

impl LibraryManager {
    /// Create a new library manager with all embedded libraries loaded
    pub fn new() -> Self {
        let mut manager = Self {
            models: HashMap::new(),
            subcircuits: HashMap::new(),
            models_by_type: HashMap::new(),
        };

        // Parse all embedded libraries
        manager.load_library(DIODE_LIB, "diode.lib");
        manager.load_library(MOSFET_LIB, "mosfet.lib");
        manager.load_library(OPAMP_LIB, "opamp.lib");
        manager.load_library(TRANSISTOR_LIB, "transistor.lib");

        manager
    }

    /// Load and parse a library's content
    fn load_library(&mut self, content: &'static str, library_name: &'static str) {
        let (models, subcircuits) = parse_library_content(content, library_name);

        for model in models {
            // Add to type index
            self.models_by_type
                .entry(model.model_type)
                .or_default()
                .push(model.name.clone());

            self.models.insert(model.name.clone(), model);
        }

        for subckt in subcircuits {
            self.subcircuits.insert(subckt.name.clone(), subckt);
        }
    }

    /// Get all available model types (sorted)
    pub fn available_types(&self) -> Vec<ModelType> {
        let mut types: Vec<_> = self.models_by_type.keys().copied().collect();
        types.sort_by_key(|t| t.display_name());
        types
    }

    /// Get all models of a specific type
    pub fn models_of_type(&self, model_type: ModelType) -> Vec<&ModelDefinition> {
        self.models_by_type
            .get(&model_type)
            .map(|names| {
                let mut models: Vec<_> = names
                    .iter()
                    .filter_map(|name| self.models.get(name))
                    .collect();
                models.sort_by(|a, b| a.name.cmp(&b.name));
                models
            })
            .unwrap_or_default()
    }

    /// Get a specific model by name
    pub fn get_model(&self, name: &str) -> Option<&ModelDefinition> {
        // Try exact match first
        if let Some(model) = self.models.get(name) {
            return Some(model);
        }
        // Try case-insensitive match
        let upper = name.to_uppercase();
        self.models
            .values()
            .find(|m| m.name.to_uppercase() == upper)
    }

    /// Select the best matching model for given geometry and model type
    ///
    /// This is the core PDK binning API. Given a model name prefix (e.g., "nch")
    /// and device geometry (W, L), it finds the model bin whose lmin/lmax/wmin/wmax
    /// range contains the specified geometry.
    ///
    /// # Arguments
    /// * `prefix` - Model name prefix to search for (e.g., "nch", "pch_hvt")
    /// * `width` - Device width in meters
    /// * `length` - Device length in meters
    /// * `model_type` - Expected model type (e.g., Nmos, Pmos)
    ///
    /// # Returns
    /// * `Some(&ModelDefinition)` - Best matching model for the geometry
    /// * `None` - No matching bin found
    ///
    /// # Selection Algorithm
    /// 1. Find all models with matching prefix and type
    /// 2. Filter to those whose binning range contains (W, L)
    /// 3. If multiple match, prefer the one with tightest bounds (smallest bin)
    /// 4. If no binned models match, fall back to unbinned model with same prefix
    pub fn select_model_for_geometry(
        &self,
        prefix: &str,
        width: f64,
        length: f64,
        model_type: ModelType,
    ) -> Option<&ModelDefinition> {
        let prefix_lower = prefix.to_lowercase();

        // Collect all candidate models matching prefix and type
        let candidates: Vec<_> = self
            .models
            .values()
            .filter(|m| {
                m.model_type == model_type && {
                    // Match by explicit bin_prefix or by name prefix
                    if let Some(ref bp) = m.bin_prefix {
                        bp.to_lowercase() == prefix_lower
                    } else {
                        m.name.to_lowercase().starts_with(&prefix_lower)
                    }
                }
            })
            .collect();

        if candidates.is_empty() {
            return None;
        }

        // Find binned models that match the geometry
        let binned_matches: Vec<_> = candidates
            .iter()
            .filter(|m| m.has_binning() && m.matches_geometry(width, length))
            .collect();

        if !binned_matches.is_empty() {
            // If multiple binned models match, prefer the one with tightest bounds
            // (smallest difference between max and min for both L and W)
            return binned_matches
                .into_iter()
                .min_by(|a, b| {
                    let a_range = Self::bin_range_size(a);
                    let b_range = Self::bin_range_size(b);
                    a_range
                        .partial_cmp(&b_range)
                        .unwrap_or(std::cmp::Ordering::Equal)
                })
                .copied();
        }

        // Fall back to unbinned model with exact prefix match
        candidates
            .iter()
            .find(|m| !m.has_binning() && m.name.to_lowercase() == prefix_lower)
            .copied()
            // Or any unbinned model with matching prefix
            .or_else(|| candidates.iter().find(|m| !m.has_binning()).copied())
    }

    /// Calculate total bin range size for prioritizing tighter bins
    fn bin_range_size(model: &ModelDefinition) -> f64 {
        let l_range = match (model.lmin, model.lmax) {
            (Some(min), Some(max)) => max - min,
            _ => f64::MAX,
        };
        let w_range = match (model.wmin, model.wmax) {
            (Some(min), Some(max)) => max - min,
            _ => f64::MAX,
        };
        l_range + w_range
    }

    /// Get all subcircuits
    pub fn all_subcircuits(&self) -> Vec<&SubcircuitDefinition> {
        let mut subcircuits: Vec<_> = self.subcircuits.values().collect();
        subcircuits.sort_by(|a, b| a.name.cmp(&b.name));
        subcircuits
    }

    /// Get a specific subcircuit by name
    pub fn get_subcircuit(&self, name: &str) -> Option<&SubcircuitDefinition> {
        // Try exact match first
        if let Some(subckt) = self.subcircuits.get(name) {
            return Some(subckt);
        }
        // Try case-insensitive match
        let upper = name.to_uppercase();
        self.subcircuits
            .values()
            .find(|s| s.name.to_uppercase() == upper)
    }

    /// Get the library file content by name
    pub fn get_library_content(&self, library_name: &str) -> Option<&'static str> {
        match library_name.to_lowercase().as_str() {
            "diode.lib" => Some(DIODE_LIB),
            "mosfet.lib" => Some(MOSFET_LIB),
            "opamp.lib" => Some(OPAMP_LIB),
            "transistor.lib" => Some(TRANSISTOR_LIB),
            _ => None,
        }
    }

    /// Get total model count
    pub fn model_count(&self) -> usize {
        self.models.len()
    }

    /// Get total subcircuit count
    pub fn subcircuit_count(&self) -> usize {
        self.subcircuits.len()
    }

    /// Load an external .lib file with optional section/corner selection
    ///
    /// # Arguments
    /// * `path` - Path to the .lib file
    /// * `section` - Optional section name (e.g., "TT", "FF", "SS") to load specific corner
    ///
    /// # Returns
    /// * Ok(count) - Number of models loaded
    /// * Err(message) - Error description
    pub fn load_external_lib(
        &mut self,
        path: impl AsRef<std::path::Path>,
        section: Option<&str>,
    ) -> Result<usize, String> {
        use super::lib_parser::LibParser;

        let path = path.as_ref();
        let base_dir = path.parent().unwrap_or(std::path::Path::new("."));

        let mut parser = LibParser::new(base_dir);
        let result = parser.parse_file(path).map_err(|e| e.to_string())?;

        if !result.errors.is_empty() {
            // Log warnings but continue
            for err in &result.errors {
                eprintln!("Warning: {}", err);
            }
        }

        let mut count = 0;

        // Get library name for display
        let lib_name = path
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("external.lib");
        let lib_name: &'static str = Box::leak(lib_name.to_string().into_boxed_str());

        // Load models based on section selection
        if let Some(section_name) = section {
            // Load only the specified section
            if let Some(lib_section) = result.get_section(section_name) {
                for model in &lib_section.models {
                    let def = model.to_model_definition(lib_name);
                    self.models_by_type
                        .entry(def.model_type)
                        .or_default()
                        .push(def.name.clone());
                    self.models.insert(def.name.clone(), def);
                    count += 1;
                }
            } else {
                return Err(format!(
                    "Section '{}' not found in library. Available: {:?}",
                    section_name,
                    result.section_names()
                ));
            }
        } else {
            // Load all top-level models and all sections
            for model in &result.top_level_models {
                let def = model.to_model_definition(lib_name);
                self.models_by_type
                    .entry(def.model_type)
                    .or_default()
                    .push(def.name.clone());
                self.models.insert(def.name.clone(), def);
                count += 1;
            }

            for section in &result.sections {
                for model in &section.models {
                    let def = model.to_model_definition(lib_name);
                    self.models_by_type
                        .entry(def.model_type)
                        .or_default()
                        .push(def.name.clone());
                    self.models.insert(def.name.clone(), def);
                    count += 1;
                }
            }
        }

        Ok(count)
    }

    /// Get available sections/corners from a .lib file without loading
    pub fn peek_lib_sections(path: impl AsRef<std::path::Path>) -> Result<Vec<String>, String> {
        use super::lib_parser::LibParser;

        let path = path.as_ref();
        let base_dir = path.parent().unwrap_or(std::path::Path::new("."));

        let mut parser = LibParser::new(base_dir);
        let result = parser.parse_file(path).map_err(|e| e.to_string())?;

        Ok(result
            .section_names()
            .into_iter()
            .map(|s| s.to_string())
            .collect())
    }
}

impl Default for LibraryManager {
    fn default() -> Self {
        Self::new()
    }
}

//=============================================================================
// Tests
//=============================================================================
