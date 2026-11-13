//! What a project file persists about its simulation runs.
//!
//! Deliberately separate from `SimulationState`: a project file keeps the
//! result data a user can see and re-open, not the transient runner flags,
//! progress text, and UI trigger bits that only mean something inside a
//! session.
//!
//! Every retained run carries digests over its own results, and the
//! validators here are per schema version rather than shared. A file written
//! by an older build sealed fewer fields, so checking it against today's rule
//! set would reject a valid project; checking it against no rule set would
//! accept a tampered one. The version-specific validators are what let both
//! be true at once.

use rspice_design_model::cell_view::CellViewRef;
use rspice_results::analysis_payload::AnalysisResultPayload;
use rspice_results::analysis_result::AnalysisResult;
use rspice_results::analysis_type::AnalysisType;
use rspice_results::family_metadata::AnalysisResultFamilyMetadata;
use rspice_results::noise::{NoiseContributorRow, NoiseSummary};
use rspice_results::operating_point::{DcOpResult, OperatingPointValue};
use rspice_results::provenance::{
    AnalysisResultFamilyMember, AnalysisResultProvenance, AnalysisResultPvtPoint,
    AnalysisResultSourceDomain, AnalysisResultTransientState,
};
use rspice_results::result_digest::ResultDigestEncoding;
use rspice_results::run::{ExecutionTarget, RunRetention, SimulationRun, SimulationRunLifecycle};
use rspice_results::run_receipt::{
    PreparedModelSourceIdentity, PreparedRunReceipt, PreparedRunTaskReceipt,
    PreparedSourceCheckReceipt, SimulationRunProvenance,
};
use rspice_results::saved_output::SavedOutputReceipt;
use rspice_results::specification::{
    PreparedSpecification, PreparedSpecificationPolicy, SpecEntry, SpecificationDefinition,
    SpecificationPolicy,
};
use rspice_results::specification_verdict::SpecificationVerdict;
use rspice_results::waveform::RetainedWaveform;
use rspice_vocabulary::product::{
    AnalysisInstanceId, ContentDigest, DatasetId, DerivedAnalysisIdentity, JobId, ModelSourceId,
    ObjectRevision, RunId, SimulationPlanId,
};
use rspice_vocabulary::run_log::RunLog;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::HashSet;

#[cfg(any(test, feature = "result-validation-observation"))]
thread_local! {
    pub static RESULT_VALIDATIONS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

mod checks;
pub use checks::{ProjectAssertViolation, ProjectAssertionReport, ProjectRunChecks};
mod codec;
pub use codec::*;

mod executed_deck;
mod snapshot;
pub use snapshot::{ProjectSimulationResults, RestoredProjectSimulationResults};
mod provenance;
pub use executed_deck::ProjectExecutedDecks;
// The rows inside are named only where a file is written by hand: production
// code builds them from the session archive and reads them back through it.
pub use executed_deck::{ProjectExecutedDeck, ProjectExecutedDeckPoint};
pub use provenance::*;
/// Stable project-file representation of result history.
///
/// This is intentionally separate from `SimulationState`: project files should
/// persist user-visible result data, not transient runner flags, progress text,
/// or UI trigger bits.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectSimulationResultsData {
    pub schema_version: u32,
    #[serde(default)]
    pub runs: Vec<ProjectSimulationRun>,
    #[serde(
        default,
        skip_serializing_if = "rspice_results::monte_carlo_checkpoint::MonteCarloCheckpointLibrary::is_empty"
    )]
    pub imported_monte_carlo_checkpoints:
        rspice_results::monte_carlo_checkpoint::MonteCarloCheckpointLibrary,
    #[serde(
        default,
        skip_serializing_if = "rspice_results::transient_checkpoint::TransientCheckpointLibrary::is_empty"
    )]
    pub imported_transient_checkpoints:
        rspice_results::transient_checkpoint::TransientCheckpointLibrary,
    /// Last allocated display sequence, retained even after all runs are cleared.
    #[serde(default)]
    pub next_run_id: u64,
    /// How many datasets this project retains. Absent means the built-in
    /// default, which is what every project written before the limit became
    /// visible was running under.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub retained_dataset_limit: Option<usize>,
    /// Stable v2 selection identity.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active_run_stable_id: Option<RunId>,
    /// Stable v2 selected immutable dataset identity.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active_dataset_id: Option<DatasetId>,
    /// Run-local analysis sequence within the selected immutable dataset.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active_analysis_sequence: Option<u64>,
    /// Stable v2 dataset overlay identities.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub overlay_dataset_ids: Vec<DatasetId>,
    /// The exact deck each retained run's engine read, per point.
    #[serde(default, skip_serializing_if = "ProjectExecutedDecks::is_empty")]
    pub executed_decks: ProjectExecutedDecks,
}

impl Default for ProjectSimulationResultsData {
    fn default() -> Self {
        Self {
            schema_version: PROJECT_SIMULATION_RESULTS_SCHEMA_VERSION,
            runs: Vec::new(),
            imported_monte_carlo_checkpoints: Default::default(),
            imported_transient_checkpoints: Default::default(),
            next_run_id: 0,
            retained_dataset_limit: None,
            active_run_stable_id: None,
            active_dataset_id: None,
            active_analysis_sequence: None,
            overlay_dataset_ids: Vec::new(),
            executed_decks: ProjectExecutedDecks::default(),
        }
    }
}

impl ProjectSimulationResultsData {
    pub fn is_empty(&self) -> bool {
        self.runs.is_empty()
            && self.imported_monte_carlo_checkpoints.is_empty()
            && self.imported_transient_checkpoints.is_empty()
            && self.next_run_id == 0
            && self.active_run_stable_id.is_none()
            && self.active_dataset_id.is_none()
            && self.active_analysis_sequence.is_none()
            && self.overlay_dataset_ids.is_empty()
            && self.executed_decks.is_empty()
            && self.retained_dataset_limit.is_none()
    }

    fn validate(&self) -> Result<(), String> {
        #[cfg(any(test, feature = "result-validation-observation"))]
        RESULT_VALIDATIONS.with(|passes| passes.set(passes.get() + 1));
        if self.schema_version != PROJECT_SIMULATION_RESULTS_SCHEMA_VERSION {
            return Err(format!(
                "unsupported simulation results schema version {}",
                self.schema_version
            ));
        }

        let mut run_sequences = HashSet::new();
        let mut job_ids = HashSet::new();
        let mut run_ids = HashSet::new();
        let mut dataset_ids = HashSet::new();
        for (run_idx, run) in self.runs.iter().enumerate() {
            if run.id == 0 {
                return Err(format!("runs[{run_idx}].id must be greater than zero"));
            }
            if !run_sequences.insert(run.id) {
                return Err(format!("duplicate simulation run id {}", run.id));
            }
            if let Some(job_id) = run.job_id
                && !job_ids.insert(job_id)
            {
                return Err(format!("duplicate stable simulation job id {job_id}"));
            }
            let run_id = run.run_id.ok_or_else(|| {
                format!("runs[{run_idx}].run_id is required by simulation results schema v2")
            })?;
            if !run_ids.insert(run_id) {
                return Err(format!("duplicate stable simulation run id {run_id}"));
            }
            let dataset_id = run.dataset_id.ok_or_else(|| {
                format!("runs[{run_idx}].dataset_id is required by simulation results schema v2")
            })?;
            if !dataset_ids.insert(dataset_id) {
                return Err(format!("duplicate immutable dataset id {dataset_id}"));
            }
            run.validate(run_idx)?;
        }
        if let Some(active_run_id) = self.active_run_stable_id
            && !run_ids.contains(&active_run_id)
        {
            return Err(format!(
                "active simulation run id {} does not exist in persisted history",
                active_run_id
            ));
        }
        match (self.active_run_stable_id, self.active_dataset_id) {
            (Some(active_run_id), Some(active_dataset_id)) => {
                let Some(active_run) = self
                    .runs
                    .iter()
                    .find(|run| run.run_id == Some(active_run_id))
                else {
                    return Err(format!(
                        "active simulation run id {} does not exist in persisted history",
                        active_run_id
                    ));
                };
                if active_run.dataset_id != Some(active_dataset_id) {
                    return Err(format!(
                        "active dataset id {} does not belong to active run {}",
                        active_dataset_id, active_run_id
                    ));
                }
                if let Some(sequence) = self.active_analysis_sequence
                    && !active_run
                        .analyses
                        .iter()
                        .any(|analysis| analysis.id == sequence)
                {
                    return Err(format!(
                        "active analysis sequence {} does not exist in active dataset {}",
                        sequence, active_dataset_id
                    ));
                }
            }
            (Some(active_run_id), None) => {
                return Err(format!(
                    "active simulation run id {} has no active dataset id",
                    active_run_id
                ));
            }
            (None, Some(active_dataset_id)) => {
                return Err(format!(
                    "active dataset id {} has no active run id",
                    active_dataset_id
                ));
            }
            (None, None) if self.active_analysis_sequence.is_some() => {
                return Err("active analysis sequence has no active dataset".to_owned());
            }
            (None, None) => {}
        }
        let mut overlay_ids = HashSet::new();
        for overlay_id in &self.overlay_dataset_ids {
            if !dataset_ids.contains(overlay_id) {
                return Err(format!(
                    "overlay dataset id {} does not exist in persisted history",
                    overlay_id
                ));
            }
            if !overlay_ids.insert(*overlay_id) {
                return Err(format!("duplicate overlay dataset id {}", overlay_id));
            }
            if Some(*overlay_id) == self.active_dataset_id {
                return Err(format!(
                    "active dataset id {} cannot also be an overlay",
                    overlay_id
                ));
            }
        }
        self.executed_decks.validate(&run_sequences)?;
        Ok(())
    }
}

pub fn seal_project_result_digests(run: &mut ProjectSimulationRun) -> Result<(), String> {
    for analysis in &mut run.analyses {
        let digest = analysis
            .clone()
            .into_analysis()?
            .result_data_ref()
            .digest(ResultDigestEncoding::CURRENT);
        analysis.result_data_digest = PersistedField::Value(digest);
    }
    let digest = run
        .clone()
        .into_run()?
        .dataset_content_digest_with_encoding(ResultDigestEncoding::CURRENT);
    run.dataset_content_digest = PersistedField::Value(digest);
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectSimulationRun {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_id: Option<JobId>,
    #[serde(default)]
    pub run_id: Option<RunId>,
    #[serde(default)]
    pub dataset_id: Option<DatasetId>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub execution_target: Option<ExecutionTarget>,
    /// Required by schema v6. Historical schemas are migrated to an explicit
    /// `LegacyUnknown` value without outcome inference.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lifecycle: Option<SimulationRunLifecycle>,
    /// Display sequence retained for labels and v1 migration.
    pub id: u64,
    pub label: String,
    pub timestamp: f64,
    #[serde(default)]
    pub analyses: Vec<ProjectAnalysisResult>,
    /// Canonical identity of the ordered retained analysis content. Schemas
    /// through v7 omitted this field and acquire it during migration.
    #[serde(default, skip_serializing_if = "PersistedField::is_missing")]
    pub dataset_content_digest: PersistedField<ContentDigest>,
    #[serde(default, skip_serializing_if = "PersistedField::is_missing")]
    pub provenance_mode: PersistedField<ProjectRunProvenanceMode>,
    #[serde(default, skip_serializing_if = "PersistedField::is_missing")]
    pub prepared_receipt: PersistedField<ProjectPreparedRunReceipt>,
    /// Optional immutable grouping identity for a multi-plan campaign. The
    /// member run remains independently authenticated by `prepared_receipt`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub campaign_membership: Option<rspice_results::run::SimulationCampaignMembership>,
    /// Retention classification. Absent is `Pruneable`, which is what every
    /// project written before baselines existed was already under; it is a
    /// user policy over the run rather than sealed result content, so it is
    /// outside the dataset digest and needs no schema era of its own.
    #[serde(default, skip_serializing_if = "RunRetention::is_pruneable")]
    pub retention: RunRetention,
    #[serde(default)]
    pub elapsed_time: f64,
    #[serde(default = "default_true")]
    pub success: bool,
    /// Immutable judgments against the requirements sealed into the prepared
    /// receipt. `None` is a legacy result era; `Some([])` is an authoritative
    /// current run with no authored specifications.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub specification_verdicts: Option<Vec<SpecificationVerdict>>,
    /// The run's own log. Absent is empty, which is every run written before
    /// runs kept one. Like retention it is outside the dataset digest, and the
    /// result history's fingerprint reads past it
    /// ([`ProjectSimulationResults::without_run_logs`]): it is the run's
    /// account of itself, not result content, so it needs no schema era.
    #[serde(default, skip_serializing_if = "RunLog::is_empty")]
    pub log: RunLog,
}

impl ProjectSimulationRun {
    pub fn into_run(self) -> Result<SimulationRun, String> {
        self.into_run_with_partial_restore(true, ProjectAnalysisResult::into_analysis)
    }

    pub fn into_run_with<A, W: AsRef<RetainedWaveform>>(
        self,
        restore_analysis: impl FnMut(ProjectAnalysisResult) -> Result<A, String>,
    ) -> Result<SimulationRun<A>, String>
    where
        A: AsRef<AnalysisResult<W>> + AsMut<AnalysisResult<W>>,
    {
        self.into_run_with_partial_restore(true, restore_analysis)
    }

    fn into_run_with_partial_restore<A, W: AsRef<RetainedWaveform>>(
        self,
        normalize_live: bool,
        restore_analysis: impl FnMut(ProjectAnalysisResult) -> Result<A, String>,
    ) -> Result<SimulationRun<A>, String>
    where
        A: AsRef<AnalysisResult<W>> + AsMut<AnalysisResult<W>>,
    {
        let campaign_membership = self.campaign_membership.clone();
        let specification_verdicts = self.specification_verdicts.clone();
        let run_id = self
            .run_id
            .ok_or_else(|| format!("simulation run sequence {} has no stable id", self.id))?;
        let dataset_id = self
            .dataset_id
            .ok_or_else(|| format!("simulation run sequence {} has no dataset id", self.id))?;
        let mut analyses = self
            .analyses
            .into_iter()
            .map(restore_analysis)
            .collect::<Result<Vec<_>, _>>()?;
        if self.dataset_content_digest.is_null() {
            return Err(format!(
                "simulation run sequence {} has an explicitly null dataset content digest",
                self.id
            ));
        }
        if self.provenance_mode.is_null() {
            return Err(format!(
                "simulation run sequence {} has an explicitly null provenance mode",
                self.id
            ));
        }
        if self.prepared_receipt.is_null() {
            return Err(format!(
                "simulation run sequence {} has an explicitly null prepared receipt",
                self.id
            ));
        }
        let provenance = match self.provenance_mode.into_value() {
            Some(ProjectRunProvenanceMode::LegacyUnattributed) => {
                if self.prepared_receipt.is_present() {
                    return Err(format!(
                        "simulation run sequence {} is legacy-unattributed but carries a prepared receipt",
                        self.id
                    ));
                }
                SimulationRunProvenance::LegacyUnattributed
            }
            Some(ProjectRunProvenanceMode::LegacyPreparedUnclassified) => {
                if self.prepared_receipt.is_present() {
                    return Err(format!(
                        "simulation run sequence {} is legacy prepared-unclassified but carries a current prepared receipt",
                        self.id
                    ));
                }
                SimulationRunProvenance::LegacyPreparedUnclassified
            }
            Some(ProjectRunProvenanceMode::PreparedTaskBound) => {
                SimulationRunProvenance::Prepared(Box::new(
                    self.prepared_receipt
                        .into_value()
                        .ok_or_else(|| {
                            format!(
                                "simulation run sequence {} is prepared-task-bound but has no receipt",
                                self.id
                            )
                        })?
                        .into_receipt()?,
                ))
            }
            Some(ProjectRunProvenanceMode::Unspecified) | None => {
                return Err(format!(
                    "simulation run sequence {} has no authoritative provenance mode",
                    self.id
                ));
            }
        };
        let mut run = SimulationRun::from_persisted_identity(
            self.id,
            self.label,
            self.timestamp,
            run_id,
            dataset_id,
            self.job_id,
            self.execution_target,
        );
        let restored_lifecycle = self
            .lifecycle
            .unwrap_or(SimulationRunLifecycle::LegacyUnknown);
        let was_active = matches!(
            restored_lifecycle,
            SimulationRunLifecycle::Preparing
                | SimulationRunLifecycle::Running
                | SimulationRunLifecycle::Cancelling
        );
        if was_active && normalize_live {
            for analysis in &mut analyses {
                let analysis = analysis.as_mut();
                if analysis.is_live_partial() {
                    analysis.error_message = Some(if analysis.monte_carlo_checkpoint.is_some() {
                        "Monte Carlo was interrupted; committed trials are retained in its checkpoint"
                    } else if analysis.transient_checkpoint.is_some() {
                        "Transient was interrupted; accepted state is retained in its checkpoint"
                    } else {
                        "Simulation was interrupted before completion; retained waveforms are accepted partial samples"
                    }.to_owned());
                }
            }
        }
        run.restore_lifecycle(restored_lifecycle, self.elapsed_time)?;
        run.analyses = analyses;
        run.log = self.log;
        run.set_retention(self.retention);
        run.success = !was_active && self.success;
        run.restore_campaign_membership(campaign_membership)?;
        run.restore_provenance(provenance)?;
        if specification_verdicts.is_none() && was_active && run.prepared_receipt().is_some() {
            run.seal_interrupted_specification_verdicts()?;
        } else {
            run.restore_specification_verdicts(specification_verdicts)?;
        }
        Ok(run)
    }

    fn validate(&self, run_idx: usize) -> Result<(), String> {
        if let Some(membership) = &self.campaign_membership {
            membership
                .validate()
                .map_err(|error| format!("runs[{run_idx}].campaign_membership: {error}"))?;
        }
        require_finite(self.timestamp, &format!("runs[{run_idx}].timestamp"))?;
        SimulationRun::<AnalysisResult>::validate_elapsed_time(self.elapsed_time)
            .map_err(|error| format!("runs[{run_idx}].elapsed_time: {error}"))?;
        let lifecycle = self.lifecycle.ok_or_else(|| {
            format!("runs[{run_idx}].lifecycle is required by simulation results schema v6")
        })?;
        match lifecycle {
            SimulationRunLifecycle::LegacyUnknown => {
                if self.job_id.is_some() || self.execution_target.is_some() {
                    return Err(format!(
                        "runs[{run_idx}] is legacy_unknown but carries current execution job/target identity"
                    ));
                }
            }
            _ => {
                if self.job_id.is_none() {
                    return Err(format!(
                        "runs[{run_idx}].job_id is required for a non-legacy lifecycle"
                    ));
                }
                if self.execution_target.is_none() {
                    return Err(format!(
                        "runs[{run_idx}].execution_target is required for a non-legacy lifecycle"
                    ));
                }
            }
        }
        match lifecycle {
            SimulationRunLifecycle::Completed if !self.success => {
                return Err(format!(
                    "runs[{run_idx}] is completed but its success outcome is false"
                ));
            }
            SimulationRunLifecycle::Preparing
            | SimulationRunLifecycle::Running
            | SimulationRunLifecycle::Cancelling
            | SimulationRunLifecycle::Failed
            | SimulationRunLifecycle::Aborted
            | SimulationRunLifecycle::Interrupted
                if self.success =>
            {
                return Err(format!(
                    "runs[{run_idx}] lifecycle {lifecycle:?} cannot carry a successful outcome"
                ));
            }
            SimulationRunLifecycle::LegacyUnknown
            | SimulationRunLifecycle::Preparing
            | SimulationRunLifecycle::Running
            | SimulationRunLifecycle::Cancelling
            | SimulationRunLifecycle::Completed
            | SimulationRunLifecycle::Failed
            | SimulationRunLifecycle::Aborted
            | SimulationRunLifecycle::Interrupted => {}
        }
        let mut analysis_ids = HashSet::new();
        let provenance_count = self
            .analyses
            .iter()
            .filter(|analysis| analysis.provenance.is_some())
            .count();
        let legacy_domain_count = self
            .analyses
            .iter()
            .filter_map(|analysis| analysis.provenance.as_ref())
            .filter(|provenance| {
                provenance.source_domain.as_ref()
                    == Some(&AnalysisResultSourceDomain::LegacyUnclassified)
            })
            .count();
        if self.provenance_mode.is_null() {
            return Err(format!(
                "runs[{run_idx}].provenance_mode must not be explicitly null"
            ));
        }
        if self.prepared_receipt.is_null() {
            return Err(format!(
                "runs[{run_idx}].prepared_receipt must not be explicitly null"
            ));
        }
        match self.provenance_mode.as_ref().copied() {
            None | Some(ProjectRunProvenanceMode::Unspecified) => {
                return Err(format!(
                    "runs[{run_idx}].provenance_mode is missing from current simulation results"
                ));
            }
            Some(ProjectRunProvenanceMode::LegacyUnattributed) if provenance_count != 0 => {
                return Err(format!(
                    "runs[{run_idx}] is marked legacy_unattributed but contains prepared-task provenance"
                ));
            }
            Some(ProjectRunProvenanceMode::LegacyPreparedUnclassified)
                if provenance_count != self.analyses.len()
                    || legacy_domain_count != self.analyses.len() =>
            {
                return Err(format!(
                    "runs[{run_idx}] is legacy_prepared_unclassified but its prepared source domains are not uniformly legacy-unclassified"
                ));
            }
            Some(ProjectRunProvenanceMode::PreparedTaskBound)
                if provenance_count != self.analyses.len() || legacy_domain_count != 0 =>
            {
                return Err(format!(
                    "runs[{run_idx}] is prepared_task_bound but {} of {} analyses lack provenance",
                    self.analyses.len() - provenance_count,
                    self.analyses.len()
                ));
            }
            Some(ProjectRunProvenanceMode::LegacyUnattributed)
            | Some(ProjectRunProvenanceMode::LegacyPreparedUnclassified)
            | Some(ProjectRunProvenanceMode::PreparedTaskBound) => {}
        }
        match (
            self.provenance_mode.as_ref().copied(),
            self.prepared_receipt.as_ref(),
        ) {
            (Some(ProjectRunProvenanceMode::PreparedTaskBound), Some(receipt)) => {
                receipt.validate()?;
            }
            (Some(ProjectRunProvenanceMode::PreparedTaskBound), None) => {
                return Err(format!(
                    "runs[{run_idx}] is prepared_task_bound but has no authenticated run receipt"
                ));
            }
            (
                Some(
                    ProjectRunProvenanceMode::LegacyUnattributed
                    | ProjectRunProvenanceMode::LegacyPreparedUnclassified,
                ),
                Some(_),
            ) => {
                return Err(format!(
                    "runs[{run_idx}] is legacy but carries a current prepared run receipt"
                ));
            }
            _ => {}
        }

        let mut source_instance_ids = HashSet::new();
        let mut prepared_snapshot_digest = None;
        let mut source_revision = None;
        let mut source_domain = None;
        for (analysis_idx, analysis) in self.analyses.iter().enumerate() {
            if analysis.id == 0 {
                return Err(format!(
                    "runs[{run_idx}].analyses[{analysis_idx}].id must be greater than zero"
                ));
            }
            if !analysis_ids.insert(analysis.id) {
                return Err(format!(
                    "duplicate analysis id {} in runs[{run_idx}]",
                    analysis.id
                ));
            }
            analysis.validate(run_idx, analysis_idx)?;

            if let Some(provenance) = &analysis.provenance {
                let prefix = format!("runs[{run_idx}].analyses[{analysis_idx}].provenance");
                if !source_instance_ids.insert(provenance.source_instance_id) {
                    return Err(format!(
                        "{prefix}.source_instance_id duplicates prepared analysis instance {}",
                        provenance.source_instance_id
                    ));
                }
                match source_domain {
                    Some(expected) if Some(&expected) != provenance.source_domain.as_ref() => {
                        return Err(format!(
                            "{prefix}.source_domain mixes prepared source domains within one run"
                        ));
                    }
                    None => {
                        source_domain =
                            Some(*provenance.source_domain.as_ref().ok_or_else(|| {
                                format!("{prefix}.source_domain is required by schema v5")
                            })?)
                    }
                    Some(_) => {}
                }
                match prepared_snapshot_digest {
                    Some(expected) if expected != provenance.prepared_snapshot_digest => {
                        return Err(format!(
                            "{prefix}.prepared_snapshot_digest does not match the run's frozen snapshot"
                        ));
                    }
                    None => prepared_snapshot_digest = Some(provenance.prepared_snapshot_digest),
                    Some(_) => {}
                }
                match source_revision {
                    Some(expected) if expected != provenance.source_revision => {
                        return Err(format!(
                            "{prefix}.source_revision does not match the run's frozen plan revision"
                        ));
                    }
                    None => source_revision = Some(provenance.source_revision),
                    Some(_) => {}
                }
                for dependency_id in &provenance.dependency_ids {
                    if !source_instance_ids.contains(dependency_id) {
                        return Err(format!(
                            "{prefix}.dependency_ids references {dependency_id} before that result appears in the frozen execution order"
                        ));
                    }
                }
            }
        }
        if let Some(receipt) = self.prepared_receipt.as_ref() {
            let receipt = receipt.clone().into_receipt()?;
            let analyses = self
                .analyses
                .iter()
                .cloned()
                .map(ProjectAnalysisResult::into_analysis)
                .collect::<Result<Vec<_>, _>>()?;
            receipt.validate_result_prefix(analyses.iter())?;
        }
        let retained_digest = self.dataset_content_digest.as_ref().copied().ok_or_else(|| {
            format!("runs[{run_idx}].dataset_content_digest is required by simulation results schema v12")
        })?;
        let computed_digest = self
            .clone()
            // Authenticate the bytes that were written before restoration
            // replaces any live status with an interrupted status.
            .into_run_with_partial_restore(false, ProjectAnalysisResult::into_analysis)
            .map_err(|error| {
                format!("runs[{run_idx}] cannot compute its dataset content digest: {error}")
            })?
            .dataset_content_digest_with_encoding(ResultDigestEncoding::CURRENT);
        if retained_digest != computed_digest {
            return Err(format!(
                "runs[{run_idx}].dataset_content_digest does not match retained analysis content"
            ));
        }
        Ok(())
    }
}

impl ProjectSimulationRun {
    pub fn from_run<A, W: AsRef<RetainedWaveform>>(
        run: &SimulationRun<A>,
        mut waveform: impl FnMut(&W) -> ProjectWaveformData,
    ) -> Self
    where
        A: AsRef<AnalysisResult<W>>,
    {
        let (provenance_mode, prepared_receipt) = match run.provenance() {
            None => (PersistedField::Missing, PersistedField::Missing),
            Some(SimulationRunProvenance::LegacyUnattributed) => (
                PersistedField::Value(ProjectRunProvenanceMode::LegacyUnattributed),
                PersistedField::Missing,
            ),
            Some(SimulationRunProvenance::LegacyPreparedUnclassified) => (
                PersistedField::Value(ProjectRunProvenanceMode::LegacyPreparedUnclassified),
                PersistedField::Missing,
            ),
            Some(SimulationRunProvenance::Prepared(receipt)) => (
                PersistedField::Value(ProjectRunProvenanceMode::PreparedTaskBound),
                PersistedField::Value(ProjectPreparedRunReceipt::from(receipt.as_ref())),
            ),
        };
        let success = if matches!(
            run.lifecycle,
            SimulationRunLifecycle::Preparing
                | SimulationRunLifecycle::Running
                | SimulationRunLifecycle::Cancelling
        ) {
            false
        } else {
            run.success
        };
        Self {
            job_id: run.job_id,
            run_id: Some(run.run_id),
            dataset_id: Some(run.dataset_id),
            execution_target: run.execution_target,
            lifecycle: Some(run.lifecycle),
            id: run.id,
            label: run.label.clone(),
            timestamp: run.timestamp,
            analyses: run
                .analyses
                .iter()
                .map(|analysis| {
                    ProjectAnalysisResult::from_analysis(analysis.as_ref(), &mut waveform)
                })
                .collect(),
            dataset_content_digest: PersistedField::Value(
                run.dataset_content_digest_with_encoding(ResultDigestEncoding::CURRENT),
            ),
            provenance_mode,
            prepared_receipt,
            campaign_membership: run.campaign_membership().cloned(),
            retention: run.retention(),
            elapsed_time: run.elapsed_time,
            success,
            specification_verdicts: run.specification_verdicts().map(<[_]>::to_vec),
            log: run.log.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectAnalysisResult {
    /// Run-local display sequence retained for labels and v1 migration.
    pub id: u64,
    pub analysis_type: String,
    pub label: String,
    pub timestamp: f64,
    /// Canonical identity of authoritative result samples and evidence.
    /// Schemas through v7 omitted this field and acquire it during migration.
    #[serde(default, skip_serializing_if = "PersistedField::is_missing")]
    pub result_data_digest: PersistedField<ContentDigest>,
    #[serde(default)]
    pub waveforms: Vec<ProjectWaveformData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dc_op: Option<ProjectDcOpResult>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub device_op: Option<ProjectDeviceOpReport>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub noise_summary: Option<ProjectNoiseSummary>,
    /// Exact source metadata for multi-run and advanced analysis families.
    /// Result schemas through v6 omitted this field and migrate to `None`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub family_metadata: Option<AnalysisResultFamilyMetadata>,
    /// Exact analysis-native result evidence. Schemas through v8 omitted this
    /// field; presence-aware persistence prevents a relabeled legacy document
    /// from injecting current-schema evidence.
    #[serde(default, skip_serializing_if = "PersistedField::is_missing")]
    pub result_payload: PersistedField<AnalysisResultPayload>,
    /// Producer units for native scalars, introduced in result schema v38.
    #[serde(default, skip_serializing_if = "PersistedField::is_missing")]
    pub native_scalar_units:
        PersistedField<std::collections::BTreeMap<String, rspice_core::analysis::MeasurementUnit>>,
    /// Exact portable trial journal, introduced in result schema v35.
    #[serde(default, skip_serializing_if = "PersistedField::is_missing")]
    pub monte_carlo_checkpoint:
        PersistedField<rspice_results::monte_carlo_checkpoint::MonteCarloCheckpointEvidence>,
    /// Accepted transient continuation state, introduced in result schema v44.
    #[serde(default, skip_serializing_if = "PersistedField::is_missing")]
    pub transient_checkpoint:
        PersistedField<rspice_results::transient_checkpoint::TransientCheckpointEvidence>,
    #[serde(default)]
    pub measurements: Vec<ProjectMeasurement>,
    /// The run's checks (HDL severity-task records and `.ASSERT` reports),
    /// introduced in result schema v45. A result without checks omits it.
    #[serde(default, skip_serializing_if = "PersistedField::is_missing")]
    pub checks: PersistedField<ProjectRunChecks>,
    /// Authenticated outcomes for the immutable saved-output contracts that
    /// applied to this analysis. Older project files legitimately omit it.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub saved_output_receipts: Vec<SavedOutputReceipt>,
    #[serde(default = "default_true")]
    pub success: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// The design objects the engine named for this failure. Every schema
    /// written before the engine could name them omits the field, which
    /// reads back as the honest "it named none".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failure_attribution:
        Option<rspice_results::convergence_attribution::ConvergenceAttribution>,
    /// The capability token of a refused analysis. Every schema written
    /// before refusals were recorded omits it, which reads back as "not a
    /// refusal" — the message still carries the token in brackets.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failure_capability: Option<String>,
    /// Numerical quality introduced by result schema v21. Missing stays unknown.
    #[serde(default, skip_serializing_if = "PersistedField::is_missing")]
    pub convergence:
        PersistedField<rspice_results::convergence_quality::TransientConvergenceEvidence>,
    /// Complete source identity for prepared-task results. `None` is retained only
    /// when loading result history written by v1/v2 projects.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provenance: Option<ProjectAnalysisResultProvenance>,
    /// Detected external file attribution, introduced in result schema v20.
    /// Missing means unknown; explicit null is invalid current evidence.
    #[serde(default, skip_serializing_if = "PersistedField::is_missing")]
    pub import_source: PersistedField<rspice_results::result_import::ResultImportSource>,
}

/// Project-file representation of one frozen prepared analysis task.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProjectAnalysisResultProvenance {
    #[serde(default, skip_serializing_if = "PersistedField::is_missing")]
    pub source_domain: PersistedField<AnalysisResultSourceDomain>,
    pub source_instance_id: AnalysisInstanceId,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authored_source_instance_id: Option<AnalysisInstanceId>,
    pub source_revision: ObjectRevision,
    pub prepared_snapshot_digest: ContentDigest,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dependency_ids: Vec<AnalysisInstanceId>,
    /// The PVT point the producing task was expanded to. Absent both for
    /// projects written before results were attributed and for a task that
    /// legitimately has no point; the two are indistinguishable here on
    /// purpose, because neither may be read as evidence about a corner.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pvt_point: Option<ProjectAnalysisResultPvtPoint>,
    /// The transient state a small-signal result was linearized around
    /// (`ACTIMES=` or `CHECKPOINT=`). Absent for every result linearized at
    /// the operating point, which is every result written before it existed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transient_state: Option<AnalysisResultTransientState>,
    /// The Monte Carlo trial or `.STEP` point a transient result solved.
    /// Absent for every result that solved its deck as it stands, which is
    /// every result written before members were recorded.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub family_member: Option<AnalysisResultFamilyMember>,
}

/// Project-file representation of one attributed PVT point.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProjectAnalysisResultPvtPoint {
    pub process: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub supply_voltage: Option<f64>,
    pub temperature_celsius: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub corner_contract: Option<ContentDigest>,
    pub nominal: bool,
}

/// Bitwise, so a persisted point is `Eq` like the rest of the provenance wire
/// record. Load-time validation refuses the non-finite quantities that would
/// make this anything other than an equivalence.
impl PartialEq for ProjectAnalysisResultPvtPoint {
    fn eq(&self, other: &Self) -> bool {
        self.process == other.process
            && self.supply_voltage.map(f64::to_bits) == other.supply_voltage.map(f64::to_bits)
            && self.temperature_celsius.to_bits() == other.temperature_celsius.to_bits()
            && self.corner_contract == other.corner_contract
            && self.nominal == other.nominal
    }
}

impl Eq for ProjectAnalysisResultPvtPoint {}

impl ProjectAnalysisResultPvtPoint {
    fn into_point(self) -> Result<AnalysisResultPvtPoint, String> {
        AnalysisResultPvtPoint::new(
            self.process,
            self.supply_voltage,
            self.temperature_celsius,
            self.corner_contract,
            self.nominal,
        )
    }
}

impl From<&AnalysisResultPvtPoint> for ProjectAnalysisResultPvtPoint {
    fn from(point: &AnalysisResultPvtPoint) -> Self {
        Self {
            process: point.process().to_owned(),
            supply_voltage: point.supply_voltage(),
            temperature_celsius: point.temperature_celsius(),
            corner_contract: point.corner_contract(),
            nominal: point.is_nominal(),
        }
    }
}

impl ProjectAnalysisResultProvenance {
    fn into_provenance(self) -> Result<AnalysisResultProvenance, String> {
        let pvt_point = self
            .pvt_point
            .map(ProjectAnalysisResultPvtPoint::into_point)
            .transpose()?;
        AnalysisResultProvenance::new_with_authored_source_domain(
            self.source_domain
                .into_value()
                .ok_or_else(|| "prepared result source_domain is missing or null".to_owned())?,
            self.source_instance_id,
            self.authored_source_instance_id
                .unwrap_or(self.source_instance_id),
            self.source_revision,
            self.prepared_snapshot_digest,
            self.dependency_ids,
        )?
        .with_pvt_point(pvt_point)
        .with_transient_state(self.transient_state)?
        .with_family_member(self.family_member)
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(point) = self.pvt_point.clone() {
            point.into_point()?;
        }
        AnalysisResultProvenance::new_with_authored_source_domain(
            self.source_domain
                .as_ref()
                .copied()
                .ok_or_else(|| "prepared result source_domain is missing or null".to_owned())?,
            self.source_instance_id,
            self.authored_source_instance_id
                .unwrap_or(self.source_instance_id),
            self.source_revision,
            self.prepared_snapshot_digest,
            self.dependency_ids.clone(),
        )?
        .with_transient_state(self.transient_state)?
        .with_family_member(self.family_member.clone())
        .map(|_| ())
    }
}

impl From<&AnalysisResultProvenance> for ProjectAnalysisResultProvenance {
    fn from(provenance: &AnalysisResultProvenance) -> Self {
        Self {
            source_domain: PersistedField::Value(provenance.source_domain()),
            source_instance_id: provenance.source_instance_id(),
            authored_source_instance_id: (provenance.authored_source_instance_id()
                != provenance.source_instance_id())
            .then_some(provenance.authored_source_instance_id()),
            source_revision: provenance.source_revision(),
            prepared_snapshot_digest: provenance.prepared_snapshot_digest(),
            dependency_ids: provenance.dependency_ids().to_vec(),
            pvt_point: provenance
                .pvt_point()
                .map(ProjectAnalysisResultPvtPoint::from),
            transient_state: provenance.transient_state().copied(),
            family_member: provenance.family_member().cloned(),
        }
    }
}

impl ProjectAnalysisResult {
    pub fn into_analysis(self) -> Result<AnalysisResult, String> {
        self.into_analysis_with_waveforms(ProjectWaveformData::into_waveform)
    }

    pub fn into_analysis_with_waveforms<W: AsRef<RetainedWaveform>>(
        self,
        waveform: impl FnMut(ProjectWaveformData) -> W,
    ) -> Result<AnalysisResult<W>, String> {
        if self.native_scalar_units.is_null() {
            return Err(format!(
                "analysis sequence {} has explicitly null native scalar units",
                self.id
            ));
        }
        if self.transient_checkpoint.is_null() {
            return Err(format!(
                "analysis sequence {} has an explicitly null transient checkpoint",
                self.id
            ));
        }
        if self.checks.is_null() {
            return Err(format!(
                "analysis sequence {} has explicitly null checks",
                self.id
            ));
        }
        if self.monte_carlo_checkpoint.is_null() {
            return Err(format!(
                "analysis sequence {} has an explicitly null Monte Carlo checkpoint",
                self.id
            ));
        }
        if self.convergence.is_null() {
            return Err(format!(
                "analysis sequence {} has explicitly null convergence evidence",
                self.id
            ));
        }
        if let Some(quality) = self.convergence.as_ref() {
            quality.validate()?;
        }
        if self.result_data_digest.is_null() {
            return Err(format!(
                "analysis sequence {} has an explicitly null result data digest",
                self.id
            ));
        }
        if self.import_source.is_null() {
            return Err(format!(
                "analysis sequence {} has an explicitly null import source",
                self.id
            ));
        }
        if self.result_payload.is_null() {
            return Err(format!(
                "analysis sequence {} has an explicitly null result payload",
                self.id
            ));
        }
        let analysis_type = analysis_type_from_key(&self.analysis_type)
            .ok_or_else(|| format!("unknown persisted analysis type '{}'", self.analysis_type))?;
        let provenance = self
            .provenance
            .map(ProjectAnalysisResultProvenance::into_provenance)
            .transpose()?;
        let checks = self
            .checks
            .into_value()
            .map(ProjectRunChecks::into_checks)
            .transpose()?
            .unwrap_or_default();
        let analysis = AnalysisResult {
            id: self.id,
            analysis_type,
            label: self.label,
            timestamp: self.timestamp,
            waveforms: self.waveforms.into_iter().map(waveform).collect(),
            dc_op: self.dc_op.map(ProjectDcOpResult::into_dc_op),
            device_op: self.device_op.map(ProjectDeviceOpReport::into_report),
            noise_summary: self
                .noise_summary
                .map(ProjectNoiseSummary::into_noise_summary),
            family_metadata: self.family_metadata,
            result_payload: self.result_payload.into_value(),
            native_scalar_units: self.native_scalar_units.into_value(),
            monte_carlo_checkpoint: self.monte_carlo_checkpoint.into_value(),
            transient_checkpoint: self.transient_checkpoint.into_value(),
            measurements: self
                .measurements
                .into_iter()
                .map(ProjectMeasurement::into_measurement)
                .collect(),
            checks,
            saved_output_receipts: self.saved_output_receipts,
            success: self.success,
            error_message: self.error_message,
            failure_attribution: self.failure_attribution,
            failure_capability: self.failure_capability,
            convergence: self.convergence.into_value().map(std::sync::Arc::new),
            provenance,
            import_source: self.import_source.into_value(),
        };
        Ok(analysis)
    }

    fn validate(&self, run_idx: usize, analysis_idx: usize) -> Result<(), String> {
        let prefix = format!("runs[{run_idx}].analyses[{analysis_idx}]");
        if self.convergence.is_null() {
            return Err(format!("{prefix}.convergence must not be null"));
        }
        if let Some(quality) = self.convergence.as_ref() {
            quality
                .validate()
                .map_err(|error| format!("{prefix}.convergence is invalid: {error}"))?;
        }
        if analysis_type_from_key(&self.analysis_type).is_none() {
            return Err(format!(
                "{prefix}.analysis_type has unknown analysis type '{}'",
                self.analysis_type
            ));
        }
        require_finite(self.timestamp, &format!("{prefix}.timestamp"))?;
        let mut waveform_names = HashSet::new();
        for (waveform_idx, waveform) in self.waveforms.iter().enumerate() {
            if !waveform_names.insert(waveform.name.as_str()) {
                return Err(format!(
                    "{prefix} has duplicate waveform name '{}'",
                    waveform.name
                ));
            }
            waveform.validate(&format!("{prefix}.waveforms[{waveform_idx}]"))?;
        }
        if let Some(dc_op) = &self.dc_op {
            dc_op.validate(&format!("{prefix}.dc_op"))?;
        }
        if let Some(device_op) = &self.device_op {
            device_op.validate(&format!("{prefix}.device_op"))?;
        }
        if let Some(noise_summary) = &self.noise_summary {
            noise_summary.validate(&format!("{prefix}.noise_summary"))?;
        }
        if let Some(metadata) = &self.family_metadata {
            metadata
                .validate_for(
                    analysis_type_from_key(&self.analysis_type)
                        .expect("analysis type was checked above"),
                )
                .map_err(|error| format!("{prefix}.family_metadata is invalid: {error}"))?;
        }
        if self.result_payload.is_null() {
            return Err(format!("{prefix}.result_payload must not be null"));
        }
        if let Some(payload) = self.result_payload.as_ref() {
            payload
                .validate_for(
                    analysis_type_from_key(&self.analysis_type)
                        .expect("analysis type was checked above"),
                )
                .map_err(|error| format!("{prefix}.result_payload is invalid: {error}"))?;
            if let AnalysisResultPayload::DcSweep { evidence } = payload {
                evidence
                    .validate_retained_traces(self.waveforms.iter().map(|trace| {
                        rspice_results::dc_sweep::DcTraceView {
                            name: &trace.name,
                            unit: trace.unit.as_deref(),
                            x: &trace.x,
                            sample_count: trace.y.len(),
                            complex: trace.complex.is_some(),
                        }
                    }))
                    .map_err(|error| format!("{prefix}.result_payload is invalid: {error}"))?;
            }
            if !self.success {
                return Err(format!(
                    "{prefix}.result_payload is not permitted on a failed analysis"
                ));
            }
        }
        let analysis_type =
            analysis_type_from_key(&self.analysis_type).expect("analysis type was checked above");
        if self.success {
            let has_required_periodic_payload = match analysis_type {
                AnalysisType::Pss => matches!(
                    self.result_payload.as_ref(),
                    Some(AnalysisResultPayload::PssFloquet { .. })
                ),
                AnalysisType::Pstb => matches!(
                    self.result_payload.as_ref(),
                    Some(AnalysisResultPayload::Pstb { .. })
                ),
                _ => true,
            };
            if !has_required_periodic_payload {
                return Err(format!(
                    "{prefix}.result_payload is required and must match successful periodic analysis type {analysis_type:?}"
                ));
            }
        }
        self.clone()
            .into_analysis()?
            .validate_retained_evidence()
            .map_err(|error| format!("{prefix}.retained evidence is invalid: {error}"))?;
        for (measurement_idx, measurement) in self.measurements.iter().enumerate() {
            measurement.validate(&format!("{prefix}.measurements[{measurement_idx}]"))?;
        }
        if let Some(checks) = self.checks.as_ref() {
            checks.validate(&format!("{prefix}.checks"))?;
        }
        if let Some(provenance) = &self.provenance {
            provenance
                .validate()
                .map_err(|error| format!("{prefix}.provenance is invalid: {error}"))?;
        }
        let retained_digest = self.result_data_digest.as_ref().copied().ok_or_else(|| {
            format!("{prefix}.result_data_digest is required by simulation results schema v12")
        })?;
        let analysis = self
            .clone()
            .into_analysis()
            .map_err(|error| format!("{prefix} cannot compute its result data digest: {error}"))?;
        analysis
            .validate_retained_evidence()
            .map_err(|error| format!("{prefix} retained evidence is inconsistent: {error}"))?;
        let computed_digest = analysis
            .result_data_ref()
            .digest(ResultDigestEncoding::CURRENT);
        if retained_digest != computed_digest {
            return Err(format!(
                "{prefix}.result_data_digest does not match retained analysis content"
            ));
        }
        Ok(())
    }
}

impl ProjectAnalysisResult {
    pub fn from_analysis<W: AsRef<RetainedWaveform>>(
        analysis: &AnalysisResult<W>,
        waveform: impl FnMut(&W) -> ProjectWaveformData,
    ) -> Self {
        Self {
            id: analysis.id,
            analysis_type: analysis_type_key(analysis.analysis_type).to_string(),
            label: analysis.label.clone(),
            timestamp: analysis.timestamp,
            result_data_digest: PersistedField::Value(
                analysis
                    .result_data_ref()
                    .digest(ResultDigestEncoding::CURRENT),
            ),
            waveforms: analysis.waveforms.iter().map(waveform).collect(),
            dc_op: analysis.dc_op.as_ref().map(ProjectDcOpResult::from),
            device_op: analysis.device_op.as_ref().map(ProjectDeviceOpReport::from),
            noise_summary: analysis
                .noise_summary
                .as_ref()
                .map(ProjectNoiseSummary::from),
            family_metadata: analysis.family_metadata.clone(),
            native_scalar_units: analysis
                .native_scalar_units
                .clone()
                .map_or(PersistedField::Missing, PersistedField::Value),
            result_payload: analysis
                .result_payload
                .clone()
                .map_or(PersistedField::Missing, PersistedField::Value),
            transient_checkpoint: analysis
                .transient_checkpoint
                .clone()
                .map_or(PersistedField::Missing, PersistedField::Value),
            monte_carlo_checkpoint: analysis
                .monte_carlo_checkpoint
                .clone()
                .map_or(PersistedField::Missing, PersistedField::Value),
            measurements: analysis
                .measurements
                .iter()
                .map(ProjectMeasurement::from)
                .collect(),
            checks: if analysis.checks.is_empty() {
                PersistedField::Missing
            } else {
                PersistedField::Value(ProjectRunChecks::from(&analysis.checks))
            },
            saved_output_receipts: analysis.saved_output_receipts.clone(),
            success: analysis.success,
            error_message: analysis.error_message.clone(),
            failure_attribution: analysis.failure_attribution.clone(),
            failure_capability: analysis.failure_capability.clone(),
            convergence: analysis
                .convergence
                .as_deref()
                .cloned()
                .map_or(PersistedField::Missing, PersistedField::Value),
            import_source: analysis
                .import_source
                .clone()
                .map_or(PersistedField::Missing, PersistedField::Value),
            provenance: analysis
                .provenance
                .as_ref()
                .map(ProjectAnalysisResultProvenance::from),
        }
    }
}

/// Presence-aware persisted field used at schema-era boundaries.
///
/// Serde's `Option<T>` intentionally treats an explicit JSON `null` exactly
/// like an absent field. That is unsafe for fields introduced by a later
/// schema because a caller could otherwise add the field as `null`, relabel
/// the container as an older schema, and have migration accept it as genuine
/// absence. This representation keeps all three wire states distinct.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum PersistedField<T> {
    #[default]
    Missing,
    Null,
    Value(T),
}

impl<T> PersistedField<T> {
    #[must_use]
    pub const fn is_missing(&self) -> bool {
        matches!(self, Self::Missing)
    }

    #[must_use]
    pub const fn is_null(&self) -> bool {
        matches!(self, Self::Null)
    }

    #[must_use]
    pub const fn is_present(&self) -> bool {
        !self.is_missing()
    }

    #[must_use]
    pub const fn as_ref(&self) -> Option<&T> {
        match self {
            Self::Value(value) => Some(value),
            Self::Missing | Self::Null => None,
        }
    }

    #[must_use]
    pub fn as_mut(&mut self) -> Option<&mut T> {
        match self {
            Self::Value(value) => Some(value),
            Self::Missing | Self::Null => None,
        }
    }

    #[must_use]
    pub fn into_value(self) -> Option<T> {
        match self {
            Self::Value(value) => Some(value),
            Self::Missing | Self::Null => None,
        }
    }
}

impl<T: Serialize> Serialize for PersistedField<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // `Some`, the inverse of the `Option` this is read through. JSON writes
        // a bare value the same way, but RON, the format of the saved working
        // session, does not, and refused every session that held a result.
        match self {
            Self::Value(value) => serializer.serialize_some(value),
            Self::Missing | Self::Null => serializer.serialize_none(),
        }
    }
}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for PersistedField<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Option::<T>::deserialize(deserializer).map(|value| match value {
            Some(value) => Self::Value(value),
            None => Self::Null,
        })
    }
}

/// The result-history schema this build writes, and the only one it reads.
const PROJECT_SIMULATION_RESULTS_SCHEMA_VERSION: u32 = 45;

fn analysis_type_key(analysis_type: AnalysisType) -> &'static str {
    match analysis_type {
        AnalysisType::DcOp => "DcOp",
        AnalysisType::DcSweep => "DcSweep",
        AnalysisType::Ac => "Ac",
        AnalysisType::Disto => "Disto",
        AnalysisType::Transient => "Transient",
        AnalysisType::Noise => "Noise",
        AnalysisType::PoleZero => "PoleZero",
        AnalysisType::Tf => "Tf",
        AnalysisType::Sensitivity => "Sensitivity",
        AnalysisType::Pac => "Pac",
        AnalysisType::Pnoise => "Pnoise",
        AnalysisType::Pxf => "Pxf",
        AnalysisType::Pstb => "Pstb",
        AnalysisType::Stb => "Stb",
        AnalysisType::MonteCarlo => "MonteCarlo",
        AnalysisType::Parametric => "Parametric",
        AnalysisType::Corner => "Corner",
        AnalysisType::Optimization => "Optimization",
        AnalysisType::Soa => "Soa",
        AnalysisType::SParameter => "SParameter",
        AnalysisType::Envelope => "Envelope",
        AnalysisType::Fourier => "Fourier",
        AnalysisType::HarmonicBalance => "HarmonicBalance",
        AnalysisType::Pss => "Pss",
        AnalysisType::Qpss => "Qpss",
        AnalysisType::Hbsp => "Hbsp",
        AnalysisType::Hbnoise => "Hbnoise",
        AnalysisType::Psp => "Psp",
        AnalysisType::Qpac => "Qpac",
        AnalysisType::Qpnoise => "Qpnoise",
        AnalysisType::Qpxf => "Qpxf",
        AnalysisType::TransientNoise => "TransientNoise",
        AnalysisType::DcMismatch => "DcMismatch",
    }
}

fn analysis_type_from_key(key: &str) -> Option<AnalysisType> {
    match key {
        "DcOp" | ".op" => Some(AnalysisType::DcOp),
        "DcSweep" | ".dc" => Some(AnalysisType::DcSweep),
        "Ac" | ".ac" => Some(AnalysisType::Ac),
        "Disto" | ".disto" => Some(AnalysisType::Disto),
        "Transient" | ".tran" => Some(AnalysisType::Transient),
        "Noise" | ".noise" => Some(AnalysisType::Noise),
        "PoleZero" | ".pz" => Some(AnalysisType::PoleZero),
        "Tf" | ".tf" => Some(AnalysisType::Tf),
        "Sensitivity" | ".sens" => Some(AnalysisType::Sensitivity),
        "Pac" | ".pac" => Some(AnalysisType::Pac),
        "Pnoise" | ".pnoise" => Some(AnalysisType::Pnoise),
        "Pxf" | ".pxf" => Some(AnalysisType::Pxf),
        "Pstb" | ".pstb" => Some(AnalysisType::Pstb),
        "Stb" | ".stb" => Some(AnalysisType::Stb),
        "MonteCarlo" | ".mc" => Some(AnalysisType::MonteCarlo),
        "Parametric" | ".step" => Some(AnalysisType::Parametric),
        "Corner" => Some(AnalysisType::Corner),
        "Optimization" | ".opt" => Some(AnalysisType::Optimization),
        "Soa" | ".soa" => Some(AnalysisType::Soa),
        "SParameter" | ".sp" => Some(AnalysisType::SParameter),
        "Envelope" | ".envlp" => Some(AnalysisType::Envelope),
        "Fourier" | ".four" => Some(AnalysisType::Fourier),
        "HarmonicBalance" | ".hb" => Some(AnalysisType::HarmonicBalance),
        "Pss" | ".pss" => Some(AnalysisType::Pss),
        "Qpss" | ".qpss" => Some(AnalysisType::Qpss),
        "Hbsp" | ".hbsp" => Some(AnalysisType::Hbsp),
        "Hbnoise" | ".hbnoise" => Some(AnalysisType::Hbnoise),
        "Psp" | ".psp" => Some(AnalysisType::Psp),
        "Qpac" | ".qpac" => Some(AnalysisType::Qpac),
        "Qpnoise" | ".qpnoise" => Some(AnalysisType::Qpnoise),
        "Qpxf" | ".qpxf" => Some(AnalysisType::Qpxf),
        "TransientNoise" | ".tnoise" => Some(AnalysisType::TransientNoise),
        "DcMismatch" | ".dcmatch" => Some(AnalysisType::DcMismatch),
        _ => None,
    }
}

fn default_true() -> bool {
    true
}

fn require_finite(value: f64, field: &str) -> Result<(), String> {
    if value.is_finite() {
        Ok(())
    } else {
        Err(format!("{field} is not finite"))
    }
}

fn require_optional_finite(value: Option<f64>, field: &str) -> Result<(), String> {
    if let Some(value) = value {
        require_finite(value, field)?;
    }
    Ok(())
}

fn require_finite_values(values: &[f64], field: &str) -> Result<(), String> {
    for (idx, value) in values.iter().enumerate() {
        if !value.is_finite() {
            return Err(format!("{field}[{idx}] is not finite"));
        }
    }
    Ok(())
}

fn require_monotonic_non_decreasing(values: &[f64], field: &str) -> Result<(), String> {
    for (idx, pair) in values.windows(2).enumerate() {
        if pair[1] < pair[0] {
            return Err(format!(
                "{field} must be monotonic non-decreasing; sample {} ({}) is less than sample {} ({})",
                idx + 1,
                pair[1],
                idx,
                pair[0]
            ));
        }
    }
    Ok(())
}

fn require_static_label(value: &str, field: &str) -> Result<(), String> {
    if known_static_label(value).is_some() {
        Ok(())
    } else {
        Err(format!("{field} has unknown static label '{value}'"))
    }
}

/// A noise mechanism is checked for shape rather than membership.
///
/// It is not a static label: the summary keeps it as owned text on both sides
/// of the file, so nothing is interned and no vocabulary has to cover it. Nor
/// could one — half the mechanisms are composed by the Verilog-A code
/// generator out of a model's own node and label names, and no build can list
/// what a future catalog will contain. The engine owns the shape those names
/// keep, so it is asked.
fn require_noise_mechanism(value: &str, field: &str) -> Result<(), String> {
    if rspice_core::analysis::is_persistable_noise_mechanism(value) {
        Ok(())
    } else {
        Err(format!(
            "{field} is not a noise mechanism: '{}'",
            value.chars().take(64).collect::<String>()
        ))
    }
}

/// Intern a persisted label for restoration.
///
/// Restoration is only ever reached through a document that already validated,
/// and validation rejects a label this predicate cannot resolve, so the
/// placeholder is what makes the function total rather than a decision to
/// accept unrecognized text.
fn intern_static_label(value: String) -> &'static str {
    known_static_label(&value).unwrap_or("unknown")
}

/// Labels a persisted result may carry, interned back to the `&'static str`
/// the running build uses.
///
/// The vocabulary belongs to the engine, which is the only thing that knows
/// what its device families report; asking it keeps the reader in step with
/// the emitter instead of restating its list here.
fn known_static_label(value: &str) -> Option<&'static str> {
    rspice_core::circuit::resolve_op_label(value)
}

#[cfg(test)]
mod tests;
