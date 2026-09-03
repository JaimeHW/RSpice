//! The versioned round-trip codec for a compressed transient result.
//!
//! A compressed result pickles as a plain tuple of primitives rather than
//! through PyO3's derive, because the core types it wraps are not themselves
//! Python objects. The tuple is versioned, and an older state that could not
//! carry the whole analog inventory is refused rather than restored with
//! channels, event traces, identity or post-results silently missing.
//!
//! One submodule per projected concern: [`channel`] the descriptor-keyed
//! waveform channels and their per-sample validity, [`traces`] the XSPICE
//! event traces, [`identity`] the parent-run identity, [`post`] the `.FOUR`
//! and `.MEASURE` products computed before decimation, and [`report`] the
//! compression error certificate.

use super::*;

mod channel;
mod identity;
mod post;
mod report;
mod traces;

use channel::*;
use identity::*;
use post::*;
use traces::*;

pub(crate) use report::{
    CompressionReportPersistenceState, compression_report_persistence_state,
    rebuild_compression_report,
};

pub(crate) const COMPRESSED_TRANSIENT_ANALOG_STATE_VERSION: usize = 3;

pub(crate) type CompressedTransientAnalogState = (
    usize,
    Vec<f64>,
    Vec<CompressedChannelPersistenceState>,
    Vec<CompressedDigitalTracePersistenceState>,
    Vec<CompressedRealTracePersistenceState>,
    CompressedIdentityPersistenceState,
    Vec<CompressedFourierPersistenceState>,
    Vec<CompressedMeasurementPersistenceState>,
);

/// Project the lossless analog inventory of a compressed transient result.
pub(crate) fn compressed_transient_analog_state(
    result: &rspice_core::engine::TransientResultCompressed,
) -> CompressedTransientAnalogState {
    (
        COMPRESSED_TRANSIENT_ANALOG_STATE_VERSION,
        result.step_sizes.clone(),
        result
            .channels
            .iter()
            .map(channel_persistence_state)
            .collect(),
        result
            .digital_traces
            .iter()
            .map(digital_trace_persistence_state)
            .collect(),
        result
            .real_traces
            .iter()
            .map(real_trace_persistence_state)
            .collect(),
        identity_persistence_state(&result.identity),
        result
            .post_results
            .fourier
            .iter()
            .map(fourier_persistence_state)
            .collect(),
        result
            .post_results
            .measurements
            .iter()
            .map(measurement_persistence_state)
            .collect(),
    )
}

/// Rebuild a validated compressed transient result from its pickled state.
///
/// Every refusal here names what the old state could not prove, because a
/// compressed result whose channels, event traces or error certificate cannot
/// be reconstructed is not a smaller result: it is a different one.
pub(crate) fn rebuild_compressed_transient(
    time: Vec<f64>,
    compression_ratio: f64,
    input_points: usize,
    fft_state: Option<TransientFftPersistenceState>,
    analog_state: Option<CompressedTransientAnalogState>,
    compression_state: Option<CompressionReportPersistenceState>,
) -> PyResult<rspice_core::engine::TransientResultCompressed> {
    let Some((
        version,
        step_sizes,
        channels,
        digital_traces,
        real_traces,
        identity,
        fourier,
        measurements,
    )) = analog_state
    else {
        return Err(crate::errors::value_error(
            "legacy compressed-transient pickle predates lossless analog inventory persistence; rerun the analysis",
        ));
    };
    if version < COMPRESSED_TRANSIENT_ANALOG_STATE_VERSION {
        return Err(crate::errors::value_error(format!(
            "compressed-transient analog pickle state version {version} predates the descriptor-indexed channel container with per-sample validity, event traces, parent identity, and post-results; rerun the analysis"
        )));
    }
    if version != COMPRESSED_TRANSIENT_ANALOG_STATE_VERSION {
        return Err(crate::errors::value_error(format!(
            "unsupported compressed-transient analog pickle state version {version}"
        )));
    }
    let Some(compression_report) = compression_state else {
        return Err(crate::errors::value_error(
            "compressed-transient pickle is missing its required compression error certificate; rerun the analysis",
        ));
    };
    let inner = rspice_core::engine::TransientResultCompressed {
        time,
        step_sizes,
        channels: channels
            .into_iter()
            .map(rebuild_channel)
            .collect::<PyResult<Vec<_>>>()?,
        digital_traces: digital_traces
            .into_iter()
            .map(rebuild_digital_trace)
            .collect::<PyResult<Vec<_>>>()?,
        real_traces: real_traces.into_iter().map(rebuild_real_trace).collect(),
        post_results: rspice_core::engine::TransientPostResults {
            fft: rebuild_transient_fft_results(fft_state)?,
            fourier: fourier
                .into_iter()
                .map(rebuild_fourier)
                .collect::<PyResult<Vec<_>>>()?,
            measurements: measurements.into_iter().map(rebuild_measurement).collect(),
        },
        identity: rebuild_identity(identity)?,
        compression_ratio,
        input_points,
        compression_report: rebuild_compression_report(compression_report)?,
    };
    inner.validate().map_err(crate::errors::value_error)?;
    Ok(inner)
}
