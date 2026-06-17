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
    let points = local_symbol_extent_points(symbol);
    if points.is_empty() {
        return None;
    }

    let mut min_x = i32::MAX;
    let mut min_y = i32::MAX;
    let mut max_x = i32::MIN;
    let mut max_y = i32::MIN;
    for point in points {
        let world = component.pos + component.transform_point(point);
        min_x = min_x.min(world.x);
        min_y = min_y.min(world.y);
        max_x = max_x.max(world.x);
        max_y = max_y.max(world.y);
    }
    Some((Point::new(min_x, min_y), Point::new(max_x, max_y)))
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
                    .map(|point| to_screen(origin, scale, component, *point))
                    .collect();
                for pair in screen_points.windows(2) {
                    painter.line_segment([pair[0], pair[1]], stroke);
                }
                if *closed && screen_points.len() > 2 {
                    painter.line_segment(
                        [*screen_points.last().expect("last point"), screen_points[0]],
                        stroke,
                    );
                }
            }
            SymbolShape::Circle { center, radius } => {
                painter.circle_stroke(
                    to_screen(origin, scale, component, *center),
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
                        .map(|point| to_screen(origin, scale, component, point))
                        .collect();
                painter.add(Shape::line(points, stroke));
            }
            SymbolShape::Arrow {
                tip,
                rotation_quarters,
            } => {
                let points: Vec<Pos2> = arrow_points(*tip, *rotation_quarters)
                    .into_iter()
                    .map(|point| to_screen(origin, scale, component, point))
                    .collect();
                painter.add(Shape::convex_polygon(points, stroke.color, Stroke::NONE));
            }
            SymbolShape::Dot { center, radius } => {
                painter.circle_filled(
                    to_screen(origin, scale, component, *center),
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
        let terminal = to_screen(origin, scale, component, pin.offset);
        let inner = to_screen(origin, scale, component, stub_inner(pin.offset));
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
            to_screen(origin, scale, component, symbol.document().name_anchor),
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
            to_screen(origin, scale, component, symbol.document().value_anchor),
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
                    let (x, y) = to_svg(component, *point, config);
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
                let (cx, cy) = to_svg(component, *center, config);
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
                    let (x, y) = to_svg(component, point, config);
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
                    &arrow_points(*tip, *rotation_quarters),
                    config,
                );
            }
            SymbolShape::Dot { center, radius } => {
                let (cx, cy) = to_svg(component, *center, config);
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
        let (x1, y1) = to_svg(component, pin.offset, config);
        let (x2, y2) = to_svg(component, stub_inner(pin.offset), config);
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
        let (tx, ty) = to_svg(component, stub_inner(pin.offset), config);
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
        let (x, y) = to_svg(component, symbol.document().name_anchor, config);
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
        let (x, y) = to_svg(component, symbol.document().value_anchor, config);
        write_text_svg(svg, x, y, value);
    }
}

fn write_polygon_svg(
    svg: &mut String,
    component: &Component,
    points: &[Point],
    config: &SvgExportConfig,
) {
    svg.push_str("<polygon points=\"");
    for (index, point) in points.iter().enumerate() {
        let (x, y) = to_svg(component, *point, config);
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

fn to_svg(component: &Component, point: Point, config: &SvgExportConfig) -> (f64, f64) {
    let transformed = component.pos + component.transform_point(point);
    (
        transformed.x as f64 * config.grid_size,
        transformed.y as f64 * config.grid_size,
    )
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

fn local_symbol_extent_points(symbol: &ResolvedCellSymbol) -> Vec<Point> {
    let mut points = vec![
        symbol.document().origin,
        symbol.document().name_anchor,
        symbol.document().value_anchor,
    ];
    for pin in symbol.connectable_pins() {
        points.push(pin.offset);
        points.push(stub_inner(pin.offset));
    }
    for shape in &symbol.document().body {
        match shape {
            SymbolShape::Polyline {
                points: shape_points,
                ..
            } => points.extend(shape_points.iter().copied()),
            SymbolShape::Circle { center, radius } | SymbolShape::Dot { center, radius } => {
                points.extend(cardinal_bounds(*center, *radius));
            }
            SymbolShape::Arc { center, radius, .. } => {
                points.extend(cardinal_bounds(*center, *radius));
            }
            SymbolShape::Arrow {
                tip,
                rotation_quarters,
            } => points.extend(arrow_points(*tip, *rotation_quarters)),
        }
    }
    points
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
