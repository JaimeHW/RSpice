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

pub(crate) const COMPRESSED_TRANSIENT_ANALOG_STATE_VERSION: usize = 4;

/// The version written before the digital bus contract existed.
///
/// Read rather than refused, for the reason the full result's version 1 is:
/// nothing that wrote a version-3 state could declare a bus, so an empty table
/// is what it says rather than what it lost. Every version below it is refused,
/// because those really did lack channels, event traces, identity or
/// post-results a restored result cannot do without.
pub(crate) const COMPRESSED_TRANSIENT_ANALOG_STATE_VERSION_WITHOUT_BUSES: usize = 3;

pub(crate) type CompressedTransientAnalogState = (
    usize,
    Vec<f64>,
    Vec<CompressedChannelPersistenceState>,
    Vec<CompressedDigitalTracePersistenceState>,
    Vec<CompressedRealTracePersistenceState>,
    CompressedIdentityPersistenceState,
    Vec<CompressedFourierPersistenceState>,
    Vec<CompressedMeasurementPersistenceState>,
    Vec<CompressedDigitalBusPersistenceState>,
);

/// The same state as written by a build that had no bus contract.
pub(crate) type CompressedTransientAnalogStateWithoutBuses = (
    usize,
    Vec<f64>,
    Vec<CompressedChannelPersistenceState>,
    Vec<CompressedDigitalTracePersistenceState>,
    Vec<CompressedRealTracePersistenceState>,
    CompressedIdentityPersistenceState,
    Vec<CompressedFourierPersistenceState>,
    Vec<CompressedMeasurementPersistenceState>,
);

/// A pickled analog state at whichever version wrote it.
///
/// The variants are tried in order and told apart by field count, so a state
/// is matched to a shape before its version tag is read rather than after.
#[derive(Debug, Clone, pyo3::FromPyObject)]
pub(crate) enum VersionedCompressedTransientAnalogState {
    /// Nine fields: the current contract, carrying a bus table.
    #[pyo3(transparent)]
    Current(CompressedTransientAnalogState),
    /// Eight fields: written before the bus contract existed.
    #[pyo3(transparent)]
    WithoutBuses(CompressedTransientAnalogStateWithoutBuses),
}

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
        result
            .digital_buses
            .iter()
            .map(digital_bus_persistence_state)
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
    analog_state: Option<VersionedCompressedTransientAnalogState>,
    compression_state: Option<CompressionReportPersistenceState>,
) -> PyResult<rspice_core::engine::TransientResultCompressed> {
    let Some(analog_state) = analog_state else {
        return Err(crate::errors::value_error(
            "legacy compressed-transient pickle predates lossless analog inventory persistence; rerun the analysis",
        ));
    };
    let (
        version,
        expected_version,
        step_sizes,
        channels,
        digital_traces,
        real_traces,
        identity,
        fourier,
        measurements,
        buses,
    ) = match analog_state {
        VersionedCompressedTransientAnalogState::Current((
            version,
            step_sizes,
            channels,
            digital_traces,
            real_traces,
            identity,
            fourier,
            measurements,
            buses,
        )) => (
            version,
            COMPRESSED_TRANSIENT_ANALOG_STATE_VERSION,
            step_sizes,
            channels,
            digital_traces,
            real_traces,
            identity,
            fourier,
            measurements,
            buses,
        ),
        VersionedCompressedTransientAnalogState::WithoutBuses((
            version,
            step_sizes,
            channels,
            digital_traces,
            real_traces,
            identity,
            fourier,
            measurements,
        )) => (
            version,
            COMPRESSED_TRANSIENT_ANALOG_STATE_VERSION_WITHOUT_BUSES,
            step_sizes,
            channels,
            digital_traces,
            real_traces,
            identity,
            fourier,
            measurements,
            // Nothing that wrote this state could declare a bus, so an empty
            // table is what it says rather than what it lost.
            Vec::new(),
        ),
    };
    if version < COMPRESSED_TRANSIENT_ANALOG_STATE_VERSION_WITHOUT_BUSES {
        return Err(crate::errors::value_error(format!(
            "compressed-transient analog pickle state version {version} predates the descriptor-indexed channel container with per-sample validity, event traces, parent identity, and post-results; rerun the analysis"
        )));
    }
    // The shape a state arrived in is what its version has to agree with: a
    // nine-field state is version 4 and an eight-field one is version 3, so a
    // state claiming the other's number is refused rather than read as either.
    if version != expected_version {
        return Err(crate::errors::value_error(format!(
            "unsupported compressed-transient analog pickle state version {version}; this build reads version {COMPRESSED_TRANSIENT_ANALOG_STATE_VERSION_WITHOUT_BUSES} (no bus table) and version {COMPRESSED_TRANSIENT_ANALOG_STATE_VERSION}, each in its own shape"
        )));
    }
    let Some(compression_report) = compression_state else {
        return Err(crate::errors::value_error(
            "compressed-transient pickle is missing its required compression error certificate; rerun the analysis",
        ));
    };
    let digital_traces = digital_traces
        .into_iter()
        .map(rebuild_digital_trace)
        .collect::<PyResult<Vec<_>>>()?;
    let digital_buses = buses
        .into_iter()
        .map(rebuild_digital_bus)
        .collect::<PyResult<Vec<_>>>()?;
    rspice_core::engine::validate_digital_bus_table(
        &digital_buses,
        digital_traces.iter().map(|trace| trace.node_name.as_str()),
    )
    .map_err(|error| {
        crate::errors::value_error(format!(
            "pickled compressed-transient state declares a digital bus it cannot carry: {error}"
        ))
    })?;
    let inner = rspice_core::engine::TransientResultCompressed {
        time,
        step_sizes,
        channels: channels
            .into_iter()
            .map(rebuild_channel)
            .collect::<PyResult<Vec<_>>>()?,
        digital_traces,
        digital_buses,
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
