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
//! The page is one dense table with a toolbar over it and one detail region
//! under it. Columns are sized to their content, measured in the font they are
//! printed in; they used to be fixed at 58 px and 62 px — eight characters —
//! which was enough for the Console's `DRC` source tag and nowhere near enough
//! for `Component M1`. One row is one line: the message is elided in the table
//! and stated in full in the detail region, which is also what lets the body
//! scroll by a row height that is actually a row's height.
//!
//! The current row is not the page's own: it is the one current finding, held
//! by canonical identity in the store, and the canvas finding tour stands on
//! exactly the same one.

mod arrange;
mod detail;
mod grid;
mod strip;
#[cfg(test)]
mod tests;

use egui::{Key, Modifiers, Ui, Vec2};

use super::{AppState, CONSOLE_ROW_MIN_HEIGHT, muted};
use crate::workbench::app_state::diagnostic_store::listing::{
    DiagnosticListing, DiagnosticRow, EmptyReason, listing,
};
use crate::workbench::app_state::diagnostic_store::{DiagnosticLevel, DiagnosticRoute};
use crate::workbench::app_state::problems_view::ProblemsView;

use crate::ui::widgets::Problem;
use arrange::{Arrangement, Line};
use grid::Columns;

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

/// What one frame of the page asks the session to change.
///
/// The page holds the list it is drawing, which borrows the session; the
/// session cannot be moved while that borrow stands. Collecting the frame's
/// intent and applying it afterwards is what keeps a page from being a place
/// where anything can happen.
#[derive(Default)]
struct PageIntent {
    route: Option<DiagnosticRoute>,
    /// A finding whose producer-offered edits the reader asked to apply.
    fix: Option<uuid::Uuid>,
    clipboard: Option<String>,
}

/// The current row, as this frame is reading and editing it.
///
/// `finding` is the one piece of state the canvas tour shares; the other two
/// are this frame's projection of it onto the rows the page is printing.
struct Selection {
    /// Which listed row it is, when the filters are listing it at all.
    listed: Option<usize>,
    finding: Option<uuid::Uuid>,
    /// A printed line the body has to bring into view, because a key rather
    /// than the reader's own scrolling moved the selection to it.
    reveal: Option<usize>,
}

pub(super) fn problems(ui: &mut Ui, state: &mut AppState) {
    let mut view = state.problems_view.clone();
    let mut selection = Selection {
        listed: None,
        finding: state.diagnostics.current_finding(),
        reveal: None,
    };
    let mut intent = PageIntent::default();
    {
        let listed = listing(state);
        let total = listed.len();
        if total == 0 {
            // Nothing to list is not a table with no rows: it is one claim,
            // and a toolbar over it would offer to filter nothing.
            muted(ui, &empty_state(listed.empty_reason()));
            return;
        }
        let arrangement = Arrangement::build(&listed, &view, selection.finding);
        let columns = Columns::measure(
            ui,
            arrangement.tally.widest_code,
            arrangement.tally.widest_producer,
        );
        strip::filter_strip(ui, &mut view, &arrangement.tally);
        if view.is_filtered() && strip::filtered_out(ui, arrangement.listed, total) {
            view = ProblemsView {
                sort: view.sort,
                descending: view.descending,
                grouping: view.grouping,
                collapsed: view.collapsed.clone(),
                explain: view.explain,
                ..ProblemsView::default()
            };
        }
        if let Some(column) = grid::header(ui, &columns, &view) {
            view.order_by(column);
        }

        selection.listed = arrangement.current;
        keyboard(
            ui,
            &listed,
            &arrangement,
            state,
            &mut selection,
            &mut intent,
        );
        let note = note(&listed);
        let detail_height = selection
            .listed
            .map_or(0.0, |_| detail::height(ui.available_height()));
        let note_height = if note.is_some() {
            CONSOLE_ROW_MIN_HEIGHT
        } else {
            0.0
        };
        let table_height =
            (ui.available_height() - detail_height - note_height).max(CONSOLE_ROW_MIN_HEIGHT);
        // What the table's height was reserved against. A click inside the
        // table can select a row the height was not reserved for, and drawing
        // the region anyway would paint it over the rows it displaces: the
        // frame that changes the selection asks for another one instead.
        let reserved_for = selection.listed;
        table(
            ui,
            Table {
                listed: &listed,
                arrangement: &arrangement,
                columns: &columns,
                height: table_height,
                reveal: selection.reveal,
            },
            &mut view,
            &mut selection,
            &mut intent,
            state,
        );
        if let Some(note) = note {
            muted(ui, note);
        }
        if selection.listed != reserved_for {
            ui.ctx().request_repaint();
        } else if let Some(row) = selection.listed.and_then(|index| listed.row(index)) {
            let refusal = row
                .route
                .as_ref()
                .and_then(|route| route_refusal(state, route));
            let actions = ui
                .allocate_ui_with_layout(
                    Vec2::new(ui.available_width(), detail_height),
                    egui::Layout::top_down(egui::Align::Min),
                    |ui| {
                        detail::detail(
                            ui,
                            &row,
                            view.explain,
                            arrangement.listed,
                            refusal.as_deref(),
                        )
                    },
                )
                .inner;
            if actions.explain {
                view.explain = !view.explain;
            }
            if actions.follow && refusal.is_none() {
                intent.route = row.route.clone();
            }
            if let Some(route) = actions.related {
                intent.route = Some(route);
            }
            if actions.quick_fix {
                intent.fix = Some(row.id);
            }
            if actions.copy_row {
                intent.clipboard = Some(copy_of(&row));
            }
            if actions.copy_listed {
                intent.clipboard = Some(copy_of_listed(&listed, &arrangement));
            }
        }
    }

    state.problems_view = view;
    state.diagnostics.set_current_finding(selection.finding);
    if let Some(route) = intent.route {
        state.diagnostics.request_route(route);
    }
    if let Some(text) = intent.clipboard {
        ui.ctx().copy_text(text);
    }
    if let Some(finding) = intent.fix {
        match crate::workbench::documents::netlist_document::language::apply_quick_fix_for_finding(
            state, finding,
        ) {
            Ok(applied) => {
                state.push_user_message(crate::diagnostics::ConsoleMessage::info(applied));
            }
            Err(refused) => {
                state.push_user_message(crate::diagnostics::ConsoleMessage::warning(refused));
            }
        }
    }
}

/// Everything the table needs to draw itself, so the row loop takes one
/// argument per concern rather than nine positional ones.
struct Table<'a, 'listing> {
    listed: &'a DiagnosticListing<'listing>,
    arrangement: &'a Arrangement<'listing>,
    columns: &'a Columns,
    height: f32,
    /// A line the keyboard moved to, which the body scrolls into view.
    reveal: Option<usize>,
}

/// Where the body last scrolled to, so a keyboard move scrolls the least it
/// can rather than re-centring the list under the reader on every press.
fn drawn_range_id() -> egui::Id {
    egui::Id::new("workbench.problems.drawn")
}

fn table(
    ui: &mut Ui,
    table: Table<'_, '_>,
    view: &mut ProblemsView,
    selection: &mut Selection,
    intent: &mut PageIntent,
    state: &AppState,
) {
    let lines = table.arrangement.lines();
    let previous: Option<std::ops::Range<usize>> = ui.data(|data| data.get_temp(drawn_range_id()));
    let mut area = egui::ScrollArea::vertical()
        .id_salt("workbench.problems")
        .max_height(table.height);
    // The pitch the body lays rows out at, which is the row height plus
    // whatever the enclosing dock spaces its rows by — the same arithmetic
    // `show_rows` does, read from the same place rather than assumed.
    let pitch = CONSOLE_ROW_MIN_HEIGHT + ui.spacing().item_spacing.y;
    if let Some(offset) = reveal_offset(table.reveal, previous.as_ref(), table.height, pitch) {
        area = area.vertical_scroll_offset(offset);
    }
    let drawn = area
        .show_rows(ui, CONSOLE_ROW_MIN_HEIGHT, lines, |ui, range| {
            for line in range.clone() {
                match table.arrangement.line(line) {
                    Some(Line::Group(group)) => {
                        if let Some(group) = table.arrangement.group(group)
                            && grid::group_header(ui, table.columns, group)
                        {
                            view.collapse(&group.label, !group.collapsed);
                        }
                    }
                    Some(Line::Row(index)) => {
                        finding_row(ui, &table, index, selection, intent, state);
                    }
                    None => {}
                }
            }
            range
        })
        .inner;
    ui.data_mut(|data| data.insert_temp(drawn_range_id(), drawn));
}

/// One finding's row, with the click that selects it and the click that
/// follows it.
fn finding_row(
    ui: &mut Ui,
    table: &Table<'_, '_>,
    index: usize,
    selection: &mut Selection,
    intent: &mut PageIntent,
    state: &AppState,
) {
    let Some(row) = table.listed.row(index) else {
        return;
    };
    // Asked before the row is drawn: a finding whose object the drawing no
    // longer carries must not look like a jump.
    let route = row
        .route
        .clone()
        .filter(|route| route_refusal(state, route).is_none());
    let response = grid::row(
        ui,
        table.columns,
        &grid::RowCells {
            problem: row.current().then(|| level_mark(row.level)).flatten(),
            severity: &severity_cell(&row),
            code: row.code,
            subject: &row.subject,
            producer: row.producer,
            location: &row.location,
            message: row.message,
            current: row.current(),
            target: route.is_some(),
            selected: selection.finding == Some(row.id),
        },
        grid::row_id(row.id),
    );
    if response.clicked() {
        selection.finding = Some(row.id);
        selection.listed = Some(index);
        response.request_focus();
        intent.route = route;
    } else if response.gained_focus() {
        // Tab lands on a row; the row it lands on is the one being read.
        selection.finding = Some(row.id);
        selection.listed = Some(index);
    }
    response.on_hover_text(hover(&row));
}

/// How far the body has to scroll to bring `reveal` into view, or `None` when
/// it is already in it.
fn reveal_offset(
    reveal: Option<usize>,
    previous: Option<&std::ops::Range<usize>>,
    height: f32,
    pitch: f32,
) -> Option<f32> {
    let line = reveal?;
    let previous = previous?;
    if line < previous.start {
        return Some(line as f32 * pitch);
    }
    if line + 1 >= previous.end {
        return Some(((line + 1) as f32 * pitch - height).max(0.0));
    }
    None
}

/// Move the current row, follow it, or clear it.
///
/// The keys are taken only while the current row holds focus, so the page
/// never swallows an arrow key from the surface a reader is actually in. They
/// are consumed rather than read, so a row that handles Enter does not also
/// hand it to whatever is listening behind the dock.
fn keyboard(
    ui: &Ui,
    listed: &DiagnosticListing<'_>,
    arrangement: &Arrangement<'_>,
    state: &AppState,
    selection: &mut Selection,
    intent: &mut PageIntent,
) {
    let standing = selection.finding.map(grid::row_id);
    let focused = standing.is_some_and(|id| ui.memory(|memory| memory.focused() == Some(id)));
    if !focused {
        return;
    }
    let (down, up, enter, escape) = ui.input_mut(|input| {
        (
            input.consume_key(Modifiers::NONE, Key::ArrowDown),
            input.consume_key(Modifiers::NONE, Key::ArrowUp),
            input.consume_key(Modifiers::NONE, Key::Enter),
            input.consume_key(Modifiers::NONE, Key::Escape),
        )
    });
    if escape {
        // The row keeps the arrows while it is focused, so a row that is no
        // longer current has to give the focus back as well: a focused row
        // with nothing selected would hold keys that nothing acts on.
        if let Some(id) = standing {
            ui.memory_mut(|memory| memory.surrender_focus(id));
        }
        selection.listed = None;
        selection.finding = None;
        return;
    }
    let step = isize::from(down) - isize::from(up);
    if step != 0
        && let Some(next) = arrangement.step(selection.listed, step)
    {
        selection.listed = Some(next);
        selection.finding = listed.facet(next).map(|facet| facet.id);
        selection.reveal = arrangement.line_of(next);
        if let Some(id) = selection.finding {
            ui.memory_mut(|memory| memory.request_focus(grid::row_id(id)));
        }
    }
    if enter
        && let Some(row) = selection.listed.and_then(|index| listed.row(index))
        && let Some(route) = row.route
        && route_refusal(state, &route).is_none()
    {
        intent.route = Some(route);
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
        EmptyReason::OutOfDate => {
            "No findings before the last edit. Run the checks again.".to_owned()
        }
        EmptyReason::NoFindings => "No findings.".to_owned(),
    }
}

/// One severity word, with its state said in front of it when the finding is
/// not about the design as it stands, and with a waiver said where one is
/// recorded — a waived finding is a decision, and the page says so.
fn severity_cell(row: &DiagnosticRow<'_>) -> String {
    let severity = row.level.display_name();
    match (row.current(), row.waived()) {
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

/// One row as text, tab-separated in the column order the grid prints — the
/// shape that pastes into a sheet or a report as a row rather than as a
/// sentence somebody has to take apart again.
fn copy_of(row: &DiagnosticRow<'_>) -> String {
    [
        severity_cell(row),
        row.code.to_owned(),
        row.subject.clone(),
        row.producer.to_owned(),
        row.location.clone(),
        row.message.to_owned(),
    ]
    .join("\t")
}

/// Every listed row, with the heading the columns carry.
fn copy_of_listed(listed: &DiagnosticListing<'_>, arrangement: &Arrangement<'_>) -> String {
    use crate::workbench::app_state::problems_view::ProblemColumn;

    let mut text = ProblemColumn::ALL
        .iter()
        .map(|column| column.label())
        .collect::<Vec<_>>()
        .join("\t");
    for index in arrangement.rows() {
        if let Some(row) = listed.row(index) {
            text.push('\n');
            text.push_str(&copy_of(&row));
        }
    }
    text
}

const fn level_mark(level: DiagnosticLevel) -> Option<Problem> {
    match level {
        DiagnosticLevel::Critical | DiagnosticLevel::Error => Some(Problem::Failure),
        DiagnosticLevel::Warning => Some(Problem::Warning),
        DiagnosticLevel::Info | DiagnosticLevel::Hint => None,
    }
}
