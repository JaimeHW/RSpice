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

            self.sheets.insert(kind, sheet);
        }
    }
}
