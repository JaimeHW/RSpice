//! Return edges out of a retained dataset.
//!
//! Everything in the studio flows into a run; these are the resolutions that
//! let a reader walk back out of one. Each resolves the whole destination —
//! workspace, page, and selection — before anything navigates, so a hop that
//! cannot land says why instead of arriving somewhere with the object lost.

use crate::ui::widgets::NotificationAction;
use crate::workbench::RSpiceApp;
use crate::workbench::state::ConsoleChannel;
use crate::workbench::state::plan_provenance::{self, ProducingPlanHop};

use super::vocabulary::Command;

/// Whether "Export dataset…" has a displayed view to write.
///
/// The export writes the *displayed* result view, and that resolution starts
/// at the active Results document. Offering the command whenever any dataset
/// was retained meant pressing it from another workspace produced a console
/// warning instead of a file.
pub(super) fn dataset_export_is_resolvable(state: &crate::workbench::AppState) -> bool {
    use crate::workbench::state::{Workspace, WorkspaceDocumentId};

    state.simulation.retained.has_results()
        && matches!(
            state.workbench.documents.active(Workspace::Results),
            Some(
                WorkspaceDocumentId::ResultDataset(_)
                    | WorkspaceDocumentId::VisualizationDocument(_)
            )
        )
}

/// Select the newest run that actually holds a dataset, then activate Results.
///
/// The controls that say "the results" without naming a run — the status
/// bar's engine chip, the split-stage toggle — mean the newest materialized
/// one. Returns false when there is none, so the caller states that rather
/// than navigating to an empty document.
pub(crate) fn open_newest_retained_run(app: &mut RSpiceApp) -> bool {
    let Some(index) = app
        .state
        .simulation
        .retained
        .newest_retained_result_run_index()
    else {
        return false;
    };
    if !app.state.simulation.select_run(index) {
        return false;
    }
    Command::OpenRunInResults.execute(app);
    true
}

/// Take the navigation offer a notice carried.
///
/// A retained notice can outlive what it points at — history is pruned, a
/// project is closed — so the run is re-resolved by its display sequence at
/// the moment the offer is taken, and a lost run is reported rather than
/// leaving the click to do nothing.
pub(crate) fn perform_notification_action(app: &mut RSpiceApp, action: NotificationAction) {
    match action {
        NotificationAction::OpenRunInResults { run_sequence } => {
            if app.state.simulation.select_run_by_sequence(run_sequence) {
                Command::OpenRunInResults.execute(app);
            } else {
                app.state
                    .prompt(crate::workbench::AppState::retired_run_message(
                        run_sequence,
                    ));
            }
        }
        NotificationAction::ShowInConsole { log_id } => show_session_line(&mut app.state, log_id),
    }
}

/// Open the Console on the session's log with line `log_id` selected and in
/// view, or at the end of the log when the line has since been dropped.
pub(crate) fn show_session_line(state: &mut crate::workbench::AppState, log_id: u64) {
    let severity = state
        .log_buffer
        .entries()
        .find(|entry| entry.id == log_id)
        .map(|entry| entry.severity);
    let view = &mut state.workbench.console_view;
    match severity {
        Some(severity) => view.show_session_line(log_id, severity),
        None => view.open(ConsoleChannel::Session),
    }
    open_console_page(&mut state.workbench);
}

/// Open the Console on one run's own log, from its latest line.
///
/// Callers offer this only for a run the session still holds; a run dropped
/// between the offer and the click is reported rather than opened as an empty
/// channel.
pub(crate) fn open_run_log(state: &mut crate::workbench::AppState, run_sequence: u64) {
    if state
        .simulation
        .retained
        .run_by_sequence(run_sequence)
        .is_none()
    {
        state.prompt(crate::workbench::AppState::retired_run_message(
            run_sequence,
        ));
        return;
    }
    state
        .workbench
        .console_view
        .open(ConsoleChannel::Run(run_sequence));
    open_console_page(&mut state.workbench);
}

/// Open the Console on one run's own log with line `line` selected and in
/// view: a line a finding names as the one that states it.
///
/// A run dropped since the finding was made is reported rather than opened
/// as an empty channel.
pub(crate) fn show_run_line(
    state: &mut crate::workbench::AppState,
    run_sequence: u64,
    line: usize,
) {
    let Some(severity) = state
        .simulation
        .retained
        .run_by_sequence(run_sequence)
        .map(|run| run.log.lines().get(line).map(|line| line.severity))
    else {
        state.prompt(format!(
            "Run {run_sequence} is no longer retained in this session, so its log could not be \
             opened."
        ));
        return;
    };
    let view = &mut state.workbench.console_view;
    match severity {
        Some(severity) => view.show_line(ConsoleChannel::Run(run_sequence), line as u64, severity),
        None => view.open(ConsoleChannel::Run(run_sequence)),
    }
    open_console_page(&mut state.workbench);
}

/// The run that produced dataset `dataset`, while the session still holds it.
pub(crate) fn producing_run(
    runs: &[crate::state::SimulationRun],
    dataset: crate::product::DatasetId,
) -> Option<u64> {
    runs.iter()
        .find(|run| run.dataset_id == dataset)
        .map(|run| run.id)
}

fn open_console_page(workbench: &mut crate::workbench::state::WorkbenchState) {
    // Opening the dock alone shows whichever page was last shown.
    workbench.open_console();
    workbench.console_page = crate::workbench::state::ConsolePage::Console;
}

/// Resolve the producing-plan hop for the running application.
///
/// One line of glue over the owner in `workbench::state::plan_provenance`, so
/// the dispatcher and the read-only surfaces that draw the same claim cannot
/// drift apart.
pub(crate) fn producing_plan_hop(app: &RSpiceApp) -> Result<ProducingPlanHop, &'static str> {
    plan_provenance::producing_plan_hop(
        &app.state.simulation,
        app.state.sim_setup.stable_analysis_plan().ok(),
    )
}

/// The run whose executed source the deck hop would open, or why there is
/// none.
///
/// The two refusals are different facts. A session with no run selected has
/// nothing to open. A selected run whose deck the archive no longer holds has
/// something to open and cannot, and the reader has to be told which of the two
/// it is rather than left to infer it from a control that does nothing.
pub(crate) fn task_deck_hop(app: &RSpiceApp) -> Result<u64, &'static str> {
    let run = app
        .state
        .simulation
        .active_run()
        .ok_or("no run is selected")?;
    if app
        .state
        .simulation
        .retained
        .executed_decks
        .get(run.id)
        .is_none()
    {
        return Err(crate::state::absent_deck_reason());
    }
    Ok(run.id)
}

/// Open the source one run's engine was actually handed, or say why not.
///
/// The owner for every route to it. The Jobs manager's row reached
/// `netlist_document::reveal_executed_deck` and spelled its own refusal beside
/// it; a second caller spelling a second refusal for the same missing deck is
/// how one absence acquires two names.
pub(crate) fn open_task_deck(state: &mut crate::workbench::AppState, sequence: u64) -> bool {
    if crate::workbench::documents::netlist_document::reveal_executed_deck(state, sequence, 0) {
        return true;
    }
    state.prompt(format!(
        "The source Run {sequence} executed cannot be opened: {}.",
        crate::state::absent_deck_reason()
    ));
    false
}

/// The run that produced the one check-marked Data Browser quantity, whose
/// log the hop would open.
///
/// The context menus that offer this act on the row they were opened on. The
/// palette has no row, so it acts on the browser's check-marks — and refuses
/// rather than choosing, because five quantities can come from five runs, and
/// picking one silently would be the studio deciding which the reader meant.
pub(crate) fn producer_log_hop(app: &RSpiceApp) -> Result<u64, &'static str> {
    if app.state.simulation.active_run().is_none() {
        return Err("no run is selected");
    }
    let checked = &app.state.ui.results.session.checked_result_quantities;
    let key = match checked.len() {
        0 => return Err("check-mark the Data Browser quantity whose run log to open"),
        1 => checked.iter().next().expect("one check-marked quantity"),
        _ => return Err("check-mark exactly one Data Browser quantity"),
    };
    producing_run(&app.state.simulation.retained.runs, key.dataset_id())
        .ok_or("the run that produced the check-marked quantity is no longer held")
}
