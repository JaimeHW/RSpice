use super::*;

impl PropertyRegistry {
    /// Register schematic transformer properties.
    ///
    /// Transformers are authored as a single 4-pin schematic component and
    /// synthesized into two winding inductors plus a coupling statement during
    /// netlist generation.
    pub(super) fn register_transformer(&mut self) {
        let mut sheet = PropertySheet::new();

        sheet.add(
            PropertyDefinition::new("name")
                .with_display_name("Instance Name")
                .with_description("Unique identifier for this transformer")
                .with_type(PropertyType::String)
                .with_default(PropertyValue::string("T1"))
                .with_order(0)
                .with_category("Instance")
                .required(),
        );

        sheet.add(
            PropertyDefinition::new("lp")
                .with_display_name("Primary Inductance")
                .with_description("Primary winding inductance in Henries")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::expression("1m"))
                .with_unit("H")
                .with_range(0.0, 1e9)
                .with_order(10)
                .with_category("Electrical")
                .required(),
        );
        sheet.add(
            PropertyDefinition::new("turns_ratio")
                .with_display_name("Turns Ratio")
                .with_description("Secondary-to-primary turns ratio (Ns/Np) used when secondary inductance is not specified")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::expression("1"))
                .with_order(11)
                .with_category("Electrical")
                .required(),
        );
        sheet.add(
            PropertyDefinition::new("ls")
                .with_display_name("Secondary Inductance")
                .with_description("Optional explicit secondary inductance override; leave empty to derive from turns ratio")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::expression(""))
                .with_unit("H")
                .with_range(0.0, 1e9)
                .with_order(12)
                .with_category("Electrical"),
        );
        sheet.add(
            PropertyDefinition::new("k")
                .with_display_name("Coupling Factor")
                .with_description("Magnetic coupling coefficient (0 < k <= 1)")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::expression("0.999"))
                .with_range(0.0, 1.0)
                .with_order(13)
                .with_category("Electrical")
                .required(),
        );

        sheet.add(
            PropertyDefinition::new("rp")
                .with_display_name("Primary Series Resistance")
                .with_description("Primary winding series resistance")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::expression("0"))
                .with_unit("Ohm")
                .with_order(20)
                .with_category("Primary"),
        );
        sheet.add(
            PropertyDefinition::new("icp")
                .with_display_name("Primary Initial Current")
                .with_description("Initial primary winding current for transient analysis")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(0.0))
                .with_unit("A")
                .with_order(21)
                .with_category("Primary"),
        );
        sheet.add(
            PropertyDefinition::new("rs")
                .with_display_name("Secondary Series Resistance")
                .with_description("Secondary winding series resistance")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::expression("0"))
                .with_unit("Ohm")
                .with_order(30)
                .with_category("Secondary"),
        );
        sheet.add(
            PropertyDefinition::new("ics")
                .with_display_name("Secondary Initial Current")
                .with_description("Initial secondary winding current for transient analysis")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(0.0))
                .with_unit("A")
                .with_order(31)
                .with_category("Secondary"),
        );

        self.sheets.insert(ComponentType::Transformer, sheet);
    }

    /// Register Coupled Inductor / transformer coupling statement.
    pub(super) fn register_coupled_inductor(&mut self) {
        let mut sheet = PropertySheet::new();

        sheet.add(
            PropertyDefinition::new("name")
                .with_display_name("Instance Name")
                .with_description("Coupling statement instance name")
                .with_type(PropertyType::String)
                .with_default(PropertyValue::string("K1"))
                .with_order(0)
                .with_category("Instance")
                .required(),
        );
        sheet.add(
            PropertyDefinition::new("k")
                .with_display_name("Coupling Coefficient")
                .with_description("Mutual coupling coefficient (0 < k <= 1)")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::expression("0.99"))
                .with_order(10)
                .with_category("Electrical")
                .required(),
        );
        sheet.add(
            PropertyDefinition::new("inductors")
                .with_display_name("Windings")
                .with_description(
                    "Space- or comma-separated inductor instance names (for example: L1 L2)",
                )
                .with_type(PropertyType::String)
                .with_default(PropertyValue::string("L1 L2"))
                .with_order(11)
                .with_category("Electrical")
                .required(),
        );

        self.sheets.insert(ComponentType::CoupledInductor, sheet);
    }
}
