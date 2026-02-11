use super::*;

impl PropertyRegistry {
    /// Create a new registry with default property sheets for all component types
    pub fn new() -> Self {
        let mut registry = Self {
            sheets: HashMap::new(),
        };
        registry.register_defaults();
        registry
    }

    /// Get the property sheet for a component type
    pub fn get(&self, comp_type: ComponentType) -> Option<&PropertySheet> {
        self.sheets.get(&comp_type)
    }

    /// Register default property sheets for all standard components
    fn register_defaults(&mut self) {
        self.register_passive_components();
        self.register_sources();
        self.register_semiconductors();
        self.register_controlled_sources();
    }

    fn register_passive_components(&mut self) {
        self.register_resistor();
        self.register_capacitor();
        self.register_inductor();
    }

    /// Register Resistor with commercial-grade parameters
    fn register_resistor(&mut self) {
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
    fn register_capacitor(&mut self) {
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
    fn register_inductor(&mut self) {
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

    // =========================================================================
    // Spectre-Parity Helper Functions
    // =========================================================================
    // These helper functions add standard parameter groups to source sheets,
    // ensuring consistency and reducing code duplication across all source types.

    /// Add AC small-signal parameters to a source sheet.
    ///
    /// Parameters: ac (magnitude), acphase (phase in degrees)
    /// Used by: All voltage and current sources for AC analysis
    fn add_ac_params(sheet: &mut PropertySheet, unit: &str, ac_default: f64) {
        sheet.add(
            PropertyDefinition::new("ac")
                .with_display_name("AC Magnitude")
                .with_description("Small-signal AC magnitude for AC analysis")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(ac_default))
                .with_unit(unit)
                .with_order(20)
                .with_category("AC"),
        );
        sheet.add(
            PropertyDefinition::new("acphase")
                .with_display_name("AC Phase")
                .with_description("Small-signal AC phase in degrees")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(0.0))
                .with_unit("°")
                .with_range(-360.0, 360.0)
                .with_order(21)
                .with_category("AC"),
        );
    }

    /// Add Advanced AC analysis parameters (XF/PAC) to a source sheet.
    ///
    /// Parameters: xfmag, pacmag, pacdbm, pacphase
    /// Used by: All sources for transfer function and periodic AC analysis
    fn add_advanced_ac_params(sheet: &mut PropertySheet, unit: &str) {
        sheet.add(
            PropertyDefinition::new("xfmag")
                .with_display_name("XF Magnitude")
                .with_description("Transfer function (XF) analysis magnitude")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(0.0))
                .with_unit(unit)
                .with_order(30)
                .with_category("Advanced AC")
                .advanced(),
        );
        sheet.add(
            PropertyDefinition::new("pacmag")
                .with_display_name("PAC Magnitude")
                .with_description("Periodic AC analysis magnitude")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(0.0))
                .with_unit(unit)
                .with_order(31)
                .with_category("Advanced AC")
                .advanced(),
        );
        sheet.add(
            PropertyDefinition::new("pacdbm")
                .with_display_name("PAC Power (dBm)")
                .with_description("Periodic AC power in dBm (alternative to pacmag)")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(f64::NEG_INFINITY))
                .with_unit("dBm")
                .with_order(32)
                .with_category("Advanced AC")
                .advanced(),
        );
        sheet.add(
            PropertyDefinition::new("pacphase")
                .with_display_name("PAC Phase")
                .with_description("Periodic AC phase in degrees")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(0.0))
                .with_unit("°")
                .with_range(-360.0, 360.0)
                .with_order(33)
                .with_category("Advanced AC")
                .advanced(),
        );
    }

    /// Add parasitic element parameters to a source sheet.
    ///
    /// Parameters: rs (series resistance), rp (parallel resistance), cpar (parasitic capacitance)
    /// Note: Current sources typically omit `rs` as it would change topology.
    fn add_parasitics_params(sheet: &mut PropertySheet, include_series_resistance: bool) {
        if include_series_resistance {
            sheet.add(
                PropertyDefinition::new("rs")
                    .with_display_name("Series Resistance")
                    .with_description("Internal series resistance (0 = ideal)")
                    .with_type(PropertyType::Expression)
                    .with_default(PropertyValue::number(0.0))
                    .with_unit("Ω")
                    .with_range(0.0, 1e15)
                    .with_order(40)
                    .with_category("Parasitics"),
            );
        }
        sheet.add(
            PropertyDefinition::new("rp")
                .with_display_name("Parallel Resistance")
                .with_description("Parallel leakage resistance (inf = ideal)")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(f64::INFINITY))
                .with_unit("Ω")
                .with_range(0.0, f64::INFINITY)
                .with_order(41)
                .with_category("Parasitics"),
        );
        sheet.add(
            PropertyDefinition::new("cpar")
                .with_display_name("Parasitic Capacitance")
                .with_description("Parasitic shunt capacitance")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(0.0))
                .with_unit("F")
                .with_range(0.0, 1e-3)
                .with_order(42)
                .with_category("Parasitics"),
        );
    }

    /// Add noise contribution parameter to a source sheet.
    ///
    /// Parameters: isnoisy (boolean to enable/disable noise contribution)
    fn add_noise_params(sheet: &mut PropertySheet) {
        sheet.add(
            PropertyDefinition::new("isnoisy")
                .with_display_name("Noisy")
                .with_description("Enable noise contribution in noise analysis")
                .with_type(PropertyType::Boolean)
                .with_default(PropertyValue::boolean(true))
                .with_order(50)
                .with_category("Noise"),
        );
    }

    fn register_sources(&mut self) {
        // DC Voltage Source
        self.register_vsource_dc();
        // AC Voltage Source
        self.register_vsource_ac();
        // Transient Voltage Sources
        self.register_vsource_pulse();
        self.register_vsource_sin();
        self.register_vsource_pwl();
        self.register_vsource_exp();
        self.register_vsource_sffm();

        // DC Current Source
        self.register_isource_dc();
        // AC Current Source
        self.register_isource_ac();
        // Transient Current Sources
        self.register_isource_pulse();
        self.register_isource_sin();
        self.register_isource_pwl();
        self.register_isource_exp();
        self.register_isource_noise();
    }

    /// Register DC Voltage Source with Spectre-parity parameters.
    ///
    /// Implements the complete vsource parameter set matching Cadence Spectre:
    /// - DC bias value
    /// - AC small-signal parameters (magnitude, phase)
    /// - Advanced AC analysis (XF, PAC)
    /// - Parasitics (series/parallel resistance, capacitance)
    /// - Noise contribution control
    fn register_vsource_dc(&mut self) {
        let mut sheet = PropertySheet::new();

        // =========================================================================
        // Instance Category (order 0-9)
        // =========================================================================
        sheet.add(
            PropertyDefinition::new("name")
                .with_display_name("Instance Name")
                .with_description("Unique identifier for this voltage source")
                .with_type(PropertyType::String)
                .with_default(PropertyValue::string("V1"))
                .with_order(0)
                .with_category("Instance")
                .required(),
        );

        // =========================================================================
        // DC Category (order 10-19)
        // =========================================================================
        sheet.add(
            PropertyDefinition::new("dc")
                .with_display_name("DC Voltage")
                .with_description("DC voltage value. Used as operating point for AC analysis.")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(0.0))
                .with_unit("V")
                .with_order(10)
                .with_category("DC")
                .required(),
        );

        // =========================================================================
        // AC Category (order 20-29) - Small-signal AC analysis
        // =========================================================================
        sheet.add(
            PropertyDefinition::new("ac")
                .with_display_name("AC Magnitude")
                .with_description("Small-signal AC magnitude for frequency analysis")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(0.0))
                .with_unit("V")
                .with_order(20)
                .with_category("AC"),
        );
        sheet.add(
            PropertyDefinition::new("acphase")
                .with_display_name("AC Phase")
                .with_description("Small-signal AC phase angle")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(0.0))
                .with_unit("°")
                .with_range(-360.0, 360.0)
                .with_order(21)
                .with_category("AC"),
        );

        // =========================================================================
        // Advanced AC Category (order 30-39) - XF/PAC analysis
        // =========================================================================
        sheet.add(
            PropertyDefinition::new("xfmag")
                .with_display_name("XF Magnitude")
                .with_description("Transfer function (XF) analysis magnitude")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(0.0))
                .with_unit("V")
                .with_order(30)
                .with_category("Advanced AC")
                .advanced(),
        );
        sheet.add(
            PropertyDefinition::new("pacmag")
                .with_display_name("PAC Magnitude")
                .with_description("Periodic AC analysis magnitude (linear)")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(0.0))
                .with_unit("V")
                .with_order(31)
                .with_category("Advanced AC")
                .advanced(),
        );
        sheet.add(
            PropertyDefinition::new("pacdbm")
                .with_display_name("PAC dBm")
                .with_description("Periodic AC magnitude in dBm (alternative to pacmag)")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(f64::NEG_INFINITY))
                .with_unit("dBm")
                .with_order(32)
                .with_category("Advanced AC")
                .advanced(),
        );
        sheet.add(
            PropertyDefinition::new("pacphase")
                .with_display_name("PAC Phase")
                .with_description("Periodic AC phase angle")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(0.0))
                .with_unit("°")
                .with_range(-360.0, 360.0)
                .with_order(33)
                .with_category("Advanced AC")
                .advanced(),
        );

        // =========================================================================
        // Parasitics Category (order 40-49) - Non-ideal source characteristics
        // =========================================================================
        sheet.add(
            PropertyDefinition::new("rs")
                .with_display_name("Series Resistance")
                .with_description("Internal series resistance (0 = ideal)")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(0.0))
                .with_unit("Ω")
                .with_range(0.0, 1e15)
                .with_order(40)
                .with_category("Parasitics"),
        );
        sheet.add(
            PropertyDefinition::new("rp")
                .with_display_name("Parallel Resistance")
                .with_description("Internal parallel (shunt) resistance (inf = ideal)")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(f64::INFINITY))
                .with_unit("Ω")
                .with_order(41)
                .with_category("Parasitics"),
        );
        sheet.add(
            PropertyDefinition::new("cpar")
                .with_display_name("Parallel Capacitance")
                .with_description("Parasitic shunt capacitance (0 = none)")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(0.0))
                .with_unit("F")
                .with_range(0.0, 1e3)
                .with_order(42)
                .with_category("Parasitics"),
        );

        // =========================================================================
        // Noise Category (order 50-59)
        // =========================================================================
        sheet.add(
            PropertyDefinition::new("isnoisy")
                .with_display_name("Noisy")
                .with_description("Enable noise contribution in noise analysis")
                .with_type(PropertyType::Boolean)
                .with_default(PropertyValue::boolean(true))
                .with_order(50)
                .with_category("Noise"),
        );

        self.sheets.insert(ComponentType::VoltageSource, sheet);
    }

    /// Register AC Voltage Source with Spectre-parity parameters.
    ///
    /// Similar to DC source but with AC magnitude = 1V by default (the primary parameter).
    /// Used primarily for AC small-signal analysis.
    fn register_vsource_ac(&mut self) {
        let mut sheet = PropertySheet::new();

        // =========================================================================
        // Instance Category (order 0-9)
        // =========================================================================
        sheet.add(
            PropertyDefinition::new("name")
                .with_display_name("Instance Name")
                .with_description("Unique identifier for this AC voltage source")
                .with_type(PropertyType::String)
                .with_default(PropertyValue::string("V1"))
                .with_order(0)
                .with_category("Instance")
                .required(),
        );

        // =========================================================================
        // DC Category (order 10-19) - DC offset for operating point
        // =========================================================================
        sheet.add(
            PropertyDefinition::new("dc")
                .with_display_name("DC Offset")
                .with_description("DC offset voltage for operating point calculation")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(0.0))
                .with_unit("V")
                .with_order(10)
                .with_category("DC"),
        );

        // =========================================================================
        // AC Category (order 20-29) - Primary parameters for AC source
        // =========================================================================
        sheet.add(
            PropertyDefinition::new("ac")
                .with_display_name("AC Magnitude")
                .with_description("Small-signal AC magnitude (primary parameter)")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(1.0)) // Default to 1V for AC source
                .with_unit("V")
                .with_order(20)
                .with_category("AC")
                .required(),
        );
        sheet.add(
            PropertyDefinition::new("acphase")
                .with_display_name("AC Phase")
                .with_description("Small-signal AC phase angle")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(0.0))
                .with_unit("°")
                .with_range(-360.0, 360.0)
                .with_order(21)
                .with_category("AC"),
        );

        // =========================================================================
        // Advanced AC Category (order 30-39) - XF/PAC analysis
        // =========================================================================
        sheet.add(
            PropertyDefinition::new("xfmag")
                .with_display_name("XF Magnitude")
                .with_description("Transfer function (XF) analysis magnitude")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(0.0))
                .with_unit("V")
                .with_order(30)
                .with_category("Advanced AC")
                .advanced(),
        );
        sheet.add(
            PropertyDefinition::new("pacmag")
                .with_display_name("PAC Magnitude")
                .with_description("Periodic AC analysis magnitude (linear)")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(0.0))
                .with_unit("V")
                .with_order(31)
                .with_category("Advanced AC")
                .advanced(),
        );
        sheet.add(
            PropertyDefinition::new("pacdbm")
                .with_display_name("PAC dBm")
                .with_description("Periodic AC magnitude in dBm (alternative to pacmag)")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(f64::NEG_INFINITY))
                .with_unit("dBm")
                .with_order(32)
                .with_category("Advanced AC")
                .advanced(),
        );
        sheet.add(
            PropertyDefinition::new("pacphase")
                .with_display_name("PAC Phase")
                .with_description("Periodic AC phase angle")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(0.0))
                .with_unit("°")
                .with_range(-360.0, 360.0)
                .with_order(33)
                .with_category("Advanced AC")
                .advanced(),
        );

        // =========================================================================
        // Parasitics Category (order 40-49)
        // =========================================================================
        sheet.add(
            PropertyDefinition::new("rs")
                .with_display_name("Series Resistance")
                .with_description("Internal series resistance (0 = ideal)")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(0.0))
                .with_unit("Ω")
                .with_range(0.0, 1e15)
                .with_order(40)
                .with_category("Parasitics"),
        );
        sheet.add(
            PropertyDefinition::new("rp")
                .with_display_name("Parallel Resistance")
                .with_description("Internal parallel (shunt) resistance (inf = ideal)")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(f64::INFINITY))
                .with_unit("Ω")
                .with_order(41)
                .with_category("Parasitics"),
        );
        sheet.add(
            PropertyDefinition::new("cpar")
                .with_display_name("Parallel Capacitance")
                .with_description("Parasitic shunt capacitance (0 = none)")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(0.0))
                .with_unit("F")
                .with_range(0.0, 1e3)
                .with_order(42)
                .with_category("Parasitics"),
        );

        // =========================================================================
        // Noise Category (order 50-59)
        // =========================================================================
        sheet.add(
            PropertyDefinition::new("isnoisy")
                .with_display_name("Noisy")
                .with_description("Enable noise contribution in noise analysis")
                .with_type(PropertyType::Boolean)
                .with_default(PropertyValue::boolean(true))
                .with_order(50)
                .with_category("Noise"),
        );

        self.sheets.insert(ComponentType::VoltageSourceAc, sheet);
    }

    /// Register Pulse Voltage Source with all SPICE PULSE parameters
    fn register_vsource_pulse(&mut self) {
        let mut sheet = PropertySheet::new();

        sheet.add(
            PropertyDefinition::new("name")
                .with_display_name("Instance Name")
                .with_type(PropertyType::String)
                .with_default(PropertyValue::string("V1"))
                .with_order(0)
                .with_category("Instance")
                .required(),
        );

        // PULSE(V1 V2 TD TR TF PW PER)
        sheet.add(
            PropertyDefinition::new("v1")
                .with_display_name("Initial Value (V1)")
                .with_description("Initial voltage before pulse")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(0.0))
                .with_unit("V")
                .with_order(10)
                .with_category("Pulse")
                .required(),
        );
        sheet.add(
            PropertyDefinition::new("v2")
                .with_display_name("Pulsed Value (V2)")
                .with_description("Voltage during pulse")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(1.0))
                .with_unit("V")
                .with_order(11)
                .with_category("Pulse")
                .required(),
        );
        sheet.add(
            PropertyDefinition::new("td")
                .with_display_name("Delay Time (TD)")
                .with_description("Time delay before first pulse")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(0.0))
                .with_unit("s")
                .with_order(12)
                .with_category("Pulse"),
        );
        sheet.add(
            PropertyDefinition::new("tr")
                .with_display_name("Rise Time (TR)")
                .with_description("Rise time from V1 to V2")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(1e-9))
                .with_unit("s")
                .with_order(13)
                .with_category("Pulse"),
        );
        sheet.add(
            PropertyDefinition::new("tf")
                .with_display_name("Fall Time (TF)")
                .with_description("Fall time from V2 to V1")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(1e-9))
                .with_unit("s")
                .with_order(14)
                .with_category("Pulse"),
        );
        sheet.add(
            PropertyDefinition::new("pw")
                .with_display_name("Pulse Width (PW)")
                .with_description("Duration at V2")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(1e-6))
                .with_unit("s")
                .with_order(15)
                .with_category("Pulse"),
        );
        sheet.add(
            PropertyDefinition::new("per")
                .with_display_name("Period (PER)")
                .with_description("Period of pulse waveform")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(2e-6))
                .with_unit("s")
                .with_order(16)
                .with_category("Pulse"),
        );

        // Spectre-parity categories: AC, Advanced AC, Parasitics, Noise
        Self::add_ac_params(&mut sheet, "V", 0.0);
        Self::add_advanced_ac_params(&mut sheet, "V");
        Self::add_parasitics_params(&mut sheet, true);
        Self::add_noise_params(&mut sheet);

        self.sheets.insert(ComponentType::VoltageSourcePulse, sheet);
    }

    /// Register Sinusoidal Voltage Source with all SPICE SIN parameters
    fn register_vsource_sin(&mut self) {
        let mut sheet = PropertySheet::new();

        sheet.add(
            PropertyDefinition::new("name")
                .with_display_name("Instance Name")
                .with_type(PropertyType::String)
                .with_default(PropertyValue::string("V1"))
                .with_order(0)
                .with_category("Instance")
                .required(),
        );

        // SIN(VO VA FREQ TD THETA PHASE)
        sheet.add(
            PropertyDefinition::new("vo")
                .with_display_name("DC Offset (VO)")
                .with_description("DC offset voltage")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(0.0))
                .with_unit("V")
                .with_order(10)
                .with_category("Sinusoid")
                .required(),
        );
        sheet.add(
            PropertyDefinition::new("va")
                .with_display_name("Amplitude (VA)")
                .with_description("Peak amplitude")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(1.0))
                .with_unit("V")
                .with_order(11)
                .with_category("Sinusoid")
                .required(),
        );
        sheet.add(
            PropertyDefinition::new("freq")
                .with_display_name("Frequency (FREQ)")
                .with_description("Frequency in Hz")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(1e6))
                .with_unit("Hz")
                .with_order(12)
                .with_category("Sinusoid")
                .required(),
        );
        sheet.add(
            PropertyDefinition::new("td")
                .with_display_name("Delay Time (TD)")
                .with_description("Time delay before sinusoid starts")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(0.0))
                .with_unit("s")
                .with_order(13)
                .with_category("Sinusoid"),
        );
        sheet.add(
            PropertyDefinition::new("theta")
                .with_display_name("Damping (THETA)")
                .with_description("Damping factor (1/tau)")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(0.0))
                .with_unit("1/s")
                .with_order(14)
                .with_category("Sinusoid"),
        );
        sheet.add(
            PropertyDefinition::new("phase")
                .with_display_name("Phase (PHASE)")
                .with_description("Phase offset in degrees")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(0.0))
                .with_unit("°")
                .with_order(15)
                .with_category("Sinusoid"),
        );

        // Spectre-parity categories: AC, Advanced AC, Parasitics, Noise
        Self::add_ac_params(&mut sheet, "V", 0.0);
        Self::add_advanced_ac_params(&mut sheet, "V");
        Self::add_parasitics_params(&mut sheet, true);
        Self::add_noise_params(&mut sheet);

        self.sheets.insert(ComponentType::VoltageSourceSin, sheet);
    }

    /// Register PWL (Piecewise Linear) Voltage Source
    fn register_vsource_pwl(&mut self) {
        let mut sheet = PropertySheet::new();

        sheet.add(
            PropertyDefinition::new("name")
                .with_display_name("Instance Name")
                .with_type(PropertyType::String)
                .with_default(PropertyValue::string("V1"))
                .with_order(0)
                .with_category("Instance")
                .required(),
        );

        // PWL data string (time-value pairs)
        sheet.add(
            PropertyDefinition::new("pwl_data")
                .with_display_name("PWL Data")
                .with_description("Time-value pairs: t1 v1 t2 v2 ...")
                .with_type(PropertyType::String)
                .with_default(PropertyValue::string("0 0 1u 1 2u 0"))
                .with_order(10)
                .with_category("PWL")
                .required(),
        );
        sheet.add(
            PropertyDefinition::new("td")
                .with_display_name("Delay Time")
                .with_description("Time delay before PWL starts")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(0.0))
                .with_unit("s")
                .with_order(11)
                .with_category("PWL"),
        );
        sheet.add(
            PropertyDefinition::new("repeat")
                .with_display_name("Repeat")
                .with_description("Repeat the waveform periodically")
                .with_type(PropertyType::Boolean)
                .with_default(PropertyValue::boolean(false))
                .with_order(12)
                .with_category("PWL"),
        );

        // Spectre-parity categories: AC, Advanced AC, Parasitics, Noise
        Self::add_ac_params(&mut sheet, "V", 0.0);
        Self::add_advanced_ac_params(&mut sheet, "V");
        Self::add_parasitics_params(&mut sheet, true);
        Self::add_noise_params(&mut sheet);

        self.sheets.insert(ComponentType::VoltageSourcePwl, sheet);
    }

    /// Register Exponential Voltage Source
    fn register_vsource_exp(&mut self) {
        let mut sheet = PropertySheet::new();

        sheet.add(
            PropertyDefinition::new("name")
                .with_display_name("Instance Name")
                .with_type(PropertyType::String)
                .with_default(PropertyValue::string("V1"))
                .with_order(0)
                .with_category("Instance")
                .required(),
        );

        // EXP(V1 V2 TD1 TAU1 TD2 TAU2)
        sheet.add(
            PropertyDefinition::new("v1")
                .with_display_name("Initial Value (V1)")
                .with_description("Initial voltage")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(0.0))
                .with_unit("V")
                .with_order(10)
                .with_category("Exponential")
                .required(),
        );
        sheet.add(
            PropertyDefinition::new("v2")
                .with_display_name("Peak Value (V2)")
                .with_description("Target voltage of first exponential")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(1.0))
                .with_unit("V")
                .with_order(11)
                .with_category("Exponential")
                .required(),
        );
        sheet.add(
            PropertyDefinition::new("td1")
                .with_display_name("Rise Delay (TD1)")
                .with_description("Time delay for rising exponential")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(0.0))
                .with_unit("s")
                .with_order(12)
                .with_category("Exponential"),
        );
        sheet.add(
            PropertyDefinition::new("tau1")
                .with_display_name("Rise Time Const (TAU1)")
                .with_description("Time constant for rising exponential")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(1e-6))
                .with_unit("s")
                .with_order(13)
                .with_category("Exponential"),
        );
        sheet.add(
            PropertyDefinition::new("td2")
                .with_display_name("Fall Delay (TD2)")
                .with_description("Time delay for falling exponential")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(5e-6))
                .with_unit("s")
                .with_order(14)
                .with_category("Exponential"),
        );
        sheet.add(
            PropertyDefinition::new("tau2")
                .with_display_name("Fall Time Const (TAU2)")
                .with_description("Time constant for falling exponential")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(1e-6))
                .with_unit("s")
                .with_order(15)
                .with_category("Exponential"),
        );

        // Spectre-parity categories: AC, Advanced AC, Parasitics, Noise
        Self::add_ac_params(&mut sheet, "V", 0.0);
        Self::add_advanced_ac_params(&mut sheet, "V");
        Self::add_parasitics_params(&mut sheet, true);
        Self::add_noise_params(&mut sheet);

        self.sheets.insert(ComponentType::VoltageSourceExp, sheet);
    }

    /// Register SFFM (Single-Frequency FM) Voltage Source
    fn register_vsource_sffm(&mut self) {
        let mut sheet = PropertySheet::new();

        sheet.add(
            PropertyDefinition::new("name")
                .with_display_name("Instance Name")
                .with_type(PropertyType::String)
                .with_default(PropertyValue::string("V1"))
                .with_order(0)
                .with_category("Instance")
                .required(),
        );

        // SFFM(VO VA FC MDI FS)
        sheet.add(
            PropertyDefinition::new("vo")
                .with_display_name("DC Offset (VO)")
                .with_description("DC offset voltage")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(0.0))
                .with_unit("V")
                .with_order(10)
                .with_category("SFFM")
                .required(),
        );
        sheet.add(
            PropertyDefinition::new("va")
                .with_display_name("Amplitude (VA)")
                .with_description("Carrier amplitude")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(1.0))
                .with_unit("V")
                .with_order(11)
                .with_category("SFFM")
                .required(),
        );
        sheet.add(
            PropertyDefinition::new("fc")
                .with_display_name("Carrier Freq (FC)")
                .with_description("Carrier frequency")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(1e6))
                .with_unit("Hz")
                .with_order(12)
                .with_category("SFFM")
                .required(),
        );
        sheet.add(
            PropertyDefinition::new("mdi")
                .with_display_name("Mod Index (MDI)")
                .with_description("Modulation index")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(1.0))
                .with_order(13)
                .with_category("SFFM"),
        );
        sheet.add(
            PropertyDefinition::new("fs")
                .with_display_name("Signal Freq (FS)")
                .with_description("Signal (modulating) frequency")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(1e3))
                .with_unit("Hz")
                .with_order(14)
                .with_category("SFFM"),
        );

        // Spectre-parity categories: AC, Advanced AC, Parasitics, Noise
        Self::add_ac_params(&mut sheet, "V", 0.0);
        Self::add_advanced_ac_params(&mut sheet, "V");
        Self::add_parasitics_params(&mut sheet, true);
        Self::add_noise_params(&mut sheet);

        self.sheets.insert(ComponentType::VoltageSourceSffm, sheet);
    }

    /// Register DC Current Source with Spectre-parity parameters.
    ///
    /// Mirror of vsource DC parameters but with current units (A instead of V).
    fn register_isource_dc(&mut self) {
        let mut sheet = PropertySheet::new();

        // =========================================================================
        // Instance Category (order 0-9)
        // =========================================================================
        sheet.add(
            PropertyDefinition::new("name")
                .with_display_name("Instance Name")
                .with_description("Unique identifier for this current source")
                .with_type(PropertyType::String)
                .with_default(PropertyValue::string("I1"))
                .with_order(0)
                .with_category("Instance")
                .required(),
        );

        // =========================================================================
        // DC Category (order 10-19)
        // =========================================================================
        sheet.add(
            PropertyDefinition::new("dc")
                .with_display_name("DC Current")
                .with_description("DC current value. Used as operating point for AC analysis.")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(0.0))
                .with_unit("A")
                .with_order(10)
                .with_category("DC")
                .required(),
        );

        // =========================================================================
        // AC Category (order 20-29) - Small-signal AC analysis
        // =========================================================================
        sheet.add(
            PropertyDefinition::new("ac")
                .with_display_name("AC Magnitude")
                .with_description("Small-signal AC magnitude for frequency analysis")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(0.0))
                .with_unit("A")
                .with_order(20)
                .with_category("AC"),
        );
        sheet.add(
            PropertyDefinition::new("acphase")
                .with_display_name("AC Phase")
                .with_description("Small-signal AC phase angle")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(0.0))
                .with_unit("°")
                .with_range(-360.0, 360.0)
                .with_order(21)
                .with_category("AC"),
        );

        // =========================================================================
        // Advanced AC Category (order 30-39) - XF/PAC analysis
        // =========================================================================
        sheet.add(
            PropertyDefinition::new("xfmag")
                .with_display_name("XF Magnitude")
                .with_description("Transfer function (XF) analysis magnitude")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(0.0))
                .with_unit("A")
                .with_order(30)
                .with_category("Advanced AC")
                .advanced(),
        );
        sheet.add(
            PropertyDefinition::new("pacmag")
                .with_display_name("PAC Magnitude")
                .with_description("Periodic AC analysis magnitude (linear)")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(0.0))
                .with_unit("A")
                .with_order(31)
                .with_category("Advanced AC")
                .advanced(),
        );
        sheet.add(
            PropertyDefinition::new("pacdbm")
                .with_display_name("PAC dBm")
                .with_description("Periodic AC magnitude in dBm (alternative to pacmag)")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(f64::NEG_INFINITY))
                .with_unit("dBm")
                .with_order(32)
                .with_category("Advanced AC")
                .advanced(),
        );
        sheet.add(
            PropertyDefinition::new("pacphase")
                .with_display_name("PAC Phase")
                .with_description("Periodic AC phase angle")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(0.0))
                .with_unit("°")
                .with_range(-360.0, 360.0)
                .with_order(33)
                .with_category("Advanced AC")
                .advanced(),
        );

        // =========================================================================
        // Parasitics Category (order 40-49) - Non-ideal source characteristics
        // =========================================================================
        sheet.add(
            PropertyDefinition::new("rp")
                .with_display_name("Parallel Resistance")
                .with_description("Internal parallel resistance (inf = ideal)")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(f64::INFINITY))
                .with_unit("Ω")
                .with_order(40)
                .with_category("Parasitics"),
        );
        sheet.add(
            PropertyDefinition::new("cpar")
                .with_display_name("Parallel Capacitance")
                .with_description("Parasitic shunt capacitance (0 = none)")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(0.0))
                .with_unit("F")
                .with_range(0.0, 1e3)
                .with_order(41)
                .with_category("Parasitics"),
        );

        // =========================================================================
        // Noise Category (order 50-59)
        // =========================================================================
        sheet.add(
            PropertyDefinition::new("isnoisy")
                .with_display_name("Noisy")
                .with_description("Enable noise contribution in noise analysis")
                .with_type(PropertyType::Boolean)
                .with_default(PropertyValue::boolean(true))
                .with_order(50)
                .with_category("Noise"),
        );

        self.sheets.insert(ComponentType::CurrentSource, sheet);
    }

    /// Register AC Current Source with Spectre-parity parameters.
    ///
    /// Similar to DC current source but with AC magnitude = 1A by default.
    fn register_isource_ac(&mut self) {
        let mut sheet = PropertySheet::new();

        // =========================================================================
        // Instance Category (order 0-9)
        // =========================================================================
        sheet.add(
            PropertyDefinition::new("name")
                .with_display_name("Instance Name")
                .with_description("Unique identifier for this AC current source")
                .with_type(PropertyType::String)
                .with_default(PropertyValue::string("I1"))
                .with_order(0)
                .with_category("Instance")
                .required(),
        );

        // =========================================================================
        // DC Category (order 10-19)
        // =========================================================================
        sheet.add(
            PropertyDefinition::new("dc")
                .with_display_name("DC Offset")
                .with_description("DC offset current for operating point calculation")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(0.0))
                .with_unit("A")
                .with_order(10)
                .with_category("DC"),
        );

        // =========================================================================
        // AC Category (order 20-29) - Primary parameters for AC source
        // =========================================================================
        sheet.add(
            PropertyDefinition::new("ac")
                .with_display_name("AC Magnitude")
                .with_description("Small-signal AC magnitude (primary parameter)")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(1.0)) // Default to 1A for AC source
                .with_unit("A")
                .with_order(20)
                .with_category("AC")
                .required(),
        );
        sheet.add(
            PropertyDefinition::new("acphase")
                .with_display_name("AC Phase")
                .with_description("Small-signal AC phase angle")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(0.0))
                .with_unit("°")
                .with_range(-360.0, 360.0)
                .with_order(21)
                .with_category("AC"),
        );

        // =========================================================================
        // Advanced AC Category (order 30-39)
        // =========================================================================
        sheet.add(
            PropertyDefinition::new("xfmag")
                .with_display_name("XF Magnitude")
                .with_description("Transfer function (XF) analysis magnitude")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(0.0))
                .with_unit("A")
                .with_order(30)
                .with_category("Advanced AC")
                .advanced(),
        );
        sheet.add(
            PropertyDefinition::new("pacmag")
                .with_display_name("PAC Magnitude")
                .with_description("Periodic AC analysis magnitude (linear)")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(0.0))
                .with_unit("A")
                .with_order(31)
                .with_category("Advanced AC")
                .advanced(),
        );
        sheet.add(
            PropertyDefinition::new("pacdbm")
                .with_display_name("PAC dBm")
                .with_description("Periodic AC magnitude in dBm (alternative to pacmag)")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(f64::NEG_INFINITY))
                .with_unit("dBm")
                .with_order(32)
                .with_category("Advanced AC")
                .advanced(),
        );
        sheet.add(
            PropertyDefinition::new("pacphase")
                .with_display_name("PAC Phase")
                .with_description("Periodic AC phase angle")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(0.0))
                .with_unit("°")
                .with_range(-360.0, 360.0)
                .with_order(33)
                .with_category("Advanced AC")
                .advanced(),
        );

        // =========================================================================
        // Parasitics Category (order 40-49)
        // =========================================================================
        sheet.add(
            PropertyDefinition::new("rp")
                .with_display_name("Parallel Resistance")
                .with_description("Internal parallel resistance (inf = ideal)")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(f64::INFINITY))
                .with_unit("Ω")
                .with_order(40)
                .with_category("Parasitics"),
        );
        sheet.add(
            PropertyDefinition::new("cpar")
                .with_display_name("Parallel Capacitance")
                .with_description("Parasitic shunt capacitance (0 = none)")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(0.0))
                .with_unit("F")
                .with_range(0.0, 1e3)
                .with_order(41)
                .with_category("Parasitics"),
        );

        // =========================================================================
        // Noise Category (order 50-59)
        // =========================================================================
        sheet.add(
            PropertyDefinition::new("isnoisy")
                .with_display_name("Noisy")
                .with_description("Enable noise contribution in noise analysis")
                .with_type(PropertyType::Boolean)
                .with_default(PropertyValue::boolean(true))
                .with_order(50)
                .with_category("Noise"),
        );

        self.sheets.insert(ComponentType::CurrentSourceAc, sheet);
    }

    /// Register Pulse Current Source
    fn register_isource_pulse(&mut self) {
        let mut sheet = PropertySheet::new();

        sheet.add(
            PropertyDefinition::new("name")
                .with_display_name("Instance Name")
                .with_type(PropertyType::String)
                .with_default(PropertyValue::string("I1"))
                .with_order(0)
                .with_category("Instance")
                .required(),
        );

        // PULSE parameters for current
        sheet.add(
            PropertyDefinition::new("i1")
                .with_display_name("Initial Value (I1)")
                .with_description("Initial current before pulse")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(0.0))
                .with_unit("A")
                .with_order(10)
                .with_category("Pulse")
                .required(),
        );
        sheet.add(
            PropertyDefinition::new("i2")
                .with_display_name("Pulsed Value (I2)")
                .with_description("Current during pulse")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(1e-3))
                .with_unit("A")
                .with_order(11)
                .with_category("Pulse")
                .required(),
        );
        sheet.add(
            PropertyDefinition::new("td")
                .with_display_name("Delay Time (TD)")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(0.0))
                .with_unit("s")
                .with_order(12)
                .with_category("Pulse"),
        );
        sheet.add(
            PropertyDefinition::new("tr")
                .with_display_name("Rise Time (TR)")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(1e-9))
                .with_unit("s")
                .with_order(13)
                .with_category("Pulse"),
        );
        sheet.add(
            PropertyDefinition::new("tf")
                .with_display_name("Fall Time (TF)")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(1e-9))
                .with_unit("s")
                .with_order(14)
                .with_category("Pulse"),
        );
        sheet.add(
            PropertyDefinition::new("pw")
                .with_display_name("Pulse Width (PW)")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(1e-6))
                .with_unit("s")
                .with_order(15)
                .with_category("Pulse"),
        );
        sheet.add(
            PropertyDefinition::new("per")
                .with_display_name("Period (PER)")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(2e-6))
                .with_unit("s")
                .with_order(16)
                .with_category("Pulse"),
        );

        // Spectre-parity categories: AC, Advanced AC, Parasitics, Noise
        // Current sources don't have series resistance (would change node topology)
        Self::add_ac_params(&mut sheet, "A", 0.0);
        Self::add_advanced_ac_params(&mut sheet, "A");
        Self::add_parasitics_params(&mut sheet, false);
        Self::add_noise_params(&mut sheet);

        self.sheets.insert(ComponentType::CurrentSourcePulse, sheet);
    }

    /// Register Sinusoidal Current Source
    fn register_isource_sin(&mut self) {
        let mut sheet = PropertySheet::new();

        sheet.add(
            PropertyDefinition::new("name")
                .with_display_name("Instance Name")
                .with_type(PropertyType::String)
                .with_default(PropertyValue::string("I1"))
                .with_order(0)
                .with_category("Instance")
                .required(),
        );

        // SIN parameters for current
        sheet.add(
            PropertyDefinition::new("io")
                .with_display_name("DC Offset (IO)")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(0.0))
                .with_unit("A")
                .with_order(10)
                .with_category("Sinusoid")
                .required(),
        );
        sheet.add(
            PropertyDefinition::new("ia")
                .with_display_name("Amplitude (IA)")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(1e-3))
                .with_unit("A")
                .with_order(11)
                .with_category("Sinusoid")
                .required(),
        );
        sheet.add(
            PropertyDefinition::new("freq")
                .with_display_name("Frequency")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(1e6))
                .with_unit("Hz")
                .with_order(12)
                .with_category("Sinusoid")
                .required(),
        );
        sheet.add(
            PropertyDefinition::new("td")
                .with_display_name("Delay Time")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(0.0))
                .with_unit("s")
                .with_order(13)
                .with_category("Sinusoid"),
        );
        sheet.add(
            PropertyDefinition::new("theta")
                .with_display_name("Damping")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(0.0))
                .with_unit("1/s")
                .with_order(14)
                .with_category("Sinusoid"),
        );
        sheet.add(
            PropertyDefinition::new("phase")
                .with_display_name("Phase")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(0.0))
                .with_unit("°")
                .with_order(15)
                .with_category("Sinusoid"),
        );

        // Spectre-parity categories: AC, Advanced AC, Parasitics, Noise
        Self::add_ac_params(&mut sheet, "A", 0.0);
        Self::add_advanced_ac_params(&mut sheet, "A");
        Self::add_parasitics_params(&mut sheet, false);
        Self::add_noise_params(&mut sheet);

        self.sheets.insert(ComponentType::CurrentSourceSin, sheet);
    }

    /// Register PWL Current Source
    fn register_isource_pwl(&mut self) {
        let mut sheet = PropertySheet::new();

        sheet.add(
            PropertyDefinition::new("name")
                .with_display_name("Instance Name")
                .with_type(PropertyType::String)
                .with_default(PropertyValue::string("I1"))
                .with_order(0)
                .with_category("Instance")
                .required(),
        );
        sheet.add(
            PropertyDefinition::new("pwl_data")
                .with_display_name("PWL Data")
                .with_description("Time-value pairs: t1 i1 t2 i2 ...")
                .with_type(PropertyType::String)
                .with_default(PropertyValue::string("0 0 1u 1m 2u 0"))
                .with_order(10)
                .with_category("PWL")
                .required(),
        );
        sheet.add(
            PropertyDefinition::new("td")
                .with_display_name("Delay Time")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(0.0))
                .with_unit("s")
                .with_order(11)
                .with_category("PWL"),
        );
        sheet.add(
            PropertyDefinition::new("repeat")
                .with_display_name("Repeat")
                .with_type(PropertyType::Boolean)
                .with_default(PropertyValue::boolean(false))
                .with_order(12)
                .with_category("PWL"),
        );

        // Spectre-parity categories: AC, Advanced AC, Parasitics, Noise
        Self::add_ac_params(&mut sheet, "A", 0.0);
        Self::add_advanced_ac_params(&mut sheet, "A");
        Self::add_parasitics_params(&mut sheet, false);
        Self::add_noise_params(&mut sheet);

        self.sheets.insert(ComponentType::CurrentSourcePwl, sheet);
    }

    /// Register Exponential Current Source
    fn register_isource_exp(&mut self) {
        let mut sheet = PropertySheet::new();

        sheet.add(
            PropertyDefinition::new("name")
                .with_display_name("Instance Name")
                .with_type(PropertyType::String)
                .with_default(PropertyValue::string("I1"))
                .with_order(0)
                .with_category("Instance")
                .required(),
        );

        // EXP parameters
        sheet.add(
            PropertyDefinition::new("i1")
                .with_display_name("Initial Value (I1)")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(0.0))
                .with_unit("A")
                .with_order(10)
                .with_category("Exponential")
                .required(),
        );
        sheet.add(
            PropertyDefinition::new("i2")
                .with_display_name("Peak Value (I2)")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(1e-3))
                .with_unit("A")
                .with_order(11)
                .with_category("Exponential")
                .required(),
        );
        sheet.add(
            PropertyDefinition::new("td1")
                .with_display_name("Rise Delay")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(0.0))
                .with_unit("s")
                .with_order(12)
                .with_category("Exponential"),
        );
        sheet.add(
            PropertyDefinition::new("tau1")
                .with_display_name("Rise Time Const")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(1e-6))
                .with_unit("s")
                .with_order(13)
                .with_category("Exponential"),
        );
        sheet.add(
            PropertyDefinition::new("td2")
                .with_display_name("Fall Delay")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(5e-6))
                .with_unit("s")
                .with_order(14)
                .with_category("Exponential"),
        );
        sheet.add(
            PropertyDefinition::new("tau2")
                .with_display_name("Fall Time Const")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(1e-6))
                .with_unit("s")
                .with_order(15)
                .with_category("Exponential"),
        );

        // Spectre-parity categories: AC, Advanced AC, Parasitics, Noise
        Self::add_ac_params(&mut sheet, "A", 0.0);
        Self::add_advanced_ac_params(&mut sheet, "A");
        Self::add_parasitics_params(&mut sheet, false);
        Self::add_noise_params(&mut sheet);

        self.sheets.insert(ComponentType::CurrentSourceExp, sheet);
    }

    /// Register Noise Current Source (for noise analysis)
    fn register_isource_noise(&mut self) {
        let mut sheet = PropertySheet::new();

        sheet.add(
            PropertyDefinition::new("name")
                .with_display_name("Instance Name")
                .with_type(PropertyType::String)
                .with_default(PropertyValue::string("I1"))
                .with_order(0)
                .with_category("Instance")
                .required(),
        );
        sheet.add(
            PropertyDefinition::new("dc")
                .with_display_name("DC Current")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(0.0))
                .with_unit("A")
                .with_order(1)
                .with_category("DC"),
        );
        sheet.add(
            PropertyDefinition::new("noise_type")
                .with_display_name("Noise Type")
                .with_description("Type of noise source")
                .with_type(PropertyType::Enum)
                .with_default(PropertyValue::enumeration(
                    "white",
                    vec![
                        "white".to_string(),
                        "flicker".to_string(),
                        "shot".to_string(),
                    ],
                ))
                .with_order(10)
                .with_category("Noise"),
        );
        sheet.add(
            PropertyDefinition::new("noiseval")
                .with_display_name("Noise Value")
                .with_description("Noise spectral density (A²/Hz)")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(1e-24))
                .with_unit("A²/Hz")
                .with_order(11)
                .with_category("Noise"),
        );
        sheet.add(
            PropertyDefinition::new("kf")
                .with_display_name("Flicker Coeff (KF)")
                .with_description("Flicker noise coefficient")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(0.0))
                .with_order(12)
                .with_category("Noise"),
        );
        sheet.add(
            PropertyDefinition::new("af")
                .with_display_name("Flicker Exp (AF)")
                .with_description("Flicker noise exponent")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(1.0))
                .with_order(13)
                .with_category("Noise"),
        );

        // Spectre-parity categories: AC, Advanced AC, Parasitics (noise source already has noise params)
        Self::add_ac_params(&mut sheet, "A", 0.0);
        Self::add_advanced_ac_params(&mut sheet, "A");
        Self::add_parasitics_params(&mut sheet, false);

        // Add isnoisy flag for consistency (noise source is always noisy by definition)
        sheet.add(
            PropertyDefinition::new("isnoisy")
                .with_display_name("Noisy")
                .with_description("Enable noise contribution (always true for noise source)")
                .with_type(PropertyType::Boolean)
                .with_default(PropertyValue::boolean(true))
                .with_order(200)
                .with_category("Noise"),
        );

        self.sheets.insert(ComponentType::CurrentSourceNoise, sheet);
    }

    fn register_semiconductors(&mut self) {
        // Diode with commercial-grade parameters
        self.register_diode();

        // MOSFET (NMOS/PMOS) with full Spectre-compatible parameters
        self.register_mosfet();

        // BJT (NPN/PNP) with commercial-grade parameters
        self.register_bjt();

        // JFET (NJFET/PJFET)
        self.register_jfet();
    }

    /// Register diode with all SPICE-standard parameters
    fn register_diode(&mut self) {
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

        // Temperature category
        diode.add(
            PropertyDefinition::new("dtemp")
                .with_display_name("Temp Rise")
                .with_description("Instance temperature rise above ambient")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(0.0))
                .with_unit("°C")
                .with_order(30)
                .with_category("Temperature"),
        );

        // Initial Conditions category
        diode.add(
            PropertyDefinition::new("off")
                .with_display_name("Initially Off")
                .with_description("Start in off state for DC operating point")
                .with_type(PropertyType::Boolean)
                .with_default(PropertyValue::boolean(false))
                .with_order(40)
                .with_category("Initial Conditions"),
        );
        diode.add(
            PropertyDefinition::new("ic")
                .with_display_name("Initial Voltage")
                .with_description("Initial voltage across diode for transient")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(0.0))
                .with_unit("V")
                .with_order(41)
                .with_category("Initial Conditions"),
        );

        self.sheets.insert(ComponentType::Diode, diode);
    }

    /// Register MOSFET (NMOS/PMOS) with all Spectre-compatible parameters
    fn register_mosfet(&mut self) {
        let nmos = self.create_mosfet_sheet("M1", "nmos");
        self.sheets.insert(ComponentType::Nmos, nmos);

        let pmos = self.create_mosfet_sheet("M1", "pmos");
        self.sheets.insert(ComponentType::Pmos, pmos);
    }

    /// Create a MOSFET property sheet with commercial-grade parameters
    fn create_mosfet_sheet(&self, default_name: &str, default_model: &str) -> PropertySheet {
        let mut sheet = PropertySheet::new();

        // =========================================================================
        // Instance Category (order 0-9)
        // =========================================================================
        sheet.add(
            PropertyDefinition::new("name")
                .with_display_name("Instance Name")
                .with_description("Unique identifier for this MOSFET instance")
                .with_type(PropertyType::String)
                .with_default(PropertyValue::string(default_name))
                .with_order(0)
                .with_category("Instance")
                .required(),
        );

        // =========================================================================
        // Model Category (order 10-19)
        // =========================================================================
        sheet.add(
            PropertyDefinition::new("model")
                .with_display_name("Model")
                .with_description("MOSFET model name from PDK library")
                .with_type(PropertyType::String)
                .with_default(PropertyValue::string(default_model))
                .with_order(10)
                .with_category("Model"),
        );

        // =========================================================================
        // Geometry Category (order 20-39)
        // =========================================================================
        sheet.add(
            PropertyDefinition::new("w")
                .with_display_name("Width")
                .with_description("Channel width (drawn)")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::expression("1u"))
                .with_unit("m")
                .with_range(1e-9, 1e-3)
                .with_order(20)
                .with_category("Geometry")
                .required(),
        );
        sheet.add(
            PropertyDefinition::new("l")
                .with_display_name("Length")
                .with_description("Channel length (drawn)")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::expression("180n"))
                .with_unit("m")
                .with_range(1e-9, 1e-3)
                .with_order(21)
                .with_category("Geometry")
                .required(),
        );
        sheet.add(
            PropertyDefinition::new("m")
                .with_display_name("Multiplier")
                .with_description("Number of parallel devices (total W = m × nf × w)")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(1.0))
                .with_range(1.0, 10000.0)
                .with_order(22)
                .with_category("Geometry"),
        );
        sheet.add(
            PropertyDefinition::new("nf")
                .with_display_name("# Fingers")
                .with_description("Number of gate fingers")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(1.0))
                .with_range(1.0, 1000.0)
                .with_order(23)
                .with_category("Geometry"),
        );

        // =========================================================================
        // Parasitics Category (order 40-59)
        // Source/Drain areas and perimeters for junction capacitance
        // =========================================================================
        sheet.add(
            PropertyDefinition::new("as")
                .with_display_name("Source Area")
                .with_description("Source diffusion area for junction capacitance")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::expression("0"))
                .with_unit("m²")
                .with_order(40)
                .with_category("Parasitics"),
        );
        sheet.add(
            PropertyDefinition::new("ad")
                .with_display_name("Drain Area")
                .with_description("Drain diffusion area for junction capacitance")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::expression("0"))
                .with_unit("m²")
                .with_order(41)
                .with_category("Parasitics"),
        );
        sheet.add(
            PropertyDefinition::new("ps")
                .with_display_name("Source Perimeter")
                .with_description("Source diffusion perimeter for sidewall capacitance")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::expression("0"))
                .with_unit("m")
                .with_order(42)
                .with_category("Parasitics"),
        );
        sheet.add(
            PropertyDefinition::new("pd")
                .with_display_name("Drain Perimeter")
                .with_description("Drain diffusion perimeter for sidewall capacitance")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::expression("0"))
                .with_unit("m")
                .with_order(43)
                .with_category("Parasitics"),
        );
        sheet.add(
            PropertyDefinition::new("nrd")
                .with_display_name("Drain Squares")
                .with_description("Number of squares for drain series resistance")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(0.0))
                .with_order(44)
                .with_category("Parasitics"),
        );
        sheet.add(
            PropertyDefinition::new("nrs")
                .with_display_name("Source Squares")
                .with_description("Number of squares for source series resistance")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(0.0))
                .with_order(45)
                .with_category("Parasitics"),
        );

        // =========================================================================
        // Stress Category (order 60-79) - STI stress effects for advanced nodes
        // =========================================================================
        sheet.add(
            PropertyDefinition::new("sa")
                .with_display_name("SA Distance")
                .with_description("Distance from gate edge to STI on source side")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::expression("0"))
                .with_unit("m")
                .with_order(60)
                .with_category("Stress"),
        );
        sheet.add(
            PropertyDefinition::new("sb")
                .with_display_name("SB Distance")
                .with_description("Distance from gate edge to STI on drain side")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::expression("0"))
                .with_unit("m")
                .with_order(61)
                .with_category("Stress"),
        );
        sheet.add(
            PropertyDefinition::new("sd")
                .with_display_name("SD Spacing")
                .with_description("Source-drain spacing for multi-finger devices")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::expression("0"))
                .with_unit("m")
                .with_order(62)
                .with_category("Stress"),
        );
        sheet.add(
            PropertyDefinition::new("sca")
                .with_display_name("SCA")
                .with_description("Integral of first distribution function for STI stress")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(0.0))
                .with_order(63)
                .with_category("Stress"),
        );
        sheet.add(
            PropertyDefinition::new("scb")
                .with_display_name("SCB")
                .with_description("Integral of second distribution function for STI stress")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(0.0))
                .with_order(64)
                .with_category("Stress"),
        );
        sheet.add(
            PropertyDefinition::new("scc")
                .with_display_name("SCC")
                .with_description("Integral of third distribution function for STI stress")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(0.0))
                .with_order(65)
                .with_category("Stress"),
        );

        // =========================================================================
        // Temperature Category (order 80-89)
        // =========================================================================
        sheet.add(
            PropertyDefinition::new("dtemp")
                .with_display_name("Temp Rise")
                .with_description("Instance temperature rise above ambient")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(0.0))
                .with_unit("°C")
                .with_order(80)
                .with_category("Temperature"),
        );

        // =========================================================================
        // Initial Conditions Category (order 90-99)
        // =========================================================================
        sheet.add(
            PropertyDefinition::new("off")
                .with_display_name("Initially Off")
                .with_description("Start in off state for DC operating point analysis")
                .with_type(PropertyType::Boolean)
                .with_default(PropertyValue::boolean(false))
                .with_order(90)
                .with_category("Initial Conditions"),
        );
        sheet.add(
            PropertyDefinition::new("region")
                .with_display_name("Region Hint")
                .with_description("Estimated operating region for convergence aid")
                .with_type(PropertyType::Enum)
                .with_default(PropertyValue::enumeration(
                    "auto",
                    vec![
                        "auto".to_string(),
                        "off".to_string(),
                        "triode".to_string(),
                        "sat".to_string(),
                        "subth".to_string(),
                    ],
                ))
                .with_order(91)
                .with_category("Initial Conditions"),
        );
        sheet.add(
            PropertyDefinition::new("ic_vds")
                .with_display_name("IC VDS")
                .with_description("Initial drain-source voltage for transient analysis")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(0.0))
                .with_unit("V")
                .with_order(92)
                .with_category("Initial Conditions"),
        );
        sheet.add(
            PropertyDefinition::new("ic_vgs")
                .with_display_name("IC VGS")
                .with_description("Initial gate-source voltage for transient analysis")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(0.0))
                .with_unit("V")
                .with_order(93)
                .with_category("Initial Conditions"),
        );
        sheet.add(
            PropertyDefinition::new("ic_vbs")
                .with_display_name("IC VBS")
                .with_description("Initial bulk-source voltage for transient analysis")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(0.0))
                .with_unit("V")
                .with_order(94)
                .with_category("Initial Conditions"),
        );

        sheet
    }

    /// Register BJT (NPN/PNP) with commercial-grade parameters
    fn register_bjt(&mut self) {
        let npn = self.create_bjt_sheet("Q1", "npn");
        self.sheets.insert(ComponentType::NpnBjt, npn);

        let pnp = self.create_bjt_sheet("Q1", "pnp");
        self.sheets.insert(ComponentType::PnpBjt, pnp);
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
        sheet.add(
            PropertyDefinition::new("areab")
                .with_display_name("Base Area Factor")
                .with_description("Base area multiplier for parasitic capacitance")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(1.0))
                .with_range(1e-6, 1e6)
                .with_order(21)
                .with_category("Geometry"),
        );
        sheet.add(
            PropertyDefinition::new("areac")
                .with_display_name("Collector Area Factor")
                .with_description("Collector area multiplier for parasitic capacitance")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(1.0))
                .with_range(1e-6, 1e6)
                .with_order(22)
                .with_category("Geometry"),
        );
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
            PropertyDefinition::new("dtemp")
                .with_display_name("Temp Rise")
                .with_description("Instance temperature rise above ambient")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(0.0))
                .with_unit("°C")
                .with_order(30)
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
        sheet.add(
            PropertyDefinition::new("region")
                .with_display_name("Region Hint")
                .with_description("Estimated operating region for convergence aid")
                .with_type(PropertyType::Enum)
                .with_default(PropertyValue::enumeration(
                    "auto",
                    vec![
                        "auto".to_string(),
                        "off".to_string(),
                        "fwd".to_string(),
                        "rev".to_string(),
                        "sat".to_string(),
                    ],
                ))
                .with_order(41)
                .with_category("Initial Conditions"),
        );
        sheet.add(
            PropertyDefinition::new("ic_vbe")
                .with_display_name("IC VBE")
                .with_description("Initial base-emitter voltage for transient analysis")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(0.0))
                .with_unit("V")
                .with_order(42)
                .with_category("Initial Conditions"),
        );
        sheet.add(
            PropertyDefinition::new("ic_vce")
                .with_display_name("IC VCE")
                .with_description("Initial collector-emitter voltage for transient analysis")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(0.0))
                .with_unit("V")
                .with_order(43)
                .with_category("Initial Conditions"),
        );

        sheet
    }

    /// Register JFET (NJFET/PJFET) with commercial-grade parameters
    fn register_jfet(&mut self) {
        let njfet = self.create_jfet_sheet("J1", "njfet");
        self.sheets.insert(ComponentType::Njfet, njfet);

        let pjfet = self.create_jfet_sheet("J1", "pjfet");
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

        // Temperature category
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
        sheet.add(
            PropertyDefinition::new("ic_vds")
                .with_display_name("IC VDS")
                .with_description("Initial drain-source voltage for transient analysis")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(0.0))
                .with_unit("V")
                .with_order(41)
                .with_category("Initial Conditions"),
        );
        sheet.add(
            PropertyDefinition::new("ic_vgs")
                .with_display_name("IC VGS")
                .with_description("Initial gate-source voltage for transient analysis")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(0.0))
                .with_unit("V")
                .with_order(42)
                .with_category("Initial Conditions"),
        );

        sheet
    }

    fn register_controlled_sources(&mut self) {
        self.register_vcvs();
        self.register_vccs();
        self.register_ccvs();
        self.register_cccs();
    }

    /// Register VCVS (Voltage-Controlled Voltage Source) with commercial parameters
    fn register_vcvs(&mut self) {
        let mut sheet = PropertySheet::new();

        // Instance
        sheet.add(
            PropertyDefinition::new("name")
                .with_display_name("Instance Name")
                .with_type(PropertyType::String)
                .with_default(PropertyValue::string("E1"))
                .with_order(0)
                .with_category("Instance")
                .required(),
        );

        // Electrical - Gain
        sheet.add(
            PropertyDefinition::new("gain")
                .with_display_name("Voltage Gain")
                .with_description("Output voltage / Input voltage (linear multiplier)")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(1.0))
                .with_order(10)
                .with_category("Electrical")
                .required(),
        );
        sheet.add(
            PropertyDefinition::new("m")
                .with_display_name("Multiplier")
                .with_description("Number of parallel sources")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(1.0))
                .with_range(1.0, 10000.0)
                .with_order(11)
                .with_category("Electrical"),
        );

        // Polynomial coefficients (for nonlinear behavior)
        sheet.add(
            PropertyDefinition::new("poly")
                .with_display_name("Poly Coefficients")
                .with_description("Polynomial coefficients: output = c0 + c1*v + c2*v² + ...")
                .with_type(PropertyType::String)
                .with_default(PropertyValue::string(""))
                .with_order(20)
                .with_category("Polynomial"),
        );

        // AC/Small-signal parameters
        sheet.add(
            PropertyDefinition::new("ac_gain")
                .with_display_name("AC Gain")
                .with_description("AC analysis gain (if different from DC)")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(0.0))
                .with_order(30)
                .with_category("AC"),
        );
        sheet.add(
            PropertyDefinition::new("ac_phase")
                .with_display_name("AC Phase")
                .with_description("AC phase shift")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(0.0))
                .with_unit("°")
                .with_order(31)
                .with_category("AC"),
        );

        // Limiting parameters
        sheet.add(
            PropertyDefinition::new("vmax")
                .with_display_name("Max Output")
                .with_description("Maximum output voltage (clipping)")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(1e308))
                .with_unit("V")
                .with_order(40)
                .with_category("Limits"),
        );
        sheet.add(
            PropertyDefinition::new("vmin")
                .with_display_name("Min Output")
                .with_description("Minimum output voltage (clipping)")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(-1e308))
                .with_unit("V")
                .with_order(41)
                .with_category("Limits"),
        );

        self.sheets.insert(ComponentType::Vcvs, sheet);
    }

    /// Register VCCS (Voltage-Controlled Current Source) with commercial parameters
    fn register_vccs(&mut self) {
        let mut sheet = PropertySheet::new();

        sheet.add(
            PropertyDefinition::new("name")
                .with_display_name("Instance Name")
                .with_type(PropertyType::String)
                .with_default(PropertyValue::string("G1"))
                .with_order(0)
                .with_category("Instance")
                .required(),
        );

        sheet.add(
            PropertyDefinition::new("gm")
                .with_display_name("Transconductance")
                .with_description("Output current / Input voltage")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(1e-3))
                .with_unit("S")
                .with_order(10)
                .with_category("Electrical")
                .required(),
        );
        sheet.add(
            PropertyDefinition::new("m")
                .with_display_name("Multiplier")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(1.0))
                .with_range(1.0, 10000.0)
                .with_order(11)
                .with_category("Electrical"),
        );

        // Polynomial
        sheet.add(
            PropertyDefinition::new("poly")
                .with_display_name("Poly Coefficients")
                .with_description("Polynomial: output = c0 + c1*v + c2*v² ...")
                .with_type(PropertyType::String)
                .with_default(PropertyValue::string(""))
                .with_order(20)
                .with_category("Polynomial"),
        );

        // AC parameters
        sheet.add(
            PropertyDefinition::new("ac_gm")
                .with_display_name("AC Transconductance")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(0.0))
                .with_unit("S")
                .with_order(30)
                .with_category("AC"),
        );

        // Limiting
        sheet.add(
            PropertyDefinition::new("imax")
                .with_display_name("Max Output")
                .with_description("Maximum output current")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(1e308))
                .with_unit("A")
                .with_order(40)
                .with_category("Limits"),
        );
        sheet.add(
            PropertyDefinition::new("imin")
                .with_display_name("Min Output")
                .with_description("Minimum output current")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(-1e308))
                .with_unit("A")
                .with_order(41)
                .with_category("Limits"),
        );

        self.sheets.insert(ComponentType::Vccs, sheet);
    }

    /// Register CCVS (Current-Controlled Voltage Source) with commercial parameters
    fn register_ccvs(&mut self) {
        let mut sheet = PropertySheet::new();

        sheet.add(
            PropertyDefinition::new("name")
                .with_display_name("Instance Name")
                .with_type(PropertyType::String)
                .with_default(PropertyValue::string("H1"))
                .with_order(0)
                .with_category("Instance")
                .required(),
        );

        sheet.add(
            PropertyDefinition::new("rm")
                .with_display_name("Transresistance")
                .with_description("Output voltage / Input current")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(1000.0))
                .with_unit("Ω")
                .with_order(10)
                .with_category("Electrical")
                .required(),
        );
        sheet.add(
            PropertyDefinition::new("m")
                .with_display_name("Multiplier")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(1.0))
                .with_range(1.0, 10000.0)
                .with_order(11)
                .with_category("Electrical"),
        );

        sheet.add(
            PropertyDefinition::new("vref")
                .with_display_name("Sensing Branch")
                .with_description("Name of voltage source sensing control current")
                .with_type(PropertyType::String)
                .with_default(PropertyValue::string(""))
                .with_order(12)
                .with_category("Electrical"),
        );

        // Polynomial
        sheet.add(
            PropertyDefinition::new("poly")
                .with_display_name("Poly Coefficients")
                .with_description("Polynomial: output = c0 + c1*i + c2*i² ...")
                .with_type(PropertyType::String)
                .with_default(PropertyValue::string(""))
                .with_order(20)
                .with_category("Polynomial"),
        );

        // Limiting
        sheet.add(
            PropertyDefinition::new("vmax")
                .with_display_name("Max Output")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(1e308))
                .with_unit("V")
                .with_order(40)
                .with_category("Limits"),
        );
        sheet.add(
            PropertyDefinition::new("vmin")
                .with_display_name("Min Output")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(-1e308))
                .with_unit("V")
                .with_order(41)
                .with_category("Limits"),
        );

        self.sheets.insert(ComponentType::Ccvs, sheet);
    }

    /// Register CCCS (Current-Controlled Current Source) with commercial parameters
    fn register_cccs(&mut self) {
        let mut sheet = PropertySheet::new();

        sheet.add(
            PropertyDefinition::new("name")
                .with_display_name("Instance Name")
                .with_type(PropertyType::String)
                .with_default(PropertyValue::string("F1"))
                .with_order(0)
                .with_category("Instance")
                .required(),
        );

        sheet.add(
            PropertyDefinition::new("gain")
                .with_display_name("Current Gain")
                .with_description("Output current / Input current")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(1.0))
                .with_order(10)
                .with_category("Electrical")
                .required(),
        );
        sheet.add(
            PropertyDefinition::new("m")
                .with_display_name("Multiplier")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(1.0))
                .with_range(1.0, 10000.0)
                .with_order(11)
                .with_category("Electrical"),
        );

        sheet.add(
            PropertyDefinition::new("vref")
                .with_display_name("Sensing Branch")
                .with_description("Name of voltage source sensing control current")
                .with_type(PropertyType::String)
                .with_default(PropertyValue::string(""))
                .with_order(12)
                .with_category("Electrical"),
        );

        // Polynomial
        sheet.add(
            PropertyDefinition::new("poly")
                .with_display_name("Poly Coefficients")
                .with_description("Polynomial: output = c0 + c1*i + c2*i² ...")
                .with_type(PropertyType::String)
                .with_default(PropertyValue::string(""))
                .with_order(20)
                .with_category("Polynomial"),
        );

        // Limiting
        sheet.add(
            PropertyDefinition::new("imax")
                .with_display_name("Max Output")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(1e308))
                .with_unit("A")
                .with_order(40)
                .with_category("Limits"),
        );
        sheet.add(
            PropertyDefinition::new("imin")
                .with_display_name("Min Output")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(-1e308))
                .with_unit("A")
                .with_order(41)
                .with_category("Limits"),
        );

        self.sheets.insert(ComponentType::Cccs, sheet);
    }
}
