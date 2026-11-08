//! The way out of a filter field.

use egui::{Key, Modifiers, Response, Sense, Ui};

use crate::panels::WorkbenchIcon;
use crate::theme;
use crate::tokens::Tokens;

/// The right inset a field keeps for [`filter_field_clear`], so its text ends
/// before the control that empties it.
pub const FILTER_CLEAR_INSET: i8 = 26;
/// The control's hit target: the field's own height less its border, so the
/// whole right inset answers the pointer.
const FILTER_CLEAR_TARGET: f32 = 22.0;
/// The crossing strokes inside that target.
const FILTER_CLEAR_MARK: f32 = 13.0;

/// Empties `query` from the mark in `field`'s right inset, or on Escape.
///
/// An empty field draws nothing and keeps its hands off the press, so Escape
/// goes on meaning "put the tool away" everywhere else. egui surrenders a text
/// field's focus on Escape on its own, so that press is one gesture: the query
/// goes and the keyboard comes back out. The pointer leaves the reader in the
/// field it just emptied.
pub fn filter_field_clear(ui: &mut Ui, field: &Response, query: &mut String) {
    if query.is_empty() {
        return;
    }
    let cleared = clear_mark(ui, field);
    // `lost_focus` as well as `has_focus`: egui takes the focus off a text
    // field the moment it reads the Escape, so by the time the field is
    // painted the press has already cost it the focus it is being asked about.
    let escaped = (field.has_focus() || field.lost_focus())
        && ui.input_mut(|input| input.consume_key(Modifiers::NONE, Key::Escape));
    if cleared || escaped {
        query.clear();
    }
    if cleared {
        field.request_focus();
    }
}

/// The mark is [`WorkbenchIcon::Close`], the same crossing strokes every other
/// dismissal in the workbench paints, rather than a `\u{2715}` set in text, so it
/// matches them stroke for stroke whichever face holds the character.
fn clear_mark(ui: &mut Ui, field: &Response) -> bool {
    let t = Tokens::get(ui.ctx());
    let rect = egui::Rect::from_center_size(
        egui::pos2(
            field.rect.right() - f32::from(FILTER_CLEAR_INSET) * 0.5 - 2.0,
            field.rect.center().y,
        ),
        egui::Vec2::splat(FILTER_CLEAR_TARGET),
    );
    let response = ui.interact(rect, field.id.with("clear"), Sense::click());
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::Button,
            ui.is_enabled(),
            "Clear the filter",
        )
    });
    WorkbenchIcon::Close.paint(
        ui.painter(),
        egui::Rect::from_center_size(rect.center(), egui::Vec2::splat(FILTER_CLEAR_MARK)),
        if response.hovered() || response.has_focus() {
            t.color.text
        } else {
            t.color.text_faint
        },
    );
    theme::paint_focus_ring(ui, &response, rect);
    response.clicked()
}
