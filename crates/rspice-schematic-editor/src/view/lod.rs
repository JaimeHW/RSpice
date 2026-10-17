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

/// Whether the object is drawn plain or raised by selection or a highlight.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Emphasis {
    Plain,
    Raised,
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
    let add = match emphasis {
        Emphasis::Plain => 0.0,
        Emphasis::Raised => RAISED_ADD_PT * ppp,
    };
    let mut width = (limits.nominal_wu * zoom.max(0.0) * ppp).clamp(floor, ceiling) + add;
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
    mark_pt(3.0, 4.0, 7.0, zoom)
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

#[cfg(test)]
pub(crate) mod review;

#[cfg(test)]
mod tests {
    use super::*;

    fn dpx(pt: f32, ppp: f32) -> f32 {
        pt * ppp
    }

    /// The design's proof table: wire, selected wire and junction diameter in
    /// device pixels at each zoom and display density.
    #[test]
    fn junctions_read_apart_from_crossings_at_every_zoom_and_density() {
        // (zoom, ppp, wire, selected wire, dot diameter plain, dot diameter selected)
        const TABLE: [(f32, f32, f32, f32, f32, f32); 10] = [
            (0.25, 1.0, 1.0, 2.0, 5.0, 5.0),
            (0.32, 1.0, 1.0, 2.0, 5.0, 5.0),
            (1.0, 1.0, 1.0, 2.0, 5.0, 5.0),
            (2.0, 1.0, 2.0, 3.0, 6.0, 7.5),
            (4.0, 1.0, 3.0, 4.0, 10.0, 10.0),
            (8.0, 1.0, 3.0, 4.0, 10.0, 10.0),
            (0.25, 2.0, 2.0, 4.0, 10.0, 10.0),
            (1.0, 2.0, 2.0, 4.0, 10.0, 10.0),
            (4.0, 2.0, 6.0, 8.0, 20.0, 20.0),
            (8.0, 2.0, 6.0, 8.0, 20.0, 20.0),
        ];
        for (zoom, ppp, wire, selected, dot, dot_selected) in TABLE {
            let plain_pt = stroke_pt(StrokeRole::Wire, zoom, ppp, Emphasis::Plain);
            let raised_pt = stroke_pt(StrokeRole::Wire, zoom, ppp, Emphasis::Raised);
            let at = format!("zoom {zoom} at {ppp}x");
            assert_eq!(dpx(plain_pt, ppp), wire, "wire, {at}");
            assert_eq!(dpx(raised_pt, ppp), selected, "selected wire, {at}");
            let plain_dot = 2.0 * dpx(junction_radius_pt(zoom, plain_pt), ppp);
            let raised_dot = 2.0 * dpx(junction_radius_pt(zoom, raised_pt), ppp);
            assert_eq!(plain_dot, dot, "dot, {at}");
            assert_eq!(raised_dot, dot_selected, "selected dot, {at}");
            assert!(plain_dot >= 2.5 * dpx(plain_pt, ppp), "dot over wire, {at}");
            assert!(
                raised_dot >= 2.5 * dpx(raised_pt, ppp),
                "dot over selected, {at}"
            );
            assert!(plain_dot >= 5.0 * ppp, "dot floor, {at}");
        }
    }

    #[test]
    fn thin_strokes_land_on_whole_device_pixels() {
        for ppp in [1.0, 1.5, 2.0] {
            for step in 1..=320 {
                let zoom = step as f32 * 0.025;
                for role in [
                    StrokeRole::Wire,
                    StrokeRole::BusStrand,
                    StrokeRole::BusTap,
                    StrokeRole::Symbol,
                    StrokeRole::Documentation,
                    StrokeRole::Probe,
                ] {
                    let width = dpx(stroke_pt(role, zoom, ppp, Emphasis::Plain), ppp);
                    assert!(width >= 1.0, "{role:?} at zoom {zoom}, {ppp}x");
                    if width < CRISP_BELOW_DPX {
                        assert!(
                            (width - width.round()).abs() < 1e-4,
                            "{role:?} at zoom {zoom}, {ppp}x is {width} dpx"
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn a_bus_collapses_to_one_stroke_below_half_zoom_at_1x() {
        assert!(bus_collapses(0.49, 1.0));
        assert!(!bus_collapses(0.5, 1.0));
        assert!(!bus_collapses(1.0, 1.0));
        assert!(bus_collapses(0.49, 2.0));
        assert!(!bus_collapses(0.5, 2.0));
        assert_eq!(
            stroke_pt(StrokeRole::BusCollapsed, 0.25, 1.0, Emphasis::Plain),
            3.0
        );
        assert_eq!(
            stroke_pt(StrokeRole::BusCollapsed, 0.25, 1.0, Emphasis::Raised),
            4.0
        );
    }

    #[test]
    fn marks_keep_their_floors_and_ceilings() {
        assert_eq!(diamond_half_pt(0.1), 3.0);
        assert_eq!(diamond_half_pt(1.0), 3.0);
        assert_eq!(diamond_half_pt(2.0), 4.5);
        assert_eq!(diamond_half_pt(8.0), 5.0);
        assert_eq!(open_pin_side_pt(0.25), 4.0);
        assert_eq!(open_pin_side_pt(2.0), 6.0);
        assert_eq!(open_pin_side_pt(8.0), 7.0);
        assert_eq!(mark_outline_pt(1.0), 1.0);
        assert_eq!(mark_outline_pt(2.0), 1.0);
        assert!((mark_outline_pt(1.5) * 1.5 - 2.0).abs() < 1e-5);
    }

    /// A hovered junction's ring is whole device pixels at every density:
    /// 2 px at 1x, 2 at 1.25x and 1.5x, 3 at 2x, 5 at 3x.
    #[test]
    fn a_hover_ring_is_whole_device_pixels() {
        for (ppp, device) in [(1.0, 2.0), (1.25, 2.0), (1.5, 2.0), (2.0, 3.0), (3.0, 5.0)] {
            assert!(
                (hover_ring_pt(ppp) * ppp - device).abs() < 1e-5,
                "{ppp}x: {} device pixels",
                hover_ring_pt(ppp) * ppp
            );
        }
    }

    /// The design's text thresholds: where net names, instance names and pin
    /// names start to draw at 1x and at 2x.
    #[test]
    fn text_draws_from_the_design_thresholds_and_greeks_only_electrical_names() {
        // (world size, first zoom that draws at 1x, first at 2x)
        for (world, at_1x, at_2x) in [(11.0, 0.55, 0.32), (10.0, 0.6, 0.35), (5.0, 1.2, 0.7)] {
            assert!(matches!(
                text_lod(world, at_1x, 1.0, TextClass::Annotation),
                TextLod::Draw(_)
            ));
            assert!(matches!(
                text_lod(world, at_2x, 2.0, TextClass::Annotation),
                TextLod::Draw(_)
            ));
            assert_eq!(
                text_lod(world, at_1x - 0.06, 1.0, TextClass::Annotation),
                TextLod::Hide,
                "{world} wu at 1x"
            );
            assert_eq!(
                text_lod(world, at_2x - 0.06, 2.0, TextClass::Annotation),
                TextLod::Hide,
                "{world} wu at 2x"
            );
        }
        let TextLod::Greek(bar) = text_lod(11.0, 0.25, 1.0, TextClass::Electrical) else {
            panic!("a net name greeks below the floor");
        };
        assert_eq!(bar.size_pt, 2.75);
        assert_eq!(bar.height_pt, 1.0);
        assert_eq!(greek_bar_width_pt(1, 6.7, 0.25, 1.0), 3.0);
        assert_eq!(greek_bar_width_pt(4, 6.7, 0.25, 1.0), 6.7);
        assert_eq!(greek_tab_width_pt(12.0, 0.25, 1.0), 4.0);
    }
}
