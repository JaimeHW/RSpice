//! The simulate feature: the Simulate and Verify workspaces, the plans and
//! analyses a run is made from, and the runs themselves.

pub(crate) mod commands;
mod docks;
mod run_host;

use crate::state::SimulationPage;
use crate::state::command_catalog::CommandKey;
use crate::workbench::app_state::Model;
use crate::workbench::framework::command_registry::{CommandAvailability, CommandSpec, ids};
use crate::workbench::framework::feature::{
    CommandSet, Cx, Feature, FeatureInspector, FeatureNavigator, FeatureRegistry, NoHandles, Reads,
};
use crate::workbench::slots::SimulateState;
use crate::workbench::state::{
    SimulationPlanManagerDraft, SimulationWorkflowDialog, WorkbenchSession, Workspace,
};
use crate::workbench::{RouteTransitionSource, SurfaceId, SurfaceRoute};
use commands::SimulateCommand;
pub(in crate::workbench) use run_host::run_host;

/// The simulate feature.
pub(crate) struct SimulateFeature;

impl Feature for SimulateFeature {
    type State = SimulateState;
    type Session = WorkbenchSession;
    type Handles = NoHandles;
    /// The run history's primary runs or stops what the dispatcher would:
    /// on the Netlist workspace the shell runs that page's deck in place
    /// of the active plan. The Specifications page states why Results will
    /// not open a limit's evidence, and the Analyses route's heading why the
    /// split stage will not open, in the dispatcher's own words.
    const FOREIGN_AVAILABILITY: &'static [CommandKey] = &[
        SimulateCommand::Run.key(),
        SimulateCommand::Stop.key(),
        ids::RESULT_SPECIFICATIONS,
        ids::TOGGLE_SPLIT_VIEW,
    ];

    fn register(registry: &mut FeatureRegistry<Self>) {
        registry.commands::<SimulateCommand>();
        registry.navigator(FeatureNavigator {
            show: docks::show_navigator,
        });
        registry.inspector(FeatureInspector {
            show: docks::show_inspector,
            // Nothing the section draws holds an edit for the shell to commit.
            commit_pending_edit: |_, _| {},
        });
    }
}

impl SimulateFeature {
    /// Draw the review of a staged plan removal that orphans something,
    /// while one is staged.
    pub(in crate::workbench) fn render_plan_removal_review(
        cx: &mut Cx<'_, Self>,
        ctx: &egui::Context,
    ) {
        cx.state.plan_removal_review.process(ctx);
    }

    /// Draw the statement of a plan removal the plan refused, while one
    /// stands.
    pub(in crate::workbench) fn render_plan_removal_refusal(
        cx: &mut Cx<'_, Self>,
        ctx: &egui::Context,
    ) {
        cx.state.plan_removal_refusal.process(ctx);
    }
}

impl CommandSet for SimulateCommand {
    type Feature = SimulateFeature;
    const REGISTERED: &'static [Self] = &Self::ALL;
    const UNLISTED: &'static [Self] = &Self::UNLISTED;

    fn spec(self) -> CommandSpec {
        self.declared_spec()
    }

    /// An unrunnable plan leaves Run enabled on purpose, so the run says why
    /// it cannot start; Run is grey only while one runs. Stop is enabled
    /// while a check or a run can take it, and names why not otherwise.
    /// Preflight waits for the decks of the last check. The plan manager and
    /// the analysis catalogue edit the plan catalog through the active plan's
    /// stable identity, so a project whose plan never migrated to it has
    /// nothing to select, nothing to add to, and no dialog worth opening.
    /// The run history, the solver settings and the pages always run.
    fn availability(self, reads: &Reads<'_, SimulateFeature>) -> CommandAvailability {
        let simulation = &reads.model.simulation;
        let (enabled, reason) = match self {
            Self::Run => (
                !simulation.execution.has_active_execution()
                    && !simulation.execution.trigger_simulation,
                "a run is already in progress",
            ),
            Self::Stop => (
                reads.runs.preflight.checking.is_some()
                    || simulation.can_request_abort_active_run(),
                if simulation.cancellation_is_pending() {
                    "simulation cancellation is already in progress"
                } else if simulation.execution.has_active_execution() {
                    "the active simulation execution cannot accept cancellation"
                } else {
                    "no simulation is running"
                },
            ),
            Self::Preflight => (
                reads.runs.preflight.checking.is_none(),
                "the engine is still building the decks of the last check",
            ),
            Self::ManagePlans => (
                reads.model.sim_setup.stable_analysis_plan().is_ok(),
                "the active simulation plan has no stable analysis identity to manage",
            ),
            Self::AddAnalysis => (
                reads.model.sim_setup.stable_analysis_plan().is_ok(),
                "the active simulation plan has no stable analysis identity to add to",
            ),
            Self::JobsManager | Self::Options | Self::Page(_) | Self::Verify(_) => (true, ""),
        };
        if enabled {
            CommandAvailability::Available
        } else {
            CommandAvailability::Disabled(reason)
        }
    }

    fn execute(self, cx: &mut Cx<'_, SimulateFeature>) {
        match self {
            Self::Run => cx.runs.preflight.request_run_and_queue(),
            // Stop is disabled, and names why, while the run cannot take it.
            Self::Stop => {
                if cx.runs.preflight.checking.is_some() {
                    // The frame loop stops it before it looks for a verdict.
                    cx.runs.preflight.request_check_cancel();
                } else if cx.model.simulation.can_request_abort_active_run()
                    && let Err(error) = cx.model.simulation.request_abort_active_run()
                {
                    cx.wb.prompt(error);
                }
            }
            Self::JobsManager => open_jobs_manager(cx.wb),
            Self::Preflight => {
                let (mut host, controller) = run_host::run_host(cx);
                rspice_app_runs::workbench::preflight::start(&mut host, controller, false);
            }
            // Sole owner of "open the plan manager". The toolbar plan chip,
            // the palette and the Simulation Studio title row all route here
            // instead of building the draft themselves.
            Self::ManagePlans => match cx
                .model
                .sim_setup
                .stable_analysis_plan()
                .map(|plan| plan.id())
            {
                Ok(plan_id) => {
                    let name = cx.model.sim_setup.active_plan_name().clone();
                    cx.state.simulation_workflow = Some(SimulationWorkflowDialog::PlanManager(
                        SimulationPlanManagerDraft::new(plan_id, name.as_str()),
                    ));
                }
                Err(error) => cx.wb.prompt(error),
            },
            Self::AddAnalysis => open_analysis_catalog(cx.model, cx.wb),
            Self::Options => {
                // The Solver & convergence page is the only editor of the
                // plan's engine options: every field of `SimulationOptions`
                // reaches the engine from a control on that page, so the
                // command routes there rather than opening a second surface
                // over the same state.
                open_simulation_options(cx.model);
                cx.state.simulation_page = SimulationPage::Solver;
                cx.wb.activate(Workspace::Simulate);
            }
            Self::Page(page) => {
                cx.wb.activate(Workspace::Simulate);
                cx.state.simulation_page = page;
            }
            Self::Verify(page) => {
                cx.wb.activate(Workspace::Verify);
                cx.state.verification_page = page;
            }
        }
    }
}

/// Drop what the last preflight check authorised, as any edit to what a
/// run reads does: the check's verdict, and the run it prepared.
pub(in crate::workbench) fn invalidate_preflight(cx: &mut Cx<'_, SimulateFeature>) {
    rspice_app_runs::workbench::preflight::invalidate(
        &mut cx.runs.preflight,
        cx.handles.run_controller,
    );
}

/// [`Model::record_plan_receipt`] in the feature's context.
///
/// [`Model::record_plan_receipt`]: crate::workbench::app_state::Model::record_plan_receipt
pub(in crate::workbench) fn record_plan_receipt(
    cx: &mut Cx<'_, SimulateFeature>,
    message: impl Into<String>,
) {
    cx.model.record_plan_receipt(message);
}

/// [`Model::record_plan_refusal`] in the feature's context, answering the
/// gesture on the status bar when the outcome is new.
///
/// [`Model::record_plan_refusal`]: crate::workbench::app_state::Model::record_plan_refusal
pub(in crate::workbench) fn record_plan_refusal(
    cx: &mut Cx<'_, SimulateFeature>,
    message: impl Into<String>,
) {
    if let Some(prompt) = cx.model.record_plan_refusal(message) {
        cx.wb.prompt(prompt);
    }
}

/// Show the run history on its own route.
fn open_jobs_manager(wb: &mut WorkbenchSession) {
    let route = SurfaceRoute::surface(SurfaceId::JobsManager);
    if let Err(error) = wb.navigate(route, RouteTransitionSource::User) {
        wb.prompt(error.to_string());
    }
}

/// Whether the run history is in front, on its own route.
pub(in crate::workbench) fn jobs_manager_open(wb: &WorkbenchSession) -> bool {
    wb.current_route().surface_id() == SurfaceId::JobsManager
}

/// Leave the run history for the route it was opened from, or for the
/// workspace's own surface where nothing opened it.
pub(in crate::workbench) fn close_jobs_manager(wb: &mut WorkbenchSession) {
    if wb.navigate_back(RouteTransitionSource::User).is_some() {
        return;
    }
    let fallback = SurfaceRoute::surface(SurfaceId::from_workspace(wb.workspace));
    if let Err(error) = wb.replace_route(fallback, RouteTransitionSource::User) {
        wb.prompt(format!(
            "Could not close {}: {error}",
            SurfaceId::JobsManager.label()
        ));
    }
}

/// Arm the analysis catalogue, from wherever the reader asked for it.
///
/// `palette_open` withdraws the application's keyboard, and the three routes
/// that arm the catalogue — the navigator's creating action, the empty
/// analysis rail and the Simulate menu — each used to write some of these
/// four fields themselves. A route that forgot the query left the catalogue
/// filtered by whatever the last search had been.
fn open_analysis_catalog(model: &mut Model, wb: &mut WorkbenchSession) {
    // On tablet and phone the creating action is hosted by the navigator
    // drawer. Close that transient layer first, so the modal is never
    // obscured by its own invoker.
    wb.close_drawer();
    model.sim_setup.session.palette_open = true;
    model.sim_setup.session.palette_query.clear();
    model.sim_setup.session.palette_active = 0;
    model.sim_setup.session.palette_scroll_to_active = true;
}

/// Prepare the solver options for editing.
///
/// The draft is refreshed from the currently-effective options so the Solver &
/// convergence page opens on what the next run would actually use, never on a
/// stale draft left behind by an abandoned edit.
fn open_simulation_options(model: &mut Model) {
    model.sim_setup.session.options_draft =
        crate::simulation::plan::OptionsDialogState::from_options(&model.sim_setup.options);
}
