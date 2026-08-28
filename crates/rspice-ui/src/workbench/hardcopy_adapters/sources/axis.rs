//! How a printed plot's axes map their quantities, and what they rule.
//!
//! The printed page used one mapping for every sheet: value minus minimum,
//! over the span, across the frame. That is the right mapping for a time
//! sweep and the wrong one for a frequency response, where the reader is
//! looking at decades — so a 1 Hz to 1 MHz Bode plot printed with its first
//! five decades squeezed against the left margin and nine evenly spaced
//! division lines ruled across a span that has no even divisions.
//!
//! Geometry is mapped here, in source space, before anything becomes a page
//! coordinate. Exact retained samples are untouched: they travel beside the
//! geometry as IEEE-754 bit patterns, in the units the engine produced.

use super::*;

use crate::results::visualization_document::AxisScale;
use crate::ui::plot::fmt_si_significant;

/// The widest span, in decades, over which a printed axis rules its mantissas.
///
/// Beyond this the minor lines stop being a grid and become a wash, so only
/// the decade lines themselves are ruled. It is the span the axis covers, not
/// a count of whole decades that happen to fall inside it — the screen rules
/// by span (`ui::plot::scale::minor_grid_values`) and the page has to agree,
/// or the same sweep is a grid on one and a wash on the other.
const MINOR_DECADE_LIMIT: f64 = 6.0;

/// Project a source value into the space the page is laid out in.
///
/// Only the logarithmic axis moves. Decibels and degrees are already the
/// quantity the sheet plots, and mapping them again would square a scale that
/// has been applied once.
pub(super) fn project(scale: AxisScale, value: f64) -> Option<f64> {
    match scale {
        AxisScale::Logarithmic => (value > 0.0 && value.is_finite()).then(|| value.log10()),
        AxisScale::Linear | AxisScale::Decibels | AxisScale::PhaseDegrees => {
            value.is_finite().then_some(value)
        }
    }
}

/// The gridlines one plot rules, in page coordinates.
///
/// A linear axis keeps the frame's own even divisions, which is what it has
/// always drawn and what a linear sweep deserves. A logarithmic axis rules
/// every decade boundary it contains, and the eight interior mantissas — 2
/// through 9 — of every decade it touches, while the span is narrow enough
/// for them to read as a grid rather than as ink.
///
/// The mantissas are ruled per decade *touched*, not per decade *contained*.
/// A 2 kHz to 8 kHz sweep contains no whole decade at all, and ruling only
/// contained ones left it with no logarithmic ruling whatever — the page fell
/// back on the frame's even divisions, which is ten equal slices of a span
/// that has no equal slices.
///
/// A window with no decade boundary inside it has nothing else to caption, so
/// there the mantissas are the majors and carry the labels. That is what the
/// sheet does: `ui::plot::scale::decade_ticks` degrades its ladder to
/// captioned mantissas inside a decade rather than leaving a log axis with no
/// numbers on it, and `plot_grid` prints no caption for an empty label — so a
/// page that ruled the mantissas as minors ruled a frequency axis with no
/// frequencies stated anywhere on it.
pub(super) fn plot_axis_ticks(
    x_scale: AxisScale,
    frame: &PlotFrame,
) -> Result<Vec<SemanticAxisTick>, HardcopySourceError> {
    if x_scale != AxisScale::Logarithmic {
        return Ok(Vec::new());
    }
    if !frame.x_minimum.is_finite() || !frame.x_maximum.is_finite() {
        return Ok(Vec::new());
    }
    // The frame's bounds are already in log space, so a decade is an integer.
    let first = frame.x_minimum.ceil() as i64;
    let last = frame.x_maximum.floor() as i64;
    let mut ticks = Vec::new();
    for decade in first..=last {
        push_vertical_rule(
            &mut ticks,
            frame,
            decade as f64,
            fmt_si_significant(10.0_f64.powi(decade as i32), "", 3),
            true,
        )?;
    }
    if frame.x_maximum - frame.x_minimum > MINOR_DECADE_LIMIT {
        return Ok(ticks);
    }
    // No decade boundary fell inside the window, so nothing above carries a
    // caption and the mantissas are all the axis has left to state itself
    // with. They are promoted rather than added to: the window is narrower
    // than a decade, so there is no coarser rule for them to subdivide.
    let mantissas_are_the_ladder = first > last;
    // Every decade the window touches, including the partial ones at its ends.
    let touched_first = frame.x_minimum.floor() as i64;
    let touched_last = frame.x_maximum.floor() as i64;
    for decade in touched_first..=touched_last {
        let exponent = decade as f64;
        for mantissa in 2..10 {
            let position = exponent + f64::from(mantissa).log10();
            // Strictly inside: a mantissa sitting on the frame's own edge is
            // the axis boundary, not a subdivision of it.
            if position <= frame.x_minimum || position >= frame.x_maximum {
                continue;
            }
            let label = if mantissas_are_the_ladder {
                fmt_si_significant(10.0_f64.powi(decade as i32) * f64::from(mantissa), "", 3)
            } else {
                String::new()
            };
            push_vertical_rule(&mut ticks, frame, position, label, mantissas_are_the_ladder)?;
        }
    }
    Ok(ticks)
}

fn push_vertical_rule(
    ticks: &mut Vec<SemanticAxisTick>,
    frame: &PlotFrame,
    position: f64,
    label: String,
    major: bool,
) -> Result<(), HardcopySourceError> {
    if position < frame.x_minimum || position > frame.x_maximum {
        return Ok(());
    }
    let point = |y| {
        map_plot_point(
            position,
            y,
            frame.x_minimum,
            frame.y_minimum,
            frame.x_span,
            frame.y_span,
            frame.plot_width,
            frame.plot_height,
        )
    };
    ticks.push(SemanticAxisTick {
        axis: SemanticAxisKind::Horizontal,
        start: point(frame.y_minimum)?,
        end: point(frame.y_maximum)?,
        label,
        major,
    });
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A frame over `[10^x_minimum, 10^x_maximum]`, already in log space.
    fn log_frame(x_minimum: f64, x_maximum: f64) -> PlotFrame {
        PlotFrame {
            x_minimum,
            x_maximum,
            y_minimum: 0.0,
            y_maximum: 1.0,
            x_span: x_maximum - x_minimum,
            y_span: 1.0,
            plot_width: PLOT_WIDTH_UM - 2 * PLOT_INSET_UM,
            plot_height: PLOT_HEIGHT_UM - 2 * PLOT_INSET_UM,
        }
    }

    fn ticks(x_minimum: f64, x_maximum: f64) -> Vec<SemanticAxisTick> {
        plot_axis_ticks(AxisScale::Logarithmic, &log_frame(x_minimum, x_maximum))
            .expect("the frame is inside the page")
    }

    /// The screen rules its minor lines by the span the axis covers; the page
    /// counted whole decades that happened to fall inside it. A sweep from
    /// 3.16 Hz to 7.9 MHz spans 6.4 decades and contains 6 whole ones, so the
    /// page washed it in minor lines the sheet had already stood down. A
    /// sweep from 1 Hz to 1 MHz spans exactly 6 and contains 7, so the page
    /// dropped minor lines the sheet was drawing.
    #[test]
    fn minor_lines_stand_down_by_the_span_the_screen_measures() {
        let wash = ticks(0.5, 6.9);
        assert!(
            wash.iter().all(|tick| tick.major),
            "6.4 decades is past the point where minor lines read as a grid"
        );

        let ruled = ticks(0.0, 6.0);
        assert!(
            ruled.iter().any(|tick| !tick.major),
            "a sweep of exactly six decades rules its minor lines on the sheet"
        );
    }

    /// A sub-decade sweep is ruled *and captioned* at its mantissas.
    ///
    /// 1.9 kHz to 8.5 kHz contains no whole decade at all, so the page
    /// produced no rules whatever and fell back on the frame's even divisions
    /// — ten equal slices of a span that has no equal slices.
    ///
    /// Ruling them was half of it. The screen is the oracle for what an axis
    /// says, and inside a decade `ui::plot::scale::decade_ticks` degrades its
    /// ladder to captioned mantissas — precisely because a log axis with no
    /// numbers on it is useless at the one zoom a reader measures at. The
    /// page shipped the same mantissas as uncaptioned minors, and
    /// `plot_grid` draws no caption for an empty label, so a window narrower
    /// than a decade printed with no frequency captions anywhere on it. The
    /// assertion below therefore reads the other way round from the one this
    /// test first froze.
    #[test]
    fn a_sub_decade_sweep_is_ruled_and_captioned_at_its_mantissas() {
        let ticks = ticks(1900.0_f64.log10(), 8500.0_f64.log10());
        assert!(
            !ticks.is_empty(),
            "a sub-decade frequency sweep printed with no logarithmic ruling at all"
        );
        // 2 through 8 kHz lie inside the window. There is no decade boundary
        // in it, so these are the only captions the axis can carry.
        assert_eq!(ticks.len(), 7);
        assert!(
            ticks.iter().all(|tick| tick.major),
            "the only rules on a sub-decade page were the ones that print no caption"
        );
        assert_eq!(
            ticks
                .iter()
                .map(|tick| tick.label.as_str())
                .collect::<Vec<_>>(),
            [
                "2.00 k", "3.00 k", "4.00 k", "5.00 k", "6.00 k", "7.00 k", "8.00 k"
            ]
        );
    }

    /// A decade rules eight interior mantissas — 2 through 9 — not nine.
    #[test]
    fn one_decade_rules_eight_interior_mantissas() {
        let ticks = ticks(1.0, 2.0);
        assert_eq!(ticks.iter().filter(|tick| tick.major).count(), 2);
        assert_eq!(ticks.iter().filter(|tick| !tick.major).count(), 8);
    }
}
