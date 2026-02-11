use super::*;

// Transition Filter Tests
// ========================================================================

#[test]
fn test_transition_filter_new() {
    let filter = TransitionFilter::new();
    assert_eq!(filter.output, 0.0);
    assert_eq!(filter.target, 0.0);
}

#[test]
fn test_transition_filter_instantaneous() {
    let mut filter = TransitionFilter::new();
    // Zero rise/fall time = instantaneous
    let result = filter.update(10.0, 0.0, 0.0, 0.0, 0.0);
    assert!((result - 10.0).abs() < 1e-12);
}

#[test]
fn test_transition_filter_with_delay() {
    let mut filter = TransitionFilter::new();
    // Set up: input=10, delay=1, rise=1, fall=1
    let _ = filter.update(10.0, 0.0, 1.0, 1.0, 1.0);
    // At t=0 (before delay), output should still be 0
    let result = filter.update(10.0, 0.5, 1.0, 1.0, 1.0);
    assert!((result - 0.0).abs() < 1e-12, "Before delay, output=0");
}

#[test]
fn test_transition_vm_dc() {
    // DC: transition returns input immediately
    let mut ctx = VmContext::new(2);

    let program = make_program(vec![
        Instruction::PushConst(100.0), // expr
        Instruction::PushConst(0.0),   // delay
        Instruction::PushConst(1e-9),  // rise
        Instruction::PushConst(1e-9),  // fall
        Instruction::TransitionState(0),
    ]);
    let result = Vm::new(&mut ctx).execute(&program).unwrap();
    assert!(
        (result - 100.0).abs() < 1e-12,
        "DC transition: input passthrough"
    );
}

#[test]
fn test_transition_vm_transient_ramps_over_rise_time() {
    let mut ctx = VmContext::new(2);
    ctx.analysis_type = 2; // transient

    ctx.time = 0.0;
    let result_t0 = Vm::new(&mut ctx)
        .execute(&make_program(vec![
            Instruction::PushConst(0.0),
            Instruction::PushConst(0.0),
            Instruction::PushConst(1.0),
            Instruction::PushConst(1.0),
            Instruction::TransitionState(0),
        ]))
        .unwrap();
    assert!((result_t0 - 0.0).abs() < 1e-12);

    ctx.time = 0.5;
    let result_t05 = Vm::new(&mut ctx)
        .execute(&make_program(vec![
            Instruction::PushConst(10.0),
            Instruction::PushConst(0.0),
            Instruction::PushConst(1.0),
            Instruction::PushConst(1.0),
            Instruction::TransitionState(0),
        ]))
        .unwrap();
    assert!((result_t05 - 0.0).abs() < 1e-12);

    ctx.time = 1.0;
    let result_t10 = Vm::new(&mut ctx)
        .execute(&make_program(vec![
            Instruction::PushConst(10.0),
            Instruction::PushConst(0.0),
            Instruction::PushConst(1.0),
            Instruction::PushConst(1.0),
            Instruction::TransitionState(0),
        ]))
        .unwrap();
    assert!((result_t10 - 5.0).abs() < 1e-12);

    ctx.time = 1.5;
    let result_t15 = Vm::new(&mut ctx)
        .execute(&make_program(vec![
            Instruction::PushConst(10.0),
            Instruction::PushConst(0.0),
            Instruction::PushConst(1.0),
            Instruction::PushConst(1.0),
            Instruction::TransitionState(0),
        ]))
        .unwrap();
    assert!((result_t15 - 10.0).abs() < 1e-12);
}

#[test]
fn test_transition_vm_transient_uses_independent_filter_ids() {
    let mut ctx = VmContext::new(2);
    ctx.analysis_type = 2; // transient

    ctx.time = 0.0;
    let _ = Vm::new(&mut ctx)
        .execute(&make_program(vec![
            Instruction::PushConst(10.0),
            Instruction::PushConst(0.0),
            Instruction::PushConst(1.0),
            Instruction::PushConst(1.0),
            Instruction::TransitionState(0),
        ]))
        .unwrap();

    ctx.time = 0.5;
    let filter0 = Vm::new(&mut ctx)
        .execute(&make_program(vec![
            Instruction::PushConst(10.0),
            Instruction::PushConst(0.0),
            Instruction::PushConst(1.0),
            Instruction::PushConst(1.0),
            Instruction::TransitionState(0),
        ]))
        .unwrap();
    let filter1 = Vm::new(&mut ctx)
        .execute(&make_program(vec![
            Instruction::PushConst(10.0),
            Instruction::PushConst(0.0),
            Instruction::PushConst(1.0),
            Instruction::PushConst(1.0),
            Instruction::TransitionState(1),
        ]))
        .unwrap();

    assert!((filter0 - 5.0).abs() < 1e-12);
    assert!((filter1 - 0.0).abs() < 1e-12);
}

// ========================================================================
// Slew Filter Tests
// ========================================================================

#[test]
fn test_slew_filter_new() {
    let filter = SlewFilter::new();
    assert_eq!(filter.output, 0.0);
    assert_eq!(filter.prev_time, 0.0);
}

#[test]
fn test_slew_filter_unlimited() {
    let mut filter = SlewFilter::new();
    // Infinite slew = no limiting
    let result = filter.update(1000.0, 1.0, f64::INFINITY, f64::INFINITY);
    assert!((result - 1000.0).abs() < 1e-12);
}

#[test]
fn test_slew_filter_positive_limited() {
    let mut filter = SlewFilter::new();
    filter.output = 0.0;
    filter.prev_time = 0.0;
    // Max slew = 10 V/s, input=100, dt=1s => max delta=10
    let result = filter.update(100.0, 1.0, 10.0, 10.0);
    assert!((result - 10.0).abs() < 1e-12, "Positive slew limited to 10");
}

#[test]
fn test_slew_filter_negative_limited() {
    let mut filter = SlewFilter::new();
    filter.output = 100.0;
    filter.prev_time = 0.0;
    // Max slew = 10 V/s, input=0, dt=1s => max delta=-10
    let result = filter.update(0.0, 1.0, 10.0, 10.0);
    assert!(
        (result - 90.0).abs() < 1e-12,
        "Negative slew limited to -10"
    );
}

#[test]
fn test_slew_vm_dc() {
    let mut ctx = VmContext::new(2);

    let program = make_program(vec![
        Instruction::PushConst(50.0),          // expr
        Instruction::PushConst(f64::INFINITY), // max_pos
        Instruction::PushConst(f64::INFINITY), // max_neg
        Instruction::SlewState(0),
    ]);
    let result = Vm::new(&mut ctx).execute(&program).unwrap();
    assert!((result - 50.0).abs() < 1e-12, "DC slew: input passthrough");
}

#[test]
fn test_slew_vm_transient_limits_slope_over_time() {
    let mut ctx = VmContext::new(2);
    ctx.analysis_type = 2; // transient

    let positive = make_program(vec![
        Instruction::PushConst(100.0),
        Instruction::PushConst(10.0),
        Instruction::PushConst(10.0),
        Instruction::SlewState(0),
    ]);

    ctx.time = 0.0;
    let t0 = Vm::new(&mut ctx).execute(&positive).unwrap();
    assert!((t0 - 0.0).abs() < 1e-12);

    ctx.time = 1.0;
    let t1 = Vm::new(&mut ctx).execute(&positive).unwrap();
    assert!((t1 - 10.0).abs() < 1e-12);

    ctx.time = 2.0;
    let t2 = Vm::new(&mut ctx).execute(&positive).unwrap();
    assert!((t2 - 20.0).abs() < 1e-12);

    let negative = make_program(vec![
        Instruction::PushConst(-100.0),
        Instruction::PushConst(10.0),
        Instruction::PushConst(10.0),
        Instruction::SlewState(0),
    ]);
    ctx.time = 3.0;
    let t3 = Vm::new(&mut ctx).execute(&negative).unwrap();
    assert!((t3 - 10.0).abs() < 1e-12);
}

// ========================================================================
// Cross Detector Tests
// ========================================================================

#[test]
fn test_cross_detector_new() {
    let detector = CrossDetector::new();
    assert_eq!(detector.prev_value, 0.0);
    assert_eq!(detector.prev_time, 0.0);
}

#[test]
fn test_cross_detector_rising() {
    let mut detector = CrossDetector::new();
    detector.prev_value = -1.0;
    detector.prev_time = 0.0;
    // Rising cross: -1 -> +1
    let result = detector.update(1.0, 1.0, 1);
    assert!((result - 1.0).abs() < 1e-12, "Rising cross detected");
}

#[test]
fn test_cross_detector_falling() {
    let mut detector = CrossDetector::new();
    detector.prev_value = 1.0;
    detector.prev_time = 0.0;
    // Falling cross: +1 -> -1
    let result = detector.update(-1.0, 1.0, -1);
    assert!((result - 1.0).abs() < 1e-12, "Falling cross detected");
}

#[test]
fn test_cross_detector_both_directions() {
    let mut detector = CrossDetector::new();
    detector.prev_value = -1.0;
    detector.prev_time = 0.0;
    // Both directions (0): rising detected
    let result = detector.update(1.0, 1.0, 0);
    assert!((result - 1.0).abs() < 1e-12, "Both: rising cross detected");
}

#[test]
fn test_cross_detector_no_crossing() {
    let mut detector = CrossDetector::new();
    detector.prev_value = 1.0;
    detector.prev_time = 0.0;
    // No crossing: both positive
    let result = detector.update(2.0, 1.0, 0);
    assert!((result - 0.0).abs() < 1e-12, "No crossing");
}

#[test]
fn test_cross_vm_dc() {
    let mut ctx = VmContext::new(2);

    let program = make_program(vec![
        Instruction::PushConst(1.0), // expr
        Instruction::PushConst(0.0), // direction
        Instruction::CrossState(0),
    ]);
    let result = Vm::new(&mut ctx).execute(&program).unwrap();
    assert!((result - 0.0).abs() < 1e-12, "DC cross: always 0");
}

#[test]
fn test_cross_vm_transient_detects_rising_and_falling_edges() {
    let mut ctx = VmContext::new(2);
    ctx.analysis_type = 2; // transient

    let rising = |value: f64| {
        make_program(vec![
            Instruction::PushConst(value),
            Instruction::PushConst(1.0),
            Instruction::CrossState(0),
        ])
    };
    let falling = |value: f64| {
        make_program(vec![
            Instruction::PushConst(value),
            Instruction::PushConst(-1.0),
            Instruction::CrossState(0),
        ])
    };

    ctx.time = 0.0;
    let t0 = Vm::new(&mut ctx).execute(&rising(-1.0)).unwrap();
    assert!((t0 - 0.0).abs() < 1e-12);

    ctx.time = 1.0;
    let t1 = Vm::new(&mut ctx).execute(&rising(1.0)).unwrap();
    assert!((t1 - 1.0).abs() < 1e-12);

    ctx.time = 2.0;
    let t2 = Vm::new(&mut ctx).execute(&rising(2.0)).unwrap();
    assert!((t2 - 0.0).abs() < 1e-12);

    ctx.time = 3.0;
    let t3 = Vm::new(&mut ctx).execute(&falling(-1.0)).unwrap();
    assert!((t3 - 1.0).abs() < 1e-12);
}

#[test]
fn test_cross_vm_transient_direction_zero_detects_both_edges() {
    let mut ctx = VmContext::new(2);
    ctx.analysis_type = 2; // transient

    let both = |value: f64| {
        make_program(vec![
            Instruction::PushConst(value),
            Instruction::PushConst(0.0),
            Instruction::CrossState(0),
        ])
    };

    ctx.time = 0.0;
    let t0 = Vm::new(&mut ctx).execute(&both(-1.0)).unwrap();
    assert!((t0 - 0.0).abs() < 1e-12);

    ctx.time = 1.0;
    let t1 = Vm::new(&mut ctx).execute(&both(1.0)).unwrap();
    assert!((t1 - 1.0).abs() < 1e-12);

    ctx.time = 2.0;
    let t2 = Vm::new(&mut ctx).execute(&both(-1.0)).unwrap();
    assert!((t2 - 1.0).abs() < 1e-12);
}

// ========================================================================
// Noise Function Tests
// ========================================================================

#[test]
fn test_white_noise_vm() {
    let mut ctx = VmContext::new(2);

    let program = make_program(vec![
        Instruction::PushConst(1e-12), // power
        Instruction::WhiteNoise,
    ]);
    let result = Vm::new(&mut ctx).execute(&program).unwrap();
    assert_eq!(result, 0.0, "White noise returns 0 in time domain");
}

#[test]
fn test_flicker_noise_vm() {
    let mut ctx = VmContext::new(2);

    let program = make_program(vec![
        Instruction::PushConst(1e-12), // power
        Instruction::PushConst(1.0),   // exponent
        Instruction::FlickerNoise,
    ]);
    let result = Vm::new(&mut ctx).execute(&program).unwrap();
    assert_eq!(result, 0.0, "Flicker noise returns 0 in time domain");
}

// ========================================================================
// Analysis Function Tests
// ========================================================================

#[test]
fn test_analysis_dc_check_true() {
    let mut ctx = VmContext::new(2);
    ctx.analysis_type = 0; // DC

    // Check if analysis("dc") returns 1
    let program = make_program(vec![Instruction::Analysis(0)]);
    let result = Vm::new(&mut ctx).execute(&program).unwrap();
    assert_eq!(result, 1.0, "DC analysis check should be true during DC");
}

#[test]
fn test_analysis_dc_check_false() {
    let mut ctx = VmContext::new(2);
    ctx.analysis_type = 2; // Transient

    // Check if analysis("dc") returns 0
    let program = make_program(vec![Instruction::Analysis(0)]);
    let result = Vm::new(&mut ctx).execute(&program).unwrap();
    assert_eq!(result, 0.0, "DC check should be false during transient");
}

#[test]
fn test_analysis_ac_check_true() {
    let mut ctx = VmContext::new(2);
    ctx.analysis_type = 1; // AC

    let program = make_program(vec![Instruction::Analysis(1)]);
    let result = Vm::new(&mut ctx).execute(&program).unwrap();
    assert_eq!(result, 1.0, "AC analysis check should be true during AC");
}

#[test]
fn test_analysis_tran_check_true() {
    let mut ctx = VmContext::new(2);
    ctx.analysis_type = 2; // Transient

    let program = make_program(vec![Instruction::Analysis(2)]);
    let result = Vm::new(&mut ctx).execute(&program).unwrap();
    assert_eq!(result, 1.0, "Tran check should be true during transient");
}

#[test]
fn test_analysis_unknown_type() {
    let mut ctx = VmContext::new(2);

    // Unknown analysis type (99) should return 0
    let program = make_program(vec![Instruction::Analysis(99)]);
    let result = Vm::new(&mut ctx).execute(&program).unwrap();
    assert_eq!(result, 0.0, "Unknown analysis type returns 0");
}

// ========================================================================
// Above Event Tests
// ========================================================================

#[test]
fn test_above_value_greater_than_threshold() {
    let mut ctx = VmContext::new(2);

    // above(5.0, 3.0) should return 1 (5 > 3)
    let program = make_program(vec![
        Instruction::PushConst(5.0), // value
        Instruction::PushConst(3.0), // threshold
        Instruction::AboveState(0),
    ]);
    let result = Vm::new(&mut ctx).execute(&program).unwrap();
    assert_eq!(result, 1.0, "above(5, 3) should be 1");
}

#[test]
fn test_above_value_less_than_threshold() {
    let mut ctx = VmContext::new(2);

    // above(2.0, 3.0) should return 0 (2 < 3)
    let program = make_program(vec![
        Instruction::PushConst(2.0), // value
        Instruction::PushConst(3.0), // threshold
        Instruction::AboveState(0),
    ]);
    let result = Vm::new(&mut ctx).execute(&program).unwrap();
    assert_eq!(result, 0.0, "above(2, 3) should be 0");
}

#[test]
fn test_above_value_equal_to_threshold() {
    let mut ctx = VmContext::new(2);

    // above(3.0, 3.0) should return 0 (not strictly greater)
    let program = make_program(vec![
        Instruction::PushConst(3.0),
        Instruction::PushConst(3.0),
        Instruction::AboveState(0),
    ]);
    let result = Vm::new(&mut ctx).execute(&program).unwrap();
    assert_eq!(
        result, 0.0,
        "above(3, 3) should be 0 (not strictly greater)"
    );
}

#[test]
fn test_above_negative_values() {
    let mut ctx = VmContext::new(2);

    // above(-1.0, -2.0) should return 1 (-1 > -2)
    let program = make_program(vec![
        Instruction::PushConst(-1.0),
        Instruction::PushConst(-2.0),
        Instruction::AboveState(0),
    ]);
    let result = Vm::new(&mut ctx).execute(&program).unwrap();
    assert_eq!(result, 1.0, "above(-1, -2) should be 1");
}

// ========================================================================
// Timer Event Tests
// ========================================================================

#[test]
fn test_timer_before_start() {
    let mut ctx = VmContext::new(2);
    ctx.time = 0.5;
    ctx.timestep = 0.01;

    // timer(start=1.0, period=0.5) at time=0.5 should return 0
    let program = make_program(vec![
        Instruction::PushConst(1.0), // start time
        Instruction::PushConst(0.5), // period
        Instruction::TimerState(0),
    ]);
    let result = Vm::new(&mut ctx).execute(&program).unwrap();
    assert_eq!(result, 0.0, "Timer before start should return 0");
}

#[test]
fn test_timer_at_start() {
    let mut ctx = VmContext::new(2);
    ctx.time = 1.0;
    ctx.timestep = 0.01;

    // timer(start=1.0, period=0.5) at time=1.0 should fire
    let program = make_program(vec![
        Instruction::PushConst(1.0),
        Instruction::PushConst(0.5),
        Instruction::TimerState(0),
    ]);
    let result = Vm::new(&mut ctx).execute(&program).unwrap();
    assert_eq!(result, 1.0, "Timer at start time should fire");
}

#[test]
fn test_timer_at_period_multiple() {
    let mut ctx = VmContext::new(2);
    ctx.time = 2.0; // start + 2*period
    ctx.timestep = 0.01;

    // timer(start=1.0, period=0.5) at time=2.0 fires (1 + 2*0.5 = 2)
    let program = make_program(vec![
        Instruction::PushConst(1.0),
        Instruction::PushConst(0.5),
        Instruction::TimerState(0),
    ]);
    let result = Vm::new(&mut ctx).execute(&program).unwrap();
    assert_eq!(result, 1.0, "Timer at period multiple should fire");
}

#[test]
fn test_timer_not_at_period() {
    let mut ctx = VmContext::new(2);
    ctx.time = 1.25; // start + 0.5*period (not at multiple)
    ctx.timestep = 0.001;

    let program = make_program(vec![
        Instruction::PushConst(1.0),
        Instruction::PushConst(0.5),
        Instruction::TimerState(0),
    ]);
    let result = Vm::new(&mut ctx).execute(&program).unwrap();
    assert_eq!(result, 0.0, "Timer not at period multiple should not fire");
}

#[test]
fn test_timer_zero_period() {
    let mut ctx = VmContext::new(2);
    ctx.time = 1.0;
    ctx.timestep = 0.01;

    // timer with period=0 should never fire after start
    let program = make_program(vec![
        Instruction::PushConst(1.0),
        Instruction::PushConst(0.0), // zero period
        Instruction::TimerState(0),
    ]);
    let result = Vm::new(&mut ctx).execute(&program).unwrap();
    assert_eq!(result, 0.0, "Timer with zero period should not fire");
}

// ========================================================================
// Laplace Transfer Function Tests
// ========================================================================

#[test]
fn test_laplace_state_unity_gain() {
    let mut ctx = VmContext::new(2);

    // For DC (s=0), laplace with unity gain should pass through
    let program = make_program(vec![
        Instruction::PushConst(5.0), // input
        Instruction::LaplaceState(0),
    ]);
    let result = Vm::new(&mut ctx).execute(&program).unwrap();
    assert_eq!(result, 5.0, "Laplace with unity DC gain passes input");
}

#[test]
fn test_laplace_state_preserves_negative() {
    let mut ctx = VmContext::new(2);

    let program = make_program(vec![
        Instruction::PushConst(-10.0),
        Instruction::LaplaceState(0),
    ]);
    let result = Vm::new(&mut ctx).execute(&program).unwrap();
    assert_eq!(result, -10.0, "Laplace preserves negative values");
}

#[test]
fn test_laplace_state_zero_input() {
    let mut ctx = VmContext::new(2);

    let program = make_program(vec![
        Instruction::PushConst(0.0),
        Instruction::LaplaceState(0),
    ]);
    let result = Vm::new(&mut ctx).execute(&program).unwrap();
    assert_eq!(result, 0.0, "Laplace with zero input returns zero");
}

#[test]
fn test_laplace_chained() {
    let mut ctx = VmContext::new(2);

    // Chained Laplace filters (cascaded)
    let program = make_program(vec![
        Instruction::PushConst(3.0),
        Instruction::LaplaceState(0),
        Instruction::LaplaceState(1),
    ]);
    let result = Vm::new(&mut ctx).execute(&program).unwrap();
    assert_eq!(result, 3.0, "Chained Laplace filters pass through");
}

#[test]
fn test_laplace_in_expression() {
    let mut ctx = VmContext::new(2);

    // 2 * laplace(3) + 1 = 2*3 + 1 = 7
    let program = make_program(vec![
        Instruction::PushConst(2.0),
        Instruction::PushConst(3.0),
        Instruction::LaplaceState(0),
        Instruction::Mul,
        Instruction::PushConst(1.0),
        Instruction::Add,
    ]);
    let result = Vm::new(&mut ctx).execute(&program).unwrap();
    assert_eq!(result, 7.0, "Laplace works in expressions");
}

#[test]
fn test_multiple_analysis_types_in_sequence() {
    let mut ctx = VmContext::new(2);

    // Check all analysis types in sequence
    for (analysis_id, expected_type) in [(0, 0), (1, 1), (2, 2), (3, 3)] {
        ctx.analysis_type = expected_type;
        let program = make_program(vec![Instruction::Analysis(analysis_id)]);
        let result = Vm::new(&mut ctx).execute(&program).unwrap();
        assert_eq!(
            result, 1.0,
            "Analysis type {} should match when set",
            analysis_id
        );
    }
}

#[test]
fn test_above_with_expression_result() {
    let mut ctx = VmContext::new(2);

    // above(3+2, 4) = above(5, 4) = 1
    let program = make_program(vec![
        Instruction::PushConst(3.0),
        Instruction::PushConst(2.0),
        Instruction::Add,
        Instruction::PushConst(4.0),
        Instruction::AboveState(0),
    ]);
    let result = Vm::new(&mut ctx).execute(&program).unwrap();
    assert_eq!(result, 1.0, "above works with expression inputs");
}
