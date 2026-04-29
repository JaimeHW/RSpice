use std::fmt::Write;

use crate::state::ComponentType;

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
