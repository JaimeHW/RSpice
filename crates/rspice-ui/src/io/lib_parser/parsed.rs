use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

use super::parser::LibraryParser;
use super::types::{IncludeDirective, LibrarySection, ModelDef, ParamValue, SubcircuitDef};

// =============================================================================
// Parsed Library
// =============================================================================

/// A fully parsed library file
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ParsedLibrary {
    /// Library name
    pub name: String,
    /// Source file path
    pub source_path: Option<PathBuf>,
    /// Sections (corners)
    pub sections: HashMap<String, LibrarySection>,
    /// Global models (not in any section)
    pub global_models: HashMap<String, ModelDef>,
    /// Global subcircuits
    pub global_subcircuits: HashMap<String, SubcircuitDef>,
    /// Global parameters
    pub global_parameters: HashMap<String, ParamValue>,
    /// Include directives
    pub includes: Vec<IncludeDirective>,
}

impl ParsedLibrary {
    /// Parse from string
    pub fn parse(input: &str) -> Result<Self, String> {
        let mut parser = LibraryParser::new(input)?;
        parser.parse()
    }

    /// Parse from file
    pub fn from_file(path: &Path) -> Result<Self, String> {
        let content =
            std::fs::read_to_string(path).map_err(|e| format!("Failed to read file: {}", e))?;
        let mut library = content.parse::<Self>()?;
        library.source_path = Some(path.to_path_buf());
        Ok(library)
    }

    /// Get available section names (corners)
    pub fn section_names(&self) -> Vec<&str> {
        self.sections.keys().map(|s| s.as_str()).collect()
    }

    /// Get a section by name
    pub fn get_section(&self, name: &str) -> Option<&LibrarySection> {
        self.sections.get(name)
    }

    /// Get all models (global + from specified section)
    pub fn models_for_section(&self, section: &str) -> HashMap<String, &ModelDef> {
        let mut models: HashMap<String, &ModelDef> = HashMap::new();

        // Add global models
        for (name, model) in &self.global_models {
            models.insert(name.clone(), model);
        }

        // Add section models (override globals)
        if let Some(sec) = self.sections.get(section) {
            for (name, model) in &sec.models {
                models.insert(name.clone(), model);
            }
        }

        models
    }

    /// Total model count
    pub fn model_count(&self) -> usize {
        let section_models: usize = self.sections.values().map(|s| s.models.len()).sum();
        self.global_models.len() + section_models
    }

    /// Total subcircuit count
    pub fn subcircuit_count(&self) -> usize {
        let section_subcircuits: usize = self.sections.values().map(|s| s.subcircuits.len()).sum();
        self.global_subcircuits.len() + section_subcircuits
    }
}

impl std::str::FromStr for ParsedLibrary {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Self::parse(input)
    }
}
