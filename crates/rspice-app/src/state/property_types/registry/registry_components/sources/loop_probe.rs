//! Property sheet for the loop probe.
//!
//! The probe has one property, and it is the one that matters: its name. That
//! name is what a stability analysis refers to, and what the engine looks up in
//! the circuit's voltage-source table.
//!
//! There is deliberately no value here. The emitted card is fixed at 0 V
//! because `.STB` refuses a probe carrying any other DC value, so a field
//! offering one could only describe a run that will not happen.

use super::*;

impl PropertyRegistry {
    pub(super) fn register_loop_probe(&mut self) {
        let mut sheet = PropertySheet::new();
        sheet.add(
            PropertyDefinition::new("name")
                .with_display_name("Instance Name")
                .with_description(
                    "Names this probe; a stability analysis refers to it by this name",
                )
                .with_type(PropertyType::String)
                .with_default(PropertyValue::string("VLOOP1"))
                .with_order(0)
                .with_category("Instance"),
        );
        self.sheets.insert(ComponentType::LoopProbe, sheet);
    }
}
