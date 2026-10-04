//! Real checkpoint evidence for app retention and persistence tests.

use super::*;
use crate::simulation::plan::{AnalysisDraft, AnalysisKind};
use rspice_simulation::execution_options::SpecExecutionOptions;
use rspice_simulation_contract::analysis_spec::AnalysisSpec;
use rspice_simulation_contract::design_variable::{
    DesignVariable, DesignVariableOverridePolicy, DesignVariableScope,
    DesignVariableSweepEligibility,
};
use rspice_simulation_contract::design_variable_quantity::DesignVariableQuantity;
use rspice_simulation_contract::mc_checkpoint::McCheckpointConfig;
use rspice_simulation_contract::mc_draft::{McConfig, McDialogState, McDistribution};
use std::sync::Arc;

pub(crate) fn checkpoint_state() -> AppState {
    let mut state = AppState::default();
    state.simulation.execution.run_intent = SimulationRunIntent::SimulateRunSet;
    for dimension in &mut state.sim_setup.run_set.dimensions {
        if dimension.kind
            == rspice_simulation_contract::run_set::RunSetDimensionKind::ProcessSection
        {
            dimension.enabled = false;
        }
    }
    crate::workbench::examples::load_example("Voltage Divider", &mut state.schematic);
    for component in &mut state.schematic.document_mut_for_test().components {
        match component.name.as_str() {
            "VCC" => component.value = "1".into(),
            "R1" => component.value = "{r}".into(),
            "R2" => component.value = "1k".into(),
            _ => {}
        }
    }
    let plan = state.sim_setup.analysis_plan.as_mut().unwrap();
    let ids: Vec<_> = plan
        .instances()
        .iter()
        .map(|instance| instance.id())
        .collect();
    for id in ids {
        plan.set_enabled(id, false).unwrap();
    }
    let (op, _) = plan.insert(AnalysisKind::OperatingPoint).unwrap();
    let (mc, _) = plan.insert(AnalysisKind::MonteCarlo).unwrap();
    plan.edit(mc, |draft| {
        *draft = AnalysisDraft::MonteCarlo(McDialogState::from_config(&McConfig {
            num_runs: 1,
            seed: Some(37),
            distribution: McDistribution::Uniform,
            variation_pct: 20.0,
            base_analysis: Some(op),
            measurements: vec!["scalar:V(out)".into()],
            histogram_bins: 5,
            checkpoint: Some(McCheckpointConfig {
                publish_every: 1.try_into().unwrap(),
                resume: vec![],
            }),
            ..Default::default()
        }));
    })
    .unwrap();
    let plan_id = plan.id();
    state
        .workspace
        .content
        .ensure_active_plan_data(plan_id)
        .design_variables = vec![
        DesignVariable::new(
            "r",
            "1k",
            DesignVariableQuantity::Dimensionless,
            DesignVariableScope::Testbench,
            "",
            None,
            DesignVariableSweepEligibility::FixedParameter,
            DesignVariableOverridePolicy::InheritOwnerOnly,
        )
        .unwrap(),
    ];
    state.sync_active_schematic_to_workspace();
    let checked = state
        .run_active_design_checks()
        .expect("check the fixture design");
    assert!(!checked.has_errors(), "{checked:?}");
    state
}

pub(crate) fn start_checkpoint_run(
    state: &AppState,
) -> ((AnalysisSpec, SpecExecutionOptions), SimulationRunner) {
    let mut controller = SimulationController::new();
    controller
        .prepare_run_set_for_preflight(state)
        .expect("prepare the complete fixture plan");
    let dispatch = controller
        .consume_snapshot_for_dispatch(state)
        .expect("authorize the unchanged fixture plan");
    let mut tasks = dispatch
        .into_tasks()
        .into_iter()
        .filter(|task| matches!(task.spec(), AnalysisSpec::MonteCarlo { .. }));
    let task = tasks.next().expect("prepared Monte Carlo task");
    assert!(tasks.next().is_none(), "one checkpoint population");
    let metadata = (task.spec().clone(), task.spec_options().clone());
    let task = task
        .resolve_dependency_artifacts(&Default::default())
        .expect("the study executes its own prepared operating point");
    let mut runner = SimulationRunner::new();
    runner
        .start_prepared(task, false)
        .expect("start checked Monte Carlo task");
    (metadata, runner)
}

pub(crate) fn completed_checkpoint_fixture() -> (
    (AnalysisSpec, SpecExecutionOptions),
    Arc<[u8]>,
    SimulationResult,
) {
    let (metadata, mut runner) = start_checkpoint_run(&checkpoint_state());
    wait_until_finished_unpolled(&runner);
    let bytes = runner
        .take_monte_carlo_checkpoint()
        .expect("completed trial journal");
    let result = runner.poll_result().unwrap().expect("Monte Carlo succeeds");
    (metadata, bytes, result)
}
