//! Schematic Export
//!
//! Export schematic diagrams to SVG format for documentation and printing.
//! Produces clean, scalable vector graphics suitable for professional
//! circuit documentation.
//!
//! Matches Cadence Virtuoso export quality for commercial-grade output.

use std::fmt::Write;

use crate::state::{Component, ComponentType, SchematicState};

mod bjt_diode_symbols;
mod block_symbols;
mod config;
mod controlled_symbols;
mod geometry;
mod jfet_symbols;
mod mos_symbols;
mod passive_symbols;
mod source_symbols;

pub use self::config::SvgExportConfig;

use self::bjt_diode_symbols::{write_diode_symbol, write_npn_symbol, write_pnp_symbol};
use self::block_symbols::{write_cell_instance_symbol, write_xspice_symbol};
use self::controlled_symbols::{
    write_cccs_symbol, write_ccvs_symbol, write_opamp_symbol, write_vccs_symbol,
    write_vcvs_symbol, write_vswitch_symbol,
};
use self::geometry::{calculate_bounds, get_rotation_transform, write_wire};
use self::jfet_symbols::{write_njfet_symbol, write_pjfet_symbol};
use self::mos_symbols::{write_nmos_symbol, write_pmos_symbol};
use self::passive_symbols::{
    write_capacitor_symbol, write_coupled_inductor_symbol, write_inductor_symbol,
    write_resistor_symbol, write_tline_symbol, write_transformer_symbol,
};
use self::source_symbols::{
    write_behavioral_source_symbol, write_current_source_symbol, write_ground_symbol,
    write_vsource_symbol,
};

/// Export a schematic to SVG format
pub fn export_to_svg(state: &SchematicState, config: &SvgExportConfig) -> String {
    let mut svg = String::new();

    // Calculate bounds
    let (min_x, min_y, max_x, max_y) = calculate_bounds(state, config);
    let width = max_x - min_x + 2.0 * config.margin;
    let height = max_y - min_y + 2.0 * config.margin;

    // SVG header
    let _ = writeln!(
        svg,
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n\
<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{width}\" height=\"{height}\" viewBox=\"{vx} {vy} {width} {height}\">\n\
<style>\n\
  .wire {{ stroke: {wire_color}; stroke-width: {wire_width}; fill: none; stroke-linecap: round; stroke-linejoin: round; }}\n\
  .component {{ stroke: {comp_color}; stroke-width: {comp_width}; fill: none; }}\n\
  .text {{ font-family: monospace; font-size: {font_size}px; fill: {text_color}; }}\n\
</style>\n\
<rect width=\"100%\" height=\"100%\" fill=\"#1a1a1a\"/>",
        vx = min_x - config.margin,
        vy = min_y - config.margin,
        wire_color = config.wire_color,
        wire_width = config.wire_stroke_width,
        comp_color = config.component_color,
        comp_width = config.component_stroke_width,
        font_size = config.font_size,
        text_color = config.text_color
    );

    // Export wires
    for wire in &state.wires {
        write_wire(&mut svg, wire, config);
    }

    // Export components
    for component in &state.components {
        write_component(&mut svg, component, config);
    }

    // Close SVG
    svg.push_str("</svg>");

    svg
}
fn write_component(svg: &mut String, component: &Component, config: &SvgExportConfig) {
    let cx = component.pos.x as f64 * config.grid_size;
    let cy = component.pos.y as f64 * config.grid_size;
    let transform = get_rotation_transform(component.rotation, cx, cy);

    writeln!(svg, "<g transform=\"{}\">", transform).unwrap();

    match component.kind {
        ComponentType::Resistor => write_resistor_symbol(svg, cx, cy, config),
        ComponentType::Capacitor => write_capacitor_symbol(svg, cx, cy, config),
        ComponentType::Inductor | ComponentType::SaturableInductor => {
            write_inductor_symbol(svg, cx, cy, config)
        }
        ComponentType::Transformer => write_transformer_symbol(svg, cx, cy, config),
        ComponentType::CoupledInductor => write_coupled_inductor_symbol(svg, cx, cy, config),
        ComponentType::TransmissionLine => write_tline_symbol(svg, cx, cy, config),
        ComponentType::VoltageSource
        | ComponentType::VoltageSourceAc
        | ComponentType::VoltageSourcePulse
        | ComponentType::VoltageSourceSin
        | ComponentType::VoltageSourcePwl
        | ComponentType::VoltageSourceExp
        | ComponentType::VoltageSourceSffm => write_vsource_symbol(svg, cx, cy, config),
        ComponentType::Ground => write_ground_symbol(svg, cx, cy, config),
        ComponentType::Nmos | ComponentType::NVdmos => write_nmos_symbol(svg, cx, cy, config),
        ComponentType::Pmos | ComponentType::PVdmos => write_pmos_symbol(svg, cx, cy, config),
        ComponentType::NpnBjt => write_npn_symbol(svg, cx, cy, config),
        ComponentType::PnpBjt => write_pnp_symbol(svg, cx, cy, config),
        ComponentType::Diode => write_diode_symbol(svg, cx, cy, config),
        ComponentType::CurrentSource
        | ComponentType::CurrentSourceAc
        | ComponentType::CurrentSourcePulse
        | ComponentType::CurrentSourceSin
        | ComponentType::CurrentSourcePwl
        | ComponentType::CurrentSourceExp
        | ComponentType::CurrentSourceNoise => write_current_source_symbol(svg, cx, cy, config),
        ComponentType::BehavioralSource => write_behavioral_source_symbol(svg, cx, cy, config),
        // JFET symbols
        ComponentType::Njfet => write_njfet_symbol(svg, cx, cy, config),
        ComponentType::Pjfet => write_pjfet_symbol(svg, cx, cy, config),
        // Controlled source symbols
        ComponentType::OpAmp => write_opamp_symbol(svg, cx, cy, config),
        ComponentType::Vcvs => write_vcvs_symbol(svg, cx, cy, config),
        ComponentType::Vccs => write_vccs_symbol(svg, cx, cy, config),
        ComponentType::Ccvs => write_ccvs_symbol(svg, cx, cy, config),
        ComponentType::Cccs => write_cccs_symbol(svg, cx, cy, config),
        ComponentType::VSwitch => write_vswitch_symbol(svg, cx, cy, config),
        ComponentType::CellInstance => write_cell_instance_symbol(svg, cx, cy, config),
        ComponentType::XspiceGain
        | ComponentType::XspiceSummer
        | ComponentType::XspiceMultiplier
        | ComponentType::XspiceDivider
        | ComponentType::XspiceLimiter
        | ComponentType::XspiceIntegrator
        | ComponentType::XspiceDifferentiator
        | ComponentType::XspiceInverter
        | ComponentType::XspiceBuffer
        | ComponentType::XspiceAndGate
        | ComponentType::XspiceOrGate
        | ComponentType::XspiceNandGate
        | ComponentType::XspiceNorGate
        | ComponentType::XspiceXorGate
        | ComponentType::XspiceTristate
        | ComponentType::XspiceDFlipFlop
        | ComponentType::XspiceJkFlipFlop
        | ComponentType::XspiceSrLatch
        | ComponentType::XspiceAdcBridge
        | ComponentType::XspiceDacBridge => {
            write_xspice_symbol(svg, cx, cy, component.kind, config)
        }
    }

    // Component label
    writeln!(
        svg,
        "<text class=\"text\" x=\"{}\" y=\"{}\">{}</text>",
        cx,
        cy - 25.0,
        component.name
    )
    .unwrap();

    if !component.value.is_empty() {
        writeln!(
            svg,
            "<text class=\"text\" x=\"{}\" y=\"{}\">{}</text>",
            cx,
            cy + 35.0,
            component.value
        )
        .unwrap();
    }

    svg.push_str("</g>\n");
}
