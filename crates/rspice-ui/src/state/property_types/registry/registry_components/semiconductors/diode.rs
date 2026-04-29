use super::*;

impl PropertyRegistry {
    /// Register diode with all SPICE-standard parameters
    pub(super) fn register_diode(&mut self) {
        let mut diode = PropertySheet::new();

        // Instance category
        diode.add(
            PropertyDefinition::new("name")
                .with_display_name("Instance Name")
                .with_description("Unique identifier for this diode instance")
                .with_type(PropertyType::String)
                .with_default(PropertyValue::string("D1"))
                .with_order(0)
                .with_category("Instance")
                .required(),
        );

        // Model category
        diode.add(
            PropertyDefinition::new("model")
                .with_display_name("Model")
                .with_description("Diode model name from library")
                .with_type(PropertyType::String)
                .with_default(PropertyValue::string("D"))
                .with_order(10)
                .with_category("Model"),
        );

        // Geometry category
        diode.add(
            PropertyDefinition::new("area")
                .with_display_name("Area Factor")
                .with_description("Junction area relative to model default")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(1.0))
                .with_range(1e-6, 1e6)
                .with_order(20)
                .with_category("Geometry"),
        );
        diode.add(
            PropertyDefinition::new("pj")
                .with_display_name("Perimeter")
                .with_description("Junction perimeter")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(0.0))
                .with_unit("m")
                .with_order(21)
                .with_category("Geometry"),
        );
        diode.add(
            PropertyDefinition::new("m")
                .with_display_name("Multiplier")
                .with_description("Number of parallel devices")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(1.0))
                .with_range(1.0, 10000.0)
                .with_order(22)
                .with_category("Geometry"),
        );

        // Temperature category
        diode.add(
            PropertyDefinition::new("dtemp")
                .with_display_name("Temp Rise")
                .with_description("Instance temperature rise above ambient")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(0.0))
                .with_unit("°C")
                .with_order(30)
                .with_category("Temperature"),
        );

        // Initial Conditions category
        diode.add(
            PropertyDefinition::new("off")
                .with_display_name("Initially Off")
                .with_description("Start in off state for DC operating point")
                .with_type(PropertyType::Boolean)
                .with_default(PropertyValue::boolean(false))
                .with_order(40)
                .with_category("Initial Conditions"),
        );
        diode.add(
            PropertyDefinition::new("ic")
                .with_display_name("Initial Voltage")
                .with_description("Initial voltage across diode for transient")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(0.0))
                .with_unit("V")
                .with_order(41)
                .with_category("Initial Conditions"),
        );
        diode.add(symbol_variant_property(
            "Symbol",
            "Schematic symbol skin for the diode family",
            "default",
            &["default", "schottky", "zener", "led", "tunnel"],
        ));

        self.sheets.insert(ComponentType::Diode, diode);
    }
}
