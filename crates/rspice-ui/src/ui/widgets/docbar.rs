//! Document toolbar — the 34 px strip at the top of each center view holding
//! the breadcrumb and view-local actions.

use egui::{InnerResponse, Ui, vec2};

use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};

/// Height of the document bar.
pub const DOCBAR_HEIGHT: f32 = 34.0;

/// Render a document bar across the available width. The closure lays out the
/// bar's content left-to-right; use `ui.add_space(ui.available_width() - …)`
/// or a right-to-left child for trailing actions.
pub fn docbar<R>(ui: &mut Ui, add_contents: impl FnOnce(&mut Ui) -> R) -> InnerResponse<R> {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let width = ui.available_width();
    let (rect, _) = ui.allocate_exact_size(vec2(width, DOCBAR_HEIGHT), egui::Sense::hover());

    let painter = ui.painter();
    painter.rect_filled(rect, 0.0, c.bg_panel);
    painter.hline(
        rect.x_range(),
        rect.bottom() - 0.5,
        egui::Stroke::new(1.0, c.border),
    );

    let content_rect = rect.shrink2(vec2(10.0, 0.0));
    let mut child = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(content_rect)
            .layout(egui::Layout::left_to_right(egui::Align::Center)),
    );
    child.spacing_mut().item_spacing.x = 8.0;
    let inner = add_contents(&mut child);
    InnerResponse::new(inner, ui.interact(rect, ui.id().with("docbar"), egui::Sense::hover()))
}

/// Render breadcrumb text ("work_amp › **ota_5t** › schematic"): mono 11 px,
/// dim segments with the emphasized segment in primary text color.
pub fn crumb_text(ui: &mut Ui, segments: &[(&str, bool)]) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let mut job = egui::text::LayoutJob::default();
    for (i, (segment, emphasized)) in segments.iter().enumerate() {
        if i > 0 {
            job.append(
                " › ",
                0.0,
                egui::TextFormat {
                    font_id: theme::mono(tokens::FS_0, FontWeight::Regular),
                    color: c.text_dim,
                    ..Default::default()
                },
            );
        }
        let (color, weight) = if *emphasized {
            (c.text, FontWeight::Medium)
        } else {
            (c.text_dim, FontWeight::Regular)
        };
        job.append(
            segment,
            0.0,
            egui::TextFormat {
                font_id: theme::mono(tokens::FS_0, weight),
                color,
                ..Default::default()
            },
        );
    }
    ui.label(job);
}
