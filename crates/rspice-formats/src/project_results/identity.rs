//! Retained results as a dirty check compares them: each run by who it is.
//!
//! A run is immutable once written. Its samples, evidence and per-signal
//! presentation never change after it is kept, so what tells one retained
//! history from another is which runs it holds, where each stands in its
//! lifecycle, what the reader made of each (its label, retention and note),
//! and each analysis by its header. A record holding a run's every column and
//! one holding only its headers ([`ProjectSimulationRun::header_of`]) are the
//! same run, and compare equal here; reading either never walks its signals.

use super::*;

/// What a fingerprint of retained results reads: the runs by identity, the
/// selection, the retention limit, the imported resume inputs and the executed
/// decks by digest. The runs' logs are their account of themselves, not
/// result content, and are not read.
#[derive(Serialize)]
pub struct ProjectResultsIdentity<'a> {
    runs: Vec<RunIdentity<'a>>,
    imported_monte_carlo_checkpoints:
        &'a rspice_results::monte_carlo_checkpoint::MonteCarloCheckpointLibrary,
    imported_transient_checkpoints:
        &'a rspice_results::transient_checkpoint::TransientCheckpointLibrary,
    next_run_id: u64,
    retained_dataset_limit: Option<usize>,
    active_run_stable_id: Option<RunId>,
    active_dataset_id: Option<DatasetId>,
    active_analysis_sequence: Option<u64>,
    overlay_dataset_ids: &'a [DatasetId],
    executed_deck_sources: Vec<ContentDigest>,
    executed_decks: &'a [ProjectExecutedDeck],
}

#[derive(Serialize)]
struct RunIdentity<'a> {
    job_id: Option<JobId>,
    run_id: Option<RunId>,
    dataset_id: Option<DatasetId>,
    execution_target: Option<ExecutionTarget>,
    lifecycle: Option<SimulationRunLifecycle>,
    id: u64,
    label: &'a str,
    timestamp: f64,
    analyses: Vec<AnalysisIdentity<'a>>,
    provenance_mode: &'a PersistedField<ProjectRunProvenanceMode>,
    prepared_receipt: &'a PersistedField<ProjectPreparedRunReceipt>,
    campaign_membership: Option<&'a rspice_results::run::SimulationCampaignMembership>,
    retention: RunRetention,
    elapsed_time: f64,
    success: bool,
    specification_verdicts: Option<&'a [SpecificationVerdict]>,
    note: &'a str,
    results_storage_stop: Option<&'a rspice_results::run::ResultsStorageStop>,
    facts: Option<&'a rspice_results::run_facts::RunFacts>,
}

#[derive(Serialize)]
struct AnalysisIdentity<'a> {
    id: u64,
    analysis_type: &'a str,
    label: &'a str,
    timestamp: f64,
    success: bool,
    error_message: Option<&'a str>,
    failure_capability: Option<&'a str>,
}

impl ProjectSimulationResultsData {
    /// These results as a dirty check compares them, reading no signal.
    #[must_use]
    pub fn identity(&self) -> ProjectResultsIdentity<'_> {
        ProjectResultsIdentity {
            runs: self.runs.iter().map(RunIdentity::of).collect(),
            imported_monte_carlo_checkpoints: &self.imported_monte_carlo_checkpoints,
            imported_transient_checkpoints: &self.imported_transient_checkpoints,
            next_run_id: self.next_run_id,
            retained_dataset_limit: self.retained_dataset_limit,
            active_run_stable_id: self.active_run_stable_id,
            active_dataset_id: self.active_dataset_id,
            active_analysis_sequence: self.active_analysis_sequence,
            overlay_dataset_ids: &self.overlay_dataset_ids,
            executed_deck_sources: self
                .executed_decks
                .sources
                .iter()
                .map(ExecutedDeckSource::digest)
                .collect(),
            executed_decks: &self.executed_decks.runs,
        }
    }
}

impl<'a> RunIdentity<'a> {
    fn of(run: &'a ProjectSimulationRun) -> Self {
        Self {
            job_id: run.job_id,
            run_id: run.run_id,
            dataset_id: run.dataset_id,
            execution_target: run.execution_target,
            lifecycle: run.lifecycle,
            id: run.id,
            label: &run.label,
            timestamp: run.timestamp,
            analyses: run
                .analyses
                .iter()
                .map(|analysis| AnalysisIdentity {
                    id: analysis.id,
                    analysis_type: &analysis.analysis_type,
                    label: &analysis.label,
                    timestamp: analysis.timestamp,
                    success: analysis.success,
                    error_message: analysis.error_message.as_deref(),
                    failure_capability: analysis.failure_capability.as_deref(),
                })
                .collect(),
            provenance_mode: &run.provenance_mode,
            prepared_receipt: &run.prepared_receipt,
            campaign_membership: run.campaign_membership.as_ref(),
            retention: run.retention,
            elapsed_time: run.elapsed_time,
            success: run.success,
            specification_verdicts: run.specification_verdicts.as_deref(),
            note: &run.note,
            results_storage_stop: run.results_storage_stop.as_ref(),
            facts: run.facts.as_ref(),
        }
    }
}
