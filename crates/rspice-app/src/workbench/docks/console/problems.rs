//! The Problems page: the conditions that currently stand against the design.
//!
//! A problem is a condition, not an event. That is the whole difference
//! between this page and the Console beside it: the Console records what
//! happened in the session, in the order it happened, and keeps it after it
//! stops being true; this page states what is wrong with the design right
//! now, and a finding leaves it when the design stops earning it.
//!
//! Everything here follows from that. A finding is reported once, by whatever
//! owns the condition, and never copied into the log as well. A finding whose
//! inputs have since changed is shown faint and said to be stale rather than
//! quietly dropped, because "no findings" and "nobody has looked" are
//! different answers and a tool that confuses them is lying. A finding that
//! knows where it is, is a target: clicking it goes there.
//!
//! Columns are sized to their content, measured in the font they are printed
//! in. They used to be fixed at 58 px and 62 px — eight characters — which
//! was enough for the Console's `DRC` source tag and nowhere near enough for
//! `Component M1` or `opamp.sp · line 42 · column 7`.

use egui::{Sense, Ui, Vec2};

use super::{
    AppState, CONSOLE_COLUMN_GAP, CONSOLE_FONT_SIZE, CONSOLE_MARK_GUTTER, CONSOLE_ROW_MIN_HEIGHT,
    announced_row, muted, netlist_diagnostic_location, netlist_diagnostics_own_problems,
    paint_row_mark,
};
use crate::diagnostics::{LogAnchor, LogEntry, LogSeverity, LogSource};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::Tokens;
use crate::ui::widgets::Problem;
use crate::workbench::app_state::design_checks::DesignCheckStatus;
use crate::workbench::documents::netlist_document::DiagnosticSeverity;
use rspice_design::drc::{DrcResult, DrcSeverity};

/// The longest severity cell the page can print. The severity column is
/// measured against this, so no severity word is ever cut.
const WIDEST_SEVERITY_CELL: &str = "Stale Critical";

/// The narrowest location column worth printing: enough for a short file name
/// with a line and column, which is the shape a reader scans for.
const NARROWEST_LOCATION_CELL: &str = "opamp.sp · line 42 · c";

/// The share of a row the location column may take before the message is
/// starved. A location is how you find the problem; the message is what the
/// problem is, and the message is the longer text of the two.
const LOCATION_COLUMN_MAX_SHARE: f32 = 0.34;

/// The counts the tab strip shows and the status bar states, under one rule:
/// an error is a condition that stops the design working, an advisory is
/// every other current finding, and a stale finding is neither — it is a
/// finding about a design that no longer exists.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(super) struct ProblemCount {
    pub(super) errors: usize,
    pub(super) advisories: usize,
}

impl ProblemCount {
    pub(super) const fn total(self) -> usize {
        self.errors + self.advisories
    }

    /// The counts in words, for the tab's accessible name and its tooltip.
    /// The badge itself has room for one number; this is where the number is
    /// broken down, so the strip never implies that twelve advisories are
    /// twelve errors.
    pub(super) fn announcement(self) -> String {
        format!(
            "{} error{}, {} advisor{}",
            self.errors,
            if self.errors == 1 { "" } else { "s" },
            self.advisories,
            if self.advisories == 1 { "y" } else { "ies" },
        )
    }
}

/// Where the schematic findings on this page come from, and whether they
/// still describe the design in front of the reader.
enum CheckSource<'a> {
    /// No check has produced evidence for this subject.
    NotRun,
    /// Evidence for exactly the design as it stands.
    Current(&'a DrcResult),
    /// Evidence from before an edit. Its findings are still shown, because a
    /// stale finding usually still points at something real, but they are
    /// shown as what they are.
    Stale(&'a DrcResult),
    /// The check cannot be evaluated at all, in its own words.
    Unavailable(String),
}

impl CheckSource<'_> {
    fn result(&self) -> Option<&DrcResult> {
        match self {
            Self::Current(result) | Self::Stale(result) => Some(result),
            Self::NotRun | Self::Unavailable(_) => None,
        }
    }

    const fn current(&self) -> bool {
        matches!(self, Self::Current(_))
    }
}

/// Resolve the schematic check evidence for the active subject.
///
/// The receipt store is asked first: it is the only thing that knows whether
/// evidence is current, and it keeps a stale receipt instead of discarding
/// it. `dialogs.drc_results` is the fallback, because the symbol-pin checks
/// publish straight into that projection and have no receipt of their own.
fn check_source(state: &AppState) -> CheckSource<'_> {
    match state.active_design_check_status() {
        DesignCheckStatus::Current(receipt) => CheckSource::Current(&receipt.result),
        DesignCheckStatus::Stale(receipt) => CheckSource::Stale(&receipt.result),
        DesignCheckStatus::Unavailable { reason, .. } => CheckSource::Unavailable(reason),
        DesignCheckStatus::NotRun => match state.dialogs.drc_results.as_ref() {
            Some(result) => CheckSource::Current(result),
            None => CheckSource::NotRun,
        },
    }
}

/// The log entries that are problems rather than session events.
///
/// A warning the user's own action produced is a refusal, not a condition:
/// "Move selection cancelled" belongs in the Console and the status line, and
/// filing it here is what made the page a list of everything that had ever
/// gone slightly wrong. An error from that same channel is a real failure and
/// stays. Every other source — the engine, the netlister, the checks, the
/// system — reports conditions, so all of its errors and warnings stay.
fn log_problems(state: &AppState) -> impl Iterator<Item = &LogEntry> {
    state.log_buffer.entries().filter(|entry| {
        match entry.severity {
            LogSeverity::Error => true,
            LogSeverity::Warning => entry.source != LogSource::User,
            LogSeverity::Info | LogSeverity::Debug | LogSeverity::Trace => false,
        }
    })
}

/// The counts behind the tab strip's badge. Derived from exactly the rows the
/// page lists, so the badge can never disagree with the body under it.
pub(super) fn active_problem_count(state: &AppState) -> ProblemCount {
    if netlist_diagnostics_own_problems(state) {
        let summary = state.ui.netlist.diagnostics.summary();
        return ProblemCount {
            errors: summary.current_errors,
            advisories: summary.current_advisories(),
        };
    }
    let mut count = ProblemCount::default();
    if let CheckSource::Current(result) = check_source(state) {
        let summary = result.summary();
        count.errors += summary.critical + summary.errors;
        count.advisories += summary.warnings + summary.info;
    }
    for entry in log_problems(state) {
        match entry.severity {
            LogSeverity::Error => count.errors += 1,
            _ => count.advisories += 1,
        }
    }
    count
}

pub(super) fn problems(ui: &mut Ui, state: &mut AppState) {
    if netlist_diagnostics_own_problems(state) {
        netlist_problems(ui, state);
        return;
    }
    schematic_problems(ui, state);
}

/// The schematic half: check findings, then the conditions other producers
/// have reported through the log.
fn schematic_problems(ui: &mut Ui, state: &mut AppState) {
    let metrics = ColumnMetrics::measure(ui);
    let mut requested: Option<LogAnchor> = None;
    let mut rows = 0_usize;
    let mut note: Option<String> = None;

    egui::ScrollArea::vertical()
        .id_salt("workbench.problems")
        .show(ui, |ui| {
            let source = check_source(state);
            match &source {
                CheckSource::Unavailable(reason) => {
                    note = Some(format!("Schematic checks cannot run: {reason}"));
                }
                CheckSource::Stale(_) => {
                    note = Some(
                        "These findings describe the design before the last edit. Run the checks again."
                            .to_owned(),
                    );
                }
                CheckSource::NotRun | CheckSource::Current(_) => {}
            }
            let current = source.current();
            if let Some(result) = source.result() {
                for violation in result.violations() {
                    rows += 1;
                    // Asked before the row is sensed: a finding whose object
                    // the drawing no longer carries must not look like a jump.
                    let anchor = current
                        .then(|| {
                            crate::schematic::view::violations::location_anchor(
                                state,
                                &violation.location,
                            )
                        })
                        .flatten()
                        .filter(|anchor| state.log_anchor_refusal(anchor).is_none());
                    let response = problem_row(
                        ui,
                        &metrics,
                        ProblemRow {
                            problem: current.then(|| drc_problem(violation.severity)).flatten(),
                            severity: &severity_cell(severity_word(violation.severity), current),
                            location: &violation.location.display(),
                            message: &violation.message,
                            current,
                            target: anchor.is_some(),
                        },
                    );
                    if response.clicked() {
                        requested = anchor;
                    }
                    let fix = violation.violation_type.suggested_fix();
                    response.on_hover_text(format!("{}\n\n{fix}", violation.message));
                }
            }
            for entry in log_problems(state) {
                rows += 1;
                let anchor = entry
                    .anchor
                    .as_ref()
                    .filter(|anchor| state.log_anchor_refusal(anchor).is_none())
                    .cloned();
                let response = problem_row(
                    ui,
                    &metrics,
                    ProblemRow {
                        problem: log_problem_mark(entry.severity),
                        severity: &severity_cell(log_severity_name(entry.severity), true),
                        location: entry
                            .context
                            .as_deref()
                            .unwrap_or_else(|| entry.source.name()),
                        message: &entry.message,
                        current: true,
                        target: anchor.is_some(),
                    },
                );
                if response.clicked() {
                    requested = anchor;
                }
                response.on_hover_text(&entry.message);
            }
            if rows == 0 {
                muted(ui, empty_state(&check_source(state)));
            } else if let Some(note) = note.as_deref() {
                muted(ui, note);
            }
        });

    if let Some(anchor) = requested {
        state.jump_to_log_anchor(anchor);
    }
}

/// What the page says when it has nothing to list. "Nothing is wrong" and
/// "nothing has been checked" are different claims and only one of them can
/// be made honestly at a time.
fn empty_state(source: &CheckSource<'_>) -> &'static str {
    match source {
        CheckSource::NotRun => {
            "The schematic checks have not run. Run them to list what they find."
        }
        CheckSource::Unavailable(_) => "The schematic checks cannot run on this view.",
        CheckSource::Current(_) | CheckSource::Stale(_) => "No findings.",
    }
}

/// The netlist half. These diagnostics are already canonical records with
/// their own currency, so the page states it the same way it states the
/// schematic's.
fn netlist_problems(ui: &mut Ui, state: &mut AppState) {
    let diagnostics = std::sync::Arc::clone(&state.ui.netlist.diagnostics);
    if diagnostics.is_empty() {
        muted(ui, "No netlist diagnostics.");
        return;
    }
    let metrics = ColumnMetrics::measure(ui);
    let mut requested = None;
    egui::ScrollArea::vertical()
        .id_salt("workbench.problems.netlist")
        .show_rows(ui, CONSOLE_ROW_MIN_HEIGHT, diagnostics.len(), |ui, rows| {
            for index in rows {
                let Some(diagnostic) = diagnostics.get(index) else {
                    continue;
                };
                let current = diagnostic.is_current();
                let response = problem_row(
                    ui,
                    &metrics,
                    ProblemRow {
                        problem: current
                            .then(|| netlist_problem_mark(diagnostic.severity))
                            .flatten(),
                        severity: &severity_cell(
                            netlist_severity_name(diagnostic.severity),
                            current,
                        ),
                        location: &netlist_diagnostic_location(diagnostic),
                        message: &diagnostic.message,
                        current,
                        target: current,
                    },
                );
                if response.clicked() {
                    requested = Some(diagnostic.canonical.diagnostic_id);
                }
                if diagnostic.details.is_empty() {
                    response.on_hover_text(&diagnostic.message);
                } else {
                    response.on_hover_text(format!(
                        "{}\n\n{}",
                        diagnostic.message, diagnostic.details
                    ));
                }
            }
        });
    if let Some(diagnostic_id) = requested
        && let Err(error) = crate::workbench::documents::netlist_document::open_diagnostic_location(
            state,
            diagnostic_id,
        )
    {
        state.push_user_message(crate::diagnostics::ConsoleMessage::warning(error));
    }
}

/// One severity word, with its state said in front of it when the finding is
/// not about the design as it stands.
fn severity_cell(severity: &str, current: bool) -> String {
    if current {
        severity.to_owned()
    } else {
        format!("Stale {severity}")
    }
}

fn severity_word(severity: DrcSeverity) -> &'static str {
    severity.display_name()
}

fn drc_problem(severity: DrcSeverity) -> Option<Problem> {
    match severity {
        DrcSeverity::Critical | DrcSeverity::Error => Some(Problem::Failure),
        DrcSeverity::Warning => Some(Problem::Warning),
        DrcSeverity::Info => None,
    }
}

fn log_severity_name(severity: LogSeverity) -> &'static str {
    match severity {
        LogSeverity::Error => "Error",
        LogSeverity::Warning => "Warning",
        LogSeverity::Info => "Info",
        LogSeverity::Debug => "Debug",
        LogSeverity::Trace => "Trace",
    }
}

fn log_problem_mark(severity: LogSeverity) -> Option<Problem> {
    match severity {
        LogSeverity::Error => Some(Problem::Failure),
        LogSeverity::Warning => Some(Problem::Warning),
        LogSeverity::Info | LogSeverity::Debug | LogSeverity::Trace => None,
    }
}

fn netlist_severity_name(severity: DiagnosticSeverity) -> &'static str {
    match severity {
        DiagnosticSeverity::Error => "Error",
        DiagnosticSeverity::Warning => "Warning",
        DiagnosticSeverity::Info => "Info",
        DiagnosticSeverity::Hint => "Hint",
    }
}

fn netlist_problem_mark(severity: DiagnosticSeverity) -> Option<Problem> {
    match severity {
        DiagnosticSeverity::Error => Some(Problem::Failure),
        DiagnosticSeverity::Warning => Some(Problem::Warning),
        DiagnosticSeverity::Info | DiagnosticSeverity::Hint => None,
    }
}

/// Column widths for one pass of the page, measured in the font the cells are
/// painted in rather than assumed.
struct ColumnMetrics {
    severity: f32,
    location: f32,
    /// Width of one monospace character, which is what makes eliding a cell
    /// exact instead of iterative: in a monospace face the number of
    /// characters that fit is a division.
    advance: f32,
    ellipsis: f32,
}

impl ColumnMetrics {
    fn measure(ui: &Ui) -> Self {
        let severity = text_width(ui, WIDEST_SEVERITY_CELL, FontWeight::Regular);
        let advance = text_width(ui, "M", FontWeight::Regular).max(1.0);
        let ellipsis = text_width(ui, "…", FontWeight::Regular);
        let narrowest = text_width(ui, NARROWEST_LOCATION_CELL, FontWeight::Medium);
        let available = ui.available_width();
        let share = (available * LOCATION_COLUMN_MAX_SHARE).max(0.0);
        Self {
            severity,
            location: share.max(narrowest),
            advance,
            ellipsis,
        }
    }

    fn message_offset(&self) -> f32 {
        CONSOLE_MARK_GUTTER
            + self.severity
            + CONSOLE_COLUMN_GAP
            + self.location
            + CONSOLE_COLUMN_GAP
    }
}

fn text_width(ui: &Ui, text: &str, weight: FontWeight) -> f32 {
    ui.fonts_mut(|fonts| {
        fonts
            .layout_no_wrap(
                text.to_owned(),
                theme::mono(CONSOLE_FONT_SIZE, weight),
                egui::Color32::WHITE,
            )
            .size()
            .x
    })
}

struct ProblemRow<'a> {
    problem: Option<Problem>,
    severity: &'a str,
    location: &'a str,
    message: &'a str,
    current: bool,
    target: bool,
}

/// One finding: the mark gutter, the severity, where it is, and what it is.
///
/// Painted rather than built from widgets, so the row declares itself: a row
/// that can be followed is a keyboard target that announces as a button, and
/// a stale row is still text a screen reader should read.
fn problem_row(ui: &mut Ui, metrics: &ColumnMetrics, row: ProblemRow<'_>) -> egui::Response {
    let tokens = Tokens::get(ui.ctx());
    let (dim, body) = if row.current {
        (tokens.color.text_dim, tokens.color.text)
    } else {
        (tokens.color.text_faint, tokens.color.text_faint)
    };
    let message_offset = metrics.message_offset();
    let message_width = (ui.available_width() - message_offset).max(1.0);
    let message_galley = ui.painter().layout(
        row.message.to_owned(),
        theme::mono(CONSOLE_FONT_SIZE, FontWeight::Regular),
        body,
        message_width,
    );
    let height = message_galley.size().y.max(CONSOLE_ROW_MIN_HEIGHT);
    let sense = if row.target {
        // accessibility-pointer-shim: this only picks the sense. The row it
        // configures announces as a button and paints the focus ring for the
        // sense it is handed.
        Sense::click()
    } else {
        Sense::hover()
    };
    let (rect, response) =
        ui.allocate_exact_size(Vec2::new(ui.available_width(), height), sense);
    let label = announced_row(row.problem, &[row.severity, row.location, row.message]);
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            if row.target {
                egui::WidgetType::Button
            } else {
                egui::WidgetType::Label
            },
            ui.is_enabled(),
            label.clone(),
        )
    });
    if row.target && response.hovered() {
        ui.painter().rect_filled(rect, 0.0, tokens.color.bg_hover);
    }
    paint_row_mark(ui, rect.left(), rect.top(), row.problem);
    let center_y = rect.top() + CONSOLE_ROW_MIN_HEIGHT * 0.5;
    let severity_x = rect.left() + CONSOLE_MARK_GUTTER;
    let location_x = severity_x + metrics.severity + CONSOLE_COLUMN_GAP;
    ui.painter().text(
        egui::pos2(severity_x, center_y),
        egui::Align2::LEFT_CENTER,
        row.severity,
        theme::mono(CONSOLE_FONT_SIZE, FontWeight::Regular),
        dim,
    );
    // A location is read from its tail: the file and the line matter, the
    // directories above them do not, so what gets dropped is the head.
    ui.painter().text(
        egui::pos2(location_x, center_y),
        egui::Align2::LEFT_CENTER,
        &elide_head(row.location, metrics),
        theme::mono(CONSOLE_FONT_SIZE, FontWeight::Medium),
        dim,
    );
    ui.painter().galley(
        egui::pos2(rect.left() + message_offset, rect.top()),
        message_galley,
        body,
    );
    if row.target {
        theme::paint_focus_ring(ui, &response, rect);
        return response.on_hover_cursor(egui::CursorIcon::PointingHand);
    }
    response
}

/// Fit `text` into the location column, dropping characters from the front
/// and marking the cut with an ellipsis. Exact rather than iterative because
/// the face is monospace.
fn elide_head(text: &str, metrics: &ColumnMetrics) -> String {
    let characters = text.chars().count();
    let width = characters as f32 * metrics.advance;
    if width <= metrics.location {
        return text.to_owned();
    }
    let room = (metrics.location - metrics.ellipsis).max(0.0);
    let fits = (room / metrics.advance).floor().max(0.0) as usize;
    if fits == 0 {
        return "…".to_owned();
    }
    let skip = characters.saturating_sub(fits);
    let tail = text.chars().skip(skip).collect::<String>();
    format!("…{tail}")
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The severity column is sized to the longest word it can print. Before
    /// this it was a fixed 58 px, which cut "Stale Warning" to eight
    /// characters, and the location column was 62 px, which could not hold
    /// any location this page produces.
    #[test]
    fn every_severity_cell_fits_the_widest_measured_cell() {
        let mut widest = String::new();
        for severity in [
            DrcSeverity::Critical,
            DrcSeverity::Error,
            DrcSeverity::Warning,
            DrcSeverity::Info,
        ] {
            for current in [true, false] {
                let cell = severity_cell(severity_word(severity), current);
                if cell.chars().count() > widest.chars().count() {
                    widest = cell;
                }
            }
        }
        for severity in [
            DiagnosticSeverity::Error,
            DiagnosticSeverity::Warning,
            DiagnosticSeverity::Info,
            DiagnosticSeverity::Hint,
        ] {
            for current in [true, false] {
                let cell = severity_cell(netlist_severity_name(severity), current);
                assert!(
                    cell.chars().count() <= WIDEST_SEVERITY_CELL.chars().count(),
                    "{cell} is wider than the cell the column is measured against",
                );
            }
        }
        for severity in [
            LogSeverity::Error,
            LogSeverity::Warning,
            LogSeverity::Info,
            LogSeverity::Debug,
            LogSeverity::Trace,
        ] {
            let cell = severity_cell(log_severity_name(severity), false);
            assert!(
                cell.chars().count() <= WIDEST_SEVERITY_CELL.chars().count(),
                "{cell} is wider than the cell the column is measured against",
            );
        }
        assert_eq!(
            widest.chars().count(),
            WIDEST_SEVERITY_CELL.chars().count(),
            "the measured cell must be the longest one the page can print",
        );
    }

    #[test]
    fn a_location_that_fits_is_printed_whole() {
        let metrics = ColumnMetrics {
            severity: 0.0,
            location: 100.0,
            advance: 7.0,
            ellipsis: 7.0,
        };
        assert_eq!(elide_head("Component M1", &metrics), "Component M1");
    }

    /// The tail carries the file and the line, so the head is what goes.
    #[test]
    fn a_long_location_keeps_its_tail() {
        let metrics = ColumnMetrics {
            severity: 0.0,
            location: 70.0,
            advance: 7.0,
            ellipsis: 7.0,
        };
        let elided = elide_head("models/analog/opamp.sp · line 42", &metrics);
        assert!(elided.starts_with('…'), "{elided} must mark the cut");
        assert!(elided.ends_with("line 42"), "{elided} must keep its tail");
        assert!(
            elided.chars().count() <= 10,
            "{elided} must fit the column it was measured against",
        );
    }

    #[test]
    fn counts_name_their_severities_separately() {
        let count = ProblemCount {
            errors: 1,
            advisories: 12,
        };
        assert_eq!(count.total(), 13);
        assert_eq!(count.announcement(), "1 error, 12 advisories");
    }
}
