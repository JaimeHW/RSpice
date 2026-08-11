//! Property sheet for ground.

use super::shared::source_symbol_property;
use super::*;

impl PropertyRegistry {
    pub(super) fn register_ground(&mut self) {
        let mut sheet = PropertySheet::new();
        sheet.add(
            // Not a net name: every ground symbol binds its net to node 0,
            // whatever this says, and the canvas skips ground labels. What it
            // does name is the object, in the navigator's instance list.
            PropertyDefinition::new("name")
                .with_display_name("Instance Name")
                .with_description("Names this symbol in the design navigator; the net is always 0")
                .with_type(PropertyType::String)
                .with_default(PropertyValue::string("0"))
                .with_order(0)
                .with_category("Instance"),
        );
        sheet.add(source_symbol_property(
            "Schematic symbol skin for ground",
            "default",
            &["default", "earth", "chassis"],
        ));
        self.sheets.insert(ComponentType::Ground, sheet);
    }
}
