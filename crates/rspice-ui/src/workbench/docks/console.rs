//! Unified console, problems, measurements, and task history.

use egui::{Align, Layout, ScrollArea, Ui};

use crate::common::RSpiceApp;
use crate::panels::{ConsoleHistoryItem, LogSeverity};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};

use super::super::design_system::{WorkbenchIcon, icon_button};
use super::super::state::ConsolePage;

pub fn show(ui: &mut Ui, app: &mut RSpiceApp) {
    header(ui, app);
    match app.state.workbench.console_page {
        ConsolePage::Console => console(ui, app),
        ConsolePage::Problems => problems(ui, app),
        ConsolePage::Measurements => measurements(ui, app),
        ConsolePage::TaskLog => task_log(ui, app),
    }
}

fn header(ui: &mut Ui, app: &mut RSpiceApp) {
    let t = Tokens::get(ui.ctx());
    let problems = app
        .state
        .dialogs
        .drc_results
        .as_ref()
        .map_or(0, |result| result.total_count())
        + app.state.log_buffer.count_by_severity(LogSeverity::Error)
        + app.state.log_buffer.count_by_severity(LogSeverity::Warning);
    let measurements = active_measurement_count(app);
    ui.horizontal(|ui| {
        ui.add_space(6.0);
        for page in ConsolePage::ALL {
            let count = match page {
                ConsolePage::Console => app.state.log_buffer.len(),
                ConsolePage::Problems => problems,
                ConsolePage::Measurements => measurements,
                ConsolePage::TaskLog => app.state.simulation.runs.len(),
            };
            let label = if count > 0 {
                format!("{}  {count}", page.label())
            } else {
                page.label().to_owned()
            };
            if ui
                .selectable_label(app.state.workbench.console_page == page, label)
                .clicked()
            {
                app.state.workbench.console_page = page;
            }
        }
        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            if icon_button(
                ui,
                WorkbenchIcon::Close,
                "Hide console",
                false,
                egui::vec2(28.0, 28.0),
            )
            .clicked()
            {
                app.state.workbench.console_visible = false;
            }
            if icon_button(
                ui,
                WorkbenchIcon::Focus,
                if app.state.workbench.console_maximized {
                    "Restore console"
                } else {
                    "Maximize console"
                },
                app.state.workbench.console_maximized,
                egui::vec2(28.0, 28.0),
            )
            .clicked()
            {
                app.state.workbench.console_maximized = !app.state.workbench.console_maximized;
            }
            if ui
                .add(
                    egui::Button::new(egui::RichText::new("Clear").color(t.color.text_dim))
                        .frame(false),
                )
                .clicked()
            {
                match app.state.workbench.console_page {
                    ConsolePage::Console | ConsolePage::Problems => app.state.clear_primary_log(),
                    ConsolePage::Measurements => {}
                    ConsolePage::TaskLog => app.state.clear_simulation_results(),
                }
            }
        });
    });
    ui.separator();
}

fn console(ui: &mut Ui, app: &mut RSpiceApp) {
    ui.with_layout(Layout::bottom_up(Align::Min), |ui| {
        let response = ui.add_sized(
            [ui.available_width(), 30.0],
            egui::TextEdit::singleline(&mut app.state.script_console.input_buffer)
                .font(egui::TextStyle::Monospace)
                .hint_text("Automation: help · run tran · plot v(out)")
                .margin(egui::Margin::symmetric(9, 5)),
        );
        if response.lost_focus() && ui.input(|input| input.key_pressed(egui::Key::Enter)) {
            let command = app.state.script_console.input_buffer.trim().to_owned();
            if !command.is_empty() {
                let output = app
                    .state
                    .script_console
                    .executor
                    .execute_command(&command, &mut app.state.simulation);
                app.state
                    .script_console
                    .history
                    .push(ConsoleHistoryItem { command, output });
                app.state.script_console.input_buffer.clear();
                response.request_focus();
            }
        }
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
                        &violation.message,
                        &violation.location.display(),
                        violation.severity < crate::services::drc::DrcSeverity::Error,
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
                    &entry.message,
                    entry.context.as_deref().unwrap_or(entry.source.name()),
                    entry.severity == LogSeverity::Error,
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
                    let text = format!("{measurement:?}");
                    issue_row(ui, "MEAS", &text, &analysis.label, true);
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
                    run.success,
                );
            }
        });
}

fn log_row(ui: &mut Ui, entry: &crate::panels::LogEntry) {
    let t = Tokens::get(ui.ctx());
    let color = match entry.severity {
        LogSeverity::Error => t.color.err,
        LogSeverity::Warning => t.color.warn,
        _ => t.color.text_dim,
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
        color,
    );
}

fn automation_row(ui: &mut Ui, item: &ConsoleHistoryItem) {
    let t = Tokens::get(ui.ctx());
    row(ui, ">", "AUTO", &item.command, t.color.text);
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
        );
    }
}

fn issue_row(ui: &mut Ui, level: &str, message: &str, context: &str, positive: bool) {
    let t = Tokens::get(ui.ctx());
    let (rect, response) =
        ui.allocate_exact_size(egui::vec2(ui.available_width(), 38.0), egui::Sense::click());
    if response.hovered() {
        ui.painter().rect_filled(rect, 0.0, t.color.bg_hover);
    }
    let color = if positive { t.color.ok } else { t.color.err };
    ui.painter().text(
        egui::pos2(rect.left() + 10.0, rect.top() + 11.0),
        egui::Align2::LEFT_CENTER,
        level,
        theme::mono(tokens::FS_0, FontWeight::SemiBold),
        color,
    );
    ui.painter().text(
        egui::pos2(rect.left() + 76.0, rect.top() + 11.0),
        egui::Align2::LEFT_CENTER,
        message,
        theme::sans(tokens::FS_1, FontWeight::Regular),
        t.color.text,
    );
    ui.painter().text(
        egui::pos2(rect.left() + 76.0, rect.bottom() - 9.0),
        egui::Align2::LEFT_CENTER,
        context,
        theme::mono(tokens::FS_0, FontWeight::Regular),
        t.color.text_faint,
    );
}

fn row(ui: &mut Ui, time: &str, source: &str, message: &str, color: egui::Color32) {
    let t = Tokens::get(ui.ctx());
    let (rect, _) =
        ui.allocate_exact_size(egui::vec2(ui.available_width(), 20.0), egui::Sense::hover());
    ui.painter().text(
        egui::pos2(rect.left() + 10.0, rect.center().y),
        egui::Align2::LEFT_CENTER,
        time,
        theme::mono(tokens::FS_0, FontWeight::Regular),
        t.color.text_faint,
    );
    ui.painter().text(
        egui::pos2(rect.left() + 104.0, rect.center().y),
        egui::Align2::LEFT_CENTER,
        source,
        theme::mono(tokens::FS_0, FontWeight::Medium),
        color,
    );
    ui.painter().text(
        egui::pos2(rect.left() + 148.0, rect.center().y),
        egui::Align2::LEFT_CENTER,
        message,
        theme::mono(tokens::FS_0, FontWeight::Regular),
        t.color.text,
    );
}

fn active_measurement_count(app: &RSpiceApp) -> usize {
    app.state
        .simulation
        .active_run_idx
        .and_then(|index| app.state.simulation.runs.get(index))
        .map_or(0, |run| {
            run.analyses
                .iter()
                .map(|analysis| analysis.measurements.len())
                .sum()
        })
}

fn muted(ui: &mut Ui, text: &str) {
    let t = Tokens::get(ui.ctx());
    ui.add_space(10.0);
    ui.label(egui::RichText::new(text).color(t.color.text_faint));
}
