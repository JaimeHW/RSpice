//! Typed value editors for registry rows — mono inputs for numbers,
//! strings, and expressions; chips for small enums; checks for booleans.

use egui::Ui;

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
        PropertyType::Number => render_number_editor(ui, current, width),
        PropertyType::String => render_string_editor(ui, current, width),
        PropertyType::Expression => render_expression_editor(ui, current, width),
        PropertyType::Enum => render_enum_editor(ui, current),
        PropertyType::Boolean => render_boolean_editor(ui, current),
    }
}

/// Number editor with engineering notation and expression escape.
fn render_number_editor(ui: &mut Ui, current: &PropertyValue, width: f32) -> Option<PropertyValue> {
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

        if let Ok(value) = crate::properties::parse_engineering_value(&new_text) {
            return Some(PropertyValue::number(value));
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
    current: &PropertyValue,
    width: f32,
) -> Option<PropertyValue> {
    let text = match current {
        PropertyValue::Expression(e) => e.clone(),
        PropertyValue::Number { value, .. } => value.to_string(),
        _ => current.display_string(),
    };

    let mut new_text = text.clone();
    let response = mono_input(ui, &mut new_text, width);

    if response.changed() && new_text != text {
        if let Ok(value) = new_text.parse::<f64>() {
            return Some(PropertyValue::number(value));
        }
        return Some(PropertyValue::Expression(new_text));
    }

    None
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
