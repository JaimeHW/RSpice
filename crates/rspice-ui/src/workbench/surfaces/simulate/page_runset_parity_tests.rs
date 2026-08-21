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
use crate::simulation::run_set::{
    AnalysisRunAt, NETLIST_SUPPLY_SOURCE_PREFIX, RunSetDimensionKind,
};
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

/// Point the plan's supply axis at the fixture's supply.
///
/// The declared space sweeps a rail, and the axis names the rail it is
/// authorized to modify. The default label names no source, which is an
/// authoring fact this fixture has to settle before either derivation runs.
///
/// It is the plan's axis, not the corner instance's: the corner analysis reads
/// the one declaration rather than carrying a space of its own.
fn authorize_the_corner_supply_axis(state: &mut AppState) {
    for dimension in &mut state.sim_setup.run_set.dimensions {
        if dimension.kind == RunSetDimensionKind::Supply {
            dimension.source = format!("{NETLIST_SUPPLY_SOURCE_PREFIX}VCC");
        }
    }
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

/// Move the plan's reference onto a temperature the axis actually declares.
///
/// The default axis is `-40, 25, 125` against a 27 °C reference, so the default
/// space genuinely contains no nominal point. That is a real state with its own
/// refusal, pinned below; a test *about* nominal participation has to start
/// from a space that has one.
fn put_the_reference_on_a_declared_temperature(state: &mut AppState) {
    state
        .sim_setup
        .set_reference_pvt(crate::product::ProcessCorner::TT, 25.0)
        .expect("25 °C is a valid reference temperature");
}

/// Scope one enabled instance to the run set's nominal point.
fn scope_to_the_nominal_point(state: &mut AppState, kind: AnalysisKind) {
    let id = instance_of(state, kind);
    state
        .sim_setup
        .analysis_plan
        .as_mut()
        .expect("stable plan")
        .set_run_at(id, AnalysisRunAt::NominalPoint)
        .expect("an enabled instance takes a nominal-only participation");
}

/// Scope one enabled instance to the named positions of the declared space.
fn scope_to_points(state: &mut AppState, kind: AnalysisKind, positions: &[usize]) -> Vec<String> {
    let keys: Vec<String> = {
        let points = crate::simulation::run_set::resolve(&state.sim_setup.run_set)
            .expect("the fixture space expands exactly");
        positions
            .iter()
            .map(|index| points[*index].point_key())
            .collect()
    };
    let id = instance_of(state, kind);
    state
        .sim_setup
        .analysis_plan
        .as_mut()
        .expect("stable plan")
        .set_run_at(id, AnalysisRunAt::SelectedPoints(keys.clone()))
        .expect("an enabled instance takes a point selection");
    keys
}

/// The page forecast and the prepared task count, for the plan as it stands.
fn both_derivations(app: &mut RSpiceApp) -> (usize, usize) {
    let forecast = exact_plan_task_count(app)
        .expect("the page can forecast this workload")
        .expect("a declared Run Set has an exact count");
    let frozen = app.state.clone();
    let prepared = app
        .simulation_controller
        .prepare_run_set_for_preflight(&frozen)
        .expect("the fixture plan prepares")
        .task_count;
    (forecast, prepared)
}

#[test]
fn a_nominal_only_instance_is_priced_and_dispatched_at_one_point() {
    // The whole point of participation: the operating point runs everywhere and
    // the PSS pair runs once, so the total is neither "everything × points" nor
    // "everything × 1". Both derivations have to land on the mixed number.
    let mut app = app_with(&[AnalysisKind::OperatingPoint, AnalysisKind::Pss]);
    drive_pss_from_the_fixture_supply(&mut app.state);
    let points = enable_only_the_temperature_axis(&mut app.state);
    assert!(points > 1, "the fixture space must actually multiply");
    put_the_reference_on_a_declared_temperature(&mut app.state);
    scope_to_the_nominal_point(&mut app.state, AnalysisKind::Pss);

    let (forecast, prepared) = both_derivations(&mut app);
    assert_eq!(
        forecast, prepared,
        "the forecast and the prepared task list must price participation identically"
    );
    assert_eq!(
        prepared,
        points + 2,
        "the operating point at every point, and the shooting solve with its spectrum once"
    );
}

#[test]
fn a_point_selection_is_priced_and_dispatched_at_exactly_those_points() {
    let mut app = app_with(&[AnalysisKind::OperatingPoint, AnalysisKind::Pss]);
    drive_pss_from_the_fixture_supply(&mut app.state);
    let points = enable_only_the_temperature_axis(&mut app.state);
    assert!(points > 2, "the fixture space must have a point to omit");
    let chosen = scope_to_points(&mut app.state, AnalysisKind::Pss, &[0, 2]);
    assert_eq!(chosen.len(), 2);

    let (forecast, prepared) = both_derivations(&mut app);
    assert_eq!(forecast, prepared, "both derivations honour the selection");
    assert_eq!(
        prepared,
        points + 4,
        "the operating point everywhere, and the PSS pair at each of the two chosen points"
    );
}

#[test]
fn a_selection_that_names_no_declared_point_is_refused_rather_than_run() {
    // The orphaning case, forced directly: a key that never existed stands for
    // one the run set has since dropped. Preparation must refuse and name the
    // analysis, never quietly run it at the points that remain.
    let mut app = app_with(&[AnalysisKind::OperatingPoint]);
    enable_only_the_temperature_axis(&mut app.state);
    let id = instance_of(&app.state, AnalysisKind::OperatingPoint);
    app.state
        .sim_setup
        .analysis_plan
        .as_mut()
        .expect("stable plan")
        .set_run_at(
            id,
            AnalysisRunAt::SelectedPoints(vec!["point-that-no-axis-composes".to_owned()]),
        )
        .expect("the plan stores what the run set has not yet been asked about");

    let frozen = app.state.clone();
    let error = app
        .simulation_controller
        .prepare_run_set_for_preflight(&frozen)
        .expect_err("a scope naming no declared point cannot be dispatched");
    assert!(
        error.message().contains("no longer contains"),
        "the refusal must name the orphaned points: {error}"
    );
}

#[test]
fn a_dependent_may_not_run_where_its_prerequisite_does_not() {
    // Participation makes one composition expressible that has no honest
    // answer. Binding the spectrum to the PSS's other point would call two
    // conditions one solve, so preparation refuses and names both sides.
    let mut app = app_with(&[AnalysisKind::OperatingPoint, AnalysisKind::Pss]);
    drive_pss_from_the_fixture_supply(&mut app.state);
    let points = enable_only_the_temperature_axis(&mut app.state);
    assert!(points > 1, "the fixture space must actually multiply");
    put_the_reference_on_a_declared_temperature(&mut app.state);
    // The PSS is the operating point's dependent in this fixture, so narrowing
    // the operating point strands the PSS at every other point.
    scope_to_the_nominal_point(&mut app.state, AnalysisKind::OperatingPoint);

    let frozen = app.state.clone();
    let error = app
        .simulation_controller
        .prepare_run_set_for_preflight(&frozen)
        .expect_err("a dependent cannot outrun its prerequisite");
    assert!(
        error.message().contains("prerequisite"),
        "the refusal must name the prerequisite to widen: {error}"
    );
}

#[test]
fn a_space_with_no_reference_point_refuses_a_nominal_only_instance() {
    // The default axis is -40, 25, 125 against a 27 °C reference, so "nominal
    // point only" names a point that is not in the space. Running such an
    // instance at whichever point came first would attribute a nominal claim to
    // a corner, so it is a refusal that states the reference it looked for.
    let mut app = app_with(&[AnalysisKind::OperatingPoint]);
    let points = enable_only_the_temperature_axis(&mut app.state);
    assert!(points > 1, "the fixture space must actually multiply");
    scope_to_the_nominal_point(&mut app.state, AnalysisKind::OperatingPoint);

    let frozen = app.state.clone();
    let error = app
        .simulation_controller
        .prepare_run_set_for_preflight(&frozen)
        .expect_err("a nominal scope over a space with no reference point cannot be dispatched");
    assert!(
        error.message().contains("reference condition"),
        "the refusal must name the reference it looked for: {error}"
    );
}
