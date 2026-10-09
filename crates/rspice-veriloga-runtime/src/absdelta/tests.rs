use super::*;

fn controls() -> AbsDeltaControls {
    AbsDeltaControls {
        delta: 0.25,
        time_tolerance: 0.0,
        expression_tolerance: 0.01,
        enable: 1.0,
    }
}

fn interval(
    state: AbsDeltaState,
    time: f64,
    value: f64,
    controls: AbsDeltaControls,
) -> AbsDeltaInterval {
    AbsDeltaInterval::new(
        state,
        AbsDeltaSample { time, value },
        controls,
        1e-12,
        false,
    )
    .unwrap()
}

fn consume(mut interval: AbsDeltaInterval) -> (AbsDeltaState, Vec<AbsDeltaEvent>) {
    let mut events = Vec::new();
    while let Some(event) = interval.next_event().unwrap() {
        assert!(events.len() < 32, "observer did not make bounded progress");
        events.push(event);
    }
    (interval.candidate().unwrap(), events)
}

fn initialized(value: f64) -> AbsDeltaState {
    consume(interval(AbsDeltaState::default(), 0.0, value, controls())).0
}

#[test]
fn checkpoint_retains_suppressed_events_and_turning_points() {
    let control = AbsDeltaControls {
        time_tolerance: 1.0,
        expression_tolerance: 0.05,
        ..controls()
    };
    let (pending, events) = consume(interval(initialized(-0.0), 0.1, 1.0, control));
    assert!(pending.pending && events.is_empty());
    let mut original = pending;
    let mut restored = AbsDeltaState::from_checkpoint_words(&pending.checkpoint_words()).unwrap();
    for (time, value) in [(1.0, 1.0), (1.1, 0.98), (1.2, 0.94), (1.3, 1.2), (2.4, 1.2)] {
        let (next, events) = consume(interval(original, time, value, control));
        let (resumed, actual) = consume(interval(restored, time, value, control));
        assert_eq!(events, actual);
        assert_eq!(next.checkpoint_words(), resumed.checkpoint_words());
        for event in &events {
            assert_eq!(
                AbsDeltaState::from_checkpoint_words(&event.state.checkpoint_words()).unwrap(),
                event.state
            );
        }
        original = next;
        restored = AbsDeltaState::from_checkpoint_words(&resumed.checkpoint_words()).unwrap();
    }
    let zero = initialized(-0.0).checkpoint_words();
    assert_eq!(
        AbsDeltaState::from_checkpoint_words(&zero)
            .unwrap()
            .checkpoint_words(),
        zero
    );
    assert_eq!(zero[3], (-0.0f64).to_bits());
}

#[test]
fn checkpoint_retains_disabled_and_uninitialized_observers() {
    let disabled = AbsDeltaControls {
        enable: 0.0,
        ..controls()
    };
    let (late, _) = consume(interval(initialized(0.0), 2.0, 1.0, controls()));
    let (reset, _) = consume(
        AbsDeltaInterval::new(
            late,
            AbsDeltaSample {
                time: 0.0,
                value: -0.0,
            },
            disabled,
            1e-12,
            true,
        )
        .unwrap(),
    );
    for state in [
        AbsDeltaState::default(),
        reset,
        consume(interval(AbsDeltaState::default(), 0.0, 0.5, disabled)).0,
    ] {
        let restored = AbsDeltaState::from_checkpoint_words(&state.checkpoint_words()).unwrap();
        assert_eq!(restored.checkpoint_words(), state.checkpoint_words());
        assert_eq!(
            consume(interval(state, 3.0, 0.3, controls())),
            consume(interval(restored, 3.0, 0.3, controls()))
        );
    }
}

#[test]
fn checkpoint_rejects_invalid_observer_histories() {
    let good = initialized(0.0).checkpoint_words();
    for (lane, value) in [
        (0, 2),
        (1, 16),
        (2, (-1.0f64).to_bits()),
        (3, f64::NAN.to_bits()),
        (4, 1.0f64.to_bits()),
        (5, f64::INFINITY.to_bits()),
        (6, 3),
        (7, f64::NAN.to_bits()),
        (7, 1.0f64.to_bits()),
        (1, 5),
        (1, 15),
        (1, 6),
    ] {
        let mut bad = good;
        bad[lane] = value;
        assert!(
            AbsDeltaState::from_checkpoint_words(&bad).is_err(),
            "{lane}: {value}"
        );
    }
    assert!(AbsDeltaState::from_checkpoint_words(&good[..7]).is_err());
    let mut bad = AbsDeltaState::default().checkpoint_words();
    bad[3] = 1;
    assert!(AbsDeltaState::from_checkpoint_words(&bad).is_err());
}

fn close(actual: f64, expected: f64) {
    assert!((actual - expected).abs() < 1e-12, "{actual} != {expected}");
}

#[test]
fn initialization_dc_and_enable_transitions_sample_once() {
    let (first, events) = consume(interval(AbsDeltaState::default(), 0.0, 0.5, controls()));
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].reason, AbsDeltaReason::Initialization);
    let (_, events) = consume(interval(first, 0.0, 0.5, controls()));
    assert!(events.is_empty());
    let (_, events) = consume(
        AbsDeltaInterval::new(
            first,
            AbsDeltaSample {
                time: 0.0,
                value: 0.5,
            },
            controls(),
            1e-12,
            true,
        )
        .unwrap(),
    );
    assert_eq!(
        events.len(),
        1,
        "each DC sweep point initializes its observer"
    );
    let disabled = AbsDeltaControls {
        enable: 0.0,
        ..controls()
    };
    let (off, events) = consume(interval(first, 1.0, 10.0, disabled));
    assert!(events.is_empty());
    let (on, events) = consume(interval(off, 2.0, 20.0, controls()));
    assert_eq!(
        events.len(),
        1,
        "reenabling does not backfill the disabled interval"
    );
    assert_eq!(events[0].reason, AbsDeltaReason::Enabled);
    assert_eq!(
        on.last_event().unwrap(),
        AbsDeltaSample {
            time: 2.0,
            value: 20.0
        }
    );
    let (off, events) = consume(interval(AbsDeltaState::default(), 0.0, 0.0, disabled));
    assert!(events.is_empty());
    assert_eq!(
        consume(interval(off, 1.0, 0.0, controls())).1[0].reason,
        AbsDeltaReason::Enabled
    );
}

#[test]
fn one_analog_interval_produces_multiple_unquantized_events() {
    let initial = initialized(0.0);
    let mut observer = AbsDeltaInterval::new(
        initial,
        AbsDeltaSample {
            time: 1.0,
            value: 1.0,
        },
        controls(),
        0.1,
        false,
    )
    .unwrap();
    assert!(observer.candidate().is_none());
    for expected in [0.25, 0.5, 0.75, 1.0] {
        let event = observer.next_event().unwrap().unwrap();
        assert_eq!(event.reason, AbsDeltaReason::Delta);
        close(event.sample.time, expected);
        close(event.sample.value, expected);
    }
    assert!(observer.next_event().unwrap().is_none());
    assert_eq!(
        observer.candidate().unwrap().last_sample().unwrap().time,
        1.0
    );
    assert_eq!(
        initial.last_event().unwrap().time,
        0.0,
        "candidate enumeration cannot mutate accepted history"
    );
    let (plateau, events) = consume(interval(initial, 1.0, 0.0, controls()));
    assert!(events.is_empty());
    let (_, events) = consume(interval(plateau, 1.0, 1.0, controls()));
    assert_eq!(events.len(), 1, "a same-time analog jump emits once");
    assert_eq!(
        events[0].sample,
        AbsDeltaSample {
            time: 1.0,
            value: 1.0
        }
    );
}

#[test]
fn time_tolerance_defers_changes_but_control_changes_are_not_events() {
    let control = AbsDeltaControls {
        time_tolerance: 0.4,
        ..controls()
    };
    let (_, events) = consume(interval(initialized(0.0), 1.0, 1.0, control));
    assert_eq!(events.len(), 2);
    close(events[0].sample.time, 0.4);
    close(events[1].sample.time, 0.8);
    let control = AbsDeltaControls {
        time_tolerance: 1.0,
        ..controls()
    };
    let (pending, events) = consume(interval(initialized(0.0), 0.1, 1.0, control));
    assert!(events.is_empty());
    let (_, events) = consume(interval(pending, 1.0, 1.0, control));
    assert_eq!(events.len(), 1);
    close(events[0].sample.time, 1.0);
    let (_, events) = consume(interval(
        pending,
        1.0,
        1.0,
        AbsDeltaControls {
            delta: 0.0,
            ..control
        },
    ));
    assert!(
        events.is_empty(),
        "zero-delta mode does not deliver a pending unchanged value"
    );
    let (late, _) = consume(interval(AbsDeltaState::default(), 0.4, 0.0, controls()));
    let (_, events) = consume(interval(
        late,
        1.0,
        1.0,
        AbsDeltaControls {
            delta: 0.1,
            time_tolerance: 0.1,
            ..controls()
        },
    ));
    let mut previous = 0.4;
    for event in events {
        assert!(
            event.sample.time - previous >= 0.1,
            "rounded time addition must respect time_tol"
        );
        previous = event.sample.time;
    }
    let (unchanged, _) = consume(interval(initialized(0.0), 0.1, 0.125, controls()));
    let (_, events) = consume(interval(
        unchanged,
        0.2,
        0.125,
        AbsDeltaControls {
            delta: 0.05,
            ..controls()
        },
    ));
    assert!(
        events.is_empty(),
        "shrinking delta does not itself change the expression"
    );
}

#[test]
fn direction_changes_accumulate_from_the_extremum_and_ignore_time_tolerance() {
    let (peak, _) = consume(interval(initialized(0.0), 1.0, 1.0, controls()));
    let (noise, events) = consume(interval(peak, 1.1, 0.995, controls()));
    assert!(events.is_empty());
    let (_, events) = consume(interval(
        noise,
        1.2,
        0.98,
        AbsDeltaControls {
            time_tolerance: 10.0,
            ..controls()
        },
    ));
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].reason, AbsDeltaReason::Direction);
    close(events[0].sample.value, 0.99);
    close(events[0].sample.time, 1.1 + 0.1 / 3.0);
}

#[test]
fn zero_delta_uses_only_changed_endpoints_and_ignores_tolerances() {
    let control = AbsDeltaControls {
        delta: 0.0,
        time_tolerance: 100.0,
        expression_tolerance: 100.0,
        enable: 1.0,
    };
    let (state, events) = consume(interval(initialized(0.0), 1.0, 10.0, control));
    assert_eq!(events.len(), 1);
    assert_eq!(
        events[0].sample,
        AbsDeltaSample {
            time: 1.0,
            value: 10.0
        }
    );
    assert_eq!(events[0].reason, AbsDeltaReason::ValueChange);
    let (_, events) = consume(interval(state, 2.0, 10.0, control));
    assert!(events.is_empty());
    let (_, events) = consume(interval(state, 1.0, 11.0, control));
    assert_eq!(
        events.len(),
        1,
        "a same-time analog discontinuity is still a value change"
    );
}

#[test]
fn rejected_and_truncated_intervals_do_not_publish_future_events() {
    let accepted = initialized(0.0);
    let first = {
        let mut rejected = interval(accepted, 1.0, 1.0, controls());
        let first = rejected.next_event().unwrap().unwrap();
        assert!(rejected.candidate().is_none());
        close(first.sample.time, 0.25);
        first
    };
    let (_, retry) = consume(interval(accepted, 1.0, 1.0, controls()));
    assert_eq!(retry.len(), 4);
    assert_eq!(retry[0], first);
    // An analog feedback solve replaces the extrapolated remainder with a plateau.
    let (_, revised) = consume(interval(first.state, 1.0, 0.25, controls()));
    assert!(revised.is_empty());
}

#[test]
fn finite_extreme_interpolation_and_invalid_operands_are_explicit() {
    let control = AbsDeltaControls {
        delta: f64::MAX,
        ..controls()
    };
    let (_, events) = consume(interval(initialized(-f64::MAX), 2.0, f64::MAX, control));
    assert_eq!(events.len(), 2);
    close(events[0].sample.time, 1.0);
    assert_eq!(events[0].sample.value, 0.0);
    assert_eq!(events[1].sample.value, f64::MAX);
    for (control, expected) in [
        (
            AbsDeltaControls {
                delta: -1.0,
                ..controls()
            },
            AbsDeltaError::InvalidDelta,
        ),
        (
            AbsDeltaControls {
                time_tolerance: f64::NAN,
                ..controls()
            },
            AbsDeltaError::InvalidTimeTolerance,
        ),
        (
            AbsDeltaControls {
                expression_tolerance: -1.0,
                ..controls()
            },
            AbsDeltaError::InvalidExpressionTolerance,
        ),
        (
            AbsDeltaControls {
                enable: 0.5,
                ..controls()
            },
            AbsDeltaError::InvalidEnable,
        ),
    ] {
        assert_eq!(
            AbsDeltaInterval::new(
                AbsDeltaState::default(),
                AbsDeltaSample {
                    time: 0.0,
                    value: 0.0
                },
                control,
                1e-12,
                false
            )
            .unwrap_err(),
            expected
        );
    }
    assert_eq!(
        AbsDeltaInterval::new(
            initialized(0.0),
            AbsDeltaSample {
                time: 0.0,
                value: f64::INFINITY
            },
            controls(),
            1e-12,
            false
        )
        .unwrap_err(),
        AbsDeltaError::InvalidExpression
    );
}
