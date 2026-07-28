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
            active_run_stable_id: None,
            active_dataset_id: None,
            active_analysis_sequence: None,
            overlay_dataset_ids: Vec::new(),
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
            && self.active_run_id.is_none()
            && self.active_analysis_id.is_none()
            && self.overlay_run_ids.is_empty()
    }

    pub fn from_state(state: &SimulationState) -> Self {
        if state.runs.is_empty() {
            return Self::default();
        }

        let runs: Vec<_> = state.runs.iter().map(ProjectSimulationRun::from).collect();
        let max_run_id = state.runs.iter().map(|run| run.id).max().unwrap_or(0);
        Self {
            schema_version: PROJECT_SIMULATION_RESULTS_SCHEMA_VERSION,
            runs,
            next_run_id: state.next_run_id.max(max_run_id),
            active_run_stable_id: state.active_run().map(|run| run.run_id),
            active_dataset_id: state.active_run().map(|run| run.dataset_id),
            active_analysis_sequence: state.active_analysis().map(|analysis| analysis.id),
            overlay_dataset_ids: state.overlay_dataset_ids.clone(),
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
        let runs = self
            .runs
            .into_iter()
            .map(ProjectSimulationRun::into_run)
            .collect::<Result<Vec<_>, _>>()?;
        state.restore_run_history(
            runs,
            self.next_run_id,
            self.active_run_stable_id,
            self.active_dataset_id,
            self.active_analysis_sequence,
            self.overlay_dataset_ids,
        );
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
    /// and input-referred totals are admitted. Each migrated result is then
    /// resealed with the current encoding.
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
        if source_schema == TRANSFER_FUNCTION_RESULTS_SCHEMA_VERSION {
            for run in &mut self.runs {
                validate_v11_result_digests(run)?;
                seal_project_result_digests(run)?;
            }
            self.schema_version = PROJECT_SIMULATION_RESULTS_SCHEMA_VERSION;
            return self.validate();
        }
        if source_schema == RELIABILITY_SOA_RESULTS_SCHEMA_VERSION {
            for run in &mut self.runs {
                validate_v10_result_digests(run)?;
                seal_project_result_digests(run)?;
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
                seal_project_result_digests(run)?;
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
                seal_project_result_digests(run)?;
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
                seal_project_result_digests(run)?;
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
            seal_project_result_digests(run)?;
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
        Ok(())
    }
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

/// Authenticate a schema-v8 run with the exact digest encoding that wrote it.
/// This must run before any v9 fields are introduced or digests are resealed.
fn validate_v8_result_digests(run: &ProjectSimulationRun) -> Result<(), String> {
    validate_legacy_noise_summary_shape(run, CONTENT_DIGEST_RESULTS_SCHEMA_VERSION)?;
    for analysis in &run.analyses {
        if analysis.result_payload.is_present() {
            return Err(format!(
                "schema-v8 analysis {} contains a typed result payload introduced by schema v9",
                analysis.id
            ));
        }
        let retained = analysis
            .result_data_digest
            .as_ref()
            .copied()
            .ok_or_else(|| {
                format!(
                    "schema-v8 analysis {} is missing its result data digest",
                    analysis.id
                )
            })?;
        let computed = analysis
            .clone()
            .into_analysis()?
            .legacy_v1_result_data_digest();
        if retained != computed {
            return Err(format!(
                "schema-v8 analysis {} result data digest does not match retained content",
                analysis.id
            ));
        }
    }

    let retained = run
        .dataset_content_digest
        .as_ref()
        .copied()
        .ok_or_else(|| {
            format!(
                "schema-v8 simulation run {} is missing its dataset content digest",
                run.id
            )
        })?;
    let computed = run.clone().into_run()?.legacy_v1_dataset_content_digest();
    if retained != computed {
        return Err(format!(
            "schema-v8 simulation run {} dataset content digest does not match retained content",
            run.id
        ));
    }
    Ok(())
}

/// Authenticate a schema-v9 run with the exact typed-payload digest encoding
/// that wrote it before schema-v10 Reliability/SOA evidence is admitted.
fn validate_v9_result_digests(run: &ProjectSimulationRun) -> Result<(), String> {
    validate_legacy_noise_summary_shape(run, TYPED_PAYLOAD_RESULTS_SCHEMA_VERSION)?;
    reject_legacy_operating_point_evidence(run, TYPED_PAYLOAD_RESULTS_SCHEMA_VERSION)?;
    for analysis in &run.analyses {
        if matches!(
            analysis.result_payload.as_ref(),
            Some(AnalysisResultPayload::Reliability { .. })
                | Some(AnalysisResultPayload::Soa { .. })
        ) {
            return Err(format!(
                "schema-v9 analysis {} contains Reliability/SOA evidence introduced by schema v10",
                analysis.id
            ));
        }
        if matches!(
            analysis.result_payload.as_ref(),
            Some(AnalysisResultPayload::TransferFunction { .. })
        ) {
            return Err(format!(
                "schema-v9 analysis {} contains transfer-function evidence introduced by schema v11",
                analysis.id
            ));
        }
        let retained = analysis
            .result_data_digest
            .as_ref()
            .copied()
            .ok_or_else(|| {
                format!(
                    "schema-v9 analysis {} is missing its result data digest",
                    analysis.id
                )
            })?;
        let computed = analysis
            .clone()
            .into_analysis()?
            .legacy_v2_result_data_digest();
        if retained != computed {
            return Err(format!(
                "schema-v9 analysis {} result data digest does not match retained content",
                analysis.id
            ));
        }
    }

    let retained = run
        .dataset_content_digest
        .as_ref()
        .copied()
        .ok_or_else(|| {
            format!(
                "schema-v9 simulation run {} is missing its dataset content digest",
                run.id
            )
        })?;
    let computed = run.clone().into_run()?.legacy_v2_dataset_content_digest();
    if retained != computed {
        return Err(format!(
            "schema-v9 simulation run {} dataset content digest does not match retained content",
            run.id
        ));
    }
    Ok(())
}

/// Authenticate a schema-v10 run with the exact Reliability/SOA-capable
/// digest encoding that wrote it. Typed TF evidence is a schema-v11 field and
/// must be rejected before the authenticated v10 document is resealed.
fn validate_v10_result_digests(run: &ProjectSimulationRun) -> Result<(), String> {
    validate_legacy_noise_summary_shape(run, RELIABILITY_SOA_RESULTS_SCHEMA_VERSION)?;
    reject_legacy_operating_point_evidence(run, RELIABILITY_SOA_RESULTS_SCHEMA_VERSION)?;
    for analysis in &run.analyses {
        if matches!(
            analysis.result_payload.as_ref(),
            Some(AnalysisResultPayload::TransferFunction { .. })
        ) {
            return Err(format!(
                "schema-v10 analysis {} contains transfer-function evidence introduced by schema v11",
                analysis.id
            ));
        }
        let retained = analysis
            .result_data_digest
            .as_ref()
            .copied()
            .ok_or_else(|| {
                format!(
                    "schema-v10 analysis {} is missing its result data digest",
                    analysis.id
                )
            })?;
        let computed = analysis
            .clone()
            .into_analysis()?
            .legacy_v3_result_data_digest();
        if retained != computed {
            return Err(format!(
                "schema-v10 analysis {} result data digest does not match retained content",
                analysis.id
            ));
        }
    }

    let retained = run
        .dataset_content_digest
        .as_ref()
        .copied()
        .ok_or_else(|| {
            format!(
                "schema-v10 simulation run {} is missing its dataset content digest",
                run.id
            )
        })?;
    let computed = run.clone().into_run()?.legacy_v3_dataset_content_digest();
    if retained != computed {
        return Err(format!(
            "schema-v10 simulation run {} dataset content digest does not match retained content",
            run.id
        ));
    }
    Ok(())
}

/// Authenticate a schema-v11 run before admitting schema-v12 operating-point
/// payloads and optional/input-referred integrated noise evidence.
fn validate_v11_result_digests(run: &ProjectSimulationRun) -> Result<(), String> {
    validate_legacy_noise_summary_shape(run, TRANSFER_FUNCTION_RESULTS_SCHEMA_VERSION)?;
    reject_legacy_operating_point_evidence(run, TRANSFER_FUNCTION_RESULTS_SCHEMA_VERSION)?;
    for analysis in &run.analyses {
        let retained = analysis
            .result_data_digest
            .as_ref()
            .copied()
            .ok_or_else(|| {
                format!(
                    "schema-v11 analysis {} is missing its result data digest",
                    analysis.id
                )
            })?;
        let computed = analysis
            .clone()
            .into_analysis()?
            .legacy_v4_result_data_digest();
        if retained != computed {
            return Err(format!(
                "schema-v11 analysis {} result data digest does not match retained content",
                analysis.id
            ));
        }
    }

    let retained = run
        .dataset_content_digest
        .as_ref()
        .copied()
        .ok_or_else(|| {
            format!(
                "schema-v11 simulation run {} is missing its dataset content digest",
                run.id
            )
        })?;
    let computed = run.clone().into_run()?.legacy_v4_dataset_content_digest();
    if retained != computed {
        return Err(format!(
            "schema-v11 simulation run {} dataset content digest does not match retained content",
            run.id
        ));
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

pub(super) fn seal_project_result_digests(run: &mut ProjectSimulationRun) -> Result<(), String> {
    for analysis in &mut run.analyses {
        let digest = analysis.clone().into_analysis()?.result_data_digest();
        analysis.result_data_digest = PersistedField::Value(digest);
    }
    let digest = run.clone().into_run()?.dataset_content_digest();
    run.dataset_content_digest = PersistedField::Value(digest);
    Ok(())
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProjectRunProvenanceMode {
    /// Missing only while reading schemas older than v4. Current-schema
    /// validation rejects this state.
    #[default]
    Unspecified,
    /// Results written before prepared analysis-instance provenance existed.
    LegacyUnattributed,
    /// Prepared-task identity was stored before its source domain was
    /// persisted. No plan/manual origin is inferred during migration.
    LegacyPreparedUnclassified,
    /// Every result is bound to one authenticated prepared task.
    PreparedTaskBound,
}

fn set_legacy_unclassified_source_domains(run: &mut ProjectSimulationRun) {
    for provenance in run
        .analyses
        .iter_mut()
        .filter_map(|analysis| analysis.provenance.as_mut())
    {
        provenance.source_domain =
            PersistedField::Value(AnalysisResultSourceDomain::LegacyUnclassified);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "digest", rename_all = "snake_case")]
pub enum ProjectPreparedSourceCheckReceipt {
    SchematicDrc(ContentDigest),
    ManualSourceCheck(ContentDigest),
}

impl From<PreparedSourceCheckReceipt> for ProjectPreparedSourceCheckReceipt {
    fn from(receipt: PreparedSourceCheckReceipt) -> Self {
        match receipt {
            PreparedSourceCheckReceipt::SchematicDrc(digest) => Self::SchematicDrc(digest),
            PreparedSourceCheckReceipt::ManualSourceCheck(digest) => {
                Self::ManualSourceCheck(digest)
            }
        }
    }
}

impl From<ProjectPreparedSourceCheckReceipt> for PreparedSourceCheckReceipt {
    fn from(receipt: ProjectPreparedSourceCheckReceipt) -> Self {
        match receipt {
            ProjectPreparedSourceCheckReceipt::SchematicDrc(digest) => Self::SchematicDrc(digest),
            ProjectPreparedSourceCheckReceipt::ManualSourceCheck(digest) => {
                Self::ManualSourceCheck(digest)
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProjectPreparedRunTaskReceipt {
    pub source_instance_id: AnalysisInstanceId,
    pub source_revision: ObjectRevision,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dependency_ids: Vec<AnalysisInstanceId>,
    pub analysis_kind_tag: u8,
    pub config_digest: ContentDigest,
}

impl ProjectPreparedRunTaskReceipt {
    pub(super) fn into_receipt(self) -> Result<PreparedRunTaskReceipt, String> {
        PreparedRunTaskReceipt::new(
            self.source_instance_id,
            self.source_revision,
            self.dependency_ids,
            self.analysis_kind_tag,
            self.config_digest,
        )
    }
}

impl From<&PreparedRunTaskReceipt> for ProjectPreparedRunTaskReceipt {
    fn from(receipt: &PreparedRunTaskReceipt) -> Self {
        Self {
            source_instance_id: receipt.instance_id(),
            source_revision: receipt.source_revision(),
            dependency_ids: receipt.dependencies().to_vec(),
            analysis_kind_tag: receipt.analysis_kind_tag(),
            config_digest: receipt.config_digest(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProjectPreparedModelSourceIdentity {
    pub source_id: ModelSourceId,
    pub model_name: String,
    pub revision: ObjectRevision,
    pub content_digest: ContentDigest,
}

impl ProjectPreparedModelSourceIdentity {
    fn into_identity(self) -> Result<PreparedModelSourceIdentity, String> {
        PreparedModelSourceIdentity::new(
            self.source_id,
            self.model_name,
            self.revision,
            self.content_digest,
        )
    }
}

impl From<&PreparedModelSourceIdentity> for ProjectPreparedModelSourceIdentity {
    fn from(identity: &PreparedModelSourceIdentity) -> Self {
        Self {
            source_id: identity.source_id(),
            model_name: identity.model_name().to_owned(),
            revision: identity.revision(),
            content_digest: identity.content_digest(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProjectPreparedRunReceipt {
    pub source_domain: AnalysisResultSourceDomain,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub simulation_plan_id: Option<SimulationPlanId>,
    pub project_revision: ObjectRevision,
    pub prepared_snapshot_digest: ContentDigest,
    pub source_content_digest: ContentDigest,
    pub source_check_receipt: ProjectPreparedSourceCheckReceipt,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub project_model_sources: Vec<ProjectPreparedModelSourceIdentity>,
    pub tasks: Vec<ProjectPreparedRunTaskReceipt>,
}

impl ProjectPreparedRunReceipt {
    pub(super) fn into_receipt(self) -> Result<PreparedRunReceipt, String> {
        let project_model_sources = self
            .project_model_sources
            .into_iter()
            .map(ProjectPreparedModelSourceIdentity::into_identity)
            .collect::<Result<Vec<_>, _>>()?;
        let tasks = self
            .tasks
            .into_iter()
            .map(ProjectPreparedRunTaskReceipt::into_receipt)
            .collect::<Result<Vec<_>, _>>()?;
        PreparedRunReceipt::new_with_project_model_sources(
            self.source_domain,
            self.simulation_plan_id,
            self.project_revision,
            self.prepared_snapshot_digest,
            self.source_content_digest,
            self.source_check_receipt.into(),
            project_model_sources,
            tasks,
        )
    }

    fn validate(&self) -> Result<(), String> {
        self.clone().into_receipt().map(|_| ())
    }
}

impl From<&PreparedRunReceipt> for ProjectPreparedRunReceipt {
    fn from(receipt: &PreparedRunReceipt) -> Self {
        Self {
            source_domain: receipt.source_domain(),
            simulation_plan_id: receipt.simulation_plan_id(),
            project_revision: receipt.project_revision(),
            prepared_snapshot_digest: receipt.prepared_snapshot_digest(),
            source_content_digest: receipt.source_content_digest(),
            source_check_receipt: receipt.source_check_receipt().into(),
            project_model_sources: receipt
                .project_model_sources()
                .iter()
                .map(ProjectPreparedModelSourceIdentity::from)
                .collect(),
            tasks: receipt
                .tasks()
                .iter()
                .map(ProjectPreparedRunTaskReceipt::from)
                .collect(),
        }
    }
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
    #[serde(default)]
    pub elapsed_time: f64,
    #[serde(default = "default_true")]
    pub success: bool,
}

impl ProjectSimulationRun {
    pub(super) fn into_run(self) -> Result<SimulationRun, String> {
        let run_id = self
            .run_id
            .ok_or_else(|| format!("simulation run sequence {} has no stable id", self.id))?;
        let dataset_id = self
            .dataset_id
            .ok_or_else(|| format!("simulation run sequence {} has no dataset id", self.id))?;
        let analyses = self
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
            Some(ProjectRunProvenanceMode::PreparedTaskBound) => SimulationRunProvenance::Prepared(
                self.prepared_receipt
                    .into_value()
                    .ok_or_else(|| {
                        format!(
                            "simulation run sequence {} is prepared-task-bound but has no receipt",
                            self.id
                        )
                    })?
                    .into_receipt()?,
            ),
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
        run.lifecycle = match restored_lifecycle {
            SimulationRunLifecycle::Preparing
            | SimulationRunLifecycle::Running
            | SimulationRunLifecycle::Cancelling => SimulationRunLifecycle::Interrupted,
            lifecycle => lifecycle,
        };
        run.label = self.label;
        run.timestamp = self.timestamp;
        run.analyses = analyses;
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
        run.restore_provenance(provenance)?;
        Ok(run)
    }

    fn validate(&self, run_idx: usize) -> Result<(), String> {
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
                PersistedField::Value(ProjectPreparedRunReceipt::from(receipt)),
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
            elapsed_time: run.elapsed_time,
            success,
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
}

impl ProjectAnalysisResultProvenance {
    fn into_provenance(self) -> Result<AnalysisResultProvenance, String> {
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
    }

    fn validate(&self) -> Result<(), String> {
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub complex: Option<ProjectComplexWaveformComponents>,
}

impl ProjectWaveformData {
    fn into_waveform(self) -> WaveformData {
        let mut waveform = WaveformData::new(self.name, self.x, self.y, self.color);
        waveform.visible = self.visible;
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
        require_static_label(&self.mechanism, &format!("{prefix}.mechanism"))?;
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectMeasurement {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    pub passed: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expected: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tolerance: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event_axis: Option<f64>,
}

impl ProjectMeasurement {
    fn into_measurement(self) -> rspice_core::MeasureResult {
        rspice_core::MeasureResult {
            name: self.name,
            value: self.value,
            error: self.error,
            passed: self.passed,
            expected: self.expected,
            tolerance: self.tolerance,
            event_axis: self.event_axis,
        }
    }

    fn validate(&self, prefix: &str) -> Result<(), String> {
        require_optional_finite(self.value, &format!("{prefix}.value"))?;
        require_optional_finite(self.expected, &format!("{prefix}.expected"))?;
        require_optional_finite(self.tolerance, &format!("{prefix}.tolerance"))?;
        require_optional_finite(self.event_axis, &format!("{prefix}.event_axis"))
    }
}

impl From<&rspice_core::MeasureResult> for ProjectMeasurement {
    fn from(measurement: &rspice_core::MeasureResult) -> Self {
        Self {
            name: measurement.name.clone(),
            value: measurement.value,
            error: measurement.error.clone(),
            passed: measurement.passed,
            expected: measurement.expected,
            tolerance: measurement.tolerance,
            event_axis: measurement.event_axis,
        }
    }
}
