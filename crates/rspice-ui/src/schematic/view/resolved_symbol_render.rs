//! Drawing a resolved symbol.
//!
//! Paints a symbol document once its shapes, pins, and text have been
//! resolved against the instance that uses it, to either the canvas or an
//! SVG export. Both paths share this so a printed sheet matches the screen.

use std::fmt::Write;

use egui::{Align, Align2, Color32, Painter, Pos2, Shape, Stroke, vec2};

use crate::schematic::export::SvgExportConfig;
use crate::state::{
    Component, GENERATED_PIN_LABEL_SIZE, Point, PortDirection, ResolvedCellSymbol, SymbolShape,
    fit_pin_name,
};

pub(crate) fn draw_resolved_symbol(
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
        crate::state::SchematicParameterLabelVisibility::NamesAndValues,
    );
}

pub(crate) fn draw_resolved_symbol_with_visibility(
    painter: &Painter,
    origin: Pos2,
    scale: f32,
    component: &Component,
    symbol: &ResolvedCellSymbol,
    stroke: Stroke,
    parameter_labels: crate::state::SchematicParameterLabelVisibility,
) {
    draw_symbol_body(painter, origin, scale, component, symbol, stroke);
    draw_symbol_pins(painter, origin, scale, component, symbol, stroke.color);
    draw_symbol_labels(
        painter,
        origin,
        scale,
        component,
        symbol,
        stroke.color,
        parameter_labels,
    );
}

pub(crate) fn write_resolved_symbol_svg(
    svg: &mut String,
    component: &Component,
    symbol: &ResolvedCellSymbol,
    config: &SvgExportConfig,
) {
    write_symbol_body_svg(svg, component, symbol, config);
    write_symbol_pins_svg(svg, component, symbol, config);
    write_symbol_labels_svg(svg, component, symbol, config);
}

pub(crate) fn resolved_symbol_world_bounds(
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
) {
    for shape in &symbol.document().body {
        match shape {
            SymbolShape::Polyline { points, closed } => {
                let screen_points: Vec<Pos2> = points
                    .iter()
                    .map(|point| to_screen_symbol(origin, scale, component, symbol, *point))
                    .collect();
                for pair in screen_points.windows(2) {
                    painter.line_segment([pair[0], pair[1]], stroke);
                }
                if *closed
                    && screen_points.len() > 2
                    && let (Some(first), Some(last)) = (screen_points.first(), screen_points.last())
                {
                    painter.line_segment([*last, *first], stroke);
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
                let points: Vec<Pos2> =
                    arc_points(*center, *radius, *start_degrees, *sweep_degrees)
                        .into_iter()
                        .map(|point| to_screen_symbol(origin, scale, component, symbol, point))
                        .collect();
                painter.add(Shape::line(points, stroke));
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
        }
    }
}

fn draw_symbol_pins(
    painter: &Painter,
    origin: Pos2,
    scale: f32,
    component: &Component,
    symbol: &ResolvedCellSymbol,
    color: Color32,
) {
    let stroke = Stroke::new(1.1 * scale, color);
    let font_size = GENERATED_PIN_LABEL_SIZE * scale;
    for pin in symbol.connectable_pins() {
        let offset = symbol.effective_pin_offset(pin);
        let terminal = to_screen(origin, scale, component, offset);
        let inner = to_screen(origin, scale, component, symbol.pin_lead_inner(pin));
        painter.line_segment([terminal, inner], stroke);
        painter.circle_filled(terminal, 1.6 * scale, color);
        draw_direction_mark(painter, terminal, inner, pin.direction, scale, color);

        if font_size >= 4.0 {
            let anchor = to_screen(origin, scale, component, symbol.pin_label_anchor(pin));
            painter.text(
                anchor,
                pin_label_align(component, pin.side),
                fit_pin_name(&pin.name),
                crate::ui::theme::mono(font_size, crate::ui::theme::FontWeight::Regular),
                color.gamma_multiply(0.75),
            );
        }
    }
}

/// Align a pin name so it reads from its own edge into the body, whatever
/// orientation the instance is placed in.
pub(super) fn pin_label_align(component: &Component, side: crate::state::SymbolPinSide) -> Align2 {
    let inward = component.transform_point(crate::state::inward_step(side));
    if inward.x.abs() >= inward.y.abs() {
        if inward.x >= 0 {
            Align2::LEFT_CENTER
        } else {
            Align2::RIGHT_CENTER
        }
    } else if inward.y >= 0 {
        Align2::CENTER_TOP
    } else {
        Align2::CENTER_BOTTOM
    }
}

fn draw_symbol_labels(
    painter: &Painter,
    origin: Pos2,
    scale: f32,
    component: &Component,
    symbol: &ResolvedCellSymbol,
    color: Color32,
    visibility: crate::state::SchematicParameterLabelVisibility,
) {
    let font_size = (9.0 * scale).max(1.0);
    if font_size < 4.0 {
        return;
    }
    let font = crate::ui::theme::mono(font_size, crate::ui::theme::FontWeight::Medium);
    if component.display_mode.show_name(visibility) && !component.name.is_empty() {
        painter.text(
            to_screen_symbol(
                origin,
                scale,
                component,
                symbol,
                symbol.document().name_anchor,
            ),
            Align2::LEFT_CENTER,
            &component.name,
            font.clone(),
            color,
        );
    }
    let value = if component.value.is_empty() {
        component
            .library_cell
            .as_ref()
            .map(|binding| binding.cell.as_str())
            .unwrap_or("")
    } else {
        component.value.as_str()
    };
    if component.display_mode.show_value(visibility) && !value.is_empty() {
        painter.text(
            to_screen_symbol(
                origin,
                scale,
                component,
                symbol,
                symbol.document().value_anchor,
            ),
            Align2::LEFT_CENTER,
            value,
            font,
            color.gamma_multiply(0.8),
        );
    }
}

fn write_symbol_body_svg(
    svg: &mut String,
    component: &Component,
    symbol: &ResolvedCellSymbol,
    config: &SvgExportConfig,
) {
    for shape in &symbol.document().body {
        match shape {
            SymbolShape::Polyline { points, closed } => {
                if points.is_empty() {
                    continue;
                }
                svg.push_str(r#"<path class="component" d=""#);
                for (index, point) in points.iter().enumerate() {
                    let (x, y) = to_svg_symbol(component, symbol, *point, config);
                    if index == 0 {
                        write!(svg, "M {x} {y}").unwrap();
                    } else {
                        write!(svg, " L {x} {y}").unwrap();
                    }
                }
                if *closed {
                    svg.push_str(" Z");
                }
                svg.push_str("\"/>\n");
            }
            SymbolShape::Circle { center, radius } => {
                let (cx, cy) = to_svg_symbol(component, symbol, *center, config);
                let radius = *radius as f64 * config.grid_size;
                writeln!(
                    svg,
                    r#"<circle class="component" cx="{cx}" cy="{cy}" r="{radius}"/>"#
                )
                .unwrap();
            }
            SymbolShape::Arc {
                center,
                radius,
                start_degrees,
                sweep_degrees,
            } => {
                let points = arc_points(*center, *radius, *start_degrees, *sweep_degrees);
                if points.is_empty() {
                    continue;
                }
                svg.push_str(r#"<path class="component" d=""#);
                for (index, point) in points.into_iter().enumerate() {
                    let (x, y) = to_svg_symbol(component, symbol, point, config);
                    if index == 0 {
                        write!(svg, "M {x} {y}").unwrap();
                    } else {
                        write!(svg, " L {x} {y}").unwrap();
                    }
                }
                svg.push_str("\"/>\n");
            }
            SymbolShape::Arrow {
                tip,
                rotation_quarters,
            } => {
                write_polygon_svg(
                    svg,
                    component,
                    symbol,
                    &arrow_points(*tip, *rotation_quarters),
                    config,
                );
            }
            SymbolShape::Dot { center, radius } => {
                let (cx, cy) = to_svg_symbol(component, symbol, *center, config);
                let radius = *radius as f64 * config.grid_size;
                writeln!(
                    svg,
                    r#"<circle cx="{cx}" cy="{cy}" r="{radius}" fill="{fill}" stroke="none"/>"#,
                    fill = config.component_color
                )
                .unwrap();
            }
        }
    }
}

fn write_symbol_pins_svg(
    svg: &mut String,
    component: &Component,
    symbol: &ResolvedCellSymbol,
    config: &SvgExportConfig,
) {
    for pin in symbol.connectable_pins() {
        let offset = symbol.effective_pin_offset(pin);
        let (x1, y1) = to_svg(component, offset, config);
        let (x2, y2) = to_svg(component, symbol.pin_lead_inner(pin), config);
        writeln!(
            svg,
            r#"<line class="component" x1="{x1}" y1="{y1}" x2="{x2}" y2="{y2}"/>"#
        )
        .unwrap();
        let radius = 1.6 * config.grid_size;
        writeln!(
            svg,
            r#"<circle cx="{x1}" cy="{y1}" r="{radius}" fill="{fill}" stroke="none"/>"#,
            fill = config.component_color
        )
        .unwrap();
        write_direction_mark_svg(svg, (x1, y1), (x2, y2), pin.direction, config);
        let (tx, ty) = to_svg(component, symbol.pin_label_anchor(pin), config);
        write_anchored_text_svg(
            svg,
            tx,
            ty,
            &fit_pin_name(&pin.name),
            pin_label_align(component, pin.side),
        );
    }
}

/// The same signal-flow arrow the canvas paints on a pin's lead.
fn write_direction_mark_svg(
    svg: &mut String,
    terminal: (f64, f64),
    inner: (f64, f64),
    direction: PortDirection,
    config: &SvgExportConfig,
) {
    if !matches!(direction, PortDirection::In | PortDirection::Out) {
        return;
    }
    let lead = (inner.0 - terminal.0, inner.1 - terminal.1);
    let span = lead.0.hypot(lead.1);
    if span <= f64::EPSILON {
        return;
    }
    let inward = (lead.0 / span, lead.1 / span);
    let length = (f64::from(MARK_LENGTH) * config.grid_size).min(span * 0.5);
    let base_offset = (span - length) * 0.5;
    let at = |distance: f64| {
        (
            terminal.0 + inward.0 * distance,
            terminal.1 + inward.1 * distance,
        )
    };
    let (tip, base) = if direction == PortDirection::In {
        (at(base_offset + length), at(base_offset))
    } else {
        (at(base_offset), at(base_offset + length))
    };
    let across = (
        -inward.1 * f64::from(MARK_HALF_WIDTH) * config.grid_size,
        inward.0 * f64::from(MARK_HALF_WIDTH) * config.grid_size,
    );
    writeln!(
        svg,
        r#"<polygon points="{},{} {},{} {},{}" fill="{fill}" stroke="none"/>"#,
        tip.0,
        tip.1,
        base.0 + across.0,
        base.1 + across.1,
        base.0 - across.0,
        base.1 - across.1,
        fill = config.component_color
    )
    .unwrap();
}

fn write_symbol_labels_svg(
    svg: &mut String,
    component: &Component,
    symbol: &ResolvedCellSymbol,
    config: &SvgExportConfig,
) {
    if component
        .display_mode
        .show_name(crate::state::SchematicParameterLabelVisibility::NamesAndValues)
        && !component.name.is_empty()
    {
        let (x, y) = to_svg_symbol(component, symbol, symbol.document().name_anchor, config);
        write_text_svg(svg, x, y, &component.name);
    }
    let value = if component.value.is_empty() {
        component
            .library_cell
            .as_ref()
            .map(|binding| binding.cell.as_str())
            .unwrap_or("")
    } else {
        component.value.as_str()
    };
    if component
        .display_mode
        .show_value(crate::state::SchematicParameterLabelVisibility::NamesAndValues)
        && !value.is_empty()
    {
        let (x, y) = to_svg_symbol(component, symbol, symbol.document().value_anchor, config);
        write_text_svg(svg, x, y, value);
    }
}

fn write_polygon_svg(
    svg: &mut String,
    component: &Component,
    symbol: &ResolvedCellSymbol,
    points: &[Point],
    config: &SvgExportConfig,
) {
    svg.push_str("<polygon points=\"");
    for (index, point) in points.iter().enumerate() {
        let (x, y) = to_svg_symbol(component, symbol, *point, config);
        if index > 0 {
            svg.push(' ');
        }
        write!(svg, "{x},{y}").unwrap();
    }
    writeln!(
        svg,
        r#"" fill="{fill}" stroke="none"/>"#,
        fill = config.component_color
    )
    .unwrap();
}

fn write_text_svg(svg: &mut String, x: f64, y: f64, text: &str) {
    writeln!(
        svg,
        r#"<text class="text" x="{x}" y="{y}">{}</text>"#,
        escape_xml(text)
    )
    .unwrap();
}

/// Write text that hangs off its anchor point the same way the canvas draws
/// it, so an exported sheet places a pin name exactly where the screen does.
fn write_anchored_text_svg(svg: &mut String, x: f64, y: f64, text: &str, align: Align2) {
    let anchor = match align.x() {
        Align::Min => "start",
        Align::Center => "middle",
        Align::Max => "end",
    };
    let baseline = match align.y() {
        Align::Min => "hanging",
        Align::Center => "central",
        Align::Max => "text-after-edge",
    };
    writeln!(
        svg,
        r#"<text class="text" x="{x}" y="{y}" text-anchor="{anchor}" dominant-baseline="{baseline}">{}</text>"#,
        escape_xml(text)
    )
    .unwrap();
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

fn to_svg(component: &Component, point: Point, config: &SvgExportConfig) -> (f64, f64) {
    let transformed = component.pos + component.transform_point(point);
    (
        transformed.x as f64 * config.grid_size,
        transformed.y as f64 * config.grid_size,
    )
}

fn to_svg_symbol(
    component: &Component,
    symbol: &ResolvedCellSymbol,
    point: Point,
    config: &SvgExportConfig,
) -> (f64, f64) {
    to_svg(component, symbol.effective_point(point), config)
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

fn arc_points(center: Point, radius: i32, start_degrees: i32, sweep_degrees: i32) -> Vec<Point> {
    let steps = 24.max(sweep_degrees.abs() / 8) as usize;
    (0..=steps)
        .map(|step| {
            let t = step as f32 / steps as f32;
            let degrees = start_degrees as f32 + sweep_degrees as f32 * t;
            let radians = degrees.to_radians();
            Point::new(
                center.x + (radius as f32 * radians.cos()).round() as i32,
                center.y + (radius as f32 * radians.sin()).round() as i32,
            )
        })
        .collect()
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
            SymbolShape::Arrow {
                tip,
                rotation_quarters,
            } => {
                for point in arrow_points(*tip, *rotation_quarters) {
                    include(symbol.effective_point(point));
                }
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

fn escape_xml(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schematic::export::SvgExportConfig;
    use crate::state::{ComponentType, PortSpec, SymbolDocument, SymbolPin};

    fn port(name: &str, direction: PortDirection) -> PortSpec {
        PortSpec {
            name: name.to_owned(),
            direction,
        }
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

    #[test]
    fn resolved_symbol_svg_coordinates_are_relative_to_symbol_origin() {
        let symbol = symbol_with_shifted_origin();
        let component = Component::new(1, ComponentType::CellInstance, Point::new(100, 50));
        let mut svg = String::new();
        let config = SvgExportConfig {
            grid_size: 1.0,
            ..SvgExportConfig::default()
        };

        write_resolved_symbol_svg(&mut svg, &component, &symbol, &config);

        assert!(
            svg.contains(r#"<path class="component" d="M 100 50 L 130 60"/>"#),
            "svg must place authored body points relative to the symbol origin: {svg}"
        );
        // The terminal stands 20 off the body's right edge, and the lead
        // spans all of it: a pin that stops short of the artwork reads as an
        // unconnected pin.
        assert!(
            svg.contains(r#"<line class="component" x1="150" y1="50" x2="130" y2="50"/>"#),
            "svg must run authored pin leads to the body, relative to the symbol origin: {svg}"
        );
        assert!(
            svg.contains(r#"text-anchor="end""#),
            "a right-edge pin name must read inward from the body edge: {svg}"
        );
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
        let symbol = ResolvedCellSymbol::from_ports_for_test(&ports);
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
        let symbol = ResolvedCellSymbol::from_ports_for_test(&ports);
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

    #[test]
    fn resolved_symbol_local_bounds_fold_uses_effective_points_without_world_component() {
        let symbol = symbol_with_shifted_origin();

        let bounds =
            resolved_symbol_local_bounds(&symbol).expect("resolved symbol has local extents");

        assert_eq!(bounds, (Point::new(0, 0), Point::new(50, 10)));
    }
}
