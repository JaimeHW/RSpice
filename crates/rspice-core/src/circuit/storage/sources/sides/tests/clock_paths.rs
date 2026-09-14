use super::*;

#[test]
fn physical_source_clock_paths_preserve_authored_pwl_ramps_between_events() {
    let origin = 1e12_f64;
    let width = 0.0003;
    for repeating in [false, true] {
        let pair = sources(
            SourceSpec::Pwl {
                points: vec![(0.0, 0.0), (width, 1.0)],
                delay: origin,
                repeat_from: repeating.then_some(0.0),
            },
            SpiceDialect::Xyce,
        );
        for cycle in 0..=usize::from(repeating) {
            let time = if cycle == 0 {
                origin.next_up()
            } else {
                (origin + width).next_up()
            };
            let expected = ((time - origin) - cycle as Value * width) / width;
            for side in [SourceTimeSide::LeftLimit, SourceTimeSide::RightLimit] {
                for actual in [
                    pair.0.transient_value_at_on_side(0, time, side),
                    pair.1.value_at_time_on_side(0, time, side),
                ] {
                    assert!(
                        (actual - expected).abs() < 2e-15,
                        "cycle={cycle}, side={side:?}: {actual} != {expected}"
                    );
                }
            }
        }
    }
}

fn file_pair(
    scale: Value,
    offset: Value,
    delay: Value,
    repeating: bool,
) -> (VoltageSources, CurrentSources) {
    let waveform = Arc::new(
        crate::device::pwl_file::PwlWaveform::new(vec![(0.0, 2.0), (1.0, 3.0), (2.0, 8.0)])
            .unwrap()
            .with_scaling(scale, 3.0, offset, -1.0),
    );
    let spec = SourceSpec::PwlFile {
        path: "preloaded-clock-regression.csv".into(),
        time_scale: scale,
        value_scale: 3.0,
        time_offset: offset,
        value_offset: -1.0,
        delay,
        repeat_from: repeating.then_some(0.0),
    };
    let mut pair = sources(SourceSpec::Dc(0.0), SpiceDialect::Xyce);
    pair.0.source_specs[0] = Some(spec.clone());
    pair.1.source_specs[0] = Some(spec);
    pair.0.pwl_waveforms[0] = Some(waveform.clone());
    pair.1.pwl_waveforms[0] = Some(waveform);
    pair
}

fn slopes(pair: &(VoltageSources, CurrentSources), time: Value, expected: [Value; 2]) {
    for (side, expected) in [SourceTimeSide::LeftLimit, SourceTimeSide::RightLimit]
        .into_iter()
        .zip(expected)
    {
        for value in [
            pair.0.time_derivative_at_on_side(0, time, 1, side),
            pair.1.time_derivative_at_on_side(0, time, 1, side),
        ] {
            let value = value.expect("qualified regular source slope");
            assert!(
                (value - expected).abs() <= 3e-14 * expected.abs(),
                "{side:?}: {value:e} != {expected:e}"
            );
        }
    }
}

#[test]
fn physical_source_clock_paths_select_scaled_file_slopes_and_repeat_orientation() {
    for (scale, offset) in [(0.1, 0.13), (-0.1, 1.13)] {
        let delay = 0.07;
        let pair = file_pair(scale, offset, delay, true);
        let forward = scale > 0.0;
        let interior = [3.0 / scale, 15.0 / scale];
        slopes(
            &pair,
            (scale + offset) + delay,
            if forward {
                interior
            } else {
                [interior[1], interior[0]]
            },
        );
        for cycle in [0, 1, 3] {
            let seam = ((2.0 * scale + offset) + delay) + (2.0 * scale) * Value::from(cycle);
            let expected = [15.0 / scale, 3.0 / scale];
            slopes(
                &pair,
                seam,
                if forward {
                    expected
                } else {
                    [expected[1], expected[0]]
                },
            );
        }
    }
}

#[test]
fn physical_source_clock_paths_select_pat_incoming_and_outgoing_slopes() {
    let spec = SourceSpec::Pat {
        vhi: 1.0,
        vlo: 0.0,
        delay: 0.0,
        rise: 0.2,
        fall: 0.4,
        sample: 1.0,
        data: "B10".into(),
        repeat_count: -1,
    };
    let pair = sources(spec, SpiceDialect::Xyce);
    slopes(&pair, 0.8, [0.0, -2.5]);
    slopes(&pair, 1.2, [-2.5, 0.0]);
    slopes(&pair, 1.9, [0.0, 5.0]);
    slopes(&pair, 2.0, [5.0, 5.0]);
    slopes(&pair, 2.1, [5.0, 0.0]);
}

#[test]
fn physical_source_clock_paths_preserve_shifted_fractional_pwl_repeat_ramps() {
    let delay = 1e12_f64;
    let end = 0.0009;
    let start = 0.0001;
    let period = end - start;
    let pair = sources(
        SourceSpec::Pwl {
            points: vec![(0.0, 0.0), (0.0003, 3.0), (end, 9.0)],
            delay,
            repeat_from: Some(start),
        },
        SpiceDialect::Xyce,
    );
    let time = (delay + end).next_up();
    let expected = ((time - delay) - period) / 0.0003 * 3.0;
    for side in [SourceTimeSide::LeftLimit, SourceTimeSide::RightLimit] {
        for actual in [
            pair.0.transient_value_at_on_side(0, time, side),
            pair.1.value_at_time_on_side(0, time, side),
        ] {
            assert!((actual - expected).abs() < 2e-14, "{actual} != {expected}");
        }
    }
}

#[test]
fn physical_source_clock_paths_preserve_file_holds_fractional_seams_and_scaled_ratios() {
    for scale in [0.25, -0.25] {
        let mut pair = file_pair(scale, 10.0, 0.5, false);
        let first = 10.5;
        let end = 10.5 + 2.0 * scale;
        let oriented = |raw: [Value; 2]| if scale > 0.0 { raw } else { [raw[1], raw[0]] };
        slopes(&pair, first, oriented([0.0, 3.0 / scale]));
        slopes(&pair, end, oriented([15.0 / scale, 0.0]));
        for spec in [&mut pair.0.source_specs[0], &mut pair.1.source_specs[0]] {
            let Some(SourceSpec::PwlFile { repeat_from, .. }) = spec else {
                panic!()
            };
            *repeat_from = Some(0.5);
        }
        for cycle in [0.0, 1.0, 3.0] {
            slopes(
                &pair,
                end + 1.5 * scale * cycle,
                oriented([15.0 / scale, 3.0 / scale]),
            );
        }
    }
    let mut pair = file_pair(2.0, 0.0, 0.0, false);
    let waveform = Arc::new(
        crate::device::pwl_file::PwlWaveform::new(vec![
            (-Value::MAX, -Value::MAX),
            (Value::MAX, Value::MAX),
        ])
        .unwrap()
        .with_scaling(2.0, 3.0, 0.0, 0.0),
    );
    pair.0.pwl_waveforms[0] = Some(waveform.clone());
    pair.1.pwl_waveforms[0] = Some(waveform);
    slopes(&pair, 1.0, [1.5, 1.5]);
}

#[test]
fn physical_source_clock_paths_preserve_pat_shifted_corners_and_finite_counts() {
    for exponent in [-500, 0, 500] {
        let scale = 2.0_f64.powi(exponent);
        let delay = 1e12 * scale;
        for repeat_count in [-1, 0, 2] {
            let pair = sources(
                SourceSpec::Pat {
                    vhi: 1.0,
                    vlo: 0.0,
                    delay,
                    rise: 0.2 * scale,
                    fall: 0.4 * scale,
                    sample: scale,
                    data: "B10".into(),
                    repeat_count,
                },
                SpiceDialect::Xyce,
            );
            slopes(&pair, delay + 0.1 * scale, [0.0, 0.0]);
            for cycle in 0..=3 {
                let base = delay + Value::from(cycle) * (2.0 * scale);
                let ended = repeat_count >= 0 && cycle > repeat_count;
                slopes(
                    &pair,
                    base + 0.8 * scale,
                    if ended {
                        [0.0, 0.0]
                    } else {
                        [0.0, -2.5 / scale]
                    },
                );
                slopes(
                    &pair,
                    base + 1.2 * scale,
                    if ended {
                        [0.0, 0.0]
                    } else {
                        [-2.5 / scale, 0.0]
                    },
                );
                let ending = repeat_count >= 0 && cycle >= repeat_count;
                slopes(
                    &pair,
                    base + 1.9 * scale,
                    if ending {
                        [0.0, 0.0]
                    } else {
                        [0.0, 5.0 / scale]
                    },
                );
                if !ended {
                    for (at, expected) in [(0.8 * scale, 1.0), (1.2 * scale, 0.0)] {
                        for side in [SourceTimeSide::LeftLimit, SourceTimeSide::RightLimit] {
                            assert_eq!(
                                pair.0.transient_value_at_on_side(0, base + at, side),
                                expected
                            );
                            assert_eq!(pair.1.value_at_time_on_side(0, base + at, side), expected);
                        }
                    }
                }
            }
        }
    }
}

#[test]
fn physical_source_clock_paths_refuse_nonmonotonic_pat_points() {
    let pair = sources(
        SourceSpec::Pat {
            vhi: 1.0,
            vlo: 0.0,
            delay: 0.0,
            rise: 4.0,
            fall: 4.0,
            sample: 1.0,
            data: "B10".into(),
            repeat_count: -1,
        },
        SpiceDialect::Xyce,
    );
    for side in [SourceTimeSide::LeftLimit, SourceTimeSide::RightLimit] {
        assert_eq!(pair.0.time_derivative_at_on_side(0, 0.5, 1, side), None);
        assert_eq!(pair.1.time_derivative_at_on_side(0, 0.5, 1, side), None);
    }
}

#[test]
fn physical_source_clock_paths_preserve_pat_decimal_cycle_seams() {
    for delay in [0.07, 0.13, 1e12] {
        for sample in [0.1, 0.3, 0.003] {
            let duration = 2.0 * sample;
            for data in ["B10", "B01"] {
                let rise = 0.2 * sample;
                let fall = 0.4 * sample;
                let pair = sources(
                    SourceSpec::Pat {
                        vhi: 1.0,
                        vlo: 0.0,
                        delay,
                        rise,
                        fall,
                        sample,
                        data: data.into(),
                        repeat_count: -1,
                    },
                    SpiceDialect::Xyce,
                );
                let expected = if data == "B10" {
                    1.0 / rise
                } else {
                    -1.0 / fall
                };
                for cycle in [3.0, 6.0, 7.0] {
                    let seam = delay + cycle * duration;
                    let previous_end = (delay + (cycle - 1.0) * duration) + duration;
                    // Both timestamps occur in the event catalogue when
                    // floating-point association gives different results.
                    slopes(&pair, seam, [expected, expected]);
                    slopes(&pair, previous_end, [expected, expected]);
                    for side in [SourceTimeSide::LeftLimit, SourceTimeSide::RightLimit] {
                        assert_eq!(pair.0.transient_value_at_on_side(0, seam, side), 0.5);
                        assert_eq!(pair.1.value_at_time_on_side(0, seam, side), 0.5);
                    }
                }
            }
        }
    }
}
