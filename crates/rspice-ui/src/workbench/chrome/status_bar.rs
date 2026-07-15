//! Compact application status with exact engineering context.

use egui::{Align, Context, Frame, Layout, Sense, TopBottomPanel, Vec2};

use crate::common::RSpiceApp;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};

use super::super::commands::Command;
use super::super::design_system::WorkbenchIcon;
use super::super::layout::LayoutSpec;
use super::super::state::Workspace;

const STATUS_PADDING_X: f32 = 9.0;
const STATUS_MARK_SIZE: f32 = 14.0;
const STATUS_MARK_GAP: f32 = 5.0;

pub fn show(ctx: &Context, app: &mut RSpiceApp, layout: LayoutSpec) {
    let t = Tokens::get(ctx);
    let show_details = shows_detailed_status(ctx.content_rect().width());
    let shown = TopBottomPanel::bottom("workbench.status_bar")
        .exact_height(layout.status_bar_height)
        .frame(Frame::new().fill(t.color.bg_panel))
        .show_separator_line(true)
        .show(ctx, |ui| {
            ui.spacing_mut().item_spacing = Vec2::ZERO;
            let bar_width = ui.available_width();
            let (left_width, right_width) = status_group_widths(bar_width);
            let height = ui.available_height();
            ui.horizontal(|ui| {
                ui.allocate_ui_with_layout(
                    Vec2::new(left_width, height),
                    Layout::left_to_right(Align::Center),
                    |ui| {
                        ui.spacing_mut().item_spacing = Vec2::ZERO;
                        if status_item(
                            ui,
                            &check_summary(app),
                            StatusMark::Check(check_tone(app, &t)),
                            true,
                        )
                        .clicked()
                        {
                            Command::OpenProblems.execute(app);
                        }

                        if show_details {
                            status_label(ui, &engineering_context_summary(app));
                            status_label(ui, &selection_summary(app));
                        }
                    },
                );

                ui.allocate_ui_with_layout(
                    Vec2::new(right_width, height),
                    Layout::right_to_left(Align::Center),
                    |ui| {
                        ui.spacing_mut().item_spacing = Vec2::ZERO;
                        let zoom_command = if app.state.workbench.workspace == Workspace::Results {
                            Command::ZoomFit
                        } else {
                            Command::ZoomOneToOne
                        };
                        let zoom = format!("{}%", (zoom_factor(app) * 100.0).round());
                        if status_item(ui, &zoom, StatusMark::None, zoom_command.is_enabled(app))
                            .on_hover_text(if app.state.workbench.workspace == Workspace::Results {
                                "Fit the active result plot"
                            } else {
                                "Reset active canvas zoom to 100%"
                            })
                            .clicked()
                        {
                            zoom_command.execute(app);
                        }

                        let (engine, color) = if app.state.simulation.is_running {
                            (
                                format!(
                                    "Engine running · {}%",
                                    simulation_progress_percent(app.state.simulation.progress)
                                ),
                                t.color.accent,
                            )
                        } else {
                            ("Engine ready".to_owned(), t.color.ok)
                        };
                        let wash = if app.state.simulation.is_running {
                            t.color.accent_dim
                        } else {
                            semantic_wash(t.color.ok, t.mode)
                        };
                        let _ = status_item(ui, &engine, StatusMark::Dot { color, wash }, false);

                        if show_details {
                            let _ =
                                status_item(ui, &platform_label(), StatusMark::Processor, false);
                        }
                    },
                );
            });
        });
    ctx.accesskit_node_builder(shown.response.id, |node| {
        node.set_role(egui::accesskit::Role::ContentInfo);
        node.set_label("Application status");
    });
}

fn shows_detailed_status(viewport_width: f32) -> bool {
    viewport_width > 900.0
}

fn status_group_widths(available_width: f32) -> (f32, f32) {
    let left = (available_width * 0.5).floor();
    (left, available_width - left)
}

fn check_summary(app: &RSpiceApp) -> String {
    match &app.state.dialogs.drc_results {
        None => "Checks not run".to_owned(),
        Some(_)
            if app.state.dialogs.drc_checked_version != app.state.schematic.topology_version() =>
        {
            "Checks stale · run again".to_owned()
        }
        Some(result) => {
            let summary = result.summary();
            format!(
                "Schematic checks · {} error{} · {} advisor{}",
                summary.critical + summary.errors,
                if summary.critical + summary.errors == 1 {
                    ""
                } else {
                    "s"
                },
                summary.warnings,
                if summary.warnings == 1 { "y" } else { "ies" }
            )
        }
    }
}

fn check_tone(app: &RSpiceApp, tokens: &Tokens) -> egui::Color32 {
    match &app.state.dialogs.drc_results {
        None => tokens.color.warn,
        Some(_)
            if app.state.dialogs.drc_checked_version != app.state.schematic.topology_version() =>
        {
            tokens.color.warn
        }
        Some(result) => {
            let summary = result.summary();
            if summary.critical + summary.errors > 0 {
                tokens.color.err
            } else {
                tokens.color.ok
            }
        }
    }
}

fn simulation_progress_percent(progress: f64) -> u8 {
    (progress.clamp(0.0, 1.0) * 100.0).round() as u8
}

fn semantic_wash(color: egui::Color32, mode: crate::ui::tokens::Mode) -> egui::Color32 {
    let alpha = if mode == crate::ui::tokens::Mode::Light {
        23
    } else {
        28
    };
    egui::Color32::from_rgba_unmultiplied(color.r(), color.g(), color.b(), alpha)
}

fn engineering_context_summary(app: &RSpiceApp) -> String {
    if !app.state.project_lifecycle.project_open {
        return "No project loaded".to_owned();
    }
    match app.state.workbench.workspace {
        Workspace::Design => app.state.ui.canvas_hover.map_or_else(
            || {
                if matches!(
                    app.state.workspace.active_view_type(),
                    crate::state::ViewType::Schematic | crate::state::ViewType::Testbench
                ) {
                    "x — · y — mm".to_owned()
                } else {
                    "x — · y — grid".to_owned()
                }
            },
            |(x, y)| {
                if matches!(
                    app.state.workspace.active_view_type(),
                    crate::state::ViewType::Schematic | crate::state::ViewType::Testbench
                ) {
                    format!("x {x:.2} · y {y:.2} mm")
                } else {
                    format!("x {x:.2} · y {y:.2} grid")
                }
            },
        ),
        Workspace::Results => results_cursor_summary(app),
        Workspace::Project
        | Workspace::Simulate
        | Workspace::Verify
        | Workspace::Models
        | Workspace::Netlist => {
            format!("revision {}", app.state.workspace.project.revision().get())
        }
    }
}

fn results_cursor_summary(app: &RSpiceApp) -> String {
    let cursors = app.state.ui.results.cursors;
    match (cursors.a, cursors.b) {
        (Some(a), Some(b)) => format!(
            "A {} · B {} · Δ {}",
            crate::ui::plot::fmt_si(a, "", 3),
            crate::ui::plot::fmt_si(b, "", 3),
            crate::ui::plot::fmt_si(b - a, "", 3)
        ),
        (Some(a), None) => format!("A {} · B —", crate::ui::plot::fmt_si(a, "", 3)),
        (None, _) => app
            .state
            .simulation
            .active_run_idx
            .and_then(|index| app.state.simulation.runs.get(index))
            .map_or_else(
                || "No result dataset selected".to_owned(),
                |run| format!("{} · immutable dataset", run.label),
            ),
    }
}

fn selection_summary(app: &RSpiceApp) -> String {
    if !app.state.project_lifecycle.project_open {
        return "No engineering object selected".to_owned();
    }
    if app.state.workbench.workspace == Workspace::Results {
        return app
            .state
            .simulation
            .active_run_idx
            .and_then(|index| app.state.simulation.runs.get(index))
            .map_or_else(
                || "No result dataset selected".to_owned(),
                |run| {
                    let analysis_count = run.analyses.len();
                    format!(
                        "Run {} · {analysis_count} {}",
                        run.id,
                        if analysis_count == 1 {
                            "analysis"
                        } else {
                            "analyses"
                        }
                    )
                },
            );
    }
    if app.state.workbench.workspace != Workspace::Design {
        return app.state.workbench.workspace.label().to_owned();
    }
    let selection = &app.state.schematic.selection;
    if let Some(id) = selection.single_component() {
        app.state
            .schematic
            .components
            .iter()
            .find(|component| component.id == id)
            .map(|component| format!("{} · {}", component.name, component.value))
            .unwrap_or_else(|| "Selected object unavailable".to_owned())
    } else if selection.count() > 0 {
        format!("{} items selected", selection.count())
    } else {
        "No object selected".to_owned()
    }
}

fn zoom_factor(app: &RSpiceApp) -> f64 {
    if app.state.workbench.workspace == Workspace::Results {
        1.0
    } else if app.state.workbench.workspace == Workspace::Design
        && app.state.workspace.active_view_type() == crate::state::ViewType::Symbol
    {
        f64::from(app.state.ui.symbol.zoom)
    } else {
        app.state.schematic.zoom
    }
}

#[derive(Clone, Copy)]
enum StatusMark {
    None,
    Check(egui::Color32),
    Dot {
        color: egui::Color32,
        wash: egui::Color32,
    },
    Processor,
}

fn status_label(ui: &mut egui::Ui, text: &str) {
    let _ = status_item(ui, text, StatusMark::None, false);
}

fn status_item(
    ui: &mut egui::Ui,
    text: &str,
    mark: StatusMark,
    interactive: bool,
) -> egui::Response {
    let t = Tokens::get(ui.ctx());
    let font = theme::sans(tokens::FS_0, FontWeight::Regular);
    let full_galley = ui
        .painter()
        .layout_no_wrap(text.to_owned(), font.clone(), t.color.text_dim);
    let mark_width = if matches!(mark, StatusMark::None) {
        0.0
    } else {
        STATUS_MARK_SIZE + STATUS_MARK_GAP
    };
    let available_width = ui.available_width().max(0.0);
    let desired_width = STATUS_PADDING_X * 2.0 + mark_width + full_galley.size().x;
    let width = desired_width.min(available_width);
    let text_width = (width - STATUS_PADDING_X * 2.0 - mark_width).max(0.0);
    let painted_text = ellipsize_status_text(ui.painter(), text, &font, text_width);
    let galley = ui
        .painter()
        .layout_no_wrap(painted_text, font, t.color.text_dim);
    let sense = if interactive {
        Sense::click()
    } else {
        Sense::hover().difference(Sense::focusable_noninteractive())
    };
    let (rect, response) = ui.allocate_exact_size(Vec2::new(width, ui.available_height()), sense);
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            if interactive {
                egui::WidgetType::Button
            } else {
                egui::WidgetType::Label
            },
            ui.is_enabled(),
            text,
        )
    });

    if interactive && response.hovered() {
        ui.painter().rect_filled(rect, 0.0, t.color.bg_hover);
    }
    ui.painter().vline(
        rect.right(),
        rect.y_range(),
        egui::Stroke::new(1.0, t.color.border),
    );

    let mut text_x = rect.left() + STATUS_PADDING_X;
    if !matches!(mark, StatusMark::None) {
        let mark_rect = egui::Rect::from_center_size(
            egui::pos2(text_x + STATUS_MARK_SIZE * 0.5, rect.center().y),
            Vec2::splat(STATUS_MARK_SIZE),
        );
        match mark {
            StatusMark::Check(color) => {
                WorkbenchIcon::Check.paint(ui.painter(), mark_rect, color);
            }
            StatusMark::Dot { color, wash } => {
                ui.painter().circle_filled(mark_rect.center(), 5.0, wash);
                ui.painter().circle_filled(mark_rect.center(), 3.0, color);
            }
            StatusMark::Processor => paint_processor(ui.painter(), mark_rect, t.color.text_dim),
            StatusMark::None => {}
        }
        text_x += mark_width;
    }
    ui.painter().with_clip_rect(rect).galley(
        egui::pos2(text_x, rect.center().y - galley.size().y * 0.5),
        galley,
        t.color.text_dim,
    );
    if interactive {
        theme::paint_focus_ring(ui, &response, rect);
    }
    response
}

fn ellipsize_status_text(
    painter: &egui::Painter,
    value: &str,
    font: &egui::FontId,
    maximum_width: f32,
) -> String {
    if maximum_width <= 0.0 {
        return String::new();
    }
    if painter
        .layout_no_wrap(value.to_owned(), font.clone(), egui::Color32::WHITE)
        .size()
        .x
        <= maximum_width
    {
        return value.to_owned();
    }
    let ellipsis = "\u{2026}";
    if painter
        .layout_no_wrap(ellipsis.to_owned(), font.clone(), egui::Color32::WHITE)
        .size()
        .x
        > maximum_width
    {
        return String::new();
    }
    let characters = value.chars().collect::<Vec<_>>();
    let mut low = 0_usize;
    let mut high = characters.len();
    while low < high {
        let mid = (low + high).div_ceil(2);
        let candidate = characters[..mid]
            .iter()
            .copied()
            .chain(std::iter::once('\u{2026}'))
            .collect::<String>();
        if painter
            .layout_no_wrap(candidate, font.clone(), egui::Color32::WHITE)
            .size()
            .x
            <= maximum_width
        {
            low = mid;
        } else {
            high = mid - 1;
        }
    }
    characters[..low]
        .iter()
        .copied()
        .chain(std::iter::once('\u{2026}'))
        .collect()
}

fn paint_processor(painter: &egui::Painter, rect: egui::Rect, color: egui::Color32) {
    let body = rect.shrink(3.0);
    painter.rect_stroke(
        body,
        1.0,
        egui::Stroke::new(1.0, color),
        egui::StrokeKind::Inside,
    );
    painter.rect_stroke(
        body.shrink(2.0),
        0.0,
        egui::Stroke::new(1.0, color),
        egui::StrokeKind::Inside,
    );
    for offset in [4.0, 7.0, 10.0] {
        painter.hline(
            egui::Rangef::new(rect.left(), body.left()),
            rect.top() + offset,
            egui::Stroke::new(1.0, color),
        );
        painter.hline(
            egui::Rangef::new(body.right(), rect.right()),
            rect.top() + offset,
            egui::Stroke::new(1.0, color),
        );
    }
}

#[cfg(target_arch = "wasm32")]
fn platform_label() -> String {
    "Browser · WASM worker".to_owned()
}

#[cfg(not(target_arch = "wasm32"))]
fn platform_label() -> String {
    let threads = std::thread::available_parallelism().map_or(1, std::num::NonZeroUsize::get);
    format!(
        "Desktop · local {threads} thread{}",
        if threads == 1 { "" } else { "s" }
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dense_status_metadata_follows_the_mockup_nine_hundred_pixel_cutoff() {
        assert!(!shows_detailed_status(900.0));
        assert!(shows_detailed_status(901.0));
    }

    #[test]
    fn status_item_geometry_matches_the_mockup_contract() {
        assert_eq!(STATUS_PADDING_X, 9.0);
        assert_eq!(STATUS_MARK_GAP, 5.0);
        assert_eq!(tokens::FS_0, 11.0);
    }

    #[test]
    fn status_groups_keep_the_mockup_fifty_fifty_stress_partition() {
        assert_eq!(status_group_widths(1_440.0), (720.0, 720.0));
        assert_eq!(status_group_widths(901.0), (450.0, 451.0));
        assert_eq!(status_group_widths(390.0), (195.0, 195.0));
    }

    #[test]
    fn fractional_engine_progress_is_rendered_as_a_percentage() {
        assert_eq!(simulation_progress_percent(-1.0), 0);
        assert_eq!(simulation_progress_percent(0.375), 38);
        assert_eq!(simulation_progress_percent(1.5), 100);
    }
}
