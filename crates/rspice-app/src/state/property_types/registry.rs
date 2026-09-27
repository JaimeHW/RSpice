//! Default property sheets.
//!
//! Builds the [`PropertyCatalog`] every project starts from: for each
//! component type, the parameters it exposes, their units, and their
//! defaults.

use super::*;

mod registry_components;

impl PropertyCatalog {
    /// Create a new registry with default property sheets for all component types
    pub fn new() -> Self {
        let mut registry = Self {
            sheets: HashMap::new(),
        };
        registry.register_defaults();
        registry
    }

    /// Get the property sheet for a component type
    pub fn get(&self, comp_type: ComponentType) -> Option<&PropertySheet> {
        if comp_type == ComponentType::CellInstance {
            return None;
        }
        self.sheets.get(&comp_type)
    }

    /// Register default property sheets for all standard components
    fn register_defaults(&mut self) {
        self.register_passive_components();
        self.register_sources();
        self.register_semiconductors();
        self.register_controlled_sources();
        self.register_interface_components();
        self.register_xspice_components();
    }
}
