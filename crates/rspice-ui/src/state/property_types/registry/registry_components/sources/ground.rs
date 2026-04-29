use super::shared::source_symbol_property;
use super::*;

impl PropertyRegistry {
    pub(super) fn register_ground(&mut self) {
        let mut sheet = PropertySheet::new();
        sheet.add(
            PropertyDefinition::new("name")
                .with_display_name("Net Name")
                .with_description("Displayed ground net label")
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
