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
const STATUS_MARK_GAP: f32 = 5.0;

pub fn show(ctx: &Context, app: &mut RSpiceApp, layout: LayoutSpec) {
    let t = Tokens::get(ctx);
    let visibility = status_visibility(ctx.content_rect().width());
    let shown = TopBottomPanel::bottom("workbench.status_bar")
        .exact_height(layout.status_bar_height)
        .frame(Frame::new().fill(t.color.bg_panel))
        .show_separator_line(true)
        .show(ctx, |ui| {
            ui.spacing_mut().item_spacing = Vec2::ZERO;
            let bar_width = ui.available_width();
            let height = ui.available_height();
            let check = check_summary(app);
            let engineering = engineering_context_summary(app);
            let selection = selection_summary(app);
            let zoom_command = if app.state.workbench.workspace == Workspace::Results {
                Command::ZoomFit
            } else {
                Command::ZoomOneToOne
            };
            let zoom = format!("{}%", (zoom_factor(app) * 100.0).round());
            let (engine, engine_color) = if app.state.simulation.is_running {
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
            let engine_wash = if app.state.simulation.is_running {
                t.color.accent_dim
            } else {
                semantic_wash(t.color.ok, t.mode)
            };
            let platform = platform_label();
            let check_mark = StatusMark::Check(check_tone(app, &t));
            let engine_mark = StatusMark::Dot {
                color: engine_color,
                wash: engine_wash,
            };
            let left_desired = desired_status_width(ui, &check, check_mark)
                + if visibility.coordinates {
                    desired_status_width(ui, &engineering, StatusMark::None)
                } else {
                    0.0
                }
                + if visibility.selection {
                    desired_status_width(ui, &selection, StatusMark::None)
                } else {
                    0.0
                };
            let right_desired = desired_status_width(ui, &zoom, StatusMark::None)
                + desired_status_width(ui, &engine, engine_mark)
                + if visibility.platform {
                    desired_status_width(ui, &platform, StatusMark::Processor)
                } else {
                    0.0
                };
            let groups = status_group_allocation(left_desired, right_desired, bar_width);
            ui.horizontal(|ui| {
                ui.allocate_ui_with_layout(
                    Vec2::new(groups.left, height),
                    Layout::left_to_right(Align::Center),
                    |ui| {
                        ui.spacing_mut().item_spacing = Vec2::ZERO;
                        let desired = [
                            desired_status_width(ui, &check, check_mark),
                            if visibility.coordinates {
                                desired_status_width(ui, &engineering, StatusMark::None)
                            } else {
                                0.0
                            },
                            if visibility.selection {
                                desired_status_width(ui, &selection, StatusMark::None)
                            } else {
                                0.0
                            },
                        ];
                        let widths = fit_status_widths(desired, groups.left);
                        if status_item_sized(ui, &check, check_mark, true, widths[0]).clicked() {
                            Command::OpenProblems.execute(app);
                        }
                        if visibility.coordinates {
                            status_item_sized(ui, &engineering, StatusMark::None, false, widths[1]);
                        }
                        if visibility.selection {
                            status_item_sized(ui, &selection, StatusMark::None, false, widths[2]);
                        }
                    },
                );

                ui.add_space(groups.gap);

                ui.allocate_ui_with_layout(
                    Vec2::new(groups.right, height),
                    Layout::right_to_left(Align::Center),
                    |ui| {
                        ui.spacing_mut().item_spacing = Vec2::ZERO;
                        let zoom_mark = StatusMark::None;
                        let platform_mark = StatusMark::Processor;
                        let widths = fit_status_widths(
                            [
                                desired_status_width(ui, &zoom, zoom_mark),
                                desired_status_width(ui, &engine, engine_mark),
                                if visibility.platform {
                                    desired_status_width(ui, &platform, platform_mark)
                                } else {
                                    0.0
                                },
                            ],
                            groups.right,
                        );
                        if status_item_sized(
                            ui,
                            &zoom,
                            zoom_mark,
                            zoom_command.is_enabled(app),
                            widths[0],
                        )
                        .on_hover_text(if app.state.workbench.workspace == Workspace::Results {
                            "Fit the active result plot"
                        } else {
                            "Reset active canvas zoom to 100%"
                        })
                        .clicked()
                        {
                            zoom_command.execute(app);
                        }
                        status_item_sized(ui, &engine, engine_mark, false, widths[1]);
                        if visibility.platform {
                            status_item_sized(ui, &platform, platform_mark, false, widths[2]);
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct StatusVisibility {
    coordinates: bool,
    selection: bool,
    platform: bool,
}

fn status_visibility(viewport_width: f32) -> StatusVisibility {
    let show_secondary = viewport_width > 900.0;
    StatusVisibility {
        coordinates: show_secondary,
        selection: show_secondary,
        platform: show_secondary,
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct StatusGroupAllocation {
    left: f32,
    right: f32,
    gap: f32,
}

/// Reproduce the mockup's `justify-content: space-between` status composition.
/// Each group keeps its intrinsic content width while space is available. Only
/// genuine overflow compresses both groups, in proportion to their content.
fn status_group_allocation(
    left_desired: f32,
    right_desired: f32,
    available_width: f32,
) -> StatusGroupAllocation {
    let available_width = available_width.max(0.0);
    let left_desired = left_desired.max(0.0);
    let right_desired = right_desired.max(0.0);
    let desired = left_desired + right_desired;
    if desired <= available_width {
        return StatusGroupAllocation {
            left: left_desired,
            right: right_desired,
            gap: available_width - desired,
        };
    }
    if desired <= f32::EPSILON {
        return StatusGroupAllocation {
            left: 0.0,
            right: 0.0,
            gap: available_width,
        };
    }
    let scale = available_width / desired;
    let left = left_desired * scale;
    StatusGroupAllocation {
        left,
        right: available_width - left,
        gap: 0.0,
    }
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
                    crate::state::ViewType::Layout
                ) {
                    "x — · y — layout".to_owned()
                } else {
                    "x — · y — grid".to_owned()
                }
            },
            |(x, y)| {
                if matches!(
                    app.state.workspace.active_view_type(),
                    crate::state::ViewType::Layout
                ) {
                    app.state.pdk_config.layout_database_unit.map_or_else(
                        || format!("x {x:.0} · y {y:.0} DBU unavailable"),
                        |dbu| {
                            let policy = app.state.ui.preferences.quantity_presentation_policy();
                            format!(
                                "x {} · y {}",
                                policy.format_layout_coordinate(x, dbu),
                                policy.format_layout_coordinate(y, dbu)
                            )
                        },
                    )
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

fn status_padding_x(ui: &egui::Ui) -> f32 {
    if ui.ctx().content_rect().width() <= 820.0 {
        7.0
    } else {
        STATUS_PADDING_X
    }
}

fn status_mark_size(mark: StatusMark) -> f32 {
    match mark {
        StatusMark::None => 0.0,
        StatusMark::Check(_) => 14.0,
        StatusMark::Dot { .. } => 6.0,
        StatusMark::Processor => 16.0,
    }
}

fn status_text_font() -> egui::FontId {
    theme::mono(tokens::FS_0, FontWeight::Regular)
}

fn desired_status_width(ui: &egui::Ui, text: &str, mark: StatusMark) -> f32 {
    let t = Tokens::get(ui.ctx());
    let text_width = ui
        .painter()
        .layout_no_wrap(text.to_owned(), status_text_font(), t.color.text_dim)
        .size()
        .x;
    let mark_width = if matches!(mark, StatusMark::None) {
        0.0
    } else {
        status_mark_size(mark) + STATUS_MARK_GAP
    };
    status_padding_x(ui) * 2.0 + mark_width + text_width
}

fn fit_status_widths<const N: usize>(desired: [f32; N], available: f32) -> [f32; N] {
    let total = desired.iter().sum::<f32>();
    if total <= available || total <= f32::EPSILON {
        return desired;
    }
    let scale = (available / total).clamp(0.0, 1.0);
    desired.map(|width| width * scale)
}

fn status_item_sized(
    ui: &mut egui::Ui,
    text: &str,
    mark: StatusMark,
    interactive: bool,
    width: f32,
) -> egui::Response {
    let t = Tokens::get(ui.ctx());
    let font = status_text_font();
    let padding_x = status_padding_x(ui);
    let mark_size = status_mark_size(mark);
    let mark_width = if matches!(mark, StatusMark::None) {
        0.0
    } else {
        mark_size + STATUS_MARK_GAP
    };
    let width = width.min(ui.available_width().max(0.0));
    let text_width = (width - padding_x * 2.0 - mark_width).max(0.0);
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

    let mut text_x = rect.left() + padding_x;
    if !matches!(mark, StatusMark::None) {
        let mark_rect = egui::Rect::from_center_size(
            egui::pos2(text_x + mark_size * 0.5, rect.center().y),
            Vec2::splat(mark_size),
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
        assert_eq!(
            status_visibility(900.0),
            StatusVisibility {
                coordinates: false,
                selection: false,
                platform: false,
            }
        );
        assert_eq!(
            status_visibility(901.0),
            StatusVisibility {
                coordinates: true,
                selection: true,
                platform: true,
            }
        );
    }

    #[test]
    fn status_item_geometry_matches_the_mockup_contract() {
        assert_eq!(STATUS_PADDING_X, 9.0);
        assert_eq!(STATUS_MARK_GAP, 5.0);
        assert_eq!(tokens::FS_0, 11.0);
        assert_eq!(
            status_text_font(),
            theme::mono(tokens::FS_0, FontWeight::Regular)
        );
    }

    #[test]
    fn status_groups_use_intrinsic_widths_and_space_between() {
        assert_eq!(
            status_group_allocation(420.0, 310.0, 1_440.0),
            StatusGroupAllocation {
                left: 420.0,
                right: 310.0,
                gap: 710.0,
            }
        );
    }

    #[test]
    fn status_groups_compress_proportionally_only_when_they_overlap() {
        let allocation = status_group_allocation(600.0, 400.0, 900.0);
        assert!((allocation.left - 540.0).abs() < f32::EPSILON);
        assert!((allocation.right - 360.0).abs() < f32::EPSILON);
        assert_eq!(allocation.gap, 0.0);
    }

    #[test]
    fn fractional_engine_progress_is_rendered_as_a_percentage() {
        assert_eq!(simulation_progress_percent(-1.0), 0);
        assert_eq!(simulation_progress_percent(0.375), 38);
        assert_eq!(simulation_progress_percent(1.5), 100);
    }
}
