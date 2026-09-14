use super::*;
use rspice_veriloga_runtime::transport_delay::{DelayCheckpoint, DelayConfiguration};

fn history(samples: &[(Value, Value)], delay: Value) -> DelayBuffer {
    DelayBuffer::from_checkpoint(DelayCheckpoint {
        left_limits: Vec::new(),
        configuration: Some(DelayConfiguration::Fixed { delay }),
        samples: samples.to_vec(),
    })
    .unwrap()
}

#[test]
fn phase_interpolation_matches_quadratic_midpoint_error_across_units_and_grids() {
    for time_scale in [1e-200, 1.0, 1e200] {
        for signal_scale in [Value::MIN_POSITIVE, 1e-180, 1.0, 1e180] {
            for polarity in [-1.0, 1.0] {
                for (t1, t2) in [(0.5, 0.75), (0.25, 3.0), (2.0, 2.125)] {
                    let f = |t: Value| polarity * signal_scale * (2.0 + t + 0.5 * t * t);
                    let samples = [(0.0, f(0.0)), (t1 * time_scale, f(t1))];
                    let buffer = history(&samples, 10.0 * time_scale);
                    let before = buffer.clone();
                    let control = phase_interpolation_control(
                        &buffer,
                        t2 * time_scale,
                        f(t2),
                        0.0,
                        signal_scale * 0.125,
                        0,
                    )
                    .unwrap();
                    // f''=signal_scale; the maximum chord error on the NEW
                    // interval is f''*h^2/8, attained at its midpoint.
                    let expected = (t2 - t1).powi(2);
                    assert!(
                        (control.normalized_error - expected).abs() < 2e-12 * expected,
                        "time={time_scale:e} signal={signal_scale:e} p={polarity} [{t1},{t2}]: {control:?} expected {expected:e}"
                    );
                    assert_eq!(buffer, before);
                }
            }
        }
    }
}

#[test]
fn phase_interpolation_preserves_affine_cancellation_and_subnormal_error_ratios() {
    for scale in [Value::from_bits(1), 1.0, 1e300] {
        let buffer = history(&[(0.0, scale), (1.0, 3.0 * scale)], 4.0);
        let control =
            phase_interpolation_control(&buffer, 2.0, 5.0 * scale, 0.0, scale, 0).unwrap();
        assert!(control.normalized_error <= 4.0 * Value::EPSILON);
    }
    let tiny = Value::from_bits(1);
    // The physical interpolation error is 0.25 of the smallest subnormal;
    // it must survive until divided by the configured absolute tolerance.
    let buffer = history(&[(0.0, 0.0), (tiny, tiny)], 8.0 * tiny);
    let control =
        phase_interpolation_control(&buffer, 2.0 * tiny, 4.0 * tiny, 0.0, tiny, 0).unwrap();
    assert_eq!(control.normalized_error, 0.25);
}

#[test]
fn phase_interpolation_controls_startup_and_refuses_invalid_candidates_without_mutation() {
    let buffer = history(&[(0.0, 1e-3)], 1e-12);
    let before = buffer.clone();
    let startup = phase_interpolation_control(&buffer, 1e-6, 1.1e-3, 1e-3, 1e-12, 0).unwrap();
    assert!(startup.normalized_error > 90.0);
    assert_eq!(startup.next_step, 1e-7);
    let steady = phase_interpolation_control(&buffer, 1e-6, 1e-3, 1e-3, 1e-12, 0).unwrap();
    assert_eq!(steady.normalized_error, 0.0);
    assert_eq!(steady.next_step, 2e-6);
    for (time, value, rel, abs) in [
        (0.0, 1e-3, 1e-3, 1e-12),
        (-1.0, 1e-3, 1e-3, 1e-12),
        (1.0, Value::NAN, 1e-3, 1e-12),
        (1.0, 1e-3, -1e-3, 1e-12),
        (1.0, 1e-3, 1e-3, 0.0),
        (Value::INFINITY, 1e-3, 1e-3, 1e-12),
    ] {
        assert!(phase_interpolation_control(&buffer, time, value, rel, abs, 0).is_err());
    }
    assert_eq!(buffer, before);
}

#[test]
fn phase_interpolation_uses_the_post_jump_anchor_without_a_pre_jump_slope() {
    let mut buffer = DelayBuffer::new(4);
    buffer.accept_sample(0.0, 0.0, 4.0, None).unwrap();
    buffer
        .accept_discontinuity(1.0, 1.0, 100.0, 4.0, None)
        .unwrap();
    let constant = phase_interpolation_control(&buffer, 2.0, 100.0, 0.0, 1e-6, 0).unwrap();
    assert_eq!(constant.normalized_error, 0.0);
    let changed = phase_interpolation_control(&buffer, 2.0, 100.25, 0.0, 0.25, 0).unwrap();
    assert_eq!(changed.normalized_error, 1.0);
    assert_eq!(changed.next_step, 0.9);
    buffer.accept_sample(2.0, 100.25, 4.0, None).unwrap();
    let affine = phase_interpolation_control(&buffer, 3.0, 100.5, 0.0, 1e-6, 0).unwrap();
    assert_eq!(affine.normalized_error, 0.0);
    buffer
        .accept_discontinuity(3.0, 100.5, 100.5, 4.0, None)
        .unwrap();
    let corner = phase_interpolation_control(&buffer, 4.0, 100.5, 0.0, 1e-6, 0).unwrap();
    assert_eq!(corner.normalized_error, 0.0);
}

#[test]
fn phase_interpolation_rejects_before_history_capture_and_cannot_waive_a_floor_or_grid() {
    let netlist = Netlist::parse("phase check\nQ1 c b 0 qm\nRc c 0 1k\nRb b 0 1k\n.model qm NPN(IS=1e-16 TF=1n PTF=30)\n.end\n").unwrap();
    let engine = Engine::default();
    let circuit = engine.build_circuit(&netlist).unwrap();
    let mut solution = vec![0.0; circuit.matrix_size()];
    let bjt = &circuit.bjts.devices[0];
    solution[bjt.node_collector - 1] = 2.0;
    solution[bjt.node_base - 1] = 0.6;
    let mut accepted =
        Engine::initialize_bjt_history(&circuit, &solution, ReactiveHistorySeed::SolvedBias);
    Engine::initialize_bjt_phase_history(&circuit, &mut accepted).unwrap();
    let before = accepted.clone();
    let coeff = CompanionCoefficients::backward_euler();
    let time = 2.0 * bjt.legacy_excess_phase_delay();
    solution[bjt.node_base - 1] = 0.7;
    let control = engine
        .bjt_phase_step_control(
            &circuit,
            &accepted,
            AcceptedReactiveStep {
                accepted_solution: &solution,
                accepted_time: time,
                dt: time,
                coeff: &coeff,
                bsim4_trnqs_coeff: &coeff,
            },
            &[],
            Default::default(),
        )
        .unwrap()
        .unwrap();
    assert!(control.normalized_error > 100.0);
    assert!(
        control
            .retry_step(time, time * 1e-6, false, false, &circuit)
            .unwrap()
            .unwrap()
            < time
    );
    for (minimum, locked, exhausted) in [
        (time, false, false),
        (time * 1e-6, true, false),
        (time * 1e-6, false, true),
    ] {
        let error = control
            .retry_step(time, minimum, locked, exhausted, &circuit)
            .unwrap_err();
        assert!(error.to_string().contains("Q1"), "{error}");
        assert!(error.to_string().contains("phase-history interpolation"));
    }
    assert_eq!(accepted, before);
}

fn smooth_grid(tolerance: Value, delay: Value, restore: bool) -> (Vec<Value>, Value, usize) {
    let f = |t: Value| 1e-3 * (2.0 + (std::f64::consts::TAU * t).sin());
    let mut buffer = DelayBuffer::new(4);
    buffer.accept_sample(0.0, f(0.0), delay, None).unwrap();
    let mut time = 0.0;
    let mut dt: Value = 0.03;
    let mut grid = vec![0.0];
    let mut error: Value = 0.0;
    let mut rejected = 0;
    while time < 2.0 {
        dt = dt.min(2.0 - time).min(0.03);
        let next = time + dt;
        let current = f(next);
        let control =
            phase_interpolation_control(&buffer, next, current, 0.0, tolerance, 0).unwrap();
        if control.normalized_error > 1.0 {
            rejected += 1;
            dt = control.next_step;
            continue;
        }
        // Independent dense samples check the input interval before pruning,
        // including its eventual delayed-output interpretation.
        for i in 1..16 {
            let fraction = f64::from(i) / 16.0;
            let at = time + fraction * (next - time);
            let linear = (1.0 - fraction) * f(time) + fraction * current;
            error = error.max((linear - f(at)).abs());
        }
        buffer.accept_sample(next, current, delay, None).unwrap();
        let actual = buffer
            .static_dae_with_coefficients(next, current, delay, None)
            .unwrap()
            .output;
        let expected = f((next - delay).max(0.0));
        error = error.max((actual - expected).abs());
        if restore {
            buffer = DelayBuffer::from_checkpoint(buffer.checkpoint()).unwrap();
        }
        time = next;
        grid.push(time);
        dt = control.next_step;
        assert!(grid.len() < 100_000);
    }
    (grid, error, rejected)
}

#[test]
fn phase_interpolation_smooth_refinement_converges_and_replays_with_steps_above_delay() {
    for delay in [1e-8, 0.1] {
        let coarse = smooth_grid(1e-7, delay, false);
        let fine = smooth_grid(1e-9, delay, false);
        let replay = smooth_grid(1e-9, delay, true);
        assert!(coarse.2 > 0 && fine.2 > 0);
        assert!(fine.0.len() > coarse.0.len());
        assert!(
            fine.1 < coarse.1 / 20.0,
            "delay={delay}: coarse={:e} fine={:e}",
            coarse.1,
            fine.1
        );
        assert!(fine.1 < 3e-9, "delay={delay}: error={:e}", fine.1);
        assert_eq!(fine, replay);
        if delay == 1e-8 {
            assert!(fine.0.windows(2).any(|p| p[1] - p[0] > 1000.0 * delay));
        }
    }
}
