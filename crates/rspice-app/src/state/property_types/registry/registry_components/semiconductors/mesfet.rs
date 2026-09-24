//! Property sheet for the MESFET.

use super::*;

impl PropertyRegistry {
    /// Register MESFET/HFET (N/P) property sheets. Level selection (legacy,
    /// MESA, HFET1/2) rides on the bound model card, not the instance.
    pub(super) fn register_mesfet(&mut self) {
        for (kind, default_model) in [
            (ComponentType::Nmesfet, "nmf"),
            (ComponentType::Pmesfet, "pmf"),
        ] {
            let mut sheet = PropertySheet::new();

            sheet.add(
                PropertyDefinition::new("name")
                    .with_display_name("Instance Name")
                    .with_description("Unique identifier for this MESFET instance")
                    .with_type(PropertyType::String)
                    .with_default(PropertyValue::string("Z1"))
                    .with_order(0)
                    .with_category("Instance")
                    .required(),
            );
            sheet.add(
                PropertyDefinition::new("model")
                    .with_display_name("Model")
                    .with_description(
                        "MESFET model from library (NMF/PMF; NHFET/PHFET or LEVEL=5/6 for HFET)",
                    )
                    .with_type(PropertyType::String)
                    .with_default(PropertyValue::string(default_model))
                    .with_order(10)
                    .with_category("Model"),
            );
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
            // MESFET and HFET instances run the JFET instance contract, so
            // the same W/L/NF scaling applies: AREA omitted means W/L × NF,
            // and an HFET1 card takes W as the real gate width.
            sheet.add(
                PropertyDefinition::new("w")
                    .with_display_name("Width")
                    .with_description(
                        "Drawn gate width; scales the device as W/L when AREA is omitted",
                    )
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

            self.sheets.insert(kind, sheet);
        }
    }
}
