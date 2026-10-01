//! Retained result inputs without an application clock or display cache.

use rspice_formats::project_results::ProjectSimulationResults;
use rspice_results::{
    analysis_result::AnalysisResult, run::SimulationRun, waveform::RetainedWaveform,
};

/// Source samples and the persisted styling consumed by semantic hardcopy.
#[derive(Debug, Clone)]
pub struct HardcopyWaveform {
    pub data: RetainedWaveform,
    pub color: String,
    pub visible: bool,
}

impl AsRef<RetainedWaveform> for HardcopyWaveform {
    fn as_ref(&self) -> &RetainedWaveform {
        &self.data
    }
}

pub type HardcopyRun = SimulationRun<AnalysisResult<HardcopyWaveform>>;

/// Restore exact retained runs after the shared schema, provenance, source-data,
/// and executed-deck checks. Selection state and display caches are not sources.
pub fn restore_hardcopy_runs(
    results: ProjectSimulationResults,
) -> Result<Vec<HardcopyRun>, String> {
    let restored = results.restore_with(|analysis| {
        analysis.into_analysis_with_waveforms(|mut waveform| {
            let color = std::mem::take(&mut waveform.color);
            let visible = waveform.visible;
            HardcopyWaveform {
                data: waveform.into_waveform(),
                color,
                visible,
            }
        })
    })?;
    Ok(restored.runs)
}
