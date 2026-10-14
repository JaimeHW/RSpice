//! Property rows that say more about a value than the value itself: its unit
//! beside it, and what the parameter is when its label is hovered.
//!
//! The unit is painted in the sans face at the field's trailing edge, inside
//! its frame, so every field in a list keeps the same right edge whether or
//! not it has a unit, and so `Ω` — which the bundled mono face does not carry
//! — paints. The editable text stops short of it.

use egui::{Color32, Galley, Pos2, Rect, Response, Sense, Stroke, Ui, Vec2};
use std::sync::Arc;

use super::{elide_text, row_label, row_slot};
use crate::theme::{self, FontWeight};
use crate::tokens::{self, Tokens};
use crate::widgets::{PROBLEM_MARK_ADVANCE, PROBLEM_MARK_SIDE, Problem, paint_problem_mark};

/// What a property row says about its value beyond the value.
#[derive(Debug, Clone, Copy, Default)]
pub struct PropertyRowCaption<'a> {
    /// The unit the value is in, painted after it.
    pub unit: Option<&'a str>,
    /// What the property is, shown when the label is hovered and given to
    /// assistive technology as the field's description.
    pub description: Option<&'a str>,
}

/// Inset between a field's frame and its text, and between its unit and its
/// trailing edge.
const FIELD_INSET: f32 = 8.0;
/// Gap between a value and its unit.
const UNIT_GAP: f32 = 4.0;

/// Where a laid-out line's baseline sits below the top of the galley.
fn first_baseline(galley: &Galley) -> f32 {
    galley.rows.first().map_or(0.0, |placed| {
        placed.pos.y
            + placed
                .row
                .glyphs
                .first()
                .map_or(placed.row.size.y, |glyph| glyph.pos.y)
    })
}

/// The unit, set at the size of the value beside it so the two share a line
/// when both are centred on the row.
fn unit_galley(ui: &Ui, unit: Option<&str>, size: f32, color: Color32) -> Option<Arc<Galley>> {
    unit.filter(|unit| !unit.is_empty()).map(|unit| {
        ui.painter().layout_no_wrap(
            unit.to_owned(),
            theme::sans(size, FontWeight::Regular),
            color,
        )
    })
}

/// The label column of a row, with the description on hover.
fn caption_label(ui: &mut Ui, label_rect: Rect, label: &str, description: Option<&str>) {
    row_label(ui, label_rect, label);
    if let Some(description) = description.filter(|description| !description.is_empty()) {
        let _ = ui
            .interact(
                label_rect,
                ui.id().with(("property-row-caption", label)),
                Sense::hover(),
            )
            .on_hover_text(format!("{label} — {description}"));
    }
}

/// [`super::property_row_input_with_hint`] with the value's unit inside the
/// field's trailing edge and the parameter's description on its label.
pub fn property_row_input_captioned(
    ui: &mut Ui,
    label: &str,
    value: &mut String,
    hint: &str,
    invalid: bool,
    caption: PropertyRowCaption<'_>,
) -> Response {
    let t = Tokens::get(ui.ctx());
    let (slot, _) = row_slot(ui, None, Sense::hover());
    caption_label(ui, slot.label, label, caption.description);
    let value_rect = slot.field();
    let value_size = egui::TextStyle::Monospace.resolve(ui.style()).size;
    let unit = unit_galley(ui, caption.unit, value_size, t.color.text_faint);
    let unit_room = unit
        .as_ref()
        .map_or(0.0, |galley| galley.size().x + UNIT_GAP);
    let trailing = (FIELD_INSET + unit_room).min(f32::from(i8::MAX));
    let field_inset_y = super::field_margin(ui, value_rect, 0).top;
    let mut edit = egui::TextEdit::singleline(value)
        .hint_text(hint)
        .font(egui::TextStyle::Monospace)
        .margin(egui::Margin {
            left: FIELD_INSET as i8,
            right: trailing as i8,
            top: field_inset_y,
            bottom: field_inset_y,
        })
        .desired_width(value_rect.width());
    if invalid {
        edit = edit.text_color(t.color.err);
    }
    let response = ui.put(value_rect, edit);
    if let Some(galley) = unit {
        // On the value's baseline. The field sets its text at its top inset,
        // not at its centre, and the two faces sit differently in their rows.
        let value_font = egui::TextStyle::Monospace.resolve(ui.style());
        let probe = ui
            .painter()
            .layout_no_wrap("0".to_owned(), value_font, t.color.text);
        let baseline = response.rect.top() + f32::from(field_inset_y) + first_baseline(&probe);
        ui.painter().with_clip_rect(value_rect).galley(
            Pos2::new(
                response.rect.right() - FIELD_INSET - galley.size().x,
                baseline - first_baseline(&galley),
            ),
            galley,
            t.color.text_faint,
        );
    }
    let description = caption
        .description
        .filter(|description| !description.is_empty());
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_label(label);
        match (invalid, description) {
            (true, _) => node.set_description("Editable engineering value with a validation error"),
            (false, Some(description)) => node.set_description(description),
            (false, None) => node.set_description("Editable engineering value"),
        }
        if invalid {
            node.set_invalid(egui::accesskit::Invalid::True);
        } else {
            node.clear_invalid();
        }
    });
    if invalid {
        ui.painter().rect_stroke(
            value_rect,
            t.radius,
            Stroke::new(1.0, t.color.err),
            egui::StrokeKind::Inside,
        );
    }
    response
}

/// A read-only property row whose value carries its unit and whose label
/// carries the parameter's description, painted on the editable rows' grid
/// with the schematic inspector's body-sized value.
pub fn property_row_captioned(
    ui: &mut Ui,
    label: &str,
    value: &str,
    caption: PropertyRowCaption<'_>,
) -> Response {
    let t = Tokens::get(ui.ctx());
    let (slot, response) = row_slot(ui, None, Sense::hover());
    caption_label(ui, slot.label, label, caption.description);
    let value_rect = slot.value;
    let value_column = value_rect.width();
    let unit = unit_galley(ui, caption.unit, tokens::FS_1, t.color.text_faint);
    let unit_room = unit
        .as_ref()
        .map_or(0.0, |galley| galley.size().x + UNIT_GAP);
    // The body-sized mono value of the schematic inspector's read-only rows.
    let font = theme::mono(tokens::FS_1, FontWeight::Regular);
    let shown = elide_text(ui, value, &font, (value_column - unit_room).max(1.0));
    let value_galley = ui.painter().layout_no_wrap(shown, font, t.color.text);
    let value_width = value_galley.size().x;
    let value_top = value_rect.center().y - value_galley.size().y * 0.5;
    let baseline = value_top + first_baseline(&value_galley);
    ui.painter().with_clip_rect(value_rect).galley(
        Pos2::new(value_rect.left(), value_top),
        value_galley,
        t.color.text,
    );
    if let Some(galley) = unit {
        ui.painter().with_clip_rect(value_rect).galley(
            Pos2::new(
                value_rect.left() + value_width + UNIT_GAP,
                baseline - first_baseline(&galley),
            ),
            galley,
            t.color.text_faint,
        );
    }
    let stated = caption
        .unit
        .filter(|unit| !unit.is_empty())
        .map_or_else(|| value.to_owned(), |unit| format!("{value} {unit}"));
    super::announce_row(ui, &response, label, &stated);
    if let Some(description) = caption.description {
        ui.ctx()
            .accesskit_node_builder(response.id, |node| node.set_description(description));
    }
    response
}

/// A property row for a value the reader is told to deal with: the value with
/// `note` after it, and one text action at the trailing edge. Returns whether
/// the action was pressed.
///
/// The value and the note are neutral; a `problem` paints its mark in front of
/// the value and is the only colour on the row.
pub fn property_row_flagged(
    ui: &mut Ui,
    label: &str,
    value: &str,
    note: &str,
    problem: Option<Problem>,
    action: &str,
) -> bool {
    let t = Tokens::get(ui.ctx());
    let (slot, _) = row_slot(ui, None, Sense::hover());
    caption_label(ui, slot.label, label, None);
    let value_column = slot.value.width();

    let action_font = theme::sans(tokens::FS_0, FontWeight::Medium);
    let action_galley = ui
        .painter()
        .layout_no_wrap(action.to_owned(), action_font, t.color.text);
    // The action is framed like the row's field would be: the field's height,
    // ending on the inset.
    let field = slot.field();
    let action_rect = Rect::from_min_max(
        Pos2::new(
            field.right() - action_galley.size().x - 2.0 * FIELD_INSET,
            field.top(),
        ),
        field.max,
    );
    let value_rect = Rect::from_min_max(
        Pos2::new(slot.value.left(), slot.rect.top()),
        Pos2::new(action_rect.left() - UNIT_GAP, slot.rect.bottom()),
    );
    let value_font = theme::mono(tokens::FS_1, FontWeight::Regular);
    let note_font = theme::sans(tokens::FS_0, FontWeight::Regular);
    let note_galley = ui
        .painter()
        .layout_no_wrap(note.to_owned(), note_font, t.color.text_dim);
    let note_room = note_galley.size().x + UNIT_GAP;
    let mark_room = if problem.is_some() {
        PROBLEM_MARK_ADVANCE
    } else {
        0.0
    };
    let shown = elide_text(
        ui,
        value,
        &value_font,
        (value_column.min(value_rect.width()) - note_room - mark_room).max(1.0),
    );
    let value_galley = ui.painter().layout_no_wrap(shown, value_font, t.color.text);
    let value_width = value_galley.size().x;
    let value_top = value_rect.center().y - value_galley.size().y * 0.5;
    let baseline = value_top + first_baseline(&value_galley);
    let painter = ui.painter().with_clip_rect(value_rect);
    if let Some(problem) = problem {
        paint_problem_mark(
            &painter,
            Rect::from_center_size(
                Pos2::new(
                    value_rect.left() + PROBLEM_MARK_SIDE * 0.5,
                    value_rect.center().y,
                ),
                Vec2::splat(PROBLEM_MARK_SIDE),
            ),
            problem,
        );
    }
    let value_left = value_rect.left() + mark_room;
    painter.galley(Pos2::new(value_left, value_top), value_galley, t.color.text);
    painter.galley(
        Pos2::new(
            value_left + value_width + UNIT_GAP,
            baseline - first_baseline(&note_galley),
        ),
        note_galley,
        t.color.text_dim,
    );

    let response = ui.interact(
        action_rect,
        ui.id().with(("property-row-flagged", label)),
        Sense::click(),
    );
    let action_name = format!("{action} {label}");
    response.widget_info(|| {
        egui::WidgetInfo::labeled(egui::WidgetType::Button, true, action_name.clone())
    });
    ui.painter().rect(
        action_rect,
        t.radius,
        if response.hovered() {
            t.color.bg_hover
        } else {
            t.color.bg_elevated
        },
        Stroke::new(1.0, t.color.border),
        egui::StrokeKind::Inside,
    );
    let size = action_galley.size();
    ui.painter().galley(
        Pos2::new(
            action_rect.center().x - size.x * 0.5,
            action_rect.center().y - size.y * 0.5,
        ),
        action_galley,
        t.color.text,
    );
    theme::paint_focus_ring(ui, &response, action_rect);
    response.on_hover_text(action_name).clicked()
}
