//! Application capture and restoration of portable project results.
use crate::state::{AnalysisResult, SimulationRun, SimulationState, WaveformData};
use rspice_formats::project_results::*;
use std::collections::HashSet;

fn capture_waveform(waveform: &WaveformData) -> ProjectWaveformData {
    ProjectWaveformData::from_waveform(&waveform.data, waveform.color.clone(), waveform.visible)
}

impl From<&AnalysisResult> for ProjectAnalysisResult {
    fn from(analysis: &AnalysisResult) -> Self {
        Self::from_analysis(&analysis.data, capture_waveform)
    }
}
impl From<&SimulationRun> for ProjectSimulationRun {
    fn from(run: &SimulationRun) -> Self {
        Self::from_run(&run.data, capture_waveform)
    }
}
pub(crate) fn capture_simulation_results(state: &SimulationState) -> ProjectSimulationResults {
    if state.runs.is_empty() {
        // Clearing datasets preserves both their allocation history and
        // the project's retention decision across save and session restore.
        return ProjectSimulationResultsData {
            imported_monte_carlo_checkpoints: state.imported_monte_carlo_checkpoints.clone(),
            next_run_id: state.next_run_id,
            retained_dataset_limit: state.retained_dataset_limit,
            ..ProjectSimulationResultsData::default()
        }
        .into();
    }

    let runs: Vec<_> = state.runs.iter().map(ProjectSimulationRun::from).collect();
    let max_run_id = state.runs.iter().map(|run| run.id).max().unwrap_or(0);
    ProjectSimulationResultsData {
        runs,
        imported_monte_carlo_checkpoints: state.imported_monte_carlo_checkpoints.clone(),
        next_run_id: state.next_run_id.max(max_run_id),
        retained_dataset_limit: state.retained_dataset_limit,
        active_run_stable_id: state.active_run().map(|run| run.run_id),
        active_dataset_id: state.active_run().map(|run| run.dataset_id),
        active_analysis_sequence: state.active_analysis().map(|analysis| analysis.id),
        overlay_dataset_ids: state.overlay_dataset_ids.clone(),
        executed_decks: ProjectExecutedDecks::from_archive(
            &state.executed_decks,
            state.runs.iter().map(|run| run.id),
        ),
        active_run_id: None,
        active_analysis_id: None,
        overlay_run_ids: Vec::new(),
        ..ProjectSimulationResultsData::default()
    }
    .into()
}
#[cfg(test)]
pub(crate) fn simulation_state_from_results(
    results: ProjectSimulationResults,
) -> Result<SimulationState, String> {
    let mut state = SimulationState::default();
    restore_simulation_results(results, &mut state)?;
    Ok(state)
}
pub(crate) fn restore_simulation_results(
    results: ProjectSimulationResults,
    state: &mut SimulationState,
) -> Result<(), String> {
    let data = results.restore_with(restore_analysis)?;
    let runs = data
        .runs
        .into_iter()
        .map(SimulationRun::from_restored)
        .collect();
    state.retained_dataset_limit = data.retained_dataset_limit;
    state.restore_run_history(
        runs,
        data.next_run_id,
        data.active_run_stable_id,
        data.active_dataset_id,
        data.active_analysis_sequence,
        data.overlay_dataset_ids,
    );
    // After the history, because restoring it drops whatever decks this
    // session was holding for a different project.
    state.executed_decks = data.executed_decks;
    state.imported_monte_carlo_checkpoints = data.imported_monte_carlo_checkpoints;
    Ok(())
}
fn restore_analysis(analysis: ProjectAnalysisResult) -> Result<AnalysisResult, String> {
    let data = analysis.into_analysis_with_waveforms(|mut waveform| {
        let color = std::mem::take(&mut waveform.color);
        let visible = waveform.visible;
        WaveformData {
            data: waveform.into_waveform(),
            color,
            visible,
            display_cache: None,
        }
    })?;
    let mut analysis = AnalysisResult { data };
    let cached_names = analysis
        .data
        .saved_output_receipts
        .iter()
        .filter(|receipt| {
            receipt.stored_precision
                == crate::state::SavedOutputPrecision::DisplayCacheWithFullSourcePrecision
                || receipt.streaming
                    == crate::state::SavedOutputStreaming::LivePlotAdaptiveDisplayDecimation
        })
        .flat_map(|receipt| {
            receipt
                .status
                .materialized_waveforms()
                .map(|(name, _)| name)
        })
        .collect::<HashSet<_>>();
    if !cached_names.is_empty() {
        for waveform in &mut analysis.data.waveforms {
            if cached_names.contains(waveform.name.as_str()) {
                waveform
                    .rebuild_display_cache(crate::state::DEFAULT_DISPLAY_WAVEFORM_CACHE_SAMPLES);
            }
        }
    }
    Ok(analysis)
}
