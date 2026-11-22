//! The values a cursor or marker readout reads, apart from any drawing:
//! the separation between two X positions with its reciprocal, a slope in
//! the reader's number format, and where a search marker stands on its
//! trace as the trace is now.

use super::{StripModel, anchor_key};
use crate::session::{MarkerView, ResultViewerState};
use rspice_results::feature_search;
use rspice_results::result_presentation::{CrossingLevel, MarkerKind, WaveformPresentationKey};
use rspice_ui_kit::plot::SampleInterpolation;
use rspice_ui_kit::plot::{XScale, fmt_si_significant};
use rspice_vocabulary::quantity::{FrequencyDisplay, QuantityPresentationPolicy};

#[cfg(test)]
mod tests;

/// Where a marker sits on its trace as the trace is now.
///
/// A peak, valley or crossing marker is found again from where it was put
/// whenever the strip is built again, so a result that changes under it moves
/// it with its feature; `Err` carries where it was put when its trace no
/// longer has that feature there. Any other marker stays where it was put.
/// The search is made once per placement and kept on the strip model.
pub fn marker_position(
    model: &StripModel,
    anchor: &WaveformPresentationKey,
    x: f64,
    kind: MarkerKind,
) -> Result<f64, f64> {
    let Some(search) = kind.search() else {
        return Ok(x);
    };
    let mut positions = model
        .marker_positions
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    if let Some((.., found)) = positions
        .iter()
        .find(|(key, bits, held, _)| key == anchor && *bits == x.to_bits() && *held == kind)
    {
        return *found;
    }
    let found = model
        .traces
        .iter()
        .find(|trace| !trace.overlay && anchor_key(model, trace) == *anchor)
        .and_then(|trace| {
            // A stored trace is searched a block at a time, never held whole.
            let xs = feature_search::WindowedCurve::new(&trace.x);
            let ys = feature_search::WindowedCurve::new(&trace.y);
            match search {
                // On a log X axis the line runs straight in log X between
                // samples, so a crossing is found on that line, where the
                // readout reads its level, and not on a line straight in X.
                feature_search::Anchor::Crossing { .. } if model.x_scale == XScale::Log10 => {
                    feature_search::nearest(&LogX(xs), &ys, x.log10(), search)
                        .map(|found| 10f64.powf(found.x))
                }
                _ => feature_search::nearest(&xs, &ys, x, search).map(|found| found.x),
            }
        })
        .ok_or(x);
    positions.push((anchor.clone(), x.to_bits(), kind, found));
    found
}

/// A log X axis' samples as their logarithms, which a log axis spaces evenly.
struct LogX<'a>(feature_search::WindowedCurve<'a>);

impl feature_search::Curve for LogX<'_> {
    fn len(&self) -> usize {
        self.0.len()
    }

    fn at(&self, index: usize) -> f64 {
        self.0.at(index).log10()
    }

    fn ascending(&self) -> bool {
        self.0.ascending()
    }
}

/// Where `marker` stands on its strip as the strip is now: [`marker_position`]
/// for a marker of its own, and for an offset marker the crossing of its
/// marker's Y plus its amount, on the trace it rides, nearest where it was
/// put. Its marker is found first wherever that one now stands, so 3 dB
/// below a peak follows the peak. `Err` carries where it was put when its
/// marker is gone, is itself an offset, stands on no trace, or its level is
/// never crossed.
pub fn marker_x(
    model: &StripModel,
    session: &ResultViewerState,
    marker: MarkerView<'_>,
) -> Result<f64, f64> {
    let MarkerKind::Offset(offset) = marker.kind() else {
        return marker_position(model, marker.anchor(), marker.x(), marker.kind());
    };
    let Some(from) = session.strip_markers(marker.analysis()).find(|other| {
        other.0.retained_id.get() == offset.from && other.0.document_id == marker.0.document_id
    }) else {
        return Err(marker.x());
    };
    offset_position(
        model,
        (from.anchor(), from.x(), from.kind()),
        (marker.anchor(), marker.x()),
        offset.by.0,
    )
}

/// Where an offset marker put at `x` on `anchor` stands: the crossing of
/// its marker's Y plus `by`, nearest `x`, its marker (the trace it rides,
/// where it was put and its kind) found first wherever it now stands. `Err`
/// carries `x` when that marker is itself an offset, stands on no trace, or
/// the level is never crossed.
pub fn offset_position(
    model: &StripModel,
    (from_anchor, from_x, from_kind): (&WaveformPresentationKey, f64, MarkerKind),
    (anchor, x): (&WaveformPresentationKey, f64),
    by: f64,
) -> Result<f64, f64> {
    if matches!(from_kind, MarkerKind::Offset(_)) {
        return Err(x);
    }
    let Some(trace) = model
        .traces
        .iter()
        .find(|trace| !trace.overlay && anchor_key(model, trace) == *from_anchor)
    else {
        return Err(x);
    };
    let stands = marker_position(model, from_anchor, from_x, from_kind).map_err(|_| x)?;
    // The level is read off the drawn line, as the marker's own Y is.
    let level = model.trace_value(trace, stands, SampleInterpolation::Linear) + by;
    if !level.is_finite() {
        return Err(x);
    }
    marker_position(model, anchor, x, MarkerKind::Crossing(CrossingLevel(level)))
}

/// The separation between the cursors, named the way the X axis reads: a
/// time span also reports its reciprocal, because 1/Δt is the number a
/// designer is actually after when measuring a period.
pub fn x_separation(
    model: &StripModel,
    a: f64,
    b: f64,
    significant_digits: usize,
    quantity_policy: QuantityPresentationPolicy,
) -> String {
    // A separation is read, not tabulated: "1 ms  (1 kHz)", never
    // "1.000000 ms  (1.000000 kHz)" — two padded numbers in one cell.
    let dx = b - a;
    match model.x_scale {
        XScale::Linear if model.x_unit == "s" => {
            let span = fmt_si_significant(dx, "s", significant_digits);
            if dx == 0.0 {
                span
            } else {
                format!(
                    "{span}  ({})",
                    frequency_text(1.0 / dx.abs(), significant_digits, quantity_policy)
                )
            }
        }
        XScale::Log10 => frequency_text(dx, significant_digits, quantity_policy),
        _ => fmt_si_significant(dx, &model.x_unit, significant_digits),
    }
}

/// The separation between the cursors alone, as the table's Δ column reads
/// it; 1/Δx has its own cell beside it.
pub fn x_delta(
    model: &StripModel,
    a: f64,
    b: f64,
    significant_digits: usize,
    quantity_policy: QuantityPresentationPolicy,
) -> String {
    let dx = b - a;
    match model.x_scale {
        XScale::Log10 => frequency_text(dx, significant_digits, quantity_policy),
        XScale::Linear => fmt_si_significant(dx, &model.x_unit, significant_digits),
    }
}

/// 1/Δx, the number a period or a spacing is measured for: a frequency for a
/// time span (in the reader's frequency unit), a time for a frequency span,
/// otherwise the reciprocal of the X unit. `None` for cursors on one point.
pub fn x_reciprocal(
    model: &StripModel,
    a: f64,
    b: f64,
    significant_digits: usize,
    quantity_policy: QuantityPresentationPolicy,
) -> Option<String> {
    let span = (b - a).abs();
    if !(span > 0.0 && span.is_finite()) {
        return None;
    }
    let inverse = 1.0 / span;
    Some(match model.x_unit.as_str() {
        "s" => frequency_text(inverse, significant_digits, quantity_policy),
        "Hz" => fmt_si_significant(inverse, "s", significant_digits),
        unit => fmt_si_significant(inverse, &format!("/{unit}"), significant_digits),
    })
}

/// A frequency at the reader's significant digits, in the reader's frequency
/// unit. `format_frequency` counts decimals after the prefix, so a 1/Δt off a
/// span that is not round read `999.8615044 Hz` beside `1.000139 ms`.
fn frequency_text(
    hertz: f64,
    significant_digits: usize,
    quantity_policy: QuantityPresentationPolicy,
) -> String {
    match quantity_policy.frequency_display {
        FrequencyDisplay::HertzEngineering => fmt_si_significant(hertz, "Hz", significant_digits),
        FrequencyDisplay::RadiansPerSecond => {
            fmt_si_significant(hertz * std::f64::consts::TAU, "rad/s", significant_digits)
        }
    }
}

/// A slope in the reader's number format: an SI prefix on a unit that takes
/// one (`2 MV/s`, `15 mA/V`), plain digits on one that carries its own scale
/// (`-20 dB/dec`, `45 °/kHz`).
pub fn slope_text(slope: f64, y_unit: &str, x_unit: &str, significant_digits: usize) -> String {
    fmt_si_significant(slope, &format!("{y_unit}/{x_unit}"), significant_digits)
}
