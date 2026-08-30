//! SVG bodies for hierarchical blocks and ports.

use std::fmt::Write;

use crate::schematic::symbols::{PathCommand, Symbol};
use crate::state::Component;

use super::SvgExportConfig;

/// Bounds of the exact canonical target box after the same instance transform
/// used by [`write_catalog_asset_symbol`], in schematic coordinates.
pub(super) fn catalog_asset_world_bounds(
    component: &Component,
    symbol: &Symbol,
    target_width: f32,
    target_height: f32,
    rotation_degrees: i32,
) -> Option<(f64, f64, f64, f64)> {
    let (min_x, min_y, max_x, max_y) = symbol.bounds;
    let mut bounds = (f64::MAX, f64::MAX, f64::MIN, f64::MIN);
    for (x, y) in [
        (min_x, min_y),
        (max_x, min_y),
        (max_x, max_y),
        (min_x, max_y),
    ] {
        let (x, y) = catalog_asset_local_point(
            component,
            symbol,
            target_width,
            target_height,
            rotation_degrees,
            x,
            y,
        );
        let x = x + f64::from(component.pos.x);
        let y = y + f64::from(component.pos.y);
        bounds.0 = bounds.0.min(x);
        bounds.1 = bounds.1.min(y);
        bounds.2 = bounds.2.max(x);
        bounds.3 = bounds.3.max(y);
    }
    [bounds.0, bounds.1, bounds.2, bounds.3]
        .into_iter()
        .all(f64::is_finite)
        .then_some(bounds)
}

fn catalog_asset_local_point(
    component: &Component,
    symbol: &Symbol,
    target_width: f32,
    target_height: f32,
    rotation_degrees: i32,
    x: f32,
    y: f32,
) -> (f64, f64) {
    let (symbol_cx, symbol_cy) = symbol.center();
    let scale_x = f64::from(target_width / symbol.width().max(0.001));
    let scale_y = f64::from(target_height / symbol.height().max(0.001));
    let mut x = (f64::from(x) - f64::from(symbol_cx)) * scale_x;
    let mut y = (f64::from(y) - f64::from(symbol_cy)) * scale_y;
    if component.mirror_h {
        x = -x;
    }
    if component.mirror_v {
        y = -y;
    }
    let radians = f64::from(rotation_degrees).to_radians();
    let (sine, cosine) = radians.sin_cos();
    (x * cosine - y * sine, x * sine + y * cosine)
}

/// Carry artwork leads out to terminals the drawing itself does not reach.
pub(super) fn write_artwork_lead_extensions(
    svg: &mut String,
    component: &Component,
    config: &SvgExportConfig,
) {
    let local = instance_projection(component, config);
    for (edge, terminal) in component.artwork_lead_extensions() {
        let edge = local(edge);
        let terminal = local(terminal);
        writeln!(
            svg,
            r#"<line class="component" x1="{}" y1="{}" x2="{}" y2="{}"/>"#,
            edge.0, edge.1, terminal.0, terminal.1
        )
        .unwrap();
    }
}

/// Symbol-local point to page coordinates for one placed instance.
fn instance_projection<'a>(
    component: &'a Component,
    config: &'a SvgExportConfig,
) -> impl Fn(crate::state::Point) -> (f64, f64) + 'a {
    let cx = component.pos.x as f64 * config.grid_size;
    let cy = component.pos.y as f64 * config.grid_size;
    move |point| {
        let transformed = component.transform_point(point);
        (
            cx + f64::from(transformed.x) * config.grid_size,
            cy + f64::from(transformed.y) * config.grid_size,
        )
    }
}

pub(super) fn write_catalog_asset_symbol(
    svg: &mut String,
    component: &Component,
    symbol: &Symbol,
    target_width: f32,
    target_height: f32,
    rotation_degrees: i32,
    stroke_width: Option<f64>,
    config: &SvgExportConfig,
) {
    let cx = component.pos.x as f64 * config.grid_size;
    let cy = component.pos.y as f64 * config.grid_size;
    let point = |x: f32, y: f32| {
        let (x, y) = catalog_asset_local_point(
            component,
            symbol,
            target_width,
            target_height,
            rotation_degrees,
            x,
            y,
        );
        (cx + x * config.grid_size, cy + y * config.grid_size)
    };
    for path in &symbol.paths {
        let mut data = String::new();
        for command in &path.commands {
            match command {
                PathCommand::MoveTo(x, y) => {
                    let (x, y) = point(*x, *y);
                    write!(data, "M {x} {y} ").unwrap();
                }
                PathCommand::LineTo(x, y) => {
                    let (x, y) = point(*x, *y);
                    write!(data, "L {x} {y} ").unwrap();
                }
                PathCommand::CurveTo { ctrl1, ctrl2, end } => {
                    let (x1, y1) = point(ctrl1.0, ctrl1.1);
                    let (x2, y2) = point(ctrl2.0, ctrl2.1);
                    let (x, y) = point(end.0, end.1);
                    write!(data, "C {x1} {y1} {x2} {y2} {x} {y} ").unwrap();
                }
                PathCommand::Close => data.push_str("Z "),
            }
        }
        let style = stroke_width.map_or_else(String::new, |width| {
            format!(" style=\"stroke-width:{width}\"")
        });
        writeln!(
            svg,
            r#"<path class="component" d="{}" fill="{}"{style}/>"#,
            data.trim(),
            if path.filled {
                config.component_color.to_string()
            } else {
                "none".to_owned()
            },
        )
        .unwrap();
    }
}
