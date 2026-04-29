use super::*;

impl PropertyRegistry {
    /// Register Pulse Current Source
    pub(super) fn register_isource_pulse(&mut self) {
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
    pub(super) fn register_isource_sin(&mut self) {
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
    pub(super) fn register_isource_pwl(&mut self) {
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
    pub(super) fn register_isource_exp(&mut self) {
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
    pub(super) fn register_isource_noise(&mut self) {
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
