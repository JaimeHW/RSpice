//! A transport correction is a current in its own right, not the subtraction
//! of an already rounded delayed current from a rounded present current.

use rspice_veriloga_runtime::transport_delay::{DelayBuffer, DelayCheckpoint, DelayConfiguration};

fn history(samples: &[(f64, f64)], delay: f64) -> DelayBuffer {
    let mut buffer = DelayBuffer::new(0);
    buffer
        .restore_checkpoint(&DelayCheckpoint {
            left_limits: Vec::new(),
            configuration: Some(DelayConfiguration::Fixed { delay }),
            samples: samples.to_vec(),
        })
        .unwrap();
    buffer
}

fn correction(
    buffer: &DelayBuffer,
    time: f64,
    value: f64,
    delay: f64,
    gradient: f64,
) -> (f64, f64) {
    let result = buffer
        .difference_with_coefficients(time, value, delay, None)
        .unwrap();
    (
        result.output,
        result.apply_input_derivative(gradient).unwrap(),
    )
}

#[test]
fn tiny_phase_correction_and_newton_action_survive_rounded_unity_gain() {
    let delay = 2.0_f64.powi(-80);
    let buffer = history(&[(0.0, 1.0), (0.5, 1.0)], delay);
    // In the current half-second interval I(t)=2t. At t=1 the
    // delayed-minus-present current is -2*tau, with the same input weight.
    let (current, derivative) = correction(&buffer, 1.0, 2.0, delay, 1.0);
    assert_eq!(current, -2.0 * delay);
    assert_eq!(derivative, -2.0 * delay);
}

#[test]
fn accepted_interpolation_preserves_a_sub_ulp_correction() {
    let delay = 0.5_f64.next_down();
    let buffer = history(&[(0.0, 0.0), (1.0, 1.0), (1.25, 1.25)], delay);
    // The retained unit ramp is sampled exactly at 1+2^-54. The present
    // value is 1; rounding the delayed output first would erase 2^-54.
    let (current, derivative) = correction(&buffer, 1.5, 1.0, delay, 7.0);
    assert_eq!(current, 2.0_f64.powi(-54));
    assert_eq!(derivative, -7.0);
}

#[test]
fn subnormal_weight_is_applied_before_rounding_the_newton_action() {
    let delay = f64::from_bits(1);
    let amplitude = 2.0_f64.powi(1000);
    let buffer = history(&[(0.0, 0.0), (4.0, 0.0)], delay);
    // tau/4 rounds to zero, but both current and Jacobian action are -2^-76.
    let (current, derivative) = correction(&buffer, 8.0, amplitude, delay, amplitude);
    assert_eq!(current, -2.0_f64.powi(-76));
    assert_eq!(derivative, -2.0_f64.powi(-76));
}

#[test]
fn correction_does_not_overflow_an_endpoint_difference() {
    let buffer = history(&[(0.0, -f64::MAX)], 1.0);
    let (current, derivative) = correction(&buffer, 4.0, f64::MAX, 1.0, 8.0);
    assert_eq!(current, -0.5 * f64::MAX);
    assert_eq!(derivative, -2.0);
}

#[test]
fn prehistory_and_equilibrium_have_their_own_input_actions() {
    let buffer = history(&[(0.0, 3.0)], 1.0);
    assert_eq!(correction(&buffer, 0.25, 7.0, 1.0, 5.0), (-4.0, -5.0));
    let empty = DelayBuffer::new(0);
    assert_eq!(correction(&empty, 0.0, 7.0, 1.0, 5.0), (0.0, 0.0));
    assert!(empty.validate_checkpoint_ready().is_ok());
    assert!(
        empty
            .difference_with_coefficients(0.1, 7.0, 1.0, None)
            .is_err()
    );
}

#[test]
fn correction_queries_preserve_accepted_and_staged_state() {
    let mut buffer = history(&[(0.0, 0.0), (1.0, 1.0)], 1.0);
    let accepted = buffer.checkpoint();
    buffer.eval_with_coefficients(2.0, 2.0, 1.0, None).unwrap();
    assert!(buffer.validate_checkpoint_ready().is_err());
    assert_eq!(correction(&buffer, 2.0, 99.0, 1.0, 1.0), (-98.0, -1.0));
    assert!(
        buffer
            .difference_with_coefficients(0.5, 0.0, 1.0, None)
            .is_err()
    );
    assert!(
        buffer
            .difference_with_coefficients(2.0, f64::NAN, 1.0, None)
            .is_err()
    );
    assert_eq!(buffer.checkpoint(), accepted);
    assert!(buffer.validate_checkpoint_ready().is_err());
    buffer.commit().unwrap();
    assert_eq!(buffer.checkpoint().samples.last(), Some(&(2.0, 2.0)));
    assert!(buffer.validate_checkpoint_ready().is_ok());
    let restored = history(&buffer.checkpoint().samples, 1.0);
    assert_eq!(
        correction(&buffer, 2.5, 7.0, 1.0, 3.0),
        correction(&restored, 2.5, 7.0, 1.0, 3.0)
    );
}

#[test]
fn current_and_delay_actions_match_independent_perturbations() {
    let mut buffer = DelayBuffer::new(0);
    buffer
        .restore_checkpoint(&DelayCheckpoint {
            left_limits: Vec::new(),
            configuration: Some(DelayConfiguration::Bounded { max_delay: 4.0 }),
            samples: vec![(0.0, 2.0), (0.5, 3.0)],
        })
        .unwrap();
    let h = 2.0_f64.powi(-18);
    for time in [1.0, 2.0, 4.0] {
        for delay in [0.125, 0.375, 1.25, 5.0] {
            let eval = |input, tau| {
                buffer
                    .difference_with_coefficients(time, input, tau, Some(4.0))
                    .unwrap()
            };
            let center = eval(7.0, delay);
            let di = (eval(7.0 + h, delay).output - eval(7.0 - h, delay).output) / (2.0 * h);
            let dtau = (eval(7.0, delay + h).output - eval(7.0, delay - h).output) / (2.0 * h);
            assert!(
                (center.apply_input_derivative(1.0).unwrap() - di).abs() < 1e-9,
                "time={time} delay={delay}"
            );
            assert!(
                (center.delay_coefficient - dtau).abs() < 1e-9,
                "time={time} delay={delay}"
            );
        }
    }
}

#[test]
fn bounded_delay_recovers_a_finite_slope_and_rejects_invalid_results() {
    let mut buffer = DelayBuffer::new(0);
    buffer
        .restore_checkpoint(&DelayCheckpoint {
            left_limits: Vec::new(),
            configuration: Some(DelayConfiguration::Bounded { max_delay: 4.0 }),
            samples: vec![(0.0, -f64::MAX)],
        })
        .unwrap();
    let result = buffer
        .difference_with_coefficients(4.0, f64::MAX, 1.0, Some(4.0))
        .unwrap();
    assert_eq!(result.output, -0.5 * f64::MAX);
    assert_eq!(result.delay_coefficient, -0.5 * f64::MAX);
    assert!(result.apply_input_derivative(f64::INFINITY).is_err());
    assert!(
        buffer
            .difference_with_coefficients(0.5, f64::MAX, 1.0, Some(4.0))
            .is_err()
    );
}
