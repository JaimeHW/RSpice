//! Viewer stage layout, source-qualified overlays, and exact evidence tables.

use super::{
    PANEL_HEADING_HEIGHT, bar_content_height,
    chrome::status_label,
    widgets::{paint_bottom_rule, paint_top_rule, panel_heading, table_header},
};
use egui::{
    Align, Align2, Frame, Grid, Layout, Margin, Rect, RichText, ScrollArea, Sense, Stroke, Ui,
    Vec2, vec2,
};
use rspice_results::{
    result_presentation::ResultViewer,
    studio_presentation::{VisualizationAnnotation, VisualizationMarker},
    viewer_catalog::ViewerDocumentDefinition,
};
use rspice_ui_kit::{
    panels::WorkbenchIcon,
    theme::{self, FontWeight},
    tokens::{self, Tokens},
};

const VIEWER_STAGE_HEADER_HEIGHT: f32 = 44.0;
const VIEWER_STAGE_HEADER_VERTICAL_MARGIN: f32 = 6.0;
const VIEWER_STAGE_STATUS_HEIGHT: f32 = 27.0;
const VIEWER_STAGE_STATUS_VERTICAL_MARGIN: f32 = 4.0;
const EXACT_DATA_CARD_PADDING: f32 = 12.0;
const EXACT_DATA_TABLE_HEIGHT: f32 = 102.0;
const EXACT_DATA_DOCK_HEIGHT: f32 =
    EXACT_DATA_CARD_PADDING * 2.0 + PANEL_HEADING_HEIGHT + EXACT_DATA_TABLE_HEIGHT;

/// The application resolves the source before drawing and coordinates the live renderer.
pub trait StageHost {
    fn source_label(&self) -> String;
    fn render(&mut self, ui: &mut Ui, viewer: ResultViewer);
    fn revision(&self) -> u64;
    fn exact_rows(&self) -> Vec<ExactSourceRow>;
}

pub fn viewer_stage(
    ui: &mut Ui,
    definition: Option<&ViewerDocumentDefinition>,
    availability: Result<ResultViewer, String>,
    host: &mut impl StageHost,
) {
    let t = Tokens::get(ui.ctx());
    let compatible = availability.is_ok();
    let header = Frame::NONE
        .fill(t.color.bg_app)
        .inner_margin(Margin::symmetric(9, 6))
        .show(ui, |ui| {
            ui.set_min_height(bar_content_height(
                VIEWER_STAGE_HEADER_HEIGHT,
                VIEWER_STAGE_HEADER_VERTICAL_MARGIN,
            ));
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.label(
                        RichText::new(definition.map_or("RESULT VIEWER", |meta| meta.domain))
                            .font(theme::mono(tokens::FS_0, FontWeight::Medium))
                            .color(t.color.text_faint),
                    );
                    ui.label(
                        RichText::new(definition.map_or("Unknown viewer", |meta| meta.title))
                            .font(theme::sans(tokens::FS_2, FontWeight::SemiBold))
                            .color(t.color.text),
                    );
                });
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    let source = host.source_label();
                    status_label(
                        ui,
                        &source,
                        if availability.is_ok() {
                            t.color.ok
                        } else {
                            t.color.warn
                        },
                    );
                });
            });
        });
    paint_bottom_rule(ui, header.response.rect, t.color.border_strong);

    let dock_height = EXACT_DATA_DOCK_HEIGHT;
    let plot_height = (ui.available_height() - dock_height - VIEWER_STAGE_STATUS_HEIGHT).max(80.0);
    ui.allocate_ui_with_layout(
        vec2(ui.available_width(), plot_height),
        Layout::top_down(Align::Min),
        |ui| match availability {
            Ok(viewer) => host.render(ui, viewer),
            Err(reason) => unavailable_viewer(ui, definition, &reason),
        },
    );
    viewer_stage_status(ui, host.revision(), compatible);
    exact_data_dock(ui, host.exact_rows());
}

pub fn paint_markers<'a>(
    ui: &Ui,
    well: Rect,
    (x_min, x_max): (f64, f64),
    markers: impl IntoIterator<Item = &'a VisualizationMarker>,
    annotations: impl IntoIterator<Item = &'a VisualizationAnnotation>,
) {
    if !x_min.is_finite() || !x_max.is_finite() || x_min >= x_max {
        return;
    }
    let t = Tokens::get(ui.ctx());
    let plot = well.shrink2(vec2(28.0, 22.0));
    for marker in markers {
        let fraction = ((marker.x - x_min) / (x_max - x_min)).clamp(0.0, 1.0) as f32;
        let x = egui::lerp(plot.x_range(), fraction);
        ui.painter()
            .vline(x, plot.y_range(), Stroke::new(1.0, t.color.accent));
        ui.painter().text(
            egui::pos2(x + 4.0, plot.top() + 3.0),
            egui::Align2::LEFT_TOP,
            &marker.label,
            theme::mono(tokens::FS_0, FontWeight::SemiBold),
            t.color.accent,
        );
    }
    for annotation in annotations {
        let fraction = ((annotation.x - x_min) / (x_max - x_min)).clamp(0.0, 1.0) as f32;
        let x = egui::lerp(plot.x_range(), fraction);
        let anchor = egui::pos2(x, plot.bottom() - 8.0);
        ui.painter().circle_filled(anchor, 3.0, t.color.info);
        ui.painter().text(
            anchor + vec2(5.0, -1.0),
            egui::Align2::LEFT_CENTER,
            format!("NOTE-{}", annotation.id),
            theme::mono(tokens::FS_0, FontWeight::Medium),
            t.color.info,
        );
    }
}

fn unavailable_viewer(ui: &mut Ui, definition: Option<&ViewerDocumentDefinition>, reason: &str) {
    let t = Tokens::get(ui.ctx());
    let rect = ui.available_rect_before_wrap();
    ui.painter().rect_filled(rect, 0.0, t.color.canvas_bg);
    ui.scope_builder(egui::UiBuilder::new().max_rect(rect.shrink(24.0)), |ui| {
        ui.centered_and_justified(|ui| {
            ui.vertical_centered(|ui| {
                let (icon, _) = ui.allocate_exact_size(Vec2::splat(28.0), Sense::hover());
                WorkbenchIcon::Warning.paint(ui.painter(), icon, t.color.warn);
                ui.label(
                    RichText::new(format!(
                        "{} unavailable",
                        definition.map_or("Viewer", |meta| meta.title)
                    ))
                    .font(theme::sans(tokens::FS_3, FontWeight::SemiBold))
                    .color(t.color.text),
                );
                ui.label(
                    RichText::new(reason)
                        .font(theme::sans(tokens::FS_1, FontWeight::Regular))
                        .color(t.color.text_dim),
                );
                ui.label(
                    RichText::new("No fallback viewer or fabricated data was substituted.")
                        .font(theme::mono(tokens::FS_0, FontWeight::Medium))
                        .color(t.color.text_faint),
                );
            });
        });
    });
}

fn viewer_stage_status(ui: &mut Ui, revision: u64, compatible: bool) {
    let t = Tokens::get(ui.ctx());
    let status = Frame::NONE
        .fill(t.color.bg_panel)
        .inner_margin(Margin::symmetric(8, 4))
        .show(ui, |ui| {
            ui.set_min_height(bar_content_height(
                VIEWER_STAGE_STATUS_HEIGHT,
                VIEWER_STAGE_STATUS_VERTICAL_MARGIN,
            ));
            ui.horizontal(|ui| {
                ui.label(
                    RichText::new(format!("Document  VIS-{:04} · revision {}", 1, revision))
                        .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                        .color(t.color.text_faint),
                );
                ui.separator();
                ui.label(
                    RichText::new("Source  immutable result samples")
                        .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                        .color(t.color.text_faint),
                );
                ui.separator();
                ui.label(
                    RichText::new("Interpolation  source exact on dock queries")
                        .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                        .color(t.color.text_faint),
                );
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    ui.label(
                        RichText::new(if compatible {
                            "COMPATIBLE-RUNTIME"
                        } else {
                            "VIEWER-UNAVAILABLE"
                        })
                        .font(theme::mono(tokens::FS_0, FontWeight::Medium))
                        .color(if compatible {
                            t.color.ok
                        } else {
                            t.color.warn
                        }),
                    );
                });
            });
        });
    paint_top_rule(ui, status.response.rect, t.color.border_strong);
}

fn exact_data_dock(ui: &mut Ui, rows: Vec<ExactSourceRow>) {
    let t = Tokens::get(ui.ctx());
    Frame::NONE
        .fill(t.color.bg_panel)
        .stroke(Stroke::new(1.0, t.color.border))
        .corner_radius(8.0)
        .inner_margin(Margin::same(12))
        .show(ui, |ui| {
            panel_heading(
                ui,
                "Exact-data dock",
                &format!("{} source rows · no display interpolation", rows.len()),
            );
            ui.allocate_ui_with_layout(
                vec2(ui.available_width(), EXACT_DATA_TABLE_HEIGHT),
                Layout::top_down(Align::Min),
                |ui| {
                    ScrollArea::both()
                        .id_salt("visualization.exact-data")
                        .show(ui, |ui| {
                            Grid::new("visualization.exact-data.grid")
                                .num_columns(5)
                                .striped(true)
                                .spacing(vec2(14.0, 5.0))
                                .show(ui, |ui| {
                                    table_header(ui, "Binding");
                                    table_header(ui, "Stable row");
                                    table_header(ui, "Typed coordinate");
                                    table_header(ui, "Exact f64 value");
                                    table_header(ui, "Origin");
                                    ui.end_row();
                                    if rows.is_empty() {
                                        ui.label(
                                            RichText::new("No exact source row is available")
                                                .color(t.color.warn),
                                        );
                                        for _ in 0..4 {
                                            ui.label("—");
                                        }
                                        ui.end_row();
                                    }
                                    for row in rows {
                                        ui.monospace(row.binding);
                                        ui.monospace(row.stable_row);
                                        ui.monospace(row.coordinate);
                                        ui.monospace(row.value);
                                        ui.label(row.origin);
                                        ui.end_row();
                                    }
                                });
                        });
                },
            );
        });
}

fn fixed_table_row<const N: usize>(
    ui: &mut Ui,
    fractions: [f32; N],
    cells: [&str; N],
    header: bool,
    minimum_width: f32,
) {
    let t = Tokens::get(ui.ctx());
    let height = if header { 27.0 } else { 28.0 };
    let width = ui.available_width().max(minimum_width);
    let (rect, response) = ui.allocate_exact_size(vec2(width, height), Sense::hover());
    if header {
        ui.painter().rect_filled(rect, 0.0, t.color.bg_panel_2);
    } else if response.hovered() {
        ui.painter().rect_filled(rect, 0.0, t.color.bg_hover);
    }
    ui.painter().hline(
        rect.x_range(),
        rect.bottom() - 0.5,
        Stroke::new(1.0, t.color.border),
    );
    let font = if header {
        theme::sans(tokens::FS_0, FontWeight::Medium)
    } else {
        theme::mono(tokens::FS_0, FontWeight::Regular)
    };
    let color = if header {
        t.color.text_faint
    } else {
        t.color.text_dim
    };
    let mut left = rect.left();
    for index in 0..N {
        let right = if index + 1 == N {
            rect.right()
        } else {
            left + rect.width() * fractions[index]
        };
        let cell = Rect::from_min_max(
            egui::pos2(left, rect.top()),
            egui::pos2(right, rect.bottom()),
        );
        if index + 1 < N {
            ui.painter().vline(
                right - 0.5,
                rect.y_range(),
                Stroke::new(1.0, t.color.border),
            );
        }
        ui.painter().with_clip_rect(cell).text(
            egui::pos2(cell.left() + 8.0, cell.center().y),
            Align2::LEFT_CENTER,
            cells[index],
            font.clone(),
            color,
        );
        left = right;
    }
}

pub fn result_entity_table(ui: &mut Ui, rows: &[ResultEntityRow]) {
    const FRACTIONS: [f32; 4] = [0.23, 0.16, 0.37, 0.24];
    ScrollArea::both()
        .id_salt("visualization.result-entities")
        .max_height(196.0)
        .show(ui, |ui| {
            fixed_table_row(
                ui,
                FRACTIONS,
                ["IDENTITY", "TYPE", "BINDING / DEFINITION", "STATE"],
                true,
                520.0,
            );
            if rows.is_empty() {
                fixed_table_row(
                    ui,
                    FRACTIONS,
                    ["—", "none", "No versioned result entities", "empty"],
                    false,
                    520.0,
                );
            }
            for row in rows {
                fixed_table_row(
                    ui,
                    FRACTIONS,
                    [&row.identity, row.kind, &row.binding, &row.state],
                    false,
                    520.0,
                );
            }
        });
}

pub struct ExactSourceRow {
    pub binding: String,
    pub stable_row: String,
    pub coordinate: String,
    pub value: String,
    pub origin: String,
}

pub struct ResultEntityRow {
    pub identity: String,
    pub kind: &'static str,
    pub binding: String,
    pub state: String,
}
