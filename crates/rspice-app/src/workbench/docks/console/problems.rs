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
//!
//! The table is one stop in the keyboard's Tab order, not one per row. Inside
//! it the arrows, Home, End and the page keys move the current finding, Enter
//! follows it and Escape lets it go; Tab leaves. Following a finding — by
//! Enter or by a click — lets go of the keyboard, and a finding followed to
//! the drawing gives it to the canvas, so the canvas's own keys act on what
//! the finding named without a second click to reach it.

mod arrange;
mod detail;
mod grid;
mod strip;
#[cfg(test)]
mod tests;

use std::borrow::Cow;

use egui::{Key, Modifiers, Ui, Vec2};

use super::{AppState, CONSOLE_FONT_SIZE, CONSOLE_ROW_MIN_HEIGHT, muted};
use crate::diagnostics::LogAnchor;
use crate::ui::theme::{self, FontWeight};
use crate::workbench::app_state::diagnostic_store::listing::{
    DiagnosticListing, DiagnosticRow, EmptyReason, RowStanding, listing,
};
use crate::workbench::app_state::diagnostic_store::{DiagnosticLevel, DiagnosticRoute};
use crate::workbench::app_state::problems_view::ProblemsView;
use crate::workbench::app_state::session::shortcuts::{
    report_escape_owner, request_engineering_canvas_focus,
};

use crate::ui::widgets::Problem;
use arrange::{Arrangement, Line};
use grid::{Columns, RowMark};

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
/// `finding` is the one piece of state the canvas tour shares; `listed` is
/// this frame's projection of it onto the rows the page is printing.
struct Selection {
    /// Which listed row it is, when the filters are listing it at all.
    listed: Option<usize>,
    finding: Option<uuid::Uuid>,
}

impl Selection {
    fn clear(&mut self) {
        self.listed = None;
        self.finding = None;
    }
}

/// What the body was when it was last drawn: the finding it stood on, the
/// height it was given and where it was scrolled to.
///
/// The body brings the current row into view when that row moved under it —
/// by a key, by a click or by the canvas tour — and when it lost height to the
/// detail region a new selection opened. It leaves the reader's own scrolling
/// alone otherwise.
#[derive(Debug, Clone, Copy, Default)]
struct BodyMemory {
    finding: Option<uuid::Uuid>,
    height: f32,
    offset: f32,
}

impl BodyMemory {
    fn id() -> egui::Id {
        egui::Id::new("workbench.problems.body")
    }

    fn load(ctx: &egui::Context) -> Self {
        ctx.data(|data| data.get_temp(Self::id()))
            .unwrap_or_default()
    }

    fn store(self, ctx: &egui::Context) {
        ctx.data_mut(|data| data.insert_temp(Self::id(), self));
    }
}

pub(super) fn problems(ui: &mut Ui, state: &mut AppState) {
    let mut view = state.problems_view.clone();
    let mut selection = Selection {
        listed: None,
        finding: state.diagnostics.current_finding(),
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
        let (arrangement, current) = arrange::arrange(ui.ctx(), &listed, &view, selection.finding);
        let columns = Columns::measure(
            ui,
            arrangement.tally.widest_code,
            arrangement.tally.widest_producer,
            listed.unavailable().is_some(),
        );
        strip::filter_strip(ui, &mut view, &arrangement.tally);
        if view.is_filtered() && strip::filtered_out(ui, arrangement.listed, total) {
            view.clear_filters();
        }
        if let Some(column) = grid::header(ui, &columns, &view) {
            view.order_by(column);
        }

        selection.listed = current;
        let body = BodyMemory::load(ui.ctx());
        keyboard(
            ui,
            &listed,
            &arrangement,
            state,
            &body,
            &mut view,
            &mut selection,
            &mut intent,
        );
        let note = note(&listed);
        let detail_height = selection
            .listed
            .map_or(0.0, |_| detail::height(ui.available_height()));
        let note_height = note.as_deref().map_or(0.0, |note| note_height(ui, note));
        let table_height =
            (ui.available_height() - detail_height - note_height).max(CONSOLE_ROW_MIN_HEIGHT);
        // What the table's height was reserved against. A click inside the
        // table can select a row the height was not reserved for, and drawing
        // the region anyway would paint it over the rows it displaces: the
        // frame that changes the selection asks for another one instead.
        let reserved_for = selection.listed;
        table(
            ui,
            &Table {
                listed: &listed,
                arrangement: &arrangement,
                columns: &columns,
                height: table_height,
            },
            body,
            &mut view,
            &mut selection,
            &mut intent,
            state,
        );
        if let Some(note) = note {
            muted(ui, &note);
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
                            detail::Listed {
                                count: arrangement.listed,
                                collapsed: arrangement.hides_collapsed_rows(),
                            },
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
        if lands_on_canvas(&route) {
            request_engineering_canvas_focus(ui.ctx());
        }
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
    arrangement: &'a Arrangement,
    columns: &'a Columns,
    height: f32,
}

/// How a pressed row left the keyboard.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Press {
    /// It was followed into another surface, which takes the keyboard.
    Followed,
    /// It had nowhere to go, so the reader stays in the table.
    Stayed,
}

fn table(
    ui: &mut Ui,
    table: &Table<'_, '_>,
    body: BodyMemory,
    view: &mut ProblemsView,
    selection: &mut Selection,
    intent: &mut PageIntent,
    state: &AppState,
) {
    let mut area = egui::ScrollArea::vertical()
        .id_salt("workbench.problems")
        .max_height(table.height);
    // The pitch the body lays rows out at, which is the row height plus
    // whatever the enclosing dock spaces its rows by — the same arithmetic
    // `show_rows` does, read from the same place rather than assumed.
    let pitch = CONSOLE_ROW_MIN_HEIGHT + ui.spacing().item_spacing.y;
    let moved = body.finding != selection.finding;
    let shrank = table.height + 0.5 < body.height;
    if (moved || shrank)
        && let Some(line) = selection
            .listed
            .and_then(|index| table.arrangement.line_of(index))
        && let Some(offset) = reveal_offset(line, body.offset, table.height, pitch)
    {
        area = area.vertical_scroll_offset(offset);
    }
    let mut press = None;
    let mut current_row = None;
    let output = area.show_rows(
        ui,
        CONSOLE_ROW_MIN_HEIGHT,
        table.arrangement.lines(),
        |ui, range| {
            for line in range {
                match table.arrangement.line(line) {
                    Some(Line::Group(group)) => {
                        if let Some(group) = table.arrangement.group(group)
                            && grid::group_header(ui, table.columns, group)
                        {
                            view.collapse(&group.label, !group.collapsed);
                        }
                    }
                    Some(Line::Row(index)) => {
                        let Some((id, rect, pressed)) =
                            finding_row(ui, table, index, selection, intent, state)
                        else {
                            continue;
                        };
                        press = pressed.or(press);
                        if selection.listed == Some(index) {
                            current_row = Some((id, rect));
                        }
                    }
                    None => {}
                }
            }
        },
    );

    // The table's one keyboard stop, laid over the rows it moves through.
    let response = grid::body(ui, output.inner_rect, current_row);
    match press {
        Some(Press::Followed) => response.surrender_focus(),
        Some(Press::Stayed) if !response.has_focus() => response.request_focus(),
        Some(Press::Stayed) | None => {}
    }
    // After the press, so the frame a click gave the table focus records it:
    // an Escape on the very next frame is still the table's.
    report_escape_owner(&response);
    BodyMemory {
        finding: selection.finding,
        height: table.height,
        offset: output.state.offset.y,
    }
    .store(ui.ctx());
}

/// One finding's row, with the click that selects it and the click that
/// follows it. Returns the row's identity and rectangle, and how a press on
/// it left the keyboard.
fn finding_row(
    ui: &mut Ui,
    table: &Table<'_, '_>,
    index: usize,
    selection: &mut Selection,
    intent: &mut PageIntent,
    state: &AppState,
) -> Option<(egui::Id, egui::Rect, Option<Press>)> {
    let row = table.listed.row(index)?;
    // Asked before the row is drawn: a finding whose object the drawing no
    // longer carries must not look like a jump.
    let route = row
        .route
        .clone()
        .filter(|route| route_refusal(state, route).is_none());
    let id = grid::row_id(row.id);
    let response = grid::row(
        ui,
        table.columns,
        &grid::RowCells {
            mark: row_mark(&row),
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
        id,
    );
    let mut press = None;
    if response.clicked() {
        selection.finding = Some(row.id);
        selection.listed = Some(index);
        press = Some(if route.is_some() {
            Press::Followed
        } else {
            Press::Stayed
        });
        intent.route = route;
    }
    let rect = response.rect;
    response.on_hover_text(hover(&row));
    Some((id, rect, press))
}

/// Where the body has to scroll to for printed line `line` to be wholly in
/// view, or `None` when it already is. Measured in the body's own pixels, so a
/// row the edge cuts in half counts as out of view, and the move is the least
/// that brings it in.
fn reveal_offset(line: usize, offset: f32, height: f32, pitch: f32) -> Option<f32> {
    let top = line as f32 * pitch;
    let bottom = top + CONSOLE_ROW_MIN_HEIGHT;
    if top + 0.5 < offset {
        return Some(top);
    }
    if bottom - 0.5 > offset + height {
        return Some((bottom - height).max(0.0));
    }
    None
}

/// The keys the table takes while it holds focus.
#[derive(Debug, Default)]
struct Keys {
    down: bool,
    up: bool,
    left: bool,
    right: bool,
    home: bool,
    end: bool,
    page_down: bool,
    page_up: bool,
    enter: bool,
    escape: bool,
}

/// Move the current row, close or open its group, follow it, or let it go.
///
/// The keys are taken only while the table holds focus, so the page never
/// swallows a key from the surface a reader is actually in. They are consumed
/// rather than read, so a table that handles Enter does not also hand it to
/// whatever is listening behind the dock.
fn keyboard(
    ui: &Ui,
    listed: &DiagnosticListing<'_>,
    arrangement: &Arrangement,
    state: &AppState,
    body: &BodyMemory,
    view: &mut ProblemsView,
    selection: &mut Selection,
    intent: &mut PageIntent,
) {
    let id = grid::body_id();
    let (focused, let_go) = ui.memory(|memory| {
        (
            memory.has_focus(id),
            memory.had_focus_last_frame(id) && memory.focused().is_none(),
        )
    });
    if !focused {
        // egui lets go of a focused widget on Escape at the start of the
        // frame, before any page runs, unless the widget's lock filter claims
        // it — and a filter is in force only from the second frame a widget
        // holds focus. The Escape that let go on that first frame was still
        // the reader's, and still clears the row.
        if let_go && ui.input_mut(|input| input.consume_key(Modifiers::NONE, Key::Escape)) {
            selection.clear();
        }
        return;
    }
    let keys = ui.input_mut(|input| {
        let mut take = |key| input.consume_key(Modifiers::NONE, key);
        Keys {
            down: take(Key::ArrowDown),
            up: take(Key::ArrowUp),
            left: take(Key::ArrowLeft),
            right: take(Key::ArrowRight),
            home: take(Key::Home),
            end: take(Key::End),
            page_down: take(Key::PageDown),
            page_up: take(Key::PageUp),
            enter: take(Key::Enter),
            escape: take(Key::Escape),
        }
    });
    if keys.down || keys.up || keys.left || keys.right {
        // On the first frame the table holds focus its lock filter is not yet
        // in force, and egui has read the same arrow as a move to whatever
        // widget sits beside, above or below. The table keeps the key, so the
        // move is cancelled.
        ui.memory_mut(|memory| memory.move_focus(egui::FocusDirection::None));
    }
    if (keys.left || keys.right)
        && let Some(group) = selection
            .listed
            .and_then(|index| arrangement.group_of(index))
    {
        // Left closes the current finding's group and Right opens it again,
        // as a tree does. The finding stays current while its group is
        // closed; the next arrow starts again from the top or the bottom.
        view.collapse(&group.label, keys.left);
    }
    if keys.escape {
        // A table with no current row would hold keys nothing acts on.
        ui.memory_mut(|memory| memory.surrender_focus(id));
        selection.clear();
        return;
    }
    let pitch = CONSOLE_ROW_MIN_HEIGHT + ui.spacing().item_spacing.y;
    // A page is the rows the body showed whole, less one, so the row a page
    // key leaves from is still on screen after it.
    let per_page = ((body.height / pitch).floor() as usize)
        .saturating_sub(1)
        .max(1);
    let step = isize::from(keys.down) - isize::from(keys.up);
    let next = if keys.home {
        arrangement.first()
    } else if keys.end {
        arrangement.last()
    } else if keys.page_down {
        arrangement.page(selection.listed, per_page, true)
    } else if keys.page_up {
        arrangement.page(selection.listed, per_page, false)
    } else if step != 0 {
        arrangement.step(selection.listed, step)
    } else {
        None
    };
    if let Some(next) = next {
        selection.listed = Some(next);
        selection.finding = listed.facet(next).map(|facet| facet.id);
    }
    if keys.enter
        && let Some(row) = selection.listed.and_then(|index| listed.row(index))
        && let Some(route) = row.route
        && route_refusal(state, &route).is_none()
    {
        intent.route = Some(route);
        // Enter goes where the finding is, and the table lets go of the
        // keyboard; a canvas the route lands on takes it.
        ui.memory_mut(|memory| memory.surrender_focus(id));
    }
}

/// Whether following `route` lands on a drawing canvas.
///
/// Such a finding is followed to act on what it names — delete it, wire it,
/// rotate it — and the canvas's keys act only while it holds the keyboard, so
/// the canvas is asked to take it. A run's dataset and the documents the other
/// routes open are not canvases.
const fn lands_on_canvas(route: &DiagnosticRoute) -> bool {
    matches!(
        route,
        DiagnosticRoute::Anchor(
            LogAnchor::Schematic { .. } | LogAnchor::Symbol { .. } | LogAnchor::Simulation { .. }
        )
    )
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

/// What the page says under the rows when something qualifies all of them.
///
/// A check that cannot run says why in its producer's words: "some checks
/// cannot run" alone leaves the reader nothing to repair.
///
/// Stale findings come from producers that are refreshed in different ways —
/// a rule check is re-run, a failed run is run again, a dispatch is checked
/// again at the next run, a netlist is validated as it is edited — so the
/// note names no remedy. What moved under a finding, which is what says which
/// of those it needs, is stated in the detail region of the finding itself.
fn note(listed: &DiagnosticListing<'_>) -> Option<String> {
    if let Some(reason) = listed.unavailable() {
        return Some(format!(
            "Some checks cannot run: {reason}. The findings listed are what the rest reported."
        ));
    }
    listed.has_stale().then(|| {
        "Faint findings were reported against inputs that have since changed. \
         Select one to see what moved."
            .to_owned()
    })
}

/// The height `note` takes, wrapped at the width the page has. A reason is
/// the producer's own words and can run to more than one line, and a note
/// reserved one line would push the detail region off the bottom of the dock.
fn note_height(ui: &Ui, note: &str) -> f32 {
    ui.painter()
        .layout(
            note.to_owned(),
            theme::mono(CONSOLE_FONT_SIZE, FontWeight::Regular),
            egui::Color32::WHITE,
            ui.available_width(),
        )
        .size()
        .y
        .max(CONSOLE_ROW_MIN_HEIGHT)
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
/// recorded — a waived finding is a decision, and the page says so.
///
/// A finding whose inputs moved is stale; one whose inputs cannot be read at
/// all is not known to be stale, and says that instead.
fn severity_cell(row: &DiagnosticRow<'_>) -> String {
    let severity = row.level.display_name();
    match (&row.standing, row.waived()) {
        (RowStanding::Current, false) => severity.to_owned(),
        (RowStanding::Current, true) => format!("Waived {severity}"),
        (RowStanding::Stale(_), _) => format!("Stale {severity}"),
        (RowStanding::Unavailable(_), _) => format!("Unavailable {severity}"),
    }
}

/// The mark a row carries in its gutter. A waived finding keeps the shape of
/// its severity without its colour: it is still an error, and it is not
/// counted against the design, and a red cross would say only the first.
fn row_mark(row: &DiagnosticRow<'_>) -> RowMark {
    match (row.current(), row.waived(), level_mark(row.level)) {
        (true, false, Some(problem)) => RowMark::Problem(problem),
        (true, true, Some(problem)) => RowMark::Waived(problem),
        (false, _, _) | (true, _, None) => RowMark::None,
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
    let severity = severity_cell(row);
    [
        severity.as_str(),
        row.code,
        row.subject.as_str(),
        row.producer,
        row.location.as_str(),
        row.message,
    ]
    .map(tsv_field)
    .join("\t")
}

/// One field of tab-separated text, quoted the way spreadsheets write and
/// read it: a field holding a tab, a line break or a quote is wrapped in
/// quotes, with each quote inside it doubled. A message that spans lines is
/// then still one cell when it is pasted, rather than a row break and a
/// column shift.
fn tsv_field(text: &str) -> Cow<'_, str> {
    if text.contains(['\t', '\n', '\r', '"']) {
        Cow::Owned(format!("\"{}\"", text.replace('"', "\"\"")))
    } else {
        Cow::Borrowed(text)
    }
}

/// Every listed row, with the heading the columns carry. Listed means what
/// the filters admit, so a collapsed group's findings are copied too: the
/// count on the button that copies them says so.
fn copy_of_listed(listed: &DiagnosticListing<'_>, arrangement: &Arrangement) -> String {
    use crate::workbench::app_state::problems_view::ProblemColumn;

    let mut text = ProblemColumn::ALL
        .iter()
        .map(|column| column.label())
        .collect::<Vec<_>>()
        .join("\t");
    for index in arrangement.listed_rows() {
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
