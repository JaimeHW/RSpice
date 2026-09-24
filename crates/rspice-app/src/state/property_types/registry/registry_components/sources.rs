//! Property sheets for independent and controlled sources, and for ground.

use super::*;

mod current_bias;
mod current_transient;
mod ground;
mod loop_probe;
mod rf_port;
mod shared;
mod shared_waveforms;
mod voltage_bias;
mod voltage_transient;

use shared_waveforms::Driven;

impl PropertyRegistry {
    pub(in super::super) fn register_sources(&mut self) {
        self.register_vsource_dc();
        self.register_vsource_ac();
        self.register_vsource_pulse();
        self.register_vsource_sin();
        self.register_vsource_pwl();
        self.register_vsource_exp();

        self.register_isource_dc();
        self.register_isource_ac();
        self.register_isource_pulse();
        self.register_isource_sin();
        self.register_isource_pwl();
        self.register_isource_exp();

        // Families whose two sheets are stamped from one builder, so the
        // voltage and current forms cannot drift apart.
        for (driven, sffm, am, pat, noise, random, pwl_file) in [
            (
                Driven::Voltage,
                ComponentType::VoltageSourceSffm,
                ComponentType::VoltageSourceAm,
                ComponentType::VoltageSourcePat,
                ComponentType::VoltageSourceNoise,
                ComponentType::VoltageSourceRandom,
                ComponentType::VoltageSourcePwlFile,
            ),
            (
                Driven::Current,
                ComponentType::CurrentSourceSffm,
                ComponentType::CurrentSourceAm,
                ComponentType::CurrentSourcePat,
                ComponentType::CurrentSourceNoise,
                ComponentType::CurrentSourceRandom,
                ComponentType::CurrentSourcePwlFile,
            ),
        ] {
            self.sheets.insert(sffm, Self::create_sffm_sheet(driven));
            self.sheets.insert(am, Self::create_am_sheet(driven));
            self.sheets.insert(pat, Self::create_pat_sheet(driven));
            self.sheets
                .insert(noise, Self::create_trnoise_sheet(driven));
            self.sheets
                .insert(random, Self::create_trrandom_sheet(driven));
            self.sheets
                .insert(pwl_file, Self::create_pwl_file_sheet(driven));
        }

        self.register_rf_port();
        self.register_loop_probe();
        self.register_ground();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::state::property_types::TRRANDOM_DISTRIBUTIONS;

    /// The TRRANDOM sheet's defaults are the parser's own.
    ///
    /// TYPE and TS have no parser default at all — the card is refused without
    /// them — so the sheet supplies the two a placed source cannot go without,
    /// and TD, PARAM1 and PARAM2 are exactly what `parse_trrandom_spec` fills in
    /// for an omitted field (`netlist/parser/source_specs.rs:683-691`).
    #[test]
    fn both_trrandom_sheets_carry_the_parsers_own_defaults() {
        let registry = PropertyRegistry::new();
        for kind in [
            ComponentType::VoltageSourceRandom,
            ComponentType::CurrentSourceRandom,
        ] {
            let sheet = registry.get(kind).unwrap_or_else(|| {
                panic!("{kind:?} has no property sheet");
            });
            let field = |name: &str| {
                sheet
                    .get(name)
                    .unwrap_or_else(|| panic!("{kind:?} has no '{name}' field"))
            };

            assert_eq!(
                field("type").default_value,
                PropertyValue::enumeration(
                    "uniform",
                    TRRANDOM_DISTRIBUTIONS
                        .iter()
                        .map(|name| (*name).to_owned())
                        .collect()
                )
            );
            assert_eq!(field("ts").default_value, PropertyValue::expression("1u"));
            assert_eq!(field("td").default_value, PropertyValue::number(0.0));
            assert_eq!(field("param1").default_value, PropertyValue::number(1.0));
            assert_eq!(field("param2").default_value, PropertyValue::number(0.0));

            assert!(field("type").required, "TYPE is not optional on the card");
            assert!(field("ts").required, "TS is not optional on the card");
            // PARAM1 is an amplitude under three distributions and a mean count
            // under the fourth, so it carries no unit; PARAM2 is always the
            // driven quantity.
            assert_eq!(field("param1").unit, None);
            assert_eq!(field("ts").unit.as_deref(), Some("s"));
        }
    }

    /// One builder, two sheets: across the waveform block the quantity may
    /// change the unit and nothing else. TRRANDOM keeps ngspice's shared
    /// spellings on both kinds, so even the first positional is the same field.
    #[test]
    fn the_two_trrandom_sheets_differ_only_where_the_quantity_does() {
        let registry = PropertyRegistry::new();
        let voltage = registry
            .get(ComponentType::VoltageSourceRandom)
            .expect("the voltage sheet");
        let current = registry
            .get(ComponentType::CurrentSourceRandom)
            .expect("the current sheet");

        for name in ["type", "ts", "td", "param1", "param2"] {
            let authored = voltage.get(name).expect("a voltage field");
            let twin = current.get(name).expect("a current field");
            assert_eq!(twin.display_name, authored.display_name, "{name}");
            assert_eq!(twin.description, authored.description, "{name}");
            assert_eq!(twin.prop_type, authored.prop_type, "{name}");
            assert_eq!(twin.default_value, authored.default_value, "{name}");
            assert_eq!(twin.display_order, authored.display_order, "{name}");
            assert_eq!(twin.category, authored.category, "{name}");
        }
        assert_eq!(
            voltage.get("param2").expect("param2").unit.as_deref(),
            Some("V")
        );
        assert_eq!(
            current.get("param2").expect("param2").unit.as_deref(),
            Some("A")
        );
    }
}
