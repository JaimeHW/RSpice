//! Drawing a resolved symbol.
//!
//! Paints a symbol document once its body and pins have been resolved against
//! the instance that uses it.
//!
//! Authored text is one of the body's shapes, so it is drawn and measured
//! beside the artwork it letters rather than in a pass of its own.
//! Its anchor moves with the instance; its glyphs stay upright, because a
//! label nobody can read is worth less than one in the wrong corner.

use egui::{Align2, Color32, Painter, Pos2, Shape, Stroke, vec2};

use super::lod;
use crate::symbols::PinLeads;
use rspice_design::{
    resolved_symbol::ResolvedCellSymbol,
    schematic::{
        component::Component,
        component_display::SchematicParameterLabelVisibility,
        label_layout::{self, LayoutPoint},
        label_position::LabelPosition,
        value_label::{cell_instance_value as instance_value, value_pieces},
    },
    symbol::{
        SymbolCurve, SymbolDocument, SymbolShape, SymbolTextAlign, SymbolTextPlacement,
        symbol_text_bounds,
    },
    symbol_generation::fit_pin_name,
};
use rspice_design_model::{Point, port::PortDirection};
use rspice_ui_kit::{palette::Palette, tokens::Tokens};

pub fn draw_resolved_symbol(
    painter: &Painter,
    origin: Pos2,
    scale: f32,
    component: &Component,
    symbol: &ResolvedCellSymbol,
    stroke: Stroke,
) {
    draw_resolved_symbol_with_visibility(
        painter,
        origin,
        scale,
        component,
        symbol,
        stroke,
        SchematicParameterLabelVisibility::NamesAndValues,
        &Tokens::get(painter.ctx()).color,
        &PinLeads::NONE,
    );
}

#[expect(
    clippy::too_many_arguments,
    reason = "Independent canvas transform, resolved symbol, and paint inputs."
)]
pub fn draw_resolved_symbol_with_visibility(
    painter: &Painter,
    origin: Pos2,
    scale: f32,
    component: &Component,
    symbol: &ResolvedCellSymbol,
    stroke: Stroke,
    parameter_labels: SchematicParameterLabelVisibility,
    palette: &Palette,
    leads: &PinLeads<'_>,
) {
    draw_symbol_body(painter, origin, scale, component, symbol, stroke, leads);
    draw_symbol_pins(
        painter, origin, scale, component, symbol, stroke, true, leads,
    );
    draw_symbol_labels(
        painter,
        origin,
        scale,
        component,
        symbol,
        parameter_labels,
        palette,
    );
}

pub fn resolved_symbol_world_bounds(
    component: &Component,
    symbol: &ResolvedCellSymbol,
) -> Option<(Point, Point)> {
    let mut bounds = BoundsAccumulator::default();
    fold_symbol_extent_points(symbol, |point| {
        let world = component.pos + component.transform_point(point);
        bounds.include(world);
    });
    bounds.finish()
}

fn draw_symbol_body(
    painter: &Painter,
    origin: Pos2,
    scale: f32,
    component: &Component,
    symbol: &ResolvedCellSymbol,
    stroke: Stroke,
    leads: &PinLeads<'_>,
) {
    for shape in &symbol.document().body {
        match shape {
            SymbolShape::Polyline { points, closed } => {
                let mut screen_points: Vec<Pos2> = points
                    .iter()
                    .map(|point| to_screen_symbol(origin, scale, component, symbol, *point))
                    .collect();
                screen_points.dedup();
                // One path, so every corner of the outline is a mitered join
                // rather than two butt ends with a notch between them.
                if *closed && screen_points.len() > 2 {
                    painter.add(egui::Shape::closed_line(screen_points, stroke));
                } else {
                    let rest = leads.paint_leads(painter, &screen_points, stroke);
                    if rest.len() >= 2 {
                        painter.add(egui::Shape::line(rest.to_vec(), stroke));
                    }
                }
            }
            SymbolShape::Circle { center, radius } => {
                painter.circle_stroke(
                    to_screen_symbol(origin, scale, component, symbol, *center),
                    *radius as f32 * scale,
                    stroke,
                );
            }
            SymbolShape::Arc {
                center,
                radius,
                start_degrees,
                sweep_degrees,
            } => {
                let arc = SymbolCurve::of_arc(*center, *radius, *start_degrees, *sweep_degrees);
                paint_curve(painter, origin, scale, component, symbol, &arc, stroke);
            }
            SymbolShape::Curve(curve) => {
                paint_curve(painter, origin, scale, component, symbol, curve, stroke);
            }
            SymbolShape::Arrow {
                tip,
                rotation_quarters,
            } => {
                let points: Vec<Pos2> = arrow_points(*tip, *rotation_quarters)
                    .into_iter()
                    .map(|point| to_screen_symbol(origin, scale, component, symbol, point))
                    .collect();
                painter.add(Shape::convex_polygon(points, stroke.color, Stroke::NONE));
            }
            SymbolShape::Dot { center, radius } => {
                painter.circle_filled(
                    to_screen_symbol(origin, scale, component, symbol, *center),
                    *radius as f32 * scale,
                    stroke.color,
                );
            }
            SymbolShape::Text {
                anchor,
                text,
                size,
                align,
            } => {
                let Some(font_size) =
                    lod::annotation_size(size.height() as f32, scale, painter.pixels_per_point())
                else {
                    continue;
                };
                rspice_ui_kit::text::paint_text(
                    painter,
                    to_screen_symbol(origin, scale, component, symbol, *anchor),
                    symbol_text_align(component, *align),
                    text,
                    rspice_ui_kit::theme::mono(
                        font_size,
                        rspice_ui_kit::theme::FontWeight::Regular,
                    ),
                    stroke.color,
                );
            }
        }
    }
}

/// egui's spelling of where the instance's orientation put a text run.
fn symbol_text_align(component: &Component, align: SymbolTextAlign) -> Align2 {
    match align.placement(|point| component.transform_point(point)) {
        SymbolTextPlacement::On => Align2::CENTER_CENTER,
        SymbolTextPlacement::After => Align2::LEFT_CENTER,
        SymbolTextPlacement::Before => Align2::RIGHT_CENTER,
        SymbolTextPlacement::Below => Align2::CENTER_TOP,
        SymbolTextPlacement::Above => Align2::CENTER_BOTTOM,
    }
}

#[expect(
    clippy::too_many_arguments,
    reason = "Independent canvas transform, symbol, and paint inputs."
)]
fn draw_symbol_pins(
    painter: &Painter,
    origin: Pos2,
    scale: f32,
    component: &Component,
    symbol: &ResolvedCellSymbol,
    stroke: Stroke,
    named: bool,
    leads: &PinLeads<'_>,
) {
    // Leads are drawn in the body's stroke: one symbol weight.
    let color = stroke.color;
    let document = symbol.document();
    let font_size =
        lod::annotation_size(document.pin_name_world(), scale, painter.pixels_per_point())
            .filter(|_| named);
    let font = |size: f32| {
        if document.pin_name_size.is_some() {
            rspice_ui_kit::theme::sans(size, rspice_ui_kit::theme::FontWeight::Regular)
        } else {
            rspice_ui_kit::theme::mono(size, rspice_ui_kit::theme::FontWeight::Regular)
        }
    };
    for pin in symbol.connectable_pins() {
        let offset = symbol.effective_pin_offset(pin);
        let terminal = to_screen(origin, scale, component, offset);
        let inner = to_screen(origin, scale, component, symbol.pin_lead_inner(pin));
        // No dot on the terminal: a filled dot means a junction of three or
        // more, and an open pin takes the scene's open-pin mark instead.
        if leads.paint_leads(painter, &[terminal, inner], stroke).len() == 2 {
            painter.line_segment([terminal, inner], stroke);
        }
        draw_direction_mark(painter, terminal, inner, pin.direction, scale, color);

        if let Some(font_size) = font_size.filter(|_| !pin.name_hidden) {
            let anchor = to_screen(origin, scale, component, symbol.pin_label_anchor(pin));
            let hang = component.pin_name_hang(pin);
            let align = super::drawing::text_align(hang.horizontal, hang.vertical);
            let name = displayed_pin_name(document, &pin.name);
            let ink = color.gamma_multiply(0.75);
            if hang.upward {
                super::drawing::paint_upward_text(
                    painter,
                    anchor,
                    align,
                    &name,
                    font(font_size),
                    ink,
                );
            } else {
                painter.text(anchor, align, name, font(font_size), ink);
            }
        }
    }
}

/// The pin name a symbol letters: a drawing's symbol letters it whole, a
/// generated block fits it to the width it reserved.
fn displayed_pin_name(document: &SymbolDocument, name: &str) -> String {
    if document.pin_name_size.is_some() {
        name.to_owned()
    } else {
        fit_pin_name(name)
    }
}

/// An instance's name and value in the type a primitive's use (Plex Sans
/// 10 wu Medium in `text`, 9 wu Regular in `text_dim`), upright and
/// justified to grow away from the body however the instance is turned.
fn draw_symbol_labels(
    painter: &Painter,
    origin: Pos2,
    scale: f32,
    component: &Component,
    symbol: &ResolvedCellSymbol,
    visibility: SchematicParameterLabelVisibility,
    palette: &Palette,
) {
    use super::drawing::{INSTANCE_NAME_WORLD, INSTANCE_VALUE_WORLD};
    use rspice_ui_kit::theme::{FontWeight, sans};
    let ppp = painter.pixels_per_point();
    // A label a drawing placed stands where it was placed; any other at the
    // symbol's own anchor.
    let at = |placed: LabelPosition, anchor: Point| {
        placed
            .placed_text(LayoutPoint::new(origin.x, origin.y), scale)
            .map_or_else(
                || {
                    (
                        to_screen_symbol(origin, scale, component, symbol, anchor),
                        instance_label_align(component, symbol, anchor),
                        false,
                    )
                },
                |text| {
                    (
                        super::drawing::layout_pos(text.anchor),
                        super::drawing::text_align(text.horizontal, text.vertical),
                        text.upward,
                    )
                },
            )
    };
    if let Some(size) = lod::annotation_size(
        component.name_label_pos.size_world(INSTANCE_NAME_WORLD),
        scale,
        ppp,
    ) && component.display_mode.show_name(visibility)
        && !component.name.is_empty()
    {
        let (position, align, upward) = at(component.name_label_pos, symbol.document().name_anchor);
        let font = sans(size, FontWeight::Medium);
        if upward {
            super::drawing::paint_upward_text(
                painter,
                position,
                align,
                &component.name,
                font,
                palette.text,
            );
        } else {
            painter.text(position, align, &component.name, font, palette.text);
        }
    }
    // A value the drawing lettered in texts of its own stands as they were
    // set.
    if let Some(pieces) = value_pieces(component) {
        if component.display_mode.show_value(visibility) {
            super::drawing::draw_value_pieces(painter, origin, scale, pieces, palette);
        }
        return;
    }
    let value = instance_value(component);
    if let Some(size) = lod::annotation_size(
        component.value_label_pos.size_world(INSTANCE_VALUE_WORLD),
        scale,
        ppp,
    ) && component.display_mode.show_value(visibility)
        && !value.is_empty()
    {
        let (position, align, upward) =
            at(component.value_label_pos, symbol.document().value_anchor);
        let font = sans(size, FontWeight::Regular);
        if upward {
            super::drawing::paint_upward_text(
                painter,
                position,
                align,
                value,
                font,
                palette.text_dim,
            );
        } else {
            rspice_ui_kit::text::paint_text(
                painter,
                position,
                align,
                value,
                font,
                palette.text_dim,
            );
        }
    }
}

/// Where a cell instance's name stands, in world units, and how it hangs off
/// that point: for whatever must keep clear of it.
pub(crate) fn instance_name_anchor(
    component: &Component,
    symbol: &ResolvedCellSymbol,
) -> (Pos2, Align2, bool) {
    let placed = label_layout::symbol_name_placement(component, symbol.document());
    (
        super::drawing::layout_pos(placed.anchor),
        super::drawing::text_align(placed.horizontal, placed.vertical),
        placed.upward,
    )
}

/// How an instance's name or value hangs off its anchor once the instance
/// is rotated or mirrored (D1 section 7), from the shared layout
/// (`label_layout::symbol_label_placement`) the printed page reads too.
fn instance_label_align(
    component: &Component,
    symbol: &ResolvedCellSymbol,
    anchor: Point,
) -> Align2 {
    let placed = label_layout::symbol_label_placement(component, symbol.document(), anchor);
    super::drawing::text_align(placed.horizontal, placed.vertical)
}

fn to_screen(origin: Pos2, scale: f32, component: &Component, point: Point) -> Pos2 {
    let transformed = component.transform_point(point);
    origin + vec2(transformed.x as f32 * scale, transformed.y as f32 * scale)
}

fn to_screen_symbol(
    origin: Pos2,
    scale: f32,
    component: &Component,
    symbol: &ResolvedCellSymbol,
    point: Point,
) -> Pos2 {
    to_screen(origin, scale, component, symbol.effective_point(point))
}

/// Signal-flow arrow on a pin's lead: an input points into the body it
/// drives, an output points out toward the net it drives. The mark is drawn
/// in symbol units so it holds its proportion at every zoom.
fn draw_direction_mark(
    painter: &Painter,
    terminal: Pos2,
    inner: Pos2,
    direction: PortDirection,
    scale: f32,
    color: Color32,
) {
    if !matches!(direction, PortDirection::In | PortDirection::Out) {
        return;
    }
    let lead = inner - terminal;
    let inward = lead.normalized();
    if !inward.is_finite() {
        return;
    }
    // Keep the whole mark on the lead, however long the lead is.
    let length = (MARK_LENGTH * scale).min(lead.length() * 0.5);
    let base_offset = (lead.length() - length) * 0.5;
    let (tip, base) = if direction == PortDirection::In {
        (
            terminal + inward * (base_offset + length),
            terminal + inward * base_offset,
        )
    } else {
        (
            terminal + inward * base_offset,
            terminal + inward * (base_offset + length),
        )
    };
    let across = vec2(-inward.y, inward.x) * (MARK_HALF_WIDTH * scale);
    painter.add(Shape::convex_polygon(
        vec![tip, base + across, base - across],
        color,
        Stroke::NONE,
    ));
}

/// Direction-mark extents in symbol units.
const MARK_LENGTH: f32 = 5.0;
const MARK_HALF_WIDTH: f32 = 2.0;

/// `curve` as it stands on the sheet relative to `component`: the instance's
/// own transform applied to unrounded points, so a curve never meets the
/// rounding every whole-unit point does.
fn placed_curve(
    component: &Component,
    symbol: &ResolvedCellSymbol,
    curve: &SymbolCurve,
) -> SymbolCurve {
    let place = |point: Point| component.transform_point(symbol.effective_point(point));
    let origin = place(Point::origin());
    curve.placed(
        origin,
        place(Point::new(1, 0)) - origin,
        place(Point::new(0, 1)) - origin,
    )
}

/// Strokes `curve` in as many straight pieces as its size on screen needs.
fn paint_curve(
    painter: &Painter,
    origin: Pos2,
    scale: f32,
    component: &Component,
    symbol: &ResolvedCellSymbol,
    curve: &SymbolCurve,
    stroke: Stroke,
) {
    paint_placed_curve(
        painter,
        origin,
        scale,
        &placed_curve(component, symbol, curve),
        stroke,
    );
}

/// Strokes `placed`, whose numbers are units from `origin`.
pub(super) fn paint_placed_curve(
    painter: &Painter,
    origin: Pos2,
    scale: f32,
    placed: &SymbolCurve,
    stroke: Stroke,
) {
    let pixels_per_unit = f64::from(scale * painter.pixels_per_point());
    let mut points = placed
        .tessellated(pixels_per_unit)
        .into_iter()
        .map(|[x, y]| origin + vec2(x as f32 * scale, y as f32 * scale))
        .collect::<Vec<_>>();
    if placed.is_whole() {
        points.pop();
        painter.add(Shape::closed_line(points, stroke));
    } else {
        painter.add(Shape::line(points, stroke));
    }
}

fn arrow_points(tip: Point, rotation_quarters: i32) -> [Point; 3] {
    let (dx, dy) = match rotation_quarters.rem_euclid(4) {
        0 => (1, 0),
        1 => (0, 1),
        2 => (-1, 0),
        _ => (0, -1),
    };
    let (sx, sy) = (-dy, dx);
    [
        tip,
        Point::new(tip.x - dx * 12 + sx * 6, tip.y - dy * 12 + sy * 6),
        Point::new(tip.x - dx * 12 - sx * 6, tip.y - dy * 12 - sy * 6),
    ]
}

fn fold_symbol_extent_points(symbol: &ResolvedCellSymbol, mut include: impl FnMut(Point)) {
    include(Point::origin());
    include(symbol.effective_point(symbol.document().name_anchor));
    include(symbol.effective_point(symbol.document().value_anchor));
    fold_symbol_artwork_points(symbol, include);
}

/// The extent of a symbol's artwork: pins and body, and none of the
/// instance's labels.
fn fold_symbol_artwork_points(symbol: &ResolvedCellSymbol, mut include: impl FnMut(Point)) {
    for pin in symbol.connectable_pins() {
        include(symbol.effective_pin_offset(pin));
        include(symbol.pin_lead_inner(pin));
    }

    for shape in &symbol.document().body {
        match shape {
            SymbolShape::Polyline {
                points: shape_points,
                ..
            } => {
                for point in shape_points {
                    include(symbol.effective_point(*point));
                }
            }
            SymbolShape::Circle { center, radius } | SymbolShape::Dot { center, radius } => {
                for point in cardinal_bounds(symbol.effective_point(*center), *radius) {
                    include(point);
                }
            }
            SymbolShape::Arc { center, radius, .. } => {
                for point in cardinal_bounds(symbol.effective_point(*center), *radius) {
                    include(point);
                }
            }
            SymbolShape::Curve(curve) => {
                let (least, greatest) = curve.whole_unit_bounds();
                include(symbol.effective_point(least));
                include(symbol.effective_point(greatest));
            }
            SymbolShape::Arrow {
                tip,
                rotation_quarters,
            } => {
                for point in arrow_points(*tip, *rotation_quarters) {
                    include(symbol.effective_point(point));
                }
            }
            SymbolShape::Text {
                anchor,
                text,
                size,
                align,
            } => {
                let (min, max) = symbol_text_bounds(*anchor, text, *size, *align);
                include(symbol.effective_point(min));
                include(symbol.effective_point(max));
            }
        }
    }
}

#[cfg(test)]
fn resolved_symbol_local_bounds(symbol: &ResolvedCellSymbol) -> Option<(Point, Point)> {
    let mut bounds = BoundsAccumulator::default();
    fold_symbol_extent_points(symbol, |point| bounds.include(point));
    bounds.finish()
}

#[derive(Default)]
struct BoundsAccumulator {
    min: Option<Point>,
    max: Option<Point>,
}

impl BoundsAccumulator {
    fn include(&mut self, point: Point) {
        match (self.min, self.max) {
            (Some(min), Some(max)) => {
                self.min = Some(Point::new(min.x.min(point.x), min.y.min(point.y)));
                self.max = Some(Point::new(max.x.max(point.x), max.y.max(point.y)));
            }
            _ => {
                self.min = Some(point);
                self.max = Some(point);
            }
        }
    }

    fn finish(self) -> Option<(Point, Point)> {
        Some((self.min?, self.max?))
    }
}

fn cardinal_bounds(center: Point, radius: i32) -> [Point; 4] {
    [
        Point::new(center.x - radius, center.y),
        Point::new(center.x + radius, center.y),
        Point::new(center.x, center.y - radius),
        Point::new(center.x, center.y + radius),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use egui::{Align, pos2};

    use rspice_design::{
        schematic::{component_type::ComponentType, rotation::Rotation},
        symbol::{SymbolDocument, SymbolPin, SymbolTextSize},
    };
    use rspice_design_model::port::PortSpec;
    use rspice_ui_kit::raster::Canvas;

    fn port(name: &str, direction: PortDirection) -> PortSpec {
        PortSpec {
            name: name.to_owned(),
            direction,
        }
    }

    /// A symbol whose whole body is one text run, so a render says only
    /// whether authored text reached the instance.
    fn lettered_symbol(anchor: Point, align: SymbolTextAlign) -> ResolvedCellSymbol {
        symbol_lettering("AMP", anchor, align)
    }

    /// [`lettered_symbol`] whose one run is `text`.
    fn symbol_lettering(text: &str, anchor: Point, align: SymbolTextAlign) -> ResolvedCellSymbol {
        ResolvedCellSymbol::from_authored_document(
            SymbolDocument {
                body: vec![SymbolShape::Text {
                    anchor,
                    text: text.to_owned(),
                    size: SymbolTextSize::Normal,
                    align,
                }],
                ..SymbolDocument::default()
            },
            &[],
        )
    }

    /// Render one instance of `symbol` on a canvas whose symbol origin is at
    /// `origin`, four screen pixels to the symbol unit.
    fn raster_instance(component: &Component, symbol: &ResolvedCellSymbol, origin: Pos2) -> Canvas {
        rspice_ui_kit::raster::render(egui::vec2(320.0, 320.0), |ui, _| {
            draw_symbol_body(
                &ui.painter().clone(),
                origin,
                4.0,
                component,
                symbol,
                Stroke::new(1.2, Color32::from_rgb(255, 64, 64)),
                &PinLeads::NONE,
            );
        })
    }

    /// How many pixels `rect` covers, and how many of them differ from the
    /// cleared background. The first guards the vacuous pass the rasterizer's
    /// own header warns about: a crop off the canvas yields nothing at all.
    fn painted_pixels(canvas: &Canvas, rect: egui::Rect) -> (usize, usize) {
        let mut total = 0;
        let mut painted = 0;
        for pixel in canvas.pixels_in(rect) {
            total += 1;
            if pixel != canvas.background() {
                painted += 1;
            }
        }
        (total, painted)
    }

    /// How many columns and how many rows of `search` carry paint — the shape
    /// of a run rather than its area, so a wide answer means glyphs set side
    /// by side and a tall one means glyphs stacked.
    fn painted_span(canvas: &Canvas, search: egui::Rect) -> (usize, usize) {
        let columns = (search.left() as i32..search.right() as i32)
            .filter(|x| {
                let strip = egui::Rect::from_min_max(
                    pos2(*x as f32, search.top()),
                    pos2(*x as f32 + 1.0, search.bottom()),
                );
                painted_pixels(canvas, strip).1 > 0
            })
            .count();
        let rows = (search.top() as i32..search.bottom() as i32)
            .filter(|y| {
                let strip = egui::Rect::from_min_max(
                    pos2(search.left(), *y as f32),
                    pos2(search.right(), *y as f32 + 1.0),
                );
                painted_pixels(canvas, strip).1 > 0
            })
            .count();
        (columns, rows)
    }

    fn symbol_with_shifted_origin() -> ResolvedCellSymbol {
        ResolvedCellSymbol::from_authored_document(
            SymbolDocument {
                origin: Point::new(20, 10),
                name_anchor: Point::new(20, 10),
                value_anchor: Point::new(20, 10),
                pins: vec![SymbolPin::new(
                    "OUT",
                    PortDirection::Out,
                    Some(Point::new(70, 10)),
                )],
                body: vec![SymbolShape::Polyline {
                    points: vec![Point::new(20, 10), Point::new(50, 20)],
                    closed: false,
                }],
                ..SymbolDocument::default()
            },
            &[port("OUT", PortDirection::Out)],
        )
    }

    #[test]
    fn resolved_symbol_bounds_are_relative_to_symbol_origin() {
        let symbol = symbol_with_shifted_origin();
        let component = Component::new(1, ComponentType::CellInstance, Point::new(100, 50));

        let bounds =
            resolved_symbol_world_bounds(&component, &symbol).expect("resolved symbol has extents");

        assert_eq!(bounds, (Point::new(100, 50), Point::new(150, 60)));
    }

    /// A turned or mirrored cell's name and value stay upright and are
    /// justified to grow away from the body, as the design's table for the
    /// generated symbol gives them (name above its left corner, value below
    /// it, both authored to grow rightward).
    #[test]
    fn instance_labels_rejustify_with_the_instance() {
        let symbol = ResolvedCellSymbol::from_generated(&[
            port("in", PortDirection::In),
            port("out", PortDirection::Out),
        ]);
        let name = symbol.document().name_anchor;
        let value = symbol.document().value_anchor;
        let (start, end, middle) = (Align::Min, Align::Max, Align::Center);
        let (top, bottom) = (Align::Min, Align::Max);
        let cases = [
            (
                "R0",
                Rotation::R0,
                false,
                false,
                [start, middle],
                [start, middle],
            ),
            ("R90", Rotation::R90, false, false, [start, top], [end, top]),
            (
                "R180",
                Rotation::R180,
                false,
                false,
                [end, middle],
                [end, middle],
            ),
            (
                "R270",
                Rotation::R270,
                false,
                false,
                [end, bottom],
                [start, bottom],
            ),
            (
                "mirror X",
                Rotation::R0,
                true,
                false,
                [end, middle],
                [end, middle],
            ),
            (
                "mirror Y",
                Rotation::R0,
                false,
                true,
                [start, middle],
                [start, middle],
            ),
        ];
        for (orientation, rotation, mirror_h, mirror_v, name_align, value_align) in cases {
            let mut component = Component::new(1, ComponentType::CellInstance, Point::origin());
            component.rotation = rotation;
            component.mirror_h = mirror_h;
            component.mirror_v = mirror_v;
            assert_eq!(
                instance_label_align(&component, &symbol, name).0,
                name_align,
                "{orientation} name"
            );
            assert_eq!(
                instance_label_align(&component, &symbol, value).0,
                value_align,
                "{orientation} value"
            );
        }
    }

    /// A cell pin carries no solder dot, on the canvas or in the SVG: a
    /// filled dot means a junction of three or more conductors, and an open
    /// pin takes the scene's hollow open-pin mark instead.
    #[test]
    fn a_cell_pin_has_no_solder_dot() {
        let symbol = symbol_with_shifted_origin();
        let component = Component::new(1, ComponentType::CellInstance, Point::new(100, 50));
        let painter = Painter::new(
            egui::Context::default(),
            egui::LayerId::background(),
            egui::Rect::from_min_size(Pos2::ZERO, egui::vec2(400.0, 400.0)),
        );
        draw_symbol_pins(
            &painter,
            Pos2::new(100.0, 100.0),
            2.0,
            &component,
            &symbol,
            Stroke::new(1.0, egui::Color32::WHITE),
            false,
            &PinLeads::NONE,
        );
        let mut circles = 0;
        painter.for_each_shape(|shape| {
            if matches!(shape.shape, Shape::Circle(_)) {
                circles += 1;
            }
        });
        assert_eq!(circles, 0, "the canvas pin has a dot");
    }

    /// A name its drawing lettered on its side reads upward on the canvas and
    /// in the SVG, about the same anchor.
    #[test]
    fn a_pin_name_on_its_side_reads_upward_on_canvas() {
        let mut pin = SymbolPin::new("REF", PortDirection::In, Some(Point::new(0, -30)));
        pin.name_at = Some(rspice_design::symbol::PinNamePlacement {
            offset: Point::new(0, 5),
            reach: Point::new(0, 1),
            upward: true,
        });
        let symbol = ResolvedCellSymbol::from_authored_document(
            SymbolDocument {
                pins: vec![pin],
                pin_name_size: Some(15),
                body: vec![SymbolShape::Polyline {
                    points: vec![Point::new(-40, -30), Point::new(40, -30)],
                    closed: false,
                }],
                ..SymbolDocument::default()
            },
            &[port("REF", PortDirection::In)],
        );
        let component = Component::new(1, ComponentType::CellInstance, Point::new(100, 100));
        let ctx = egui::Context::default();
        rspice_ui_kit::Theme::default().apply(&ctx);
        let mut angles = Vec::new();
        let _ = ctx.run_ui(
            rspice_ui_kit::raster::display_input(egui::vec2(400.0, 400.0), 1.0),
            |ui| {
                let painter = ui.painter().clone();
                draw_symbol_pins(
                    &painter,
                    pos2(200.0, 200.0),
                    2.0,
                    &component,
                    &symbol,
                    Stroke::new(1.0, Color32::WHITE),
                    true,
                    &PinLeads::NONE,
                );
                angles.clear();
                painter.for_each_shape(|shape| {
                    if let Shape::Text(text) = &shape.shape {
                        angles.push(text.angle);
                    }
                });
            },
        );
        assert_eq!(angles, [-std::f32::consts::FRAC_PI_2]);
    }

    /// A pin whose symbol letters no name keeps its lead and terminal, and
    /// neither the canvas nor the SVG letters its name.
    #[test]
    fn a_pin_with_its_name_hidden_draws_its_lead_and_no_name() {
        let named = |hidden: bool| {
            ResolvedCellSymbol::from_authored_document(
                SymbolDocument {
                    pins: vec![
                        SymbolPin::new("IN", PortDirection::In, Some(Point::new(-30, 0)))
                            .with_name_hidden(hidden),
                    ],
                    body: vec![SymbolShape::Polyline {
                        points: vec![Point::new(-20, -10), Point::new(-20, 10)],
                        closed: false,
                    }],
                    ..SymbolDocument::default()
                },
                &[port("IN", PortDirection::In)],
            )
        };
        let component = Component::new(1, ComponentType::CellInstance, Point::origin());
        for hidden in [false, true] {
            let symbol = named(hidden);
            let canvas = rspice_ui_kit::raster::render(egui::vec2(320.0, 320.0), |ui, _| {
                draw_symbol_pins(
                    &ui.painter().clone(),
                    pos2(160.0, 160.0),
                    4.0,
                    &component,
                    &symbol,
                    Stroke::new(1.2, Color32::from_rgb(255, 64, 64)),
                    true,
                    &PinLeads::NONE,
                );
            });
            // The lead runs from x -30 to the body at -20: screen 40 to 80.
            // The name reads into the body, right of it.
            let (covered, lead) = painted_pixels(
                &canvas,
                egui::Rect::from_min_max(pos2(40.0, 150.0), pos2(78.0, 170.0)),
            );
            assert!(covered > 0 && lead > 0, "the lead is painted");
            let (covered, name) = painted_pixels(
                &canvas,
                egui::Rect::from_min_max(pos2(84.0, 130.0), pos2(300.0, 190.0)),
            );
            assert!(covered > 0, "the asserted crop must be on the canvas");
            assert_eq!(name > 0, !hidden, "hidden = {hidden}");
        }
    }

    /// The reported defect: a generated block taller than it is wide sent
    /// every outer side pin's lead up or down instead of into the body, so
    /// the pins read as unconnected and their names sat across the outline.
    #[test]
    fn a_tall_generated_block_runs_every_lead_into_the_body_it_belongs_to() {
        let mut ports = vec![port("data_out[0]", PortDirection::Out)];
        for index in 0..7 {
            ports.push(port(&format!("data_in[{index}]"), PortDirection::In));
        }
        let symbol = ResolvedCellSymbol::from_generated(&ports);
        let (min, max) = symbol
            .document()
            .drawn_body_bounds()
            .expect("a generated block has a body");
        assert!(
            max.y - min.y > max.x - min.x,
            "this fixture must be taller than it is wide"
        );

        for pin in symbol.connectable_pins() {
            let terminal = symbol.effective_pin_offset(pin);
            let inner = symbol.pin_lead_inner(pin);
            let anchor = symbol.pin_label_anchor(pin);

            assert_eq!(
                inner.y, terminal.y,
                "{} must run its lead horizontally into the body",
                pin.name
            );
            assert_eq!(
                inner.x.abs(),
                (max.x - min.x) / 2,
                "{} must stop on the body outline",
                pin.name
            );
            assert!(
                anchor.x.abs() < inner.x.abs(),
                "{} must print its name inside the body, not across the outline",
                pin.name
            );
        }
    }

    /// Two name columns inside one body must never print over each other.
    #[test]
    fn long_names_on_both_edges_stay_in_separate_columns() {
        let ports = vec![
            port("data_in[0]", PortDirection::In),
            port("write_en", PortDirection::In),
            port("data_out[0]", PortDirection::Out),
        ];
        let symbol = ResolvedCellSymbol::from_generated(&ports);
        let advance = 3; // symbol units per monospace character at the pin size

        let mut left_end = i32::MIN;
        let mut right_start = i32::MAX;
        for pin in symbol.connectable_pins() {
            let anchor = symbol.pin_label_anchor(pin);
            let span = advance * pin.name.chars().count() as i32;
            if pin.direction == PortDirection::In {
                left_end = left_end.max(anchor.x + span);
            } else {
                right_start = right_start.min(anchor.x - span);
            }
        }

        assert!(
            left_end < right_start,
            "input names end at {left_end} but output names start at {right_start}"
        );
    }

    /// T12, unrotated: authored text is body artwork, so a placed instance
    /// paints it — the defect this variant exists to close was a symbol whose
    /// `+`, `−` and `OTA` were drawn by the editor and by nothing else.
    #[test]
    fn a_placed_instance_paints_the_text_its_body_carries() {
        let symbol = lettered_symbol(Point::origin(), SymbolTextAlign::Left);
        let component = Component::new(1, ComponentType::CellInstance, Point::origin());
        let origin = pos2(80.0, 160.0);

        let canvas = raster_instance(&component, &symbol, origin);

        // The run hangs right of the anchor and is vertically centred on it:
        // 3 glyphs at a 9-unit cap height, four pixels to the unit.
        let (covered, painted) = painted_pixels(
            &canvas,
            egui::Rect::from_min_max(pos2(82.0, 144.0), pos2(138.0, 176.0)),
        );
        assert!(covered > 0, "the asserted crop must be on the canvas");
        assert!(painted > 0, "a placed instance must paint its body text");

        let (covered, painted) = painted_pixels(
            &canvas,
            egui::Rect::from_min_max(pos2(20.0, 144.0), pos2(74.0, 176.0)),
        );
        assert!(covered > 0, "the asserted crop must be on the canvas");
        assert_eq!(
            painted, 0,
            "a left-aligned run must not reach back past its anchor"
        );
    }

    /// T12, rotated: a quarter turn carries the anchor to the transformed
    /// point and swings the run round it, and the glyphs stay upright — a
    /// run painted sideways is what the upright convention exists to prevent.
    #[test]
    fn a_rotated_instance_moves_the_anchor_and_leaves_the_glyphs_upright() {
        let symbol = lettered_symbol(Point::new(10, 0), SymbolTextAlign::Left);
        let mut component = Component::new(1, ComponentType::CellInstance, Point::origin());
        component.rotation = Rotation::R90;
        let origin = pos2(160.0, 100.0);

        let canvas = raster_instance(&component, &symbol, origin);

        // (10, 0) turns to (0, 10): the anchor lands ten units below the
        // origin, and the run hangs below it, centred on the anchor.
        let below = egui::Rect::from_min_max(pos2(100.0, 138.0), pos2(224.0, 200.0));
        let (covered, painted) = painted_pixels(&canvas, below);
        assert!(covered > 0, "the asserted crop must be on the canvas");
        assert!(
            painted > 0,
            "the run must follow its anchor through the rotation"
        );

        let (covered, painted) = painted_pixels(
            &canvas,
            egui::Rect::from_min_max(pos2(164.0, 100.0), pos2(300.0, 130.0)),
        );
        assert!(covered > 0, "the asserted crop must be on the canvas");
        assert_eq!(
            painted, 0,
            "the run must not stay where an unrotated instance would put it"
        );

        // Upright glyphs set three letters side by side, so the run is wider
        // than it is tall. Glyphs carried round with the symbol would stack.
        let (columns, rows) = painted_span(&canvas, below);
        assert!(
            columns > rows,
            "the glyph run must stay horizontal: {columns} columns by {rows} rows"
        );
    }

    /// The stretches of columns of `search` that carry paint, left to right.
    fn painted_stretches(canvas: &Canvas, search: egui::Rect) -> Vec<std::ops::Range<i32>> {
        let mut stretches: Vec<std::ops::Range<i32>> = Vec::new();
        for x in search.left() as i32..search.right() as i32 {
            let strip = egui::Rect::from_min_max(
                pos2(x as f32, search.top()),
                pos2(x as f32 + 1.0, search.bottom()),
            );
            if painted_pixels(canvas, strip).1 == 0 {
                continue;
            }
            match stretches.last_mut() {
                Some(stretch) if stretch.end == x => stretch.end = x + 1,
                _ => stretches.push(x..x + 1),
            }
        }
        stretches
    }

    /// Text a symbol carries in Hebrew reads from the right, and the number
    /// inside it reads left to right where it stands: last, so first from
    /// the anchor of a left-aligned run, drawn where the number alone is.
    #[test]
    fn a_hebrew_run_is_drawn_in_the_order_it_is_read() {
        let component = Component::new(1, ComponentType::CellInstance, Point::origin());
        let origin = pos2(40.0, 160.0);
        let run = egui::Rect::from_min_max(pos2(40.0, 140.0), pos2(320.0, 180.0));
        let drawn = |text: &str| {
            let symbol = symbol_lettering(text, Point::origin(), SymbolTextAlign::Left);
            painted_stretches(&raster_instance(&component, &symbol, origin), run)
        };

        let phrase = drawn("מגבר 2");
        let number = drawn("2");

        assert!(
            !number.is_empty() && phrase.len() > number.len(),
            "{phrase:?}"
        );
        for (in_phrase, alone) in phrase.iter().zip(&number) {
            assert!(
                (in_phrase.start - alone.start).abs() <= 1
                    && (in_phrase.end - alone.end).abs() <= 1,
                "the number leads the run: {phrase:?} against {number:?}"
            );
        }
    }

    #[test]
    fn resolved_symbol_local_bounds_fold_uses_effective_points_without_world_component() {
        let symbol = symbol_with_shifted_origin();

        let bounds =
            resolved_symbol_local_bounds(&symbol).expect("resolved symbol has local extents");

        assert_eq!(bounds, (Point::new(0, 0), Point::new(50, 10)));
    }
}
