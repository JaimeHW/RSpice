use super::*;
use crate::config::SpiceDialect;
use crate::netlist::SourceSpec;

fn sources(spec: SourceSpec, dialect: SpiceDialect) -> (VoltageSources, CurrentSources) {
    let excitation = || SourceExcitation {
        dc_value: 17.0,
        ac_magnitude: 0.0,
        ac_phase: 0.0,
        source_spec: Some(spec.clone()),
    };
    let mut voltage = VoltageSources::new();
    voltage.add_with_ac_and_spec("V1".into(), 1, 0, 1, excitation());
    let mut current = CurrentSources::new();
    current.add_with_ac_and_spec("I1".into(), 1, 2, excitation());
    voltage.set_transient_context_with_dialect(0.125, 100.0, dialect);
    voltage.finalize_constraint_projection(2).unwrap();
    current.set_transient_context_with_dialect(0.125, 100.0, dialect);
    (voltage, current)
}

fn check(sources: &(VoltageSources, CurrentSources), time: Value, expected: [Value; 2]) {
    for (side, expected) in [SourceTimeSide::LeftLimit, SourceTimeSide::RightLimit]
        .into_iter()
        .zip(expected)
    {
        assert_eq!(
            sources.0.transient_value_at_on_side(0, time, side),
            expected,
            "voltage {time:e} {side:?}"
        );
        assert_eq!(
            sources.1.value_at_time_on_side(0, time, side),
            expected,
            "current {time:e} {side:?}"
        );
        let mut rhs = [0.0; 3];
        sources
            .0
            .update_transient_rhs_on_side(&mut rhs, time, |_| 3, side);
        sources.1.stamp_transient_rhs_on_side(&mut rhs, time, side);
        assert_eq!(rhs, [-expected, expected, expected]);
        let mut projected = [0.0; 2];
        sources
            .0
            .enforce_voltage_constraints_on_side(&mut projected, time, side)
            .unwrap();
        assert_eq!(projected[0], expected);
    }
    let mut published = [0.0; 3];
    sources.0.update_transient_rhs(&mut published, time, |_| 3);
    sources.1.stamp_transient_rhs(&mut published, time);
    let value = sources.0.transient_value_at(0, time);
    assert_eq!(published, [-value, value, value]);
}

fn pulse(timing: [Value; 5], count: Value) -> SourceSpec {
    let [delay, rise, fall, width, period] = timing;
    SourceSpec::Pulse {
        v1: 0.0,
        v2: 1.0,
        delay,
        rise,
        fall,
        width,
        period,
        pulse_count: count,
        width_defaults_to_zero: false,
    }
}

#[test]
fn physical_source_sides_stamp_pulse_edges_without_changing_published_values() {
    let pair = sources(pulse([1.0, 0.0, 0.0, 1.0, 4.0], 0.0), SpiceDialect::Xyce);
    for (time, expected) in [
        (1.0, [0.0, 1.0]),
        (2.0, [1.0, 0.0]),
        (5.0, [0.0, 1.0]),
        (6.0, [1.0, 0.0]),
        (3.0, [0.0, 0.0]),
    ] {
        check(&pair, time, expected);
    }
    assert_eq!(pair.0.transient_value_at(0, 1.0), 0.0);
    assert_eq!(pair.0.transient_value_at(0, 2.0), 1.0);
    let zero_width = sources(pulse([1.0, 0.0, 0.0, 0.0, 4.0], 0.0), SpiceDialect::Xyce);
    check(&zero_width, 1.0, [0.0, 0.0]);
    // ngspice resolves explicit zero rise/fall to the analysis defaults.
    let resolved = sources(pulse([1.0, 0.0, 0.0, 1.0, 4.0], 0.0), SpiceDialect::Ngspice);
    check(&resolved, 1.0, [0.0, 0.0]);
    check(&resolved, 1.125, [1.0, 1.0]);
}

#[test]
fn physical_source_sides_preserve_corners_and_truncated_pulse_cycles() {
    for scale in [2.0_f64.powi(-900), 1.0, 2.0_f64.powi(900)] {
        let pair = sources(
            pulse([0.0, scale, scale, scale, 4.0 * scale], 0.0),
            SpiceDialect::Xyce,
        );
        check(&pair, scale, [1.0, 1.0]);
        check(&pair, 2.0 * scale, [1.0, 1.0]);
        check(&pair, 2.5 * scale, [0.5, 0.5]);
        check(&pair, 3.0 * scale, [0.0, 0.0]);
        assert!(pair.0.transient_value_at(0, scale.next_down()) < 1.0);
    }
    let partial = sources(pulse([0.0, 4.0, 1.0, 1.0, 2.0], 2.0), SpiceDialect::Xyce);
    check(&partial, 2.0, [0.5, 0.0]);
    check(&partial, 4.0, [0.5, 0.0]);
    check(&partial, 4.5, [0.0, 0.0]);
    let wrapped = sources(
        SourceSpec::DcTransient {
            dc_value: 99.0,
            transient: Box::new(pulse([1.0, 0.0, 0.0, 1.0, 4.0], 0.0)),
        },
        SpiceDialect::Xyce,
    );
    check(&wrapped, 1.0, [0.0, 1.0]);
}

#[test]
fn physical_source_sides_land_on_fused_decimal_event_clocks_without_widening_them() {
    let delay = 0.13;
    let period: Value = 0.1;
    let width = 0.037;
    let pair = sources(
        pulse([delay, 0.0, 0.0, width, period], 0.0),
        SpiceDialect::Xyce,
    );
    for cycle in [0, 1, 3, 11, 99, 999] {
        let start = period.mul_add(Value::from(cycle), delay);
        check(&pair, start, [0.0, 1.0]);
        check(&pair, start + width, [1.0, 0.0]);
        for side in [SourceTimeSide::LeftLimit, SourceTimeSide::RightLimit] {
            assert_eq!(
                pair.0
                    .transient_value_at_on_side(0, start.next_down(), side),
                0.0
            );
            assert_eq!(
                pair.0.transient_value_at_on_side(0, start.next_up(), side),
                1.0
            );
        }
    }
    // Multiple real edges can share one represented clock. Its incoming
    // value belongs to the first edge and its outgoing value to the last.
    let collapsed = sources(
        pulse([1.0, f64::from_bits(1), 0.0, 1.0, 4.0], 0.0),
        SpiceDialect::Xyce,
    );
    check(&collapsed, 1.0, [0.0, 1.0]);
}

#[test]
fn physical_source_sides_preserve_pwl_duplicate_knots_and_repeat_seams() {
    let pair = sources(
        SourceSpec::Pwl {
            points: vec![(0.0, 2.0), (1.0, 3.0), (1.0, 7.0), (2.0, 9.0)],
            delay: 1.0,
            repeat_from: Some(0.0),
        },
        SpiceDialect::Xyce,
    );
    check(&pair, 1.0, [0.0, 2.0]);
    check(&pair, 1.5, [2.5, 2.5]);
    check(&pair, 2.0, [3.0, 7.0]);
    check(&pair, 3.0, [9.0, 2.0]);
    check(&pair, 5.0, [9.0, 2.0]);
    let endpoint_jump = sources(
        SourceSpec::Pwl {
            points: vec![(0.0, 1.0), (0.0, 2.0), (1.0, 3.0), (1.0, 4.0)],
            delay: 0.0,
            repeat_from: None,
        },
        SpiceDialect::Ngspice,
    );
    check(&endpoint_jump, 0.0, [0.0, 2.0]);
    check(&endpoint_jump, 1.0, [3.0, 4.0]);
    let offset = 0.13;
    let knots = [0.0, 0.037, 0.1];
    let shifted = sources(
        SourceSpec::Pwl {
            points: vec![
                (knots[0], 2.0),
                (knots[1], 3.0),
                (knots[1], 7.0),
                (knots[2], 9.0),
            ],
            delay: offset,
            repeat_from: Some(0.0),
        },
        SpiceDialect::Xyce,
    );
    for cycle in [0, 1, 3, 11] {
        let time = (knots[1] + offset) + (knots[2] - knots[0]) * Value::from(cycle);
        check(&shifted, time, [3.0, 7.0]);
    }
}

#[test]
fn physical_source_sides_preserve_file_scaling_and_reversed_repeat_orientation() {
    use crate::device::pwl_file::PwlWaveform;
    for (scale, time, expected) in [(1.0, 12.0, [23.0, 5.0]), (-1.0, 8.0, [5.0, 23.0])] {
        let waveform = Arc::new(
            PwlWaveform::new(vec![(0.0, 2.0), (1.0, 3.0), (2.0, 8.0)])
                .unwrap()
                .with_scaling(scale, 3.0, 10.0, -1.0),
        );
        let spec = SourceSpec::PwlFile {
            path: "preloaded-fixture.pwl".into(),
            time_scale: scale,
            value_scale: 3.0,
            time_offset: 10.0,
            value_offset: -1.0,
            delay: 0.0,
            repeat_from: Some(0.0),
        };
        // Bind an actual loaded waveform; evaluation must not touch the path.
        let mut pair = sources(SourceSpec::Dc(0.0), SpiceDialect::Xyce);
        pair.0.source_specs[0] = Some(spec.clone());
        pair.1.source_specs[0] = Some(spec);
        pair.0.pwl_waveforms[0] = Some(waveform.clone());
        pair.1.pwl_waveforms[0] = Some(waveform);
        check(&pair, time, expected);
    }
    for (scale, offset, expected) in [(0.1, 0.13, [23.0, 5.0]), (-0.1, 1.13, [5.0, 23.0])] {
        let delay = 0.07;
        let waveform = Arc::new(
            PwlWaveform::new(vec![(0.0, 2.0), (1.0, 3.0), (2.0, 8.0)])
                .unwrap()
                .with_scaling(scale, 3.0, offset, -1.0),
        );
        let spec = SourceSpec::PwlFile {
            path: "preloaded-decimal.pwl".into(),
            time_scale: scale,
            value_scale: 3.0,
            time_offset: offset,
            value_offset: -1.0,
            delay,
            repeat_from: Some(0.0),
        };
        let mut pair = sources(SourceSpec::Dc(0.0), SpiceDialect::Xyce);
        pair.0.source_specs[0] = Some(spec.clone());
        pair.1.source_specs[0] = Some(spec);
        pair.0.pwl_waveforms[0] = Some(waveform.clone());
        pair.1.pwl_waveforms[0] = Some(waveform);
        for cycle in [0, 1, 3] {
            check(
                &pair,
                ((2.0 * scale + offset) + delay) + (2.0 * scale) * Value::from(cycle),
                expected,
            );
        }
    }
}

#[test]
fn physical_source_sides_share_the_public_pwl_event_clock_and_refuse_collapsed_periods() {
    let engine = crate::engine::Engine::default();
    let deck = crate::Netlist::parse(
        "exact PWL events\nV1 in 0 PWL(0 2 0.037 3 0.037 7 0.1 9) TD=0.13 R=0\nR1 in 0 1k\n.end\n",
    )
    .unwrap();
    let events = engine
        .transient_source_event_times(&deck, 1.3, 0.1, &["V1".into()])
        .unwrap();
    let circuit = engine.build_circuit(&deck).unwrap();
    for cycle in [0, 1, 3, 11] {
        let event = (0.037 + 0.13) + 0.1 * Value::from(cycle);
        assert!(
            events.contains(&event),
            "missing clock {event:.17e}: {events:?}"
        );
        assert_eq!(
            circuit
                .voltage_sources
                .transient_value_at_on_side(0, event, SourceTimeSide::LeftLimit),
            3.0
        );
        assert_eq!(
            circuit.voltage_sources.transient_value_at_on_side(
                0,
                event,
                SourceTimeSide::RightLimit
            ),
            7.0
        );
    }
    assert!(
        events.windows(2).all(|pair| pair[1] - pair[0] > 0.01),
        "one seam must not split into adjacent clocks: {events:?}"
    );
    let collapsed = crate::Netlist::parse(
        "unrepresentable PWL period\nV1 in 0 PWL(0 0 1e-30 1) TD=1 R=0\nR1 in 0 1k\n.end\n",
    )
    .unwrap();
    let error = engine
        .transient_source_event_times(&collapsed, 2.0, 0.1, &["V1".into()])
        .unwrap_err();
    assert!(error.to_string().contains("cannot advance"), "{error}");
    // The clock can advance initially and then lose resolution at a power of
    // two. Reject that later loss too, rather than silently discarding cycles.
    let period = 0.75 * Value::EPSILON;
    let delay = 1.0_f64.next_down();
    let late = crate::Netlist::parse(&format!("later PWL resolution loss\nV1 in 0 PWL(0 0 {period:.17e} 1) TD={delay:.17e} R=0\nR1 in 0 1k\n.end\n")).unwrap();
    let error = engine
        .transient_source_event_times(&late, 1.0 + 8.0 * Value::EPSILON, 0.1, &["V1".into()])
        .unwrap_err();
    assert!(error.to_string().contains("cannot advance"), "{error}");
}

#[test]
fn physical_source_sides_preserve_zero_tau_exponential_limits() {
    let spec = SourceSpec::Exp {
        v1: 2.0,
        v2: 5.0,
        td1: 1.0,
        tau1: 0.0,
        td2: 2.0,
        tau2: 0.0,
    };
    let pair = sources(spec, SpiceDialect::Xyce);
    check(&pair, 1.0, [2.0, 5.0]);
    check(&pair, 2.0, [5.0, 2.0]);
    check(&pair, 1.5, [5.0, 5.0]);
    let smooth = sources(
        SourceSpec::Exp {
            v1: 2.0,
            v2: 5.0,
            td1: 1.0,
            tau1: 1.0,
            td2: 2.0,
            tau2: 1.0,
        },
        SpiceDialect::Xyce,
    );
    check(&smooth, 1.0, [2.0, 2.0]);
    let left = smooth
        .0
        .transient_value_at_on_side(0, 2.0, SourceTimeSide::LeftLimit);
    let right = smooth
        .0
        .transient_value_at_on_side(0, 2.0, SourceTimeSide::RightLimit);
    assert_eq!(left, right);
    assert!((left - (2.0 + 3.0 * (1.0 - (-1.0_f64).exp()))).abs() < 1e-14);
}

#[test]
fn physical_source_sides_preserve_delayed_modulation_start_values() {
    let fm = sources(
        SourceSpec::Sffm {
            offset: 2.0,
            amplitude: 3.0,
            carrier_freq: 1.0,
            modulation_index: 0.0,
            signal_freq: 1.0,
            delay: 1.0,
            phase_modulation: 0.0,
            phase_carrier: 90.0,
        },
        SpiceDialect::Ngspice,
    );
    check(&fm, 1.0, [0.0, 5.0]);
    let mut continuous = fm.clone();
    continuous
        .0
        .set_transient_context_with_dialect(0.125, 100.0, SpiceDialect::Xyce);
    continuous
        .1
        .set_transient_context_with_dialect(0.125, 100.0, SpiceDialect::Xyce);
    let published = continuous.0.transient_value_at(0, 1.0);
    check(&continuous, 1.0, [published, published]);
    let am = sources(
        SourceSpec::Am {
            offset: 2.0,
            modulation_offset: 1.0,
            modulation_amplitude: 0.0,
            modulating_freq: 1.0,
            carrier_freq: 1.0,
            delay: 1.0,
            phase_modulation: 0.0,
            phase_carrier: 90.0,
        },
        SpiceDialect::Xyce,
    );
    check(&am, 1.0, [0.0, 3.0]);
}
