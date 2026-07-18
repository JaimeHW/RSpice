//! Typed value editors for registry rows — mono inputs for numbers,
//! strings, and expressions; chips for small enums; checks for booleans.

use egui::Ui;

use crate::quantity::{
    QuantityInputKind, QuantityPresentationPolicy, UiNumberLocale, parse_ui_quantity,
};
use crate::state::property_types::{DisplayMode, PropertyDefinition, PropertyType, PropertyValue};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{chip, mono_input};

use super::state::numeric_source_text;

pub(super) struct ValueEditorOutput {
    pub changed: Option<PropertyValue>,
    pub control_id: Option<egui::Id>,
    pub numeric_text: Option<String>,
    pub parse_error: Option<String>,
}

impl ValueEditorOutput {
    fn readonly() -> Self {
        Self {
            changed: None,
            control_id: None,
            numeric_text: None,
            parse_error: None,
        }
    }

    fn control(id: egui::Id, changed: Option<PropertyValue>) -> Self {
        Self {
            changed,
            control_id: Some(id),
            numeric_text: None,
            parse_error: None,
        }
    }

    fn numeric(
        id: egui::Id,
        changed: Option<PropertyValue>,
        text: String,
        parse_error: Option<String>,
    ) -> Self {
        Self {
            changed,
            control_id: Some(id),
            numeric_text: Some(text),
            parse_error,
        }
    }
}

/// Render the appropriate value editor for a property type.
///
/// Returns Some(new_value) if the value was changed.
pub(super) fn render_value_editor(
    ui: &mut Ui,
    def: &PropertyDefinition,
    current: &PropertyValue,
    numeric_text_draft: Option<&str>,
    width: f32,
    quantity_policy: QuantityPresentationPolicy,
    number_locale: UiNumberLocale,
) -> ValueEditorOutput {
    if def.read_only || def.display_mode == DisplayMode::Readonly {
        let t = Tokens::get(ui.ctx());
        ui.label(
            egui::RichText::new(current.display_string())
                .font(theme::mono(tokens::FS_1, FontWeight::Regular))
                .color(t.color.text_dim),
        );
        return ValueEditorOutput::readonly();
    }

    match def.prop_type {
        PropertyType::Number => render_number_editor(
            ui,
            def,
            current,
            numeric_text_draft,
            width,
            quantity_policy,
            number_locale,
        ),
        PropertyType::String => render_string_editor(ui, current, width),
        PropertyType::Expression => render_expression_editor(
            ui,
            def,
            current,
            numeric_text_draft,
            width,
            quantity_policy,
            number_locale,
        ),
        PropertyType::Enum => render_enum_editor(ui, def, current),
        PropertyType::Boolean => render_boolean_editor(ui, current),
    }
}

/// Number editor with engineering notation and expression escape.
fn render_number_editor(
    ui: &mut Ui,
    def: &PropertyDefinition,
    current: &PropertyValue,
    retained_text: Option<&str>,
    width: f32,
    quantity_policy: QuantityPresentationPolicy,
    number_locale: UiNumberLocale,
) -> ValueEditorOutput {
    let text = retained_text
        .map(str::to_owned)
        .unwrap_or_else(|| numeric_source_text(def, current));

    let mut new_text = text.clone();
    let response = mono_input(ui, &mut new_text, width);

    if response.changed() && new_text != text {
        return match parse_number_source(def, &new_text, quantity_policy, number_locale) {
            Ok(value) => ValueEditorOutput::numeric(response.id, Some(value), new_text, None),
            Err(error) => ValueEditorOutput::numeric(response.id, None, new_text, Some(error)),
        };
    }

    let parse_error = parse_number_source(def, &new_text, quantity_policy, number_locale).err();
    ValueEditorOutput::numeric(response.id, None, new_text, parse_error)
}

pub(super) fn parse_number_source(
    def: &PropertyDefinition,
    text: &str,
    quantity_policy: QuantityPresentationPolicy,
    number_locale: UiNumberLocale,
) -> Result<PropertyValue, String> {
    let quantity_error = match parse_ui_quantity(
        text,
        property_quantity_kind(def),
        quantity_policy,
        number_locale,
    ) {
        Ok(value) => {
            let value = PropertyValue::number(value_for_property_schema(def, value));
            def.validate(&value)?;
            return Ok(value);
        }
        Err(error) => error,
    };

    let trimmed = text.trim();
    if trimmed.is_empty() || has_incomplete_exponent(trimmed) {
        return Err(quantity_error.to_string());
    }
    parse_expression_source(def, trimmed, quantity_policy, number_locale)
}

fn has_incomplete_exponent(source: &str) -> bool {
    let Some(index) = source.rfind(['e', 'E']) else {
        return false;
    };
    let (coefficient, exponent) = source.split_at(index);
    coefficient.parse::<f64>().is_ok() && matches!(exponent, "e" | "E" | "e+" | "E+" | "e-" | "E-")
}

/// String editor.
fn render_string_editor(ui: &mut Ui, current: &PropertyValue, width: f32) -> ValueEditorOutput {
    let text = match current {
        PropertyValue::String(s) => s.clone(),
        _ => current.display_string(),
    };

    let mut new_text = text.clone();
    let response = mono_input(ui, &mut new_text, width);
    if response.changed() && new_text != text {
        return ValueEditorOutput::control(response.id, Some(PropertyValue::String(new_text)));
    }

    ValueEditorOutput::control(response.id, None)
}

/// Expression editor.
fn render_expression_editor(
    ui: &mut Ui,
    def: &PropertyDefinition,
    current: &PropertyValue,
    retained_text: Option<&str>,
    width: f32,
    quantity_policy: QuantityPresentationPolicy,
    number_locale: UiNumberLocale,
) -> ValueEditorOutput {
    let text = retained_text
        .map(str::to_owned)
        .unwrap_or_else(|| numeric_source_text(def, current));

    let mut new_text = text.clone();
    let response = mono_input(ui, &mut new_text, width);

    if response.changed() && new_text != text {
        return match parse_expression_source(def, &new_text, quantity_policy, number_locale) {
            Ok(value) => ValueEditorOutput::numeric(response.id, Some(value), new_text, None),
            Err(error) => ValueEditorOutput::numeric(response.id, None, new_text, Some(error)),
        };
    }

    let parse_error = parse_expression_source(def, &new_text, quantity_policy, number_locale).err();
    ValueEditorOutput::numeric(response.id, None, new_text, parse_error)
}

/// Parse an expression-capable property without allowing numeric constants to
/// evade the property's quantity policy or numeric range. A source that can be
/// evaluated without parameters is a constant quantity; a source with an
/// unresolved parameter remains a symbolic SPICE expression.
pub(super) fn parse_expression_source(
    def: &PropertyDefinition,
    text: &str,
    quantity_policy: QuantityPresentationPolicy,
    number_locale: UiNumberLocale,
) -> Result<PropertyValue, String> {
    let quantity_error = match parse_ui_quantity(
        text,
        property_quantity_kind(def),
        quantity_policy,
        number_locale,
    ) {
        Ok(value) => {
            let value = PropertyValue::number(value_for_property_schema(def, value));
            def.validate(&value)?;
            return Ok(value);
        }
        Err(error) => error,
    };

    let trimmed = text.trim();
    if trimmed.is_empty() {
        return if def.required {
            Err(format!("{} expression is empty", def.display_name))
        } else {
            Ok(PropertyValue::Expression(String::new()))
        };
    }
    let expression = trimmed
        .strip_prefix('{')
        .and_then(|inner| inner.strip_suffix('}'))
        .unwrap_or(trimmed)
        .trim();
    let parsed = rspice_core::netlist::expr::parse_expression(expression)
        .map_err(|error| format!("{} expression: {error}", def.display_name))?;

    // Successful evaluation with an empty parameter context proves this is a
    // constant, not a symbolic parameter expression. It must therefore obey
    // the same explicit-unit policy as any other literal quantity.
    match rspice_core::netlist::expr::evaluate(
        &parsed,
        &rspice_core::netlist::expr::ParamContext::new(),
    ) {
        Ok(constant) => {
            if property_quantity_kind(def) == QuantityInputKind::EngineeringScalar {
                if !constant.is_finite() {
                    return Err(format!("{} must be finite", def.display_name));
                }
                let value = PropertyValue::number(constant);
                def.validate(&value)?;
                return Ok(value);
            }
            return Err(quantity_error.to_string());
        }
        Err(rspice_core::netlist::expr::ExprError::UndefinedParam(_)) => {}
        Err(error) => return Err(format!("{} expression: {error}", def.display_name)),
    }

    Ok(PropertyValue::Expression(expression.to_owned()))
}

fn property_quantity_kind(def: &PropertyDefinition) -> QuantityInputKind {
    match def.unit.as_deref() {
        Some("s") => QuantityInputKind::Time,
        Some("Hz") => QuantityInputKind::Frequency,
        Some("°" | "deg" | "rad") => QuantityInputKind::Angle,
        Some("K" | "°F") => QuantityInputKind::Temperature,
        Some("°C") if def.name.eq_ignore_ascii_case("temp") => QuantityInputKind::Temperature,
        Some("°C") => QuantityInputKind::TemperatureDelta,
        _ => QuantityInputKind::EngineeringScalar,
    }
}

/// Registry numeric values retain the unit declared by their schema. The
/// unit-safe parser returns SI, so only legacy degree/Celsius schemas need a
/// boundary conversion before the property bridge writes them.
fn value_for_property_schema(def: &PropertyDefinition, value_si: f64) -> f64 {
    match def.unit.as_deref() {
        Some("°" | "deg") => value_si.to_degrees(),
        Some("°C") if def.name.eq_ignore_ascii_case("temp") => value_si - 273.15,
        Some("°F") => (value_si - 273.15) * 1.8 + 32.0,
        _ => value_si,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registry_units_select_safe_interactive_quantity_kinds() {
        let time = PropertyDefinition::new("td").with_unit("s");
        let frequency = PropertyDefinition::new("freq").with_unit("Hz");
        let phase = PropertyDefinition::new("phase").with_unit("°");
        let delta_temperature = PropertyDefinition::new("dtemp").with_unit("°C");
        assert_eq!(property_quantity_kind(&time), QuantityInputKind::Time);
        assert_eq!(
            property_quantity_kind(&frequency),
            QuantityInputKind::Frequency
        );
        assert_eq!(property_quantity_kind(&phase), QuantityInputKind::Angle);
        assert_eq!(
            property_quantity_kind(&delta_temperature),
            QuantityInputKind::TemperatureDelta
        );
        assert_eq!(
            value_for_property_schema(&phase, std::f64::consts::FRAC_PI_2),
            90.0
        );
    }

    #[test]
    fn numeric_source_parser_distinguishes_quantities_expressions_and_incomplete_text() {
        let definition = PropertyDefinition::new("gain").with_type(PropertyType::Number);
        let policy = QuantityPresentationPolicy::default();
        let locale = UiNumberLocale::default();

        assert_eq!(
            parse_number_source(&definition, "10k", policy, locale),
            Ok(PropertyValue::number(10_000.0))
        );
        assert_eq!(
            parse_number_source(&definition, "gain", policy, locale),
            Ok(PropertyValue::Expression("gain".to_owned()))
        );
        assert_eq!(
            parse_number_source(&definition, "{gain * 2}", policy, locale),
            Ok(PropertyValue::Expression("gain * 2".to_owned()))
        );
        assert!(parse_number_source(&definition, "1e", policy, locale).is_err());
        assert!(parse_number_source(&definition, "", policy, locale).is_err());
        assert!(parse_number_source(&definition, "{", policy, locale).is_err());
    }

    #[test]
    fn numeric_source_parser_live_validates_ranges_and_constant_errors() {
        let definition = PropertyDefinition::new("gain")
            .with_display_name("Gain")
            .with_type(PropertyType::Number)
            .with_range(0.0, 10.0);
        let policy = QuantityPresentationPolicy::default();
        let locale = UiNumberLocale::default();

        assert!(parse_number_source(&definition, "11", policy, locale).is_err());
        assert!(parse_number_source(&definition, "6 * 2", policy, locale).is_err());
        assert!(parse_number_source(&definition, "1 / 0", policy, locale).is_err());
        assert_eq!(
            parse_number_source(&definition, "gain_parameter", policy, locale),
            Ok(PropertyValue::Expression("gain_parameter".to_owned()))
        );
    }

    #[test]
    fn retained_numeric_sources_are_unit_safe_and_do_not_quantize_schema_values() {
        let policy = QuantityPresentationPolicy::default();
        let locale = UiNumberLocale::default();
        let cases = [
            (
                PropertyDefinition::new("td").with_unit("s"),
                0.001_234_567_890_123_45,
            ),
            (
                PropertyDefinition::new("freq").with_unit("Hz"),
                1_234_567.890_123_45,
            ),
            (
                PropertyDefinition::new("phase").with_unit("°"),
                89.123_456_789_012_3,
            ),
            (
                PropertyDefinition::new("phase_rad").with_unit("rad"),
                1.234_567_890_123_45,
            ),
            (
                PropertyDefinition::new("temp").with_unit("°C"),
                27.123_456_789_012_3,
            ),
            (
                PropertyDefinition::new("temp").with_unit("°F"),
                80.123_456_789_012_3,
            ),
            (
                PropertyDefinition::new("temp_k").with_unit("K"),
                300.123_456_789_012_3,
            ),
        ];

        for (definition, stored) in cases {
            let source = numeric_source_text(&definition, &PropertyValue::number(stored));
            let parsed = parse_number_source(&definition, &source, policy, locale)
                .unwrap_or_else(|error| panic!("{source:?} rejected: {error}"));
            let parsed = parsed.as_number().expect("retained number stays numeric");
            let tolerance = stored.abs().max(1.0) * f64::EPSILON * 4.0;
            assert!(
                (parsed - stored).abs() <= tolerance,
                "{source:?} changed {stored:?} to {parsed:?}"
            );
        }

        let scalar = PropertyDefinition::new("gain").with_unit("V");
        let stored = 1.234_567_890_123_456_7;
        let source = numeric_source_text(&scalar, &PropertyValue::number(stored));
        assert_eq!(source, stored.to_string());
        assert_eq!(
            parse_number_source(&scalar, &source, policy, locale),
            Ok(PropertyValue::number(stored))
        );
    }

    #[test]
    fn expression_parser_enforces_real_registry_phase_units_and_range() {
        let registry = crate::state::PropertyRegistry::new();
        let phase = registry
            .get(crate::state::ComponentType::VoltageSource)
            .and_then(|sheet| sheet.get("acphase"))
            .expect("voltage-source AC phase definition");
        let policy = QuantityPresentationPolicy::default();
        let locale = UiNumberLocale::default();

        assert!(parse_expression_source(phase, "400", policy, locale).is_err());
        assert!(parse_expression_source(phase, "2 * 200", policy, locale).is_err());
        assert!(parse_expression_source(phase, "400 deg", policy, locale).is_err());

        let parsed = parse_expression_source(phase, "90 deg", policy, locale)
            .expect("explicit in-range phase");
        assert_eq!(parsed.as_number(), Some(90.0));
        assert_eq!(
            parse_expression_source(phase, "phase_parameter", policy, locale),
            Ok(PropertyValue::Expression("phase_parameter".to_owned()))
        );
    }

    #[test]
    fn scalar_constant_expressions_are_evaluated_and_range_checked() {
        let definition = PropertyDefinition::new("gain")
            .with_display_name("Gain")
            .with_type(PropertyType::Expression)
            .with_range(0.0, 10.0);
        let policy = QuantityPresentationPolicy::default();
        let locale = UiNumberLocale::default();

        assert_eq!(
            parse_expression_source(&definition, "4 * 2", policy, locale),
            Ok(PropertyValue::number(8.0))
        );
        assert!(parse_expression_source(&definition, "4 * 3", policy, locale).is_err());
    }

    #[test]
    fn expression_evaluation_errors_are_not_misclassified_as_symbolic() {
        let definition = PropertyDefinition::new("gain")
            .with_display_name("Gain")
            .with_type(PropertyType::Expression);
        let policy = QuantityPresentationPolicy::default();
        let locale = UiNumberLocale::default();

        let division = parse_expression_source(&definition, "1 / 0", policy, locale)
            .expect_err("division by zero is invalid");
        assert!(division.contains("Division by zero"));

        let unknown = parse_expression_source(&definition, "unknown_function(1)", policy, locale)
            .expect_err("unknown function is invalid");
        assert!(unknown.contains("Unknown function"));
    }

    #[test]
    fn expression_numeric_fallback_uses_lossless_retained_source() {
        let registry = crate::state::PropertyRegistry::new();
        let phase = registry
            .get(crate::state::ComponentType::VoltageSource)
            .and_then(|sheet| sheet.get("acphase"))
            .expect("voltage-source AC phase definition");
        let stored = 89.123_456_789_012_3;
        let source = numeric_source_text(phase, &PropertyValue::number(stored));

        assert_eq!(source, format!("{} deg", stored));
        assert_eq!(
            parse_expression_source(
                phase,
                &source,
                QuantityPresentationPolicy::default(),
                UiNumberLocale::default(),
            )
            .expect("retained expression number"),
            PropertyValue::number(stored)
        );
    }

    #[test]
    fn optional_source_sentinels_are_blank_valid_expression_drafts() {
        let registry = crate::state::PropertyRegistry::new();
        let policy = QuantityPresentationPolicy::default();
        let locale = UiNumberLocale::default();

        for kind in [
            crate::state::ComponentType::VoltageSource,
            crate::state::ComponentType::VoltageSourceAc,
            crate::state::ComponentType::CurrentSource,
            crate::state::ComponentType::CurrentSourceAc,
        ] {
            let sheet = registry.get(kind).expect("source property sheet");
            for name in ["pacdbm", "rp"] {
                let definition = sheet.get(name).expect("optional source property");
                assert_eq!(definition.prop_type, PropertyType::Expression);
                assert_eq!(
                    definition.default_value,
                    PropertyValue::Expression(String::new())
                );
                let source = numeric_source_text(definition, &definition.default_value);
                assert!(source.is_empty());
                assert_eq!(
                    parse_expression_source(definition, &source, policy, locale),
                    Ok(PropertyValue::Expression(String::new()))
                );
            }
        }
    }
}

/// Enum editor — chips for small sets, dropdown beyond that.
fn render_enum_editor(
    ui: &mut Ui,
    def: &PropertyDefinition,
    current: &PropertyValue,
) -> ValueEditorOutput {
    let (selected, options) = match current {
        PropertyValue::Enum { selected, options } => (selected.clone(), options.clone()),
        _ => (current.display_string(), vec![current.display_string()]),
    };

    let mut new_selected = selected.clone();
    let control_id;

    if options.len() <= 4 {
        let mut first = None;
        ui.horizontal_wrapped(|ui| {
            ui.spacing_mut().item_spacing.x = 4.0;
            for option in &options {
                let response = chip(ui, option, *option == selected);
                first.get_or_insert(response.id);
                if response.clicked() {
                    new_selected = option.clone();
                }
            }
        });
        control_id = first;
    } else {
        let response = egui::ComboBox::from_id_salt(("property-enum", &def.name))
            .selected_text(&new_selected)
            .show_ui(ui, |ui| {
                for option in &options {
                    if ui.selectable_label(*option == selected, option).clicked() {
                        new_selected = option.clone();
                    }
                }
            });
        control_id = Some(response.response.id);
    }

    ValueEditorOutput {
        changed: (new_selected != selected)
            .then(|| PropertyValue::enumeration(new_selected, options)),
        control_id,
        numeric_text: None,
        parse_error: None,
    }
}

/// Boolean editor (checkbox).
fn render_boolean_editor(ui: &mut Ui, current: &PropertyValue) -> ValueEditorOutput {
    let value = match current {
        PropertyValue::Boolean(b) => *b,
        _ => false,
    };

    let mut new_value = value;
    let response = ui.checkbox(&mut new_value, "");
    ValueEditorOutput::control(
        response.id,
        response
            .changed()
            .then_some(PropertyValue::Boolean(new_value)),
    )
}
