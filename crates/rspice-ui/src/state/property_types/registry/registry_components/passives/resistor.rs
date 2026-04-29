use super::*;

impl PropertyRegistry {
    /// Register Resistor with commercial-grade parameters
    pub(super) fn register_resistor(&mut self) {
        let mut sheet = PropertySheet::new();

        // Instance category (order 0-9)
        sheet.add(
            PropertyDefinition::new("name")
                .with_display_name("Instance Name")
                .with_description("Unique identifier for this resistor")
                .with_type(PropertyType::String)
                .with_default(PropertyValue::string("R1"))
                .with_order(0)
                .with_category("Instance")
                .required(),
        );

        // Electrical category (order 10-19)
        sheet.add(
            PropertyDefinition::new("r")
                .with_display_name("Resistance")
                .with_description("Resistance value in Ohms")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::expression("1k"))
                .with_unit("Ω")
                .with_range(0.0, 1e15)
                .with_order(10)
                .with_category("Electrical")
                .required(),
        );
        sheet.add(
            PropertyDefinition::new("m")
                .with_display_name("Multiplier")
                .with_description("Number of parallel resistors")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(1.0))
                .with_range(1.0, 10000.0)
                .with_order(11)
                .with_category("Electrical"),
        );
        sheet.add(
            PropertyDefinition::new("scale")
                .with_display_name("Scale Factor")
                .with_description("Resistance scaling factor")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(1.0))
                .with_order(12)
                .with_category("Electrical"),
        );

        // Temperature category (order 20-29)
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
            PropertyDefinition::new("tce")
                .with_display_name("Temp Exp Coeff")
                .with_description("Exponential temperature coefficient")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(0.0))
                .with_unit("%/°C")
                .with_order(22)
                .with_category("Temperature"),
        );
        sheet.add(
            PropertyDefinition::new("dtemp")
                .with_display_name("Temp Rise")
                .with_description("Instance temperature rise above ambient")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(0.0))
                .with_unit("°C")
                .with_order(23)
                .with_category("Temperature"),
        );

        // Noise category (order 30-39)
        sheet.add(
            PropertyDefinition::new("noisy")
                .with_display_name("Noisy")
                .with_description("Enable thermal noise generation")
                .with_type(PropertyType::Boolean)
                .with_default(PropertyValue::boolean(true))
                .with_order(30)
                .with_category("Noise"),
        );

        self.sheets.insert(ComponentType::Resistor, sheet);
    }
}
