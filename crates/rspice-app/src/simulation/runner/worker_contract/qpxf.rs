//! Worker QPXF display is an exact projection of the complete retained transfer result.
use super::*;
pub(super) fn validate_worker_qpxf_result(result: &WorkerSimulationResult) -> Result<(), String> {
    let WorkerSimulationResult::Qpxf {
        frequencies,
        waveforms,
        response,
    } = result
    else {
        return Ok(());
    };
    if *frequencies != response.metadata.output_frequencies_hz
        || *waveforms != worker_waveforms(SimulationResult::qpxf_waveforms(response)?)
    {
        return Err("QPXF worker display differs from its retained transfer result".into());
    }
    Ok(())
}
#[cfg(test)]
pub(super) fn response_bytes(response: &rspice_core::engine::QpxfAnalysisResult) -> usize {
    let complex = response
        .solutions
        .iter()
        .flat_map(|s| &s.sensitivities)
        .chain(response.transfers.iter().map(|t| &t.values))
        .fold(0usize, |n, row| {
            n.saturating_add(row.len().saturating_mul(16))
        });
    let axes_and_delay = response.transfers.iter().fold(0usize, |n, t| {
        n.saturating_add(t.input_frequencies_hz.len().saturating_mul(8))
            .saturating_add(
                t.group_delay
                    .as_ref()
                    .map_or(0, |d| d.len().saturating_mul(16)),
            )
    });
    let metadata = response
        .metadata
        .request
        .frequencies_hz
        .len()
        .saturating_mul(32)
        .saturating_add(response.metadata.input_sources.len().saturating_mul(64))
        .saturating_add(
            response
                .metadata
                .input_lattices
                .iter()
                .map(|t| t.len().saturating_mul(4))
                .sum::<usize>(),
        );
    sum_payload_bytes([complex, axes_and_delay, metadata])
}
#[cfg(test)]
mod tests;
