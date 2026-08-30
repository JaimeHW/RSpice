//! Schematic Export
//!
//! Export schematic diagrams to SVG format for documentation and printing.
//! Produces clean, scalable vector graphics suitable for professional
//! circuit documentation.
//!
//! Matches Cadence Virtuoso export quality for commercial-grade output.

use std::fmt::Write;

use crate::state::{
    Component, ComponentType, ResolvedCellSymbol, ResolvedSymbolSource, SchematicState,
    SymbolResolver,
};

mod block_symbols;
mod config;
mod geometry;

#[cfg(test)]
pub use self::config::SvgColor;
pub use self::config::SvgExportConfig;

use self::block_symbols::{
    catalog_asset_world_bounds, write_artwork_lead_extensions, write_catalog_asset_symbol,
};
#[cfg(test)]
use self::geometry::calculate_bounds;
use self::geometry::{
    estimated_text_width, include_bus_bounds, include_design_note_bounds,
    include_documentation_shape_bounds, include_junction_bounds, write_bus, write_bus_tap,
    write_design_note, write_documentation_shape, write_junction, write_wire,
};
use super::view::resolved_symbol_render::{
    resolved_symbol_world_bounds, write_resolved_symbol_svg,
};

#[derive(Clone)]
struct ResolvedSymbolExportEntry {
    component_id: u64,
    symbol: ResolvedCellSymbol,
}

/// Document identity needed to resolve view-dependent design-note properties
/// during export. Keeping it explicit prevents the exported annotation from
/// silently claiming a generic or stale cell/view.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SvgDesignContext<'a> {
    pub view_path: &'a str,
}

impl Default for SvgDesignContext<'static> {
    fn default() -> Self {
        Self {
            view_path: "schematic",
        }
    }
}

/// Export a schematic to SVG format
/// Export with no resolved symbols and a default design context.
///
/// Test-only. The shipping path is
/// [`export_to_svg_with_symbol_resolver_and_context`], which is what
/// `menu_bar::export_actions` calls; this and the two wrappers below exist so
/// tests can skip building a resolver. They are gated rather than deleted
/// because a test that exercises a convenience wrapper production never takes
/// is still exercising the same renderer underneath.
#[cfg(test)]
pub fn export_to_svg(state: &SchematicState, config: &SvgExportConfig) -> String {
    export_to_svg_with_resolved_symbol_entries(state, config, &[], SvgDesignContext::default())
}

/// Export with a resolver but a default design context. Test-only; see
/// [`export_to_svg`].
#[cfg(test)]
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
            let resolved = resolver
                .resolve_binding(binding)
                .filter(|symbol| symbol.source() == ResolvedSymbolSource::Authored)?;
            Some(ResolvedSymbolExportEntry {
                component_id: component.id,
                symbol: resolved,
            })
        })
        .collect();
    export_to_svg_with_resolved_symbol_entries(state, config, &entries, SvgDesignContext::default())
}

pub fn export_to_svg_with_symbol_resolver_and_context(
    state: &SchematicState,
    config: &SvgExportConfig,
    resolver: &SymbolResolver<'_>,
    context: SvgDesignContext<'_>,
) -> String {
    let entries: Vec<ResolvedSymbolExportEntry> = state
        .components
        .iter()
        .filter_map(|component| {
            let binding = component.library_cell.as_ref()?;
            let resolved = resolver
                .resolve_binding(binding)
                .filter(|symbol| symbol.source() == ResolvedSymbolSource::Authored)?;
            Some(ResolvedSymbolExportEntry {
                component_id: component.id,
                symbol: resolved,
            })
        })
        .collect();
    export_to_svg_with_resolved_symbol_entries(state, config, &entries, context)
}

/// Export from pre-resolved symbols with a default design context. Test-only;
/// see [`export_to_svg`].
#[cfg(test)]
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
    export_to_svg_with_resolved_symbol_entries(state, config, &entries, SvgDesignContext::default())
}

fn export_to_svg_with_resolved_symbol_entries(
    state: &SchematicState,
    config: &SvgExportConfig,
    resolved_symbols: &[ResolvedSymbolExportEntry],
    context: SvgDesignContext<'_>,
) -> String {
    let mut svg = String::new();
    let symbol_library = crate::schematic::SymbolLibrary::load_embedded()
        .map_err(|error| log::error!("Cannot load embedded symbols for SVG export: {error}"))
        .ok();

    // Calculate bounds
    let (min_x, min_y, max_x, max_y) = calculate_bounds_with_resolved_symbols(
        state,
        config,
        resolved_symbols,
        symbol_library.as_ref(),
        context,
    );
    let width = max_x - min_x + 2.0 * config.margin;
    let height = max_y - min_y + 2.0 * config.margin;

    // SVG header
    let _ = writeln!(
        svg,
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n\
<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{width}\" height=\"{height}\" viewBox=\"{vx} {vy} {width} {height}\">\n\
<style>\n\
  .wire {{ stroke: {wire_color}; stroke-width: {wire_width}; fill: none; stroke-linecap: round; stroke-linejoin: round; }}\n\
  .bus {{ stroke: {wire_color}; stroke-width: {bus_width}; fill: none; stroke-linecap: round; stroke-linejoin: round; }}\n\
  .bus-tap {{ stroke: {wire_color}; stroke-width: {tap_width}; fill: none; stroke-linecap: round; }}\n\
  .junction {{ fill: {wire_color}; stroke: none; }}\n\
  .component {{ stroke: {comp_color}; stroke-width: {comp_width}; fill: none; }}\n\
  .text {{ font-family: monospace; font-size: {font_size}px; fill: {text_color}; }}\n\
  .design-note-anchor {{ fill: {text_color}; stroke: none; }}\n\
  .design-note-requirement-link .text {{ text-decoration: underline; }}\n\
  .documentation-shape {{ stroke: {text_color}; stroke-width: {comp_width}; fill: none; stroke-linecap: round; stroke-linejoin: round; }}\n\
</style>",
        vx = min_x - config.margin,
        vy = min_y - config.margin,
        wire_color = config.wire_color,
        wire_width = config.wire_stroke_width,
        bus_width = config.wire_stroke_width * 0.6,
        tap_width = config.wire_stroke_width,
        comp_color = config.component_color,
        comp_width = config.component_stroke_width,
        font_size = config.font_size,
        text_color = config.text_color
    );

    if let Some(background_color) = config.background_color.as_ref() {
        let _ = writeln!(
            svg,
            "<rect x=\"{x}\" y=\"{y}\" width=\"{width}\" height=\"{height}\" fill=\"{background_color}\"/>",
            x = min_x - config.margin,
            y = min_y - config.margin,
        );
    }

    // Export wires
    for wire in &state.wires {
        write_wire(&mut svg, wire, config);
    }

    for bus in &state.buses {
        write_bus(&mut svg, bus, config);
    }

    for tap in &state.bus_taps {
        write_bus_tap(&mut svg, tap, config);
    }

    // Junction dots encode explicit connectivity and are document content,
    // not a transient canvas decoration.
    for junction in &state.junctions {
        write_junction(&mut svg, junction, config);
    }

    // Export components
    for component in &state.components {
        write_component(
            &mut svg,
            component,
            config,
            find_resolved_symbol(component, resolved_symbols),
            symbol_library.as_ref(),
        );
    }

    for note in &state.design_notes {
        write_design_note(&mut svg, state, note, config, context.view_path);
    }

    for shape in &state.documentation_shapes {
        write_documentation_shape(&mut svg, shape, config);
    }

    // Close SVG
    svg.push_str("</svg>");

    svg
}

fn calculate_bounds_with_resolved_symbols(
    state: &SchematicState,
    config: &SvgExportConfig,
    resolved_symbols: &[ResolvedSymbolExportEntry],
    symbol_library: Option<&crate::schematic::SymbolLibrary>,
    context: SvgDesignContext<'_>,
) -> (f64, f64, f64, f64) {
    let mut min_x = f64::MAX;
    let mut min_y = f64::MAX;
    let mut max_x = f64::MIN;
    let mut max_y = f64::MIN;

    for component in &state.components {
        let (comp_min_x, comp_min_y, comp_max_x, comp_max_y) = component_export_world_bounds(
            component,
            find_resolved_symbol(component, resolved_symbols),
            symbol_library,
            config,
        );
        min_x = min_x.min(comp_min_x * config.grid_size);
        min_y = min_y.min(comp_min_y * config.grid_size);
        max_x = max_x.max(comp_max_x * config.grid_size);
        max_y = max_y.max(comp_max_y * config.grid_size);
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

    include_design_note_bounds(
        state,
        config,
        context.view_path,
        &mut min_x,
        &mut min_y,
        &mut max_x,
        &mut max_y,
    );

    include_documentation_shape_bounds(
        state, config, &mut min_x, &mut min_y, &mut max_x, &mut max_y,
    );

    if min_x == f64::MAX {
        (0.0, 0.0, 100.0, 100.0)
    } else {
        (min_x, min_y, max_x, max_y)
    }
}

fn component_export_world_bounds(
    component: &Component,
    resolved_symbol: Option<&ResolvedCellSymbol>,
    symbol_library: Option<&crate::schematic::SymbolLibrary>,
    config: &SvgExportConfig,
) -> (f64, f64, f64, f64) {
    if component.kind == ComponentType::CellInstance {
        if let Some((symbol, width, height)) =
            symbol_library.and_then(|library| compatible_builtin_xspice_symbol(component, library))
        {
            let mut bounds = catalog_asset_world_bounds(
                component,
                symbol,
                width,
                height,
                component.rotation.degrees(),
            )
            .unwrap_or_else(|| {
                symbol_resolution_error_world_bounds(component, config, "unresolved cell")
            });
            for (edge, terminal) in component.artwork_lead_extensions() {
                for point in [edge, terminal] {
                    let point = component.pos + component.transform_point(point);
                    bounds.0 = bounds.0.min(f64::from(point.x));
                    bounds.1 = bounds.1.min(f64::from(point.y));
                    bounds.2 = bounds.2.max(f64::from(point.x));
                    bounds.3 = bounds.3.max(f64::from(point.y));
                }
            }
            return bounds;
        }
        return resolved_symbol
            .filter(|symbol| symbol.source() == ResolvedSymbolSource::Authored)
            .and_then(|symbol| resolved_symbol_world_bounds(component, symbol))
            .map(|(min, max)| {
                (
                    f64::from(min.x),
                    f64::from(min.y),
                    f64::from(max.x),
                    f64::from(max.y),
                )
            })
            .unwrap_or_else(|| {
                symbol_resolution_error_world_bounds(component, config, "unresolved cell")
            });
    }

    let Some((symbol, adjusted_rotation)) = symbol_library.and_then(|library| {
        library.get_with_rotation_variant(
            component.kind,
            component.rotation.degrees(),
            component.symbol_variant.as_deref(),
        )
    }) else {
        return symbol_resolution_error_world_bounds(component, config, "missing canonical SVG");
    };
    let mut bounds = catalog_asset_world_bounds(
        component,
        symbol,
        symbol.target_width,
        symbol.target_height,
        adjusted_rotation,
    )
    .unwrap_or_else(|| {
        symbol_resolution_error_world_bounds(component, config, "missing canonical SVG")
    });
    if component.kind == ComponentType::Port {
        let direction = component
            .port_spec()
            .map(|port| port.direction)
            .unwrap_or_default();
        for segment in crate::schematic::port_overlay::direction_segments(direction) {
            for point in [segment.start, segment.end] {
                let point = crate::schematic::port_overlay::transform_point(
                    point,
                    adjusted_rotation,
                    component.mirror_h,
                    component.mirror_v,
                );
                let x = f64::from(component.pos.x) + f64::from(point.x);
                let y = f64::from(component.pos.y) + f64::from(point.y);
                bounds.0 = bounds.0.min(x);
                bounds.1 = bounds.1.min(y);
                bounds.2 = bounds.2.max(x);
                bounds.3 = bounds.3.max(y);
            }
        }
    }
    bounds
}

fn symbol_resolution_error_world_bounds(
    component: &Component,
    config: &SvgExportConfig,
    reason: &str,
) -> (f64, f64, f64, f64) {
    const HALF_SIZE: f64 = 12.0;
    const TEXT_HORIZONTAL_PADDING: f64 = 2.0;
    let x = f64::from(component.pos.x);
    let y = f64::from(component.pos.y);
    let diagnostic = format!("{}: {reason}", component.kind.display_name());
    let grid_size = config.grid_size.max(f64::EPSILON);
    let text_half_width = (estimated_text_width(&diagnostic, config)
        + 2.0 * TEXT_HORIZONTAL_PADDING)
        / grid_size
        / 2.0;
    let text_descent = config.font_size * 0.35 / grid_size;
    (
        x - HALF_SIZE.max(text_half_width),
        y - HALF_SIZE,
        x + HALF_SIZE.max(text_half_width),
        y + HALF_SIZE + config.font_size / grid_size + text_descent,
    )
}

fn compatible_builtin_xspice_symbol<'a>(
    component: &Component,
    library: &'a crate::schematic::SymbolLibrary,
) -> Option<(&'a crate::schematic::symbols::Symbol, f32, f32)> {
    let contract = component.library_cell.as_ref()?.builtin_xspice.as_ref()?;
    let (width, height) = component.artwork_dimensions();
    let offsets = component.artwork_pin_offsets();
    if !library.asset_matches_terminal_offsets(
        &contract.symbol_asset,
        width as f32,
        height as f32,
        &offsets,
    ) {
        return None;
    }
    library
        .get_asset(&contract.symbol_asset)
        .map(|symbol| (symbol, width as f32, height as f32))
}

fn write_component(
    svg: &mut String,
    component: &Component,
    config: &SvgExportConfig,
    resolved_symbol: Option<&ResolvedCellSymbol>,
    symbol_library: Option<&crate::schematic::SymbolLibrary>,
) {
    let cx = component.pos.x as f64 * config.grid_size;
    let cy = component.pos.y as f64 * config.grid_size;
    let mut symbol_wrote_labels = false;

    svg.push_str("<g>\n");

    if component.kind == ComponentType::CellInstance {
        // Artwork first, exactly as the canvas resolves it: an exported sheet
        // must show the same authored symbol the schematic was drawn with.
        if let Some((symbol, width, height)) =
            symbol_library.and_then(|library| compatible_builtin_xspice_symbol(component, library))
        {
            write_catalog_asset_symbol(
                svg,
                component,
                symbol,
                width,
                height,
                component.rotation.degrees(),
                None,
                config,
            );
            write_artwork_lead_extensions(svg, component, config);
        } else if let Some(symbol) = resolved_symbol
            && symbol.source() == ResolvedSymbolSource::Authored
            && resolved_symbol_world_bounds(component, symbol).is_some()
        {
            write_resolved_symbol_svg(svg, component, symbol, config);
            symbol_wrote_labels = true;
        } else {
            write_symbol_resolution_error(svg, component, config, "unresolved cell");
        }
    } else if let Some((symbol, adjusted_rotation)) = symbol_library.and_then(|library| {
        library.get_with_rotation_variant(
            component.kind,
            component.rotation.degrees(),
            component.symbol_variant.as_deref(),
        )
    }) {
        let port_stroke_width = if component.kind == ComponentType::Port {
            component
                .port_spec()
                .and_then(|port| port.vector().map(|_| config.wire_stroke_width * 0.6))
        } else {
            None
        };
        write_catalog_asset_symbol(
            svg,
            component,
            symbol,
            symbol.target_width,
            symbol.target_height,
            adjusted_rotation,
            port_stroke_width,
            config,
        );
        if component.kind == ComponentType::Port {
            write_port_direction_overlay(
                svg,
                component,
                adjusted_rotation,
                component
                    .port_spec()
                    .map(|port| port.direction)
                    .unwrap_or_default(),
                config,
            );
        }
    } else {
        write_symbol_resolution_error(svg, component, config, "missing canonical SVG");
    }

    if !symbol_wrote_labels {
        // Component label
        if component
            .display_mode
            .show_name(crate::state::SchematicParameterLabelVisibility::NamesAndValues)
        {
            writeln!(
                svg,
                "<text class=\"text\" x=\"{}\" y=\"{}\">{}</text>",
                cx,
                cy - 25.0,
                escape_xml(&component.name)
            )
            .unwrap();
        }

        if component
            .display_mode
            .show_value(crate::state::SchematicParameterLabelVisibility::NamesAndValues)
            && !component.value.is_empty()
        {
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

fn write_port_direction_overlay(
    svg: &mut String,
    component: &Component,
    rotation_degrees: i32,
    direction: crate::state::PortDirection,
    config: &SvgExportConfig,
) {
    let cx = f64::from(component.pos.x) * config.grid_size;
    let cy = f64::from(component.pos.y) * config.grid_size;
    let point = |point| {
        let point = crate::schematic::port_overlay::transform_point(
            point,
            rotation_degrees,
            component.mirror_h,
            component.mirror_v,
        );
        (
            cx + f64::from(point.x) * config.grid_size,
            cy + f64::from(point.y) * config.grid_size,
        )
    };
    writeln!(
        svg,
        "<g class=\"port-direction-overlay\" data-direction=\"{}\">",
        direction.keyword()
    )
    .unwrap();
    let style = component
        .port_spec()
        .and_then(|port| port.vector())
        .map_or_else(String::new, |_| {
            format!(" style=\"stroke-width:{}\"", config.wire_stroke_width * 0.6)
        });
    for segment in crate::schematic::port_overlay::direction_segments(direction) {
        let start = point(segment.start);
        let end = point(segment.end);
        writeln!(
            svg,
            "<line class=\"component\" x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\"{style}/>",
            start.0, start.1, end.0, end.1
        )
        .unwrap();
    }
    svg.push_str("</g>\n");
}

fn write_symbol_resolution_error(
    svg: &mut String,
    component: &Component,
    config: &SvgExportConfig,
    reason: &str,
) {
    let cx = f64::from(component.pos.x) * config.grid_size;
    let cy = f64::from(component.pos.y) * config.grid_size;
    let half = 12.0 * config.grid_size;
    let x = cx - half;
    let y = cy - half;
    let size = half * 2.0;
    let kind = escape_xml(component.kind.display_name());
    let reason = escape_xml(reason);
    writeln!(
        svg,
        "<g class=\"symbol-resolution-error\" data-component-kind=\"{kind}\" data-error=\"{reason}\">"
    )
    .unwrap();
    writeln!(
        svg,
        "<rect x=\"{x}\" y=\"{y}\" width=\"{size}\" height=\"{size}\" fill=\"none\" stroke=\"#DC4646\"/>"
    )
    .unwrap();
    writeln!(
        svg,
        "<path d=\"M {x} {y} L {} {} M {} {y} L {x} {}\" stroke=\"#DC4646\" fill=\"none\"/>",
        x + size,
        y + size,
        x + size,
        y + size
    )
    .unwrap();
    writeln!(
        svg,
        "<text class=\"text\" x=\"{cx}\" y=\"{}\" text-anchor=\"middle\" fill=\"#DC4646\">{kind}: {reason}</text>",
        y + size + config.font_size
    )
    .unwrap();
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

    #[test]
    fn note_only_svg_is_bounded_escaped_multiline_and_semantic() {
        let mut schematic = SchematicState::default();
        schematic.design_notes.push(
            crate::state::DesignNote::new(
                19,
                crate::state::Point::new(30, 40),
                crate::state::DesignNoteKind::ReviewNote,
                "Check A < B & C\nSecond line",
            )
            .unwrap(),
        );

        let svg = export_to_svg(&schematic, &SvgExportConfig::default());
        assert!(svg.contains("design-note-review-note"));
        assert!(svg.contains("data-object-id=\"19\""));
        assert!(svg.contains("data-review-id=\"NOTE-0019\""));
        assert!(svg.contains("Check A &lt; B &amp; C"));
        assert!(svg.contains("<tspan"));
        assert!(svg.contains("Second line"));
        assert!(!svg.contains("Check A < B & C"));
        assert!(!svg.contains("viewBox=\"-20 -10 140 150\""));
    }

    #[test]
    fn property_display_export_uses_the_explicit_active_view_context() {
        let mut schematic = SchematicState::default();
        schematic.design_notes.push(
            crate::state::DesignNote::new(
                20,
                crate::state::Point::new(5, 6),
                crate::state::DesignNoteKind::PropertyDisplay,
                "${view} / ${component_count} components",
            )
            .unwrap(),
        );

        let svg = export_to_svg_with_resolved_symbol_entries(
            &schematic,
            &SvgExportConfig::default(),
            &[],
            SvgDesignContext {
                view_path: "user/top/schematic",
            },
        );

        assert!(svg.contains("user/top/schematic / 0 components"));
        assert!(!svg.contains(">schematic / 0 components"));
    }
    use crate::state::{
        Bus, BusDeclaration, BusSlice, BusTap, BusTapOrientation, Cell, DocumentationShape,
        DocumentationShapeGeometry, Junction, Library, LibraryCellInstance, LibraryManager, Point,
        PortDirection, PortSpec, SymbolDocument, SymbolPin, SymbolResolver, SymbolShape, View,
        ViewType,
    };
    use std::collections::HashMap;

    #[test]
    fn svg_export_uses_canonical_assets_for_every_standard_component() {
        let library = crate::schematic::SymbolLibrary::load_embedded().expect("canonical symbols");
        let mut schematic = SchematicState::default();
        let mut expected_paths = 0;
        for (index, kind) in ComponentType::ALL.into_iter().enumerate() {
            if kind == ComponentType::CellInstance {
                continue;
            }
            let symbol = library
                .get_with_rotation_variant(kind, 0, None)
                .unwrap_or_else(|| panic!("{kind:?} has no canonical symbol"))
                .0;
            expected_paths += symbol.paths.len();
            schematic.components.push(Component::new(
                index as u64 + 1,
                kind,
                Point::new(index as i32 * 100, 0),
            ));
        }

        let svg = export_to_svg(&schematic, &SvgExportConfig::default());

        assert!(!svg.contains("symbol-resolution-error"));
        assert_eq!(
            svg.matches(r#"<path class="component""#).count(),
            expected_paths
        );
    }

    #[test]
    fn missing_or_unknown_export_resolution_is_an_explicit_error() {
        let component = Component::new(1, ComponentType::Resistor, Point::origin());
        let mut missing_library = String::new();
        write_component(
            &mut missing_library,
            &component,
            &SvgExportConfig::default(),
            None,
            None,
        );
        assert!(missing_library.contains("symbol-resolution-error"));
        assert!(missing_library.contains("missing canonical SVG"));
        assert!(!missing_library.contains(r#"<path class="component""#));

        let mut unknown_variant = SchematicState::default();
        let mut diode = Component::new(2, ComponentType::Diode, Point::origin());
        diode.symbol_variant = Some("missing-variant".to_owned());
        unknown_variant.components.push(diode);
        let svg = export_to_svg(&unknown_variant, &SvgExportConfig::default());
        assert!(svg.contains("symbol-resolution-error"));
    }

    #[test]
    fn export_bounds_follow_rotated_mirrored_canonical_variant_target() {
        let library = crate::schematic::SymbolLibrary::load_embedded().expect("canonical symbols");
        let mut component = Component::new(1, ComponentType::VoltageSource, Point::new(140, -35))
            .with_rotation(crate::state::Rotation::R90)
            .with_mirror_h(true);
        component.symbol_variant = Some("battery_multi_cell".to_owned());
        let (symbol, adjusted_rotation) = library
            .get_with_rotation_variant(
                component.kind,
                component.rotation.degrees(),
                component.symbol_variant.as_deref(),
            )
            .expect("authored variant resolves");
        assert_eq!(adjusted_rotation, 90);

        let mut schematic = SchematicState::default();
        schematic.components.push(component);
        let config = SvgExportConfig {
            grid_size: 1.0,
            margin: 0.0,
            ..SvgExportConfig::default()
        };
        let bounds = calculate_bounds_with_resolved_symbols(
            &schematic,
            &config,
            &[],
            Some(&library),
            SvgDesignContext::default(),
        );
        let half_width = f64::from(symbol.target_height) / 2.0;
        let half_height = f64::from(symbol.target_width) / 2.0;

        let expected = (
            140.0 - half_width,
            -35.0 - half_height,
            140.0 + half_width,
            -35.0 + half_height,
        );
        for (actual, expected) in [
            (bounds.0, expected.0),
            (bounds.1, expected.1),
            (bounds.2, expected.2),
            (bounds.3, expected.3),
        ] {
            assert!((actual - expected).abs() < 1.0e-9);
        }
    }

    #[test]
    fn export_bounds_use_the_explicit_unresolved_cell_marker_and_diagnostic() {
        let library = crate::schematic::SymbolLibrary::load_embedded().expect("canonical symbols");
        let mut schematic = SchematicState::default();
        schematic.components.push(Component::new(
            1,
            ComponentType::CellInstance,
            Point::origin(),
        ));
        let config = SvgExportConfig {
            grid_size: 1.0,
            margin: 0.0,
            ..SvgExportConfig::default()
        };

        let component = schematic.components.first().expect("unresolved cell");
        let expected = symbol_resolution_error_world_bounds(component, &config, "unresolved cell");
        let bounds = calculate_bounds_with_resolved_symbols(
            &schematic,
            &config,
            &[],
            Some(&library),
            SvgDesignContext::default(),
        );

        assert_eq!(bounds, expected);
        assert!(bounds.0 < -12.0 && bounds.2 > 12.0);
        assert!(bounds.3 > 12.0 + config.font_size);
    }

    #[test]
    fn export_viewbox_contains_a_long_resolution_diagnostic_at_sheet_edge() {
        let mut component =
            Component::new(1, ComponentType::BehavioralSource, Point::new(-400, -300));
        component.symbol_variant = Some("missing-authored-variant".to_owned());
        let mut schematic = SchematicState::default();
        schematic.components.push(component);
        let config = SvgExportConfig {
            grid_size: 1.0,
            margin: 0.0,
            ..SvgExportConfig::default()
        };
        let expected = symbol_resolution_error_world_bounds(
            schematic.components.first().expect("component"),
            &config,
            "missing canonical SVG",
        );
        let diagnostic = "Behavioral Source: missing canonical SVG";
        let svg = export_to_svg(&schematic, &config);
        let width = expected.2 - expected.0;
        let height = expected.3 - expected.1;

        assert!(width >= estimated_text_width(diagnostic, &config));
        assert!(expected.3 > -300.0 + 12.0 + config.font_size);
        assert!(svg.contains(&format!(
            "viewBox=\"{} {} {width} {height}\"",
            expected.0, expected.1
        )));
        assert!(svg.contains(diagnostic));
    }

    #[test]
    fn interface_only_cell_cannot_export_a_generated_substitute_body() {
        let mut binding = LibraryCellInstance::new("missing", "amp", "schematic");
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
        schematic.add_library_cell_component(Point::origin(), binding);
        let libraries = LibraryManager::new();
        let buffers = HashMap::new();
        let resolver = SymbolResolver::new(&libraries, &buffers);

        let svg =
            export_to_svg_with_symbol_resolver(&schematic, &SvgExportConfig::default(), &resolver);

        assert!(svg.contains("symbol-resolution-error"));
        assert!(svg.contains("unresolved cell"));
        assert!(!svg.contains("symbol-pin-label"));
    }

    #[test]
    fn svg_port_uses_canonical_body_and_transformed_direction_overlay() {
        let config = SvgExportConfig {
            grid_size: 1.0,
            ..SvgExportConfig::default()
        };
        let component = Component::new(1, ComponentType::Port, Point::new(100, 80))
            .with_rotation(crate::state::Rotation::R90)
            .with_mirror_h(true);

        for (direction, segment_count) in [
            (PortDirection::In, 3),
            (PortDirection::Out, 3),
            (PortDirection::InOut, 5),
            (PortDirection::Supply, 2),
        ] {
            let mut overlay = String::new();
            write_port_direction_overlay(&mut overlay, &component, 90, direction, &config);
            assert!(overlay.contains(&format!("data-direction=\"{}\"", direction.keyword())));
            assert_eq!(
                overlay.matches(r#"<line class="component""#).count(),
                segment_count
            );
        }

        let library = crate::schematic::SymbolLibrary::load_embedded().expect("canonical symbols");
        let body = library
            .get_with_rotation_variant(ComponentType::Port, 90, None)
            .expect("canonical port body")
            .0;
        assert_eq!(body.paths.len(), 1);
        assert!(!body.paths[0].filled);
        assert!(!body.paths[0].commands.is_empty());

        let mut scalar_schematic = SchematicState::default();
        scalar_schematic.components.push(component.clone());
        let scalar_svg = export_to_svg(&scalar_schematic, &config);
        assert!(!scalar_svg.contains(r#"style="stroke-width:1.2""#));

        let mut vector = component;
        vector.value = "DATA[7:0]".to_owned();
        let mut schematic = SchematicState::default();
        schematic.components.push(vector);
        let svg = export_to_svg(&schematic, &config);
        assert!(svg.contains(r#"style="stroke-width:1.2""#));
    }

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

        // Each lead runs from the saved terminal all the way to the body it
        // belongs to, so neither instance exports a pin left floating.
        assert!(
            svg.contains(r#"<line class="component" x1="600" y1="1000" x2="900" y2="1000"/>"#),
            "first instance should export its saved IN terminal"
        );
        assert!(
            svg.contains(r#"<line class="component" x1="2400" y1="1000" x2="2100" y2="1000"/>"#),
            "second instance should export its saved OUT terminal, not reuse the first interface"
        );
    }

    #[test]
    fn svg_export_escapes_canonical_component_labels() {
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

    #[test]
    fn svg_export_background_is_explicit_and_can_be_transparent() {
        let schematic = SchematicState::default();
        let dark = export_to_svg(&schematic, &SvgExportConfig::default());
        assert!(dark.contains("fill=\"#1A1A1A\""));

        let transparent = export_to_svg(
            &schematic,
            &SvgExportConfig {
                background_color: None,
                ..SvgExportConfig::default()
            },
        );
        assert!(!transparent.contains("<rect"));

        let print_safe = export_to_svg(
            &schematic,
            &SvgExportConfig {
                background_color: Some(SvgColor::rgb(0xFF, 0xFF, 0xFF)),
                ..SvgExportConfig::default()
            },
        );
        assert!(print_safe.contains("fill=\"#FFFFFF\""));
    }

    #[test]
    fn svg_export_emits_explicit_junction_and_includes_marker_extents() {
        let mut schematic = SchematicState::default();
        schematic
            .junctions
            .push(Junction::new(19, Point::new(7, 11)));
        let config = SvgExportConfig {
            margin: 0.0,
            ..SvgExportConfig::default()
        };

        let svg = export_to_svg(&schematic, &config);

        assert!(svg.contains(r#".junction { fill: #00FF00; stroke: none; }"#));
        assert!(svg.contains(r#"<circle class="junction" cx="70" cy="110" r="3"/>"#));
        assert!(svg.contains(r#"width="6" height="6" viewBox="67 107 6 6""#));
    }

    #[test]
    fn svg_export_emits_complete_typed_bus_geometry_and_label_bounds() {
        let declaration = BusDeclaration::parse("DATA[7:0]").unwrap();
        let bus = Bus::segment(40, Point::new(0, 0), Point::new(20, 0), Some(declaration)).unwrap();
        let tap = BusTap::new(
            41,
            &bus,
            Point::new(10, 0),
            Point::new(10, 10),
            BusSlice::parse("DATA[3]").unwrap(),
            BusTapOrientation::Down,
        )
        .unwrap();
        let mut schematic = SchematicState::default();
        schematic.buses.push(bus);
        schematic.bus_taps.push(tap);
        let config = SvgExportConfig {
            margin: 0.0,
            ..SvgExportConfig::default()
        };

        let svg = export_to_svg(&schematic, &config);
        let (min_x, min_y, max_x, max_y) = calculate_bounds(&schematic, &config);

        assert_eq!(svg.matches(r#"<path class="bus""#).count(), 3);
        assert!(svg.contains(r#"<path class="bus-tap" d="M 100 0 L 100 100"/>"#));
        assert!(svg.contains(r#">DATA[7:0]</text>"#));
        assert!(svg.contains(r#">DATA[3]</text>"#));
        assert!(min_x <= -4.0 && min_y <= -20.0);
        assert!(max_x > 270.0 && max_y >= 100.0);
    }

    #[test]
    fn svg_export_preserves_documentation_shape_identity_layer_and_kind() {
        let mut schematic = SchematicState::default();
        schematic.documentation_shapes = vec![
            DocumentationShape::new(
                101,
                DocumentationShapeGeometry::Rectangle {
                    first: Point::new(0, 0),
                    opposite: Point::new(10, 8),
                },
            )
            .unwrap(),
            DocumentationShape::new(
                102,
                DocumentationShapeGeometry::Line {
                    start: Point::new(12, 0),
                    end: Point::new(20, 8),
                },
            )
            .unwrap(),
            DocumentationShape::new(
                103,
                DocumentationShapeGeometry::Polygon {
                    points: vec![Point::new(22, 0), Point::new(30, 0), Point::new(26, 8)],
                },
            )
            .unwrap(),
            DocumentationShape::new(
                104,
                DocumentationShapeGeometry::Arc {
                    start: Point::new(32, 8),
                    through: Point::new(36, 4),
                    end: Point::new(40, 8),
                },
            )
            .unwrap(),
            DocumentationShape::new(
                105,
                DocumentationShapeGeometry::Callout {
                    tip: Point::new(42, 0),
                    elbow: Point::new(46, 4),
                    box_corner: Point::new(54, 10),
                },
            )
            .unwrap(),
        ];

        let first = export_to_svg(&schematic, &SvgExportConfig::default());
        let second = export_to_svg(&schematic, &SvgExportConfig::default());

        assert_eq!(
            first, second,
            "unchanged documents must export deterministically"
        );
        for (id, kind) in [
            (101, "rectangle"),
            (102, "line"),
            (103, "polygon"),
            (104, "arc"),
            (105, "callout"),
        ] {
            let metadata = format!(
                "<g class=\"documentation-shape documentation-shape-{kind}\" data-object-id=\"{id}\" data-layer=\"drawing-documentation\" data-kind=\"{kind}\">"
            );
            assert!(
                first.contains(&metadata),
                "{kind} must retain its stable identity and semantic layer metadata"
            );
        }
        assert_eq!(
            first
                .matches("data-layer=\"drawing-documentation\"")
                .count(),
            5
        );
        assert!(first.contains("<rect"));
        assert!(first.contains("<line"));
        assert!(first.contains("<polygon"));
        assert!(first.contains("<path d=\"M "));
    }
}
