//! Property sheet for the JFET.

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
        // Drawn dimensions (order 22-24). With AREA omitted the device scales
        // by W/L × NF instead; on an HFET1 card W is the real gate width and
        // NF multiplies it directly.
        sheet.add(
            PropertyDefinition::new("w")
                .with_display_name("Width")
                .with_description("Drawn gate width; scales the device as W/L when AREA is omitted")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::expression(""))
                .with_unit("m")
                .with_order(22)
                .with_category("Geometry"),
        );
        sheet.add(
            PropertyDefinition::new("l")
                .with_display_name("Length")
                .with_description(
                    "Drawn gate length; scales the device as W/L when AREA is omitted",
                )
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::expression(""))
                .with_unit("m")
                .with_order(23)
                .with_category("Geometry"),
        );
        sheet.add(
            PropertyDefinition::new("nf")
                .with_display_name("# Fingers")
                .with_description("Number of gate fingers folded into the W/L scaling")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(1.0))
                .with_range(1.0, 1000.0)
                .with_order(24)
                .with_category("Geometry"),
        );

        // Temperature category
        sheet.add(
            PropertyDefinition::new("temp")
                .with_display_name("Temperature")
                .with_description(
                    "Absolute device temperature; overrides the circuit temperature entirely",
                )
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::expression(""))
                .with_unit("°C")
                .with_order(30)
                .with_category("Temperature"),
        );
        sheet.add(
            PropertyDefinition::new("dtemp")
                .with_display_name("Temp Rise")
                .with_description("Instance temperature rise above ambient")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(0.0))
                .with_unit("°C")
                .with_order(31)
                .with_category("Temperature"),
        );

        // No OFF switch and no IC vector: the JFET instance contract the
        // engine reads is AREA/W/L/NF/M/TEMP/DTEMP only. OFF and the
        // IC=VDS,VGS pair are parsed and then dropped.

        sheet
    }
}
