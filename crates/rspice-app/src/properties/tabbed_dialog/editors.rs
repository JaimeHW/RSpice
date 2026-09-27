//! Typed value editors for registry rows — mono inputs for numbers,
//! strings, and expressions; chips for small enums; checks for booleans.
//!
//! Every editor fills the width of its grid cell and stands exactly one
//! control track tall, so a boolean or a small enum carries the same visual
//! weight as the mono inputs beside it.

use egui::{Sense, Stroke, Ui, pos2, vec2};

use super::super::value::{parse_expression_source, parse_number_source, property_quantity_kind};
use super::state::numeric_source_text;
use crate::quantity::{
    QuantityInputKind, QuantityPresentationPolicy, UiNumberLocale, format_engineering_value,
};
use crate::state::property_types::{DisplayMode, PropertyDefinition, PropertyType, PropertyValue};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{chip, mono_input, select_mono_with_response};

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
        // Readonly values still occupy a full control track: dropping to a
        // bare label would pull every field below them out of alignment.
        let (rect, _) = ui.allocate_exact_size(vec2(width, t.metrics.ctl_h), Sense::hover());
        if ui.is_rect_visible(rect) {
            ui.painter().text(
                pos2(rect.left() + 8.0, rect.center().y),
                egui::Align2::LEFT_CENTER,
                current.display_string(),
                theme::mono(tokens::FS_1, FontWeight::Regular),
                t.color.text_dim,
            );
        }
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
        PropertyType::String => render_string_editor(ui, def, current, width),
        PropertyType::Expression => render_expression_editor(
            ui,
            def,
            current,
            numeric_text_draft,
            width,
            quantity_policy,
            number_locale,
        ),
        PropertyType::Enum => render_enum_editor(ui, def, current, width),
        PropertyType::Boolean => render_boolean_editor(ui, def, current, width),
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
        .unwrap_or_else(|| editor_source_text(def, current, quantity_policy, number_locale));

    let mut new_text = text.clone();
    let response = mono_input(ui, &def.display_name, &mut new_text, width);

    if response.changed() && new_text != text {
        return match parse_number_source(def, &new_text, quantity_policy, number_locale) {
            Ok(value) => ValueEditorOutput::numeric(response.id, Some(value), new_text, None),
            Err(error) => ValueEditorOutput::numeric(response.id, None, new_text, Some(error)),
        };
    }

    let parse_error = parse_number_source(def, &new_text, quantity_policy, number_locale).err();
    ValueEditorOutput::numeric(response.id, None, new_text, parse_error)
}

/// String editor.
fn render_string_editor(
    ui: &mut Ui,
    def: &PropertyDefinition,
    current: &PropertyValue,
    width: f32,
) -> ValueEditorOutput {
    let text = match current {
        PropertyValue::String(s) => s.clone(),
        _ => current.display_string(),
    };

    let mut new_text = text.clone();
    let response = mono_input(ui, &def.display_name, &mut new_text, width);
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
        .unwrap_or_else(|| editor_source_text(def, current, quantity_policy, number_locale));

    let mut new_text = text.clone();
    let response = mono_input(ui, &def.display_name, &mut new_text, width);

    if response.changed() && new_text != text {
        return match parse_expression_source(def, &new_text, quantity_policy, number_locale) {
            Ok(value) => ValueEditorOutput::numeric(response.id, Some(value), new_text, None),
            Err(error) => ValueEditorOutput::numeric(response.id, None, new_text, Some(error)),
        };
    }

    let parse_error = parse_expression_source(def, &new_text, quantity_policy, number_locale).err();
    ValueEditorOutput::numeric(response.id, None, new_text, parse_error)
}

/// Editor text for a stored numeric value.
///
/// An engineer reads `1n s`, not `0.000000001 s`, so the editor offers the
/// engineering form — but this text is also what a commit re-parses, and the
/// engineering formatter rounds. The candidate is therefore run back through
/// the real parser and used only when it reproduces the stored number bit for
/// bit; anything lossy keeps the exact decimal. Temperatures and angles are
/// left alone because SI prefixes are not idiomatic for them ("1m °C").
pub(super) fn editor_source_text(
    def: &PropertyDefinition,
    value: &PropertyValue,
    quantity_policy: QuantityPresentationPolicy,
    number_locale: UiNumberLocale,
) -> String {
    let exact = numeric_source_text(def, value);
    let PropertyValue::Number { value: number, .. } = value else {
        return exact;
    };
    if !number.is_finite()
        || matches!(
            property_quantity_kind(def),
            QuantityInputKind::Temperature
                | QuantityInputKind::TemperatureDelta
                | QuantityInputKind::Angle
        )
    {
        return exact;
    }

    // The engineering prefix has to abut its unit — the parser reads `1ns`,
    // never `1n s`, because it strips the unit and then expects a bare prefix.
    let magnitude = format_engineering_value(*number);
    let candidate = match def.unit.as_deref() {
        Some("s") => format!("{magnitude}s"),
        Some("Hz") => format!("{magnitude}Hz"),
        _ => magnitude,
    };
    if candidate == exact {
        return exact;
    }
    match parse_number_source(def, &candidate, quantity_policy, number_locale) {
        Ok(PropertyValue::Number { value: parsed, .. }) if parsed.to_bits() == number.to_bits() => {
            candidate
        }
        _ => exact,
    }
}

/// Enum editor — chips while the whole option set fits the cell, the
/// design-system select once it does not.
fn render_enum_editor(
    ui: &mut Ui,
    def: &PropertyDefinition,
    current: &PropertyValue,
    width: f32,
) -> ValueEditorOutput {
    let (selected, options) = match current {
        PropertyValue::Enum { selected, options } => (selected.clone(), options.clone()),
        _ => (current.display_string(), vec![current.display_string()]),
    };

    let mut new_selected = selected.clone();
    let control_id;

    if chips_fit(ui, &options, width) {
        let t = Tokens::get(ui.ctx());
        let mut first = None;
        ui.allocate_ui_with_layout(
            vec2(width, t.metrics.ctl_h),
            egui::Layout::left_to_right(egui::Align::Center),
            |ui| {
                ui.spacing_mut().item_spacing.x = CHIP_GAP;
                for option in &options {
                    let response = chip(ui, option, *option == selected);
                    first.get_or_insert(response.id);
                    if response.clicked() {
                        new_selected = option.clone();
                    }
                }
            },
        );
        control_id = first;
    } else {
        let output = select_mono_with_response(
            ui,
            &format!("property-enum-{}", def.name),
            &def.display_name,
            &selected,
            &options,
            width,
        );
        if let Some(index) = output.picked
            && let Some(option) = options.get(index)
        {
            new_selected = option.clone();
        }
        control_id = Some(output.response.id);
    }

    ValueEditorOutput {
        changed: (new_selected != selected)
            .then(|| PropertyValue::enumeration(new_selected, options)),
        control_id,
        numeric_text: None,
        parse_error: None,
    }
}

/// Gap between adjacent option chips.
const CHIP_GAP: f32 = 4.0;

/// Whether the complete option set can be presented as chips inside `width`.
/// Chips must never wrap: a second chip line would break the fixed control
/// track every neighboring field is aligned to.
fn chips_fit(ui: &Ui, options: &[String], width: f32) -> bool {
    if options.len() > 4 {
        return false;
    }
    let font = theme::mono(tokens::FS_0, FontWeight::Regular);
    let color = Tokens::get(ui.ctx()).color.text;
    let total = options
        .iter()
        .map(|option| {
            ui.fonts_mut(|fonts| fonts.layout_no_wrap(option.clone(), font.clone(), color))
                .size()
                .x
                + CHIP_WIDTH_PADDING
        })
        .sum::<f32>()
        + CHIP_GAP * options.len().saturating_sub(1) as f32;
    total <= width
}

/// Horizontal padding `chip` adds around its label.
const CHIP_WIDTH_PADDING: f32 = 18.0;

/// Boolean editor — a full-cell check control styled as the inset well its
/// neighboring value inputs use, so a flag never reads as a stray square in
/// an otherwise empty half-row.
fn render_boolean_editor(
    ui: &mut Ui,
    def: &PropertyDefinition,
    current: &PropertyValue,
    width: f32,
) -> ValueEditorOutput {
    let value = matches!(current, PropertyValue::Boolean(true));
    let t = Tokens::get(ui.ctx());
    let c = t.color;

    let (rect, response) = ui.allocate_exact_size(vec2(width, t.metrics.ctl_h), Sense::click());
    let mut new_value = value;
    if response.clicked() {
        new_value = !value;
    }
    if response.has_focus()
        && ui.input_mut(|input| {
            input.consume_key(egui::Modifiers::NONE, egui::Key::Space)
                || input.consume_key(egui::Modifiers::NONE, egui::Key::Enter)
        })
    {
        new_value = !value;
    }
    response.widget_info(|| {
        egui::WidgetInfo::selected(
            egui::WidgetType::Checkbox,
            ui.is_enabled(),
            new_value,
            &def.display_name,
        )
    });

    if ui.is_rect_visible(rect) {
        let hover = ui.ctx().animate_bool_with_time(
            response.id,
            response.hovered(),
            ui.style().animation_time,
        );
        let painter = ui.painter();
        painter.rect(
            rect,
            t.radius,
            c.bg_inset,
            Stroke::new(1.0, theme::mix(c.border, c.border_strong, hover)),
            egui::StrokeKind::Inside,
        );

        let box_size = 13.0;
        let box_rect = egui::Rect::from_center_size(
            pos2(rect.left() + 8.0 + box_size * 0.5, rect.center().y),
            egui::Vec2::splat(box_size),
        );
        painter.rect(
            box_rect,
            2.0,
            if new_value {
                c.accent
            } else {
                egui::Color32::TRANSPARENT
            },
            Stroke::new(
                1.0,
                if new_value {
                    c.accent
                } else {
                    theme::mix(c.border_strong, c.text_dim, hover)
                },
            ),
            egui::StrokeKind::Inside,
        );
        if new_value {
            let center = box_rect.center();
            painter.add(egui::Shape::line(
                vec![
                    pos2(center.x - 3.2, center.y - 0.2),
                    pos2(center.x - 0.8, center.y + 2.4),
                    pos2(center.x + 3.4, center.y - 2.6),
                ],
                Stroke::new(1.6, c.accent_ink),
            ));
        }
        painter.text(
            pos2(box_rect.right() + 8.0, rect.center().y),
            egui::Align2::LEFT_CENTER,
            PropertyValue::Boolean(new_value).display_string(),
            theme::mono(tokens::FS_1, FontWeight::Regular),
            c.text,
        );
    }
    theme::paint_focus_ring_outset(ui, &response, rect);

    ValueEditorOutput::control(
        response.id,
        (new_value != value).then_some(PropertyValue::Boolean(new_value)),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn expression_numeric_fallback_uses_lossless_retained_source() {
        let registry = crate::properties::PropertyEditorSchema::new();
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
        let registry = crate::properties::PropertyEditorSchema::new();
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

    #[test]
    fn engineering_display_is_offered_only_when_it_round_trips_exactly() {
        let policy = QuantityPresentationPolicy::default();
        let locale = UiNumberLocale::default();

        // A time whose engineering form the parser reads back bit for bit.
        let rise = PropertyDefinition::new("tr")
            .with_type(PropertyType::Number)
            .with_unit("s");
        assert_eq!(
            editor_source_text(&rise, &PropertyValue::number(1e-9), policy, locale),
            "1ns"
        );
        // ... and one it cannot: the engineering formatter stops at three
        // decimals, so the exact decimal has to survive.
        let awkward = PropertyValue::number(1.234_567_89e-9);
        assert_eq!(
            editor_source_text(&rise, &awkward, policy, locale),
            numeric_source_text(&rise, &awkward)
        );

        // Temperatures keep their plain form: "1m °C" is not how anyone
        // writes a temperature.
        let dtemp = PropertyDefinition::new("dtemp")
            .with_type(PropertyType::Number)
            .with_unit("°C");
        assert_eq!(
            editor_source_text(&dtemp, &PropertyValue::number(0.001), policy, locale),
            "0.001 °C"
        );
    }

    #[test]
    fn every_offered_engineering_form_parses_back_to_the_stored_value() {
        let policy = QuantityPresentationPolicy::default();
        let locale = UiNumberLocale::default();
        let cases = [
            (PropertyDefinition::new("tr").with_unit("s"), 1e-9),
            (PropertyDefinition::new("per").with_unit("s"), 2e-6),
            (PropertyDefinition::new("f").with_unit("Hz"), 1.5e9),
            (PropertyDefinition::new("r"), 1e3),
            (PropertyDefinition::new("c").with_unit("F"), 1e-12),
            (PropertyDefinition::new("td").with_unit("s"), 0.0),
        ];
        for (definition, stored) in cases {
            let definition = definition.with_type(PropertyType::Number);
            let value = PropertyValue::number(stored);
            let text = editor_source_text(&definition, &value, policy, locale);
            let Ok(PropertyValue::Number { value: parsed, .. }) =
                parse_number_source(&definition, &text, policy, locale)
            else {
                panic!("offered text {text:?} for {stored} does not parse as a number");
            };
            assert_eq!(
                parsed.to_bits(),
                stored.to_bits(),
                "offered text {text:?} does not restore {stored}"
            );
        }
    }
}
