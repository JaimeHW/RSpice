//! What a retained dataset says about the plan that produced it.
//!
//! Two surfaces render this claim — the dataset manifest and the results
//! inspector — and one command acts on it. Resolving it here, over the
//! simulation history and the stable plan rather than over the application
//! root, is what lets the read-only documents ask the same question the
//! dispatcher answers without any of them reaching up a layer for it.

use crate::product::AnalysisInstanceId;
use crate::simulation::plan::SimulationPlan;
use crate::state::{AnalysisResultProvenance, SimulationState};

/// Where "open the producing plan" lands, once it is known to land at all.
pub(crate) struct ProducingPlanHop {
    /// The analysis instance to select on arrival, when the plan still owns
    /// the one that produced the selected analysis.
    pub(crate) instance: Option<AnalysisInstanceId>,
    /// What the reader is owed when the plan is reachable but the instance
    /// that produced this analysis is not. The hop still lands — the plan is
    /// the object being asked for — but it must not pretend the selection was
    /// carried.
    pub(crate) instance_note: Option<String>,
}

/// Resolve the hop from the active retained dataset back to its plan.
///
/// Fails closed with the exact reason, which the disabled control shows and
/// the console refusal repeats: a dataset with no receipt, a manual deck, and
/// a plan that is no longer the active one are three different facts, and
/// collapsing them into "unavailable" tells the reader nothing.
pub(crate) fn producing_plan_hop(
    simulation: &SimulationState,
    plan: Option<&SimulationPlan>,
) -> Result<ProducingPlanHop, &'static str> {
    let run = simulation
        .active_run()
        .ok_or("no retained dataset is selected")?;
    let plan_id = run
        .prepared_receipt()
        .ok_or("this dataset predates prepared-run receipts, so it does not name a producing plan")?
        .simulation_plan_id()
        .ok_or("this dataset was produced by a manual deck, not a simulation plan")?;
    let plan = plan.ok_or("the active simulation plan is unavailable")?;
    if plan.id() != plan_id {
        return Err("the plan that produced this dataset is not the active simulation plan");
    }
    let produced_by = simulation
        .active_analysis()
        .and_then(|analysis| analysis.provenance.as_ref())
        .map(AnalysisResultProvenance::authored_source_instance_id);
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

/// Why the dataset in view cannot be traced back to a plan, if it cannot.
///
/// The read-only surfaces use this to disable their own control with the same
/// sentence the command would have refused with, so the reason is on screen
/// before the click rather than in the console after it.
pub(crate) fn producing_plan_block(
    simulation: &SimulationState,
    plan: Option<&SimulationPlan>,
) -> Option<&'static str> {
    producing_plan_hop(simulation, plan).err()
}
