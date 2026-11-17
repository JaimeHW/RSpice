//! Truthful project overview.
//!
//! This surface is a projection of authoritative project, lifecycle, design,
//! validation, simulation, and model state. It deliberately does not infer
//! autosave success, source-control history, or dependency authentication from
//! unrelated runtime flags.

mod activity;
mod cells;
mod header;
mod snapshot;
mod status;

use egui::{Ui, pos2, vec2};

use crate::state::command_catalog::CommandKey;
use crate::state::{CellViewRef, ViewType};
use crate::ui::widgets::table::TableState;
use crate::ui::widgets::{Problem, paint_problem_mark};
use crate::workbench::app_state::Model;
use crate::workbench::framework::command_registry::{CommandAvailability, ids};
use crate::workbench::framework::feature::Cx;
use crate::workbench::project::commands::{ProjectCommand, ProjectRequest};
use crate::workbench::project::health::{lifecycle_label, lifecycle_tone};
use crate::workbench::project::workspace_state::ProjectPage;
use crate::workbench::project::{ProjectFeature, open_project_revision_history, schematic_row_key};
use crate::workbench::state::Workspace;

use super::library::{LibraryIntent, apply_library_intent};
use super::page::{self, BODY_TOP, CARD_GAP, STACK_BREAKPOINT};

use activity::activity_card;
use cells::cells_card;
use header::overview_header;
pub use header::run_plan_intent;
pub use snapshot::{
    CellRowSnapshot, OperationSnapshot, OverviewSnapshot, ProblemSnapshot, StatusSnapshot,
    blocking_findings,
};
use status::status_card;

/// How a fact on this page stands.
///
/// Three states, not five: a tone is a severity, and the only severities the
/// page can act on are "look at this" and "this stops a run". `Info` and `Ok`
/// were a blue and a green over text that was already saying what it said, so
/// they collapsed into [`Self::Neutral`] along with the colours.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tone {
    Neutral,
    Warn,
    Error,
}

impl Tone {
    /// The mark that goes in front of this fact, when the reader has to see
    /// one. The mark owns the colour; the words beside it never do.
    pub const fn problem(self) -> Option<Problem> {
        match self {
            Self::Neutral => None,
            Self::Warn => Some(Problem::Warning),
            Self::Error => Some(Problem::Failure),
        }
    }
}

/// Side of a mark painted into one of this page's rows, and the advance from
/// its left edge to where the text after it begins. Read from the kit: a
/// marked line and an unmarked one beside it differ by exactly the advance.
use crate::ui::widgets::{PROBLEM_MARK_ADVANCE as MARK_ADVANCE, PROBLEM_MARK_SIDE as MARK_SIDE};

/// Paint `tone`'s mark with its left edge at `left`, centred on `center_y`,
/// and return how far the text after it is pushed. A neutral tone paints
/// nothing and pushes nothing, so a page with no problems on it has no
/// gutter.
fn paint_tone_mark(ui: &Ui, tone: Tone, left: f32, center_y: f32) -> f32 {
    let Some(problem) = tone.problem() else {
        return 0.0;
    };
    paint_problem_mark(
        ui.painter(),
        egui::Rect::from_center_size(
            pos2(left + MARK_SIDE * 0.5, center_y),
            vec2(MARK_SIDE, MARK_SIDE),
        ),
        problem,
    );
    MARK_ADVANCE
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OverviewIntent {
    Command(CommandKey),
    /// Check the cell view runs start from, which the Schematic checks row
    /// describes: the configuration's root when one is used for runs.
    CheckSimulationRoot,
    OpenProjectRoot,
    OpenProjectModels,
    OpenRun(usize),
    /// Open a view of a cell: the one the Cells table's row names.
    OpenView(CellViewRef, ViewType),
    /// The Library page, on a library and one of its cells: a cell with no
    /// view to open.
    ShowInLibrary {
        library: String,
        cell: Option<String>,
    },
    /// The Problems page, listing one finding and standing on it: the strip
    /// under the status rows links each finding there. `None` for a deck's
    /// diagnostic, which the list finds by its words alone.
    ShowFinding {
        message: String,
        finding: Option<uuid::Uuid>,
    },
    /// Revision history, on a validated revision by its row's key.
    OpenRevision(String),
    /// The Recovery page, on a checkpoint by its identity.
    OpenCheckpoint(String),
}

impl OverviewIntent {
    /// Ask for what the reader asked of the page: a command runs, and what
    /// only the shell carries out is raised for it, as the page returns.
    pub fn raise(self, ctx: &egui::Context, cx: &mut Cx<'_, ProjectFeature>) {
        match self {
            Self::Command(key) => cx.execute(key),
            Self::CheckSimulationRoot => {
                let root = cx.model.workspace.content.simulation_root_reference();
                cx.wb.outbox.push(ProjectRequest::OpenRootAndCheck(root));
            }
            Self::OpenProjectRoot => {
                let root = project_root(cx.model);
                cx.wb
                    .outbox
                    .push(ProjectRequest::OpenView(root, ViewType::Schematic));
            }
            Self::OpenProjectModels => {
                cx.wb
                    .outbox
                    .push(crate::workbench::framework::requests::ShowProjectModels);
            }
            Self::OpenRun(index) => {
                let has_retained_dataset = cx
                    .model
                    .simulation
                    .retained
                    .runs
                    .get(index)
                    .is_some_and(|run| !run.analyses.is_empty());
                let destination = run_destination(
                    has_retained_dataset,
                    cx.model.is_netlist_first_without_schematic(),
                );
                if cx.model.simulation.select_run(index) {
                    cx.execute(ids::open_workspace(destination));
                }
            }
            Self::OpenView(reference, view_type) => {
                cx.wb
                    .outbox
                    .push(ProjectRequest::OpenView(reference, view_type));
            }
            Self::ShowInLibrary { library, cell } => {
                cx.execute(ProjectCommand::Page(ProjectPage::Library).key());
                let select = match cell {
                    Some(cell) => LibraryIntent::SelectCell { library, cell },
                    None => LibraryIntent::SelectLibrary(library),
                };
                apply_library_intent(ctx, cx, select);
            }
            Self::ShowFinding { message, finding } => {
                // Every filter cleared but the finding's own words, so the
                // list holds it whatever was being read there before.
                cx.wb.show_problem_findings();
                cx.wb.problems_view.text = message;
                if finding.is_some() {
                    cx.model.diagnostics.set_current_finding(finding);
                }
            }
            Self::OpenRevision(key) => {
                open_project_revision_history(
                    &mut cx.state.project_revision_history,
                    cx.model,
                    cx.platform,
                );
                // The history lists the open schematic's revisions; one of
                // another cell's opens the history on its newest.
                let listed = cx
                    .model
                    .schematic
                    .document()
                    .validated_revisions
                    .records()
                    .iter()
                    .any(|record| schematic_row_key(record.id()) == key);
                if listed && cx.state.project_revision_history.open {
                    cx.state.project_revision_history.selected = vec![key];
                }
            }
            Self::OpenCheckpoint(checkpoint) => {
                cx.execute(ProjectCommand::Page(ProjectPage::Recovery).key());
                cx.state.project_checkpoint_selection = Some(checkpoint);
            }
        }
    }
}

/// What the Overview's tables keep between frames: the order the reader put
/// them in, the columns they sized, the row they stand on and how far each
/// is scrolled. Presentation only, and the open project's.
#[derive(Debug, Clone, Default)]
pub struct OverviewTables {
    pub cells: TableState,
    pub activity: TableState,
    pub findings: TableState,
}

/// Where the "Latest run" affordances lead.
///
/// A retained dataset always opens in Results. Without one the destination is
/// wherever the project is actually runnable, and for a netlist-first project
/// that is the SPICE source workspace, not Simulate: the run preflight reads the
/// schematic, which such a project deliberately leaves pristine, so Simulate
/// would offer a Run button it can never enable.
pub(super) const fn run_destination(has_retained_dataset: bool, netlist_first: bool) -> Workspace {
    match (has_retained_dataset, netlist_first) {
        (true, _) => Workspace::Results,
        (false, true) => Workspace::Netlist,
        (false, false) => Workspace::Simulate,
    }
}

fn command_gate(cx: &Cx<'_, ProjectFeature>, key: CommandKey) -> (bool, Option<&'static str>) {
    match cx.foreign_availability(key) {
        CommandAvailability::Available => (true, None),
        CommandAvailability::Disabled(reason) => (false, Some(reason)),
        CommandAvailability::Hidden => (false, Some("this action is unavailable in this context")),
    }
}

/// Render the project overview from one snapshot a frame, whose derived
/// parts the frame context keeps until what they were derived from changes.
///
/// The page shares the landing surfaces' geometry — a header band and a
/// centred content column on the application ground — with the status card
/// at its content's height and the Cells and Activity tables taking the
/// height the window has, so the two columns end together.
/// The Overview page, over the dispatcher's answers the context holds for
/// the commands its rows offer. What the reader asked of it is raised as the
/// page returns, and the shell carries it out straight after.
pub fn overview(ui: &mut Ui, cx: &mut Cx<'_, ProjectFeature>) {
    // The Unsaved work row counts what the Recovery page can bring back, from
    // the catalogue that page lists: listed once, off the frame.
    super::recovery::ensure_project_recovery_catalog(ui.ctx(), cx);
    // The Dependencies row says what the Dependencies page says, so it reads
    // the model files as they are on disk now, once per change, as that page
    // does.
    crate::workbench::project::dependency_status::refresh_source_drift(ui.ctx(), cx.model);
    let snapshot = OverviewSnapshot::capture(ui.ctx(), cx);
    let mut intent = None;
    // The tables' memory is the feature's, lent to the page while the rest
    // of the context is read.
    let mut tables = std::mem::take(&mut cx.state.overview_tables);
    let shown = &*cx;
    page::show_measured(
        ui,
        "workbench.project.overview.page",
        |ui, inset, content_width, viewport_height| {
            let top = ui.cursor().top();
            retain_first_intent(
                &mut intent,
                overview_header(ui, shown, &snapshot, inset, content_width),
            );
            let body = Body {
                cx: shown,
                snapshot: &snapshot,
                inset,
                width: content_width,
                // What the window has under the header, down to the page's
                // bottom margin.
                height: viewport_height - (ui.cursor().top() - top) - BODY_TOP - page::BODY_BOTTOM,
            };
            overview_body(ui, &body, &mut tables, &mut intent);
        },
    );
    cx.state.overview_tables = tables;
    if let Some(intent) = intent {
        intent.raise(ui.ctx(), cx);
    }
}

fn retain_first_intent(slot: &mut Option<OverviewIntent>, candidate: Option<OverviewIntent>) {
    if slot.is_none() {
        *slot = candidate;
    }
}

/// The least a list card is made, so a short window still shows a few of
/// its rows, and the page scrolls to the rest of it.
const LIST_CARD_MIN_HEIGHT: f32 = 180.0;

/// What the Overview's body is drawn from, and the room it has: the content
/// column's inset and width, and the height under the header.
struct Body<'a, 'c> {
    cx: &'a Cx<'c, ProjectFeature>,
    snapshot: &'a OverviewSnapshot,
    inset: f32,
    width: f32,
    height: f32,
}

fn overview_body(
    ui: &mut Ui,
    body: &Body<'_, '_>,
    tables: &mut OverviewTables,
    intent: &mut Option<OverviewIntent>,
) {
    let Body {
        cx,
        snapshot,
        inset,
        width,
        height,
    } = *body;
    let OverviewTables {
        cells,
        activity,
        findings,
    } = tables;
    ui.add_space(BODY_TOP);
    if width >= STACK_BREAKPOINT {
        let (mut left_intent, mut right_intent) = (None, None);
        // Where the columns end: the window's bottom, or under the status
        // card and the least Cells card when that is lower. The Activity
        // card is drawn after the left column and ends where it does.
        let left_bottom = std::cell::Cell::new(0.0_f32);
        let top = ui.cursor().top();
        page::columns(
            ui,
            inset,
            width,
            |left| {
                retain_first_intent(
                    &mut left_intent,
                    status_card(left, &snapshot.statuses, &snapshot.problems, findings),
                );
                left.add_space(CARD_GAP);
                let room = (top + height - left.cursor().top()).max(LIST_CARD_MIN_HEIGHT);
                retain_first_intent(
                    &mut left_intent,
                    cells_card(left, cx, snapshot, cells, (room, true)),
                );
                left_bottom.set(left.min_rect().bottom());
            },
            |right| {
                let room = (left_bottom.get() - right.cursor().top()).max(LIST_CARD_MIN_HEIGHT);
                retain_first_intent(
                    &mut right_intent,
                    activity_card(right, cx, snapshot, activity, (room, true)),
                );
            },
        );
        retain_first_intent(intent, left_intent);
        retain_first_intent(intent, right_intent);
    } else {
        // One column: each card ends with its list, and a list longer than
        // the window scrolls inside its card.
        let room = height.max(LIST_CARD_MIN_HEIGHT);
        page::inset_column(ui, inset, width, |ui| {
            retain_first_intent(
                intent,
                status_card(ui, &snapshot.statuses, &snapshot.problems, findings),
            );
            ui.add_space(CARD_GAP);
            retain_first_intent(intent, cells_card(ui, cx, snapshot, cells, (room, false)));
            ui.add_space(CARD_GAP);
            retain_first_intent(
                intent,
                activity_card(ui, cx, snapshot, activity, (room, false)),
            );
        });
    }
}

/// A bordered dashboard card: title bar, one fact about its contents,
/// content-sized body. The rows inside carry the marks on this page, so no
/// card head takes one.
fn overview_card(
    ui: &mut Ui,
    title: &str,
    meta: &str,
    body: impl FnOnce(&mut Ui) -> Option<OverviewIntent>,
) -> Option<OverviewIntent> {
    page::card(ui, title, meta, None, body)
}

/// The project's root schematic.
fn project_root(model: &Model) -> CellViewRef {
    CellViewRef::new(
        &model.workspace.content.project.root_library,
        &model.workspace.content.project.top_cell,
        crate::state::workspace::DEFAULT_SCHEMATIC_VIEW,
    )
}

const fn singular_or_plural<'a>(count: usize, singular: &'a str, plural: &'a str) -> &'a str {
    if count == 1 { singular } else { plural }
}

#[cfg(test)]
mod tests;
