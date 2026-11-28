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
//!
//! Each test keeps its own steps, so opening another test and coming back
//! finds them as they were. A test's steps stand on the plan revision its
//! last step left: a plan edit made since outside this history refuses the
//! step and says so. What the steps of every test hold together is held to
//! a share of the machine's memory, and the oldest steps go first, so a
//! long session keeps every step the machine can spare and no count cuts it
//! short.

use std::collections::{HashMap, VecDeque};

use super::*;
use crate::product::SimulationPlanId;
use crate::simulation::plan::{OptionsDialogState, SimulationPlan};
use rspice_model_library::SimulationPlanModelBinding;
use rspice_simulation_contract::design_configuration::DesignConfiguration;
use rspice_simulation_contract::options::SimulationOptions;
use rspice_simulation_contract::output_policy::SimulationSavePolicy;
use rspice_simulation_contract::plan_payload::{
    SimulationPlanPayload, SimulationPlanPayloadRecord,
};
use rspice_simulation_contract::run_set::{ReferencePoint, RunSetSnapshot};

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
    /// The reference process and temperature the nominal run takes.
    pub const REFERENCE_PVT: Self = Self(1 << 6);
    /// The hierarchy configuration the plan's design elaborates under.
    pub const DESIGN_CONFIGURATION: Self = Self(1 << 7);

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
    reference_pvt: Option<ReferencePoint>,
    design_configuration: Option<DesignConfiguration>,
}

/// One committed setup edit.
#[derive(Debug, Clone)]
pub struct SimulationSetupRecord {
    description: String,
    plan: SimulationPlanId,
    before: SetupSnapshot,
    after: SetupSnapshot,
    /// Where the step was recorded among every test's steps, oldest lowest.
    order: u64,
    /// About what the step holds in memory.
    bytes: usize,
}

/// Every test's setup steps, each test's its own.
#[derive(Debug, Clone, Default)]
pub(super) struct SetupHistories {
    tests: HashMap<SimulationPlanId, SetupHistory>,
    /// How many steps were recorded: the order of the next.
    recorded: u64,
}

/// One test's setup steps.
#[derive(Debug, Clone, Default)]
struct SetupHistory {
    /// The steps Undo takes, oldest first.
    undo: VecDeque<SimulationSetupRecord>,
    /// The steps Redo takes, the next one last.
    redo: Vec<SimulationSetupRecord>,
    /// The revision the steps stand on: where the last one recorded, undone
    /// or redone left the test's plan.
    at: Option<ObjectRevision>,
}

/// The memory installed where the host cannot say, as a browser cannot: the
/// most a 32-bit browser page can address.
const ADDRESS_SPACE_WITHOUT_A_MACHINE: u64 = 4 << 30;

/// How much of the memory installed the setup steps may hold together: a
/// hundredth, which keeps thousands of steps of an ordinary plan and leaves
/// the rest to the runs.
const SHARE_OF_MEMORY: u64 = 100;

/// What every test's setup steps may hold together on this machine.
fn setup_history_budget() -> usize {
    let installed = rspice_simulation::execution::run_memory::machine_memory()
        .map_or(ADDRESS_SPACE_WITHOUT_A_MACHINE, |memory| {
            memory.installed_bytes
        });
    usize::try_from(installed / SHARE_OF_MEMORY).unwrap_or(usize::MAX)
}

impl SetupHistories {
    /// Where `plan`'s steps stand, `None` when it has none.
    fn at(&self, plan: SimulationPlanId) -> Option<ObjectRevision> {
        self.tests.get(&plan).and_then(|history| history.at)
    }

    /// Drop the oldest steps, whichever test holds them, until every test's
    /// steps together hold no more than `budget` bytes. The newest step is
    /// kept whatever it holds. An undo step is dropped from the oldest end of
    /// its test's list and a redo step from the far end of its, the last it
    /// would redo, so what is left still steps in order.
    fn trim(&mut self, budget: usize) {
        let mut held: usize = self
            .tests
            .values()
            .flat_map(|history| history.undo.iter().chain(&history.redo))
            .map(|record| record.bytes)
            .sum();
        let newest = self.recorded;
        while held > budget {
            let oldest = self
                .tests
                .iter()
                .flat_map(|(plan, history)| {
                    [
                        history.undo.front().map(|step| (step.order, *plan, false)),
                        history.redo.first().map(|step| (step.order, *plan, true)),
                    ]
                })
                .flatten()
                .filter(|(order, ..)| *order != newest)
                .min_by_key(|(order, ..)| *order);
            let Some((_, plan, redo)) = oldest else {
                break;
            };
            let Some(history) = self.tests.get_mut(&plan) else {
                break;
            };
            let dropped = if redo {
                (!history.redo.is_empty()).then(|| history.redo.remove(0))
            } else {
                history.undo.pop_front()
            };
            let Some(dropped) = dropped else {
                break;
            };
            held = held.saturating_sub(dropped.bytes);
        }
        self.tests
            .retain(|_, history| !history.undo.is_empty() || !history.redo.is_empty());
    }
}

/// About how many bytes `value` holds: the length of its JSON, which counts
/// every string, list and number it owns.
fn held_bytes(value: &impl serde::Serialize) -> usize {
    struct Count(usize);
    impl std::io::Write for Count {
        fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
            self.0 += bytes.len();
            Ok(bytes.len())
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }
    let mut count = Count(0);
    // A value JSON cannot spell is counted as far as it got.
    let _ = serde_json::to_writer(&mut count, value);
    count.0
}

impl SetupSnapshot {
    /// About how many bytes this side of a step holds.
    fn bytes(&self) -> usize {
        let run_set = self.run_set.as_deref().map_or(0, |run_set| {
            held_bytes(&run_set.dimensions)
                + held_bytes(&run_set.composition)
                + held_bytes(&run_set.budgets)
        });
        std::mem::size_of::<Self>()
            + self.options.as_deref().map_or(0, held_bytes)
            + self.model_bindings.as_ref().map_or(0, held_bytes)
            + self.plan_data.as_ref().map_or(0, held_bytes)
            + self.analyses.as_deref().map_or(0, held_bytes)
            + run_set
    }
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
            reference_pvt: parts
                .has(SetupParts::REFERENCE_PVT)
                .then_some(setup.reference_pvt),
            design_configuration: parts
                .has(SetupParts::DESIGN_CONFIGURATION)
                .then_some(setup.design_configuration),
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
        let histories = &mut self.project_design_history.setup;
        histories.recorded += 1;
        let bytes = std::mem::size_of::<SimulationSetupRecord>()
            + description.len()
            + before.bytes()
            + after.bytes();
        let record = SimulationSetupRecord {
            description: description.to_owned(),
            plan,
            before,
            after,
            order: histories.recorded,
            bytes,
        };
        let history = histories.tests.entry(plan).or_default();
        if history.at != Some(revision_before) {
            // The plan was edited outside this history since its last step,
            // so the steps below describe a setup that no longer stands.
            history.undo.clear();
        }
        history.at = Some(revision);
        history.undo.push_back(record);
        history.redo.clear();
        histories.trim(setup_history_budget());
    }

    /// The active test's setup steps, `None` when it has none.
    fn active_setup_history(&self) -> Option<&SetupHistory> {
        let (plan, _) = self.active_plan_revision()?;
        self.project_design_history.setup.tests.get(&plan)
    }

    /// Whether Undo on a Simulate page would take a step.
    #[must_use]
    pub fn can_undo_simulation_setup(&self) -> bool {
        self.active_setup_history()
            .and_then(|history| history.undo.back())
            .is_some_and(|record| record.stands(self))
    }

    /// Whether Redo on a Simulate page would take a step.
    #[must_use]
    pub fn can_redo_simulation_setup(&self) -> bool {
        self.active_setup_history()
            .and_then(|history| history.redo.last())
            .is_some_and(|record| record.stands(self))
    }

    /// Take back the active test's newest setup step. `None` when it has
    /// none; otherwise what was undone, or why it could not be.
    pub fn undo_simulation_setup(&mut self) -> Option<Result<String, String>> {
        let (plan, _) = self.active_plan_revision()?;
        let histories = &mut self.project_design_history.setup;
        let mut record = histories.tests.get_mut(&plan)?.undo.pop_back()?;
        let outcome = record.take(self, Direction::Undo);
        let history = self
            .project_design_history
            .setup
            .tests
            .entry(plan)
            .or_default();
        match &outcome {
            Ok(_) => history.redo.push(record),
            Err(_) => history.undo.push_back(record),
        }
        Some(outcome)
    }

    /// Take the active test's newest undone setup step again, as
    /// [`Self::undo_simulation_setup`].
    pub fn redo_simulation_setup(&mut self) -> Option<Result<String, String>> {
        let (plan, _) = self.active_plan_revision()?;
        let histories = &mut self.project_design_history.setup;
        let mut record = histories.tests.get_mut(&plan)?.redo.pop()?;
        let outcome = record.take(self, Direction::Redo);
        let history = self
            .project_design_history
            .setup
            .tests
            .entry(plan)
            .or_default();
        match &outcome {
            Ok(_) => history.undo.push_back(record),
            Err(_) => history.redo.push(record),
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
    /// Whether the plan stands where this step expects it: on the revision
    /// the history's last step left, with no edit made outside it since.
    fn stands(&self, state: &Model) -> bool {
        let at = state.project_design_history.setup.at(self.plan);
        state
            .active_plan_revision()
            .is_some_and(|(plan, revision)| plan == self.plan && at == Some(revision))
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
        let at = state.project_design_history.setup.at(self.plan);
        match state.active_plan_revision() {
            Some((plan, revision)) if plan == self.plan && at == Some(revision) => {}
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
        let revision = state.active_plan_revision().map(|(_, revision)| revision);
        state
            .project_design_history
            .setup
            .tests
            .entry(self.plan)
            .or_default()
            .at = revision;
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
    // Ahead of the options, which state its temperature again.
    if let Some(point) = side.reference_pvt {
        setup.set_reference_pvt(point.process, point.temperature_celsius)?;
    }
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
    if let Some(design) = side.design_configuration {
        setup.design_configuration = design;
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::simulation::plan::AnalysisDraft;
    use crate::state::pdk_config::PdkConfig;

    fn model() -> Model {
        Model::session_start(
            PdkConfig::default(),
            None,
            crate::workbench::workspace::restore_model_library_manager,
        )
    }

    /// The active test's transient stop, as its plan holds it.
    fn stop(model: &Model) -> String {
        model
            .sim_setup
            .stable_analysis_plan()
            .expect("the plan resolves")
            .instances()
            .iter()
            .find_map(|instance| match instance.draft() {
                AnalysisDraft::Transient(setup) => Some(setup.stop.clone()),
                _ => None,
            })
            .expect("the plan holds a transient")
    }

    /// The transient's stop set to `stop`, as the Analyses page commits a
    /// field it lets go of: one step.
    fn set_stop(model: &mut Model, stop: &str) {
        let step = model.setup_snapshot(SetupParts::ANALYSES);
        let plan = model
            .sim_setup
            .stable_analysis_plan_mut()
            .expect("the plan resolves");
        let transient = plan
            .instances()
            .iter()
            .find(|instance| matches!(instance.draft(), AnalysisDraft::Transient(_)))
            .expect("the plan holds a transient")
            .id();
        plan.edit(transient, |draft| {
            if let AnalysisDraft::Transient(setup) = draft {
                stop.clone_into(&mut setup.stop);
            }
        })
        .expect("the transient takes its stop time");
        model.sim_setup.refresh_legacy_analysis_projections();
        model.record_setup_change("Analyses", step);
    }

    /// The active test's relative tolerance set to `reltol`, as the Solver
    /// page commits it: one step.
    fn set_reltol(model: &mut Model, reltol: f64) {
        let step = model.setup_snapshot(SetupParts::OPTIONS);
        let mut options = model.sim_setup.options.clone();
        options.reltol = reltol;
        model.sim_setup.commit_options(&options);
        model
            .sim_setup
            .commit_active_plan_configuration_change("relative tolerance")
            .expect("the plan takes the options");
        model.record_setup_change("Solver", step);
    }

    fn undo(model: &mut Model) {
        model
            .undo_simulation_setup()
            .expect("a step to undo")
            .expect("the step is undone");
    }

    fn redo(model: &mut Model) {
        model
            .redo_simulation_setup()
            .expect("a step to redo")
            .expect("the step is redone");
    }

    #[test]
    fn undo_after_forty_edits_still_reaches_the_first() {
        let mut model = model();
        let drawn = stop(&model);
        for edit in 1..=40 {
            set_stop(&mut model, &format!("{edit}n"));
        }
        assert_eq!(stop(&model), "40n");

        for _ in 1..40 {
            undo(&mut model);
        }
        assert_eq!(stop(&model), "1n", "thirty-nine Undos leave the first edit");
        undo(&mut model);
        assert_eq!(
            stop(&model),
            drawn,
            "the fortieth takes the first edit back"
        );
        assert!(
            !model.can_undo_simulation_setup(),
            "and nothing stands before it"
        );
        for _ in 1..=40 {
            redo(&mut model);
        }
        assert_eq!(stop(&model), "40n", "Redo makes every edit again");
    }

    #[test]
    fn switching_tests_keeps_each_tests_own_history() {
        let mut model = model();
        let (first, _) = model.active_plan_revision().expect("a test is open");
        let drawn = model.sim_setup.options.reltol;
        set_reltol(&mut model, 2e-3);
        set_reltol(&mut model, 3e-3);
        undo(&mut model);
        assert_eq!(model.sim_setup.options.reltol, 2e-3);

        let second = model
            .sim_setup
            .create_plan("Second")
            .expect("a second test opens");
        let second_drawn = model.sim_setup.options.reltol;
        assert!(
            !model.can_undo_simulation_setup(),
            "a new test has no steps of its own"
        );
        set_reltol(&mut model, 5e-3);

        model
            .sim_setup
            .activate_plan(first)
            .expect("the first test opens again");
        assert!(model.can_redo_simulation_setup(), "its undone step waits");
        redo(&mut model);
        assert_eq!(model.sim_setup.options.reltol, 3e-3);
        undo(&mut model);
        undo(&mut model);
        assert_eq!(
            model.sim_setup.options.reltol, drawn,
            "both of its steps are taken back"
        );
        assert!(!model.can_undo_simulation_setup());

        model
            .sim_setup
            .activate_plan(second)
            .expect("the second test opens again");
        assert_eq!(model.sim_setup.options.reltol, 5e-3);
        undo(&mut model);
        assert_eq!(
            model.sim_setup.options.reltol, second_drawn,
            "the second test's own step is taken back"
        );
        assert!(!model.can_undo_simulation_setup());
    }

    /// A step of `plan` holding `bytes`, recorded `order`th.
    fn step(plan: SimulationPlanId, order: u64, bytes: usize) -> SimulationSetupRecord {
        let side = SetupSnapshot {
            plan: None,
            parts: SetupParts(0),
            options: None,
            save_policy: None,
            model_bindings: None,
            plan_data: None,
            analyses: None,
            run_set: None,
            reference_pvt: None,
            design_configuration: None,
        };
        SimulationSetupRecord {
            description: String::new(),
            plan,
            before: side.clone(),
            after: side,
            order,
            bytes,
        }
    }

    /// The order of every step `histories` holds for `plan`: those to undo,
    /// then those to redo.
    fn orders(histories: &SetupHistories, plan: SimulationPlanId) -> (Vec<u64>, Vec<u64>) {
        histories
            .tests
            .get(&plan)
            .map_or_else(Default::default, |history| {
                (
                    history.undo.iter().map(|step| step.order).collect(),
                    history.redo.iter().map(|step| step.order).collect(),
                )
            })
    }

    #[test]
    fn the_oldest_steps_go_first_whichever_test_holds_them() {
        let (first, second) = (SimulationPlanId::new(), SimulationPlanId::new());
        let mut histories = SetupHistories::default();
        // The first test made steps 1, 3 and 5 and took 5 and 3 back; the
        // second made steps 2, 4 and 6, the newest.
        let first_steps = histories.tests.entry(first).or_default();
        first_steps.undo.push_back(step(first, 1, 10));
        first_steps
            .redo
            .extend([step(first, 5, 10), step(first, 3, 10)]);
        let second_steps = histories.tests.entry(second).or_default();
        second_steps
            .undo
            .extend([2, 4, 6].map(|order| step(second, order, 10)));
        histories.recorded = 6;

        histories.trim(60);
        assert_eq!(orders(&histories, first), (vec![1], vec![5, 3]));
        assert_eq!(orders(&histories, second), (vec![2, 4, 6], vec![]));

        histories.trim(40);
        assert_eq!(
            orders(&histories, first),
            (vec![], vec![5, 3]),
            "step 1 goes first"
        );
        assert_eq!(
            orders(&histories, second),
            (vec![4, 6], vec![]),
            "then step 2, in the other test"
        );

        histories.trim(20);
        assert_eq!(orders(&histories, second), (vec![6], vec![]));
        assert_eq!(
            orders(&histories, first),
            (vec![], vec![3]),
            "a redo list loses the step it would redo last"
        );

        histories.trim(0);
        assert!(!histories.tests.contains_key(&first));
        assert_eq!(
            orders(&histories, second),
            (vec![6], vec![]),
            "the newest step stays whatever it holds"
        );
    }
}
