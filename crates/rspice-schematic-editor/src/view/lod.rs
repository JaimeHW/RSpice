//! Level of detail for the schematic canvas.
//!
//! One place owns how wide a stroke is, how large a connectivity mark is and
//! whether a run of text is drawn, greeked or hidden at a given zoom. Every
//! canvas painter asks here; none multiplies a width or a font size by the
//! zoom on its own.
//!
//! Strokes scale with the drawing between a floor and a ceiling, and every
//! cutoff is compared in device pixels, so a 2x display draws the same
//! drawing as a 1x one with twice the pixels rather than a thinner one.
//! While a stroke is thin it is rounded to whole device pixels, which keeps a
//! one-pixel wire crisp. Marks that carry connectivity (junction dots, net
//! label diamonds, open-pin squares) have floors of their own, so each one
//! survives a fit-to-sheet zoom.
//!
//! Text is part of the drawing: it scales with no ceiling, and below one
//! legibility floor an electrical name becomes a bar where the text would be
//! rather than disappearing with its anchor.

use rspice_design::schematic::{bus_route, drawing_metrics};

/// What a stroke draws, which fixes its nominal world width and its limits.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StrokeRole {
    Wire,
    /// One of a bus's three parallel strands.
    BusStrand,
    /// A bus drawn as one stroke, once its strands would merge.
    BusCollapsed,
    BusTap,
    /// Symbol body and pin leads, canonical and cell.
    Symbol,
    /// A vector port's body, at bus weight.
    VectorPort,
    Documentation,
    /// Probe ring and crosshair.
    Probe,
}

/// Whether the object is drawn plain, raised by selection or a highlight, or
/// at a plotted net's weight.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Emphasis {
    Plain,
    Raised,
    /// The net is on the plot. Its weight, not only its trace colour, sets
    /// it apart from the wires: a hue alone can sit near the wire ink.
    Plotted,
    /// The plotted net whose trace the reader is on: plotted, then raised.
    PlottedLit,
}

impl Emphasis {
    pub const fn raised_if(raised: bool) -> Self {
        if raised { Self::Raised } else { Self::Plain }
    }
}

struct StrokeLimits {
    nominal_wu: f32,
    floor_pt: f32,
    ceiling_pt: f32,
}

/// What a raised stroke adds, in points.
const RAISED_ADD_PT: f32 = 1.0;
/// A plotted net draws at this multiple of its plain weight.
const PLOTTED_SCALE: f32 = 2.0;
/// Below this a stroke is rounded to whole device pixels.
const CRISP_BELOW_DPX: f32 = 6.0;

const fn limits(role: StrokeRole) -> StrokeLimits {
    let (nominal_wu, floor_pt, ceiling_pt) = match role {
        StrokeRole::Wire => (drawing_metrics::WIRE_WIDTH_WORLD, 1.0, 3.0),
        StrokeRole::BusStrand => (drawing_metrics::BUS_STRAND_WIDTH_WORLD, 1.0, 2.5),
        StrokeRole::BusCollapsed => (0.0, 3.0, 3.0),
        StrokeRole::BusTap => (drawing_metrics::BUS_TAP_WIDTH_WORLD, 2.0, 4.0),
        StrokeRole::Symbol => (drawing_metrics::SYMBOL_WIDTH_WORLD, 1.0, 2.5),
        StrokeRole::VectorPort => (1.2, 1.0, 2.5),
        StrokeRole::Documentation => (0.8, 1.0, 2.0),
        StrokeRole::Probe => (2.0, 1.0, 3.0),
    };
    StrokeLimits {
        nominal_wu,
        floor_pt,
        ceiling_pt,
    }
}

/// A stroke's nominal width in world units: what it draws at with no floor
/// or ceiling, which is exactly what a vector export draws.
pub const fn nominal_wu(role: StrokeRole) -> f32 {
    limits(role).nominal_wu
}

fn ppp_or_one(ppp: f32) -> f32 {
    if ppp.is_finite() && ppp > 0.0 {
        ppp
    } else {
        1.0
    }
}

/// The floor of a stroke in whole device pixels: never under one.
fn floor_dpx(floor_pt: f32, ppp: f32) -> f32 {
    (floor_pt * ppp).round().max(1.0)
}

/// A stroke's width in points at `zoom` (points per world unit) on a display
/// of `ppp` device pixels per point.
pub fn stroke_pt(role: StrokeRole, zoom: f32, ppp: f32, emphasis: Emphasis) -> f32 {
    let ppp = ppp_or_one(ppp);
    let limits = limits(role);
    let floor = floor_dpx(limits.floor_pt, ppp);
    let ceiling = (limits.ceiling_pt * ppp).max(floor);
    let plain = (limits.nominal_wu * zoom.max(0.0) * ppp).clamp(floor, ceiling);
    let raise = RAISED_ADD_PT * ppp;
    let mut width = match emphasis {
        Emphasis::Plain => plain,
        Emphasis::Raised => plain + raise,
        Emphasis::Plotted => plain * PLOTTED_SCALE,
        Emphasis::PlottedLit => plain * PLOTTED_SCALE + raise,
    };
    if width < CRISP_BELOW_DPX {
        width = width.round().max(floor);
    }
    width / ppp
}

/// Centre-to-centre spacing of a bus's strands, in world units.
pub const BUS_STRAND_OFFSET_WU: f32 = bus_route::BUS_STRAND_OFFSET_WORLD;

/// Whether a bus draws as one stroke: its strands would leave less than one
/// device pixel of clear canvas between them.
pub fn bus_collapses(zoom: f32, ppp: f32) -> bool {
    let ppp = ppp_or_one(ppp);
    let strand_dpx = stroke_pt(StrokeRole::BusStrand, zoom, ppp, Emphasis::Plain) * ppp;
    BUS_STRAND_OFFSET_WU * zoom * ppp - strand_dpx < ppp
}

fn mark_pt(size_wu: f32, floor_pt: f32, ceiling_pt: f32, zoom: f32) -> f32 {
    (size_wu * zoom.max(0.0)).clamp(floor_pt, ceiling_pt)
}

/// A junction dot's nominal radius, in world units.
pub const JUNCTION_RADIUS_WU: f32 = drawing_metrics::JUNCTION_RADIUS_WORLD;
/// Half the diagonal of a net label's attachment diamond, in world units.
pub const DIAMOND_HALF_WU: f32 =
    rspice_design::schematic::label_layout::NET_LABEL_DIAMOND_HALF_WORLD;

/// A junction dot's radius in points. Never less than 1.25 times the drawn
/// width of the thickest conductor it sits on, so its diameter is always at
/// least two and a half wires: a junction reads apart from a crossing at
/// every zoom.
pub fn junction_radius_pt(zoom: f32, incident_width_pt: f32) -> f32 {
    mark_pt(JUNCTION_RADIUS_WU, 2.5, 5.0, zoom).max(1.25 * incident_width_pt.max(0.0))
}

/// Half the diagonal of a net label's attachment diamond, in points.
pub fn diamond_half_pt(zoom: f32) -> f32 {
    mark_pt(DIAMOND_HALF_WU, 3.0, 5.0, zoom)
}

/// The side of an open pin's square mark, in points.
pub fn open_pin_side_pt(zoom: f32) -> f32 {
    mark_pt(drawing_metrics::OPEN_PIN_SIDE_WORLD, 4.0, 7.0, zoom)
}

/// The outline of a mark (an open-pin square, a greeked connector tab): one
/// point, rounded to whole device pixels.
pub fn mark_outline_pt(ppp: f32) -> f32 {
    let ppp = ppp_or_one(ppp);
    floor_dpx(1.0, ppp) / ppp
}

/// A hovered junction's ring, in points (D1 section 6).
const HOVER_RING_PT: f32 = 1.5;

/// The stroke of a hovered junction's ring in points: D1's 1.5 pt rounded to
/// whole device pixels, so the ring is as crisp as the wires it sits on at
/// every pixel density (2 px at 1x, 3 at 2x) rather than a soft blur.
pub fn hover_ring_pt(ppp: f32) -> f32 {
    let ppp = ppp_or_one(ppp);
    floor_dpx(HOVER_RING_PT, ppp) / ppp
}

/// Quantize a zoom-scaled font size to quarter points: consecutive zoom
/// frames then reuse egui's cached galleys instead of re-shaping every
/// label on screen each frame (the dominant zoom-gesture cost).
pub fn quantize_font_size(size: f32) -> f32 {
    ((size * 4.0).round() * 0.25).max(1.0)
}

/// The smallest legible type, in device pixels.
const LEGIBLE_DPX: f32 = 6.0;
/// The smallest legible type, in points: a 2x display resolves 3-point type,
/// but nobody reads it.
const LEGIBLE_PT: f32 = 3.5;

/// Whether text of `size_pt` points is legible on a display of `ppp`.
pub fn legible(size_pt: f32, ppp: f32) -> bool {
    size_pt * ppp_or_one(ppp) >= LEGIBLE_DPX && size_pt >= LEGIBLE_PT
}

/// What a run of text is below the legibility floor.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextClass {
    /// A name that carries connectivity (net label, off-sheet connector,
    /// port, bus declaration, tap slice): it greeks, so the reader still sees
    /// that a name is there.
    Electrical,
    /// Everything else (instance names and values, pin names, symbol text,
    /// probe labels): it hides.
    Annotation,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TextLod {
    /// Draw the text at this many points.
    Draw(f32),
    /// Draw a bar where the text would be.
    Greek(GreekBar),
    Hide,
}

/// A greeked name: a bar where the text box would be.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GreekBar {
    /// The size the text would have drawn at, in points.
    pub size_pt: f32,
    /// The bar's height, in points.
    pub height_pt: f32,
}

/// The alpha of a greeked bar, applied to the text colour.
pub const GREEK_ALPHA: f32 = 0.55;

/// How text of `world_size` world units draws at `zoom` on a display of
/// `ppp`.
pub fn text_lod(world_size: f32, zoom: f32, ppp: f32, class: TextClass) -> TextLod {
    let ppp = ppp_or_one(ppp);
    let size_pt = quantize_font_size(world_size * zoom.max(0.0));
    if legible(size_pt, ppp) {
        return TextLod::Draw(size_pt);
    }
    match class {
        TextClass::Annotation => TextLod::Hide,
        TextClass::Electrical => TextLod::Greek(GreekBar {
            size_pt,
            height_pt: (0.4 * size_pt * ppp).round().max(1.0) / ppp,
        }),
    }
}

/// The drawn size of text that hides below the floor, or `None` when it
/// hides.
pub fn annotation_size(world_size: f32, zoom: f32, ppp: f32) -> Option<f32> {
    match text_lod(world_size, zoom, ppp, TextClass::Annotation) {
        TextLod::Draw(size) => Some(size),
        TextLod::Greek(_) | TextLod::Hide => None,
    }
}

/// The width of a greeked bar in points: `chars` glyphs of `glyph_wu` world
/// units each, at least three device pixels.
pub fn greek_bar_width_pt(chars: usize, glyph_wu: f32, zoom: f32, ppp: f32) -> f32 {
    let ppp = ppp_or_one(ppp);
    (chars as f32 * glyph_wu * zoom.max(0.0)).max(3.0 / ppp)
}

/// The width of a greeked connector tab in points: at least four device
/// pixels.
pub fn greek_tab_width_pt(tab_wu: f32, zoom: f32, ppp: f32) -> f32 {
    let ppp = ppp_or_one(ppp);
    (tab_wu * zoom.max(0.0)).max(4.0 / ppp)
}
