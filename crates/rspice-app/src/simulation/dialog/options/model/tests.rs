//! Option persistence and execution round-trip tests.
//!
//! Each assertion follows the authored card through the parser into core.

use super::*;

/// Resolve `options` exactly the way a run does.
///
/// The deck is the only channel these options have: the block is spliced
/// into the prepared source, the engine re-parses that source, and every
/// shipping runner resolves the parsed `.OPTIONS` against the core
/// defaults with no override layer. Asserting on the emitted string
/// instead would pass for a key the parser does not know, which is how
/// fields drifted out of the engine in the first place.
fn parse_through_the_deck(options: &SimulationOptions) -> rspice_core::Netlist {
    let deck = crate::simulation::SimulationController::apply_simulation_options_to_netlist(
        "round trip\nV1 1 0 1\nR1 1 0 1k\n.op\n.end\n",
        options,
    );
    rspice_core::netlist::parse_netlist(&deck)
        .unwrap_or_else(|error| panic!("the emitted deck must parse: {error}\n{deck}"))
}

fn resolve_through_the_deck(options: &SimulationOptions) -> rspice_core::engine::SimulationConfig {
    let netlist = parse_through_the_deck(options);
    rspice_core::resolve_simulation_config(
        &rspice_core::engine::SimulationConfig::default(),
        Some(&netlist.options),
        &rspice_core::SimulationConfigOverrides::default(),
    )
}

#[test]
fn simulation_compatibility_survives_draft_persistence_and_execution_routes() {
    use crate::simulation::dialog::OptionsDialogState;
    for compatibility in SimulationCompatibility::ALL {
        let options = SimulationOptions {
            compatibility,
            ..Default::default()
        };
        let draft = OptionsDialogState::from_options(&options);
        let restored: SimulationOptions =
            serde_json::from_str(&serde_json::to_string(&draft.to_options().unwrap()).unwrap())
                .unwrap();
        assert_eq!(restored.compatibility, compatibility);
        // Inherited source policy must survive; each explicit choice must
        // override it through both the prepared deck and direct API path.
        let source = "compatibility\nV1 in 0 1\nR1 in 0 1k\n.options RSPICE_DIALECT=XYCE\n.end\n";
        let authored = rspice_core::Netlist::parse(source).unwrap();
        let deck = crate::simulation::SimulationController::apply_simulation_options_to_netlist(
            source, &restored,
        );
        let parsed = rspice_core::Netlist::parse(&deck).unwrap();
        let engine = crate::services::simulation_runner::build_engine_config(&parsed, None);
        let expected = compatibility
            .core_override()
            .unwrap_or(rspice_core::SpiceDialect::Xyce);
        assert_eq!(engine.spice_dialect, expected);
        assert_eq!(
            restored
                .resolve_simulation_config(Some(&authored.options))
                .spice_dialect,
            expected
        );
        assert_eq!(
            restored.preset_name(),
            SimulationOptions::default().preset_name()
        );
    }
    let mut legacy = serde_json::to_value(SimulationOptions::default()).unwrap();
    assert!(
        legacy.get("compatibility").is_none(),
        "inheriting must preserve the legacy serialized options"
    );
    let restored: SimulationOptions = serde_json::from_value(legacy.clone()).unwrap();
    assert_eq!(restored.compatibility, SimulationCompatibility::Inherit);
    assert!(!restored.to_spice_options().contains("RSPICE_DIALECT"));
    legacy["compatibility"] = serde_json::json!("unknown-policy");
    assert!(serde_json::from_value::<SimulationOptions>(legacy).is_err());
}

#[test]
fn the_newly_connected_controls_survive_the_deck_round_trip() {
    let options = SimulationOptions {
        iabstol: 7.0e-13,
        chgtol: 3.0e-16,
        transient_lte_reltol: Some(2.5e-4),
        transient_lte_abstol: Some(3.5e-9),
        gmin_stepping: false,
        source_stepping: false,
        pseudo_transient: false,
        arc_length: true,
        damping: DampingStrategy::Combined,
        solver: MatrixSolver::Klu,
        min_timestep: 2.0e-18,
        ..SimulationOptions::default()
    };

    let resolved = resolve_through_the_deck(&options);

    assert_eq!(resolved.convergence_config.current_abstol, 7.0e-13);
    assert_eq!(resolved.convergence_config.charge_abstol, 3.0e-16);
    assert_eq!(resolved.transient_lte_reltol, Some(2.5e-4));
    assert_eq!(resolved.transient_lte_abstol, Some(3.5e-9));
    assert!(!resolved.convergence_config.gmin_stepping);
    assert!(!resolved.convergence_config.source_stepping);
    assert!(!resolved.convergence_config.pseudo_transient);
    assert!(resolved.convergence_config.arc_length);
    assert_eq!(
        resolved.convergence_config.damping_strategy,
        rspice_core::engine::DampingStrategy::Combined
    );
    assert_eq!(
        resolved.matrix_solver,
        Some(rspice_core::solver::RealSolverBackend::Klu)
    );
    assert_eq!(resolved.min_timestep, 2.0e-18);
}

#[test]
fn the_timeint_card_leaves_the_global_tolerances_alone() {
    // The two LTE bounds are spelled RELTOL and ABSTOL inside the TIMEINT
    // package, which are also global key names. They travel on their own
    // card so the package selector cannot re-scope the keys after them,
    // and they must not land on the global tolerances either.
    let options = SimulationOptions {
        reltol: 4.0e-4,
        abstol: 6.0e-13,
        transient_lte_reltol: Some(2.5e-7),
        transient_lte_abstol: Some(3.5e-11),
        temp: 85.0,
        ..SimulationOptions::default()
    };

    let resolved = resolve_through_the_deck(&options);

    assert_eq!(resolved.transient_lte_reltol, Some(2.5e-7));
    assert_eq!(resolved.transient_lte_abstol, Some(3.5e-11));
    assert_eq!(resolved.convergence_config.voltage_reltol, 4.0e-4);
    assert_eq!(resolved.convergence_config.current_abstol, 6.0e-13);
    // TEMP is emitted after the global tolerances and before the TIMEINT
    // card; a scoped key emitted inline would have swallowed it.
    assert_eq!(resolved.temperature, 85.0 + 273.15);
}

#[test]
fn the_bypass_controls_reach_the_resolved_bypass_config() {
    let options = SimulationOptions {
        bypass_enabled: true,
        bypass_reltol: 5.0e-4,
        bypass_abstol: 2.0e-7,
        ..SimulationOptions::default()
    };

    let resolved = resolve_through_the_deck(&options);

    assert!(resolved.bypass_config.enabled);
    assert_eq!(resolved.bypass_config.reltol, 5.0e-4);
    assert_eq!(resolved.bypass_config.abstol, 2.0e-7);
}

/// The Solver page's bypass voltage floor is the only editor of
/// `bypass_abstol`, so the field is pinned separately from the relative
/// bound: a regression that dropped `BYPASSABSTOL` from the emitter would
/// otherwise still pass while the floor silently reverted to the core
/// default the two share nothing with.
#[test]
fn the_bypass_voltage_floor_reaches_the_engine_on_its_own() {
    let default_floor = resolve_through_the_deck(&SimulationOptions {
        bypass_enabled: true,
        ..SimulationOptions::default()
    })
    .bypass_config
    .abstol;
    let options = SimulationOptions {
        bypass_enabled: true,
        bypass_abstol: 4.0e-9,
        ..SimulationOptions::default()
    };

    let resolved = resolve_through_the_deck(&options);

    assert_ne!(
        4.0e-9, default_floor,
        "the fixture value has to differ from the untouched one to prove anything"
    );
    assert_eq!(resolved.bypass_config.abstol, 4.0e-9);
    assert_eq!(
        resolved.bypass_config.reltol,
        SimulationOptions::default().bypass_reltol,
        "editing the floor must not disturb the relative bound beside it"
    );
}

/// TNOM is the model reference temperature, and it does not travel on
/// `SimulationConfig` the way TEMP does: the builder reads
/// `netlist.options.tnom` when it resolves each model card. So the round
/// trip is asserted where the value actually lands, and against TEMP, to
/// pin that the two temperatures stay separate keys.
#[test]
fn the_model_reference_temperature_reaches_the_parsed_deck_separately_from_temp() {
    let options = SimulationOptions {
        temp: 85.0,
        tnom: 40.0,
        ..SimulationOptions::default()
    };

    let netlist = parse_through_the_deck(&options);

    assert_eq!(netlist.options.tnom, Some(40.0));
    assert_eq!(
        resolve_through_the_deck(&options).temperature,
        85.0 + 273.15,
        "TNOM must not be read as the simulation temperature"
    );
}

#[test]
fn an_untouched_model_reference_temperature_states_nothing() {
    assert!(
        !SimulationOptions::default()
            .to_spice_options()
            .contains("TNOM"),
        "the shipping TNOM matches the engine's own, so the deck states no opinion"
    );
    assert_eq!(
        parse_through_the_deck(&SimulationOptions::default())
            .options
            .tnom,
        None
    );
}

#[test]
fn bypass_stays_off_and_states_no_bounds_when_the_page_leaves_it_alone() {
    let resolved = resolve_through_the_deck(&SimulationOptions::default());
    assert!(!resolved.bypass_config.enabled);
    assert!(
        !SimulationOptions::default()
            .to_spice_options()
            .contains("BYPASS")
    );
}

#[test]
fn shipping_solver_policy_reaches_the_engine_even_where_core_defaults_differ() {
    let options = SimulationOptions::default();
    let resolved = resolve_through_the_deck(&options);

    assert_eq!(resolved.transient_max_iterations, options.itl4);
    assert_eq!(
        resolved.matrix_absolute_pivot_tolerance.to_bits(),
        options.pivtol.to_bits()
    );
    assert_eq!(
        resolved.min_timestep.to_bits(),
        options.min_timestep.to_bits()
    );
    assert_eq!(
        resolved.max_timestep.to_bits(),
        options.max_timestep.to_bits()
    );
}

#[test]
fn the_bypass_bounds_do_not_leak_into_the_timeint_package() {
    // BYPASS rides the global card, and the LTE bounds ride TIMEINT. If the
    // bypass keys ever moved behind a package selector, one of these two
    // groups would swallow the other.
    let options = SimulationOptions {
        bypass_enabled: true,
        bypass_reltol: 5.0e-4,
        transient_lte_reltol: Some(2.5e-7),
        transient_lte_abstol: Some(3.5e-11),
        ..SimulationOptions::default()
    };

    let resolved = resolve_through_the_deck(&options);

    assert_eq!(resolved.bypass_config.reltol, 5.0e-4);
    assert_eq!(resolved.transient_lte_reltol, Some(2.5e-7));
    assert_eq!(resolved.transient_lte_abstol, Some(3.5e-11));
    assert_eq!(
        resolved.convergence_config.voltage_reltol,
        rspice_core::engine::SimulationConfig::default()
            .convergence_config
            .voltage_reltol
    );
}

#[test]
fn every_offered_solver_reaches_the_backend_it_names() {
    for solver in MatrixSolver::all() {
        let options = SimulationOptions {
            solver: *solver,
            ..SimulationOptions::default()
        };
        assert_eq!(
            resolve_through_the_deck(&options).matrix_solver,
            solver.core_backend_override(),
            "{} must resolve to the backend the page names",
            solver.display_name()
        );
    }
}

#[test]
fn a_step_floor_alone_still_reaches_the_deck() {
    // The shipping policy always states values that differ from core
    // defaults. The edited floor must remain on its own TIMEINT card and
    // must not disturb those authoritative global values.
    let options = SimulationOptions {
        min_timestep: 2.0e-18,
        ..SimulationOptions::default()
    };

    assert_eq!(
        options.to_spice_options(),
        ".OPTIONS\n+ PIVTOL=1e-13\n+ ITL4=6\n+ MAXTIMESTEP=1e-3\n.OPTIONS TIMEINT\n+ MINTIMESTEP=2e-18"
    );
    assert_eq!(resolve_through_the_deck(&options).min_timestep, 2.0e-18);
}

#[test]
fn every_offered_integration_method_names_one_the_engine_knows() {
    for method in IntegrationMethod::all() {
        let options = SimulationOptions {
            method: *method,
            ..SimulationOptions::default()
        };
        assert_eq!(
            resolve_through_the_deck(&options).integration_method,
            options.core_integration_method(),
            "{} must resolve to the method the page names",
            method.display_name()
        );
    }
}

#[test]
fn the_run_step_ceiling_reaches_the_engine_through_the_deck() {
    let options = SimulationOptions {
        max_timestep: 4.0e-9,
        ..SimulationOptions::default()
    };

    assert_eq!(
        resolve_through_the_deck(&options).max_timestep,
        4.0e-9,
        "the transient clamps its step against this field, so the deck must carry it"
    );
}

#[test]
fn the_plan_ceiling_and_an_analysis_step_ceiling_bound_the_step_separately() {
    // A run splices the plan's options block and then the analysis's own
    // override block into the same deck. The two ceilings are different
    // engine fields and the transient applies both, so each must arrive
    // whole: a key that overwrote the other would silently drop a bound
    // the page and the ledger both still report.
    let options = SimulationOptions {
        max_timestep: 4.0e-9,
        ..SimulationOptions::default()
    };
    let mut record = crate::simulation::plan::AnalysisNumericOverride::default();
    record
        .set_for_instance(
            crate::simulation::plan::AnalysisKind::Soa,
            crate::simulation::plan::SolverOwnership::NONE,
            crate::simulation::plan::NumericOverrideOption::MaximumTimestep,
            "700p",
        )
        .expect("SOA runs a stress transient");

    let deck = format!(
        "two ceilings\nV1 1 0 1\nR1 1 0 1k\n{}\n{}\n.op\n.end\n",
        options.to_spice_options(),
        record.to_spice_options()
    );
    let netlist = rspice_core::netlist::parse_netlist(&deck)
        .unwrap_or_else(|error| panic!("the spliced deck must parse: {error}\n{deck}"));
    let resolved = rspice_core::resolve_simulation_config(
        &rspice_core::engine::SimulationConfig::default(),
        Some(&netlist.options),
        &rspice_core::SimulationConfigOverrides::default(),
    );

    assert_eq!(resolved.max_timestep, 4.0e-9);
    assert_eq!(resolved.transient_timeint_max_timestep, Some(7.0e-10));
}

#[test]
fn a_plan_ceiling_alone_leaves_the_integrators_ceiling_unstated() {
    let options = SimulationOptions {
        max_timestep: 4.0e-9,
        ..SimulationOptions::default()
    };

    assert_eq!(
        resolve_through_the_deck(&options).transient_timeint_max_timestep,
        None
    );
}

/// Every advanced option an analysis can author wins over the plan block
/// that states the same key.
///
/// The two blocks meet only in a prepared deck, so this is the one place
/// the pairing can be checked against the real plan emitter rather than a
/// hand-written card. Both are emitted here exactly as a run emits them,
/// and the analysis's value has to be the one that survives.
///
/// The current floor is why this test exists. `ABSTOL` and `IABSTOL` are
/// two spellings of one engine field, and the resolver reads
/// `opts.iabstol.or(opts.abstol)` — *field* precedence, not card order. An
/// analysis stating `ABSTOL` was therefore overruled by any plan stating
/// `IABSTOL`, however late its card came, and the departure was accepted,
/// persisted and reported before the solve ignored it.
#[test]
fn an_authored_option_wins_over_the_plan_block_that_states_the_same_key() {
    use crate::simulation::plan::{
        AnalysisKind, AnalysisNumericOverride, NumericOverrideOption, OverrideValueKind,
    };

    // A plan that has moved every key it can, so no option is compared
    // against a preset that happens to equal the authored value.
    let options = SimulationOptions {
        reltol: 1.0e-4,
        residual_reltol: 2.0e-4,
        abstol: 3.0e-13,
        iabstol: 4.0e-13,
        vntol: 5.0e-7,
        chgtol: 6.0e-15,
        pivrel: 7.0e-4,
        pivtol: 8.0e-14,
        gmin: 9.0e-13,
        itl1: 111,
        itl4: 9,
        trtol: 3.5,
        min_timestep: 2.0e-17,
        max_timestep: 4.0e-9,
        transient_lte_reltol: Some(1.0e-6),
        transient_lte_abstol: Some(2.0e-11),
        bypass_enabled: true,
        bypass_reltol: 3.0e-4,
        bypass_abstol: 4.0e-7,
        ..SimulationOptions::default()
    };

    // SOA runs its own stress transient and uses the shared numeric controls.
    let kind = AnalysisKind::Soa;
    let mut record = AnalysisNumericOverride::default();
    for option in NumericOverrideOption::applicable_to_instance(
        kind,
        crate::simulation::plan::SolverOwnership::NONE,
    )
    .into_iter()
    // The two output-schedule keys are mutually exclusive in the engine's
    // own parser, so no record can state both and this one states the
    // list. That is the record's rule and not a gap in it —
    // `an_output_schedule_is_a_strobe_or_a_stop_list_and_not_both` is
    // where it is proven.
    .filter(|option| *option != NumericOverrideOption::StrobeInterval)
    {
        let authored = match option.value_kind() {
            OverrideValueKind::PositiveReal | OverrideValueKind::NonNegativeReal => "1.25e-8",
            OverrideValueKind::IterationCount => "77",
            OverrideValueKind::Flag => "off",
            OverrideValueKind::Method => "EULER",
            OverrideValueKind::Damping => "BANKROSE",
            OverrideValueKind::Solver => "KLU",
            OverrideValueKind::TimeDomainMode => "Transient-assisted",
            OverrideValueKind::TimeList => "1u 2u 3u",
        };
        record
            .set_for_instance(
                kind,
                crate::simulation::plan::SolverOwnership::NONE,
                option,
                authored,
            )
            .unwrap_or_else(|error| panic!("{} is authorable: {error}", option.key()));
    }

    // The plan's block first, the analysis's second, as a run splices them.
    let deck = crate::simulation::SimulationController::apply_simulation_options_to_netlist(
        "two blocks\nV1 1 0 1\nR1 1 0 1k\n.op\n.end\n",
        &options,
    );
    let spliced = deck.replace(".end", &format!("{}\n.end", record.to_spice_options()));
    let netlist = rspice_core::netlist::parse_netlist(&spliced)
        .unwrap_or_else(|error| panic!("the two-block deck must parse: {error}\n{spliced}"));
    let resolved = rspice_core::resolve_simulation_config(
        &rspice_core::engine::SimulationConfig::default(),
        Some(&netlist.options),
        &rspice_core::SimulationConfigOverrides::default(),
    );

    assert_eq!(resolved.convergence_config.voltage_reltol, 1.25e-8);
    assert_eq!(resolved.convergence_config.residual_reltol, 1.25e-8);
    assert_eq!(
        resolved.convergence_config.current_abstol, 1.25e-8,
        "the analysis's current floor must outrank the plan's IABSTOL"
    );
    assert_eq!(resolved.convergence_config.voltage_abstol, 1.25e-8);
    assert_eq!(resolved.convergence_config.charge_abstol, 1.25e-8);
    assert_eq!(resolved.convergence_config.junction_gmin_target, 1.25e-8);
    assert_eq!(resolved.matrix_pivot_tolerance, 1.25e-8);
    assert_eq!(resolved.matrix_absolute_pivot_tolerance, 1.25e-8);
    assert_eq!(resolved.transient_trtol, 1.25e-8);
    assert_eq!(resolved.transient_lte_reltol, Some(1.25e-8));
    assert_eq!(resolved.transient_lte_abstol, Some(1.25e-8));
    assert_eq!(resolved.min_timestep, 1.25e-8);
    assert_eq!(resolved.transient_timeint_max_timestep, Some(1.25e-8));
    assert_eq!(resolved.bypass_config.reltol, 1.25e-8);
    assert_eq!(resolved.bypass_config.abstol, 1.25e-8);
    assert_eq!(resolved.max_iterations, 77);
    assert_eq!(resolved.transient_max_iterations, 77);
    assert!(!resolved.convergence_config.gmin_stepping);
    assert!(!resolved.convergence_config.source_stepping);
    assert!(!resolved.bypass_config.enabled);
    assert_eq!(
        resolved.convergence_config.damping_strategy,
        rspice_core::engine::DampingStrategy::BankRose
    );

    // And the one bound the analysis cannot state is still the plan's.
    assert_eq!(
        resolved.max_timestep, 4.0e-9,
        "the plan's own run ceiling is not a key this record states"
    );
}

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

#[test]
fn full_width_seed_survives_options_editing_and_prepared_deck_parsing() {
    for seed in [(1_u64 << 53) + 1, u64::MAX - 1, u64::MAX] {
        let options = SimulationOptions {
            statistical_seed: Some(seed),
            ..SimulationOptions::default()
        };
        let draft = crate::simulation::dialog::OptionsDialogState::from_options(&options);
        let restored = draft
            .to_options()
            .expect("the UI supports the complete u64 seed range");
        assert_eq!(restored.statistical_seed, Some(seed));
        assert_eq!(parse_through_the_deck(&restored).options.seed, Some(seed));
    }
}

#[test]
fn solver_options_preserve_legal_zero_bounds_and_temperature_limits() {
    let options = SimulationOptions {
        gmin: -0.0,
        bypass_enabled: true,
        bypass_reltol: -0.0,
        bypass_abstol: -0.0,
        pivrel: 1.0,
        temp: (-273.15_f64).next_up(),
        tnom: (-273.15_f64).next_up(),
        ..SimulationOptions::default()
    };
    options
        .validate()
        .expect("these boundary values are allowed");
    let draft = crate::simulation::dialog::OptionsDialogState::from_options(&options);
    let restored = draft.to_options().unwrap();
    let parsed = parse_through_the_deck(&restored).options;
    for (name, authored, restored, parsed) in [
        ("GMIN", options.gmin, restored.gmin, parsed.gmin.unwrap()),
        (
            "BYPASSRELTOL",
            options.bypass_reltol,
            restored.bypass_reltol,
            parsed.bypass_reltol.unwrap(),
        ),
        (
            "BYPASSABSTOL",
            options.bypass_abstol,
            restored.bypass_abstol,
            parsed.bypass_abstol.unwrap(),
        ),
        ("TEMP", options.temp, restored.temp, parsed.temp.unwrap()),
        ("TNOM", options.tnom, restored.tnom, parsed.tnom.unwrap()),
    ] {
        assert_eq!(
            restored.to_bits(),
            authored.to_bits(),
            "{name} changed in the editor"
        );
        assert_eq!(
            parsed.to_bits(),
            authored.to_bits(),
            "{name} changed in the deck"
        );
    }
}

#[test]
fn solver_options_preserve_full_precision_through_the_deck() {
    let options = SimulationOptions {
        reltol: 1.234_567_891_234e-5,
        residual_reltol: 2.345_678_912_345e-5,
        abstol: 3.456_789_123_456e-13,
        iabstol: 4.567_891_234_567e-13,
        vntol: 5.678_912_345_678e-7,
        chgtol: 6.789_123_456_789e-15,
        pivrel: 0.123_456_789,
        pivtol: 1.234_567_891_234e-14,
        gmin: 9.123_456_789_123e-13,
        min_timestep: 1.234_567_891_234e-18,
        max_timestep: 1.234_567_891_234e-4,
        transient_lte_reltol: Some(1.234_567_891_234e-7),
        transient_lte_abstol: Some(2.345_678_912_345e-11),
        bypass_enabled: true,
        bypass_reltol: 3.456_789_123_456e-4,
        bypass_abstol: 4.567_891_234_567e-7,
        temp: 27.123_456_789,
        tnom: -40.123_456_789,
        ..SimulationOptions::default()
    };
    options.validate().unwrap();
    let netlist = parse_through_the_deck(&options);
    let resolved = resolve_through_the_deck(&options);
    for (name, authored, parsed) in [
        ("RELTOL", options.reltol, netlist.options.reltol.unwrap()),
        (
            "RESIDUAL_RELTOL",
            options.residual_reltol,
            netlist.options.residual_reltol.unwrap(),
        ),
        ("ABSTOL", options.abstol, netlist.options.abstol.unwrap()),
        ("IABSTOL", options.iabstol, netlist.options.iabstol.unwrap()),
        ("VNTOL", options.vntol, netlist.options.vntol.unwrap()),
        ("CHGTOL", options.chgtol, netlist.options.chgtol.unwrap()),
        ("GMIN", options.gmin, netlist.options.gmin.unwrap()),
        ("PIVREL", options.pivrel, resolved.matrix_pivot_tolerance),
        (
            "PIVTOL",
            options.pivtol,
            resolved.matrix_absolute_pivot_tolerance,
        ),
        ("MINTIMESTEP", options.min_timestep, resolved.min_timestep),
        ("MAXTIMESTEP", options.max_timestep, resolved.max_timestep),
        (
            "TIMEINT RELTOL",
            options.transient_lte_reltol.unwrap(),
            resolved.transient_lte_reltol.unwrap(),
        ),
        (
            "TIMEINT ABSTOL",
            options.transient_lte_abstol.unwrap(),
            resolved.transient_lte_abstol.unwrap(),
        ),
        (
            "BYPASSRELTOL",
            options.bypass_reltol,
            resolved.bypass_config.reltol,
        ),
        (
            "BYPASSABSTOL",
            options.bypass_abstol,
            resolved.bypass_config.abstol,
        ),
        ("TEMP", options.temp, netlist.options.temp.unwrap()),
        ("TNOM", options.tnom, netlist.options.tnom.unwrap()),
    ] {
        assert_eq!(
            parsed.to_bits(),
            authored.to_bits(),
            "{name} was rounded in the prepared deck"
        );
    }
    assert_eq!(resolved.temperature, options.temp_kelvin());
}

#[test]
fn solver_options_preserve_changes_adjacent_to_defaults() {
    let default = SimulationOptions::default();
    let options = SimulationOptions {
        reltol: default.reltol.next_up(),
        residual_reltol: default.residual_reltol.next_up(),
        abstol: default.abstol.next_up(),
        iabstol: default.iabstol.next_up(),
        vntol: default.vntol.next_up(),
        chgtol: default.chgtol.next_up(),
        gmin: default.gmin.next_up(),
        temp: default.temp.next_up(),
        tnom: default.tnom.next_up(),
        bypass_enabled: true,
        bypass_reltol: default.bypass_reltol.next_up(),
        bypass_abstol: default.bypass_abstol.next_up(),
        ..default
    };
    options.validate().unwrap();
    let parsed = parse_through_the_deck(&options).options;
    for (name, authored, parsed) in [
        ("RELTOL", options.reltol, parsed.reltol),
        (
            "RESIDUAL_RELTOL",
            options.residual_reltol,
            parsed.residual_reltol,
        ),
        ("ABSTOL", options.abstol, parsed.abstol),
        ("IABSTOL", options.iabstol, parsed.iabstol),
        ("VNTOL", options.vntol, parsed.vntol),
        ("CHGTOL", options.chgtol, parsed.chgtol),
        ("GMIN", options.gmin, parsed.gmin),
        ("TEMP", options.temp, parsed.temp),
        ("TNOM", options.tnom, parsed.tnom),
        ("BYPASSRELTOL", options.bypass_reltol, parsed.bypass_reltol),
        ("BYPASSABSTOL", options.bypass_abstol, parsed.bypass_abstol),
    ] {
        let parsed =
            parsed.unwrap_or_else(|| panic!("{name} was omitted despite an authored change"));
        assert_eq!(
            parsed.to_bits(),
            authored.to_bits(),
            "{name} lost the authored change"
        );
    }
}
