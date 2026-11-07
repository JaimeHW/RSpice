//! Default property sheets.
//!
//! Builds the [`PropertyCatalog`] every project starts from: for each
//! component type, the parameters it exposes, their units, and their
//! defaults.

use super::*;

mod components;

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
        self.register_bom_fields();
    }

    /// Every part that can be bought carries the purchasing fields, last on
    /// its sheet. Ground, ports and probes are net and measurement constructs
    /// with nothing to buy, so their sheets stay as they are.
    fn register_bom_fields(&mut self) {
        use crate::schematic::bom::BomField;
        // Description first and full width: it is the one free-text line;
        // the identifiers and ratings pair up beneath it.
        const ROWS: [BomField; 7] = [
            BomField::Description,
            BomField::Manufacturer,
            BomField::Mpn,
            BomField::Package,
            BomField::PowerRating,
            BomField::VoltageRating,
            BomField::CurrentRating,
        ];
        for (kind, sheet) in &mut self.sheets {
            if matches!(
                kind,
                ComponentType::Ground
                    | ComponentType::Port
                    | ComponentType::LoopProbe
                    | ComponentType::RfPort
            ) {
                continue;
            }
            for (order, field) in (800..).zip(ROWS) {
                sheet.add(
                    PropertyDefinition::new(field.key())
                        .with_display_name(field.label())
                        .with_type(PropertyType::String)
                        .with_default(PropertyValue::string(""))
                        .with_order(order)
                        .with_category("Bill of materials"),
                );
            }
        }
    }
}
