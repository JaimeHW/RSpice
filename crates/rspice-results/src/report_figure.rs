//! A plot a report keeps, as the values its pane draws.
//!
//! A report used to keep a figure as a picture of its pane: a fixed grid of
//! pixels, with no values on its axes, no key, and no way to rule a
//! logarithmic sweep by its decades. It keeps the plot itself instead: each
//! axis as the pane names, scales and pins it, and each trace's samples as
//! the run produced them. Print lays the figure out at whatever size the
//! report gives it, ruled, valued and keyed as a printed sheet is, in lines
//! rather than pixels.

use serde::{Deserialize, Serialize};

use crate::report_document::{FrozenReportArtifact, ReportError, ReportReferenceSnapshot};
use crate::visualization_document::{AxisRange, AxisScale, PageId, PaneId, VisualizationDocument};
use crate::visualization_raster::{VisualizationRasterError, resolve_cartesian_line_scene};

/// The media type a report figure is kept under.
pub const REPORT_FIGURE_MEDIA_TYPE: &str = "application/vnd.rspice.report-figure+json";

/// One plot, as a report keeps it.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub struct ReportFigure {
    pub x_axis: FigureAxis,
    pub y_axis: FigureAxis,
    /// The traces the pane shows, in its order.
    pub traces: Vec<FigureTrace>,
    /// The pane's cursors, each a full-height line at its abscissa.
    pub cursors: Vec<FigureCursor>,
}

/// One axis of a kept plot.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub struct FigureAxis {
    /// What the axis measures, as the pane names it.
    pub label: String,
    pub unit: Option<String>,
    pub scale: AxisScale,
    /// The window the pane pins the axis to; `None` where it fits its values.
    pub pinned: Option<AxisRange>,
}

/// One trace of a kept plot.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub struct FigureTrace {
    pub label: String,
    /// Each unbroken stretch of the trace, in its source values: a sample
    /// the run does not have breaks the line, as it does on the sheet.
    pub paths: Vec<Vec<(f64, f64)>>,
}

/// One cursor of a kept plot.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub struct FigureCursor {
    pub label: String,
    pub x: f64,
}

impl ReportFigure {
    /// The figure, kept as a report artifact.
    pub fn artifact(&self) -> Result<FrozenReportArtifact, ReportError> {
        self.validate()?;
        let payload = serde_json::to_vec(self).map_err(|error| ReportError::InvalidValue {
            field: "report-figure",
            message: error.to_string(),
        })?;
        FrozenReportArtifact::new(REPORT_FIGURE_MEDIA_TYPE, payload)
    }

    /// The figure `artifact` keeps; refused when it keeps something else.
    pub fn read(artifact: &FrozenReportArtifact) -> Result<Self, ReportError> {
        if artifact.media_type() != REPORT_FIGURE_MEDIA_TYPE {
            return Err(ReportError::InvalidValue {
                field: "report-figure.media-type",
                message: format!(
                    "a report figure is kept as {REPORT_FIGURE_MEDIA_TYPE}, not {}",
                    artifact.media_type()
                ),
            });
        }
        let figure: Self = serde_json::from_slice(artifact.payload()).map_err(|error| {
            ReportError::InvalidValue {
                field: "report-figure",
                message: error.to_string(),
            }
        })?;
        figure.validate()?;
        Ok(figure)
    }

    /// A figure draws something, at values a page can place.
    fn validate(&self) -> Result<(), ReportError> {
        let invalid = |message: &str| {
            Err(ReportError::InvalidValue {
                field: "report-figure",
                message: message.to_owned(),
            })
        };
        if self.traces.iter().all(|trace| trace.paths.is_empty()) {
            return invalid("a figure draws at least one trace");
        }
        let finite = |(x, y): &(f64, f64)| x.is_finite() && y.is_finite();
        if self
            .traces
            .iter()
            .flat_map(|trace| &trace.paths)
            .any(|path| path.is_empty() || !path.iter().all(finite))
        {
            return invalid("every stretch of a trace is finite samples");
        }
        if self.cursors.iter().any(|cursor| !cursor.x.is_finite()) {
            return invalid("a cursor stands at a finite abscissa");
        }
        for axis in [&self.x_axis, &self.y_axis] {
            if axis.pinned.is_some_and(|range| {
                !range.minimum.is_finite()
                    || !range.maximum.is_finite()
                    || range.minimum >= range.maximum
            }) {
                return invalid("a pinned window is an interval");
            }
        }
        Ok(())
    }
}

/// Keep one pane of `document` as a report figure, once the reference
/// proves the document, its revision and its datasets are the ones it names.
pub fn freeze_report_figure(
    document: &VisualizationDocument,
    reference: &ReportReferenceSnapshot,
    page_id: PageId,
    pane_id: PaneId,
) -> Result<FrozenReportArtifact, VisualizationRasterError> {
    let scene = resolve_cartesian_line_scene(document, reference, page_id, pane_id)?;
    let axis = |axis: &crate::visualization_document::Axis| FigureAxis {
        label: axis.label.clone(),
        unit: axis.unit.clone(),
        scale: axis.scale,
        pinned: axis.range,
    };
    let figure = ReportFigure {
        x_axis: axis(scene.x_axis()),
        y_axis: axis(scene.y_axis()),
        traces: scene
            .traces()
            .iter()
            .map(|trace| FigureTrace {
                label: trace.label().to_owned(),
                paths: trace
                    .points()
                    .split(|point| !point.x().is_finite() || !point.y().is_finite())
                    .filter(|path| !path.is_empty())
                    .map(|path| path.iter().map(|point| (point.x(), point.y())).collect())
                    .collect(),
            })
            .collect(),
        cursors: scene
            .cursors()
            .iter()
            .map(|cursor| FigureCursor {
                label: cursor.label().to_owned(),
                x: cursor.x(),
            })
            .collect(),
    };
    figure
        .artifact()
        .map_err(|error| VisualizationRasterError::ReportArtifact(error.to_string()))
}
