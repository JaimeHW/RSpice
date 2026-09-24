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
        // Phase only means anything alongside a magnitude, and only a
        // multi-port drive needs it — but the lowered source carries it into
        // `.AC` exactly as a voltage source's would, so leaving it out made a
        // driven port unable to express anything but zero degrees.
        sheet.add(
            PropertyDefinition::new("ac_phase")
                .with_display_name("AC Phase")
                .with_description("Excitation phase; applied only when a magnitude is set")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::string(""))
                .with_unit("°")
                .with_order(22)
                .with_category("Excitation"),
        );

        // Large-signal RF drive. `pwr` is the power a matched load actually
        // receives, as it is on an ADS or Spectre port: the generator sits
        // behind Z0 and drops half its EMF there, so the lowered source carries
        // sqrt(8·P·Z0). An ngspice `portnum=` deck keeps ngspice's own
        // sqrt(4·P·Z0), because there the source drives the plane directly.
        sheet.add(
            PropertyDefinition::new("pwr")
                .with_display_name("Available Power")
                .with_description("Power delivered into a matched Z0; empty = no RF drive")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::string(""))
                .with_unit("W")
                .with_order(30)
                .with_category("RF Drive"),
        );
        sheet.add(
            PropertyDefinition::new("freq")
                .with_display_name("Drive Frequency")
                .with_description("Applied only when an available power is set; defaults to 1 GHz")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::string(""))
                .with_unit("Hz")
                .with_order(31)
                .with_category("RF Drive"),
        );
        sheet.add(
            PropertyDefinition::new("phase")
                .with_display_name("Drive Phase")
                .with_description("Applied only when an available power is set")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::string(""))
                .with_unit("°")
                .with_order(32)
                .with_category("RF Drive"),
        );

        self.sheets.insert(ComponentType::RfPort, sheet);
    }
}
