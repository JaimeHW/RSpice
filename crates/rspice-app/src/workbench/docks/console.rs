//! Unified console, problems, measurements, and task history.

mod log_anchor;
#[cfg(test)]
mod measurement_rows_tests;
mod problems;
mod transcript;

use problems::{ProblemCount, active_problem_count, problems};

use egui::{Align, Grid, Layout, ScrollArea, Sense, Ui, Vec2};

use crate::diagnostics::LogSeverity;
use crate::state::SimulationState;
use crate::ui::plot::fmt_si;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{Problem, paint_problem_mark};
use crate::workbench::{AppState, RSpiceApp};

use super::super::design_system::WorkbenchIcon;
use super::super::layout::LayoutSpec;
use super::super::state::{ConsoleChannel, ConsolePage};
use crate::workbench::commands::vocabulary::Command;

const CONSOLE_HEADER_HEIGHT: f32 = 31.0;
const CONSOLE_TAB_HEIGHT: f32 = 26.0;
const CONSOLE_TOUCH_HEADER_HEIGHT: f32 = 44.0;
const CONSOLE_ACTION_SIZE: f32 = 27.0;
const CONSOLE_ACTION_MARGIN_RIGHT: f32 = 3.0;
const CONSOLE_BODY_PADDING_TOP: f32 = 7.0;
const CONSOLE_BODY_PADDING_BOTTOM: f32 = 7.0;
const CONSOLE_BODY_PADDING_X: f32 = 10.0;
const CONSOLE_ROW_MIN_HEIGHT: f32 = 16.0;
const CONSOLE_COLUMN_GAP: f32 = 9.0;
/// Space between the tabs and the Console page's controls beside them.
const CONSOLE_TOOLBAR_GAP: f32 = 12.0;
/// Side of a row's leading problem mark, and the gutter every row reserves for
/// it so a marked row's columns line up with an unmarked row's. Both read from
/// the kit, so a console row's mark is the mark every other panel sets.
use crate::ui::widgets::{
    PROBLEM_MARK_ADVANCE as CONSOLE_MARK_GUTTER, PROBLEM_MARK_SIDE as CONSOLE_MARK_SIDE,
};
const CONSOLE_FONT_SIZE: f32 = tokens::FS_1;

pub fn show(ui: &mut Ui, app: &mut RSpiceApp, layout: LayoutSpec) {
    transcript::collect_save(ui.ctx(), app);
    ui.spacing_mut().item_spacing.y = 0.0;
    header(ui, app, layout);
    if !layout.show_console_body {
        return;
    }
    let page = app.state.workbench.console_page;
    let body_rect = ui.available_rect_before_wrap();
    ui.painter()
        .rect_filled(body_rect, 0.0, Tokens::get(ui.ctx()).color.bg_inset);
    let content_rect = egui::Rect::from_min_max(
        egui::pos2(
            body_rect.left() + CONSOLE_BODY_PADDING_X,
            body_rect.top() + CONSOLE_BODY_PADDING_TOP,
        ),
        egui::pos2(
            body_rect.right() - CONSOLE_BODY_PADDING_X,
            body_rect.bottom() - CONSOLE_BODY_PADDING_BOTTOM,
        ),
    );
    let body = ui.scope_builder(
        egui::UiBuilder::new()
            .max_rect(content_rect)
            .layout(Layout::top_down(Align::Min)),
        |ui| match page {
            ConsolePage::Console => transcript::console(ui, &mut app.state),
            ConsolePage::Problems => problems(ui, &mut app.state),
            ConsolePage::Measurements => measurements(ui, app),
            ConsolePage::TaskLog => task_log(ui, app),
        },
    );
    ui.ctx().accesskit_node_builder(body.response.id, |node| {
        node.set_role(egui::accesskit::Role::TabPanel);
        node.set_label(page.label());
    });
}

fn header(ui: &mut Ui, app: &mut RSpiceApp, layout: LayoutSpec) {
    let t = Tokens::get(ui.ctx());
    let problems = active_problem_count(&app.state, ui.ctx().cumulative_pass_nr());
    let touch_targets = Tokens::get(ui.ctx()).metrics.ctl_h >= 44.0;
    let header_height = if touch_targets {
        CONSOLE_TOUCH_HEADER_HEIGHT
    } else {
        CONSOLE_HEADER_HEIGHT
    };
    let console_page = app.state.workbench.console_page == ConsolePage::Console;
    let clear_shown = clear_is_offered(
        layout,
        app.state.workbench.console_page,
        app.state.workbench.console_view.channel,
    );
    let header = ui.allocate_ui_with_layout(
        Vec2::new(ui.available_width(), header_height),
        Layout::left_to_right(Align::Center),
        |ui| {
            ui.spacing_mut().item_spacing = Vec2::ZERO;
            let action_width = console_trailing_actions_width(layout, clear_shown, touch_targets);
            egui::ScrollArea::horizontal()
                .id_salt("workbench.console.tabs.scroll")
                .max_width((ui.available_width() - action_width).max(header_height))
                .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysHidden)
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        console_tabs(
                            ui,
                            app,
                            problems,
                            if touch_targets {
                                header_height
                            } else {
                                CONSOLE_TAB_HEIGHT
                            },
                        );
                        // A compact header has no band beside the tabs, so the
                        // page's controls scroll with them.
                        if layout.compact_shell && console_page {
                            ui.add_space(CONSOLE_TOOLBAR_GAP);
                            transcript::toolbar(ui, &mut app.state);
                        }
                    });
                });
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                let control_size = if touch_targets {
                    CONSOLE_TOUCH_HEADER_HEIGHT
                } else {
                    CONSOLE_ACTION_SIZE
                };
                ui.add_space(CONSOLE_ACTION_MARGIN_RIGHT);
                if console_action(
                    ui,
                    ConsoleActionIcon::Workbench(WorkbenchIcon::ChevronDown),
                    if layout.show_console_body {
                        "Collapse console"
                    } else {
                        "Expand console"
                    },
                    false,
                    control_size,
                )
                .clicked()
                {
                    if layout.show_console_body {
                        app.state.workbench.console_maximized = false;
                    }
                    Command::ToggleConsole.execute(app);
                }
                if !layout.compact_shell {
                    if !touch_targets {
                        ui.add_space(CONSOLE_ACTION_MARGIN_RIGHT);
                        if console_action(
                            ui,
                            ConsoleActionIcon::Workbench(WorkbenchIcon::Focus),
                            if app.state.workbench.console_maximized {
                                "Restore console"
                            } else {
                                "Maximize console"
                            },
                            app.state.workbench.console_maximized,
                            control_size,
                        )
                        .clicked()
                        {
                            Command::ToggleConsoleMaximized.execute(app);
                        }
                    }
                    // The command decides what the visible page has to clear
                    // and does the clearing; this control contributes only the
                    // wording for the page in front of the user.
                    if clear_shown {
                        let clear = console_clear_action(
                            app.state.workbench.console_page,
                            Command::ClearConsole.is_enabled(app),
                        );
                        ui.add_space(CONSOLE_ACTION_MARGIN_RIGHT);
                        let response = ui
                            .add_enabled_ui(clear.enabled, |ui| {
                                console_action(
                                    ui,
                                    ConsoleActionIcon::Clear,
                                    clear.label,
                                    false,
                                    control_size,
                                )
                            })
                            .inner;
                        if response.clicked() {
                            Command::ClearConsole.execute(app);
                        }
                    }
                    if console_page {
                        // The rest of the band, left to right after the tabs.
                        // Clipped to it, so a narrow dock cuts the find field
                        // short rather than painting it under the actions.
                        ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                            ui.set_clip_rect(ui.max_rect().intersect(ui.clip_rect()));
                            ui.add_space(CONSOLE_TOOLBAR_GAP);
                            transcript::toolbar(ui, &mut app.state);
                        });
                    }
                }
            });
        },
    );
    if layout.show_console_body {
        ui.painter().hline(
            header.response.rect.x_range(),
            header.response.rect.bottom(),
            egui::Stroke::new(1.0, t.color.border),
        );
    }
}

/// Whether the header offers Clear. Every page but a run's log shows it, the
/// pages with nothing to clear disabled with the reason; a run's log is a
/// record, so it has no Clear at all.
fn clear_is_offered(layout: LayoutSpec, page: ConsolePage, channel: ConsoleChannel) -> bool {
    !layout.compact_shell
        && !(page == ConsolePage::Console && matches!(channel, ConsoleChannel::Run(_)))
}

fn console_trailing_actions_width(
    layout: LayoutSpec,
    clear_shown: bool,
    touch_targets: bool,
) -> f32 {
    let control_size = if touch_targets {
        CONSOLE_TOUCH_HEADER_HEIGHT
    } else {
        CONSOLE_ACTION_SIZE
    };
    let mut actions = 1_usize;
    if !layout.compact_shell {
        actions += usize::from(!touch_targets);
    }
    actions += usize::from(clear_shown);
    actions as f32 * (control_size + CONSOLE_ACTION_MARGIN_RIGHT)
}

fn console_tabs(ui: &mut Ui, app: &mut RSpiceApp, problems: ProblemCount, height: f32) {
    let mut requested = None;
    let mut focus = None;
    let mut ids = Vec::with_capacity(ConsolePage::ALL.len());
    let tabs = ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing = Vec2::ZERO;
        for (index, page) in ConsolePage::ALL.into_iter().enumerate() {
            let count = match page {
                ConsolePage::Problems => problems.total(),
                ConsolePage::Measurements => active_measurement_count(&app.state.simulation),
                ConsolePage::TaskLog => app.state.simulation.retained.runs.len(),
                ConsolePage::Console => 0,
            };
            // The badge has room for one number. Which severities that number
            // is made of is said in the tab's name and under the pointer, so
            // the strip can never imply that twelve advisories are twelve
            // errors.
            let breakdown =
                (page == ConsolePage::Problems && count > 0).then(|| problems.announcement());
            let response = console_tab(
                ui,
                page,
                app.state.workbench.console_page == page,
                count,
                breakdown,
                height,
            );
            ids.push(response.id);
            if response.clicked() {
                requested = Some(page);
                focus = Some(index);
            }
            if response.has_focus() {
                let key = ui.input_mut(|input| {
                    if input.consume_key(egui::Modifiers::NONE, egui::Key::ArrowLeft) {
                        Some(-1_i32)
                    } else if input.consume_key(egui::Modifiers::NONE, egui::Key::ArrowRight) {
                        Some(1_i32)
                    } else if input.consume_key(egui::Modifiers::NONE, egui::Key::Home) {
                        Some(i32::MIN)
                    } else if input.consume_key(egui::Modifiers::NONE, egui::Key::End) {
                        Some(i32::MAX)
                    } else {
                        None
                    }
                });
                focus = key.map(|direction| {
                    if direction == i32::MIN {
                        0
                    } else if direction == i32::MAX {
                        ConsolePage::ALL.len() - 1
                    } else {
                        (index as i32 + direction).rem_euclid(ConsolePage::ALL.len() as i32)
                            as usize
                    }
                });
            }
        }
    });
    ui.ctx().accesskit_node_builder(tabs.response.id, |node| {
        node.set_role(egui::accesskit::Role::TabList);
        node.set_label("Console pages");
    });
    if let Some(index) = focus {
        requested = Some(ConsolePage::ALL[index]);
        ui.memory_mut(|memory| memory.request_focus(ids[index]));
    }
    if let Some(page) = requested {
        app.state.workbench.console_page = page;
    }
}

fn console_tab(
    ui: &mut Ui,
    page: ConsolePage,
    selected: bool,
    count: usize,
    breakdown: Option<String>,
    height: f32,
) -> egui::Response {
    let t = Tokens::get(ui.ctx());
    let font = theme::sans(CONSOLE_FONT_SIZE, FontWeight::Regular);
    let galley = ui
        .painter()
        .layout_no_wrap(page.label().to_owned(), font, t.color.text_dim);
    let count_width = if count > 0 {
        let digits = count.to_string().len() as f32;
        (digits * 6.5 + 8.0).max(15.0) + 4.0
    } else {
        0.0
    };
    let width = 20.0 + galley.size().x + count_width;
    let mut sense = Sense::click();
    if !selected {
        sense = sense.difference(Sense::focusable_noninteractive());
    }
    let (rect, response) = ui.allocate_exact_size(Vec2::new(width, height), sense);
    response.widget_info(|| {
        egui::WidgetInfo::selected(
            egui::WidgetType::SelectableLabel,
            ui.is_enabled(),
            selected,
            page.label(),
        )
    });
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_role(egui::accesskit::Role::Tab);
        node.set_selected(selected);
        match breakdown.as_deref() {
            Some(breakdown) => node.set_label(format!("{}, {breakdown}", page.label())),
            None if count > 0 => node.set_label(format!("{}, {count}", page.label())),
            None => {}
        }
    });
    if response.hovered() {
        ui.painter().rect_filled(rect, 0.0, t.color.bg_hover);
    }
    let text_color = if selected {
        t.color.text
    } else {
        t.color.text_dim
    };
    ui.painter().galley(
        egui::pos2(rect.left() + 10.0, rect.center().y - galley.size().y * 0.5),
        galley,
        text_color,
    );
    if count > 0 {
        let digits = count.to_string();
        let badge_width = (digits.len() as f32 * 6.5 + 8.0).max(15.0);
        let badge = egui::Rect::from_center_size(
            egui::pos2(rect.right() - 10.0 - badge_width * 0.5, rect.center().y),
            Vec2::new(badge_width, 15.0),
        );
        // A count is a number, not a verdict: every tab's pill is neutral.
        ui.painter().rect_filled(badge, 7.0, t.color.bg_active);
        ui.painter().text(
            badge.center(),
            egui::Align2::CENTER_CENTER,
            digits,
            theme::mono(CONSOLE_FONT_SIZE, FontWeight::Medium),
            t.color.text_dim,
        );
    }
    if selected {
        ui.painter().rect_filled(
            egui::Rect::from_min_max(
                egui::pos2(rect.left() + 8.0, rect.bottom() - 2.0),
                egui::pos2(rect.right() - 8.0, rect.bottom()),
            ),
            0.0,
            t.color.accent,
        );
    }
    theme::paint_focus_ring(ui, &response, rect);
    match breakdown {
        Some(breakdown) => response.on_hover_text(breakdown),
        None => response,
    }
}

enum ConsoleActionIcon {
    Clear,
    Workbench(WorkbenchIcon),
}

fn console_action(
    ui: &mut Ui,
    icon: ConsoleActionIcon,
    label: &str,
    selected: bool,
    size: f32,
) -> egui::Response {
    let t = Tokens::get(ui.ctx());
    let (rect, response) = ui.allocate_exact_size(Vec2::splat(size), Sense::click());
    response.widget_info(|| {
        egui::WidgetInfo::selected(egui::WidgetType::Button, ui.is_enabled(), selected, label)
    });
    if selected {
        ui.painter().rect_filled(rect, t.radius, t.color.accent_dim);
    } else if response.hovered() {
        ui.painter().rect_filled(rect, t.radius, t.color.bg_hover);
    }
    let icon_rect = egui::Rect::from_center_size(rect.center(), Vec2::splat(16.0));
    let icon_color = if ui.is_enabled() {
        t.color.text_dim
    } else {
        t.color.text_faint
    };
    match icon {
        ConsoleActionIcon::Workbench(icon) => {
            icon.paint(ui.painter(), icon_rect, icon_color);
        }
        ConsoleActionIcon::Clear => paint_trash(ui.painter(), icon_rect, icon_color),
    }
    theme::paint_focus_ring(ui, &response, rect);
    response.on_hover_text(label)
}

fn paint_trash(painter: &egui::Painter, rect: egui::Rect, color: egui::Color32) {
    let stroke = egui::Stroke::new(1.2, color);
    let body = egui::Rect::from_min_max(
        egui::pos2(rect.left() + 4.0, rect.top() + 5.0),
        egui::pos2(rect.right() - 4.0, rect.bottom() - 2.0),
    );
    painter.rect_stroke(body, 1.0, stroke, egui::StrokeKind::Inside);
    painter.hline(
        egui::Rangef::new(rect.left() + 2.0, rect.right() - 2.0),
        rect.top() + 4.0,
        stroke,
    );
    painter.hline(
        egui::Rangef::new(rect.left() + 6.0, rect.right() - 6.0),
        rect.top() + 2.0,
        stroke,
    );
}

fn active_measurement_count(simulation: &SimulationState) -> usize {
    simulation.active_run().map_or(0, |run| {
        run.analyses
            .iter()
            .map(|analysis| analysis.measurements.len())
            .sum()
    })
}

/// Join the columns a painted console row shows into the one string a screen
/// reader announces, skipping the columns a continuation row leaves blank.
fn row_announcement(columns: &[&str]) -> String {
    columns
        .iter()
        .filter(|column| !column.is_empty())
        .copied()
        .collect::<Vec<_>>()
        .join(" · ")
}

/// [`row_announcement`], with the severity a row's mark shows said first,
/// because a screen reader cannot see the mark.
fn announced_row(problem: Option<Problem>, columns: &[&str]) -> String {
    let label = row_announcement(columns);
    match problem {
        Some(problem) => problem.announce(&label),
        None => label,
    }
}

/// Paint a row's leading problem mark into the gutter every row reserves, so
/// marked and unmarked rows keep their columns aligned. A row with no problem
/// leaves the gutter empty.
fn paint_row_mark(ui: &Ui, left: f32, top: f32, problem: Option<Problem>) {
    if let Some(problem) = problem {
        paint_problem_mark(
            ui.painter(),
            egui::Rect::from_center_size(
                egui::pos2(
                    left + CONSOLE_MARK_SIDE * 0.5,
                    top + CONSOLE_ROW_MIN_HEIGHT * 0.5,
                ),
                Vec2::splat(CONSOLE_MARK_SIDE),
            ),
            problem,
        );
    }
}

fn measurements(ui: &mut Ui, app: &mut RSpiceApp) {
    let specifications = active_specifications(app);
    let rows = active_measurement_rows(app, &specifications);
    let run_label = app
        .state
        .simulation
        .active_run()
        .map(|run| run.label.clone());
    let mut add_measurement = false;
    ScrollArea::vertical()
        .id_salt("workbench.measurements")
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                if let Some(label) = run_label.as_deref() {
                    ui.label(
                        egui::RichText::new(label)
                            .font(theme::mono(CONSOLE_FONT_SIZE, FontWeight::Regular))
                            .color(Tokens::get(ui.ctx()).color.text_dim),
                    );
                } else {
                    muted(ui, "No retained result selected");
                }
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    add_measurement = ui
                        .add_enabled(
                            app.state.simulation.active_analysis().is_some(),
                            egui::Button::new("+ Add measurement…"),
                        )
                        .on_disabled_hover_text(
                            "Select a retained result analysis before creating a measurement",
                        )
                        .clicked();
                });
            });
            ui.separator();
            if rows.is_empty() {
                if app.state.simulation.active_run().is_some() {
                    muted(ui, "This dataset has no computed measurements.");
                } else {
                    muted(
                        ui,
                        "Select a retained result dataset to inspect measurements.",
                    );
                }
                return;
            }
            Grid::new("workbench.measurements.table")
                .num_columns(7)
                .striped(true)
                .spacing(Vec2::new(16.0, 4.0))
                .show(ui, |ui| {
                    measurement_header(ui, "Measurement");
                    measurement_header(ui, "Expression");
                    measurement_header(ui, "Value");
                    measurement_header(ui, "Spec");
                    measurement_header(ui, "Margin");
                    measurement_header(ui, "Worst point");
                    measurement_header(ui, "Status");
                    ui.end_row();
                    for row in &rows {
                        measurement_cell(ui, &row.name, false);
                        measurement_cell(ui, &row.expression, true);
                        measurement_cell(ui, &row.value, true);
                        measurement_cell(ui, &row.specification, true);
                        measurement_cell(ui, &row.margin, true);
                        measurement_cell(ui, &row.worst_point, false);
                        measurement_status_cell(ui, row.status, row.failed);
                        ui.end_row();
                    }
                });
        });
    // Add measurement draws over the result viewer, so it opens there.
    if add_measurement {
        app.state
            .workbench
            .activate(crate::workbench::state::Workspace::Results);
        Command::MeasurementLibrary.execute(app);
    }
}

fn task_log(ui: &mut Ui, app: &mut RSpiceApp) {
    let mut open_log = None;
    ScrollArea::vertical()
        .id_salt("workbench.task_log")
        .show(ui, |ui| {
            if app.state.simulation.retained.runs.is_empty() {
                muted(
                    ui,
                    "Queued, active, cancelled, and completed simulation tasks will appear here.",
                );
            }
            // Both columns fit their widest text, measured in the row's own
            // font: the status over every state a run can be in, so a row
            // does not shift when its run finishes, and the label over the
            // runs listed.
            let columns = TaskColumns {
                status: widest_status_width(ui),
                label: app
                    .state
                    .simulation
                    .retained
                    .runs
                    .iter()
                    .map(|run| task_text_width(ui, &run.label, FontWeight::Medium))
                    .fold(0.0, f32::max),
            };
            for run in &app.state.simulation.retained.runs {
                let active = app.state.simulation.execution.active_execution
                    == run.execution_identity()
                    && !run.lifecycle.is_terminal();
                let (status, problem) = run_lifecycle_presentation(run.lifecycle);
                let progress = if active {
                    format!(
                        " · {}%",
                        simulation_progress_percent(app.state.simulation.execution.progress)
                    )
                } else {
                    String::new()
                };
                let revision = run.prepared_receipt().map_or_else(
                    || "revision unavailable".to_owned(),
                    |receipt| format!("revision {}", receipt.project_revision().get()),
                );
                let analyses = run.analyses.len();
                let row = issue_row(
                    ui,
                    columns,
                    problem,
                    status,
                    &run.label,
                    &format!(
                        "{analyses} {} · {:.3} s{progress} · {revision}",
                        if analyses == 1 { "analysis" } else { "analyses" },
                        run.elapsed_time
                    ),
                );
                row.context_menu(|ui| {
                    if ui.button("Open run log").clicked() {
                        open_log = Some(run.id);
                        ui.close();
                    }
                });
            }
        });
    if let Some(run) = open_log {
        crate::workbench::commands::result_navigation::open_run_log(app, run);
    }
}

#[derive(Debug, Clone)]
struct MeasurementTableRow {
    name: String,
    expression: String,
    value: String,
    specification: String,
    margin: String,
    worst_point: String,
    status: &'static str,
    /// The measurement failed its contract or could not be evaluated. Only
    /// the status cell of a failed row is coloured; a pass is neutral.
    failed: bool,
}

fn active_specifications(app: &RSpiceApp) -> Vec<crate::state::SpecEntry> {
    if let Some(receipt) = app
        .state
        .simulation
        .active_run()
        .and_then(crate::state::SimulationRun::prepared_receipt)
    {
        return receipt
            .specifications()
            .iter()
            .map(|specification| specification.entry().clone())
            .collect();
    }
    let active = app
        .state
        .sim_setup
        .analysis_plan
        .as_ref()
        .and_then(|plan| app.state.workspace.content.plan_data(plan.id()))
        .map(|payload| payload.specs.as_slice())
        .unwrap_or(app.state.workspace.content.specs.as_slice());
    active.to_vec()
}

fn active_measurement_rows(
    app: &RSpiceApp,
    specifications: &[crate::state::SpecEntry],
) -> Vec<MeasurementTableRow> {
    let Some(run) = app.state.simulation.active_run() else {
        return Vec::new();
    };
    let mut rows: Vec<MeasurementTableRow> = run
        .analyses
        .iter()
        .flat_map(|analysis| {
            analysis.measurements.iter().map(move |measurement| {
                let spec = specifications
                    .iter()
                    .find(|spec| spec.measurement.eq_ignore_ascii_case(&measurement.name));
                measurement_table_row(measurement, &analysis.label, spec)
            })
        })
        .collect();
    // Derived AC stability margins join the table as first-class rows: a
    // `phase_margin` specification binds to them exactly like a .measure
    // result, and the numbers are the stability inspector card's own. A
    // deck-declared measurement of the same name keeps authority.
    //
    // Fail closed exactly as the Bode sheet does. A failed solve's retained
    // vectors are whatever the engine emitted before it gave up, and a margin
    // read off them is not a measurement — which matters more here than on
    // the sheet, because a specification binds to these rows.
    if let Some(summary) = crate::state::ac_bode_summary_for_selection(
        run,
        app.state.simulation.view.active_analysis_idx,
    ) && let Some(analysis) = run.analyses.get(summary.analysis_index)
        && analysis.success
    {
        let label = analysis.label.as_str();
        let metrics = summary.metrics;
        for (name, value, event_axis) in [
            ("phase_margin", metrics.pm_deg, metrics.ugf),
            ("gain_margin", metrics.gm_db, metrics.f180),
            ("unity_gain_freq", metrics.ugf, metrics.ugf),
            ("bandwidth_3db", metrics.f3db, metrics.f3db),
        ] {
            let Some(value) = value else { continue };
            if rows.iter().any(|row| row.name.eq_ignore_ascii_case(name)) {
                continue;
            }
            let spec = specifications
                .iter()
                .find(|spec| spec.measurement.eq_ignore_ascii_case(name));
            let derived = rspice_core::MeasureResult {
                name: name.to_owned(),
                value: Some(value),
                raw_value: Some(value),
                error: None,
                passed: true,
                expected: None,
                tolerance: None,
                failure_limit: None,
                failure_limit_exceeded: false,
                event_axis,
                units: None,
            };
            rows.push(measurement_table_row(&derived, label, spec));
        }
    }
    rows
}

fn measurement_table_row(
    measurement: &rspice_core::MeasureResult,
    analysis_label: &str,
    specification: Option<&crate::state::SpecEntry>,
) -> MeasurementTableRow {
    let requested_unit = specification.map_or("", |spec| spec.unit.as_str());
    let value = measurement.value_in_unit(requested_unit).ok().flatten();
    let display_unit = if !requested_unit.trim().is_empty() {
        Some(requested_unit.trim())
    } else {
        measurement
            .units
            .as_ref()
            .and_then(|units| units.value.symbol())
    };
    let specification_text = measurement_contract_text(measurement, specification);
    let (status, failed, margin) = if let Some(error) = measurement
        .error
        .as_deref()
        .filter(|_| measurement.value.is_none())
        .filter(|error| !error.trim().is_empty())
    {
        ("ERROR", true, error.to_owned())
    } else if measurement_has_contract(measurement, specification) {
        measurement_contract_verdict(measurement, specification)
    } else if let Some(error) = measurement
        .error
        .as_deref()
        .filter(|error| !error.trim().is_empty())
    {
        ("ERROR", true, error.to_owned())
    } else {
        ("NO SPEC", false, "—".to_owned())
    };
    MeasurementTableRow {
        name: measurement.name.clone(),
        expression: "—".to_owned(),
        value: value
            .map(|value| {
                let formatted = format_measure_value(value);
                if measurement.units.is_some() {
                    display_unit
                        .map_or_else(|| formatted.clone(), |unit| format!("{formatted} {unit}"))
                } else {
                    formatted
                }
            })
            .unwrap_or_else(|| "—".to_owned()),
        specification: specification_text,
        margin,
        worst_point: analysis_label.to_owned(),
        status,
        failed,
    }
}

fn measurement_has_contract(
    measurement: &rspice_core::MeasureResult,
    specification: Option<&crate::state::SpecEntry>,
) -> bool {
    specification.is_some() || measurement.expected.is_some() || measurement.failure_limit.is_some()
}

fn measurement_contract_text(
    measurement: &rspice_core::MeasureResult,
    specification: Option<&crate::state::SpecEntry>,
) -> String {
    let mut contracts = Vec::with_capacity(3);
    if let Some(specification) = specification {
        let mut text = specification_text(specification);
        if measurement.units.is_some() && !specification.unit.trim().is_empty() {
            text.push(' ');
            text.push_str(specification.unit.trim());
        }
        contracts.push(("PROJECT", text));
    }
    if let Some(expected) = measurement.expected {
        let text = measurement.tolerance.map_or_else(
            || format!("= {}", format_measure_value(expected)),
            |tolerance| {
                format!(
                    "{} ± {}",
                    format_measure_value(expected),
                    format_measure_value(tolerance)
                )
            },
        );
        let text = measurement
            .units
            .as_ref()
            .and_then(|units| units.value.symbol())
            .map_or_else(|| text.clone(), |unit| format!("{text} {unit}"));
        contracts.push(("GOAL", text));
    }
    if let Some(limit) = measurement.failure_limit {
        contracts.push((
            "FAILVALUE",
            format!(
                "|raw| < {}{}",
                format_measure_value(limit),
                measurement
                    .units
                    .as_ref()
                    .and_then(|units| units.raw_value.symbol())
                    .map_or(String::new(), |unit| format!(" {unit}"))
            ),
        ));
    }

    match contracts.as_slice() {
        [] => "—".to_owned(),
        [(_, text)] => text.clone(),
        _ => contracts
            .into_iter()
            .map(|(label, text)| format!("{label} {text}"))
            .collect::<Vec<_>>()
            .join(" · "),
    }
}

/// The status word, whether it is a failure, and the margin text.
fn measurement_contract_verdict(
    measurement: &rspice_core::MeasureResult,
    specification: Option<&crate::state::SpecEntry>,
) -> (&'static str, bool, String) {
    let Some(value) = measurement.value else {
        return ("NO DATA", false, "—".to_owned());
    };
    if measurement.failure_limit.is_some() && measurement.raw_value.is_none() {
        return ("NO DATA", false, "raw value unavailable".to_owned());
    }

    if let Some(units) = &measurement.units {
        let project_value =
            match measurement.value_in_unit(specification.map_or("", |spec| spec.unit.as_str())) {
                Ok(Some(value)) => value,
                Ok(None) => return ("NO DATA", false, "value unavailable".into()),
                Err(error) => return ("ERROR", true, error),
            };
        let passed =
            measurement.passed && specification.is_none_or(|spec| spec.passes(project_value));
        let mut margins = Vec::new();
        let show = |label: &str, margin: f64, unit: Option<&str>| {
            let value = format_signed_measure_value(margin);
            format!(
                "{label} {value}{}",
                unit.map_or(String::new(), |unit| format!(" {unit}"))
            )
        };
        if let Some(spec) = specification {
            let margin = spec
                .min
                .map(|min| project_value - min)
                .into_iter()
                .chain(spec.max.map(|max| max - project_value))
                .reduce(f64::min);
            if let Some(margin) = margin {
                let unit = if spec.unit.trim().is_empty() {
                    units.value.symbol()
                } else {
                    Some(spec.unit.trim())
                };
                margins.push(show("PROJECT", margin, unit));
            }
        }
        if let Some((expected, tolerance)) = measurement.expected.zip(measurement.tolerance) {
            margins.push(show(
                "GOAL",
                tolerance - (value - expected).abs(),
                units.value.symbol(),
            ));
        }
        if let Some((raw, limit)) = measurement.raw_value.zip(measurement.failure_limit) {
            let margin = limit - raw.abs();
            margins.push(show(
                "FAILVALUE",
                if measurement.failure_limit_exceeded && margin == 0.0 {
                    -0.0
                } else {
                    margin
                },
                units.raw_value.symbol(),
            ));
        }
        if !passed && let Some(error) = &measurement.error {
            margins.push(error.clone());
        }
        return (
            if passed { "PASS" } else { "FAIL" },
            !passed,
            if margins.is_empty() {
                "unbounded".into()
            } else {
                margins.join("; ")
            },
        );
    }

    let project_passed = specification.is_none_or(|specification| specification.passes(value));
    let passed = measurement.passed && project_passed;
    let mut margins = Vec::with_capacity(4);
    let has_numeric_bound = measurement.expected.zip(measurement.tolerance).is_some()
        || measurement.failure_limit.is_some()
        || specification.is_some_and(|specification| {
            specification.min.is_some() || specification.max.is_some()
        });

    if let (Some(expected), Some(tolerance)) = (measurement.expected, measurement.tolerance) {
        margins.push(tolerance - (value - expected).abs());
    }
    if let (Some(raw_value), Some(limit)) = (measurement.raw_value, measurement.failure_limit) {
        let margin = limit - raw_value.abs();
        // FAILVALUE is inclusive: equality is a failure. Preserve that fact in
        // the signed presentation even though the mathematical clearance is
        // exactly zero.
        margins.push(if measurement.failure_limit_exceeded && margin == 0.0 {
            -0.0
        } else {
            margin
        });
    }
    if let Some(specification) = specification {
        margins.extend(
            specification
                .min
                .map(|minimum| value - minimum)
                .into_iter()
                .chain(specification.max.map(|maximum| maximum - value)),
        );
    }

    let limiting_margin = margins
        .into_iter()
        .filter(|margin| margin.is_finite())
        .reduce(f64::min);
    let margin = if passed {
        limiting_margin.map_or_else(
            || {
                if has_numeric_bound {
                    "—".to_owned()
                } else {
                    "unbounded".to_owned()
                }
            },
            format_signed_measure_value,
        )
    } else if let Some(margin) =
        limiting_margin.filter(|margin| *margin < 0.0 || margin.is_sign_negative())
    {
        format_signed_measure_value(margin)
    } else {
        measurement
            .error
            .as_deref()
            .filter(|error| !error.trim().is_empty())
            .unwrap_or("contract failed")
            .to_owned()
    };
    (if passed { "PASS" } else { "FAIL" }, !passed, margin)
}

fn specification_text(specification: &crate::state::SpecEntry) -> String {
    match (specification.min, specification.max) {
        (Some(minimum), Some(maximum)) => format!(
            "{} … {}",
            format_measure_value(minimum),
            format_measure_value(maximum)
        ),
        (Some(minimum), None) => format!("≥ {}", format_measure_value(minimum)),
        (None, Some(maximum)) => format!("≤ {}", format_measure_value(maximum)),
        (None, None) => "tracked".to_owned(),
    }
}

fn measurement_header(ui: &mut Ui, text: &str) {
    ui.label(
        egui::RichText::new(text)
            .font(theme::sans(CONSOLE_FONT_SIZE, FontWeight::SemiBold))
            .color(Tokens::get(ui.ctx()).color.text_dim),
    );
}

fn measurement_cell(ui: &mut Ui, text: &str, mono: bool) {
    let font = if mono {
        theme::mono(CONSOLE_FONT_SIZE, FontWeight::Regular)
    } else {
        theme::sans(CONSOLE_FONT_SIZE, FontWeight::Regular)
    };
    ui.label(
        egui::RichText::new(text)
            .font(font)
            .color(Tokens::get(ui.ctx()).color.text),
    );
}

/// The status cell: the one verdict cell of a results table. A failure reads
/// in the error colour, as ADE Assembler marks it; a pass and every other
/// status are plain text.
fn measurement_status_cell(ui: &mut Ui, text: &str, failed: bool) {
    let tokens = Tokens::get(ui.ctx());
    ui.label(
        egui::RichText::new(text)
            .font(theme::mono(CONSOLE_FONT_SIZE, FontWeight::Regular))
            .color(if failed {
                tokens.color.err
            } else {
                tokens.color.text
            }),
    );
}

/// A run's lifecycle word, and the mark it earns. Lifecycle words are states,
/// not severities: only a failed run is a problem.
fn run_lifecycle_presentation(
    lifecycle: crate::state::SimulationRunLifecycle,
) -> (&'static str, Option<Problem>) {
    use crate::state::SimulationRunLifecycle as Lifecycle;
    match lifecycle {
        Lifecycle::LegacyUnknown => ("UNKNOWN", None),
        Lifecycle::Preparing => ("QUEUED", None),
        Lifecycle::Running => ("RUNNING", None),
        Lifecycle::Cancelling => ("CANCELLING", None),
        Lifecycle::Completed => ("DONE", None),
        Lifecycle::Failed => ("FAIL", Some(Problem::Failure)),
        Lifecycle::Aborted => ("CANCELLED", None),
        Lifecycle::Interrupted => ("INTERRUPTED", None),
    }
}

/// One task-history row: a leading mark for a failed run, then the run's
/// lifecycle, its label and its figures in neutral columns. It senses a
/// press so it can carry a context menu, and takes no focus.
fn issue_row(
    ui: &mut Ui,
    columns: TaskColumns,
    problem: Option<Problem>,
    status: &str,
    label: &str,
    message: &str,
) -> egui::Response {
    let t = Tokens::get(ui.ctx());
    row_with_sense(
        ui,
        columns,
        problem,
        status,
        label,
        message,
        t.color.text_dim,
        t.color.text,
        Sense::CLICK,
    )
}

/// The widths of a task row's status and label columns.
#[derive(Debug, Clone, Copy)]
struct TaskColumns {
    status: f32,
    label: f32,
}

/// The width of the widest status a run can show.
fn widest_status_width(ui: &Ui) -> f32 {
    use crate::state::SimulationRunLifecycle as Lifecycle;
    [
        Lifecycle::LegacyUnknown,
        Lifecycle::Preparing,
        Lifecycle::Running,
        Lifecycle::Cancelling,
        Lifecycle::Completed,
        Lifecycle::Failed,
        Lifecycle::Aborted,
        Lifecycle::Interrupted,
    ]
    .into_iter()
    .map(|lifecycle| {
        task_text_width(
            ui,
            run_lifecycle_presentation(lifecycle).0,
            FontWeight::Regular,
        )
    })
    .fold(0.0, f32::max)
}

fn task_text_width(ui: &Ui, text: &str, weight: FontWeight) -> f32 {
    ui.painter()
        .layout_no_wrap(
            text.to_owned(),
            theme::mono(CONSOLE_FONT_SIZE, weight),
            egui::Color32::PLACEHOLDER,
        )
        .size()
        .x
}

fn log_problem(severity: LogSeverity) -> Option<Problem> {
    match severity {
        LogSeverity::Error => Some(Problem::Failure),
        LogSeverity::Warning => Some(Problem::Warning),
        LogSeverity::Info | LogSeverity::Debug | LogSeverity::Trace => None,
    }
}

/// The neutral ink a log line's message is painted in: primary for the lines
/// a reader acts on, quieter for notes, faintest for tracing.
fn log_message_color(tokens: &Tokens, severity: LogSeverity) -> egui::Color32 {
    match severity {
        LogSeverity::Error | LogSeverity::Warning => tokens.color.text,
        LogSeverity::Info | LogSeverity::Debug => tokens.color.text_dim,
        LogSeverity::Trace => tokens.color.text_faint,
    }
}

fn format_measure_value(value: f64) -> String {
    fmt_si(value, "", 6).trim().to_owned()
}

fn format_signed_measure_value(value: f64) -> String {
    let value = format_measure_value(value);
    if value.starts_with('-') {
        value
    } else {
        format!("+{value}")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ConsoleClearAction {
    enabled: bool,
    label: &'static str,
}

/// The Clear control's state on `page`. On the Console page it is the
/// session's log it clears; a run's log offers no Clear at all.
fn console_clear_action(page: ConsolePage, console_has_output: bool) -> ConsoleClearAction {
    match page {
        ConsolePage::Console => ConsoleClearAction {
            enabled: console_has_output,
            label: if console_has_output {
                "Clear the session log"
            } else {
                "The session log is already empty"
            },
        },
        ConsolePage::Problems => ConsoleClearAction {
            enabled: false,
            // A finding leaves the list when the check that made it no longer
            // does, not when a control hides it.
            label: "Problems lists current findings and is not cleared from here",
        },
        ConsolePage::Measurements => ConsoleClearAction {
            enabled: false,
            label: "Measurements belong to the selected run's results",
        },
        ConsolePage::TaskLog => ConsoleClearAction {
            enabled: false,
            label: "Task history is retained and cannot be cleared here",
        },
    }
}

/// One task-history line: the problem-mark gutter, then the status, run and
/// figures columns. Severity is the mark's alone; the columns are neutral.
fn row_with_sense(
    ui: &mut Ui,
    columns: TaskColumns,
    problem: Option<Problem>,
    status: &str,
    label: &str,
    message: &str,
    label_color: egui::Color32,
    message_color: egui::Color32,
    sense: egui::Sense,
) -> egui::Response {
    let t = Tokens::get(ui.ctx());
    let message_x_offset = CONSOLE_MARK_GUTTER
        + columns.status
        + CONSOLE_COLUMN_GAP
        + columns.label
        + CONSOLE_COLUMN_GAP;
    let message_width = (ui.available_width() - message_x_offset).max(1.0);
    let message_galley = ui.painter().layout(
        message.to_owned(),
        theme::mono(CONSOLE_FONT_SIZE, FontWeight::Regular),
        message_color,
        message_width,
    );
    let row_height = message_galley.size().y.max(CONSOLE_ROW_MIN_HEIGHT);
    let (rect, response) =
        ui.allocate_exact_size(egui::vec2(ui.available_width(), row_height), sense);
    // All three columns are painted galleys. Without this the row is
    // invisible to a screen reader, and a row a focusable sense turns into a
    // tab stop would take focus with nothing announced and nothing drawn.
    let announcement = announced_row(problem, &[status, label, message]);
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            if sense.is_focusable() {
                egui::WidgetType::Button
            } else {
                egui::WidgetType::Label
            },
            ui.is_enabled(),
            announcement.clone(),
        )
    });
    // A row that can be clicked says so under the pointer. Painted before the
    // columns, so the fill sits behind its own text rather than over it.
    if sense.senses_click() && response.hovered() {
        ui.painter().rect_filled(rect, 0.0, t.color.bg_hover);
    }
    paint_row_mark(ui, rect.left(), rect.top(), problem);
    let status_x = rect.left() + CONSOLE_MARK_GUTTER;
    let label_x = status_x + columns.status + CONSOLE_COLUMN_GAP;
    let message_x = label_x + columns.label + CONSOLE_COLUMN_GAP;
    ui.painter().text(
        egui::pos2(status_x, rect.top() + CONSOLE_ROW_MIN_HEIGHT * 0.5),
        egui::Align2::LEFT_CENTER,
        status,
        theme::mono(CONSOLE_FONT_SIZE, FontWeight::Regular),
        t.color.text_faint,
    );
    ui.painter().text(
        egui::pos2(label_x, rect.top() + CONSOLE_ROW_MIN_HEIGHT * 0.5),
        egui::Align2::LEFT_CENTER,
        label,
        theme::mono(CONSOLE_FONT_SIZE, FontWeight::Medium),
        label_color,
    );
    let message_clip = egui::Rect::from_min_max(
        egui::pos2(message_x, rect.top()),
        egui::pos2(rect.right(), rect.bottom()),
    );
    let message_y = if message_galley.size().y <= CONSOLE_ROW_MIN_HEIGHT {
        rect.center().y - message_galley.size().y * 0.5
    } else {
        rect.top()
    };
    ui.painter().with_clip_rect(message_clip).galley(
        egui::pos2(message_x, message_y),
        message_galley,
        message_color,
    );
    theme::paint_focus_ring(ui, &response, rect);
    response
}

fn simulation_progress_percent(progress: f64) -> u8 {
    (progress.clamp(0.0, 1.0) * 100.0).round() as u8
}

/// The line a page paints instead of rows.
///
/// It stands exactly where the page's first row would: same left edge, same
/// leading, same ink as a quiet row. An inset, centred notice floated in the
/// middle of an otherwise tight panel and read as a different surface from
/// the one the reader switched to; this reads as a list with nothing in it.
fn muted(ui: &mut Ui, text: &str) {
    let t = Tokens::get(ui.ctx());
    ui.add(
        egui::Label::new(
            egui::RichText::new(text)
                .font(theme::mono(CONSOLE_FONT_SIZE, FontWeight::Regular))
                .color(t.color.text_faint),
        )
        .wrap(),
    );
}
#[cfg(test)]
mod tests;
