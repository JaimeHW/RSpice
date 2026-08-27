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

/// The largest number of decades a printed axis rules individually.
///
/// Beyond this the minor lines stop being a grid and become a wash, so only
/// the decade lines themselves are ruled.
const MINOR_DECADE_LIMIT: usize = 6;

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
/// its decades, and the nine minor lines inside each while there are few
/// enough decades for them to read as a grid rather than as ink.
pub(super) fn plot_axis_ticks(
    x_scale: AxisScale,
    frame: &PlotFrame,
) -> Result<Vec<SemanticAxisTick>, HardcopySourceError> {
    if x_scale != AxisScale::Logarithmic {
        return Ok(Vec::new());
    }
    // The frame's bounds are already in log space, so a decade is an integer.
    let first = frame.x_minimum.ceil() as i64;
    let last = frame.x_maximum.floor() as i64;
    if last < first {
        return Ok(Vec::new());
    }
    let decades = usize::try_from(last - first + 1).unwrap_or(usize::MAX);
    let rule_minors = decades <= MINOR_DECADE_LIMIT;
    let mut ticks = Vec::new();
    for decade in first..=last {
        let exponent = decade as f64;
        push_vertical_rule(
            &mut ticks,
            frame,
            exponent,
            fmt_si_significant(10.0_f64.powi(decade as i32), "", 3),
            true,
        )?;
        if !rule_minors {
            continue;
        }
        for step in 2..10 {
            let position = exponent + f64::from(step).log10();
            if position > frame.x_maximum {
                break;
            }
            push_vertical_rule(&mut ticks, frame, position, String::new(), false)?;
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
