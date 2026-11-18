//! The tab strip of a dialog whose body holds several pages.
//!
//! One band across a flush dialog body, as [`band_header`](super::band_header)
//! is: each tab's label stands [`DIALOG_BODY_INSET`] in from the tab's leading
//! edge, so the first one is on the title's left edge. A tab is as wide as its
//! label needs, never narrower than the others' floor, and the chosen one is
//! raised with the accent under it. Where the strip is too narrow for that,
//! as on a phone, the tabs fold into equal shares of it with their labels
//! centred, and where a label would not fit its share the strip scrolls
//! under a finger instead: a tab is never cut. Every dialog with pages wears
//! this one strip: three strips for one control read as three different
//! controls.

use std::sync::Arc;

use egui::{
    Galley, Rect, Response, ScrollArea, Sense, Stroke, Ui, WidgetInfo, WidgetType, pos2, vec2,
};

use super::DIALOG_BODY_INSET;
use crate::theme::{self, FontWeight};
use crate::tokens::{self, Tokens};

/// The strip's height under a pointer; under a finger it takes the touch
/// control height.
const TAB_HEIGHT: f32 = 34.0;
/// The narrowest a tab is, so a strip of short labels still reads as tabs.
const TAB_MIN_WIDTH: f32 = 112.0;
/// The least room either side of a label in a folded tab.
const TAB_FOLD_INSET: f32 = 8.0;

/// Draws the tab strip across `ui`'s width and switches `active` to a tab
/// that is clicked. Returns the first tab's response, which a dialog gives
/// its initial focus.
pub fn dialog_tabs(
    ui: &mut Ui,
    id_salt: &'static str,
    labels: &[&str],
    active: &mut usize,
) -> Response {
    let t = Tokens::get(ui.ctx());
    let font = theme::sans(tokens::FS_1, FontWeight::Regular);
    let galleys = labels
        .iter()
        .map(|label| {
            ui.painter()
                .layout_no_wrap((*label).to_owned(), font.clone(), t.color.text)
        })
        .collect::<Vec<_>>();
    let natural = galleys
        .iter()
        .map(|galley| (galley.size().x + 2.0 * DIALOG_BODY_INSET).max(TAB_MIN_WIDTH))
        .collect::<Vec<_>>();
    let room = ui.available_width();
    let share = room / labels.len().max(1) as f32;
    if natural.iter().sum::<f32>() <= room {
        strip(ui, id_salt, labels, galleys, &natural, false, active)
    } else if galleys
        .iter()
        .all(|galley| galley.size().x + 2.0 * TAB_FOLD_INSET <= share)
    {
        let folded = vec![share; labels.len()];
        strip(ui, id_salt, labels, galleys, &folded, true, active)
    } else {
        ScrollArea::horizontal()
            .id_salt((id_salt, "scroll"))
            .auto_shrink([false, true])
            .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysHidden)
            .show(ui, |ui| {
                strip(ui, id_salt, labels, galleys, &natural, false, active)
            })
            .inner
    }
}

/// The strip itself: one tab per label at `widths`, each label at the tab's
/// inset or, when `centred`, in the middle of its tab.
fn strip(
    ui: &mut Ui,
    id_salt: &'static str,
    labels: &[&str],
    galleys: Vec<Arc<Galley>>,
    widths: &[f32],
    centred: bool,
    active: &mut usize,
) -> Response {
    let t = Tokens::get(ui.ctx());
    let height = TAB_HEIGHT.max(t.metrics.ctl_h);
    let width = ui.available_width().max(widths.iter().sum());
    let (strip, strip_response) = ui.allocate_exact_size(vec2(width, height), Sense::hover());
    ui.painter().rect_filled(strip, 0.0, t.color.bg_panel);
    ui.painter().hline(
        strip.x_range(),
        strip.bottom() - 0.5,
        Stroke::new(1.0, t.color.border),
    );
    let id = ui.id().with(id_salt);
    let mut x = strip.left();
    let mut first = None;
    for (index, (label, galley)) in labels.iter().zip(galleys).enumerate() {
        let width = widths[index];
        let rect = Rect::from_min_size(pos2(x, strip.top()), vec2(width, height));
        let response = ui.interact(rect, id.with(index), Sense::click());
        let selected = *active == index;
        response.widget_info(|| {
            WidgetInfo::selected(
                WidgetType::SelectableLabel,
                ui.is_enabled(),
                selected,
                *label,
            )
        });
        if response.clicked() {
            *active = index;
            response.scroll_to_me(None);
        }
        if selected {
            ui.painter().rect_filled(rect, 0.0, t.color.bg_elevated);
            ui.painter().hline(
                rect.x_range(),
                rect.bottom() - 1.0,
                Stroke::new(2.0, t.color.accent),
            );
        } else if response.hovered() {
            ui.painter().rect_filled(rect, 0.0, t.color.bg_hover);
        }
        ui.painter().vline(
            rect.right() - 0.5,
            rect.y_range(),
            Stroke::new(1.0, t.color.border),
        );
        let left = if centred {
            rect.center().x - galley.size().x / 2.0
        } else {
            rect.left() + DIALOG_BODY_INSET
        };
        ui.painter().galley(
            pos2(left, rect.center().y - galley.size().y / 2.0),
            galley,
            if selected {
                t.color.text
            } else {
                t.color.text_dim
            },
        );
        theme::paint_focus_ring(ui, &response, rect);
        first.get_or_insert(response);
        x += width;
    }
    first.unwrap_or(strip_response)
}

#[cfg(test)]
mod tests {}
