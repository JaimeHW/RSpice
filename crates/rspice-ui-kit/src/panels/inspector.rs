//! Notes: the one way a panel writes a sentence.

use super::body::{panel_inset, property_row_height};
use super::text_pad;
use crate::theme::{self, FontWeight};
use crate::tokens::{self, Tokens};
use egui::{Color32, Pos2, Sense, Ui, Vec2};

/// A sentence in a panel body: what a section's rows mean, rather than one
/// more of them.
///
/// It wraps between the same left and right inset a property row gives its
/// label and value, and its first line sits where a row's text sits, so a
/// note lines up with the rows around it and keeps their rhythm.
pub fn muted_inspector_copy(ui: &mut Ui, text: &str) {
    let tone = Tokens::get(ui.ctx()).color.text_dim;
    toned_inspector_copy(ui, text, tone);
}

/// [`muted_inspector_copy`] in a tone that carries meaning: a warning a reader
/// should act on, or a remark quieter than the rows it sits under.
pub fn toned_inspector_copy(ui: &mut Ui, text: &str, tone: Color32) {
    let inset = panel_inset(ui);
    let galley = ui.painter().layout(
        text.to_owned(),
        theme::sans(tokens::FS_0, FontWeight::Regular),
        tone,
        (ui.available_width() - 2.0 * inset).max(1.0),
    );
    let pad = text_pad(property_row_height(ui), &galley);
    let (rect, response) = ui.allocate_exact_size(
        Vec2::new(ui.available_width(), galley.size().y + 2.0 * pad),
        Sense::hover(),
    );
    response
        .widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Label, ui.is_enabled(), text));
    ui.painter().galley(
        Pos2::new(rect.left() + inset, rect.top() + pad),
        galley,
        tone,
    );
}
