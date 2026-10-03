//! Stable viewer selections over exact retained result identities.
use rspice_app_types::product::DatasetId;
use rspice_results::{
    analysis_result::AnalysisResult,
    result_presentation::{AnalysisPresentationKey, TracePresentationKey, WaveformPresentationKey},
    run::SimulationRun,
    waveform::RetainedWaveform,
};

/// Stable identity of one retained source waveform whose quick-view
/// visibility has been overridden for this session.
///
/// The solver-owned [`WaveformData`](crate::waveform::WaveformData) remains an
/// immutable result. A visibility click records presentation state against
/// the dataset, authored analysis, and source name instead of rewriting the
/// retained run (or its legacy live projection).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SourceWaveformPresentationKey {
    analysis: AnalysisPresentationKey,
    source_name: String,
}

impl SourceWaveformPresentationKey {
    pub fn new(analysis: AnalysisPresentationKey, source_name: impl Into<String>) -> Self {
        Self {
            analysis,
            source_name: source_name.into(),
        }
    }

    pub fn resolve<'a, R, A, W>(&self, runs: &'a [R]) -> Option<(usize, usize, usize, &'a W)>
    where
        R: AsRef<SimulationRun<A>>,
        A: AsRef<AnalysisResult<W>> + 'a,
        W: AsRef<RetainedWaveform> + 'a,
    {
        let (run_index, run) = runs
            .iter()
            .enumerate()
            .find(|(_, run)| run.as_ref().dataset_id == self.analysis.dataset_id())?;
        let (analysis_index, analysis) = self.analysis.resolve(run.as_ref())?;
        let mut matches = analysis
            .as_ref()
            .waveforms
            .iter()
            .enumerate()
            .filter(|(_, waveform)| waveform.as_ref().name == self.source_name);
        let (waveform_index, waveform) = matches.next()?;
        matches
            .next()
            .is_none()
            .then_some((run_index, analysis_index, waveform_index, waveform))
    }

    pub const fn analysis(&self) -> AnalysisPresentationKey {
        self.analysis
    }
}

/// Stable identity of a non-waveform quantity retained by one immutable
/// analysis: scalar evidence, an exact array, an event stream, a contribution
/// table, or family metadata. `canonical_name` is producer-authored inventory
/// identity, not a translated display label.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ResultArtifactPresentationKey {
    analysis: AnalysisPresentationKey,
    canonical_name: String,
}

/// Stable identity of one user-authored expression within a retained
/// analysis. Expression text is unique within its analysis and is also the
/// calculator source, so it survives insertion/removal of neighboring rows.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ResultExpressionPresentationKey {
    analysis: AnalysisPresentationKey,
    text: String,
}

impl ResultExpressionPresentationKey {
    pub fn new(analysis: AnalysisPresentationKey, text: impl Into<String>) -> Self {
        Self {
            analysis,
            text: text.into(),
        }
    }

    pub const fn analysis(&self) -> AnalysisPresentationKey {
        self.analysis
    }

    pub fn text(&self) -> &str {
        &self.text
    }
}

impl ResultArtifactPresentationKey {
    pub fn new(analysis: AnalysisPresentationKey, canonical_name: impl Into<String>) -> Self {
        Self {
            analysis,
            canonical_name: canonical_name.into(),
        }
    }

    pub fn analysis(&self) -> AnalysisPresentationKey {
        self.analysis
    }

    pub fn canonical_name(&self) -> &str {
        &self.canonical_name
    }

    pub fn resolve<'a, R, A, W>(&self, runs: &'a [R]) -> Option<(usize, usize, &'a A)>
    where
        R: AsRef<SimulationRun<A>>,
        A: AsRef<AnalysisResult<W>> + 'a,
        W: AsRef<RetainedWaveform> + 'a,
    {
        let (run_index, run) = runs
            .iter()
            .enumerate()
            .find(|(_, run)| run.as_ref().dataset_id == self.analysis.dataset_id())?;
        let (analysis_index, analysis) = self.analysis.resolve(run.as_ref())?;
        Some((run_index, analysis_index, analysis))
    }
}

/// One stable row identity in the mixed typed Data Browser inventory.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ResultBrowserSelectionKey {
    Waveform(SourceWaveformPresentationKey),
    Artifact(ResultArtifactPresentationKey),
}

impl ResultBrowserSelectionKey {
    pub fn waveform(&self) -> Option<&SourceWaveformPresentationKey> {
        match self {
            Self::Waveform(key) => Some(key),
            Self::Artifact(_) => None,
        }
    }

    pub fn dataset_id(&self) -> DatasetId {
        match self {
            Self::Waveform(key) => key.analysis().dataset_id(),
            Self::Artifact(key) => key.analysis().dataset_id(),
        }
    }
}

impl From<SourceWaveformPresentationKey> for ResultBrowserSelectionKey {
    fn from(value: SourceWaveformPresentationKey) -> Self {
        Self::Waveform(value)
    }
}

impl From<ResultArtifactPresentationKey> for ResultBrowserSelectionKey {
    fn from(value: ResultArtifactPresentationKey) -> Self {
        Self::Artifact(value)
    }
}

/// Stable session identity for a selected retained waveform.
///
/// The immutable dataset, prepared analysis identity, and source waveform
/// name survive retained-result reordering. Duplicate source names fail
/// closed instead of silently inheriting an old ordinal selection.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SelectedResultTrace {
    waveform: WaveformPresentationKey,
}

impl SelectedResultTrace {
    pub fn from_identity(
        analysis: AnalysisPresentationKey,
        source_name: impl Into<String>,
    ) -> Self {
        Self {
            waveform: WaveformPresentationKey {
                analysis,
                trace: TracePresentationKey {
                    source_name: source_name.into(),
                    kind: 0,
                    family_group: 0,
                },
            },
        }
    }

    pub fn from_run_indices<A, W>(
        run: &SimulationRun<A>,
        analysis_index: usize,
        waveform_index: usize,
    ) -> Option<Self>
    where
        A: AsRef<AnalysisResult<W>>,
        W: AsRef<RetainedWaveform>,
    {
        let analysis = run.analyses.get(analysis_index)?.as_ref();
        let waveform = analysis.waveforms.get(waveform_index)?;
        Some(Self::from_identity(
            AnalysisPresentationKey::new(run.dataset_id, analysis),
            waveform.as_ref().name.clone(),
        ))
    }

    pub const fn analysis_key(&self) -> AnalysisPresentationKey {
        self.waveform.analysis
    }

    pub fn source_name(&self) -> &str {
        &self.waveform.trace.source_name
    }

    pub fn table_binding(&self) -> (AnalysisPresentationKey, TracePresentationKey) {
        (self.waveform.analysis, self.waveform.trace.clone())
    }

    pub const fn dataset_id(&self) -> DatasetId {
        self.waveform.analysis.dataset_id()
    }

    pub fn resolve<'a, A, W>(
        &self,
        run: &'a SimulationRun<A>,
    ) -> Option<(usize, usize, &'a A, &'a W)>
    where
        A: AsRef<AnalysisResult<W>> + 'a,
        W: AsRef<RetainedWaveform> + 'a,
    {
        let (analysis_index, analysis) = self.waveform.analysis.resolve(run)?;
        let mut matching = analysis
            .as_ref()
            .waveforms
            .iter()
            .enumerate()
            .filter(|(_, waveform)| waveform.as_ref().name == self.waveform.trace.source_name);
        let (waveform_index, waveform) = matching.next()?;
        matching
            .next()
            .is_none()
            .then_some((analysis_index, waveform_index, analysis, waveform))
    }
}
