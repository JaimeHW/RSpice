use super::*;

#[path = "registry_components.rs"]
mod registry_components;

impl PropertyRegistry {
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
        self.sheets.get(&comp_type)
    }

    /// Register default property sheets for all standard components
    fn register_defaults(&mut self) {
        self.register_passive_components();
        self.register_sources();
        self.register_semiconductors();
        self.register_controlled_sources();
    }
}
