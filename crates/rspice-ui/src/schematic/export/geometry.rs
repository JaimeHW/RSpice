use std::fmt::Write;

use crate::state::{Rotation, SchematicState, Wire};

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

    if min_x == f64::MAX {
        (0.0, 0.0, 100.0, 100.0)
    } else {
        (min_x, min_y, max_x, max_y)
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

pub(super) fn get_rotation_transform(rotation: Rotation, cx: f64, cy: f64) -> String {
    match rotation {
        Rotation::R0 => String::new(),
        Rotation::R90 => format!("rotate(90, {}, {})", cx, cy),
        Rotation::R180 => format!("rotate(180, {}, {})", cx, cy),
        Rotation::R270 => format!("rotate(270, {}, {})", cx, cy),
    }
}
