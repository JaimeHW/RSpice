//! Solver-option persistence, migration, and direct engine-policy projection.

use super::*;

#[test]
fn a_project_saved_with_the_retired_gear_only_method_still_opens() {
    let mut persisted = serde_json::to_value(SimulationOptions::default()).expect("options encode");
    persisted
        .as_object_mut()
        .expect("options are an object")
        .insert("method".to_owned(), serde_json::json!("Gear2Only"));

    let restored: SimulationOptions =
        serde_json::from_value(persisted).expect("a project naming the retired method decodes");

    assert_eq!(restored.method, IntegrationMethod::Gear2);
}

#[test]
fn a_project_saved_with_the_retired_plain_gear_method_still_opens() {
    // `Gear` and `Gear2` both resolved to the engine's one second-order
    // Gear integrator, so a project that chose either was always running
    // the survivor.
    let mut persisted = serde_json::to_value(SimulationOptions::default()).expect("options encode");
    persisted
        .as_object_mut()
        .expect("options are an object")
        .insert("method".to_owned(), serde_json::json!("Gear"));

    let restored: SimulationOptions =
        serde_json::from_value(persisted).expect("a project naming the retired method decodes");

    assert_eq!(restored.method, IntegrationMethod::Gear2);
}

#[test]
fn a_project_saved_with_the_retired_controls_still_opens() {
    // The shape refuses unknown fields so a typo in a project file is
    // caught rather than dropped, which means every retired key has to be
    // named explicitly for older projects to keep opening.
    let mut persisted = serde_json::to_value(SimulationOptions::default()).expect("options encode");
    let object = persisted.as_object_mut().expect("options are an object");
    object.insert("itl2".to_owned(), serde_json::json!(50));
    object.insert("timestep_factor".to_owned(), serde_json::json!(16.0));
    object.insert("verbose".to_owned(), serde_json::json!(true));
    object.insert("save_internals".to_owned(), serde_json::json!(true));

    let restored: SimulationOptions = serde_json::from_value(persisted)
        .expect("a project written before these controls were retired decodes");

    assert_eq!(
        serde_json::to_value(&restored).expect("restored options re-encode"),
        serde_json::to_value(SimulationOptions::default()).expect("options encode"),
        "a retired key must decode away, not resurface on the next save"
    );
}

#[test]
fn matrix_selection_and_pivrel_reach_core_configuration() {
    for (solver, expected) in [
        (MatrixSolver::Lu, None),
        (
            MatrixSolver::SparseLu,
            Some(rspice_core::solver::RealSolverBackend::Faer),
        ),
        (
            MatrixSolver::Klu,
            Some(rspice_core::solver::RealSolverBackend::Klu),
        ),
        (MatrixSolver::Gmres, None),
    ] {
        let options = SimulationOptions {
            solver,
            pivrel: 0.125,
            pivtol: 2.5e-14,
            ..SimulationOptions::default()
        };
        let config = options.resolve_simulation_config(None);
        assert_eq!(config.matrix_solver, expected);
        assert_eq!(config.matrix_pivot_tolerance, 0.125);
        assert_eq!(config.matrix_absolute_pivot_tolerance, 2.5e-14);
    }
}

#[test]
fn truncation_error_controls_reach_the_core_configuration() {
    let options = SimulationOptions {
        trtol: 1.5,
        transient_lte_reltol: Some(2.5e-4),
        transient_lte_abstol: Some(3.5e-9),
        ..SimulationOptions::default()
    };

    let overrides = options.simulation_config_overrides();

    assert_eq!(overrides.transient_trtol, Some(1.5));
    assert_eq!(overrides.transient_lte_reltol, Some(2.5e-4));
    assert_eq!(overrides.transient_lte_abstol, Some(3.5e-9));
}

#[test]
fn an_unset_truncation_bound_leaves_the_engines_own_in_force() {
    let overrides = SimulationOptions::default().simulation_config_overrides();

    assert_eq!(
        overrides.transient_trtol,
        Some(7.0),
        "TRTOL always has a value, and it is the SPICE default"
    );
    assert_eq!(overrides.transient_lte_reltol, None);
    assert_eq!(overrides.transient_lte_abstol, None);
}

#[test]
fn projects_written_before_trtol_existed_open_on_the_spice_default() {
    let mut persisted = serde_json::to_value(SimulationOptions::default()).expect("options encode");
    let object = persisted.as_object_mut().expect("options are an object");
    object.remove("trtol");
    object.remove("transient_lte_reltol");
    object.remove("transient_lte_abstol");

    let restored: SimulationOptions =
        serde_json::from_value(persisted).expect("legacy options decode");

    assert_eq!(restored.trtol, 7.0);
    assert_eq!(restored.transient_lte_reltol, None);
}

#[test]
fn a_stated_statistical_seed_reaches_the_deck() {
    let options = SimulationOptions {
        statistical_seed: Some(20260811),
        ..SimulationOptions::default()
    };

    assert!(
        options.to_spice_options().contains("SEED=20260811"),
        "the parser seeds the statistical stream from the deck, so the seed must be in it"
    );
}

#[test]
fn no_seed_emits_no_seed_line() {
    assert!(
        !SimulationOptions::default()
            .to_spice_options()
            .contains("SEED")
    );
}
