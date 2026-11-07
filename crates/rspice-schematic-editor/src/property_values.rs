//! Typed value editors for registry rows — mono inputs for numbers,
//! strings, and expressions; chips for small enums; checks for booleans.
//!
//! Every editor takes the width of its grid cell and stands exactly one
//! control track tall, so the fields after a boolean or a small enum stay on
//! the rows of the mono inputs beside it.

use egui::{Sense, Stroke, Ui, pos2, vec2};

use rspice_design::properties::value::{
    parse_expression_source, parse_number_source, property_quantity_kind,
};
use rspice_ui_kit::theme::{self, FontWeight};
use rspice_ui_kit::tokens::{self, Tokens};
use rspice_ui_kit::widgets::{chip, mono_input, mono_input_with_unit, select_mono_with_response};
use rspice_vocabulary::property::{DisplayMode, PropertyDefinition, PropertyType, PropertyValue};
use rspice_vocabulary::quantity::{
    ESeries, QuantityInputKind, QuantityPresentationPolicy, UiNumberLocale,
    format_engineering_value,
};

/// Step a focused value field's text to the next standard value of `series`:
/// Up and Down step up and down, and so does the wheel while the pointer is
/// over the field. Returns whether the text moved.
///
/// The wheel over a focused field belongs to the field, so the panel behind
/// it does not scroll as the value steps. A text with no neighbour in the
/// series, such as an expression or a parameter, is left as it is.
pub fn step_standard_value(
    ui: &mut Ui,
    field: egui::Id,
    series: ESeries,
    text: &mut String,
) -> bool {
    if !ui.memory(|memory| memory.has_focus(field)) {
        return false;
    }
    // The vertical arrows are the field's from the frame it takes the
    // keyboard, or egui's focus navigation would move to the next field.
    ui.memory_mut(|memory| {
        memory.set_focus_lock_filter(
            field,
            egui::EventFilter {
                tab: false,
                horizontal_arrows: true,
                vertical_arrows: true,
                escape: false,
            },
        );
    });
    let over_field = ui
        .ctx()
        .read_response(field)
        .zip(ui.ctx().pointer_hover_pos())
        .is_some_and(|(response, pointer)| response.rect.contains(pointer));
    let (keys, notches) = ui.input_mut(|input| {
        let keys = i32::from(input.consume_key(egui::Modifiers::NONE, egui::Key::ArrowUp))
            - i32::from(input.consume_key(egui::Modifiers::NONE, egui::Key::ArrowDown));
        let mut notches = 0.0;
        if over_field {
            // Line and page notches step; a trackpad's continuous points are
            // too fine to count and are only kept from the panel.
            for event in &input.events {
                if let egui::Event::MouseWheel {
                    unit: egui::MouseWheelUnit::Line | egui::MouseWheelUnit::Page,
                    delta,
                    ..
                } = event
                {
                    notches += delta.y;
                }
            }
            input.smooth_scroll_delta = egui::Vec2::ZERO;
        }
        (keys, notches)
    });
    let up = match (keys, notches) {
        (0, notches) if notches > 0.0 => true,
        (0, notches) if notches < 0.0 => false,
        (0, _) => return false,
        (keys, _) => keys > 0,
    };
    let Some(next) = series.step_text(text.trim(), up) else {
        return false;
    };
    *text = next;
    // The caret follows the value's end, as it does after typing.
    if let Some(mut state) = egui::TextEdit::load_state(ui.ctx(), field) {
        let end = egui::text::CCursor::new(text.chars().count());
        state
            .cursor
            .set_char_range(Some(egui::text::CCursorRange::one(end)));
        state.store(ui.ctx(), field);
    }
    true
}

/// Whether the editor's own text carries this property's unit.
///
/// The unit-safe parser refuses a bare number for these kinds, so the unit is
/// part of the value's syntax rather than metadata about it. The well reads
/// the same predicate and states no unit beside such a value, so the unit is
/// presented exactly once.
pub fn unit_is_part_of_value_text(definition: &PropertyDefinition) -> bool {
    definition.prop_type == PropertyType::Number
        && matches!(
            definition.unit.as_deref(),
            Some("s" | "Hz" | "°" | "deg" | "rad" | "K" | "°C" | "°F")
        )
}

/// The unit a field states inside its well's trailing edge: the schema's,
/// unless the value's own text already carries it.
pub fn unit_beside_value(definition: &PropertyDefinition) -> Option<&str> {
    definition
        .unit
        .as_deref()
        .filter(|_| !unit_is_part_of_value_text(definition))
}

/// Append the unit the value's own syntax must carry, if any.
///
/// One owner for the suffix so the exact form and the engineering form the
/// editor offers can never disagree about it.
fn unit_suffixed(definition: &PropertyDefinition, magnitude: &str) -> String {
    match definition.unit.as_deref() {
        Some("s") => format!("{magnitude} s"),
        Some("Hz") => format!("{magnitude} Hz"),
        Some("°" | "deg") => format!("{magnitude} deg"),
        Some("rad") => format!("{magnitude} rad"),
        Some("K") => format!("{magnitude} K"),
        Some("°C") => format!("{magnitude} °C"),
        Some("°F") => format!("{magnitude} °F"),
        _ => magnitude.to_owned(),
    }
}

/// Build the retained editor source from the schema value without applying a
/// presentation-only precision limit. Quantity-aware fields receive an
/// explicit base unit so merely opening their tab cannot turn a valid stored
/// value into an invalid draft under the strict input policy.
pub fn numeric_source_text(definition: &PropertyDefinition, value: &PropertyValue) -> String {
    match value {
        PropertyValue::Number { value, .. } => unit_suffixed(definition, &value.to_string()),
        PropertyValue::Expression(expression) => expression.clone(),
        _ => value.display_string(),
    }
}

/// Typed edits, retained numeric text and focus identity from one field.
pub struct ValueEditorOutput {
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
/// Returns typed edits, retained numeric text, parse errors and control identity.
pub fn render_value_editor(
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
    let response = mono_input_with_unit(
        ui,
        &def.display_name,
        &mut new_text,
        width,
        unit_beside_value(def),
    );

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
    let response = mono_input_with_unit(
        ui,
        &def.display_name,
        &mut new_text,
        width,
        unit_beside_value(def),
    );

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
/// The editor offers a compact form such as `1ns` for `0.000000001 s`. The
/// text is also what a commit re-parses, and the
/// engineering formatter rounds. The candidate is therefore run back through
/// the real parser and used only when it reproduces the stored number bit for
/// bit; anything lossy keeps the exact decimal. Temperatures and angles are
/// left alone because SI prefixes are not idiomatic for them ("1m °C").
pub fn editor_source_text(
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

/// Side of a check box, as the kit's forms draw theirs.
const CHECK_SIDE: f32 = 14.0;

/// Boolean editor: a check box at the start of the control track, as the
/// kit's forms draw theirs, under the caption that names it.
///
/// It used to be drawn inside an inset well the width of the cell, with the
/// words `yes` or `no` after the box, which made a flag read as a text field
/// holding a check box. The box states the value; the caption names it. The
/// press target is the control track's height square, so the box is as easy
/// to reach as the wells beside it.
fn render_boolean_editor(
    ui: &mut Ui,
    def: &PropertyDefinition,
    current: &PropertyValue,
    width: f32,
) -> ValueEditorOutput {
    let value = matches!(current, PropertyValue::Boolean(true));
    let t = Tokens::get(ui.ctx());
    let c = t.color;

    let (track, _) = ui.allocate_exact_size(vec2(width, t.metrics.ctl_h), Sense::hover());
    let target =
        egui::Rect::from_min_size(track.min, vec2(t.metrics.ctl_h.min(width), t.metrics.ctl_h));
    let response = ui.interact(
        target,
        ui.id().with(("property-check", &def.name)),
        Sense::click(),
    );
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

    let mark = egui::Rect::from_center_size(
        pos2(track.left() + CHECK_SIDE * 0.5, track.center().y),
        egui::Vec2::splat(CHECK_SIDE),
    );
    if ui.is_rect_visible(track) {
        let border = if ui.is_enabled() && response.hovered() {
            c.border_strong
        } else {
            c.border
        };
        let painter = ui.painter();
        painter.rect(
            mark,
            3.0,
            c.bg_inset,
            Stroke::new(1.0, border),
            egui::StrokeKind::Inside,
        );
        if new_value {
            // A tick on the 14-unit box: down to the base, then up and out.
            let at = |x: f32, y: f32| pos2(mark.left() + x, mark.top() + y);
            painter.add(egui::Shape::line(
                vec![at(3.5, 7.2), at(6.0, 9.8), at(10.6, 4.4)],
                Stroke::new(1.6, c.text),
            ));
        }
    }
    theme::paint_focus_ring_outset(ui, &response, mark);

    ValueEditorOutput::control(
        response.id,
        (new_value != value).then_some(PropertyValue::Boolean(new_value)),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use rspice_design::properties::PropertyCatalog;
    use rspice_design::schematic::component_type::ComponentType;

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
        let registry = PropertyCatalog::new();
        let phase = registry
            .get(ComponentType::VoltageSource)
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
        let registry = PropertyCatalog::new();
        let policy = QuantityPresentationPolicy::default();
        let locale = UiNumberLocale::default();

        for kind in [
            ComponentType::VoltageSource,
            ComponentType::VoltageSourceAc,
            ComponentType::CurrentSource,
            ComponentType::CurrentSourceAc,
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
