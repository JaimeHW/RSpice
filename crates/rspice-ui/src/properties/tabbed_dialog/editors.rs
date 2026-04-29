use crate::state::property_types::{
    DisplayMode, PropertyDefinition, PropertyType, PropertyValue, format_engineering,
};
use egui::Ui;

/// Render the appropriate value editor for a property type.
///
/// Returns Some(new_value) if the value was changed.
pub(super) fn render_value_editor(
    ui: &mut Ui,
    def: &PropertyDefinition,
    current: &PropertyValue,
) -> Option<PropertyValue> {
    if def.read_only || def.display_mode == DisplayMode::Readonly {
        ui.label(current.display_string());
        return None;
    }

    match def.prop_type {
        PropertyType::Number => render_number_editor(ui, def, current),
        PropertyType::String => render_string_editor(ui, current),
        PropertyType::Expression => render_expression_editor(ui, current),
        PropertyType::Enum => render_enum_editor(ui, current),
        PropertyType::Boolean => render_boolean_editor(ui, current),
    }
}

/// Number editor with engineering notation support
fn render_number_editor(
    ui: &mut Ui,
    _def: &PropertyDefinition,
    current: &PropertyValue,
) -> Option<PropertyValue> {
    let text = match current {
        PropertyValue::Number { value, .. } => format_engineering(*value),
        PropertyValue::Expression(e) => e.clone(),
        _ => current.display_string(),
    };

    let mut new_text = text.clone();
    let response = ui.text_edit_singleline(&mut new_text);

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

/// String editor
fn render_string_editor(ui: &mut Ui, current: &PropertyValue) -> Option<PropertyValue> {
    let text = match current {
        PropertyValue::String(s) => s.clone(),
        _ => current.display_string(),
    };

    let mut new_text = text.clone();
    if ui.text_edit_singleline(&mut new_text).changed() && new_text != text {
        return Some(PropertyValue::String(new_text));
    }

    None
}

/// Expression editor
fn render_expression_editor(ui: &mut Ui, current: &PropertyValue) -> Option<PropertyValue> {
    let text = match current {
        PropertyValue::Expression(e) => e.clone(),
        PropertyValue::Number { value, .. } => value.to_string(),
        _ => current.display_string(),
    };

    let mut new_text = text.clone();
    let response = ui.text_edit_singleline(&mut new_text);

    if response.changed() && new_text != text {
        if let Ok(value) = new_text.parse::<f64>() {
            return Some(PropertyValue::number(value));
        }
        return Some(PropertyValue::Expression(new_text));
    }

    None
}

/// Enum editor (dropdown)
fn render_enum_editor(ui: &mut Ui, current: &PropertyValue) -> Option<PropertyValue> {
    let (selected, options) = match current {
        PropertyValue::Enum { selected, options } => (selected.clone(), options.clone()),
        _ => (current.display_string(), vec![current.display_string()]),
    };

    let mut new_selected = selected.clone();

    egui::ComboBox::from_id_salt(&selected)
        .selected_text(&new_selected)
        .show_ui(ui, |ui| {
            for option in &options {
                if ui.selectable_label(*option == selected, option).clicked() {
                    new_selected = option.clone();
                }
            }
        });

    if new_selected != selected {
        return Some(PropertyValue::enumeration(new_selected, options));
    }

    None
}

/// Boolean editor (checkbox)
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
