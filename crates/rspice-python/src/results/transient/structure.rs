//! Structural validation of a complete transient waveform set.
//!
//! This is the pass a persisted result must survive in both directions: a
//! restored result can never be shorter, wider, or less aligned than the one
//! that was written, and a result that could not be read back is never
//! written in the first place.

use super::*;

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
