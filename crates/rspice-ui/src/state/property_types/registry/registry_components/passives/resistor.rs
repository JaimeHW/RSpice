//! Property sheet for the resistor.

use super::*;

impl PropertyRegistry {
    /// Register Resistor with commercial-grade parameters
    pub(super) fn register_resistor(&mut self) {
        let mut sheet = PropertySheet::new();

        // Instance category (order 0-9)
        sheet.add(
            PropertyDefinition::new("name")
                .with_display_name("Instance Name")
                .with_description("Unique identifier for this resistor")
                .with_type(PropertyType::String)
                .with_default(PropertyValue::string("R1"))
                .with_order(0)
                .with_category("Instance")
                .required(),
        );

        // Electrical category (order 10-19)
        sheet.add(
            PropertyDefinition::new("r")
                .with_display_name("Resistance")
                .with_description("Resistance value in Ohms")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::expression("1k"))
                .with_unit("Ω")
                .with_range(0.0, 1e15)
                .with_order(10)
                .with_category("Electrical")
                .required(),
        );
        sheet.add(
            PropertyDefinition::new("m")
                .with_display_name("Multiplier")
                .with_description("Number of parallel resistors")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(1.0))
                .with_range(1.0, 10000.0)
                .with_order(11)
                .with_category("Electrical"),
        );
        sheet.add(
            PropertyDefinition::new("scale")
                .with_display_name("Scale Factor")
                .with_description("Resistance scaling factor")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(1.0))
                .with_order(12)
                .with_category("Electrical"),
        );
        sheet.add(
            PropertyDefinition::new("ac")
                .with_display_name("AC Resistance")
                .with_description(
                    "Small-signal resistance used by AC and noise analyses instead of the DC value",
                )
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::expression(""))
                .with_unit("Ω")
                .with_order(13)
                .with_category("Electrical"),
        );
        sheet.add(
            PropertyDefinition::new("noisy")
                .with_display_name("Generates Noise")
                .with_description(
                    "Clear to make this resistor noiseless in noise analysis; on by default",
                )
                .with_type(PropertyType::Boolean)
                .with_default(PropertyValue::boolean(true))
                .with_order(35)
                .with_category("Noise"),
        );

        // Model category (order 14). Ordered after the primary value so the
        // editor still opens with the resistance focused.
        sheet.add(
            PropertyDefinition::new("model")
                .with_display_name("Resistor Model")
                .with_description(
                    "Library R model card; enables sheet-resistance geometry and flicker noise",
                )
                .with_type(PropertyType::String)
                .with_default(PropertyValue::string(""))
                .with_order(14)
                .with_category("Model"),
        );

        // Geometry category (order 15-19). Read only through a model card:
        // RSH turns these into the resistance, and KF into flicker noise.
        sheet.add(
            PropertyDefinition::new("l")
                .with_display_name("Length")
                .with_description("Drawn resistor length; with model RSH this sets the resistance")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::expression(""))
                .with_unit("m")
                .with_order(15)
                .with_category("Geometry")
                .when_set("model"),
        );
        sheet.add(
            PropertyDefinition::new("w")
                .with_display_name("Width")
                .with_description("Drawn resistor width; defaults to the model DEFW when omitted")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::expression(""))
                .with_unit("m")
                .with_order(16)
                .with_category("Geometry")
                .when_set("model"),
        );
        sheet.add(
            PropertyDefinition::new("nrs")
                .with_display_name("Squares")
                .with_description(
                    "Number of squares; overrides the length/width count against model RSH",
                )
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::expression(""))
                .with_order(17)
                .with_category("Geometry")
                .when_set("model"),
        );
        sheet.add(
            PropertyDefinition::new("area")
                .with_display_name("Cross Section")
                .with_description("Conductor cross-sectional area for the thermal resistor level")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::expression(""))
                .with_unit("m²")
                .with_order(18)
                .with_category("Geometry")
                .when_set("model"),
        );

        // Temperature category (order 20-29)
        sheet.add(
            PropertyDefinition::new("temp")
                .with_display_name("Temperature")
                .with_description(
                    "Absolute instance temperature; overrides the circuit temperature entirely",
                )
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::expression(""))
                .with_unit("°C")
                .with_order(19)
                .with_category("Temperature"),
        );
        sheet.add(
            PropertyDefinition::new("tc1")
                .with_display_name("Temp Coeff 1")
                .with_description("First-order temperature coefficient")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(0.0))
                .with_unit("1/°C")
                .with_order(20)
                .with_category("Temperature"),
        );
        sheet.add(
            PropertyDefinition::new("tc2")
                .with_display_name("Temp Coeff 2")
                .with_description("Second-order temperature coefficient")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(0.0))
                .with_unit("1/°C²")
                .with_order(21)
                .with_category("Temperature"),
        );
        sheet.add(
            PropertyDefinition::new("tce")
                .with_display_name("Temp Exp Coeff")
                .with_description("Exponential temperature coefficient")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(0.0))
                .with_unit("%/°C")
                .with_order(22)
                .with_category("Temperature"),
        );
        sheet.add(
            PropertyDefinition::new("dtemp")
                .with_display_name("Temp Rise")
                .with_description("Instance temperature rise above ambient")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(0.0))
                .with_unit("°C")
                .with_order(23)
                .with_category("Temperature"),
        );

        // Noise category (order 30-39)
        sheet.add(
            PropertyDefinition::new("noisy")
                .with_display_name("Noisy")
                .with_description("Enable thermal noise generation")
                .with_type(PropertyType::Boolean)
                .with_default(PropertyValue::boolean(true))
                .with_order(30)
                .with_category("Noise"),
        );

        // Self-heating category (order 40-49). The self-consistent thermal
        // resistor only engages when the model supplies LEVEL=2 material
        // constants and the instance supplies both length and cross section,
        // so these stay behind the advanced switch.
        for (name, display, description, unit, order) in [
            (
                "resistivity",
                "Resistivity",
                "Conductor resistivity, overriding the model value",
                "Ω·m",
                40,
            ),
            (
                "heatcapacity",
                "Heat Capacity",
                "Volumetric heat capacity of the conductor",
                "J/(m³·K)",
                41,
            ),
            (
                "thermal_l",
                "Thermal Length",
                "Heat-flow path length of the surrounding thermal body",
                "m",
                42,
            ),
            (
                "thermal_a",
                "Thermal Cross Section",
                "Heat-flow cross section of the surrounding thermal body",
                "m²",
                43,
            ),
            (
                "thermal_heatcapacity",
                "Thermal Heat Capacity",
                "Volumetric heat capacity of the surrounding thermal body",
                "J/(m³·K)",
                44,
            ),
        ] {
            sheet.add(
                PropertyDefinition::new(name)
                    .with_display_name(display)
                    .with_description(description)
                    .with_type(PropertyType::Expression)
                    .with_default(PropertyValue::expression(""))
                    .with_unit(unit)
                    .with_order(order)
                    .with_category("Self-Heating")
                    .when_set("model")
                    .advanced(),
            );
        }

        self.sheets.insert(ComponentType::Resistor, sheet);
    }
}
