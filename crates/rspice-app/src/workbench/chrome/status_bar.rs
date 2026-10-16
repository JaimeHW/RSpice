//! Compact application status with exact engineering context.

use egui::{Align, Frame, Layout, Panel, Sense, Ui, Vec2};

use crate::simulation::status::EngineAvailability;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{Problem, paint_problem_mark};
use crate::workbench::RSpiceApp;
use crate::workbench::app_state::diagnostic_store::listing::{DiagnosticCounts, listing_in_pass};

use super::super::commands::vocabulary::Command;
use super::super::layout::LayoutSpec;
use super::super::state::Workspace;

const STATUS_PADDING_X: f32 = 9.0;
const STATUS_MARK_GAP: f32 = 5.0;
const STATUS_FONT_SIZE: f32 = tokens::FS_1;

pub fn show(root: &mut Ui, app: &mut RSpiceApp, layout: LayoutSpec) {
    let ctx = root.ctx().clone();
    let ctx = &ctx;
    let t = Tokens::get(ctx);
    let visibility = status_visibility(ctx.content_rect().width());
    let shown = Panel::bottom("workbench.status_bar")
        .exact_size(layout.status_bar_height)
        .frame(Frame::new().fill(t.color.bg_panel))
        .show_separator_line(true)
        .show(root, |ui| {
            ui.spacing_mut().item_spacing = Vec2::ZERO;
            let bar_width = ui.available_width();
            let height = ui.available_height();
            let check_counts = listing_in_pass(&app.state, ctx.cumulative_pass_nr()).counts();
            let check = check_summary(check_counts);
            // The Results workspace reports the view its instrument shows:
            // the visible X interval in the coordinate segment, and that
            // interval's magnification in the zoom chip beside it.
            let results_view = (app.state.workbench.workspace == Workspace::Results)
                .then(|| {
                    crate::workbench::documents::result_document::active_shared_x_status(
                        &t,
                        &mut app.state,
                    )
                })
                .flatten();
            let engineering = engineering_context_summary(app, results_view.as_ref());
            let selection = selection_summary(app);
            let sheet_position = sheet_position_summary(&app.state);
            let zoom_command = if app.state.workbench.workspace == Workspace::Results {
                Command::ZoomFit
            } else {
                Command::ZoomOneToOne
            };
            let zoom = if let Some(view) = results_view.as_ref() {
                format_plot_zoom(view.zoom)
            } else if app.state.workbench.workspace == Workspace::Results {
                // A result sheet with nothing plotted still reports in the
                // instrument's unit rather than flipping the chip to a
                // canvas percentage it never uses.
                format_plot_zoom(1.0)
            } else {
                format!("{}%", (zoom_factor(app) * 100.0).round())
            };
            let availability = app.simulation_controller.engine_availability();
            let engine = simulation_engine_status(&app.state.simulation, &availability);
            let engine_mark = engine_status_mark(
                &availability,
                app.state.simulation.execution.has_active_execution(),
            );
            let platform = platform_label();
            let check_mark =
                check_problem(check_counts).map_or(StatusMark::None, StatusMark::Problem);
            let left_desired = optional_status_width(ui, check.as_deref(), check_mark)
                + if visibility.coordinates {
                    desired_status_width(ui, &engineering, StatusMark::None)
                } else {
                    0.0
                }
                + if visibility.selection {
                    desired_status_width(ui, &selection, StatusMark::None)
                } else {
                    0.0
                }
                + sheet_position.as_ref().map_or(0.0, |(text, _)| {
                    desired_status_width(ui, text, StatusMark::None)
                });
            let right_desired = desired_status_width(ui, &zoom, StatusMark::None)
                + optional_status_width(ui, engine.as_deref(), engine_mark)
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
                            optional_status_width(ui, check.as_deref(), check_mark),
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
                            sheet_position.as_ref().map_or(0.0, |(text, _)| {
                                desired_status_width(ui, text, StatusMark::None)
                            }),
                        ];
                        let widths = fit_status_widths(desired, groups.left);
                        // Rules go between items, so the last item on the left
                        // draws none: a rule there would face empty bar.
                        let last = if sheet_position.is_some() {
                            3
                        } else if visibility.selection {
                            2
                        } else if visibility.coordinates {
                            1
                        } else {
                            0
                        };
                        if let Some(check) = check.as_deref()
                            && status_item_sized(ui, check, check_mark, true, last != 0, widths[0])
                                .clicked()
                        {
                            Command::OpenProblems.execute(app);
                        }
                        if visibility.coordinates {
                            status_item_sized(
                                ui,
                                &engineering,
                                StatusMark::None,
                                false,
                                last != 1,
                                widths[1],
                            );
                        }
                        if visibility.selection {
                            status_item_sized(
                                ui,
                                &selection,
                                StatusMark::None,
                                false,
                                last != 2,
                                widths[2],
                            );
                        }
                        // The page a multi-sheet drawing is open at reads
                        // beside where the cursor is, because both answer
                        // where the reader is rather than what is selected.
                        if let Some((text, detail)) = sheet_position.as_ref() {
                            status_item_sized(ui, text, StatusMark::None, false, false, widths[3])
                                .on_hover_text(detail);
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
                                optional_status_width(ui, engine.as_deref(), engine_mark),
                                if visibility.platform {
                                    desired_status_width(ui, &platform, platform_mark)
                                } else {
                                    0.0
                                },
                            ],
                            groups.right,
                        );
                        // The right group lays out from the window edge, so
                        // each item's rule falls between it and the one to its
                        // right; the rightmost would draw on the edge itself.
                        if status_item_sized(
                            ui,
                            &zoom,
                            zoom_mark,
                            zoom_command.is_enabled(app),
                            false,
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
                        // The engine chip reports what the solver is doing; the
                        // dataset it produces is the one thing a reader wants
                        // next, so the chip is the route to it rather than a
                        // label about it. An idle, ready engine is nothing to
                        // report, so the bar shows no chip then.
                        if let Some(engine) = engine.as_deref() {
                        let can_retry = (availability == EngineAvailability::Restartable || availability.failure_reason().is_some())
                            && !app.state.simulation.execution.has_active_execution();
                        let can_open = availability == EngineAvailability::Ready
                            && app.state.project_lifecycle.is_open()
                            && app
                                .state
                                .simulation
                                .retained.newest_retained_result_run_index()
                                .is_some();
                        let engine_response = ui
                            .scope(|ui| {
                                if !can_open && !can_retry {
                                    ui.disable();
                                }
                                status_item_sized(ui, engine, engine_mark, true, true, widths[1])
                            })
                            .inner;
                        if can_retry {
                            let detail = availability.failure_reason().map_or_else(
                                || "Restart the simulation engine. This does not queue a simulation.".to_owned(),
                                |reason| format!("{reason}\nRetry engine startup. This does not queue a simulation."),
                            );
                            if engine_response.on_hover_text(detail).clicked()
                                && let Err(error) = app.simulation_controller.retry_engine_startup()
                            {
                                app.state.push_sim_message(crate::diagnostics::ConsoleMessage::error(
                                    format!("Engine startup could not be retried: {error}"),
                                ));
                            }
                        } else if can_open {
                            if engine_response
                                .on_hover_text(
                                    "Open the newest retained result dataset in Results",
                                )
                                .clicked()
                            {
                                crate::workbench::commands::result_navigation::open_newest_retained_run(app);
                            }
                        } else {
                            let detail = availability.failure_reason().unwrap_or(match &availability {
                                EngineAvailability::Starting => "The simulation engine is starting. Validated Run requests wait for initialization.",
                                EngineAvailability::Restartable => "The current execution is finishing its cancellation before the engine can restart.",
                                _ => "No run has retained a dataset yet, so there is nothing to open in Results.",
                            });
                            engine_response.on_disabled_hover_text(detail);
                        }
                        }
                        if visibility.platform {
                            status_item_sized(ui, &platform, platform_mark, false, true, widths[2]);
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

/// What the checks segment states, from the same list the Problems page
/// lists and the tab badge counts, or nothing.
///
/// With nothing open, any retained result describes the bootstrap placeholder
/// rather than the reader's work, and the coordinate segment already says no
/// project is loaded. Checks that have never run are not a finding either.
///
/// Three surfaces used to derive this independently — the status bar from
/// `dialogs.drc_results` or the netlist summary depending on which workspace
/// was open, the badge from the page, the page from four producers — and they
/// disagreed routinely. There is one list now, so there is one answer, and
/// the bar reads it once per pass for both the text and the mark.
fn check_summary(counts: DiagnosticCounts) -> Option<String> {
    if counts == DiagnosticCounts::default() {
        return None;
    }
    let mut label = format!(
        "Problems · {} error{} · {} advisor{}",
        counts.errors,
        if counts.errors == 1 { "" } else { "s" },
        counts.advisories,
        if counts.advisories == 1 { "y" } else { "ies" }
    );
    use std::fmt::Write as _;
    if counts.stale > 0 {
        let _ = write!(label, " · {} stale", counts.stale);
    }
    // Findings whose inputs cannot be read are not stale — nothing is known
    // to have moved — and saying so would send the reader to re-run checks
    // that cannot run.
    if counts.unavailable > 0 {
        let _ = write!(label, " · {} unavailable", counts.unavailable);
    }
    Some(label)
}

/// The mark in front of the checks segment: a failure while anything errors,
/// a warning while anything warns, is stale or cannot be evaluated, and
/// nothing otherwise, the way a compiler's problem count reads. A note or a
/// hint is counted among the advisories but raises no mark.
fn check_problem(counts: DiagnosticCounts) -> Option<Problem> {
    if counts.errors > 0 {
        Some(Problem::Failure)
    } else if counts.warnings > 0 || counts.stale > 0 || counts.unavailable > 0 {
        Some(Problem::Warning)
    } else {
        None
    }
}

fn simulation_progress_percent(progress: f64) -> u8 {
    (progress.clamp(0.0, 1.0) * 100.0).round() as u8
}

/// The engine segment, or nothing while the engine sits ready and idle.
fn simulation_engine_status(
    simulation: &crate::state::SimulationState,
    availability: &EngineAvailability,
) -> Option<String> {
    if availability.failure_reason().is_some() {
        return Some("Engine unavailable".to_owned());
    }
    if *availability == EngineAvailability::Starting && !simulation.cancellation_is_pending() {
        return Some("Engine starting".to_owned());
    }
    if !simulation.execution.has_active_execution() {
        return (*availability == EngineAvailability::Restartable)
            .then(|| "Engine stopped".to_owned());
    }
    let phase = match simulation.active_execution_lifecycle() {
        Some(crate::state::SimulationRunLifecycle::Preparing) => "preparing",
        Some(crate::state::SimulationRunLifecycle::Cancelling) => "stopping",
        Some(crate::state::SimulationRunLifecycle::Running)
        | Some(
            crate::state::SimulationRunLifecycle::LegacyUnknown
            | crate::state::SimulationRunLifecycle::Completed
            | crate::state::SimulationRunLifecycle::Failed
            | crate::state::SimulationRunLifecycle::Aborted
            | crate::state::SimulationRunLifecycle::Interrupted,
        )
        | None => "running",
    };
    let phase = if simulation.cancellation_is_pending() {
        "stopping"
    } else {
        phase
    };
    Some(format!(
        "Engine {phase} · {}%",
        simulation_progress_percent(simulation.execution.progress)
    ))
}

/// The mark in front of the engine segment: a failure when the engine cannot
/// run, the one live dot while a run is in progress, and nothing otherwise.
fn engine_status_mark(availability: &EngineAvailability, run_in_progress: bool) -> StatusMark {
    if availability.failure_reason().is_some() {
        StatusMark::Problem(Problem::Failure)
    } else if run_in_progress {
        StatusMark::Live
    } else {
        StatusMark::None
    }
}

fn schematic_cursor_summary(
    state: &crate::workbench::app_state::AppState,
    grid_x: f64,
    grid_y: f64,
) -> String {
    let grid = f64::from(state.schematic.document().grid_size.max(1));
    let sheet = crate::schematic::view::drawing_sheet::resolve_active_drawing_sheet(state);
    sheet.cursor_status(grid_x * grid, grid_y * grid)
}

fn empty_schematic_cursor_summary(state: &crate::workbench::app_state::AppState) -> String {
    let unit = crate::schematic::view::drawing_sheet::resolve_active_drawing_sheet(state)
        .format
        .display_unit;
    format!("x — · y — {}", unit.suffix())
}

/// The zoom chip's waveform form: whole numbers once the magnification is
/// deep, one decimal while it is shallow enough for the fraction to matter.
fn format_plot_zoom(zoom: f64) -> String {
    if zoom >= 10.0 {
        return format!("{zoom:.0}×");
    }
    let text = format!("{zoom:.1}");
    format!("{}×", text.strip_suffix(".0").unwrap_or(text.as_str()))
}

fn engineering_context_summary(
    app: &RSpiceApp,
    results_view: Option<&crate::workbench::documents::result_document::SharedXStatus>,
) -> String {
    if !app.state.project_lifecycle.is_open() {
        return "No project loaded".to_owned();
    }
    match app.state.workbench.workspace {
        Workspace::Design => {
            let view_type = app.state.workspace.content.active_view_type();
            app.state.ui.canvas_hover.map_or_else(
                || match view_type {
                    crate::state::ViewType::Schematic | crate::state::ViewType::Testbench => {
                        empty_schematic_cursor_summary(&app.state)
                    }
                    crate::state::ViewType::Layout => "x — · y — layout".to_owned(),
                    crate::state::ViewType::Symbol => {
                        format!(
                            "x — · y — · grid {}",
                            app.state.schematic.document().grid_size.max(1)
                        )
                    }
                    _ => format!(
                        "revision {}",
                        app.state.workspace.content.project.revision().get()
                    ),
                },
                |(x, y)| match view_type {
                    crate::state::ViewType::Schematic | crate::state::ViewType::Testbench => {
                        schematic_cursor_summary(&app.state, x, y)
                    }
                    crate::state::ViewType::Layout => {
                        app.state.pdk_config.layout_database_unit.map_or_else(
                            || format!("x {x:.0} · y {y:.0} DBU unavailable"),
                            |dbu| {
                                let policy =
                                    app.state.ui.preferences.quantity_presentation_policy();
                                format!(
                                    "x {} · y {}",
                                    policy.format_layout_coordinate(x, dbu),
                                    policy.format_layout_coordinate(y, dbu)
                                )
                            },
                        )
                    }
                    crate::state::ViewType::Symbol => format!(
                        "x {x:.0} · y {y:.0} · grid {}",
                        app.state.schematic.document().grid_size.max(1)
                    ),
                    _ => format!(
                        "revision {}",
                        app.state.workspace.content.project.revision().get()
                    ),
                },
            )
        }
        Workspace::Results => results_view_summary(app, results_view),
        Workspace::Project
        | Workspace::Simulate
        | Workspace::Stimulus
        | Workspace::Verify
        | Workspace::Models
        | Workspace::Netlist => {
            format!(
                "revision {}",
                app.state.workspace.content.project.revision().get()
            )
        }
    }
}

/// The Results coordinate segment.
///
/// The instrument bar already carries the A/B cursor readout, so this states
/// the interval the panes are showing. Sheets with no plotted interval — a
/// table, or a run that has not been opened yet — name the dataset instead.
fn results_view_summary(
    app: &RSpiceApp,
    results_view: Option<&crate::workbench::documents::result_document::SharedXStatus>,
) -> String {
    if let Some(view) = results_view {
        return view.span.clone();
    }
    app.state
        .simulation
        .view
        .active_run_idx
        .and_then(|index| app.state.simulation.retained.runs.get(index))
        .map_or_else(
            // The selection segment beside this one already reports that no
            // dataset is chosen. This segment answers a different question —
            // what interval am I reading — and has to say so in its own
            // words, or the bar prints the same sentence twice.
            || "No plotted interval".to_owned(),
            |run| format!("{} · immutable dataset", run.label),
        )
}

/// Where the reader is in a multi-sheet drawing: the active sheet's page
/// number over the number of sheets, and the sentence that expands it.
///
/// A single-sheet cell view has no position to report, so the segment is
/// absent rather than reading `p1/1`. The page number is the catalog's, so it
/// is the number the sheet strip's chip and the printed title block carry.
fn sheet_position_summary(
    state: &crate::workbench::app_state::AppState,
) -> Option<(String, String)> {
    let catalog = crate::schematic::view::sheet_visibility::multi_sheet_catalog(state)?;
    let active = catalog.active_sheet_id()?;
    let (page, count) = catalog.page_number_and_count(active)?;
    let name = catalog.find(active)?.name();
    Some((
        format!("p{page}/{count}"),
        format!("Sheet {page} of {count} \u{00b7} {name}"),
    ))
}

fn selection_summary(app: &RSpiceApp) -> String {
    if !app.state.project_lifecycle.is_open() {
        return "No engineering object selected".to_owned();
    }
    if app.state.workbench.workspace == Workspace::Results {
        return app
            .state
            .simulation
            .view
            .active_run_idx
            .and_then(|index| app.state.simulation.retained.runs.get(index))
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
    let selection = &app.state.schematic.session.editor.selection;
    if let Some(id) = selection.single_component() {
        app.state
            .schematic
            .document()
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

/// The canvas zoom behind the percentage chip. The Results workspace never
/// reaches here — its chip reports plot magnification instead.
fn zoom_factor(app: &RSpiceApp) -> f64 {
    if app.state.workbench.workspace == Workspace::Design
        && app.state.workspace.content.active_view_type() == crate::state::ViewType::Symbol
    {
        f64::from(app.state.ui.symbol.editor.zoom)
    } else {
        app.state.schematic.session.editor.zoom
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum StatusMark {
    None,
    /// A warning or a failure the reader has to notice.
    Problem(Problem),
    /// A run in progress: the bar's one live indicator.
    Live,
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
        StatusMark::Problem(_) => 11.0,
        StatusMark::Live => 6.0,
        StatusMark::Processor => 16.0,
    }
}

fn status_text_font() -> egui::FontId {
    theme::mono(STATUS_FONT_SIZE, FontWeight::Regular)
}

/// The width a segment that may be absent asks for: none when it is.
fn optional_status_width(ui: &egui::Ui, text: Option<&str>, mark: StatusMark) -> f32 {
    text.map_or(0.0, |text| desired_status_width(ui, text, mark))
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
    rule_after: bool,
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
    // A screen reader cannot see the mark, so it hears the severity first.
    let announced = match mark {
        StatusMark::Problem(problem) => problem.announce(text),
        StatusMark::None | StatusMark::Live | StatusMark::Processor => text.to_owned(),
    };
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            if interactive {
                egui::WidgetType::Button
            } else {
                egui::WidgetType::Label
            },
            ui.is_enabled(),
            &announced,
        )
    });

    if interactive && response.hovered() {
        ui.painter().rect_filled(rect, 0.0, t.color.bg_hover);
    }
    if rule_after {
        ui.painter().vline(
            rect.right(),
            rect.y_range(),
            egui::Stroke::new(1.0, t.color.border),
        );
    }

    let mut text_x = rect.left() + padding_x;
    if !matches!(mark, StatusMark::None) {
        let mark_rect = egui::Rect::from_center_size(
            egui::pos2(text_x + mark_size * 0.5, rect.center().y),
            Vec2::splat(mark_size),
        );
        match mark {
            StatusMark::Problem(problem) => {
                paint_problem_mark(ui.painter(), mark_rect, problem);
            }
            StatusMark::Live => {
                ui.painter()
                    .circle_filled(mark_rect.center(), 3.0, t.color.accent);
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

    /// The counts the bar states, read as the page reads them.
    fn counts(app: &RSpiceApp) -> DiagnosticCounts {
        crate::workbench::app_state::diagnostic_store::listing::listing(&app.state).counts()
    }

    /// A rule separates two items. The last item on the left used to draw
    /// one anyway, so a rule stood alone in front of the empty middle of the
    /// bar. Every rule now has an item's ink within a gutter on both sides.
    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn every_status_rule_stands_between_two_items() {
        let mut app = RSpiceApp::test_instance();
        app.state.project_lifecycle.authority.open_session();
        app.state.workbench.activate(Workspace::Design);
        let size = egui::vec2(1440.0, 900.0);
        let canvas = crate::ui::raster::render(size, |ui, _| {
            let layout = LayoutSpec::resolve(size.x, size.y, &app.state.workbench);
            show(ui, &mut app, layout);
        });
        let themed = egui::Context::default();
        crate::ui::Theme::default().apply(&themed);
        let border = Tokens::get(&themed).color.border;
        let y = size.y - 11.0;
        let row = (0..size.x as usize)
            .map(|x| {
                let rect = egui::Rect::from_min_size(egui::pos2(x as f32, y), egui::vec2(1.0, 1.0));
                canvas.pixels_in(rect).next().expect("pixel on the canvas")
            })
            .collect::<Vec<_>>();
        let fill = *row
            .iter()
            .max_by_key(|colour| row.iter().filter(|other| other == colour).count())
            .expect("a status row");
        let ink = |range: std::ops::Range<usize>| {
            row[range]
                .iter()
                .any(|colour| *colour != fill && *colour != border)
        };
        let rules = (1..row.len() - 1)
            .filter(|&x| row[x] == border)
            .collect::<Vec<_>>();
        assert!(!rules.is_empty(), "the bar draws its rules");
        for x in rules {
            assert!(
                ink(x.saturating_sub(14)..x) && ink(x + 1..(x + 15).min(row.len())),
                "the rule at x = {x} faces empty bar"
            );
        }
    }

    /// The segment states the same count wherever the reader is standing.
    ///
    /// It used to swap between the netlist summary and the schematic result
    /// depending on the open workspace, so the bar reported a different
    /// number of problems on the Netlist page than on the Design page in a
    /// session that had both.
    #[test]
    fn the_checks_segment_counts_every_producer_from_one_list() {
        let mut app = RSpiceApp::test_instance();
        app.state.project_lifecycle.authority.open_session();
        app.state.workbench.activate(Workspace::Netlist);
        let diagnostics = vec![
            crate::workbench::documents::netlist_document::Diagnostic::error("bad card"),
            crate::workbench::documents::netlist_document::Diagnostic::current(
                "rspice.test",
                "TEST-WARNING",
                crate::workbench::documents::netlist_document::DiagnosticSeverity::Warning,
                "implicit option",
            ),
        ];
        app.state.ui.netlist.diagnostics = std::sync::Arc::new(
            crate::workbench::documents::netlist_document::NetlistDiagnosticCollection::try_new(
                diagnostics,
                "",
            )
            .unwrap(),
        );

        assert_eq!(
            check_summary(counts(&app)).as_deref(),
            Some("Problems · 1 error · 1 advisory")
        );
        assert_eq!(check_problem(counts(&app)), Some(Problem::Failure));

        app.state.workbench.activate(Workspace::Design);
        assert_eq!(
            check_summary(counts(&app)).as_deref(),
            Some("Problems · 1 error · 1 advisory"),
            "the count cannot depend on which workspace is open"
        );
    }

    /// The checks segment is a problem count. A closed project, or checks
    /// that have never run, put nothing on the bar; zero and zero is a count
    /// with no mark; warnings and staleness earn a warning, errors a failure.
    #[test]
    fn the_checks_segment_marks_only_a_problem_and_is_absent_until_checks_run() {
        use rspice_design::drc::{DrcLocation, DrcResult, DrcViolation, DrcViolationType};

        let violation = |kind| {
            DrcViolation::new(
                1,
                kind,
                "fixture",
                DrcLocation::NetLabel {
                    name: "OUT".to_owned(),
                },
            )
        };
        let publish = |app: &mut RSpiceApp, result: DrcResult| {
            app.state
                .publish_active_design_check_result(result)
                .expect("the fixture publishes a current receipt");
        };
        let completed = |violation: Option<DrcViolation>| {
            let mut result = DrcResult::new();
            if let Some(violation) = violation {
                result.add_violation(violation);
            }
            result.completed = true;
            result
        };

        let mut app = RSpiceApp::test_instance();
        app.state.project_lifecycle.authority.close_project();
        assert_eq!(check_summary(counts(&app)), None);
        assert_eq!(check_problem(counts(&app)), None);

        app.state.project_lifecycle.authority.open_session();
        app.state.workbench.activate(Workspace::Design);
        assert_eq!(
            check_summary(counts(&app)),
            None,
            "checks that never ran say nothing"
        );
        assert_eq!(check_problem(counts(&app)), None);

        publish(&mut app, completed(None));
        assert_eq!(
            check_summary(counts(&app)),
            None,
            "zero and zero puts nothing on the bar"
        );
        assert_eq!(check_problem(counts(&app)), None);

        publish(
            &mut app,
            completed(Some(violation(DrcViolationType::UnconnectedPin))),
        );
        assert_eq!(check_problem(counts(&app)), Some(Problem::Warning));

        publish(
            &mut app,
            completed(Some(violation(DrcViolationType::ShortedOutputs))),
        );
        assert_eq!(check_problem(counts(&app)), Some(Problem::Failure));
        assert_eq!(
            check_summary(counts(&app)).as_deref(),
            Some("Problems · 1 error · 0 advisories")
        );

        // An edit under the evidence leaves the finding standing and faint:
        // the count moves to the stale column, which is neither an error nor
        // an advisory about the design in front of the reader.
        app.state
            .workspace
            .content
            .connectivity
            .policy
            .width_mismatch = crate::state::BundleWidthMismatchPolicy::ExplicitSliceOrExtend;
        assert_eq!(
            check_summary(counts(&app)).as_deref(),
            Some("Problems · 0 errors · 0 advisories · 1 stale")
        );
        assert_eq!(check_problem(counts(&app)), Some(Problem::Warning));
    }

    /// Findings whose inputs cannot be read are said to be that, not stale:
    /// nothing is known to have moved under them.
    #[test]
    fn findings_that_cannot_be_evaluated_are_not_called_stale() {
        let counts = DiagnosticCounts {
            unavailable: 2,
            ..DiagnosticCounts::default()
        };
        assert_eq!(
            check_summary(counts).as_deref(),
            Some("Problems · 0 errors · 0 advisories · 2 unavailable")
        );
        assert_eq!(check_problem(counts), Some(Problem::Warning));
    }

    /// A note or a hint is counted, but it is not a warning and raises no
    /// warning mark; it used to, because every advisory did.
    #[test]
    fn only_a_warning_raises_the_warning_mark() {
        let notes = DiagnosticCounts {
            advisories: 2,
            ..DiagnosticCounts::default()
        };
        assert_eq!(
            check_summary(notes).as_deref(),
            Some("Problems · 0 errors · 2 advisories")
        );
        assert_eq!(check_problem(notes), None);

        let warned = DiagnosticCounts {
            warnings: 1,
            ..notes
        };
        assert_eq!(check_problem(warned), Some(Problem::Warning));
    }

    /// The coordinate and selection segments sit side by side. They answer
    /// different questions, so neither may borrow the other's sentence — with
    /// no dataset open the bar used to print "No result dataset selected"
    /// twice in a row.
    #[test]
    fn adjacent_result_segments_never_print_the_same_sentence() {
        let mut app = RSpiceApp::test_instance();
        app.state.workbench.activate(Workspace::Results);
        app.state.project_lifecycle.authority.open_session();

        let coordinates = results_view_summary(&app, None);
        let selection = selection_summary(&app);
        assert_ne!(
            coordinates, selection,
            "two segments cannot report the same fact in the same words"
        );
        assert_eq!(coordinates, "No plotted interval");
        assert_eq!(selection, "No result dataset selected");
    }

    #[test]
    fn the_zoom_chip_drops_the_decimal_only_once_magnification_is_deep() {
        assert_eq!(format_plot_zoom(1.0), "1×");
        assert_eq!(format_plot_zoom(2.5), "2.5×");
        assert_eq!(format_plot_zoom(9.94), "9.9×");
        assert_eq!(format_plot_zoom(10.0), "10×");
        assert_eq!(format_plot_zoom(137.4), "137×");
    }

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
        assert_eq!(STATUS_FONT_SIZE, 12.0);
        assert_eq!(STATUS_FONT_SIZE, tokens::FS_1);
        assert_eq!(
            status_text_font(),
            theme::mono(tokens::FS_1, FontWeight::Regular)
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

    #[test]
    fn engine_status_follows_execution_lifecycle_not_worker_activity() {
        let mut simulation = crate::state::SimulationState::default();
        let identity = simulation
            .start_run()
            .execution_identity()
            .expect("current run has execution identity");
        simulation.execution.active_execution = Some(identity);
        simulation.execution.progress = 0.375;
        simulation.execution.is_running = false;

        assert_eq!(
            simulation_engine_status(&simulation, &EngineAvailability::Ready).as_deref(),
            Some("Engine preparing · 38%")
        );

        simulation.request_abort_active_run().unwrap();
        assert_eq!(
            simulation_engine_status(&simulation, &EngineAvailability::Ready).as_deref(),
            Some("Engine stopping · 38%")
        );
    }

    /// An idle engine that is ready is nothing to report, so the bar shows no
    /// engine segment at all; every other idle state still names itself.
    #[test]
    fn idle_engine_status_says_nothing_while_the_engine_is_ready() {
        let simulation = crate::state::SimulationState::default();
        for (availability, expected) in [
            (EngineAvailability::Ready, None),
            (EngineAvailability::Starting, Some("Engine starting")),
            (EngineAvailability::Restartable, Some("Engine stopped")),
            (
                EngineAvailability::Unavailable("worker module missing".into()),
                Some("Engine unavailable"),
            ),
        ] {
            assert_eq!(
                simulation_engine_status(&simulation, &availability).as_deref(),
                expected
            );
        }
    }

    /// The engine segment's one dot means a run is live. Unavailability is a
    /// failure mark, and no other state carries a mark at all.
    #[test]
    fn the_engine_mark_is_a_live_dot_only_while_a_run_is_in_progress() {
        let unavailable = EngineAvailability::Unavailable("worker module missing".into());
        assert_eq!(
            engine_status_mark(&unavailable, false),
            StatusMark::Problem(Problem::Failure)
        );
        assert_eq!(
            engine_status_mark(&unavailable, true),
            StatusMark::Problem(Problem::Failure)
        );
        assert_eq!(
            engine_status_mark(&EngineAvailability::Ready, true),
            StatusMark::Live
        );
        for availability in [
            EngineAvailability::Ready,
            EngineAvailability::Starting,
            EngineAvailability::Restartable,
        ] {
            assert_eq!(engine_status_mark(&availability, false), StatusMark::None);
        }
    }

    #[test]
    fn backend_startup_and_failure_do_not_advertise_running_progress() {
        let mut simulation = crate::state::SimulationState::default();
        let identity = simulation.start_run().execution_identity().unwrap();
        simulation.execution.active_execution = Some(identity);
        simulation.execution.progress = 0.375;
        assert_eq!(
            simulation_engine_status(&simulation, &EngineAvailability::Starting).as_deref(),
            Some("Engine starting")
        );
        assert_eq!(
            simulation_engine_status(
                &simulation,
                &EngineAvailability::Unavailable("worker initialization failed".into())
            )
            .as_deref(),
            Some("Engine unavailable")
        );
    }

    /// The page segment is the reader's position in a multi-sheet drawing. It
    /// follows activation because it projects the catalog rather than holding
    /// a copy, and it is absent below two sheets because `p1/1` is not a
    /// position.
    #[test]
    fn the_page_segment_appears_only_above_one_sheet_and_follows_activation() {
        use crate::state::{SheetDefinition, SheetPortPolicy, SheetTemplate};

        let mut app = RSpiceApp::test_instance();
        app.state.project_lifecycle.authority.open_session();
        let key = app
            .state
            .workspace
            .content
            .active_schematic_reference()
            .key();
        let first = app
            .state
            .workspace
            .content
            .design_management
            .bootstrap_for_cell_view(&key, "Input stage", [10])
            .expect("the fixture cell view takes a sheet catalog");
        assert_eq!(sheet_position_summary(&app.state), None);

        let catalog = app
            .state
            .workspace
            .content
            .design_management
            .sheet_catalog_mut(&key)
            .expect("the catalog was just bootstrapped");
        for (page, name) in [(2, "Output stage"), (3, "Supplies"), (4, "Test points")] {
            catalog
                .create_sheet(
                    SheetDefinition {
                        name: name.to_owned(),
                        template: SheetTemplate::AnalogSchematic,
                        port_policy: SheetPortPolicy::TypedOffSheetPorts,
                        explicit_page_number: Some(page),
                    },
                    None,
                )
                .expect("a further sheet");
        }
        catalog
            .set_active(first)
            .expect("the first sheet is active");
        let second = catalog.sheets()[1].id();
        assert_eq!(
            sheet_position_summary(&app.state).map(|(segment, _)| segment),
            Some("p1/4".to_owned())
        );

        crate::workbench::app::sheets::activate_sheet(&mut app.state, second)
            .expect("the second sheet activates");
        assert_eq!(
            sheet_position_summary(&app.state),
            Some((
                "p2/4".to_owned(),
                "Sheet 2 of 4 \u{00b7} Output stage".to_owned()
            ))
        );
    }

    #[test]
    fn schematic_coordinates_use_the_permanent_sheet_origin_and_status_context() {
        let mut state = crate::workbench::app_state::AppState::default();
        state.schematic.document_mut_for_test().grid_size = 10;
        assert_eq!(
            schematic_cursor_summary(&state, -14.0, -4.0),
            "x 0 · y 0 mm"
        );
        assert_eq!(
            schematic_cursor_summary(&state, -15.0, -4.0),
            "x -2.5 · y 0 mm · off sheet"
        );
    }
}
