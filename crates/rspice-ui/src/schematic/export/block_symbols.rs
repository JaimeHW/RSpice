//! SVG bodies for hierarchical blocks and ports.

use std::fmt::Write;

use crate::schematic::symbols::{PathCommand, Symbol};
use crate::state::{Component, ComponentType, PortDirection};

use super::SvgExportConfig;

pub(super) fn write_block_symbol(
    svg: &mut String,
    cx: f64,
    cy: f64,
    symbol: &str,
    _config: &SvgExportConfig,
) {
    writeln!(
        svg,
        r#"<rect class="component" x="{}" y="{}" width="36" height="26" rx="2" ry="2"/>"#,
        cx - 18.0,
        cy - 13.0
    )
    .unwrap();
    writeln!(
        svg,
        r#"<text class="text" x="{}" y="{}" text-anchor="middle">{}</text>"#,
        cx,
        cy + 4.0,
        symbol
    )
    .unwrap();
}

pub(super) fn write_cell_instance_symbol(
    svg: &mut String,
    component: &Component,
    config: &SvgExportConfig,
) {
    let cx = component.pos.x as f64 * config.grid_size;
    let cy = component.pos.y as f64 * config.grid_size;
    let (_, height) = component.symbol_dimensions();
    let half_body_height = (height / 2 - 5).max(15);
    let local = |point: crate::state::Point| {
        let x = if component.mirror_h {
            -point.x
        } else {
            point.x
        };
        let y = if component.mirror_v {
            -point.y
        } else {
            point.y
        };
        (
            cx + f64::from(x) * config.grid_size,
            cy + f64::from(y) * config.grid_size,
        )
    };
    let corners = [
        local(crate::state::Point::new(-20, -half_body_height)),
        local(crate::state::Point::new(20, -half_body_height)),
        local(crate::state::Point::new(20, half_body_height)),
        local(crate::state::Point::new(-20, half_body_height)),
    ];
    writeln!(
        svg,
        r#"<polygon class="component" points="{},{} {},{} {},{} {},{}"/>"#,
        corners[0].0,
        corners[0].1,
        corners[1].0,
        corners[1].1,
        corners[2].0,
        corners[2].1,
        corners[3].0,
        corners[3].1,
    )
    .unwrap();
    for (name, terminal) in component.instance_pin_layout() {
        let inner = if terminal.y.abs() > half_body_height {
            crate::state::Point::new(terminal.x, terminal.y.signum() * half_body_height)
        } else {
            crate::state::Point::new(terminal.x.signum() * 20, terminal.y)
        };
        let terminal = local(terminal);
        let inner = local(inner);
        writeln!(
            svg,
            r#"<line class="component" x1="{}" y1="{}" x2="{}" y2="{}"/>"#,
            terminal.0, terminal.1, inner.0, inner.1
        )
        .unwrap();
        writeln!(
            svg,
            r#"<circle class="component" fill="{}" cx="{}" cy="{}" r="{}"/>"#,
            config.component_color,
            terminal.0,
            terminal.1,
            1.6 * config.grid_size,
        )
        .unwrap();
        if let Some(name) = name {
            writeln!(
                svg,
                r#"<text class="text" x="{}" y="{}" text-anchor="middle">{}</text>"#,
                inner.0,
                inner.1,
                super::escape_xml(&name),
            )
            .unwrap();
        }
    }
}

pub(super) fn write_catalog_asset_symbol(
    svg: &mut String,
    component: &Component,
    symbol: &Symbol,
    target_width: f32,
    target_height: f32,
    config: &SvgExportConfig,
) {
    let cx = component.pos.x as f64 * config.grid_size;
    let cy = component.pos.y as f64 * config.grid_size;
    let (symbol_cx, symbol_cy) = symbol.center();
    let scale_x = f64::from(target_width / symbol.width().max(0.001)) * config.grid_size;
    let scale_y = f64::from(target_height / symbol.height().max(0.001)) * config.grid_size;
    let point = |x: f32, y: f32| {
        let mut x = (f64::from(x) - f64::from(symbol_cx)) * scale_x;
        let mut y = (f64::from(y) - f64::from(symbol_cy)) * scale_y;
        if component.mirror_h {
            x = -x;
        }
        if component.mirror_v {
            y = -y;
        }
        (cx + x, cy + y)
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
        writeln!(
            svg,
            r#"<path class="component" d="{}" fill="{}"/>"#,
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

/// Interface port: a flag whose tip is the attachment point at (-10, 0).
pub(super) fn write_port_symbol(
    svg: &mut String,
    cx: f64,
    cy: f64,
    direction: PortDirection,
    mirror_h: bool,
    mirror_v: bool,
    config: &SvgExportConfig,
) {
    let local = |dx: f64, dy: f64| {
        (
            cx + if mirror_h { -dx } else { dx },
            cy + if mirror_v { -dy } else { dy },
        )
    };
    let tip = local(-10.0, 0.0);
    let upper_left = local(-4.0, -6.0);
    let upper_right = local(10.0, -6.0);
    let lower_right = local(10.0, 6.0);
    let lower_left = local(-4.0, 6.0);
    writeln!(
        svg,
        r#"<polygon class="component" fill="none" points="{},{} {},{} {},{} {},{} {},{}"/>"#,
        tip.0,
        tip.1,
        upper_left.0,
        upper_left.1,
        upper_right.0,
        upper_right.1,
        lower_right.0,
        lower_right.1,
        lower_left.0,
        lower_left.1
    )
    .unwrap();
    writeln!(
        svg,
        r#"<circle class="component" fill="{}" cx="{}" cy="{}" r="1.6"/>"#,
        config.component_color, tip.0, tip.1
    )
    .unwrap();
    let path = match direction {
        PortDirection::In => "M -1.5 0 L 5 0 M 5 0 L 2 -2.5 M 5 0 L 2 2.5",
        PortDirection::Out => "M 5 0 L -1.5 0 M -1.5 0 L 1.5 -2.5 M -1.5 0 L 1.5 2.5",
        PortDirection::InOut => {
            "M -1.5 0 L 5 0 M 5 0 L 2 -2.5 M 5 0 L 2 2.5 M -1.5 0 L 1.5 -2.5 M -1.5 0 L 1.5 2.5"
        }
        PortDirection::Supply => "M 2 -3 L 2 3 M -1 -3 L 5 -3",
    };
    writeln!(
        svg,
        r#"<path class="component" fill="none" transform="translate({cx} {cy}) scale({} {})" d="{path}"/>"#,
        if mirror_h { -1 } else { 1 },
        if mirror_v { -1 } else { 1 }
    )
    .unwrap();
}

pub(super) fn write_xspice_symbol(
    svg: &mut String,
    cx: f64,
    cy: f64,
    kind: ComponentType,
    config: &SvgExportConfig,
) {
    let glyph = match kind {
        ComponentType::XspiceGain => "G",
        ComponentType::XspiceSummer => "SUM",
        ComponentType::XspiceMultiplier => "MUL",
        ComponentType::XspiceDivider => "DIV",
        ComponentType::XspiceLimiter => "LIM",
        ComponentType::XspiceIntegrator => "INT",
        ComponentType::XspiceDifferentiator => "DIF",
        ComponentType::XspiceInverter => "INV",
        ComponentType::XspiceBuffer => "BUF",
        ComponentType::XspiceAndGate => "AND",
        ComponentType::XspiceOrGate => "OR",
        ComponentType::XspiceNandGate => "NAND",
        ComponentType::XspiceNorGate => "NOR",
        ComponentType::XspiceXorGate => "XOR",
        ComponentType::XspiceTristate => "TRI",
        ComponentType::XspiceDFlipFlop => "DFF",
        ComponentType::XspiceJkFlipFlop => "JK",
        ComponentType::XspiceSrLatch => "SR",
        ComponentType::XspiceAdcBridge => "ADC",
        ComponentType::XspiceDacBridge => "DAC",
        _ => "A",
    };
    write_block_symbol(svg, cx, cy, glyph, config);
}
