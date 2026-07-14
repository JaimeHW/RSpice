//! Unified console, problems, measurements, and task history.

use egui::{Align, Layout, ScrollArea, Ui};

use crate::common::{AppState, RSpiceApp};
use crate::panels::{ConsoleHistoryItem, LogSeverity};
use crate::ui::plot::fmt_si;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};

use super::super::design_system::{WorkbenchIcon, icon_button};
use super::super::layout::LayoutSpec;
use super::super::state::ConsolePage;

pub fn show(ui: &mut Ui, app: &mut RSpiceApp, layout: LayoutSpec) {
    header(ui, app, layout);
    if !layout.show_console_body {
        return;
    }
    match app.state.workbench.console_page {
        ConsolePage::Console => console(ui, app),
        ConsolePage::Problems => problems(ui, app),
        ConsolePage::Measurements => measurements(ui, app),
        ConsolePage::TaskLog => task_log(ui, app),
    }
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
            let control_size = if layout.coarse_pointer { 44.0 } else { 28.0 };
            if icon_button(
                ui,
                if layout.show_console_body {
                    WorkbenchIcon::Close
                } else {
                    WorkbenchIcon::Console
                },
                if layout.show_console_body {
                    "Collapse console"
                } else {
                    "Expand console"
                },
                false,
                egui::vec2(control_size, control_size),
            )
            .clicked()
            {
                if layout.show_console_body {
                    app.state.workbench.console_visible = false;
                    app.state.workbench.console_maximized = false;
                } else if !app.state.workbench.focus_mode {
                    app.state.workbench.console_visible = true;
                }
            }
            if !layout.compact_shell
                && !layout.coarse_pointer
                && icon_button(
                    ui,
                    WorkbenchIcon::Focus,
                    if app.state.workbench.console_maximized {
                        "Restore console"
                    } else {
                        "Maximize console"
                    },
                    app.state.workbench.console_maximized,
                    egui::vec2(control_size, control_size),
                )
                .clicked()
            {
                app.state.workbench.console_maximized = !app.state.workbench.console_maximized;
                app.state.workbench.console_visible = true;
            }
            // Problems, Measurements, and Task log are projections of owned
            // engineering records. A generic console action must never erase
            // DRC evidence, immutable measurement data, or run history.
            if !layout.compact_shell
                && page_owns_clear_action(app.state.workbench.console_page)
                && ui
                    .add(
                        egui::Button::new(egui::RichText::new("Clear").color(t.color.text_dim))
                            .frame(false),
                    )
                    .on_hover_text("Clear console output")
                    .clicked()
            {
                clear_console_output(&mut app.state);
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
                    &entry.message,
                    entry.context.as_deref().unwrap_or(entry.source.name()),
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
    let color = tone_color(&t, log_tone(entry.severity));
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

fn issue_row(ui: &mut Ui, level: &str, message: &str, context: &str, tone: SemanticTone) {
    let t = Tokens::get(ui.ctx());
    let (rect, response) =
        ui.allocate_exact_size(egui::vec2(ui.available_width(), 38.0), egui::Sense::hover());
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::Label,
            ui.is_enabled(),
            format!("{level}: {message}. {context}"),
        )
    });
    if response.hovered() {
        ui.painter().rect_filled(rect, 0.0, t.color.bg_hover);
    }
    let color = tone_color(&t, tone);
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
        SemanticTone::Info => tokens.color.accent,
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

/// Clear only the two output streams owned by the Console page. Diagnostic
/// evidence and simulation result history live in other owners and are never
/// touched by this action.
fn clear_console_output(state: &mut AppState) {
    state.clear_primary_log();
    state.script_console.history.clear();
}

fn page_owns_clear_action(page: ConsolePage) -> bool {
    matches!(page, ConsolePage::Console)
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::panels::LogSource;
    use crate::services::drc::{DrcResult, DrcSeverity};
    use crate::state::{AnalysisResult, AnalysisType, SimulationRun};

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
    fn only_console_exposes_the_clear_action() {
        assert!(page_owns_clear_action(ConsolePage::Console));
        assert!(!page_owns_clear_action(ConsolePage::Problems));
        assert!(!page_owns_clear_action(ConsolePage::Measurements));
        assert!(!page_owns_clear_action(ConsolePage::TaskLog));
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

        clear_console_output(&mut state);

        assert!(state.log_buffer.is_empty());
        assert!(state.script_console.history.is_empty());
        assert_eq!(state.script_console.input_buffer, "pending command");
        assert!(state.dialogs.drc_results.is_some());
        assert_eq!(state.simulation.runs.len(), 1);
        assert_eq!(state.simulation.runs[0].analyses[0].measurements.len(), 1);
    }
}
