use super::*;

fn source_symbol_property(
    description: &str,
    default_variant: &str,
    options: &[&str],
) -> PropertyDefinition {
    PropertyDefinition::new("symbol")
        .with_display_name("Symbol")
        .with_description(description)
        .with_type(PropertyType::Enum)
        .with_default(PropertyValue::enumeration(
            default_variant,
            options.iter().map(|option| (*option).to_string()).collect(),
        ))
        .with_order(900)
        .with_category("Appearance")
}

impl PropertyRegistry {
    pub(in super::super) fn register_sources(&mut self) {
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

        self.register_ground();
    }

    /// Register DC Voltage Source with Spectre-parity parameters.
    ///
    /// Implements the complete vsource parameter set matching Cadence Spectre:
    /// - DC bias value
    /// - AC small-signal parameters (magnitude, phase)
    /// - Advanced AC analysis (XF, PAC)
    /// - Parasitics (series/parallel resistance, capacitance)
    /// - Noise contribution control
    pub(in super::super) fn register_vsource_dc(&mut self) {
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
        sheet.add(source_symbol_property(
            "Schematic symbol skin for this voltage source",
            "default",
            &["default", "battery", "battery_multi_cell"],
        ));

        self.sheets.insert(ComponentType::VoltageSource, sheet);
    }

    pub(in super::super) fn register_ground(&mut self) {
        let mut sheet = PropertySheet::new();
        sheet.add(
            PropertyDefinition::new("name")
                .with_display_name("Net Name")
                .with_description("Displayed ground net label")
                .with_type(PropertyType::String)
                .with_default(PropertyValue::string("0"))
                .with_order(0)
                .with_category("Instance"),
        );
        sheet.add(source_symbol_property(
            "Schematic symbol skin for ground",
            "default",
            &["default", "earth", "chassis"],
        ));
        self.sheets.insert(ComponentType::Ground, sheet);
    }

    /// Register AC Voltage Source with Spectre-parity parameters.
    ///
    /// Similar to DC source but with AC magnitude = 1V by default (the primary parameter).
    /// Used primarily for AC small-signal analysis.
    pub(in super::super) fn register_vsource_ac(&mut self) {
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
    pub(in super::super) fn register_vsource_pulse(&mut self) {
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
    pub(in super::super) fn register_vsource_sin(&mut self) {
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
    pub(in super::super) fn register_vsource_pwl(&mut self) {
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
    pub(in super::super) fn register_vsource_exp(&mut self) {
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
    pub(in super::super) fn register_vsource_sffm(&mut self) {
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
    pub(in super::super) fn register_isource_dc(&mut self) {
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
    pub(in super::super) fn register_isource_ac(&mut self) {
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
    pub(in super::super) fn register_isource_pulse(&mut self) {
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
    pub(in super::super) fn register_isource_sin(&mut self) {
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
    pub(in super::super) fn register_isource_pwl(&mut self) {
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
    pub(in super::super) fn register_isource_exp(&mut self) {
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
    pub(in super::super) fn register_isource_noise(&mut self) {
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
}
