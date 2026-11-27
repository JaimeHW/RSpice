//! A saved result document's pane, printed as its sheet prints.
//!
//! The pane is laid out by the builder that lays out a run's sheet: a strip
//! for each unit under one abscissa, each ruled and valued, the abscissa
//! named with its unit, the traces keyed in the sheet's colours, and the
//! pane's cursors and markers standing on them. A pane the reader zoomed
//! prints the window they pinned, and a strip they set logarithmic is ruled
//! by its decades.

use super::quick_plot::quick_plot_in;
use super::result_view::{drawn_bounds, keyed_as_the_sheet, series_unit, with_fact_row};
use super::*;
use rspice_results::analysis_type::AnalysisType;
use rspice_results::bode::retained::response_signal;
use rspice_results::nyquist::LOOP_PHASE_TRACE;
use rspice_results::pane_figure::{
    PaneFigure, PaneFigureNoteAnchor, PaneFigureTrace, resolve_pane_figure,
};
use rspice_results::result_presentation::{MarkerKind, MarkerOffset};
use rspice_results::visualization_document::PlotMarkerKind;

/// Resolve an active result-document pane without depending on a window,
/// screenshot, framebuffer, or transient viewer cache.
pub fn resolve_visualization_pane_source(
    source: VisualizationPaneHardcopySource<'_>,
) -> Result<ResolvedHardcopyDocument, HardcopySourceError> {
    validate_label("source key", &source.source_key, SOURCE_KEY_LIMIT)?;
    validate_label("display name", &source.display_name, DISPLAY_NAME_LIMIT)?;
    if !matches!(
        &source.scope,
        HardcopyScope::ActivePlotDocument | HardcopyScope::ActiveDocument
    ) {
        return Err(HardcopySourceError::UnsupportedScope(source.scope));
    }
    for (name, reading) in source.figures {
        validate_label("printed figure name", name, DISPLAY_NAME_LIMIT)?;
        validate_label("printed figure", reading, DISPLAY_NAME_LIMIT)?;
    }
    let figure = resolve_pane_figure(
        source.document,
        source.reference,
        source.page_id,
        source.pane_id,
    )
    .map_err(map_visualization_error)?;
    let plot = pane_plot(&figure, source.page_id, source.pane_id, source.figures)?;
    // Its strips take the page, as a sheet's do.
    let bounds = drawn_bounds(&plot)?;
    let semantic_document = HardcopySemanticDocument::Plot(plot);
    let digest = canonical_digest(
        b"rspice-hardcopy-visualization-pane-v3",
        &(source.reference.content_digest, &semantic_document),
    )?;
    let identity = HardcopySourceIdentity::try_new(
        source.source_key,
        HardcopyDocumentId::try_from_uuid(source.document.id().as_uuid())
            .map_err(|error| HardcopySourceError::HardcopyContract(error.to_string()))?,
        source.document.revision(),
        source.display_name,
    )?;
    resolve_semantic_source(
        identity,
        digest,
        HardcopyDocumentKind::PlotOrWorksheet,
        source.scope,
        semantic_document,
        bounds,
    )
}

/// The page `figure` prints as: the page its sheet prints for the run,
/// with the line of figures a Bode pane's card reads under it.
fn pane_plot(
    figure: &PaneFigure,
    page_id: PageId,
    pane_id: PaneId,
    figures: &[(String, String)],
) -> Result<SemanticPlot, HardcopySourceError> {
    let bode = figure.viewer_id == "viewer-bode";
    let viewer = if bode {
        ResultViewer::Bode
    } else {
        ResultViewer::Waves
    };
    let (series, keys): (Vec<_>, Vec<_>) = figure
        .traces
        .iter()
        .map(|trace| printed_series(figure, trace, bode))
        .unzip();
    let overlay = pane_overlay(figure, &series)?;
    let abscissa = match figure.analysis {
        // A pane of a run states its abscissa as the run's sheet does.
        Some(analysis) => {
            let (quantity, unit, _, _) = analysis.axis_info();
            QuickAbscissa::new(
                quantity,
                figure.x_axis.unit.as_deref().unwrap_or(unit),
                figure.x_axis.scale,
            )
        }
        None => QuickAbscissa::new(
            figure.x_axis.label.clone(),
            figure.x_axis.unit.clone().unwrap_or_default(),
            figure.x_axis.scale,
        ),
    };
    // A strip is logarithmic when the axis its traces read against is, or
    // when the pane keeps its unit on a logarithmic axis, as the sheet keeps
    // the scale of each unit pane. A plain number's pane is kept as "1".
    let logarithmic =
        |axis: &rspice_results::visualization_document::Axis| axis.scale == AxisScale::Logarithmic;
    let log_units = figure
        .traces
        .iter()
        .zip(&series)
        .filter(|(trace, series)| {
            let kept = if series.unit.is_empty() {
                "1"
            } else {
                &series.unit
            };
            logarithmic(&trace.y_axis)
                || figure
                    .y_axes
                    .iter()
                    .any(|axis| axis.unit.as_deref() == Some(kept) && logarithmic(axis))
        })
        .map(|(_, series)| series.unit.clone())
        .collect::<Vec<_>>();
    let mut plot = quick_plot_in(
        PlotArea::PAGE,
        viewer,
        &format!("result-document-page-{}", page_id.get()),
        pane_id.get(),
        series,
        Some(&overlay),
        &abscissa,
        |unit| {
            if log_units.iter().any(|log| log == unit) {
                AxisScale::Logarithmic
            } else {
                AxisScale::Linear
            }
        },
    )?;
    keyed_as_the_sheet(&mut plot, &keys.into_iter().flatten().collect::<Vec<_>>());
    Ok(if bode {
        with_fact_row(plot, figures)
    } else {
        plot
    })
}

/// One trace as its sheet prints it, and the name its key gives it where
/// that is not its label. A Bode sheet draws a response's magnitude in
/// decibels and its phase in degrees, each keyed without the unit its axis
/// states.
fn printed_series(
    figure: &PaneFigure,
    trace: &PaneFigureTrace,
    bode: bool,
) -> (QuickResultSeries, Option<(String, String)>) {
    let identity = format!("{}:{}", trace.trace_id.get(), trace.signal);
    let analysis = figure.analysis;
    let response = bode
        .then(|| analysis.and_then(|analysis| response_signal(analysis, &trace.signal)))
        .flatten();
    if let Some(signal) = response {
        // A stability analysis retains its loop gain in decibels; every
        // other response retains a linear magnitude, which has no decibels
        // at zero.
        let decibels = analysis == Some(AnalysisType::Stb);
        let label = format!("|{signal}| (dB)");
        let key = (label.clone(), format!("|{signal}|"));
        let points = trace
            .points
            .iter()
            .map(|&(frequency, gain)| match gain {
                _ if decibels => (frequency, gain),
                gain if gain > 0.0 => (frequency, 20.0 * gain.log10()),
                _ => (frequency, f64::NAN),
            })
            .collect();
        let series = QuickResultSeries {
            identity,
            label,
            unit: "dB".to_owned(),
            points,
        };
        return (series, Some(key));
    }
    if bode && (trace.signal.starts_with("phase(") || trace.signal.starts_with(LOOP_PHASE_TRACE)) {
        let label = format!("{} (\u{b0})", trace.signal);
        let key = (label.clone(), trace.signal.clone());
        let series = QuickResultSeries {
            identity,
            label,
            unit: "\u{b0}".to_owned(),
            points: trace.points.clone(),
        };
        return (series, Some(key));
    }
    let series = QuickResultSeries {
        identity,
        label: trace.label.clone(),
        unit: series_unit(
            &trace.signal,
            trace.unit.as_deref(),
            trace.y_axis.unit.as_deref(),
            analysis,
        ),
        points: trace.points.clone(),
    };
    (series, None)
}

/// The pane's cursors, markers and notes as the reading a sheet's print
/// carries, and the window the reader pinned it to.
fn pane_overlay(
    figure: &PaneFigure,
    series: &[QuickResultSeries],
) -> Result<RetainedQuickViewOverlay, HardcopySourceError> {
    // A cursor is a full-height line at its abscissa, named as the pane
    // names it: a marker that rides no trace.
    let cursors = figure.cursors.iter().map(|cursor| RetainedQuickMarker {
        label: match cursor.label.trim() {
            "" => "Cursor".to_owned(),
            label => label.to_owned(),
        },
        kind: MarkerKind::Note,
        x: cursor.x,
        trace_name: None,
    });
    // A marker is tagged as the sheet tags it, by its id and the reader's
    // note, and rides the series of its trace; a limit stands across the
    // strip, since it constrains the axis rather than one curve.
    let markers = figure.markers.iter().filter_map(|marker| {
        let index = figure
            .traces
            .iter()
            .position(|trace| trace.trace_id == marker.trace_id)?;
        let kind = marker_kind(marker.kind);
        let id = format!("M{}", marker.marker_id.get());
        let note = marker.label.trim();
        let label = if note.is_empty() || note == figure.traces[index].label {
            id
        } else {
            format!("{id} \u{b7} {note}")
        };
        Some(RetainedQuickMarker {
            label,
            kind,
            x: marker.x,
            trace_name: (kind != MarkerKind::Spec).then(|| series[index].label.clone()),
        })
    });
    // A note pinned to a trace rides it, lettered with what the reader
    // wrote; one pinned to a place in the plot or a point of its data stands
    // there.
    let mut on_traces = Vec::new();
    let mut on_plot = Vec::new();
    for note in &figure.notes {
        let anchor = match &note.anchor {
            PaneFigureNoteAnchor::Trace { trace_id, x } => {
                if let Some(index) = figure
                    .traces
                    .iter()
                    .position(|trace| trace.trace_id == *trace_id)
                {
                    on_traces.push(RetainedQuickMarker {
                        label: note.text.clone(),
                        kind: MarkerKind::Note,
                        x: *x,
                        trace_name: Some(series[index].label.clone()),
                    });
                }
                continue;
            }
            PaneFigureNoteAnchor::Place {
                unit,
                x_fraction,
                y_fraction,
            } => RetainedQuickNoteAnchor::Place {
                unit: unit.clone(),
                x_fraction: f64::from(*x_fraction),
                y_fraction: f64::from(*y_fraction),
            },
            PaneFigureNoteAnchor::Point { unit, x, y } => RetainedQuickNoteAnchor::Point {
                unit: unit.clone(),
                x: *x,
                y: *y,
            },
        };
        on_plot.push(RetainedQuickNote {
            id: note.id,
            text: note.text.clone(),
            anchor,
        });
    }
    // A pinned ordinate names one axis's window, so it holds only where
    // every trace reads against that axis.
    let y = match figure.traces.as_slice() {
        [first, rest @ ..] if rest.iter().all(|trace| trace.y_axis.id == first.y_axis.id) => {
            first.y_axis.range
        }
        _ => None,
    };
    RetainedQuickViewOverlay::try_new(
        None,
        None,
        None,
        cursors.chain(markers).chain(on_traces).collect(),
        RetainedCursorInterpolation::Linear,
        Some(RetainedQuickViewport {
            x: figure
                .x_axis
                .range
                .map(|range| (range.minimum, range.maximum)),
            y: y.map(|range| (range.minimum, range.maximum)),
        }),
    )?
    .with_notes(on_plot)
}

/// What a retained marker is on the sheet.
const fn marker_kind(kind: PlotMarkerKind) -> MarkerKind {
    match kind {
        PlotMarkerKind::PointNote | PlotMarkerKind::MeasurementAnchor => MarkerKind::Note,
        PlotMarkerKind::Peak { size } => MarkerKind::Peak(size),
        PlotMarkerKind::Valley { size } => MarkerKind::Valley(size),
        PlotMarkerKind::Crossing { level } => MarkerKind::Crossing(level),
        PlotMarkerKind::SpecificationLine => MarkerKind::Spec,
        PlotMarkerKind::VerticalLine => MarkerKind::Vertical,
        PlotMarkerKind::Offset { from, by } => MarkerKind::Offset(MarkerOffset { from, by }),
    }
}

fn map_visualization_error(error: VisualizationRasterError) -> HardcopySourceError {
    match error {
        error @ (VisualizationRasterError::PageNotFound(_)
        | VisualizationRasterError::PaneNotFound(_)
        | VisualizationRasterError::DatasetNotFound(_)
        | VisualizationRasterError::EmptyTrace(_)
        | VisualizationRasterError::NoVisibleTraces) => {
            HardcopySourceError::UnretainedResult(error.to_string())
        }
        error => HardcopySourceError::InvalidVisualizationSource(error.to_string()),
    }
}
