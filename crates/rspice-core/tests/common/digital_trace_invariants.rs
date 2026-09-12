//! One invariant every suite that runs a mixed deck states the same way.
//!
//! A digital net holds one value at one instant. The trace channel is a change
//! list over accepted timepoints, so a node with two points at the same time
//! has published two committed values for one instant — a zero-width glitch,
//! which is neither a waveform a viewer can draw nor a transition a VCD reader
//! can spell. The engine refuses to record one; this is the deck-level
//! statement of the same rule, so a suite gets it on every deck its shared
//! runner runs without restating the reasoning per case.
//!
//! Included with `#[path]` rather than through `common/mod.rs`, because the
//! suites that want it are not the suites that want the model-path helpers.

use rspice_core::engine::TransientResult;

/// Assert that no node in `result` records two trace points at one instant.
///
/// `context` names the deck that produced the result: a suite calls this from
/// its shared runner, so the failure has to say which of that suite's many runs
/// it came from.
pub fn assert_one_digital_value_per_instant(result: &TransientResult, context: &str) {
    for trace in &result.digital_traces {
        for pair in trace.points.windows(2) {
            assert!(
                pair[0].time != pair[1].time,
                "{context}: digital node {} records two values at t={:e}: {:?} then {:?}",
                trace.node_name,
                pair[0].time,
                pair[0].value,
                pair[1].value
            );
        }
    }
}
