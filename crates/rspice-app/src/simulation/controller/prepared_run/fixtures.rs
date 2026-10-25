//! Runnable states that the run tier's tests and the application's feature
//! tests prepare runs from.

use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

use rspice_app_state::workbench::examples::load_example;
use rspice_design::drc::DrcResult;

use crate::simulation::controller::test_host::RunTestHost;
use crate::simulation::execution::SimulationRunIntent;
use crate::state::{SpecEntry, SpecPointScope, SpecificationDefinition};

static FIXTURE_NONCE: AtomicU64 = AtomicU64::new(0);

pub fn fixture_dir(label: &str) -> PathBuf {
    let nonce = FIXTURE_NONCE.fetch_add(1, Ordering::Relaxed);
    let directory = std::env::temp_dir().join(format!(
        "rspice-prepared-run-{label}-{}-{nonce}",
        std::process::id()
    ));
    fs::create_dir_all(&directory).expect("create prepared-run fixture");
    directory
}

/// A runnable design whose project owns no technology. The contract admits
/// this: a technology is required only when the project has one or the plan
/// demands one.
pub fn technology_free_runnable_state() -> RunTestHost {
    let mut state = RunTestHost::default();
    for dimension in &mut state.model.sim_setup.run_set.dimensions {
        if dimension.kind
            == rspice_simulation_contract::run_set::RunSetDimensionKind::ProcessSection
        {
            dimension.enabled = false;
        }
    }
    load_example("Voltage Divider", &mut state.model.schematic);
    let mut drc = DrcResult::new();
    drc.completed = true;
    state.model.design_checks.drc_results = Some(drc);
    state.model.design_checks.drc_checked_version = state.model.schematic.topology_version();
    state
}

pub fn runnable_state() -> RunTestHost {
    let mut state = technology_free_runnable_state();
    state.provision_test_project_technology_contract();
    state
}

pub fn manual_deck_state(deck: &str) -> RunTestHost {
    let mut state = RunTestHost::default();
    state.model.simulation.execution.run_intent = SimulationRunIntent::ManualDeck;
    state.model.workspace.content.netlist_source = Some(deck.to_owned());
    state
}

/// A runnable state holding two plans a campaign can freeze. The second,
/// created last, is the active one.
pub fn two_plan_campaign_state() -> (
    RunTestHost,
    crate::product::SimulationPlanId,
    crate::product::SimulationPlanId,
) {
    let mut state = runnable_state();
    let first_plan_id = state.model.sim_setup.stable_analysis_plan().unwrap().id();
    state
        .model
        .workspace
        .content
        .migrate_active_plan_data(first_plan_id);
    let second_plan_id = state
        .model
        .sim_setup
        .create_plan("Second campaign plan")
        .expect("create campaign plan");
    state
        .model
        .workspace
        .content
        .migrate_inactive_plan_data(second_plan_id);
    state
        .model
        .workspace
        .content
        .sync_legacy_specs_projection(second_plan_id);
    (state, first_plan_id, second_plan_id)
}

pub fn definition(name: &str, card: &str) -> SpecificationDefinition {
    let mut definition = SpecificationDefinition::new_from_projection(&SpecEntry {
        measurement: name.into(),
        expression: card.into(),
        min: None,
        max: None,
        unit: "V".into(),
        scope: SpecPointScope::AllPoints,
    });
    definition.define_measurement = true;
    definition
}
