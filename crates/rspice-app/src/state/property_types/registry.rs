//! Default property sheets.
//!
//! Builds the [`PropertyRegistry`] every project starts from: for each
//! component type, the parameters it exposes, their units, and their
//! defaults.

use super::*;

mod registry_components;

impl PropertyRegistry {
    /// Create a new registry with default property sheets for all component types
    pub fn new() -> Self {
        let mut registry = Self {
            sheets: HashMap::new(),
            cell_instance_sheet: None,
        };
        registry.register_defaults();
        registry
    }

    /// Get the property sheet for a component type
    pub fn get(&self, comp_type: ComponentType) -> Option<&PropertySheet> {
        if comp_type == ComponentType::CellInstance {
            return self.cell_instance_sheet.as_ref();
        }
        self.sheets.get(&comp_type)
    }

    /// Install the authoritative parameter form for one active cell-instance
    /// property transaction. The sheet is validated before it can replace the
    /// prior transaction, preventing malformed library metadata from leaking
    /// into the generic property renderer.
    pub(crate) fn install_cell_instance_sheet(
        &mut self,
        sheet: PropertySheet,
    ) -> Result<(), String> {
        let Some(identity) = sheet.get("name") else {
            return Err("cell-instance property sheet has no reference designator".to_owned());
        };
        if identity.prop_type != PropertyType::String || identity.read_only {
            return Err("cell-instance reference designator must be an editable string".to_owned());
        }
        for definition in sheet.iter().filter(|definition| definition.name != "name") {
            definition
                .validate(&definition.default_value)
                .map_err(|error| format!("invalid `{}` default: {error}", definition.name))?;
        }
        self.cell_instance_sheet = Some(sheet);
        Ok(())
    }

    /// Clear the transaction-scoped cell-instance schema. A later legacy or
    /// unbound cell can therefore never reuse the previous master's fields.
    pub(crate) fn clear_cell_instance_sheet(&mut self) {
        self.cell_instance_sheet = None;
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
