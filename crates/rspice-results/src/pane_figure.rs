//! A result-document pane, read as print lays it out.
//!
//! A printed pane is laid out as a printed sheet is: one strip per unit under
//! one abscissa, ruled, valued and keyed. So it is read here with every
//! vertical axis it has, each trace with the axis it reads against and the
//! unit its source states for it, rather than as the one-ordinate scene a
//! raster figure maps linearly.

use rspice_vocabulary::product::DatasetBinding;

use crate::analysis_type::AnalysisType;
use crate::report_document::{ReportReferenceSnapshot, ReportSourceId};
use crate::report_figure::FigureCursor;
use crate::visualization_document::{
    Axis, AxisOrientation, ColumnRole, MarkerId, PageId, PaneId, PaneKind, PlotMarkerKind,
    SourceDataset, Trace, TraceId, TypedValue, ValueType, VisualizationDocument,
};
use crate::visualization_raster::{
    MAX_RASTER_CURSORS, MAX_RASTER_TRACES, VisualizationRasterError,
    visualization_document_content_digest,
};

/// The sheets whose panes draw each trace as a line over one abscissa.
pub const LINE_VIEWERS: [&str; 2] = ["viewer-waveform", "viewer-bode"];

/// The largest integer an `f64` holds exactly.
const MAX_EXACT_F64_INTEGER: i64 = 9_007_199_254_740_992;

/// One pane of a result document, as print reads it.
#[derive(Debug, Clone, PartialEq)]
pub struct PaneFigure {
    /// The sheet the pane is drawn by.
    pub viewer_id: String,
    /// The kind of analysis the pane draws, when it draws one of a run.
    pub analysis: Option<AnalysisType>,
    /// The pane's abscissa, as it names, scales and pins it.
    pub x_axis: Axis,
    /// Every vertical axis of the pane, a trace reading against it or not:
    /// a pane keeps the scale of each unit's strip on an axis of that unit.
    pub y_axes: Vec<Axis>,
    /// The traces the pane shows, in its order.
    pub traces: Vec<PaneFigureTrace>,
    /// The pane's cursors, each a full-height line at its abscissa.
    pub cursors: Vec<FigureCursor>,
    /// The markers that ride the traces it shows.
    pub markers: Vec<PaneFigureMarker>,
}

/// One trace of a printed pane.
#[derive(Debug, Clone, PartialEq)]
pub struct PaneFigureTrace {
    pub trace_id: TraceId,
    /// What the reader sees it called.
    pub label: String,
    /// The signal it reads, by the name its source gives it.
    pub signal: String,
    /// The unit its source states its values in; `None` where it states none.
    pub unit: Option<String>,
    /// The vertical axis it reads against.
    pub y_axis: Axis,
    /// Its samples in source values; a `NaN` value breaks the line.
    pub points: Vec<(f64, f64)>,
}

/// One marker of a printed pane, on the trace it rides.
#[derive(Debug, Clone, PartialEq)]
pub struct PaneFigureMarker {
    pub marker_id: MarkerId,
    pub trace_id: TraceId,
    pub label: String,
    pub kind: PlotMarkerKind,
    pub x: f64,
}

/// Read one pane of `document` for print, once the reference proves the
/// document, its revision and the datasets the pane reads are the ones it
/// names.
pub fn resolve_pane_figure(
    document: &VisualizationDocument,
    reference: &ReportReferenceSnapshot,
    page_id: PageId,
    pane_id: PaneId,
) -> Result<PaneFigure, VisualizationRasterError> {
    authenticate(document, reference)?;
    document
        .pages()
        .iter()
        .find(|page| page.id == page_id)
        .ok_or(VisualizationRasterError::PageNotFound(page_id.get()))?;
    let pane = document
        .panes()
        .iter()
        .find(|pane| pane.id == pane_id)
        .ok_or(VisualizationRasterError::PaneNotFound(pane_id.get()))?;
    if pane.page_id != page_id {
        return Err(VisualizationRasterError::PanePageMismatch {
            pane_id: pane_id.get(),
            page_id: page_id.get(),
            actual_page_id: pane.page_id.get(),
        });
    }
    if pane.kind != PaneKind::Cartesian {
        return Err(VisualizationRasterError::UnsupportedPaneKind(pane.kind));
    }
    if !LINE_VIEWERS.contains(&pane.viewer_id.as_str()) {
        return Err(VisualizationRasterError::UnsupportedViewer(
            pane.viewer_id.clone(),
        ));
    }
    if pane.family_policy.is_some() {
        return Err(VisualizationRasterError::UnsupportedStyleMode(
            "family presentation",
        ));
    }
    if document
        .measurements()
        .iter()
        .any(|measurement| measurement.pane_id == pane_id)
    {
        return Err(VisualizationRasterError::UnsupportedOverlay("measurement"));
    }

    let axes = document
        .axes()
        .iter()
        .filter(|axis| axis.pane_id == pane_id)
        .collect::<Vec<_>>();
    let mut horizontal = axes
        .iter()
        .filter(|axis| axis.orientation == AxisOrientation::Horizontal);
    let (Some(x_axis), None) = (horizontal.next(), horizontal.next()) else {
        return Err(VisualizationRasterError::UnsupportedAxisConfiguration(
            "a printed pane reads against one horizontal axis".to_owned(),
        ));
    };
    if axes.iter().any(|axis| {
        matches!(
            axis.orientation,
            AxisOrientation::Radial | AxisOrientation::Angular
        )
    }) {
        return Err(VisualizationRasterError::UnsupportedAxisConfiguration(
            "a printed pane has no polar axis".to_owned(),
        ));
    }

    if let Some(binding) = pane.binding {
        authenticated_dataset(document, reference, binding.dataset)?;
    }
    let analysis = pane.binding.and_then(|binding| {
        let dataset = document
            .datasets()
            .iter()
            .find(|dataset| dataset.binding() == binding.dataset)?;
        Some(
            dataset
                .run_source()?
                .analysis(binding.analysis_id)?
                .analysis_type(),
        )
    });

    let shown = document
        .traces()
        .iter()
        // A trace whose run lacks its signal draws nothing.
        .filter(|trace| trace.pane_id == pane_id && trace.visible && !trace.missing)
        .collect::<Vec<_>>();
    if shown.len() > MAX_RASTER_TRACES {
        return Err(VisualizationRasterError::ResourceLimit {
            resource: "printed pane traces",
            maximum: MAX_RASTER_TRACES,
            actual: shown.len(),
        });
    }
    let mut traces = Vec::with_capacity(shown.len());
    for trace in shown {
        let y_axis = axes
            .iter()
            .find(|axis| {
                axis.id == trace.y_axis_id
                    && matches!(
                        axis.orientation,
                        AxisOrientation::VerticalLeft | AxisOrientation::VerticalRight
                    )
            })
            .filter(|_| trace.x_axis_id == x_axis.id)
            .ok_or_else(|| {
                VisualizationRasterError::UnsupportedAxisConfiguration(format!(
                    "trace {} reads against the pane's horizontal axis and one of its vertical axes",
                    trace.id.get()
                ))
            })?;
        let dataset = authenticated_dataset(document, reference, trace.binding)?;
        let Some((points, unit)) = trace_points(dataset, trace)? else {
            continue;
        };
        traces.push(PaneFigureTrace {
            trace_id: trace.id,
            label: trace.label.clone(),
            signal: trace.source_signal().unwrap_or(&trace.label).to_owned(),
            unit,
            y_axis: (*y_axis).clone(),
            points,
        });
    }
    if traces.is_empty() {
        return Err(VisualizationRasterError::NoVisibleTraces);
    }

    let pane_cursors = document
        .cursors()
        .iter()
        .filter(|cursor| cursor.pane_id == pane_id)
        .collect::<Vec<_>>();
    if pane_cursors.len() > MAX_RASTER_CURSORS {
        return Err(VisualizationRasterError::ResourceLimit {
            resource: "printed pane cursors",
            maximum: MAX_RASTER_CURSORS,
            actual: pane_cursors.len(),
        });
    }
    let mut cursors = Vec::with_capacity(pane_cursors.len());
    for cursor in pane_cursors {
        if cursor.axis_id != x_axis.id {
            return Err(VisualizationRasterError::UnsupportedAxisConfiguration(
                format!(
                    "cursor {} stands on the pane's horizontal axis",
                    cursor.id.get()
                ),
            ));
        }
        let x = exact_position(&cursor.position).ok_or(
            VisualizationRasterError::UnsupportedOverlay("non-numeric or inexact cursor"),
        )?;
        cursors.push(FigureCursor {
            label: cursor.label.clone(),
            x,
        });
    }
    // A marker on a trace the pane does not show has nothing to ride.
    let markers = document
        .markers()
        .iter()
        .filter(|marker| marker.pane_id == pane_id)
        .filter(|marker| traces.iter().any(|trace| trace.trace_id == marker.trace_id))
        .filter_map(|marker| {
            Some(PaneFigureMarker {
                marker_id: marker.id,
                trace_id: marker.trace_id,
                label: marker.label.clone(),
                kind: marker.kind,
                x: exact_position(&marker.coordinate)?,
            })
        })
        .collect();

    Ok(PaneFigure {
        viewer_id: pane.viewer_id.clone(),
        analysis,
        x_axis: (*x_axis).clone(),
        y_axes: axes
            .iter()
            .filter(|axis| {
                matches!(
                    axis.orientation,
                    AxisOrientation::VerticalLeft | AxisOrientation::VerticalRight
                )
            })
            .map(|axis| (*axis).clone())
            .collect(),
        traces,
        cursors,
        markers,
    })
}

/// The reference names `document` itself: its identity, its revision and
/// its content.
fn authenticate(
    document: &VisualizationDocument,
    reference: &ReportReferenceSnapshot,
) -> Result<(), VisualizationRasterError> {
    let ReportSourceId::VisualizationDocument { document_id } = &reference.source else {
        return Err(VisualizationRasterError::InvalidReferenceSource);
    };
    if *document_id != document.id() {
        return Err(VisualizationRasterError::DocumentIdentityMismatch {
            expected: *document_id,
            actual: document.id(),
        });
    }
    let revision = reference
        .source_revision
        .ok_or(VisualizationRasterError::MissingSourceRevision)?;
    if revision != document.revision() {
        return Err(VisualizationRasterError::RevisionMismatch {
            expected: revision,
            actual: document.revision(),
        });
    }
    let digest = visualization_document_content_digest(document)?;
    if digest != reference.content_digest {
        return Err(VisualizationRasterError::SourceDigestMismatch {
            expected: reference.content_digest,
            actual: digest,
        });
    }
    Ok(())
}

/// The dataset `binding` names, held by the document under that digest and
/// named by the reference under it.
fn authenticated_dataset<'d>(
    document: &'d VisualizationDocument,
    reference: &ReportReferenceSnapshot,
    binding: DatasetBinding,
) -> Result<&'d SourceDataset, VisualizationRasterError> {
    let dataset = document
        .datasets()
        .iter()
        .find(|dataset| dataset.binding().dataset_id == binding.dataset_id)
        .ok_or(VisualizationRasterError::DatasetNotFound(
            binding.dataset_id,
        ))?;
    if dataset.binding().content_digest != binding.content_digest {
        return Err(VisualizationRasterError::DatasetDigestMismatch {
            dataset_id: binding.dataset_id,
            requested: binding.content_digest,
            actual: dataset.binding().content_digest,
        });
    }
    if !reference.dataset_bindings.contains(&binding) {
        return Err(VisualizationRasterError::DatasetManifestMismatch {
            expected: reference.dataset_bindings.clone(),
            actual: vec![binding],
        });
    }
    Ok(dataset)
}

/// The samples one trace draws from `dataset`, and the unit its source
/// states them in. `None` for a run signal a report copy carries no samples
/// of, since the run has no signal by that name.
fn trace_points(
    dataset: &SourceDataset,
    trace: &Trace,
) -> Result<Option<(Vec<(f64, f64)>, Option<String>)>, VisualizationRasterError> {
    let empty = || VisualizationRasterError::EmptyTrace(trace.id.get());
    if let Some(run) = dataset.run_source() {
        // A run reference draws only from the samples a report copy of the
        // document carries; the run store is not consulted here.
        let carried = run
            .select_one(&trace.row_predicates)
            .ok()
            .and_then(|series| run.figure_samples(&series));
        return match carried {
            Some(samples) if !samples.x().is_empty() => Ok(Some((
                samples.x().iter().copied().zip(samples.y().iter().copied()).collect(),
                samples.unit().map(str::to_owned),
            ))),
            None if run.carries_figure_samples() => Ok(None),
            _ => Err(empty()),
        };
    }
    let column = |key: &str| {
        dataset
            .columns()
            .iter()
            .position(|column| column.key() == key)
            .ok_or_else(|| VisualizationRasterError::ColumnNotFound {
                trace_id: trace.id.get(),
                column: key.to_owned(),
            })
    };
    let (x_index, y_index) = (column(&trace.coordinate_key)?, column(&trace.signal_key)?);
    let predicates = trace
        .row_predicates
        .iter()
        .map(|predicate| Ok((column(&predicate.column)?, &predicate.value)))
        .collect::<Result<Vec<_>, VisualizationRasterError>>()?;
    let (x_column, y_column) = (&dataset.columns()[x_index], &dataset.columns()[y_index]);
    // A trace may be drawn against another of its dataset's signals, row by
    // row, as a transfer curve is.
    let signal_abscissa = x_column.role() == ColumnRole::Signal && x_index != y_index;
    if !(x_column.role() == ColumnRole::Coordinate || signal_abscissa)
        || y_column.role() != ColumnRole::Signal
    {
        return Err(VisualizationRasterError::UnsupportedStyleMode(
            "trace source-column role",
        ));
    }
    for column in [x_column, y_column] {
        if !matches!(column.value_type(), ValueType::Real | ValueType::Integer) {
            return Err(VisualizationRasterError::UnsupportedValueType {
                trace_id: trace.id.get(),
                column: column.key().to_owned(),
                value_type: column.value_type(),
            });
        }
    }
    let number = |value: &TypedValue, column: &str| match value {
        TypedValue::Real(value) if value.is_finite() => Ok(*value),
        TypedValue::Integer(value)
            if (-MAX_EXACT_F64_INTEGER..=MAX_EXACT_F64_INTEGER).contains(value) =>
        {
            Ok(*value as f64)
        }
        TypedValue::Integer(value) => Err(VisualizationRasterError::InexactInteger {
            trace_id: trace.id.get(),
            column: column.to_owned(),
            value: *value,
        }),
        other => Err(VisualizationRasterError::UnsupportedValueType {
            trace_id: trace.id.get(),
            column: column.to_owned(),
            value_type: other.value_type(),
        }),
    };
    let mut points = Vec::new();
    for row in dataset.rows() {
        let values = row.values();
        if !predicates
            .iter()
            .all(|(index, expected)| values[*index].exact_eq(expected))
        {
            continue;
        }
        // A sample the source does not have breaks the line where the line
        // has reached.
        if signal_abscissa && values[x_index].is_missing() {
            if let Some(&(x, _)) = points.last() {
                points.push((x, f64::NAN));
            }
            continue;
        }
        let x = number(&values[x_index], x_column.key())?;
        let y = if values[y_index].is_missing() {
            f64::NAN
        } else {
            number(&values[y_index], y_column.key())?
        };
        points.push((x, y));
    }
    if points.is_empty() {
        return Err(empty());
    }
    Ok(Some((points, y_column.unit().map(str::to_owned))))
}

/// A cursor's or marker's position as an exact abscissa.
fn exact_position(value: &TypedValue) -> Option<f64> {
    let x = match value {
        TypedValue::Real(value) => *value,
        TypedValue::Integer(value) if value.abs() <= MAX_EXACT_F64_INTEGER => *value as f64,
        _ => return None,
    };
    x.is_finite().then_some(x)
}
