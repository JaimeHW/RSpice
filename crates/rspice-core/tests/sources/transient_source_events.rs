use rspice_core::NoAbort;
use rspice_core::ResourceLimits;
use rspice_core::abort_signal::ImmediateAbort;
use rspice_core::engine::{Engine, SimulationConfig, SimulationError};
use rspice_core::netlist::Netlist;

fn contains_time(events: &[f64], expected: f64) -> bool {
    let tolerance = 32.0 * f64::EPSILON * expected.abs().max(f64::MIN_POSITIVE);
    events
        .iter()
        .any(|event| (*event - expected).abs() <= tolerance)
}

fn event_deck() -> Netlist {
    Netlist::parse(
        "* independent source event schedule\n\
         Vpulse pulse 0 PULSE(0 1 1u 100n 200n 2u 5u)\n\
         Ipwl pwl 0 PWL(0 0 2u 1 4u 0)\n\
         Vquiet quiet 0 SIN(0 1 1k 3u)\n\
         Rpulse pulse 0 1k\n\
         Rpwl pwl 0 1k\n\
         Rquiet quiet 0 1k\n\
         .end\n",
    )
    .expect("event deck parses")
}

#[test]
fn repeating_pwl_events_preserve_distinct_authored_times_at_small_scales() {
    for scale in [1e-6, 1e-18, 1e-30] {
        let netlist = Netlist::parse(&format!(
            "scaled PWL events\nI1 0 out PWL(0 0 {scale:e} 1 {:e} 0) R=0\nR1 out 0 1\n.end\n",
            2.0 * scale,
        ))
        .unwrap();
        let events = Engine::default()
            .transient_source_event_times_with_abort(
                &netlist,
                6.5 * scale,
                scale / 16.0,
                &[],
                &NoAbort,
            )
            .unwrap();
        for knot in 0..=6 {
            assert!(
                contains_time(&events, f64::from(knot) * scale),
                "scale={scale:e}, knot={knot}: {events:?}"
            );
        }
        assert!(events.windows(2).all(|pair| pair[0] < pair[1]));
    }
}

#[test]
fn authored_event_catalog_keeps_redundant_flat_pwl_knots() {
    let netlist = Netlist::parse(
        "authored flat events\nV1 in 0 PWL(0 0 1n 0 2n 0 3n 1 4n 0) R=0\nR1 in 0 1k\n.end\n",
    )
    .unwrap();
    let events = Engine::default()
        .transient_source_event_times_with_abort(&netlist, 4e-9, 1e-10, &[], &NoAbort)
        .unwrap();
    for knot in 0..=4 {
        assert!(
            contains_time(&events, knot as f64 * 1e-9),
            "missing authored knot {knot}: {events:?}"
        );
    }
}

#[test]
fn selected_source_events_use_the_transient_breakpoint_contract() {
    let engine = Engine::new(SimulationConfig::default());
    let events = engine
        .transient_source_event_times_with_abort(
            &event_deck(),
            6.0e-6,
            1.0e-7,
            &["vPuLsE".to_string()],
            &NoAbort,
        )
        .expect("pulse event schedule");

    let expected = [1.0e-6, 1.1e-6, 3.1e-6, 3.3e-6, 6.0e-6];
    assert_eq!(events.len(), expected.len());
    assert!(
        expected
            .iter()
            .all(|expected| contains_time(&events, *expected))
    );
}

#[test]
fn ac_annotations_preserve_waveform_event_schedules() {
    use rspice_core::netlist::ElementKind;
    let mut netlist = event_deck();
    let engine = Engine::default();
    let expected = engine
        .transient_source_event_times_with_abort(&netlist, 6e-6, 1e-7, &[], &NoAbort)
        .unwrap();
    for element in &mut netlist.elements {
        if let ElementKind::VoltageSource(spec) | ElementKind::CurrentSource(spec) =
            &mut element.kind
        {
            *spec = spec.clone().with_ac(1.0, 0.0);
        }
    }
    let actual = engine
        .transient_source_event_times_with_abort(&netlist, 6e-6, 1e-7, &[], &NoAbort)
        .unwrap();
    assert_eq!(actual, expected);
}

#[test]
fn transient_source_catalog_is_canonical_sorted_and_excludes_dc_only_sources() {
    let netlist = Netlist::parse(
        "source catalog\n\
         Vz z 0 PULSE(0 1 1u 1n 1n 1u 4u)\n\
         Ia a 0 SIN(0 1m 1k)\n\
         Vbias bias 0 1.2\n\
         Rz z 0 1k\n\
         Ra a 0 1k\n\
         Rbias bias 0 1k\n\
         .end\n",
    )
    .expect("catalog deck parses");

    let names = Engine::new(SimulationConfig::default())
        .transient_source_names(&netlist)
        .expect("source catalog resolves");
    assert_eq!(names, vec!["IA".to_string(), "VZ".to_string()]);
}

#[test]
fn all_source_events_are_sorted_and_deduplicated() {
    let engine = Engine::new(SimulationConfig::default());
    let events = engine
        .transient_source_event_times_with_abort(&event_deck(), 6.0e-6, 1.0e-7, &[], &NoAbort)
        .expect("complete event schedule");

    assert!(events.windows(2).all(|pair| pair[0] < pair[1]));
    for expected in [
        0.0, 1.0e-6, 1.1e-6, 2.0e-6, 3.0e-6, 3.1e-6, 3.3e-6, 4.0e-6, 6.0e-6,
    ] {
        assert!(
            contains_time(&events, expected),
            "missing expected source event {expected:e} from {events:?}"
        );
    }
}

#[test]
fn unknown_selected_source_is_rejected() {
    let engine = Engine::new(SimulationConfig::default());
    let error = engine
        .transient_source_event_times_with_abort(
            &event_deck(),
            6.0e-6,
            1.0e-7,
            &["missing".to_string()],
            &NoAbort,
        )
        .expect_err("unknown source must fail closed");

    assert!(
        matches!(error, SimulationError::Circuit(message) if message.contains("unknown independent source 'missing'"))
    );
}

#[test]
fn selected_sources_require_unique_time_varying_waveforms() {
    let engine = Engine::new(SimulationConfig::default());
    let duplicate = engine
        .validate_transient_source_names_with_abort(
            &event_deck(),
            &["VpUlSe".to_string(), "vpulse".to_string()],
            &NoAbort,
        )
        .expect_err("case-insensitive duplicate selections must fail");
    assert!(duplicate.to_string().contains("repeats independent source"));

    let dc_only =
        Netlist::parse("dc source\nVbias out 0 1\nR1 out 0 1k\n.end\n").expect("DC deck parses");
    let error = engine
        .validate_transient_source_names_with_abort(&dc_only, &["Vbias".to_string()], &NoAbort)
        .expect_err("DC-only source cannot be a modulation waveform");
    assert!(error.to_string().contains("no time-varying waveform"));
}

#[test]
fn event_enumeration_is_cancellable_and_resource_bounded() {
    let engine = Engine::new(SimulationConfig::default());
    let aborted = engine
        .transient_source_event_times_with_abort(
            &event_deck(),
            6.0e-6,
            1.0e-7,
            &["Vpulse".to_string()],
            &ImmediateAbort,
        )
        .expect_err("pre-aborted enumeration must stop");
    assert!(matches!(aborted, SimulationError::Aborted));

    let mut config = SimulationConfig::default();
    let mut limits = ResourceLimits::default();
    limits.max_result_values = 3;
    config.resource_limits = limits;
    let bounded = Engine::new(config)
        .transient_source_event_times_with_abort(
            &event_deck(),
            6.0e-6,
            1.0e-7,
            &["Vpulse".to_string()],
            &NoAbort,
        )
        .expect_err("event schedule must obey the configured value limit");
    assert!(
        bounded.to_string().contains("result_values") && bounded.to_string().contains("3"),
        "unexpected resource-bound error: {bounded}"
    );
}

#[test]
fn hierarchical_canonical_source_name_is_accepted() {
    let netlist = Netlist::parse(
        "hierarchical source\n\
         .subckt driver p n\n\
         Vmod p n PULSE(0 1 1u 10n 10n 1u 4u)\n\
         .ends driver\n\
         Xdrv out 0 driver\n\
         R1 out 0 1k\n\
         .end\n",
    )
    .expect("hierarchical deck parses");
    let engine = Engine::new(SimulationConfig::default());

    engine
        .validate_transient_source_names_with_abort(&netlist, &["Xdrv.Vmod".to_string()], &NoAbort)
        .expect("elaborated hierarchical source name should resolve");
    assert_eq!(
        engine
            .transient_source_names(&netlist)
            .expect("hierarchical source catalog resolves"),
        vec!["Xdrv.VMOD".to_string()]
    );
}

fn source_events(
    deck: &str,
    stop: f64,
    config: SimulationConfig,
) -> Result<Vec<f64>, SimulationError> {
    let netlist = Netlist::parse(deck).expect("deck parses");
    Engine::new(config).transient_source_event_times_with_abort(
        &netlist,
        stop,
        stop / 1.0e7,
        &[],
        &NoAbort,
    )
}

/// ngspice and Xyce schedule a repeating source for as many periods as the
/// run spans. Each of these used to be refused, or with an unlimited policy
/// silently truncated, past one million cycles; their edges now reach the
/// end of the run.
#[test]
fn repeating_sources_past_a_million_cycles_schedule_every_cycle() {
    // PULSE: 1.2 million 500 ns periods, four edges each.
    let pulse = source_events(
        "pulse train\nV1 p 0 PULSE(0 1 0 1n 1n 248n 500n)\nR1 p 0 1k\n.end\n",
        0.6,
        SimulationConfig::default(),
    )
    .expect("a 1.2-million-cycle pulse train is scheduled");
    assert!(pulse.len() >= 4 * 1_200_000, "{} edges", pulse.len());
    for cycle in [1_000_000.0, 1_100_000.0, 1_199_999.0] {
        let start = cycle * 500.0e-9;
        assert!(contains_time(&pulse, start), "cycle {cycle} start");
        assert!(
            contains_time(&pulse, start + 250.0e-9),
            "cycle {cycle} fall"
        );
    }

    // PAT: a two-bit pattern repeated 1.2 million times. Its 1-to-0 edge is
    // centred on the 10 ns bit boundary, 9.5 ns to 10.5 ns into each cycle.
    let pattern = source_events(
        "pattern\nV1 p 0 PAT(1 0 0 1n 1n 10n b10) R=-1\nR1 p 0 1k\n.end\n",
        24.0e-3,
        SimulationConfig::default(),
    )
    .expect("a 1.2-million-cycle pattern is scheduled");
    for cycle in [1_000_000.0, 1_199_999.0] {
        let start = cycle * 20.0e-9;
        assert!(
            contains_time(&pattern, start),
            "pattern cycle {cycle} start"
        );
        for edge in [9.5e-9, 10.5e-9] {
            assert!(
                contains_time(&pattern, start + edge),
                "pattern cycle {cycle} edge {edge:e}"
            );
        }
    }

    // PWL R=0: a 2 ns triangle repeated 1.2 million times.
    let pwl = source_events(
        "repeating pwl\nV1 p 0 PWL(0 0 1n 1 2n 0) R=0\nR1 p 0 1k\n.end\n",
        2.4e-3,
        SimulationConfig::default(),
    )
    .expect("a 1.2-million-cycle PWL repetition is scheduled");
    for cycle in [1_000_000.0, 1_199_999.0] {
        let start = cycle * 2.0e-9;
        assert!(
            contains_time(&pwl, start + 1.0e-9),
            "pwl cycle {cycle} peak"
        );
    }
}

/// A schedule no table can hold is refused before a single cycle is
/// enumerated: by the result-value policy when one is set, and by the
/// allocator as a typed failure when the policy is unlimited.
#[test]
fn a_repeating_source_no_table_can_hold_is_refused_before_enumeration() {
    let deck = "fine pulse\nV1 p 0 PULSE(0 1 0 1e-28 1e-28 1e-28 4e-28)\nR1 p 0 1k\n.end\n";
    let policy = source_events(deck, 1.0, SimulationConfig::default())
        .expect_err("2.5e27 cycles exceed the result-value policy");
    assert!(
        matches!(policy, SimulationError::ResourceLimit(_)),
        "{policy}"
    );

    let mut unlimited = SimulationConfig::default();
    unlimited.resource_limits = ResourceLimits::unlimited();
    let allocation =
        source_events(deck, 1.0, unlimited).expect_err("2.5e27 cycles exceed every address space");
    assert!(
        matches!(allocation, SimulationError::Allocation { .. }),
        "an unholdable schedule is an allocation failure, not an abort: {allocation:?}"
    );
}
