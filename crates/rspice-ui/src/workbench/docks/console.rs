//! Unified console, problems, measurements, and task history.

use egui::{Align, Layout, ScrollArea, Sense, Ui, Vec2};

use crate::common::RSpiceApp;
use crate::panels::{ConsoleHistoryItem, LogSeverity};
use crate::ui::plot::fmt_si;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};

use super::super::commands::Command;
use super::super::design_system::WorkbenchIcon;
use super::super::layout::LayoutSpec;
use super::super::state::ConsolePage;

const CONSOLE_HEADER_HEIGHT: f32 = 31.0;
const CONSOLE_TOUCH_HEADER_HEIGHT: f32 = 44.0;
const CONSOLE_ACTION_SIZE: f32 = 27.0;
const CONSOLE_ACTION_MARGIN_RIGHT: f32 = 3.0;
const CONSOLE_BODY_PADDING_TOP: f32 = 7.0;
const CONSOLE_BODY_PADDING_BOTTOM: f32 = 7.0;
const CONSOLE_BODY_PADDING_X: f32 = 10.0;
const CONSOLE_ROW_MIN_HEIGHT: f32 = 16.0;
const CONSOLE_TIME_WIDTH: f32 = 58.0;
const CONSOLE_SOURCE_WIDTH: f32 = 62.0;
const CONSOLE_COLUMN_GAP: f32 = 9.0;
const EMPTY_HINT_PADDING_X: i8 = 12;
const EMPTY_HINT_PADDING_Y: i8 = 20;

pub fn show(ui: &mut Ui, app: &mut RSpiceApp, layout: LayoutSpec) {
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
            ConsolePage::Console => console(ui, app),
            ConsolePage::Problems => problems(ui, app),
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
    let problems = app
        .state
        .dialogs
        .drc_results
        .as_ref()
        .map_or(0, |result| result.total_count())
        + app.state.log_buffer.count_by_severity(LogSeverity::Error)
        + app.state.log_buffer.count_by_severity(LogSeverity::Warning);
    let touch_targets = Tokens::get(ui.ctx()).metrics.ctl_h >= 44.0;
    let header_height = if touch_targets {
        CONSOLE_TOUCH_HEADER_HEIGHT
    } else {
        CONSOLE_HEADER_HEIGHT
    };
    let header = ui.allocate_ui_with_layout(
        Vec2::new(ui.available_width(), header_height),
        Layout::left_to_right(Align::Center),
        |ui| {
            ui.spacing_mut().item_spacing = Vec2::ZERO;
            let action_width = console_trailing_actions_width(
                layout,
                app.state.workbench.console_page,
                touch_targets,
            );
            egui::ScrollArea::horizontal()
                .id_salt("workbench.console.tabs.scroll")
                .max_width((ui.available_width() - action_width).max(header_height))
                .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysHidden)
                .show(ui, |ui| console_tabs(ui, app, problems, header_height));
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
                if !layout.compact_shell {
                    console_context(ui, app);
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

fn console_trailing_actions_width(
    layout: LayoutSpec,
    _page: ConsolePage,
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
        // Clear remains present for every page so switching tabs never moves
        // Maximize or Collapse. Pages backed by immutable/derived data expose
        // the same affordance disabled with a precise explanation.
        actions += 1;
    }
    actions as f32 * (control_size + CONSOLE_ACTION_MARGIN_RIGHT)
}

fn console_tabs(ui: &mut Ui, app: &mut RSpiceApp, problems: usize, height: f32) {
    let mut requested = None;
    let mut focus = None;
    let mut ids = Vec::with_capacity(ConsolePage::ALL.len());
    let tabs = ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing = Vec2::ZERO;
        for (index, page) in ConsolePage::ALL.into_iter().enumerate() {
            let count = match page {
                ConsolePage::Problems => problems,
                ConsolePage::TaskLog => app.state.simulation.runs.len(),
                ConsolePage::Console | ConsolePage::Measurements => 0,
            };
            let count_tone = match page {
                ConsolePage::Problems => ConsoleCountTone::Warning,
                ConsolePage::TaskLog => ConsoleCountTone::Success,
                ConsolePage::Console | ConsolePage::Measurements => ConsoleCountTone::Neutral,
            };
            let response = console_tab(
                ui,
                page,
                app.state.workbench.console_page == page,
                count,
                count_tone,
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

#[derive(Clone, Copy)]
enum ConsoleCountTone {
    Neutral,
    Warning,
    Success,
}

fn console_tab(
    ui: &mut Ui,
    page: ConsolePage,
    selected: bool,
    count: usize,
    count_tone: ConsoleCountTone,
    height: f32,
) -> egui::Response {
    let t = Tokens::get(ui.ctx());
    let font = theme::sans(tokens::FS_0, FontWeight::Regular);
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
    let sense = if selected {
        Sense::click()
    } else {
        Sense::click().difference(Sense::focusable_noninteractive())
    };
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
        if count > 0 {
            node.set_label(format!("{}, {count}", page.label()));
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
        let (ink, fill) = match count_tone {
            ConsoleCountTone::Warning => (
                t.color.warn,
                theme::mix(t.color.bg_panel, t.color.warn, 0.11),
            ),
            ConsoleCountTone::Success => {
                (t.color.ok, theme::mix(t.color.bg_panel, t.color.ok, 0.11))
            }
            ConsoleCountTone::Neutral => (t.color.text_dim, t.color.bg_active),
        };
        ui.painter().rect_filled(badge, 7.0, fill);
        ui.painter().text(
            badge.center(),
            egui::Align2::CENTER_CENTER,
            digits,
            theme::mono(tokens::FS_0, FontWeight::Medium),
            ink,
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
    response
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

fn console_context(ui: &mut Ui, app: &RSpiceApp) {
    let t = Tokens::get(ui.ctx());
    let text = if app.state.simulation.is_running {
        format!(
            "Active job · {}% · execution in progress",
            simulation_progress_percent(app.state.simulation.progress)
        )
    } else if let Some(index) = app.state.simulation.active_run_idx {
        app.state.simulation.runs.get(index).map_or_else(
            || "No retained result selected".to_owned(),
            |run| {
                format!(
                    "Latest result · {} · completed in {:.2} s",
                    run.label, run.elapsed_time
                )
            },
        )
    } else {
        "No retained result selected".to_owned()
    };
    ui.add(
        egui::Label::new(
            egui::RichText::new(text)
                .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                .color(t.color.text_faint),
        )
        .truncate(),
    );
}

fn console(ui: &mut Ui, app: &mut RSpiceApp) {
    ScrollArea::vertical()
        .id_salt("workbench.console.body")
        .stick_to_bottom(true)
        .show(ui, |ui| {
            for entry in app.state.log_buffer.entries() {
                log_row(ui, entry);
            }
            for item in &app.state.script_console.history {
                automation_row(ui, item);
            }
            if app.state.log_buffer.is_empty() && app.state.script_console.history.is_empty() {
                muted(ui, "Engine and automation messages will appear here.");
            }
        });
}

fn problems(ui: &mut Ui, app: &mut RSpiceApp) {
    ScrollArea::vertical()
        .id_salt("workbench.problems")
        .show(ui, |ui| {
            let mut any = false;
            if let Some(result) = &app.state.dialogs.drc_results {
                for violation in result.violations() {
                    any = true;
                    issue_row(
                        ui,
                        violation.severity.display_name(),
                        &violation.location.display(),
                        &violation.message,
                        drc_tone(violation.severity),
                    );
                }
            }
            for entry in
                app.state.log_buffer.entries().filter(|entry| {
                    matches!(entry.severity, LogSeverity::Error | LogSeverity::Warning)
                })
            {
                any = true;
                issue_row(
                    ui,
                    entry.severity.name(),
                    entry.context.as_deref().unwrap_or(entry.source.name()),
                    &entry.message,
                    log_tone(entry.severity),
                );
            }
            if !any {
                muted(ui, "No current errors or advisories.");
            }
        });
}

fn measurements(ui: &mut Ui, app: &mut RSpiceApp) {
    ScrollArea::vertical()
        .id_salt("workbench.measurements")
        .show(ui, |ui| {
            let Some(run_index) = app.state.simulation.active_run_idx else {
                muted(ui, "Select a result dataset to inspect measurements.");
                return;
            };
            let Some(run) = app.state.simulation.runs.get(run_index) else {
                muted(ui, "The selected dataset is no longer available.");
                return;
            };
            let mut any = false;
            for analysis in &run.analyses {
                if analysis.measurements.is_empty() {
                    continue;
                }
                any = true;
                ui.label(egui::RichText::new(&analysis.label).strong());
                for measurement in &analysis.measurements {
                    let row = measurement_presentation(measurement, &analysis.label);
                    issue_row(ui, row.status, &row.name, &row.detail, row.tone);
                }
            }
            if !any {
                muted(ui, "This dataset has no .MEAS results.");
            }
        });
}

fn task_log(ui: &mut Ui, app: &mut RSpiceApp) {
    ScrollArea::vertical()
        .id_salt("workbench.task_log")
        .show(ui, |ui| {
            if app.state.simulation.runs.is_empty() {
                muted(ui, "Completed simulation tasks will appear here.");
            }
            for run in &app.state.simulation.runs {
                issue_row(
                    ui,
                    if run.success { "DONE" } else { "FAIL" },
                    &run.label,
                    &format!(
                        "{} analyses · {:.3} s",
                        run.analyses.len(),
                        run.elapsed_time
                    ),
                    if run.success {
                        SemanticTone::Success
                    } else {
                        SemanticTone::Error
                    },
                );
            }
        });
}

fn log_row(ui: &mut Ui, entry: &crate::panels::LogEntry) {
    let t = Tokens::get(ui.ctx());
    let tone = log_tone(entry.severity);
    let source_color = tone_color(&t, tone);
    let message_color = match tone {
        SemanticTone::Error | SemanticTone::Warning | SemanticTone::Success => source_color,
        SemanticTone::Info | SemanticTone::Debug => t.color.text_dim,
        SemanticTone::Trace => t.color.text_faint,
    };
    let message = entry.context.as_ref().map_or_else(
        || entry.message.clone(),
        |context| format!("{} · {context}", entry.message),
    );
    row(
        ui,
        &entry.format_timestamp(),
        entry.source.name(),
        &message,
        source_color,
        message_color,
    );
}

fn automation_row(ui: &mut Ui, item: &ConsoleHistoryItem) {
    let t = Tokens::get(ui.ctx());
    row(
        ui,
        ">",
        "AUTO",
        &item.command,
        t.color.info,
        t.color.text_dim,
    );
    for line in item.output.message.lines() {
        row(
            ui,
            "",
            "",
            line,
            if item.output.success {
                t.color.ok
            } else {
                t.color.err
            },
            if item.output.success {
                t.color.ok
            } else {
                t.color.err
            },
        );
    }
}

fn issue_row(ui: &mut Ui, status: &str, source: &str, message: &str, tone: SemanticTone) {
    let t = Tokens::get(ui.ctx());
    let color = tone_color(&t, tone);
    row(ui, status, source, message, color, color);
}

/// Semantic row tones are kept independent of the active palette so severity
/// cannot accidentally collapse into a success/error boolean.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SemanticTone {
    Error,
    Warning,
    Success,
    Info,
    Debug,
    Trace,
}

fn tone_color(tokens: &Tokens, tone: SemanticTone) -> egui::Color32 {
    match tone {
        SemanticTone::Error => tokens.color.err,
        SemanticTone::Warning => tokens.color.warn,
        SemanticTone::Success => tokens.color.ok,
        SemanticTone::Info => tokens.color.info,
        SemanticTone::Debug => tokens.color.text_dim,
        SemanticTone::Trace => tokens.color.text_faint,
    }
}

fn log_tone(severity: LogSeverity) -> SemanticTone {
    match severity {
        LogSeverity::Error => SemanticTone::Error,
        LogSeverity::Warning => SemanticTone::Warning,
        LogSeverity::Info => SemanticTone::Info,
        LogSeverity::Debug => SemanticTone::Debug,
        LogSeverity::Trace => SemanticTone::Trace,
    }
}

fn drc_tone(severity: crate::services::drc::DrcSeverity) -> SemanticTone {
    match severity {
        crate::services::drc::DrcSeverity::Critical | crate::services::drc::DrcSeverity::Error => {
            SemanticTone::Error
        }
        crate::services::drc::DrcSeverity::Warning => SemanticTone::Warning,
        crate::services::drc::DrcSeverity::Info => SemanticTone::Info,
    }
}

struct MeasurementPresentation {
    status: &'static str,
    name: String,
    detail: String,
    tone: SemanticTone,
}

/// Convert the structured `.MEAS` contract into a stable presentation row.
/// This deliberately names every field instead of relying on Rust's Debug
/// output, which is neither user-facing nor a compatibility contract.
fn measurement_presentation(
    measurement: &rspice_core::MeasureResult,
    analysis_label: &str,
) -> MeasurementPresentation {
    let mut details = Vec::new();
    if let Some(value) = measurement.value {
        details.push(format!("value {}", format_measure_value(value)));
    } else {
        details.push("no computed value".to_owned());
    }

    if let Some(expected) = measurement.expected {
        let goal = format_measure_value(expected);
        if let Some(tolerance) = measurement.tolerance {
            details.push(format!("goal {goal} ± {}", format_measure_value(tolerance)));
        } else {
            details.push(format!("goal {goal}"));
        }
        if let Some(value) = measurement.value {
            details.push(format!(
                "deviation {}",
                format_measure_value(value - expected)
            ));
        }
    } else if let Some(tolerance) = measurement.tolerance {
        // Imported data may retain a tolerance after its source goal was
        // removed. Surface that inconsistency faithfully; do not invent a
        // requirement or silently discard the retained field.
        details.push(format!("tolerance {}", format_measure_value(tolerance)));
    }

    if let Some(error) = measurement
        .error
        .as_deref()
        .filter(|error| !error.is_empty())
    {
        details.push(error.to_owned());
    }
    details.push(analysis_label.to_owned());

    MeasurementPresentation {
        status: if measurement.passed { "PASS" } else { "FAIL" },
        name: measurement.name.clone(),
        detail: details.join(" · "),
        tone: if measurement.passed {
            SemanticTone::Success
        } else {
            SemanticTone::Error
        },
    }
}

fn format_measure_value(value: f64) -> String {
    fmt_si(value, "", 6).trim().to_owned()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ConsoleClearAction {
    enabled: bool,
    label: &'static str,
}

fn console_clear_action(page: ConsolePage, console_has_output: bool) -> ConsoleClearAction {
    match page {
        ConsolePage::Console => ConsoleClearAction {
            enabled: console_has_output,
            label: if console_has_output {
                "Clear console output"
            } else {
                "Console output is already empty"
            },
        },
        ConsolePage::Problems => ConsoleClearAction {
            enabled: false,
            label: "Problems reflect current diagnostics and cannot be cleared",
        },
        ConsolePage::Measurements => ConsoleClearAction {
            enabled: false,
            label: "Measurements belong to the selected immutable dataset",
        },
        ConsolePage::TaskLog => ConsoleClearAction {
            enabled: false,
            label: "Task history is retained and cannot be cleared here",
        },
    }
}

fn row(
    ui: &mut Ui,
    time: &str,
    source: &str,
    message: &str,
    source_color: egui::Color32,
    message_color: egui::Color32,
) {
    let t = Tokens::get(ui.ctx());
    let message_x_offset =
        CONSOLE_TIME_WIDTH + CONSOLE_COLUMN_GAP + CONSOLE_SOURCE_WIDTH + CONSOLE_COLUMN_GAP;
    let message_width = (ui.available_width() - message_x_offset).max(1.0);
    let message_galley = ui.painter().layout(
        message.to_owned(),
        theme::mono(tokens::FS_0, FontWeight::Regular),
        message_color,
        message_width,
    );
    let row_height = message_galley.size().y.max(CONSOLE_ROW_MIN_HEIGHT);
    let (rect, _) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), row_height),
        egui::Sense::hover(),
    );
    let time_x = rect.left();
    let source_x = time_x + CONSOLE_TIME_WIDTH + CONSOLE_COLUMN_GAP;
    let message_x = source_x + CONSOLE_SOURCE_WIDTH + CONSOLE_COLUMN_GAP;
    let time_clip = egui::Rect::from_min_max(
        egui::pos2(time_x, rect.top()),
        egui::pos2(time_x + CONSOLE_TIME_WIDTH, rect.bottom()),
    );
    ui.painter().with_clip_rect(time_clip).text(
        egui::pos2(time_x, rect.top() + CONSOLE_ROW_MIN_HEIGHT * 0.5),
        egui::Align2::LEFT_CENTER,
        time,
        theme::mono(tokens::FS_0, FontWeight::Regular),
        t.color.text_faint,
    );
    let source_clip = egui::Rect::from_min_max(
        egui::pos2(source_x, rect.top()),
        egui::pos2(source_x + CONSOLE_SOURCE_WIDTH, rect.bottom()),
    );
    ui.painter().with_clip_rect(source_clip).text(
        egui::pos2(source_x, rect.top() + CONSOLE_ROW_MIN_HEIGHT * 0.5),
        egui::Align2::LEFT_CENTER,
        source,
        theme::mono(tokens::FS_0, FontWeight::Medium),
        source_color,
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
}

fn simulation_progress_percent(progress: f64) -> u8 {
    (progress.clamp(0.0, 1.0) * 100.0).round() as u8
}

fn muted(ui: &mut Ui, text: &str) {
    let t = Tokens::get(ui.ctx());
    egui::Frame::new()
        .inner_margin(egui::Margin {
            left: EMPTY_HINT_PADDING_X,
            right: EMPTY_HINT_PADDING_X,
            top: EMPTY_HINT_PADDING_Y,
            bottom: EMPTY_HINT_PADDING_Y,
        })
        .show(ui, |ui| {
            ui.set_width(ui.available_width().max(1.0));
            ui.add(
                egui::Label::new(
                    egui::RichText::new(text)
                        .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                        .color(t.color.text_faint),
                )
                .wrap()
                .halign(Align::Center),
            );
        });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::AppState;
    use crate::panels::LogSource;
    use crate::services::drc::{DrcResult, DrcSeverity};
    use crate::state::{AnalysisResult, AnalysisType, SimulationRun};
    use crate::workbench::state::WorkbenchState;

    #[test]
    fn console_empty_hints_match_mockup_spacing_and_type_scale() {
        assert_eq!(EMPTY_HINT_PADDING_X, 12);
        assert_eq!(EMPTY_HINT_PADDING_Y, 20);
        assert_eq!(tokens::FS_0, 11.0);
    }

    #[test]
    fn severity_tones_preserve_warning_info_and_diagnostic_meaning() {
        assert_eq!(log_tone(LogSeverity::Error), SemanticTone::Error);
        assert_eq!(log_tone(LogSeverity::Warning), SemanticTone::Warning);
        assert_eq!(log_tone(LogSeverity::Info), SemanticTone::Info);
        assert_eq!(log_tone(LogSeverity::Debug), SemanticTone::Debug);
        assert_eq!(log_tone(LogSeverity::Trace), SemanticTone::Trace);

        assert_eq!(drc_tone(DrcSeverity::Critical), SemanticTone::Error);
        assert_eq!(drc_tone(DrcSeverity::Error), SemanticTone::Error);
        assert_eq!(drc_tone(DrcSeverity::Warning), SemanticTone::Warning);
        assert_eq!(drc_tone(DrcSeverity::Info), SemanticTone::Info);
    }

    #[test]
    fn measurement_rows_are_derived_from_structured_fields() {
        let passing = rspice_core::MeasureResult {
            name: "gain_margin".to_owned(),
            value: Some(1.2e-3),
            error: None,
            passed: true,
            expected: Some(1.0e-3),
            tolerance: Some(0.25e-3),
        };
        let row = measurement_presentation(&passing, "Transient");
        assert_eq!(row.status, "PASS");
        assert_eq!(row.name, "gain_margin");
        assert_eq!(row.tone, SemanticTone::Success);
        assert!(row.detail.contains("value 1.200000 m"));
        assert!(row.detail.contains("goal 1.000000 m ± 250.000000 µ"));
        assert!(row.detail.contains("deviation 200.000000 µ"));
        assert!(row.detail.ends_with("Transient"));

        let failed = rspice_core::MeasureResult::failed("rise_time", "crossing not found");
        let row = measurement_presentation(&failed, "TRAN (10 ms)");
        assert_eq!(row.status, "FAIL");
        assert_eq!(row.name, "rise_time");
        assert_eq!(row.tone, SemanticTone::Error);
        assert!(row.detail.contains("no computed value"));
        assert!(row.detail.contains("crossing not found"));
        assert!(row.detail.ends_with("TRAN (10 ms)"));
    }

    #[test]
    fn clear_affordance_is_truthful_for_every_console_page() {
        let console = console_clear_action(ConsolePage::Console, true);
        assert!(console.enabled);
        assert_eq!(console.label, "Clear console output");
        assert!(!console_clear_action(ConsolePage::Console, false).enabled);

        for page in [
            ConsolePage::Problems,
            ConsolePage::Measurements,
            ConsolePage::TaskLog,
        ] {
            let action = console_clear_action(page, true);
            assert!(!action.enabled);
            assert_ne!(action.label, "Clear console output");
        }
    }

    #[test]
    fn clearing_console_preserves_diagnostics_measurements_and_run_history() {
        let mut state = AppState::default();
        state
            .log_buffer
            .warning(LogSource::Simulation, "visible console warning");
        state.script_console.input_buffer = "pending command".to_owned();
        state.script_console.history.push(ConsoleHistoryItem {
            command: "help".to_owned(),
            output: Default::default(),
        });
        state.dialogs.drc_results = Some(DrcResult::new());

        let measurement = rspice_core::MeasureResult::success("gain", 42.0);
        let analysis =
            AnalysisResult::new(1, AnalysisType::Ac, "AC").with_measurements(vec![measurement]);
        let mut run = SimulationRun::new(1);
        run.add_analysis(analysis);
        state.simulation.runs.push(run);

        state.clear_primary_log();
        state.script_console.history.clear();

        assert!(state.log_buffer.is_empty());
        assert!(state.script_console.history.is_empty());
        assert_eq!(state.script_console.input_buffer, "pending command");
        assert!(state.dialogs.drc_results.is_some());
        assert_eq!(state.simulation.runs.len(), 1);
        assert_eq!(state.simulation.runs[0].analyses[0].measurements.len(), 1);
    }

    #[test]
    fn fractional_engine_progress_is_rendered_as_a_percentage() {
        assert_eq!(simulation_progress_percent(0.0), 0);
        assert_eq!(simulation_progress_percent(0.375), 38);
        assert_eq!(simulation_progress_percent(1.0), 100);
    }

    #[test]
    fn console_chrome_and_grid_geometry_match_the_mockup() {
        assert_eq!(CONSOLE_HEADER_HEIGHT, 31.0);
        assert_eq!(CONSOLE_TOUCH_HEADER_HEIGHT, 44.0);
        assert_eq!(CONSOLE_ACTION_SIZE, 27.0);
        assert_eq!(CONSOLE_ACTION_MARGIN_RIGHT, 3.0);
        assert_eq!(CONSOLE_BODY_PADDING_TOP, 7.0);
        assert_eq!(CONSOLE_BODY_PADDING_BOTTOM, 7.0);
        assert_eq!(CONSOLE_BODY_PADDING_X, 10.0);
        assert_eq!(CONSOLE_TIME_WIDTH, 58.0);
        assert_eq!(CONSOLE_SOURCE_WIDTH, 62.0);
        assert_eq!(CONSOLE_COLUMN_GAP, 9.0);
        assert_eq!(CONSOLE_ROW_MIN_HEIGHT, 16.0);
        assert_eq!(tokens::FS_0, 11.0);
    }

    #[test]
    fn console_tab_lane_reserves_every_visible_trailing_action() {
        let desktop = LayoutSpec::resolve(1_280.0, 900.0, &WorkbenchState::default());
        for page in ConsolePage::ALL {
            assert_eq!(console_trailing_actions_width(desktop, page, false), 90.0);
        }

        let phone =
            LayoutSpec::resolve_with_pointer(390.0, 844.0, true, &WorkbenchState::default());
        for page in ConsolePage::ALL {
            assert_eq!(console_trailing_actions_width(phone, page, true), 47.0);
        }

        let tablet =
            LayoutSpec::resolve_with_pointer(1_024.0, 768.0, true, &WorkbenchState::default());
        for page in ConsolePage::ALL {
            assert_eq!(console_trailing_actions_width(tablet, page, true), 94.0);
        }
    }
}
