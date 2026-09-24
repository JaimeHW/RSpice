//! Property sheet for the capacitor.

use super::*;

impl PropertyRegistry {
    /// Register Capacitor with commercial-grade parameters
    pub(super) fn register_capacitor(&mut self) {
        let mut sheet = PropertySheet::new();

        // Instance category
        sheet.add(
            PropertyDefinition::new("name")
                .with_display_name("Instance Name")
                .with_type(PropertyType::String)
                .with_default(PropertyValue::string("C1"))
                .with_order(0)
                .with_category("Instance")
                .required(),
        );

        // Electrical category
        sheet.add(
            PropertyDefinition::new("c")
                .with_display_name("Capacitance")
                .with_description("Capacitance value in Farads")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::expression("1u"))
                .with_unit("F")
                .with_range(0.0, 1e3)
                .with_order(10)
                .with_category("Electrical")
                .required(),
        );
        sheet.add(
            PropertyDefinition::new("m")
                .with_display_name("Multiplier")
                .with_description("Number of parallel capacitors")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(1.0))
                .with_range(1.0, 10000.0)
                .with_order(11)
                .with_category("Electrical"),
        );
        sheet.add(
            PropertyDefinition::new("scale")
                .with_display_name("Scale Factor")
                .with_description("Capacitance scaling factor")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(1.0))
                .with_order(12)
                .with_category("Electrical"),
        );

        // Model category (order 14). Ordered after the primary value so the
        // editor still opens with the capacitance focused.
        sheet.add(
            PropertyDefinition::new("model")
                .with_display_name("Capacitor Model")
                .with_description(
                    "Library C model card; enables per-area/perimeter geometry and CJ/CJSW scaling",
                )
                .with_type(PropertyType::String)
                .with_default(PropertyValue::string(""))
                .with_order(14)
                .with_category("Model"),
        );

        // Geometry (order 15-16). Only meaningful against a model card, whose
        // CJ/CJSW turn plate area and perimeter into the capacitance.
        sheet.add(
            PropertyDefinition::new("w")
                .with_display_name("Width")
                .with_description("Drawn plate width; defaults to the model DEFW when omitted")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::expression(""))
                .with_unit("m")
                .with_order(15)
                .with_category("Geometry")
                .when_set("model"),
        );
        sheet.add(
            PropertyDefinition::new("l")
                .with_display_name("Length")
                .with_description("Drawn plate length; defaults to the model DEFL when omitted")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::expression(""))
                .with_unit("m")
                .with_order(16)
                .with_category("Geometry")
                .when_set("model"),
        );

        // Parasitics (order 17-19). Explicit only: the parser synthesizes a
        // real element per parasitic and never imposes a default.
        sheet.add(
            PropertyDefinition::new("rser")
                .with_display_name("Series Resistance")
                .with_description("ESR inserted in series through an internal node")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::expression(""))
                .with_unit("Ω")
                .with_order(17)
                .with_category("Parasitics"),
        );
        sheet.add(
            PropertyDefinition::new("rpar")
                .with_display_name("Parallel Resistance")
                .with_description("Leakage resistance placed across the capacitor")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::expression(""))
                .with_unit("Ω")
                .with_order(18)
                .with_category("Parasitics"),
        );
        sheet.add(
            PropertyDefinition::new("cpar")
                .with_display_name("Parallel Capacitance")
                .with_description("Additional capacitance placed across the capacitor")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::expression(""))
                .with_unit("F")
                .with_order(19)
                .with_category("Parasitics"),
        );

        // No voltage-coefficient fields: the engine models no
        // voltage-dependent capacitance, so `vc1`/`vc2` controls would write
        // a parameter that reaches the deck and is then silently discarded.

        // Temperature coefficients
        sheet.add(
            PropertyDefinition::new("tc1")
                .with_display_name("Temp Coeff 1")
                .with_description("First-order temperature coefficient")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(0.0))
                .with_unit("1/°C")
                .with_order(30)
                .with_category("Temperature"),
        );
        sheet.add(
            PropertyDefinition::new("tc2")
                .with_display_name("Temp Coeff 2")
                .with_description("Second-order temperature coefficient")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(0.0))
                .with_unit("1/°C²")
                .with_order(31)
                .with_category("Temperature"),
        );
        sheet.add(
            PropertyDefinition::new("dtemp")
                .with_display_name("Temp Rise")
                .with_description("Instance temperature rise above ambient")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(0.0))
                .with_unit("°C")
                .with_order(32)
                .with_category("Temperature"),
        );
        sheet.add(
            PropertyDefinition::new("temp")
                .with_display_name("Temperature")
                .with_description(
                    "Absolute instance temperature; overrides the circuit temperature entirely",
                )
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::expression(""))
                .with_unit("°C")
                .with_order(33)
                .with_category("Temperature"),
        );

        // Aging (order 35-39). Read only under the Xyce dialect.
        sheet.add(
            PropertyDefinition::new("age")
                .with_display_name("Age")
                .with_description("Elapsed service time for Xyce capacitor aging")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::expression(""))
                .with_unit("h")
                .with_order(35)
                .with_category("Aging")
                .advanced(),
        );
        sheet.add(
            PropertyDefinition::new("d")
                .with_display_name("Aging Coefficient")
                .with_description("Xyce capacitance degradation coefficient applied over the age")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::expression(""))
                .with_order(36)
                .with_category("Aging")
                .advanced(),
        );

        // Initial conditions
        sheet.add(
            PropertyDefinition::new("ic")
                .with_display_name("Initial Voltage")
                .with_description("Initial voltage across capacitor for transient")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(0.0))
                .with_unit("V")
                .with_order(40)
                .with_category("Initial Conditions"),
        );

        self.sheets.insert(ComponentType::Capacitor, sheet);
    }
}
