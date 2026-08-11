//! Property sheet for the bipolar transistor.

use super::*;

impl PropertyRegistry {
    /// Register BJT (NPN/PNP) with commercial-grade parameters
    pub(super) fn register_bjt(&mut self) {
        let mut npn = self.create_bjt_sheet("Q1", "npn");
        npn.add(symbol_variant_property(
            "Symbol",
            "Schematic symbol skin for this BJT",
            "default",
            &["default", "discrete"],
        ));
        self.sheets.insert(ComponentType::NpnBjt, npn);

        let mut pnp = self.create_bjt_sheet("Q1", "pnp");
        pnp.add(symbol_variant_property(
            "Symbol",
            "Schematic symbol skin for this BJT",
            "default",
            &["default", "discrete"],
        ));
        self.sheets.insert(ComponentType::PnpBjt, pnp);

        // Substrate (4T) and substrate+thermal (5T) variants share the
        // standard BJT instance parameters; the extra terminals are
        // resolved by the model card (GP substrate or VBIC thermal).
        self.sheets
            .insert(ComponentType::NpnBjt4, self.create_bjt_sheet("Q1", "npn"));
        self.sheets
            .insert(ComponentType::PnpBjt4, self.create_bjt_sheet("Q1", "pnp"));
        self.sheets
            .insert(ComponentType::NpnBjt5, self.create_bjt_sheet("Q1", "npn"));
        self.sheets
            .insert(ComponentType::PnpBjt5, self.create_bjt_sheet("Q1", "pnp"));
    }

    /// Create a BJT property sheet with commercial-grade parameters
    fn create_bjt_sheet(&self, default_name: &str, default_model: &str) -> PropertySheet {
        let mut sheet = PropertySheet::new();

        // Instance category
        sheet.add(
            PropertyDefinition::new("name")
                .with_display_name("Instance Name")
                .with_description("Unique identifier for this BJT instance")
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
                .with_description("BJT model name from library")
                .with_type(PropertyType::String)
                .with_default(PropertyValue::string(default_model))
                .with_order(10)
                .with_category("Model"),
        );

        // Geometry category
        sheet.add(
            PropertyDefinition::new("area")
                .with_display_name("Area Factor")
                .with_description("Emitter area multiplier relative to model default")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(1.0))
                .with_range(1e-6, 1e6)
                .with_order(20)
                .with_category("Geometry"),
        );
        // No AREAB/AREAC: the parser accepts both spellings, but the device
        // scales every junction off the single AREA factor, so a separate
        // base or collector area would be written to the deck and dropped.
        sheet.add(
            PropertyDefinition::new("m")
                .with_display_name("Multiplier")
                .with_description("Number of parallel devices")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(1.0))
                .with_range(1.0, 10000.0)
                .with_order(23)
                .with_category("Geometry"),
        );

        // Temperature category
        sheet.add(
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
        // No region hint and no per-junction initial conditions: the BJT
        // instance contract the engine reads is AREA/M/OFF/TEMP/DTEMP only,
        // so those controls would write parameters nothing consumes.

        sheet
    }
}
