//! Manual-deck integration with application forms and engine services.

use super::{AnalysisSpec, AppState, NoiseSweepType, analysis_inputs};
use rspice_core::Netlist;
use rspice_simulation::manual_deck::build_manual_deck_queue;
use rspice_simulation_contract::{
    periodic_carrier::PeriodicCarrier as CarrierSelector, run_set::ReferencePoint,
};

#[test]
fn manual_deck_planning_has_no_panic_shortcuts() {
    for source in [
        include_str!("../../../../rspice-simulation/src/manual_deck.rs"),
        include_str!("../../../../rspice-simulation/src/manual_deck/periodic.rs"),
    ] {
        let production = crate::source_guard::production_source(source);
        for forbidden in [".expect(", ".unwrap(", "panic!(", "unreachable!("] {
            assert!(
                !production.contains(forbidden),
                "manual-deck production code contains panic shortcut {forbidden}"
            );
        }
    }
}

#[test]
fn studio_hb_card_retains_solver_controls_and_automatic_grid() {
    use crate::simulation::dialog::hb::{HbConfig, HbDialogState, HbSolverType};
    let config = HbConfig {
        fundamental_freq: 1000.0,
        num_harmonics: 5,
        fundamental_source: Some("V1".into()),
        maxiter: 42,
        damping: 0.5,
        min_damping: 0.02,
        oversample: 4,
        reltol: 2e-8,
        abstol: 3e-13,
        gmres_restart: 16,
        solver: HbSolverType::Krylov,
        source_stepping: true,
        use_exact_jacobian: false,
        verbose: true,
        ..Default::default()
    };
    let restored = HbDialogState::from_config(&config).to_config().unwrap();
    let specs = specs_for(&format!(
        "HB export\nV1 in 0 AC 1\nR1 in 0 1k\n{}\n.end\n",
        restored.to_spice()
    ));
    let [
        AnalysisSpec::HarmonicBalance {
            tones,
            reltol,
            abstol,
            max_iterations,
            damping,
            min_damping,
            oversample,
            collocation_points,
            use_krylov,
            gmres_restart,
            source_stepping,
            use_exact_jacobian,
            verbose,
            ..
        },
        AnalysisSpec::DcOp { .. } | AnalysisSpec::LegacyDcOp,
    ] = specs.as_slice()
    else {
        panic!("{specs:?}");
    };
    assert_eq!(tones[0].source.as_deref(), Some("V1"));
    assert_eq!(tones[0].harmonics, 5);
    assert_eq!((*reltol, *abstol, *max_iterations), (2e-8, 3e-13, 42));
    assert_eq!((*damping, *min_damping, *oversample), (0.5, 0.02, 4));
    assert_eq!(*collocation_points, None);
    assert!(*use_krylov && *source_stepping && !*use_exact_jacobian && *verbose);
    assert_eq!(*gmres_restart, 16);
}

#[test]
fn manual_noise_data_preserves_authored_axis_and_executes_row_contexts() {
    let state = AppState::default();
    let source = "noise data deck\n\
.param rload=1k\n\
V1 in 0 AC 1\n\
R1 in out 1k\n\
Rload out 0 {rload}\n\
.noise V(out) V1 DATA=points\n\
.DATA points\n\
+ rload HERTZ\n\
+ 1000 10\n\
+ 2000 1\n\
.ENDDATA\n\
.end\n";
    let queue = build_manual_deck_queue(state.sim_setup.reference_pvt.temperature_celsius, source)
        .expect("NOISE DATA queues");
    assert_eq!(queue.len(), 1);
    assert!(matches!(
        &queue[0].spec,
        AnalysisSpec::Noise {
            sweep: NoiseSweepType::ExplicitFrequencyList,
            explicit_frequencies: Some(frequencies),
            data_table_name: Some(table),
            ..
        } if frequencies == &[10.0, 1.0] && table.eq_ignore_ascii_case("points")
    ));
    let config = queue[0].config.as_ref().expect("exact config retained");
    let result = crate::simulation::EngineBridge::new()
        .run_with_abort(config, source, &rspice_core::abort_signal::NoAbort)
        .expect("NOISE DATA executes through the ordinary config path");
    assert!(matches!(
        result,
        crate::simulation::SimulationResult::Noise { frequencies, .. }
            if frequencies == vec![10.0, 1.0]
    ));
}

/// A harmonic-balance carrier is read, bound and run, on every card of the
/// family that can read one.
///
/// Core's `.PAC`, `.PNOISE` and `.PXF` grammars all admit `FROM=PSS|HB`,
/// the plan binds such a card to the deck's preceding `.HB`, and the
/// engine runs it through `Engine::run_pac_from_hb_with_abort` and its two
/// siblings. This reader used to refuse all three by name with "no route
/// in the Studio" — a limitation of this crate stated as a fact about the
/// card. The route exists now, so the test that pinned the refusal is
/// replaced by one that drives it: the same three cards, in both deck
/// shapes, read into a queue whose dependent carries a harmonic-balance
/// basis, and then run through the three service entries against a real
/// converged `.HB` operating point.
#[test]
fn an_hb_carrier_accepts_the_periodic_dependents_that_can_read_it() {
    use crate::services::simulation_runner::{
        self as svc, HbRunConfig, HbToneRunConfig, run_hb_analysis_with_source_path_and_abort,
    };
    use rspice_core::abort_signal::NoAbort;

    const CIRCUIT: &str =
        "periodic\nV1 in 0 SIN(0 0.001 1Meg) AC 1\nR1 in out 1k\nC1 out 0 159.154943091895p\n";
    const HB: &str = ".hb 1Meg\n";
    const FUNDAMENTAL: f64 = 1.0e6;

    // One converged carrier for all three runs, as the plan hands the one
    // artifact to every dependent bound to that instance.
    let carrier_deck = format!("{CIRCUIT}.end\n");
    let operating_point = run_hb_analysis_with_source_path_and_abort(
        &carrier_deck,
        &HbRunConfig {
            tones: vec![HbToneRunConfig::new(FUNDAMENTAL, 12)],
            reltol: 1.0e-10,
            ..HbRunConfig::default()
        },
        None,
        &NoAbort,
    )
    .expect("the harmonic-balance carrier converges")
    .operating_point;

    for card in [
        ".pxf dec 10 1k 1Meg input=V1 out=out from=hb",
        ".pac dec 10 1k 1Meg input=V1 out=out from=hb",
        ".pnoise dec 10 1k 1Meg out=out from=hb",
    ] {
        for seed in ["", ".pss fund=1Meg\n"] {
            let source = format!("{CIRCUIT}{HB}{seed}{card}\n.end\n");
            let netlist = Netlist::parse(&source).unwrap_or_else(|error| {
                panic!(
                    "the engine accepts `{card}`; this case no longer tests what it claims: {error}"
                )
            });
            rspice_core::execution::DeckPlan::from_netlist(
                &netlist,
                &rspice_core::resource::ResourceLimits::default(),
            )
            .unwrap_or_else(|error| {
                panic!("the engine binds `{card}` to the deck's .HB carrier: {error}")
            });

            let tasks =
                build_manual_deck_queue(ReferencePoint::default().temperature_celsius, &source)
                    .unwrap_or_else(|errors| panic!("`{card}` must read: {errors:?}"));
            let dependent = tasks
                .iter()
                .find(|task| {
                    matches!(
                        task.spec,
                        AnalysisSpec::Pac | AnalysisSpec::Pxf | AnalysisSpec::Pnoise
                    )
                })
                .unwrap_or_else(|| panic!("`{card}` queues a dependent task"));
            let options = &dependent.spec_options;
            // Bound to the harmonic-balance basis, not to the `.PSS`
            // that may be sitting beside it.
            let basis = options
                .pac
                .as_ref()
                .map(|config| (config.carrier, config.pss_fundamental_freq))
                .or_else(|| {
                    options
                        .pxf
                        .as_ref()
                        .map(|config| (config.carrier, config.pss_fundamental_freq))
                })
                .or_else(|| {
                    options
                        .pnoise
                        .as_ref()
                        .map(|config| (config.carrier, config.pss_fundamental_freq))
                })
                .unwrap_or_else(|| panic!("`{card}` freezes its run configuration"));
            assert_eq!(basis.0, CarrierSelector::Hb, "`{card}`");
            assert!(
                (basis.1 - FUNDAMENTAL).abs() < 1.0,
                "`{card}` must carry the .HB fundamental, got {}",
                basis.1
            );

            // And it runs. The service entries below are the ones the
            // dispatch calls once the plan has handed over the artifact.
            if let Some(config) = options.pac.as_ref() {
                let data = svc::run_pac_analysis_from_hb_with_source_path_and_abort(
                    &carrier_deck,
                    config,
                    operating_point.as_ref(),
                    None,
                    &NoAbort,
                )
                .unwrap_or_else(|error| panic!("`{card}` must run: {error}"));
                assert!(!data.frequencies.is_empty(), "`{card}`");
                assert!(!data.traces.is_empty(), "`{card}`");
            } else if let Some(config) = options.pxf.as_ref() {
                let data = svc::run_pxf_analysis_from_hb_with_source_path_and_abort(
                    &carrier_deck,
                    config,
                    operating_point.as_ref(),
                    None,
                    &NoAbort,
                )
                .unwrap_or_else(|error| panic!("`{card}` must run: {error}"));
                assert!(!data.transfer.is_empty(), "`{card}`");
            } else if let Some(config) = options.pnoise.as_ref() {
                let data = svc::run_pnoise_analysis_from_hb_with_source_path_and_abort(
                    &carrier_deck,
                    config,
                    operating_point.as_ref(),
                    None,
                    &NoAbort,
                )
                .unwrap_or_else(|error| panic!("`{card}` must run: {error}"));
                assert!(
                    data.output_noise.iter().all(|value| *value > 0.0),
                    "`{card}` must publish a positive spectrum"
                );
            } else {
                panic!("`{card}` queues one of the three typed configurations");
            }
        }
    }
}

fn specs_for(source: &str) -> Vec<AnalysisSpec> {
    let state = AppState::default();
    build_manual_deck_queue(state.sim_setup.reference_pvt.temperature_celsius, source)
        .expect("manual deck queue")
        .into_iter()
        .map(|q| q.spec)
        .collect()
}

#[test]
fn dc_mismatch_moment_controls_survive_authoring_storage_decks_and_workers() {
    use crate::simulation::plan::{AnalysisDraft, DcMismatchDraft};
    use crate::simulation::runner::worker_contract::WorkerAnalysisSpec;

    let draft = DcMismatchDraft {
        moment_relative_tolerance: "2m".into(),
        moment_max_points: "131072".into(),
        output_expression: "V(OUT)".into(),
        ..Default::default()
    };
    let saved = serde_json::to_value(&draft).unwrap();
    let restored: DcMismatchDraft = serde_json::from_value(saved.clone()).unwrap();
    assert_eq!(restored.moment_relative_tolerance, "2m");
    let spec = rspice_simulation::analysis_preparation::analysis_draft_spec(
        &analysis_inputs(&AppState::default()),
        &AnalysisDraft::DcMismatch(restored),
    )
    .unwrap();
    let AnalysisSpec::DcMismatch { moment_options, .. } = &spec else {
        panic!("DC mismatch specification expected");
    };
    assert_eq!(moment_options.relative_tolerance, 0.002);
    assert_eq!(moment_options.max_points, 131_072);
    let command =
        rspice_simulation::analysis_preparation::build_dc_mismatch_command(&spec).unwrap();
    let deck = format!("controls\nV1 in 0 1\nR1 in out 1k\nR2 out 0 1k\n{command}\n.end\n");
    assert_eq!(specs_for(&deck), vec![spec.clone()]);
    let worker = WorkerAnalysisSpec::try_from(&spec).unwrap();
    let carried: WorkerAnalysisSpec =
        serde_json::from_str(&serde_json::to_string(&worker).unwrap()).unwrap();
    assert_eq!(AnalysisSpec::from(carried), spec);

    // Old saved drafts/specifications retain the engine's default policy.
    let mut legacy = saved;
    legacy
        .as_object_mut()
        .unwrap()
        .remove("moment_relative_tolerance");
    legacy.as_object_mut().unwrap().remove("moment_max_points");
    assert_eq!(
        serde_json::from_value::<DcMismatchDraft>(legacy)
            .unwrap()
            .moment_options()
            .unwrap(),
        Default::default()
    );
    let mut legacy = serde_json::to_value(&spec).unwrap();
    legacy["DcMismatch"]
        .as_object_mut()
        .unwrap()
        .remove("moment_options");
    let legacy: AnalysisSpec = serde_json::from_value(legacy).unwrap();
    assert!(
        !rspice_simulation::analysis_preparation::build_dc_mismatch_command(&legacy)
            .unwrap()
            .contains("MOMENT_")
    );

    for (tolerance, points) in [
        ("0", "131072"),
        ("0.2", "131072"),
        ("2m", "1023"),
        ("2m", "1.5"),
    ] {
        let invalid = DcMismatchDraft {
            moment_relative_tolerance: tolerance.into(),
            moment_max_points: points.into(),
            ..draft.clone()
        };
        assert!(
            AnalysisDraft::DcMismatch(invalid)
                .manifest_configuration_error()
                .is_some()
        );
        let invalid_deck = format!(
            "invalid\nV1 in 0 1\nR1 in 0 1k\n.DCMATCH OUT=V(in) MOMENT_RELTOL={tolerance} MOMENT_MAX_POINTS={points}\n.end\n"
        );
        assert!(rspice_core::netlist::Netlist::parse(&invalid_deck).is_err());
    }
}

/// A bare card reads as the engine's own defaults, not as this crate's.
#[test]
fn a_bare_dcmatch_card_reads_as_the_engine_defaults() {
    let specs = specs_for(
        "bare mismatch\n\
             V1 in 0 DC 1\n\
             R1 in out 10k\n\
             R2 out 0 10k\n\
             .DCMATCH OUT=V(out)\n\
             .end\n",
    );
    let [
        AnalysisSpec::DcMismatch {
            sigma_multiplier,
            contributor_limit,
            include_process,
            include_mismatch,
            contribution_threshold,
            ..
        },
    ] = specs.as_slice()
    else {
        panic!("a bare .DCMATCH card plans one analysis: {specs:?}");
    };
    assert_eq!(*sigma_multiplier, 1.0);
    assert_eq!(
        *contributor_limit,
        rspice_core::netlist::DcMatchCard::DEFAULT_CONTRIBUTORS
    );
    assert!(*include_mismatch);
    assert!(!*include_process);
    assert_eq!(
        *contribution_threshold, None,
        "an unstated threshold is the unauthored card, not a stated zero"
    );

    // And a fresh Studio draft is that same specification, which is the
    // reason the draft's defaults are the card's.
    let state = AppState::default();
    let draft = crate::simulation::plan::AnalysisDraft::for_kind(
        crate::simulation::plan::AnalysisKind::DcMismatch,
    );
    let authored = rspice_simulation::analysis_preparation::analysis_draft_spec(
        &analysis_inputs(&state),
        &draft,
    )
    .expect("a default DC mismatch draft builds a specification");
    let AnalysisSpec::DcMismatch {
        sigma_multiplier: draft_sigma,
        contributor_limit: draft_limit,
        include_process: draft_process,
        include_mismatch: draft_mismatch,
        contribution_threshold: draft_threshold,
        ..
    } = authored
    else {
        panic!("the draft builds a DC mismatch specification");
    };
    assert_eq!(draft_sigma, *sigma_multiplier);
    assert_eq!(draft_limit, *contributor_limit);
    assert_eq!(draft_process, *include_process);
    assert_eq!(draft_mismatch, *include_mismatch);
    assert_eq!(draft_threshold, *contribution_threshold);
}
