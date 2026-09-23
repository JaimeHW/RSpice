use super::*;

fn event(time: Value) -> TransmissionLineHistoryEvent {
    TransmissionLineHistoryEvent {
        time,
        incoming: [0.0, 0.0, 0.0, 0.0],
        outgoing: [1.0, 0.0, -2.0, 0.0],
        incoming_wave_slopes: [0.0; 2],
        outgoing_wave_slopes: [0.0; 2],
    }
}

#[test]
fn sampled_delay_event_promotion_preserves_rates_and_rejects_partial_state() {
    for outgoing_at_event in [false, true] {
        let time = -1.0_f64;
        let incoming = if outgoing_at_event {
            time.next_down()
        } else {
            time
        };
        let outgoing = if outgoing_at_event {
            time
        } else {
            time.next_up()
        };
        let mut line = TransmissionLine::new("T1".into(), 1, 0, 2, 0, 50.0, 2.25);
        for (t, value) in [
            (-2.0, -1.0),
            (-1.5, 0.0),
            (incoming, 2.0_f64.mul_add(incoming + 1.0, 1.0)),
            (outgoing, 3.0_f64.mul_add(outgoing + 1.0, 4.0)),
            (-0.75, 4.75),
            (-0.25, 6.25),
            (0.0, 7.0),
        ] {
            line.update_history(t, value, 0.0, -2.0 * value, 0.0);
        }
        let before = line.checkpoint_state().unwrap();
        line.promote_sampled_history_events(&[]).unwrap();
        assert_eq!(line.checkpoint_state().unwrap(), before);
        // Validate every declared edge before changing even the first one.
        assert!(
            line.promote_sampled_history_events(&[
                [time, incoming, outgoing],
                [-0.5, (-0.5_f64).next_down(), -0.5],
            ])
            .is_err()
        );
        assert_eq!(line.checkpoint_state().unwrap(), before);
        line.promote_sampled_history_events(&[[time, incoming, outgoing]])
            .unwrap();
        assert_eq!(
            line.next_history_event_arrival_after(0.0).unwrap(),
            Some(1.25)
        );
        for (side, value, slope) in [
            (TransmissionLineTimeSide::Incoming, 1.0, 2.0),
            (TransmissionLineTimeSide::Outgoing, 4.0, 3.0),
        ] {
            assert!((line.lossless_wave_on_side(1.25, true, side) - value).abs() < 1e-14);
            assert!(
                (line.lossless_wave_slope_on_side(1.25, true, side).unwrap() - slope).abs() < 1e-14
            );
            assert!(
                (line.lossless_wave_slope_on_side(1.25, false, side).unwrap() + 2.0 * slope).abs()
                    < 1e-14
            );
        }
        for (t, value) in [(1.0, 0.5), (1.375, 4.375), (2.0, 6.25)] {
            assert!(
                (line.lossless_wave_on_side(t, true, TransmissionLineTimeSide::Outgoing) - value)
                    .abs()
                    < 1e-14
            );
        }
        let saved = line.checkpoint_state().unwrap();
        assert_eq!(saved.state_history.len(), before.state_history.len() - 1);
        line.restore_checkpoint_state(&saved).unwrap();
        assert_eq!(line.checkpoint_state().unwrap(), saved);
    }
}

#[test]
fn sided_line_event_preserves_limits_interpolation_and_exact_arrivals() {
    for mode in [
        DelayedInterpolationMode::Linear,
        DelayedInterpolationMode::Quadratic,
        DelayedInterpolationMode::Mixed,
        DelayedInterpolationMode::XyceTra,
    ] {
        let mut line = TransmissionLine::new("T1".into(), 1, 0, 2, 0, 50.0, 1.0);
        line.lossless_interpolation_mode = mode;
        line.update_history(0.0, 0.0, 0.0, 0.0, 0.0);
        let launch = 2.0_f64.powi(-54);
        line.accept_history_event(event(launch)).unwrap();
        line.update_history(0.5, 1.0, 0.0, -2.0, 0.0);
        let clock = 1.0_f64.next_up();
        assert_eq!(
            line.next_history_event_arrival_after(1.0).unwrap(),
            Some(clock)
        );
        assert_eq!(
            line.lossless_wave_on_side(clock, true, TransmissionLineTimeSide::Incoming),
            0.0
        );
        assert_eq!(
            line.lossless_wave_on_side(clock, true, TransmissionLineTimeSide::Outgoing),
            1.0
        );
        assert_eq!(
            line.lossless_wave_on_side(clock, false, TransmissionLineTimeSide::Outgoing),
            -2.0
        );
        assert_eq!(
            line.transient_port_response_on_side(clock, TransmissionLineTimeSide::Incoming)
                .i_eq_port2(),
            0.0
        );
        assert_eq!(
            line.transient_port_response_on_side(clock, TransmissionLineTimeSide::Outgoing)
                .i_eq_port2(),
            0.02
        );
        for (time, expected) in [(1.0, 0.0), (clock, 1.0), (1.25, 1.0)] {
            assert_eq!(
                line.lossless_wave_on_side(time, true, TransmissionLineTimeSide::Outgoing),
                expected
            );
            assert_eq!(line.delayed_forward_raw_at(time), expected);
        }
        assert_eq!(line.next_history_event_arrival_after(clock).unwrap(), None);
        let saved = line.checkpoint_state().unwrap();
        let mut restored = line.clone();
        restored.reset();
        restored.restore_checkpoint_state(&saved).unwrap();
        assert_eq!(restored.checkpoint_state().unwrap(), saved);
        assert_eq!(
            restored.lossless_wave_on_side(clock, true, TransmissionLineTimeSide::Incoming),
            0.0
        );
        assert_eq!(
            restored.lossless_wave_on_side(clock, true, TransmissionLineTimeSide::Outgoing),
            1.0
        );
    }
}

#[test]
fn sided_line_event_rebases_and_rejects_bad_state_without_mutation() {
    let mut line = TransmissionLine::new("T1".into(), 1, 0, 2, 0, 50.0, 1.0);
    line.update_history(0.0, 0.0, 0.0, 0.0, 0.0);
    line.accept_history_event(event(0.25)).unwrap();
    line.update_history(0.5, 1.0, 0.0, -2.0, 0.0);
    line.rebase_lossless_history(0.0).unwrap();
    let saved = line.checkpoint_state().unwrap();
    assert_eq!(saved.events[0][0], -0.25);
    assert_eq!(
        line.next_history_event_arrival_after(0.0).unwrap(),
        Some(0.75)
    );
    assert_eq!(
        line.lossless_wave_on_side(0.75, true, TransmissionLineTimeSide::Incoming),
        0.0
    );
    assert_eq!(
        line.lossless_wave_on_side(0.75, true, TransmissionLineTimeSide::Outgoing),
        1.0
    );
    let mut invalid = event(0.25);
    invalid.incoming_wave_slopes[0] = Value::NAN;
    assert!(line.accept_history_event(invalid).is_err());
    assert_eq!(line.checkpoint_state().unwrap(), saved);
    let mut invalid = saved.clone();
    invalid.events[0][0] = -0.125;
    assert!(line.restore_checkpoint_state(&invalid).is_err());
    assert_eq!(line.checkpoint_state().unwrap(), saved);
    invalid = saved.clone();
    invalid.events[0][7] = 1.0;
    assert!(line.restore_checkpoint_state(&invalid).is_err());
    assert_eq!(line.checkpoint_state().unwrap(), saved);
}

#[test]
fn sided_line_event_preserves_finite_slopes_and_initial_anchor() {
    let mut line = TransmissionLine::new("T1".into(), 1, 0, 2, 0, 50.0, 1.0);
    line.update_history(0.0, 0.0, 0.0, 0.0, 0.0);
    line.update_history(0.125, 0.25, 0.0, 0.0, 0.0);
    line.accept_history_event(TransmissionLineHistoryEvent {
        time: 0.25,
        incoming: [0.5, 0.0, 0.0, 0.0],
        outgoing: [1.5, 0.0, 0.0, 0.0],
        incoming_wave_slopes: [2.0, 0.0],
        outgoing_wave_slopes: [2.0, 0.0],
    })
    .unwrap();
    line.update_history(0.5, 2.0, 0.0, 0.0, 0.0);
    for (time, value) in [(1.1875, 0.375), (1.25, 1.5), (1.3125, 1.625)] {
        assert_eq!(
            line.lossless_wave_on_side(time, true, TransmissionLineTimeSide::Outgoing),
            value
        );
        assert_eq!(line.delayed_forward_raw_at(time), value);
    }
    let saved = line.checkpoint_state().unwrap();
    assert_eq!(saved.events[0][5..], [2.0, 0.0, 2.0, 0.0]);
    line.restore_checkpoint_state(&saved).unwrap();
    assert_eq!(line.delayed_forward_raw_at(1.3125), 1.625);

    line.reset();
    line.update_history(0.0, 0.0, 0.0, 0.0, 0.0);
    line.accept_history_event(event(0.0)).unwrap();
    assert_eq!(line.state_history.len(), 1);
    assert_eq!(
        line.lossless_wave_on_side(1.0, true, TransmissionLineTimeSide::Incoming),
        0.0
    );
    assert_eq!(
        line.lossless_wave_on_side(1.0, true, TransmissionLineTimeSide::Outgoing),
        1.0
    );
    assert_eq!(line.delayed_forward_raw_at(0.5), 0.0);
    line.checkpoint_state().unwrap();
}

#[test]
fn line_event_slopes_follow_native_interpolation_and_solved_limits() {
    for mode in [
        DelayedInterpolationMode::Linear,
        DelayedInterpolationMode::Quadratic,
        DelayedInterpolationMode::Mixed,
        DelayedInterpolationMode::XyceTra,
    ] {
        let mut line = TransmissionLine::new("T1".into(), 1, 0, 2, 0, 50.0, 4.0);
        line.lossless_interpolation_mode = mode;
        for t in [0.0, 1.0, 2.0] {
            line.update_history(t, t * t, 0.0, -t * t, 0.0);
        }
        // Linear has slope 3; the other modes select t^2 here, with slope 2t.
        let expected = if matches!(mode, DelayedInterpolationMode::Linear) {
            3.0
        } else {
            2.5
        };
        for (forward, sign) in [(true, 1.0), (false, -1.0)] {
            assert_eq!(
                line.lossless_wave_slope_on_side(5.25, forward, TransmissionLineTimeSide::Outgoing)
                    .unwrap(),
                sign * expected
            );
        }
        line.accept_history_event(TransmissionLineHistoryEvent {
            time: 3.0,
            incoming: [9.0, 0.0, -9.0, 0.0],
            outgoing: [19.0, 0.0, -19.0, 0.0],
            incoming_wave_slopes: [6.0, -6.0],
            outgoing_wave_slopes: [2.0, -2.0],
        })
        .unwrap();
        for (side, expected) in [
            (TransmissionLineTimeSide::Incoming, 6.0),
            (TransmissionLineTimeSide::Outgoing, 2.0),
        ] {
            assert_eq!(
                line.lossless_wave_slope_on_side(7.0, true, side).unwrap(),
                expected
            );
        }
        line.update_history(4.0, 21.0, 0.0, -21.0, 0.0);
        assert_eq!(
            line.lossless_wave_slope_on_side(7.5, true, TransmissionLineTimeSide::Outgoing)
                .unwrap(),
            2.0
        );
    }
    // Mixed rejects a quadratic overshoot and TRA selects its flat-segment
    // fallback. Their slopes must follow that same choice, not the parabola.
    for mode in [
        DelayedInterpolationMode::Quadratic,
        DelayedInterpolationMode::Mixed,
        DelayedInterpolationMode::XyceTra,
    ] {
        let mut line = TransmissionLine::new("T1".into(), 1, 0, 2, 0, 50.0, 4.0);
        line.lossless_interpolation_mode = mode;
        for (t, v) in [(0.0, 200.0), (1.0, 0.0), (2.0, 0.0)] {
            line.update_history(t, v, 0.0, 0.0, 0.0);
        }
        let expected = if matches!(mode, DelayedInterpolationMode::Quadratic) {
            -50.0
        } else {
            0.0
        };
        assert_eq!(
            line.lossless_wave_slope_on_side(5.25, true, TransmissionLineTimeSide::Outgoing)
                .unwrap(),
            expected
        );
        if matches!(mode, DelayedInterpolationMode::Mixed) {
            for (time, side) in [
                (5.0, TransmissionLineTimeSide::Outgoing),
                (6.0, TransmissionLineTimeSide::Incoming),
            ] {
                assert_eq!(
                    line.lossless_wave_slope_on_side(time, true, side).unwrap(),
                    0.0
                );
            }
        }
    }
    // The receiver clock rounds upward: both derivatives still belong to
    // the exact launch, not to an unrelated nearby interpolation segment.
    let mut line = TransmissionLine::new("T1".into(), 1, 0, 2, 0, 50.0, 1.0);
    line.update_history(0.0, 0.0, 0.0, 0.0, 0.0);
    let mut edge = event(2.0_f64.powi(-54));
    edge.incoming_wave_slopes = [2.0, -3.0];
    edge.outgoing_wave_slopes = [5.0, -7.0];
    line.accept_history_event(edge).unwrap();
    for (side, expected) in [
        (TransmissionLineTimeSide::Incoming, 2.0),
        (TransmissionLineTimeSide::Outgoing, 5.0),
    ] {
        assert_eq!(
            line.lossless_wave_slope_on_side(1.0_f64.next_up(), true, side)
                .unwrap(),
            expected
        );
    }
    line.reset();
    edge.time = 0.0;
    line.accept_history_event(edge).unwrap();
    line.update_history(1.0, 6.0, 0.0, -9.0, 0.0);
    line.rebase_lossless_history(0.0).unwrap();
    // t-TD rounds to the old event, but its positive low word is strictly
    // after that arrival: even an incoming query must use the outgoing wave.
    let time = 2.0_f64.powi(-54);
    assert_eq!(
        line.lossless_wave_on_side(time, true, TransmissionLineTimeSide::Incoming),
        5.0_f64.mul_add(time, 1.0)
    );
    assert_eq!(
        line.lossless_wave_slope_on_side(time, true, TransmissionLineTimeSide::Incoming)
            .unwrap(),
        5.0
    );
}
