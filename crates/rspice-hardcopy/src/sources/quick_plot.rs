//! Deterministic plot pages built from exact series and captured reading controls.

mod axis;
mod overlay;

use super::*;
pub use axis::plot_axes;
use axis::project;
use overlay::resolved_overlay_geometry;
pub use overlay::{
    PlotFrame, RetainedCursorInterpolation, RetainedQuickMarker, RetainedQuickViewOverlay,
    RetainedQuickViewOverlays, RetainedQuickViewport,
};

#[derive(Debug)]
pub struct QuickResultSeries {
    pub identity: String,
    pub label: String,
    pub points: Vec<(f64, f64)>,
}

/// Build the printable plot for one sheet.
///
/// `overlay` is the reading the sheet was carrying — hidden traces already
/// applied by the caller, plus the cursors and markers this function places.
/// It is `None` for the sheets that compute their own abscissa: a marker
/// anchored in seconds has no position on a folded eye or a binned
/// distribution, and drawing it at one would be an invention.
pub fn quick_plot_from_series(
    viewer: ResultViewer,
    page: &str,
    pane_id: u64,
    series: Vec<QuickResultSeries>,
    overlay: Option<&RetainedQuickViewOverlay>,
) -> Result<SemanticPlot, HardcopySourceError> {
    quick_plot_from_scaled_series(
        viewer,
        page,
        pane_id,
        series,
        overlay,
        AxisScale::Linear,
        AxisScale::Linear,
    )
}

/// The same page, on axes that say how they map.
///
/// The geometry below is laid out in the axes' own space, so a decade of a
/// logarithmic sweep occupies the same width as every other decade. Retained
/// samples are untouched: they travel as the engine's own values, and only
/// the page coordinates move.
pub fn quick_plot_from_scaled_series(
    viewer: ResultViewer,
    page: &str,
    pane_id: u64,
    series: Vec<QuickResultSeries>,
    overlay: Option<&RetainedQuickViewOverlay>,
    x_scale: AxisScale,
    y_scale: AxisScale,
) -> Result<SemanticPlot, HardcopySourceError> {
    if series.is_empty() {
        return Err(HardcopySourceError::MissingViewerEvidence(
            "visible plot series",
        ));
    }
    if series.iter().any(|series| {
        series.points.is_empty()
            || series
                .points
                .iter()
                .any(|(x, y)| !x.is_finite() || !y.is_finite())
    }) {
        return Err(HardcopySourceError::InvalidRetainedWaveform(
            "active viewer series".to_owned(),
        ));
    }
    if let Some(overlay) = overlay {
        overlay.validate()?;
    }
    // A logarithmic axis has no position for a non-positive value, so those
    // samples are dropped rather than clamped — exactly as the sheet drops
    // them. A series that is entirely non-positive on a log axis has nothing
    // the page can show.
    let projected = series
        .iter()
        .map(|series| {
            series
                .points
                .iter()
                .filter_map(|&(x, y)| Some((project(x_scale, x)?, project(y_scale, y)?)))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    if projected.iter().any(Vec::is_empty) {
        return Err(HardcopySourceError::InvalidRetainedWaveform(
            "active viewer series has no sample its axis can place".to_owned(),
        ));
    }
    let extreme = |select: fn(&(f64, f64)) -> f64, pick: fn(&f64, &f64) -> std::cmp::Ordering| {
        projected
            .iter()
            .flat_map(|points| points.iter().map(select))
            .min_by(pick)
            .ok_or(HardcopySourceError::InvalidResultRange)
    };
    let x_minimum = extreme(|point| point.0, f64::total_cmp)?;
    let x_maximum = extreme(|point| point.0, |left, right| right.total_cmp(left))?;
    let y_minimum = extreme(|point| point.1, f64::total_cmp)?;
    let y_maximum = extreme(|point| point.1, |left, right| right.total_cmp(left))?;
    // The window the reader had pinned is part of the sheet they captured,
    // exactly as the hidden traces and the placed cursors are.
    let data_bounds = (x_minimum, x_maximum, y_minimum, y_maximum);
    let (x_minimum, x_maximum, y_minimum, y_maximum) = overlay.map_or(data_bounds, |overlay| {
        overlay.framed_bounds(x_scale, y_scale, data_bounds)
    });
    let (x_minimum, x_maximum) = nondegenerate_range(x_minimum, x_maximum);
    let (y_minimum, y_maximum) = nondegenerate_range(y_minimum, y_maximum);
    let plot_width = PLOT_WIDTH_UM - 2 * PLOT_INSET_UM;
    let plot_height = PLOT_HEIGHT_UM - 2 * PLOT_INSET_UM;
    let frame = PlotFrame {
        x_minimum,
        x_maximum,
        y_minimum,
        y_maximum,
        x_span: x_maximum - x_minimum,
        y_span: y_maximum - y_minimum,
        plot_width,
        plot_height,
    };
    let (axis_ticks, captions) = plot_axes(x_scale, y_scale, &frame)?;
    let (cursors, markers) = overlay.map_or_else(
        || Ok((Vec::new(), Vec::new())),
        |overlay| resolved_overlay_geometry(viewer, overlay, &series, x_scale, y_scale, &frame),
    )?;
    let mut trace_ids = std::collections::HashSet::new();
    let traces = series
        .iter()
        .zip(projected.iter())
        .enumerate()
        .map(|(index, (series, points))| {
            let trace_id = stable_quick_trace_id(viewer, index, &series.identity);
            if !trace_ids.insert(trace_id) {
                return Err(HardcopySourceError::DuplicateStableTraceIdentity(trace_id));
            }
            Ok(SemanticPlotTrace {
                trace_id,
                label: series.label.clone(),
                paths: clipped_plot_paths(
                    points,
                    x_minimum,
                    x_maximum,
                    y_minimum,
                    y_maximum,
                    plot_width,
                    plot_height,
                )?,
                source_samples: series
                    .points
                    .iter()
                    .map(|(x, y)| (x.to_bits(), y.to_bits()))
                    .collect(),
            })
        })
        .collect::<Result<Vec<_>, HardcopySourceError>>()?;
    Ok(SemanticPlot {
        viewer,
        page_id: stable_page_id(page),
        pane_id,
        x_scale,
        y_scale,
        axis_ticks,
        traces,
        cursors,
        markers,
        annotations: Vec::new(),
        captions,
    })
}

pub fn stable_quick_trace_id(viewer: ResultViewer, index: usize, identity: &str) -> u64 {
    let mut hasher = Sha256::new();
    hasher.update(b"rspice-hardcopy-results-trace-v1");
    hasher.update(viewer.label().as_bytes());
    hasher.update((index as u64).to_be_bytes());
    hasher.update(identity.as_bytes());
    let bytes: [u8; 8] = hasher.finalize()[..8]
        .try_into()
        .expect("SHA-256 prefix has fixed length");
    u64::from_be_bytes(bytes)
}

pub fn nondegenerate_range(minimum: f64, maximum: f64) -> (f64, f64) {
    if minimum < maximum {
        (minimum, maximum)
    } else {
        let padding = (minimum.abs() * 0.05).max(1.0e-12);
        (minimum - padding, maximum + padding)
    }
}

pub fn stable_page_id(page: &str) -> u64 {
    let digest = Sha256::digest([b"rspice-studio-page-id-v1".as_slice(), page.as_bytes()].concat());
    let mut bytes = [0_u8; 8];
    bytes.copy_from_slice(&digest[..8]);
    u64::from_be_bytes(bytes).max(1)
}

#[cfg(test)]
mod tests;
