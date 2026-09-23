//! Manual-card import and per-instance execution contracts, including configuration round trips.

use super::*;

#[test]
fn manual_deck_planning_has_no_panic_shortcuts() {
    for source in [
        include_str!("../manual_deck.rs"),
        include_str!("periodic.rs"),
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
use crate::services::simulation_runner::CornerBaseMode;
use crate::simulation::multi_run::FrequencySweep;

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
fn studio_sp_card_retains_differential_ports_and_effective_impedances() {
    use crate::simulation::dialog::sp::{SpConfig, SpPortConfig};
    let config = SpConfig {
        z0: 75.0,
        ports: vec![
            SpPortConfig::single_ended(1, "in"),
            SpPortConfig {
                number: 2,
                node_pos: "out".into(),
                node_neg: "ref".into(),
                z0: Some(100.0),
            },
        ],
        do_noise: true,
        ..Default::default()
    };
    let specs = specs_for(&format!(
        "SP export\nR1 in out 100\nR2 ref 0 50\n{}\n.end\n",
        config.to_spice()
    ));
    let [
        AnalysisSpec::SParameter {
            ports, do_noise, ..
        },
    ] = specs.as_slice()
    else {
        panic!("{specs:?}");
    };
    assert_eq!(ports.len(), 2);
    assert_eq!(ports[0].z0, Some(75.0));
    assert_eq!(ports[1].z0, Some(100.0));
    assert!(ports[1].node_neg.eq_ignore_ascii_case("ref"));
    assert!(*do_noise);
}

#[test]
fn studio_stb_card_retains_disabled_nyquist() {
    let config = crate::simulation::dialog::stb::StbConfig {
        compute_nyquist: false,
        ..Default::default()
    };
    let specs = specs_for(&format!(
        "STB export\nV1 in 0 1\nR1 in out 1k\nLSTB out load 1n\nR2 load 0 1k\n{}\n.end\n",
        config.to_spice()
    ));
    assert!(matches!(
        &specs[0],
        AnalysisSpec::Stb {
            compute_nyquist: false,
            ..
        }
    ));
}

#[test]
fn manual_fourier_retains_periods_and_explicit_window() {
    let specs = specs_for(
        "Fourier window\nV1 in 0 SIN(0 1 1k)\nR1 in 0 1k\n.tran 1u 10m\n.four 1k 5 V(in) PERIODS=3 FROM=2m TO=8m\n.end\n",
    );
    assert!(
        specs
            .iter()
            .any(|spec| matches!(spec, AnalysisSpec::Fourier {
            num_periods: 3, start_time, stop_time, ..
        } if *start_time == 0.002 && *stop_time == 0.008))
    );
}

fn specs_for(source: &str) -> Vec<AnalysisSpec> {
    let state = AppState::default();
    build_manual_deck_queue(&state, source)
        .expect("manual deck queue")
        .into_iter()
        .map(|q| q.spec)
        .collect()
}

/// A hand-written `.TRAN` card with noise keywords is a transient-noise
/// analysis, not an ordinary transient with a request quietly discarded.
///
/// The keywords land in the deck's options rather than on the command, so
/// nothing in the command this reader matches on says the run is noisy.
/// Before this arm existed the deck planned a plain transient, ran without
/// noise, and reported no difference — the run the author asked for was
/// simply not the run that happened.
#[test]
fn a_manual_deck_with_noisefmax_is_read_as_transient_noise() {
    let specs = specs_for(
        "noisy deck\n\
             V1 in 0 DC 1\n\
             R1 in out 10k\n\
             R2 out 0 10k\n\
             .tran 1n 1u 0 2n NOISEFMAX=5e8 NOISEFMIN=1k NOISESEED=97 NOISESCALE=0.5\n\
             .end\n",
    );
    let [
        AnalysisSpec::TransientNoise {
            stop_time,
            step_time,
            start_time,
            max_timestep,
            seed,
            noise_fmax,
            noise_fmin,
            scale,
            uic,
        },
    ] = specs.as_slice()
    else {
        panic!("a noisy .tran card plans one transient-noise analysis: {specs:?}");
    };
    assert_eq!(*stop_time, 1.0e-6);
    assert_eq!(*step_time, 1.0e-9);
    assert_eq!(*start_time, 0.0);
    assert_eq!(*max_timestep, 2.0e-9);
    assert_eq!(*seed, Some(97));
    assert_eq!(*noise_fmax, 5.0e8);
    assert_eq!(*noise_fmin, Some(1.0e3));
    assert_eq!(*scale, 0.5);
    assert!(!*uic);

    // The same deck without the keywords is still the ordinary transient
    // it always was, so this arm reads a request rather than reclassifying
    // every `.tran` card in the corpus.
    let plain = specs_for(
        "plain deck\n\
             V1 in 0 DC 1\n\
             R1 in out 10k\n\
             R2 out 0 10k\n\
             .tran 1n 1u\n\
             .end\n",
    );
    assert!(
        matches!(plain.as_slice(), [AnalysisSpec::Transient { .. }]),
        "{plain:?}"
    );
}

/// The card the Studio writes, read back by the reader a hand-written deck
/// goes through, is the specification it was written from.
///
/// Two routes reach one run: the typed plan, and a deck someone was handed.
/// They have to agree, or a deck exported from the Studio and re-opened is
/// a different analysis from the one that produced it — with the same name
/// on it. Every field is compared, because the fields that would go missing
/// silently are exactly the ones the card spells as optional keywords.
#[test]
fn the_studio_card_round_trips_through_the_manual_deck_reader() {
    use crate::simulation::controller::SimulationController;

    for noise_fmin in [None, Some(1.0e3)] {
        for scale in [1.0, 0.5, 0.0] {
            let authored = AnalysisSpec::TransientNoise {
                stop_time: 1.0e-6,
                step_time: 1.0e-9,
                start_time: 2.0e-7,
                max_timestep: 2.5e-10,
                seed: Some(0),
                noise_fmax: 5.0e8,
                noise_fmin,
                scale,
                uic: false,
            };
            let card = SimulationController::build_transient_noise_command(&authored)
                .expect("the plan writes its card");
            let specs = specs_for(&format!(
                "round trip\n\
                     V1 in 0 DC 1\n\
                     R1 in out 10k\n\
                     R2 out 0 10k\n\
                     {card}\n\
                     .end\n"
            ));
            assert_eq!(
                specs.as_slice(),
                &[authored],
                "the card `{card}` read back as something else"
            );
        }
    }
}

/// A deck someone wrote by hand, with its own table name and its own row
/// order, opens as the frequency-table analysis on exactly that axis.
///
/// The two halves that could go wrong independently: the card has to be
/// recognized as this kind rather than as a graded `.ac`, and the axis has
/// to come from the deck's table rather than from any default — including
/// when the table is not sorted, which is the author's choice to make and
/// not a list this reader may reorder.
#[test]
fn a_hand_written_ac_data_deck_is_read_as_the_frequency_table_analysis() {
    let specs = specs_for(
        "hand written ac data\n\
             I1 out 0 AC 1\n\
             R1 out 0 1k\n\
             .ac data=measured\n\
             .data measured\n\
             + FREQ\n\
             + 1k\n\
             + 100\n\
             + 10k\n\
             .enddata\n\
             .print ac V(out)\n\
             .end\n",
    );
    let [
        AnalysisSpec::AcData {
            table_name,
            frequencies,
            ..
        },
    ] = specs.as_slice()
    else {
        panic!("a hand-written `.ac data=` deck is one frequency-table analysis: {specs:?}");
    };
    assert!(table_name.eq_ignore_ascii_case("measured"));
    assert_eq!(frequencies.as_slice(), [1.0e3, 100.0, 1.0e4]);
}

/// The card the Studio writes for a stated axis, read back by the reader a
/// hand-written deck goes through, is the specification it came from.
#[test]
fn the_studio_ac_frequency_table_cards_round_trip_through_the_manual_deck_reader() {
    use crate::simulation::controller::SimulationController;

    let authored = AnalysisSpec::AcData {
        table_name: crate::simulation::config::AC_FREQUENCY_TABLE.to_owned(),
        frequencies: vec![37.0, 74.0, 148.5],
        table_options: Default::default(),
    };
    let cards = SimulationController::build_ac_data_command(&authored)
        .expect("the plan writes its card and its table");
    let specs = specs_for(&format!(
        "round trip\n\
             I1 out 0 AC 1\n\
             R1 out 0 1k\n\
             {cards}\n\
             .end\n"
    ));
    let [
        AnalysisSpec::AcData {
            table_name,
            frequencies,
            ..
        },
    ] = specs.as_slice()
    else {
        panic!("the cards `{cards}` read back as something else: {specs:?}");
    };
    let AnalysisSpec::AcData {
        table_name: authored_table,
        frequencies: authored_frequencies,
        ..
    } = &authored
    else {
        unreachable!("the fixture is a frequency-table specification");
    };
    // The axis round-trips exactly. The table name round-trips up to case:
    // the parser canonicalizes an identifier to upper case and the
    // engine's table lookup is case-insensitive, so both spellings name
    // the same table.
    assert_eq!(frequencies, authored_frequencies, "{cards}");
    assert!(
        table_name.eq_ignore_ascii_case(authored_table),
        "the cards `{cards}` came back naming '{table_name}'"
    );
}

/// A deck that says nothing about the seed carries the seed the engine
/// will actually play.
///
/// The reader cannot leave it unresolved: the specification's seed is what
/// the form shows and what the Studio's own card would write, so a plan
/// read from a silent deck has to name the realization that deck ran. The
/// value is proved rather than asserted — two runs, one silent about the
/// seed and one stating the resolved constant, must produce bit-identical
/// waveforms. That is what ties this crate's constant to the engine's
/// without exporting it.
#[test]
fn a_deck_without_a_noise_seed_carries_the_engine_resolved_default() {
    const SILENT: &str = "silent seed\n\
                              V1 in 0 DC 1\n\
                              R1 in out 10k\n\
                              R2 out 0 10k\n\
                              .tran 1n 1u NOISEFMAX=1e9\n\
                              .end\n";

    let specs = specs_for(SILENT);
    let [AnalysisSpec::TransientNoise { seed, .. }] = specs.as_slice() else {
        panic!("a silent-seed noisy card still plans transient noise: {specs:?}");
    };
    assert_eq!(*seed, Some(ENGINE_DEFAULT_NOISE_SEED));

    // The engine's own answer, measured: the same deck with this seed
    // stated must play the realization the silent deck played.
    let run = |deck: &str| -> Vec<u64> {
        let netlist = rspice_core::Netlist::parse(deck).expect("the deck parses");
        let result = rspice_core::Engine::new(rspice_core::SimulationConfig::default())
            .run_tran(&netlist, 1.0e-6, 1.0e-9)
            .expect("the noisy transient converges");
        let node = result
            .node_names
            .iter()
            .position(|name| name.eq_ignore_ascii_case("out"))
            .expect("the divider's output node is solved");
        result.voltages[node].iter().map(|v| v.to_bits()).collect()
    };
    let stated = SILENT.replace(
        "NOISEFMAX=1e9",
        &format!("NOISEFMAX=1e9 NOISESEED={ENGINE_DEFAULT_NOISE_SEED}"),
    );
    assert_eq!(
        run(SILENT),
        run(&stated),
        "this crate's default seed is not the one the engine resolves"
    );
}

#[test]
fn manual_deck_preserves_common_analysis_order() {
    let specs =
        specs_for("deck\nR1 out 0 1k\nV1 out 0 1 AC 1\n.op\n.ac dec 20 1 1g\n.tran 1n 1u\n.end\n");

    assert!(matches!(specs[0], AnalysisSpec::DcOp { .. }));
    assert!(matches!(
        specs[1],
        AnalysisSpec::Ac {
            start_freq,
            stop_freq,
            points_per_unit: 20,
            sweep: FrequencySweep::Decade
        } if (start_freq - 1.0).abs() < 1e-12 && (stop_freq - 1e9).abs() < 1.0
    ));
    assert!(matches!(
        specs[2],
        AnalysisSpec::Transient {
            step_time,
            stop_time,
            start_time,
            max_timestep: None,
            uic: false
        } if (step_time - 1e-9).abs() < 1e-21
            && (stop_time - 1e-6).abs() < 1e-18
            && start_time == 0.0
    ));
}

#[test]
fn manual_periodic_deck_owns_implicit_seed_and_typed_analysis_options() {
    let state = AppState::default();
    let queue = build_manual_deck_queue(
            &state,
            "periodic\nV1 in 0 SIN(0 1 1Meg)\nR1 in out 1k\nC1 out 0 1n\n.pss fund=1Meg points=128 harms=8\n.pac dec 20 1k 100Meg input=V1 out=out maxsideband=4\n.pnoise dec 10 1 1Meg out=out\n.end\n",
        )
        .expect("manual periodic queue");

    assert_eq!(queue.len(), 5);
    assert!(matches!(queue[0].spec, AnalysisSpec::DcOp { .. }));
    assert_eq!(queue[0].analysis_line, ".op (implicit PSS seed)");
    assert!(matches!(queue[1].spec, AnalysisSpec::Pss { .. }));
    assert!(matches!(
        queue[2].spec,
        AnalysisSpec::PssSpectrum { num_harmonics: 8 }
    ));
    assert!(queue[3].spec_options.pac.is_some());
    assert!(queue[4].spec_options.pnoise.is_some());
}

#[test]
fn manual_fourier_inherits_the_exact_transient_window() {
    let specs = specs_for(
        "Fourier deck\nV1 out 0 SIN(0 1 1k)\nR1 out 0 1k\n.four 1k V(out)\n.tran 10u 5m 1m\n.end\n",
    );

    assert!(matches!(
        specs[0],
        AnalysisSpec::Fourier {
            start_time,
            stop_time,
            ..
        } if (start_time - 1.0e-3).abs() < 1.0e-15
            && (stop_time - 5.0e-3).abs() < 1.0e-15
    ));
}

#[test]
fn manual_fourier_preserves_each_voltage_output_and_reference() {
    let specs = specs_for(
        "Fourier deck\nV1 in 0 SIN(0 1 1k)\nR1 in out 1k\nR2 out 0 1k\n.four 1k V(out) V(out,in)\n.tran 10u 5m\n.end\n",
    );

    assert_eq!(specs.len(), 3);
    assert!(matches!(
        &specs[0],
        AnalysisSpec::Fourier {
            output_node,
            output_ref,
            ..
        } if output_node.eq_ignore_ascii_case("out")
            && (output_ref == "0" || output_ref.eq_ignore_ascii_case("gnd"))
    ));
    assert!(matches!(
        &specs[1],
        AnalysisSpec::Fourier {
            output_node,
            output_ref,
            ..
        } if output_node.eq_ignore_ascii_case("out")
            && output_ref.eq_ignore_ascii_case("in")
    ));
}

#[test]
fn manual_fourier_preserves_a_branch_current_output() {
    let specs = specs_for(
        "Fourier current deck\nV1 out 0 SIN(0 1 1k)\nR1 out 0 1k\n.four 1k I(V1)\n.tran 10u 5m\n.end\n",
    );

    assert!(matches!(
        &specs[0],
        AnalysisSpec::Fourier {
            output_node,
            output_ref,
            ..
        } if output_node.eq_ignore_ascii_case("I(V1)") && output_ref.is_empty()
    ));
}

#[test]
fn manual_fourier_preserves_an_explicit_harmonic_count() {
    let specs = specs_for(
        "Fourier harmonic count\nV1 out 0 SIN(0 1 60)\nR1 out 0 1k\n.four 60 15 I(V1)\n.tran 100u 50m\n.end\n",
    );

    assert!(matches!(
        &specs[0],
        AnalysisSpec::Fourier {
            num_harmonics: 15,

            num_periods: 1,
            output_node,
            ..
        } if output_node.eq_ignore_ascii_case("I(V1)")
    ));
}

#[test]
fn manual_fourier_current_output_fails_closed_before_transient_when_unavailable() {
    let state = AppState::default();
    let semiconductor = build_manual_deck_queue(
            &state,
            "Fourier diode current\nV1 in 0 SIN(0 1 1k)\nD1 in 0 DMOD\n.model DMOD D\n.four 1k I(D1)\n.tran 10u 5m\n.end\n",
        )
        .expect_err("an unavailable semiconductor terminal current must fail preflight");
    assert!(
        semiconductor
            .join("; ")
            .contains("typed terminal-current selector"),
        "{semiconductor:?}"
    );

    let missing = build_manual_deck_queue(
        &state,
        "Fourier missing current\nV1 in 0 SIN(0 1 1k)\n.four 1k I(R404)\n.tran 10u 5m\n.end\n",
    )
    .expect_err("a missing branch identity must fail preflight");
    assert!(missing.join("; ").contains("does not name a top-level"));
}

#[test]
fn manual_fourier_rejects_missing_or_ambiguous_transient_producers() {
    let state = AppState::default();
    let missing = build_manual_deck_queue(
        &state,
        "Fourier deck\nV1 out 0 SIN(0 1 1k)\nR1 out 0 1k\n.four 1k V(out)\n.end\n",
    )
    .expect_err(".FOUR without .TRAN must fail closed");
    assert!(missing.join("; ").contains("exactly one .TRAN"));

    let ambiguous = build_manual_deck_queue(
            &state,
            "Fourier deck\nV1 out 0 SIN(0 1 1k)\nR1 out 0 1k\n.tran 10u 1m\n.tran 20u 2m\n.four 1k V(out)\n.end\n",
        )
        .expect_err(".FOUR with multiple .TRAN producers must fail closed");
    assert!(ambiguous.join("; ").contains("found 2"));
}

#[test]
fn manual_deck_maps_hbint_order_to_exact_collocation_grid() {
    let specs = specs_for(
        "HB deck\nV1 in 0 PULSE(0 1 0 1n 1n 4n 10n)\nR1 in 0 1k\n.hb 100meg\n.options hbint numfreq=12\n.end\n",
    );

    assert!(matches!(
        &specs[0],
        AnalysisSpec::HarmonicBalance {
            tones,
            collocation_points: Some(25),
            ..
        } if tones.len() == 1
            && tones[0].harmonics == 12
            && (tones[0].frequency - 100.0e6).abs() < 1.0
    ));
}

#[test]
fn manual_deck_ac_data_uses_table_frequencies() {
    let state = AppState::default();
    let queue = build_manual_deck_queue(
        &state,
        "deck\n\
             I1 out 0 AC 1\n\
             R1 out 0 1k\n\
             .ac data=pts\n\
             .data pts\n\
             + FREQ\n\
             + 10\n\
             + 1\n\
             + 2.5\n\
             .enddata\n\
             .end\n",
    )
    .expect("manual AC DATA deck should queue");

    assert_eq!(queue.len(), 1);
    assert!(queue[0].config.is_none());
    assert!(queue[0].analysis_line.eq_ignore_ascii_case(".ac data=pts"));
    let AnalysisSpec::AcData {
        table_name,
        frequencies,
        ..
    } = &queue[0].spec
    else {
        panic!("expected AC DATA analysis");
    };
    assert!(table_name.eq_ignore_ascii_case("pts"));
    assert_eq!(frequencies.as_slice(), [10.0, 1.0, 2.5]);
}

#[test]
fn manual_deck_ac_data_requires_freq_column() {
    let state = AppState::default();
    let err = build_manual_deck_queue(
        &state,
        "deck\n\
             I1 out 0 AC 1\n\
             R1 out 0 1k\n\
             .ac data=pts\n\
             .data pts\n\
             + TEMP\n\
             + 25\n\
             .enddata\n\
             .end\n",
    )
    .expect_err("AC DATA table without FREQ should be rejected");

    assert!(
        err.iter().any(|message| message.contains("FREQ")),
        "expected FREQ diagnostic, got {err:?}"
    );
}

#[test]
fn manual_deck_sp_uses_rf_port_annotations() {
    let specs = specs_for(
        "deck\n\
             V2 out 0 dc 0 ac 1 portnum 2 z0 75\n\
             V1 in 0 dc 0 ac 1 portnum 1 z0 50\n\
             R1 in out 100\n\
             .sp lin 3 1Meg 3Meg\n\
             .end\n",
    );

    let AnalysisSpec::SParameter {
        start_freq,
        stop_freq,
        points_per_unit,
        sweep,
        z0,
        ports,
        do_noise,
    } = &specs[0]
    else {
        panic!("expected S-parameter analysis");
    };

    assert_eq!(*points_per_unit, 3);
    assert_eq!(*sweep, FrequencySweep::Linear);
    assert!((*start_freq - 1.0e6).abs() < 1e-6);
    assert!((*stop_freq - 3.0e6).abs() < 1e-6);
    assert_eq!(*z0, 50.0);
    assert!(ports.is_empty(), "authored ports resolve during execution");
    assert!(!do_noise);
    assert!(specs[0].validate().is_ok());
    let hierarchical = specs_for(
        "deck\n.subckt generator a b params: reference=75\nP1 a b portnum=1 z0={reference}\n.ends generator\nX1 p 0 generator\nR1 p 0 100\n.sp lin 3 1Meg 3Meg\n.end\n",
    );
    assert!(matches!(&hierarchical[0], AnalysisSpec::SParameter { ports, .. } if ports.is_empty()));
    assert!(hierarchical[0].validate().is_ok());
    let noisy = specs_for("deck\nP1 p 0 portnum=1\nR1 p 0 100\n.sp lin 3 1Meg 3Meg 1\n.end\n");
    assert!(matches!(
        noisy[0],
        AnalysisSpec::SParameter { do_noise: true, .. }
    ));
}

#[test]
fn temp_command_queue_fallback_reports_error_without_panicking() {
    let state = AppState::default();
    let netlist = Netlist::default();
    let err = command_to_queue_item(
        &state,
        &netlist,
        &AnalysisCommand::Temp {
            temperatures: vec![25.0],
        },
    )
    .expect_err(".temp fallback should be a recoverable queueing error");

    assert!(err.contains(".temp"));
    assert!(err.contains("temperature sweeps"));
}

#[test]
fn manual_deck_dc_and_noise_build_configs_without_dialog_state() {
    let state = AppState::default();
    let queue = build_manual_deck_queue(
        &state,
        "deck\nV1 in 0 0 AC 1\nR1 in out 1k\n.ac lin 5 1 5\n.noise v(out) V1 dec 10 1 1e6\n.end\n",
    )
    .expect("queue builds");

    assert!(matches!(queue[0].spec, AnalysisSpec::Ac { .. }));
    assert!(matches!(queue[1].config, Some(AnalysisConfig::Noise(_))));
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
    let queue = build_manual_deck_queue(&state, source).expect("NOISE DATA queues");
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

#[test]
fn manual_deck_mc_and_step_are_runnable_specs() {
    let state = AppState::default();
    let queue = build_manual_deck_queue(
            &state,
            "deck\n.param rload=1k\nV1 in 0 1\nR1 in out {rload}\nR2 out 0 1k\n.step param rload 500 1500 500\n.mc 8 seed 7 dist uniform spread 0.05\n.end\n",
        )
        .expect("manual deck queue");

    assert_eq!(queue.len(), 2);
    assert!(matches!(queue[0].spec, AnalysisSpec::Parametric));
    assert!(queue[0].config.is_none());
    assert!(matches!(queue[1].spec, AnalysisSpec::MonteCarlo { .. }));
    assert!(queue[1].config.is_none());
}

#[test]
fn manual_deck_tf_builds_classic_dc_transfer_spec() {
    let state = AppState::default();
    let queue = build_manual_deck_queue(
        &state,
        "deck\nV1 in 0 DC 1\nR1 in out 1k\nR2 out 0 1k\n.tf V(out) V1\n.end\n",
    )
    .expect("classic .tf should produce a runnable typed analysis");

    assert_eq!(queue.len(), 1);
    assert_eq!(queue[0].analysis_line, ".tf");
    assert!(queue[0].config.is_none());
    assert!(matches!(
        &queue[0].spec,
        AnalysisSpec::Tf {
            input_source,
            output_expression,
            transfer_gain: true,
            input_resistance: true,
            output_resistance: true,
            normalization: crate::simulation::multi_run::TfNormalization::None,
            accuracy: crate::simulation::multi_run::TfAccuracy::Balanced,
        } if input_source == "V1" && output_expression == "V(OUT)"
    ));
}

#[test]
fn manual_deck_tf_preserves_differential_voltage_probe() {
    let state = AppState::default();
    let queue = build_manual_deck_queue(
        &state,
        "deck\nV1 in 0 DC 1\nR1 in out 1k\nR2 out ref 1k\nR3 ref 0 1k\n.tf V(out,ref) V1\n.end\n",
    )
    .expect("differential .tf should produce a runnable typed analysis");

    assert!(matches!(
        &queue[0].spec,
        AnalysisSpec::Tf {
            input_source,
            output_expression,
            ..
        } if input_source == "V1" && output_expression == "V(OUT,REF)"
    ));
}

#[test]
fn manual_deck_tf_preserves_branch_current_probe() {
    let state = AppState::default();
    let queue = build_manual_deck_queue(
            &state,
            "deck\nV1 in 0 DC 1\nR1 in mid 1k\nVMEAS mid out DC 0\nR2 out 0 1k\n.tf I(VMEAS) V1\n.end\n",
        )
        .expect("branch-current .tf should produce a runnable typed analysis");

    assert!(matches!(
        &queue[0].spec,
        AnalysisSpec::Tf {
            input_source,
            output_expression,
            ..
        } if input_source == "V1" && output_expression == "I(VMEAS)"
    ));
}

#[test]
fn manual_deck_rejects_multiple_step_commands() {
    let state = AppState::default();
    let err = build_manual_deck_queue(
            &state,
            "deck\n.param rload=1k cload=1p\nV1 out 0 1\nR1 out 0 {rload}\nC1 out 0 {cload}\n.step param rload 1k 2k 1k\n.step param cload 1p 2p 1p\n.end\n",
        )
        .expect_err("multiple step commands should be diagnosed");

    assert!(
        err.iter().any(|message| message.contains("one .step")),
        "expected duplicate .step diagnostic, got {err:?}"
    );
}

#[test]
fn manual_deck_rejects_multiple_monte_carlo_commands() {
    let state = AppState::default();
    let err = build_manual_deck_queue(
        &state,
        "deck\n.param rload=1k\nV1 out 0 1\nR1 out 0 {rload}\n.mc 4 seed 1\n.mc 5 seed 2\n.end\n",
    )
    .expect_err("multiple Monte Carlo commands should be diagnosed");

    assert!(
        err.iter().any(|message| message.contains("one .mc")),
        "expected duplicate .mc diagnostic, got {err:?}"
    );
}

#[test]
fn manual_deck_temp_directive_does_not_block_runnable_analysis() {
    let state = AppState::default();
    let queue = build_manual_deck_queue(&state, "deck\n.temp 125\nV1 out 0 1\n.op\n.end\n")
        .expect("manual deck queue");

    assert_eq!(queue.len(), 1);
    assert!(matches!(queue[0].spec, AnalysisSpec::Parametric));
    let temp = queue[0]
        .spec_options
        .temp
        .as_ref()
        .expect("temperature options");
    assert_eq!(temp.temperatures_c, vec![125.0]);
    assert!(matches!(temp.base_mode, CornerBaseMode::Op));
}

#[test]
fn manual_deck_step_temp_uses_paired_transient_base_analysis() {
    let state = AppState::default();
    let queue = build_manual_deck_queue(
            &state,
            "deck\nV1 out 0 pulse(0 1 0 1n 1n 5n 10n)\nR1 out 0 1k\n.tran 1n 10n\n.step temp list 0 25 125\n.end\n",
        )
        .expect("manual deck queue");

    assert_eq!(queue.len(), 1);
    assert!(matches!(queue[0].spec, AnalysisSpec::Parametric));
    let temp = queue[0]
        .spec_options
        .temp
        .as_ref()
        .expect("temperature options");
    assert_eq!(temp.temperatures_c, vec![0.0, 25.0, 125.0]);
    assert!(matches!(
        temp.base_mode,
        CornerBaseMode::Transient {
            stop_time,
            step_time,
        } if (stop_time - 10e-9).abs() < 1e-21
            && (step_time - 1e-9).abs() < 1e-21
    ));
}

#[test]
fn manual_deck_temp_list_uses_paired_op_base_analysis() {
    let state = AppState::default();
    let queue = build_manual_deck_queue(&state, "deck\n.temp 25 125\nV1 out 0 1\n.op\n.end\n")
        .expect("manual deck queue");

    assert_eq!(queue.len(), 1);
    assert!(matches!(queue[0].spec, AnalysisSpec::Parametric));
    let temp = queue[0]
        .spec_options
        .temp
        .as_ref()
        .expect("temperature options");
    assert_eq!(temp.temperatures_c, vec![25.0, 125.0]);
    assert!(matches!(temp.base_mode, CornerBaseMode::Op));
}

#[test]
fn manual_deck_parameter_step_retains_transient_base_analysis() {
    let state = AppState::default();
    let queue = build_manual_deck_queue(
            &state,
            "deck\n.param rload=1k\nV1 out 0 pulse(0 1 0 1n 1n 5n 10n)\nR1 out 0 {rload}\n.tran 1n 10n\n.step param rload 1k 2k 1k\n.end\n",
        )
        .expect("parameter step with transient should be planned");

    assert_eq!(queue.len(), 1);
    assert!(matches!(queue[0].spec, AnalysisSpec::Parametric));
    assert!(matches!(
        queue[0].spec_options.parametric_base,
        Some(CornerBaseMode::Transient { .. })
    ));
}

#[test]
fn manual_temperature_sweep_retains_nested_dc_and_full_transient_forms() {
    let state = AppState::default();
    let nested = build_manual_deck_queue(
            &state,
            "deck\nV1 in 0 0\nV2 bias 0 0\nR1 in bias 1k\n.dc V1 0 1 0.5 V2 0 2 1\n.step temp list 25 125\n.end\n",
        )
        .expect("nested DC temperature sweep");
    assert!(matches!(
        nested[0].spec_options.temp.as_ref().map(|config| &config.base_mode),
        Some(CornerBaseMode::DcSweepNested { source2, .. }) if source2 == "V2"
    ));

    let transient = build_manual_deck_queue(
            &state,
            "deck\nV1 out 0 pulse(0 1 0 1n 1n 5n 10n)\nR1 out 0 1k\n.tran 1n 20n 2n 0.5n uic\n.step temp list 25 125\n.end\n",
        )
        .expect("full transient temperature sweep");
    assert!(matches!(
        transient[0]
            .spec_options
            .temp
            .as_ref()
            .map(|config| &config.base_mode),
        Some(CornerBaseMode::TransientWindow {
            start_time,
            max_timestep: Some(max_timestep),
            uic: true,
            ..
        }) if (*start_time - 2e-9).abs() < 1e-21
            && (*max_timestep - 0.5e-9).abs() < 1e-21
    ));
}

#[test]
fn manual_deck_temp_only_reports_no_runnable_analysis() {
    let state = AppState::default();
    let err = build_manual_deck_queue(&state, "deck\n.temp 25 125\n.end\n")
        .expect_err("temperature directive alone should not run");

    assert!(
        err.iter()
            .any(|message| message.contains("No runnable analysis command"))
    );
}

#[test]
fn manual_deck_source_does_not_append_analysis_lines() {
    let source = "deck\nR1 out 0 1k\n.op\n.end\n";
    assert_eq!(compose_manual_deck_source(source), source);
}

#[test]
fn manual_deck_adds_end_only_when_missing() {
    assert_eq!(compose_manual_deck_source("deck\n.op"), "deck\n.op\n.end\n");
    assert_eq!(compose_manual_deck_source(".end\n.op"), ".end\n.op\n.end\n");
    for terminal in [".end; done", ".END // done", ".end $ done", ".end   "] {
        let source = format!("deck\r\n.op\r\n{terminal}\r\n");
        assert_eq!(compose_manual_deck_source(&source), source);
    }
}

#[test]
fn manual_deck_reports_no_analysis() {
    let state = AppState::default();
    let err = build_manual_deck_queue(&state, "deck\nR1 a 0 1k\n.end\n")
        .expect_err("no analysis should fail");
    assert!(
        err.iter()
            .any(|e| e.contains("No analysis command in netlist"))
    );
}

/// A hand-written `.SENS` card reaches the run with its own filter and
/// its own band.
///
/// Before this, the reader dropped the filter list and kept only the
/// band's lower edge: a deck that asked for four variables across sixty
/// frequencies ran every variable at one. That is the defect being fixed,
/// and a bare card now means what the engine means by it.
#[test]
fn a_hand_written_sens_card_keeps_its_filter_and_its_sweep() {
    use crate::simulation::config::AcSweepType;

    let deck =
        |card: &str| format!("sens deck\nV1 in 0 AC 1\nR1 in out 1k\nC1 out 0 1n\n{card}\n.end\n");
    let spec_for = |card: &str| {
        specs_for(&deck(card))
            .into_iter()
            .next()
            .expect("one .sens card is one analysis")
    };

    // A bare card: the engine's own default set, and no band.
    let AnalysisSpec::Sensitivity {
        filter,
        sweep,
        ac_mode,
        frequency,
        ..
    } = spec_for(".sens V(out)")
    else {
        panic!("a .sens card is a sensitivity analysis");
    };
    assert_eq!(filter, "");
    assert_eq!(sweep, None);
    assert!(!ac_mode);
    assert_eq!(frequency, None);

    let AnalysisSpec::Sensitivity { filter, sweep, .. } =
        spec_for(".sens V(out) R1 PARAM:* AC DEC 1 1k 1k")
    else {
        panic!("a .sens card is a sensitivity analysis");
    };
    assert_eq!(filter, "R1 PARAM:*");
    assert_eq!(sweep, None, "DEC 1 f f is the one-frequency card");

    let AnalysisSpec::Sensitivity {
        filter,
        sweep,
        frequency,
        ..
    } = spec_for(".sens V(out) C* AC OCT 5 10 1Meg")
    else {
        panic!("a .sens card is a sensitivity analysis");
    };
    assert_eq!(filter, "C*");
    assert_eq!(frequency, Some(10.0));
    let sweep = sweep.expect("an authored band reaches the spec");
    assert_eq!(sweep.stop_frequency, 1.0e6);
    assert_eq!(sweep.points, 5);
    assert_eq!(
        crate::simulation::config::SensitivitySweep::from_spec(sweep).variation,
        AcSweepType::Octave
    );
}

/// The single-frequency card and the band are told apart by exactly what
/// the form writes for one, not by a count alone.
#[test]
fn a_bare_hand_written_sens_card_means_what_the_engine_means() {
    use rspice_core::netlist::{FreqVariation, SensitivityAcSweep};

    assert_eq!(
        sensitivity_sweep_from_card(&SensitivityAcSweep {
            variation: FreqVariation::Dec,
            points: 1,
            start_freq: 1.0e3,
            stop_freq: 1.0e3,
        }),
        None
    );
    // One point over a real band is a band, and so is one decade point
    // spelled linearly.
    for sweep in [
        SensitivityAcSweep {
            variation: FreqVariation::Dec,
            points: 1,
            start_freq: 1.0e3,
            stop_freq: 1.0e6,
        },
        SensitivityAcSweep {
            variation: FreqVariation::Lin,
            points: 1,
            start_freq: 1.0e3,
            stop_freq: 1.0e3,
        },
    ] {
        assert!(sensitivity_sweep_from_card(&sweep).is_some(), "{sweep:?}");
    }
}

mod dc_mismatch;

#[test]
fn qpac_manual_deck_preserves_authored_grid_and_all_controls() {
    let source = "QPAC import\nV1 in 0 DC 1\nR1 in out 1k\n.QPSS 1k 1414.2135623730951 1732.0508075688772 HARMS=1\n.QPAC OCT 3 2 128 SOURCE=V1 OUT=V(out,0) INLATTICE=(0,1,-1) OUTLATTICE=(-1,0,1) MAG=.2 PHASE=73 SOLVER=KRYLOV KRYLOVRESTART=16 KRYLOVCYCLES=12 LINEARTOL=2e-11 IABSTOL=3e-13 VABSTOL=4e-10\n.end\n";
    let specs = specs_for(source);
    let spec = specs
        .iter()
        .find(|s| matches!(s, AnalysisSpec::Qpac { .. }))
        .unwrap();
    let AnalysisSpec::Qpac {
        start_freq,
        stop_freq,
        points_per_unit,
        sweep,
        input_lattice,
        output_lattice,
        controls,
        ..
    } = spec
    else {
        unreachable!()
    };
    assert_eq!(
        (*start_freq, *stop_freq, *points_per_unit, *sweep),
        (2.0, 128.0, 3, FrequencySweep::Octave)
    );
    assert_eq!(input_lattice, &[0, 1, -1]);
    assert_eq!(output_lattice, &[-1, 0, 1]);
    assert_eq!(controls.magnitude, 0.2);
    assert_eq!(controls.phase_degrees, 73.0);
    assert_eq!(controls.solver.linear.restart, 16);
    assert_eq!(controls.solver.linear.max_cycles, 12);
    assert_eq!(controls.solver.linear.relative_tolerance, 2e-11);
    let card = spec.qpac_card().unwrap();
    let reparsed = specs_for(&format!(
        "QPAC rewrite\nV1 in 0 1\nR1 in out 1k\n{}\n.end\n",
        card.to_spice()
    ));
    assert_eq!(reparsed, [spec.clone()]);
}

#[test]
fn qpxf_manual_deck_preserves_native_sweep_and_complete_selections() {
    let source = "QPXF import\nV1 in 0 DC 1\nR1 in out 1k\n.QPXF OCT 3 2 128 SOURCES=ALL OUT=I(L1) MAXORDERS=(2,1,3) OUTLATTICE=(-1,0,1) AXIS=OFFSET GROUPDELAY=YES GDFLOOR=3e-9 SOLVER=KRYLOV KRYLOVRESTART=16 KRYLOVCYCLES=12 LINEARTOL=2e-11\n.end\n";
    let specs = specs_for(source);
    let spec = &specs[0];
    let AnalysisSpec::Qpxf {
        start_freq,
        stop_freq,
        points_per_unit,
        sweep,
        output_lattice,
        controls,
        group_delay,
        ..
    } = spec
    else {
        panic!("wrong imported analysis")
    };
    assert_eq!(
        (*start_freq, *stop_freq, *points_per_unit, *sweep),
        (2.0, 128.0, 3, FrequencySweep::Octave)
    );
    assert_eq!(output_lattice, &[-1, 0, 1]);
    assert!(*group_delay);
    assert_eq!(
        controls.input_sources,
        Some(rspice_core::engine::QpxfSources::AllIndependent)
    );
    assert_eq!(
        controls.input_lattices,
        Some(rspice_core::engine::QpxfInputLattices::MaxOrders(vec![
            2, 1, 3
        ]))
    );
    assert_eq!(
        controls.frequency_axis,
        rspice_core::engine::QpxfFrequencyAxis::Offset
    );
    assert_eq!(controls.branch_current.as_deref(), Some("L1"));
    assert_eq!(controls.group_delay_magnitude_floor, 3e-9);
    assert_eq!(controls.solver.restart, 16);
    assert_eq!(controls.solver.max_cycles, 12);
    assert_eq!(controls.solver.relative_tolerance, 2e-11);
    let reparsed = specs_for(&format!(
        "QPXF rewrite\nV1 in 0 1\nR1 in out 1k\n{}\n.end\n",
        spec.qpxf_card().unwrap().to_spice()
    ));
    assert_eq!(reparsed, [spec.clone()]);
}

#[test]
fn periodic_op_handoff_manual_deck_inserts_one_unambiguous_seed() {
    let card = crate::simulation::plan::QpssDraft {
        tones: "1000,1414.2135623730951".into(),
        harmonics: "1,1".into(),
        dc_initialization: true,
        ..Default::default()
    }
    .to_spec()
    .unwrap()
    .driven_qpss_config()
    .unwrap()
    .to_spice()
    .unwrap();
    for card in [card, ".hb 1000".into()] {
        for explicit in [false, true] {
            let source = format!(
                "Manual QP\nV1 out 0 1\nR1 out 0 1k\n{card}\n{}\n.end\n",
                if explicit { ".op" } else { "" }
            );
            let queue = build_manual_deck_queue(&AppState::default(), &source).unwrap();
            assert_eq!(
                queue
                    .iter()
                    .filter(|task| matches!(
                        task.spec,
                        AnalysisSpec::DcOp { .. } | AnalysisSpec::LegacyDcOp
                    ))
                    .count(),
                1
            );
            assert_eq!(
                queue
                    .iter()
                    .filter(|task| matches!(
                        task.spec,
                        AnalysisSpec::Qpss { .. } | AnalysisSpec::HarmonicBalance { .. }
                    ))
                    .count(),
                1
            );
        }
    }
}
