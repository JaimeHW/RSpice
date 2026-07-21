use rspice_core::engine::{Engine, SimulationConfig, SimulationError};
use rspice_core::netlist::Netlist;
use rspice_core::{ResourceLimits, abort_signal::AbortSignal};

struct AlwaysAbort;

impl AbortSignal for AlwaysAbort {
    fn is_aborted(&self) -> bool {
        true
    }
}

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
fn selected_source_events_use_the_transient_breakpoint_contract() {
    let engine = Engine::new(SimulationConfig::default());
    let events = engine
        .transient_source_event_times(&event_deck(), 6.0e-6, 1.0e-7, &["vPuLsE".to_string()])
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
        .transient_source_event_times(&event_deck(), 6.0e-6, 1.0e-7, &[])
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
        .transient_source_event_times(&event_deck(), 6.0e-6, 1.0e-7, &["missing".to_string()])
        .expect_err("unknown source must fail closed");

    assert!(
        matches!(error, SimulationError::Circuit(message) if message.contains("unknown independent source 'missing'"))
    );
}

#[test]
fn selected_sources_require_unique_time_varying_waveforms() {
    let engine = Engine::new(SimulationConfig::default());
    let duplicate = engine
        .validate_transient_source_names(
            &event_deck(),
            &["VpUlSe".to_string(), "vpulse".to_string()],
        )
        .expect_err("case-insensitive duplicate selections must fail");
    assert!(duplicate.to_string().contains("repeats independent source"));

    let dc_only =
        Netlist::parse("dc source\nVbias out 0 1\nR1 out 0 1k\n.end\n").expect("DC deck parses");
    let error = engine
        .validate_transient_source_names(&dc_only, &["Vbias".to_string()])
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
            &AlwaysAbort,
        )
        .expect_err("pre-aborted enumeration must stop");
    assert!(matches!(aborted, SimulationError::Aborted));

    let mut config = SimulationConfig::default();
    let mut limits = ResourceLimits::default();
    limits.max_analysis_points = 3;
    config.resource_limits = limits;
    let bounded = Engine::new(config)
        .transient_source_event_times(&event_deck(), 6.0e-6, 1.0e-7, &["Vpulse".to_string()])
        .expect_err("event schedule must obey the configured point limit");
    assert!(
        bounded.to_string().contains("analysis") && bounded.to_string().contains("3"),
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
        .validate_transient_source_names(&netlist, &["Xdrv.Vmod".to_string()])
        .expect("elaborated hierarchical source name should resolve");
    assert_eq!(
        engine
            .transient_source_names(&netlist)
            .expect("hierarchical source catalog resolves"),
        vec!["Xdrv.VMOD".to_string()]
    );
}
