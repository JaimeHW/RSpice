//! The nominal sizes of a drawn sheet's strokes and marks, in world units,
//! and the tint a shaded symbol body takes.
//!
//! The printed page draws exactly these; the canvas scales them with the
//! zoom between floors and ceilings of its own. One table, so the page and
//! the screen carry the same weights.

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
/// The side of the hollow square that marks a pin nothing meets.
pub const OPEN_PIN_SIDE_WORLD: f32 = 3.0;
/// How much of its outline's ink a symbol shape's background fill takes
/// over the ground under it: the canvas's background on screen, the paper
/// on a page. A light tint, so every line and letter drawn over it reads.
pub const SYMBOL_BACKGROUND_TINT: f32 = 0.15;
