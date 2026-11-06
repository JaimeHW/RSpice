//! The two-pane master–detail chrome of the v2 dialog grammar
//! one bordered container split into a fixed-width rail on `bg_panel` and a
//! fluid detail region on `bg_elevated`, each flowing top-down with a
//! 36 pt header strip and an optional footer strip pinned to the bottom.
//!
//! The container pre-allocates its exact rect and hands each side a
//! clipped, top-down child `Ui` — pane contents can rely on
//! `available_width()` being the pane width regardless of the layout the
//! dialog body happens to be in.

use egui::{Align, CornerRadius, Layout, Rect, Sense, Stroke, Ui, UiBuilder, pos2, vec2};

use crate::theme::{self, FontWeight};
use crate::tokens::{self, Tokens};

/// Height of the `.hd` header strip under a pointer; see
/// [`pane_header_height`].
pub const PANE_HEADER_H: f32 = 36.0;
/// Height of the footer strip (the spec reuses `.hd` at the pane foot).
pub const PANE_FOOTER_H: f32 = 36.0;
/// Width of the fixed rail (the spec's `grid-template-columns: 244px 1fr`).
pub const PANE_RAIL_W: f32 = 244.0;

/// Which side of the master–detail surface a `two_pane` callback is
/// filling.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PaneSide {
    /// The fixed-width rail on the left.
    Rail,
    /// The fluid detail region on the right.
    Detail,
}

/// One bordered master–detail surface: `rail_width` rail left, fluid
/// detail right, hairline divider between, spanning the available width
/// at exactly `height`. `add_pane` runs once per side (rail first) so a
/// single capture set can fill both.
pub fn two_pane(
    ui: &mut Ui,
    rail_width: f32,
    height: f32,
    mut add_pane: impl FnMut(&mut Ui, PaneSide),
) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;

    let width = ui.available_width();
    let rail_width = rail_width.min(width);
    let (outer, _) = ui.allocate_exact_size(vec2(width, height), Sense::hover());
    if !ui.is_rect_visible(outer) {
        return;
    }

    let rail_rect = Rect::from_min_size(outer.min, vec2(rail_width, height));
    let detail_rect = Rect::from_min_max(pos2(outer.min.x + rail_width, outer.top()), outer.max);

    let painter = ui.painter();
    painter.rect_filled(outer, t.radius, c.bg_panel);
    if detail_rect.width() > 0.0 {
        painter.rect_filled(
            detail_rect,
            CornerRadius {
                nw: 0,
                sw: 0,
                ne: t.radius.round() as u8,
                se: t.radius.round() as u8,
            },
            c.bg_elevated,
        );
    }

    for (rect, side) in [(rail_rect, PaneSide::Rail), (detail_rect, PaneSide::Detail)] {
        let mut pane_ui = ui.new_child(
            UiBuilder::new()
                .max_rect(rect)
                .layout(Layout::top_down(Align::Min)),
        );
        pane_ui.set_clip_rect(rect.intersect(ui.clip_rect()));
        pane_ui.spacing_mut().item_spacing.y = 0.0;
        add_pane(&mut pane_ui, side);
    }

    // Chrome over the contents: outer border and the rail/detail divider.
    let painter = ui.painter();
    painter.vline(
        outer.min.x + rail_width - 0.5,
        outer.y_range(),
        Stroke::new(1.0, c.border),
    );
    painter.rect_stroke(
        outer,
        t.radius,
        Stroke::new(1.0, c.border),
        egui::StrokeKind::Inside,
    );
}

/// How tall a pane's header strip is: [`PANE_HEADER_H`], or under a finger
/// as tall as the controls it holds and the strip's 4 pt of air over and
/// under them, so a 44 pt button is not taller than its strip.
pub fn pane_header_height(ctx: &egui::Context) -> f32 {
    let metrics = Tokens::get(ctx).metrics;
    if metrics.is_touch() {
        PANE_HEADER_H.max(metrics.ctl_h + 8.0)
    } else {
        PANE_HEADER_H
    }
}

/// The side padding of a pane's header and footer strips.
pub const PANE_INSET: f32 = 10.0;

/// The `.hd` strip at the top of a pane: [`pane_header_height`] tall,
/// contents centered vertically inside [`PANE_INSET`] side padding
/// (right-aligned sub-layouts land on the padded edge, not the pane border),
/// hairline rule underneath.
pub fn pane_header(ui: &mut Ui, add_contents: impl FnOnce(&mut Ui)) {
    pane_header_inset(ui, PANE_INSET, add_contents);
}

/// [`pane_header`] with `inset` side padding, for panes laid edge to edge in
/// a dialog, whose contents stand on the dialog's own left edge.
pub fn pane_header_inset(ui: &mut Ui, inset: f32, add_contents: impl FnOnce(&mut Ui)) {
    let t = Tokens::get(ui.ctx());
    let height = pane_header_height(ui.ctx());
    let (rect, _) = ui.allocate_exact_size(vec2(ui.available_width(), height), Sense::hover());
    let mut header_ui = ui.new_child(
        UiBuilder::new()
            .max_rect(rect.shrink2(vec2(inset, 0.0)))
            .layout(Layout::left_to_right(Align::Center)),
    );
    header_ui.spacing_mut().item_spacing.x = 8.0;
    add_contents(&mut header_ui);
    ui.painter().hline(
        rect.x_range(),
        rect.bottom() - 0.5,
        Stroke::new(1.0, t.color.border),
    );
}

/// The footer strip pinned to the pane bottom: faint sans meta text over
/// a hairline rule (the spec's bottom `.hd` variant).
pub fn pane_footer(ui: &mut Ui, text: &str) {
    pane_footer_inset(ui, PANE_INSET, text);
}

/// [`pane_footer`] with its text `inset` from the pane's left edge.
pub fn pane_footer_inset(ui: &mut Ui, inset: f32, text: &str) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    ui.with_layout(Layout::bottom_up(Align::Min), |ui| {
        let (rect, _) =
            ui.allocate_exact_size(vec2(ui.available_width(), PANE_FOOTER_H), Sense::hover());
        let painter = ui.painter();
        painter.hline(rect.x_range(), rect.top() + 0.5, Stroke::new(1.0, c.border));
        painter.text(
            pos2(rect.left() + inset, rect.center().y),
            egui::Align2::LEFT_CENTER,
            text,
            theme::sans(tokens::FS_0, FontWeight::Regular),
            c.text_faint,
        );
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Under a finger a header's 44 pt buttons stand inside its strip with
    /// the strip's air over and under them; under a pointer it stays 36 pt.
    #[test]
    fn a_pane_header_holds_the_controls_a_finger_gets() {
        let ctx = egui::Context::default();
        crate::Theme::default().apply(&ctx);
        assert_eq!(pane_header_height(&ctx), PANE_HEADER_H);
        crate::Theme::default().apply_responsive_metrics_with_target(&ctx, Some(44.0));
        let ctl_h = Tokens::get(&ctx).metrics.ctl_h;
        assert!(pane_header_height(&ctx) >= ctl_h + 8.0);
    }
}
