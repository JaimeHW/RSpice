//! The Run Set page's task count is the count the run will actually queue.
//!
//! Two derivations exist. The page computes a workload forecast so it can warn
//! before a run starts; preparation expands the real task list. They are
//! written separately and read the same authored state, so they can disagree —
//! and a forecast that disagrees is worse than none, because it is the number
//! the operator budgets against. The snapshot is the authority: if these ever
//! diverge, the page is what changes.

use crate::services::drc::DrcResult;
use crate::simulation::plan::{AnalysisDraft, AnalysisInstance, AnalysisKind};
use crate::simulation::run_set::{NETLIST_SUPPLY_SOURCE_PREFIX, RunSetDimensionKind};
use crate::workbench::RSpiceApp;
use crate::workbench::app_state::AppState;

use super::page_runset::exact_plan_task_count;

/// An app whose design is checked, whose technology is attached, and whose
/// plan holds exactly the requested analyses.
fn app_with(kinds: &[AnalysisKind]) -> RSpiceApp {
    let mut app = RSpiceApp::test_instance();
    crate::workbench::examples::load_example("Voltage Divider", &mut app.state.schematic);
    let mut drc = DrcResult::new();
    drc.completed = true;
    app.state.dialogs.drc_results = Some(drc);
    app.state.dialogs.drc_checked_version = app.state.schematic.topology_version();
    app.state.provision_test_project_technology_contract();
    app.state.sync_active_schematic_to_workspace();

    let plan = app
        .state
        .sim_setup
        .analysis_plan
        .as_mut()
        .expect("a test instance owns a stable analysis plan");
    for existing in plan
        .instances()
        .iter()
        .map(AnalysisInstance::id)
        .collect::<Vec<_>>()
    {
        plan.set_enabled(existing, false)
            .expect("the default analysis has no enabled dependent");
    }
    for kind in kinds.iter().copied() {
        let (id, _) = plan
            .insert(kind)
            .unwrap_or_else(|error| panic!("{kind:?} inserts: {error}"));
        for prerequisite in kind.prerequisites() {
            let target = plan
                .instances()
                .iter()
                .find(|instance| instance.enabled() && instance.kind() == *prerequisite)
                .map(AnalysisInstance::id)
                .unwrap_or_else(|| panic!("{kind:?} needs a {prerequisite:?} to bind to"));
            plan.bind_dependency(id, *prerequisite, target)
                .unwrap_or_else(|error| panic!("{kind:?} binds {prerequisite:?}: {error}"));
        }
    }
    app
}

/// The enabled instance of one kind.
fn instance_of(state: &AppState, kind: AnalysisKind) -> crate::product::AnalysisInstanceId {
    state
        .sim_setup
        .enabled_analysis_instances()
        .find(|instance| instance.kind() == kind)
        .map(AnalysisInstance::id)
        .unwrap_or_else(|| panic!("the fixture plan holds a {kind:?} instance"))
}

/// Drive the PSS from the fixture's own supply, which the shooting solve needs
/// to be periodic. The retained harmonic count is left at its default, which
/// is what earns the run its companion spectrum task.
fn drive_pss_from_the_fixture_supply(state: &mut AppState) {
    let supply = state
        .schematic
        .components
        .iter_mut()
        .find(|component| component.name == "VCC")
        .expect("the fixture design owns a supply named VCC");
    supply.value = "SIN(0 1 1k)".to_owned();
    state.sync_active_schematic_to_workspace();

    let pss = instance_of(state, AnalysisKind::Pss);
    state
        .sim_setup
        .analysis_plan
        .as_mut()
        .expect("stable plan")
        .edit(pss, |draft| {
            let AnalysisDraft::Pss(draft) = draft else {
                panic!("a PSS instance owns a PSS draft");
            };
            draft.tone_sources = "VCC".to_owned();
        })
        .expect("the PSS tone source edits");
}

/// Point the corner analysis's own supply axis at the fixture's supply.
///
/// The default corner space sweeps a rail, and the axis names the rail it is
/// authorized to modify. The default label names no source, which is an
/// authoring fact this fixture has to settle before either derivation runs.
fn authorize_the_corner_supply_axis(state: &mut AppState) {
    let corner = instance_of(state, AnalysisKind::Corner);
    state
        .sim_setup
        .analysis_plan
        .as_mut()
        .expect("stable plan")
        .edit(corner, |draft| {
            let AnalysisDraft::Corner(draft) = draft else {
                panic!("a corner instance owns a corner draft");
            };
            for dimension in &mut draft.run_set.dimensions {
                if dimension.kind == RunSetDimensionKind::Supply {
                    dimension.source = format!("{NETLIST_SUPPLY_SOURCE_PREFIX}VCC");
                }
            }
        })
        .expect("the corner supply authority edits");
}

/// Enable exactly one global axis, so the run multiplies over a declared space
/// the page and the snapshot must count identically.
fn enable_only_the_temperature_axis(state: &mut AppState) -> usize {
    for dimension in &mut state.sim_setup.run_set.dimensions {
        dimension.enabled = dimension.kind == RunSetDimensionKind::Temperature;
    }
    state
        .sim_setup
        .run_set
        .enabled_dimension_of(RunSetDimensionKind::Temperature)
        .expect("the temperature axis is enabled")
        .values
        .len()
}

#[test]
fn the_page_forecast_equals_the_prepared_task_count_over_a_declared_space() {
    // Temperature and Corner are deliberately absent: an analysis that owns an
    // internal point declaration may not also execute across global axes, so a
    // plan holding one has no prepared task list to compare against. That
    // refusal is pinned separately below.
    let mut app = app_with(&[AnalysisKind::OperatingPoint, AnalysisKind::Pss]);
    drive_pss_from_the_fixture_supply(&mut app.state);
    let points = enable_only_the_temperature_axis(&mut app.state);
    assert!(points > 1, "the fixture space must actually multiply");

    let forecast = exact_plan_task_count(&app)
        .expect("the page can forecast this workload")
        .expect("a reference-only Run Set has an exact count");
    let frozen = app.state.clone();
    let prepared = app
        .simulation_controller
        .prepare_run_set_for_preflight(&frozen)
        .expect("the fixture plan prepares")
        .task_count;

    assert_eq!(
        forecast, prepared,
        "the Run Set page forecast and the prepared task list must be the same number"
    );
    assert_eq!(
        prepared,
        3 * points,
        "the operating point, the shooting solve and its spectrum, once per point"
    );
}

#[test]
fn the_page_forecast_equals_the_prepared_task_count_with_no_global_axes() {
    // Without axes, Temperature and Corner keep their own point declarations
    // and the page counts them per analysis instead of multiplying. That is
    // the other half of the derivation, and it has to agree too.
    let mut app = app_with(&[
        AnalysisKind::OperatingPoint,
        AnalysisKind::Temperature,
        AnalysisKind::Corner,
        AnalysisKind::Pss,
    ]);
    drive_pss_from_the_fixture_supply(&mut app.state);
    authorize_the_corner_supply_axis(&mut app.state);
    for dimension in &mut app.state.sim_setup.run_set.dimensions {
        dimension.enabled = false;
    }

    let forecast = exact_plan_task_count(&app)
        .expect("the page can forecast this workload")
        .expect("a reference-only Run Set has an exact count");
    let frozen = app.state.clone();
    let prepared = app
        .simulation_controller
        .prepare_run_set_for_preflight(&frozen)
        .expect("the fixture plan prepares")
        .task_count;

    assert_eq!(
        forecast, prepared,
        "the analysis-owned point declarations must count the same on both sides"
    );
}

#[test]
fn a_nested_point_declaration_under_global_axes_is_refused_by_both_derivations() {
    // The page still forecasts a number for this plan — a budget figure is
    // useful even for a plan that will not run — but it must not be a number
    // the operator can act on without also being told the composition is
    // prohibited, and preparation must refuse rather than pick one authority.
    let mut app = app_with(&[
        AnalysisKind::OperatingPoint,
        AnalysisKind::Temperature,
        AnalysisKind::Corner,
        AnalysisKind::Pss,
    ]);
    drive_pss_from_the_fixture_supply(&mut app.state);
    authorize_the_corner_supply_axis(&mut app.state);
    enable_only_the_temperature_axis(&mut app.state);

    let kinds = app
        .state
        .sim_setup
        .enabled_analysis_instances()
        .map(AnalysisInstance::kind)
        .collect::<Vec<_>>();
    let validation = crate::simulation::run_set::validate_for_plan(
        &app.state.sim_setup.run_set,
        &kinds,
        exact_plan_task_count(&app).expect("the page can still forecast"),
    );
    assert!(
        validation
            .errors
            .iter()
            .any(|issue| issue.id == "RUNSET-ANALYSIS-COMPOSITION"),
        "the page must name the prohibited composition"
    );

    let frozen = app.state.clone();
    let error = app
        .simulation_controller
        .prepare_run_set_for_preflight(&frozen)
        .expect_err("two point authorities cannot be crossed implicitly");
    assert!(
        error.message().contains("global multi-point Run Set"),
        "{error}"
    );
}
