//! The catalog's admission rule, proven rather than asserted.
//!
//! [`every_option_moves_the_resolved_engine_configuration`] is the ratchet: it
//! takes each option in turn, emits it, parses the deck with the engine's own
//! parser, resolves it with the engine's own resolver, and fails on any option
//! that leaves the resolved [`SimulationConfig`] untouched. An option that
//! reaches nothing cannot pass, whatever the catalog claims about it.

use super::*;

use rspice_core::engine::SimulationConfig;

/// The resolved engine configuration one record produces, through the exact
/// path a prepared run takes: emit, splice, parse, resolve.
fn resolve(record: &AnalysisNumericOverride) -> SimulationConfig {
    let emitted = record.to_spice_options();
    let deck = format!("ratchet\nV1 1 0 1\nR1 1 0 1k\n{emitted}\n.op\n.end\n");
    let netlist = rspice_core::netlist::parse_netlist(&deck)
        .unwrap_or_else(|error| panic!("the emitted cards must parse: {error}\n{deck}"));
    rspice_core::resolve_simulation_config(
        &SimulationConfig::default(),
        Some(&netlist.options),
        &rspice_core::SimulationConfigOverrides::default(),
    )
}

/// Authored strings to try for one option.
///
/// An option is live if *any* candidate moves the resolved configuration, so
/// the pool only has to contain one value the option actually distinguishes
/// from the engine's default.
fn candidates(option: NumericOverrideOption) -> Vec<String> {
    match option.value_kind() {
        OverrideValueKind::PositiveReal | OverrideValueKind::NonNegativeReal => {
            ["3.25e-7", "1.5e-3", "7", "2.5e-14"]
                .iter()
                .map(|value| (*value).to_owned())
                .collect()
        }
        OverrideValueKind::IterationCount => ["37", "211"]
            .iter()
            .map(|value| (*value).to_owned())
            .collect(),
        OverrideValueKind::Flag => ["on", "off"]
            .iter()
            .map(|value| (*value).to_owned())
            .collect(),
        OverrideValueKind::Method => IntegrationMethod::all()
            .iter()
            .map(|method| method.spice_name().to_owned())
            .collect(),
        OverrideValueKind::Damping => DampingStrategy::all()
            .iter()
            .map(|strategy| strategy.spice_name().to_owned())
            .collect(),
        OverrideValueKind::Solver => MatrixSolver::all()
            .iter()
            .filter_map(|solver| solver.spice_name())
            .map(str::to_owned)
            .collect(),
    }
}

/// The first analysis kind that may carry this option.
fn authoring_kind(option: NumericOverrideOption) -> AnalysisKind {
    AnalysisKind::ALL
        .into_iter()
        .find(|kind| option.refusal_for(*kind).is_none())
        .unwrap_or_else(|| {
            panic!(
                "{} is refused by every analysis kind, so nothing could ever author it",
                option.key()
            )
        })
}

#[test]
fn the_catalog_states_one_entry_and_one_consumer_per_option() {
    let mut seen: Vec<NumericOverrideOption> = Vec::new();
    for option in NumericOverrideOption::all() {
        assert!(
            !seen.contains(&option),
            "{} appears twice in the catalog",
            option.key()
        );
        seen.push(option);
        let spec = option.spec();
        // A file and a line, because that is what the module doc promises and
        // what makes the admission rule checkable by reading the table. A bare
        // subsystem name is the kind of citation nobody can falsify.
        let cited_line = spec.consumer.split_once(".rs:").map(|(_, rest)| {
            rest.split(|c: char| !c.is_ascii_digit())
                .next()
                .unwrap_or("")
        });
        assert!(
            cited_line.is_some_and(|line| !line.is_empty()),
            "{} must cite the engine file and line that reads it, not a subsystem: {}",
            option.key(),
            spec.consumer
        );
        assert!(
            !spec.config_field.is_empty(),
            "{} must name the SimulationConfig field it resolves onto",
            option.key()
        );
    }
    assert_eq!(seen.len(), 25, "the catalog size changed; update the count");
}

/// The admission rule, mechanically.
///
/// This is what stops the studio growing a twenty-sixth control that is
/// rendered, validated, persisted, emitted — and read by nothing. A key the
/// resolver has no arm for (`ITL2` and `ITL6` are the live examples) resolves
/// to a byte-identical configuration and fails here.
#[test]
fn every_option_moves_the_resolved_engine_configuration() {
    let baseline = format!("{:?}", resolve(&AnalysisNumericOverride::default()));
    let mut inert = Vec::new();

    for option in NumericOverrideOption::all() {
        let kind = authoring_kind(option);
        let mut moved = false;
        for authored in candidates(option) {
            let mut record = AnalysisNumericOverride::default();
            if record.set(kind, option, &authored).is_err() {
                continue;
            }
            if format!("{:?}", resolve(&record)) != baseline {
                moved = true;
                break;
            }
        }
        if !moved {
            inert.push(format!(
                "  {} ({}) — claims {}",
                option.key(),
                option.config_field(),
                option.consumer()
            ));
        }
    }

    assert!(
        inert.is_empty(),
        "options that reach no engine configuration — delete them, or find the key the resolver \
         actually reads:\n{}",
        inert.join("\n")
    );
}

/// Every option's authored value survives the deck exactly.
///
/// Moving the configuration is not enough on its own: a value that arrived
/// rounded, or onto the wrong field, would still move it. This checks the
/// value that lands.
#[test]
fn every_stated_option_round_trips_through_the_deck_at_full_precision() {
    let mut record = AnalysisNumericOverride::default();
    for option in NumericOverrideOption::all() {
        let kind = authoring_kind(option);
        let authored = candidates(option)
            .into_iter()
            .find(|authored| {
                let mut probe = AnalysisNumericOverride::default();
                probe.set(kind, option, authored).is_ok()
            })
            .unwrap_or_else(|| panic!("{} has an authorable candidate", option.key()));
        record
            .set(kind, option, &authored)
            .unwrap_or_else(|error| panic!("{} is authorable: {error}", option.key()));
    }

    // Every option at once, so a key that re-scoped the card after it — the
    // package-selector trap — shows up as a neighbour that stopped arriving.
    let emitted = record.to_spice_options();
    let deck = format!("all options\nV1 1 0 1\nR1 1 0 1k\n{emitted}\n.op\n.end\n");
    let netlist = rspice_core::netlist::parse_netlist(&deck)
        .unwrap_or_else(|error| panic!("the full card set must parse: {error}\n{deck}"));
    let options = &netlist.options;
    let resolved = resolve(&record);

    assert_eq!(options.reltol, Some(3.25e-7));
    assert_eq!(options.iabstol, Some(3.25e-7));
    assert_eq!(options.vntol, Some(3.25e-7));
    assert_eq!(options.residual_reltol, Some(3.25e-7));
    assert_eq!(options.gmin, Some(3.25e-7));
    assert_eq!(options.chgtol, Some(3.25e-7));
    assert_eq!(options.pivrel, Some(3.25e-7));
    assert_eq!(options.pivtol, Some(3.25e-7));
    assert_eq!(options.itl1, Some(37));
    assert_eq!(options.itl4, Some(37));
    assert_eq!(options.trtol, Some(3.25e-7));
    assert_eq!(options.timeint_reltol, Some(3.25e-7));
    assert_eq!(options.timeint_abstol, Some(3.25e-7));
    assert_eq!(options.timeint_min_timestep, Some(3.25e-7));
    assert_eq!(options.timeint_delmax, Some(3.25e-7));
    assert_eq!(options.bypass_reltol, Some(3.25e-7));
    assert_eq!(options.bypass_abstol, Some(3.25e-7));

    // And the fields those keys resolve onto, which is what the engine reads.
    assert_eq!(resolved.convergence_config.voltage_reltol, 3.25e-7);
    assert_eq!(resolved.convergence_config.current_abstol, 3.25e-7);
    assert_eq!(resolved.convergence_config.voltage_abstol, 3.25e-7);
    assert_eq!(resolved.convergence_config.residual_reltol, 3.25e-7);
    assert_eq!(resolved.convergence_config.junction_gmin_target, 3.25e-7);
    assert_eq!(resolved.convergence_config.charge_abstol, 3.25e-7);
    assert_eq!(resolved.matrix_pivot_tolerance, 3.25e-7);
    assert_eq!(resolved.matrix_absolute_pivot_tolerance, 3.25e-7);
    assert_eq!(resolved.max_iterations, 37);
    assert_eq!(resolved.transient_max_iterations, 37);
    assert_eq!(resolved.transient_trtol, 3.25e-7);
    assert_eq!(resolved.transient_lte_reltol, Some(3.25e-7));
    assert_eq!(resolved.transient_lte_abstol, Some(3.25e-7));
    assert_eq!(resolved.min_timestep, 3.25e-7);
    assert_eq!(resolved.transient_timeint_max_timestep, Some(3.25e-7));
    assert_eq!(resolved.bypass_config.reltol, 3.25e-7);
    assert_eq!(resolved.bypass_config.abstol, 3.25e-7);
}

/// The two `RELTOL` spellings are different bounds and must stay apart.
///
/// A global `RELTOL` bounds the Newton update; a `TIMEINT RELTOL` bounds the
/// accepted local truncation error. They share a key and differ only by
/// package, so an emitter that lost the package boundary would silently make
/// one of them overwrite the other.
#[test]
fn the_global_and_timeint_reltols_resolve_onto_their_own_fields() {
    let mut record = AnalysisNumericOverride::default();
    record
        .set(AnalysisKind::Fourier, NumericOverrideOption::Reltol, "1e-5")
        .expect("a Fourier measurement carries a Newton bound");
    record
        .set(
            AnalysisKind::Fourier,
            NumericOverrideOption::LteReltol,
            "4e-9",
        )
        .expect("a Fourier measurement advances time");

    let resolved = resolve(&record);
    assert_eq!(resolved.convergence_config.voltage_reltol, 1e-5);
    assert_eq!(resolved.transient_lte_reltol, Some(4e-9));
    assert_ne!(
        NumericOverrideOption::Reltol.key(),
        NumericOverrideOption::LteReltol.key(),
        "the two bounds must not report the same name to a reader"
    );
}

/// The current floor survives a plan that states the other spelling.
///
/// `resolve_simulation_config` reads `opts.iabstol.or(opts.abstol)`, which is
/// field precedence and not card order. An analysis that emitted `ABSTOL`
/// would therefore be overruled by any plan stating `IABSTOL`, however late
/// the analysis's card arrived — accepted, persisted, reported on the ledger,
/// and then ignored by the solve. The record states `IABSTOL` for exactly this
/// reason, and this pins it against a plan block that states both.
#[test]
fn an_analysis_current_floor_outranks_a_plan_that_states_both_spellings() {
    let mut record = AnalysisNumericOverride::default();
    record
        .set(AnalysisKind::Ac, NumericOverrideOption::Abstol, "4e-13")
        .expect("every kind carries a current floor");

    // The plan's block first, the analysis's second, exactly as a prepared
    // deck splices them.
    let deck = format!(
        "shadowing\nV1 1 0 1\nR1 1 0 1k\n.OPTIONS\n+ ABSTOL=1e-11\n+ IABSTOL=2e-11\n{}\n.op\n.end\n",
        record.to_spice_options()
    );
    let netlist = rspice_core::netlist::parse_netlist(&deck)
        .unwrap_or_else(|error| panic!("the spliced deck must parse: {error}\n{deck}"));
    let resolved = rspice_core::resolve_simulation_config(
        &SimulationConfig::default(),
        Some(&netlist.options),
        &rspice_core::SimulationConfigOverrides::default(),
    );

    assert_eq!(
        resolved.convergence_config.current_abstol, 4e-13,
        "the analysis's floor must reach the solve, not the plan's IABSTOL"
    );
}

#[test]
fn an_empty_record_adds_nothing_to_a_deck() {
    let record = AnalysisNumericOverride::default();
    assert!(record.is_empty());
    assert!(record.to_spice_options().is_empty());
    assert!(record.entries().is_empty());
}

#[test]
fn a_step_ceiling_is_emitted_through_the_timeint_package() {
    let mut record = AnalysisNumericOverride::default();
    record
        .set(
            AnalysisKind::Fourier,
            NumericOverrideOption::MaximumTimestep,
            "500p",
        )
        .expect("a Fourier measurement runs a transient");
    let emitted = record.to_spice_options();
    assert_eq!(emitted, ".OPTIONS TIMEINT\n+ DELMAX=5e-10");
    assert_eq!(
        resolve(&record).transient_timeint_max_timestep,
        Some(5.0e-10)
    );
    assert_eq!(
        record
            .value(NumericOverrideOption::MaximumTimestep)
            .unwrap(),
        format_si_value(5.0e-10)
    );
}

/// A global key and a packaged key on one record produce two cards, global
/// first. The selector latches, so the order is load-bearing.
#[test]
fn the_global_card_precedes_the_packaged_one() {
    let mut record = AnalysisNumericOverride::default();
    record
        .set(AnalysisKind::Fourier, NumericOverrideOption::Reltol, "1e-5")
        .expect("authorable");
    record
        .set(
            AnalysisKind::Fourier,
            NumericOverrideOption::MinTimestep,
            "2e-18",
        )
        .expect("authorable");
    assert_eq!(
        record.to_spice_options(),
        ".OPTIONS\n+ RELTOL=1e-5\n.OPTIONS TIMEINT\n+ MINTIMESTEP=2e-18"
    );
    let resolved = resolve(&record);
    assert_eq!(resolved.convergence_config.voltage_reltol, 1e-5);
    assert_eq!(resolved.min_timestep, 2e-18);
}

#[test]
fn a_flag_is_emitted_as_the_digit_the_parser_reads() {
    let mut record = AnalysisNumericOverride::default();
    record
        .set(AnalysisKind::Ac, NumericOverrideOption::GminStepping, "off")
        .expect("every kind runs a DC solve");
    assert_eq!(record.to_spice_options(), ".OPTIONS\n+ GMINSTEPPING=0");
    assert!(!resolve(&record).convergence_config.gmin_stepping);
    assert_eq!(
        record.value(NumericOverrideOption::GminStepping).unwrap(),
        "off"
    );

    record
        .set(AnalysisKind::Ac, NumericOverrideOption::GminStepping, "1")
        .expect("the digit spelling is accepted too");
    assert!(resolve(&record).convergence_config.gmin_stepping);
}

/// `GMIN=0` is a request, not an empty field.
#[test]
fn a_zero_junction_conductance_floor_is_authorable_and_reaches_the_engine() {
    let mut record = AnalysisNumericOverride::default();
    record
        .set(AnalysisKind::Ac, NumericOverrideOption::Gmin, "0")
        .expect("asking for no junction floor is a real request");
    assert_eq!(
        resolve(&record).convergence_config.junction_gmin_target,
        0.0
    );
}

#[test]
fn an_option_the_kind_cannot_use_is_refused_and_stores_nothing() {
    let mut record = AnalysisNumericOverride::default();
    let error = record
        .set(AnalysisKind::Ac, NumericOverrideOption::Itl4, "12")
        .expect_err("an AC sweep never takes a timestep");
    assert!(error.contains("ITL4"), "{error}");
    assert!(record.is_empty());

    let error = record
        .set(
            AnalysisKind::Transient,
            NumericOverrideOption::MaximumTimestep,
            "1n",
        )
        .expect_err("the transient form owns its step ceiling");
    assert!(error.contains("Max step"), "{error}");

    let error = record
        .set(
            AnalysisKind::OperatingPoint,
            NumericOverrideOption::Itl1,
            "200",
        )
        .expect_err("the operating point's accuracy tier owns its Newton budget");
    assert!(error.contains("accuracy tier"), "{error}");
    assert!(record.is_empty());
}

/// Every option whose only engine reads are under `engine/transient` refuses a
/// kind that never advances time.
#[test]
fn the_time_stepped_options_are_refused_by_a_kind_that_never_steps() {
    for option in [
        NumericOverrideOption::Itl4,
        NumericOverrideOption::Chgtol,
        NumericOverrideOption::Trtol,
        NumericOverrideOption::IntegrationMethod,
        NumericOverrideOption::LteReltol,
        NumericOverrideOption::LteAbstol,
        NumericOverrideOption::MinTimestep,
        NumericOverrideOption::MaximumTimestep,
    ] {
        assert_eq!(
            option.refusal_for(AnalysisKind::Ac),
            Some(catalog::NOT_TIME_STEPPED),
            "{} is only read on a time-stepped path",
            option.key()
        );
    }
    // And a kind that does step carries all of them but the one the transient
    // form owns.
    let stepping = NumericOverrideOption::applicable_to(AnalysisKind::Fourier);
    assert!(stepping.contains(&NumericOverrideOption::Chgtol));
    assert!(stepping.contains(&NumericOverrideOption::MaximumTimestep));
}

#[test]
fn a_value_that_cannot_bound_a_solve_is_refused() {
    let mut record = AnalysisNumericOverride::default();
    for (kind, option, authored) in [
        (AnalysisKind::Ac, NumericOverrideOption::Reltol, "0"),
        (AnalysisKind::Ac, NumericOverrideOption::Reltol, "-1e-3"),
        (AnalysisKind::Ac, NumericOverrideOption::Reltol, "wide"),
        (AnalysisKind::Ac, NumericOverrideOption::Itl1, "0"),
        (AnalysisKind::Ac, NumericOverrideOption::Itl1, "2.5"),
        (AnalysisKind::Ac, NumericOverrideOption::Gmin, "-1e-12"),
        (
            AnalysisKind::Ac,
            NumericOverrideOption::GminStepping,
            "yes please",
        ),
        (AnalysisKind::Ac, NumericOverrideOption::Damping, "gentle"),
        // The automatic backend names no key, so it cannot be an override.
        (AnalysisKind::Ac, NumericOverrideOption::Solver, "LU"),
        (
            AnalysisKind::Transient,
            NumericOverrideOption::IntegrationMethod,
            "simpson",
        ),
        // Retired chooser spellings. A saved project decodes them onto
        // the surviving method, but authoring one here is a typo.
        (
            AnalysisKind::Transient,
            NumericOverrideOption::IntegrationMethod,
            "GEAR2ONLY",
        ),
        (
            AnalysisKind::Transient,
            NumericOverrideOption::IntegrationMethod,
            "GEAR",
        ),
    ] {
        assert!(
            record.set(kind, option, authored).is_err(),
            "{option:?} must refuse {authored:?}"
        );
    }
    assert!(record.is_empty());
}

#[test]
fn a_restored_record_is_re_checked_against_its_kind() {
    let mut record = AnalysisNumericOverride::default();
    record
        .set(AnalysisKind::Transient, NumericOverrideOption::Itl4, "12")
        .expect("a transient takes timesteps");
    assert!(record.first_refusal_for(AnalysisKind::Transient).is_none());
    let (option, _) = record
        .first_refusal_for(AnalysisKind::Ac)
        .expect("the same record cannot be carried by an AC sweep");
    assert_eq!(option, NumericOverrideOption::Itl4);
}

#[test]
fn a_record_naming_a_retired_method_decodes_onto_the_survivor() {
    // The chooser stopped offering the plain `Gear` and `Gear2Only`
    // spellings, but an analysis authored under either still has to open
    // and still has to emit a card the parser reads.
    for retired in ["Gear", "Gear2Only"] {
        let record: AnalysisNumericOverride =
            serde_json::from_str(&format!(r#"{{"integration_method":"{retired}"}}"#))
                .unwrap_or_else(|error| panic!("a record naming {retired} decodes: {error}"));

        assert_eq!(
            record.stated(NumericOverrideOption::IntegrationMethod),
            Some(OverrideValue::Method(IntegrationMethod::Gear2))
        );
        assert_eq!(record.to_spice_options(), ".OPTIONS\n+ METHOD=GEAR2");
    }
}

/// A project written before the advanced options existed still opens, and
/// states exactly what it stated then.
#[test]
fn a_record_persisted_before_the_advanced_options_still_opens() {
    let record: AnalysisNumericOverride = serde_json::from_str(
        r#"{"reltol":0.0002,"itl4":12,"integration_method":"Gear2","max_timestep":5e-10}"#,
    )
    .expect("a record written under the original nine fields decodes");

    assert_eq!(
        record.entries(),
        vec![
            (NumericOverrideOption::Reltol, "2e-4".to_owned()),
            (NumericOverrideOption::Itl4, "12".to_owned()),
            (NumericOverrideOption::IntegrationMethod, "GEAR2".to_owned()),
            (
                NumericOverrideOption::MaximumTimestep,
                format_si_value(5.0e-10)
            ),
        ],
        "an old record must state its four options and nothing else"
    );
    for option in NumericOverrideOption::all() {
        if matches!(
            option,
            NumericOverrideOption::Reltol
                | NumericOverrideOption::Itl4
                | NumericOverrideOption::IntegrationMethod
                | NumericOverrideOption::MaximumTimestep
        ) {
            continue;
        }
        assert_eq!(
            record.stated(option),
            None,
            "{} was not stated by an old project and must not be invented",
            option.key()
        );
    }
}

#[test]
fn clearing_an_option_returns_it_to_the_plan() {
    let mut record = AnalysisNumericOverride::default();
    record
        .set(AnalysisKind::Ac, NumericOverrideOption::Pivtol, "2.5e-14")
        .expect("authorable");
    record
        .set(AnalysisKind::Ac, NumericOverrideOption::Solver, "KLU")
        .expect("authorable");
    assert_eq!(record.entries().len(), 2);
    record.clear(NumericOverrideOption::Solver);
    assert_eq!(record.entries().len(), 1);
    assert_eq!(record.stated(NumericOverrideOption::Solver), None);
    record.clear(NumericOverrideOption::Pivtol);
    assert!(record.is_empty(), "a cleared record is an absent one");
}

/// The sections partition the catalog: every option is in exactly one, and
/// none of them is empty.
#[test]
fn every_option_belongs_to_exactly_one_section() {
    let mut counted = 0;
    for section in OverrideSection::ALL {
        let members: Vec<_> = NumericOverrideOption::all()
            .filter(|option| option.section() == section)
            .collect();
        assert!(
            !members.is_empty(),
            "{} is an empty section; delete it or fill it",
            section.title()
        );
        counted += members.len();
    }
    assert_eq!(
        counted,
        NumericOverrideOption::all().count(),
        "an option is in two sections, or in none"
    );
}
