//! Library Manager
//!
//! Central registry for embedded component libraries. Parses library files
//! at initialization and provides query APIs for available models and subcircuits.

use std::collections::HashMap;
use std::sync::Arc;

use super::parser::parse_library_content;
use crate::abort_signal::{AbortSignal, NoAbort};
use crate::resource::ResourceLimits;

//=============================================================================
// Embedded Library Content
//=============================================================================

// The RSpice-authored foundation pack is embedded by crate::builtin_lib.
use crate::builtin_lib::FOUNDATION_LIB;

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
    pub library: Arc<str>,
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
    pub fn new(name: String, model_type: ModelType, library: impl Into<Arc<str>>) -> Self {
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
            library: library.into(),
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
    pub library: Arc<str>,
    /// Description extracted from comments
    pub description: Option<String>,
}

impl SubcircuitDefinition {
    /// Create a new subcircuit definition
    pub fn new(name: String, pins: Vec<String>, library: impl Into<Arc<str>>) -> Self {
        Self {
            name,
            pins,
            library: library.into(),
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

        manager.load_library(FOUNDATION_LIB, "foundation.lib");

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

    /// All embedded subcircuits, ordered by name for stable catalog display.
    pub fn subcircuits(&self) -> Vec<&SubcircuitDefinition> {
        let mut subcircuits = self.subcircuits.values().collect::<Vec<_>>();
        subcircuits.sort_by(|left, right| left.name.cmp(&right.name));
        subcircuits
    }

    /// Get the library file content by name
    pub fn get_library_content(&self, library_name: &str) -> Option<&'static str> {
        match library_name.to_lowercase().as_str() {
            "foundation.lib" => Some(FOUNDATION_LIB),
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
        self.load_external_lib_with_limits_and_abort(
            path,
            section,
            ResourceLimits::default(),
            &NoAbort,
        )
    }

    /// Load an external library with explicit ingestion limits and cancellation.
    ///
    /// Parsing and definition conversion complete before the manager is
    /// mutated. Cancellation, resource violations, and parser diagnostics are
    /// therefore fail-closed and cannot register a partial library.
    pub(crate) fn load_external_lib_with_limits_and_abort(
        &mut self,
        path: impl AsRef<std::path::Path>,
        section: Option<&str>,
        resource_limits: ResourceLimits,
        abort: &dyn AbortSignal,
    ) -> Result<usize, String> {
        use super::lib_parser::LibParser;

        let path = path.as_ref();
        let base_dir = path.parent().unwrap_or(std::path::Path::new("."));

        let mut parser = LibParser::new(base_dir).with_resource_limits(resource_limits);
        let result = parser.parse_file_with_abort(path, abort).map_err(|error| {
            if error.kind() == std::io::ErrorKind::Interrupted {
                "external library load aborted".to_owned()
            } else {
                error.to_string()
            }
        })?;

        if !result.errors.is_empty() {
            return Err(format_lib_parse_errors(&result.errors));
        }

        // Get library name for display
        let lib_name = path
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("external.lib");
        let lib_name: Arc<str> = Arc::from(lib_name);
        let definitions = if let Some(section_name) = section {
            if let Some(lib_section) = result.get_section(section_name) {
                lib_section
                    .models
                    .iter()
                    .map(|model| model.to_model_definition(Arc::clone(&lib_name)))
                    .collect::<Vec<_>>()
            } else {
                return Err(format!(
                    "Section '{}' not found in library. Available: {:?}",
                    section_name,
                    result.section_names()
                ));
            }
        } else {
            result
                .top_level_models
                .iter()
                .chain(
                    result
                        .sections
                        .iter()
                        .flat_map(|section| section.models.iter()),
                )
                .map(|model| model.to_model_definition(Arc::clone(&lib_name)))
                .collect::<Vec<_>>()
        };

        if abort.is_aborted() {
            return Err("external library load aborted".to_owned());
        }
        let count = definitions.len();
        for definition in definitions {
            self.models_by_type
                .entry(definition.model_type)
                .or_default()
                .push(definition.name.clone());
            self.models.insert(definition.name.clone(), definition);
        }

        Ok(count)
    }
}

fn format_lib_parse_errors(errors: &[super::lib_parser::ParseError]) -> String {
    let mut message = format!("library parse failed with {} error(s)", errors.len());
    for error in errors {
        message.push_str("; ");
        message.push_str(&error.to_string());
    }
    message
}

impl Default for LibraryManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn external_library_names_are_owned_model_metadata() {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock after epoch")
            .as_nanos();
        let dir = std::env::temp_dir().join(format!("rspice-library-manager-{unique}"));
        fs::create_dir_all(&dir).expect("temporary directory is created");
        let lib_path = dir.join("custom_models.lib");
        fs::write(
            &lib_path,
            ".model nch_custom NMOS (LEVEL=1)\n.model pch_custom PMOS (LEVEL=1)\n",
        )
        .expect("external library fixture is written");

        let mut manager = LibraryManager::new();
        let loaded = manager
            .load_external_lib(&lib_path, None)
            .expect("external library loads");
        let nmos = manager
            .get_model("nch_custom")
            .expect("external model is registered");
        let pmos = manager
            .get_model("pch_custom")
            .expect("second external model is registered");
        let owned_library_name = nmos.library.clone();

        assert_eq!(loaded, 2);
        assert_eq!(&*owned_library_name, "custom_models.lib");
        assert!(
            Arc::ptr_eq(&nmos.library, &pmos.library),
            "models loaded from one file should share library metadata"
        );

        fs::remove_dir_all(&dir).expect("temporary directory is removed");
    }

    #[test]
    fn external_library_parser_errors_are_fatal() {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock after epoch")
            .as_nanos();
        let dir = std::env::temp_dir().join(format!("rspice-library-manager-bad-{unique}"));
        fs::create_dir_all(&dir).expect("temporary directory is created");
        let lib_path = dir.join("partial.lib");
        fs::write(
            &lib_path,
            ".include missing_models.inc\n.model should_not_load NMOS (LEVEL=1)\n",
        )
        .expect("external library fixture is written");

        let mut manager = LibraryManager::new();
        let err = manager
            .load_external_lib(&lib_path, None)
            .expect_err("parser errors must reject external libraries");

        assert!(
            err.contains("Include file not found"),
            "error should surface parser failure: {err}"
        );
        assert!(
            manager.get_model("should_not_load").is_none(),
            "partially parsed external models must not be registered"
        );

        fs::remove_dir_all(&dir).expect("temporary directory is removed");
    }

    #[test]
    fn external_library_resource_failure_and_abort_are_atomic() {
        use crate::abort_signal::ImmediateAbort;

        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock after epoch")
            .as_nanos();
        let dir = std::env::temp_dir().join(format!("rspice-library-manager-limit-{unique}"));
        fs::create_dir_all(&dir).expect("temporary directory is created");
        let lib_path = dir.join("bounded.lib");
        fs::write(&lib_path, ".model must_not_load NMOS (LEVEL=1)\n")
            .expect("external library fixture is written");

        let mut manager = LibraryManager::new();
        let initial_models = manager.model_count();
        let mut limits = ResourceLimits::default();
        limits.max_netlist_bytes = 8;
        let limit_error = manager
            .load_external_lib_with_limits_and_abort(&lib_path, None, limits, &NoAbort)
            .expect_err("resource violation must reject the entire import");
        assert!(limit_error.contains("netlist_bytes"), "{limit_error}");
        assert_eq!(manager.model_count(), initial_models);
        assert!(manager.get_model("must_not_load").is_none());

        let abort_error = manager
            .load_external_lib_with_limits_and_abort(
                &lib_path,
                None,
                ResourceLimits::default(),
                &ImmediateAbort,
            )
            .expect_err("abort must reject the entire import");
        assert_eq!(abort_error, "external library load aborted");
        assert_eq!(manager.model_count(), initial_models);
        assert!(manager.get_model("must_not_load").is_none());

        fs::remove_dir_all(&dir).expect("temporary directory is removed");
    }
}

//=============================================================================
// Tests
//=============================================================================
