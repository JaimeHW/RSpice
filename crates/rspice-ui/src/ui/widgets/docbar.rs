//! Document toolbar — the compact strip at the top of a center view holding
//! the breadcrumb and view-local actions.

use egui::{InnerResponse, Ui, vec2};

use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};

const RESULTS_DOCBAR_INSET: f32 = 8.0;

/// Render a document bar with a surface-specific height.
///
/// Every caller supplies its own height — Results uses the mockup's 41 px
/// desktop / 39 px phone strip — so the fixed-height `docbar` wrapper and its
/// `DOCBAR_HEIGHT`/`COMPACT_DOCBAR_INSET` constants are gone. Keeping
/// allocation and painting here guarantees the border remains part of the
/// exact owned height.
pub fn docbar_at_height<R>(
    ui: &mut Ui,
    height: f32,
    add_contents: impl FnOnce(&mut Ui) -> R,
) -> InnerResponse<R> {
    docbar_with_geometry(ui, height, RESULTS_DOCBAR_INSET, add_contents)
}

fn docbar_with_geometry<R>(
    ui: &mut Ui,
    height: f32,
    horizontal_inset: f32,
    add_contents: impl FnOnce(&mut Ui) -> R,
) -> InnerResponse<R> {
    debug_assert!(height.is_finite() && height > 0.0);
    debug_assert!(horizontal_inset.is_finite() && horizontal_inset >= 0.0);
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let width = ui.available_width();
    let (rect, _) = ui.allocate_exact_size(vec2(width, height), egui::Sense::hover());

    let painter = ui.painter();
    painter.rect_filled(rect, 0.0, c.bg_panel);
    painter.hline(
        rect.x_range(),
        rect.bottom() - 0.5,
        egui::Stroke::new(1.0, c.border),
    );

    let content_rect = rect.shrink2(vec2(horizontal_inset, 0.0));
    let mut child = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(content_rect)
            .layout(egui::Layout::left_to_right(egui::Align::Center)),
    );
    child.spacing_mut().item_spacing.x = 8.0;
    let inner = add_contents(&mut child);
    InnerResponse::new(
        inner,
        ui.interact(rect, ui.id().with("docbar"), egui::Sense::hover()),
    )
}

