//! Run-set model contract.

use super::*;
use crate::simulation::dialog::corner::{CornerBaseAnalysis, ProcessCorner};

fn reference() -> ReferencePoint {
    ReferencePoint {
        process: ProcessCorner::TT,
        temperature_celsius: 27.0,
    }
}

fn enable(state: &mut RunSetState, kind: RunSetDimensionKind, enabled: bool) {
    let id = state
        .dimensions
        .iter()
        .find(|dimension| dimension.kind == kind)
        .expect("the default run set declares every kind")
        .id
        .clone();
    let transaction = dispatch(state, RunSetAction::SetEnabled { id, enabled }, 1);
    assert!(transaction.was_adopted(), "{:?}", transaction.receipt);
}

fn set_values(state: &mut RunSetState, kind: RunSetDimensionKind, text: &str) {
    let id = state
        .dimensions
        .iter()
        .find(|dimension| dimension.kind == kind)
        .expect("the default run set declares every kind")
        .id
        .clone();
    let transaction = dispatch(
        state,
        RunSetAction::SetValues {
            id,
            text: text.to_owned(),
        },
        1,
    );
    assert!(transaction.was_adopted(), "{:?}", transaction.receipt);
}

#[test]
fn the_default_run_set_is_the_commercial_pvt_space_and_validates() {
    let state = RunSetState::default();
    let validation = validate(&state, 1);

    assert!(validation.is_ready(), "{:?}", validation.errors);
    assert_eq!(validation.forecast.point_count, 27);
    assert!(
        state
            .enabled_dimension_of(RunSetDimensionKind::Supply)
            .is_some()
    );
}

#[test]
fn a_cartesian_space_multiplies_every_enabled_axis() {
    let mut state = RunSetState::default();
    set_values(
        &mut state,
        RunSetDimensionKind::ProcessSection,
        "TT\nSS\nFF",
    );

    let validation = validate(&state, 2);
    assert!(validation.is_ready(), "{:?}", validation.errors);
    assert_eq!(validation.forecast.point_count, 3 * 3 * 3);
    assert_eq!(validation.forecast.task_count, 27 * 2);

    let points = resolve(&state).expect("a validating cartesian space resolves");
    assert_eq!(points.len(), 27);
    assert_eq!(points[0].coordinates.len(), 3);

    // Disabling an axis removes it from the space and from every point's
    // identity, rather than pinning it to a value the run did not vary.
    enable(&mut state, RunSetDimensionKind::Supply, false);
    assert_eq!(validate(&state, 1).forecast.point_count, 9);
    assert_eq!(
        resolve(&state).expect("still resolvable")[0]
            .coordinates
            .len(),
        2
    );
}

#[test]
fn a_zipped_space_refuses_unequal_non_scalar_lengths_instead_of_cycling() {
    let mut state = RunSetState::default();
    set_values(&mut state, RunSetDimensionKind::ProcessSection, "TT\nSS");
    let transaction = dispatch(
        &mut state,
        RunSetAction::SetComposition(RunSetComposition {
            mode: RunSetCompositionMode::Zipped,
        }),
        1,
    );
    assert!(transaction.was_adopted());

    let validation = validate(&state, 1);
    assert!(
        validation
            .errors
            .iter()
            .any(|error| error.id == "RUNSET-ZIP-LENGTH"),
        "{:?}",
        validation.errors
    );
    assert!(
        resolve(&state).is_none(),
        "an unresolvable zip must produce no point list"
    );

    // Equal non-scalar lengths pair index by index, and a scalar axis is
    // shared by every point rather than being cycled.
    set_values(
        &mut state,
        RunSetDimensionKind::ProcessSection,
        "TT\nSS\nFF",
    );
    set_values(&mut state, RunSetDimensionKind::Supply, "1.0");
    let validation = validate(&state, 1);
    assert!(validation.is_ready(), "{:?}", validation.errors);
    assert_eq!(validation.forecast.point_count, 3);
    let points = resolve(&state).expect("equal lengths resolve");
    assert_eq!(points.len(), 3);
    assert_eq!(points[1].coordinates[0].1.lexical, "SS");
    assert_eq!(points[1].coordinates[1].1.lexical, "1.0");
    assert_eq!(points[1].coordinates[2].1.lexical, "25");
}

#[test]
fn an_unparseable_value_is_retained_and_blocks_the_run_set() {
    let mut state = RunSetState::default();
    set_values(
        &mut state,
        RunSetDimensionKind::Temperature,
        "-40\nwarm\n125",
    );

    let validation = validate(&state, 1);
    assert!(
        validation
            .errors
            .iter()
            .any(|error| error.id == "RUNSET-VALUE-SOURCE"),
        "{:?}",
        validation.errors
    );
    assert!(resolve(&state).is_none());

    let dimension = state
        .enabled_dimension_of(RunSetDimensionKind::Temperature)
        .expect("temperature stays declared");
    assert_eq!(dimension.values.len(), 3, "the invalid value is retained");
    assert_eq!(dimension.values[1].lexical, "warm");
    assert!(dimension.values[1].canonical.is_none());
}

#[test]
fn a_budget_refusal_names_the_budget_it_exceeded() {
    let mut state = RunSetState::default();
    let transaction = dispatch(
        &mut state,
        RunSetAction::SetBudgets(RunSetBudgets {
            maximum_tasks: 2,
            ..RunSetBudgets::default()
        }),
        1,
    );
    assert!(transaction.was_adopted());

    let validation = validate(&state, 1);
    assert!(
        validation
            .errors
            .iter()
            .any(|error| error.id == "RUNSET-TASK-BUDGET"),
        "{:?}",
        validation.errors
    );
}

#[test]
fn a_preview_freezes_a_forecast_that_the_next_edit_clears() {
    let mut state = RunSetState::default();
    let transaction = dispatch(&mut state, RunSetAction::Preview, 1);

    assert!(transaction.was_adopted());
    assert_eq!(state.preview.expect("preview frozen").point_count, 27);
    assert_eq!(
        state.revision, 1,
        "a preview evaluates the declaration and does not change it"
    );

    enable(&mut state, RunSetDimensionKind::Supply, false);
    assert!(
        state.preview.is_none(),
        "an edit must clear a forecast it has invalidated"
    );
    assert_eq!(state.revision, 2);
}

#[test]
fn a_refused_command_changes_nothing_and_still_leaves_a_receipt() {
    let mut state = RunSetState::default();
    let before = state.clone();

    let transaction = dispatch(
        &mut state,
        RunSetAction::RemoveDimension {
            id: "dimension-absent".to_owned(),
        },
        1,
    );

    assert!(!transaction.was_adopted());
    assert_eq!(transaction.receipt.error_ids, ["RUNSET-DIMENSION-MISSING"]);
    assert_eq!(state.dimensions, before.dimensions);
    assert_eq!(state.revision, before.revision);
    assert_eq!(state.receipts.len(), 1);
    assert!(transaction.receipt.digest.starts_with("fnv1a64-"));
}

#[test]
fn undo_restores_the_declaration_and_keeps_the_receipt_log() {
    let mut state = RunSetState::default();
    set_values(&mut state, RunSetDimensionKind::Temperature, "0\n50");
    assert_eq!(validate(&state, 1).forecast.point_count, 18);

    let transaction = dispatch(&mut state, RunSetAction::Undo, 1);
    assert!(transaction.was_adopted());
    assert_eq!(validate(&state, 1).forecast.point_count, 27);
    assert_eq!(
        state.receipts.len(),
        2,
        "undoing an edit does not unmake the record that it happened"
    );

    let transaction = dispatch(&mut state, RunSetAction::Redo, 1);
    assert!(transaction.was_adopted());
    assert_eq!(validate(&state, 1).forecast.point_count, 18);
}

#[test]
fn a_disabled_process_axis_is_still_declared_and_cannot_be_added_twice() {
    let mut state = RunSetState::default();
    enable(&mut state, RunSetDimensionKind::ProcessSection, false);

    assert!(
        state
            .enabled_dimension_of(RunSetDimensionKind::ProcessSection)
            .is_none()
    );
    assert!(
        state.addable_kinds().is_empty(),
        "a disabled dimension is declared; adding a second would be two owners"
    );
}

#[test]
fn a_kind_can_only_be_declared_once_because_the_engine_binds_one_per_point() {
    let mut state = RunSetState::default();
    assert!(state.addable_kinds().is_empty());

    let transaction = dispatch(
        &mut state,
        RunSetAction::AddDimension(RunSetDimensionKind::Supply),
        1,
    );
    assert!(!transaction.was_adopted());
    assert_eq!(transaction.receipt.error_ids, ["RUNSET-DIMENSION-KIND"]);

    let id = state
        .dimensions
        .iter()
        .find(|dimension| dimension.kind == RunSetDimensionKind::Supply)
        .expect("supply is declared")
        .id
        .clone();
    assert!(dispatch(&mut state, RunSetAction::RemoveDimension { id }, 1).was_adopted());
    assert_eq!(state.addable_kinds(), vec![RunSetDimensionKind::Supply]);

    let transaction = dispatch(
        &mut state,
        RunSetAction::AddDimension(RunSetDimensionKind::Supply),
        1,
    );
    assert!(transaction.was_adopted(), "{:?}", transaction.receipt);
    let added = state
        .dimensions
        .last()
        .expect("the new dimension is appended");
    assert!(
        !added.values.is_empty(),
        "a dimension with no values would be refused the moment it appeared"
    );
}

#[test]
fn every_declared_axis_reaches_the_corner_configuration_it_binds() {
    let mut state = RunSetState::default();
    set_values(&mut state, RunSetDimensionKind::ProcessSection, "TT\nSS");
    set_values(&mut state, RunSetDimensionKind::Supply, "0.9\n1.1");
    set_values(&mut state, RunSetDimensionKind::Temperature, "-40\n125");

    let config = state
        .to_corner_config(CornerBaseAnalysis::Transient, reference())
        .expect("a validating run set is executable");

    assert_eq!(
        config.process_corners,
        vec![ProcessCorner::TT, ProcessCorner::SS]
    );
    assert_eq!(config.voltages, vec![0.9, 1.1]);
    assert_eq!(config.temperatures, vec![-40.0, 125.0]);
    assert!(config.full_matrix);
    assert_eq!(config.num_corners(), 8);
}

#[test]
fn an_undeclared_axis_resolves_to_the_plan_reference_rather_than_a_constant() {
    let mut state = RunSetState::default();
    enable(&mut state, RunSetDimensionKind::ProcessSection, false);
    enable(&mut state, RunSetDimensionKind::Supply, false);
    enable(&mut state, RunSetDimensionKind::Temperature, false);

    let config = state
        .to_corner_config(
            CornerBaseAnalysis::Op,
            ReferencePoint {
                process: ProcessCorner::FF,
                temperature_celsius: -55.0,
            },
        )
        .expect("an empty space is one point, not an error");

    assert_eq!(config.process_corners, vec![ProcessCorner::FF]);
    assert_eq!(config.temperatures, vec![-55.0]);
    assert_eq!(
        config.voltages,
        vec![1.0],
        "an unswept supply must divide out to a ratio of exactly one"
    );
    assert_eq!(config.num_corners(), 1);
}

#[test]
fn a_run_set_that_does_not_validate_is_never_turned_into_a_configuration() {
    let mut state = RunSetState::default();
    set_values(&mut state, RunSetDimensionKind::Temperature, "hot");

    let error = state
        .to_corner_config(CornerBaseAnalysis::Transient, reference())
        .expect_err("an invalid declaration cannot become an executable space");
    assert!(error.contains("hot"), "{error}");
}

#[test]
fn storage_budgets_round_trip_through_the_form_they_are_shown_in() {
    for bytes in [
        0u64,
        512,
        4 * 1024,
        4 * 1024 * 1024,
        10 * 1024 * 1024 * 1024,
    ] {
        let text = format_bytes(bytes);
        assert_eq!(parse_bytes(&text), Ok(bytes), "{text}");
    }
    assert_eq!(parse_bytes("2 GB"), Ok(2_000_000_000));
    assert_eq!(parse_bytes("1_048_576"), Ok(1_048_576));
    assert!(parse_bytes("plenty").is_err());
}

#[test]
fn a_point_label_states_every_coordinate_with_its_unit() {
    let mut state = RunSetState::default();
    set_values(&mut state, RunSetDimensionKind::ProcessSection, "TT");
    set_values(&mut state, RunSetDimensionKind::Supply, "1.0");
    let points = resolve(&state).expect("the default space resolves");

    assert_eq!(points[0].label(), "TT · 1.0 V · -40 °C");
    assert_eq!(points[2].label(), "TT · 1.0 V · 125 °C");
}
