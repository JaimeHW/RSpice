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
//! it the arrows, Home, End and the page keys move over the findings and the
//! group headers between them, as in a tree: Left goes from a finding to its
//! header and closes an open one, Right opens a closed header and goes into an
//! open one, and Enter or Space opens or closes the header stood on. Enter
//! follows a finding and Escape lets it go; Tab leaves. Following a finding — by
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
    DiagnosticListing, DiagnosticRow, EmptyReason, RowStanding, listing_in_pass,
};
use crate::workbench::app_state::diagnostic_store::{DiagnosticLevel, DiagnosticRoute};
use crate::workbench::app_state::problems_view::{ProblemsView, WaiverDraft};
use crate::workbench::app_state::session::shortcuts::{
    report_escape_owner, request_engineering_canvas_focus,
};
use crate::workbench::app_state::waivers::{WaiverAction, WaiverTarget};

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
/// page lists, read once for the pass the page reads them in, so the badge can
/// never disagree with the body under it.
pub(in crate::workbench) fn active_problem_count(state: &AppState, pass: u64) -> ProblemCount {
    let counts = listing_in_pass(state, pass).counts();
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
    /// A waiver the reader asked to record, or to take away.
    waiver: Option<WaiverEdit>,
    /// The sign-off report, asked for from the toolbar.
    signoff: bool,
}

/// A change to a waiver stored with a cellview, as the page asks for it.
enum WaiverEdit {
    Record {
        target: WaiverTarget,
        reason: String,
    },
    Remove(WaiverTarget),
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

/// What the body was when it was last drawn: the finding or group header it
/// stood on, the height it was given and where it was scrolled to.
///
/// The body brings the current line into view when it moved under it — by a
/// key, by a click or by the canvas tour — and when height the body lost, to
/// the detail region a new selection opened or to a smaller dock, took the
/// current line out of view. It leaves the reader's own scrolling alone
/// otherwise.
#[derive(Debug, Clone, Copy, Default)]
struct BodyMemory {
    finding: Option<uuid::Uuid>,
    /// The header stood on, by its identity.
    group: Option<egui::Id>,
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

/// Draw the page. Returns whether the reader recorded or took away a waiver:
/// that changes what refuses a run, so the caller drops whatever run was
/// prepared or checked under the waivers as they stood.
pub(super) fn problems(ui: &mut Ui, state: &mut AppState) -> bool {
    let mut view = state.problems_view.clone();
    let mut selection = Selection {
        listed: None,
        finding: state.diagnostics.current_finding(),
    };
    let mut intent = PageIntent::default();
    {
        let listed = listing_in_pass(state, ui.ctx().cumulative_pass_nr());
        let total = listed.len();
        if total == 0 {
            // Nothing to list is not a table with no rows: it is one claim,
            // and a toolbar over it would offer to filter nothing.
            muted(ui, &empty_state(listed.empty_reason()));
            return false;
        }
        let (arrangement, current) = arrange::arrange(ui.ctx(), &listed, &view, selection.finding);
        let columns = Columns::measure(
            ui,
            arrangement.tally.widest_code,
            arrangement.tally.widest_producer,
            listed.unavailable().is_some(),
        );
        intent.signoff = strip::filter_strip(ui, &mut view, &arrangement.tally);
        if view.is_filtered() && strip::filtered_out(ui, arrangement.listed, total) {
            view.clear_filters();
        }
        if let Some(column) = grid::header(ui, &columns, &view) {
            view.order_by(column);
        }

        selection.listed = current;
        // A header is stood on only while no finding is current, so a finding
        // made current anywhere — the canvas tour included — takes the
        // keyboard off it; and a header no longer printed is stood on by
        // nobody.
        if selection.finding.is_some()
            || view
                .current_group
                .as_deref()
                .is_some_and(|label| arrangement.group_named(label).is_none())
        {
            view.current_group = None;
        }
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
            let waiver_refusal = row
                .waiver_action
                .and_then(|action| state.waiver_edit_refusal(&action.target().cell_view));
            // A finding can stop being waivable while its reason is being
            // typed — waived from elsewhere, or gone stale under a re-check —
            // and the reason then has nothing left to be recorded against.
            if view
                .waiving
                .as_ref()
                .is_some_and(|draft| draft.finding == row.id)
                && !matches!(row.waiver_action, Some(WaiverAction::Waive(_)))
            {
                view.waiving = None;
            }
            let waiving = view
                .waiving
                .as_mut()
                .filter(|draft| draft.finding == row.id);
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
                            waiving,
                            waiver_refusal.as_deref(),
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
            match (actions.waiver, row.waiver_action) {
                (Some(detail::WaiverStep::Begin), Some(WaiverAction::Waive(_))) => {
                    view.waiving = Some(WaiverDraft::new(row.id));
                }
                (Some(detail::WaiverStep::Record), Some(WaiverAction::Waive(target))) => {
                    if let Some(draft) = &view.waiving {
                        intent.waiver = Some(WaiverEdit::Record {
                            target: target.clone(),
                            reason: draft.reason.clone(),
                        });
                    }
                }
                (Some(detail::WaiverStep::Cancel), _) => view.waiving = None,
                (
                    Some(detail::WaiverStep::Remove),
                    Some(WaiverAction::Remove(target) | WaiverAction::Delete(target)),
                ) => intent.waiver = Some(WaiverEdit::Remove(target.clone())),
                _ => {}
            }
        }
    }

    // A reason typed for one finding is never recorded against another: the
    // draft goes when the reader stands anywhere else.
    if view
        .waiving
        .as_ref()
        .is_some_and(|draft| selection.finding != Some(draft.finding))
    {
        view.waiving = None;
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
            Ok(text) | Err(text) => state.prompt(text),
        }
    }
    if intent.signoff {
        crate::workbench::workflows::export_workflow::export_signoff_report(
            state,
            &crate::workbench::workflows::export_workflow::NativeExportWorkflowIo,
        );
    }
    intent
        .waiver
        .is_some_and(|edit| apply_waiver_edit(state, edit))
}

/// Record or take away a waiver, as an edit of the cellview that stores it.
/// Returns whether it was made.
///
/// A recorded waiver closes the reason it was written in. A refused one keeps
/// it, so what the reader typed is not lost to a refusal they can repair —
/// a read-only project made writable, a cellview opened.
fn apply_waiver_edit(state: &mut AppState, edit: WaiverEdit) -> bool {
    let outcome = match edit {
        WaiverEdit::Record { target, reason } => state
            .waive_finding(&target, &reason)
            .map(|()| state.problems_view.waiving = None),
        WaiverEdit::Remove(target) => state.remove_finding_waiver(&target),
    };
    outcome.map_err(|refused| state.prompt(refused)).is_ok()
}

/// Everything the table needs to draw itself, so the row loop takes one
/// argument per concern rather than nine positional ones.
struct Table<'a, 'listing> {
    listed: &'a DiagnosticListing<'listing>,
    arrangement: &'a Arrangement,
    columns: &'a Columns,
    height: f32,
}

/// How a pressed line left the keyboard.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Press {
    /// A finding was followed into another surface.
    Followed,
    /// A finding with nowhere to go, or a group header: the reader stays in
    /// the table.
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
    let cursor = cursor_group(table.arrangement, selection, view);
    let header = view.current_group.as_deref().map(grid::group_id);
    let moved = body.finding != selection.finding || body.group != header;
    let shrank = table.height + 0.5 < body.height;
    if (moved || shrank)
        && let Some(line) = cursor_line(table.arrangement, selection, cursor)
        && (moved || shrink_hid(line, &body, table.height, pitch))
        && let Some(offset) = reveal_offset(line, body.offset, table.height, pitch)
    {
        area = area.vertical_scroll_offset(offset);
    }
    let mut press = None;
    // The line the keyboard stood on as the frame was drawn, and a line the
    // pointer pressed during it, which is where the keyboard stands after.
    let mut cursor_row = None;
    let mut pressed_row = None;
    let output = area.show_rows(
        ui,
        CONSOLE_ROW_MIN_HEIGHT,
        table.arrangement.lines(),
        |ui, range| {
            for line in range {
                match table.arrangement.line(line) {
                    Some(Line::Group(group)) => {
                        let Some(info) = table.arrangement.group(group) else {
                            continue;
                        };
                        let response =
                            grid::group_header(ui, table.columns, info, cursor == Some(group));
                        if response.clicked() {
                            // A header click opens or closes it and stands the
                            // keyboard on it, in the table.
                            view.collapse(&info.label, !info.collapsed);
                            selection.clear();
                            view.current_group = Some(info.label.clone());
                            press = Some(Press::Stayed);
                            pressed_row = Some((response.id, response.rect));
                        } else if cursor == Some(group) {
                            cursor_row = Some((response.id, response.rect));
                        }
                    }
                    Some(Line::Row(index)) => {
                        let Some((id, rect, pressed)) =
                            finding_row(ui, table, index, view, selection, intent, state)
                        else {
                            continue;
                        };
                        if pressed.is_some() {
                            press = pressed;
                            pressed_row = Some((id, rect));
                            view.current_group = None;
                        } else if cursor.is_none() && selection.listed == Some(index) {
                            cursor_row = Some((id, rect));
                        }
                    }
                    None => {}
                }
            }
        },
    );

    // The table's one keyboard stop, laid over the rows it moves through.
    let response = grid::body(ui, output.inner_rect, pressed_row.or(cursor_row));
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
        group: view.current_group.as_deref().map(grid::group_id),
        height: table.height,
        offset: output.state.offset.y,
    }
    .store(ui.ctx());
}

/// The group header the keyboard stands on, by group index: the header stood
/// on, or the header of a closed group the current finding is in.
fn cursor_group(
    arrangement: &Arrangement,
    selection: &Selection,
    view: &ProblemsView,
) -> Option<usize> {
    match (selection.finding, selection.listed) {
        (None, _) => arrangement.group_named(view.current_group.as_deref()?),
        // Only a closed group can hide the finding, so the walk that finds a
        // finding's group is made only while one is closed.
        (Some(_), Some(index)) if arrangement.hides_collapsed_rows() => {
            arrangement.group_of(index).filter(|group| {
                arrangement
                    .group(*group)
                    .is_some_and(|group| group.collapsed)
            })
        }
        (Some(_), _) => None,
    }
}

/// The printed line the keyboard stands on: `group`'s header, where it stands
/// on one, or else the current finding's row.
fn cursor_line(
    arrangement: &Arrangement,
    selection: &Selection,
    group: Option<usize>,
) -> Option<usize> {
    match group {
        Some(group) => arrangement.header_line(group),
        None => selection
            .listed
            .and_then(|index| arrangement.line_of(index)),
    }
}

/// Stand the keyboard on printed line `line`. A finding's line makes it the
/// current finding; a header's makes it the header stood on, with no finding
/// current.
fn stand_on(
    line: usize,
    listed: &DiagnosticListing<'_>,
    arrangement: &Arrangement,
    view: &mut ProblemsView,
    selection: &mut Selection,
) {
    match arrangement.line(line) {
        Some(Line::Row(index)) => {
            selection.listed = Some(index);
            selection.finding = listed.facet(index).map(|facet| facet.id);
            view.current_group = None;
        }
        Some(Line::Group(group)) => {
            selection.clear();
            view.current_group = arrangement.group(group).map(|group| group.label.clone());
        }
        None => {}
    }
}

/// Whether the height the body lost since it was last drawn took line `line`
/// out of view: the line showed then, if only in part, and the body's new
/// bottom edge now cuts it.
///
/// A line the reader had scrolled away from is not brought back. Bringing it
/// back is what took the reader's place from them whenever the dock shrank.
fn shrink_hid(line: usize, body: &BodyMemory, height: f32, pitch: f32) -> bool {
    let top = line as f32 * pitch;
    let bottom = top + CONSOLE_ROW_MIN_HEIGHT;
    let showed = bottom > body.offset + 0.5 && top < body.offset + body.height - 0.5;
    showed && bottom - 0.5 > body.offset + height
}

/// One finding's row, with the click that selects it, the click that follows
/// it and the menu that waives it. Returns the row's identity and rectangle,
/// and how a press on it left the keyboard.
fn finding_row(
    ui: &mut Ui,
    table: &Table<'_, '_>,
    index: usize,
    view: &mut ProblemsView,
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
    if let Some(action) = row.waiver_action {
        // Disabled, saying why, where the document that stores the waiver
        // cannot be edited here: the edit would refuse for that reason.
        let refusal = state.waiver_edit_refusal(&action.target().cell_view);
        response.context_menu(|ui| {
            let verb = ui.add_enabled(
                refusal.is_none(),
                egui::Button::new(detail::waiver_verb(action)),
            );
            let verb = match &refusal {
                Some(refusal) => verb.on_disabled_hover_text(refusal),
                None => verb,
            };
            if verb.clicked() {
                match action {
                    // The reason is asked for in the detail region, which
                    // the row has to be current to open.
                    WaiverAction::Waive(_) => {
                        selection.finding = Some(row.id);
                        selection.listed = Some(index);
                        view.current_group = None;
                        view.waiving = Some(WaiverDraft::new(row.id));
                    }
                    WaiverAction::Remove(target) | WaiverAction::Delete(target) => {
                        intent.waiver = Some(WaiverEdit::Remove(target.clone()));
                    }
                }
                ui.close();
            }
        });
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

/// Move over the findings and group headers, open or close the header stood
/// on, follow a finding, or let go.
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
        // the reader's, and still lets go of the line stood on.
        if let_go && ui.input_mut(|input| input.consume_key(Modifiers::NONE, Key::Escape)) {
            selection.clear();
            view.current_group = None;
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
    if keys.escape {
        // A table with nothing stood on would hold keys nothing acts on.
        ui.memory_mut(|memory| memory.surrender_focus(id));
        selection.clear();
        view.current_group = None;
        return;
    }
    let group = cursor_group(arrangement, selection, view);
    // Space opens and closes the header stood on. On a finding it is not the
    // table's key, and is left alone.
    let space =
        group.is_some() && ui.input_mut(|input| input.consume_key(Modifiers::NONE, Key::Space));
    let step = isize::from(keys.down) - isize::from(keys.up);
    let moves = keys.home || keys.end || keys.page_down || keys.page_up || step != 0;
    if !(moves || keys.left || keys.right || keys.enter || space) {
        return;
    }
    let at = cursor_line(arrangement, selection, group);
    let header = group.and_then(|group| arrangement.group(group));
    let pitch = CONSOLE_ROW_MIN_HEIGHT + ui.spacing().item_spacing.y;
    // A page is the lines the body showed whole, less one, so the line a page
    // key leaves from is still on screen after it.
    let per_page = ((body.height / pitch).floor() as usize)
        .saturating_sub(1)
        .max(1);
    let next = if keys.home {
        arrangement.step_line(None, 1)
    } else if keys.end {
        arrangement.step_line(None, -1)
    } else if keys.page_down {
        arrangement.page_line(at, per_page, true)
    } else if keys.page_up {
        arrangement.page_line(at, per_page, false)
    } else if step != 0 {
        arrangement.step_line(at, step)
    } else if keys.left {
        // As in a tree: Left closes an open header, and goes from a finding
        // to its group's header.
        match header {
            Some(header) => {
                view.collapse(&header.label, true);
                None
            }
            None => selection
                .listed
                .and_then(|index| arrangement.group_of(index))
                .and_then(|group| arrangement.header_line(group)),
        }
    } else if keys.right {
        // Right opens a closed header, and goes into an open one.
        match (header, at) {
            (Some(header), _) if header.collapsed => {
                view.collapse(&header.label, false);
                None
            }
            (Some(_), Some(line)) => {
                Some(line + 1).filter(|line| matches!(arrangement.line(*line), Some(Line::Row(_))))
            }
            _ => None,
        }
    } else {
        None
    };
    if let Some(next) = next {
        stand_on(next, listed, arrangement, view, selection);
    }
    if !(keys.enter || space) {
        return;
    }
    // Enter and Space act where the keyboard stands once it has moved.
    let group = if next.is_some() {
        cursor_group(arrangement, selection, view)
    } else {
        group
    };
    if let Some(header) = group.and_then(|group| arrangement.group(group)) {
        view.collapse(&header.label, !header.collapsed);
    } else if keys.enter
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
        | DiagnosticRoute::Repair(_)
        | DiagnosticRoute::RunLogLine { .. } => None,
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
        EmptyReason::OutOfDate => {
            "No findings when last checked. The design has changed since.".to_owned()
        }
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
///
/// The waiver covering the finding follows, as the last column: a waived
/// finding pasted into a review without who accepted it and why is a finding
/// the review cannot audit.
fn copy_of(row: &DiagnosticRow<'_>) -> String {
    let severity = severity_cell(row);
    let waiver = row.suppression.map(detail::waiver_text).unwrap_or_default();
    [
        severity.as_str(),
        row.code,
        row.subject.as_str(),
        row.producer,
        row.location.as_str(),
        row.message,
        waiver.as_str(),
    ]
    .map(tsv_field)
    .join("\t")
}

/// The heading of the copied waiver column.
const WAIVER_COLUMN: &str = "Waiver";

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
        .chain([WAIVER_COLUMN])
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
