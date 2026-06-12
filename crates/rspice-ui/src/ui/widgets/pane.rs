//! The two-pane master–detail chrome of the v2 dialog grammar
//! (`design/volta-dialogs-v2.html` `.panes` / `.hd` / `.seclab`): one
//! bordered container split into a fixed-width rail on `bg_panel` and a
//! fluid detail region on `bg_elevated`, each flowing top-down with a
//! 36 pt header strip and an optional footer strip pinned to the bottom.
//!
//! The container pre-allocates its exact rect and hands each side a
//! clipped, top-down child `Ui` — pane contents can rely on
//! `available_width()` being the pane width regardless of the layout the
//! dialog body happens to be in.

use egui::{Align, Layout, Rect, Rounding, Sense, Stroke, Ui, UiBuilder, pos2, vec2};

use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};

/// Height of the `.hd` header strip.
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
            Rounding {
                nw: 0.0,
                sw: 0.0,
                ne: t.radius,
                se: t.radius,
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
    painter.rect_stroke(outer, t.radius, Stroke::new(1.0, c.border));
}

/// The `.hd` strip at the top of a pane: 36 pt, contents centered
/// vertically with 10 pt side padding, hairline rule underneath.
pub fn pane_header(ui: &mut Ui, add_contents: impl FnOnce(&mut Ui)) {
    let t = Tokens::get(ui.ctx());
    let response = ui.allocate_ui_with_layout(
        vec2(ui.available_width(), PANE_HEADER_H),
        Layout::left_to_right(egui::Align::Center),
        |ui| {
            ui.set_min_size(vec2(ui.available_width(), PANE_HEADER_H));
            ui.spacing_mut().item_spacing.x = 8.0;
            ui.add_space(10.0);
            add_contents(ui);
            ui.add_space(10.0);
        },
    );
    ui.painter().hline(
        response.response.rect.x_range(),
        response.response.rect.bottom() - 0.5,
        Stroke::new(1.0, t.color.border),
    );
}

/// The footer strip pinned to the pane bottom: faint sans meta text over
/// a hairline rule (the spec's bottom `.hd` variant).
pub fn pane_footer(ui: &mut Ui, text: &str) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    ui.with_layout(Layout::bottom_up(Align::Min), |ui| {
        ui.allocate_ui_with_layout(
            vec2(ui.available_width(), PANE_FOOTER_H),
            Layout::left_to_right(egui::Align::Center),
            |ui| {
                ui.set_min_size(vec2(ui.available_width(), PANE_FOOTER_H));
                ui.add_space(10.0);
                ui.label(
                    egui::RichText::new(text)
                        .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                        .color(c.text_faint),
                );
            },
        );
        ui.painter().hline(
            ui.min_rect().x_range(),
            ui.min_rect().top() + 0.5,
            Stroke::new(1.0, c.border),
        );
    });
}

/// The `.seclab` section label: small mono uppercase, baseline near the
/// strip bottom so it caps the list section that follows.
pub fn pane_section_label(ui: &mut Ui, text: &str) {
    let t = Tokens::get(ui.ctx());
    let (rect, _) = ui.allocate_exact_size(vec2(ui.available_width(), 22.0), Sense::hover());
    if !ui.is_rect_visible(rect) {
        return;
    }
    let mut job = egui::text::LayoutJob::default();
    job.append(
        &text.to_uppercase(),
        0.0,
        egui::TextFormat {
            font_id: theme::mono(9.5, FontWeight::Medium),
            color: t.color.text_faint,
            extra_letter_spacing: 0.12 * 9.5,
            ..Default::default()
        },
    );
    let galley = ui.fonts(|f| f.layout_job(job));
    ui.painter().galley(
        pos2(rect.left() + 10.0, rect.bottom() - galley.size().y - 2.0),
        galley,
        t.color.text_faint,
    );
}
