use rspice_veriloga_runtime::transport_delay::{DelayBuffer, DelayCheckpoint, DelayConfiguration};

#[test]
fn direct_acceptance_matches_staged_history_through_pruning_and_restore() {
    for bounded in [false, true] {
        let mut direct = DelayBuffer::new(0);
        let mut staged = DelayBuffer::new(0);
        for index in 0..80 {
            let time = index as f64 * 0.125;
            let value = time.sin();
            let delay = if bounded {
                0.25 + 0.01 * index as f64
            } else {
                0.75
            };
            let maximum = bounded.then_some(0.75);
            let before = direct.clone();
            direct.validate_sample(time, value, delay, maximum).unwrap();
            assert_eq!(direct, before);
            direct.accept_sample(time, value, delay, maximum).unwrap();
            staged.begin_evaluation();
            staged.eval(time, value, delay, maximum).unwrap();
            staged.commit().unwrap();
            assert_eq!(direct, staged);
            direct.validate_accepted_time(time).unwrap();
            assert_eq!(
                direct.accepted_samples().collect::<Vec<_>>(),
                direct.checkpoint().samples
            );
            if index == 40 {
                direct = DelayBuffer::from_checkpoint(direct.checkpoint()).unwrap();
            }
        }
        assert!(direct.accepted_sample_count() < 10);
    }
}

#[test]
fn failed_direct_acceptance_preserves_staged_and_accepted_state() {
    let mut history = DelayBuffer::new(0);
    history.accept_sample(0.0, 2.0, 1.0, None).unwrap();
    history.eval(0.5, 3.0, 1.0, None).unwrap();
    let original = history.clone();
    for (time, value, delay, maximum) in [
        (0.0, 5.0, 1.0, None),
        (-1.0, 5.0, 1.0, None),
        (f64::INFINITY, 5.0, 1.0, None),
        (0.25, f64::NAN, 1.0, None),
        (0.25, 5.0, 1.0, Some(f64::NAN)),
    ] {
        assert!(
            history
                .validate_sample(time, value, delay, maximum)
                .is_err()
        );
        assert!(history.accept_sample(time, value, delay, maximum).is_err());
        assert_eq!(history, original);
    }
    history.commit().unwrap();
    assert_eq!(history.accepted_samples().next_back(), Some((0.5, 3.0)));
}

#[test]
fn direct_acceptance_obeys_frozen_and_dynamic_delay_definitions() {
    let mut fixed = DelayBuffer::new(0);
    assert!(fixed.accept_sample(0.0, 1.0, -1.0, None).is_err());
    fixed.accept_sample(0.0, 1.0, 0.75, None).unwrap();
    // The accepted fixed definition replaces all later authored td values.
    fixed.accept_sample(0.5, 2.0, f64::NAN, None).unwrap();
    assert_eq!(
        fixed.accepted_configuration(),
        Some(DelayConfiguration::Fixed { delay: 0.75 })
    );
    let mut bounded = DelayBuffer::new(0);
    bounded.accept_sample(0.0, 1.0, 0.5, Some(0.75)).unwrap();
    let before = bounded.clone();
    assert!(bounded.accept_sample(0.5, 2.0, -1.0, Some(0.75)).is_err());
    assert_eq!(bounded, before);
    bounded
        .accept_sample(0.5, 2.0, 0.5, Some(f64::NAN))
        .unwrap();
    assert_eq!(
        bounded.accepted_configuration(),
        Some(DelayConfiguration::Bounded { max_delay: 0.75 })
    );
}

#[test]
fn direct_acceptance_selects_accepted_sample_without_leaking_trial() {
    let mut history = DelayBuffer::new(0);
    history.eval(0.0, 900.0, 2.0, None).unwrap();
    history.accept_sample(0.0, -0.0, 0.25, None).unwrap();
    assert_eq!(
        history.accepted_configuration(),
        Some(DelayConfiguration::Fixed { delay: 0.25 })
    );
    assert_eq!(
        history.accepted_samples().next().unwrap().1.to_bits(),
        (-0.0f64).to_bits()
    );
    history.validate_checkpoint_ready().unwrap();
    history.eval(0.8, 800.0, 0.25, None).unwrap();
    history.accept_sample(0.4, 4.0, 0.25, None).unwrap();
    history.validate_accepted_time(0.4).unwrap();
    assert_eq!(history.accepted_samples().next_back(), Some((0.4, 4.0)));
}

#[test]
fn accepted_capture_rejects_stale_time_trial_and_missing_exact_bracket() {
    let delay = 0.5_f64.next_up();
    let mut history = DelayBuffer::from_checkpoint(DelayCheckpoint {
        event_orders: Vec::new(),
        left_limits: Vec::new(),
        configuration: Some(DelayConfiguration::Fixed { delay }),
        samples: vec![(0.0, 1.0), (0.5, 2.0), (1.0, 3.0)],
    })
    .unwrap();
    history.validate_accepted_time(1.0).unwrap();
    assert!(history.validate_accepted_time(1.0_f64.next_up()).is_err());
    history.eval(1.25, 4.0, delay, None).unwrap();
    assert!(
        history
            .validate_accepted_time(1.0)
            .unwrap_err()
            .contains("in-flight")
    );
    let missing = DelayBuffer::from_checkpoint(DelayCheckpoint {
        event_orders: Vec::new(),
        left_limits: Vec::new(),
        configuration: Some(DelayConfiguration::Fixed { delay }),
        samples: vec![(0.5, 2.0), (1.0, 3.0)],
    })
    .unwrap();
    assert!(
        missing
            .validate_accepted_time(1.0)
            .unwrap_err()
            .contains("interpolation bracket")
    );
    assert!(
        DelayBuffer::from_checkpoint(DelayCheckpoint {
            event_orders: Vec::new(),
            left_limits: Vec::new(),
            configuration: Some(DelayConfiguration::Fixed { delay }),
            samples: vec![(0.0, 1.0), (0.0, 2.0)],
        })
        .is_err()
    );
}
