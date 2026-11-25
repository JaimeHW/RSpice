//! Deterministic plot pages built from exact series and captured reading controls.

mod axis;
mod columns;
mod overlay;
mod strips;

use super::*;
pub use axis::plot_axes;
use axis::{project, ruled_axis};
pub(crate) use columns::print_read;
pub use columns::{PRINT_COLUMNS_PER_INCH, print_columns};
use overlay::resolved_overlay_geometry;
pub use overlay::{
    PlotFrame, RetainedCursorInterpolation, RetainedQuickMarker, RetainedQuickViewOverlay,
    RetainedQuickViewOverlays, RetainedQuickViewport,
};
pub use strips::QuickAbscissa;
use strips::{StripRect, strip_rects};

#[derive(Debug)]
pub struct QuickResultSeries {
    pub identity: String,
    pub label: String,
    /// The unit the ordinate reads in. Series of one unit share a strip,
    /// whose top tick states the unit, as the sheet opens one strip per unit.
    pub unit: String,
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
    abscissa: &QuickAbscissa,
) -> Result<SemanticPlot, HardcopySourceError> {
    quick_plot_from_scaled_series(
        viewer,
        page,
        pane_id,
        series,
        overlay,
        abscissa,
        AxisScale::Linear,
    )
}

/// The same page, on axes that say how they map.
///
/// The geometry below is laid out in the axes' own space, so a decade of a
/// logarithmic sweep occupies the same width as every other decade. Retained
/// samples are untouched: they travel as the engine's own values, and only
/// the page coordinates move.
///
/// Series of one unit share a strip and series of another unit get their
/// own, stacked under one abscissa as the sheet stacks them: a 1 mA current
/// drawn against a 1 V axis is a flat line at zero.
pub fn quick_plot_from_scaled_series(
    viewer: ResultViewer,
    page: &str,
    pane_id: u64,
    series: Vec<QuickResultSeries>,
    overlay: Option<&RetainedQuickViewOverlay>,
    abscissa: &QuickAbscissa,
    y_scale: AxisScale,
) -> Result<SemanticPlot, HardcopySourceError> {
    let x_scale = abscissa.scale;
    if series.is_empty() {
        return Err(HardcopySourceError::MissingViewerEvidence(
            "visible plot series",
        ));
    }
    // An unavailable sample (NaN) is a gap the line breaks at, as the sheet
    // draws it; an infinity is no sample at all.
    if series.iter().any(|series| {
        series.points.is_empty()
            || series
                .points
                .iter()
                .any(|(x, y)| !x.is_finite() || y.is_infinite())
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
    // the page can show. Each projected point keeps the index of the sample
    // it came from.
    let projected = series
        .iter()
        .map(|series| {
            series
                .points
                .iter()
                .enumerate()
                .filter_map(|(index, &(x, y))| {
                    let y = if y.is_nan() {
                        f64::NAN
                    } else {
                        project(y_scale, y)?
                    };
                    Some((index, (project(x_scale, x)?, y)))
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    if projected
        .iter()
        .any(|points| points.iter().all(|(_, (_, y))| y.is_nan()))
    {
        return Err(HardcopySourceError::InvalidRetainedWaveform(
            "active viewer series has no sample its axis can place".to_owned(),
        ));
    }
    // One strip per unit, in the order the units first appear.
    let mut units: Vec<&str> = Vec::new();
    let strip_of = series
        .iter()
        .map(|series| {
            units
                .iter()
                .position(|unit| *unit == series.unit)
                .unwrap_or_else(|| {
                    units.push(&series.unit);
                    units.len() - 1
                })
        })
        .collect::<Vec<_>>();
    let (x_minimum, x_maximum) = available_range(
        projected
            .iter()
            .flat_map(|points| points.iter().map(|(_, point)| point.0)),
    )?;
    // The window the reader had pinned is part of the sheet they captured,
    // exactly as the hidden traces and the placed cursors are. A pinned
    // ordinate names one window, so it holds only on a page of one strip.
    let (x_minimum, x_maximum) = overlay.map_or((x_minimum, x_maximum), |overlay| {
        overlay.framed_x(x_scale, (x_minimum, x_maximum))
    });
    let (x_minimum, x_maximum) = nondegenerate_range(x_minimum, x_maximum);
    let plot_width = PLOT_WIDTH_UM - 2 * PLOT_INSET_UM;
    let plot_height = PLOT_HEIGHT_UM - 2 * PLOT_INSET_UM;
    let frames = (0..units.len())
        .map(|strip| {
            let data = available_range(
                projected
                    .iter()
                    .zip(&strip_of)
                    .filter(|(_, of)| **of == strip)
                    .flat_map(|(points, _)| points.iter().map(|(_, point)| point.1)),
            )?;
            let (y_minimum, y_maximum) = match overlay {
                Some(overlay) if units.len() == 1 => overlay.framed_y(y_scale, data),
                _ => data,
            };
            let (y_minimum, y_maximum) = nondegenerate_range(y_minimum, y_maximum);
            Ok(PlotFrame {
                x_minimum,
                x_maximum,
                y_minimum,
                y_maximum,
                x_span: x_maximum - x_minimum,
                y_span: y_maximum - y_minimum,
                plot_width,
                plot_height,
            })
        })
        .collect::<Result<Vec<_>, HardcopySourceError>>()?;
    let rects = strip_rects(frames.len(), plot_height);
    // The page keeps what its columns can draw of each series, and the
    // samples it keeps are the ones it states it drew.
    let columns = columns::print_columns(plot_width);
    let projected = projected
        .into_iter()
        .map(|points| {
            let page = points.iter().map(|(_, point)| *point).collect::<Vec<_>>();
            columns::column_extremes(&page, x_minimum, x_maximum, columns)
                .into_iter()
                .map(|kept| points[kept])
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let (axis_ticks, captions) = strip_axes(&frames, &rects, &units, abscissa, y_scale)?;
    let (cursors, markers) = overlay.map_or_else(
        || Ok((Vec::new(), Vec::new())),
        |overlay| {
            resolved_overlay_geometry(
                viewer,
                overlay,
                &series,
                (x_scale, y_scale),
                &frames,
                &strip_of,
                |strip, point| rects[strip].place(point),
            )
        },
    )?;
    let mut trace_ids = std::collections::HashSet::new();
    let traces = series
        .iter()
        .zip(projected.iter())
        .zip(&strip_of)
        .enumerate()
        .map(|(index, ((series, points), &strip))| {
            let trace_id = stable_quick_trace_id(viewer, index, &series.identity);
            if !trace_ids.insert(trace_id) {
                return Err(HardcopySourceError::DuplicateStableTraceIdentity(trace_id));
            }
            let frame = &frames[strip];
            let paths = clipped_plot_paths(
                &points.iter().map(|(_, point)| *point).collect::<Vec<_>>(),
                frame.x_minimum,
                frame.x_maximum,
                frame.y_minimum,
                frame.y_maximum,
                plot_width,
                plot_height,
            )?
            .into_iter()
            .map(|path| {
                path.into_iter()
                    .map(|point| rects[strip].place(point))
                    .collect()
            })
            .collect();
            Ok(SemanticPlotTrace {
                trace_id,
                label: series.label.clone(),
                paths,
                source_samples: points
                    .iter()
                    .map(|&(index, _)| {
                        let (x, y) = series.points[index];
                        (x.to_bits(), y.to_bits())
                    })
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
        frames: rects
            .iter()
            .map(|rect| SemanticPlotFrame {
                minimum: SemanticPoint::new(PLOT_INSET_UM, rect.top),
                maximum: SemanticPoint::new(PLOT_INSET_UM + plot_width, rect.bottom()),
            })
            .collect(),
    })
}

/// The least and greatest of `values`, an unavailable value counting as
/// neither.
fn available_range(values: impl Iterator<Item = f64>) -> Result<(f64, f64), HardcopySourceError> {
    values
        .filter(|value| !value.is_nan())
        .fold(None, |range: Option<(f64, f64)>, value| {
            Some(range.map_or((value, value), |(low, high)| {
                (low.min(value), high.max(value))
            }))
        })
        .ok_or(HardcopySourceError::InvalidResultRange)
}

/// Every strip's rules and the words beside them: the shared abscissa ruled
/// down each strip and valued under the last, its quantity and unit in the
/// gutter of that row, and each strip's ordinate with its unit on its top
/// value, as the sheet states them.
fn strip_axes(
    frames: &[PlotFrame],
    rects: &[StripRect],
    units: &[&str],
    abscissa: &QuickAbscissa,
    y_scale: AxisScale,
) -> Result<(Vec<SemanticAxisTick>, Vec<SemanticPlotCaption>), HardcopySourceError> {
    let mut ticks = Vec::new();
    let mut captions = Vec::new();
    let last = frames.len() - 1;
    let (verticals, x_anchor) = ruled_axis(
        SemanticAxisKind::Horizontal,
        abscissa.scale,
        &frames[0],
        &abscissa.unit,
    )?;
    for (strip, rect) in rects.iter().enumerate() {
        ticks.extend(verticals.iter().map(|tick| SemanticAxisTick {
            start: rect.place(tick.start),
            end: rect.place(tick.end),
            label: if strip == last {
                tick.label.clone()
            } else {
                String::new()
            },
            ..tick.clone()
        }));
    }
    let foot = rects[last].bottom();
    captions.push(SemanticPlotCaption {
        text: abscissa.caption(),
        position: SemanticPoint::new(PLOT_INSET_UM - 1_000, foot + 2_600),
        anchor: SemanticCaptionAnchor::End,
    });
    if let Some(anchor) = x_anchor {
        captions.push(SemanticPlotCaption::new(
            format!("x: {anchor}"),
            SemanticPoint::new(PLOT_INSET_UM, PLOT_HEIGHT_UM - 1_500),
        ));
    }
    for ((frame, rect), unit) in frames.iter().zip(rects).zip(units) {
        let (mut horizontals, anchor) =
            ruled_axis(SemanticAxisKind::Vertical, y_scale, frame, unit)?;
        // The unit stands on the top value, where the sheet writes it.
        if let Some(top) = horizontals
            .iter_mut()
            .filter(|tick| !tick.label.is_empty())
            .min_by_key(|tick| tick.start.y_um)
        {
            top.label = rspice_results::number_format::tick_with_unit(&top.label, unit);
        }
        ticks.extend(horizontals.into_iter().map(|tick| SemanticAxisTick {
            start: rect.place(tick.start),
            end: rect.place(tick.end),
            ..tick
        }));
        if let Some(anchor) = anchor {
            captions.push(SemanticPlotCaption::new(
                format!("y: {anchor}"),
                SemanticPoint::new(PLOT_INSET_UM, rect.top - 1_200),
            ));
        }
    }
    Ok((ticks, captions))
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
