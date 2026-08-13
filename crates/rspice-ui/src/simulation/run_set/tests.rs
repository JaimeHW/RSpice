//! Run-set model contract.

use super::*;
use crate::simulation::dialog::corner::{CornerBaseAnalysis, ProcessCorner};
use crate::simulation::plan::AnalysisKind;

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
fn plan_aware_validation_refuses_an_empty_plan_without_inventing_a_task() {
    let state = RunSetState::default();
    let validation = validate_for_plan(&state, &[], Some(0));

    assert_eq!(validation.forecast.enabled_analysis_count, 0);
    assert_eq!(validation.forecast.task_count, 0);
    assert!(
        validation
            .errors
            .iter()
            .any(|error| error.id == "RUNSET-PLAN-EMPTY"),
        "{:?}",
        validation.errors
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
            ..RunSetComposition::default()
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
fn the_engine_batch_limit_blocks_preview_even_when_the_authored_budget_is_larger() {
    let state = RunSetState::default();
    let engine_limit = rspice_core::ResourceLimits::default().max_batch_runs;
    let validation = validate_with_task_count(&state, 1, Some(engine_limit + 1));

    assert!(validation.errors.iter().any(|error| {
        error.id == "RUNSET-ENGINE-TASK-LIMIT" && error.message.contains(&engine_limit.to_string())
    }));
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

/// The identity of the `index`-th point of the composed space.
fn key_of(state: &RunSetState, index: usize) -> String {
    compose(state)
        .expect("the space composes")
        .get(index)
        .expect("the composed space reaches this point")
        .point_key()
}

fn exclude(state: &mut RunSetState, key: &str) {
    let transaction = dispatch(
        state,
        RunSetAction::ExcludePoint {
            key: key.to_owned(),
        },
        1,
    );
    assert!(transaction.was_adopted(), "{:?}", transaction.receipt);
}

#[test]
fn excluding_one_point_shortens_the_space_by_exactly_that_point() {
    let mut state = RunSetState::default();
    let key = key_of(&state, 13);
    let label = compose(&state).expect("composes")[13].label();

    exclude(&mut state, &key);

    assert_eq!(
        state.composition.mode,
        RunSetCompositionMode::Filtered,
        "excluding a point is what makes the composition a filtered one"
    );
    let validation = validate(&state, 1);
    assert!(validation.is_ready(), "{:?}", validation.errors);
    assert_eq!(validation.forecast.point_count, 26);
    assert_eq!(validation.forecast.task_count, 26);

    let points = resolve(&state).expect("a filtered space resolves");
    assert_eq!(points.len(), 26);
    assert!(
        points.iter().all(|point| point.label() != label),
        "the excluded point must not be in the executed list"
    );
    assert_eq!(
        compose(&state).expect("composes").len(),
        27,
        "the axes are untouched: exclusion removes a point, not a value"
    );

    // Putting it back is the same transaction in reverse, and leaves no
    // filtered composition that filters nothing.
    let transaction = dispatch(&mut state, RunSetAction::IncludePoint { key }, 1);
    assert!(transaction.was_adopted(), "{:?}", transaction.receipt);
    assert_eq!(state.composition.mode, RunSetCompositionMode::Cartesian);
    assert_eq!(validate(&state, 1).forecast.point_count, 27);
}

#[test]
fn a_point_identity_survives_reordering_the_dimensions_that_built_it() {
    let mut state = RunSetState::default();
    let key = key_of(&state, 7);
    exclude(&mut state, &key);

    let id = state
        .dimensions
        .first()
        .expect("the default run set declares axes")
        .id
        .clone();
    let transaction = dispatch(
        &mut state,
        RunSetAction::MoveDimension { id, later: true },
        1,
    );
    assert!(transaction.was_adopted(), "{:?}", transaction.receipt);

    let validation = validate(&state, 1);
    assert_eq!(
        validation.forecast.point_count, 26,
        "reordering axes reorders points; it does not release exclusions"
    );
    assert!(
        validation
            .warnings
            .iter()
            .all(|warning| warning.id != "RUNSET-EXCLUSION-UNKNOWN")
    );
}

#[test]
fn an_exclusion_the_space_outgrew_is_reported_by_identity_and_never_dropped() {
    let mut state = RunSetState::default();
    // Point 26 is the last of the cartesian expansion, so it uses the last
    // value of every axis — including the temperature about to be removed.
    let key = key_of(&state, 26);
    exclude(&mut state, &key);
    assert_eq!(validate(&state, 1).forecast.point_count, 26);

    set_values(&mut state, RunSetDimensionKind::Temperature, "-40\n25");

    let validation = validate(&state, 1);
    let warning = validation
        .warnings
        .iter()
        .find(|warning| warning.id == "RUNSET-EXCLUSION-UNKNOWN")
        .expect("a stranded exclusion is reported");
    assert!(warning.message.contains(&key), "{}", warning.message);
    assert!(
        validation.errors.is_empty(),
        "a stranded exclusion must not block the space: {:?}",
        validation.errors
    );
    assert_eq!(
        validation.forecast.point_count, 18,
        "an exclusion that names nothing subtracts nothing"
    );
    assert_eq!(resolve(&state).expect("still resolvable").len(), 18);
    assert!(
        state.composition.excluded_points.contains(&key),
        "the exclusion is retained so restoring the value restores it"
    );

    // Restoring the value restores the exclusion, by the same identity.
    set_values(&mut state, RunSetDimensionKind::Temperature, "-40\n25\n125");
    assert_eq!(validate(&state, 1).forecast.point_count, 26);
}

#[test]
fn a_run_set_with_every_point_excluded_is_refused() {
    let mut state = RunSetState::default();
    set_values(&mut state, RunSetDimensionKind::ProcessSection, "TT");
    set_values(&mut state, RunSetDimensionKind::Supply, "1.0");
    set_values(&mut state, RunSetDimensionKind::Temperature, "25");
    let key = key_of(&state, 0);
    exclude(&mut state, &key);

    let validation = validate(&state, 1);
    assert_eq!(validation.forecast.point_count, 0);
    assert!(
        validation
            .errors
            .iter()
            .any(|error| error.id == "RUNSET-ALL-POINTS-EXCLUDED"),
        "{:?}",
        validation.errors
    );
    assert!(
        state
            .to_corner_config(CornerBaseAnalysis::Op, reference())
            .is_err(),
        "a space with nothing to run must never become an executable configuration"
    );
}

#[test]
fn excluding_from_a_zipped_composition_is_refused_rather_than_reinterpreted() {
    let mut state = RunSetState::default();
    let key = key_of(&state, 4);
    let transaction = dispatch(
        &mut state,
        RunSetAction::SetComposition(RunSetComposition {
            mode: RunSetCompositionMode::Zipped,
            ..RunSetComposition::default()
        }),
        1,
    );
    assert!(transaction.was_adopted());

    let transaction = dispatch(&mut state, RunSetAction::ExcludePoint { key }, 1);
    assert!(!transaction.was_adopted());
    assert_eq!(transaction.receipt.error_ids, ["RUNSET-EXCLUSION-MODE"]);
    assert_eq!(state.composition.mode, RunSetCompositionMode::Zipped);
    assert!(state.composition.excluded_points.is_empty());
    assert_eq!(state.receipts.len(), 2, "a blocked command still records");
}

#[test]
fn an_exclusion_naming_no_point_is_refused_at_the_transaction() {
    let mut state = RunSetState::default();
    let transaction = dispatch(
        &mut state,
        RunSetAction::ExcludePoint {
            key: "dimension-process-value-009".to_owned(),
        },
        1,
    );

    assert!(!transaction.was_adopted());
    assert_eq!(transaction.receipt.error_ids, ["RUNSET-EXCLUSION-UNKNOWN"]);
    assert_eq!(state.composition.mode, RunSetCompositionMode::Cartesian);
}

#[test]
fn undo_restores_an_excluded_point_and_the_composition_it_changed() {
    let mut state = RunSetState::default();
    let key = key_of(&state, 3);
    exclude(&mut state, &key);
    assert_eq!(validate(&state, 1).forecast.point_count, 26);

    let transaction = dispatch(&mut state, RunSetAction::Undo, 1);
    assert!(transaction.was_adopted(), "{:?}", transaction.receipt);
    assert_eq!(state.composition.mode, RunSetCompositionMode::Cartesian);
    assert_eq!(validate(&state, 1).forecast.point_count, 27);

    let transaction = dispatch(&mut state, RunSetAction::Redo, 1);
    assert!(transaction.was_adopted());
    assert_eq!(state.composition.mode, RunSetCompositionMode::Filtered);
    assert_eq!(validate(&state, 1).forecast.point_count, 26);
}

#[test]
fn a_filtered_space_reaches_the_executor_as_the_points_it_resolved() {
    let mut state = RunSetState::default();
    set_values(&mut state, RunSetDimensionKind::ProcessSection, "TT\nSS");
    set_values(&mut state, RunSetDimensionKind::Supply, "0.9\n1.1");
    set_values(&mut state, RunSetDimensionKind::Temperature, "-40\n125");
    let key = key_of(&state, 5);
    exclude(&mut state, &key);

    let config = state
        .to_corner_config(CornerBaseAnalysis::Op, reference())
        .expect("a filtered space is executable");
    assert_eq!(config.points.len(), 7);
    assert_eq!(config.num_corners(), 7);

    // Same count, same coordinates, same order as the run set resolved.
    let resolved = resolve(&state).expect("the filtered space resolves");
    assert_eq!(config.points.len(), resolved.len());
    for (point, spec) in resolved.iter().zip(&config.points) {
        assert_eq!(
            point.label(),
            format!(
                "{} · {} V · {} °C",
                spec.process.short_name(),
                spec.voltage,
                spec.temperature_celsius
            )
        );
    }

    // The expansion the executor and operating-point preparation share must
    // return exactly that list, not the matrix the axes would rebuild.
    let expanded = crate::services::simulation_runner::expand_corner_pvt_points(
        &crate::services::simulation_runner::CornerRunConfig {
            process_corners: vec![
                crate::services::simulation_runner::CornerProcess::TT,
                crate::services::simulation_runner::CornerProcess::SS,
            ],
            voltages: config.voltages.clone(),
            temperatures_c: config.temperatures.clone(),
            full_matrix: true,
            points: config
                .points
                .iter()
                .map(|point| crate::services::simulation_runner::CornerPoint {
                    process: match point.process {
                        ProcessCorner::TT => crate::services::simulation_runner::CornerProcess::TT,
                        _ => crate::services::simulation_runner::CornerProcess::SS,
                    },
                    voltage: point.voltage,
                    temperature_c: point.temperature_celsius,
                })
                .collect(),
            ..Default::default()
        },
    )
    .expect("an explicit list expands");
    assert_eq!(expanded.len(), 7);
    for (index, (_, voltage, temperature)) in expanded.iter().enumerate() {
        assert_eq!(*voltage, config.points[index].voltage);
        assert_eq!(*temperature, config.points[index].temperature_celsius);
    }
}

#[test]
fn a_filtered_space_narrows_its_axes_to_the_values_its_points_still_use() {
    let mut state = RunSetState::default();
    set_values(&mut state, RunSetDimensionKind::ProcessSection, "TT\nSS");
    set_values(&mut state, RunSetDimensionKind::Supply, "1.0");
    set_values(&mut state, RunSetDimensionKind::Temperature, "25\n125");

    // Remove both SS points, leaving a process axis value nothing reaches.
    for index in [2, 3] {
        let key = key_of(&state, index);
        exclude(&mut state, &key);
    }

    let config = state
        .to_corner_config(CornerBaseAnalysis::Op, reference())
        .expect("a filtered space is executable");
    assert_eq!(config.points.len(), 2);
    assert_eq!(
        config.process_corners,
        vec![ProcessCorner::TT],
        "a process no point uses must not demand a PDK section it never binds"
    );
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

#[test]
fn plan_validation_blocks_an_analysis_that_cannot_accept_active_pvt_axes() {
    let mut state = RunSetState::reference_only();
    enable(&mut state, RunSetDimensionKind::Temperature, true);

    let validation = validate_for_plan(&state, &[AnalysisKind::SParameter], Some(3));

    assert!(!validation.is_ready());
    assert!(validation.errors.iter().any(|error| {
        error.id == "RUNSET-ANALYSIS-COMPOSITION" && error.message.contains("S-parameters")
    }));
}

#[test]
fn plan_validation_accepts_monte_carlo_across_active_pvt_axes() {
    let mut state = RunSetState::reference_only();
    enable(&mut state, RunSetDimensionKind::Temperature, true);

    let validation = validate_for_plan(&state, &[AnalysisKind::MonteCarlo], Some(3));

    assert!(validation.is_ready(), "{:?}", validation.errors);
    assert_eq!(validation.forecast.task_count, 3);
}

#[test]
fn plan_preview_receipt_blocks_incompatible_analysis_composition() {
    let mut state = RunSetState::reference_only();
    enable(&mut state, RunSetDimensionKind::Supply, true);

    let transaction = dispatch_for_plan(
        &mut state,
        RunSetAction::Preview,
        &[AnalysisKind::Pss],
        Some(3),
        None,
    );

    assert!(!transaction.was_adopted());
    assert!(
        transaction
            .receipt
            .error_ids
            .contains(&"RUNSET-ANALYSIS-COMPOSITION")
    );
    assert!(state.preview.is_none());
}
