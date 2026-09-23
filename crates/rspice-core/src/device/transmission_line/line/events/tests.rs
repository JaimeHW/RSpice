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
