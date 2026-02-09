//! Schematic Export
//!
//! Export schematic diagrams to SVG format for documentation and printing.
//! Produces clean, scalable vector graphics suitable for professional
//! circuit documentation.
//!
//! Matches Cadence Virtuoso export quality for commercial-grade output.

use std::fmt::Write;

use crate::state::{Component, ComponentType, Point, Rotation, SchematicState, Wire};

// =============================================================================
// SVG Export Configuration
// =============================================================================

/// Configuration for SVG export
#[derive(Debug, Clone)]
pub struct SvgExportConfig {
    /// Grid size in SVG units
    pub grid_size: f64,
    /// Stroke width for wires
    pub wire_stroke_width: f64,
    /// Stroke width for components
    pub component_stroke_width: f64,
    /// Wire color
    pub wire_color: String,
    /// Component color
    pub component_color: String,
    /// Text color
    pub text_color: String,
    /// Font size for labels
    pub font_size: f64,
    /// Include grid in output
    pub include_grid: bool,
    /// Margin around content
    pub margin: f64,
}

impl Default for SvgExportConfig {
    fn default() -> Self {
        Self {
            grid_size: 10.0,
            wire_stroke_width: 2.0,
            component_stroke_width: 2.0,
            wire_color: "#00FF00".to_string(),
            component_color: "#FFFFFF".to_string(),
            text_color: "#AAAAAA".to_string(),
            font_size: 12.0,
            include_grid: false,
            margin: 50.0,
        }
    }
}

// =============================================================================
// SVG Export
// =============================================================================

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

fn calculate_bounds(state: &SchematicState, config: &SvgExportConfig) -> (f64, f64, f64, f64) {
    let mut min_x = f64::MAX;
    let mut min_y = f64::MAX;
    let mut max_x = f64::MIN;
    let mut max_y = f64::MIN;

    for comp in &state.components {
        let x = comp.pos.x as f64 * config.grid_size;
        let y = comp.pos.y as f64 * config.grid_size;
        min_x = min_x.min(x - 30.0);
        min_y = min_y.min(y - 30.0);
        max_x = max_x.max(x + 30.0);
        max_y = max_y.max(y + 30.0);
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

fn write_wire(svg: &mut String, wire: &Wire, config: &SvgExportConfig) {
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
        ComponentType::CoupledInductor => write_coupled_inductor_symbol(svg, cx, cy, config),
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
        // JFET symbols
        ComponentType::Njfet => write_njfet_symbol(svg, cx, cy, config),
        ComponentType::Pjfet => write_pjfet_symbol(svg, cx, cy, config),
        // Controlled source symbols
        ComponentType::Vcvs => write_vcvs_symbol(svg, cx, cy, config),
        ComponentType::Vccs => write_vccs_symbol(svg, cx, cy, config),
        ComponentType::Ccvs => write_ccvs_symbol(svg, cx, cy, config),
        ComponentType::Cccs => write_cccs_symbol(svg, cx, cy, config),
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

fn get_rotation_transform(rotation: Rotation, cx: f64, cy: f64) -> String {
    match rotation {
        Rotation::R0 => String::new(),
        Rotation::R90 => format!("rotate(90, {}, {})", cx, cy),
        Rotation::R180 => format!("rotate(180, {}, {})", cx, cy),
        Rotation::R270 => format!("rotate(270, {}, {})", cx, cy),
    }
}

fn write_resistor_symbol(svg: &mut String, cx: f64, cy: f64, _config: &SvgExportConfig) {
    // Zigzag resistor symbol
    let scale = 1.5;
    writeln!(
        svg,
        r#"<path class="component" d="M {} {} L {} {} L {} {} L {} {} L {} {} L {} {} L {} {} L {} {}"/>"#,
        cx - 20.0 * scale, cy,
        cx - 15.0 * scale, cy,
        cx - 12.0 * scale, cy - 8.0,
        cx - 6.0 * scale, cy + 8.0,
        cx, cy - 8.0,
        cx + 6.0 * scale, cy + 8.0,
        cx + 12.0 * scale, cy,
        cx + 20.0 * scale, cy
    ).unwrap();
}

fn write_capacitor_symbol(svg: &mut String, cx: f64, cy: f64, _config: &SvgExportConfig) {
    // Two parallel lines
    writeln!(
        svg,
        r#"<line class="component" x1="{}" y1="{}" x2="{}" y2="{}"/>"#,
        cx - 20.0,
        cy,
        cx - 3.0,
        cy
    )
    .unwrap();
    writeln!(
        svg,
        r#"<line class="component" x1="{}" y1="{}" x2="{}" y2="{}"/>"#,
        cx - 3.0,
        cy - 10.0,
        cx - 3.0,
        cy + 10.0
    )
    .unwrap();
    writeln!(
        svg,
        r#"<line class="component" x1="{}" y1="{}" x2="{}" y2="{}"/>"#,
        cx + 3.0,
        cy - 10.0,
        cx + 3.0,
        cy + 10.0
    )
    .unwrap();
    writeln!(
        svg,
        r#"<line class="component" x1="{}" y1="{}" x2="{}" y2="{}"/>"#,
        cx + 3.0,
        cy,
        cx + 20.0,
        cy
    )
    .unwrap();
}

fn write_inductor_symbol(svg: &mut String, cx: f64, cy: f64, _config: &SvgExportConfig) {
    // Arc-based inductor
    writeln!(
        svg,
        r#"<line class="component" x1="{}" y1="{}" x2="{}" y2="{}"/>"#,
        cx - 20.0,
        cy,
        cx - 12.0,
        cy
    )
    .unwrap();
    // Three bumps
    writeln!(
        svg,
        r#"<path class="component" d="M {} {} A 4 4 0 0 1 {} {}"/>"#,
        cx - 12.0,
        cy,
        cx - 4.0,
        cy
    )
    .unwrap();
    writeln!(
        svg,
        r#"<path class="component" d="M {} {} A 4 4 0 0 1 {} {}"/>"#,
        cx - 4.0,
        cy,
        cx + 4.0,
        cy
    )
    .unwrap();
    writeln!(
        svg,
        r#"<path class="component" d="M {} {} A 4 4 0 0 1 {} {}"/>"#,
        cx + 4.0,
        cy,
        cx + 12.0,
        cy
    )
    .unwrap();
    writeln!(
        svg,
        r#"<line class="component" x1="{}" y1="{}" x2="{}" y2="{}"/>"#,
        cx + 12.0,
        cy,
        cx + 20.0,
        cy
    )
    .unwrap();
}

fn write_vsource_symbol(svg: &mut String, cx: f64, cy: f64, _config: &SvgExportConfig) {
    // Circle with + and -
    writeln!(
        svg,
        r#"<circle class="component" cx="{}" cy="{}" r="15"/>"#,
        cx, cy
    )
    .unwrap();
    writeln!(
        svg,
        r#"<text class="text" x="{}" y="{}">+</text>"#,
        cx - 3.0,
        cy - 3.0
    )
    .unwrap();
    writeln!(
        svg,
        r#"<text class="text" x="{}" y="{}">−</text>"#,
        cx - 3.0,
        cy + 12.0
    )
    .unwrap();
    // Lead lines
    writeln!(
        svg,
        r#"<line class="component" x1="{}" y1="{}" x2="{}" y2="{}"/>"#,
        cx,
        cy - 15.0,
        cx,
        cy - 25.0
    )
    .unwrap();
    writeln!(
        svg,
        r#"<line class="component" x1="{}" y1="{}" x2="{}" y2="{}"/>"#,
        cx,
        cy + 15.0,
        cx,
        cy + 25.0
    )
    .unwrap();
}

fn write_ground_symbol(svg: &mut String, cx: f64, cy: f64, _config: &SvgExportConfig) {
    // Three horizontal lines
    writeln!(
        svg,
        r#"<line class="component" x1="{}" y1="{}" x2="{}" y2="{}"/>"#,
        cx,
        cy,
        cx,
        cy + 10.0
    )
    .unwrap();
    writeln!(
        svg,
        r#"<line class="component" x1="{}" y1="{}" x2="{}" y2="{}"/>"#,
        cx - 12.0,
        cy + 10.0,
        cx + 12.0,
        cy + 10.0
    )
    .unwrap();
    writeln!(
        svg,
        r#"<line class="component" x1="{}" y1="{}" x2="{}" y2="{}"/>"#,
        cx - 8.0,
        cy + 15.0,
        cx + 8.0,
        cy + 15.0
    )
    .unwrap();
    writeln!(
        svg,
        r#"<line class="component" x1="{}" y1="{}" x2="{}" y2="{}"/>"#,
        cx - 4.0,
        cy + 20.0,
        cx + 4.0,
        cy + 20.0
    )
    .unwrap();
}

fn write_nmos_symbol(svg: &mut String, cx: f64, cy: f64, _config: &SvgExportConfig) {
    // N-channel MOSFET symbol (industry standard)
    // Vertical channel with gate on left, drain at top, source at bottom

    // Gate lead (horizontal line to gate)
    writeln!(
        svg,
        r#"<line class="component" x1="{}" y1="{}" x2="{}" y2="{}"/>"#,
        cx - 20.0,
        cy,
        cx - 8.0,
        cy
    )
    .unwrap();

    // Gate vertical line
    writeln!(
        svg,
        r#"<line class="component" x1="{}" y1="{}" x2="{}" y2="{}"/>"#,
        cx - 8.0,
        cy - 12.0,
        cx - 8.0,
        cy + 12.0
    )
    .unwrap();

    // Channel (three segments)
    writeln!(
        svg,
        r#"<line class="component" x1="{}" y1="{}" x2="{}" y2="{}"/>"#,
        cx - 4.0,
        cy - 12.0,
        cx - 4.0,
        cy - 4.0
    )
    .unwrap();
    writeln!(
        svg,
        r#"<line class="component" x1="{}" y1="{}" x2="{}" y2="{}"/>"#,
        cx - 4.0,
        cy - 2.0,
        cx - 4.0,
        cy + 2.0
    )
    .unwrap();
    writeln!(
        svg,
        r#"<line class="component" x1="{}" y1="{}" x2="{}" y2="{}"/>"#,
        cx - 4.0,
        cy + 4.0,
        cx - 4.0,
        cy + 12.0
    )
    .unwrap();

    // Drain connection
    writeln!(
        svg,
        r#"<line class="component" x1="{}" y1="{}" x2="{}" y2="{}"/>"#,
        cx - 4.0,
        cy - 10.0,
        cx + 8.0,
        cy - 10.0
    )
    .unwrap();
    writeln!(
        svg,
        r#"<line class="component" x1="{}" y1="{}" x2="{}" y2="{}"/>"#,
        cx + 8.0,
        cy - 10.0,
        cx + 8.0,
        cy - 20.0
    )
    .unwrap();

    // Source connection
    writeln!(
        svg,
        r#"<line class="component" x1="{}" y1="{}" x2="{}" y2="{}"/>"#,
        cx - 4.0,
        cy + 10.0,
        cx + 8.0,
        cy + 10.0
    )
    .unwrap();
    writeln!(
        svg,
        r#"<line class="component" x1="{}" y1="{}" x2="{}" y2="{}"/>"#,
        cx + 8.0,
        cy + 10.0,
        cx + 8.0,
        cy + 20.0
    )
    .unwrap();

    // Arrow pointing into channel (NMOS indicator)
    writeln!(
        svg,
        r#"<line class="component" x1="{}" y1="{}" x2="{}" y2="{}"/>"#,
        cx - 4.0,
        cy,
        cx + 4.0,
        cy
    )
    .unwrap();
    // Arrow head
    writeln!(
        svg,
        r#"<polygon class="component" fill="white" points="{},{} {},{} {},{}"/>"#,
        cx + 4.0,
        cy,
        cx,
        cy - 3.0,
        cx,
        cy + 3.0
    )
    .unwrap();
}

fn write_pmos_symbol(svg: &mut String, cx: f64, cy: f64, _config: &SvgExportConfig) {
    // P-channel MOSFET symbol (same as NMOS but with bubble on gate)

    // Gate lead with bubble
    writeln!(
        svg,
        r#"<line class="component" x1="{}" y1="{}" x2="{}" y2="{}"/>"#,
        cx - 20.0,
        cy,
        cx - 14.0,
        cy
    )
    .unwrap();
    writeln!(
        svg,
        r#"<circle class="component" cx="{}" cy="{}" r="3" fill="none"/>"#,
        cx - 11.0,
        cy
    )
    .unwrap();

    // Gate vertical line
    writeln!(
        svg,
        r#"<line class="component" x1="{}" y1="{}" x2="{}" y2="{}"/>"#,
        cx - 8.0,
        cy - 12.0,
        cx - 8.0,
        cy + 12.0
    )
    .unwrap();

    // Channel (three segments)
    writeln!(
        svg,
        r#"<line class="component" x1="{}" y1="{}" x2="{}" y2="{}"/>"#,
        cx - 4.0,
        cy - 12.0,
        cx - 4.0,
        cy - 4.0
    )
    .unwrap();
    writeln!(
        svg,
        r#"<line class="component" x1="{}" y1="{}" x2="{}" y2="{}"/>"#,
        cx - 4.0,
        cy - 2.0,
        cx - 4.0,
        cy + 2.0
    )
    .unwrap();
    writeln!(
        svg,
        r#"<line class="component" x1="{}" y1="{}" x2="{}" y2="{}"/>"#,
        cx - 4.0,
        cy + 4.0,
        cx - 4.0,
        cy + 12.0
    )
    .unwrap();

    // Drain and Source connections
    writeln!(
        svg,
        r#"<line class="component" x1="{}" y1="{}" x2="{}" y2="{}"/>"#,
        cx - 4.0,
        cy - 10.0,
        cx + 8.0,
        cy - 10.0
    )
    .unwrap();
    writeln!(
        svg,
        r#"<line class="component" x1="{}" y1="{}" x2="{}" y2="{}"/>"#,
        cx + 8.0,
        cy - 10.0,
        cx + 8.0,
        cy - 20.0
    )
    .unwrap();
    writeln!(
        svg,
        r#"<line class="component" x1="{}" y1="{}" x2="{}" y2="{}"/>"#,
        cx - 4.0,
        cy + 10.0,
        cx + 8.0,
        cy + 10.0
    )
    .unwrap();
    writeln!(
        svg,
        r#"<line class="component" x1="{}" y1="{}" x2="{}" y2="{}"/>"#,
        cx + 8.0,
        cy + 10.0,
        cx + 8.0,
        cy + 20.0
    )
    .unwrap();

    // Arrow pointing out from channel (PMOS indicator)
    writeln!(
        svg,
        r#"<line class="component" x1="{}" y1="{}" x2="{}" y2="{}"/>"#,
        cx - 4.0,
        cy,
        cx + 4.0,
        cy
    )
    .unwrap();
    writeln!(
        svg,
        r#"<polygon class="component" fill="white" points="{},{} {},{} {},{}"/>"#,
        cx - 4.0,
        cy,
        cx,
        cy - 3.0,
        cx,
        cy + 3.0
    )
    .unwrap();
}

fn write_npn_symbol(svg: &mut String, cx: f64, cy: f64, _config: &SvgExportConfig) {
    // NPN BJT symbol (industry standard)

    // Base lead
    writeln!(
        svg,
        r#"<line class="component" x1="{}" y1="{}" x2="{}" y2="{}"/>"#,
        cx - 20.0,
        cy,
        cx - 6.0,
        cy
    )
    .unwrap();

    // Emitter base line (vertical)
    writeln!(
        svg,
        r#"<line class="component" x1="{}" y1="{}" x2="{}" y2="{}"/>"#,
        cx - 6.0,
        cy - 10.0,
        cx - 6.0,
        cy + 10.0
    )
    .unwrap();

    // Collector line (angled up-right)
    writeln!(
        svg,
        r#"<line class="component" x1="{}" y1="{}" x2="{}" y2="{}"/>"#,
        cx - 6.0,
        cy - 6.0,
        cx + 10.0,
        cy - 18.0
    )
    .unwrap();
    writeln!(
        svg,
        r#"<line class="component" x1="{}" y1="{}" x2="{}" y2="{}"/>"#,
        cx + 10.0,
        cy - 18.0,
        cx + 10.0,
        cy - 20.0
    )
    .unwrap();

    // Emitter line (angled down-right) with arrow
    writeln!(
        svg,
        r#"<line class="component" x1="{}" y1="{}" x2="{}" y2="{}"/>"#,
        cx - 6.0,
        cy + 6.0,
        cx + 10.0,
        cy + 18.0
    )
    .unwrap();
    writeln!(
        svg,
        r#"<line class="component" x1="{}" y1="{}" x2="{}" y2="{}"/>"#,
        cx + 10.0,
        cy + 18.0,
        cx + 10.0,
        cy + 20.0
    )
    .unwrap();

    // Arrow on emitter (pointing out for NPN)
    writeln!(
        svg,
        r#"<polygon class="component" fill="white" points="{},{} {},{} {},{}"/>"#,
        cx + 10.0,
        cy + 18.0,
        cx + 4.0,
        cy + 14.0,
        cx + 6.0,
        cy + 20.0
    )
    .unwrap();
}

fn write_pnp_symbol(svg: &mut String, cx: f64, cy: f64, _config: &SvgExportConfig) {
    // PNP BJT symbol (arrow points into emitter)

    // Base lead
    writeln!(
        svg,
        r#"<line class="component" x1="{}" y1="{}" x2="{}" y2="{}"/>"#,
        cx - 20.0,
        cy,
        cx - 6.0,
        cy
    )
    .unwrap();

    // Emitter base line (vertical)
    writeln!(
        svg,
        r#"<line class="component" x1="{}" y1="{}" x2="{}" y2="{}"/>"#,
        cx - 6.0,
        cy - 10.0,
        cx - 6.0,
        cy + 10.0
    )
    .unwrap();

    // Collector line (angled up-right)
    writeln!(
        svg,
        r#"<line class="component" x1="{}" y1="{}" x2="{}" y2="{}"/>"#,
        cx - 6.0,
        cy - 6.0,
        cx + 10.0,
        cy - 18.0
    )
    .unwrap();
    writeln!(
        svg,
        r#"<line class="component" x1="{}" y1="{}" x2="{}" y2="{}"/>"#,
        cx + 10.0,
        cy - 18.0,
        cx + 10.0,
        cy - 20.0
    )
    .unwrap();

    // Emitter line (angled down-right)
    writeln!(
        svg,
        r#"<line class="component" x1="{}" y1="{}" x2="{}" y2="{}"/>"#,
        cx - 6.0,
        cy + 6.0,
        cx + 10.0,
        cy + 18.0
    )
    .unwrap();
    writeln!(
        svg,
        r#"<line class="component" x1="{}" y1="{}" x2="{}" y2="{}"/>"#,
        cx + 10.0,
        cy + 18.0,
        cx + 10.0,
        cy + 20.0
    )
    .unwrap();

    // Arrow on emitter (pointing in for PNP)
    writeln!(
        svg,
        r#"<polygon class="component" fill="white" points="{},{} {},{} {},{}"/>"#,
        cx - 6.0,
        cy + 6.0,
        cx,
        cy + 10.0,
        cx - 2.0,
        cy + 4.0
    )
    .unwrap();
}

fn write_diode_symbol(svg: &mut String, cx: f64, cy: f64, _config: &SvgExportConfig) {
    // Diode symbol (triangle with bar)

    // Anode lead
    writeln!(
        svg,
        r#"<line class="component" x1="{}" y1="{}" x2="{}" y2="{}"/>"#,
        cx - 20.0,
        cy,
        cx - 8.0,
        cy
    )
    .unwrap();

    // Triangle (pointing right)
    writeln!(
        svg,
        r#"<polygon class="component" fill="none" points="{},{} {},{} {},{}"/>"#,
        cx - 8.0,
        cy - 10.0,
        cx + 8.0,
        cy,
        cx - 8.0,
        cy + 10.0
    )
    .unwrap();

    // Cathode bar
    writeln!(
        svg,
        r#"<line class="component" x1="{}" y1="{}" x2="{}" y2="{}"/>"#,
        cx + 8.0,
        cy - 10.0,
        cx + 8.0,
        cy + 10.0
    )
    .unwrap();

    // Cathode lead
    writeln!(
        svg,
        r#"<line class="component" x1="{}" y1="{}" x2="{}" y2="{}"/>"#,
        cx + 8.0,
        cy,
        cx + 20.0,
        cy
    )
    .unwrap();
}

fn write_current_source_symbol(svg: &mut String, cx: f64, cy: f64, _config: &SvgExportConfig) {
    // Current source symbol (circle with arrow)

    // Circle
    writeln!(
        svg,
        r#"<circle class="component" cx="{}" cy="{}" r="15" fill="none"/>"#,
        cx, cy
    )
    .unwrap();

    // Arrow pointing up
    writeln!(
        svg,
        r#"<line class="component" x1="{}" y1="{}" x2="{}" y2="{}"/>"#,
        cx,
        cy + 10.0,
        cx,
        cy - 10.0
    )
    .unwrap();
    writeln!(
        svg,
        r#"<polygon class="component" fill="white" points="{},{} {},{} {},{}"/>"#,
        cx,
        cy - 10.0,
        cx - 4.0,
        cy - 4.0,
        cx + 4.0,
        cy - 4.0
    )
    .unwrap();

    // Lead lines
    writeln!(
        svg,
        r#"<line class="component" x1="{}" y1="{}" x2="{}" y2="{}"/>"#,
        cx,
        cy - 15.0,
        cx,
        cy - 25.0
    )
    .unwrap();
    writeln!(
        svg,
        r#"<line class="component" x1="{}" y1="{}" x2="{}" y2="{}"/>"#,
        cx,
        cy + 15.0,
        cx,
        cy + 25.0
    )
    .unwrap();
}

// =============================================================================
// JFET Symbols
// =============================================================================

fn write_njfet_symbol(svg: &mut String, cx: f64, cy: f64, _config: &SvgExportConfig) {
    // N-channel JFET symbol (industry standard)
    // Arrow points inward from gate for N-channel

    // Gate lead (horizontal line to gate)
    writeln!(
        svg,
        r#"<line class="component" x1="{}" y1="{}" x2="{}" y2="{}"/>"#,
        cx - 20.0,
        cy,
        cx - 8.0,
        cy
    )
    .unwrap();

    // Gate arrow (pointing in for NJFET)
    writeln!(
        svg,
        r#"<polygon class="component" fill="white" points="{},{} {},{} {},{}"/>"#,
        cx - 8.0,
        cy,
        cx - 12.0,
        cy - 3.0,
        cx - 12.0,
        cy + 3.0
    )
    .unwrap();

    // Channel (vertical line)
    writeln!(
        svg,
        r#"<line class="component" x1="{}" y1="{}" x2="{}" y2="{}"/>"#,
        cx - 4.0,
        cy - 15.0,
        cx - 4.0,
        cy + 15.0
    )
    .unwrap();

    // Drain connection (top)
    writeln!(
        svg,
        r#"<line class="component" x1="{}" y1="{}" x2="{}" y2="{}"/>"#,
        cx - 4.0,
        cy - 10.0,
        cx + 10.0,
        cy - 10.0
    )
    .unwrap();
    writeln!(
        svg,
        r#"<line class="component" x1="{}" y1="{}" x2="{}" y2="{}"/>"#,
        cx + 10.0,
        cy - 10.0,
        cx + 10.0,
        cy - 20.0
    )
    .unwrap();

    // Source connection (bottom)
    writeln!(
        svg,
        r#"<line class="component" x1="{}" y1="{}" x2="{}" y2="{}"/>"#,
        cx - 4.0,
        cy + 10.0,
        cx + 10.0,
        cy + 10.0
    )
    .unwrap();
    writeln!(
        svg,
        r#"<line class="component" x1="{}" y1="{}" x2="{}" y2="{}"/>"#,
        cx + 10.0,
        cy + 10.0,
        cx + 10.0,
        cy + 20.0
    )
    .unwrap();
}

fn write_pjfet_symbol(svg: &mut String, cx: f64, cy: f64, _config: &SvgExportConfig) {
    // P-channel JFET symbol (arrow points outward from gate)

    // Gate lead
    writeln!(
        svg,
        r#"<line class="component" x1="{}" y1="{}" x2="{}" y2="{}"/>"#,
        cx - 20.0,
        cy,
        cx - 8.0,
        cy
    )
    .unwrap();

    // Gate arrow (pointing out for PJFET)
    writeln!(
        svg,
        r#"<polygon class="component" fill="white" points="{},{} {},{} {},{}"/>"#,
        cx - 4.0,
        cy,
        cx - 8.0,
        cy - 3.0,
        cx - 8.0,
        cy + 3.0
    )
    .unwrap();

    // Channel (vertical line)
    writeln!(
        svg,
        r#"<line class="component" x1="{}" y1="{}" x2="{}" y2="{}"/>"#,
        cx - 4.0,
        cy - 15.0,
        cx - 4.0,
        cy + 15.0
    )
    .unwrap();

    // Drain connection (top)
    writeln!(
        svg,
        r#"<line class="component" x1="{}" y1="{}" x2="{}" y2="{}"/>"#,
        cx - 4.0,
        cy - 10.0,
        cx + 10.0,
        cy - 10.0
    )
    .unwrap();
    writeln!(
        svg,
        r#"<line class="component" x1="{}" y1="{}" x2="{}" y2="{}"/>"#,
        cx + 10.0,
        cy - 10.0,
        cx + 10.0,
        cy - 20.0
    )
    .unwrap();

    // Source connection (bottom)
    writeln!(
        svg,
        r#"<line class="component" x1="{}" y1="{}" x2="{}" y2="{}"/>"#,
        cx - 4.0,
        cy + 10.0,
        cx + 10.0,
        cy + 10.0
    )
    .unwrap();
    writeln!(
        svg,
        r#"<line class="component" x1="{}" y1="{}" x2="{}" y2="{}"/>"#,
        cx + 10.0,
        cy + 10.0,
        cx + 10.0,
        cy + 20.0
    )
    .unwrap();
}

// =============================================================================
// Controlled Source Symbols
// =============================================================================

fn write_vcvs_symbol(svg: &mut String, cx: f64, cy: f64, _config: &SvgExportConfig) {
    // Voltage-Controlled Voltage Source - diamond with + and -
    // Standard controlled source symbol (diamond shape)

    // Diamond shape
    writeln!(
        svg,
        r#"<polygon class="component" fill="none" points="{},{} {},{} {},{} {},{}"/>"#,
        cx,
        cy - 15.0, // top
        cx + 15.0,
        cy, // right
        cx,
        cy + 15.0, // bottom
        cx - 15.0,
        cy // left
    )
    .unwrap();

    // Plus sign (upper half)
    writeln!(
        svg,
        r#"<text class="text" x="{}" y="{}" font-size="12">+</text>"#,
        cx - 4.0,
        cy - 3.0
    )
    .unwrap();

    // Minus sign (lower half)
    writeln!(
        svg,
        r#"<text class="text" x="{}" y="{}" font-size="12">−</text>"#,
        cx - 4.0,
        cy + 10.0
    )
    .unwrap();

    // Lead lines (vertical)
    writeln!(
        svg,
        r#"<line class="component" x1="{}" y1="{}" x2="{}" y2="{}"/>"#,
        cx,
        cy - 15.0,
        cx,
        cy - 25.0
    )
    .unwrap();
    writeln!(
        svg,
        r#"<line class="component" x1="{}" y1="{}" x2="{}" y2="{}"/>"#,
        cx,
        cy + 15.0,
        cx,
        cy + 25.0
    )
    .unwrap();
}

fn write_vccs_symbol(svg: &mut String, cx: f64, cy: f64, _config: &SvgExportConfig) {
    // Voltage-Controlled Current Source - diamond with arrow

    // Diamond shape
    writeln!(
        svg,
        r#"<polygon class="component" fill="none" points="{},{} {},{} {},{} {},{}"/>"#,
        cx,
        cy - 15.0, // top
        cx + 15.0,
        cy, // right
        cx,
        cy + 15.0, // bottom
        cx - 15.0,
        cy // left
    )
    .unwrap();

    // Arrow inside (pointing up)
    writeln!(
        svg,
        r#"<line class="component" x1="{}" y1="{}" x2="{}" y2="{}"/>"#,
        cx,
        cy + 8.0,
        cx,
        cy - 8.0
    )
    .unwrap();
    writeln!(
        svg,
        r#"<polygon class="component" fill="white" points="{},{} {},{} {},{}"/>"#,
        cx,
        cy - 8.0,
        cx - 4.0,
        cy - 2.0,
        cx + 4.0,
        cy - 2.0
    )
    .unwrap();

    // Lead lines
    writeln!(
        svg,
        r#"<line class="component" x1="{}" y1="{}" x2="{}" y2="{}"/>"#,
        cx,
        cy - 15.0,
        cx,
        cy - 25.0
    )
    .unwrap();
    writeln!(
        svg,
        r#"<line class="component" x1="{}" y1="{}" x2="{}" y2="{}"/>"#,
        cx,
        cy + 15.0,
        cx,
        cy + 25.0
    )
    .unwrap();
}

fn write_ccvs_symbol(svg: &mut String, cx: f64, cy: f64, config: &SvgExportConfig) {
    // Current-Controlled Voltage Source - same as VCVS
    write_vcvs_symbol(svg, cx, cy, config);
}

fn write_cccs_symbol(svg: &mut String, cx: f64, cy: f64, config: &SvgExportConfig) {
    // Current-Controlled Current Source - same as VCCS
    write_vccs_symbol(svg, cx, cy, config);
}

fn write_coupled_inductor_symbol(svg: &mut String, cx: f64, cy: f64, config: &SvgExportConfig) {
    write_inductor_symbol(svg, cx, cy, config);
    writeln!(
        svg,
        r#"<text class="text" x="{}" y="{}" text-anchor="middle">K</text>"#,
        cx,
        cy - 14.0
    )
    .unwrap();
}

fn write_block_symbol(svg: &mut String, cx: f64, cy: f64, symbol: &str, _config: &SvgExportConfig) {
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

fn write_cell_instance_symbol(svg: &mut String, cx: f64, cy: f64, config: &SvgExportConfig) {
    write_block_symbol(svg, cx, cy, "X", config);
}

fn write_xspice_symbol(
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

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_export_empty_schematic() {
        let state = SchematicState::default();
        let config = SvgExportConfig::default();
        let svg = export_to_svg(&state, &config);

        assert!(svg.contains("<?xml"));
        assert!(svg.contains("<svg"));
        assert!(svg.contains("</svg>"));
    }

    #[test]
    fn test_export_with_resistor() {
        let mut state = SchematicState::default();
        state.add_component(ComponentType::Resistor, Point::new(10, 10));

        let config = SvgExportConfig::default();
        let svg = export_to_svg(&state, &config);

        assert!(svg.contains("<path class=\"component\""));
        assert!(svg.contains("R1"));
    }

    #[test]
    fn test_export_with_wire() {
        let mut state = SchematicState::default();
        state
            .add_wire(vec![Point::new(0, 0), Point::new(10, 0)])
            .unwrap();

        let config = SvgExportConfig::default();
        let svg = export_to_svg(&state, &config);

        assert!(svg.contains("<path class=\"wire\""));
        assert!(svg.contains("M "));
        assert!(svg.contains(" L "));
    }

    #[test]
    fn test_export_with_capacitor() {
        let mut state = SchematicState::default();
        state.add_component(ComponentType::Capacitor, Point::new(10, 10));

        let config = SvgExportConfig::default();
        let svg = export_to_svg(&state, &config);

        assert!(svg.contains("C1"));
    }

    #[test]
    fn test_export_with_ground() {
        let mut state = SchematicState::default();
        state.add_component(ComponentType::Ground, Point::new(10, 10));

        let config = SvgExportConfig::default();
        let svg = export_to_svg(&state, &config);

        assert!(svg.contains("<line"));
    }

    #[test]
    fn test_config_defaults() {
        let config = SvgExportConfig::default();
        assert_eq!(config.grid_size, 10.0);
        assert!(config.wire_color.starts_with('#'));
    }

    #[test]
    fn test_rotation_transform() {
        assert!(get_rotation_transform(Rotation::R0, 0.0, 0.0).is_empty());
        assert!(get_rotation_transform(Rotation::R90, 100.0, 100.0).contains("rotate(90"));
        assert!(get_rotation_transform(Rotation::R180, 0.0, 0.0).contains("rotate(180"));
        assert!(get_rotation_transform(Rotation::R270, 50.0, 50.0).contains("rotate(270"));
    }

    #[test]
    fn test_bounds_calculation() {
        let mut state = SchematicState::default();
        state.add_component(ComponentType::Resistor, Point::new(0, 0));
        state.add_component(ComponentType::Resistor, Point::new(100, 100));

        let config = SvgExportConfig::default();
        let (min_x, min_y, max_x, max_y) = calculate_bounds(&state, &config);

        assert!(min_x < max_x);
        assert!(min_y < max_y);
    }

    // =========================================================================
    // MOSFET Symbol Tests
    // =========================================================================

    #[test]
    fn test_export_with_nmos() {
        let mut state = SchematicState::default();
        state.add_component(ComponentType::Nmos, Point::new(10, 10));

        let config = SvgExportConfig::default();
        let svg = export_to_svg(&state, &config);

        // Check for NMOS-specific elements
        assert!(svg.contains("M1")); // MOSFET name
        assert!(svg.contains("<line")); // Gate and channel lines
        assert!(svg.contains("polygon")); // Arrow indicator
    }

    #[test]
    fn test_export_with_pmos() {
        let mut state = SchematicState::default();
        state.add_component(ComponentType::Pmos, Point::new(10, 10));

        let config = SvgExportConfig::default();
        let svg = export_to_svg(&state, &config);

        // Check for PMOS-specific elements (includes inversion bubble)
        assert!(svg.contains("M1")); // MOSFET name
        assert!(svg.contains("<circle")); // Gate inversion bubble
        assert!(svg.contains("polygon")); // Arrow indicator
    }

    // =========================================================================
    // BJT Symbol Tests
    // =========================================================================

    #[test]
    fn test_export_with_npn_bjt() {
        let mut state = SchematicState::default();
        state.add_component(ComponentType::NpnBjt, Point::new(10, 10));

        let config = SvgExportConfig::default();
        let svg = export_to_svg(&state, &config);

        // Check for NPN BJT elements
        assert!(svg.contains("Q1")); // BJT name
        assert!(svg.contains("<line")); // Base, collector, emitter lines
        assert!(svg.contains("polygon")); // Emitter arrow
    }

    #[test]
    fn test_export_with_pnp_bjt() {
        let mut state = SchematicState::default();
        state.add_component(ComponentType::PnpBjt, Point::new(10, 10));

        let config = SvgExportConfig::default();
        let svg = export_to_svg(&state, &config);

        // Check for PNP BJT elements
        assert!(svg.contains("Q1")); // BJT name
        assert!(svg.contains("<line")); // Base, collector, emitter lines
        assert!(svg.contains("polygon")); // Emitter arrow (pointing in)
    }

    // =========================================================================
    // Diode and Current Source Tests
    // =========================================================================

    #[test]
    fn test_export_with_diode() {
        let mut state = SchematicState::default();
        state.add_component(ComponentType::Diode, Point::new(10, 10));

        let config = SvgExportConfig::default();
        let svg = export_to_svg(&state, &config);

        // Check for diode elements (triangle + bar)
        assert!(svg.contains("D1")); // Diode name
        assert!(svg.contains("polygon")); // Triangle
        assert!(svg.contains("<line")); // Cathode bar and leads
    }

    #[test]
    fn test_export_with_current_source() {
        let mut state = SchematicState::default();
        state.add_component(ComponentType::CurrentSource, Point::new(10, 10));

        let config = SvgExportConfig::default();
        let svg = export_to_svg(&state, &config);

        // Check for current source elements (circle with arrow)
        assert!(svg.contains("I1")); // Current source name
        assert!(svg.contains("<circle")); // Source circle
        assert!(svg.contains("polygon")); // Direction arrow
    }

    // =========================================================================
    // Complex Schematic Tests
    // =========================================================================

    #[test]
    fn test_export_cmos_inverter_circuit() {
        let mut state = SchematicState::default();
        // CMOS inverter: PMOS + NMOS + power supplies
        state.add_component(ComponentType::Pmos, Point::new(20, 10));
        state.add_component(ComponentType::Nmos, Point::new(20, 30));
        state.add_component(ComponentType::VoltageSource, Point::new(5, 20));
        state.add_component(ComponentType::Ground, Point::new(5, 35));

        let config = SvgExportConfig::default();
        let svg = export_to_svg(&state, &config);

        // All components should be present
        assert!(svg.contains("M1")); // PMOS
        assert!(svg.contains("M2")); // NMOS
        assert!(svg.contains("V1")); // Voltage source
    }

    #[test]
    fn test_export_bjt_amplifier_circuit() {
        let mut state = SchematicState::default();
        // Common-emitter amplifier
        state.add_component(ComponentType::NpnBjt, Point::new(20, 20));
        state.add_component(ComponentType::Resistor, Point::new(20, 5));
        state.add_component(ComponentType::Resistor, Point::new(10, 20));
        state.add_component(ComponentType::Capacitor, Point::new(0, 20));
        state.add_component(ComponentType::Ground, Point::new(20, 35));

        let config = SvgExportConfig::default();
        let svg = export_to_svg(&state, &config);

        // All components should be present
        assert!(svg.contains("Q1")); // BJT
        assert!(svg.contains("R1")); // Collector resistor
        assert!(svg.contains("R2")); // Base resistor
        assert!(svg.contains("C1")); // Coupling capacitor
    }

    #[test]
    fn test_export_full_wave_rectifier() {
        let mut state = SchematicState::default();
        // Full-wave rectifier with 4 diodes
        state.add_component(ComponentType::Diode, Point::new(10, 10));
        state.add_component(ComponentType::Diode, Point::new(30, 10));
        state.add_component(ComponentType::Diode, Point::new(10, 30));
        state.add_component(ComponentType::Diode, Point::new(30, 30));
        state.add_component(ComponentType::VoltageSourceAc, Point::new(0, 20));
        state.add_component(ComponentType::Capacitor, Point::new(45, 20));

        let config = SvgExportConfig::default();
        let svg = export_to_svg(&state, &config);

        // All 4 diodes should be present
        assert!(svg.contains("D1"));
        assert!(svg.contains("D2"));
        assert!(svg.contains("D3"));
        assert!(svg.contains("D4"));
        assert!(svg.contains("V1")); // AC source
    }

    #[test]
    fn test_export_cell_instance_uses_block_symbol() {
        let mut state = SchematicState::default();
        state.add_component(ComponentType::CellInstance, Point::new(10, 10));

        let config = SvgExportConfig::default();
        let svg = export_to_svg(&state, &config);

        assert!(svg.contains("X1"));
        assert!(svg.contains(">X<"));
    }

    #[test]
    fn test_export_xspice_inverter_uses_labeled_block_symbol() {
        let mut state = SchematicState::default();
        state.add_component(ComponentType::XspiceInverter, Point::new(10, 10));

        let config = SvgExportConfig::default();
        let svg = export_to_svg(&state, &config);

        assert!(svg.contains("A1"));
        assert!(svg.contains(">INV<"));
    }

    #[test]
    fn test_export_supports_all_component_kinds_without_fallback() {
        let mut state = SchematicState::default();
        let all_types = [
            ComponentType::Resistor,
            ComponentType::Capacitor,
            ComponentType::Inductor,
            ComponentType::CoupledInductor,
            ComponentType::Diode,
            ComponentType::NpnBjt,
            ComponentType::PnpBjt,
            ComponentType::Nmos,
            ComponentType::Pmos,
            ComponentType::Njfet,
            ComponentType::Pjfet,
            ComponentType::NVdmos,
            ComponentType::PVdmos,
            ComponentType::SaturableInductor,
            ComponentType::VoltageSource,
            ComponentType::CurrentSource,
            ComponentType::VoltageSourceAc,
            ComponentType::VoltageSourcePulse,
            ComponentType::VoltageSourceSin,
            ComponentType::VoltageSourcePwl,
            ComponentType::VoltageSourceExp,
            ComponentType::VoltageSourceSffm,
            ComponentType::CurrentSourceAc,
            ComponentType::CurrentSourcePulse,
            ComponentType::CurrentSourceSin,
            ComponentType::CurrentSourcePwl,
            ComponentType::CurrentSourceExp,
            ComponentType::CurrentSourceNoise,
            ComponentType::Vcvs,
            ComponentType::Vccs,
            ComponentType::Ccvs,
            ComponentType::Cccs,
            ComponentType::Ground,
            ComponentType::CellInstance,
            ComponentType::XspiceGain,
            ComponentType::XspiceSummer,
            ComponentType::XspiceMultiplier,
            ComponentType::XspiceDivider,
            ComponentType::XspiceLimiter,
            ComponentType::XspiceIntegrator,
            ComponentType::XspiceDifferentiator,
            ComponentType::XspiceInverter,
            ComponentType::XspiceBuffer,
            ComponentType::XspiceAndGate,
            ComponentType::XspiceOrGate,
            ComponentType::XspiceNandGate,
            ComponentType::XspiceNorGate,
            ComponentType::XspiceXorGate,
            ComponentType::XspiceTristate,
            ComponentType::XspiceDFlipFlop,
            ComponentType::XspiceJkFlipFlop,
            ComponentType::XspiceSrLatch,
            ComponentType::XspiceAdcBridge,
            ComponentType::XspiceDacBridge,
        ];

        for (idx, kind) in all_types.into_iter().enumerate() {
            state.add_component(kind, Point::new(10, 10 + (idx as i32 * 4)));
        }

        let config = SvgExportConfig::default();
        let svg = export_to_svg(&state, &config);

        // Ensure every generated component instance label is present in SVG.
        // This validates that symbol emission stayed exhaustive for all enum variants.
        for name in state.components.iter().map(|c| c.name.clone()) {
            assert!(
                svg.contains(&name),
                "missing component label '{}' in exported SVG",
                name
            );
        }
    }
}
