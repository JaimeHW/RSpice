//! SVG bodies for hierarchical blocks and ports.

use std::fmt::Write;

use crate::state::{ComponentType, PortDirection};

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
    cx: f64,
    cy: f64,
    config: &SvgExportConfig,
) {
    write_block_symbol(svg, cx, cy, "X", config);
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
