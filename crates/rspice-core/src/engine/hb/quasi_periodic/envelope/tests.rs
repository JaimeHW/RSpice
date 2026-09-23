use super::*;
use crate::abort_signal::ImmediateAbort;
use crate::config::SimulationConfig;
mod transition;
mod mission;

fn config(names: &[&str]) -> SpectralEnvelopeConfig {
    SpectralEnvelopeConfig {
        carrier: EnvelopeCarrierBasis::Periodic {
            frequency_hz: 1e6,
            harmonics: 1,
            samples: 8,
        },
        solver: QuasiPeriodicSolveConfig {
            relative_tolerance: 1e-9,
            current_absolute_tolerance: 1e-13,
            voltage_absolute_tolerance: 1e-11,
            ..Default::default()
        },
        source_tones: Vec::new(),
        modulation_sources: names.iter().map(|s| (*s).into()).collect(),
        stop_time: 0.002,
        source_time_step: 1e-6,
    }
}

fn coefficient(state: &NetlistEnvelopeState, name: &str, tuple: &[i32]) -> Complex64 {
    let row = state
        .node_names()
        .iter()
        .chain(state.branch_names())
        .chain(state.auxiliary_names())
        .position(|n| n.eq_ignore_ascii_case(name))
        .unwrap();
    state.spectra()[row][state.grid().index_of(tuple).unwrap()]
}

fn close(actual: Complex64, expected: Complex64, tolerance: Value) {
    assert!(
        (actual - expected).norm() < tolerance,
        "{actual:?} != {expected:?}"
    );
}

#[test]
fn netlist_spectral_envelope_samples_original_sources_and_preserves_rc_history() {
    let netlist = Netlist::parse("slow voltage and current\nVc rf 0 SIN(0 .4 1Meg 0 0 90)\nVmod bias 0 DC 7 AC 100 45 PWL(0 0 2m 1)\nImod out 0 DC 0 AC 200 PWL(0 0 2m -1m)\nRrf rf out 1k\nRmod bias out 1k\nRload out 0 1k\nCout out 0 1u\n.end\n").unwrap();
    let engine = Engine::new(SimulationConfig::default());
    let mut prepared = engine
        .prepare_spectral_envelope_with_abort(&netlist, config(&["vmod", "IMOD"]), &NoAbort)
        .unwrap();
    let dc = engine.run_dc_op_with_abort(&netlist, &NoAbort).unwrap();
    let seed = PeriodicDcOperatingPointSeed::try_new(
        dc.node_names.into_iter().skip(1).collect(),
        dc.branch_names,
        dc.node_voltages
            .into_iter()
            .skip(1)
            .chain(dc.branch_currents)
            .collect(),
    )
    .unwrap();
    let initial = prepared
        .initialize_with_abort(0.0, EnvelopeSourceSide::Published, Some(&seed), &NoAbort)
        .unwrap();
    let carrier = Complex64::new(0.2e-3, 0.0) / Complex64::new(0.003, std::f64::consts::TAU);
    close(coefficient(&initial, "out", &[0]), Complex64::ZERO, 1e-10);
    close(coefficient(&initial, "out", &[1]), carrier, 1e-10);
    assert_eq!(initial.grid().dimensions().len(), 1);
    assert_eq!(prepared.modulation_sources().len(), 2);
    let first = prepared
        .step_with_abort(
            &initial,
            0.001,
            SpectralEnvelopeMethod::Bdf2,
            EnvelopeSourceSide::Published,
            &NoAbort,
        )
        .unwrap();
    // Backward Euler: (G + C/h) v1 = (Vmod/R - Imod) + C/h * v0.
    close(
        coefficient(&first, "out", &[0]),
        Complex64::new(0.25, 0.0),
        1e-10,
    );
    close(
        coefficient(&first, "bias", &[0]),
        Complex64::new(0.5, 0.0),
        1e-10,
    );
    close(
        coefficient(&first, "Vmod", &[0]),
        Complex64::new(-0.25e-3, 0.0),
        1e-11,
    );
    close(coefficient(&first, "out", &[1]), carrier, 1e-10);
    assert_eq!(first.order(), 1);
    // A backwards-in-time probe on the original accepted state must use the
    // original PWL, not the source value frozen by the previous trial.
    let probe = prepared
        .step_with_abort(
            &initial,
            0.0005,
            SpectralEnvelopeMethod::BackwardEuler,
            EnvelopeSourceSide::Published,
            &NoAbort,
        )
        .unwrap();
    close(
        coefficient(&probe, "out", &[0]),
        Complex64::new(0.1, 0.0),
        1e-10,
    );
    let second = prepared
        .step_with_abort(
            &first,
            0.0016,
            SpectralEnvelopeMethod::Bdf2,
            EnvelopeSourceSide::Published,
            &NoAbort,
        )
        .unwrap();
    let h = 0.0006;
    let previous_h = 0.001;
    let a0 = (2.0 * h + previous_h) / (h * (h + previous_h));
    let a1 = -(h + previous_h) / (h * previous_h);
    let expected = (0.0016 - 1e-6 * a1 * 0.25) / (0.003 + 1e-6 * a0);
    close(
        coefficient(&second, "out", &[0]),
        Complex64::new(expected, 0.0),
        1e-10,
    );
    close(coefficient(&second, "out", &[1]), carrier, 1e-10);
    assert_eq!(second.order(), 2);
    close(coefficient(&initial, "out", &[0]), Complex64::ZERO, 1e-10);
}

#[test]
fn netlist_spectral_envelope_adapts_a_two_tone_modulated_rc_circuit() {
    let netlist = Netlist::parse("independent carriers\nV1 a 0 AC .4 30\nV2 b 0 SIN(0 .2 1414213.562373095 0 0 -40)\nImod out 0 PWL(0 0 2m -1m)\nIprobe out 0 AC 100\nR1 a out 1k\nR2 b out 1k\nRload out 0 1k\nC1 out 0 1u\n.end\n").unwrap();
    let mut request = config(&["Imod"]);
    request.carrier = EnvelopeCarrierBasis::QuasiPeriodic {
        grid: QuasiPeriodicGridConfig::new(vec![1e6, std::f64::consts::SQRT_2 * 1e6], vec![1, 1]),
    };
    request.source_tones.push(QpssSourceTone {
        source: "V1".into(),
        tone: 0,
    });
    let mut prepared = Engine::new(SimulationConfig::default())
        .prepare_spectral_envelope_with_abort(&netlist, request, &NoAbort)
        .unwrap();
    let mut state = prepared
        .initialize_with_abort(0.0, EnvelopeSourceSide::Published, None, &NoAbort)
        .unwrap();
    let first_carrier = Complex64::from_polar(0.2e-3, 30.0_f64.to_radians())
        / Complex64::new(0.003, std::f64::consts::TAU);
    let second_carrier = Complex64::from_polar(0.1e-3, (-130.0_f64).to_radians())
        / Complex64::new(0.003, std::f64::consts::TAU * std::f64::consts::SQRT_2);
    close(coefficient(&state, "out", &[1, 0]), first_carrier, 1e-10);
    close(coefficient(&state, "out", &[0, 1]), second_carrier, 1e-10);
    let control = SpectralEnvelopeControl {
        method: SpectralEnvelopeMethod::Bdf2,
        minimum_step: 1e-9,
        maximum_step: 1e-3,
        relative_tolerance: 2e-3,
        absolute_tolerances: (0..state.spectra().len())
            .map(|i| {
                if i < state.node_names().len() {
                    1e-8
                } else {
                    1e-11
                }
            })
            .collect(),
        max_rejections: 12,
    };
    let mut step = control.maximum_step;
    let mut count = 0;
    let mut rejected = 0;
    let mut second_order = false;
    while state.time() < 0.002 {
        let next = prepared
            .advance_with_abort(
                &state,
                step,
                0.002,
                EnvelopeSourceSide::Published,
                &control,
                &NoAbort,
            )
            .unwrap();
        assert!(next.error_ratio <= 1.0);
        rejected += next.rejected_steps;
        step = next.suggested_step;
        state = next.state;
        second_order |= state.order() == 2;
        let tau = 1.0 / 3000.0;
        let expected = (0.5 / 0.003) * (state.time() - tau * (1.0 - (-state.time() / tau).exp()));
        close(
            coefficient(&state, "out", &[0, 0]),
            Complex64::new(expected, 0.0),
            1e-3,
        );
        count += 1;
        assert!(
            count < 250,
            "slow integration must not march every RF cycle"
        );
    }
    assert!(rejected > 0 && second_order);
    assert_eq!(state.time().to_bits(), 0.002_f64.to_bits());
    close(coefficient(&state, "out", &[1, 0]), first_carrier, 1e-10);
    close(coefficient(&state, "out", &[0, 1]), second_carrier, 1e-10);
}

#[test]
fn netlist_spectral_envelope_preserves_event_sides_and_rejects_foreign_history() {
    let netlist = Netlist::parse("exact source limits\nVjump out 0 DC 8 AC 100 PWL(0 1 1m 1 1m 3 2m 3)\nIjump load 0 PWL(0 0 1m 0 1m 2m 2m 2m)\nRload load 0 1k\nRout out 0 1k\n.end\n").unwrap();
    let engine = Engine::new(SimulationConfig::default());
    let request = config(&["Vjump", "Ijump"]);
    let mut prepared = engine
        .prepare_spectral_envelope_with_abort(&netlist, request.clone(), &NoAbort)
        .unwrap();
    let initial = prepared
        .initialize_with_abort(0.0, EnvelopeSourceSide::Published, None, &NoAbort)
        .unwrap();
    let left = prepared
        .step_with_abort(
            &initial,
            0.001,
            SpectralEnvelopeMethod::BackwardEuler,
            EnvelopeSourceSide::LeftLimit,
            &NoAbort,
        )
        .unwrap();
    let right = prepared
        .step_with_abort(
            &initial,
            0.001,
            SpectralEnvelopeMethod::BackwardEuler,
            EnvelopeSourceSide::RightLimit,
            &NoAbort,
        )
        .unwrap();
    close(
        coefficient(&left, "out", &[0]),
        Complex64::new(1.0, 0.0),
        1e-10,
    );
    close(
        coefficient(&right, "out", &[0]),
        Complex64::new(3.0, 0.0),
        1e-10,
    );
    close(coefficient(&left, "load", &[0]), Complex64::ZERO, 1e-10);
    close(
        coefficient(&right, "load", &[0]),
        Complex64::new(-2.0, 0.0),
        1e-10,
    );
    close(
        coefficient(&right, "Vjump", &[0]),
        Complex64::new(-3e-3, 0.0),
        1e-10,
    );
    let control = SpectralEnvelopeControl {
        method: SpectralEnvelopeMethod::Bdf2,
        minimum_step: 1e-8,
        maximum_step: 0.001,
        relative_tolerance: 1e-3,
        absolute_tolerances: vec![1e-9; initial.spectra().len()],
        max_rejections: 5,
    };
    let advance = prepared
        .advance_with_abort(
            &initial,
            0.001,
            0.001,
            EnvelopeSourceSide::LeftLimit,
            &control,
            &NoAbort,
        )
        .unwrap();
    assert_eq!(advance.state.time().to_bits(), 0.001_f64.to_bits());
    close(
        coefficient(&advance.state, "out", &[0]),
        Complex64::new(1.0, 0.0),
        1e-10,
    );
    let mut foreign = engine
        .prepare_spectral_envelope_with_abort(&netlist, request.clone(), &NoAbort)
        .unwrap();
    assert!(
        foreign
            .step_with_abort(
                &initial,
                0.001,
                SpectralEnvelopeMethod::Bdf2,
                EnvelopeSourceSide::Published,
                &NoAbort
            )
            .unwrap_err()
            .to_string()
            .contains("different prepared circuit")
    );
    assert!(matches!(
        prepared.step_with_abort(
            &initial,
            0.001,
            SpectralEnvelopeMethod::Bdf2,
            EnvelopeSourceSide::Published,
            &ImmediateAbort
        ),
        Err(SimulationError::Aborted)
    ));
    assert!(
        prepared
            .step_with_abort(
                &initial,
                0.003,
                SpectralEnvelopeMethod::Bdf2,
                EnvelopeSourceSide::Published,
                &NoAbort
            )
            .is_err()
    );
    let limits = prepared.limits;
    prepared.limits.max_result_values = 1;
    assert!(matches!(
        prepared.advance_with_abort(
            &initial,
            0.001,
            0.001,
            EnvelopeSourceSide::LeftLimit,
            &control,
            &NoAbort
        ),
        Err(SimulationError::ResourceLimit(_))
    ));
    prepared.limits = limits;
    let after_failure = prepared
        .step_with_abort(
            &initial,
            0.001,
            SpectralEnvelopeMethod::BackwardEuler,
            EnvelopeSourceSide::RightLimit,
            &NoAbort,
        )
        .unwrap();
    assert_eq!(after_failure.spectra(), right.spectra());
    for names in [vec!["missing"], vec!["Vjump", "vJUMP"], vec![" Vjump"]] {
        assert!(
            engine
                .prepare_spectral_envelope_with_abort(&netlist, config(&names), &NoAbort)
                .is_err()
        );
    }
    let mut conflicting = request.clone();
    conflicting.source_tones.push(QpssSourceTone {
        source: "vjump".into(),
        tone: 0,
    });
    assert!(
        engine
            .prepare_spectral_envelope_with_abort(&netlist, conflicting, &NoAbort)
            .err()
            .unwrap()
            .to_string()
            .contains("slow-time source")
    );
    assert!(matches!(
        engine.prepare_spectral_envelope_with_abort(&netlist, request, &ImmediateAbort),
        Err(SimulationError::Aborted)
    ));
}

#[test]
fn netlist_spectral_envelope_modulation_events_bound_steps_without_enumerating_rf_cycles() {
    let event = 0.00073_f64;
    let adjacent = event.next_up();
    let deck = format!(
        "selected physical clocks\nVrf rf 0 PULSE(0 1 0 1n 1n 400n 1u)\nRrf rf 0 1k\nVmod out 0 PWL(0 0 {event:.17e} 0 {event:.17e} 1 2m 1)\nRmod out 0 1k\nVnear near 0 PWL(0 0 {adjacent:.17e} 0 {adjacent:.17e} 2 2m 2)\nRnear near 0 1k\nImod load 0 PWL(0 0 {event:.17e} 0 {event:.17e} -1m 2m -1m)\nRload load 0 1k\nVdefault d 0 PULSE(0 1 1.2m)\nRdefault d 0 1k\n.end\n"
    );
    let netlist = Netlist::parse(&deck).unwrap();
    let mut settings = SimulationConfig::default();
    settings.spice_dialect = crate::config::SpiceDialect::Ngspice;
    settings.resource_limits.max_analysis_points = 64;
    let mut request = config(&["vmod", "VNEAR", "Imod", "Vdefault"]);
    request.source_time_step = 20e-6;
    let mut prepared = Engine::new(settings)
        .prepare_spectral_envelope_with_abort(&netlist, request, &NoAbort)
        .unwrap();
    assert_eq!(prepared.next_source_event_after(0.0).unwrap(), Some(event));
    assert_eq!(
        prepared.next_source_event_after(event).unwrap(),
        Some(adjacent)
    );
    assert_eq!(
        prepared.next_source_event_after(adjacent).unwrap(),
        Some(0.0012)
    );
    assert_eq!(
        prepared.next_source_event_after(0.0012).unwrap(),
        Some(0.0012 + 20e-6)
    );
    let events = prepared
        .source_events_at(event)
        .unwrap()
        .collect::<Vec<_>>();
    assert_eq!(events.len(), 2);
    let voltage = events
        .iter()
        .find(|e| e.source.eq_ignore_ascii_case("Vmod"))
        .unwrap();
    assert_eq!((voltage.left_value, voltage.right_value), (0.0, 1.0));
    let current = events
        .iter()
        .find(|e| e.source.eq_ignore_ascii_case("Imod"))
        .unwrap();
    assert_eq!((current.left_value, current.right_value), (0.0, -0.001));
    assert!(
        events
            .iter()
            .all(|e| e.derivative_order_lower_bound == Some(0))
    );
    assert_eq!(
        prepared
            .source_events_at(adjacent)
            .unwrap()
            .next()
            .unwrap()
            .source,
        "VNEAR"
    );
    assert!(prepared.source_events_at(0.0004).unwrap().next().is_none());
    assert!(prepared.source_events_at(Value::NAN).is_err());
    assert!(prepared.next_source_event_after(0.003).is_err());
    let initial = prepared
        .initialize_with_abort(0.0, EnvelopeSourceSide::RightLimit, None, &NoAbort)
        .unwrap();
    let control = SpectralEnvelopeControl {
        method: SpectralEnvelopeMethod::Bdf2,
        minimum_step: 1e-9,
        maximum_step: 0.002,
        relative_tolerance: 1e-3,
        absolute_tolerances: vec![1e-9; initial.spectra().len()],
        max_rejections: 5,
    };
    let accepted = prepared
        .advance_with_abort(
            &initial,
            0.002,
            0.002,
            EnvelopeSourceSide::Published,
            &control,
            &NoAbort,
        )
        .unwrap();
    assert_eq!(accepted.state.time().to_bits(), event.to_bits());
    assert_eq!(accepted.rejected_steps, 0);
    close(
        coefficient(&accepted.state, "out", &[0]),
        Complex64::ZERO,
        1e-10,
    );
    close(
        coefficient(&accepted.state, "load", &[0]),
        Complex64::ZERO,
        1e-10,
    );
    // Queries and accepted steps do not consume clocks or merge adjacent roots.
    assert_eq!(prepared.next_source_event_after(0.0).unwrap(), Some(event));
    assert_eq!(
        prepared
            .next_source_event_after(accepted.state.time())
            .unwrap(),
        Some(adjacent)
    );
    // The unchanged all-source collection would include thousands of RF
    // edges and exceed this deliberately small event budget.
    assert!(
        Engine::collect_selected_physical_source_events(
            &prepared.circuit,
            0.002,
            None,
            &prepared.limits,
            &NoAbort
        )
        .is_err()
    );
}
