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

use super::*;

mod executed_deck;
mod legacy_digests;
mod provenance;
pub use executed_deck::ProjectExecutedDecks;
// The rows inside are named only where a file is written by hand: production
// code builds them from the session archive and reads them back through it.
use executed_deck::reject_executed_decks_before_schema_v15;
#[cfg(test)]
pub use executed_deck::{ProjectExecutedDeck, ProjectExecutedDeckPoint};
use legacy_digests::{
    validate_v8_result_digests, validate_v9_result_digests, validate_v10_result_digests,
    validate_v11_result_digests, validate_v12_result_digests, validate_v13_to_v15_result_digests,
    validate_v16_result_digests, validate_v17_result_digests, validate_v18_result_digests,
};
pub use provenance::*;
use provenance::{
    migrate_legacy_specification_receipts, reject_derived_task_identities_before_schema_v15,
    reject_hierarchy_maps_before_schema_v15, set_legacy_unclassified_source_domains,
};

/// Stable project-file representation of result history.
///
/// This is intentionally separate from `SimulationState`: project files should
/// persist user-visible result data, not transient runner flags, progress text,
/// or UI trigger bits.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectSimulationResults {
    #[serde(default = "default_simulation_results_schema_version")]
    pub schema_version: u32,
    #[serde(default)]
    pub runs: Vec<ProjectSimulationRun>,
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
    /// Legacy v1 display-sequence selection; consumed during migration and
    /// never written by a current project.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active_run_id: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active_analysis_id: Option<u64>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub overlay_run_ids: Vec<u64>,
}

impl Default for ProjectSimulationResults {
    fn default() -> Self {
        Self {
            schema_version: PROJECT_SIMULATION_RESULTS_SCHEMA_VERSION,
            runs: Vec::new(),
            next_run_id: 0,
            retained_dataset_limit: None,
            active_run_stable_id: None,
            active_dataset_id: None,
            active_analysis_sequence: None,
            overlay_dataset_ids: Vec::new(),
            executed_decks: ProjectExecutedDecks::default(),
            active_run_id: None,
            active_analysis_id: None,
            overlay_run_ids: Vec::new(),
        }
    }
}

impl ProjectSimulationResults {
    pub fn is_empty(&self) -> bool {
        self.runs.is_empty()
            && self.next_run_id == 0
            && self.active_run_stable_id.is_none()
            && self.active_dataset_id.is_none()
            && self.active_analysis_sequence.is_none()
            && self.overlay_dataset_ids.is_empty()
            && self.executed_decks.is_empty()
            && self.active_run_id.is_none()
            && self.active_analysis_id.is_none()
            && self.overlay_run_ids.is_empty()
            && self.retained_dataset_limit.is_none()
    }

    pub fn from_state(state: &SimulationState) -> Self {
        if state.runs.is_empty() {
            // A project with no results can still carry a retention decision
            // the reader made; dropping it here would silently reset the
            // policy the moment the history was cleared.
            return Self {
                retained_dataset_limit: state.retained_dataset_limit,
                ..Self::default()
            };
        }

        let runs: Vec<_> = state.runs.iter().map(ProjectSimulationRun::from).collect();
        let max_run_id = state.runs.iter().map(|run| run.id).max().unwrap_or(0);
        Self {
            schema_version: PROJECT_SIMULATION_RESULTS_SCHEMA_VERSION,
            runs,
            next_run_id: state.next_run_id.max(max_run_id),
            retained_dataset_limit: state.retained_dataset_limit,
            active_run_stable_id: state.active_run().map(|run| run.run_id),
            active_dataset_id: state.active_run().map(|run| run.dataset_id),
            active_analysis_sequence: state.active_analysis().map(|analysis| analysis.id),
            overlay_dataset_ids: state.overlay_dataset_ids.clone(),
            executed_decks: ProjectExecutedDecks::from_state(state),
            active_run_id: None,
            active_analysis_id: None,
            overlay_run_ids: Vec::new(),
        }
    }

    pub fn into_simulation_state(self) -> Result<SimulationState, String> {
        let mut state = SimulationState::default();
        self.apply_to_state(&mut state)?;
        Ok(state)
    }

    /// Apply an already-current, validated result document. Legacy migration
    /// requires the owning [`ProjectId`] and must be completed explicitly at
    /// the project/session boundary before this method is called.
    pub fn apply_to_state(self, state: &mut SimulationState) -> Result<(), String> {
        self.validate()?;
        let executed_decks = self.executed_decks.into_archive()?;
        let runs = self
            .runs
            .into_iter()
            .map(ProjectSimulationRun::into_run)
            .collect::<Result<Vec<_>, _>>()?;
        // Restored before the history, so the project's own limit is the one
        // that prunes it rather than the built-in default.
        state.retained_dataset_limit = self.retained_dataset_limit;
        state.restore_run_history(
            runs,
            self.next_run_id,
            self.active_run_stable_id,
            self.active_dataset_id,
            self.active_analysis_sequence,
            self.overlay_dataset_ids,
        );
        // After the history, because restoring it drops whatever decks this
        // session was holding for a different project.
        state.executed_decks = executed_decks;
        Ok(())
    }

    /// Upgrade historical result schemas without fabricating analysis-source
    /// identity. V1 display-sequence references are converted to stable run and
    /// dataset IDs. V1/v2 analyses retain `provenance: None`; schema v5 records
    /// that fact explicitly per run so provenance cannot disappear from a
    /// current prepared-task result history without validation failing. Runs
    /// written before v6 become explicitly `LegacyUnknown`: execution identity
    /// and lifecycle are never inferred from historical result payloads. V6
    /// analyses migrate with `family_metadata: None`; metadata absent from an
    /// historical payload is never reconstructed from display waveforms.
    /// Result schemas through v7 acquire canonical result-data and dataset
    /// digests from the exact retained values during migration; no samples or
    /// analysis evidence are reconstructed. Schema v8 digests are verified
    /// with their original encoding before payload absence is migrated. Schema
    /// v9 digests are likewise authenticated before Reliability/SOA evidence
    /// absence is preserved. Schema-v10 digests are authenticated before TF
    /// evidence absence is preserved. Schema-v11 digests are authenticated
    /// with their required scalar output-noise encoding before optional output
    /// and input-referred totals are admitted. Schema-v12 digests are
    /// authenticated with their unit-free waveform encoding before per-waveform
    /// units are admitted; a v12 waveform that already carries one is rejected
    /// rather than resealed, because no v12 digest ever covered those bytes.
    /// Schemas v13 through v15 are authenticated with the last required-gain
    /// payload encoding before optional pole-zero gain is admitted. Schema-v16
    /// is authenticated with its exact V7 encoding before recognizable PSS or
    /// PSTB curve-only results acquire an explicit legacy-unknown periodic
    /// marker. No spectrum, orbit policy, or verdict is inferred. Schema-v17
    /// is authenticated with its exact V8 encoding before measurement raw
    /// values are restored from the retained result and absent FAILVALUE
    /// verdicts are made explicit. Schema-v14
    /// receipts predate the deck's hierarchy map and keep an empty
    /// one: a run that executed before the map was sealed has no occurrence
    /// record, and inventing rows for it would forge the provenance the map
    /// exists to carry. Each migrated result is then resealed with the current
    /// encoding.
    pub(crate) fn migrate_to_current(&mut self, project_id: ProjectId) -> Result<(), String> {
        if self.schema_version == PROJECT_SIMULATION_RESULTS_SCHEMA_VERSION {
            return Ok(());
        }
        let mut candidate = self.clone();
        candidate.migrate_to_current_in_place(project_id)?;
        *self = candidate;
        Ok(())
    }

    fn migrate_to_current_in_place(&mut self, project_id: ProjectId) -> Result<(), String> {
        let source_schema = self.schema_version;
        reject_digital_buses_before_schema_v19(self, source_schema)?;
        if source_schema == MEASUREMENT_VERIFICATION_RESULTS_SCHEMA_VERSION {
            for run in &self.runs {
                validate_v18_result_digests(run)?;
            }
            for run in &mut self.runs {
                // `seal_` rather than `reseal_`: a schema-v18 file already
                // carries real measurement-verification evidence, and the
                // reseal path exists to synthesize that evidence for files
                // written before it, which would overwrite it here.
                seal_project_result_digests(run)?;
            }
            self.schema_version = PROJECT_SIMULATION_RESULTS_SCHEMA_VERSION;
            return self.validate();
        }
        reject_measurement_verification_before_schema_v18(self, source_schema)?;
        if source_schema == PERIODIC_STABILITY_RESULTS_SCHEMA_VERSION {
            for run in &self.runs {
                validate_v17_result_digests(run)?;
            }
            for run in &mut self.runs {
                reseal_legacy_project_result_digests(run)?;
            }
            self.schema_version = PROJECT_SIMULATION_RESULTS_SCHEMA_VERSION;
            return self.validate();
        }
        migrate_legacy_specification_receipts(self, source_schema)?;
        reject_periodic_stability_payload_before_schema_v17(self, source_schema)?;
        reject_pole_zero_evidence_before_schema_v16(self, source_schema)?;
        reject_hierarchy_maps_before_schema_v15(self, source_schema)?;
        reject_executed_decks_before_schema_v15(self, source_schema)?;
        reject_derived_task_identities_before_schema_v15(self, source_schema)?;
        if source_schema == POLE_ZERO_EVIDENCE_RESULTS_SCHEMA_VERSION {
            for run in &self.runs {
                validate_v16_result_digests(run)?;
            }
            for run in &mut self.runs {
                synthesize_legacy_periodic_markers(run)?;
                reseal_legacy_project_result_digests(run)?;
            }
            self.schema_version = PROJECT_SIMULATION_RESULTS_SCHEMA_VERSION;
            return self.validate();
        }
        if matches!(
            source_schema,
            WAVEFORM_UNIT_RESULTS_SCHEMA_VERSION
                | GOVERNED_SPECIFICATION_RESULTS_SCHEMA_VERSION
                | EXECUTED_DECK_RESULTS_SCHEMA_VERSION
        ) {
            for run in &mut self.runs {
                validate_v13_to_v15_result_digests(run, source_schema)?;
                synthesize_legacy_periodic_markers(run)?;
                reseal_legacy_project_result_digests(run)?;
            }
            self.schema_version = PROJECT_SIMULATION_RESULTS_SCHEMA_VERSION;
            return self.validate();
        }
        if source_schema == OPERATING_POINT_RESULTS_SCHEMA_VERSION {
            for run in &mut self.runs {
                validate_v12_result_digests(run)?;
                synthesize_legacy_periodic_markers(run)?;
                reseal_legacy_project_result_digests(run)?;
            }
            self.schema_version = PROJECT_SIMULATION_RESULTS_SCHEMA_VERSION;
            return self.validate();
        }
        if source_schema == TRANSFER_FUNCTION_RESULTS_SCHEMA_VERSION {
            for run in &mut self.runs {
                validate_v11_result_digests(run)?;
                synthesize_legacy_periodic_markers(run)?;
                reseal_legacy_project_result_digests(run)?;
            }
            self.schema_version = PROJECT_SIMULATION_RESULTS_SCHEMA_VERSION;
            return self.validate();
        }
        if source_schema == RELIABILITY_SOA_RESULTS_SCHEMA_VERSION {
            for run in &mut self.runs {
                validate_v10_result_digests(run)?;
                synthesize_legacy_periodic_markers(run)?;
                reseal_legacy_project_result_digests(run)?;
            }
            self.schema_version = PROJECT_SIMULATION_RESULTS_SCHEMA_VERSION;
            return self.validate();
        }
        if source_schema == TYPED_PAYLOAD_RESULTS_SCHEMA_VERSION {
            for run in &mut self.runs {
                validate_v9_result_digests(run)?;
                if let Some(analysis) = run.analyses.iter().find(|analysis| {
                    matches!(
                        analysis.result_payload.as_ref(),
                        Some(AnalysisResultPayload::Reliability { .. })
                            | Some(AnalysisResultPayload::Soa { .. })
                    )
                }) {
                    return Err(format!(
                        "schema-v9 analysis {} contains Reliability/SOA evidence introduced by schema v10",
                        analysis.id
                    ));
                }
                if let Some(analysis) = run.analyses.iter().find(|analysis| {
                    matches!(
                        analysis.result_payload.as_ref(),
                        Some(AnalysisResultPayload::TransferFunction { .. })
                    )
                }) {
                    return Err(format!(
                        "schema-v9 analysis {} contains transfer-function evidence introduced by schema v11",
                        analysis.id
                    ));
                }
                synthesize_legacy_periodic_markers(run)?;
                reseal_legacy_project_result_digests(run)?;
            }
            self.schema_version = PROJECT_SIMULATION_RESULTS_SCHEMA_VERSION;
            return self.validate();
        }
        if source_schema == CONTENT_DIGEST_RESULTS_SCHEMA_VERSION {
            for run in &mut self.runs {
                validate_v8_result_digests(run)?;
                if let Some(analysis) = run
                    .analyses
                    .iter()
                    .find(|analysis| analysis.result_payload.is_present())
                {
                    return Err(format!(
                        "schema-v8 analysis {} contains a typed result payload introduced by schema v9",
                        analysis.id
                    ));
                }
                synthesize_legacy_periodic_markers(run)?;
                reseal_legacy_project_result_digests(run)?;
            }
            self.schema_version = PROJECT_SIMULATION_RESULTS_SCHEMA_VERSION;
            return self.validate();
        }
        if matches!(
            source_schema,
            EXECUTION_IDENTITY_RESULTS_SCHEMA_VERSION | FAMILY_METADATA_RESULTS_SCHEMA_VERSION
        ) {
            for run in &mut self.runs {
                require_legacy_result_digest_absence(run, source_schema)?;
                validate_result_fields_for_source_schema(run, source_schema)?;
                synthesize_legacy_periodic_markers(run)?;
                reseal_legacy_project_result_digests(run)?;
            }
            self.schema_version = PROJECT_SIMULATION_RESULTS_SCHEMA_VERSION;
            return self.validate();
        }
        let migrate_v1_references = match source_schema {
            PROJECT_SIMULATION_RESULTS_SCHEMA_VERSION => return Ok(()),
            LEGACY_SIMULATION_RESULTS_SCHEMA_VERSION => true,
            STABLE_DATASET_RESULTS_SCHEMA_VERSION
            | PREPARED_PROVENANCE_RESULTS_SCHEMA_VERSION
            | EXPLICIT_PROVENANCE_MODE_RESULTS_SCHEMA_VERSION
            | SOURCE_DOMAIN_RESULTS_SCHEMA_VERSION => false,
            unsupported => {
                return Err(format!(
                    "unsupported simulation results schema version {unsupported}"
                ));
            }
        };

        self.validate_legacy_schema_shape(source_schema)?;

        if migrate_v1_references {
            for (run_idx, run) in self.runs.iter_mut().enumerate() {
                let identity = serde_json::to_vec(&(
                    project_id,
                    run_idx,
                    run.id,
                    &run.label,
                    run.timestamp.to_bits(),
                    &run.analyses,
                    run.elapsed_time.to_bits(),
                    run.success,
                ))
                .map_err(|error| {
                    format!(
                        "legacy simulation run sequence {} could not be assigned a reproducible identity: {error}",
                        run.id
                    )
                })?;
                if run.run_id.is_none() {
                    run.run_id = Some(RunId::from_namespace(
                        LEGACY_RESULT_RUN_ID_NAMESPACE,
                        &identity,
                    ));
                }
                if run.dataset_id.is_none() {
                    run.dataset_id = Some(DatasetId::from_namespace(
                        LEGACY_RESULT_DATASET_ID_NAMESPACE,
                        &identity,
                    ));
                }
            }

            if let Some(run_sequence) = self.active_run_id {
                let active_run = self
                    .runs
                    .iter()
                    .find(|run| run.id == run_sequence)
                    .ok_or_else(|| {
                        format!(
                            "legacy active simulation run sequence {run_sequence} does not exist"
                        )
                    })?;
                self.active_run_stable_id = Some(active_run.run_id.ok_or_else(|| {
                    format!(
                        "legacy simulation run sequence {} has no stable id",
                        active_run.id
                    )
                })?);
                self.active_dataset_id = Some(active_run.dataset_id.ok_or_else(|| {
                    format!(
                        "legacy simulation run sequence {} has no dataset id",
                        active_run.id
                    )
                })?);
                self.active_analysis_sequence = if let Some(sequence) = self.active_analysis_id {
                    if !active_run
                        .analyses
                        .iter()
                        .any(|analysis| analysis.id == sequence)
                    {
                        return Err(format!(
                            "legacy active analysis sequence {sequence} does not exist in run {}",
                            active_run.id
                        ));
                    }
                    Some(sequence)
                } else {
                    None
                };
            } else if self.active_analysis_id.is_some() {
                return Err("legacy active analysis has no active simulation run".to_owned());
            }

            if !self.overlay_run_ids.is_empty() {
                self.overlay_dataset_ids.clear();
                for sequence in &self.overlay_run_ids {
                    let dataset_id = self
                        .runs
                        .iter()
                        .find(|run| run.id == *sequence)
                        .map(|run| {
                            run.dataset_id.ok_or_else(|| {
                                format!("legacy overlay run sequence {sequence} has no dataset id")
                            })
                        })
                        .transpose()?
                        .ok_or_else(|| {
                            format!("legacy overlay run sequence {sequence} does not exist")
                        })?;
                    if Some(dataset_id) != self.active_dataset_id
                        && !self.overlay_dataset_ids.contains(&dataset_id)
                    {
                        self.overlay_dataset_ids.push(dataset_id);
                    }
                }
            }
        }

        self.active_run_id = None;
        self.active_analysis_id = None;
        self.overlay_run_ids.clear();

        for (run_idx, run) in self.runs.iter_mut().enumerate() {
            let provenance_count = run
                .analyses
                .iter()
                .filter(|analysis| analysis.provenance.is_some())
                .count();
            let migrated_mode = match source_schema {
                LEGACY_SIMULATION_RESULTS_SCHEMA_VERSION
                | STABLE_DATASET_RESULTS_SCHEMA_VERSION => {
                    ProjectRunProvenanceMode::LegacyUnattributed
                }
                PREPARED_PROVENANCE_RESULTS_SCHEMA_VERSION => {
                    if provenance_count == 0 {
                        ProjectRunProvenanceMode::LegacyUnattributed
                    } else if provenance_count == run.analyses.len() {
                        set_legacy_unclassified_source_domains(run);
                        ProjectRunProvenanceMode::LegacyPreparedUnclassified
                    } else {
                        return Err(format!(
                            "runs[{run_idx}] mixes schema-v3 analyses with and without prepared-task provenance"
                        ));
                    }
                }
                EXPLICIT_PROVENANCE_MODE_RESULTS_SCHEMA_VERSION => {
                    match run.provenance_mode.as_ref().copied() {
                        Some(ProjectRunProvenanceMode::LegacyUnattributed)
                            if provenance_count == 0 =>
                        {
                            ProjectRunProvenanceMode::LegacyUnattributed
                        }
                        Some(ProjectRunProvenanceMode::PreparedTaskBound)
                            if provenance_count == run.analyses.len() && provenance_count != 0 =>
                        {
                            set_legacy_unclassified_source_domains(run);
                            ProjectRunProvenanceMode::LegacyPreparedUnclassified
                        }
                        Some(ProjectRunProvenanceMode::LegacyUnattributed) => {
                            return Err(format!(
                                "runs[{run_idx}] is schema-v4 legacy_unattributed but contains prepared-task provenance"
                            ));
                        }
                        Some(ProjectRunProvenanceMode::PreparedTaskBound) => {
                            return Err(format!(
                                "runs[{run_idx}] is schema-v4 prepared_task_bound but its complete provenance is missing"
                            ));
                        }
                        Some(ProjectRunProvenanceMode::Unspecified) | None => {
                            return Err(format!(
                                "runs[{run_idx}].provenance_mode is missing from schema-v4 simulation results"
                            ));
                        }
                        Some(ProjectRunProvenanceMode::LegacyPreparedUnclassified) => {
                            return Err(format!(
                                "runs[{run_idx}] uses a provenance mode introduced after schema v4"
                            ));
                        }
                    }
                }
                SOURCE_DOMAIN_RESULTS_SCHEMA_VERSION => run
                    .provenance_mode
                    .as_ref()
                    .copied()
                    .ok_or_else(|| {
                        format!(
                            "runs[{run_idx}].provenance_mode is missing from schema-v5 simulation results"
                        )
                    })?,
                _ => unreachable!("supported legacy result schema matched above"),
            };
            run.provenance_mode = PersistedField::Value(migrated_mode);
            run.lifecycle = Some(SimulationRunLifecycle::LegacyUnknown);
            synthesize_legacy_periodic_markers(run)?;
            reseal_legacy_project_result_digests(run)?;
            run.validate(run_idx)?;
        }
        self.schema_version = PROJECT_SIMULATION_RESULTS_SCHEMA_VERSION;
        Ok(())
    }

    fn validate_legacy_schema_shape(&self, source_schema: u32) -> Result<(), String> {
        let stable_ids_required = source_schema >= STABLE_DATASET_RESULTS_SCHEMA_VERSION;
        let legacy_sequence_refs_allowed =
            source_schema == LEGACY_SIMULATION_RESULTS_SCHEMA_VERSION;
        if legacy_sequence_refs_allowed {
            if self.active_run_stable_id.is_some()
                || self.active_dataset_id.is_some()
                || self.active_analysis_sequence.is_some()
                || !self.overlay_dataset_ids.is_empty()
            {
                return Err(
                    "schema-v1 simulation results contain stable references introduced by schema v2"
                        .to_owned(),
                );
            }
        } else if self.active_run_id.is_some()
            || self.active_analysis_id.is_some()
            || !self.overlay_run_ids.is_empty()
        {
            return Err(format!(
                "schema-v{source_schema} simulation results retain schema-v1 sequence references"
            ));
        }

        for (run_idx, run) in self.runs.iter().enumerate() {
            require_legacy_result_digest_absence(run, source_schema)?;
            validate_result_fields_for_source_schema(run, source_schema)?;
            if run.job_id.is_some() || run.execution_target.is_some() || run.lifecycle.is_some() {
                return Err(format!(
                    "runs[{run_idx}] contains execution identity or lifecycle introduced after schema v{source_schema}"
                ));
            }
            if source_schema < SOURCE_DOMAIN_RESULTS_SCHEMA_VERSION
                && run.prepared_receipt.is_present()
            {
                return Err(format!(
                    "runs[{run_idx}] contains a prepared run receipt introduced after schema v{source_schema}"
                ));
            }
            if stable_ids_required {
                if run.run_id.is_none() || run.dataset_id.is_none() {
                    return Err(format!(
                        "runs[{run_idx}] is missing stable run/dataset identity required by schema v{source_schema}"
                    ));
                }
            } else if run.run_id.is_some() || run.dataset_id.is_some() {
                return Err(format!(
                    "runs[{run_idx}] contains stable identity introduced after schema v1"
                ));
            }

            match source_schema {
                LEGACY_SIMULATION_RESULTS_SCHEMA_VERSION
                | STABLE_DATASET_RESULTS_SCHEMA_VERSION => {
                    if run.provenance_mode.is_present() {
                        return Err(format!(
                            "runs[{run_idx}] contains provenance_mode introduced after schema v{source_schema}"
                        ));
                    }
                    if run
                        .analyses
                        .iter()
                        .any(|analysis| analysis.provenance.is_some())
                    {
                        return Err(format!(
                            "runs[{run_idx}] contains prepared provenance introduced after schema v{source_schema}"
                        ));
                    }
                }
                PREPARED_PROVENANCE_RESULTS_SCHEMA_VERSION => {
                    if run.provenance_mode.is_present() {
                        return Err(format!(
                            "runs[{run_idx}] contains provenance_mode introduced after schema v3"
                        ));
                    }
                }
                EXPLICIT_PROVENANCE_MODE_RESULTS_SCHEMA_VERSION => {
                    if run.provenance_mode.as_ref().is_none() {
                        return Err(format!(
                            "runs[{run_idx}].provenance_mode is required by schema v4"
                        ));
                    }
                }
                SOURCE_DOMAIN_RESULTS_SCHEMA_VERSION => {
                    if run.provenance_mode.as_ref().is_none() {
                        return Err(format!(
                            "runs[{run_idx}].provenance_mode is required by schema v5"
                        ));
                    }
                }
                _ => unreachable!("supported legacy result schema matched above"),
            }

            for (analysis_idx, analysis) in run.analyses.iter().enumerate() {
                if let Some(provenance) = &analysis.provenance {
                    if source_schema < SOURCE_DOMAIN_RESULTS_SCHEMA_VERSION
                        && provenance.source_domain.is_present()
                    {
                        return Err(format!(
                            "runs[{run_idx}].analyses[{analysis_idx}].provenance.source_domain was introduced after schema v{source_schema}"
                        ));
                    }
                    if source_schema == SOURCE_DOMAIN_RESULTS_SCHEMA_VERSION
                        && provenance.source_domain.as_ref().is_none()
                    {
                        return Err(format!(
                            "runs[{run_idx}].analyses[{analysis_idx}].provenance.source_domain is required by schema v5"
                        ));
                    }
                    if provenance.pvt_point.is_some() {
                        return Err(format!(
                            "runs[{run_idx}].analyses[{analysis_idx}].provenance.pvt_point was introduced after schema v{source_schema}"
                        ));
                    }
                }
            }
        }
        Ok(())
    }

    pub(crate) fn validate(&self) -> Result<(), String> {
        if self.schema_version != PROJECT_SIMULATION_RESULTS_SCHEMA_VERSION {
            return Err(format!(
                "unsupported simulation results schema version {}",
                self.schema_version
            ));
        }
        if self.active_run_id.is_some()
            || self.active_analysis_id.is_some()
            || !self.overlay_run_ids.is_empty()
        {
            return Err("current simulation results retain unmigrated v1 references".to_owned());
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

fn reject_pole_zero_evidence_before_schema_v16(
    results: &ProjectSimulationResults,
    source_schema: u32,
) -> Result<(), String> {
    if source_schema >= POLE_ZERO_EVIDENCE_RESULTS_SCHEMA_VERSION {
        return Ok(());
    }
    for run in &results.runs {
        for analysis in &run.analyses {
            let PersistedField::Value(AnalysisResultPayload::PoleZero {
                pole_evidence,
                zero_evidence,
                ..
            }) = &analysis.result_payload
            else {
                continue;
            };
            if !matches!(
                pole_evidence,
                crate::state::PoleZeroRootSetEvidence::LegacyUnknown
            ) || !matches!(
                zero_evidence,
                crate::state::PoleZeroRootSetEvidence::LegacyUnknown
            ) {
                return Err(format!(
                    "schema-v{source_schema} analysis {} contains pole-zero root evidence introduced by schema v16",
                    analysis.id
                ));
            }
        }
    }
    Ok(())
}

fn reject_periodic_stability_payload_before_schema_v17(
    results: &ProjectSimulationResults,
    source_schema: u32,
) -> Result<(), String> {
    if source_schema >= PERIODIC_STABILITY_RESULTS_SCHEMA_VERSION {
        return Ok(());
    }
    for run in &results.runs {
        for analysis in &run.analyses {
            if matches!(
                analysis.result_payload.as_ref(),
                Some(AnalysisResultPayload::PssFloquet { .. })
                    | Some(AnalysisResultPayload::Pstb { .. })
            ) {
                return Err(format!(
                    "schema-v{source_schema} analysis {} contains periodic stability evidence introduced by schema v17",
                    analysis.id
                ));
            }
        }
    }
    Ok(())
}

fn synthesize_legacy_periodic_markers(run: &mut ProjectSimulationRun) -> Result<(), String> {
    for analysis in &mut run.analyses {
        if !analysis.success || !analysis.result_payload.is_missing() {
            continue;
        }
        let analysis_type = analysis_type_from_key(&analysis.analysis_type).ok_or_else(|| {
            format!(
                "legacy analysis {} has unknown analysis type '{}'",
                analysis.id, analysis.analysis_type
            )
        })?;
        if let Some(marker) = AnalysisResultPayload::legacy_periodic_marker(analysis_type) {
            analysis.result_payload = PersistedField::Value(marker);
        }
    }
    Ok(())
}

fn require_legacy_result_digest_absence(
    run: &ProjectSimulationRun,
    source_schema: u32,
) -> Result<(), String> {
    if run.dataset_content_digest.is_present() {
        return Err(format!(
            "schema-v{source_schema} simulation run {} contains a dataset content digest introduced by schema v8",
            run.id
        ));
    }
    if let Some(analysis) = run
        .analyses
        .iter()
        .find(|analysis| analysis.result_data_digest.is_present())
    {
        return Err(format!(
            "schema-v{source_schema} analysis {} contains a result data digest introduced by schema v8",
            analysis.id
        ));
    }
    Ok(())
}

pub(super) fn validate_result_fields_for_source_schema(
    run: &ProjectSimulationRun,
    source_schema: u32,
) -> Result<(), String> {
    validate_legacy_noise_summary_shape(run, source_schema)?;
    reject_legacy_waveform_units(run, source_schema)?;
    for analysis in &run.analyses {
        if source_schema < FAMILY_METADATA_RESULTS_SCHEMA_VERSION
            && analysis.family_metadata.is_some()
        {
            return Err(format!(
                "schema-v{source_schema} analysis {} contains family metadata introduced by schema v7",
                analysis.id
            ));
        }
        if source_schema < TYPED_PAYLOAD_RESULTS_SCHEMA_VERSION
            && analysis.result_payload.is_present()
        {
            return Err(format!(
                "schema-v{source_schema} analysis {} contains a typed result payload introduced by schema v9",
                analysis.id
            ));
        }
    }
    Ok(())
}

/// A unit on a waveform written before schema v13 is content the digest that
/// sealed the file never covered. Admitting it would let an edited unit ride
/// into a resealed document under an authentic older digest.
fn reject_legacy_waveform_units(
    run: &ProjectSimulationRun,
    source_schema: u32,
) -> Result<(), String> {
    for analysis in &run.analyses {
        if analysis
            .waveforms
            .iter()
            .any(|waveform| waveform.unit.is_some())
        {
            return Err(format!(
                "schema-v{source_schema} analysis {} contains a waveform unit introduced by schema v13",
                analysis.id
            ));
        }
    }
    Ok(())
}

fn reject_legacy_operating_point_evidence(
    run: &ProjectSimulationRun,
    source_schema: u32,
) -> Result<(), String> {
    if let Some(analysis) = run.analyses.iter().find(|analysis| {
        matches!(
            analysis.result_payload.as_ref(),
            Some(AnalysisResultPayload::OperatingPoint { .. })
        )
    }) {
        return Err(format!(
            "schema-v{source_schema} analysis {} contains operating-point evidence introduced by schema v12",
            analysis.id
        ));
    }
    Ok(())
}

fn validate_legacy_noise_summary_shape(
    run: &ProjectSimulationRun,
    source_schema: u32,
) -> Result<(), String> {
    for analysis in &run.analyses {
        let Some(summary) = analysis.noise_summary.as_ref() else {
            continue;
        };
        if summary.total_rms.is_none() {
            return Err(format!(
                "schema-v{source_schema} analysis {} is missing required noise_summary.total_rms",
                analysis.id
            ));
        }
        if summary.input_rms.is_some() {
            return Err(format!(
                "schema-v{source_schema} analysis {} contains noise_summary.input_rms introduced by schema v12",
                analysis.id
            ));
        }
    }
    Ok(())
}

fn reject_measurement_verification_before_schema_v18(
    results: &ProjectSimulationResults,
    source_schema: u32,
) -> Result<(), String> {
    if source_schema >= MEASUREMENT_VERIFICATION_RESULTS_SCHEMA_VERSION {
        return Ok(());
    }
    for run in &results.runs {
        for analysis in &run.analyses {
            for measurement in &analysis.measurements {
                if measurement.raw_value.is_some()
                    || measurement.failure_limit.is_some()
                    || measurement.failure_limit_exceeded
                {
                    return Err(format!(
                        "schema-v{source_schema} analysis {} measurement '{}' contains measurement verification evidence introduced by schema v18",
                        analysis.id, measurement.name
                    ));
                }
            }
        }
    }
    Ok(())
}

/// A digital bus table is schema-v19 evidence. A file that says it was
/// written under an earlier schema cannot have had one, so one that carries a
/// bus is not an old file — it is a file whose stated version disagrees with
/// its content, and admitting it would authenticate a table under a digest
/// encoding that never covered it.
fn reject_digital_buses_before_schema_v19(
    results: &ProjectSimulationResults,
    source_schema: u32,
) -> Result<(), String> {
    if source_schema >= DIGITAL_BUS_RESULTS_SCHEMA_VERSION {
        return Ok(());
    }
    for run in &results.runs {
        for analysis in &run.analyses {
            if let Some(AnalysisResultPayload::TransientEvents { digital_buses, .. }) =
                analysis.result_payload.as_ref()
                && !digital_buses.is_empty()
            {
                return Err(format!(
                    "schema-v{source_schema} analysis {} declares digital bus '{}', which was introduced by schema v19",
                    analysis.id, digital_buses[0].name
                ));
            }
        }
    }
    Ok(())
}

fn restore_legacy_measurement_verification(run: &mut ProjectSimulationRun) {
    for analysis in &mut run.analyses {
        for measurement in &mut analysis.measurements {
            measurement.raw_value = measurement.value;
            measurement.failure_limit = None;
            measurement.failure_limit_exceeded = false;
        }
    }
}

pub(super) fn seal_project_result_digests(run: &mut ProjectSimulationRun) -> Result<(), String> {
    for analysis in &mut run.analyses {
        let digest = analysis.clone().into_analysis()?.result_data_digest();
        analysis.result_data_digest = PersistedField::Value(digest);
    }
    let digest = run.clone().into_run()?.dataset_content_digest();
    run.dataset_content_digest = PersistedField::Value(digest);
    Ok(())
}

fn reseal_legacy_project_result_digests(run: &mut ProjectSimulationRun) -> Result<(), String> {
    restore_legacy_measurement_verification(run);
    seal_project_result_digests(run)
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
    pub campaign_membership: Option<crate::state::SimulationCampaignMembership>,
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
}

impl ProjectSimulationRun {
    pub(super) fn into_run(self) -> Result<SimulationRun, String> {
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
            .map(ProjectAnalysisResult::into_analysis)
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
        let mut run = SimulationRun::new(self.id);
        run.job_id = self.job_id;
        run.run_id = run_id;
        run.dataset_id = dataset_id;
        run.execution_target = self.execution_target;
        let restored_lifecycle = self
            .lifecycle
            .unwrap_or(SimulationRunLifecycle::LegacyUnknown);
        if matches!(
            restored_lifecycle,
            SimulationRunLifecycle::Preparing
                | SimulationRunLifecycle::Running
                | SimulationRunLifecycle::Cancelling
        ) {
            for analysis in &mut analyses {
                if analysis.is_live_partial() {
                    analysis.error_message = Some(
                        "Simulation was interrupted before completion; retained waveforms are accepted partial samples"
                            .to_owned(),
                    );
                }
            }
        }
        run.lifecycle = match restored_lifecycle {
            SimulationRunLifecycle::Preparing
            | SimulationRunLifecycle::Running
            | SimulationRunLifecycle::Cancelling => SimulationRunLifecycle::Interrupted,
            lifecycle => lifecycle,
        };
        run.label = self.label;
        run.timestamp = self.timestamp;
        run.analyses = analyses;
        run.set_retention(self.retention);
        run.elapsed_time = self.elapsed_time;
        run.success = if matches!(
            restored_lifecycle,
            SimulationRunLifecycle::Preparing
                | SimulationRunLifecycle::Running
                | SimulationRunLifecycle::Cancelling
        ) {
            false
        } else {
            self.success
        };
        run.restore_campaign_membership(campaign_membership)?;
        run.restore_provenance(provenance)?;
        if specification_verdicts.is_none()
            && matches!(
                restored_lifecycle,
                SimulationRunLifecycle::Preparing
                    | SimulationRunLifecycle::Running
                    | SimulationRunLifecycle::Cancelling
            )
            && run.prepared_receipt().is_some()
        {
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
        require_finite(self.elapsed_time, &format!("runs[{run_idx}].elapsed_time"))?;
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
            receipt.validate_result_prefix(&analyses)?;
        }
        let retained_digest = self.dataset_content_digest.as_ref().copied().ok_or_else(|| {
            format!("runs[{run_idx}].dataset_content_digest is required by simulation results schema v12")
        })?;
        let computed_digest = self
            .clone()
            .into_run()
            .map_err(|error| {
                format!("runs[{run_idx}] cannot compute its dataset content digest: {error}")
            })?
            .dataset_content_digest();
        if retained_digest != computed_digest {
            return Err(format!(
                "runs[{run_idx}].dataset_content_digest does not match retained analysis content"
            ));
        }
        Ok(())
    }
}

impl From<&SimulationRun> for ProjectSimulationRun {
    fn from(run: &SimulationRun) -> Self {
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
                .map(ProjectAnalysisResult::from)
                .collect(),
            dataset_content_digest: PersistedField::Value(run.dataset_content_digest()),
            provenance_mode,
            prepared_receipt,
            campaign_membership: run.campaign_membership().cloned(),
            retention: run.retention(),
            elapsed_time: run.elapsed_time,
            success,
            specification_verdicts: run.specification_verdicts().map(<[_]>::to_vec),
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
    #[serde(default)]
    pub measurements: Vec<ProjectMeasurement>,
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
    pub failure_attribution: Option<crate::state::ConvergenceAttribution>,
    /// Complete source identity for prepared-task results. `None` is retained only
    /// when loading result history written by v1/v2 projects.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provenance: Option<ProjectAnalysisResultProvenance>,
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
        )
        .map(|provenance| provenance.with_pvt_point(pvt_point))
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
        )
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
        }
    }
}

impl ProjectAnalysisResult {
    pub(super) fn into_analysis(self) -> Result<AnalysisResult, String> {
        if self.result_data_digest.is_null() {
            return Err(format!(
                "analysis sequence {} has an explicitly null result data digest",
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
        let mut analysis = AnalysisResult {
            id: self.id,
            analysis_type,
            label: self.label,
            timestamp: self.timestamp,
            waveforms: self
                .waveforms
                .into_iter()
                .map(ProjectWaveformData::into_waveform)
                .collect(),
            dc_op: self.dc_op.map(ProjectDcOpResult::into_dc_op),
            device_op: self.device_op.map(ProjectDeviceOpReport::into_report),
            noise_summary: self
                .noise_summary
                .map(ProjectNoiseSummary::into_noise_summary),
            family_metadata: self.family_metadata,
            result_payload: self.result_payload.into_value(),
            measurements: self
                .measurements
                .into_iter()
                .map(ProjectMeasurement::into_measurement)
                .collect(),
            saved_output_receipts: self.saved_output_receipts,
            success: self.success,
            error_message: self.error_message,
            failure_attribution: self.failure_attribution,
            provenance,
        };
        for receipt in &analysis.saved_output_receipts {
            let SavedOutputMaterializationStatus::Materialized { waveform_name, .. } =
                &receipt.status
            else {
                continue;
            };
            if (receipt.stored_precision
                == crate::state::SavedOutputPrecision::DisplayCacheWithFullSourcePrecision
                || receipt.streaming
                    == crate::state::SavedOutputStreaming::LivePlotAdaptiveDisplayDecimation)
                && let Some(waveform) = analysis
                    .waveforms
                    .iter_mut()
                    .find(|waveform| waveform.name == *waveform_name)
            {
                waveform
                    .rebuild_display_cache(crate::state::DEFAULT_DISPLAY_WAVEFORM_CACHE_SAMPLES);
            }
        }
        Ok(analysis)
    }

    fn validate(&self, run_idx: usize, analysis_idx: usize) -> Result<(), String> {
        let prefix = format!("runs[{run_idx}].analyses[{analysis_idx}]");
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
        let mut receipt_ids = HashSet::new();
        let mut receipt_digests = HashSet::new();
        for (receipt_idx, receipt) in self.saved_output_receipts.iter().enumerate() {
            let receipt_prefix = format!("{prefix}.saved_output_receipts[{receipt_idx}]");
            if !receipt_ids.insert(receipt.output_id) {
                return Err(format!(
                    "{prefix} has duplicate saved-output identity {}",
                    receipt.output_id
                ));
            }
            if !receipt_digests.insert(receipt.contract_digest) {
                return Err(format!(
                    "{prefix} has duplicate saved-output contract digest {}",
                    receipt.contract_digest
                ));
            }
            if receipt.name.trim().is_empty() || receipt.source_expression.trim().is_empty() {
                return Err(format!(
                    "{receipt_prefix} has an empty name or source expression"
                ));
            }
            if let Some(provenance) = &self.provenance
                && receipt.analysis_id != provenance.source_instance_id
            {
                return Err(format!(
                    "{receipt_prefix} analysis identity does not match result provenance"
                ));
            }
            match &receipt.status {
                SavedOutputMaterializationStatus::Materialized {
                    waveform_name,
                    sample_count,
                } => {
                    if self.success
                        && receipt.save_policy
                            == crate::state::SavedOutputPolicy::FailureDiagnosticsOnly
                    {
                        return Err(format!(
                            "{receipt_prefix} materializes failure-only data on a successful analysis"
                        ));
                    }
                    let waveform = self
                        .waveforms
                        .iter()
                        .find(|waveform| waveform.name == *waveform_name)
                        .ok_or_else(|| {
                            format!(
                                "{receipt_prefix} names absent materialized waveform '{waveform_name}'"
                            )
                        })?;
                    if usize::try_from(*sample_count).ok() != Some(waveform.x.len()) {
                        return Err(format!(
                            "{receipt_prefix} sample count does not match waveform '{waveform_name}'"
                        ));
                    }
                }
                SavedOutputMaterializationStatus::SuppressedOnSuccess if !self.success => {
                    return Err(format!(
                        "{receipt_prefix} suppresses failure diagnostics on a failed analysis"
                    ));
                }
                SavedOutputMaterializationStatus::SuppressedOnSuccess
                    if receipt.save_policy
                        != crate::state::SavedOutputPolicy::FailureDiagnosticsOnly =>
                {
                    return Err(format!(
                        "{receipt_prefix} uses success suppression for a non-diagnostic save policy"
                    ));
                }
                SavedOutputMaterializationStatus::Deferred
                    if receipt.save_policy
                        != crate::state::SavedOutputPolicy::OnDemandFromRetainedState =>
                {
                    return Err(format!(
                        "{receipt_prefix} defers a save policy that requires immediate materialization"
                    ));
                }
                SavedOutputMaterializationStatus::Unavailable { reason }
                    if reason.trim().is_empty() =>
                {
                    return Err(format!(
                        "{receipt_prefix} has an empty unavailability reason"
                    ));
                }
                SavedOutputMaterializationStatus::Deferred
                | SavedOutputMaterializationStatus::SuppressedOnSuccess
                | SavedOutputMaterializationStatus::Unavailable { .. } => {}
            }
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
        let computed_digest = analysis.result_data_digest();
        if retained_digest != computed_digest {
            return Err(format!(
                "{prefix}.result_data_digest does not match retained analysis content"
            ));
        }
        Ok(())
    }
}

impl From<&AnalysisResult> for ProjectAnalysisResult {
    fn from(analysis: &AnalysisResult) -> Self {
        Self {
            id: analysis.id,
            analysis_type: analysis_type_key(analysis.analysis_type).to_string(),
            label: analysis.label.clone(),
            timestamp: analysis.timestamp,
            result_data_digest: PersistedField::Value(analysis.result_data_digest()),
            waveforms: analysis
                .waveforms
                .iter()
                .map(ProjectWaveformData::from)
                .collect(),
            dc_op: analysis.dc_op.as_ref().map(ProjectDcOpResult::from),
            device_op: analysis.device_op.as_ref().map(ProjectDeviceOpReport::from),
            noise_summary: analysis
                .noise_summary
                .as_ref()
                .map(ProjectNoiseSummary::from),
            family_metadata: analysis.family_metadata.clone(),
            result_payload: analysis
                .result_payload
                .clone()
                .map_or(PersistedField::Missing, PersistedField::Value),
            measurements: analysis
                .measurements
                .iter()
                .map(ProjectMeasurement::from)
                .collect(),
            saved_output_receipts: analysis.saved_output_receipts.clone(),
            success: analysis.success,
            error_message: analysis.error_message.clone(),
            failure_attribution: analysis.failure_attribution.clone(),
            provenance: analysis
                .provenance
                .as_ref()
                .map(ProjectAnalysisResultProvenance::from),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectWaveformData {
    pub name: String,
    pub x: Vec<f64>,
    pub y: Vec<f64>,
    pub color: String,
    #[serde(default = "default_true")]
    pub visible: bool,
    /// The unit the samples are measured in, introduced by schema v13.
    /// Absent in every earlier project, and absent in a current one whose
    /// producer stated no unit — the two are the same fact, so both read
    /// back as unstated rather than as a fabricated default.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub complex: Option<ProjectComplexWaveformComponents>,
}

impl ProjectWaveformData {
    fn into_waveform(self) -> WaveformData {
        let mut waveform = WaveformData::new(self.name, self.x, self.y, self.color);
        waveform.visible = self.visible;
        waveform.unit = self.unit;
        if let Some(complex) = self.complex {
            waveform =
                waveform.with_complex_components(complex.source_name, complex.real, complex.imag);
        }
        waveform
    }

    fn validate(&self, prefix: &str) -> Result<(), String> {
        if self.x.len() != self.y.len() {
            return Err(format!(
                "{prefix} has mismatched x/y sample counts ({} vs {})",
                self.x.len(),
                self.y.len()
            ));
        }
        require_finite_values(&self.x, &format!("{prefix}.x"))?;
        require_monotonic_non_decreasing(&self.x, &format!("{prefix}.x"))?;
        require_finite_values(&self.y, &format!("{prefix}.y"))?;
        if let Some(complex) = &self.complex {
            complex.validate(prefix, self.y.len())?;
        }
        Ok(())
    }
}

impl From<&WaveformData> for ProjectWaveformData {
    fn from(waveform: &WaveformData) -> Self {
        Self {
            name: waveform.name.clone(),
            x: waveform.x.iter().copied().collect(),
            y: waveform.y.iter().copied().collect(),
            color: waveform.color.clone(),
            visible: waveform.visible,
            unit: waveform.unit.clone(),
            complex: waveform
                .complex
                .as_ref()
                .map(|complex| ProjectComplexWaveformComponents {
                    source_name: complex.source_name.clone(),
                    real: complex.real.iter().copied().collect(),
                    imag: complex.imag.iter().copied().collect(),
                }),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectComplexWaveformComponents {
    pub source_name: String,
    pub real: Vec<f64>,
    pub imag: Vec<f64>,
}

impl ProjectComplexWaveformComponents {
    fn validate(&self, prefix: &str, expected_len: usize) -> Result<(), String> {
        if self.real.len() != self.imag.len() || self.real.len() != expected_len {
            return Err(format!(
                "{prefix}.complex has mismatched real/imag/display sample counts"
            ));
        }
        require_finite_values(&self.real, &format!("{prefix}.complex.real"))?;
        require_finite_values(&self.imag, &format!("{prefix}.complex.imag"))?;
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ProjectDcOpResult {
    #[serde(default)]
    pub node_voltages: Vec<ProjectOperatingPointValue>,
    #[serde(default)]
    pub branch_currents: Vec<ProjectOperatingPointValue>,
    #[serde(default)]
    pub power_dissipation: Vec<ProjectOperatingPointValue>,
}

impl ProjectDcOpResult {
    fn into_dc_op(self) -> DcOpResult {
        DcOpResult {
            node_voltages: self
                .node_voltages
                .into_iter()
                .map(ProjectOperatingPointValue::into_op_value)
                .collect(),
            branch_currents: self
                .branch_currents
                .into_iter()
                .map(ProjectOperatingPointValue::into_op_value)
                .collect(),
            power_dissipation: self
                .power_dissipation
                .into_iter()
                .map(ProjectOperatingPointValue::into_op_value)
                .collect(),
        }
    }

    fn validate(&self, prefix: &str) -> Result<(), String> {
        for (idx, value) in self.node_voltages.iter().enumerate() {
            value.validate(&format!("{prefix}.node_voltages[{idx}]"))?;
        }
        for (idx, value) in self.branch_currents.iter().enumerate() {
            value.validate(&format!("{prefix}.branch_currents[{idx}]"))?;
        }
        for (idx, value) in self.power_dissipation.iter().enumerate() {
            value.validate(&format!("{prefix}.power_dissipation[{idx}]"))?;
        }
        Ok(())
    }
}

impl From<&DcOpResult> for ProjectDcOpResult {
    fn from(dc_op: &DcOpResult) -> Self {
        Self {
            node_voltages: dc_op
                .node_voltages
                .iter()
                .map(ProjectOperatingPointValue::from)
                .collect(),
            branch_currents: dc_op
                .branch_currents
                .iter()
                .map(ProjectOperatingPointValue::from)
                .collect(),
            power_dissipation: dc_op
                .power_dissipation
                .iter()
                .map(ProjectOperatingPointValue::from)
                .collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectOperatingPointValue {
    pub name: String,
    pub value: f64,
    pub unit: String,
}

impl ProjectOperatingPointValue {
    fn into_op_value(self) -> OperatingPointValue {
        OperatingPointValue {
            name: self.name,
            value: self.value,
            unit: self.unit,
        }
    }

    fn validate(&self, prefix: &str) -> Result<(), String> {
        require_finite(self.value, &format!("{prefix}.value"))
    }
}

impl From<&OperatingPointValue> for ProjectOperatingPointValue {
    fn from(value: &OperatingPointValue) -> Self {
        Self {
            name: value.name.clone(),
            value: value.value,
            unit: value.unit.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectNoiseSummary {
    #[serde(default)]
    pub rows: Vec<ProjectNoiseContributorRow>,
    #[serde(default)]
    pub total_rms: Option<f64>,
    #[serde(default)]
    pub input_rms: Option<f64>,
    pub band: (f64, f64),
}

impl ProjectNoiseSummary {
    pub(super) fn into_noise_summary(self) -> NoiseSummary {
        NoiseSummary {
            rows: self
                .rows
                .into_iter()
                .map(ProjectNoiseContributorRow::into_row)
                .collect(),
            total_rms: self.total_rms,
            input_rms: self.input_rms,
            band: self.band,
        }
    }

    fn validate(&self, prefix: &str) -> Result<(), String> {
        if let Some(total_rms) = self.total_rms {
            require_finite(total_rms, &format!("{prefix}.total_rms"))?;
        }
        if let Some(input_rms) = self.input_rms {
            require_finite(input_rms, &format!("{prefix}.input_rms"))?;
        }
        require_finite(self.band.0, &format!("{prefix}.band.0"))?;
        require_finite(self.band.1, &format!("{prefix}.band.1"))?;
        for (idx, row) in self.rows.iter().enumerate() {
            row.validate(&format!("{prefix}.rows[{idx}]"))?;
        }
        Ok(())
    }
}

impl From<&NoiseSummary> for ProjectNoiseSummary {
    fn from(summary: &NoiseSummary) -> Self {
        Self {
            rows: summary
                .rows
                .iter()
                .map(ProjectNoiseContributorRow::from)
                .collect(),
            total_rms: summary.total_rms,
            input_rms: summary.input_rms,
            band: summary.band,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectNoiseContributorRow {
    pub device: String,
    pub mechanism: String,
    pub power: f64,
    pub share_pct: f64,
}

impl ProjectNoiseContributorRow {
    fn into_row(self) -> NoiseContributorRow {
        NoiseContributorRow {
            device: self.device,
            mechanism: self.mechanism,
            power: self.power,
            share_pct: self.share_pct,
        }
    }

    fn validate(&self, prefix: &str) -> Result<(), String> {
        require_noise_mechanism(&self.mechanism, &format!("{prefix}.mechanism"))?;
        require_finite(self.power, &format!("{prefix}.power"))?;
        require_finite(self.share_pct, &format!("{prefix}.share_pct"))
    }
}

impl From<&NoiseContributorRow> for ProjectNoiseContributorRow {
    fn from(row: &NoiseContributorRow) -> Self {
        Self {
            device: row.device.clone(),
            mechanism: row.mechanism.to_string(),
            power: row.power,
            share_pct: row.share_pct,
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ProjectDeviceOpReport {
    #[serde(default)]
    pub entries: Vec<ProjectDeviceOpEntry>,
}

impl ProjectDeviceOpReport {
    fn into_report(self) -> rspice_core::circuit::DeviceOpReport {
        rspice_core::circuit::DeviceOpReport {
            entries: self
                .entries
                .into_iter()
                .map(ProjectDeviceOpEntry::into_entry)
                .collect(),
        }
    }

    fn validate(&self, prefix: &str) -> Result<(), String> {
        for (idx, entry) in self.entries.iter().enumerate() {
            entry.validate(&format!("{prefix}.entries[{idx}]"))?;
        }
        Ok(())
    }
}

impl From<&rspice_core::circuit::DeviceOpReport> for ProjectDeviceOpReport {
    fn from(report: &rspice_core::circuit::DeviceOpReport) -> Self {
        Self {
            entries: report
                .entries
                .iter()
                .map(ProjectDeviceOpEntry::from)
                .collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectDeviceOpEntry {
    pub name: String,
    pub device_kind: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(default)]
    pub params: Vec<ProjectNamedValue>,
}

impl ProjectDeviceOpEntry {
    fn into_entry(self) -> rspice_core::circuit::DeviceOpEntry {
        rspice_core::circuit::DeviceOpEntry {
            name: self.name,
            device_kind: intern_static_label(self.device_kind),
            region: self.region.map(intern_static_label),
            params: self
                .params
                .into_iter()
                .map(|param| (intern_static_label(param.name), param.value))
                .collect(),
        }
    }

    fn validate(&self, prefix: &str) -> Result<(), String> {
        require_static_label(&self.device_kind, &format!("{prefix}.device_kind"))?;
        if let Some(region) = &self.region {
            require_static_label(region, &format!("{prefix}.region"))?;
        }
        for (idx, param) in self.params.iter().enumerate() {
            param.validate(&format!("{prefix}.params[{idx}]"))?;
        }
        Ok(())
    }
}

impl From<&rspice_core::circuit::DeviceOpEntry> for ProjectDeviceOpEntry {
    fn from(entry: &rspice_core::circuit::DeviceOpEntry) -> Self {
        Self {
            name: entry.name.clone(),
            device_kind: entry.device_kind.to_string(),
            region: entry.region.map(str::to_string),
            params: entry
                .params
                .iter()
                .map(|(name, value)| ProjectNamedValue {
                    name: (*name).to_string(),
                    value: *value,
                })
                .collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectNamedValue {
    pub name: String,
    pub value: f64,
}

impl ProjectNamedValue {
    fn validate(&self, prefix: &str) -> Result<(), String> {
        require_static_label(&self.name, &format!("{prefix}.name"))?;
        require_finite(self.value, &format!("{prefix}.value"))
    }
}

fn is_false(value: &bool) -> bool {
    !*value
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectMeasurement {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub raw_value: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    pub passed: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expected: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tolerance: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failure_limit: Option<f64>,
    #[serde(default, skip_serializing_if = "is_false")]
    pub failure_limit_exceeded: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event_axis: Option<f64>,
}

impl ProjectMeasurement {
    fn into_measurement(self) -> rspice_core::MeasureResult {
        rspice_core::MeasureResult {
            name: self.name,
            value: self.value,
            raw_value: self.raw_value,
            error: self.error,
            passed: self.passed,
            expected: self.expected,
            tolerance: self.tolerance,
            failure_limit: self.failure_limit,
            failure_limit_exceeded: self.failure_limit_exceeded,
            event_axis: self.event_axis,
        }
    }

    fn validate(&self, prefix: &str) -> Result<(), String> {
        require_optional_finite(self.value, &format!("{prefix}.value"))?;
        require_optional_finite(self.raw_value, &format!("{prefix}.raw_value"))?;
        if self.value.is_some() != self.raw_value.is_some() {
            return Err(format!(
                "{prefix}.raw_value must be present exactly when value is present"
            ));
        }
        require_optional_finite(self.expected, &format!("{prefix}.expected"))?;
        require_optional_finite(self.tolerance, &format!("{prefix}.tolerance"))?;
        require_optional_finite(self.failure_limit, &format!("{prefix}.failure_limit"))?;
        let expected_exceeded = match (self.raw_value, self.failure_limit) {
            (Some(raw_value), Some(limit)) => raw_value.abs() >= limit,
            _ => false,
        };
        if self.failure_limit_exceeded != expected_exceeded {
            return Err(format!(
                "{prefix}.failure_limit_exceeded does not match abs(raw_value) >= failure_limit"
            ));
        }
        if self.failure_limit_exceeded && self.passed {
            return Err(format!(
                "{prefix} cannot pass after its FAILVALUE limit was reached"
            ));
        }
        require_optional_finite(self.event_axis, &format!("{prefix}.event_axis"))?;
        Ok(())
    }
}

impl From<&rspice_core::MeasureResult> for ProjectMeasurement {
    fn from(measurement: &rspice_core::MeasureResult) -> Self {
        Self {
            name: measurement.name.clone(),
            value: measurement.value,
            raw_value: measurement.raw_value,
            error: measurement.error.clone(),
            passed: measurement.passed,
            expected: measurement.expected,
            tolerance: measurement.tolerance,
            failure_limit: measurement.failure_limit,
            failure_limit_exceeded: measurement.failure_limit_exceeded,
            event_axis: measurement.event_axis,
        }
    }
}
