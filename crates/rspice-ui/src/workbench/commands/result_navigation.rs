//! Return edges out of a retained dataset.
//!
//! Everything in the studio flows into a run; these are the resolutions that
//! let a reader walk back out of one. Each resolves the whole destination —
//! workspace, page, and selection — before anything navigates, so a hop that
//! cannot land says why instead of arriving somewhere with the object lost.

use crate::diagnostics::ConsoleMessage;
use crate::ui::widgets::NotificationAction;
use crate::workbench::RSpiceApp;
use crate::workbench::state::plan_provenance::{self, ProducingPlanHop};

use super::vocabulary::Command;

/// Select the newest run that actually holds a dataset, then activate Results.
///
/// The controls that say "the results" without naming a run — the status
/// bar's engine chip, the split-stage toggle — mean the newest materialized
/// one. Returns false when there is none, so the caller states that rather
/// than navigating to an empty document.
pub(crate) fn open_newest_retained_run(app: &mut RSpiceApp) -> bool {
    let Some(index) = app.state.simulation.newest_retained_result_run_index() else {
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
                app.state.push_user_message(ConsoleMessage::warning(
                    crate::workbench::AppState::retired_run_message(run_sequence),
                ));
            }
        }
    }
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
    if app.state.simulation.executed_decks.get(run.id).is_none() {
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
    state.push_user_message(ConsoleMessage::warning(format!(
        "The source Run {sequence} executed cannot be opened: {}.",
        crate::state::absent_deck_reason()
    )));
    false
}

/// The one Data Browser quantity whose producer log the hop would reveal: its
/// stable path, and the name it is known by.
///
/// The context menus that offer this reveal act on the row they were opened
/// on. The palette has no row, so it acts on the browser's check-marks — and
/// refuses rather than choosing, because the producer log of five quantities
/// is five destinations, and picking one silently would be the studio deciding
/// which object the reader meant.
pub(crate) fn producer_log_hop(app: &RSpiceApp) -> Result<(String, String), &'static str> {
    if app.state.simulation.active_run().is_none() {
        return Err("no run is selected");
    }
    let checked = &app.state.ui.results.checked_result_quantities;
    let key = match checked.len() {
        0 => return Err("check-mark the Data Browser quantity whose producer log to reveal"),
        1 => checked.iter().next().expect("one check-marked quantity"),
        _ => return Err("check-mark exactly one Data Browser quantity"),
    };
    let runs = &app.state.simulation.runs;
    let (Ok(path), Ok(quantity)) = (
        crate::workbench::documents::result_document::result_browser_selection_stable_path(
            key, runs,
        ),
        crate::workbench::documents::result_document::result_browser_selection_canonical_name(
            key, runs,
        ),
    ) else {
        return Err("the check-marked quantity no longer resolves in its immutable dataset");
    };
    Ok((path, quantity))
}

/// Narrow the console to one producer's entries and show its newest.
///
/// The filter it installs is the console's own, so the strip above the log
/// names the producer and can be cleared from there.
pub(crate) fn reveal_producer_log(app: &mut RSpiceApp, producer: String, quantity: &str) {
    app.state.workbench.console_producer_filter = Some(
        crate::workbench::state::ConsoleProducerFilter::new(producer, quantity),
    );
    app.state.workbench.console_page = crate::workbench::state::ConsolePage::Console;
    Command::OpenConsole.execute(app);
}
