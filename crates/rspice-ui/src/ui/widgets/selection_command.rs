//! Mockup-owned selected-object command composition.
//!
//! Object Properties and other selection transactions share one exact shell:
//! a 1.55fr/.8fr split, 250 pt schematic preview, three equal impact cards,
//! and a single command-options pane. The widget owns presentation only.

use egui::{Align, Frame, Layout, Margin, Rect, Stroke, Ui, Vec2};

use crate::state::Point;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};

/// The mockup media query is viewport-scoped, not container-scoped. The
/// 760-point workflow surface has 12-point body padding, so comparing this
/// breakpoint with `ui.available_width()` would make the desktop columns
/// unreachable even on a wide monitor.
const SPLIT_VIEWPORT_BREAKPOINT: f32 = 760.0;
const COMPACT_COLUMNS_BREAKPOINT: f32 = 980.0;
const RIGHT_MIN_WIDTH: f32 = 270.0;
const COMPACT_RIGHT_MIN_WIDTH: f32 = 240.0;
const PANE_PADDING: i8 = 14;
const CANVAS_HEIGHT: f32 = 250.0;
const COMPACT_CANVAS_HEIGHT: f32 = 220.0;
const PANE_MIN_HEIGHT: f32 = 414.0;
const OPTIONS_HEIGHT: f32 = 356.0;

#[derive(Debug, Clone)]
pub(crate) enum SelectionPreview {
    Component {
        label: String,
    },
    Bus {
        points: Vec<Point>,
        label: String,
    },
    BusTap {
        bus_point: Point,
        connection_point: Point,
        label: String,
    },
    NetLabel {
        position: Point,
        label: String,
    },
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct SelectionImpact<'a> {
    pub scope: &'a str,
    pub effect: &'a str,
    pub recovery: &'a str,
}

pub(crate) fn workflow_preview_status(ui: &mut Ui, ok: bool, headline: &str, detail: &str) {
    let t = Tokens::get(ui.ctx());
    let divider = ui
        .allocate_exact_size(Vec2::new(ui.available_width(), 1.0), egui::Sense::hover())
        .0;
    ui.painter()
        .rect_filled(divider, 0.0, t.color.border_strong);
    Frame::new()
        .inner_margin(Margin::symmetric(10, 8))
        .show(ui, |ui| {
            ui.set_min_width(ui.available_width());
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 6.0;
                let (dot, _) = ui.allocate_exact_size(Vec2::splat(10.0), egui::Sense::hover());
                ui.painter().circle_filled(
                    dot.center(),
                    3.0,
                    if ok { t.color.ok } else { t.color.err },
                );
                ui.label(
                    egui::RichText::new(headline)
                        .font(theme::sans(tokens::FS_0, FontWeight::SemiBold))
                        .color(t.color.text),
                );
            });
            Frame::new()
                .inner_margin(Margin {
                    left: 17,
                    right: 0,
                    top: 0,
                    bottom: 0,
                })
                .show(ui, |ui| {
                    ui.set_max_width(ui.available_width());
                    ui.add(
                        egui::Label::new(
                            egui::RichText::new(detail)
                                .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                                .color(t.color.text_faint),
                        )
                        .wrap(),
                    );
                });
        });
}

pub(crate) fn selection_command_workflow<R>(
    ui: &mut Ui,
    code: &str,
    preview: &SelectionPreview,
    impact: SelectionImpact<'_>,
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
            let viewport_width = ui.ctx().content_rect().width();
            if selection_workflow_uses_columns(viewport_width) {
                let (right_fraction, right_minimum) =
                    selection_workflow_right_track(viewport_width);
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
                    left_pane(&mut left_ui, code, preview, impact);
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
                left_pane(ui, code, preview, impact);
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
    output.expect("selection command options pane rendered")
}

fn selection_workflow_uses_columns(viewport_width: f32) -> bool {
    viewport_width > SPLIT_VIEWPORT_BREAKPOINT
}

fn selection_workflow_right_track(viewport_width: f32) -> (f32, f32) {
    if viewport_width <= COMPACT_COLUMNS_BREAKPOINT {
        (0.72 / 2.27, COMPACT_RIGHT_MIN_WIDTH)
    } else {
        (0.8 / 2.35, RIGHT_MIN_WIDTH)
    }
}

fn left_pane(ui: &mut Ui, code: &str, preview: &SelectionPreview, impact: SelectionImpact<'_>) {
    Frame::new()
        .inner_margin(Margin::same(PANE_PADDING))
        .show(ui, |ui| {
            ui.set_min_height(PANE_MIN_HEIGHT - f32::from(PANE_PADDING) * 2.0);
            section_head(
                ui,
                &format!("{code} · selected-object preview"),
                "SEL-06",
                true,
            );
            selection_canvas(ui, code, preview);
            ui.add_space(9.0);
            impact_grid(ui, impact);
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
            section_head(ui, "Command options", status, status_ok);
            egui::ScrollArea::vertical()
                .id_salt("selection-command-options")
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
    ui.horizontal(|ui| {
        ui.label(
            egui::RichText::new(title)
                .font(theme::sans(tokens::FS_1, FontWeight::SemiBold))
                .color(t.color.text),
        );
        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            ui.label(
                egui::RichText::new(status)
                    .font(theme::mono(tokens::FS_0, FontWeight::Medium))
                    .color(if ok { t.color.ok } else { t.color.err }),
            );
        });
    });
    ui.add_space(9.0);
}

fn selection_canvas(ui: &mut Ui, code: &str, preview: &SelectionPreview) {
    let t = Tokens::get(ui.ctx());
    let (rect, response) = ui.allocate_exact_size(
        Vec2::new(
            ui.available_width(),
            if ui.ctx().content_rect().width() <= SPLIT_VIEWPORT_BREAKPOINT {
                COMPACT_CANVAS_HEIGHT
            } else {
                CANVAS_HEIGHT
            },
        ),
        egui::Sense::hover(),
    );
    response.widget_info(|| {
        egui::WidgetInfo::labeled(egui::WidgetType::Image, true, "Selected object preview")
    });
    let accessible_description = match preview {
        SelectionPreview::Component { label }
        | SelectionPreview::Bus { label, .. }
        | SelectionPreview::BusTap { label, .. }
        | SelectionPreview::NetLabel { label, .. } => label,
    };
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_description(format!("{code}: {accessible_description}"));
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

    let content = rect.shrink2(Vec2::new(34.0, 38.0));
    let selection = match preview {
        SelectionPreview::Component { .. } => {
            // The mockup's selected-object canvas shows the selected instance
            // in circuit context: op-amp, load, capacitor, and retained wires.
            // Keep these percentages aligned with `.selection-object` and
            // `.selection-wire` in the restored design source.
            let opamp = Rect::from_min_size(
                egui::pos2(
                    rect.left() + rect.width() * 0.26,
                    rect.top() + rect.height() * 0.31,
                ),
                Vec2::new(64.0, 70.0),
            );
            painter.add(egui::Shape::convex_polygon(
                vec![opamp.left_top(), opamp.left_bottom(), opamp.right_center()],
                t.color.symbol.gamma_multiply(0.08),
                Stroke::new(2.0, t.color.symbol),
            ));
            let resistor = Rect::from_min_size(
                egui::pos2(
                    rect.left() + rect.width() * 0.54,
                    rect.top() + rect.height() * 0.41,
                ),
                Vec2::new(72.0, 20.0),
            );
            painter.rect_filled(resistor, 0.0, t.color.symbol.gamma_multiply(0.08));
            painter.rect_stroke(
                resistor,
                0.0,
                Stroke::new(2.0, t.color.symbol),
                egui::StrokeKind::Inside,
            );
            let capacitor = Rect::from_min_size(
                egui::pos2(
                    rect.left() + rect.width() * 0.67,
                    rect.top() + rect.height() * 0.58,
                ),
                Vec2::new(24.0, 42.0),
            );
            for x in [capacitor.left(), capacitor.right()] {
                painter.vline(x, capacitor.y_range(), Stroke::new(2.0, t.color.symbol));
            }
            painter.line_segment(
                [
                    egui::pos2(
                        rect.left() + rect.width() * 0.09,
                        rect.top() + rect.height() * 0.46,
                    ),
                    egui::pos2(
                        rect.left() + rect.width() * 0.85,
                        rect.top() + rect.height() * 0.46,
                    ),
                ],
                Stroke::new(2.0, t.color.wire),
            );
            painter.line_segment(
                [
                    egui::pos2(
                        rect.left() + rect.width() * 0.70,
                        rect.top() + rect.height() * 0.67,
                    ),
                    egui::pos2(
                        rect.left() + rect.width() * 0.70,
                        rect.top() + rect.height() * 0.87,
                    ),
                ],
                Stroke::new(2.0, t.color.wire),
            );
            Rect::from_min_size(
                egui::pos2(
                    rect.left() + rect.width() * 0.20,
                    rect.top() + rect.height() * 0.23,
                ),
                Vec2::new(rect.width() * 0.58, rect.height() * 0.58),
            )
        }
        SelectionPreview::Bus { points, label } => {
            let mapped = map_points(points, content);
            for pair in mapped.windows(2) {
                painter.line_segment([pair[0], pair[1]], Stroke::new(4.0, t.color.symbol));
                painter.line_segment([pair[0], pair[1]], Stroke::new(1.5, t.color.wire));
            }
            if let Some(point) = mapped.first() {
                painter.text(
                    *point + Vec2::new(8.0, -8.0),
                    egui::Align2::LEFT_BOTTOM,
                    label,
                    theme::mono(tokens::FS_0, FontWeight::Medium),
                    t.color.text,
                );
            }
            points_bounds(&mapped).expand(15.0)
        }
        SelectionPreview::BusTap {
            bus_point,
            connection_point,
            label,
        } => {
            let mapped = map_points(&[*bus_point, *connection_point], content);
            let source = mapped[0];
            let destination = mapped[1];
            painter.line_segment(
                [
                    egui::pos2(content.left(), source.y),
                    egui::pos2(content.right(), source.y),
                ],
                Stroke::new(4.0, t.color.symbol),
            );
            painter.line_segment([source, destination], Stroke::new(2.0, t.color.wire));
            painter.circle_filled(source, 4.0, t.color.accent);
            painter.circle_stroke(destination, 4.0, Stroke::new(1.5, t.color.accent));
            painter.text(
                destination + Vec2::new(8.0, -8.0),
                egui::Align2::LEFT_BOTTOM,
                label,
                theme::mono(tokens::FS_0, FontWeight::Medium),
                t.color.text,
            );
            points_bounds(&mapped).expand(18.0)
        }
        SelectionPreview::NetLabel { position, label } => {
            let anchor = map_points(&[*position], content)[0];
            painter.line_segment(
                [
                    egui::pos2(content.left(), anchor.y),
                    egui::pos2(content.right(), anchor.y),
                ],
                Stroke::new(2.0, t.color.wire),
            );
            let diamond = 5.0;
            painter.add(egui::Shape::convex_polygon(
                vec![
                    anchor + Vec2::new(0.0, -diamond),
                    anchor + Vec2::new(diamond, 0.0),
                    anchor + Vec2::new(0.0, diamond),
                    anchor + Vec2::new(-diamond, 0.0),
                ],
                t.color.canvas_bg,
                Stroke::new(1.5, t.color.accent),
            ));
            let galley = painter.layout_no_wrap(
                label.clone(),
                theme::mono(tokens::FS_1, FontWeight::Medium),
                t.color.text,
            );
            let text_pos = anchor + Vec2::new(10.0, -galley.size().y - 7.0);
            painter.galley(text_pos, galley.clone(), t.color.text);
            Rect::from_two_pos(anchor, text_pos + galley.size()).expand(10.0)
        }
    }
    .intersect(rect.shrink(8.0));
    paint_dashed_rect(ui, selection, Stroke::new(1.0, t.color.accent));
    paint_selection_handles(ui, selection, t.color.bg_inset, t.color.accent);

    let badge = Rect::from_min_size(
        egui::pos2(rect.right() - 62.0, rect.bottom() - 29.0),
        Vec2::new(52.0, 19.0),
    );
    painter.rect_filled(badge, 5.0, t.color.bg_inset);
    painter.rect_stroke(
        badge,
        5.0,
        Stroke::new(1.0, t.color.accent.gamma_multiply(0.7)),
        egui::StrokeKind::Inside,
    );
    painter.text(
        badge.center(),
        egui::Align2::CENTER_CENTER,
        code,
        theme::mono(tokens::FS_0, FontWeight::SemiBold),
        t.color.accent,
    );
}

fn map_points(points: &[Point], rect: Rect) -> Vec<egui::Pos2> {
    if points.is_empty() {
        return vec![rect.center()];
    }
    let min_x = points
        .iter()
        .map(|point| i64::from(point.x))
        .min()
        .unwrap_or_default();
    let max_x = points
        .iter()
        .map(|point| i64::from(point.x))
        .max()
        .unwrap_or_default();
    let min_y = points
        .iter()
        .map(|point| i64::from(point.y))
        .min()
        .unwrap_or_default();
    let max_y = points
        .iter()
        .map(|point| i64::from(point.y))
        .max()
        .unwrap_or_default();
    let span_x = (max_x - min_x) as f64;
    let span_y = (max_y - min_y) as f64;
    let scale_x = (span_x > 0.0).then(|| f64::from(rect.width()) / span_x);
    let scale_y = (span_y > 0.0).then(|| f64::from(rect.height()) / span_y);
    let scale = match (scale_x, scale_y) {
        (Some(x), Some(y)) => x.min(y),
        (Some(x), None) => x,
        (None, Some(y)) => y,
        (None, None) => 1.0,
    };
    let model_center_x = (min_x as f64 + max_x as f64) * 0.5;
    let model_center_y = (min_y as f64 + max_y as f64) * 0.5;
    points
        .iter()
        .map(|point| {
            egui::pos2(
                rect.center().x + ((f64::from(point.x) - model_center_x) * scale) as f32,
                rect.center().y + ((f64::from(point.y) - model_center_y) * scale) as f32,
            )
        })
        .collect()
}

fn points_bounds(points: &[egui::Pos2]) -> Rect {
    points
        .iter()
        .copied()
        .fold(Rect::NOTHING, |mut rect, point| {
            rect.extend_with(point);
            rect
        })
}

fn paint_dashed_rect(ui: &Ui, rect: Rect, stroke: Stroke) {
    for points in [
        [rect.left_top(), rect.right_top()],
        [rect.right_top(), rect.right_bottom()],
        [rect.right_bottom(), rect.left_bottom()],
        [rect.left_bottom(), rect.left_top()],
    ] {
        ui.painter()
            .extend(egui::Shape::dashed_line(&points, stroke, 4.0, 3.0));
    }
}

fn paint_selection_handles(ui: &Ui, rect: Rect, border: egui::Color32, fill: egui::Color32) {
    for center in [
        rect.left_top(),
        rect.right_top(),
        rect.left_bottom(),
        rect.right_bottom(),
    ] {
        let handle = Rect::from_center_size(center, Vec2::splat(7.0));
        ui.painter().rect_filled(handle, 0.0, fill);
        ui.painter().rect_stroke(
            handle,
            0.0,
            Stroke::new(1.0, border),
            egui::StrokeKind::Inside,
        );
    }
}

fn impact_grid(ui: &mut Ui, impact: SelectionImpact<'_>) {
    let values = [
        ("Scope", impact.scope),
        ("Effect", impact.effect),
        ("Recovery", impact.recovery),
    ];
    let gap = 8.0;
    if selection_workflow_uses_columns(ui.ctx().content_rect().width()) {
        let width = ((ui.available_width() - gap * 2.0) / 3.0).max(1.0);
        ui.horizontal_top(|ui| {
            ui.spacing_mut().item_spacing.x = gap;
            for (label, value) in values {
                impact_card(ui, label, value, width, 82.0);
            }
        });
    } else {
        ui.vertical(|ui| {
            ui.spacing_mut().item_spacing.y = gap;
            for (label, value) in values {
                impact_card(ui, label, value, ui.available_width(), 70.0);
            }
        });
    }
}

fn impact_card(ui: &mut Ui, label: &str, value: &str, width: f32, height: f32) {
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
    fn mockup_breakpoint_is_viewport_scoped_and_desktop_columns_are_reachable() {
        assert!(!selection_workflow_uses_columns(760.0));
        assert!(selection_workflow_uses_columns(761.0));
        assert!(selection_workflow_uses_columns(1_100.0));
        assert_eq!(
            selection_workflow_right_track(980.0),
            (0.72 / 2.27, COMPACT_RIGHT_MIN_WIDTH)
        );
        assert_eq!(
            selection_workflow_right_track(981.0),
            (0.8 / 2.35, RIGHT_MIN_WIDTH)
        );
    }
}
