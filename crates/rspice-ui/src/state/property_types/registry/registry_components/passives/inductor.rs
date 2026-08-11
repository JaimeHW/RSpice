//! Property sheet for the inductor.

use super::*;

impl PropertyRegistry {
    /// Register Inductor with commercial-grade parameters
    pub(super) fn register_inductor(&mut self) {
        let mut sheet = PropertySheet::new();

        // Instance category
        sheet.add(
            PropertyDefinition::new("name")
                .with_display_name("Instance Name")
                .with_type(PropertyType::String)
                .with_default(PropertyValue::string("L1"))
                .with_order(0)
                .with_category("Instance")
                .required(),
        );

        // Electrical category
        sheet.add(
            PropertyDefinition::new("l")
                .with_display_name("Inductance")
                .with_description("Inductance value in Henries")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::expression("1m"))
                .with_unit("H")
                .with_range(0.0, 1e6)
                .with_order(10)
                .with_category("Electrical")
                .required(),
        );
        sheet.add(
            PropertyDefinition::new("m")
                .with_display_name("Multiplier")
                .with_description("Number of parallel inductors")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(1.0))
                .with_range(1.0, 10000.0)
                .with_order(11)
                .with_category("Electrical"),
        );
        sheet.add(
            PropertyDefinition::new("scale")
                .with_display_name("Scale Factor")
                .with_description("Inductance scaling factor")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(1.0))
                .with_order(12)
                .with_category("Electrical"),
        );

        // Model category (order 13). Ordered after the primary value so the
        // editor still opens with the inductance focused.
        sheet.add(
            PropertyDefinition::new("model")
                .with_display_name("Inductor Model")
                .with_description(
                    "Library L model card supplying the nominal and temperature terms",
                )
                .with_type(PropertyType::String)
                .with_default(PropertyValue::string(""))
                .with_order(13)
                .with_category("Model"),
        );

        // Parasitics (order 14-16). Explicit only: the parser synthesizes a
        // real element per parasitic and never imposes a default. The winding
        // resistance is `rser`, not `r` — a bare `r` on an inductor card is
        // not read by any device.
        sheet.add(
            PropertyDefinition::new("rser")
                .with_display_name("Series Resistance")
                .with_description("Winding DC resistance inserted through an internal node")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::expression(""))
                .with_unit("Ω")
                .with_order(14)
                .with_category("Parasitics"),
        );
        sheet.add(
            PropertyDefinition::new("rpar")
                .with_display_name("Parallel Resistance")
                .with_description("Core-loss resistance placed across the inductor")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::expression(""))
                .with_unit("Ω")
                .with_order(15)
                .with_category("Parasitics"),
        );
        sheet.add(
            PropertyDefinition::new("cpar")
                .with_display_name("Parallel Capacitance")
                .with_description("Inter-winding capacitance placed across the inductor")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::expression(""))
                .with_unit("F")
                .with_order(16)
                .with_category("Parasitics"),
        );

        // Temperature coefficients
        sheet.add(
            PropertyDefinition::new("tc1")
                .with_display_name("Temp Coeff 1")
                .with_description("First-order temperature coefficient")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(0.0))
                .with_unit("1/°C")
                .with_order(20)
                .with_category("Temperature"),
        );
        sheet.add(
            PropertyDefinition::new("tc2")
                .with_display_name("Temp Coeff 2")
                .with_description("Second-order temperature coefficient")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(0.0))
                .with_unit("1/°C²")
                .with_order(21)
                .with_category("Temperature"),
        );
        sheet.add(
            PropertyDefinition::new("dtemp")
                .with_display_name("Temp Rise")
                .with_description("Instance temperature rise above ambient")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(0.0))
                .with_unit("°C")
                .with_order(22)
                .with_category("Temperature"),
        );
        sheet.add(
            PropertyDefinition::new("temp")
                .with_display_name("Temperature")
                .with_description(
                    "Absolute instance temperature; overrides the circuit temperature entirely",
                )
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::expression(""))
                .with_unit("°C")
                .with_order(23)
                .with_category("Temperature"),
        );

        // Initial conditions
        sheet.add(
            PropertyDefinition::new("ic")
                .with_display_name("Initial Current")
                .with_description("Initial current through inductor for transient")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(0.0))
                .with_unit("A")
                .with_order(30)
                .with_category("Initial Conditions"),
        );

        // Mutual inductance coupling
        sheet.add(
            PropertyDefinition::new("coupling_factor")
                .with_display_name("Coupling Factor")
                .with_description("K-factor for mutual inductance (0-1)")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(0.0))
                .with_range(0.0, 1.0)
                .with_order(31)
                .with_category("Coupling"),
        );
        sheet.add(
            PropertyDefinition::new("coupled_to")
                .with_display_name("Coupled To")
                .with_description("Name of coupled inductor")
                .with_type(PropertyType::String)
                .with_default(PropertyValue::string(""))
                .with_order(32)
                .with_category("Coupling"),
        );

        self.sheets.insert(ComponentType::Inductor, sheet);
    }
}
