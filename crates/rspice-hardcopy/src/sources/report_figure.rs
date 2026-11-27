//! A report figure, laid out as a printed plot.
//!
//! A report keeps a figure as the plot its pane draws. Print lays it out
//! as it lays out a sheet: its traces in one strip under one abscissa,
//! ruled and valued on the scales the pane reads them on, with its cursors
//! standing across it, at the size the report gives the figure rather than
//! a page's.

use super::quick_plot::quick_plot_in;
use super::*;
use rspice_results::report_figure::{FigureAxis, ReportFigure};
use rspice_results::result_presentation::MarkerKind;

/// The plot `figure` prints as, its frames standing in `area`.
pub(crate) fn report_figure_plot(
    figure: &ReportFigure,
    area: PlotArea,
) -> Result<SemanticPlot, HardcopySourceError> {
    let unit = figure.y_axis.unit.clone().unwrap_or_default();
    let series = figure
        .traces
        .iter()
        .filter(|trace| !trace.paths.is_empty())
        .map(|trace| QuickResultSeries {
            identity: trace.label.clone(),
            label: trace.label.clone(),
            unit: unit.clone(),
            // Two stretches of a trace are set apart by a sample the run
            // does not have, which the line breaks at, as on the sheet.
            points: trace
                .paths
                .iter()
                .enumerate()
                .flat_map(|(index, path)| {
                    let gap = (index > 0).then(|| (path[0].0, f64::NAN));
                    gap.into_iter().chain(path.iter().copied())
                })
                .collect(),
        })
        .collect();
    let window = |axis: &FigureAxis| axis.pinned.map(|range| (range.minimum, range.maximum));
    // A cursor is a full-height line at its abscissa, named as the pane
    // names it: a marker that rides no trace.
    let cursors = figure
        .cursors
        .iter()
        .map(|cursor| RetainedQuickMarker {
            label: match cursor.label.trim() {
                "" => "Cursor".to_owned(),
                label => label.to_owned(),
            },
            kind: MarkerKind::Note,
            x: cursor.x,
            trace_name: None,
        })
        .collect();
    let overlay = RetainedQuickViewOverlay::try_new(
        None,
        None,
        None,
        cursors,
        RetainedCursorInterpolation::default(),
        Some(RetainedQuickViewport {
            x: window(&figure.x_axis),
            y: window(&figure.y_axis),
        }),
    )?;
    let abscissa = QuickAbscissa::new(
        figure.x_axis.label.clone(),
        figure.x_axis.unit.clone().unwrap_or_default(),
        figure.x_axis.scale,
    );
    quick_plot_in(
        area,
        ResultViewer::Waves,
        "report-figure",
        0,
        series,
        Some(&overlay),
        &abscissa,
        |_| figure.y_axis.scale,
    )
}
