//! The catalog's admission rule, proven rather than asserted.
//!
//! [`every_option_moves_the_resolved_engine_configuration`] is the ratchet: it
//! takes each option in turn, emits it, parses the deck with the engine's own
//! parser, resolves it with the engine's own resolver, and fails on any option
//! that leaves the resolved [`SimulationConfig`] untouched. An option that
//! reaches nothing cannot pass, whatever the catalog claims about it.

use super::*;

use crate::simulation::dialog::{HbTimeDomainMode, format_si_value};
use rspice_core::engine::SimulationConfig;

/// The resolved engine configuration one record produces, through the exact
/// path a prepared run takes: emit, splice, parse, resolve.
fn resolve(record: &AnalysisNumericOverride) -> SimulationConfig {
    resolve_and_parse(record).0
}

/// Both halves of what an emitted record reaches: the resolved configuration
/// and the parsed option record the engine also reads directly.
///
/// The catalog admits an option on either route — the resolver maps it onto a
/// `SimulationConfig` field, *or* the engine reads the parsed package record
/// off `netlist.options` — so the ratchet has to be able to look at both. A
/// key that moves neither is inert whichever route it claimed.
fn resolve_and_parse(
    record: &AnalysisNumericOverride,
) -> (SimulationConfig, rspice_core::netlist::SimulationOptions) {
    let emitted = record.to_spice_options();
    let deck = format!("ratchet\nV1 1 0 1\nR1 1 0 1k\n{emitted}\n.op\n.end\n");
    let netlist = rspice_core::netlist::parse_netlist(&deck)
        .unwrap_or_else(|error| panic!("the emitted cards must parse: {error}\n{deck}"));
    let resolved = rspice_core::resolve_simulation_config(
        &SimulationConfig::default(),
        Some(&netlist.options),
        &rspice_core::SimulationConfigOverrides::default(),
    );
    (resolved, netlist.options)
}

/// What one record reaches, as one comparable string.
fn reach_of(record: &AnalysisNumericOverride) -> String {
    let (resolved, options) = resolve_and_parse(record);
    format!("{resolved:?}\n{options:?}")
}

/// Authored strings to try for one option.
///
/// An option is live if *any* candidate moves the resolved configuration, so
/// the pool only has to contain one value the option actually distinguishes
/// from the engine's default.
fn candidates(option: NumericOverrideOption) -> Vec<String> {
    match option.value_kind() {
        OverrideValueKind::PositiveReal | OverrideValueKind::NonNegativeReal => {
            // The last one is tighter than every ceiling
            // `AccuracyPolicy::apply` imposes, so an option this pool has to
            // move under the `Accurate` tier still has one candidate the tier
            // does not clamp. A tier only ever tightens, so a value below all
            // of its ceilings is the one that reaches the solve unchanged.
            ["3.25e-7", "1.5e-3", "7", "2.5e-14", "1.0e-16"]
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
        OverrideValueKind::TimeDomainMode => HbTimeDomainMode::all()
            .iter()
            .map(|mode| mode.display_name().to_owned())
            .collect(),
        // Three increasing stops, well inside any transient's window. The
        // pool holds one candidate because a schedule is a schedule: there is
        // no engine default for it to fail to differ from.
        OverrideValueKind::TimeList => vec!["1u 2u 3u".to_owned()],
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
        // A packaged key reports qualified. Two options spell the same bare
        // key under different packages — three spell `RELTOL` — so an entry
        // that forgot its `packaged_name` arm would report a name the ledger
        // shows twice and a refusal cannot tell apart.
        if option.package() != OptionPackage::Global {
            assert_eq!(
                option.key(),
                format!("{} {}", option.package().name(), spec.key),
                "a packaged option reports its package and its key"
            );
        } else {
            assert_eq!(
                option.key(),
                spec.key,
                "a global option reports its bare key"
            );
        }
    }
    assert_eq!(seen.len(), 36, "the catalog size changed; update the count");
}

/// The admission rule, mechanically.
///
/// This is what stops the studio growing a twenty-sixth control that is
/// rendered, validated, persisted, emitted — and read by nothing. A key the
/// resolver has no arm for (`ITL2` and `ITL6` are the live examples) resolves
/// to a byte-identical configuration and fails here.
#[test]
fn every_option_moves_the_resolved_engine_configuration() {
    let baseline = reach_of(&AnalysisNumericOverride::default());
    let mut inert = Vec::new();

    for option in NumericOverrideOption::all() {
        let kind = authoring_kind(option);
        let mut moved = false;
        for authored in candidates(option) {
            let mut record = AnalysisNumericOverride::default();
            if record
                .set_for_instance(kind, SolverOwnership::NONE, option, &authored)
                .is_err()
            {
                continue;
            }
            if reach_of(&record) != baseline {
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

/// The same ratchet, carried past the two layers the deck does not own.
///
/// [`resolve`] stops at `resolve_simulation_config`, which is where the
/// operating point's own resolution *begins* its last two steps: the accuracy
/// tier and the homotopy control are both applied on top of it
/// (`simulation/engine_bridge/dc.rs` `resolved_op_config`, the function the
/// engine is actually constructed from). Five options land on fields those two
/// assign, so an option accepted here that did not survive them would be
/// accepted, persisted, reported on the advanced-options panel — and then
/// overwritten before the first Newton step.
///
/// Every combination of tier and homotopy is walked, because the gate is per
/// instance: what `Balanced`/`Adaptive` may author, `Robust` may not.
#[test]
fn every_option_an_operating_point_accepts_reaches_the_engine_it_is_built_from() {
    use crate::simulation::accuracy::AnalysisAccuracy;
    use crate::simulation::dialog::{OpConfig, OpHomotopy};

    fn resolve_for_op(record: &AnalysisNumericOverride, config: &OpConfig) -> String {
        let emitted = record.to_spice_options();
        let deck = format!("op ratchet\nV1 1 0 1\nR1 1 0 1k\n{emitted}\n.op\n.end\n");
        let netlist = rspice_core::netlist::parse_netlist(&deck)
            .unwrap_or_else(|error| panic!("the emitted cards must parse: {error}\n{deck}"));
        format!(
            "{:?}",
            crate::simulation::engine_bridge::resolved_op_config(
                &SimulationConfig::default(),
                &netlist.options,
                config,
            )
        )
    }

    let kind = AnalysisKind::OperatingPoint;
    let mut discarded = Vec::new();
    for accuracy in AnalysisAccuracy::ALL {
        for homotopy in OpHomotopy::ALL {
            let config = OpConfig {
                accuracy,
                homotopy,
                ..OpConfig::default()
            };
            let ownership = SolverOwnership {
                accuracy: Some(accuracy),
                homotopy: Some(homotopy),
            };
            let baseline = resolve_for_op(&AnalysisNumericOverride::default(), &config);
            for option in NumericOverrideOption::all() {
                if option.refusal_for_instance(kind, ownership).is_some() {
                    continue;
                }
                let moved = candidates(option).into_iter().any(|authored| {
                    let mut record = AnalysisNumericOverride::default();
                    record
                        .set_for_instance(kind, ownership, option, &authored)
                        .is_ok()
                        && resolve_for_op(&record, &config) != baseline
                });
                if !moved {
                    discarded.push(format!(
                        "  {} under {} · {:?}",
                        option.key(),
                        accuracy.display_name(),
                        homotopy
                    ));
                }
            }
        }
    }

    assert!(
        discarded.is_empty(),
        "these options are accepted on an operating point and then overwritten by its own tier \
         or homotopy — refuse them there, naming the owner:\n{}",
        discarded.join("\n")
    );
}

/// The refusal names the control that decides and the setting that releases it.
///
/// "Not applicable" would send a reader hunting for a rule that does not
/// exist. Both owners here are controls on the same analysis's own form, so
/// the refusal can say which one and what to set it to.
#[test]
fn an_owned_option_is_refused_by_the_owner_that_assigns_it() {
    use crate::simulation::accuracy::AnalysisAccuracy;
    use crate::simulation::dialog::OpHomotopy;

    let robust = SolverOwnership {
        accuracy: Some(AnalysisAccuracy::Robust),
        homotopy: Some(OpHomotopy::Adaptive),
    };
    let refusal = NumericOverrideOption::GminStepping
        .refusal_for_instance(AnalysisKind::OperatingPoint, robust)
        .expect("Robust assigns every continuation aid after the deck");
    assert!(
        refusal.contains("Robust accuracy tier") && refusal.contains("set this analysis's tier"),
        "the refusal must name the owner and the fix: {refusal}"
    );

    // The transfer function resolves the same tier through the same
    // `AccuracyPolicy::apply`, and carries no homotopy control of its own.
    let tf = SolverOwnership {
        accuracy: Some(AnalysisAccuracy::Fast),
        homotopy: None,
    };
    assert!(
        NumericOverrideOption::Damping
            .refusal_for_instance(AnalysisKind::TransferFunction, tf)
            .is_some_and(|reason| reason.contains("Fast accuracy tier")),
        "a Fast transfer function assigns its damping strategy after the deck too"
    );

    // The homotopy control is applied after the tier, so under both it is the
    // one a reader has to change.
    let stepping = SolverOwnership {
        accuracy: Some(AnalysisAccuracy::Robust),
        homotopy: Some(OpHomotopy::SourceStepping),
    };
    let refusal = NumericOverrideOption::ArcLength
        .refusal_for_instance(AnalysisKind::OperatingPoint, stepping)
        .expect("an explicit homotopy assigns every aid");
    assert!(
        refusal.contains("Homotopy") && refusal.contains("Adaptive"),
        "the last writer is the one to name: {refusal}"
    );

    // Damping is not one of the fields a homotopy choice touches, so under an
    // inheriting tier it stays authorable however the homotopy is set.
    assert_eq!(
        NumericOverrideOption::Damping.refusal_for_instance(
            AnalysisKind::OperatingPoint,
            SolverOwnership {
                accuracy: Some(AnalysisAccuracy::Balanced),
                homotopy: Some(OpHomotopy::SourceStepping),
            }
        ),
        None
    );

    // And a record already holding one is refused when the instance is bound
    // to it, which is the path a restored project takes.
    let mut record = AnalysisNumericOverride::default();
    record
        .set_for_instance(
            AnalysisKind::OperatingPoint,
            SolverOwnership {
                accuracy: Some(AnalysisAccuracy::Balanced),
                homotopy: Some(OpHomotopy::Adaptive),
            },
            NumericOverrideOption::GminStepping,
            "on",
        )
        .expect("an inheriting instance may author the aid");
    assert!(
        record
            .first_refusal_for_instance(AnalysisKind::OperatingPoint, robust)
            .is_some(),
        "the same record must be refused once the instance's tier owns the field"
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
                probe
                    .set_for_instance(kind, SolverOwnership::NONE, option, authored)
                    .is_ok()
            })
            .unwrap_or_else(|| panic!("{} has an authorable candidate", option.key()));
        // One pair of options is mutually exclusive by design, so "every
        // option at once" cannot include both. Whichever comes first in the
        // catalog is stated and the other is skipped here; both are proven
        // separately by
        // `an_output_schedule_is_a_strobe_or_a_stop_list_and_not_both`.
        if option == NumericOverrideOption::OutputTimePoints
            && record
                .value(NumericOverrideOption::StrobeInterval)
                .is_some()
        {
            continue;
        }
        record
            .set_for_instance(kind, SolverOwnership::NONE, option, &authored)
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

    // The three packages admitted under the fourth route, read off the parsed
    // record rather than the resolved configuration — which is the whole point
    // of that route, and the reason this assertion block exists twice.
    assert_eq!(options.nonlin_transient_reltol, Some(3.25e-7));
    assert_eq!(options.nonlin_transient_abstol, Some(3.25e-7));
    assert_eq!(options.nonlin_transient_deltaxtol, Some(3.25e-7));
    assert_eq!(options.nonlin_transient_rhstol, Some(3.25e-7));
    assert_eq!(options.nonlin_transient_maxstep, Some(37));
    assert_eq!(
        options.nonlin_transient_enforce_device_convergence,
        Some(true)
    );
    assert_eq!(options.nonlin_transient_nox, Some(true));
    assert_eq!(
        options
            .output_interval_schedule
            .as_ref()
            .map(|schedule| schedule.initial_interval),
        Some(3.25e-7)
    );
    assert_eq!(options.output_snapshots, Some(true));
    assert_eq!(
        options.hb_time_domain_mode,
        Some(rspice_core::netlist::XyceHbTimeDomainMode::Direct)
    );

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
        .set_for_instance(
            AnalysisKind::Fourier,
            SolverOwnership::NONE,
            NumericOverrideOption::Reltol,
            "1e-5",
        )
        .expect("a Fourier measurement carries a Newton bound");
    record
        .set_for_instance(
            AnalysisKind::Fourier,
            SolverOwnership::NONE,
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
        .set_for_instance(
            AnalysisKind::Ac,
            SolverOwnership::NONE,
            NumericOverrideOption::Abstol,
            "4e-13",
        )
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
        .set_for_instance(
            AnalysisKind::Fourier,
            SolverOwnership::NONE,
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
        .set_for_instance(
            AnalysisKind::Fourier,
            SolverOwnership::NONE,
            NumericOverrideOption::Reltol,
            "1e-5",
        )
        .expect("authorable");
    record
        .set_for_instance(
            AnalysisKind::Fourier,
            SolverOwnership::NONE,
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
        .set_for_instance(
            AnalysisKind::Ac,
            SolverOwnership::NONE,
            NumericOverrideOption::GminStepping,
            "off",
        )
        .expect("every kind runs a DC solve");
    assert_eq!(record.to_spice_options(), ".OPTIONS\n+ GMINSTEPPING=0");
    assert!(!resolve(&record).convergence_config.gmin_stepping);
    assert_eq!(
        record.value(NumericOverrideOption::GminStepping).unwrap(),
        "off"
    );

    record
        .set_for_instance(
            AnalysisKind::Ac,
            SolverOwnership::NONE,
            NumericOverrideOption::GminStepping,
            "1",
        )
        .expect("the digit spelling is accepted too");
    assert!(resolve(&record).convergence_config.gmin_stepping);
}

/// `GMIN=0` is a request, not an empty field.
#[test]
fn a_zero_junction_conductance_floor_is_authorable_and_reaches_the_engine() {
    let mut record = AnalysisNumericOverride::default();
    record
        .set_for_instance(
            AnalysisKind::Ac,
            SolverOwnership::NONE,
            NumericOverrideOption::Gmin,
            "0",
        )
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
        .set_for_instance(
            AnalysisKind::Ac,
            SolverOwnership::NONE,
            NumericOverrideOption::Itl4,
            "12",
        )
        .expect_err("an AC sweep never takes a timestep");
    assert!(error.contains("ITL4"), "{error}");
    assert!(record.is_empty());

    let error = record
        .set_for_instance(
            AnalysisKind::Transient,
            SolverOwnership::NONE,
            NumericOverrideOption::MaximumTimestep,
            "1n",
        )
        .expect_err("the transient form owns its step ceiling");
    assert!(error.contains("Max step"), "{error}");

    let error = record
        .set_for_instance(
            AnalysisKind::OperatingPoint,
            SolverOwnership::NONE,
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
        NumericOverrideOption::TransientNewtonReltol,
        NumericOverrideOption::TransientNewtonAbstol,
        NumericOverrideOption::TransientNewtonUpdateBound,
        NumericOverrideOption::TransientNewtonResidualBound,
        NumericOverrideOption::TransientNewtonBudget,
        NumericOverrideOption::TransientDeviceConvergence,
        NumericOverrideOption::TransientNoxSolver,
        NumericOverrideOption::StrobeInterval,
        NumericOverrideOption::OutputTimePoints,
        NumericOverrideOption::RetainEverySignal,
    ] {
        assert_eq!(
            option.refusal_for(AnalysisKind::Ac),
            Some(catalog::NOT_TIME_STEPPED),
            "{} is only read on a time-stepped path",
            option.key()
        );
    }
    // The harmonic-balance package is read on no other family's solve, and the
    // refusal says which family reads it rather than "not applicable".
    for kind in [AnalysisKind::Ac, AnalysisKind::Transient] {
        assert_eq!(
            NumericOverrideOption::HbInitialState.refusal_for(kind),
            Some(catalog::NOT_HARMONIC_BALANCE),
            "{} does not run a harmonic-balance solve",
            kind.label()
        );
    }
    for kind in [
        AnalysisKind::HarmonicBalance,
        AnalysisKind::Hbsp,
        AnalysisKind::Hbnoise,
    ] {
        assert_eq!(
            NumericOverrideOption::HbInitialState.refusal_for(kind),
            None,
            "{} solves a harmonic-balance fixed point first",
            kind.label()
        );
    }
    // And a kind that does step carries all of them but the one the transient
    // form owns.
    let stepping =
        NumericOverrideOption::applicable_to_instance(AnalysisKind::Fourier, SolverOwnership::NONE);
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
            record
                .set_for_instance(kind, SolverOwnership::NONE, option, authored)
                .is_err(),
            "{option:?} must refuse {authored:?}"
        );
    }
    assert!(record.is_empty());
}

#[test]
fn a_restored_record_is_re_checked_against_its_kind() {
    let mut record = AnalysisNumericOverride::default();
    record
        .set_for_instance(
            AnalysisKind::Transient,
            SolverOwnership::NONE,
            NumericOverrideOption::Itl4,
            "12",
        )
        .expect("a transient takes timesteps");
    assert!(
        record
            .first_refusal_for_instance(AnalysisKind::Transient, SolverOwnership::NONE)
            .is_none()
    );
    let (option, _) = record
        .first_refusal_for_instance(AnalysisKind::Ac, SolverOwnership::NONE)
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
            (NumericOverrideOption::Reltol, "200u".to_owned()),
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
        .set_for_instance(
            AnalysisKind::Ac,
            SolverOwnership::NONE,
            NumericOverrideOption::Pivtol,
            "2.5e-14",
        )
        .expect("authorable");
    record
        .set_for_instance(
            AnalysisKind::Ac,
            SolverOwnership::NONE,
            NumericOverrideOption::Solver,
            "KLU",
        )
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

/// Every real an override states is spelled the way its preset is spelled.
///
/// The solver ledger puts the authored value beside the plan preset it departs
/// from, and only two of the options reached a shared spelling: a RELTOL of
/// 1e-4 was reported as "1e-4" against a preset of "1m", so a reader had to
/// convert one of the two numbers in their head before they could tell which
/// was tighter. `format_si_value` moves the decimal point rather than
/// dividing, so nothing is lost by using it for all of them.
#[test]
fn a_real_override_is_spelled_the_way_the_preset_beside_it_is() {
    for (authored, expected) in [
        ("1e-4", "100u"),
        ("2e-4", "200u"),
        ("1e-3", "1m"),
        ("4e-9", "4n"),
        ("1.5", "1.5"),
        ("2.5e6", "2.5Meg"),
    ] {
        let mut record = AnalysisNumericOverride::default();
        record
            .set_for_instance(
                AnalysisKind::Ac,
                SolverOwnership::NONE,
                NumericOverrideOption::Reltol,
                authored,
            )
            .expect("every kind carries an update bound");
        assert_eq!(
            record.value(NumericOverrideOption::Reltol).as_deref(),
            Some(expected),
            "{authored} is reported as {expected}"
        );
    }
}

/// The catalog reports every package table in the order it always has.
///
/// That order is observable: [`NumericOverrideOption::all`] is what the
/// ledger and the option picker walk. It is also not the order a naive
/// concatenation of the package tables would give — the four `TIMEINT` keys
/// report inside the integration section, not after the device-bypass one, and
/// the three packages added after them report inside their own sections rather
/// than at the end — so an assembly that appended instead of interleaving
/// would move a dozen rows with nothing else failing.
#[test]
fn the_catalog_order_survives_the_split_into_package_tables() {
    assert_eq!(
        NumericOverrideOption::all().collect::<Vec<_>>(),
        vec![
            NumericOverrideOption::Reltol,
            NumericOverrideOption::Abstol,
            NumericOverrideOption::Vntol,
            NumericOverrideOption::ResidualReltol,
            NumericOverrideOption::Gmin,
            NumericOverrideOption::Itl1,
            NumericOverrideOption::Itl4,
            NumericOverrideOption::GminStepping,
            NumericOverrideOption::SourceStepping,
            NumericOverrideOption::PseudoTransient,
            NumericOverrideOption::ArcLength,
            NumericOverrideOption::Damping,
            NumericOverrideOption::Chgtol,
            NumericOverrideOption::Trtol,
            NumericOverrideOption::IntegrationMethod,
            NumericOverrideOption::LteReltol,
            NumericOverrideOption::LteAbstol,
            NumericOverrideOption::MinTimestep,
            NumericOverrideOption::MaximumTimestep,
            NumericOverrideOption::TransientNewtonReltol,
            NumericOverrideOption::TransientNewtonAbstol,
            NumericOverrideOption::TransientNewtonUpdateBound,
            NumericOverrideOption::TransientNewtonResidualBound,
            NumericOverrideOption::TransientNewtonBudget,
            NumericOverrideOption::TransientDeviceConvergence,
            NumericOverrideOption::TransientNoxSolver,
            NumericOverrideOption::StrobeInterval,
            NumericOverrideOption::OutputTimePoints,
            NumericOverrideOption::RetainEverySignal,
            NumericOverrideOption::HbInitialState,
            NumericOverrideOption::Pivrel,
            NumericOverrideOption::Pivtol,
            NumericOverrideOption::Solver,
            NumericOverrideOption::Bypass,
            NumericOverrideOption::BypassReltol,
            NumericOverrideOption::BypassAbstol,
        ]
    );
}

/// Every package header the Studio writes is one the parser scopes.
///
/// The writer and the reader keep two lists of package names, and a header the
/// reader does not know is not an error — it is read as an ordinary global
/// key, so every key after it on that card lands in the global set. A typo of
/// `NONLIN-TRANS` would therefore resolve a transient Newton bound onto the
/// operating point's, silently. `option_package_key_is_known` is public for
/// exactly this check.
#[test]
fn every_engine_option_package_the_studio_writes_is_one_the_parser_scopes() {
    for package in OptionPackage::ALL {
        if package == OptionPackage::Global {
            assert_eq!(
                package.header(),
                ".OPTIONS",
                "the global set has no selector to name"
            );
            continue;
        }
        assert!(
            rspice_core::netlist::option_package_key_is_known(package.name()),
            "the parser does not scope `{}`, so every key on that card would be read as a global \
             one",
            package.name()
        );
        assert_eq!(
            package.header(),
            format!(".OPTIONS {}", package.name()),
            "a package's card header is its name"
        );
    }
}

/// One card per package, and a scoped card never re-scopes what follows it.
///
/// The parser's package selector stays in force for the rest of the command it
/// appears on. That is the trap the per-package cards exist to avoid, and it
/// is invisible in the emitted text: a record that put `TAHB` on the global
/// card would emit something that parses, and the global keys after it would
/// land in `HBINT` and be dropped.
#[test]
fn an_option_card_per_package_keeps_the_parsers_scope_from_leaking() {
    let mut record = AnalysisNumericOverride::default();
    // One key in each package, on a kind that carries all of them — and a
    // global key authored *last*, so an emitter that kept declaration order
    // instead of package order would put it after a scoped header.
    for (kind, option, authored) in [
        (
            AnalysisKind::Fourier,
            NumericOverrideOption::LteReltol,
            "4e-9",
        ),
        (
            AnalysisKind::Fourier,
            NumericOverrideOption::TransientNewtonUpdateBound,
            "0.125",
        ),
        (
            AnalysisKind::Fourier,
            NumericOverrideOption::RetainEverySignal,
            "on",
        ),
        (
            AnalysisKind::HarmonicBalance,
            NumericOverrideOption::HbInitialState,
            "DC operating point",
        ),
        (AnalysisKind::Fourier, NumericOverrideOption::Reltol, "1e-5"),
    ] {
        record
            .set_for_instance(kind, SolverOwnership::NONE, option, authored)
            .unwrap_or_else(|error| panic!("{} is authorable: {error}", option.key()));
    }

    let emitted = record.to_spice_options();
    assert_eq!(
        emitted,
        ".OPTIONS\n+ RELTOL=1e-5\n.OPTIONS TIMEINT\n+ RELTOL=4e-9\n.OPTIONS NONLIN-TRAN\n+ \
         DELTAXTOL=1.25e-1\n.OPTIONS OUTPUT\n+ SNAPSHOTS=1\n.OPTIONS HBINT\n+ TAHB=2"
    );
    // One card per package that has a stated key, and not one more.
    assert_eq!(
        emitted.matches(".OPTIONS").count(),
        5,
        "a package with a stated key gets exactly one card"
    );

    // And each key landed in its own package rather than in the one before it.
    let deck = format!("scope\nV1 1 0 1\nR1 1 0 1k\n{emitted}\n.op\n.end\n");
    let netlist = rspice_core::netlist::parse_netlist(&deck)
        .unwrap_or_else(|error| panic!("the per-package cards must parse: {error}\n{deck}"));
    let options = &netlist.options;
    assert_eq!(
        options.reltol,
        Some(1e-5),
        "the global RELTOL stayed global"
    );
    assert_eq!(
        options.timeint_reltol,
        Some(4e-9),
        "the TIMEINT RELTOL is a different bound and kept its own field"
    );
    assert_eq!(options.nonlin_transient_deltaxtol, Some(0.125));
    assert_eq!(options.output_snapshots, Some(true));
    assert_eq!(
        options.hb_time_domain_mode,
        Some(rspice_core::netlist::XyceHbTimeDomainMode::DcOperatingPoint)
    );
}

/// An authored strobe schedule changes which times a core run reports.
///
/// `OUTPUTTIMEPOINTS` stops are exact accepted solver points, so the proof is
/// the run's own time grid: every authored stop is a sample, and none of them
/// is merely near one. A tiny RC deck, run through the same engine entry point
/// the Studio's transient service calls.
#[test]
fn a_transient_strobe_interval_reaches_the_engine() {
    use rspice_core::engine::Engine;

    let mut record = AnalysisNumericOverride::default();
    record
        .set_for_instance(
            AnalysisKind::Transient,
            SolverOwnership::NONE,
            NumericOverrideOption::OutputTimePoints,
            "137u 651u 1.339m",
        )
        .expect("a transient owns its output schedule");

    let emitted = record.to_spice_options();
    assert_eq!(
        emitted,
        ".OPTIONS OUTPUT\n+ OUTPUTTIMEPOINTS=1.37e-4,6.51e-4,1.339e-3"
    );
    const CIRCUIT: &str = "strobe\nV1 1 0 1\nR1 1 2 1k\nC1 2 0 1u\n";
    let netlist =
        rspice_core::netlist::parse_netlist(&format!("{CIRCUIT}{emitted}\n.tran 100u 2m\n.end\n"))
            .unwrap_or_else(|error| panic!("the strobe deck must parse: {error}"));
    let unscheduled =
        rspice_core::netlist::parse_netlist(&format!("{CIRCUIT}.tran 100u 2m\n.end\n"))
            .expect("the unscheduled deck parses");

    let engine = Engine::new(SimulationConfig::default());
    let baseline = engine
        .run_tran(&unscheduled, 2.0e-3, 100.0e-6)
        .expect("the unscheduled transient runs");
    let scheduled = engine
        .run_tran(&netlist, 2.0e-3, 100.0e-6)
        .expect("the scheduled transient runs");

    // Exact accepted solver points, not interpolations: the engine adds each
    // authored stop to the breakpoint schedule, which is the whole reason this
    // key is a control rather than a post-processing preference.
    for stop in [1.37e-4, 6.51e-4, 1.339e-3] {
        assert!(
            scheduled
                .time
                .binary_search_by(|time| time.total_cmp(&stop))
                .is_ok(),
            "the authored stop {stop} is not an accepted sample: {:?}",
            scheduled.time
        );
    }
    assert_ne!(
        baseline.time, scheduled.time,
        "an authored schedule that left the sample times alone would not be a schedule"
    );
}

/// An authored HB initial state changes the record the HB engine reads.
///
/// Admitted under the fourth route, so the proof is the parsed package record
/// rather than a resolved configuration field: `resolve_simulation_config` has
/// no arm for `TAHB`, and `Engine::hb_config_for_netlist` reads
/// `netlist.options.hb_time_domain_mode` on the way into every HB solve.
#[test]
fn an_hb_integration_option_reaches_the_engine() {
    let baseline = resolve_and_parse(&AnalysisNumericOverride::default());
    assert_eq!(
        baseline.1.hb_time_domain_mode, None,
        "an empty record leaves the engine on its own DC seed"
    );

    for (authored, expected) in [
        ("0", rspice_core::netlist::XyceHbTimeDomainMode::Direct),
        (
            "Transient-assisted",
            rspice_core::netlist::XyceHbTimeDomainMode::TransientAssisted,
        ),
        (
            "DC operating point",
            rspice_core::netlist::XyceHbTimeDomainMode::DcOperatingPoint,
        ),
    ] {
        let mut record = AnalysisNumericOverride::default();
        record
            .set_for_instance(
                AnalysisKind::HarmonicBalance,
                SolverOwnership::NONE,
                NumericOverrideOption::HbInitialState,
                authored,
            )
            .unwrap_or_else(|error| panic!("{authored:?} is an authorable initial state: {error}"));
        let (resolved, options) = resolve_and_parse(&record);
        assert_eq!(
            options.hb_time_domain_mode,
            Some(expected),
            "{authored:?} must reach the record the HB engine reads"
        );
        assert_eq!(
            format!("{resolved:?}"),
            format!("{:?}", baseline.0),
            "TAHB reaches the engine without passing through SimulationConfig, which is the \
             fourth admission route and the reason this option needs it"
        );
    }
}

/// An output schedule is a strobe interval or a list of stops, never both.
///
/// The engine's own parser refuses a card carrying both keys, so a record that
/// accepted the pair would emit a deck that cannot be read — and the analysis
/// would be reported as a broken netlist rather than as an over-specified
/// schedule. Refused where the second value is authored, with the key to clear
/// named.
#[test]
fn an_output_schedule_is_a_strobe_or_a_stop_list_and_not_both() {
    let mut record = AnalysisNumericOverride::default();
    record
        .set_for_instance(
            AnalysisKind::Transient,
            SolverOwnership::NONE,
            NumericOverrideOption::StrobeInterval,
            "10u",
        )
        .expect("a transient owns its strobe interval");
    let error = record
        .set_for_instance(
            AnalysisKind::Transient,
            SolverOwnership::NONE,
            NumericOverrideOption::OutputTimePoints,
            "1u 2u",
        )
        .expect_err("two output schedules cannot both be stated");
    assert!(
        error.contains("OUTPUT INITIAL_INTERVAL") && error.contains("Clear"),
        "the refusal must name the key to clear: {error}"
    );
    assert_eq!(
        record.value(NumericOverrideOption::OutputTimePoints),
        None,
        "a refused value is not stored"
    );

    // Clearing the first one frees the other, and the record emits one key.
    record.clear(NumericOverrideOption::StrobeInterval);
    record
        .set_for_instance(
            AnalysisKind::Transient,
            SolverOwnership::NONE,
            NumericOverrideOption::OutputTimePoints,
            "1u 2u",
        )
        .expect("the schedule is free once the strobe interval is cleared");
    assert_eq!(
        record.to_spice_options(),
        ".OPTIONS OUTPUT\n+ OUTPUTTIMEPOINTS=1e-6,2e-6"
    );
}

/// A stop list is validated against the schedule the engine will build.
///
/// Each rule here is one the engine's breakpoint schedule enforces, so a list
/// this accepts is a list that run can use. Refusing them at authoring time is
/// the difference between a field that says what is wrong and a run that fails
/// with a netlist error three surfaces away.
#[test]
fn an_output_stop_list_refuses_what_the_engine_schedule_would() {
    let mut record = AnalysisNumericOverride::default();
    for authored in ["", "  ", "1u 1u", "2u 1u", "-1u", "1u soon"] {
        assert!(
            record
                .set_for_instance(
                    AnalysisKind::Transient,
                    SolverOwnership::NONE,
                    NumericOverrideOption::OutputTimePoints,
                    authored,
                )
                .is_err(),
            "{authored:?} is not a schedule the engine could run"
        );
    }
    assert!(record.is_empty());

    // Both separators, because the deck writes commas and the well reports
    // spaces, and a value has to survive the round trip through either.
    for authored in ["1u,2u,3u", "1u 2u 3u", " 1u, 2u 3u "] {
        record
            .set_for_instance(
                AnalysisKind::Transient,
                SolverOwnership::NONE,
                NumericOverrideOption::OutputTimePoints,
                authored,
            )
            .unwrap_or_else(|error| panic!("{authored:?} is a schedule: {error}"));
        assert_eq!(
            record
                .value(NumericOverrideOption::OutputTimePoints)
                .unwrap(),
            "1u 2u 3u"
        );
    }
}
