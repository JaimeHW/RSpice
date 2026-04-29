use super::*;

impl PropertyRegistry {
    /// Register MOSFET (NMOS/PMOS) with all Spectre-compatible parameters
    pub(super) fn register_mosfet(&mut self) {
        let nmos = self.create_mosfet_sheet("M1", "nmos");
        self.sheets.insert(ComponentType::Nmos, nmos);

        let pmos = self.create_mosfet_sheet("M1", "pmos");
        self.sheets.insert(ComponentType::Pmos, pmos);
    }

    /// Create a MOSFET property sheet with commercial-grade parameters
    fn create_mosfet_sheet(&self, default_name: &str, default_model: &str) -> PropertySheet {
        let mut sheet = PropertySheet::new();

        // =========================================================================
        // Instance Category (order 0-9)
        // =========================================================================
        sheet.add(
            PropertyDefinition::new("name")
                .with_display_name("Instance Name")
                .with_description("Unique identifier for this MOSFET instance")
                .with_type(PropertyType::String)
                .with_default(PropertyValue::string(default_name))
                .with_order(0)
                .with_category("Instance")
                .required(),
        );

        // =========================================================================
        // Model Category (order 10-19)
        // =========================================================================
        sheet.add(
            PropertyDefinition::new("model")
                .with_display_name("Model")
                .with_description("MOSFET model name from PDK library")
                .with_type(PropertyType::String)
                .with_default(PropertyValue::string(default_model))
                .with_order(10)
                .with_category("Model"),
        );

        // =========================================================================
        // Geometry Category (order 20-39)
        // =========================================================================
        sheet.add(
            PropertyDefinition::new("w")
                .with_display_name("Width")
                .with_description("Channel width (drawn)")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::expression("1u"))
                .with_unit("m")
                .with_range(1e-9, 1e-3)
                .with_order(20)
                .with_category("Geometry")
                .required(),
        );
        sheet.add(
            PropertyDefinition::new("l")
                .with_display_name("Length")
                .with_description("Channel length (drawn)")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::expression("180n"))
                .with_unit("m")
                .with_range(1e-9, 1e-3)
                .with_order(21)
                .with_category("Geometry")
                .required(),
        );
        sheet.add(
            PropertyDefinition::new("m")
                .with_display_name("Multiplier")
                .with_description("Number of parallel devices (total W = m × nf × w)")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(1.0))
                .with_range(1.0, 10000.0)
                .with_order(22)
                .with_category("Geometry"),
        );
        sheet.add(
            PropertyDefinition::new("nf")
                .with_display_name("# Fingers")
                .with_description("Number of gate fingers")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(1.0))
                .with_range(1.0, 1000.0)
                .with_order(23)
                .with_category("Geometry"),
        );

        // =========================================================================
        // Parasitics Category (order 40-59)
        // Source/Drain areas and perimeters for junction capacitance
        // =========================================================================
        sheet.add(
            PropertyDefinition::new("as")
                .with_display_name("Source Area")
                .with_description("Source diffusion area for junction capacitance")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::expression("0"))
                .with_unit("m²")
                .with_order(40)
                .with_category("Parasitics"),
        );
        sheet.add(
            PropertyDefinition::new("ad")
                .with_display_name("Drain Area")
                .with_description("Drain diffusion area for junction capacitance")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::expression("0"))
                .with_unit("m²")
                .with_order(41)
                .with_category("Parasitics"),
        );
        sheet.add(
            PropertyDefinition::new("ps")
                .with_display_name("Source Perimeter")
                .with_description("Source diffusion perimeter for sidewall capacitance")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::expression("0"))
                .with_unit("m")
                .with_order(42)
                .with_category("Parasitics"),
        );
        sheet.add(
            PropertyDefinition::new("pd")
                .with_display_name("Drain Perimeter")
                .with_description("Drain diffusion perimeter for sidewall capacitance")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::expression("0"))
                .with_unit("m")
                .with_order(43)
                .with_category("Parasitics"),
        );
        sheet.add(
            PropertyDefinition::new("nrd")
                .with_display_name("Drain Squares")
                .with_description("Number of squares for drain series resistance")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(0.0))
                .with_order(44)
                .with_category("Parasitics"),
        );
        sheet.add(
            PropertyDefinition::new("nrs")
                .with_display_name("Source Squares")
                .with_description("Number of squares for source series resistance")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(0.0))
                .with_order(45)
                .with_category("Parasitics"),
        );

        // =========================================================================
        // Stress Category (order 60-79) - STI stress effects for advanced nodes
        // =========================================================================
        sheet.add(
            PropertyDefinition::new("sa")
                .with_display_name("SA Distance")
                .with_description("Distance from gate edge to STI on source side")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::expression("0"))
                .with_unit("m")
                .with_order(60)
                .with_category("Stress"),
        );
        sheet.add(
            PropertyDefinition::new("sb")
                .with_display_name("SB Distance")
                .with_description("Distance from gate edge to STI on drain side")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::expression("0"))
                .with_unit("m")
                .with_order(61)
                .with_category("Stress"),
        );
        sheet.add(
            PropertyDefinition::new("sd")
                .with_display_name("SD Spacing")
                .with_description("Source-drain spacing for multi-finger devices")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::expression("0"))
                .with_unit("m")
                .with_order(62)
                .with_category("Stress"),
        );
        sheet.add(
            PropertyDefinition::new("sca")
                .with_display_name("SCA")
                .with_description("Integral of first distribution function for STI stress")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(0.0))
                .with_order(63)
                .with_category("Stress"),
        );
        sheet.add(
            PropertyDefinition::new("scb")
                .with_display_name("SCB")
                .with_description("Integral of second distribution function for STI stress")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(0.0))
                .with_order(64)
                .with_category("Stress"),
        );
        sheet.add(
            PropertyDefinition::new("scc")
                .with_display_name("SCC")
                .with_description("Integral of third distribution function for STI stress")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(0.0))
                .with_order(65)
                .with_category("Stress"),
        );

        // =========================================================================
        // Temperature Category (order 80-89)
        // =========================================================================
        sheet.add(
            PropertyDefinition::new("dtemp")
                .with_display_name("Temp Rise")
                .with_description("Instance temperature rise above ambient")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(0.0))
                .with_unit("°C")
                .with_order(80)
                .with_category("Temperature"),
        );

        // =========================================================================
        // Initial Conditions Category (order 90-99)
        // =========================================================================
        sheet.add(
            PropertyDefinition::new("off")
                .with_display_name("Initially Off")
                .with_description("Start in off state for DC operating point analysis")
                .with_type(PropertyType::Boolean)
                .with_default(PropertyValue::boolean(false))
                .with_order(90)
                .with_category("Initial Conditions"),
        );
        sheet.add(
            PropertyDefinition::new("region")
                .with_display_name("Region Hint")
                .with_description("Estimated operating region for convergence aid")
                .with_type(PropertyType::Enum)
                .with_default(PropertyValue::enumeration(
                    "auto",
                    vec![
                        "auto".to_string(),
                        "off".to_string(),
                        "triode".to_string(),
                        "sat".to_string(),
                        "subth".to_string(),
                    ],
                ))
                .with_order(91)
                .with_category("Initial Conditions"),
        );
        sheet.add(
            PropertyDefinition::new("ic_vds")
                .with_display_name("IC VDS")
                .with_description("Initial drain-source voltage for transient analysis")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(0.0))
                .with_unit("V")
                .with_order(92)
                .with_category("Initial Conditions"),
        );
        sheet.add(
            PropertyDefinition::new("ic_vgs")
                .with_display_name("IC VGS")
                .with_description("Initial gate-source voltage for transient analysis")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(0.0))
                .with_unit("V")
                .with_order(93)
                .with_category("Initial Conditions"),
        );
        sheet.add(
            PropertyDefinition::new("ic_vbs")
                .with_display_name("IC VBS")
                .with_description("Initial bulk-source voltage for transient analysis")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(0.0))
                .with_unit("V")
                .with_order(94)
                .with_category("Initial Conditions"),
        );

        sheet
    }
}
