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
    let netlist = Netlist::parse("Undeclared event model\nVjump in 0 PWL(0 0 1m 0 1m .1 2m .1)\nR1 in out 1k\nB1 out 0 I=V(out)*1m\n.end\n").unwrap();
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

#[test]
fn netlist_spectral_envelope_events_bind_native_diode_charge_and_finite_currents() {
    use crate::analysis::quasi_periodic::QuasiPeriodicTransform;
    let netlist = Netlist::parse("Native junction events\nVmod bias 0 PWL(0 0 1m 0 1m .1 2m .2)\nVrf input bias SIN(0 .01 1Meg 0 0 90)\nDdrive input 0 dm\nIjump a 0 PWL(0 0 1m 0 1m -1m 2m -1m)\nDfloat a b dm\nRa a 0 1k\nRb b 0 2k\n.model dm D(IS=1e-14 CJO=1u VJ=.6 M=.5)\n.end\n").unwrap();
    let mut prepared = Engine::new(SimulationConfig::default())
        .prepare_spectral_envelope_with_abort(&netlist, config(&["Vmod", "Ijump"]), &NoAbort)
        .unwrap();
    let diode = prepared.circuit.diodes.devices[0].clone();
    let grid = prepared.grid().clone();
    let mut transform = QuasiPeriodicTransform::new_with_abort(grid.clone(), &NoAbort).unwrap();
    let events = prepared
        .event_config_with_abort(&EnvelopeEventTolerances::default(), &NoAbort)
        .unwrap();
    let initial = prepared
        .initialize_with_abort(0.0, EnvelopeSourceSide::RightLimit, None, &NoAbort)
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
    let event = prepared
        .transition_event_with_abort(&incoming, &events, &NoAbort)
        .unwrap();
    // Independent depletion-law oracle, using the elaborated temperature's
    // Cj0/Vj. All samples are below the forward polynomial continuation.
    let q = |v: Value| 2.0 * diode.cj0 * diode.vj * (1.0 - (1.0 - v / diode.vj).sqrt());
    let c = |v: Value| diode.cj0 / (1.0 - v / diode.vj).sqrt();
    let old_v: Vec<_> = (0..grid.sample_count())
        .map(|s| 0.01 * grid.phases(s).unwrap()[0].cos())
        .collect();
    let impulse_samples: Vec<_> = old_v
        .iter()
        .map(|&v| Complex64::new(q(v) - q(v + 0.1), 0.0))
        .collect();
    let expected_impulses = transform
        .to_spectrum_with_abort(&impulse_samples, &NoAbort)
        .unwrap();
    let charge_samples: Vec<_> = old_v
        .iter()
        .map(|&v| Complex64::new(q(v + 0.1), 0.0))
        .collect();
    let charge = transform
        .to_spectrum_with_abort(&charge_samples, &NoAbort)
        .unwrap();
    let fast_current = grid.differentiate_with_abort(&charge, &NoAbort).unwrap();
    let finite_samples: Vec<_> = old_v
        .iter()
        .map(|&v| {
            let v = v + 0.1;
            Complex64::new(
                -diode.is * (v / (diode.n * diode.vt)).exp_m1() - c(v) * 100.0,
                0.0,
            )
        })
        .collect();
    let finite = transform
        .to_spectrum_with_abort(&finite_samples, &NoAbort)
        .unwrap();
    let row = |name: &str| {
        event
            .state
            .node_names()
            .iter()
            .chain(event.state.branch_names())
            .position(|n| n.eq_ignore_ascii_case(name))
            .unwrap()
    };
    for source in ["Vmod", "Vrf"] {
        for k in 0..grid.len() {
            close(
                event.current_impulses[row(source)][k],
                expected_impulses[k],
                1e-13,
            );
            close(
                event.state.spectra()[row(source)][k],
                finite[k] - fast_current[k],
                1e-8,
            );
        }
    }
    assert!(expected_impulses[grid.index_of(&[1]).unwrap()].norm() > 1e-10);
    // Floating nonlinear charge preserves its differential voltage while the
    // current step moves both nodes. A charge-free topology union would lose
    // this constraint; a grounded assumption would overconstrain the jump.
    for node in ["a", "b"] {
        close(
            coefficient(&event.state, node, &[0]),
            Complex64::new(2.0 / 3.0, 0.0),
            1e-9,
        );
    }
    let dc = grid.dc_index();
    close(
        event.slow_rates[row("a")][dc],
        Complex64::new(1.0e-3 / (9.0 * diode.cj0), 0.0),
        1e-5,
    );
    close(
        event.slow_rates[row("b")][dc],
        Complex64::new(-2.0e-3 / (9.0 * diode.cj0), 0.0),
        1e-5,
    );
    assert_eq!(event.state.source_side(), EnvelopeSourceSide::RightLimit);
    assert_eq!(event.state.order(), 0);
}
