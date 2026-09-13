//! Language-independent delay state contracts used by native device owners.

use rspice_veriloga_runtime::transport_delay::{DelayBuffer, DelayCheckpoint, DelayConfiguration};

#[test]
fn nonlinear_current_candidate_exposes_its_chain_rule_coefficient() {
    let mut delay = DelayBuffer::new(2);
    delay.eval_operating_point(0.0, 1.0, 0.0625, None).unwrap();
    delay.commit().unwrap();
    let accepted = delay.checkpoint();
    let voltage = 0.2_f64;
    let current = voltage.exp();
    let candidate = delay
        .eval_with_coefficients(0.25, current, 0.0625, None)
        .unwrap();
    let expected = 0.25 + 0.75 * current;
    assert!((candidate.output - expected).abs() <= 2.0 * f64::EPSILON * expected);
    assert_eq!(candidate.input_coefficient, 0.75);
    assert_eq!(candidate.delay_coefficient, 0.0);
    let delta = 1e-5;
    let plus = delay
        .eval(0.25, (voltage + delta).exp(), 0.0625, None)
        .unwrap();
    let minus = delay
        .eval(0.25, (voltage - delta).exp(), 0.0625, None)
        .unwrap();
    assert!(((plus - minus) / (2.0 * delta) - 0.75 * current).abs() < 3e-11);
    assert_eq!(delay.checkpoint(), accepted);
    delay.begin_evaluation();
    assert!(delay.validate_checkpoint_ready().is_ok());
}

fn seeded() -> DelayBuffer {
    let mut delay = DelayBuffer::new(2);
    for (time, current) in [(0.0, 1.0), (0.25, 2.0), (0.5, 4.0)] {
        delay.eval(time, current, 0.375, None).unwrap();
        delay.commit().unwrap();
    }
    delay
}

#[test]
fn invalid_public_restoration_preserves_accepted_and_candidate_state() {
    let mut delay = seeded();
    delay.eval(0.75, 8.0, 0.375, None).unwrap();
    let original = delay.clone();
    let snapshot = delay.checkpoint();
    let mut invalids = Vec::new();
    let mut state = snapshot.clone();
    state.samples[1].0 = state.samples[0].0;
    invalids.push(state);
    let mut state = snapshot.clone();
    state.samples[0].1 = f64::NAN;
    invalids.push(state);
    let mut state = snapshot.clone();
    state.configuration = Some(DelayConfiguration::Fixed { delay: -1.0 });
    invalids.push(state);
    let mut state = snapshot.clone();
    state.configuration = None;
    invalids.push(state);
    for state in invalids {
        assert!(delay.restore_checkpoint(&state).is_err());
        assert_eq!(delay.checkpoint(), snapshot);
        assert!(delay.validate_checkpoint_ready().is_err());
        let mut after = delay.clone();
        let mut before = original.clone();
        after.commit().unwrap();
        before.commit().unwrap();
        assert_eq!(after.checkpoint(), before.checkpoint());
    }
}

#[test]
fn public_restoration_replays_coefficients_and_discards_speculation() {
    let mut baseline = seeded();
    let checkpoint = baseline.checkpoint();
    let mut restored = DelayBuffer::new(0);
    restored.restore_checkpoint(&checkpoint).unwrap();
    let candidate = baseline
        .eval_with_coefficients(0.75, 8.0, 0.375, None)
        .unwrap();
    restored.eval(0.75, 99.0, 0.375, None).unwrap();
    restored.restore_checkpoint(&checkpoint).unwrap();
    assert!(restored.validate_checkpoint_ready().is_ok());
    assert_eq!(
        restored
            .eval_with_coefficients(0.75, 8.0, 0.375, None)
            .unwrap(),
        candidate
    );
    baseline.commit().unwrap();
    restored.commit().unwrap();
    assert_eq!(baseline.checkpoint(), restored.checkpoint());
    assert_eq!(
        baseline.accepted_sample_count(),
        restored.accepted_sample_count()
    );
}

#[test]
fn oversized_public_checkpoint_is_refused_before_it_replaces_state() {
    let mut delay = seeded();
    let before = delay.checkpoint();
    let oversized = DelayCheckpoint {
        configuration: Some(DelayConfiguration::Fixed { delay: 1.0 }),
        samples: vec![(0.0, 0.0); 1_048_577],
    };
    let error = delay.restore_checkpoint(&oversized).unwrap_err();
    assert!(error.contains("exceeds"), "{error}");
    assert_eq!(delay.checkpoint(), before);
}
