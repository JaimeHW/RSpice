use super::*;

impl PropertyRegistry {
    /// Register DC Current Source with Spectre-parity parameters.
    ///
    /// Mirror of vsource DC parameters but with current units (A instead of V).
    pub(super) fn register_isource_dc(&mut self) {
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
    pub(super) fn register_isource_ac(&mut self) {
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
}
