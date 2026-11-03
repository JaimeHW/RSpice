//! The tab strip of a dialog whose body holds several pages.
//!
//! One band across a flush dialog body, as [`band_header`](super::band_header)
//! is: each tab's label stands [`DIALOG_BODY_INSET`] in from the tab's leading
//! edge, so the first one is on the title's left edge. A tab is as wide as its
//! label needs, never narrower than the others' floor, and the chosen one is
//! raised with the accent under it. Every dialog with pages wears this one
//! strip: three strips for one control read as three different controls.

use egui::{Rect, Response, Sense, Stroke, Ui, WidgetInfo, WidgetType, pos2, vec2};

use super::DIALOG_BODY_INSET;
use crate::theme::{self, FontWeight};
use crate::tokens::{self, Tokens};

/// The strip's height under a pointer; under a finger it takes the touch
/// control height.
const TAB_HEIGHT: f32 = 34.0;
/// The narrowest a tab is, so a strip of short labels still reads as tabs.
const TAB_MIN_WIDTH: f32 = 112.0;

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
    let height = TAB_HEIGHT.max(t.metrics.ctl_h);
    let font = theme::sans(tokens::FS_1, FontWeight::Regular);
    let (strip, strip_response) =
        ui.allocate_exact_size(vec2(ui.available_width(), height), Sense::hover());
    ui.painter().rect_filled(strip, 0.0, t.color.bg_panel);
    ui.painter().hline(
        strip.x_range(),
        strip.bottom() - 0.5,
        Stroke::new(1.0, t.color.border),
    );
    let id = ui.id().with(id_salt);
    let mut x = strip.left();
    let mut first = None;
    for (index, label) in labels.iter().enumerate() {
        let galley = ui
            .painter()
            .layout_no_wrap((*label).to_owned(), font.clone(), t.color.text);
        let width = (galley.size().x + 2.0 * DIALOG_BODY_INSET).max(TAB_MIN_WIDTH);
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
        ui.painter().galley(
            pos2(
                rect.left() + DIALOG_BODY_INSET,
                rect.center().y - galley.size().y / 2.0,
            ),
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
mod tests {
    use super::*;

    fn strip(target: Option<f32>, labels: &[&str], active: usize) -> Vec<Rect> {
        let ctx = egui::Context::default();
        crate::Theme::default().apply_responsive_metrics_with_target(&ctx, target);
        ctx.enable_accesskit();
        let mut tabs = Vec::new();
        let nodes = ctx
            .run_ui(egui::RawInput::default(), |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    let mut active = active;
                    let first = dialog_tabs(ui, "tabs", labels, &mut active);
                    tabs.push(first.rect);
                });
            })
            .platform_output
            .accesskit_update
            .expect("AccessKit tree update")
            .nodes;
        let selected: Vec<_> = nodes
            .iter()
            .filter(|(_, node)| node.toggled() == Some(egui::accesskit::Toggled::True))
            .filter_map(|(_, node)| node.label().map(str::to_owned))
            .collect();
        assert_eq!(
            selected,
            [labels[active]],
            "only the chosen tab is selected"
        );
        tabs
    }

    /// The first label stands on the body's left edge, a tab grows to a label
    /// longer than the floor, and a finger gets a touch-height strip.
    #[test]
    fn tabs_fit_their_labels_and_stand_on_the_bodys_edge() {
        let ctx = egui::Context::default();
        crate::Theme::default().apply(&ctx);
        let long = "Session restore and recovery of every window";
        let mut widths = Vec::new();
        let mut first = Rect::NOTHING;
        let _ = ctx.run_ui(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                let mut active = 1;
                first = dialog_tabs(ui, "tabs", &["Sheets", long], &mut active).rect;
                let font = theme::sans(tokens::FS_1, FontWeight::Regular);
                widths.push(
                    ui.painter()
                        .layout_no_wrap(long.to_owned(), font, egui::Color32::WHITE)
                        .size()
                        .x,
                );
            });
        });
        assert_eq!(
            first.width(),
            TAB_MIN_WIDTH,
            "a short label keeps the floor"
        );
        assert_eq!(first.height(), TAB_HEIGHT);
        assert!(widths[0] + 2.0 * DIALOG_BODY_INSET > TAB_MIN_WIDTH);
        let touch = strip(Some(44.0), &["Sheets", "Variants"], 0);
        assert_eq!(touch[0].height(), 44.0, "a finger gets the touch height");
        let pointer = strip(None, &["Sheets", "Variants"], 1);
        assert_eq!(pointer[0].height(), TAB_HEIGHT);
    }
}
