use super::*;

impl PropertyRegistry {
    pub(in super::super) fn register_controlled_sources(&mut self) {
        self.register_vcvs();
        self.register_vccs();
        self.register_ccvs();
        self.register_cccs();
        self.register_opamp();
        self.register_behavioral_source();
        self.register_vswitch();
    }

    /// Register the ideal op-amp (ground-referenced VCVS behind a triangle).
    pub(in super::super) fn register_opamp(&mut self) {
        let mut sheet = PropertySheet::new();

        sheet.add(
            PropertyDefinition::new("name")
                .with_display_name("Instance Name")
                .with_type(PropertyType::String)
                .with_default(PropertyValue::string("E1"))
                .with_order(0)
                .with_category("Instance")
                .required(),
        );
        sheet.add(
            PropertyDefinition::new("gain")
                .with_display_name("Open-Loop Gain")
                .with_description("Output voltage / differential input voltage (linear)")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(100_000.0))
                .with_order(10)
                .with_category("Electrical")
                .required(),
        );

        self.sheets.insert(ComponentType::OpAmp, sheet);
    }

    /// Register the behavioral source (arbitrary V=/I= expression, B element).
    pub(in super::super) fn register_behavioral_source(&mut self) {
        let mut sheet = PropertySheet::new();

        sheet.add(
            PropertyDefinition::new("name")
                .with_display_name("Instance Name")
                .with_type(PropertyType::String)
                .with_default(PropertyValue::string("B1"))
                .with_order(0)
                .with_category("Instance")
                .required(),
        );
        sheet.add(
            PropertyDefinition::new("value")
                .with_display_name("Expression")
                .with_description(
                    "V=<expr> makes a voltage output, I=<expr> a current output, \
                     e.g. V=V(a)*sqrt(V(b)) or I=1m+V(in)/100",
                )
                .with_type(PropertyType::String)
                .with_default(PropertyValue::string("V=0"))
                .with_order(10)
                .with_category("Electrical")
                .required(),
        );

        self.sheets.insert(ComponentType::BehavioralSource, sheet);
    }

    /// Register the voltage-controlled switch (S element + SW model card).
    pub(in super::super) fn register_vswitch(&mut self) {
        let mut sheet = PropertySheet::new();

        sheet.add(
            PropertyDefinition::new("name")
                .with_display_name("Instance Name")
                .with_type(PropertyType::String)
                .with_default(PropertyValue::string("S1"))
                .with_order(0)
                .with_category("Instance")
                .required(),
        );
        sheet.add(
            PropertyDefinition::new("vt")
                .with_display_name("Threshold Voltage")
                .with_description("Control voltage at which the switch toggles")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(0.0))
                .with_unit("V")
                .with_order(10)
                .with_category("Electrical")
                .required(),
        );
        sheet.add(
            PropertyDefinition::new("vh")
                .with_display_name("Hysteresis Voltage")
                .with_description("Half-width of the hysteresis band around the threshold")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(0.0))
                .with_unit("V")
                .with_order(11)
                .with_category("Electrical"),
        );
        sheet.add(
            PropertyDefinition::new("ron")
                .with_display_name("On Resistance")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(1.0))
                .with_unit("Ω")
                .with_order(12)
                .with_category("Electrical"),
        );
        sheet.add(
            PropertyDefinition::new("roff")
                .with_display_name("Off Resistance")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(1e12))
                .with_unit("Ω")
                .with_order(13)
                .with_category("Electrical"),
        );

        self.sheets.insert(ComponentType::VSwitch, sheet);
    }

    /// Register VCVS (Voltage-Controlled Voltage Source) with commercial parameters
    pub(in super::super) fn register_vcvs(&mut self) {
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
    pub(in super::super) fn register_vccs(&mut self) {
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
    pub(in super::super) fn register_ccvs(&mut self) {
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
    pub(in super::super) fn register_cccs(&mut self) {
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
