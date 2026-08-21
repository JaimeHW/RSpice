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
