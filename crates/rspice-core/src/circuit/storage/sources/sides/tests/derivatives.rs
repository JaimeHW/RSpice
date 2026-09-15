use super::*;

#[test]
fn causal_event_orders_affine_source_curvature_does_not_erase_value_or_slope_jumps() {
    for dialect in [SpiceDialect::Ngspice, SpiceDialect::Xyce] {
        for waveform in [
            SourceSpec::Dc(2.0),
            pulse([1.0, 0.25, 0.25, 1.0, 4.0], 0.0),
            SourceSpec::DcTransient {
                dc_value: 17.0,
                transient: Box::new(SourceSpec::Pwl {
                    points: vec![(0.0, 0.0), (1.0, 1.0), (1.0, 2.0), (2.0, 0.0)],
                    delay: 0.0,
                    repeat_from: None,
                }),
            },
        ] {
            let pair = sources(waveform, dialect);
            for time in [0.0, 1.0, 1.125, 1.25, 2.0] {
                for side in [SourceTimeSide::LeftLimit, SourceTimeSide::RightLimit] {
                    assert_eq!(
                        pair.0.time_derivative_at_on_side(0, time, 2, side),
                        Some(0.0)
                    );
                    assert_eq!(
                        pair.1.time_derivative_at_on_side(0, time, 2, side),
                        Some(0.0)
                    );
                }
            }
            if matches!(pair.0.source_specs[0], Some(SourceSpec::DcTransient { .. })) {
                check(&pair, 1.0, [1.0, 2.0]);
                check_slopes(&pair, 1.0, [1.0, -2.0]);
            }
        }
        let tone = sources(
            SourceSpec::Sin {
                offset: 0.0,
                amplitude: 1.0,
                frequency: 1.0,
                delay: 1.0,
                damping: 0.0,
                phase: 0.0,
            },
            dialect,
        );
        for side in [SourceTimeSide::LeftLimit, SourceTimeSide::RightLimit] {
            assert_eq!(tone.0.time_derivative_at_on_side(0, 1.0, 2, side), None);
            assert_eq!(tone.1.time_derivative_at_on_side(0, 1.0, 2, side), None);
        }
    }
}

fn check_slopes(pair: &(VoltageSources, CurrentSources), time: Value, expected: [Value; 2]) {
    for (side, expected) in [SourceTimeSide::LeftLimit, SourceTimeSide::RightLimit]
        .into_iter()
        .zip(expected)
    {
        for actual in [
            pair.0.time_derivative_at_on_side(0, time, 1, side),
            pair.1.time_derivative_at_on_side(0, time, 1, side),
        ] {
            let actual = actual.expect("finite regular one-sided slope");
            assert!(
                (actual - expected).abs() <= 2e-14 * expected.abs(),
                "{time:e} {side:?}: {actual:e} != {expected:e}"
            );
        }
        assert_eq!(
            pair.0.time_derivative_at_on_side(0, time, 0, side),
            Some(pair.0.transient_value_at_on_side(0, time, side))
        );
        assert_eq!(
            pair.1.time_derivative_at_on_side(0, time, 0, side),
            Some(pair.1.value_at_time_on_side(0, time, side))
        );
    }
}

#[test]
fn physical_source_slopes_select_both_pulse_corners_without_clock_perturbation() {
    for scale in [2.0_f64.powi(-900), 1.0, 2.0_f64.powi(900)] {
        let pair = sources(
            pulse(
                [
                    0.25 * scale,
                    0.5 * scale,
                    0.25 * scale,
                    0.75 * scale,
                    2.0 * scale,
                ],
                0.0,
            ),
            SpiceDialect::Xyce,
        );
        for (time, slopes) in [
            (0.25, [0.0, 2.0]),
            (0.75, [2.0, 0.0]),
            (1.5, [0.0, -4.0]),
            (1.75, [-4.0, 0.0]),
            (2.25, [0.0, 2.0]),
        ] {
            check_slopes(&pair, time * scale, slopes.map(|value| value / scale));
        }
    }
    let pair = sources(pulse([0.25, 0.5, 0.25, 0.75, 2.0], 1.0), SpiceDialect::Xyce);
    check_slopes(&pair, 2.25, [0.0, 0.0]);
    let pair = sources(pulse([1.0, 0.0, 0.0, 1.0, 4.0], 0.0), SpiceDialect::Xyce);
    check_slopes(&pair, 1.0, [0.0, 0.0]);
    check_slopes(&pair, 2.0, [0.0, 0.0]);
    assert!(
        pair.0.time_derivative_at(0, 1.0, 1).unwrap().is_nan(),
        "the published derivative still exposes its impulse"
    );
}

#[test]
fn physical_source_slopes_preserve_pwl_jumps_and_fractional_repeat_seams() {
    for dialect in [SpiceDialect::Ngspice, SpiceDialect::Xyce] {
        let pair = sources(
            SourceSpec::Pwl {
                points: vec![(0.0, 2.0), (1.0, 4.0), (1.0, 9.0), (2.0, 5.0)],
                delay: 10.0,
                repeat_from: None,
            },
            dialect,
        );
        check_slopes(&pair, 10.0, [0.0, 2.0]);
        check_slopes(&pair, 11.0, [2.0, -4.0]);
        check_slopes(&pair, 12.0, [-4.0, 0.0]);
        let pair = sources(
            SourceSpec::Pwl {
                points: vec![(0.0, 0.0), (1.0, 2.0), (2.0, 1.0)],
                delay: 0.0,
                repeat_from: Some(0.5),
            },
            dialect,
        );
        for time in [2.0, 3.5, 5.0] {
            check_slopes(&pair, time, [-1.0, 2.0]);
        }
        check_slopes(&pair, 2.5, [2.0, -1.0]);
    }
}

#[test]
fn physical_source_slopes_use_scaled_differences_and_exact_large_clocks() {
    let pair = sources(
        SourceSpec::Pwl {
            points: vec![(0.0, -Value::MAX), (Value::MAX, Value::MAX)],
            delay: 0.0,
            repeat_from: None,
        },
        SpiceDialect::Xyce,
    );
    check_slopes(&pair, 0.0, [0.0, 2.0]);
    check_slopes(&pair, Value::MAX, [2.0, 0.0]);
    let origin = 1e12;
    let step = 1e12_f64.next_up() - origin;
    let pair = sources(
        SourceSpec::Pwl {
            points: vec![(0.0, 0.0), (step, 1.0), (step, 3.0), (2.0 * step, 2.0)],
            delay: origin,
            repeat_from: None,
        },
        SpiceDialect::Xyce,
    );
    check_slopes(&pair, origin + step, [1.0 / step, -1.0 / step]);
    // This authored duration rounds to a different absolute clock gap.
    let width = 0.00015;
    assert_ne!((origin + width) - origin, width);
    let pair = sources(
        SourceSpec::Pwl {
            points: vec![(0.0, 0.0), (width, 1.0), (width, 3.0), (3.0 * width, 2.0)],
            delay: origin,
            repeat_from: None,
        },
        SpiceDialect::Xyce,
    );
    check_slopes(&pair, origin + width, [1.0 / width, -1.0 / (2.0 * width)]);
}

#[test]
fn physical_source_slopes_separate_exp_impulses_from_regular_tails() {
    let pair = sources(
        SourceSpec::Exp {
            v1: 0.0,
            v2: 1.0,
            td1: 1.0,
            tau1: 0.25,
            td2: 2.0,
            tau2: 0.5,
        },
        SpiceDialect::Xyce,
    );
    check_slopes(&pair, 1.0, [0.0, 4.0]);
    let rise = 4.0 * (-4.0_f64).exp();
    check_slopes(&pair, 2.0, [rise, rise - 2.0]);
    let pair = sources(
        SourceSpec::Exp {
            v1: 0.0,
            v2: 1.0,
            td1: 1.0,
            tau1: 0.0,
            td2: 2.0,
            tau2: 0.5,
        },
        SpiceDialect::Xyce,
    );
    check_slopes(&pair, 1.0, [0.0, 0.0]);
    check_slopes(&pair, 2.0, [0.0, -2.0]);
    let tiny = Value::from_bits(1);
    let pair = sources(
        SourceSpec::Exp {
            v1: 0.0,
            v2: tiny,
            td1: 1.0,
            tau1: tiny,
            td2: 2.0,
            tau2: 1.0,
        },
        SpiceDialect::Xyce,
    );
    check_slopes(&pair, 1.0, [0.0, 1.0]);
}

#[test]
fn physical_source_slopes_select_delayed_analytic_carriers_and_wrappers() {
    let inner = SourceSpec::Sin {
        offset: 1.0,
        amplitude: 2.0,
        frequency: 0.25,
        delay: 1.0,
        damping: 0.1,
        phase: 0.3,
    };
    let expected = 2.0 * (std::f64::consts::FRAC_PI_2 * 0.3_f64.cos() - 0.1 * 0.3_f64.sin());
    let pair = sources(
        SourceSpec::DcTransient {
            dc_value: 17.0,
            transient: Box::new(inner),
        },
        SpiceDialect::Ngspice,
    );
    check_slopes(&pair, 1.0, [0.0, expected]);
    let pair = sources(
        SourceSpec::Sffm {
            offset: 1.0,
            amplitude: 1.0,
            carrier_freq: 1.0,
            modulation_index: 1.0,
            signal_freq: 0.5,
            delay: 1.0,
            phase_modulation: 0.0,
            phase_carrier: 0.0,
        },
        SpiceDialect::Ngspice,
    );
    check_slopes(&pair, 1.0, [0.0, 3.0 * std::f64::consts::PI]);
    assert!(pair.0.time_derivative_at(0, 1.0, 1).unwrap().is_nan());
    let pair = sources(
        SourceSpec::Am {
            offset: 1.0,
            modulation_offset: 2.0,
            modulation_amplitude: 1.0,
            modulating_freq: 0.5,
            carrier_freq: 1.0,
            delay: 1.0,
            phase_modulation: 0.0,
            phase_carrier: 0.0,
        },
        SpiceDialect::Ngspice,
    );
    check_slopes(&pair, 1.0, [0.0, 4.0 * std::f64::consts::PI]);
}

#[test]
fn physical_source_slopes_do_not_invent_higher_distributional_derivatives() {
    let pair = sources(
        SourceSpec::Pwl {
            points: vec![(0.0, 0.0), (1.0, 1.0)],
            delay: 0.0,
            repeat_from: None,
        },
        SpiceDialect::Ngspice,
    );
    for side in [SourceTimeSide::LeftLimit, SourceTimeSide::RightLimit] {
        assert_eq!(
            pair.0.time_derivative_at_on_side(0, 1.0, 2, side),
            Some(0.0)
        );
        assert_eq!(
            pair.1.time_derivative_at_on_side(0, 1.0, 2, side),
            Some(0.0)
        );
    }
    // Finite curvature on the open sides does not erase the slope jump or
    // make the published second derivative an ordinary finite value.
    check_slopes(&pair, 1.0, [1.0, 0.0]);
    assert_eq!(pair.0.time_derivative_at(0, 1.0, 2), None);
    assert_eq!(pair.1.time_derivative_at(0, 1.0, 2), None);
}

#[test]
fn physical_source_slopes_refuse_missing_files_invalid_patterns_and_invalid_times() {
    let specs = [
        SourceSpec::PwlFile {
            path: "not-loaded-for-an-unqualified-slope.csv".into(),
            time_scale: 1.0,
            value_scale: 1.0,
            time_offset: 0.0,
            value_offset: 0.0,
            delay: 0.0,
            repeat_from: Some(0.0),
        },
        SourceSpec::Pat {
            vhi: 1.0,
            vlo: 0.0,
            delay: 0.0,
            rise: 0.1,
            fall: 0.1,
            sample: 1.0,
            data: "B1X".into(),
            repeat_count: -1,
        },
    ];
    for spec in specs {
        let mut pair = sources(SourceSpec::Dc(0.0), SpiceDialect::Xyce);
        pair.0.source_specs[0] = Some(spec.clone());
        pair.1.source_specs[0] = Some(spec);
        for side in [SourceTimeSide::LeftLimit, SourceTimeSide::RightLimit] {
            assert_eq!(pair.0.time_derivative_at_on_side(0, 1.0, 1, side), None);
            assert_eq!(pair.1.time_derivative_at_on_side(0, 1.0, 1, side), None);
            assert_eq!(pair.0.time_derivative_at_on_side(0, 1.0, 2, side), None);
            assert_eq!(pair.1.time_derivative_at_on_side(0, 1.0, 2, side), None);
        }
    }
    let pair = sources(SourceSpec::Dc(1.0), SpiceDialect::Xyce);
    for time in [Value::NAN, Value::INFINITY, Value::NEG_INFINITY] {
        for side in [SourceTimeSide::LeftLimit, SourceTimeSide::RightLimit] {
            assert_eq!(pair.0.time_derivative_at_on_side(0, time, 1, side), None);
            assert_eq!(pair.1.time_derivative_at_on_side(0, time, 1, side), None);
        }
    }
}
