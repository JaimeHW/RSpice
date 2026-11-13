//! The strip as a drop target: rows dragged out of the data browser go on
//! the plot where they are let go.

use egui::Ui;

use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};

use super::super::ResultsParts;
use super::super::browser_plot::{ResultQuantityDrag, drag_name, set_result_quantity_ticked};

/// Take browser rows dropped anywhere on the strips' area, `zone`. A
/// waveform goes on the pane of its unit, a new pane when its unit has none
/// yet; an event net or a bus goes into the lanes; a run's own strip comes
/// back if it was closed. While rows hover the area it says what letting go
/// will do.
pub(super) fn accept_browser_drop(ui: &Ui, state: &mut ResultsParts<'_>, zone: egui::Rect) {
    let ctx = ui.ctx().clone();
    let Some(drag) = egui::DragAndDrop::payload::<ResultQuantityDrag>(&ctx) else {
        return;
    };
    let Some(pointer) = ctx.pointer_latest_pos() else {
        return;
    };
    let covered = ctx
        .layer_id_at(pointer)
        .is_some_and(|layer| layer != ui.layer_id());
    if drag.keys.is_empty() || !zone.contains(pointer) || covered {
        return;
    }
    let t = Tokens::get(&ctx);
    let painter = ui.painter().with_clip_rect(zone);
    painter.rect_filled(zone, 0.0, t.color.accent.gamma_multiply(0.06));
    painter.rect_stroke(
        zone.shrink(1.0),
        t.radius,
        egui::Stroke::new(2.0, t.color.accent),
        egui::StrokeKind::Inside,
    );
    let caption = drop_caption(&drag);
    let galley = painter.layout_no_wrap(
        caption,
        theme::sans(tokens::FS_0, FontWeight::Medium),
        t.color.text,
    );
    let banner = egui::Rect::from_center_size(
        egui::pos2(zone.center().x, zone.top() + 22.0),
        galley.size() + egui::vec2(20.0, 10.0),
    );
    painter.rect_filled(banner, t.radius, t.color.bg_panel);
    painter.rect_stroke(
        banner,
        t.radius,
        egui::Stroke::new(1.0, t.color.accent),
        egui::StrokeKind::Inside,
    );
    painter.galley(banner.center() - galley.size() * 0.5, galley, t.color.text);
    if ctx.input(|input| input.pointer.any_released()) {
        egui::DragAndDrop::clear_payload(&ctx);
        for key in &drag.keys {
            set_result_quantity_ticked(state, key, true);
        }
        ctx.request_repaint();
    }
}

/// What letting go will do, in the words of the rows being dragged.
fn drop_caption(drag: &ResultQuantityDrag) -> String {
    match drag.keys.as_slice() {
        [one] => format!("Drop to plot {}", drag_name(one)),
        many => format!("Drop to plot {} signals", many.len()),
    }
}
