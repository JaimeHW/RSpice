//! Pickle encoding of a transient result's XSPICE event histories.
//!
//! Digital and real event traces are change histories rather than one sample
//! per accepted point, and their logic states have no natural numeric form, so
//! they need their own stable wire spelling. That spelling is core's own
//! `DigitalStateTag`/`DigitalStrengthTag`, so a pickle and a shared result
//! document name the same states.

/// Pickle schema version of the full transient result's structural contract.
///
/// Version zero carried neither a version tag nor the XSPICE event histories a
/// mixed-signal transient computes. It cannot be migrated honestly: an empty
/// digital-trace list would equally mean "this deck has no event nodes" and
/// "an older binding dropped them", so a version-zero state is rejected rather
/// than restored as an analog-only result.
pub(super) const TRANSIENT_STRUCTURE_STATE_VERSION: usize = 1;

/// One digital event as `(time, state label, strength label)`.
pub(super) type DigitalEventState = (f64, String, String);

/// One XSPICE digital node's committed event history.
pub(super) type DigitalTraceState = (String, Vec<DigitalEventState>);

/// One XSPICE real-valued event node's committed history.
pub(super) type RealTraceState = (String, Vec<(f64, f64)>);

/// Version plus the event histories a full transient result carries.
pub(super) type TransientEventPersistenceState =
    (usize, Vec<DigitalTraceState>, Vec<RealTraceState>);

/// Stable wire spelling of one digital logic state.
///
/// The spellings are core's own `DigitalStateTag`, so the pickle and the
/// shared result document name the same states.
fn digital_state_label(state: rspice_core::xspice::DigitalState) -> &'static str {
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

/// Stable wire spelling of one digital drive strength.
fn digital_strength_label(strength: rspice_core::xspice::DigitalStrength) -> &'static str {
    use rspice_core::execution::result_document::DigitalStrengthTag as Tag;
    match Tag::from(strength) {
        Tag::Undetermined => "undetermined",
        Tag::HighZ => "high_z",
        Tag::Resistive => "resistive",
        Tag::Strong => "strong",
    }
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

/// Persist the XSPICE event histories a transient result carries.
pub(super) fn transient_event_persistence_state(
    digital_traces: &[rspice_core::engine::DigitalTrace],
    real_traces: &[rspice_core::engine::RealTrace],
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
    )
}

/// Rebuild the event histories, rejecting a state this build cannot read.
#[allow(clippy::type_complexity)]
pub(super) fn rebuild_transient_event_traces(
    state: Option<TransientEventPersistenceState>,
) -> Result<
    (
        Vec<rspice_core::engine::DigitalTrace>,
        Vec<rspice_core::engine::RealTrace>,
    ),
    String,
> {
    let Some((version, digital, real)) = state else {
        return Err(
            "legacy transient pickle predates the versioned result contract and carries no XSPICE \
             event histories; rerun the analysis"
                .to_string(),
        );
    };
    if version != TRANSIENT_STRUCTURE_STATE_VERSION {
        return Err(format!(
            "unsupported transient pickle state version {version}; this build reads version {TRANSIENT_STRUCTURE_STATE_VERSION}"
        ));
    }
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
    Ok((digital_traces, real_traces))
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

        let state = transient_event_persistence_state(&digital, &real);
        assert_eq!(state.0, TRANSIENT_STRUCTURE_STATE_VERSION);
        let (restored_digital, restored_real) =
            rebuild_transient_event_traces(Some(state)).unwrap();
        assert_eq!(restored_digital, digital);
        assert_eq!(restored_real, real);
    }

    #[test]
    fn a_legacy_state_is_refused_with_an_actionable_message() {
        let error = rebuild_transient_event_traces(None)
            .expect_err("a state without a version tag cannot be restored");
        assert!(error.contains("legacy transient pickle"), "{error}");
        assert!(error.contains("rerun the analysis"), "{error}");
    }

    #[test]
    fn a_future_state_version_is_refused_by_number() {
        let future = TRANSIENT_STRUCTURE_STATE_VERSION + 1;
        let error = rebuild_transient_event_traces(Some((future, Vec::new(), Vec::new())))
            .expect_err("a newer contract cannot be read by this build");
        assert!(
            error.contains(&format!("state version {future}")),
            "{error}"
        );
    }

    #[test]
    fn an_unknown_logic_label_is_refused_rather_than_defaulted() {
        let error = rebuild_transient_event_traces(Some((
            TRANSIENT_STRUCTURE_STATE_VERSION,
            vec![(
                "clk".to_string(),
                vec![(0.0, "floating".to_string(), "strong".to_string())],
            )],
            Vec::new(),
        )))
        .expect_err("an unknown logic state must not become Unknown");
        assert!(error.contains("'floating'"), "{error}");
    }
}
