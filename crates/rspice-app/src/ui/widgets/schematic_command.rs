//! Mockup-owned live schematic command composition.
//!
//! Placement and transform commands share one exact shell: a 1.55fr/.8fr
//! split, a 250-point dotted schematic preview, three equal engineering
//! outcome cards, and a placement/transform parameter pane. This module owns
//! presentation only; callers retain all document authority and mutations.

use egui::{Align, Frame, Layout, Margin, Rect, Stroke, Ui, Vec2};

use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};

const SPLIT_VIEWPORT_BREAKPOINT: f32 = 760.0;
const COMPACT_COLUMNS_BREAKPOINT: f32 = 980.0;
const RIGHT_MIN_WIDTH: f32 = 270.0;
const COMPACT_RIGHT_MIN_WIDTH: f32 = 240.0;
const PANE_PADDING: i8 = 14;
const CANVAS_HEIGHT: f32 = 250.0;
const COMPACT_CANVAS_HEIGHT: f32 = 220.0;
const PANE_MIN_HEIGHT: f32 = 414.0;
const OPTIONS_HEIGHT: f32 = 356.0;

#[derive(Debug, Clone, Copy)]
pub(crate) struct SchematicCommandPreview<'a> {
    pub subject: &'a str,
    pub location: &'a str,
    pub electrical_outcome: &'a str,
    pub grid: &'a str,
}

pub(crate) fn schematic_command_workflow<R>(
    ui: &mut Ui,
    code: &str,
    preview: SchematicCommandPreview<'_>,
    options_status: &str,
    status_ok: bool,
    options: impl FnOnce(&mut Ui) -> R,
) -> R {
    let t = Tokens::get(ui.ctx());
    let mut options = Some(options);
    let mut output = None;
    Frame::new()
        .fill(t.color.bg_inset)
        .stroke(Stroke::new(1.0, t.color.border))
        .corner_radius(10.0)
        .show(ui, |ui| {
            ui.spacing_mut().item_spacing = Vec2::ZERO;
            let viewport_width = crate::ui::viewport::root_viewport_width(ui.ctx());
            if uses_columns(viewport_width) {
                let (right_fraction, right_minimum) = right_track(viewport_width);
                let right = (ui.available_width() * right_fraction).max(right_minimum);
                let left = (ui.available_width() - right - 1.0).max(1.0);
                ui.horizontal_top(|ui| {
                    let (left_rect, _) = ui.allocate_exact_size(
                        Vec2::new(left, PANE_MIN_HEIGHT),
                        egui::Sense::hover(),
                    );
                    let mut left_ui = ui.new_child(
                        egui::UiBuilder::new()
                            .max_rect(left_rect)
                            .layout(Layout::top_down(Align::Min)),
                    );
                    left_pane(&mut left_ui, code, preview);

                    let divider = ui
                        .allocate_exact_size(Vec2::new(1.0, PANE_MIN_HEIGHT), egui::Sense::hover())
                        .0;
                    ui.painter().rect_filled(divider, 0.0, t.color.border);

                    let (right_rect, _) = ui.allocate_exact_size(
                        Vec2::new(right, PANE_MIN_HEIGHT),
                        egui::Sense::hover(),
                    );
                    let mut right_ui = ui.new_child(
                        egui::UiBuilder::new()
                            .max_rect(right_rect)
                            .layout(Layout::top_down(Align::Min)),
                    );
                    output = Some(right_pane(
                        &mut right_ui,
                        options_status,
                        status_ok,
                        options.take().expect("options pane renders once"),
                    ));
                });
            } else {
                left_pane(ui, code, preview);
                let divider = ui
                    .allocate_exact_size(Vec2::new(ui.available_width(), 1.0), egui::Sense::hover())
                    .0;
                ui.painter().rect_filled(divider, 0.0, t.color.border);
                output = Some(right_pane(
                    ui,
                    options_status,
                    status_ok,
                    options.take().expect("options pane renders once"),
                ));
            }
        });
    output.expect("schematic command options pane rendered")
}

fn uses_columns(viewport_width: f32) -> bool {
    viewport_width > SPLIT_VIEWPORT_BREAKPOINT
}

fn right_track(viewport_width: f32) -> (f32, f32) {
    if viewport_width <= COMPACT_COLUMNS_BREAKPOINT {
        (0.72 / 2.27, COMPACT_RIGHT_MIN_WIDTH)
    } else {
        (0.8 / 2.35, RIGHT_MIN_WIDTH)
    }
}

fn left_pane(ui: &mut Ui, code: &str, preview: SchematicCommandPreview<'_>) {
    Frame::new()
        .inner_margin(Margin::same(PANE_PADDING))
        .show(ui, |ui| {
            ui.set_min_height(PANE_MIN_HEIGHT - f32::from(PANE_PADDING) * 2.0);
            section_head(
                ui,
                &format!("{code} \u{00b7} live schematic preview"),
                &format!("{} grid", preview.grid),
                true,
            );
            schematic_canvas(ui, preview);
            ui.add_space(9.0);
            status_grid(ui, preview.electrical_outcome);
        });
}

fn right_pane<R>(
    ui: &mut Ui,
    status: &str,
    status_ok: bool,
    options: impl FnOnce(&mut Ui) -> R,
) -> R {
    let t = Tokens::get(ui.ctx());
    Frame::new()
        .fill(theme::mix(t.color.bg_inset, t.color.bg_panel, 0.94))
        .inner_margin(Margin::same(PANE_PADDING))
        .show(ui, |ui| {
            ui.set_min_height(PANE_MIN_HEIGHT - f32::from(PANE_PADDING) * 2.0);
            section_head(ui, "Placement / transform parameters", status, status_ok);
            egui::ScrollArea::vertical()
                .id_salt("schematic-command-options")
                .max_height(OPTIONS_HEIGHT)
                .min_scrolled_height(OPTIONS_HEIGHT)
                .auto_shrink([false, false])
                .show(ui, options)
                .inner
        })
        .inner
}

fn section_head(ui: &mut Ui, title: &str, status: &str, ok: bool) {
    let t = Tokens::get(ui.ctx());
    ui.horizontal_wrapped(|ui| {
        ui.spacing_mut().item_spacing.x = 10.0;
        ui.label(
            egui::RichText::new(title)
                .font(theme::sans(tokens::FS_1, FontWeight::SemiBold))
                .color(t.color.text),
        );
        // The mockup's section head is one wrapping flex row. Keeping both
        // children in this wrapping layout lets the status fall onto its own
        // line in the 240/270-point parameter tracks instead of allowing a
        // nested right-to-left layout to paint through the title.
        ui.label(
            egui::RichText::new(status)
                .font(theme::mono(tokens::FS_0, FontWeight::Medium))
                .color(if ok { t.color.ok } else { t.color.err }),
        );
    });
    ui.add_space(9.0);
}

fn schematic_canvas(ui: &mut Ui, preview: SchematicCommandPreview<'_>) {
    let t = Tokens::get(ui.ctx());
    let height = if crate::ui::viewport::root_viewport_width(ui.ctx()) <= SPLIT_VIEWPORT_BREAKPOINT
    {
        COMPACT_CANVAS_HEIGHT
    } else {
        CANVAS_HEIGHT
    };
    let (rect, response) = ui.allocate_exact_size(
        Vec2::new(ui.available_width(), height),
        egui::Sense::hover(),
    );
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::Image,
            true,
            format!(
                "Preview {} for {}",
                preview.electrical_outcome, preview.subject
            ),
        )
    });
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_description(format!("{}; {}", preview.subject, preview.location));
    });

    let painter = ui.painter_at(rect);
    painter.rect_filled(rect, 8.0, t.color.canvas_bg);
    for x in ((rect.left() as i32)..=(rect.right() as i32)).step_by(12) {
        for y in ((rect.top() as i32)..=(rect.bottom() as i32)).step_by(12) {
            painter.circle_filled(egui::pos2(x as f32, y as f32), 0.8, t.color.canvas_grid);
        }
    }
    painter.rect_stroke(
        rect,
        8.0,
        Stroke::new(1.0, t.color.border_strong),
        egui::StrokeKind::Inside,
    );

    // Exact percentages from `.schematic-command-*` in the restored mockup.
    let source = Rect::from_min_size(
        egui::pos2(
            rect.left() + rect.width() * 0.22,
            rect.top() + rect.height() * 0.34,
        ),
        Vec2::new(54.0, 62.0),
    );
    painter.add(egui::Shape::convex_polygon(
        vec![
            source.left_top(),
            source.left_bottom(),
            source.right_center(),
        ],
        egui::Color32::TRANSPARENT,
        Stroke::new(2.0, t.color.symbol),
    ));
    let target = Rect::from_min_size(
        egui::pos2(
            rect.right() - rect.width() * 0.22 - 56.0,
            rect.top() + rect.height() * 0.42,
        ),
        Vec2::new(56.0, 20.0),
    );
    painter.rect_stroke(
        target,
        0.0,
        Stroke::new(2.0, t.color.symbol),
        egui::StrokeKind::Middle,
    );

    let cross = egui::pos2(
        rect.left() + rect.width() * 0.51,
        rect.top() + rect.height() * 0.47,
    );
    painter.hline(
        rect.x_range().shrink(rect.width() * 0.08),
        cross.y,
        Stroke::new(2.0, t.color.wire),
    );
    painter.vline(
        cross.x,
        egui::Rangef::new(
            rect.top() + rect.height() * 0.20,
            rect.bottom() - rect.height() * 0.18,
        ),
        Stroke::new(2.0, t.color.wire),
    );
    painter.circle_filled(cross, 7.0, t.color.accent.gamma_multiply(0.14));
    painter.circle_stroke(cross, 7.0, Stroke::new(2.0, t.color.accent));

    let tooltip_width = (rect.width() * 0.44).clamp(120.0, 230.0);
    let tooltip = Rect::from_min_size(cross + Vec2::splat(12.0), Vec2::new(tooltip_width, 55.0))
        .intersect(rect.shrink(8.0));
    painter.rect_filled(tooltip, 6.0, t.color.bg_inset);
    painter.rect_stroke(
        tooltip,
        6.0,
        Stroke::new(1.0, t.color.accent.gamma_multiply(0.66)),
        egui::StrokeKind::Inside,
    );
    let subject = painter.layout(
        preview.subject.to_owned(),
        theme::sans(tokens::FS_0, FontWeight::SemiBold),
        t.color.text,
        (tooltip.width() - 18.0).max(1.0),
    );
    let location = painter.layout(
        preview.location.to_owned(),
        theme::mono(tokens::FS_0, FontWeight::Regular),
        t.color.text_dim,
        (tooltip.width() - 18.0).max(1.0),
    );
    painter.galley(tooltip.min + Vec2::new(9.0, 7.0), subject, t.color.text);
    painter.galley(
        tooltip.min + Vec2::new(9.0, 28.0),
        location,
        t.color.text_dim,
    );
}

fn status_grid(ui: &mut Ui, electrical_outcome: &str) {
    let values = [
        ("Electrical outcome", electrical_outcome),
        (
            "Checks",
            "connectivity \u{00b7} discipline \u{00b7} hierarchy",
        ),
        ("Commit", "stable IDs + one undo record"),
    ];
    let gap = 8.0;
    if uses_columns(crate::ui::viewport::root_viewport_width(ui.ctx())) {
        let width = ((ui.available_width() - gap * 2.0) / 3.0).max(1.0);
        ui.horizontal_top(|ui| {
            ui.spacing_mut().item_spacing.x = gap;
            for (label, value) in values {
                status_card(ui, label, value, width, 82.0);
            }
        });
    } else {
        ui.vertical(|ui| {
            ui.spacing_mut().item_spacing.y = gap;
            for (label, value) in values {
                status_card(ui, label, value, ui.available_width(), 70.0);
            }
        });
    }
}

fn status_card(ui: &mut Ui, label: &str, value: &str, width: f32, height: f32) {
    ui.allocate_ui_with_layout(
        Vec2::new(width, height),
        Layout::top_down(Align::Min),
        |ui| {
            let t = Tokens::get(ui.ctx());
            Frame::new()
                .fill(t.color.bg_panel)
                .stroke(Stroke::new(1.0, t.color.border))
                .corner_radius(7.0)
                .inner_margin(Margin::same(10))
                .show(ui, |ui| {
                    ui.set_min_width((width - 20.0).max(1.0));
                    ui.set_min_height((height - 20.0).max(1.0));
                    ui.label(
                        egui::RichText::new(label.to_ascii_uppercase())
                            .font(theme::mono(tokens::FS_0, FontWeight::SemiBold))
                            .color(t.color.text_dim),
                    );
                    ui.label(
                        egui::RichText::new(value)
                            .font(theme::sans(tokens::FS_1, FontWeight::SemiBold))
                            .color(t.color.text),
                    );
                });
        },
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mockup_tracks_use_the_exact_viewport_breakpoints() {
        assert!(!uses_columns(760.0));
        assert!(uses_columns(761.0));
        assert_eq!(right_track(980.0), (0.72 / 2.27, 240.0));
        assert_eq!(right_track(981.0), (0.8 / 2.35, 270.0));
    }
}
