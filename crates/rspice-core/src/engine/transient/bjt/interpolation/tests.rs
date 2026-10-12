use super::*;
use rspice_veriloga_runtime::transport_delay::{DelayCheckpoint, DelayConfiguration};

fn history(samples: &[(Value, Value)], delay: Value) -> DelayBuffer {
    DelayBuffer::from_checkpoint(DelayCheckpoint {
        event_orders: Vec::new(),
        left_limits: Vec::new(),
        configuration: Some(DelayConfiguration::Fixed { delay }),
        samples: samples.to_vec(),
    })
    .unwrap()
}

#[test]
fn phase_interpolation_uses_analytic_outgoing_slope_after_a_corner() {
    for time_scale in [1e-100, 1.0, 1e100] {
        for signal_scale in [1e-100, 1.0, 1e100] {
            for polarity in [-1.0, 1.0] {
                let buffer = DelayBuffer::from_checkpoint(DelayCheckpoint {
                    configuration: Some(DelayConfiguration::Fixed {
                        delay: 4.0 * time_scale,
                    }),
                    samples: vec![
                        (0.0, -10.0 * polarity * signal_scale),
                        (time_scale, 2.0 * polarity * signal_scale),
                    ],
                    left_limits: vec![(time_scale, 100.0 * polarity * signal_scale)],
                    event_orders: Vec::new(),
                })
                .unwrap();
                let before = buffer.clone();
                let control = phase_interpolation_control_with_slope(
                    &buffer,
                    Some(polarity * signal_scale / time_scale),
                    1.5 * time_scale,
                    2.625 * polarity * signal_scale,
                    PhaseInterpolationTolerance {
                        reltol: 0.0,
                        abstol: 0.03125 * signal_scale,
                        input_referred: 0.0,
                    },
                    None,
                    0,
                )
                .unwrap();
                // Outgoing f(u)=2+u+u²/2. At u=.25 the chord to u=.5
                // differs by .03125; the arbitrary incoming slope is irrelevant.
                assert!(
                    (control.normalized_error - 1.0).abs() < 1e-12,
                    "{control:?}"
                );
                assert_eq!(buffer, before);
            }
        }
    }
}

#[test]
fn phase_interpolation_refuses_nonfinite_outgoing_slope() {
    let buffer = history(&[(0.0, 1.0)], 1.0);
    for slope in [Value::NAN, Value::INFINITY, Value::NEG_INFINITY] {
        let tolerance = PhaseInterpolationTolerance {
            reltol: 1e-3,
            abstol: 1e-9,
            input_referred: 0.0,
        };
        assert!(
            phase_interpolation_control_with_slope(
                &buffer,
                Some(slope),
                0.1,
                1.1,
                tolerance,
                None,
                0
            )
            .is_err()
        );
    }
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
        Engine::initialize_bjt_history(&circuit, &solution, ReactiveHistorySeed::SolvedBias)
            .unwrap();
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

/// A fixed delay shorter than half the interval reads the new interval only
/// on its last `delay`: for `f = t^2` the chord error there peaks at the read
/// window's start, `delay * (h - delay)`, not at the midpoint's `h^2 / 4`. The
/// predicted step brings that read-window error to 0.81 of the tolerance.
#[test]
fn phase_interpolation_judges_only_the_window_a_fixed_delay_reads() {
    let buffer = history(&[(0.0, 0.0), (1.0, 1.0)], 0.1);
    let tolerance = |abstol| PhaseInterpolationTolerance {
        reltol: 0.0,
        abstol,
        input_referred: 0.0,
    };
    let control = |abstol, delay| {
        phase_interpolation_control_with_slope(&buffer, None, 2.0, 4.0, tolerance(abstol), delay, 0)
            .unwrap()
    };
    assert!((control(1.0, None).normalized_error - 0.25).abs() < 1e-15);
    let windowed = control(1.0, Some(0.1));
    assert!((windowed.normalized_error - 0.1 * 0.9).abs() < 1e-15);
    // Accepted on the read window (0.9) though the whole interval is not
    // (2.5); the next step lands the read-window error on 0.81.
    let windowed = control(0.1, Some(0.1));
    assert!((windowed.normalized_error - 0.9).abs() < 1e-14);
    let next = windowed.next_step;
    assert!((0.1 * (next - 0.1) / 0.1 - 0.81).abs() < 1e-12, "{next}");
    // A step below twice the delay is read whole: the sqrt rule applies.
    let short = control(0.01, Some(0.1));
    assert!((short.next_step - 0.18).abs() < 1e-12, "{short:?}");
    // A delay of half the interval or more reads all of it.
    assert_eq!(
        control(1.0, Some(0.5)).normalized_error,
        control(1.0, None).normalized_error
    );
}

/// The input-referred budget replaces the current tolerance only where it is
/// the larger of the two.
#[test]
fn phase_interpolation_takes_the_larger_of_the_current_and_input_budgets() {
    let buffer = history(&[(0.0, 0.0), (1.0, 1.0)], 10.0);
    let control = |abstol, input_referred| {
        phase_interpolation_control_with_slope(
            &buffer,
            None,
            2.0,
            4.0,
            PhaseInterpolationTolerance {
                reltol: 0.0,
                abstol,
                input_referred,
            },
            None,
            0,
        )
        .unwrap()
        .normalized_error
    };
    assert!((control(0.01, 0.0) - 25.0).abs() < 1e-12);
    assert!((control(0.01, 0.5) - 0.5).abs() < 1e-15);
    assert!((control(1.0, 0.01) - 0.25).abs() < 1e-15);
    for invalid in [Value::NAN, Value::INFINITY, -1.0] {
        assert!(
            phase_interpolation_control_with_slope(
                &buffer,
                None,
                2.0,
                4.0,
                PhaseInterpolationTolerance {
                    reltol: 0.0,
                    abstol: 1.0,
                    input_referred: invalid,
                },
                None,
                0,
            )
            .is_err()
        );
    }
}

/// On a smooth signal with a delay far below the step, the windowed control
/// takes about a tenth of the steps (334 against 3,785), and every value the
/// history can supply, read at `t - delay` for any later `t`, stays within
/// the slack the whole-interval control is held to: 1.67e-9 here against a
/// 1e-9 tolerance, where the whole-interval control's worst is 1.21e-9.
#[test]
fn phase_interpolation_window_holds_every_delayed_read_to_the_tolerance() {
    let delay: Value = 1e-5;
    let tolerance: Value = 1e-9;
    let f = |t: Value| 1e-3 * (2.0 + (std::f64::consts::TAU * t).sin());
    let mut buffer = DelayBuffer::new(4);
    buffer.accept_sample(0.0, f(0.0), delay, None).unwrap();
    let (mut time, mut dt, mut points, mut worst): (Value, Value, usize, Value) =
        (0.0, 0.03, 1, 0.0);
    while time < 2.0 {
        dt = dt.min(2.0 - time).min(0.03);
        let next = time + dt;
        let current = f(next);
        let control = phase_interpolation_control_with_slope(
            &buffer,
            None,
            next,
            current,
            PhaseInterpolationTolerance {
                reltol: 0.0,
                abstol: tolerance,
                input_referred: 0.0,
            },
            Some(delay),
            0,
        )
        .unwrap();
        if control.normalized_error > 1.0 {
            dt = control.next_step;
            continue;
        }
        buffer.accept_sample(next, current, delay, None).unwrap();
        // The history's linear interpolant, read at `t - delay` for the solve
        // at `next` and every later `t` that still lands in this interval.
        for i in 0..=32 {
            let at = next - f64::from(i) / 32.0 * delay.min(next - time);
            let fraction = (at - time) / (next - time);
            let interpolated = (1.0 - fraction) * f(time) + fraction * current;
            worst = worst.max((interpolated - f(at)).abs());
        }
        time = next;
        points += 1;
        dt = control.next_step;
        assert!(points < 100_000);
    }
    let (unwindowed, ..) = smooth_grid(tolerance, delay, false);
    assert!(
        points * 3 < unwindowed.len(),
        "windowed {points} against whole-interval {}",
        unwindowed.len()
    );
    // The same three-knot curvature estimate, and the same slack, as the
    // whole-interval control's refinement test above.
    assert!(
        worst < 3.0 * tolerance,
        "worst delayed read error {worst:e}"
    );
}
