//! Property sheets for time-varying current sources: pulse, sine, PWL, and
//! exponential. The families that exist identically for both quantities — SFFM,
//! AM, PAT, TRNOISE — are built in `shared_waveforms`.

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
        sheet.add(
            PropertyDefinition::new("np")
                .with_display_name("Pulse Count (NP)")
                .with_description(
                    "How many periods the train runs for; 0 runs for the whole analysis",
                )
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(0.0))
                .with_range(0.0, 1e9)
                .with_order(17)
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
}
