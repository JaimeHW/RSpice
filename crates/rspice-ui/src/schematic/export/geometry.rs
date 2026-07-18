use std::fmt::Write;

use crate::state::{Bus, BusTap, Junction, Rotation, SchematicState, Wire};

use super::SvgExportConfig;

pub(super) fn calculate_bounds(
    state: &SchematicState,
    config: &SvgExportConfig,
) -> (f64, f64, f64, f64) {
    let mut min_x = f64::MAX;
    let mut min_y = f64::MAX;
    let mut max_x = f64::MIN;
    let mut max_y = f64::MIN;

    for comp in &state.components {
        let (comp_min_x, comp_min_y, comp_max_x, comp_max_y) = comp.bounding_box();
        min_x = min_x.min(comp_min_x as f64 * config.grid_size);
        min_y = min_y.min(comp_min_y as f64 * config.grid_size);
        max_x = max_x.max(comp_max_x as f64 * config.grid_size);
        max_y = max_y.max(comp_max_y as f64 * config.grid_size);
    }

    for wire in &state.wires {
        for point in &wire.points {
            let x = point.x as f64 * config.grid_size;
            let y = point.y as f64 * config.grid_size;
            min_x = min_x.min(x);
            min_y = min_y.min(y);
            max_x = max_x.max(x);
            max_y = max_y.max(y);
        }
    }

    include_bus_bounds(
        state, config, &mut min_x, &mut min_y, &mut max_x, &mut max_y,
    );

    include_junction_bounds(
        state, config, &mut min_x, &mut min_y, &mut max_x, &mut max_y,
    );

    if min_x == f64::MAX {
        (0.0, 0.0, 100.0, 100.0)
    } else {
        (min_x, min_y, max_x, max_y)
    }
}

pub(super) fn include_bus_bounds(
    state: &SchematicState,
    config: &SvgExportConfig,
    min_x: &mut f64,
    min_y: &mut f64,
    max_x: &mut f64,
    max_y: &mut f64,
) {
    for bus in &state.buses {
        for point in &bus.points {
            let x = point.x as f64 * config.grid_size;
            let y = point.y as f64 * config.grid_size;
            *min_x = (*min_x).min(x - 4.0);
            *min_y = (*min_y).min(y - 4.0);
            *max_x = (*max_x).max(x + 4.0);
            *max_y = (*max_y).max(y + 4.0);
        }
        if let (Some(declaration), Some(anchor)) = (&bus.declaration, bus.points.last()) {
            let x = anchor.x as f64 * config.grid_size + 8.0;
            let baseline = anchor.y as f64 * config.grid_size - 8.0;
            *min_x = (*min_x).min(x);
            *min_y = (*min_y).min(baseline - config.font_size);
            *max_x = (*max_x).max(x + estimated_text_width(&declaration.to_string(), config));
            *max_y = (*max_y).max(baseline);
        }
    }

    for tap in &state.bus_taps {
        for point in crate::schematic::bus_geometry::bus_tap_route_points(tap) {
            let x = point.x as f64 * config.grid_size;
            let y = point.y as f64 * config.grid_size;
            *min_x = (*min_x).min(x);
            *min_y = (*min_y).min(y);
            *max_x = (*max_x).max(x);
            *max_y = (*max_y).max(y);
        }
        let x = tap.connection_point.x as f64 * config.grid_size + 5.0;
        let baseline = tap.connection_point.y as f64 * config.grid_size - 7.0;
        *min_x = (*min_x).min(x);
        *min_y = (*min_y).min(baseline - config.font_size);
        *max_x = (*max_x).max(x + estimated_text_width(&tap.slice.to_string(), config));
        *max_y = (*max_y).max(baseline);
    }
}

pub(super) fn junction_radius(config: &SvgExportConfig) -> f64 {
    (config.wire_stroke_width * 1.5).max(1.5)
}

pub(super) fn include_junction_bounds(
    state: &SchematicState,
    config: &SvgExportConfig,
    min_x: &mut f64,
    min_y: &mut f64,
    max_x: &mut f64,
    max_y: &mut f64,
) {
    let radius = junction_radius(config);
    for junction in &state.junctions {
        let x = junction.pos.x as f64 * config.grid_size;
        let y = junction.pos.y as f64 * config.grid_size;
        *min_x = (*min_x).min(x - radius);
        *min_y = (*min_y).min(y - radius);
        *max_x = (*max_x).max(x + radius);
        *max_y = (*max_y).max(y + radius);
    }
}

pub(super) fn write_wire(svg: &mut String, wire: &Wire, config: &SvgExportConfig) {
    if wire.points.len() < 2 {
        return;
    }

    svg.push_str("<path class=\"wire\" d=\"");
    for (i, point) in wire.points.iter().enumerate() {
        let x = point.x as f64 * config.grid_size;
        let y = point.y as f64 * config.grid_size;
        if i == 0 {
            write!(svg, "M {} {}", x, y).unwrap();
        } else {
            write!(svg, " L {} {}", x, y).unwrap();
        }
    }
    svg.push_str("\"/>\n");
}

pub(super) fn write_bus(svg: &mut String, bus: &Bus, config: &SvgExportConfig) {
    if bus.points.len() < 2 {
        return;
    }

    let centerline: Vec<(f64, f64)> = bus
        .points
        .iter()
        .map(|point| {
            (
                point.x as f64 * config.grid_size,
                point.y as f64 * config.grid_size,
            )
        })
        .collect();
    for offset in [-4.0, 0.0, 4.0] {
        write_svg_path(svg, "bus", &offset_svg_polyline(&centerline, offset));
    }
    if let (Some(declaration), Some(&(x, y))) = (&bus.declaration, centerline.last()) {
        writeln!(
            svg,
            "<text class=\"text bus-declaration\" x=\"{}\" y=\"{}\">{}</text>",
            x + 8.0,
            y - 8.0,
            super::escape_xml(&declaration.to_string())
        )
        .unwrap();
    }
}

pub(super) fn write_bus_tap(svg: &mut String, tap: &BusTap, config: &SvgExportConfig) {
    let x2 = tap.connection_point.x as f64 * config.grid_size;
    let y2 = tap.connection_point.y as f64 * config.grid_size;
    let route: Vec<(f64, f64)> = crate::schematic::bus_geometry::bus_tap_route_points(tap)
        .into_iter()
        .map(|point| {
            (
                point.x as f64 * config.grid_size,
                point.y as f64 * config.grid_size,
            )
        })
        .collect();
    write_svg_path(svg, "bus-tap", &route);
    writeln!(
        svg,
        "<text class=\"text bus-tap-label\" x=\"{}\" y=\"{}\">{}</text>",
        x2 + 5.0,
        y2 - 7.0,
        super::escape_xml(&tap.slice.to_string())
    )
    .unwrap();
}

fn write_svg_path(svg: &mut String, class: &str, points: &[(f64, f64)]) {
    if points.len() < 2 {
        return;
    }
    write!(svg, "<path class=\"{class}\" d=\"").unwrap();
    for (index, &(x, y)) in points.iter().enumerate() {
        if index == 0 {
            write!(svg, "M {x} {y}").unwrap();
        } else {
            write!(svg, " L {x} {y}").unwrap();
        }
    }
    svg.push_str("\"/>\n");
}

fn offset_svg_polyline(points: &[(f64, f64)], offset: f64) -> Vec<(f64, f64)> {
    if points.len() < 2 || offset == 0.0 {
        return points.to_vec();
    }
    let normal = |start: (f64, f64), end: (f64, f64)| {
        let (dx, dy) = (end.0 - start.0, end.1 - start.1);
        let length = (dx * dx + dy * dy).sqrt().max(f64::EPSILON);
        (-dy / length, dx / length)
    };
    points
        .iter()
        .enumerate()
        .map(|(index, &point)| {
            let vector = if index == 0 {
                normal(points[0], points[1])
            } else if index + 1 == points.len() {
                normal(points[index - 1], points[index])
            } else {
                let previous = normal(points[index - 1], points[index]);
                let next = normal(points[index], points[index + 1]);
                let sum = (previous.0 + next.0, previous.1 + next.1);
                let length = (sum.0 * sum.0 + sum.1 * sum.1).sqrt();
                if length <= f64::EPSILON {
                    next
                } else {
                    let miter = (sum.0 / length, sum.1 / length);
                    let denominator = (miter.0 * next.0 + miter.1 * next.1).abs().max(0.25);
                    (miter.0 / denominator, miter.1 / denominator)
                }
            };
            (point.0 + vector.0 * offset, point.1 + vector.1 * offset)
        })
        .collect()
}

fn estimated_text_width(text: &str, config: &SvgExportConfig) -> f64 {
    text.chars().count() as f64 * config.font_size * 0.62
}

pub(super) fn write_junction(svg: &mut String, junction: &Junction, config: &SvgExportConfig) {
    let cx = junction.pos.x as f64 * config.grid_size;
    let cy = junction.pos.y as f64 * config.grid_size;
    writeln!(
        svg,
        "<circle class=\"junction\" cx=\"{cx}\" cy=\"{cy}\" r=\"{}\"/>",
        junction_radius(config)
    )
    .unwrap();
}

pub(super) fn get_rotation_transform(rotation: Rotation, cx: f64, cy: f64) -> String {
    match rotation {
        Rotation::R0 => String::new(),
        Rotation::R90 => format!("rotate(90, {}, {})", cx, cy),
        Rotation::R180 => format!("rotate(180, {}, {})", cx, cy),
        Rotation::R270 => format!("rotate(270, {}, {})", cx, cy),
    }
}
