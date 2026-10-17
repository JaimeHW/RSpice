//! The nominal sizes of a drawn sheet's strokes and marks, in world units.
//!
//! A vector drawing (the SVG export, the printed page) draws exactly these;
//! the canvas scales them with the zoom between floors and ceilings of its
//! own. One table, so a page and an export carry the same weights.

/// A wire's stroke.
pub const WIRE_WIDTH_WORLD: f32 = 1.1;
/// One of a bus's three strands.
pub const BUS_STRAND_WIDTH_WORLD: f32 = 1.2;
/// A bus tap's stroke.
pub const BUS_TAP_WIDTH_WORLD: f32 = 2.0;
/// A symbol's body and pin leads.
pub const SYMBOL_WIDTH_WORLD: f32 = 1.0;
/// A junction dot's radius.
pub const JUNCTION_RADIUS_WORLD: f32 = 1.5;
