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

    /// These results as their content fingerprint reads them: without the
    /// runs' logs.
    ///
    /// A run's log is its account of itself, not result content, so two
    /// histories that differ only in their logs are the same history.
    /// Borrowed when no run has a log, which is every history written before
    /// runs kept one, so their fingerprints do not move.
    pub fn without_run_logs(&self) -> std::borrow::Cow<'_, Self> {
        if self.runs.iter().all(|run| run.log.is_empty()) {
            return std::borrow::Cow::Borrowed(self);
        }
        let mut stripped = self.clone();
        for run in &mut stripped.runs {
            run.log = RunLog::default();
        }
        std::borrow::Cow::Owned(stripped)
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

    /// Consume a validated result document.
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
}
