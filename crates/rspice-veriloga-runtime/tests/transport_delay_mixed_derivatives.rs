//! Independent mixed partials of the piecewise bilinear delay interpolant.

use rspice_veriloga_runtime::transport_delay::{DelayBuffer, DelayCheckpoint, DelayConfiguration};

fn history(configuration: DelayConfiguration) -> DelayBuffer {
    let mut buffer = DelayBuffer::new(0);
    buffer
        .restore_checkpoint(&DelayCheckpoint {
            samples: vec![(0.0, 1.0), (0.5, 2.0)],
            configuration: Some(configuration),
            left_limits: vec![],
            event_orders: vec![],
        })
        .unwrap();
    buffer
}

#[test]
fn mixed_action_follows_the_primal_interpolation_branch_without_advancing_history() {
    let mut buffer = history(DelayConfiguration::Bounded { max_delay: 2.0 });
    let accepted = buffer.checkpoint();
    for (delay, expected) in [
        (0.25, -24.0),
        (0.5_f64.next_down(), -24.0),
        (0.5, -24.0), // Exact knots use the right-hand interpolation segment.
        (0.5_f64.next_up(), 0.0),
        (0.75, 0.0), // Both endpoints are accepted samples.
        (1.0, 0.0),  // Time-zero anchor.
        (1.5, 0.0),
    ] {
        let evaluation = buffer
            .eval_with_coefficients(1.0, 5.0, delay, Some(2.0))
            .unwrap();
        // On the current segment C = -1/(1-.5); apply p=3, q=4.
        assert_eq!(
            evaluation.apply_mixed_derivative(3.0, 4.0).unwrap(),
            expected
        );
        assert_eq!(buffer.checkpoint(), accepted);
        let observation = buffer
            .static_dae_with_coefficients(1.0, 99.0, delay, Some(2.0))
            .unwrap();
        assert_eq!(observation.apply_mixed_derivative(3.0, 4.0).unwrap(), 0.0);
        assert_eq!(buffer.checkpoint(), accepted);
    }
    buffer.commit().unwrap();
    assert_eq!(buffer.checkpoint().samples.last(), Some(&(1.0, 5.0)));
}

#[test]
fn fixed_clamped_and_initial_delays_have_no_mixed_action() {
    let mut fixed = history(DelayConfiguration::Fixed { delay: 0.25 });
    let evaluation = fixed.eval_with_coefficients(1.0, 5.0, 0.125, None).unwrap();
    assert_eq!(evaluation.apply_mixed_derivative(3.0, 4.0).unwrap(), 0.0);
    let mut bounded = history(DelayConfiguration::Bounded { max_delay: 0.25 });
    for (delay, expected) in [(0.125, -24.0), (0.25, 0.0), (0.5, 0.0)] {
        let evaluation = bounded
            .eval_with_coefficients(1.0, 5.0, delay, Some(0.25))
            .unwrap();
        assert_eq!(
            evaluation.apply_mixed_derivative(3.0, 4.0).unwrap(),
            expected
        );
    }
    let mut initial = DelayBuffer::new(0);
    let evaluation = initial
        .eval_with_coefficients(0.0, 5.0, 0.125, Some(1.0))
        .unwrap();
    assert_eq!(evaluation.apply_mixed_derivative(3.0, 4.0).unwrap(), 0.0);
    assert!(evaluation.apply_mixed_derivative(f64::NAN, 0.0).is_err());
}

#[test]
fn mixed_action_retains_a_representable_result_when_the_coefficient_overflows() {
    let mut buffer = DelayBuffer::new(0);
    buffer
        .eval(0.0, 0.0, 2.0_f64.powi(-1071), Some(1.0))
        .unwrap();
    buffer.commit().unwrap();
    let interval = 2.0_f64.powi(-1070);
    assert!((1.0 / interval).is_infinite());
    let evaluation = buffer
        .eval_with_coefficients(interval, 0.0, interval / 2.0, Some(1.0))
        .unwrap();
    let operand = 2.0_f64.powi(-600);
    assert_eq!(operand * operand, 0.0);
    assert_eq!(
        evaluation.apply_mixed_derivative(operand, operand).unwrap(),
        -2.0_f64.powi(-130)
    );
    assert!(
        evaluation
            .apply_mixed_derivative(f64::MAX, f64::MAX)
            .is_err()
    );
}
