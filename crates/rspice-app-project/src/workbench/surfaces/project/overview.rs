//! Truthful project overview.
//!
//! This surface is a projection of authoritative project, lifecycle, design,
//! validation, simulation, and model state. It deliberately does not infer
//! autosave success, source-control history, or dependency authentication from
//! unrelated runtime flags.

mod activity;
mod design;
mod header;
mod snapshot;
mod status;

use egui::{Ui, pos2, vec2};

use crate::state::command_catalog::CommandKey;
use crate::state::{CellViewRef, ViewType};
use crate::ui::widgets::{Problem, paint_problem_mark};
use crate::workbench::app_state::Model;
use crate::workbench::framework::command_registry::{CommandAvailability, ids};
use crate::workbench::framework::feature::Cx;
use crate::workbench::project::ProjectFeature;
use crate::workbench::project::commands::ProjectRequest;
use crate::workbench::project::health::{lifecycle_label, lifecycle_tone};
use crate::workbench::state::Workspace;

use super::page::{self, BODY_TOP, CARD_GAP, STACK_BREAKPOINT};

use activity::operations_card;
use design::design_card;
pub use design::{DesignObjectSnapshot, design_objects};
use header::overview_header;
pub use header::run_plan_intent;
pub use snapshot::{
    ConfigurationSnapshot, OperationSnapshot, OverviewSnapshot, ProblemSnapshot, StatusSnapshot,
    problem_snapshot,
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
}

impl OverviewIntent {
    /// Ask for what the reader asked of the page: a command runs, and what
    /// only the shell carries out is raised for it, as the page returns.
    pub fn raise(self, cx: &mut Cx<'_, ProjectFeature>) {
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
        }
    }
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
/// centred content column on the application ground — and projects the
/// dashboard as bordered cards that size to their content, so a sparse
/// project shows page ground below the cards rather than stretched panels.
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
    let shown = &*cx;
    page::show(
        ui,
        "workbench.project.overview.page",
        |ui, inset, content_width| {
            retain_first_intent(
                &mut intent,
                overview_header(ui, shown, &snapshot, inset, content_width),
            );
            overview_body(ui, shown, &snapshot, &mut intent, inset, content_width);
        },
    );
    if let Some(intent) = intent {
        intent.raise(cx);
    }
}

fn retain_first_intent(slot: &mut Option<OverviewIntent>, candidate: Option<OverviewIntent>) {
    if slot.is_none() {
        *slot = candidate;
    }
}

fn overview_body(
    ui: &mut Ui,
    cx: &Cx<'_, ProjectFeature>,
    snapshot: &OverviewSnapshot,
    intent: &mut Option<OverviewIntent>,
    inset: f32,
    content_width: f32,
) {
    ui.add_space(BODY_TOP);
    if content_width >= STACK_BREAKPOINT {
        let (mut left_intent, mut right_intent) = (None, None);
        page::columns(
            ui,
            inset,
            content_width,
            |left| {
                retain_first_intent(
                    &mut left_intent,
                    status_card(left, &snapshot.statuses, snapshot.problem.as_ref()),
                );
                left.add_space(CARD_GAP);
                retain_first_intent(&mut left_intent, design_card(left, cx, snapshot));
            },
            |right| {
                retain_first_intent(&mut right_intent, operations_card(right, snapshot));
            },
        );
        retain_first_intent(intent, left_intent);
        retain_first_intent(intent, right_intent);
    } else {
        page::inset_column(ui, inset, content_width, |ui| {
            retain_first_intent(
                intent,
                status_card(ui, &snapshot.statuses, snapshot.problem.as_ref()),
            );
            ui.add_space(CARD_GAP);
            retain_first_intent(intent, design_card(ui, cx, snapshot));
            ui.add_space(CARD_GAP);
            retain_first_intent(intent, operations_card(ui, snapshot));
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
