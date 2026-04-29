use super::*;

impl PropertyRegistry {
    /// Register Pulse Voltage Source with all SPICE PULSE parameters
    pub(super) fn register_vsource_pulse(&mut self) {
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
    pub(super) fn register_vsource_sin(&mut self) {
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
    pub(super) fn register_vsource_pwl(&mut self) {
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
    pub(super) fn register_vsource_exp(&mut self) {
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
    pub(super) fn register_vsource_sffm(&mut self) {
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
}
