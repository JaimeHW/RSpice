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

/// The node shunt reaches the engine on the deck's own card, the parser reads
/// it back as the resistance that was set, and a plan that sets none writes no
/// key at all and serializes as it did before the field existed.
#[test]
fn a_stated_node_shunt_is_on_the_deck_and_an_unset_one_is_nowhere() {
    let options = SimulationOptions {
        rshunt: Some(1.0e9),
        ..SimulationOptions::default()
    };
    let card = options.to_spice_options();
    assert!(card.contains("+ RSHUNT=1e9"), "{card}");
    let deck = format!("node shunt\nR1 a 0 1k\n{card}\n.end\n");
    let parsed = rspice_core::Netlist::parse(&deck).expect("the options card parses");
    assert_eq!(parsed.options.rshunt, Some(1.0e9));

    let unset = SimulationOptions::default();
    assert!(!unset.to_spice_options().contains("RSHUNT"));
    let persisted = serde_json::to_value(&unset).expect("options encode");
    assert!(
        persisted.get("rshunt").is_none(),
        "an unset shunt adds no key to the project"
    );
    let restored: SimulationOptions =
        serde_json::from_value(serde_json::to_value(&options).expect("options encode"))
            .expect("options decode");
    assert_eq!(restored.rshunt, Some(1.0e9));
}

/// A preset is a numerical policy. Applying one, or reading which one is in
/// force, neither places nor removes the resistor the plan put in the circuit.
#[test]
fn a_node_shunt_is_not_part_of_any_preset() {
    let shunted = SimulationOptions {
        rshunt: Some(1.0e9),
        ..SimulationOptions::default()
    };
    assert_eq!(shunted.preset_name(), Some("Balanced"));
    assert!(
        SimulationOptions {
            rshunt: Some(0.0),
            ..SimulationOptions::default()
        }
        .validate()
        .is_err(),
        "a zero-ohm shunt would short every node to ground"
    );
}

/// The connect-rule block reaches the engine on the deck's own card, the
/// parser reads back the block's name in the case it was written, and a plan
/// that names none writes no key and serializes as it did before the field.
#[test]
fn a_named_connect_rule_block_is_on_the_deck_and_an_unnamed_one_is_nowhere() {
    let options = SimulationOptions {
        connect_rules: Some("Low_vdd".to_owned()),
        ..SimulationOptions::default()
    };
    options
        .validate()
        .expect("a simple identifier names a block");
    let card = options.to_spice_options();
    assert!(card.contains("+ CONNECTRULES=\"Low_vdd\""), "{card}");
    let deck = format!("connect rules\nR1 a 0 1k\n{card}\n.end\n");
    let parsed = rspice_core::Netlist::parse(&deck).expect("the options card parses");
    assert_eq!(parsed.options.connect_rules.as_deref(), Some("Low_vdd"));

    let unset = SimulationOptions::default();
    assert!(!unset.to_spice_options().contains("CONNECTRULES"));
    let persisted = serde_json::to_value(&unset).expect("options encode");
    assert!(
        persisted.get("connect_rules").is_none(),
        "an unnamed block adds no key to the project"
    );
    let restored: SimulationOptions =
        serde_json::from_value(serde_json::to_value(&options).expect("options encode"))
            .expect("options decode");
    assert_eq!(restored.connect_rules.as_deref(), Some("Low_vdd"));
}

/// Which converters the design elaborates is not a numerical policy: no
/// preset names a block or clears one. A name no library could declare is
/// refused before it reaches the card.
#[test]
fn a_connect_rule_block_is_not_part_of_any_preset_and_is_an_identifier() {
    let named = SimulationOptions {
        connect_rules: Some("only_d2a".to_owned()),
        ..SimulationOptions::default()
    };
    assert_eq!(named.preset_name(), Some("Balanced"));
    for refused in ["", "2fast", "low vdd", "a\"b", "rules;x"] {
        assert!(
            SimulationOptions {
                connect_rules: Some(refused.to_owned()),
                ..SimulationOptions::default()
            }
            .validate()
            .is_err(),
            "'{refused}' names no block"
        );
    }
}

#[test]
fn no_seed_emits_no_seed_line() {
    assert!(
        !SimulationOptions::default()
            .to_spice_options()
            .contains("SEED")
    );
}
