//! Project and plan integration for signed-technology demand and preflight messages.

use crate::product::AnalysisInstanceId;
use crate::product::ProcessCorner;
use crate::simulation::dialog::corner::CornerDialogState;
use crate::simulation::plan::AnalysisDraft;
use crate::simulation::run_set::RunSetDimensionKind;
use crate::workbench::app_state::AppState;
use rspice_simulation::preparation::{TechnologyDemand, TechnologyDemandReason};

fn insert_corner(state: &mut AppState, corner: CornerDialogState) -> AnalysisInstanceId {
    let plan = state
        .sim_setup
        .stable_analysis_plan_mut()
        .expect("a default state owns a stable plan");
    let position = plan.instances().len();
    plan.insert_draft_with_id(
        AnalysisInstanceId::new(),
        AnalysisDraft::Corner(corner),
        true,
        position,
    )
    .expect("a corner analysis has no prerequisites")
    .0
}

/// Turn the plan's process axis on, declaring SS/TT/FF.
///
/// A fresh plan declares every axis but enables none, and the sections a
/// corner analysis demands come from the plan's axis rather than from the
/// instance — so a test about section demand has to declare one.
fn enable_global_process_axis(state: &mut AppState) {
    for dimension in &mut state.sim_setup.run_set.dimensions {
        if dimension.kind == RunSetDimensionKind::ProcessSection {
            dimension.enabled = true;
        }
    }
}

fn sole_reason(demand: &TechnologyDemand) -> &TechnologyDemandReason {
    assert_eq!(
        demand.reasons().len(),
        1,
        "expected exactly one reason: {:?}",
        demand.reasons()
    );
    &demand.reasons()[0]
}

fn disable_global_process_axis(state: &mut AppState) {
    for dimension in &mut state.sim_setup.run_set.dimensions {
        if dimension.kind == RunSetDimensionKind::ProcessSection {
            dimension.enabled = false;
        }
    }
}

fn bind_supply_source(run_set: &mut crate::simulation::run_set::RunSetState) {
    for dimension in &mut run_set.dimensions {
        if dimension.kind == RunSetDimensionKind::Supply {
            dimension.source = format!(
                "{}VDD",
                crate::simulation::run_set::NETLIST_SUPPLY_SOURCE_PREFIX
            );
        }
    }
}

#[test]
fn a_nominal_global_run_set_demands_no_technology() {
    let mut state = AppState::default();
    disable_global_process_axis(&mut state);
    let demand = state.technology_demand();
    assert!(demand.is_empty(), "{:?}", demand.reasons());
    assert_eq!(demand.block_reason(), None);
    assert_eq!(state.technology_gate_block_reason(), Ok(()));
}

#[test]
fn the_default_global_pvt_run_set_demands_its_non_typical_sections() {
    let mut state = AppState::default();
    bind_supply_source(&mut state.sim_setup.run_set);
    for dimension in &mut state.sim_setup.run_set.dimensions {
        dimension.enabled = true;
    }
    let demand = state.technology_demand();
    assert_eq!(
        sole_reason(&demand),
        &TechnologyDemandReason::GlobalRunSetSections {
            sections: vec![ProcessCorner::SS, ProcessCorner::FF],
        }
    );
    assert_eq!(
        sole_reason(&demand).observed(),
        "Global Run Set requests SS, FF process sections"
    );
}

#[test]
fn a_non_typical_reference_process_demands_its_section() {
    let mut state = AppState::default();
    disable_global_process_axis(&mut state);
    state
        .sim_setup
        .set_reference_pvt(ProcessCorner::SS, 27.0)
        .expect("the reference point is valid");
    let demand = state.technology_demand();
    assert_eq!(
        sole_reason(&demand),
        &TechnologyDemandReason::NonTtReference(ProcessCorner::SS)
    );
    assert_eq!(sole_reason(&demand).observed(), "Reference process is SS");
    let required = sole_reason(&demand).required();
    assert!(required.contains("SS process section"), "{required}");
    let blocked = state
        .technology_gate_block_reason()
        .expect_err("no technology is attached");
    assert_eq!(
        blocked,
        "This plan requires an attached project technology: reference process is SS."
    );
}

/// The sections a corner analysis will materialize are the plan's, and the
/// plan states its demand once however many corner instances read it.
///
/// This used to be two reasons — one for the global space, one per corner
/// instance — because the instance carried a space of its own. Now that
/// there is one declaration, a second reason derived from it would demand
/// the same technology twice for the same fact.
#[test]
fn an_enabled_corner_analysis_demands_the_plans_non_typical_sections_once() {
    let mut state = AppState::default();
    enable_global_process_axis(&mut state);
    bind_supply_source(&mut state.sim_setup.run_set);
    insert_corner(&mut state, CornerDialogState::default());
    insert_corner(&mut state, CornerDialogState::default());

    let demand = state.technology_demand();

    let TechnologyDemandReason::GlobalRunSetSections { sections } = sole_reason(&demand) else {
        panic!("expected the plan's own sections: {:?}", demand.reasons());
    };
    assert_eq!(sections, &[ProcessCorner::SS, ProcessCorner::FF]);
    let blocked = state
        .technology_gate_block_reason()
        .expect_err("no technology is attached");
    assert_eq!(
        blocked,
        "This plan requires an attached project technology: global Run Set requests SS, FF \
         process sections."
    );
}

#[test]
fn a_corner_without_a_process_axis_demands_nothing_at_a_typical_reference() {
    let mut state = AppState::default();
    disable_global_process_axis(&mut state);
    insert_corner(&mut state, CornerDialogState::default());
    let demand = state.technology_demand();
    assert!(demand.is_empty(), "{:?}", demand.reasons());
    assert_eq!(state.technology_gate_block_reason(), Ok(()));
}

#[test]
fn a_corner_without_a_process_axis_does_not_restate_the_reference_section() {
    let mut state = AppState::default();
    disable_global_process_axis(&mut state);
    state
        .sim_setup
        .set_reference_pvt(ProcessCorner::SS, 27.0)
        .expect("the reference point is valid");
    insert_corner(&mut state, CornerDialogState::default());
    let demand = state.technology_demand();
    assert_eq!(
        sole_reason(&demand),
        &TechnologyDemandReason::NonTtReference(ProcessCorner::SS)
    );
}

/// Attaching a layout document needs an exact signed pin, which is what
/// this reason reports the absence of; the cells it renders are stated
/// directly instead.
#[test]
fn a_physical_layout_names_itself_and_the_signed_stack() {
    let reason = TechnologyDemandReason::PhysicalLayout {
        documents: vec!["top".to_owned()],
    };
    assert_eq!(
        reason.observed(),
        "Physical layout 'top' requires a signed technology"
    );
    let required = reason.required();
    assert!(required.contains("signed PDK"), "{required}");
    let several = TechnologyDemandReason::PhysicalLayout {
        documents: vec!["top".to_owned(), "pads".to_owned()],
    };
    assert_eq!(
        several.observed(),
        "Physical layouts 'top', 'pads' require a signed technology"
    );
}
