//! Physical delay fractions must survive subtraction from absolute time.

use rspice_veriloga_runtime::transport_delay::{DelayBuffer, DelayCheckpoint, DelayConfiguration};

fn history(samples: Vec<(f64, f64)>, delay: f64, bounded: bool) -> DelayBuffer {
    let mut buffer = DelayBuffer::new(0);
    buffer
        .restore_checkpoint(&DelayCheckpoint {
            configuration: Some(if bounded {
                DelayConfiguration::Bounded { max_delay: 4.0 }
            } else {
                DelayConfiguration::Fixed { delay }
            }),
            samples,
        })
        .unwrap();
    buffer
}

#[test]
fn tiny_delay_retains_a_finite_signal_below_absolute_time_resolution() {
    for delay in [2.0_f64.powi(-80), 2.0_f64.powi(-1022), f64::from_bits(1)] {
        for bounded in [false, true] {
            let mut buffer = history(vec![(0.0, 1.0), (0.5, 1.0)], delay, bounded);
            let accepted = buffer.checkpoint();
            assert_eq!(1.0 - delay, 1.0);
            let evaluation = buffer
                .eval_with_coefficients(1.0, 0.0, delay, bounded.then_some(4.0))
                .unwrap();
            // The last interval is 1/2 second; its falling unit ramp at
            // t = 1 - delay is exactly 2*delay, even for a subnormal delay.
            assert_eq!(evaluation.output.to_bits(), (2.0 * delay).to_bits());
            assert_eq!(evaluation.input_coefficient, 1.0);
            assert_eq!(
                evaluation.delay_coefficient,
                if bounded { 2.0 } else { 0.0 }
            );
            assert_eq!(buffer.checkpoint(), accepted);
            buffer.begin_evaluation();
            assert!(buffer.validate_checkpoint_ready().is_ok());
        }
    }
}

#[test]
fn rounded_target_keeps_the_nonzero_candidate_fraction() {
    let delay = 0.5_f64.next_down();
    assert_eq!(1.0 - delay, 0.5);
    let mut buffer = history(vec![(0.0, 0.0), (0.5, 0.0)], delay, true);
    let evaluation = buffer
        .eval_with_coefficients(1.0, 1.0, delay, Some(4.0))
        .unwrap();
    assert_eq!(evaluation.output, 2.0_f64.powi(-53));
    assert_eq!(evaluation.input_coefficient, 2.0_f64.powi(-53));
    assert_eq!(evaluation.delay_coefficient, -2.0);
}

#[test]
fn rounded_target_selects_the_correct_side_of_an_accepted_knot() {
    let knot = 2.0_f64.powi(40);
    let time = knot + 2.0;
    let delay = 2.0_f64.next_up();
    assert_eq!(time - delay, knot);
    let mut buffer = history(vec![(knot - 1.0, 0.0), (knot, 1.0)], delay, true);
    let evaluation = buffer
        .eval_with_coefficients(time, -3.0, delay, Some(4.0))
        .unwrap();
    assert_eq!(evaluation.output, 1.0 - 2.0_f64.powi(-51));
    assert_eq!(evaluation.input_coefficient, 0.0);
    assert_eq!(evaluation.delay_coefficient, -1.0);
}

#[test]
fn tiny_weight_does_not_underflow_before_multiplying_the_signal() {
    let delay = f64::from_bits(1);
    let amplitude = 2.0_f64.powi(1000);
    let mut buffer = history(vec![(0.0, amplitude), (4.0, amplitude)], delay, false);
    // delay / interval rounds to zero, but the delayed signal is 2^-76.
    assert_eq!(delay / 4.0, 0.0);
    let evaluation = buffer
        .eval_with_coefficients(8.0, 0.0, delay, None)
        .unwrap();
    assert_eq!(evaluation.output, 2.0_f64.powi(-76));
    assert_eq!(evaluation.input_coefficient, 1.0);
    assert_eq!(evaluation.delay_coefficient, 0.0);
}

#[test]
fn fixed_delay_does_not_require_an_unrepresentable_unused_slope() {
    let tick = f64::from_bits(1);
    let mut buffer = history(vec![(0.0, 0.0)], tick, false);
    let evaluation = buffer
        .eval_with_coefficients(4.0 * tick, 1.0, tick, None)
        .unwrap();
    assert_eq!(evaluation.output, 0.75);
    assert_eq!(evaluation.input_coefficient, 0.75);
    assert_eq!(evaluation.delay_coefficient, 0.0);
}

#[test]
fn bounded_delay_slope_survives_overflowing_endpoint_difference() {
    let mut buffer = history(vec![(0.0, -f64::MAX)], 1.0, true);
    let evaluation = buffer
        .eval_with_coefficients(4.0, f64::MAX, 1.0, Some(4.0))
        .unwrap();
    assert_eq!(evaluation.output, 0.5 * f64::MAX);
    assert_eq!(evaluation.input_coefficient, 0.75);
    assert_eq!(evaluation.delay_coefficient, -0.5 * f64::MAX);
}

#[test]
fn interpolated_cancellation_retains_the_exact_small_signal() {
    for bounded in [false, true] {
        let delay = 1.0 / 3.0;
        let mut buffer = history(vec![(0.0, 2.0)], delay, bounded);
        let evaluation = buffer
            .eval_with_coefficients(1.0, -1.0, delay, bounded.then_some(4.0))
            .unwrap();
        // With the authored binary64 delay, 3*delay-1 is exactly -2^-54.
        // Rounding the two weights first doubles that residual.
        assert_eq!(evaluation.output, -2.0_f64.powi(-54));
        assert_eq!(evaluation.input_coefficient, 1.0 - delay);
        assert_eq!(
            evaluation.delay_coefficient,
            if bounded { 3.0 } else { 0.0 }
        );
    }
}
