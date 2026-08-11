//! Property sheet for the diode.

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
        // Drawn dimensions (order 23-24). A LEVEL=3 geometric card derives
        // AREA and PJ from W/L, and giving either one suppresses the model
        // card's own AREA/PJ defaults.
        diode.add(
            PropertyDefinition::new("w")
                .with_display_name("Width")
                .with_description("Drawn junction width; LEVEL=3 derives AREA and PJ from W and L")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::expression(""))
                .with_unit("m")
                .with_order(23)
                .with_category("Geometry"),
        );
        diode.add(
            PropertyDefinition::new("l")
                .with_display_name("Length")
                .with_description("Drawn junction length; LEVEL=3 derives AREA and PJ from W and L")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::expression(""))
                .with_unit("m")
                .with_order(24)
                .with_category("Geometry"),
        );

        // Overlap (order 25-28). LEVEL=3 metal and poly overlap capacitance;
        // each dimension falls back to the model card when omitted.
        for (name, display, description, order) in [
            (
                "wm",
                "Metal Width",
                "Metal overlap width for LEVEL=3 overlap capacitance",
                25,
            ),
            (
                "lm",
                "Metal Length",
                "Metal overlap length for LEVEL=3 overlap capacitance",
                26,
            ),
            (
                "wp",
                "Poly Width",
                "Polysilicon overlap width for LEVEL=3 overlap capacitance",
                27,
            ),
            (
                "lp",
                "Poly Length",
                "Polysilicon overlap length for LEVEL=3 overlap capacitance",
                28,
            ),
        ] {
            diode.add(
                PropertyDefinition::new(name)
                    .with_display_name(display)
                    .with_description(description)
                    .with_type(PropertyType::Expression)
                    .with_default(PropertyValue::expression(""))
                    .with_unit("m")
                    .with_order(order)
                    .with_category("Overlap")
                    .advanced(),
            );
        }

        // Temperature category
        diode.add(
            PropertyDefinition::new("temp")
                .with_display_name("Temperature")
                .with_description(
                    "Absolute junction temperature; overrides the circuit temperature entirely",
                )
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::expression(""))
                .with_unit("°C")
                .with_order(30)
                .with_category("Temperature"),
        );
        diode.add(
            PropertyDefinition::new("dtemp")
                .with_display_name("Temp Rise")
                .with_description("Instance temperature rise above ambient")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(0.0))
                .with_unit("°C")
                .with_order(31)
                .with_category("Temperature"),
        );

        // No OFF switch and no junction IC: the diode build reads AREA, PJ,
        // M, W/L, the overlap dimensions, and TEMP/DTEMP only. OFF and IC
        // are parsed and then dropped, so exposing them would promise a
        // convergence aid the solver never applies.
        diode.add(symbol_variant_property(
            "Symbol",
            "Schematic symbol skin for the diode family",
            "default",
            &["default", "schottky", "zener", "led", "tunnel"],
        ));

        self.sheets.insert(ComponentType::Diode, diode);
    }
}
