//! The strip-plot engine.
//!
//! Implements the design's plot grammar for every results surface: token
//! styled axes and grid, linear/log scales, dual Y axes, decimated traces,
//! reference lines, σ bands, bordered marker tags, a hover crosshair with
//! readout, and A/B measurement cursors.
//!
//! Design contract:
//! - grid lines in `canvas_grid`, frame in `border_strong`, ticks in
//!   `text_dim` 10 px mono, axis units in `text_dim` mono at the axis ends
//! - traces are 1.8 pt (1.4 pt thin), dashed for secondary quantities
//! - numbers never float over traces — they live in marker tags, the hover
//!   readout, or the right panel
//! - cursor A is `accent`, cursor B is `trace-5`, letter flags at the top
//!
//! Performance: traces are decimated to a per-pixel-column min/max envelope
//! (at most two points per column) and the envelope is cached keyed by data
//! identity, range and width — see [`decimate`]. A 1 M-point transient costs
//! one O(n) pass when first shown at a given zoom, then ~2·width points per
//! frame thereafter.

mod cursor;
mod decimate;
mod format;
mod render;
mod scale;
mod spec;

pub use cursor::CursorPair;
pub use decimate::{DecimationCache, sample_at};
pub use format::fmt_si;
pub use render::{PlotResponse, ViewChange, plot_rect, show, square_outer_rect};
pub use scale::XScale;
pub use spec::{Axis, Band, Marker, PlotMapper, PlotSpec, RefLine, Trace, YSide};
