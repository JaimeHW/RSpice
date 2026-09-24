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

/// Resolve both axes in source space before mapping ticks to the page.
/// Offset labels retain their anchor as a caption, as on the interactive plot.
pub(super) fn plot_axes(
    x_scale: AxisScale,
    y_scale: AxisScale,
    frame: &PlotFrame,
) -> Result<(Vec<SemanticAxisTick>, Vec<SemanticPlotCaption>), HardcopySourceError> {
    let mut ticks = Vec::new();
    let mut captions = Vec::new();
    for (kind, scale, min, max) in [
        (
            SemanticAxisKind::Horizontal,
            x_scale,
            frame.x_minimum,
            frame.x_maximum,
        ),
        (
            SemanticAxisKind::Vertical,
            y_scale,
            frame.y_minimum,
            frame.y_maximum,
        ),
    ] {
        if !min.is_finite() || !max.is_finite() || min >= max || !(max - min).is_finite() {
            return Err(HardcopySourceError::InvalidResultRange);
        }
        let rules = if scale == AxisScale::Logarithmic {
            logarithmic_rules(min, max)
        } else {
            let axis = crate::ui::plot::Axis::linear(min, max, "");
            if let Some(anchor) = axis.offset_anchor() {
                let (label, y) = match kind {
                    SemanticAxisKind::Horizontal => ("x", PLOT_HEIGHT_UM - 1_500),
                    SemanticAxisKind::Vertical => ("y", 9_000),
                };
                captions.push(SemanticPlotCaption {
                    text: format!("{label}: {anchor}"),
                    position: SemanticPoint::new(PLOT_INSET_UM, y),
                });
            }
            axis.ticks
                .into_iter()
                .map(|(value, label)| (value, label, true))
                .collect()
        };
        for (value, label, major) in rules {
            if value < min || value > max {
                continue;
            }
            let (start, end) = match kind {
                SemanticAxisKind::Horizontal => {
                    ((value, frame.y_minimum), (value, frame.y_maximum))
                }
                SemanticAxisKind::Vertical => ((frame.x_minimum, value), (frame.x_maximum, value)),
            };
            let point = |(x, y)| {
                map_plot_point(
                    x,
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
                axis: kind,
                start: point(start)?,
                end: point(end)?,
                label,
                major,
            });
        }
    }
    Ok((ticks, captions))
}

fn logarithmic_rules(min: f64, max: f64) -> Vec<(f64, String, bool)> {
    let first = min.ceil() as i32;
    let last = max.floor() as i32;
    let mut rules = Vec::new();
    for decade in first..=last {
        let value = 10.0_f64.powi(decade);
        if value.is_finite() && value > 0.0 {
            rules.push((f64::from(decade), fmt_si_significant(value, "", 3), true));
        }
    }
    if max - min > MINOR_DECADE_LIMIT {
        return rules;
    }
    let promote = rules.is_empty();
    for decade in min.floor() as i32..=max.floor() as i32 {
        for mantissa in 2..10 {
            let position = f64::from(decade) + f64::from(mantissa).log10();
            if position <= min || position >= max {
                continue;
            }
            let value = 10.0_f64.powf(position);
            if !value.is_finite() || value <= 0.0 {
                continue;
            }
            let label = if promote {
                fmt_si_significant(value, "", 3)
            } else {
                String::new()
            };
            rules.push((position, label, promote));
        }
    }
    rules
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn linear_axes_are_captioned_on_both_dimensions_and_keep_offset_anchors() {
        let frame = log_frame(1e9, 1e9 + 0.001);
        let (ticks, captions) = plot_axes(AxisScale::Linear, AxisScale::Linear, &frame).unwrap();
        let screen = crate::ui::plot::Axis::linear(frame.x_minimum, frame.x_maximum, "");
        let anchor = screen
            .offset_anchor()
            .expect("this span requires an offset anchor");
        assert!(
            captions
                .iter()
                .any(|caption| caption.text == format!("x: {anchor}"))
        );
        for kind in [SemanticAxisKind::Horizontal, SemanticAxisKind::Vertical] {
            let axis: Vec<_> = ticks.iter().filter(|tick| tick.axis == kind).collect();
            assert!(!axis.is_empty());
            assert!(axis.iter().all(|tick| tick.major && !tick.label.is_empty()));
            for tick in axis {
                match kind {
                    SemanticAxisKind::Horizontal => assert_eq!(tick.start.x_um, tick.end.x_um),
                    SemanticAxisKind::Vertical => assert_eq!(tick.start.y_um, tick.end.y_um),
                }
            }
        }
    }

    #[test]
    fn logarithmic_ordinates_are_mapped_as_decades() {
        let mut frame = log_frame(0.0, 2.0);
        frame.y_maximum = 2.0;
        frame.y_span = 2.0;
        let (ticks, _) = plot_axes(AxisScale::Linear, AxisScale::Logarithmic, &frame).unwrap();
        let major: Vec<_> = ticks
            .iter()
            .filter(|tick| tick.axis == SemanticAxisKind::Vertical && tick.major)
            .collect();
        assert_eq!(
            major
                .iter()
                .map(|tick| tick.label.as_str())
                .collect::<Vec<_>>(),
            vec!["1.00", "10.0", "100"]
        );
        let first = major[0].start.y_um - major[1].start.y_um;
        let second = major[1].start.y_um - major[2].start.y_um;
        assert!(
            first.abs_diff(second) <= 1,
            "equal decades may differ only by micrometre rounding"
        );
    }

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
        plot_axes(
            AxisScale::Logarithmic,
            AxisScale::Linear,
            &log_frame(x_minimum, x_maximum),
        )
        .expect("the frame is inside the page")
        .0
        .into_iter()
        .filter(|tick| tick.axis == SemanticAxisKind::Horizontal)
        .collect()
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
