//! The Project Overview: the project's summary, as a band of facts over
//! one table.
//!
//! The band states the plan, what the design is made of, what it depends
//! on, how its checks went, and the latest run: how it went, what it
//! analysed and on what engine. A fact whose value leads somewhere links
//! there. The table under it is the latest run's outputs, each judged as
//! Verify judges it, and each row opens Verify on its specification. What
//! the inspector beside the page holds, and edits — the top cell, the
//! configuration runs use, the technology — the page does not say again.
//!
//! This surface is a projection of authoritative project, lifecycle, design,
//! validation, simulation, and model state. It deliberately does not infer
//! autosave success, source-control history, or dependency authentication from
//! unrelated runtime flags.

mod census;
mod page;
mod snapshot;

use egui::Ui;

use crate::state::VerificationPage;
use crate::state::command_catalog::CommandKey;
use crate::ui::widgets::Problem;
use crate::ui::widgets::table::TableState;
use crate::workbench::framework::command_registry::{CommandAvailability, ids};
use crate::workbench::framework::feature::Cx;
use crate::workbench::framework::requests::{OpenInVerify, ShowProjectDeck};
use crate::workbench::project::commands::{ProjectCommand, ProjectRequest};
use crate::workbench::project::workspace_state::ProjectPage;
use crate::workbench::project::{DependencyLibrary, ProjectFeature};
use crate::workbench::state::Workspace;

pub use census::DesignCensus;
pub use snapshot::{
    Action, DeckSummary, MeasurementLine, Outputs, OverviewSnapshot, SpecificationLine,
    SummaryFact, deck_summary,
};

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

/// What the reader asked of the page: a linked value, a row's action, or
/// an empty table's button.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OverviewIntent {
    Command(CommandKey),
    /// Check the cell view runs start from, which the Checks fact
    /// describes: the configuration's root when one is used for runs.
    CheckSimulationRoot,
    /// The project's deck, in front of the Netlist workspace, whatever was
    /// in front there before.
    OpenProjectDeck,
    /// The newest run, in Results.
    OpenResults,
    /// The Dependencies page, standing on the dependency named, or on none.
    OpenDependency(Option<DependencyLibrary>),
    /// Verify's Specifications, on the specification of the measurement
    /// named.
    OpenVerify(Option<String>),
}

impl OverviewIntent {
    /// Ask for what the reader asked of the page: a command runs, and what
    /// only the shell carries out is raised for it, as the page returns.
    pub fn raise(self, cx: &mut Cx<'_, ProjectFeature>) {
        let results = ids::open_workspace(Workspace::Results);
        match self {
            Self::Command(key) => cx.execute(key),
            Self::CheckSimulationRoot => {
                let root = cx.model.workspace.content.simulation_root_reference();
                cx.wb.outbox.push(ProjectRequest::OpenRootAndCheck(root));
            }
            Self::OpenProjectDeck => cx.wb.outbox.push(ShowProjectDeck),
            Self::OpenResults => {
                if cx.model.simulation.select_run(0) {
                    cx.execute(results);
                }
            }
            Self::OpenDependency(library) => {
                // The row is listed whatever the page was filtered to.
                if library.is_some() {
                    cx.state.dependencies.filter.clear();
                    cx.state.dependencies.kind = None;
                }
                cx.state.project_dependency_selection = library;
                cx.execute(ProjectCommand::Page(ProjectPage::Dependencies).key());
            }
            Self::OpenVerify(specification) => cx.wb.outbox.push(OpenInVerify {
                page: VerificationPage::Specifications,
                specification,
                check: None,
            }),
        }
    }
}

/// What the Overview keeps between frames: its outputs table's selection
/// and the columns the reader sized. Presentation only, and the open
/// project's.
#[derive(Debug, Clone, Default)]
pub struct OverviewTables {
    pub outputs: TableState,
}

fn command_gate(cx: &Cx<'_, ProjectFeature>, key: CommandKey) -> (bool, Option<&'static str>) {
    match cx.foreign_availability(key) {
        CommandAvailability::Available => (true, None),
        CommandAvailability::Disabled(reason) => (false, Some(reason)),
        CommandAvailability::Hidden => (false, Some("this action is unavailable in this context")),
    }
}

/// The Overview page: the project's summary, over the dispatcher's answers
/// the context holds for the actions it offers. What the reader asked of it
/// is raised as the page returns, and the shell carries it out straight
/// after.
pub fn overview(ui: &mut Ui, cx: &mut Cx<'_, ProjectFeature>) {
    // The Dependencies fact says what the Dependencies page says, so it
    // reads the model files as they are on disk now, once per change, as
    // that page does.
    crate::workbench::project::dependency_status::refresh_source_drift(ui.ctx(), cx.model);
    let snapshot = OverviewSnapshot::capture(ui.ctx(), cx);
    // The table's memory is the feature's, lent to the page while the rest
    // of the context is read.
    let mut tables = std::mem::take(&mut cx.state.overview_tables);
    let intent = page::page(ui, &snapshot, &mut tables.outputs);
    cx.state.overview_tables = tables;
    if let Some(intent) = intent {
        intent.raise(cx);
    }
}

#[cfg(test)]
mod tests;
