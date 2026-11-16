//! The Simulate setup's own history: each page's committed edit, one step.
//!
//! Every Simulate setup page commits through a plan transaction, and each
//! commit is recorded here as one step holding the parts of the setup it
//! changed, as they were and as they became. Undo makes the parts what they
//! were and Redo what they became, each through a plan transaction of its
//! own, so the plan's revision only moves forward, every check made under the
//! setup it leaves expires, and the plan's receipts are kept.
//!
//! The steps are a history beside the project's design history, not in it:
//! Undo on a Simulate page takes back that page's last setup edit and never a
//! schematic edit, and Undo on the schematic never takes back a setup edit.
//! Each step is guarded by the plan and the revision it expects to find: the
//! active plan replaced, or a plan edit made since outside this history,
//! refuses the step and says which.

use super::*;
use crate::product::SimulationPlanId;
use crate::simulation::plan::{OptionsDialogState, SimulationPlan};
use rspice_model_library::SimulationPlanModelBinding;
use rspice_simulation_contract::options::SimulationOptions;
use rspice_simulation_contract::output_policy::SimulationSavePolicy;
use rspice_simulation_contract::plan_payload::{
    SimulationPlanPayload, SimulationPlanPayloadRecord,
};
use rspice_simulation_contract::run_set::RunSetSnapshot;

/// Which parts of the setup a step holds.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SetupParts(u8);

impl SetupParts {
    /// The solver options, connect rules among them.
    pub const OPTIONS: Self = Self(1);
    /// What is saved and how it streams.
    pub const SAVE_POLICY: Self = Self(1 << 1);
    /// The model libraries bound to the plan.
    pub const MODEL_BINDINGS: Self = Self(1 << 2);
    /// The plan's design variables, saved outputs, capture groups and
    /// specifications.
    pub const PLAN_DATA: Self = Self(1 << 3);
    /// The analyses: each one's settings, name, enablement and place, and
    /// the removed ones.
    pub const ANALYSES: Self = Self(1 << 4);
    /// The run set's corners and sweeps.
    pub const RUN_SET: Self = Self(1 << 5);

    #[must_use]
    pub const fn and(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }

    const fn has(self, part: Self) -> bool {
        self.0 & part.0 != 0
    }
}

/// The parts of the setup one side of a step holds.
#[derive(Debug, Clone)]
pub struct SetupSnapshot {
    plan: Option<(SimulationPlanId, ObjectRevision)>,
    parts: SetupParts,
    options: Option<Box<SimulationOptions>>,
    save_policy: Option<Box<SimulationSavePolicy>>,
    model_bindings: Option<Vec<SimulationPlanModelBinding>>,
    /// The active plan's payload, `None` inside when it owns none.
    plan_data: Option<Option<Box<SimulationPlanPayload>>>,
    analyses: Option<Box<SimulationPlan>>,
    run_set: Option<Box<RunSetSnapshot>>,
}

/// One committed setup edit.
#[derive(Debug, Clone)]
pub struct SimulationSetupRecord {
    description: String,
    plan: SimulationPlanId,
    /// The revision the plan must stand at for the step to be taken in the
    /// direction its stack offers it.
    guard: ObjectRevision,
    before: SetupSnapshot,
    after: SetupSnapshot,
}

impl Model {
    /// The `parts` of the setup as they stand, for a commit to record.
    #[must_use]
    pub fn setup_snapshot(&self, parts: SetupParts) -> SetupSnapshot {
        let setup = &self.sim_setup;
        let plan = self.active_plan_revision();
        SetupSnapshot {
            plan,
            parts,
            options: parts
                .has(SetupParts::OPTIONS)
                .then(|| Box::new(setup.options.clone())),
            save_policy: parts
                .has(SetupParts::SAVE_POLICY)
                .then(|| Box::new(setup.save_policy)),
            model_bindings: parts
                .has(SetupParts::MODEL_BINDINGS)
                .then(|| setup.model_bindings.clone()),
            plan_data: parts.has(SetupParts::PLAN_DATA).then(|| {
                plan.and_then(|(id, _)| self.workspace.content.plan_data(id))
                    .map(|payload| Box::new(payload.clone()))
            }),
            analyses: parts
                .has(SetupParts::ANALYSES)
                .then(|| setup.analysis_plan.clone().map(Box::new))
                .flatten(),
            run_set: parts
                .has(SetupParts::RUN_SET)
                .then(|| Box::new(setup.run_set.snapshot())),
        }
    }

    /// Record a committed setup edit, from `before` to the same parts as they
    /// now stand, as one step named `description`. A commit that moved no
    /// plan revision changed nothing and records nothing.
    pub fn record_setup_change(&mut self, description: &str, before: SetupSnapshot) {
        let after = self.setup_snapshot(before.parts);
        let (Some((plan, revision)), Some((plan_before, revision_before))) =
            (after.plan, before.plan)
        else {
            return;
        };
        if plan != plan_before || revision == revision_before {
            return;
        }
        let history = &mut self.project_design_history;
        history.setup_undo.push(SimulationSetupRecord {
            description: description.to_owned(),
            plan,
            guard: revision,
            before,
            after,
        });
        if history.setup_undo.len() > MAX_PROJECT_DESIGN_STEPS {
            history.setup_undo.remove(0);
        }
        history.setup_redo.clear();
    }

    /// Whether Undo on a Simulate page would take a step.
    #[must_use]
    pub fn can_undo_simulation_setup(&self) -> bool {
        self.project_design_history
            .setup_undo
            .last()
            .is_some_and(|record| record.stands(self))
    }

    /// Whether Redo on a Simulate page would take a step.
    #[must_use]
    pub fn can_redo_simulation_setup(&self) -> bool {
        self.project_design_history
            .setup_redo
            .last()
            .is_some_and(|record| record.stands(self))
    }

    /// Take back the newest setup step. `None` when there is none; otherwise
    /// what was undone, or why it could not be.
    pub fn undo_simulation_setup(&mut self) -> Option<Result<String, String>> {
        let mut record = self.project_design_history.setup_undo.pop()?;
        let outcome = record.take(self, Direction::Undo);
        let history = &mut self.project_design_history;
        match &outcome {
            Ok(_) => history.setup_redo.push(record),
            Err(_) => history.setup_undo.push(record),
        }
        Some(outcome)
    }

    /// Take the newest undone setup step again, as [`Self::undo_simulation_setup`].
    pub fn redo_simulation_setup(&mut self) -> Option<Result<String, String>> {
        let mut record = self.project_design_history.setup_redo.pop()?;
        let outcome = record.take(self, Direction::Redo);
        let history = &mut self.project_design_history;
        match &outcome {
            Ok(_) => history.setup_undo.push(record),
            Err(_) => history.setup_redo.push(record),
        }
        Some(outcome)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Undo,
    Redo,
}

impl SimulationSetupRecord {
    /// Whether the plan stands where this step expects it.
    fn stands(&self, state: &Model) -> bool {
        state.active_plan_revision() == Some((self.plan, self.guard))
            && state.project_open.is_open()
            && !state.safe_mode.project_read_only()
    }

    fn take(&mut self, state: &mut Model, direction: Direction) -> Result<String, String> {
        let (verb, side) = match direction {
            Direction::Undo => ("undone", &self.before),
            Direction::Redo => ("redone", &self.after),
        };
        let description = &self.description;
        if !state.project_open.is_open() {
            return Err(format!(
                "The {description} edit cannot be {verb} without an open project."
            ));
        }
        if state.safe_mode.project_read_only() {
            return Err(format!(
                "The {description} edit cannot be {verb} while the project is read-only."
            ));
        }
        match state.active_plan_revision() {
            Some((plan, revision)) if plan == self.plan && revision == self.guard => {}
            Some((plan, _)) if plan == self.plan => {
                return Err(format!(
                    "The {description} edit cannot be {verb} because the plan changed since."
                ));
            }
            _ => {
                return Err(format!(
                    "The {description} edit cannot be {verb} while another simulation plan is active."
                ));
            }
        }
        let past = match direction {
            Direction::Undo => "Undid",
            Direction::Redo => "Redid",
        };
        let detail = format!("{past} the {description} edit.");
        restore(state, self.plan, side, &detail)?;
        self.guard = state
            .active_plan_revision()
            .map_or(self.guard, |(_, revision)| revision);
        Ok(detail)
    }
}

/// Make the parts `side` holds the plan's again, through one plan
/// transaction, adopted only once every part and the transaction succeeded.
fn restore(
    state: &mut Model,
    plan: SimulationPlanId,
    side: &SetupSnapshot,
    detail: &str,
) -> Result<(), String> {
    let mut setup = state.sim_setup.clone();
    if let Some(options) = &side.options {
        setup.commit_options(options);
        setup.session.options_draft = OptionsDialogState::from_options(options);
    }
    if let Some(policy) = &side.save_policy {
        setup.save_policy = **policy;
    }
    if let Some(bindings) = &side.model_bindings {
        state
            .model_library_manager
            .validate_simulation_plan_bindings(bindings)?;
        setup.model_bindings.clone_from(bindings);
    }
    if let Some(run_set) = &side.run_set {
        setup.run_set.restore_snapshot((**run_set).clone());
    }
    let workspace = match &side.plan_data {
        Some(payload) => {
            let mut workspace = state.workspace.clone();
            let records = &mut workspace.content.simulation_plan_payloads;
            match (
                records.iter().position(|record| record.plan_id == plan),
                payload,
            ) {
                (Some(index), Some(payload)) => records[index].payload = (**payload).clone(),
                (Some(index), None) => {
                    records.remove(index);
                }
                (None, Some(payload)) => records.push(SimulationPlanPayloadRecord {
                    plan_id: plan,
                    payload: (**payload).clone(),
                }),
                (None, None) => {}
            }
            workspace
                .content
                .validate_simulation_configuration()
                .map_err(|error| error.to_string())?;
            Some(workspace)
        }
        None => None,
    };
    // The analyses commit with the plan's revision, which stamps them, so
    // they are the transaction itself when the step holds them.
    let receipt = match &side.analyses {
        Some(analyses) => {
            setup
                .validate_plan_catalog()
                .map_err(|error| error.to_string())?;
            let receipt = setup
                .stable_analysis_plan_mut()?
                .restore_analyses(analyses, detail)
                .map_err(|error| error.to_string())?;
            setup.refresh_legacy_analysis_projections();
            receipt
        }
        None => setup
            .commit_active_plan_configuration_change(detail)
            .map_err(|error| error.to_string())?,
    };
    state.sim_setup = setup;
    if let Some(workspace) = workspace {
        state.workspace = workspace;
    }
    state.record_plan_receipt(receipt.status_line());
    Ok(())
}
