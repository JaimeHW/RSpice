//! Real event lanes, `E()`, as step traces in an ordinary value pane.
//!
//! A real net holds a value from one event instant to the next, so it is
//! drawn horizontal, then vertical at each instant, never sloped. It is a
//! trace of the pane it is placed in and reads on that pane's unit axis, so
//! an `E()` lane and the `V()` traces it is compared with share one axis.

use super::model::RealLane;
use crate::waves::pane::ResolvedExpr;
use rspice_ui_kit::plot::sample::SweepShape;
use std::sync::Arc;

/// An `E()` lane as a step trace through `end`, the strip's last instant.
pub fn real_step_trace(
    lane: &RealLane,
    end: f64,
    color: egui::Color32,
    cache_key: u64,
) -> ResolvedExpr {
    let (x, y) = lane.step_polyline(end);
    let y_extremes = y.iter().copied().filter(|value| value.is_finite()).fold(
        None,
        |extremes: Option<(f64, f64)>, value| {
            Some(extremes.map_or((value, value), |(low, high)| {
                (low.min(value), high.max(value))
            }))
        },
    );
    let shape = Arc::new(SweepShape::of(&x));
    ResolvedExpr {
        x: Arc::new(x),
        y: Arc::new(y),
        shape,
        color,
        cache_key,
        label: format!("E({})", lane.name()),
        y_extremes,
        family_style: None,
    }
}
