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
const DIODE_LIB: &str = include_str!("../../../../lib/diode.lib");

/// Embedded MOSFET library
const MOSFET_LIB: &str = include_str!("../../../../lib/mosfet.lib");

/// Embedded OpAmp library
const OPAMP_LIB: &str = include_str!("../../../../lib/opamp.lib");

/// Embedded transistor library
const TRANSISTOR_LIB: &str = include_str!("../../../../lib/transistor.lib");

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
        }
    }

    /// Set description
    pub fn with_description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
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
}

impl Default for LibraryManager {
    fn default() -> Self {
        Self::new()
    }
}

//=============================================================================
// Tests
//=============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_library_manager_loads() {
        let manager = LibraryManager::new();
        assert!(manager.model_count() > 0, "Should have loaded models");
    }

    #[test]
    fn test_model_type_parsing() {
        assert_eq!(ModelType::from_spice_type("D"), ModelType::Diode);
        assert_eq!(ModelType::from_spice_type("NPN"), ModelType::NpnBjt);
        assert_eq!(ModelType::from_spice_type("NMOS"), ModelType::Nmos);
    }

    #[test]
    fn test_get_model() {
        let manager = LibraryManager::new();
        let model = manager.get_model("1N4148");
        assert!(model.is_some(), "Should find 1N4148");

        let model = model.unwrap();
        assert_eq!(model.model_type, ModelType::Diode);
    }

    #[test]
    fn test_models_by_type() {
        let manager = LibraryManager::new();
        let diodes = manager.models_of_type(ModelType::Diode);
        assert!(!diodes.is_empty(), "Should have diode models");
    }
}
