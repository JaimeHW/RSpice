//! Persistent Results page geometry and pane chrome over retained display data.

use egui::{Align, Layout, Rect, RichText, Sense, Ui, UiBuilder, pos2, vec2};
use rspice_results::visualization_document::{Annotation, Measurement, PageLayout, PaneId};
use rspice_ui_kit::{
    panels::painted_label,
    theme::{self, FontWeight},
    tokens::{self, Tokens},
};

/// Allocate each retained pane once in a clipped slot, preserving document order.
pub fn page<R>(
    ui: &mut Ui,
    layout: PageLayout,
    pane_ids: impl ExactSizeIterator<Item = PaneId>,
    mut render_pane: impl FnMut(&mut Ui, usize) -> Option<R>,
) -> Option<R> {
    // Reserve the stage exactly once. Pane viewers render into clipped slots
    // inside that fixed rectangle, so their own minimum sizes and scroll areas
    // cannot enlarge the Results document or create recursive scroll growth.
    let size = finite_stage_size(ui.available_size());
    let (stage_rect, _) = ui.allocate_exact_size(size, Sense::hover());
    let slots = bounded_pane_slots(stage_rect, layout, pane_ids.len());
    let mut activated_pane_id = None;
    for (index, (pane_id, rect)) in pane_ids.zip(slots).enumerate() {
        let mut pane_ui = ui.new_child(
            UiBuilder::new()
                .id_salt(("persistent-result-pane-slot", pane_id.get()))
                .max_rect(rect)
                .layout(Layout::top_down(Align::Min)),
        );
        pane_ui.set_clip_rect(pane_ui.clip_rect().intersect(rect));
        let activated = render_pane(&mut pane_ui, index);
        activated_pane_id = activated.or(activated_pane_id);
    }
    activated_pane_id
}

const PANE_GAP: f32 = 1.0;

fn finite_stage_size(size: egui::Vec2) -> egui::Vec2 {
    vec2(finite_extent(size.x), finite_extent(size.y))
}

fn finite_extent(value: f32) -> f32 {
    if value.is_finite() {
        value.max(0.0)
    } else {
        0.0
    }
}

fn bounded_axis_cells(extent: f32, count: usize) -> (f32, f32) {
    let count = count.max(1);
    let total_gap = (PANE_GAP * count.saturating_sub(1) as f32).min(extent);
    ((extent - total_gap) / count as f32, total_gap)
}

fn bounded_pane_slots(stage: Rect, layout: PageLayout, pane_count: usize) -> Vec<Rect> {
    if pane_count == 0 {
        return Vec::new();
    }
    let stage = Rect::from_min_size(stage.min, finite_stage_size(stage.size()));
    let (columns, rows) = match layout {
        PageLayout::SinglePane => (1, 1),
        PageLayout::Rows => (1, pane_count),
        PageLayout::Columns => (pane_count, 1),
        PageLayout::Grid { columns } => {
            let columns = usize::from(columns.max(1)).min(pane_count);
            (columns, pane_count.div_ceil(columns))
        }
    };
    let (pane_width, horizontal_gap_budget) = bounded_axis_cells(stage.width(), columns);
    let (pane_height, vertical_gap_budget) = bounded_axis_cells(stage.height(), rows);
    let horizontal_gap = if columns > 1 {
        horizontal_gap_budget / (columns - 1) as f32
    } else {
        0.0
    };
    let vertical_gap = if rows > 1 {
        vertical_gap_budget / (rows - 1) as f32
    } else {
        0.0
    };

    (0..pane_count)
        .map(|index| {
            let column = index % columns;
            let row = index / columns;
            let min = pos2(
                stage.left() + column as f32 * (pane_width + horizontal_gap),
                stage.top() + row as f32 * (pane_height + vertical_gap),
            );
            Rect::from_min_size(min, vec2(pane_width, pane_height)).intersect(stage)
        })
        .collect()
}

fn bounded_inset(rect: Rect, requested: f32) -> Rect {
    let inset = requested
        .max(0.0)
        .min(rect.width() * 0.5)
        .min(rect.height() * 0.5);
    rect.shrink(inset)
}

/// Draw a pane frame around host-composed controls and its qualified viewer.
pub fn pane_frame<R>(
    ui: &mut Ui,
    pane_id: PaneId,
    is_active: bool,
    add_contents: impl FnOnce(&mut Ui) -> R,
) -> R {
    let t = Tokens::get(ui.ctx());
    let rect = ui.available_rect_before_wrap();
    ui.painter().rect_filled(rect, 0.0, t.color.canvas_bg);
    ui.painter().rect_stroke(
        rect,
        0.0,
        egui::Stroke::new(
            if is_active { 2.0 } else { 1.0 },
            if is_active {
                t.color.accent
            } else {
                t.color.border
            },
        ),
        egui::StrokeKind::Inside,
    );
    ui.scope_builder(
        UiBuilder::new()
            .id_salt(("persistent-result-pane", pane_id.get()))
            .max_rect(bounded_inset(rect, 1.0))
            .layout(Layout::top_down(Align::Min)),
        add_contents,
    )
    .inner
}

pub fn evidence_bar(
    ui: &mut Ui,
    pane_id: PaneId,
    measurements: &[Measurement],
    annotations: &[Annotation],
) {
    if measurements.is_empty() && annotations.is_empty() {
        return;
    }
    let t = Tokens::get(ui.ctx());
    egui::Frame::NONE
        .fill(t.color.bg_panel)
        .inner_margin(egui::Margin::symmetric(6, 3))
        .show(ui, |ui| {
            egui::ScrollArea::horizontal()
                .id_salt(("persistent-pane-evidence", pane_id.get()))
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        for measurement in measurements {
                            let value = measurement.value.map_or_else(
                                || "unevaluated".to_owned(),
                                |value| format!("{value:.6}"),
                            );
                            ui.label(
                                RichText::new(format!("{} = {value}", measurement.label))
                                    .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                                    .color(t.color.text),
                            )
                            .on_hover_text(
                                measurement
                                    .expression
                                    .as_deref()
                                    .unwrap_or("Retained document measurement"),
                            );
                        }
                        for annotation in annotations {
                            ui.label(
                                RichText::new(format!("Note: {}", annotation.text))
                                    .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                                    .color(t.color.text_dim),
                            )
                            .on_hover_text("Retained visualization annotation");
                        }
                    });
                });
        });
}

/// Retained entity counts shown in the pane header's detail.
#[derive(Clone, Copy)]
pub struct PaneCounts {
    pub axes: usize,
    pub traces: usize,
    pub cursors: usize,
    pub markers: usize,
    pub measurements: usize,
    pub annotations: usize,
}

/// Return an activation request; the host validates and applies the selection.
pub fn pane_header(
    ui: &mut Ui,
    display_title: &str,
    viewer_title: &str,
    is_active: bool,
    counts: PaneCounts,
) -> bool {
    let t = Tokens::get(ui.ctx());
    let (rect, response) = ui.allocate_exact_size(vec2(ui.available_width(), 25.0), Sense::click());
    ui.painter().rect_filled(
        rect,
        0.0,
        if is_active {
            t.color.accent_dim
        } else {
            t.color.bg_panel
        },
    );
    ui.painter().hline(
        rect.x_range(),
        rect.bottom(),
        egui::Stroke::new(1.0, t.color.border),
    );
    ui.scope_builder(
        UiBuilder::new()
            .max_rect(rect.shrink2(vec2(8.0, 3.0)))
            .layout(Layout::left_to_right(Align::Center)),
        |ui| {
            // Painted, not labels: a label over the tab takes the presses on its text.
            painted_label(
                ui,
                RichText::new(display_title)
                    .font(theme::sans(tokens::FS_1, FontWeight::Medium))
                    .color(t.color.text),
                egui::TextWrapMode::Extend,
            );
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                painted_label(
                    ui,
                    RichText::new(viewer_title)
                        .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                        .color(t.color.text_faint),
                    egui::TextWrapMode::Extend,
                );
            });
        },
    );
    theme::paint_focus_ring(ui, &response, rect);
    let tab_label = format!("{display_title}, {viewer_title}");
    response.widget_info(|| {
        egui::WidgetInfo::selected(
            egui::WidgetType::SelectableLabel,
            ui.is_enabled(),
            is_active,
            tab_label.clone(),
        )
    });
    response
        .on_hover_cursor(egui::CursorIcon::PointingHand)
        .on_hover_text(format!(
            "{} axes · {} traces · {} cursors · {} markers · {} measurements · {} annotations",
            counts.axes,
            counts.traces,
            counts.cursors,
            counts.markers,
            counts.measurements,
            counts.annotations
        ))
        .clicked()
}

/// State why Latest tracking cannot advance while preserving the retained view.
pub fn tracking_banner(ui: &mut Ui, reason: &str) {
    let t = Tokens::get(ui.ctx());
    let height = ui.text_style_height(&egui::TextStyle::Body) + 10.0;
    let (rect, response) =
        ui.allocate_exact_size(egui::vec2(ui.available_width(), height), Sense::hover());
    ui.painter().rect_filled(rect, 0.0, t.color.bg_inset);
    ui.painter().rect_filled(
        egui::Rect::from_min_size(rect.left_top(), egui::vec2(3.0, rect.height())),
        0.0,
        t.color.warn,
    );
    ui.painter().text(
        pos2(rect.left() + 13.0, rect.center().y),
        egui::Align2::LEFT_CENTER,
        reason,
        theme::sans(tokens::FS_0, FontWeight::Regular),
        t.color.text,
    );
    response.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Label, true, reason));
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_role(egui::accesskit::Role::Status);
        node.set_label(reason);
    });
}

pub fn unavailable_surface(ui: &mut Ui, title: &str, reason: &str) {
    let t = Tokens::get(ui.ctx());
    let rect = ui.available_rect_before_wrap();
    ui.painter().rect_filled(rect, 0.0, t.color.canvas_bg);
    ui.scope_builder(
        UiBuilder::new()
            .max_rect(rect.shrink(20.0))
            .layout(Layout::top_down_justified(Align::Center)),
        |ui| {
            ui.add_space((ui.available_height() * 0.35).max(0.0));
            ui.label(
                RichText::new(title)
                    .font(theme::sans(tokens::FS_2, FontWeight::SemiBold))
                    .color(t.color.text),
            );
            ui.label(
                RichText::new(reason)
                    .font(theme::sans(tokens::FS_1, FontWeight::Regular))
                    .color(t.color.text_dim),
            );
        },
    );
}

#[cfg(test)]
mod tests;
