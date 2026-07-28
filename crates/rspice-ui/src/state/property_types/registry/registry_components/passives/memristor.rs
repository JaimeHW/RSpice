//! Property sheet for the memristor, including the window function and the
//! state bounds its model integrates between.

use super::*;

impl PropertyRegistry {
    /// Register the memristor (Xyce YMEMRISTOR + TEAM LEVEL=2 model card).
    pub(super) fn register_memristor(&mut self) {
        let mut sheet = PropertySheet::new();

        sheet.add(
            PropertyDefinition::new("name")
                .with_display_name("Instance Name")
                .with_type(PropertyType::String)
                .with_default(PropertyValue::string("MR1"))
                .with_order(0)
                .with_category("Instance")
                .required(),
        );
        sheet.add(
            PropertyDefinition::new("model")
                .with_display_name("Model")
                .with_description("Library MEMRISTOR model name; overrides RON/ROFF")
                .with_type(PropertyType::String)
                .with_default(PropertyValue::string(""))
                .with_order(9)
                .with_category("Model"),
        );
        sheet.add(
            PropertyDefinition::new("ron")
                .with_display_name("On Resistance")
                .with_description("Fully-doped (low) resistance state")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::expression("50"))
                .with_unit("Ω")
                .with_order(10)
                .with_category("Electrical"),
        );
        sheet.add(
            PropertyDefinition::new("roff")
                .with_display_name("Off Resistance")
                .with_description("Undoped (high) resistance state")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::expression("1k"))
                .with_unit("Ω")
                .with_order(11)
                .with_category("Electrical"),
        );

        self.sheets.insert(ComponentType::Memristor, sheet);
    }
}
