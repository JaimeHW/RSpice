//! Property sheet for the saturable inductor and its B-H curve parameters.

use super::*;

impl PropertyRegistry {
    /// Register the saturable (Jiles-Atherton magnetic-core) inductor.
    ///
    /// The winding's inductance target is the primary value; the core
    /// physics ride on a generated `.MODEL <name> CORE` card built from
    /// the magnetics parameters, unless `model` binds a library core.
    pub(super) fn register_saturable_inductor(&mut self) {
        let mut sheet = PropertySheet::new();

        sheet.add(
            PropertyDefinition::new("name")
                .with_display_name("Instance Name")
                .with_type(PropertyType::String)
                .with_default(PropertyValue::string("L1"))
                .with_order(0)
                .with_category("Instance")
                .required(),
        );

        sheet.add(
            PropertyDefinition::new("value")
                .with_display_name("Inductance")
                .with_description("Small-signal inductance target at zero bias")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::expression("1m"))
                .with_unit("H")
                .with_order(10)
                .with_category("Electrical")
                .required(),
        );

        sheet.add(
            PropertyDefinition::new("model")
                .with_display_name("Core Model")
                .with_description("Library CORE model name; overrides the magnetics parameters")
                .with_type(PropertyType::String)
                .with_default(PropertyValue::string(""))
                .with_order(11)
                .with_category("Model"),
        );

        sheet.add(
            PropertyDefinition::new("ms")
                .with_display_name("Saturation Magnetization")
                .with_description("Jiles-Atherton saturation magnetization MS")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(0.4))
                .with_unit("T")
                .with_order(20)
                .with_category("Core"),
        );
        sheet.add(
            PropertyDefinition::new("a")
                .with_display_name("Anhysteretic Shape")
                .with_description("Jiles-Atherton anhysteretic shape parameter A")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(200.0))
                .with_unit("A/m")
                .with_order(21)
                .with_category("Core"),
        );
        sheet.add(
            PropertyDefinition::new("k")
                .with_display_name("Pinning")
                .with_description("Jiles-Atherton domain-wall pinning parameter K")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(150.0))
                .with_unit("A/m")
                .with_order(22)
                .with_category("Core"),
        );
        sheet.add(
            PropertyDefinition::new("c")
                .with_display_name("Reversibility")
                .with_description("Jiles-Atherton reversibility coefficient C (0-1)")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(0.1))
                .with_range(0.0, 1.0)
                .with_order(23)
                .with_category("Core"),
        );
        sheet.add(
            PropertyDefinition::new("alpha")
                .with_display_name("Inter-domain Coupling")
                .with_description("Jiles-Atherton mean-field coupling ALPHA")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(1e-4))
                .with_order(24)
                .with_category("Core"),
        );

        sheet.add(
            PropertyDefinition::new("area")
                .with_display_name("Core Area")
                .with_description("Magnetic core cross-sectional area")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::expression("1e-4"))
                .with_unit("m²")
                .with_order(30)
                .with_category("Geometry"),
        );
        sheet.add(
            PropertyDefinition::new("length")
                .with_display_name("Path Length")
                .with_description("Mean magnetic path length")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::expression("0.1"))
                .with_unit("m")
                .with_order(31)
                .with_category("Geometry"),
        );
        sheet.add(
            PropertyDefinition::new("n")
                .with_display_name("Turns")
                .with_description(
                    "Winding turns count; when omitted the turns are fitted to the inductance",
                )
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(100.0))
                .with_order(32)
                .with_category("Geometry"),
        );

        self.sheets.insert(ComponentType::SaturableInductor, sheet);
    }
}
