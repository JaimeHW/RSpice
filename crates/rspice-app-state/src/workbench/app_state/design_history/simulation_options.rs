//! The active plan's simulation options, changed as one step.
//!
//! The Solver and Connect rules pages commit the whole options record at
//! once, so undo restores the record the commit replaced and redo the one it
//! wrote. Each direction is guarded by the plan and the record it expects to
//! find: another plan made active, or an options change made since by
//! another page or another step, stands in the way, and the step refuses
//! rather than overwrite it.

use super::*;
use rspice_simulation_contract::options::SimulationOptions;

#[derive(Debug, Clone)]
pub struct SimulationOptionsRecord {
    pub(super) description: String,
    /// The plan whose options the step changed.
    plan: Option<crate::product::SimulationPlanId>,
    before: Box<SimulationOptions>,
    after: Box<SimulationOptions>,
}

impl Model {
    /// Record a committed change of the active plan's simulation options,
    /// from `before` to the options the plan now holds, as one undoable
    /// step named `description`.
    pub fn record_simulation_options_change(
        &mut self,
        description: &str,
        before: SimulationOptions,
    ) {
        let after = self.sim_setup.options.clone();
        if same(&before, &after) {
            return;
        }
        self.push_project_record(
            RecordHeader::project_only(),
            ProjectDesignBody::SimulationOptions(Box::new(SimulationOptionsRecord {
                description: description.to_owned(),
                plan: active_plan(self),
                before: Box::new(before),
                after: Box::new(after),
            })),
        );
    }
}

impl SimulationOptionsRecord {
    pub(super) fn after_design_matches(&self, state: &Model) -> bool {
        active_plan(state) == self.plan && same(&state.sim_setup.options, &self.after)
    }

    pub(super) fn before_design_matches(&self, state: &Model) -> bool {
        active_plan(state) == self.plan && same(&state.sim_setup.options, &self.before)
    }

    pub(super) fn validate_mutation(&self, state: &Model, operation: &str) -> Result<(), String> {
        if !state.project_open.is_open() {
            return Err(format!(
                "{} cannot be {operation} without an open project.",
                self.description
            ));
        }
        if state.safe_mode.project_read_only() {
            return Err(format!(
                "{} cannot be {operation} while the project is read-only.",
                self.description
            ));
        }
        Ok(())
    }

    /// Why the step cannot be `operation` on `state`.
    fn stale(&self, state: &Model, operation: &str) -> String {
        if active_plan(state) == self.plan {
            format!(
                "{} cannot be {operation} because the plan's options changed since.",
                self.description
            )
        } else {
            format!(
                "{} cannot be {operation} while another simulation plan is active.",
                self.description
            )
        }
    }

    pub(super) fn apply_before(&mut self, state: &mut Model) -> Result<(), String> {
        if !self.after_design_matches(state) {
            return Err(self.stale(state, "undone"));
        }
        self.validate_mutation(state, "undone")?;
        restore(state, &self.before, &self.description)
    }

    pub(super) fn apply_after(&mut self, state: &mut Model) -> Result<(), String> {
        if !self.before_design_matches(state) {
            return Err(self.stale(state, "redone"));
        }
        self.validate_mutation(state, "redone")?;
        restore(state, &self.after, &self.description)
    }
}

/// Make `options` the plan's again, through the same plan transaction a
/// page's commit takes, so the plan's revision moves and every check made
/// under the options it leaves is out of date.
fn restore(
    state: &mut Model,
    options: &SimulationOptions,
    description: &str,
) -> Result<(), String> {
    let mut candidate = state.sim_setup.clone();
    candidate.commit_options(options);
    candidate.session.options_draft =
        crate::simulation::plan::OptionsDialogState::from_options(options);
    candidate
        .commit_active_plan_configuration_change(format!("Restored {description}."))
        .map_err(|error| error.to_string())?;
    state.sim_setup = candidate;
    Ok(())
}

fn active_plan(state: &Model) -> Option<crate::product::SimulationPlanId> {
    state.active_plan_revision().map(|(plan, _)| plan)
}

/// Whether two options records say the same thing. Compared in their stored
/// form, as a page's commit compares them, so a field added to the options
/// is never mistaken for no change.
fn same(left: &SimulationOptions, right: &SimulationOptions) -> bool {
    match (serde_json::to_vec(left), serde_json::to_vec(right)) {
        (Ok(left), Ok(right)) => left == right,
        _ => false,
    }
}
