//! Solver-option persistence, migration, and direct engine-policy projection.

use super::*;

#[test]
fn a_project_saved_with_a_retired_gear_method_still_opens() {
    // `Gear2Only`, `Gear` and `Gear2` all resolved to the engine's one
    // second-order Gear integrator, so a project that chose any of them was
    // always running the survivor.
    for retired in ["Gear2Only", "Gear"] {
        let mut persisted =
            serde_json::to_value(SimulationOptions::default()).expect("options encode");
        persisted
            .as_object_mut()
            .expect("options are an object")
            .insert("method".to_owned(), serde_json::json!(retired));

        let restored: SimulationOptions =
            serde_json::from_value(persisted).expect("a project naming the retired method decodes");

        assert_eq!(restored.method, IntegrationMethod::Gear2, "{retired}");
    }
}

/// The options record names only what the engine reads. A key it does not
/// name, a retired control among them, is refused rather than dropped, and the
/// retired matrix value names no solver.
#[test]
fn a_retired_control_or_matrix_value_is_refused() {
    for (key, value) in [
        ("itl2", serde_json::json!(50)),
        ("timestep_factor", serde_json::json!(16.0)),
        ("verbose", serde_json::json!(true)),
        ("save_internals", serde_json::json!(true)),
        ("solver", serde_json::json!("Gmres")),
    ] {
        let mut persisted =
            serde_json::to_value(SimulationOptions::default()).expect("options encode");
        persisted
            .as_object_mut()
            .expect("options are an object")
            .insert(key.to_owned(), value);
        assert!(
            serde_json::from_value::<SimulationOptions>(persisted).is_err(),
            "{key} decoded"
        );
    }
}

#[test]
fn an_unset_truncation_bound_leaves_the_engines_own_in_force() {
    let card = SimulationOptions::default().to_spice_options(27.0);
    for key in ["TRTOL", "RELTOL", "ABSTOL"] {
        assert!(
            !card.contains(key),
            "the default plan states {key}:\n{card}"
        );
    }
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
        options.to_spice_options(27.0).contains("SEED=20260811"),
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
    let card = options.to_spice_options(27.0);
    assert!(card.contains("+ RSHUNT=1e9"), "{card}");
    let deck = format!("node shunt\nR1 a 0 1k\n{card}\n.end\n");
    let parsed = rspice_core::Netlist::parse(&deck).expect("the options card parses");
    assert_eq!(parsed.options.rshunt, Some(1.0e9));

    let unset = SimulationOptions::default();
    assert!(!unset.to_spice_options(27.0).contains("RSHUNT"));
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
    let card = options.to_spice_options(27.0);
    assert!(card.contains("+ CONNECTRULES=\"Low_vdd\""), "{card}");
    let deck = format!("connect rules\nR1 a 0 1k\n{card}\n.end\n");
    let parsed = rspice_core::Netlist::parse(&deck).expect("the options card parses");
    assert_eq!(parsed.options.connect_rules.as_deref(), Some("Low_vdd"));

    let unset = SimulationOptions::default();
    assert!(!unset.to_spice_options(27.0).contains("CONNECTRULES"));
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
fn a_new_project_integrates_trapezoidally_and_an_explicit_switch_is_written() {
    // New projects run the trapezoidal rule, the ngspice, Xyce, HSPICE and
    // Spectre default, and leave METHOD off the deck; the trap/Gear-2 switch
    // is a choice the deck then states.
    let options = SimulationOptions::default();
    assert_eq!(options.method, IntegrationMethod::Trap);
    assert!(!options.to_spice_options(27.0).contains("METHOD"));

    let switched = SimulationOptions {
        method: IntegrationMethod::TrapGear,
        ..SimulationOptions::default()
    };
    assert!(switched.to_spice_options(27.0).contains("METHOD=TRAPGEAR"));
}

#[test]
fn no_seed_emits_no_seed_line() {
    assert!(
        !SimulationOptions::default()
            .to_spice_options(27.0)
            .contains("SEED")
    );
}
