//! Worker QPNOISE display is an exact projection of the complete retained transfer result.
use super::*;
pub(super) fn validate_worker_qpnoise_result(
    result: &WorkerSimulationResult,
) -> Result<(), String> {
    let WorkerSimulationResult::Qpnoise {
        frequencies,
        waveforms,
        response,
    } = result
    else {
        return Ok(());
    };
    if *frequencies
        != response
            .outputs
            .first()
            .ok_or("QPNOISE has no output")?
            .frequencies_hz
        || *waveforms != worker_waveforms(SimulationResult::qpnoise_waveforms(response)?)
    {
        return Err("QPNOISE worker display differs from its retained transfer result".into());
    }
    Ok(())
}
pub(super) fn response_bytes(response: &rspice_core::engine::QpnoiseAnalysisResult) -> usize {
    crate::state::AnalysisResultPayload::qpnoise_response_bytes(response)
}
#[cfg(test)]
mod tests;
