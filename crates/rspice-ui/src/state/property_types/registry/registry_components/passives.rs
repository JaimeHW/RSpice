use super::*;

impl PropertyRegistry {
    pub(in super::super) fn register_passive_components(&mut self) {
        self.register_resistor();
        self.register_capacitor();
        self.register_inductor();
        self.register_transformer();
        self.register_coupled_inductor();
    }

    /// Register Resistor with commercial-grade parameters
    pub(in super::super) fn register_resistor(&mut self) {
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

        // Temperature category (order 20-29)
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

        self.sheets.insert(ComponentType::Resistor, sheet);
    }

    /// Register Capacitor with commercial-grade parameters
    pub(in super::super) fn register_capacitor(&mut self) {
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

        // Voltage coefficients (order 20-29) - for voltage-dependent capacitance
        sheet.add(
            PropertyDefinition::new("vc1")
                .with_display_name("Voltage Coeff 1")
                .with_description("First-order voltage coefficient")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(0.0))
                .with_unit("1/V")
                .with_order(20)
                .with_category("Voltage Coefficients"),
        );
        sheet.add(
            PropertyDefinition::new("vc2")
                .with_display_name("Voltage Coeff 2")
                .with_description("Second-order voltage coefficient")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(0.0))
                .with_unit("1/V²")
                .with_order(21)
                .with_category("Voltage Coefficients"),
        );

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

    /// Register Inductor with commercial-grade parameters
    pub(in super::super) fn register_inductor(&mut self) {
        let mut sheet = PropertySheet::new();

        // Instance category
        sheet.add(
            PropertyDefinition::new("name")
                .with_display_name("Instance Name")
                .with_type(PropertyType::String)
                .with_default(PropertyValue::string("L1"))
                .with_order(0)
                .with_category("Instance")
                .required(),
        );

        // Electrical category
        sheet.add(
            PropertyDefinition::new("l")
                .with_display_name("Inductance")
                .with_description("Inductance value in Henries")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::expression("1m"))
                .with_unit("H")
                .with_range(0.0, 1e6)
                .with_order(10)
                .with_category("Electrical")
                .required(),
        );
        sheet.add(
            PropertyDefinition::new("m")
                .with_display_name("Multiplier")
                .with_description("Number of parallel inductors")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(1.0))
                .with_range(1.0, 10000.0)
                .with_order(11)
                .with_category("Electrical"),
        );
        sheet.add(
            PropertyDefinition::new("scale")
                .with_display_name("Scale Factor")
                .with_description("Inductance scaling factor")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(1.0))
                .with_order(12)
                .with_category("Electrical"),
        );

        // Resistance (for lossy inductors)
        sheet.add(
            PropertyDefinition::new("r")
                .with_display_name("Series Resistance")
                .with_description("Series DC resistance")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(0.0))
                .with_unit("Ω")
                .with_order(13)
                .with_category("Electrical"),
        );

        // Temperature coefficients
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
            PropertyDefinition::new("dtemp")
                .with_display_name("Temp Rise")
                .with_description("Instance temperature rise above ambient")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(0.0))
                .with_unit("°C")
                .with_order(22)
                .with_category("Temperature"),
        );

        // Initial conditions
        sheet.add(
            PropertyDefinition::new("ic")
                .with_display_name("Initial Current")
                .with_description("Initial current through inductor for transient")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(0.0))
                .with_unit("A")
                .with_order(30)
                .with_category("Initial Conditions"),
        );

        // Mutual inductance coupling
        sheet.add(
            PropertyDefinition::new("coupling_factor")
                .with_display_name("Coupling Factor")
                .with_description("K-factor for mutual inductance (0-1)")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(0.0))
                .with_range(0.0, 1.0)
                .with_order(31)
                .with_category("Coupling"),
        );
        sheet.add(
            PropertyDefinition::new("coupled_to")
                .with_display_name("Coupled To")
                .with_description("Name of coupled inductor")
                .with_type(PropertyType::String)
                .with_default(PropertyValue::string(""))
                .with_order(32)
                .with_category("Coupling"),
        );

        self.sheets.insert(ComponentType::Inductor, sheet);
    }

    /// Register schematic transformer properties.
    ///
    /// Transformers are authored as a single 4-pin schematic component and
    /// synthesized into two winding inductors plus a coupling statement during
    /// netlist generation.
    pub(in super::super) fn register_transformer(&mut self) {
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
    pub(in super::super) fn register_coupled_inductor(&mut self) {
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
