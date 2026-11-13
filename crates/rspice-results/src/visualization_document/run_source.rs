//! Datasets a document reads from the run store instead of holding.
//!
//! A run source names one retained run by identity and content digest, the
//! analyses of it the document reads, and each analysis's signals. It holds
//! no samples, so no limit below the run's own size applies to it: the viewer
//! reads them from the run. Its signal catalog lets the document check every
//! trace it addresses, provision panes, and re-point at another run with the
//! same signals while its own run is not retained.
//!
//! Two optional parts embed samples. An archive is the run's own record in
//! the host's encoding, so a shared document stands alone and the host can
//! restore the exact run, digest and all. Figure samples are the exact
//! samples of the signals a report figure draws, carried by the copy of a
//! document that a report prepares, so the figure needs no run store.

use super::*;
use crate::analysis_type::AnalysisType;

pub(super) const TRACE_INDEX_COLUMN: &str = "trace-index";
pub(super) const COMPONENT_COLUMN: &str = "component";
pub(super) const ANALYSIS_ID_COLUMN: &str = "analysis-id";
const SAMPLE_COLUMN: &str = "sample";
const X_COLUMN: &str = "x";
const Y_COLUMN: &str = "y";

/// Maximum analyses one run source reads.
pub const MAX_RUN_SOURCE_ANALYSES: usize = 4_096;
/// Maximum signals one run source catalogs, over all of its analyses.
pub const MAX_RUN_SOURCE_SIGNALS: usize = 262_144;

/// The columns a run source exposes to the traces that address it: one row
/// per sample of one component of one signal of one analysis.
pub(super) fn run_source_columns() -> Vec<SourceColumn> {
    let column = |key: &str, label: &str, value_type, role| SourceColumn {
        key: key.to_owned(),
        label: label.to_owned(),
        value_type,
        role,
        unit: None,
    };
    vec![
        column(
            TRACE_INDEX_COLUMN,
            "Trace index",
            ValueType::Integer,
            ColumnRole::Coordinate,
        ),
        column(
            TRACE_NAME_COLUMN,
            "Trace",
            ValueType::Text,
            ColumnRole::Coordinate,
        ),
        column(
            COMPONENT_COLUMN,
            "Component",
            ValueType::Text,
            ColumnRole::Coordinate,
        ),
        column(
            SAMPLE_COLUMN,
            "Sample",
            ValueType::Integer,
            ColumnRole::Coordinate,
        ),
        column(X_COLUMN, "X", ValueType::Real, ColumnRole::Coordinate),
        column(Y_COLUMN, "Y", ValueType::Real, ColumnRole::Signal),
        column(
            ANALYSIS_ID_COLUMN,
            "Analysis identity",
            ValueType::Text,
            ColumnRole::Coordinate,
        ),
    ]
}

/// Which samples of a signal a series holds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SignalComponent {
    /// The values the signal is plotted with.
    Display,
    /// The real part of a complex signal.
    Real,
    /// The imaginary part of a complex signal.
    Imag,
}

impl SignalComponent {
    /// The `component` coordinate value that selects this series.
    #[must_use]
    pub const fn key(self) -> &'static str {
        match self {
            Self::Display => "display",
            Self::Real => "real",
            Self::Imag => "imag",
        }
    }
}

/// One signal of one analysis of a referenced run.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunSignal {
    index: u32,
    #[serde(deserialize_with = "deserialize_source_text_string")]
    name: String,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    complex: bool,
}

impl RunSignal {
    /// The signal at `index` among its analysis's waveforms, named `name`,
    /// with real and imaginary components when it is `complex`.
    #[must_use]
    pub fn new(index: u32, name: impl Into<String>, complex: bool) -> Self {
        Self {
            index,
            name: name.into(),
            complex,
        }
    }

    #[must_use]
    pub const fn index(&self) -> u32 {
        self.index
    }

    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[must_use]
    pub const fn complex(&self) -> bool {
        self.complex
    }

    fn has_component(&self, component: SignalComponent) -> bool {
        component == SignalComponent::Display || self.complex
    }

    fn validate(&self) -> Result<(), VisualizationError> {
        if self.name.is_empty()
            || self.name.len() > MAX_SOURCE_TEXT_BYTES
            || self.name.chars().any(char::is_control)
        {
            return Err(VisualizationError::InvalidValue {
                field: "run-source.signal.name",
                message: format!(
                    "a signal name holds 1 to {MAX_SOURCE_TEXT_BYTES} bytes and no control characters"
                ),
            });
        }
        Ok(())
    }
}

/// One analysis of a referenced run, with the signals it holds.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunSourceAnalysis {
    id: AnalysisInstanceId,
    analysis_type: AnalysisType,
    #[serde(deserialize_with = "deserialize_run_signals")]
    signals: Vec<RunSignal>,
}

impl RunSourceAnalysis {
    #[must_use]
    pub fn new(
        id: AnalysisInstanceId,
        analysis_type: AnalysisType,
        signals: Vec<RunSignal>,
    ) -> Self {
        Self {
            id,
            analysis_type,
            signals,
        }
    }

    #[must_use]
    pub const fn id(&self) -> AnalysisInstanceId {
        self.id
    }

    /// What kind of analysis it is, so another run's analysis can stand in
    /// for it only when it is the same kind.
    #[must_use]
    pub const fn analysis_type(&self) -> AnalysisType {
        self.analysis_type
    }

    #[must_use]
    pub fn signals(&self) -> &[RunSignal] {
        &self.signals
    }
}

/// A referenced run's own record, embedded so the document stands alone.
///
/// The record is in the encoding the host names; the document carries it
/// without reading it. The host restores the run from it only when the
/// restored run's content digest is the one the document is bound to.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunArchive {
    #[serde(deserialize_with = "deserialize_key_string")]
    encoding: String,
    record: String,
}

impl RunArchive {
    pub fn new(
        encoding: impl Into<String>,
        record: impl Into<String>,
    ) -> Result<Self, VisualizationError> {
        let archive = Self {
            encoding: encoding.into(),
            record: record.into(),
        };
        archive.validate()?;
        Ok(archive)
    }

    #[must_use]
    pub fn encoding(&self) -> &str {
        &self.encoding
    }

    #[must_use]
    pub fn record(&self) -> &str {
        &self.record
    }

    fn validate(&self) -> Result<(), VisualizationError> {
        validate_key("run-archive.encoding", &self.encoding)?;
        if self.record.is_empty() {
            return Err(VisualizationError::InvalidValue {
                field: "run-archive.record",
                message: "an archived run holds its record".to_owned(),
            });
        }
        Ok(())
    }
}

impl fmt::Debug for RunArchive {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("RunArchive")
            .field("encoding", &self.encoding)
            .field("record_bytes", &self.record.len())
            .finish()
    }
}

/// The exact samples of one series a report figure draws.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RunSeriesSamples {
    analysis: AnalysisInstanceId,
    index: u32,
    component: SignalComponent,
    x: Vec<f64>,
    /// An unavailable sample is written as `null`.
    #[serde(with = "unavailable_as_null")]
    y: Vec<f64>,
}

impl RunSeriesSamples {
    #[must_use]
    pub fn new(
        analysis: AnalysisInstanceId,
        index: u32,
        component: SignalComponent,
        x: Vec<f64>,
        y: Vec<f64>,
    ) -> Self {
        Self {
            analysis,
            index,
            component,
            x,
            y,
        }
    }

    #[must_use]
    pub fn x(&self) -> &[f64] {
        &self.x
    }

    /// The series' values; `NaN` marks an unavailable sample.
    #[must_use]
    pub fn y(&self) -> &[f64] {
        &self.y
    }

    fn selects(&self, series: SeriesKey) -> bool {
        self.analysis == series.analysis
            && self.index == series.index
            && self.component == series.component
    }
}

mod unavailable_as_null {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S: Serializer>(values: &[f64], serializer: S) -> Result<S::Ok, S::Error> {
        values
            .iter()
            .map(|value| (!value.is_nan()).then_some(*value))
            .collect::<Vec<_>>()
            .serialize(serializer)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Vec<f64>, D::Error> {
        Ok(Vec::<Option<f64>>::deserialize(deserializer)?
            .into_iter()
            .map(|value| value.unwrap_or(f64::NAN))
            .collect())
    }
}

/// One series of a run source: a component of a signal of an analysis.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SeriesKey {
    pub analysis: AnalysisInstanceId,
    pub index: u32,
    pub component: SignalComponent,
}

fn deserialize_run_signals<'de, D>(deserializer: D) -> Result<Vec<RunSignal>, D::Error>
where
    D: Deserializer<'de>,
{
    deserialize_bounded_vec::<D, RunSignal, MAX_RUN_SOURCE_SIGNALS>(deserializer)
}

fn deserialize_run_analyses<'de, D>(deserializer: D) -> Result<Vec<RunSourceAnalysis>, D::Error>
where
    D: Deserializer<'de>,
{
    deserialize_bounded_vec::<D, RunSourceAnalysis, MAX_RUN_SOURCE_ANALYSES>(deserializer)
}

fn deserialize_figure_samples<'de, D>(deserializer: D) -> Result<Vec<RunSeriesSamples>, D::Error>
where
    D: Deserializer<'de>,
{
    deserialize_bounded_vec::<D, RunSeriesSamples, MAX_RUN_SOURCE_SIGNALS>(deserializer)
}

/// Reads the samples of the runs a document references.
pub trait SourceSamples {
    /// The X samples and values of one series of the run `binding` names:
    /// `None` when the reader does not hold that exact run or series. A
    /// `NaN` value is an unavailable sample.
    fn series_samples(
        &self,
        binding: DatasetBinding,
        series: SeriesKey,
    ) -> Option<(&[f64], &[f64])>;
}

/// The signals of a retained run a document reads, without its samples.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RunSource {
    /// How the run was named when the document was bound to it, so the
    /// document can say which run it misses once the run is gone.
    #[serde(deserialize_with = "deserialize_label_string")]
    run_label: String,
    #[serde(deserialize_with = "deserialize_run_analyses")]
    analyses: Vec<RunSourceAnalysis>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    archive: Option<RunArchive>,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "deserialize_figure_samples"
    )]
    figure_samples: Vec<RunSeriesSamples>,
}

impl RunSource {
    /// The run's name when the document was bound to it.
    #[must_use]
    pub fn run_label(&self) -> &str {
        &self.run_label
    }

    #[must_use]
    pub fn analyses(&self) -> &[RunSourceAnalysis] {
        &self.analyses
    }

    /// The run's own record, when the document embeds it.
    #[must_use]
    pub const fn archive(&self) -> Option<&RunArchive> {
        self.archive.as_ref()
    }

    #[must_use]
    pub fn analysis(&self, id: AnalysisInstanceId) -> Option<&RunSourceAnalysis> {
        self.analyses.iter().find(|analysis| analysis.id == id)
    }

    pub(super) fn validate(&self) -> Result<(), VisualizationError> {
        validate_label("run-source.run-label", &self.run_label)?;
        if self.analyses.is_empty() || self.analyses.len() > MAX_RUN_SOURCE_ANALYSES {
            return Err(VisualizationError::InvalidValue {
                field: "run-source.analyses",
                message: format!(
                    "a run reference reads 1 to {MAX_RUN_SOURCE_ANALYSES} of its run's analyses"
                ),
            });
        }
        let mut analyses = HashSet::with_capacity(self.analyses.len());
        let mut signals = 0_usize;
        for analysis in &self.analyses {
            if !analyses.insert(analysis.id) {
                return Err(VisualizationError::InvalidValue {
                    field: "run-source.analyses",
                    message: format!("analysis {} is listed twice", analysis.id),
                });
            }
            signals = checked_bounded_sum(
                "run-source.signals",
                signals,
                analysis.signals.len(),
                MAX_RUN_SOURCE_SIGNALS,
            )?;
            let mut indices = HashSet::with_capacity(analysis.signals.len());
            for signal in &analysis.signals {
                signal.validate()?;
                if !indices.insert(signal.index) {
                    return Err(VisualizationError::InvalidValue {
                        field: "run-source.signals",
                        message: format!(
                            "analysis {} lists signal index {} twice",
                            analysis.id, signal.index
                        ),
                    });
                }
            }
        }
        if let Some(archive) = &self.archive {
            archive.validate()?;
        }
        let mut series = HashSet::with_capacity(self.figure_samples.len());
        for samples in &self.figure_samples {
            let key = SeriesKey {
                analysis: samples.analysis,
                index: samples.index,
                component: samples.component,
            };
            if !series.insert(key) || self.signal(key).is_none() {
                return Err(VisualizationError::InvalidValue {
                    field: "run-source.figure-samples",
                    message: format!(
                        "figure samples name signal {} of analysis {} twice or not at all",
                        samples.index, samples.analysis
                    ),
                });
            }
            if samples.x.len() != samples.y.len()
                || samples.x.iter().any(|x| !x.is_finite())
                || samples.y.iter().any(|y| y.is_infinite())
            {
                return Err(VisualizationError::InvalidValue {
                    field: "run-source.figure-samples",
                    message: "figure samples pair one finite X with each value".to_owned(),
                });
            }
        }
        Ok(())
    }

    fn signal(&self, series: SeriesKey) -> Option<&RunSignal> {
        self.analysis(series.analysis)?
            .signals
            .iter()
            .find(|signal| signal.index == series.index && signal.has_component(series.component))
    }

    /// Add one analysis this run source does not read yet; reading it again
    /// is accepted only when it lists the same signals.
    pub(super) fn merge(&mut self, projection: Self) -> Result<(), VisualizationError> {
        if projection.archive.is_some() || !projection.figure_samples.is_empty() {
            return Err(VisualizationError::InvalidValue {
                field: "visualization-document.datasets",
                message: "an analysis projection carries no embedded samples".to_owned(),
            });
        }
        for analysis in projection.analyses {
            match self.analysis(analysis.id) {
                Some(existing) if *existing == analysis => {}
                Some(_) => {
                    return Err(VisualizationError::InvalidValue {
                        field: "visualization-document.datasets",
                        message: "an attached analysis projection cannot be rewritten under the same immutable binding"
                            .to_owned(),
                    });
                }
                None => self.analyses.push(analysis),
            }
        }
        self.validate()
    }

    pub(super) fn set_archive(&mut self, archive: RunArchive) -> Result<(), VisualizationError> {
        archive.validate()?;
        if self.archive.is_some() {
            return Err(VisualizationError::InvalidValue {
                field: "run-source.archive",
                message: "the document already holds this run's samples".to_owned(),
            });
        }
        self.archive = Some(archive);
        Ok(())
    }

    /// The series a trace's exact predicates select, or why they cannot
    /// select one: a run reference selects whole series, never samples.
    pub(crate) fn select(
        &self,
        predicates: &[QueryCoordinate],
    ) -> Result<Vec<SeriesKey>, VisualizationError> {
        let mut analysis = None::<&str>;
        let mut index = None::<i64>;
        let mut name = None::<&str>;
        let mut component = None::<&str>;
        for predicate in predicates {
            match (predicate.column.as_str(), &predicate.value) {
                (ANALYSIS_ID_COLUMN, TypedValue::Text(value)) => analysis = Some(value),
                (TRACE_INDEX_COLUMN, TypedValue::Integer(value)) => index = Some(*value),
                (TRACE_NAME_COLUMN, TypedValue::Text(value)) => name = Some(value),
                (COMPONENT_COLUMN, TypedValue::Text(value)) => component = Some(value),
                (column, _) => {
                    return Err(VisualizationError::InvalidValue {
                        field: "trace.row-predicates",
                        message: format!(
                            "a run reference selects whole signals; a predicate on '{column}' needs the samples"
                        ),
                    });
                }
            }
        }
        let mut selected = Vec::new();
        for candidate in &self.analyses {
            if analysis.is_some_and(|analysis| analysis != candidate.id.to_string()) {
                continue;
            }
            for signal in &candidate.signals {
                if index.is_some_and(|index| index != i64::from(signal.index))
                    || name.is_some_and(|name| name != signal.name)
                {
                    continue;
                }
                for each in [
                    SignalComponent::Display,
                    SignalComponent::Real,
                    SignalComponent::Imag,
                ] {
                    if signal.has_component(each)
                        && component.is_none_or(|component| component == each.key())
                    {
                        selected.push(SeriesKey {
                            analysis: candidate.id,
                            index: signal.index,
                            component: each,
                        });
                    }
                }
            }
        }
        if selected.is_empty() {
            return Err(VisualizationError::InvalidValue {
                field: "trace.row-predicates",
                message: "trace row predicates select no signal of the referenced run".to_owned(),
            });
        }
        Ok(selected)
    }

    /// The single series a trace draws.
    pub(crate) fn select_one(
        &self,
        predicates: &[QueryCoordinate],
    ) -> Result<SeriesKey, VisualizationError> {
        let selected = self.select(predicates)?;
        match selected.as_slice() {
            [series] => Ok(*series),
            _ => Err(VisualizationError::InvalidValue {
                field: "trace.row-predicates",
                message: format!(
                    "trace row predicates select {} series of the referenced run, not one",
                    selected.len()
                ),
            }),
        }
    }

    /// The display signals of one analysis, as (trace index, name).
    pub(super) fn display_signals(&self, analysis: AnalysisInstanceId) -> Vec<(i64, String)> {
        self.analysis(analysis).map_or_else(Vec::new, |analysis| {
            analysis
                .signals
                .iter()
                .map(|signal| (i64::from(signal.index), signal.name.clone()))
                .collect()
        })
    }

    pub(crate) fn figure_samples(&self, series: SeriesKey) -> Option<&RunSeriesSamples> {
        self.figure_samples
            .iter()
            .find(|samples| samples.selects(series))
    }

    pub(super) fn carries_figure_samples(&self) -> bool {
        !self.figure_samples.is_empty()
    }

    pub(super) fn set_figure_samples(&mut self, samples: Vec<RunSeriesSamples>) {
        self.figure_samples = samples;
    }
}

impl SourceDataset {
    /// A dataset that references analyses of a retained run: the run is named
    /// by identity and content digest, and its samples stay in the run.
    pub fn run_reference(
        binding: DatasetBinding,
        run_label: impl Into<String>,
        analyses: Vec<RunSourceAnalysis>,
    ) -> Result<Self, VisualizationError> {
        let dataset = Self {
            binding,
            columns: run_source_columns(),
            rows: Vec::new(),
            run: Some(RunSource {
                run_label: run_label.into(),
                analyses,
                archive: None,
                figure_samples: Vec::new(),
            }),
        };
        dataset.validate()?;
        Ok(dataset)
    }

    /// The run this dataset references, when its samples stay in the run.
    #[must_use]
    pub const fn run_source(&self) -> Option<&RunSource> {
        self.run.as_ref()
    }

    /// Whether the document holds every sample of this dataset: an embedded
    /// table, or a run reference with its run archived.
    #[must_use]
    pub const fn is_self_contained(&self) -> bool {
        match &self.run {
            Some(run) => run.archive.is_some(),
            None => true,
        }
    }
}

/// A run reference's samples read through a host reader, or its figure
/// samples when the document carries them.
pub(super) fn run_series_points(
    run: &RunSource,
    binding: DatasetBinding,
    series: SeriesKey,
    samples: Option<&dyn SourceSamples>,
) -> Option<(Vec<f64>, Vec<f64>)> {
    if let Some(embedded) = run.figure_samples(series) {
        return Some((embedded.x.clone(), embedded.y.clone()));
    }
    let (x, y) = samples?.series_samples(binding, series)?;
    (x.len() == y.len()).then(|| (x.to_vec(), y.to_vec()))
}

impl VisualizationDocument {
    /// A copy of this document whose run references carry the exact samples
    /// of every series one of its traces draws, read through `samples`, so a
    /// report can draw its figures without the run store.
    ///
    /// The copy keeps every identity and binding; it refuses when the reader
    /// does not hold a referenced run's series.
    pub fn with_figure_samples(
        &self,
        samples: &dyn SourceSamples,
    ) -> Result<Self, VisualizationError> {
        let mut copy = self.clone();
        for dataset in &mut copy.datasets {
            let binding = dataset.binding;
            let Some(run) = dataset.run.as_mut() else {
                continue;
            };
            let mut series = Vec::<SeriesKey>::new();
            for trace in self.traces.iter().filter(|trace| trace.binding == binding) {
                let key = run.select_one(&trace.row_predicates)?;
                if !series.contains(&key) {
                    series.push(key);
                }
            }
            let mut embedded = Vec::with_capacity(series.len());
            for key in series {
                let (x, y) =
                    run_series_points(run, binding, key, Some(samples)).ok_or_else(|| {
                        VisualizationError::InvalidValue {
                            field: "run-source.figure-samples",
                            message: format!(
                                "the samples of signal {} of run {} are not retained",
                                key.index, binding.dataset_id
                            ),
                        }
                    })?;
                embedded.push(RunSeriesSamples::new(
                    key.analysis,
                    key.index,
                    key.component,
                    x,
                    y,
                ));
            }
            run.set_figure_samples(embedded);
            run.validate()?;
        }
        Ok(copy)
    }

    /// Whether the document holds every sample it shows, so it reads with no
    /// run store.
    #[must_use]
    pub fn is_archived(&self) -> bool {
        self.datasets.iter().all(SourceDataset::is_self_contained)
    }
}

/// The samples of the retained runs a host holds, read for the run
/// references of a document: a series is read only from the run with the
/// bound identity and content digest.
#[cfg(feature = "engine-evidence")]
pub struct RetainedRunSamples<'a, R, A, W> {
    runs: &'a [R],
    digests: RefCell<HashMap<DatasetId, ContentDigest>>,
    analysis: PhantomData<(A, W)>,
}

#[cfg(feature = "engine-evidence")]
impl<'a, R, A, W> RetainedRunSamples<'a, R, A, W>
where
    R: AsRef<crate::run::SimulationRun<A>>,
    A: AsRef<crate::analysis_result::AnalysisResult<W>>,
    W: AsRef<crate::waveform::RetainedWaveform>,
{
    #[must_use]
    pub fn new(runs: &'a [R]) -> Self {
        Self {
            runs,
            digests: RefCell::new(HashMap::new()),
            analysis: PhantomData,
        }
    }

    fn run(&self, binding: DatasetBinding) -> Option<&'a crate::run::SimulationRun<A>> {
        let run = self
            .runs
            .iter()
            .map(AsRef::as_ref)
            .find(|run| run.dataset_id == binding.dataset_id)?;
        let digest = *self
            .digests
            .borrow_mut()
            .entry(binding.dataset_id)
            .or_insert_with(|| {
                run.dataset_content_digest_with_encoding(
                    crate::result_digest::ResultDigestEncoding::CURRENT,
                )
            });
        (digest == binding.content_digest).then_some(run)
    }
}

#[cfg(feature = "engine-evidence")]
impl<R, A, W> SourceSamples for RetainedRunSamples<'_, R, A, W>
where
    R: AsRef<crate::run::SimulationRun<A>>,
    A: AsRef<crate::analysis_result::AnalysisResult<W>>,
    W: AsRef<crate::waveform::RetainedWaveform>,
{
    fn series_samples(
        &self,
        binding: DatasetBinding,
        series: SeriesKey,
    ) -> Option<(&[f64], &[f64])> {
        let run = self.run(binding)?;
        let analysis = run.analyses.iter().map(AsRef::as_ref).find(|analysis| {
            crate::result_presentation::AnalysisPresentationKey::new(run.dataset_id, analysis)
                .retained_instance_id()
                == series.analysis
        })?;
        let waveform = analysis
            .waveforms
            .get(usize::try_from(series.index).ok()?)?
            .as_ref();
        let values = match series.component {
            SignalComponent::Display => &waveform.y,
            SignalComponent::Real => &waveform.complex.as_ref()?.real,
            SignalComponent::Imag => &waveform.complex.as_ref()?.imag,
        };
        Some((waveform.x.as_slice(), values.as_slice()))
    }
}
