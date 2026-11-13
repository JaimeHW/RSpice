//! The values a cursor or marker readout reads, apart from any drawing:
//! the separation between two X positions with its reciprocal, a slope in
//! the reader's number format, and where a search marker stands on its
//! trace as the trace is now.

use super::units::as_readout;
use super::{StripModel, anchor_key};
use rspice_results::feature_search;
use rspice_results::result_presentation::{MarkerKind, WaveformPresentationKey};
use rspice_ui_kit::plot::{XScale, fmt_si_significant, fmt_significant};
use rspice_vocabulary::quantity::{FrequencyDisplay, QuantityPresentationPolicy};

#[cfg(test)]
mod tests;

/// Where a marker sits on its trace as the trace is now.
///
/// A peak, valley or crossing marker is found again from where it was put
/// every time it is read, so a result that changes under it moves it with
/// its feature; `Err` carries where it was put when its trace no longer has
/// that feature there. Any other marker stays where it was put.
pub fn marker_position(
    model: &StripModel,
    anchor: &WaveformPresentationKey,
    x: f64,
    kind: MarkerKind,
) -> Result<f64, f64> {
    let Some(search) = kind.search() else {
        return Ok(x);
    };
    model
        .traces
        .iter()
        .find(|trace| !trace.overlay && anchor_key(model, trace) == *anchor)
        .and_then(|trace| feature_search::nearest(&trace.x, &trace.y, x, search))
        .map(|found| found.x)
        .ok_or(x)
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
            let span = as_readout(&fmt_si_significant(dx, "s", significant_digits));
            if dx == 0.0 {
                span
            } else {
                format!(
                    "{span}  ({})",
                    as_readout(&frequency_text(
                        1.0 / dx.abs(),
                        significant_digits,
                        quantity_policy
                    ))
                )
            }
        }
        XScale::Log10 => as_readout(&frequency_text(dx, significant_digits, quantity_policy)),
        _ => as_readout(&fmt_si_significant(dx, &model.x_unit, significant_digits)),
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
        XScale::Log10 => as_readout(&frequency_text(dx, significant_digits, quantity_policy)),
        XScale::Linear => as_readout(&fmt_si_significant(dx, &model.x_unit, significant_digits)),
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
    Some(as_readout(&match model.x_unit.as_str() {
        "s" => frequency_text(inverse, significant_digits, quantity_policy),
        "Hz" => fmt_si_significant(inverse, "s", significant_digits),
        "" => fmt_significant(inverse, significant_digits, ""),
        unit => fmt_significant(inverse, significant_digits, &format!(" /{unit}")),
    }))
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
    let compound = format!("{y_unit}/{x_unit}");
    let prefixed = !y_unit.is_empty()
        && !y_unit.starts_with("dB")
        && !matches!(y_unit, "°" | "deg" | "rad")
        && !super::units::unit_carries_its_decade(y_unit);
    if prefixed {
        as_readout(&fmt_si_significant(slope, &compound, significant_digits))
    } else {
        as_readout(&fmt_significant(
            slope,
            significant_digits,
            &format!(" {compound}"),
        ))
    }
}
