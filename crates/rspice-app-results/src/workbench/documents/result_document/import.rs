//! The draft an external result dataset is imported through, before it is
//! committed to the retained history.

/// Step currently presented by the external-result import transaction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResultImportStage {
    Detect,
    Map,
    Validate,
}

/// Runtime-only, non-mutating draft for importing an external result dataset.
///
/// Parsed samples remain outside `SimulationState` until the user reaches the
/// final commit action. `Arc` keeps workbench/window projection clones cheap
/// even for a large import preview.
#[derive(Debug, Clone)]
pub struct ResultImportDialogState {
    pub open: bool,
    pub stage: ResultImportStage,
    pub source_name: String,
    /// Canonical ID from the result-data interchange contract. This is kept
    /// with the draft so review never has to infer the format a second time.
    pub source_format_id: String,
    pub delimiter: u8,
    pub analysis_type: crate::state::AnalysisType,
    pub coordinate_name: String,
    pub coordinate_unit: Option<String>,
    pub sample_count: usize,
    pub waveforms: std::sync::Arc<Vec<crate::state::RetainedTrace>>,
    /// Analysis-family authority recovered from the source (for example,
    /// Touchstone per-port reference impedances). It is committed atomically
    /// with the selected waveforms or discarded with the draft.
    pub family_metadata: Option<crate::state::AnalysisResultFamilyMetadata>,
    /// What the adapter had to decide that the source did not state. These are
    /// not errors — the draft is committable with them — so they are shown
    /// beside the review summary and repeated in the console at commit,
    /// rather than filling `validation_error`.
    pub notes: Vec<String>,
    /// The exact event history and bus declarations a digital source
    /// carried, held across review so the commit does not have to parse the
    /// file a second time. Committed whole: it is evidence about events, not
    /// a projection of the sampled columns the reader is selecting among.
    pub event_payload: Option<crate::state::AnalysisResultPayload>,
    /// The source's other analyses, when it holds several (a rawfile of an
    /// operating point and a transient, say). The fields above review one of
    /// them; these are committed beside it, as written, in the same run.
    pub plots: Option<std::sync::Arc<ResultImportPlots>>,
    pub selected_signals: Vec<bool>,
    pub validation_error: Option<String>,
}

/// The analyses of a source that holds more than one.
#[derive(Debug, Clone, PartialEq)]
pub struct ResultImportPlots {
    /// The source's own title for the analysis the draft reviews.
    pub reviewed: String,
    /// Every other analysis, in source order.
    pub further: Vec<ResultImportPlot>,
}

/// One further analysis of a multi-analysis source, imported as written.
#[derive(Debug, Clone, PartialEq)]
pub struct ResultImportPlot {
    /// The source's own title for this analysis.
    pub name: String,
    pub analysis_type: crate::state::AnalysisType,
    pub coordinate_name: String,
    pub coordinate_unit: Option<String>,
    pub sample_count: usize,
    pub waveforms: Vec<crate::state::RetainedTrace>,
    pub event_payload: Option<crate::state::AnalysisResultPayload>,
}

impl Default for ResultImportDialogState {
    fn default() -> Self {
        Self {
            open: false,
            stage: ResultImportStage::Detect,
            source_name: String::new(),
            source_format_id: String::new(),
            delimiter: b',',
            analysis_type: crate::state::AnalysisType::Transient,
            coordinate_name: String::new(),
            coordinate_unit: None,
            sample_count: 0,
            waveforms: std::sync::Arc::new(Vec::new()),
            family_metadata: None,
            notes: Vec::new(),
            event_payload: None,
            plots: None,
            selected_signals: Vec::new(),
            validation_error: None,
        }
    }
}
