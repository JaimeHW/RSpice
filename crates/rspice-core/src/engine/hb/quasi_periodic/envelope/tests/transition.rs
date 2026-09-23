use super::*;

#[test]
fn netlist_spectral_envelope_events_conserve_storage_and_bind_outgoing_history() {
    let netlist = Netlist::parse("Spectral event binding\nVjump input 0 PWL(0 1 1m 1 1m 3 2m 3)\nVrf rf 0 SIN(0 .4 1Meg 0 0 90)\nCshunt input 0 2u\nCcouple input out 1u\nRout out 0 1k\nRrf rf out 2k\nLout out load 1m\nRload load 0 1k\nIjump a 0 PWL(0 0 1m 0 1m -1m 2m -2m)\nCf a b .5u\nRa a 0 1k\nRb b 0 2k\n.end\n").unwrap();
    let engine = Engine::new(SimulationConfig::default());
    let request = config(&["Vjump", "Ijump"]);
    let mut prepared = engine
        .prepare_spectral_envelope_with_abort(&netlist, request.clone(), &NoAbort)
        .unwrap();
    let initial = prepared
        .initialize_with_abort(0.0, EnvelopeSourceSide::Published, None, &NoAbort)
        .unwrap();
    let incoming = prepared
        .step_with_abort(
            &initial,
            0.001,
            SpectralEnvelopeMethod::BackwardEuler,
            EnvelopeSourceSide::LeftLimit,
            &NoAbort,
        )
        .unwrap();
    assert!(
        prepared
            .step_with_abort(
                &incoming,
                0.0011,
                SpectralEnvelopeMethod::Bdf2,
                EnvelopeSourceSide::Published,
                &NoAbort
            )
            .unwrap_err()
            .to_string()
            .contains("must be transitioned")
    );
    let event_config = SpectralEnvelopeEventConfig {
        solver: request.solver.clone(),
        charge_tolerances: vec![1e-13; incoming.spectra().len()],
        rate_tolerances: vec![1e-10; incoming.spectra().len()],
    };
    assert!(matches!(
        prepared.transition_event_with_abort(&incoming, &event_config, &ImmediateAbort),
        Err(SimulationError::Aborted)
    ));
    let mut foreign = engine
        .prepare_spectral_envelope_with_abort(&netlist, request, &NoAbort)
        .unwrap();
    assert!(
        foreign
            .transition_event_with_abort(&incoming, &event_config, &NoAbort)
            .unwrap_err()
            .to_string()
            .contains("different prepared circuit")
    );
    let limits = prepared.limits;
    prepared.limits.max_result_values = 1;
    assert!(matches!(
        prepared.transition_event_with_abort(&incoming, &event_config, &NoAbort),
        Err(SimulationError::ResourceLimit(_))
    ));
    prepared.limits = limits;
    let before = incoming.spectra().to_vec();
    let event = prepared
        .transition_event_with_abort(&incoming, &event_config, &NoAbort)
        .unwrap();
    assert_eq!(incoming.spectra(), before);
    assert_eq!(event.state.time(), incoming.time());
    assert_eq!(event.state.source_side(), EnvelopeSourceSide::RightLimit);
    assert_eq!(event.state.order(), 0);
    assert!(
        prepared
            .transition_event_with_abort(&event.state, &event_config, &NoAbort)
            .is_err()
    );
    let dc = |state: &NetlistEnvelopeState, name| coefficient(state, name, &[0]);
    for (name, value) in [
        ("input", 3.0),
        ("out", 2.0),
        ("load", 0.0),
        ("Lout", 0.0),
        ("Vjump", -0.003),
        ("a", 2.0 / 3.0),
        ("b", 2.0 / 3.0),
    ] {
        close(dc(&event.state, name), Complex64::new(value, 0.0), 1e-9);
    }
    close(
        dc(&event.state, "input") - dc(&event.state, "out"),
        dc(&incoming, "input") - dc(&incoming, "out"),
        1e-10,
    );
    for name in ["out", "Lout", "Vjump"] {
        close(
            coefficient(&event.state, name, &[1]),
            coefficient(&incoming, name, &[1]),
            1e-10,
        );
    }
    let row = |name: &str| {
        event
            .state
            .node_names()
            .iter()
            .chain(event.state.branch_names())
            .position(|n| n.eq_ignore_ascii_case(name))
            .unwrap()
    };
    let dc_index = event.state.grid().dc_index();
    close(
        event.current_impulses[row("Vjump")][dc_index],
        Complex64::new(-4e-6, 0.0),
        1e-13,
    );
    for (name, value) in [
        ("out", -3000.0),
        ("Lout", 2000.0),
        ("a", 8000.0 / 9.0),
        ("b", 2000.0 / 9.0),
    ] {
        close(
            event.slow_rates[row(name)][dc_index],
            Complex64::new(value, 0.0),
            1e-6,
        );
    }
    let next = prepared
        .step_with_abort(
            &event.state,
            0.00101,
            SpectralEnvelopeMethod::Bdf2,
            EnvelopeSourceSide::Published,
            &NoAbort,
        )
        .unwrap();
    assert_eq!(next.order(), 1, "a discontinuity restarts the BDF history");
    assert!(dc(&next, "out").re < 2.0);
    assert!(dc(&next, "Lout").re > 0.0);
}

#[test]
fn netlist_spectral_envelope_events_refuse_undeclared_charge_topology() {
    let netlist = Netlist::parse("Undeclared event model\nVjump in 0 PWL(0 0 1m 0 1m .1 2m .1)\nR1 in out 1k\nD1 out 0 dm\n.model dm D(IS=1e-14 CJO=1p)\n.end\n").unwrap();
    let request = config(&["Vjump"]);
    let mut prepared = Engine::new(SimulationConfig::default())
        .prepare_spectral_envelope_with_abort(&netlist, request.clone(), &NoAbort)
        .unwrap();
    let initial = prepared
        .initialize_with_abort(0.0, EnvelopeSourceSide::Published, None, &NoAbort)
        .unwrap();
    let incoming = prepared
        .step_with_abort(
            &initial,
            0.001,
            SpectralEnvelopeMethod::BackwardEuler,
            EnvelopeSourceSide::LeftLimit,
            &NoAbort,
        )
        .unwrap();
    let event_config = SpectralEnvelopeEventConfig {
        solver: request.solver,
        charge_tolerances: vec![1e-13; incoming.spectra().len()],
        rate_tolerances: vec![1e-10; incoming.spectra().len()],
    };
    let before = incoming.spectra().to_vec();
    assert!(
        prepared
            .transition_event_with_abort(&incoming, &event_config, &NoAbort)
            .unwrap_err()
            .to_string()
            .contains("require declared spectral event charge ports")
    );
    assert_eq!(incoming.spectra(), before);
}
