//! Schematic Export
//!
//! Export schematic diagrams to SVG format for documentation and printing.
//! Produces clean, scalable vector graphics suitable for professional
//! circuit documentation.
//!
//! Matches Cadence Virtuoso export quality for commercial-grade output.

use std::fmt::Write;

use crate::state::{Component, ComponentType, ResolvedCellSymbol, SchematicState, SymbolResolver};

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
use self::block_symbols::{write_cell_instance_symbol, write_port_symbol, write_xspice_symbol};
use self::controlled_symbols::{
    write_cccs_symbol, write_ccvs_symbol, write_opamp_symbol, write_vccs_symbol, write_vcvs_symbol,
    write_vswitch_symbol,
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
use super::view::resolved_symbol_render::{
    resolved_symbol_world_bounds, write_resolved_symbol_svg,
};

#[derive(Clone)]
struct ResolvedSymbolExportEntry {
    component_id: u64,
    symbol: ResolvedCellSymbol,
}

/// Export a schematic to SVG format
pub fn export_to_svg(state: &SchematicState, config: &SvgExportConfig) -> String {
    export_to_svg_with_resolved_symbol_entries(state, config, &[])
}

pub fn export_to_svg_with_symbol_resolver(
    state: &SchematicState,
    config: &SvgExportConfig,
    resolver: &SymbolResolver<'_>,
) -> String {
    let entries: Vec<ResolvedSymbolExportEntry> = state
        .components
        .iter()
        .filter_map(|component| {
            let binding = component.library_cell.as_ref()?;
            let resolved = resolver.resolve_binding(binding)?;
            Some(ResolvedSymbolExportEntry {
                component_id: component.id,
                symbol: resolved,
            })
        })
        .collect();
    export_to_svg_with_resolved_symbol_entries(state, config, &entries)
}

pub fn export_to_svg_with_resolved_symbols(
    state: &SchematicState,
    config: &SvgExportConfig,
    resolved_symbols: &[(u64, ResolvedCellSymbol)],
) -> String {
    let entries: Vec<ResolvedSymbolExportEntry> = resolved_symbols
        .iter()
        .map(|(component_id, symbol)| ResolvedSymbolExportEntry {
            component_id: *component_id,
            symbol: symbol.clone(),
        })
        .collect();
    export_to_svg_with_resolved_symbol_entries(state, config, &entries)
}

fn export_to_svg_with_resolved_symbol_entries(
    state: &SchematicState,
    config: &SvgExportConfig,
    resolved_symbols: &[ResolvedSymbolExportEntry],
) -> String {
    let mut svg = String::new();

    // Calculate bounds
    let (min_x, min_y, max_x, max_y) =
        calculate_bounds_with_resolved_symbols(state, config, resolved_symbols);
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
        write_component(
            &mut svg,
            component,
            config,
            find_resolved_symbol(component, resolved_symbols),
        );
    }

    // Close SVG
    svg.push_str("</svg>");

    svg
}

fn calculate_bounds_with_resolved_symbols(
    state: &SchematicState,
    config: &SvgExportConfig,
    resolved_symbols: &[ResolvedSymbolExportEntry],
) -> (f64, f64, f64, f64) {
    if resolved_symbols.is_empty() {
        return calculate_bounds(state, config);
    }

    let mut min_x = f64::MAX;
    let mut min_y = f64::MAX;
    let mut max_x = f64::MIN;
    let mut max_y = f64::MIN;

    for component in &state.components {
        let (comp_min, comp_max) =
            if let Some(symbol) = find_resolved_symbol(component, resolved_symbols) {
                resolved_symbol_world_bounds(component, symbol).unwrap_or_else(|| {
                    let (min_x, min_y, max_x, max_y) = component.bounding_box();
                    (
                        crate::state::Point::new(min_x, min_y),
                        crate::state::Point::new(max_x, max_y),
                    )
                })
            } else {
                let (min_x, min_y, max_x, max_y) = component.bounding_box();
                (
                    crate::state::Point::new(min_x, min_y),
                    crate::state::Point::new(max_x, max_y),
                )
            };
        min_x = min_x.min(comp_min.x as f64 * config.grid_size);
        min_y = min_y.min(comp_min.y as f64 * config.grid_size);
        max_x = max_x.max(comp_max.x as f64 * config.grid_size);
        max_y = max_y.max(comp_max.y as f64 * config.grid_size);
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

fn write_component(
    svg: &mut String,
    component: &Component,
    config: &SvgExportConfig,
    resolved_symbol: Option<&ResolvedCellSymbol>,
) {
    let cx = component.pos.x as f64 * config.grid_size;
    let cy = component.pos.y as f64 * config.grid_size;
    let resolved_cell_instance =
        component.kind == ComponentType::CellInstance && resolved_symbol.is_some();
    let transform = if resolved_cell_instance {
        String::new()
    } else {
        get_rotation_transform(component.rotation, cx, cy)
    };

    if transform.is_empty() {
        svg.push_str("<g>\n");
    } else {
        writeln!(svg, "<g transform=\"{}\">", transform).unwrap();
    }

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
        ComponentType::Port => write_port_symbol(svg, cx, cy, config),
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
        ComponentType::CellInstance => {
            if let Some(symbol) = resolved_symbol {
                write_resolved_symbol_svg(svg, component, symbol, config);
            } else {
                write_cell_instance_symbol(svg, cx, cy, config);
            }
        }
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

    if !resolved_cell_instance {
        // Component label
        writeln!(
            svg,
            "<text class=\"text\" x=\"{}\" y=\"{}\">{}</text>",
            cx,
            cy - 25.0,
            escape_xml(&component.name)
        )
        .unwrap();

        if !component.value.is_empty() {
            writeln!(
                svg,
                "<text class=\"text\" x=\"{}\" y=\"{}\">{}</text>",
                cx,
                cy + 35.0,
                escape_xml(&component.value)
            )
            .unwrap();
        }
    }

    svg.push_str("</g>\n");
}

fn find_resolved_symbol<'a>(
    component: &Component,
    resolved_symbols: &'a [ResolvedSymbolExportEntry],
) -> Option<&'a ResolvedCellSymbol> {
    resolved_symbols
        .iter()
        .find(|entry| entry.component_id == component.id)
        .map(|entry| &entry.symbol)
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
    use crate::state::{
        Cell, Library, LibraryCellInstance, LibraryManager, Point, PortDirection, PortSpec,
        SymbolDocument, SymbolPin, SymbolResolver, SymbolShape, View, ViewType,
    };
    use std::collections::HashMap;

    #[test]
    fn svg_export_uses_authored_cell_symbol_body_and_labels() {
        let mut binding = LibraryCellInstance::new("work", "amp", "schematic");
        binding.bind_interface(&[
            PortSpec {
                name: "IN".to_owned(),
                direction: PortDirection::In,
            },
            PortSpec {
                name: "OUT".to_owned(),
                direction: PortDirection::Out,
            },
        ]);

        let mut schematic = SchematicState::default();
        let id = schematic.add_library_cell_component(Point::new(100, 100), binding.clone());
        let component = schematic
            .components
            .iter_mut()
            .find(|component| component.id == id)
            .expect("placed component exists");
        component.name = "XAMP".to_owned();
        component.value = "gain_stage".to_owned();

        let document = SymbolDocument {
            pins: vec![
                SymbolPin::new("IN", PortDirection::In, Some(Point::new(-40, 0))),
                SymbolPin::new("OUT", PortDirection::Out, Some(Point::new(40, 0))),
            ],
            body: vec![SymbolShape::Polyline {
                points: vec![
                    Point::new(-10, -10),
                    Point::new(10, -10),
                    Point::new(10, 10),
                ],
                closed: false,
            }],
            name_anchor: Point::new(-20, -30),
            value_anchor: Point::new(-20, 30),
            ..SymbolDocument::default()
        };

        let mut libraries = LibraryManager::new();
        let mut library = Library::new("work");
        let mut cell = Cell::new("amp");
        let mut symbol_view = View::new("symbol", ViewType::Symbol);
        document
            .store_in_view(&mut symbol_view)
            .expect("symbol stores");
        cell.add_view(symbol_view);
        library.add_cell(cell);
        libraries.add_library(library);
        let resolved = SymbolResolver::new(&libraries, &HashMap::new())
            .resolve_binding(&binding)
            .expect("authored symbol resolves");

        let svg = export_to_svg_with_resolved_symbols(
            &schematic,
            &SvgExportConfig::default(),
            &[(id, resolved)],
        );

        assert!(
            svg.contains(r#"<path class="component" d="M 900 900 L 1100 900 L 1100 1100"/>"#),
            "authored body polyline should be exported"
        );
        assert!(
            svg.contains(r#"<text class="text" x="800" y="700">XAMP</text>"#),
            "instance name should use the authored name anchor"
        );
        assert!(
            svg.contains(r#"<text class="text" x="800" y="1300">gain_stage</text>"#),
            "instance value should use the authored value anchor"
        );
        assert!(
            !svg.contains(">X<"),
            "authored cell instances must not export as the generic X block"
        );
    }

    #[test]
    fn svg_export_resolves_each_instance_saved_interface_independently() {
        let mut first_binding = LibraryCellInstance::new("work", "amp", "schematic");
        first_binding.bind_interface(&[PortSpec {
            name: "IN".to_owned(),
            direction: PortDirection::In,
        }]);
        let mut second_binding = LibraryCellInstance::new("work", "amp", "schematic");
        second_binding.bind_interface(&[PortSpec {
            name: "OUT".to_owned(),
            direction: PortDirection::Out,
        }]);

        let mut schematic = SchematicState::default();
        let first_id =
            schematic.add_library_cell_component(Point::new(100, 100), first_binding.clone());
        schematic
            .components
            .iter_mut()
            .find(|component| component.id == first_id)
            .expect("first component exists")
            .name = "XIN".to_owned();
        let second_id =
            schematic.add_library_cell_component(Point::new(200, 100), second_binding.clone());
        schematic
            .components
            .iter_mut()
            .find(|component| component.id == second_id)
            .expect("second component exists")
            .name = "XOUT".to_owned();

        let document = SymbolDocument {
            pins: vec![
                SymbolPin::new("IN", PortDirection::In, Some(Point::new(-40, 0))),
                SymbolPin::new("OUT", PortDirection::Out, Some(Point::new(40, 0))),
            ],
            body: vec![SymbolShape::Polyline {
                points: vec![Point::new(-10, -10), Point::new(10, -10)],
                closed: false,
            }],
            ..SymbolDocument::default()
        };
        let mut libraries = LibraryManager::new();
        let mut library = Library::new("work");
        let mut cell = Cell::new("amp");
        let mut symbol_view = View::new("symbol", ViewType::Symbol);
        document
            .store_in_view(&mut symbol_view)
            .expect("symbol stores");
        cell.add_view(symbol_view);
        library.add_cell(cell);
        libraries.add_library(library);
        let buffers = HashMap::new();
        let resolver = SymbolResolver::new(&libraries, &buffers);

        let svg =
            export_to_svg_with_symbol_resolver(&schematic, &SvgExportConfig::default(), &resolver);

        assert!(
            svg.contains(r#"<line class="component" x1="600" y1="1000" x2="700" y2="1000"/>"#),
            "first instance should export its saved IN terminal"
        );
        assert!(
            svg.contains(r#"<line class="component" x1="2400" y1="1000" x2="2300" y2="1000"/>"#),
            "second instance should export its saved OUT terminal, not reuse the first interface"
        );
    }

    #[test]
    fn svg_export_escapes_fallback_component_labels() {
        let mut schematic = SchematicState::default();
        let id = schematic.add_component(ComponentType::Resistor, Point::new(10, 10));
        let component = schematic
            .components
            .iter_mut()
            .find(|component| component.id == id)
            .expect("component exists");
        component.name = "R<&>".to_owned();
        component.value = "1k & 2k".to_owned();

        let svg = export_to_svg(&schematic, &SvgExportConfig::default());

        assert!(svg.contains("R&lt;&amp;&gt;"));
        assert!(svg.contains("1k &amp; 2k"));
        assert!(!svg.contains("R<&>"));
        assert!(!svg.contains("1k & 2k"));
    }
}
