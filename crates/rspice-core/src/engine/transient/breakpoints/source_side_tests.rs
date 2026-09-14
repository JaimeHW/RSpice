//! Source-side evaluation and the engine scheduler share exact event clocks.

use super::*;
use crate::circuit::SourceTimeSide;

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
