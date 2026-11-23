//! Application capture and restoration of portable project results.
use crate::state::{AnalysisResult, RetainedTrace, SimulationRun, SimulationState};
use rspice_formats::project_results::*;

#[cfg(any(test, feature = "test-support"))]
thread_local! {
    /// Captures of the retained results whole, each a record of every
    /// signal of every run.
    pub static WHOLE_RESULT_CAPTURES: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

pub fn capture_simulation_results(state: &SimulationState) -> ProjectSimulationResults {
    if state.retained.runs.is_empty() {
        // Clearing datasets preserves both their allocation history and
        // the project's retention decision across save and session restore.
        return capture_simulation_results_without_runs(state);
    }
    #[cfg(any(test, feature = "test-support"))]
    WHOLE_RESULT_CAPTURES.with(|captures| captures.set(captures.get() + 1));
    with_runs(state, state.retained.runs.iter().map(capture_run).collect())
}

/// [`capture_simulation_results`] with each run by its header alone
/// ([`ProjectSimulationRun::header_of`]): what tells the retained results apart
/// for a dirty check, which reads no signal. Never a file's results: it does
/// not validate.
pub fn capture_simulation_result_headers(state: &SimulationState) -> ProjectSimulationResults {
    if state.retained.runs.is_empty() {
        return capture_simulation_results_without_runs(state);
    }
    with_runs(
        state,
        state
            .retained
            .runs
            .iter()
            .map(|run| ProjectSimulationRun::header_of(&run.data))
            .collect(),
    )
}

fn with_runs(state: &SimulationState, runs: Vec<ProjectSimulationRun>) -> ProjectSimulationResults {
    ProjectSimulationResultsData {
        runs,
        imported_monte_carlo_checkpoints: state.retained.imported_monte_carlo_checkpoints.clone(),
        imported_transient_checkpoints: state.retained.imported_transient_checkpoints.clone(),
        next_run_id: next_run_id(state),
        retained_dataset_limit: state.retained.retained_dataset_limit,
        active_run_stable_id: state.active_run().map(|run| run.run_id),
        active_dataset_id: state.active_run().map(|run| run.dataset_id),
        active_analysis_sequence: state.active_analysis().map(|analysis| analysis.id),
        overlay_dataset_ids: state.view.overlay_dataset_ids.clone(),
        executed_decks: ProjectExecutedDecks::from_archive(
            &state.retained.executed_decks,
            state.retained.runs.iter().map(|run| run.id),
        ),
        ..ProjectSimulationResultsData::default()
    }
    .into()
}

/// The record a project or session keeps of `run`, naming the results store segments its
/// samples are in.
///
/// A finished analysis is kept in the store as it is retained; one the store could not take
/// then is written to it now, into the record and not the history, so the file still holds no
/// samples. An analysis still running keeps only its record: its samples are a preview the
/// finished result replaces, whether it holds them in memory or reads them from the run's
/// segment as it grows.
pub fn capture_run(run: &SimulationRun) -> ProjectSimulationRun {
    // A preview drawn from the segment holds nothing in memory, and recording each of its
    // columns is what froze the window, half a minute at a time, while a million-device run
    // drew its preview: the record is taken again whenever the preview grows.
    let previews = run
        .analyses
        .iter()
        .any(|analysis| analysis.is_live_partial());
    if !previews && !run.holds_samples_in_memory() {
        return ProjectSimulationRun::from(run);
    }
    let mut kept = run.clone();
    for analysis in &mut kept.data.analyses {
        if analysis.is_live_partial() {
            analysis.waveforms.clear();
        } else if let Err(error) = crate::state::run_store::store_analysis(&mut analysis.data) {
            log::warn!("{}: {error}", analysis.label);
        }
    }
    ProjectSimulationRun::from(&kept)
}

/// The results with every run left out: the imported resume inputs, the
/// retention limit, and the run counter, so the next run is not numbered as
/// one of the runs that were left out.
pub fn capture_simulation_results_without_runs(
    state: &SimulationState,
) -> ProjectSimulationResults {
    ProjectSimulationResultsData {
        imported_transient_checkpoints: state.retained.imported_transient_checkpoints.clone(),
        imported_monte_carlo_checkpoints: state.retained.imported_monte_carlo_checkpoints.clone(),
        next_run_id: next_run_id(state),
        retained_dataset_limit: state.retained.retained_dataset_limit,
        ..ProjectSimulationResultsData::default()
    }
    .into()
}

fn next_run_id(state: &SimulationState) -> u64 {
    let max_run_id = state
        .retained
        .runs
        .iter()
        .map(|run| run.id)
        .max()
        .unwrap_or(0);
    state.retained.next_run_id.max(max_run_id)
}
#[cfg(any(test, feature = "test-support"))]
pub fn simulation_state_from_results(
    results: ProjectSimulationResults,
) -> Result<SimulationState, String> {
    let mut state = SimulationState::default();
    restore_simulation_results(results, &mut state)?;
    Ok(state)
}
pub fn restore_simulation_results(
    results: ProjectSimulationResults,
    state: &mut SimulationState,
) -> Result<(), String> {
    let data = results.restore_with(restore_analysis)?;
    let runs = data
        .runs
        .into_iter()
        .map(SimulationRun::from_restored)
        .collect();
    state.retained.retained_dataset_limit = data.retained_dataset_limit;
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
    state.retained.executed_decks = data.executed_decks;
    state.retained.imported_monte_carlo_checkpoints = data.imported_monte_carlo_checkpoints;
    state.retained.imported_transient_checkpoints = data.imported_transient_checkpoints;
    Ok(())
}
/// Restore one run from its project record, exactly as a project restores
/// each run of its history.
pub fn restore_project_run(run: ProjectSimulationRun) -> Result<SimulationRun, String> {
    run.into_run_with(restore_analysis)
        .map(SimulationRun::from_restored)
}
fn restore_analysis(analysis: ProjectAnalysisResult) -> Result<AnalysisResult, String> {
    let data = analysis.into_analysis_with_waveforms(|mut waveform| {
        let color = std::mem::take(&mut waveform.color);
        let visible = waveform.visible;
        RetainedTrace {
            data: waveform.into_waveform(),
            color,
            visible,
        }
    })?;
    Ok(AnalysisResult { data })
}
