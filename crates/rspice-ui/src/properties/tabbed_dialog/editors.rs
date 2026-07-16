//! Typed value editors for registry rows — mono inputs for numbers,
//! strings, and expressions; chips for small enums; checks for booleans.

use egui::Ui;

use crate::quantity::{
    QuantityInputKind, QuantityPresentationPolicy, UiNumberLocale, parse_ui_quantity,
};
use crate::state::property_types::{
    DisplayMode, PropertyDefinition, PropertyType, PropertyValue, format_engineering,
};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{chip, mono_input};

/// Render the appropriate value editor for a property type.
///
/// Returns Some(new_value) if the value was changed.
pub(super) fn render_value_editor(
    ui: &mut Ui,
    def: &PropertyDefinition,
    current: &PropertyValue,
    width: f32,
    quantity_policy: QuantityPresentationPolicy,
    number_locale: UiNumberLocale,
) -> Option<PropertyValue> {
    if def.read_only || def.display_mode == DisplayMode::Readonly {
        let t = Tokens::get(ui.ctx());
        ui.label(
            egui::RichText::new(current.display_string())
                .font(theme::mono(tokens::FS_1, FontWeight::Regular))
                .color(t.color.text_dim),
        );
        return None;
    }

    match def.prop_type {
        PropertyType::Number => {
            render_number_editor(ui, def, current, width, quantity_policy, number_locale)
        }
        PropertyType::String => render_string_editor(ui, current, width),
        PropertyType::Expression => {
            render_expression_editor(ui, def, current, width, quantity_policy, number_locale)
        }
        PropertyType::Enum => render_enum_editor(ui, current),
        PropertyType::Boolean => render_boolean_editor(ui, current),
    }
}

/// Number editor with engineering notation and expression escape.
fn render_number_editor(
    ui: &mut Ui,
    def: &PropertyDefinition,
    current: &PropertyValue,
    width: f32,
    quantity_policy: QuantityPresentationPolicy,
    number_locale: UiNumberLocale,
) -> Option<PropertyValue> {
    let text = match current {
        PropertyValue::Number { value, .. } => format_engineering(*value),
        PropertyValue::Expression(e) => e.clone(),
        _ => current.display_string(),
    };

    let mut new_text = text.clone();
    let response = mono_input(ui, &mut new_text, width);

    if response.changed() && new_text != text {
        if new_text.starts_with('{') || new_text.contains('*') || new_text.contains('/') {
            let expr = new_text.trim_matches(|c| c == '{' || c == '}').to_string();
            return Some(PropertyValue::Expression(expr));
        }

        if let Ok(value) = parse_ui_quantity(
            &new_text,
            property_quantity_kind(def),
            quantity_policy,
            number_locale,
        ) {
            return Some(PropertyValue::number(value_for_property_schema(def, value)));
        }
    }

    None
}

/// String editor.
fn render_string_editor(ui: &mut Ui, current: &PropertyValue, width: f32) -> Option<PropertyValue> {
    let text = match current {
        PropertyValue::String(s) => s.clone(),
        _ => current.display_string(),
    };

    let mut new_text = text.clone();
    if mono_input(ui, &mut new_text, width).changed() && new_text != text {
        return Some(PropertyValue::String(new_text));
    }

    None
}

/// Expression editor.
fn render_expression_editor(
    ui: &mut Ui,
    def: &PropertyDefinition,
    current: &PropertyValue,
    width: f32,
    quantity_policy: QuantityPresentationPolicy,
    number_locale: UiNumberLocale,
) -> Option<PropertyValue> {
    let text = match current {
        PropertyValue::Expression(e) => e.clone(),
        // Round-trip through engineering notation so typing "100k" rebinds
        // as "100k", not "100000" replaced under the cursor mid-edit.
        PropertyValue::Number { value, .. } => format_engineering(*value),
        _ => current.display_string(),
    };

    let mut new_text = text.clone();
    let response = mono_input(ui, &mut new_text, width);

    if response.changed() && new_text != text {
        // Engineering notation ("100k", "10n") resolves to a number; only
        // genuinely symbolic input stays an expression.
        if let Ok(value) = parse_ui_quantity(
            &new_text,
            property_quantity_kind(def),
            quantity_policy,
            number_locale,
        ) {
            return Some(PropertyValue::number(value_for_property_schema(def, value)));
        }
        return Some(PropertyValue::Expression(new_text));
    }

    None
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
}

/// Enum editor — chips for small sets, dropdown beyond that.
fn render_enum_editor(ui: &mut Ui, current: &PropertyValue) -> Option<PropertyValue> {
    let (selected, options) = match current {
        PropertyValue::Enum { selected, options } => (selected.clone(), options.clone()),
        _ => (current.display_string(), vec![current.display_string()]),
    };

    let mut new_selected = selected.clone();

    if options.len() <= 4 {
        ui.spacing_mut().item_spacing.x = 4.0;
        for option in &options {
            if chip(ui, option, *option == selected).clicked() {
                new_selected = option.clone();
            }
        }
    } else {
        egui::ComboBox::from_id_salt(&selected)
            .selected_text(&new_selected)
            .show_ui(ui, |ui| {
                for option in &options {
                    if ui.selectable_label(*option == selected, option).clicked() {
                        new_selected = option.clone();
                    }
                }
            });
    }

    if new_selected != selected {
        return Some(PropertyValue::enumeration(new_selected, options));
    }

    None
}

/// Boolean editor (checkbox).
fn render_boolean_editor(ui: &mut Ui, current: &PropertyValue) -> Option<PropertyValue> {
    let value = match current {
        PropertyValue::Boolean(b) => *b,
        _ => false,
    };

    let mut new_value = value;
    if ui.checkbox(&mut new_value, "").changed() {
        return Some(PropertyValue::Boolean(new_value));
    }

    None
}
