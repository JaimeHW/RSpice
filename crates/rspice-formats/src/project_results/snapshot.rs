//! Immutable result snapshots with validation bound to their exact content.
//!
//! Clones share complete retained records. Mutable field access detaches shared
//! content and discards validation before exposing any metadata or samples.
//! Only persisted data crosses serialization; a loaded file must validate anew.

use std::ops::{Deref, DerefMut};
use std::sync::{Arc, OnceLock};

use super::*;

/// Shared project-file results. Runtime validation never participates in the
/// wire format, content equality, or numerical provenance.
#[derive(Debug, Clone, Default)]
pub struct ProjectSimulationResults {
    content: Arc<SnapshotContent>,
}

/// Portable retained values restored from a validated project result document.
/// Host selection repair and disposable display caches remain with the caller.
#[non_exhaustive]
pub struct RestoredProjectSimulationResults<A> {
    pub runs: Vec<SimulationRun<A>>,
    pub next_run_id: u64,
    pub retained_dataset_limit: Option<usize>,
    pub active_run_stable_id: Option<RunId>,
    pub active_dataset_id: Option<DatasetId>,
    pub active_analysis_sequence: Option<u64>,
    pub overlay_dataset_ids: Vec<DatasetId>,
    pub executed_decks: rspice_results::executed_deck::ExecutedDeckArchive,
    pub imported_monte_carlo_checkpoints:
        rspice_results::monte_carlo_checkpoint::MonteCarloCheckpointLibrary,
    pub imported_transient_checkpoints:
        rspice_results::transient_checkpoint::TransientCheckpointLibrary,
}

#[derive(Debug, Clone, Default)]
struct SnapshotContent {
    data: ProjectSimulationResultsData,
    validation: OnceLock<Result<(), String>>,
}

impl From<ProjectSimulationResultsData> for ProjectSimulationResults {
    fn from(data: ProjectSimulationResultsData) -> Self {
        Self {
            content: Arc::new(SnapshotContent {
                data,
                validation: OnceLock::new(),
            }),
        }
    }
}

impl Deref for ProjectSimulationResults {
    type Target = ProjectSimulationResultsData;

    fn deref(&self) -> &Self::Target {
        &self.content.data
    }
}

impl DerefMut for ProjectSimulationResults {
    fn deref_mut(&mut self) -> &mut Self::Target {
        let content = Arc::make_mut(&mut self.content);
        content.validation.take();
        &mut content.data
    }
}

impl PartialEq for ProjectSimulationResults {
    fn eq(&self, other: &Self) -> bool {
        self.content.data == other.content.data
    }
}

impl Serialize for ProjectSimulationResults {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.content.data.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for ProjectSimulationResults {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        ProjectSimulationResultsData::deserialize(deserializer).map(Into::into)
    }
}

impl ProjectSimulationResults {
    pub fn is_empty(&self) -> bool {
        self.content.data.is_empty()
    }

    pub fn shares_content_with(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.content, &other.content)
    }

    pub fn validate(&self) -> Result<(), String> {
        self.content
            .validation
            .get_or_init(|| self.content.data.validate())
            .clone()
    }

    /// Consume an already-current, validated result document. Legacy migration
    /// requires the owning [`ProjectId`] and must be completed explicitly at
    /// the project/session boundary before this method is called.
    pub fn into_validated_data(self) -> Result<ProjectSimulationResultsData, String> {
        self.validate()?;
        Ok(Arc::unwrap_or_clone(self.content).data)
    }

    /// Validate and restore portable history before any host state is replaced.
    /// The caller decorates analyses without changing the persisted source checks.
    pub fn restore_with<A, W: AsRef<RetainedWaveform>>(
        self,
        mut restore_analysis: impl FnMut(ProjectAnalysisResult) -> Result<A, String>,
    ) -> Result<RestoredProjectSimulationResults<A>, String>
    where
        A: AsRef<AnalysisResult<W>> + AsMut<AnalysisResult<W>>,
    {
        let data = self.into_validated_data()?;
        let executed_decks = data.executed_decks.into_archive()?;
        let runs = data
            .runs
            .into_iter()
            .map(|run| run.into_run_with(&mut restore_analysis))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(RestoredProjectSimulationResults {
            runs,
            next_run_id: data.next_run_id,
            retained_dataset_limit: data.retained_dataset_limit,
            active_run_stable_id: data.active_run_stable_id,
            active_dataset_id: data.active_dataset_id,
            active_analysis_sequence: data.active_analysis_sequence,
            overlay_dataset_ids: data.overlay_dataset_ids,
            executed_decks,
            imported_monte_carlo_checkpoints: data.imported_monte_carlo_checkpoints,
            imported_transient_checkpoints: data.imported_transient_checkpoints,
        })
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
    /// v9 digests are likewise authenticated before SOA evidence
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
    pub fn migrate_to_current(&mut self, project_id: ProjectId) -> Result<(), String> {
        if self.schema_version == PROJECT_SIMULATION_RESULTS_SCHEMA_VERSION {
            return Ok(());
        }
        let mut candidate = self.clone();
        candidate.migrate_to_current_in_place(project_id)?;
        *self = candidate;
        Ok(())
    }
}
