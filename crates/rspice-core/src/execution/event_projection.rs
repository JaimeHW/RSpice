//! Digital and real event histories, projected onto a VCD document.
//!
//! [`crate::execution::transient_projection_signals`] flattens an event
//! history onto the analog time grid, which is what a table export needs and
//! what a waveform viewer plots. This is the other projection: it keeps the
//! irregular timeline and the four-state value, which is what an event dump
//! is for.
//!
//! Two things are decided here rather than in [`crate::io::vcd`], because a
//! format module never names a result type: how a twelve-state
//! [`DigitalValue`] becomes one VCD bit, and which `$timescale` the times can
//! be written at without being quantised.

use thiserror::Error;

use crate::Value;
use crate::engine::{DigitalTrace, RealTrace};
use crate::io::vcd::{
    VCD_WRITER_VERSION, VcdBit, VcdChange, VcdDocument, VcdMagnitude, VcdSignal, VcdSignalKind,
    VcdTimeUnit, VcdTimescale, VcdValue, VcdVariable, is_writable_scope_name,
    is_writable_variable_name,
};
use crate::xspice::{DigitalState, DigitalStrength, DigitalValue};

/// Femtoseconds in one second: the finest tick VCD's `$timescale` can name.
const FEMTOSECONDS_PER_SECOND: f64 = 1e15;

/// Width a `real` variable conventionally declares.
const REAL_VARIABLE_WIDTH: u32 = 64;

/// An event history that cannot be written as VCD.
#[derive(Debug, Clone, PartialEq, Error)]
pub enum EventProjectionError {
    /// The scope name would not survive the `$scope` grammar.
    #[error("'{scope}' cannot name a VCD scope")]
    Scope {
        /// The rejected scope name.
        scope: String,
    },

    /// A node name would not survive the `$var` grammar.
    #[error("node '{node}' cannot name a VCD variable")]
    NodeName {
        /// The rejected node name.
        node: String,
    },

    /// The time is not a whole number of femtoseconds, so no `$timescale`
    /// carries it exactly.
    #[error(
        "node '{node}' has an event at {time} s, which is not a whole number of femtoseconds; \
         no VCD timescale carries it without quantising"
    )]
    InexactTime {
        /// The node whose event time was rejected.
        node: String,
        /// The rejected time, in seconds.
        time: Value,
    },

    /// The time is negative, not finite, or past the last tick a `u64` can
    /// address.
    #[error("node '{node}' has an event at {time} s, which is outside the range VCD can address")]
    UnrepresentableTime {
        /// The node whose event time was rejected.
        node: String,
        /// The rejected time, in seconds.
        time: Value,
    },

    /// An event history ran backwards.
    #[error("node '{node}' has an event at {time} s after a later one")]
    UnorderedTime {
        /// The node whose event history was rejected.
        node: String,
        /// The time that arrived out of order, in seconds.
        time: Value,
    },

    /// A real event value was infinite or NaN.
    #[error("node '{node}' has a non-finite value at {time} s")]
    NonFiniteValue {
        /// The node whose value was rejected.
        node: String,
        /// The time of the rejected value, in seconds.
        time: Value,
    },
}

/// The VCD bit a digital value shows as.
///
/// The level survives and the strength does not, because VCD has four bit
/// states and no drive strength:
///
/// | [`DigitalState`] | VCD |
/// |---|---|
/// | `Zero`, `ZeroR`, `ZeroZ` | `0` |
/// | `One`, `OneR`, `OneZ` | `1` |
/// | `Unknown`, `UnknownR`, `UnknownZ` | `x` |
/// | `HighZ` | `z` |
///
/// So a resistive one and a strong one are both `1` in the dump, and
/// [`DigitalValue::strength`] is dropped. A consumer that needs the band is
/// looking for [`DigitalValue::event_code`], not for a VCD file.
pub fn digital_value_to_vcd_bit(value: DigitalValue) -> VcdBit {
    match value.state {
        DigitalState::Zero | DigitalState::ZeroR | DigitalState::ZeroZ => VcdBit::Zero,
        DigitalState::One | DigitalState::OneR | DigitalState::OneZ => VcdBit::One,
        DigitalState::Unknown | DigitalState::UnknownR | DigitalState::UnknownZ => VcdBit::Unknown,
        DigitalState::HighZ => VcdBit::HighImpedance,
    }
}

/// The digital value a VCD bit reads back as.
///
/// The inverse of [`digital_value_to_vcd_bit`] cannot recover a strength the
/// format never stored, so it returns the strongly driven member of each
/// level: `0` and `1` come back [`DigitalStrength::Strong`], `x` comes back a
/// strong [`DigitalState::Unknown`], and `z` comes back
/// [`DigitalValue::high_z`] — the only value in its level, since a strongly
/// driven high impedance is a contradiction.
pub fn digital_value_from_vcd_bit(bit: VcdBit) -> DigitalValue {
    match bit {
        VcdBit::Zero => DigitalValue::new(DigitalState::Zero, DigitalStrength::Strong),
        VcdBit::One => DigitalValue::new(DigitalState::One, DigitalStrength::Strong),
        VcdBit::Unknown => DigitalValue::new(DigitalState::Unknown, DigitalStrength::Strong),
        VcdBit::HighImpedance => DigitalValue::high_z(),
    }
}

/// Project accepted event histories onto a VCD document.
///
/// Every trace becomes one variable inside a single `$scope module <scope>`:
/// a digital trace as a one-bit `wire`, a real trace as a `real`. Values keep
/// their own irregular timeline — nothing is resampled onto the analog grid.
///
/// # Timescale
///
/// Times are seconds; VCD counts integer ticks. Each event time is rounded to
/// the nearest femtosecond, and the `$timescale` chosen is the coarsest whose
/// period divides every one of them — so a run whose events all land on
/// nanoseconds is written `1 ns` with small tick numbers rather than `1 fs`
/// with large ones. A time that is not a whole number of femtoseconds has no
/// exact tick at any scale and is refused as
/// [`EventProjectionError::InexactTime`]: quantising it silently would move an
/// edge, and an event dump that moves edges is worse than no dump.
///
/// With no positive event time there is nothing to choose between the scales,
/// and the finest is used.
pub fn event_vcd_document(
    scope: &str,
    digital_traces: &[DigitalTrace],
    real_traces: &[RealTrace],
) -> Result<VcdDocument, EventProjectionError> {
    if !is_writable_scope_name(scope) {
        return Err(EventProjectionError::Scope {
            scope: scope.to_string(),
        });
    }

    let mut pending: Vec<PendingSignal> = Vec::new();
    for trace in digital_traces {
        let mut points = Vec::new();
        let mut previous: Option<Value> = None;
        for point in &trace.points {
            let femtoseconds = event_femtoseconds(&trace.node_name, point.time, &mut previous)?;
            points.push((
                femtoseconds,
                VcdValue::Logic(vec![digital_value_to_vcd_bit(point.value)]),
            ));
        }
        pending.push(PendingSignal {
            name: checked_node_name(&trace.node_name)?,
            kind: VcdSignalKind::Logic,
            width: 1,
            points,
        });
    }
    for trace in real_traces {
        let mut points = Vec::new();
        let mut previous: Option<Value> = None;
        for point in &trace.points {
            let femtoseconds = event_femtoseconds(&trace.node_name, point.time, &mut previous)?;
            if !point.value.is_finite() {
                return Err(EventProjectionError::NonFiniteValue {
                    node: trace.node_name.clone(),
                    time: point.time,
                });
            }
            points.push((femtoseconds, VcdValue::Real(point.value)));
        }
        pending.push(PendingSignal {
            name: checked_node_name(&trace.node_name)?,
            kind: VcdSignalKind::Real,
            width: REAL_VARIABLE_WIDTH,
            points,
        });
    }

    let timescale = choose_timescale(pending.iter().flat_map(|signal| &signal.points));
    let period = timescale.femtoseconds();
    let mut document = VcdDocument::new(timescale);
    document.version = VCD_WRITER_VERSION.to_string();
    document.signals = pending
        .into_iter()
        .map(|signal| VcdSignal {
            identifier: String::new(),
            variables: vec![VcdVariable {
                scope: vec![scope.to_string()],
                name: signal.name,
            }],
            width: signal.width,
            kind: signal.kind,
            changes: signal
                .points
                .into_iter()
                .map(|(femtoseconds, value)| VcdChange {
                    tick: femtoseconds / period,
                    value,
                })
                .collect(),
        })
        .collect();
    document.assign_canonical_identifiers();
    Ok(document)
}

/// One trace, converted but not yet placed on a tick grid.
struct PendingSignal {
    name: String,
    kind: VcdSignalKind,
    width: u32,
    points: Vec<(u64, VcdValue)>,
}

fn checked_node_name(node: &str) -> Result<String, EventProjectionError> {
    if is_writable_variable_name(node) {
        Ok(node.to_string())
    } else {
        Err(EventProjectionError::NodeName {
            node: node.to_string(),
        })
    }
}

/// Round one event time to whole femtoseconds, refusing anything that is not
/// already one.
///
/// The residual is compared against the width of the arithmetic rather than
/// against zero: `1e-9 * 1e15` is not exactly `1_000_000` in binary64, and a
/// test against zero would reject every nanosecond in the run. A time that is
/// genuinely between two femtoseconds misses by half a tick, which is many
/// orders of magnitude outside that width.
fn event_femtoseconds(
    node: &str,
    time: Value,
    previous: &mut Option<Value>,
) -> Result<u64, EventProjectionError> {
    if previous.is_some_and(|earlier| time < earlier) {
        return Err(EventProjectionError::UnorderedTime {
            node: node.to_string(),
            time,
        });
    }
    *previous = Some(time);

    if !time.is_finite() || time < 0.0 {
        return Err(EventProjectionError::UnrepresentableTime {
            node: node.to_string(),
            time,
        });
    }
    let scaled = time * FEMTOSECONDS_PER_SECOND;
    if scaled >= u64::MAX as f64 {
        return Err(EventProjectionError::UnrepresentableTime {
            node: node.to_string(),
            time,
        });
    }
    let ticks = scaled.round();
    let tolerance = (scaled * 8.0 * f64::EPSILON).max(1e-9);
    if (scaled - ticks).abs() > tolerance {
        return Err(EventProjectionError::InexactTime {
            node: node.to_string(),
            time,
        });
    }
    Ok(ticks as u64)
}

fn choose_timescale<'a>(points: impl Iterator<Item = &'a (u64, VcdValue)> + Clone) -> VcdTimescale {
    let finest = VcdTimescale {
        magnitude: VcdMagnitude::One,
        unit: VcdTimeUnit::Femtoseconds,
    };
    if points.clone().all(|(femtoseconds, _)| *femtoseconds == 0) {
        return finest;
    }
    VcdTimescale::ALL
        .into_iter()
        .find(|candidate| {
            let period = candidate.femtoseconds();
            points
                .clone()
                .all(|(femtoseconds, _)| femtoseconds % period == 0)
        })
        .unwrap_or(finest)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::{DigitalTracePoint, RealTracePoint};

    fn digital_trace(node: &str, points: &[(Value, DigitalValue)]) -> DigitalTrace {
        DigitalTrace {
            node_name: node.to_string(),
            points: points
                .iter()
                .map(|(time, value)| DigitalTracePoint {
                    time: *time,
                    value: *value,
                })
                .collect(),
        }
    }

    fn real_trace(node: &str, points: &[(Value, Value)]) -> RealTrace {
        RealTrace {
            node_name: node.to_string(),
            points: points
                .iter()
                .map(|(time, value)| RealTracePoint {
                    time: *time,
                    value: *value,
                })
                .collect(),
        }
    }

    const STRENGTHS: [DigitalStrength; 4] = [
        DigitalStrength::Strong,
        DigitalStrength::Resistive,
        DigitalStrength::HighZ,
        DigitalStrength::Undetermined,
    ];

    #[test]
    fn every_strength_band_keeps_its_level() {
        let levels = [
            (DigitalState::Zero, VcdBit::Zero),
            (DigitalState::ZeroR, VcdBit::Zero),
            (DigitalState::ZeroZ, VcdBit::Zero),
            (DigitalState::One, VcdBit::One),
            (DigitalState::OneR, VcdBit::One),
            (DigitalState::OneZ, VcdBit::One),
            (DigitalState::Unknown, VcdBit::Unknown),
            (DigitalState::UnknownR, VcdBit::Unknown),
            (DigitalState::UnknownZ, VcdBit::Unknown),
            (DigitalState::HighZ, VcdBit::HighImpedance),
        ];
        for (state, expected) in levels {
            for strength in STRENGTHS {
                assert_eq!(
                    digital_value_to_vcd_bit(DigitalValue::new(state, strength)),
                    expected,
                    "{state:?} at {strength:?}"
                );
            }
        }
    }

    #[test]
    fn the_reverse_mapping_drives_strongly() {
        assert_eq!(
            digital_value_from_vcd_bit(VcdBit::Zero),
            DigitalValue::new(DigitalState::Zero, DigitalStrength::Strong)
        );
        assert_eq!(
            digital_value_from_vcd_bit(VcdBit::One),
            DigitalValue::new(DigitalState::One, DigitalStrength::Strong)
        );
        assert_eq!(
            digital_value_from_vcd_bit(VcdBit::Unknown),
            DigitalValue::new(DigitalState::Unknown, DigitalStrength::Strong)
        );
        assert_eq!(
            digital_value_from_vcd_bit(VcdBit::HighImpedance),
            DigitalValue::high_z()
        );
        // Every bit survives the round trip back to itself, whatever the
        // strength the level came from.
        for state in [
            DigitalState::ZeroR,
            DigitalState::OneZ,
            DigitalState::UnknownR,
            DigitalState::HighZ,
        ] {
            for strength in STRENGTHS {
                let bit = digital_value_to_vcd_bit(DigitalValue::new(state, strength));
                assert_eq!(
                    digital_value_to_vcd_bit(digital_value_from_vcd_bit(bit)),
                    bit
                );
            }
        }
    }

    #[test]
    fn digital_traces_become_one_bit_wires_under_the_analysis_scope() {
        let document = event_vcd_document(
            "tran",
            &[digital_trace(
                "clk",
                &[
                    (0.0, DigitalValue::zero()),
                    (5e-9, DigitalValue::one()),
                    (10e-9, DigitalValue::high_z()),
                ],
            )],
            &[],
        )
        .expect("projection");

        assert_eq!(document.timescale.to_string(), "1 ns");
        assert_eq!(document.version, VCD_WRITER_VERSION);
        let signal = document.signals.first().expect("one signal");
        assert_eq!(signal.width, 1);
        assert_eq!(signal.kind, VcdSignalKind::Logic);
        assert_eq!(signal.identifier, "!");
        let variable = signal.variables.first().expect("one variable");
        assert_eq!(variable.scoped_name(), "tran.clk");
        let ticks: Vec<u64> = signal.changes.iter().map(|change| change.tick).collect();
        assert_eq!(ticks, vec![0, 5, 10]);
        assert_eq!(
            signal.changes.last().map(|change| change.value.clone()),
            Some(VcdValue::Logic(vec![VcdBit::HighImpedance]))
        );
    }

    #[test]
    fn real_traces_become_real_variables() {
        let document = event_vcd_document(
            "tran",
            &[],
            &[real_trace("out", &[(0.0, 1.5), (1e-12, -2.25)])],
        )
        .expect("projection");
        assert_eq!(document.timescale.to_string(), "1 ps");
        let signal = document.signals.first().expect("one signal");
        assert_eq!(signal.kind, VcdSignalKind::Real);
        assert_eq!(signal.width, 64);
        assert_eq!(
            signal.changes,
            vec![
                VcdChange {
                    tick: 0,
                    value: VcdValue::Real(1.5)
                },
                VcdChange {
                    tick: 1,
                    value: VcdValue::Real(-2.25)
                },
            ]
        );
    }

    #[test]
    fn the_timescale_is_the_coarsest_exact_one() {
        let cases: [(&[Value], &str, &[u64]); 5] = [
            (&[0.0, 10e-9, 20e-9], "10 ns", &[0, 1, 2]),
            (&[0.0, 5e-9, 10e-9], "1 ns", &[0, 5, 10]),
            (&[0.0, 1e-12, 3e-12], "1 ps", &[0, 1, 3]),
            (&[0.0, 2e-15, 3e-15], "1 fs", &[0, 2, 3]),
            (&[0.0, 100e-9, 300e-9], "100 ns", &[0, 1, 3]),
        ];
        for (times, expected, ticks) in cases {
            let points: Vec<(Value, DigitalValue)> = times
                .iter()
                .enumerate()
                .map(|(index, time)| {
                    (
                        *time,
                        if index % 2 == 0 {
                            DigitalValue::zero()
                        } else {
                            DigitalValue::one()
                        },
                    )
                })
                .collect();
            let document = event_vcd_document("tran", &[digital_trace("clk", &points)], &[])
                .expect("projection");
            assert_eq!(document.timescale.to_string(), expected, "{times:?}");
            let signal = document.signals.first().expect("one signal");
            let observed: Vec<u64> = signal.changes.iter().map(|change| change.tick).collect();
            assert_eq!(observed, ticks, "{times:?}");
        }
    }

    #[test]
    fn a_sub_femtosecond_time_is_refused_by_name() {
        let error = event_vcd_document(
            "tran",
            &[digital_trace(
                "clk",
                &[(0.0, DigitalValue::zero()), (1.5e-15, DigitalValue::one())],
            )],
            &[],
        )
        .expect_err("half a femtosecond has no exact tick");
        assert_eq!(
            error,
            EventProjectionError::InexactTime {
                node: "clk".to_string(),
                time: 1.5e-15
            }
        );
        assert!(error.to_string().contains("clk"), "{error}");
        assert!(error.to_string().contains("femtoseconds"), "{error}");
    }

    #[test]
    fn nanosecond_times_are_not_mistaken_for_inexact_ones() {
        // `1e-9 * 1e15` is not exactly 1_000_000 in binary64. A residual test
        // against zero would reject a run of ordinary nanosecond edges.
        let points: Vec<(Value, DigitalValue)> = (0..64)
            .map(|index| {
                (
                    f64::from(index) * 1e-9,
                    if index % 2 == 0 {
                        DigitalValue::zero()
                    } else {
                        DigitalValue::one()
                    },
                )
            })
            .collect();
        let document =
            event_vcd_document("tran", &[digital_trace("clk", &points)], &[]).expect("projection");
        assert_eq!(document.timescale.to_string(), "1 ns");
        let signal = document.signals.first().expect("one signal");
        let ticks: Vec<u64> = signal.changes.iter().map(|change| change.tick).collect();
        assert_eq!(ticks, (0..64).collect::<Vec<u64>>());
    }

    #[test]
    fn an_empty_projection_uses_the_finest_scale() {
        let document = event_vcd_document("tran", &[], &[]).expect("projection");
        assert_eq!(document.timescale.to_string(), "1 fs");
        assert!(document.signals.is_empty());

        let only_zero = event_vcd_document(
            "tran",
            &[digital_trace("clk", &[(0.0, DigitalValue::zero())])],
            &[],
        )
        .expect("projection");
        assert_eq!(only_zero.timescale.to_string(), "1 fs");
    }

    #[test]
    fn unwritable_names_and_impossible_times_are_refused() {
        assert_eq!(
            event_vcd_document("two words", &[], &[]).expect_err("scope"),
            EventProjectionError::Scope {
                scope: "two words".to_string()
            }
        );
        assert_eq!(
            event_vcd_document(
                "tran",
                &[digital_trace("clk\tenable", &[(0.0, DigitalValue::zero())])],
                &[]
            )
            .expect_err("node name"),
            EventProjectionError::NodeName {
                node: "clk\tenable".to_string()
            }
        );
        assert_eq!(
            event_vcd_document(
                "tran",
                &[digital_trace("$end", &[(0.0, DigitalValue::zero())])],
                &[]
            )
            .expect_err("node name"),
            EventProjectionError::NodeName {
                node: "$end".to_string()
            }
        );
        assert_eq!(
            event_vcd_document(
                "tran",
                &[digital_trace("clk", &[(-1e-9, DigitalValue::zero())])],
                &[]
            )
            .expect_err("negative time"),
            EventProjectionError::UnrepresentableTime {
                node: "clk".to_string(),
                time: -1e-9
            }
        );
        assert_eq!(
            event_vcd_document(
                "tran",
                &[digital_trace(
                    "clk",
                    &[(2e-9, DigitalValue::zero()), (1e-9, DigitalValue::one())]
                )],
                &[]
            )
            .expect_err("unordered"),
            EventProjectionError::UnorderedTime {
                node: "clk".to_string(),
                time: 1e-9
            }
        );
        assert_eq!(
            event_vcd_document("tran", &[], &[real_trace("out", &[(0.0, f64::NAN)])])
                .expect_err("non-finite"),
            EventProjectionError::NonFiniteValue {
                node: "out".to_string(),
                time: 0.0
            }
        );
    }
}
