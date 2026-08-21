//! Return edges out of a retained dataset.
//!
//! Everything in the studio flows into a run; these are the resolutions that
//! let a reader walk back out of one. Each resolves the whole destination —
//! workspace, page, and selection — before anything navigates, so a hop that
//! cannot land says why instead of arriving somewhere with the object lost.

use crate::diagnostics::ConsoleMessage;
use crate::product::AnalysisInstanceId;
use crate::ui::widgets::NotificationAction;
use crate::workbench::RSpiceApp;

use super::vocabulary::Command;

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
                app.state.push_user_message(ConsoleMessage::warning(format!(
                    "Run {run_sequence} is no longer retained in this session, so it could not be \
                     opened in Results."
                )));
            }
        }
    }
}

/// Where "open the producing plan" lands, once it is known to land at all.
pub(crate) struct ProducingPlanHop {
    /// The analysis instance to select on arrival, when the plan still owns
    /// the one that produced the selected analysis.
    pub(crate) instance: Option<AnalysisInstanceId>,
    /// What the reader is owed when the plan is reachable but the instance
    /// that produced this analysis is not. The hop still lands — the plan is
    /// the object being asked for — but it must not pretend the selection
    /// was carried.
    pub(crate) instance_note: Option<String>,
}

/// Resolve the hop from the active retained dataset back to its plan.
///
/// Fails closed with the exact reason, which the command's disabled state
/// shows and its console refusal repeats: a dataset with no receipt, a manual
/// deck, and a plan that is no longer the active one are three different
/// facts, and collapsing them into "unavailable" tells the reader nothing.
pub(crate) fn producing_plan_hop(app: &RSpiceApp) -> Result<ProducingPlanHop, &'static str> {
    let run = app
        .state
        .simulation
        .active_run()
        .ok_or("no retained dataset is selected")?;
    let plan_id = run
        .prepared_receipt()
        .ok_or("this dataset predates prepared-run receipts, so it does not name a producing plan")?
        .simulation_plan_id()
        .ok_or("this dataset was produced by a manual deck, not a simulation plan")?;
    let plan = app
        .state
        .sim_setup
        .stable_analysis_plan()
        .map_err(|_| "the active simulation plan is unavailable")?;
    if plan.id() != plan_id {
        return Err("the plan that produced this dataset is not the active simulation plan");
    }
    let produced_by = app
        .state
        .simulation
        .active_analysis()
        .and_then(|analysis| analysis.provenance.as_ref())
        .map(crate::state::AnalysisResultProvenance::authored_source_instance_id);
    let (instance, instance_note) = match produced_by {
        Some(id) if plan.instances().iter().any(|entry| entry.id() == id) => (Some(id), None),
        Some(id) => (
            None,
            Some(format!(
                "Opened simulation plan {plan_id}. The analysis instance {id} that produced the \
                 selected result is no longer in the plan, so nothing was selected on arrival."
            )),
        ),
        None => (
            None,
            Some(format!(
                "Opened simulation plan {plan_id}. The selected result carries no analysis \
                 provenance, so no producing instance could be selected on arrival."
            )),
        ),
    };
    Ok(ProducingPlanHop {
        instance,
        instance_note,
    })
}
