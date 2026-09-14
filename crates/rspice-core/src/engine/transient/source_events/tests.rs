use super::*;
use crate::abort_signal::{CountingAbort, NoAbort};
use crate::resource::ResourceLimits;

fn circuit(text: &str, dialect: SpiceDialect) -> crate::CircuitData {
    let mut engine = Engine::default();
    engine.config.spice_dialect = dialect;
    let mut circuit = engine
        .build_circuit(&Netlist::parse(text).unwrap())
        .unwrap();
    circuit.set_independent_source_context(
        SourceTimeBasis {
            tstep: 0.07,
            tstop: 4.0,
        },
        dialect,
        ResourceLimits::default(),
    );
    circuit
}

fn collect(circuit: &crate::CircuitData, stop: Value) -> PhysicalSourceEvents {
    Engine::collect_physical_source_events(circuit, stop, &ResourceLimits::default(), &NoAbort)
        .unwrap()
}

#[test]
fn physical_source_events_keep_adjacent_clocks_and_simultaneous_source_owners() {
    let next = 1.0_f64.next_up();
    let mut circuit = circuit(
        &format!(
            "exact source ownership\nVdc dc 0 1\nV1 a 0 PWL(0 0 1 0 1 1)\nV2 b 0 PWL(0 0 {next:.17e} 0 {next:.17e} 1)\nI1 0 n PWL(0 0 1 0 1 2)\nRn n 0 1k\n.end\n"
        ),
        SpiceDialect::Xyce,
    );
    // An ordinary stored DC source has no transient specification.
    circuit.voltage_sources.source_specs[0] = None;
    let schedule = collect(&circuit, 2.0);
    assert_eq!(
        schedule
            .at(1.0)
            .unwrap()
            .iter()
            .map(|event| event.owner)
            .collect::<Vec<_>>(),
        [
            PhysicalSourceOwner::Voltage(1),
            PhysicalSourceOwner::Current(0)
        ]
    );
    assert_eq!(schedule.next_after(1.0, 2.0).unwrap(), Some(next));
    assert_eq!(
        schedule.at(next).unwrap()[0].owner,
        PhysicalSourceOwner::Voltage(2)
    );
    assert!(schedule.at(0.5).unwrap().is_empty());
    assert!(
        schedule
            .at(1.0)
            .unwrap()
            .iter()
            .all(|event| event.order == DelayEventOrder::AtLeast(0))
    );
    assert_eq!(schedule.next_after(next, 2.0).unwrap(), None);
    assert_eq!(collect(&circuit, 2.0), schedule);
    assert_eq!(schedule.next_after(0.5, 1.0).unwrap(), Some(1.0));
    assert!(schedule.at(Value::NAN).is_err());
    assert!(schedule.next_after(1.0, 0.5).is_err());
}

#[test]
fn physical_source_events_include_xyce_activations_with_the_prepared_defaults() {
    let circuit = circuit(
        "source activation policy\nVsin a 0 SIN(0 1 1 .2)\nVexp b 0 EXP(0 1 0)\n.end\n",
        SpiceDialect::Xyce,
    );
    let schedule = collect(&circuit, 0.4);
    assert_eq!(
        schedule
            .events
            .iter()
            .map(|event| event.time)
            .collect::<Vec<_>>(),
        [0.0, 0.07, 0.2]
    );
    assert!(
        schedule
            .events
            .iter()
            .all(|event| event.order == DelayEventOrder::AtLeast(1))
    );
    let mut ordinary = BreakpointManager::new();
    Engine::collect_independent_source_breakpoints(
        &circuit,
        BreakpointWindow {
            tstop: 0.4,
            tstep_hint: 0.001,
            dialect: SpiceDialect::Xyce,
        },
        None,
        &mut ordinary,
        &NoAbort,
        100,
        SourceBreakpointGeometry::Authored,
    )
    .unwrap();
    assert!(ordinary.times().is_empty(), "ordinary Xyce policy changed");
    assert_eq!(
        circuit
            .voltage_sources
            .time_derivative_at_on_side(0, 0.2, 1, SourceTimeSide::LeftLimit),
        Some(0.0)
    );
    assert!(
        circuit
            .voltage_sources
            .time_derivative_at_on_side(0, 0.2, 1, SourceTimeSide::RightLimit)
            .unwrap()
            > 6.0
    );
}

#[test]
fn physical_source_events_include_fractional_train_termination_without_inactive_edges() {
    let circuit = circuit(
        "bounded truncated pulse\nV1 a 0 PULSE(0 1 .125 .5 .25 2 1 1.25)\n.end\n",
        SpiceDialect::Xyce,
    );
    let schedule = collect(&circuit, 4.0);
    assert_eq!(
        schedule
            .events
            .iter()
            .map(|event| event.time)
            .collect::<Vec<_>>(),
        [0.125, 0.625, 1.125, 1.375]
    );
    assert_eq!(
        circuit
            .voltage_sources
            .transient_value_at_on_side(0, 1.375, SourceTimeSide::LeftLimit),
        0.5
    );
    assert_eq!(
        circuit
            .voltage_sources
            .transient_value_at_on_side(0, 1.375, SourceTimeSide::RightLimit),
        0.0
    );
    assert_eq!(
        schedule.at(1.375).unwrap()[0].order,
        DelayEventOrder::AtLeast(0)
    );
}

#[test]
fn physical_source_events_include_pwl_activation_and_retained_flat_repeat_end() {
    let circuit = circuit(
        "PWL activation and repeat\nV1 a 0 PWL(1 2 2 3) TD=.25\nV2 b 0 PWL(0 0 .25 1 .5 1 1 1) TD=.125 R=0\n.end\n",
        SpiceDialect::Xyce,
    );
    let schedule = collect(&circuit, 3.0);
    assert_eq!(
        schedule.at(0.25).unwrap()[0].owner,
        PhysicalSourceOwner::Voltage(0)
    );
    assert_eq!(
        circuit
            .voltage_sources
            .transient_value_at_on_side(0, 0.25, SourceTimeSide::LeftLimit),
        0.0
    );
    assert_eq!(
        circuit
            .voltage_sources
            .transient_value_at_on_side(0, 0.25, SourceTimeSide::RightLimit),
        2.0
    );
    for seam in [1.125, 2.125] {
        assert!(
            schedule
                .at(seam)
                .unwrap()
                .iter()
                .any(|event| event.owner == PhysicalSourceOwner::Voltage(1))
        );
        assert_eq!(
            circuit
                .voltage_sources
                .transient_value_at_on_side(1, seam, SourceTimeSide::LeftLimit),
            1.0
        );
        assert_eq!(
            circuit
                .voltage_sources
                .transient_value_at_on_side(1, seam, SourceTimeSide::RightLimit),
            0.0
        );
    }
}

#[test]
fn physical_source_events_keep_structural_order_bounds_conservative() {
    let circuit = circuit(
        "source continuity declarations\nV1 a 0 DC 2 PWL(0 0 1 1 2 0)\nV2 b 0 PWL(0 0 1 1 1 1 2 0)\nV3 c 0 PULSE(0 1 .125 .25 .25 .5 2)\nV4 d 0 EXP(0 1 .25 0 .75 .5)\nV5 e 0 EXP(0 1 .75 .5 .25 .5)\n.end\n",
        SpiceDialect::Xyce,
    );
    let schedule = collect(&circuit, 2.0);
    for event in &schedule.events {
        let expected = match event.owner {
            PhysicalSourceOwner::Voltage(0 | 2) => 1,
            _ => 0,
        };
        assert_eq!(event.order, DelayEventOrder::AtLeast(expected), "{event:?}");
    }
    // Equal sides at a duplicate knot do not upgrade the structural bound.
    assert!(
        !schedule
            .at(0.25)
            .unwrap()
            .iter()
            .any(|event| event.owner == PhysicalSourceOwner::Voltage(4))
    );
    assert_eq!(
        circuit
            .voltage_sources
            .transient_value_at_on_side(4, 0.75, SourceTimeSide::LeftLimit),
        0.0
    );
    assert!(
        circuit
            .voltage_sources
            .transient_value_at_on_side(4, 0.75, SourceTimeSide::RightLimit)
            < -0.6
    );
    assert_eq!(
        circuit
            .voltage_sources
            .transient_value_at_on_side(1, 1.0, SourceTimeSide::LeftLimit),
        circuit
            .voltage_sources
            .transient_value_at_on_side(1, 1.0, SourceTimeSide::RightLimit)
    );
}

#[test]
fn physical_source_events_use_file_snapshots_and_reverse_time_scaling() {
    use crate::circuit::SourceExcitation;
    use std::sync::Arc;
    for (scale, offset) in [
        (0.125, 0.0),
        (-0.125, 0.25),
        (-0.125, 0.75),
        // Millions of inactive cycles in either physical-time orientation.
        (0.125, -1000000.0),
        (-0.125, 1000000.25),
    ] {
        let mut circuit = circuit("file source owner\nR1 a 0 1k\n.end\n", SpiceDialect::Xyce);
        let spec = SourceSpec::PwlFile {
            path: "never-read-physical-source-snapshot.csv".into(),
            time_scale: scale,
            value_scale: 2.0,
            time_offset: offset,
            value_offset: -1.0,
            delay: 0.5,
            repeat_from: Some(0.0),
        };
        let waveform = Arc::new(
            crate::device::pwl_file::PwlWaveform::new(vec![(0.0, 1.0), (1.0, 2.0), (2.0, 4.0)])
                .unwrap()
                .with_scaling(scale, 2.0, offset, -1.0),
        );
        circuit.voltage_sources.add_with_ac_spec_and_pwl_waveform(
            "Vfile".into(),
            1,
            0,
            1,
            SourceExcitation {
                dc_value: 0.0,
                ac_magnitude: 0.0,
                ac_phase: 0.0,
                source_spec: Some(spec.clone()),
            },
            Some(waveform),
        );
        let schedule = collect(&circuit, 1.0);
        assert_eq!(
            schedule
                .events
                .iter()
                .map(|event| event.time)
                .collect::<Vec<_>>(),
            if scale < 0.0 && offset == 0.25 {
                vec![0.5, 0.625, 0.75]
            } else {
                vec![0.5, 0.625, 0.75, 0.875, 1.0]
            },
            "scale={scale}, offset={offset}"
        );
        if scale < 0.0 && offset >= 0.75 {
            for seam in [0.75, 1.0] {
                assert_eq!(
                    circuit.voltage_sources.transient_value_at_on_side(
                        0,
                        seam,
                        SourceTimeSide::LeftLimit
                    ),
                    1.0
                );
                assert_eq!(
                    circuit.voltage_sources.transient_value_at_on_side(
                        0,
                        seam,
                        SourceTimeSide::RightLimit
                    ),
                    7.0
                );
            }
        }
        assert!(
            schedule
                .events
                .iter()
                .all(|event| event.order == DelayEventOrder::AtLeast(0))
        );
        let mut missing = circuit.clone();
        missing.voltage_sources = VoltageSources::default();
        missing.voltage_sources.add_with_ac_spec_and_pwl_waveform(
            "Vfile".into(),
            1,
            0,
            1,
            SourceExcitation {
                dc_value: 0.0,
                ac_magnitude: 0.0,
                ac_phase: 0.0,
                source_spec: Some(spec),
            },
            None,
        );
        missing.set_independent_source_context(
            SourceTimeBasis {
                tstep: 0.07,
                tstop: 4.0,
            },
            SpiceDialect::Xyce,
            ResourceLimits::default(),
        );
        let error = Engine::collect_physical_source_events(
            &missing,
            1.0,
            &ResourceLimits::default(),
            &NoAbort,
        )
        .unwrap_err();
        assert!(
            error.to_string().contains("circuit-owned PWL snapshot"),
            "{error}"
        );
    }
}

#[test]
fn physical_source_events_refuse_context_mismatch_limits_and_cancellation() {
    let points = SourceSpec::Pwl {
        points: (0..2048)
            .map(|i| (Value::from(i), Value::from(i)))
            .collect(),
        delay: 0.0,
        repeat_from: None,
    };
    let signal = CountingAbort::new(2);
    assert!(matches!(
        continuous(
            &points,
            SourceTimeBasis {
                tstep: 0.1,
                tstop: 4.0
            },
            SpiceDialect::Xyce,
            &signal
        ),
        Err(SimulationError::Aborted)
    ));
    assert_eq!(signal.observed_at(), Some(3));
    let mut circuit = circuit(
        "source schedule limits\nV1 a 0 SIN(0 1 1 .25)\nI1 0 n SIN(0 1 1 .25)\nR1 n 0 1k\n.end\n",
        SpiceDialect::Xyce,
    );
    let mut limits = ResourceLimits {
        max_analysis_points: 1,
        ..Default::default()
    };
    assert!(
        Engine::collect_physical_source_events(&circuit, 1.0, &limits, &NoAbort).is_err(),
        "two source owners must not share one record budget"
    );
    limits.max_analysis_points = 10;
    limits.max_result_values = 0;
    assert!(Engine::collect_physical_source_events(&circuit, 1.0, &limits, &NoAbort).is_err());
    for threshold in [1, 3, 5] {
        assert!(matches!(
            Engine::collect_physical_source_events(
                &circuit,
                1.0,
                &ResourceLimits::default(),
                &CountingAbort::new(threshold)
            ),
            Err(SimulationError::Aborted)
        ));
    }
    circuit
        .current_sources
        .set_transient_context_with_dialect(0.07, 4.0, SpiceDialect::Ngspice);
    let error =
        Engine::collect_physical_source_events(&circuit, 1.0, &ResourceLimits::default(), &NoAbort)
            .unwrap_err();
    assert!(
        error
            .to_string()
            .contains("matching voltage/current defaults and dialects"),
        "{error}"
    );
    let raw = Engine::default()
        .build_circuit(&Netlist::parse("unprepared source\nV1 a 0 SIN(0 1 1 .25)\n.end\n").unwrap())
        .unwrap();
    assert!(
        Engine::collect_physical_source_events(&raw, 1.0, &ResourceLimits::default(), &NoAbort)
            .is_err()
    );
}

#[test]
fn physical_source_events_cover_all_original_bug805_source_roots() {
    for text in [
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../tests/xyce/Netlists/Certification_Tests/BUG_805/colpitts_osc1.cir"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../tests/xyce/Netlists/Certification_Tests/BUG_805/colpitts_osc2.cir"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../tests/xyce/Netlists/Certification_Tests/BUG_805/colpitts_osc3.cir"
        )),
    ] {
        let circuit = circuit(text, SpiceDialect::Xyce);
        let schedule = collect(&circuit, 80e-6);
        assert_eq!(
            schedule
                .events
                .iter()
                .map(|event| event.time)
                .collect::<Vec<_>>(),
            [0.0, 5e-9]
        );
        assert!(
            schedule
                .events
                .iter()
                .all(|event| event.order == DelayEventOrder::AtLeast(1))
        );
        let owner = match schedule.events[0].owner {
            PhysicalSourceOwner::Voltage(index) => index,
            _ => panic!("voltage root"),
        };
        assert!(circuit.voltage_sources.names[owner].eq_ignore_ascii_case("V2"));
    }
}

#[test]
fn physical_source_events_refuse_collapsed_periods_and_skip_flat_pulses() {
    let circuit = circuit(
        "flat pulse and repeated PWL\nV1 a 0 PWL(0 0 .25 1 .5 0) R=0\nV2 b 0 PULSE(0 1 0 .1 .1 1 -1)\n.end\n",
        SpiceDialect::Xyce,
    );
    let schedule = collect(&circuit, 1.0);
    assert_eq!(
        schedule
            .events
            .iter()
            .map(|event| event.time)
            .collect::<Vec<_>>(),
        [0.0, 0.25, 0.5, 0.75, 1.0]
    );
    assert!(
        schedule
            .events
            .iter()
            .all(|event| event.owner == PhysicalSourceOwner::Voltage(0))
    );
    let mut engine = Engine::default();
    engine.config.spice_dialect = SpiceDialect::Xyce;
    let mut collapsed = engine
        .build_circuit(
            &Netlist::parse("collapsed period\nV1 a 0 PWL(0 0 1e-30 1) TD=1 R=0\n.end\n").unwrap(),
        )
        .unwrap();
    collapsed.set_independent_source_context(
        SourceTimeBasis {
            tstep: 0.1,
            tstop: 2.0,
        },
        SpiceDialect::Xyce,
        ResourceLimits::default(),
    );
    let error = Engine::collect_physical_source_events(
        &collapsed,
        2.0,
        &ResourceLimits::default(),
        &NoAbort,
    )
    .unwrap_err();
    assert!(error.to_string().contains("cannot advance"), "{error}");
}
