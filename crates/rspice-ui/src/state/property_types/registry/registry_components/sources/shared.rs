use super::*;

pub(super) fn source_symbol_property(
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
    /// Add AC small-signal parameters to a source sheet.
    ///
    /// Parameters: ac (magnitude), acphase (phase in degrees)
    /// Used by: All voltage and current sources for AC analysis
    pub(super) fn add_ac_params(sheet: &mut PropertySheet, unit: &str, ac_default: f64) {
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
    pub(super) fn add_advanced_ac_params(sheet: &mut PropertySheet, unit: &str) {
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
                .with_description("Periodic AC power in dBm; leave blank to disable")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::expression(""))
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
    pub(super) fn add_parasitics_params(
        sheet: &mut PropertySheet,
        include_series_resistance: bool,
    ) {
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
                .with_description("Parallel leakage resistance; leave blank for ideal")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::expression(""))
                .with_unit("Ω")
                .with_range(0.0, f64::MAX)
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
    pub(super) fn add_noise_params(sheet: &mut PropertySheet) {
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
}
