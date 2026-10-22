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
mod tests;
