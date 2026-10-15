//! The Problems page: the conditions that currently stand against the design.
//!
//! A problem is a condition, not an event. That is the whole difference
//! between this page and the Console beside it: the Console records what
//! happened in the session, in the order it happened, and keeps it after it
//! stops being true; this page states what is wrong with the design right
//! now, and a finding leaves it when the design stops earning it.
//!
//! Everything here follows from that. A finding is reported once, by whatever
//! owns the condition, and never copied into the log as well — the page does
//! not read the log at all, which is what stopped it from being a severity
//! filter over the session's history. A finding whose inputs have since
//! changed is shown faint and said to be stale rather than quietly dropped,
//! because "no findings" and "nobody has looked" are different answers and a
//! tool that confuses them is lying. A finding that knows where it is, is a
//! target: clicking it goes there.
//!
//! Columns are sized to their content, measured in the font they are printed
//! in. They used to be fixed at 58 px and 62 px — eight characters — which
//! was enough for the Console's `DRC` source tag and nowhere near enough for
//! `Component M1` or `opamp.sp · line 42 · column 7`.

use egui::{Sense, Ui, Vec2};

use super::{
    AppState, CONSOLE_COLUMN_GAP, CONSOLE_FONT_SIZE, CONSOLE_MARK_GUTTER, CONSOLE_ROW_MIN_HEIGHT,
    announced_row, muted, paint_row_mark,
};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::Tokens;
use crate::ui::widgets::Problem;
use crate::workbench::app_state::diagnostic_store::listing::{
    DiagnosticListing, DiagnosticRow, EmptyReason, listing,
};
use crate::workbench::app_state::diagnostic_store::{DiagnosticLevel, DiagnosticRoute};

/// The longest severity cell the page can print. The severity column is
/// measured against this, so no severity word is ever cut.
const WIDEST_SEVERITY_CELL: &str = "Waived Critical";

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
pub(in crate::workbench) struct ProblemCount {
    pub(in crate::workbench) errors: usize,
    pub(in crate::workbench) advisories: usize,
}

impl ProblemCount {
    pub(in crate::workbench) const fn total(self) -> usize {
        self.errors + self.advisories
    }

    /// The counts in words, for the tab's accessible name and its tooltip.
    /// The badge itself has room for one number; this is where the number is
    /// broken down, so the strip never implies that twelve advisories are
    /// twelve errors.
    pub(in crate::workbench) fn announcement(self) -> String {
        format!(
            "{} error{}, {} advisor{}",
            self.errors,
            if self.errors == 1 { "" } else { "s" },
            self.advisories,
            if self.advisories == 1 { "y" } else { "ies" },
        )
    }
}

/// The counts behind the tab strip's badge. Derived from exactly the rows the
/// page lists, so the badge can never disagree with the body under it.
pub(in crate::workbench) fn active_problem_count(state: &AppState) -> ProblemCount {
    let counts = listing(state).counts();
    ProblemCount {
        errors: counts.errors,
        advisories: counts.advisories,
    }
}

pub(super) fn problems(ui: &mut Ui, state: &mut AppState) {
    let metrics = ColumnMetrics::measure(ui);
    let mut requested: Option<DiagnosticRoute> = None;
    let listed = listing(state);
    let rows = listed.len();
    if rows == 0 {
        muted(ui, &empty_state(listed.empty_reason()));
        return;
    }
    let note = note(&listed);

    egui::ScrollArea::vertical()
        .id_salt("workbench.problems")
        .show_rows(ui, CONSOLE_ROW_MIN_HEIGHT, rows, |ui, range| {
            for index in range {
                let Some(row) = listed.row(index) else {
                    continue;
                };
                // Asked before the row is sensed: a finding whose object the
                // drawing no longer carries must not look like a jump.
                let route = row
                    .route
                    .clone()
                    .filter(|route| route_refusal(state, route).is_none());
                let response = problem_row(
                    ui,
                    &metrics,
                    ProblemRow {
                        problem: row.current.then(|| level_mark(row.level)).flatten(),
                        severity: &severity_cell(&row),
                        location: &row.subject,
                        message: row.message,
                        current: row.current,
                        target: route.is_some(),
                    },
                );
                if response.clicked() {
                    requested = route;
                }
                response.on_hover_text(hover(&row));
            }
        });
    if let Some(note) = note {
        muted(ui, note);
    }

    if let Some(route) = requested {
        state.diagnostics.request_route(route);
    }
}

/// Why a row carrying `route` cannot be followed, or `None` if it can.
///
/// Only an anchor resolved by name can have lost its target since the finding
/// was made; the rest address a document or a workspace destination that the
/// route itself re-resolves when it is applied.
fn route_refusal(state: &AppState, route: &DiagnosticRoute) -> Option<String> {
    match route {
        DiagnosticRoute::Anchor(anchor) => state.log_anchor_refusal(anchor),
        DiagnosticRoute::NetlistDiagnostic(_)
        | DiagnosticRoute::CodeDocument { .. }
        | DiagnosticRoute::Repair(_) => None,
    }
}

/// What the page says above the rows when something qualifies all of them.
fn note(listed: &DiagnosticListing<'_>) -> Option<&'static str> {
    if listed.unavailable().is_some() {
        return Some(
            "Some checks cannot be evaluated on this design. The findings below are what the rest reported.",
        );
    }
    listed
        .has_stale()
        .then_some("Faint findings describe the design before the last edit. Run the checks again.")
}

/// What the page says when it has nothing to list. "Nothing is wrong" and
/// "nothing has been checked" are different claims and only one of them can
/// be made honestly at a time.
fn empty_state(reason: &EmptyReason) -> String {
    match reason {
        EmptyReason::NoProject => "No project is open.".to_owned(),
        EmptyReason::NothingRan => {
            "Nothing has been checked. Run the checks to list what they find.".to_owned()
        }
        EmptyReason::Unavailable(reason) => format!("The checks cannot run: {reason}"),
        EmptyReason::NoFindings => "No findings.".to_owned(),
    }
}

/// One severity word, with its state said in front of it when the finding is
/// not about the design as it stands, and with a waiver said where one is
/// recorded — a waived finding is a decision, and the page says whose.
fn severity_cell(row: &DiagnosticRow<'_>) -> String {
    let severity = row.level.display_name();
    match (row.current, row.waived) {
        (true, false) => severity.to_owned(),
        (true, true) => format!("Waived {severity}"),
        (false, _) => format!("Stale {severity}"),
    }
}

/// The whole finding, for a reader who stopped on the row: what it is, what
/// to do about it, and the rule it is an instance of. The rule is last and
/// said as a rule, because it is what a reader names when they go looking for
/// why it fired or to waive it.
fn hover(row: &DiagnosticRow<'_>) -> String {
    let mut text = row.message.to_owned();
    if !row.details.is_empty() {
        text.push_str("\n\n");
        text.push_str(row.details);
    }
    text.push_str("\n\nRule ");
    text.push_str(row.code);
    text
}

const fn level_mark(level: DiagnosticLevel) -> Option<Problem> {
    match level {
        DiagnosticLevel::Critical | DiagnosticLevel::Error => Some(Problem::Failure),
        DiagnosticLevel::Warning => Some(Problem::Warning),
        DiagnosticLevel::Info | DiagnosticLevel::Hint => None,
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
    let (rect, response) = ui.allocate_exact_size(Vec2::new(ui.available_width(), height), sense);
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
        elide_head(row.location, metrics),
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
        for level in [
            DiagnosticLevel::Critical,
            DiagnosticLevel::Error,
            DiagnosticLevel::Warning,
            DiagnosticLevel::Info,
            DiagnosticLevel::Hint,
        ] {
            for (current, waived) in [(true, false), (true, true), (false, false)] {
                let cell = severity_cell(&DiagnosticRow {
                    code: "T-1",
                    level,
                    current,
                    waived,
                    subject: String::new(),
                    message: "",
                    details: "",
                    route: None,
                });
                assert!(
                    cell.chars().count() <= WIDEST_SEVERITY_CELL.chars().count(),
                    "{cell} is wider than the cell the column is measured against",
                );
                if cell.chars().count() > widest.chars().count() {
                    widest = cell;
                }
            }
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

    /// Every cell the page paints, with the x it was painted at.
    fn painted_cells(state: &mut AppState, width: f32) -> Vec<(String, f32)> {
        fn scan(shape: &egui::epaint::Shape, found: &mut Vec<(String, f32)>) {
            match shape {
                egui::epaint::Shape::Text(text) => {
                    found.push((text.galley.job.text.clone(), text.pos.x));
                }
                egui::epaint::Shape::Vec(shapes) => {
                    for shape in shapes {
                        scan(shape, found);
                    }
                }
                _ => {}
            }
        }
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        let input = egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(width, 400.0),
            )),
            ..Default::default()
        };
        let output = ctx.run_ui(input, |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| problems(ui, state));
        });
        let mut found = Vec::new();
        for clipped in &output.shapes {
            scan(&clipped.shape, &mut found);
        }
        found
    }

    /// The columns the page actually paints, measured rather than eyeballed.
    ///
    /// Severity, location and message each start where the one before it ends
    /// plus the gap, the location keeps its tail when it is too long for its
    /// column, and no cell is painted on top of another. The old row clipped
    /// severity at a fixed 58 px and location at 62 px — eight characters —
    /// so a location was unreadable and a long severity word lost its end.
    #[test]
    fn the_painted_columns_start_where_the_measurements_say() {
        use rspice_design::drc::{DrcLocation, DrcResult, DrcViolation, DrcViolationType};

        let mut state = AppState::default();
        state.project_lifecycle.authority.open_session();
        let mut result = DrcResult::new();
        result.add_violation(DrcViolation::new(
            1,
            DrcViolationType::FloatingNode,
            "Node 'bias_reference_ladder_tap' has only 1 connection(s)",
            DrcLocation::Node {
                net_name: "bias_reference_ladder_tap".to_owned(),
            },
        ));
        result.completed = true;
        state
            .publish_active_design_check_result(result)
            .expect("the fixture publishes a current receipt");

        let width = 900.0;
        let cells = painted_cells(&mut state, width);
        let severity = cells
            .iter()
            .find(|(text, _)| text == "Error")
            .expect("the severity cell is painted");
        let location = cells
            .iter()
            .find(|(text, _)| text.contains("bias_reference") || text.starts_with('…'))
            .expect("the location cell is painted");
        let message = cells
            .iter()
            .find(|(text, _)| text.contains("only 1 connection"))
            .expect("the message cell is painted");

        assert!(
            location.1 > severity.1,
            "location at {} must start after severity at {}",
            location.1,
            severity.1,
        );
        assert!(
            message.1 > location.1,
            "message at {} must start after location at {}",
            message.1,
            location.1,
        );
        // The gap between two column starts is the earlier column's width
        // plus one gap, so a cell can never be painted over its neighbour.
        assert!(
            message.1 - location.1 >= CONSOLE_COLUMN_GAP,
            "columns must not overlap: {} then {}",
            location.1,
            message.1,
        );
        assert!(
            location.0.contains("bias_reference_ladder_tap") || location.0.ends_with("ladder_tap'"),
            "the location must keep the part that says where to go, got {:?}",
            location.0,
        );
    }

    /// The page is not a severity filter over the session log.
    ///
    /// Before the canonical store it read the log buffer, so every refusal,
    /// engine warning and system notice that had ever been printed stood here
    /// as a permanent problem — about 715 refusal sites' worth.
    #[test]
    fn a_logged_failure_is_not_a_problem() {
        let mut state = AppState::default();
        state.project_lifecycle.authority.open_session();
        state.log_buffer.log(
            crate::diagnostics::LogSeverity::Error,
            crate::diagnostics::LogSource::Engine,
            "the engine printed a failure",
            None,
        );

        assert_eq!(active_problem_count(&state), ProblemCount::default());
        let cells = painted_cells(&mut state, 900.0);
        assert!(
            cells
                .iter()
                .all(|(text, _)| !text.contains("the engine printed a failure")),
            "the page lists records, not log entries: {cells:?}",
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
