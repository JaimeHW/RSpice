//! Property sheet for the RF port, including its reference impedance and
//! the port number S-parameter runs index it by.

use super::*;

impl PropertyRegistry {
    /// Register the RF port (Xyce P element). With no source spec the port
    /// is a Z0 terminator; with DC/AC values it becomes a Thevenin source
    /// behind Z0.
    pub(super) fn register_rf_port(&mut self) {
        let mut sheet = PropertySheet::new();

        sheet.add(
            PropertyDefinition::new("name")
                .with_display_name("Instance Name")
                .with_type(PropertyType::String)
                .with_default(PropertyValue::string("P1"))
                .with_order(0)
                .with_category("Instance")
                .required(),
        );
        sheet.add(
            PropertyDefinition::new("port")
                .with_display_name("Port Number")
                .with_description("S-parameter port index")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(1.0))
                .with_range(1.0, 64.0)
                .with_order(10)
                .with_category("Port")
                .required(),
        );
        sheet.add(
            PropertyDefinition::new("z0")
                .with_display_name("Reference Impedance")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(50.0))
                .with_unit("Ω")
                .with_order(11)
                .with_category("Port")
                .required(),
        );
        sheet.add(
            PropertyDefinition::new("dc")
                .with_display_name("DC Voltage")
                .with_description("Optional DC bias behind Z0 (empty = pure termination)")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::string(""))
                .with_unit("V")
                .with_order(20)
                .with_category("Excitation"),
        );
        sheet.add(
            PropertyDefinition::new("ac_mag")
                .with_display_name("AC Magnitude")
                .with_description("Optional AC excitation magnitude behind Z0")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::string(""))
                .with_unit("V")
                .with_order(21)
                .with_category("Excitation"),
        );

        self.sheets.insert(ComponentType::RfPort, sheet);
    }
}
