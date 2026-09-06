//! Pickle encoding of a transient result's XSPICE event histories and the
//! digital buses declared over them.
//!
//! Digital and real event traces are change histories rather than one sample
//! per accepted point, and their logic states have no natural numeric form, so
//! they need their own stable wire spelling. That spelling is core's own
//! `DigitalStateTag`/`DigitalStrengthTag`/`DigitalBusSourceTag`, so a pickle
//! and a shared result document name the same states and the same declarers.
//!
//! # Versions
//!
//! Version 1 carried the two trace lists. Version 2 carries a bus table beside
//! them, and the two differ in exactly that: a version-1 state is a version-2
//! state with no bus, and is read as one rather than refused, because nothing
//! that could write a version-1 state could declare a bus. The two shapes are
//! told apart by their own field count, so a state of one version claiming the
//! other's number is refused rather than read as either.

use pyo3::FromPyObject;

/// Pickle schema version of the full transient result's structural contract.
///
/// Version zero carried neither a version tag nor the XSPICE event histories a
/// mixed-signal transient computes. It cannot be migrated honestly: an empty
/// digital-trace list would equally mean "this deck has no event nodes" and
/// "an older binding dropped them", so a version-zero state is rejected rather
/// than restored as an analog-only result.
pub(super) const TRANSIENT_STRUCTURE_STATE_VERSION: usize = 2;

/// The version written before the digital bus contract existed.
///
/// Read, not refused: the engine boundary that declares a bus landed after it,
/// so a version-1 state has no bus table because there was no bus to put in
/// one — which is not the same as a bus having been dropped.
pub(super) const TRANSIENT_STRUCTURE_STATE_VERSION_WITHOUT_BUSES: usize = 1;

/// One digital event as `(time, state label, strength label)`.
pub(super) type DigitalEventState = (f64, String, String);

/// One XSPICE digital node's committed event history.
pub(super) type DigitalTraceState = (String, Vec<DigitalEventState>);

/// One XSPICE real-valued event node's committed history.
pub(super) type RealTraceState = (String, Vec<(f64, f64)>);

/// One declared digital bus as `(name, msb, lsb, members, source label)`.
///
/// The members are node names in declaration order, declared MSB first, and
/// the range is carried exactly as declared — neither is normalized, because a
/// descending and an ascending range are different declarations.
pub(super) type DigitalBusState = (String, i64, i64, Vec<String>, String);

/// Version plus the event histories and bus table a transient result carries.
pub(super) type TransientEventPersistenceState = (
    usize,
    Vec<DigitalTraceState>,
    Vec<RealTraceState>,
    Vec<DigitalBusState>,
);

/// The same state as written by a build that had no bus contract.
pub(super) type TransientEventPersistenceStateWithoutBuses =
    (usize, Vec<DigitalTraceState>, Vec<RealTraceState>);

/// A pickled event state at whichever version wrote it.
///
/// The variants are tried in order and told apart by field count, which is
/// what lets one `_unpickle` signature read both without a version tag it has
/// not read yet deciding the shape it is about to extract.
#[derive(Debug, Clone, FromPyObject)]
pub(super) enum VersionedTransientEventState {
    /// Four fields: the current contract, carrying a bus table.
    #[pyo3(transparent)]
    Current(TransientEventPersistenceState),
    /// Three fields: written before the bus contract existed.
    #[pyo3(transparent)]
    WithoutBuses(TransientEventPersistenceStateWithoutBuses),
}

/// Stable wire spelling of one digital logic state.
///
/// The spellings are core's own `DigitalStateTag`, so the pickle and the
/// shared result document name the same states. The typed digital accessor in
/// `transient::events` spells a row's `state` with this too, so a history
/// reads the same however it reached the caller.
pub(super) fn digital_state_label(state: rspice_core::xspice::DigitalState) -> &'static str {
    use rspice_core::execution::result_document::DigitalStateTag as Tag;
    match Tag::from(state) {
        Tag::Zero => "zero",
        Tag::One => "one",
        Tag::Unknown => "unknown",
        Tag::ZeroResistive => "zero_resistive",
        Tag::OneResistive => "one_resistive",
        Tag::UnknownResistive => "unknown_resistive",
        Tag::ZeroHighZ => "zero_high_z",
        Tag::OneHighZ => "one_high_z",
        Tag::UnknownHighZ => "unknown_high_z",
        Tag::HighZ => "high_z",
    }
}

fn digital_state_from_label(label: &str) -> Result<rspice_core::xspice::DigitalState, String> {
    use rspice_core::execution::result_document::DigitalStateTag as Tag;
    let tag = match label {
        "zero" => Tag::Zero,
        "one" => Tag::One,
        "unknown" => Tag::Unknown,
        "zero_resistive" => Tag::ZeroResistive,
        "one_resistive" => Tag::OneResistive,
        "unknown_resistive" => Tag::UnknownResistive,
        "zero_high_z" => Tag::ZeroHighZ,
        "one_high_z" => Tag::OneHighZ,
        "unknown_high_z" => Tag::UnknownHighZ,
        "high_z" => Tag::HighZ,
        other => {
            return Err(format!(
                "unknown digital logic state '{other}' in pickled transient state"
            ));
        }
    };
    Ok(tag.into())
}

/// Stable wire spelling of one digital drive strength, shared with the typed
/// digital accessor for the same reason as the state label above.
pub(super) fn digital_strength_label(
    strength: rspice_core::xspice::DigitalStrength,
) -> &'static str {
    use rspice_core::execution::result_document::DigitalStrengthTag as Tag;
    match Tag::from(strength) {
        Tag::Undetermined => "undetermined",
        Tag::HighZ => "high_z",
        Tag::Resistive => "resistive",
        Tag::Strong => "strong",
    }
}

/// Stable wire spelling of who declared a digital bus.
///
/// The spellings are core's own `DigitalBusSourceTag`, so a bus named by the
/// typed accessor, by a pickle and by `document()` names the same declarer.
pub(super) fn digital_bus_source_label(
    source: rspice_core::engine::DigitalBusSource,
) -> &'static str {
    use rspice_core::execution::result_document::DigitalBusSourceTag as Tag;
    match Tag::from(source) {
        Tag::Engine => "engine",
        Tag::Schematic => "schematic",
        Tag::Import => "import",
    }
}

/// The inverse of [`digital_bus_source_label`], shared with the compressed
/// container's own codec so the two pickles read one spelling.
pub(super) fn digital_bus_source_from_label(
    label: &str,
) -> Result<rspice_core::engine::DigitalBusSource, String> {
    use rspice_core::execution::result_document::DigitalBusSourceTag as Tag;
    let tag = match label {
        "engine" => Tag::Engine,
        "schematic" => Tag::Schematic,
        "import" => Tag::Import,
        other => {
            return Err(format!(
                "unknown digital bus source '{other}' in pickled transient state"
            ));
        }
    };
    Ok(tag.into())
}

fn digital_strength_from_label(
    label: &str,
) -> Result<rspice_core::xspice::DigitalStrength, String> {
    use rspice_core::execution::result_document::DigitalStrengthTag as Tag;
    let tag = match label {
        "undetermined" => Tag::Undetermined,
        "high_z" => Tag::HighZ,
        "resistive" => Tag::Resistive,
        "strong" => Tag::Strong,
        other => {
            return Err(format!(
                "unknown digital drive strength '{other}' in pickled transient state"
            ));
        }
    };
    Ok(tag.into())
}

/// Persist the XSPICE event histories and bus table a transient result carries.
pub(super) fn transient_event_persistence_state(
    digital_traces: &[rspice_core::engine::DigitalTrace],
    real_traces: &[rspice_core::engine::RealTrace],
    digital_buses: &[rspice_core::engine::DigitalBusDeclaration],
) -> TransientEventPersistenceState {
    (
        TRANSIENT_STRUCTURE_STATE_VERSION,
        digital_traces
            .iter()
            .map(|trace| {
                (
                    trace.node_name.clone(),
                    trace
                        .points
                        .iter()
                        .map(|point| {
                            (
                                point.time,
                                digital_state_label(point.value.state).to_string(),
                                digital_strength_label(point.value.strength).to_string(),
                            )
                        })
                        .collect(),
                )
            })
            .collect(),
        real_traces
            .iter()
            .map(|trace| {
                (
                    trace.node_name.clone(),
                    trace
                        .points
                        .iter()
                        .map(|point| (point.time, point.value))
                        .collect(),
                )
            })
            .collect(),
        digital_buses
            .iter()
            .map(|bus| {
                (
                    bus.name.clone(),
                    bus.msb,
                    bus.lsb,
                    bus.members.clone(),
                    digital_bus_source_label(bus.source).to_string(),
                )
            })
            .collect(),
    )
}

/// A state whose version number does not match the shape it arrived in.
fn unsupported_state_version(version: usize, fields: usize) -> String {
    format!(
        "unsupported transient pickle state version {version} in a {fields}-field state; this \
         build reads version {TRANSIENT_STRUCTURE_STATE_VERSION_WITHOUT_BUSES} (three fields, no \
         bus table) and version {TRANSIENT_STRUCTURE_STATE_VERSION} (four fields)"
    )
}

/// Rebuild the event histories and the bus table, rejecting a state this build
/// cannot read.
///
/// The bus table is judged against the traces it was restored beside, through
/// core's own [`rspice_core::engine::validate_digital_bus_table`], so a pickle
/// that declares a bus over a conductor it did not carry is refused rather
/// than restored as a declaration nothing can answer.
#[allow(clippy::type_complexity)]
pub(super) fn rebuild_transient_event_traces(
    state: Option<VersionedTransientEventState>,
) -> Result<
    (
        Vec<rspice_core::engine::DigitalTrace>,
        Vec<rspice_core::engine::DigitalBusDeclaration>,
        Vec<rspice_core::engine::RealTrace>,
    ),
    String,
> {
    let Some(state) = state else {
        return Err(
            "legacy transient pickle predates the versioned result contract and carries no XSPICE \
             event histories; rerun the analysis"
                .to_string(),
        );
    };
    let (digital, real, buses) = match state {
        VersionedTransientEventState::Current((version, digital, real, buses)) => {
            if version != TRANSIENT_STRUCTURE_STATE_VERSION {
                return Err(unsupported_state_version(version, 4));
            }
            (digital, real, buses)
        }
        VersionedTransientEventState::WithoutBuses((version, digital, real)) => {
            if version != TRANSIENT_STRUCTURE_STATE_VERSION_WITHOUT_BUSES {
                return Err(unsupported_state_version(version, 3));
            }
            // Nothing that wrote this state could declare a bus, so an empty
            // table is what it says rather than what it lost.
            (digital, real, Vec::new())
        }
    };
    let digital_traces = digital
        .into_iter()
        .map(|(node_name, points)| {
            Ok(rspice_core::engine::DigitalTrace {
                node_name,
                points: points
                    .into_iter()
                    .map(|(time, state, strength)| {
                        Ok(rspice_core::engine::DigitalTracePoint {
                            time,
                            value: rspice_core::xspice::DigitalValue {
                                state: digital_state_from_label(&state)?,
                                strength: digital_strength_from_label(&strength)?,
                            },
                        })
                    })
                    .collect::<Result<Vec<_>, String>>()?,
            })
        })
        .collect::<Result<Vec<_>, String>>()?;
    let real_traces = real
        .into_iter()
        .map(|(node_name, points)| rspice_core::engine::RealTrace {
            node_name,
            points: points
                .into_iter()
                .map(|(time, value)| rspice_core::engine::RealTracePoint { time, value })
                .collect(),
        })
        .collect();
    let digital_buses = buses
        .into_iter()
        .map(|(name, msb, lsb, members, source)| {
            Ok(rspice_core::engine::DigitalBusDeclaration {
                name,
                msb,
                lsb,
                members,
                source: digital_bus_source_from_label(&source)?,
            })
        })
        .collect::<Result<Vec<_>, String>>()?;
    rspice_core::engine::validate_digital_bus_table(
        &digital_buses,
        digital_traces.iter().map(|trace| trace.node_name.as_str()),
    )
    .map_err(|error| {
        format!("pickled transient state declares a digital bus it cannot carry: {error}")
    })?;
    Ok((digital_traces, digital_buses, real_traces))
}
#[cfg(test)]
mod event_pickle_tests {
    use super::*;
    use rspice_core::engine::{DigitalTrace, DigitalTracePoint, RealTrace, RealTracePoint};
    use rspice_core::xspice::{DigitalState, DigitalStrength, DigitalValue};

    const EVERY_STATE: [DigitalState; 10] = [
        DigitalState::Zero,
        DigitalState::One,
        DigitalState::Unknown,
        DigitalState::ZeroR,
        DigitalState::OneR,
        DigitalState::UnknownR,
        DigitalState::ZeroZ,
        DigitalState::OneZ,
        DigitalState::UnknownZ,
        DigitalState::HighZ,
    ];

    const EVERY_SOURCE: [rspice_core::engine::DigitalBusSource; 3] = [
        rspice_core::engine::DigitalBusSource::Engine,
        rspice_core::engine::DigitalBusSource::Schematic,
        rspice_core::engine::DigitalBusSource::Import,
    ];

    const EVERY_STRENGTH: [DigitalStrength; 4] = [
        DigitalStrength::Undetermined,
        DigitalStrength::HighZ,
        DigitalStrength::Resistive,
        DigitalStrength::Strong,
    ];

    #[test]
    fn every_digital_state_and_strength_round_trips_through_its_label() {
        let mut labels = std::collections::BTreeSet::new();
        for state in EVERY_STATE {
            let label = digital_state_label(state);
            assert!(labels.insert(label), "duplicate state label '{label}'");
            assert_eq!(digital_state_from_label(label).unwrap(), state);
        }
        let mut strength_labels = std::collections::BTreeSet::new();
        for strength in EVERY_STRENGTH {
            let label = digital_strength_label(strength);
            assert!(
                strength_labels.insert(label),
                "duplicate strength label '{label}'"
            );
            assert_eq!(digital_strength_from_label(label).unwrap(), strength);
        }
    }

    /// Every declarer has a label of its own, spelled as the shared document's
    /// `DigitalBusSourceTag` spells it.
    #[test]
    fn every_digital_bus_source_has_its_own_label() {
        let mut labels = std::collections::BTreeSet::new();
        for source in EVERY_SOURCE {
            let label = digital_bus_source_label(source);
            assert!(labels.insert(label), "duplicate bus source label '{label}'");
        }
        assert_eq!(
            labels.into_iter().collect::<Vec<_>>(),
            ["engine", "import", "schematic"]
        );
    }

    #[test]
    fn event_histories_survive_the_persistence_round_trip() {
        let digital = vec![DigitalTrace {
            node_name: "clk".to_string(),
            points: vec![
                DigitalTracePoint {
                    time: 0.0,
                    value: DigitalValue {
                        state: DigitalState::ZeroZ,
                        strength: DigitalStrength::HighZ,
                    },
                },
                DigitalTracePoint {
                    time: 1.0e-9,
                    value: DigitalValue {
                        state: DigitalState::One,
                        strength: DigitalStrength::Strong,
                    },
                },
            ],
        }];
        let real = vec![RealTrace {
            node_name: "ctrl".to_string(),
            points: vec![RealTracePoint {
                time: 2.0e-9,
                value: -0.5,
            }],
        }];

        let buses = vec![
            rspice_core::engine::DigitalBusDeclaration::new(
                "x1.count",
                1,
                0,
                vec!["clk".to_string(), "clk2".to_string()],
                rspice_core::engine::DigitalBusSource::Engine,
            )
            .expect("the fixture declaration is well formed"),
        ];
        let mut digital = digital;
        digital.push(DigitalTrace {
            node_name: "clk2".to_string(),
            points: vec![DigitalTracePoint {
                time: 0.0,
                value: DigitalValue {
                    state: DigitalState::Zero,
                    strength: DigitalStrength::Strong,
                },
            }],
        });

        let state = transient_event_persistence_state(&digital, &real, &buses);
        assert_eq!(state.0, TRANSIENT_STRUCTURE_STATE_VERSION);
        let (restored_digital, restored_buses, restored_real) =
            rebuild_transient_event_traces(Some(VersionedTransientEventState::Current(state)))
                .unwrap();
        assert_eq!(restored_digital, digital);
        assert_eq!(restored_real, real);
        assert_eq!(restored_buses, buses);
    }

    #[test]
    fn a_legacy_state_is_refused_with_an_actionable_message() {
        let error = rebuild_transient_event_traces(None)
            .expect_err("a state without a version tag cannot be restored");
        assert!(error.contains("legacy transient pickle"), "{error}");
        assert!(error.contains("rerun the analysis"), "{error}");
    }

    /// A version-1 state is read, not refused: nothing that wrote one could
    /// declare a bus, so an empty table is what it says.
    #[test]
    fn a_state_written_before_the_bus_contract_restores_with_no_bus() {
        let (digital, buses, real) =
            rebuild_transient_event_traces(Some(VersionedTransientEventState::WithoutBuses((
                TRANSIENT_STRUCTURE_STATE_VERSION_WITHOUT_BUSES,
                vec![(
                    "clk".to_string(),
                    vec![(0.0, "one".to_string(), "strong".to_string())],
                )],
                vec![("ctrl".to_string(), vec![(1.0e-9, 0.25)])],
            ))))
            .expect("a version-1 state is a version-2 state with no bus");
        assert_eq!(digital.len(), 1);
        assert_eq!(real.len(), 1);
        assert!(buses.is_empty());
    }

    #[test]
    fn a_future_state_version_is_refused_by_number() {
        let future = TRANSIENT_STRUCTURE_STATE_VERSION + 1;
        let error = rebuild_transient_event_traces(Some(VersionedTransientEventState::Current((
            future,
            Vec::new(),
            Vec::new(),
            Vec::new(),
        ))))
        .expect_err("a newer contract cannot be read by this build");
        assert!(
            error.contains(&format!("state version {future}")),
            "{error}"
        );
    }

    /// A state's version has to agree with the shape it arrived in, so a
    /// version-1 tag on a four-field state is refused rather than read as
    /// either version.
    #[test]
    fn a_version_that_contradicts_the_state_shape_is_refused() {
        let error = rebuild_transient_event_traces(Some(VersionedTransientEventState::Current((
            TRANSIENT_STRUCTURE_STATE_VERSION_WITHOUT_BUSES,
            Vec::new(),
            Vec::new(),
            Vec::new(),
        ))))
        .expect_err("version 1 has three fields, not four");
        assert!(error.contains("4-field state"), "{error}");

        let error =
            rebuild_transient_event_traces(Some(VersionedTransientEventState::WithoutBuses((
                TRANSIENT_STRUCTURE_STATE_VERSION,
                Vec::new(),
                Vec::new(),
            ))))
            .expect_err("version 2 has four fields, not three");
        assert!(error.contains("3-field state"), "{error}");
    }

    #[test]
    fn an_unknown_logic_label_is_refused_rather_than_defaulted() {
        let error = rebuild_transient_event_traces(Some(VersionedTransientEventState::Current((
            TRANSIENT_STRUCTURE_STATE_VERSION,
            vec![(
                "clk".to_string(),
                vec![(0.0, "floating".to_string(), "strong".to_string())],
            )],
            Vec::new(),
            Vec::new(),
        ))))
        .expect_err("an unknown logic state must not become Unknown");
        assert!(error.contains("'floating'"), "{error}");
    }

    #[test]
    fn an_unknown_bus_source_is_refused_rather_than_defaulted() {
        let error = rebuild_transient_event_traces(Some(VersionedTransientEventState::Current((
            TRANSIENT_STRUCTURE_STATE_VERSION,
            vec![
                ("q1".to_string(), Vec::new()),
                ("q0".to_string(), Vec::new()),
            ],
            Vec::new(),
            vec![(
                "q".to_string(),
                1,
                0,
                vec!["q1".to_string(), "q0".to_string()],
                "guessed".to_string(),
            )],
        ))))
        .expect_err("an unknown declarer must not become one of the three");
        assert!(error.contains("'guessed'"), "{error}");
    }

    /// A pickled bus is judged against the traces restored beside it, so a
    /// declaration nothing can answer is refused rather than restored.
    #[test]
    fn a_pickled_bus_whose_member_has_no_trace_is_refused() {
        let error = rebuild_transient_event_traces(Some(VersionedTransientEventState::Current((
            TRANSIENT_STRUCTURE_STATE_VERSION,
            vec![("q1".to_string(), Vec::new())],
            Vec::new(),
            vec![(
                "q".to_string(),
                1,
                0,
                vec!["q1".to_string(), "q0".to_string()],
                "engine".to_string(),
            )],
        ))))
        .expect_err("a member with no trace cannot be restored");
        assert!(error.contains("cannot carry"), "{error}");
        assert!(error.contains("q0"), "{error}");
    }

    /// A range that does not describe the member list is refused by core's own
    /// declaration check, reached through the table check.
    #[test]
    fn a_pickled_bus_whose_range_contradicts_its_members_is_refused() {
        let error = rebuild_transient_event_traces(Some(VersionedTransientEventState::Current((
            TRANSIENT_STRUCTURE_STATE_VERSION,
            vec![
                ("q1".to_string(), Vec::new()),
                ("q0".to_string(), Vec::new()),
            ],
            Vec::new(),
            vec![(
                "q".to_string(),
                7,
                0,
                vec!["q1".to_string(), "q0".to_string()],
                "engine".to_string(),
            )],
        ))))
        .expect_err("a [7:0] range needs eight members");
        assert!(error.contains("cannot carry"), "{error}");
    }
}
