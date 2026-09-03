//! XSPICE event traces, which compression carries through untouched.
//!
//! Digital and real event traces are committed events rather than sampled
//! waveforms, so decimation never touches them and their pickled form is the
//! event list itself. Digital states and strengths travel as the stable tags
//! core exposes, never as enum ordinals.

use super::*;

/// One digital event trace: node name and `(time, state, strength)` events.
pub(crate) type CompressedDigitalTracePersistenceState = (String, Vec<(f64, String, String)>);
/// One real event trace: node name and `(time, value)` events.
pub(crate) type CompressedRealTracePersistenceState = (String, Vec<(f64, f64)>);

pub(crate) fn digital_trace_persistence_state(
    trace: &rspice_core::engine::DigitalTrace,
) -> CompressedDigitalTracePersistenceState {
    (
        trace.node_name.clone(),
        trace
            .points
            .iter()
            .map(|point| {
                (
                    point.time,
                    rspice_core::engine::digital_state_tag(point.value.state).to_string(),
                    rspice_core::engine::digital_strength_tag(point.value.strength).to_string(),
                )
            })
            .collect(),
    )
}

pub(crate) fn rebuild_digital_trace(
    state: CompressedDigitalTracePersistenceState,
) -> PyResult<rspice_core::engine::DigitalTrace> {
    let (node_name, points) = state;
    let mut rebuilt = Vec::with_capacity(points.len());
    for (time, state_tag, strength_tag) in points {
        let state = rspice_core::engine::digital_state_from_tag(&state_tag).ok_or_else(|| {
            crate::errors::value_error(format!(
                "unsupported compressed-transient digital state '{state_tag}'"
            ))
        })?;
        let strength =
            rspice_core::engine::digital_strength_from_tag(&strength_tag).ok_or_else(|| {
                crate::errors::value_error(format!(
                    "unsupported compressed-transient digital strength '{strength_tag}'"
                ))
            })?;
        rebuilt.push(rspice_core::engine::DigitalTracePoint {
            time,
            value: rspice_core::xspice::DigitalValue { state, strength },
        });
    }
    Ok(rspice_core::engine::DigitalTrace {
        node_name,
        points: rebuilt,
    })
}

pub(crate) fn real_trace_persistence_state(
    trace: &rspice_core::engine::RealTrace,
) -> CompressedRealTracePersistenceState {
    (
        trace.node_name.clone(),
        trace
            .points
            .iter()
            .map(|point| (point.time, point.value))
            .collect(),
    )
}

pub(crate) fn rebuild_real_trace(
    state: CompressedRealTracePersistenceState,
) -> rspice_core::engine::RealTrace {
    let (node_name, points) = state;
    rspice_core::engine::RealTrace {
        node_name,
        points: points
            .into_iter()
            .map(|(time, value)| rspice_core::engine::RealTracePoint { time, value })
            .collect(),
    }
}
