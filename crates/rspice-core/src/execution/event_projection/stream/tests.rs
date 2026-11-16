use super::*;
use crate::engine::{DigitalTracePoint, RealTracePoint};
use crate::io::{VcdError, write_vcd, write_vcd_streamed};

#[derive(Debug)]
enum Failure {
    Projection(EventProjectionError),
    Vcd(String),
}

impl From<VcdError> for Failure {
    fn from(error: VcdError) -> Self {
        Self::Vcd(error.to_string())
    }
}

fn level(state: DigitalState) -> DigitalValue {
    DigitalValue::new(state, DigitalStrength::Strong)
}

fn digital(node: &str, precision: Option<Value>, points: &[(Value, DigitalState)]) -> DigitalTrace {
    DigitalTrace {
        node_name: node.to_string(),
        points: points
            .iter()
            .map(|&(time, state)| DigitalTracePoint {
                time,
                value: level(state),
            })
            .collect(),
        hdl_time_precision: precision,
    }
}

fn real(node: &str, points: &[(Value, Value)]) -> RealTrace {
    RealTrace {
        node_name: node.to_string(),
        points: points
            .iter()
            .map(|&(time, value)| RealTracePoint { time, value })
            .collect(),
    }
}

fn bus(name: &str, msb: i64, lsb: i64, members: &[&str]) -> DigitalBusDeclaration {
    DigitalBusDeclaration::new(
        name,
        msb,
        lsb,
        members.iter().map(|member| (*member).to_string()).collect(),
        DigitalBusSource::Schematic,
    )
    .expect("the fixture bus is well formed")
}

/// The dump written from the document, and the one written from the stream.
fn both(
    digital: &[DigitalTrace],
    real: &[RealTrace],
    buses: &[DigitalBusDeclaration],
) -> (Result<String, String>, Result<String, String>) {
    let held = event_vcd_document("events", digital, real, buses)
        .map_err(|error| format!("{error:?}"))
        .map(|document| {
            let mut bytes = Vec::new();
            write_vcd(&mut bytes, &document).expect("the document writes");
            String::from_utf8(bytes).expect("a dump is text")
        });
    let streamed = event_vcd_stream("events", digital, real, buses, HeldEvents { digital, real })
        .map_err(|error| format!("{error:?}"))
        .and_then(|stream| {
            let mut bytes = Vec::new();
            write_vcd_streamed(
                &mut bytes,
                stream.header(),
                stream
                    .changes()
                    .map(|change| change.map_err(Failure::Projection)),
            )
            .map_err(|failure| match failure {
                Failure::Projection(error) => format!("{error:?}"),
                Failure::Vcd(error) => error,
            })?;
            Ok(String::from_utf8(bytes).expect("a dump is text"))
        });
    (held, streamed)
}

use DigitalState::{One, Unknown, Zero};

/// A dump written while its changes are read, a signal at a time and merged by tick, is the
/// dump the document gives, byte for byte: femtosecond and HDL grids, delta changes on one
/// instant and on one HDL tick, a value a later tick settles back to, real events, and a bus
/// folded from members on an HDL's grid, one of which has no point yet.
#[test]
fn a_streamed_dump_is_the_document_s_dump() {
    let digital = [
        digital(
            "clk",
            None,
            &[
                (0.0, Zero),
                (1e-9, One),
                (2e-9, Zero),
                (2e-9, One),
                (3.5e-9, Zero),
            ],
        ),
        digital(
            "hdl",
            Some(1e-12),
            &[
                (0.0, Zero),
                (1.0001e-9, One),
                (1.0002e-9, Zero),
                (1.4e-9, One),
                (2e-9, One),
                (2.0003e-9, Zero),
                (2.0004e-9 + 1e-16, One),
            ],
        ),
        digital("b0", Some(1e-9), &[(0.0, Zero), (2e-9, One), (4e-9, Zero)]),
        digital(
            "b1",
            Some(1e-9),
            &[(1e-9, One), (2e-9, Unknown), (2e-9, Zero)],
        ),
        digital("b2", Some(1e-9), &[]),
        digital("quiet", None, &[]),
    ];
    let real = [
        real("level", &[(0.0, 0.5), (1e-9, 0.5), (2.5e-9, -1.25)]),
        real("never", &[]),
    ];
    let buses = [bus("word", 2, 0, &["b2", "b1", "b0"])];
    for buses in [&buses[..], &[]] {
        let (held, streamed) = both(&digital, &real, buses);
        let held = held.expect("the document holds the histories");
        assert!(held.contains("$dumpvars"), "{held}");
        assert_eq!(streamed.as_deref(), Ok(held.as_str()));
    }
    // With no event, the dump is declarations alone.
    let (held, streamed) = both(&digital[5..], &real[1..], &[]);
    assert_eq!(streamed, held);
}

/// What the document refuses, the stream refuses with the same error.
#[test]
fn a_history_the_document_refuses_the_stream_refuses_alike() {
    let cases: [(
        Vec<DigitalTrace>,
        Vec<RealTrace>,
        Vec<DigitalBusDeclaration>,
    ); 5] = [
        // Two values one signal held at different times on one femtosecond.
        (
            vec![digital("a", None, &[(1e-9, Zero), (1e-9 + 1e-17, One)])],
            vec![],
            vec![],
        ),
        // A time before the one recorded before it.
        (
            vec![digital("a", None, &[(2e-9, Zero), (1e-9, One)])],
            vec![],
            vec![],
        ),
        // A real value that is not finite, after a signal that is fine.
        (
            vec![digital("a", None, &[(1e-9, Zero)])],
            vec![real("r", &[(0.0, 1.0), (1e-9, Value::NAN)])],
            vec![],
        ),
        // A precision no timescale names.
        (
            vec![digital("a", Some(3e-9), &[(1e-9, Zero)])],
            vec![],
            vec![],
        ),
        // A bus member out of order.
        (
            vec![
                digital("m0", None, &[(2e-9, Zero), (1e-9, One)]),
                digital("m1", None, &[]),
            ],
            vec![],
            vec![bus("w", 1, 0, &["m1", "m0"])],
        ),
    ];
    for (digital, real, buses) in &cases {
        let (held, streamed) = both(digital, real, buses);
        let held = held.expect_err("the document refuses the history");
        assert_eq!(streamed, Err(held));
    }
}
