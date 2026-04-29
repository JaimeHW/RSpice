use super::*;

impl PropertyRegistry {
    /// Register JFET (NJFET/PJFET) with commercial-grade parameters
    pub(super) fn register_jfet(&mut self) {
        let mut njfet = self.create_jfet_sheet("J1", "njfet");
        njfet.add(symbol_variant_property(
            "Symbol",
            "Schematic symbol skin for this JFET",
            "default",
            &["default", "discrete"],
        ));
        self.sheets.insert(ComponentType::Njfet, njfet);

        let mut pjfet = self.create_jfet_sheet("J1", "pjfet");
        pjfet.add(symbol_variant_property(
            "Symbol",
            "Schematic symbol skin for this JFET",
            "default",
            &["default", "discrete"],
        ));
        self.sheets.insert(ComponentType::Pjfet, pjfet);
    }

    /// Create a JFET property sheet with commercial-grade parameters
    fn create_jfet_sheet(&self, default_name: &str, default_model: &str) -> PropertySheet {
        let mut sheet = PropertySheet::new();

        // Instance category
        sheet.add(
            PropertyDefinition::new("name")
                .with_display_name("Instance Name")
                .with_description("Unique identifier for this JFET instance")
                .with_type(PropertyType::String)
                .with_default(PropertyValue::string(default_name))
                .with_order(0)
                .with_category("Instance")
                .required(),
        );

        // Model category
        sheet.add(
            PropertyDefinition::new("model")
                .with_display_name("Model")
                .with_description("JFET model name from library")
                .with_type(PropertyType::String)
                .with_default(PropertyValue::string(default_model))
                .with_order(10)
                .with_category("Model"),
        );

        // Geometry category
        sheet.add(
            PropertyDefinition::new("area")
                .with_display_name("Area Factor")
                .with_description("Device area multiplier")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(1.0))
                .with_range(1e-6, 1e6)
                .with_order(20)
                .with_category("Geometry"),
        );
        sheet.add(
            PropertyDefinition::new("m")
                .with_display_name("Multiplier")
                .with_description("Number of parallel devices")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(1.0))
                .with_range(1.0, 10000.0)
                .with_order(21)
                .with_category("Geometry"),
        );

        // Temperature category
        sheet.add(
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
        sheet.add(
            PropertyDefinition::new("off")
                .with_display_name("Initially Off")
                .with_description("Start in off state for DC operating point")
                .with_type(PropertyType::Boolean)
                .with_default(PropertyValue::boolean(false))
                .with_order(40)
                .with_category("Initial Conditions"),
        );
        sheet.add(
            PropertyDefinition::new("ic_vds")
                .with_display_name("IC VDS")
                .with_description("Initial drain-source voltage for transient analysis")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(0.0))
                .with_unit("V")
                .with_order(41)
                .with_category("Initial Conditions"),
        );
        sheet.add(
            PropertyDefinition::new("ic_vgs")
                .with_display_name("IC VGS")
                .with_description("Initial gate-source voltage for transient analysis")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(0.0))
                .with_unit("V")
                .with_order(42)
                .with_category("Initial Conditions"),
        );

        sheet
    }
}
