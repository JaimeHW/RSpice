//! The persisted form of a transient result, and the structural pass it must
//! survive in both directions.
//!
//! A restored result can never be shorter, wider, or less aligned than the one
//! that was written, and a result that could not be read back is never written
//! in the first place. Both directions of that contract live here: the state
//! `__reduce__` publishes, the reconstruction `_unpickle` performs, and
//! [`validate_transient_state`], the pass each of them runs.

use super::*;

/// Everything a pickled transient result carries, in `_unpickle` order.
///
/// Naming the tuple keeps the shape in one place: `__reduce__` publishes it and
/// `_unpickle` consumes it, and a field added to one that is missing from the
/// other would not compile.
pub(super) type TransientPersistenceState = (
    Vec<f64>,
    Vec<f64>,
    Vec<Vec<f64>>,
    Vec<Vec<f64>>,
    usize,
    (Vec<String>, Vec<String>),
    Vec<(String, String, Vec<f64>)>,
    Vec<(String, Vec<f64>)>,
    TransientFftPersistenceState,
    TransientEventPersistenceState,
);

/// Rebuild a transient result from the state `_unpickle` was handed.
///
/// Nothing here is zero-filled: a state that does not describe a complete,
/// aligned result is rejected rather than repaired, because a result that
/// reads back as something else is worse than one that does not read back.
#[allow(clippy::too_many_arguments)]
pub(super) fn restore_transient_result(
    time: Vec<f64>,
    step_sizes: Vec<f64>,
    voltages: Vec<Vec<f64>>,
    branch_currents: Vec<Vec<f64>>,
    num_nodes: usize,
    names: (Vec<String>, Vec<String>),
    device_op_traces: Vec<(String, String, Vec<f64>)>,
    store_traces: Vec<(String, Vec<f64>)>,
    fft_state: Option<TransientFftPersistenceState>,
    event_state: Option<VersionedTransientEventState>,
) -> PyResult<TransientResult> {
    let (digital_traces, digital_buses, real_traces) =
        rebuild_transient_event_traces(event_state).map_err(crate::errors::value_error)?;
    let (node_names, branch_names) = names;
    let restored = TransientResult {
        time,
        step_sizes,
        voltages,
        branch_currents,
        num_nodes,
        node_names,
        branch_names,
        digital_traces,
        digital_buses,
        real_traces,
        device_op_traces: device_op_traces
            .into_iter()
            .map(
                |(device_name, parameter, values)| rspice_core::engine::TransientDeviceOpTrace {
                    device_name,
                    parameter,
                    values,
                },
            )
            .collect(),
        store_traces: store_traces
            .into_iter()
            .map(|(name, values)| rspice_core::engine::TransientStoreTrace { name, values })
            .collect(),
        fft_results: rebuild_transient_fft_results(fft_state)?,
    };
    validate_transient_state(&restored).map_err(crate::errors::value_error)?;
    Ok(restored)
}

/// Publish the state `_unpickle` will be handed back.
///
/// The structural pass runs before anything is published: a pickle is
/// evidence, and a file that cannot be loaded back is worse than a failed call.
pub(super) fn transient_persistence_state(
    result: &TransientResult,
) -> PyResult<TransientPersistenceState> {
    validate_transient_state(result).map_err(crate::errors::value_error)?;
    Ok((
        result.time.clone(),
        result.step_sizes.clone(),
        result.voltages.clone(),
        result.branch_currents.clone(),
        result.num_nodes,
        (result.node_names.clone(), result.branch_names.clone()),
        result
            .device_op_traces
            .iter()
            .map(|trace| {
                (
                    trace.device_name.clone(),
                    trace.parameter.clone(),
                    trace.values.clone(),
                )
            })
            .collect(),
        result
            .store_traces
            .iter()
            .map(|trace| (trace.name.clone(), trace.values.clone()))
            .collect(),
        transient_fft_persistence_state(&result.fft_results)?,
        transient_event_persistence_state(
            &result.digital_traces,
            &result.real_traces,
            &result.digital_buses,
        ),
    ))
}

/// Prove a transient result is a complete, internally aligned waveform set.
///
/// Every check names the channel it failed on. The point of the pass is that a
/// restored result can never be shorter, wider, or less aligned than the one
/// that was persisted: a caller reading `voltage_waveform("out")` must get the
/// samples the solver produced or an error, never a truncated array.
pub(crate) fn validate_transient_state(result: &TransientResult) -> Result<(), String> {
    let points = result.time.len();
    if result.step_sizes.len() != points {
        return Err(format!(
            "transient result has {} step sizes for {points} time points",
            result.step_sizes.len()
        ));
    }
    if result.time.iter().any(|time| !time.is_finite()) {
        return Err("transient result time points must all be finite".to_string());
    }
    if result
        .time
        .windows(2)
        .any(|window| matches!(window, [previous, next] if next <= previous))
    {
        return Err("transient result time points must be strictly increasing".to_string());
    }
    if result.voltages.len() != result.num_nodes || result.node_names.len() != result.num_nodes {
        return Err(format!(
            "transient result declares {} nodes but has {} voltage channels and {} node names",
            result.num_nodes,
            result.voltages.len(),
            result.node_names.len()
        ));
    }
    if result.branch_currents.len() != result.branch_names.len() {
        return Err(format!(
            "transient result has {} branch-current channels but {} branch names",
            result.branch_currents.len(),
            result.branch_names.len()
        ));
    }
    // A deliberately unretained channel is empty; a retained one is aligned
    // with the time axis. Anything between the two is a truncated waveform.
    for (name, series) in result.node_names.iter().zip(&result.voltages) {
        if !series.is_empty() && series.len() != points {
            return Err(format!(
                "transient voltage waveform V({name}) has {} samples but time has {points}",
                series.len()
            ));
        }
    }
    for (name, series) in result.branch_names.iter().zip(&result.branch_currents) {
        if !series.is_empty() && series.len() != points {
            return Err(format!(
                "transient branch waveform I({name}) has {} samples but time has {points}",
                series.len()
            ));
        }
    }
    for trace in &result.device_op_traces {
        if trace.values.len() != points {
            return Err(format!(
                "transient device operating-point trace @{}[{}] has {} samples but time has {points}",
                trace.device_name,
                trace.parameter,
                trace.values.len()
            ));
        }
    }
    for trace in &result.store_traces {
        if trace.values.len() != points {
            return Err(format!(
                "transient store trace '{}' has {} samples but time has {points}",
                trace.name,
                trace.values.len()
            ));
        }
    }
    // Event traces record changes rather than one sample per accepted point,
    // so they are checked for monotone, in-window event times instead.
    let window = result.time.first().zip(result.time.last());
    for trace in &result.digital_traces {
        let times = trace.points.iter().map(|point| point.time);
        validate_event_times("digital", &trace.node_name, times, window)?;
    }
    for trace in &result.real_traces {
        let times = trace.points.iter().map(|point| point.time);
        validate_event_times("real", &trace.node_name, times, window)?;
    }
    Ok(())
}

fn validate_event_times(
    kind: &str,
    node: &str,
    times: impl Iterator<Item = f64>,
    window: Option<(&f64, &f64)>,
) -> Result<(), String> {
    let mut previous: Option<f64> = None;
    for time in times {
        if !time.is_finite() {
            return Err(format!(
                "transient {kind} event trace '{node}' has a non-finite event time"
            ));
        }
        if previous.is_some_and(|previous| time < previous) {
            return Err(format!(
                "transient {kind} event trace '{node}' has out-of-order event times"
            ));
        }
        if let Some((start, stop)) = window
            && (time < *start || time > *stop)
        {
            return Err(format!(
                "transient {kind} event trace '{node}' has an event at {time:.16e} s outside the \
                 result window [{start:.16e}, {stop:.16e}] s"
            ));
        }
        previous = Some(time);
    }
    Ok(())
}

/// Discard every sample before a requested SPICE `TSTART` output boundary.
///
/// The solver still integrates from zero so dynamic state at `TSTART` is
/// correct; only the published data is clipped. Every time-aligned vector is
/// proved to match the time axis before a single one is mutated, so a
/// malformed core result becomes a typed error rather than a set of Python
/// arrays that are silently misaligned with each other.
///
/// Event traces record changes rather than one value per accepted analog
/// point, so the state in force at `TSTART` is carried forward to the boundary
/// instead of being dropped along with the event that established it.
pub(crate) fn clip_transient_to_start(
    result: &mut TransientResult,
    start_time: f64,
) -> Result<(), String> {
    if start_time <= 0.0 {
        return Ok(());
    }

    let original_len = result.time.len();
    let start_index = result.time.partition_point(|time| *time < start_time);
    if start_index >= original_len {
        return Err(format!(
            "transient result contains no sample at or after requested start_time {start_time}"
        ));
    }

    for (kind, series) in result
        .voltages
        .iter()
        .map(|series| ("voltage", series))
        .chain(
            result
                .branch_currents
                .iter()
                .map(|series| ("branch-current", series)),
        )
        .chain(
            result
                .device_op_traces
                .iter()
                .map(|trace| ("device operating-point", &trace.values)),
        )
    {
        if series.len() != original_len {
            return Err(format!(
                "malformed transient result: {kind} series has {} samples but time has \
                 {original_len}",
                series.len()
            ));
        }
    }

    result.time.drain(..start_index);
    for series in &mut result.voltages {
        series.drain(..start_index);
    }
    for series in &mut result.branch_currents {
        series.drain(..start_index);
    }
    for trace in &mut result.device_op_traces {
        trace.values.drain(..start_index);
    }

    for trace in &mut result.digital_traces {
        let prior = trace
            .points
            .iter()
            .rev()
            .find(|point| point.time < start_time)
            .copied();
        trace.points.retain(|point| point.time >= start_time);
        if trace
            .points
            .first()
            .is_none_or(|point| point.time > start_time)
            && let Some(mut point) = prior
        {
            point.time = start_time;
            trace.points.insert(0, point);
        }
    }
    for trace in &mut result.real_traces {
        let prior = trace
            .points
            .iter()
            .rev()
            .find(|point| point.time < start_time)
            .copied();
        trace.points.retain(|point| point.time >= start_time);
        if trace
            .points
            .first()
            .is_none_or(|point| point.time > start_time)
            && let Some(mut point) = prior
        {
            point.time = start_time;
            trace.points.insert(0, point);
        }
    }
    Ok(())
}

#[cfg(test)]
mod structural_tests {
    use super::*;
    use rspice_core::engine::{DigitalTrace, DigitalTracePoint};
    use rspice_core::xspice::{DigitalState, DigitalStrength, DigitalValue};

    fn two_point_result() -> TransientResult {
        TransientResult {
            time: vec![0.0, 1.0e-9],
            step_sizes: vec![0.0, 1.0e-9],
            voltages: vec![vec![0.0, 1.0]],
            branch_currents: vec![vec![0.0, -1.0e-3]],
            num_nodes: 1,
            node_names: vec!["out".to_string()],
            branch_names: vec!["V1".to_string()],
            digital_traces: Vec::new(),
            digital_buses: Vec::new(),
            real_traces: Vec::new(),
            device_op_traces: Vec::new(),
            store_traces: Vec::new(),
            fft_results: Vec::new(),
        }
    }

    #[test]
    fn a_complete_aligned_result_validates() {
        assert_eq!(validate_transient_state(&two_point_result()), Ok(()));
    }

    #[test]
    fn an_unretained_channel_stays_legal_but_a_truncated_one_does_not() {
        let mut unretained = two_point_result();
        unretained.voltages[0].clear();
        assert_eq!(validate_transient_state(&unretained), Ok(()));

        let mut truncated = two_point_result();
        truncated.voltages[0].pop();
        let message = validate_transient_state(&truncated)
            .expect_err("a half-length waveform is not a result");
        assert!(message.contains("V(out) has 1 samples"), "{message}");
    }

    #[test]
    fn a_channel_count_that_contradicts_the_node_count_is_refused() {
        let mut result = two_point_result();
        result.num_nodes = 2;
        let message = validate_transient_state(&result)
            .expect_err("a declared node without a channel is not a result");
        assert!(message.contains("declares 2 nodes"), "{message}");
    }

    #[test]
    fn a_non_monotone_time_axis_is_refused() {
        let mut result = two_point_result();
        result.time = vec![1.0e-9, 0.0];
        let message = validate_transient_state(&result).expect_err("time must advance");
        assert!(message.contains("strictly increasing"), "{message}");
    }

    #[test]
    fn an_event_outside_the_result_window_is_refused() {
        let mut result = two_point_result();
        result.digital_traces = vec![DigitalTrace {
            node_name: "clk".to_string(),
            points: vec![DigitalTracePoint {
                time: 5.0e-9,
                value: DigitalValue {
                    state: DigitalState::One,
                    strength: DigitalStrength::Strong,
                },
            }],
        }];
        let message = validate_transient_state(&result)
            .expect_err("an event after the last accepted sample is not in this result");
        assert!(message.contains("outside the result window"), "{message}");
    }

    #[test]
    fn a_store_trace_that_does_not_cover_the_run_is_refused() {
        let mut result = two_point_result();
        result.store_traces = vec![rspice_core::engine::TransientStoreTrace {
            name: "R1:power".to_string(),
            values: vec![1.0],
        }];
        let message = validate_transient_state(&result)
            .expect_err("a store trace is sampled at every accepted point");
        assert!(message.contains("'R1:power' has 1 samples"), "{message}");
    }
}
