use std::fmt::Write;

use egui::{Align2, Color32, Painter, Pos2, Shape, Stroke, vec2};

use crate::schematic::export::SvgExportConfig;
use crate::state::{
    Component, Point, PortDirection, ResolvedCellSymbol, SYMBOL_TERMINAL_GRID, SymbolShape,
};

pub(crate) fn draw_resolved_symbol(
    painter: &Painter,
    origin: Pos2,
    scale: f32,
    component: &Component,
    symbol: &ResolvedCellSymbol,
    stroke: Stroke,
) {
    draw_symbol_body(painter, origin, scale, component, symbol, stroke);
    draw_symbol_pins(painter, origin, scale, component, symbol, stroke.color);
    draw_symbol_labels(painter, origin, scale, component, symbol, stroke.color);
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
    let font_size = 6.5 * scale;
    for pin in symbol.connectable_pins() {
        let offset = symbol.effective_pin_offset(pin);
        let terminal = to_screen(origin, scale, component, offset);
        let inner = to_screen(origin, scale, component, stub_inner(offset));
        painter.line_segment([terminal, inner], stroke);
        painter.circle_filled(terminal, 1.6 * scale, color);
        draw_direction_mark(painter, terminal, inner, pin.direction, color);

        if font_size >= 4.0 {
            painter.text(
                inner,
                Align2::CENTER_CENTER,
                &pin.name,
                crate::ui::theme::mono(font_size, crate::ui::theme::FontWeight::Regular),
                color.gamma_multiply(0.75),
            );
        }
    }
}

fn draw_symbol_labels(
    painter: &Painter,
    origin: Pos2,
    scale: f32,
    component: &Component,
    symbol: &ResolvedCellSymbol,
    color: Color32,
) {
    let font_size = (9.0 * scale).max(1.0);
    if font_size < 4.0 {
        return;
    }
    let font = crate::ui::theme::mono(font_size, crate::ui::theme::FontWeight::Medium);
    if !component.name.is_empty() {
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
    if !value.is_empty() {
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
        let (x2, y2) = to_svg(component, stub_inner(offset), config);
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
        let (tx, ty) = to_svg(component, stub_inner(offset), config);
        write_text_svg(svg, tx, ty, &pin.name);
    }
}

fn write_symbol_labels_svg(
    svg: &mut String,
    component: &Component,
    symbol: &ResolvedCellSymbol,
    config: &SvgExportConfig,
) {
    if !component.name.is_empty() {
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
    if !value.is_empty() {
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

fn draw_direction_mark(
    painter: &Painter,
    terminal: Pos2,
    inner: Pos2,
    direction: PortDirection,
    color: Color32,
) {
    if !matches!(direction, PortDirection::In | PortDirection::Out) {
        return;
    }
    let wire_dir = (terminal - inner).normalized();
    if !wire_dir.is_finite() {
        return;
    }
    let tip = if direction == PortDirection::In {
        inner + wire_dir * 5.0
    } else {
        terminal - wire_dir * 5.0
    };
    let side = vec2(-wire_dir.y, wire_dir.x);
    painter.add(Shape::convex_polygon(
        vec![
            tip,
            tip - wire_dir * 7.0 + side * 3.5,
            tip - wire_dir * 7.0 - side * 3.5,
        ],
        color,
        Stroke::NONE,
    ));
}

fn stub_inner(position: Point) -> Point {
    if position.x.abs() >= position.y.abs() {
        Point::new(
            position.x - SYMBOL_TERMINAL_GRID * position.x.signum(),
            position.y,
        )
    } else {
        Point::new(
            position.x,
            position.y - SYMBOL_TERMINAL_GRID * position.y.signum(),
        )
    }
}

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
        let offset = symbol.effective_pin_offset(pin);
        include(offset);
        include(stub_inner(offset));
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
        assert!(
            svg.contains(r#"<line class="component" x1="150" y1="50" x2="140" y2="50"/>"#),
            "svg must place authored pin stubs relative to the symbol origin: {svg}"
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
