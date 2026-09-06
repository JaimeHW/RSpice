//! What the VCD writer emits, the VCD reader reads back unchanged.
//!
//! The inline tests in `io::vcd` pin one document's bytes. This states the
//! law over generated documents instead: for any set of signals, widths,
//! four-state values, aliases, reals and sparse ticks the format can carry,
//! `write` then `parse` is the identity.
//!
//! The second half is the other end of the same claim — that the projection
//! from an event history chooses a `$timescale` at which every event time is
//! an exact tick, and refuses rather than quantising when there is none.

use proptest::prelude::*;

use rspice_core::engine::{DigitalTrace, DigitalTracePoint};
use rspice_core::execution::{EventProjectionError, event_vcd_document};
use rspice_core::io::{
    VcdBit, VcdChange, VcdDocument, VcdMagnitude, VcdSignal, VcdSignalKind, VcdTimeUnit,
    VcdTimescale, VcdValue, VcdVariable, parse_vcd_reader, write_vcd,
};
use rspice_core::xspice::DigitalValue;

/// Longest variable this exercises. Well past the 53 bits an `f64`-mapping
/// reader has to stop at, which is the point.
const MAX_WIDTH: u32 = 256;

fn timescale() -> impl Strategy<Value = VcdTimescale> {
    (
        prop_oneof![
            Just(VcdMagnitude::One),
            Just(VcdMagnitude::Ten),
            Just(VcdMagnitude::Hundred),
        ],
        prop_oneof![
            Just(VcdTimeUnit::Seconds),
            Just(VcdTimeUnit::Milliseconds),
            Just(VcdTimeUnit::Microseconds),
            Just(VcdTimeUnit::Nanoseconds),
            Just(VcdTimeUnit::Picoseconds),
            Just(VcdTimeUnit::Femtoseconds),
        ],
    )
        .prop_map(|(magnitude, unit)| VcdTimescale { magnitude, unit })
}

fn bit() -> impl Strategy<Value = VcdBit> {
    prop_oneof![
        Just(VcdBit::Zero),
        Just(VcdBit::One),
        Just(VcdBit::Unknown),
        Just(VcdBit::HighImpedance),
    ]
}

/// Header text that survives the grammar: no `$end`, and no run of
/// whitespace, which the token reader would collapse.
fn header_text() -> impl Strategy<Value = String> {
    prop_oneof![
        Just(String::new()).boxed(),
        "[a-z]{1,6}( [a-z]{1,6}){0,2}".boxed(),
    ]
}

fn variable() -> impl Strategy<Value = VcdVariable> {
    (
        prop::collection::vec("[a-z][a-z0-9_]{0,5}", 0..3),
        "[a-z][a-z0-9_]{0,7}",
    )
        .prop_map(|(scope, name)| VcdVariable { scope, name })
}

fn ticks(count: usize) -> impl Strategy<Value = Vec<u64>> {
    prop::collection::vec(0_u64..1_000_000, count).prop_map(|mut ticks| {
        ticks.sort_unstable();
        ticks
    })
}

fn logic_signal() -> impl Strategy<Value = VcdSignal> {
    (1_u32..=MAX_WIDTH, 0_usize..6)
        .prop_flat_map(|(width, changes)| {
            (
                Just(width),
                prop::collection::vec(variable(), 1..3),
                ticks(changes),
                prop::collection::vec(
                    prop::collection::vec(bit(), width as usize..=width as usize),
                    changes,
                ),
            )
        })
        .prop_map(|(width, variables, ticks, values)| VcdSignal {
            identifier: String::new(),
            variables,
            width,
            kind: VcdSignalKind::Logic,
            changes: ticks
                .into_iter()
                .zip(values)
                .map(|(tick, bits)| VcdChange {
                    tick,
                    value: VcdValue::Logic(bits),
                })
                .collect(),
        })
}

fn real_signal() -> impl Strategy<Value = VcdSignal> {
    (0_usize..6)
        .prop_flat_map(|changes| {
            (
                prop::collection::vec(variable(), 1..3),
                ticks(changes),
                prop::collection::vec(
                    proptest::num::f64::NORMAL
                        | proptest::num::f64::SUBNORMAL
                        | proptest::num::f64::ZERO,
                    changes,
                ),
            )
        })
        .prop_map(|(variables, ticks, values)| VcdSignal {
            identifier: String::new(),
            variables,
            width: 64,
            kind: VcdSignalKind::Real,
            changes: ticks
                .into_iter()
                .zip(values)
                .map(|(tick, real)| VcdChange {
                    tick,
                    value: VcdValue::Real(real),
                })
                .collect(),
        })
}

fn document() -> impl Strategy<Value = VcdDocument> {
    (
        timescale(),
        header_text(),
        header_text(),
        prop::collection::vec(header_text(), 0..3),
        prop::collection::vec(prop_oneof![logic_signal(), real_signal()], 0..5),
    )
        .prop_map(|(timescale, date, version, comments, signals)| {
            let mut document = VcdDocument::new(timescale);
            document.date = date;
            document.version = version;
            document.comments = comments;
            document.signals = signals;
            document.assign_canonical_identifiers();
            document
        })
}

fn round_trip(document: &VcdDocument) -> VcdDocument {
    let mut bytes = Vec::new();
    write_vcd(&mut bytes, document).expect("the generated document is writable");
    parse_vcd_reader(bytes.as_slice()).expect("what the writer emits, the reader reads")
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(128))]

    /// Ticks, bits, reals, widths, aliases, scopes and header text all come
    /// back as they went in.
    #[test]
    fn a_written_document_reads_back_as_itself(document in document()) {
        prop_assert_eq!(round_trip(&document), document.clone());
    }

    /// Writing what was just read produces the same bytes again, so the
    /// encoding has no state the second pass could differ on.
    #[test]
    fn writing_is_idempotent(document in document()) {
        let mut first = Vec::new();
        write_vcd(&mut first, &document).expect("writable");
        let mut second = Vec::new();
        write_vcd(&mut second, &round_trip(&document)).expect("writable");
        prop_assert_eq!(first, second);
    }
}

fn digital_trace(node: &str, times: &[f64]) -> DigitalTrace {
    DigitalTrace {
        node_name: node.to_string(),
        points: times
            .iter()
            .enumerate()
            .map(|(index, time)| DigitalTracePoint {
                time: *time,
                value: if index % 2 == 0 {
                    DigitalValue::zero()
                } else {
                    DigitalValue::one()
                },
            })
            .collect(),
    }
}

#[test]
fn the_timescale_makes_every_event_time_an_exact_tick() {
    // Each case is the coarsest scale that divides all of its times. The `ns`
    // case lands on `1 ns` rather than `2 ns`, which the format cannot spell.
    let cases: [(&[f64], &str, &[u64]); 6] = [
        (&[0.0, 1e-3, 2e-3], "1 ms", &[0, 1, 2]),
        (&[0.0, 1e-6, 5e-6], "1 us", &[0, 1, 5]),
        (&[0.0, 2e-9, 4e-9, 6e-9], "1 ns", &[0, 2, 4, 6]),
        (&[0.0, 1e-12, 7e-12], "1 ps", &[0, 1, 7]),
        (&[0.0, 3e-15, 4e-15], "1 fs", &[0, 3, 4]),
        (&[0.0, 100e-15, 300e-15], "100 fs", &[0, 1, 3]),
    ];
    for (times, expected, expected_ticks) in cases {
        let document = event_vcd_document("tran", &[digital_trace("clk", times)], &[], &[])
            .expect("every time is a whole femtosecond");
        assert_eq!(document.timescale.to_string(), expected, "{times:?}");

        let signal = document.signals.first().expect("one signal");
        let ticks: Vec<u64> = signal.changes.iter().map(|change| change.tick).collect();
        assert_eq!(ticks, expected_ticks, "{times:?}");

        // The tick grid means what it says: tick times period is the event
        // time again, to the femtosecond.
        let period = document.timescale.seconds();
        for (tick, time) in ticks.iter().zip(times.iter()) {
            let reconstructed = *tick as f64 * period;
            assert!(
                (reconstructed - time).abs() <= time.abs() * 1e-9 + 1e-16,
                "tick {tick} at {period} s is not {time} s"
            );
        }

        // And the document the projection built is one the codec can carry.
        assert_eq!(round_trip(&document), document);
    }
}

#[test]
fn a_time_between_two_femtoseconds_is_refused_rather_than_quantised() {
    let error = event_vcd_document("tran", &[digital_trace("clk", &[0.0, 1.5e-15])], &[], &[])
        .expect_err("half a femtosecond has no exact tick at any VCD timescale");
    assert_eq!(
        error,
        EventProjectionError::InexactTime {
            node: "clk".to_string(),
            time: 1.5e-15,
        }
    );

    // A tenth of a femtosecond is the same refusal, not a rounding.
    assert!(matches!(
        event_vcd_document("tran", &[digital_trace("clk", &[0.0, 1.1e-15])], &[], &[]),
        Err(EventProjectionError::InexactTime { .. })
    ));
}
