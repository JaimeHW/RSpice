//! Datasets a document reads from the run store instead of holding.
//!
//! A run source names one retained run by identity and content digest and
//! the analyses of it the document reads. It holds neither the run's samples
//! nor its list of signals: a trace names the signal it draws by analysis,
//! name and component, and the host reads that signal from the run when it
//! draws it. A view of a million-signal run is as small as a view of a
//! ten-signal one, and following another run of the same plan moves each
//! trace onto the signal of the same name there without reading either run's
//! list of signals. A trace whose run has no signal by its name draws nothing.
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

/// One analysis of a referenced run: which it is and what kind. Its signals
/// stay in the run.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunSourceAnalysis {
    id: AnalysisInstanceId,
    analysis_type: AnalysisType,
}

impl RunSourceAnalysis {
    #[must_use]
    pub const fn new(id: AnalysisInstanceId, analysis_type: AnalysisType) -> Self {
        Self { id, analysis_type }
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
    /// The results stores the record reads its samples from, by digest: a stored run's record
    /// holds no samples, and the document keeps these with it.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    segments: Vec<String>,
}

impl RunArchive {
    /// The record of a run, in `encoding`, that reads its samples from the results stores
    /// `segments` names by digest.
    pub fn new(
        encoding: impl Into<String>,
        record: impl Into<String>,
        segments: Vec<String>,
    ) -> Result<Self, VisualizationError> {
        let archive = Self {
            encoding: encoding.into(),
            record: record.into(),
            segments,
        };
        archive.validate()?;
        Ok(archive)
    }

    /// The digests of the results stores the record reads.
    #[must_use]
    pub fn segments(&self) -> &[String] {
        &self.segments
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
        let digest = |hex: &String| {
            hex.len() == 64
                && hex
                    .bytes()
                    .all(|byte| matches!(byte, b'0'..=b'9' | b'a'..=b'f'))
        };
        if !self.segments.iter().all(digest) {
            return Err(VisualizationError::InvalidValue {
                field: "run-archive.segments",
                message: "an archived run names each results store by its 64-digit digest"
                    .to_owned(),
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
            .field("segments", &self.segments)
            .finish()
    }
}

/// The exact samples of one series a report figure draws.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RunSeriesSamples {
    analysis: AnalysisInstanceId,
    #[serde(deserialize_with = "deserialize_source_text_string")]
    name: String,
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
        name: impl Into<String>,
        component: SignalComponent,
        x: Vec<f64>,
        y: Vec<f64>,
    ) -> Self {
        Self {
            analysis,
            name: name.into(),
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

    fn selects(&self, series: &SeriesKey) -> bool {
        self.analysis == series.analysis
            && self.name == series.name
            && self.component == series.component
    }

    fn key(&self) -> SeriesKey {
        SeriesKey {
            analysis: self.analysis,
            name: self.name.clone(),
            component: self.component,
        }
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

/// One series of a run source: a component of the signal an analysis of the
/// run names `name`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SeriesKey {
    pub analysis: AnalysisInstanceId,
    pub name: String,
    pub component: SignalComponent,
}

/// The reader does not hold the exact run a document names.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RunNotHeld;

/// A series as a figure draws it: its X samples and its values, one of each per point.
pub type FigurePoints = (Vec<f64>, Vec<f64>);

/// The pixel columns a report figure's series is drawn at: as many as the widest figure has.
pub const FIGURE_COLUMNS: usize = crate::visualization_raster::MAX_RASTER_DIMENSION as usize;

/// Reads the samples of the runs a document references.
pub trait SourceSamples {
    /// One series of the run `binding` names as a figure `columns` pixels wide over x in `range`
    /// (all of it when `None`) draws it, as X samples and values: every row where a column holds
    /// at most two, else each column's least and greatest value at its centre. `Ok(None)` when
    /// the run has no such series, and [`RunNotHeld`] when the reader does not hold that exact
    /// run. A `NaN` value is an unavailable sample.
    fn series_figure(
        &self,
        binding: DatasetBinding,
        series: &SeriesKey,
        range: Option<(f64, f64)>,
        columns: usize,
    ) -> Result<Option<FigurePoints>, RunNotHeld>;
}

/// The points a figure draws of a tile over x in `x0 ..= x1`: a sample tile's rows with the
/// nearest row either side, or each touched column's least and greatest value at its centre,
/// in the order that leaves the line nearest the previous column, the first and last columns
/// at the ends of the span so its extent stays the series'. A column with no available value
/// breaks the line.
#[cfg(feature = "engine-evidence")]
pub fn tile_points(tile: crate::waveform::ColumnTile, x0: f64, x1: f64) -> (Vec<f64>, Vec<f64>) {
    use crate::waveform::ColumnTile;
    match tile {
        ColumnTile::Samples {
            x,
            y,
            before,
            after,
        } => {
            let mut points: Vec<(f64, f64)> = Vec::with_capacity(x.len() + 2);
            points.extend(before);
            points.extend(x.into_iter().zip(y));
            points.extend(after);
            points.into_iter().unzip()
        }
        ColumnTile::Envelope { min, max, touched } => {
            let columns = min.len();
            let first = touched.iter().position(|&touched| touched);
            let last = touched.iter().rposition(|&touched| touched);
            let width = (x1 - x0) / columns as f64;
            let mut xs = Vec::with_capacity(2 * columns);
            let mut ys = Vec::with_capacity(2 * columns);
            let mut previous = f64::NAN;
            for column in (0..columns).filter(|&column| touched[column]) {
                let at = if Some(column) == first {
                    x0
                } else if Some(column) == last {
                    x1
                } else {
                    x0 + (column as f64 + 0.5) * width
                };
                let (low, high) = (min[column], max[column]);
                if low.is_nan() {
                    xs.push(at);
                    ys.push(f64::NAN);
                    previous = f64::NAN;
                    continue;
                }
                let (start, end) = if (previous - high).abs() < (previous - low).abs() {
                    (high, low)
                } else {
                    (low, high)
                };
                xs.push(at);
                ys.push(start);
                if end != start {
                    xs.push(at);
                    ys.push(end);
                }
                previous = end;
            }
            (xs, ys)
        }
    }
}

/// `y` over `x` as a figure `columns` wide over x in `range` draws it: from the store's tiles
/// for a stored window, and from the samples for one in memory.
#[cfg(feature = "engine-evidence")]
pub fn figure_points(
    x: &crate::waveform::WaveformValues,
    y: &crate::waveform::WaveformValues,
    range: Option<(f64, f64)>,
    columns: usize,
) -> Option<(Vec<f64>, Vec<f64>)> {
    if x.len() != y.len() || columns == 0 {
        return None;
    }
    let (x0, x1) = match range {
        Some(range) => range,
        None => x.ends()?,
    };
    if let Some(tile) = y.tile_over(x, x0, x1, columns) {
        return Some(tile_points(tile, x0, x1));
    }
    let first = x.partition_point(|value| value < x0);
    let end = x.partition_point(|value| value <= x1);
    let row = |row: usize| x.sample(row).zip(y.sample(row));
    let before = first.checked_sub(1).and_then(row);
    let after = (end < x.len()).then(|| row(end)).flatten();
    let tile = if end - first <= 2 * columns {
        crate::waveform::ColumnTile::Samples {
            x: x.samples(first..end).collect(),
            y: y.samples(first..end).collect(),
            before,
            after,
        }
    } else {
        let mut min = vec![f64::NAN; columns];
        let mut max = vec![f64::NAN; columns];
        let mut touched = vec![false; columns];
        let scale = columns as f64 / (x1 - x0);
        for (at, value) in x.samples(first..end).zip(y.samples(first..end)) {
            let column = (((at - x0) * scale) as usize).min(columns - 1);
            touched[column] = true;
            if !value.is_nan() {
                min[column] = if min[column].is_nan() {
                    value
                } else {
                    min[column].min(value)
                };
                max[column] = if max[column].is_nan() {
                    value
                } else {
                    max[column].max(value)
                };
            }
        }
        crate::waveform::ColumnTile::Envelope { min, max, touched }
    };
    Some(tile_points(tile, x0, x1))
}

/// A retained run a document reads, named without its samples or its list
/// of signals.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RunSource {
    /// How the run was named when the document was bound to it, so the
    /// document can say which run it misses once the run is gone.
    #[serde(deserialize_with = "deserialize_label_string")]
    run_label: String,
    analyses: Vec<RunSourceAnalysis>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    archive: Option<RunArchive>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
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
        if self.analyses.is_empty() {
            return Err(VisualizationError::InvalidValue {
                field: "run-source.analyses",
                message: "a run reference reads at least one of its run's analyses".to_owned(),
            });
        }
        let mut analyses = HashSet::with_capacity(self.analyses.len());
        for analysis in &self.analyses {
            if !analyses.insert(analysis.id) {
                return Err(VisualizationError::InvalidValue {
                    field: "run-source.analyses",
                    message: format!("analysis {} is listed twice", analysis.id),
                });
            }
        }
        if let Some(archive) = &self.archive {
            archive.validate()?;
        }
        let mut series = HashSet::with_capacity(self.figure_samples.len());
        for samples in &self.figure_samples {
            validate_signal_name(&samples.name)?;
            if !analyses.contains(&samples.analysis) || !series.insert(samples.key()) {
                return Err(VisualizationError::InvalidValue {
                    field: "run-source.figure-samples",
                    message: format!(
                        "figure samples name {:?} of analysis {} twice, or an analysis the document does not read",
                        samples.name, samples.analysis
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

    /// The one series a trace's exact predicates name: an analysis this
    /// reference reads, a signal by name and a component. Whether the run
    /// holds that signal is the run's to say when the trace is drawn.
    pub(crate) fn select_one(
        &self,
        predicates: &[QueryCoordinate],
    ) -> Result<SeriesKey, VisualizationError> {
        let mut analysis = None::<&str>;
        let mut name = None::<&str>;
        let mut component = None::<&str>;
        for predicate in predicates {
            match (predicate.column.as_str(), &predicate.value) {
                (ANALYSIS_ID_COLUMN, TypedValue::Text(value)) => analysis = Some(value),
                (TRACE_NAME_COLUMN, TypedValue::Text(value)) => name = Some(value),
                (COMPONENT_COLUMN, TypedValue::Text(value)) => component = Some(value),
                (column, _) => {
                    return Err(VisualizationError::InvalidValue {
                        field: "trace.row-predicates",
                        message: format!(
                            "a run reference selects whole signals by analysis, name and component; a predicate on '{column}' needs the samples"
                        ),
                    });
                }
            }
        }
        let refused = |message: &str| VisualizationError::InvalidValue {
            field: "trace.row-predicates",
            message: message.to_owned(),
        };
        let analysis = analysis
            .and_then(|analysis| {
                self.analyses
                    .iter()
                    .find(|candidate| candidate.id.to_string() == analysis)
            })
            .ok_or_else(|| {
                refused("a trace on a run names an analysis of that run the document reads")
            })?;
        let name = name.ok_or_else(|| refused("a trace on a run names the signal it draws"))?;
        validate_signal_name(name)?;
        let component = [
            SignalComponent::Display,
            SignalComponent::Real,
            SignalComponent::Imag,
        ]
        .into_iter()
        .find(|each| component == Some(each.key()))
        .ok_or_else(|| {
            refused("a trace on a run draws its signal's display, real or imaginary part")
        })?;
        Ok(SeriesKey {
            analysis: analysis.id,
            name: name.to_owned(),
            component,
        })
    }

    pub(crate) fn figure_samples(&self, series: &SeriesKey) -> Option<&RunSeriesSamples> {
        self.figure_samples
            .iter()
            .find(|samples| samples.selects(series))
    }

    pub(crate) fn carries_figure_samples(&self) -> bool {
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

/// A signal name a trace or figure names: what the run calls it, one line of
/// at most [`MAX_VISUALIZATION_LABEL_BYTES`] bytes, so it can be a label too.
pub(super) fn validate_signal_name(name: &str) -> Result<(), VisualizationError> {
    if name.is_empty()
        || name.len() > MAX_VISUALIZATION_LABEL_BYTES
        || name.chars().any(char::is_control)
    {
        return Err(VisualizationError::InvalidValue {
            field: "run-source.signal.name",
            message: format!(
                "a signal name holds 1 to {MAX_VISUALIZATION_LABEL_BYTES} bytes and no control characters"
            ),
        });
    }
    Ok(())
}

/// The exact predicates that name the `component` of the signal `name` of
/// `analysis`, on a run reference.
pub(super) fn run_signal_predicates(
    analysis: AnalysisInstanceId,
    name: &str,
    component: SignalComponent,
) -> Vec<QueryCoordinate> {
    run_signal_predicates_of(&analysis.to_string(), name, component)
}

/// [`run_signal_predicates`] with the analysis already spelled out.
pub(super) fn run_signal_predicates_of(
    analysis: &str,
    name: &str,
    component: SignalComponent,
) -> Vec<QueryCoordinate> {
    vec![
        QueryCoordinate {
            column: TRACE_NAME_COLUMN.to_owned(),
            value: TypedValue::Text(name.to_owned()),
        },
        QueryCoordinate {
            column: COMPONENT_COLUMN.to_owned(),
            value: TypedValue::Text(component.key().to_owned()),
        },
        QueryCoordinate {
            column: ANALYSIS_ID_COLUMN.to_owned(),
            value: TypedValue::Text(analysis.to_owned()),
        },
    ]
}

/// A run reference's series as a figure draws it: its figure samples when the document carries
/// them, else read through a host reader. `Ok(None)` when the run has no such series.
pub(super) fn run_series_points(
    run: &RunSource,
    binding: DatasetBinding,
    series: &SeriesKey,
    range: Option<(f64, f64)>,
    samples: Option<&dyn SourceSamples>,
) -> Result<Option<FigurePoints>, RunNotHeld> {
    if let Some(embedded) = run.figure_samples(series) {
        return Ok(Some((embedded.x.clone(), embedded.y.clone())));
    }
    let read = samples
        .ok_or(RunNotHeld)?
        .series_figure(binding, series, range, FIGURE_COLUMNS)?;
    Ok(read.filter(|(x, y)| x.len() == y.len()))
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
        copy.changes = journal::Changes::fresh();
        for dataset in copy.datasets.edit() {
            let binding = dataset.binding;
            let Some(run) = dataset.run.as_mut() else {
                continue;
            };
            // Each series is drawn over the span its traces' horizontal axes fix, or all of it
            // where one of them fits its data.
            let mut series = Vec::<(SeriesKey, Option<(f64, f64)>)>::new();
            for trace in self
                .traces
                .iter()
                .filter(|trace| trace.binding == binding && !trace.missing)
            {
                let key = run.select_one(&trace.row_predicates)?;
                let range = self
                    .axes
                    .iter()
                    .find(|axis| axis.id == trace.x_axis_id)
                    .and_then(|axis| axis.range)
                    .map(|range| (range.minimum, range.maximum));
                match series.iter_mut().find(|(known, _)| *known == key) {
                    Some((_, known)) => {
                        *known = known
                            .zip(range)
                            .map(|(known, range)| (known.0.min(range.0), known.1.max(range.1)));
                    }
                    None => series.push((key, range)),
                }
            }
            let mut embedded = Vec::with_capacity(series.len());
            for (key, range) in series {
                // A signal the run does not have is drawn by no figure.
                let read = run_series_points(run, binding, &key, range, Some(samples)).map_err(
                    |RunNotHeld| VisualizationError::InvalidValue {
                        field: "run-source.figure-samples",
                        message: format!(
                            "the samples of {:?} of run {} are not retained",
                            key.name, binding.dataset_id
                        ),
                    },
                )?;
                if let Some((x, y)) = read {
                    embedded.push(RunSeriesSamples::new(
                        key.analysis,
                        key.name,
                        key.component,
                        x,
                        y,
                    ));
                }
            }
            run.set_figure_samples(embedded);
            run.validate()?;
        }
        Ok(copy)
    }

    /// The digests of the results stores its archived runs read.
    pub fn archived_segments(&self) -> impl Iterator<Item = &str> {
        self.datasets
            .iter()
            .filter_map(|dataset| dataset.run.as_ref()?.archive.as_ref())
            .flat_map(|archive| archive.segments.iter().map(String::as_str))
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
    fn series_figure(
        &self,
        binding: DatasetBinding,
        series: &SeriesKey,
        range: Option<(f64, f64)>,
        columns: usize,
    ) -> Result<Option<FigurePoints>, RunNotHeld> {
        let run = self.run(binding).ok_or(RunNotHeld)?;
        let Some(analysis) = run.analyses.iter().map(AsRef::as_ref).find(|analysis| {
            crate::result_presentation::AnalysisPresentationKey::new(run.dataset_id, analysis)
                .retained_instance_id()
                == series.analysis
        }) else {
            return Ok(None);
        };
        let Some(waveform) = analysis
            .waveforms
            .iter()
            .map(AsRef::as_ref)
            .find(|waveform| waveform.name == series.name)
        else {
            return Ok(None);
        };
        let values = match series.component {
            SignalComponent::Display => Some(&waveform.y),
            SignalComponent::Real => waveform.complex.as_ref().map(|complex| &complex.real),
            SignalComponent::Imag => waveform.complex.as_ref().map(|complex| &complex.imag),
        };
        Ok(values.and_then(|values| figure_points(&waveform.x, values, range, columns)))
    }
}
