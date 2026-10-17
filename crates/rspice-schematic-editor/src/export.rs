//! Schematic Export
//!
//! Export schematic diagrams to SVG format for documentation and printing.
//! The page is paper: the active sheet only, in the light print palette,
//! with the canvas's nominal stroke widths and its world-unit type, every
//! label placed by the canvas's own layout functions.

use std::borrow::Cow;
use std::fmt::Write;

use rspice_design::{
    resolved_symbol::{ResolvedCellSymbol, ResolvedSymbolSource},
    schematic::{
        component::Component, component_display::SchematicParameterLabelVisibility,
        component_type::ComponentType, document::SchematicDocument,
    },
    symbol_resolver::SymbolResolver,
};
use rspice_design_model::design_management::SheetCatalog;

mod block_symbols;
mod config;
mod geometry;
#[cfg(test)]
mod label_placement_tests;
#[cfg(test)]
mod parity_tests;

pub use self::config::{SvgColor, SvgColorParseError, SvgExportConfig};

use self::block_symbols::{
    catalog_asset_world_bounds, write_artwork_lead_extensions, write_catalog_asset_symbol,
};
use self::geometry::{
    Extent, estimated_text_width, include_bus_bounds, include_design_note_bounds,
    include_documentation_shape_bounds, include_junction_bounds, include_net_label_bounds,
    include_probe_bounds, write_bus, write_bus_tap, write_conductor, write_design_note,
    write_documentation_shape, write_junction, write_net_label, write_probe,
};
use super::view::design_view::object_is_on_active_sheet;
use super::view::drawing::{chain_conductors, compute_label_layout};
use super::view::lod::{StrokeRole, nominal_wu};
use super::view::resolved_symbol_render::{
    InstanceLabel, resolved_symbol_world_bounds, write_resolved_symbol_svg, write_text_svg,
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
    /// The view's sheets: only the active one is exported, as only it is on
    /// the canvas. `None`, or a catalog with no active sheet, is one sheet.
    pub sheet_catalog: Option<&'a SheetCatalog>,
}

impl Default for SvgDesignContext<'static> {
    fn default() -> Self {
        Self {
            view_path: "schematic",
            sheet_catalog: None,
        }
    }
}

/// What the active sheet draws: every drawn object whose sheet is the active
/// one. A document with no sheets to choose between is borrowed whole.
fn active_sheet<'a>(
    document: &'a SchematicDocument,
    catalog: Option<&SheetCatalog>,
) -> Cow<'a, SchematicDocument> {
    if catalog.is_none_or(|catalog| catalog.active_sheet_id().is_none()) {
        return Cow::Borrowed(document);
    }
    let on = |id: u64| object_is_on_active_sheet(catalog, id);
    Cow::Owned(SchematicDocument {
        components: document
            .components
            .iter()
            .filter(|item| on(item.id))
            .cloned()
            .collect(),
        wires: document
            .wires
            .iter()
            .filter(|item| on(item.id))
            .cloned()
            .collect(),
        buses: document
            .buses
            .iter()
            .filter(|item| on(item.id))
            .cloned()
            .collect(),
        bus_taps: document
            .bus_taps
            .iter()
            .filter(|item| on(item.id))
            .cloned()
            .collect(),
        design_notes: document
            .design_notes
            .iter()
            .filter(|item| on(item.id))
            .cloned()
            .collect(),
        documentation_shapes: document
            .documentation_shapes
            .iter()
            .filter(|item| on(item.id))
            .cloned()
            .collect(),
        probes: document
            .probes
            .iter()
            .filter(|item| on(item.id))
            .cloned()
            .collect(),
        net_labels: document
            .net_labels
            .iter()
            .filter(|item| on(item.id))
            .cloned()
            .collect(),
        junctions: document
            .junctions
            .iter()
            .filter(|item| on(item.id))
            .cloned()
            .collect(),
        ..SchematicDocument::default()
    })
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
pub fn export_to_svg(document: &SchematicDocument, config: &SvgExportConfig) -> String {
    export_to_svg_with_resolved_symbol_entries(document, config, &[], SvgDesignContext::default())
}

/// Export with a resolver but a default design context. Test-only; see
/// [`export_to_svg`].
#[cfg(test)]
fn export_to_svg_with_symbol_resolver<S: AsRef<SchematicDocument>>(
    document: &SchematicDocument,
    config: &SvgExportConfig,
    resolver: &SymbolResolver<'_, S>,
) -> String {
    export_to_svg_with_symbol_resolver_and_context(
        document,
        config,
        resolver,
        SvgDesignContext::default(),
    )
}

/// Render an immutable schematic with its authored symbol bindings and view path.
pub fn export_to_svg_with_symbol_resolver_and_context<S: AsRef<SchematicDocument>>(
    document: &SchematicDocument,
    config: &SvgExportConfig,
    resolver: &SymbolResolver<'_, S>,
    context: SvgDesignContext<'_>,
) -> String {
    let entries: Vec<ResolvedSymbolExportEntry> = document
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
    export_to_svg_with_resolved_symbol_entries(document, config, &entries, context)
}

/// Export from pre-resolved symbols with a default design context. Test-only;
/// see [`export_to_svg`].
#[cfg(test)]
pub fn export_to_svg_with_resolved_symbols(
    document: &SchematicDocument,
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
    export_to_svg_with_resolved_symbol_entries(
        document,
        config,
        &entries,
        SvgDesignContext::default(),
    )
}

fn export_to_svg_with_resolved_symbol_entries(
    document: &SchematicDocument,
    config: &SvgExportConfig,
    resolved_symbols: &[ResolvedSymbolExportEntry],
    context: SvgDesignContext<'_>,
) -> String {
    let mut svg = String::new();
    let symbol_library = crate::SymbolLibrary::load_embedded()
        .map_err(|error| log::error!("Cannot load embedded symbols for SVG export: {error}"))
        .ok();
    let sheet = active_sheet(document, context.sheet_catalog);
    let sheet = sheet.as_ref();

    // Calculate bounds
    let (min_x, min_y, max_x, max_y) = calculate_bounds_with_resolved_symbols(
        document,
        sheet,
        config,
        resolved_symbols,
        symbol_library.as_ref(),
        context,
    );
    let width = max_x - min_x + 2.0 * config.margin;
    let height = max_y - min_y + 2.0 * config.margin;

    let _ = writeln!(
        svg,
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n\
<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{width}\" height=\"{height}\" viewBox=\"{vx} {vy} {width} {height}\">",
        vx = min_x - config.margin,
        vy = min_y - config.margin,
    );
    write_style(&mut svg, config);

    if let Some(background_color) = config.background_color.as_ref() {
        let _ = writeln!(
            svg,
            "<rect x=\"{x}\" y=\"{y}\" width=\"{width}\" height=\"{height}\" fill=\"{background_color}\"/>",
            x = min_x - config.margin,
            y = min_y - config.margin,
        );
    }

    // Wires as the canvas joins them: one path a chain, so a corner is a
    // mitered join rather than two butt ends.
    for chain in chain_conductors(sheet.wires.iter().map(|wire| wire.points.as_slice())) {
        write_conductor(&mut svg, &chain, config);
    }

    for bus in &sheet.buses {
        write_bus(&mut svg, bus, config);
    }

    for tap in &sheet.bus_taps {
        write_bus_tap(&mut svg, tap, config);
    }

    // Junction dots encode explicit connectivity and are document content,
    // not a transient canvas decoration.
    for junction in &sheet.junctions {
        write_junction(&mut svg, junction, config);
    }

    for component in &sheet.components {
        write_component(
            &mut svg,
            component,
            config,
            find_resolved_symbol(component, resolved_symbols),
            symbol_library.as_ref(),
        );
    }

    for label in &sheet.net_labels {
        write_net_label(&mut svg, label, config);
    }

    for probe in &sheet.probes {
        write_probe(&mut svg, probe, config);
    }

    // A note's properties read the whole document, as on the canvas.
    for note in &sheet.design_notes {
        write_design_note(&mut svg, document, note, config, context.view_path);
    }

    for shape in &sheet.documentation_shapes {
        write_documentation_shape(&mut svg, shape, config);
    }

    // Close SVG
    svg.push_str("</svg>");

    svg
}

/// Font stacks for the two families the canvas sets type in.
const SANS: &str = "'IBM Plex Sans', 'Helvetica Neue', Arial, sans-serif";
const MONO: &str = "'IBM Plex Mono', 'DejaVu Sans Mono', Consolas, monospace";

/// The page's classes: D1 section 11's inks, nominal stroke widths and
/// world-unit type. Conductors end square to their points (butt) and join
/// mitered; symbols and documentation round, the vector form of the
/// canvas's corner dots.
fn write_style(svg: &mut String, config: &SvgExportConfig) {
    let stroke = |role| geometry::world(nominal_wu(role), config);
    let size = |world: f32| geometry::world(world, config);
    let wire = config.wire_color;
    let symbol = config.symbol_color;
    let name = config.name_color;
    let text = config.text_color;
    let net = config.net_label_color;
    let paper = config
        .background_color
        .map_or_else(|| "none".to_owned(), |color| color.to_string());
    let _ = writeln!(
        svg,
        "<style>\n\
  .wire {{ stroke: {wire}; stroke-width: {wire_width}; fill: none; stroke-linecap: butt; stroke-linejoin: miter; stroke-miterlimit: 4; }}\n\
  .bus {{ stroke: {wire}; stroke-width: {bus_width}; fill: none; stroke-linecap: butt; stroke-linejoin: miter; stroke-miterlimit: 4; }}\n\
  .tap {{ stroke: {wire}; stroke-width: {tap_width}; fill: none; stroke-linecap: butt; }}\n\
  .junction {{ fill: {wire}; stroke: none; }}\n\
  .symbol {{ stroke: {symbol}; stroke-width: {symbol_width}; fill: none; stroke-linecap: round; stroke-linejoin: round; }}\n\
  .symbol-text {{ font-family: {MONO}; font-weight: 400; fill: {symbol}; }}\n\
  .name {{ font-family: {SANS}; font-size: {name_size}px; font-weight: 500; fill: {name}; }}\n\
  .value {{ font-family: {SANS}; font-size: {value_size}px; font-weight: 400; fill: {text}; }}\n\
  .pin {{ font-family: {MONO}; font-size: {pin_size}px; font-weight: 400; fill: {text}; }}\n\
  .net-mark {{ fill: {net}; stroke: none; }}\n\
  .net-name {{ font-family: {MONO}; font-size: {net_size}px; font-weight: 500; fill: {net}; }}\n\
  .net-tab {{ stroke: {net}; fill: none; stroke-linecap: round; stroke-linejoin: round; }}\n\
  .slice {{ font-family: {MONO}; font-size: {slice_size}px; font-weight: 500; fill: {net}; }}\n\
  .probe {{ stroke: {text}; stroke-width: {probe_width}; fill: none; stroke-linecap: butt; }}\n\
  .probe circle {{ fill: {paper}; }}\n\
  .probe-label {{ font-family: {MONO}; font-size: {slice_size}px; font-weight: 500; fill: {text}; stroke: none; }}\n\
  .note {{ font-family: {MONO}; font-size: {note_size}px; font-weight: 400; fill: {text}; }}\n\
  .design-note-anchor {{ fill: {text}; stroke: none; }}\n\
  .design-note-requirement-link .note {{ text-decoration: underline; }}\n\
  .documentation-shape {{ stroke: {text}; stroke-width: {shape_width}; fill: none; stroke-linecap: round; stroke-linejoin: round; }}\n\
</style>",
        wire_width = stroke(StrokeRole::Wire),
        bus_width = stroke(StrokeRole::BusStrand),
        tap_width = stroke(StrokeRole::BusTap),
        symbol_width = stroke(StrokeRole::Symbol),
        probe_width = stroke(StrokeRole::Probe),
        shape_width = stroke(StrokeRole::Documentation),
        name_size = InstanceLabel::Name.size(config),
        value_size = InstanceLabel::Value.size(config),
        pin_size = size(rspice_design::symbol_generation::GENERATED_PIN_LABEL_SIZE),
        net_size = size(crate::view::net_labels::LABEL_FONT_WORLD_SIZE),
        slice_size = size(crate::view::drawing::SMALL_TEXT_WORLD),
        note_size = size(crate::view::design_notes::FONT_WORLD_SIZE),
    );
}

/// `document` is the whole document, which a note's properties read;
/// `sheet` is what the active sheet draws.
fn calculate_bounds_with_resolved_symbols(
    document: &SchematicDocument,
    sheet: &SchematicDocument,
    config: &SvgExportConfig,
    resolved_symbols: &[ResolvedSymbolExportEntry],
    symbol_library: Option<&crate::SymbolLibrary>,
    context: SvgDesignContext<'_>,
) -> (f64, f64, f64, f64) {
    let mut extent = Extent::default();

    for component in &sheet.components {
        let resolved = find_resolved_symbol(component, resolved_symbols);
        let (min_x, min_y, max_x, max_y) =
            component_export_world_bounds(component, resolved, symbol_library, config);
        let unit = config.units_per_world;
        extent.include_box((min_x * unit, min_y * unit, max_x * unit, max_y * unit));
        if !symbol_writes_labels(component, resolved, symbol_library) {
            include_primitive_label_bounds(component, config, &mut extent);
        }
    }

    for wire in &sheet.wires {
        for point in &wire.points {
            let (x, y) = geometry::page(*point, config);
            extent.include(x, y);
        }
    }

    include_bus_bounds(&sheet.buses, &sheet.bus_taps, config, &mut extent);
    include_junction_bounds(&sheet.junctions, config, &mut extent);
    include_net_label_bounds(&sheet.net_labels, config, &mut extent);
    include_probe_bounds(&sheet.probes, config, &mut extent);
    include_design_note_bounds(
        document,
        &sheet.design_notes,
        config,
        context.view_path,
        &mut extent,
    );
    include_documentation_shape_bounds(sheet, config, &mut extent);

    if extent.is_empty() {
        (0.0, 0.0, 100.0, 100.0)
    } else {
        (extent.min_x, extent.min_y, extent.max_x, extent.max_y)
    }
}

fn component_export_world_bounds(
    component: &Component,
    resolved_symbol: Option<&ResolvedCellSymbol>,
    symbol_library: Option<&crate::SymbolLibrary>,
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
        for segment in crate::port_overlay::direction_segments(direction) {
            for point in [segment.start, segment.end] {
                let point = crate::port_overlay::transform_point(
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

/// What the placeholder says on the sheet.
///
/// The exported drawing is read on its own, so the gap has to be visible in
/// it — but in the sheet's own words and the sheet's own ink. `Resistor:
/// missing canonical SVG` named an internal asset, in alarm red, across a
/// drawing that is otherwise one colour. Which resolution step failed stays
/// on the group's `data-error`, for whoever is debugging the library.
fn symbol_resolution_error_label(component: &Component) -> String {
    format!("{}: no symbol", component.kind.display_name())
}

fn symbol_resolution_error_world_bounds(
    component: &Component,
    config: &SvgExportConfig,
    _reason: &str,
) -> (f64, f64, f64, f64) {
    const HALF_SIZE: f64 = 12.0;
    const TEXT_HORIZONTAL_PADDING: f64 = 2.0;
    let x = f64::from(component.pos.x);
    let y = f64::from(component.pos.y);
    let diagnostic = symbol_resolution_error_label(component);
    let unit = config.units_per_world.max(f64::EPSILON);
    // In world units: the label is value type, hung below the box.
    let size = InstanceLabel::Value.size(config) / unit;
    let text_half_width =
        (estimated_text_width(&diagnostic, size) + 2.0 * TEXT_HORIZONTAL_PADDING) / 2.0;
    (
        x - HALF_SIZE.max(text_half_width),
        y - HALF_SIZE,
        x + HALF_SIZE.max(text_half_width),
        y + HALF_SIZE + 1.3 * size,
    )
}

fn compatible_builtin_xspice_symbol<'a>(
    component: &Component,
    library: &'a crate::SymbolLibrary,
) -> Option<(&'a crate::symbols::Symbol, f32, f32)> {
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
    symbol_library: Option<&crate::SymbolLibrary>,
) {
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
            vector_port_stroke_width(component, config)
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
        for label in primitive_labels(component, config) {
            label
                .kind
                .write(svg, label.at, &label.text, label.align, config);
        }
    }

    svg.push_str("</g>\n");
}

/// A vector port's body, at bus weight, as the canvas draws it.
fn vector_port_stroke_width(component: &Component, config: &SvgExportConfig) -> Option<f64> {
    component.port_spec().and_then(|port| {
        port.vector()
            .map(|_| geometry::world(nominal_wu(StrokeRole::VectorPort), config))
    })
}

/// One instance label as the page sets it.
struct PrimitiveLabel {
    kind: InstanceLabel,
    at: (f64, f64),
    align: egui::Align2,
    text: String,
}

/// A primitive's name and value where the canvas puts them, through the
/// canvas's own layout evaluated in SVG units, turned with the instance, and
/// in the class the shared layout gives the value (a port's name is a net
/// name). A cell's labels are its symbol's; ground has none.
fn primitive_labels(component: &Component, config: &SvgExportConfig) -> Vec<PrimitiveLabel> {
    if component.kind == ComponentType::Ground {
        return Vec::new();
    }
    let (cx, cy) = geometry::page(component.pos, config);
    let layout = compute_label_layout(
        egui::Pos2::new(cx as f32, cy as f32),
        config.units_per_world as f32,
        component,
    );
    let at = |position: egui::Pos2| (f64::from(position.x), f64::from(position.y));
    let mut labels = Vec::new();
    if component
        .display_mode
        .show_name(SchematicParameterLabelVisibility::NamesAndValues)
        && !component.name.is_empty()
    {
        labels.push(PrimitiveLabel {
            kind: InstanceLabel::Name,
            at: at(layout.name_pos),
            align: layout.name_align,
            text: component.name.clone(),
        });
    }
    let value = crate::source_labels::component_value_label_cached(component);
    if component
        .display_mode
        .show_value(SchematicParameterLabelVisibility::NamesAndValues)
        && !value.is_empty()
    {
        labels.push(PrimitiveLabel {
            kind: InstanceLabel::value(
                rspice_design::schematic::label_layout::instance_value_class(component),
            ),
            at: at(layout.value_pos),
            align: layout.value_align,
            text: value.to_string(),
        });
    }
    labels
}

/// Whether the instance's authored symbol letters its own name and value,
/// which is the branch [`write_component`] takes for a cell it can draw.
fn symbol_writes_labels(
    component: &Component,
    resolved_symbol: Option<&ResolvedCellSymbol>,
    symbol_library: Option<&crate::SymbolLibrary>,
) -> bool {
    component.kind == ComponentType::CellInstance
        && symbol_library
            .and_then(|library| compatible_builtin_xspice_symbol(component, library))
            .is_none()
        && resolved_symbol.is_some_and(|symbol| {
            symbol.source() == ResolvedSymbolSource::Authored
                && resolved_symbol_world_bounds(component, symbol).is_some()
        })
}

/// The page box a primitive's labels take, so a value below a part is
/// never cut off at the sheet's edge.
fn include_primitive_label_bounds(
    component: &Component,
    config: &SvgExportConfig,
    extent: &mut Extent,
) {
    for label in primitive_labels(component, config) {
        extent.include_text(label.at, label.align, &label.text, label.kind.size(config));
    }
}

fn write_port_direction_overlay(
    svg: &mut String,
    component: &Component,
    rotation_degrees: i32,
    direction: rspice_design_model::port::PortDirection,
    config: &SvgExportConfig,
) {
    let (cx, cy) = geometry::page(component.pos, config);
    let point = |point| {
        let point = crate::port_overlay::transform_point(
            point,
            rotation_degrees,
            component.mirror_h,
            component.mirror_v,
        );
        (
            cx + f64::from(point.x) * config.units_per_world,
            cy + f64::from(point.y) * config.units_per_world,
        )
    };
    writeln!(
        svg,
        "<g class=\"port-direction-overlay\" data-direction=\"{}\">",
        direction.keyword()
    )
    .unwrap();
    let style = vector_port_stroke_width(component, config).map_or_else(String::new, |width| {
        format!(" style=\"stroke-width:{width}\"")
    });
    for segment in crate::port_overlay::direction_segments(direction) {
        let start = point(segment.start);
        let end = point(segment.end);
        writeln!(
            svg,
            "<line class=\"symbol\" x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\"{style}/>",
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
    let (cx, cy) = geometry::page(component.pos, config);
    let half = 12.0 * config.units_per_world;
    let x = cx - half;
    let y = cy - half;
    let size = half * 2.0;
    let kind = escape_xml(component.kind.display_name());
    let reason = escape_xml(reason);
    // The marks take the ink the drawing is in: a placeholder is part of the
    // drawing, and an exported schematic has no second colour to spend on an
    // alarm. The label sits outside the marks, in the value type.
    writeln!(
        svg,
        "<g class=\"symbol-resolution-error\" data-component-kind=\"{kind}\" data-error=\"{reason}\">"
    )
    .unwrap();
    writeln!(
        svg,
        "<g stroke=\"{stroke}\" stroke-width=\"{width}\" fill=\"none\">",
        stroke = config.symbol_color,
        width = geometry::world(nominal_wu(StrokeRole::Symbol), config)
    )
    .unwrap();
    writeln!(
        svg,
        "<rect x=\"{x}\" y=\"{y}\" width=\"{size}\" height=\"{size}\"/>"
    )
    .unwrap();
    writeln!(
        svg,
        "<path d=\"M {x} {y} L {} {} M {} {y} L {x} {}\"/>",
        x + size,
        y + size,
        x + size,
        y + size
    )
    .unwrap();
    svg.push_str("</g>\n");
    write_text_svg(
        svg,
        InstanceLabel::Value.class(),
        (cx, y + size),
        &symbol_resolution_error_label(component),
        egui::Align2::CENTER_TOP,
        InstanceLabel::Value.size(config),
    );
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
        let mut schematic = Schematic::default();
        schematic.document_mut_for_test().design_notes.push(
            rspice_design::schematic::design_note::DesignNote::new(
                19,
                rspice_design_model::Point::new(30, 40),
                rspice_design::schematic::design_note::DesignNoteKind::ReviewNote,
                "Check A < B & C\nSecond line",
            )
            .unwrap(),
        );

        let svg = export_to_svg(schematic.document(), &SvgExportConfig::default());
        assert!(svg.contains("design-note-review-note"));
        assert!(svg.contains("data-object-id=\"19\""));
        assert!(svg.contains("data-review-id=\"NOTE-0019\""));
        assert!(svg.contains("data-review-state=\"open\""));
        assert!(svg.contains("Check A &lt; B &amp; C"));
        assert!(svg.contains("<tspan"));
        assert!(svg.contains("Second line"));
        assert!(!svg.contains("Check A < B & C"));
        assert!(!svg.contains("viewBox=\"-20 -10 140 150\""));
    }

    #[test]
    fn property_display_export_uses_the_explicit_active_view_context() {
        let mut schematic = Schematic::default();
        schematic.document_mut_for_test().design_notes.push(
            rspice_design::schematic::design_note::DesignNote::new(
                20,
                rspice_design_model::Point::new(5, 6),
                rspice_design::schematic::design_note::DesignNoteKind::PropertyDisplay,
                "${view} / ${component_count} components",
            )
            .unwrap(),
        );

        let svg = export_to_svg_with_resolved_symbol_entries(
            schematic.document(),
            &SvgExportConfig::default(),
            &[],
            SvgDesignContext {
                view_path: "user/top/schematic",
                ..SvgDesignContext::default()
            },
        );

        assert!(svg.contains("user/top/schematic / 0 components"));
        assert!(!svg.contains(">schematic / 0 components"));
    }
    use rspice_design::{
        library::{Cell, Library, LibraryCatalog, View, ViewType},
        schematic::{
            bus::{Bus, BusDeclaration, BusSlice, BusTap, BusTapOrientation},
            component::LibraryCellInstance,
            component_edit::ComponentPlacement,
            documentation_shape::{DocumentationShape, DocumentationShapeGeometry},
            net_label::Junction,
            owned::Schematic,
            rotation::Rotation,
        },
        symbol::{SymbolDocument, SymbolPin, SymbolShape},
    };
    use rspice_design_model::{
        Point,
        port::{PortDirection, PortSpec},
    };

    fn placement(position: Point) -> ComponentPlacement {
        ComponentPlacement {
            position,
            rotation: Rotation::R0,
            mirror_h: false,
        }
    }
    use std::collections::HashMap;

    #[test]
    fn svg_export_uses_canonical_assets_for_every_standard_component() {
        let library = crate::SymbolLibrary::load_embedded().expect("canonical symbols");
        let mut schematic = Schematic::default();
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
            schematic
                .document_mut_for_test()
                .components
                .push(Component::new(
                    index as u64 + 1,
                    kind,
                    Point::new(index as i32 * 100, 0),
                ));
        }

        let svg = export_to_svg(schematic.document(), &SvgExportConfig::default());

        assert!(!svg.contains("symbol-resolution-error"));
        assert_eq!(
            svg.matches(r#"<path class="symbol""#).count(),
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
        assert!(!missing_library.contains(r#"<path class="symbol""#));

        let mut unknown_variant = Schematic::default();
        let mut diode = Component::new(2, ComponentType::Diode, Point::origin());
        diode.symbol_variant = Some("missing-variant".to_owned());
        unknown_variant
            .document_mut_for_test()
            .components
            .push(diode);
        let svg = export_to_svg(unknown_variant.document(), &SvgExportConfig::default());
        assert!(svg.contains("symbol-resolution-error"));
    }

    #[test]
    fn export_bounds_follow_rotated_mirrored_canonical_variant_target() {
        let library = crate::SymbolLibrary::load_embedded().expect("canonical symbols");
        let mut component = Component::new(1, ComponentType::VoltageSource, Point::new(140, -35))
            .with_rotation(rspice_design::schematic::rotation::Rotation::R90)
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

        let mut schematic = Schematic::default();
        schematic.document_mut_for_test().components.push(component);
        let config = SvgExportConfig {
            units_per_world: 1.0,
            margin: 0.0,
            ..SvgExportConfig::default()
        };
        // The symbol's own box: since C-10 the page's bounds also hold the
        // instance's labels, which this transform says nothing about.
        let bounds = component_export_world_bounds(
            &schematic.document().components[0],
            None,
            Some(&library),
            &config,
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
        let library = crate::SymbolLibrary::load_embedded().expect("canonical symbols");
        let mut schematic = Schematic::default();
        schematic
            .document_mut_for_test()
            .components
            .push(Component::new(
                1,
                ComponentType::CellInstance,
                Point::origin(),
            ));
        let config = SvgExportConfig {
            units_per_world: 1.0,
            margin: 0.0,
            ..SvgExportConfig::default()
        };

        let component = schematic
            .document()
            .components
            .first()
            .expect("unresolved cell");
        let expected = symbol_resolution_error_world_bounds(component, &config, "unresolved cell");
        let bounds = calculate_bounds_with_resolved_symbols(
            schematic.document(),
            schematic.document(),
            &config,
            &[],
            Some(&library),
            SvgDesignContext::default(),
        );

        assert_eq!(bounds, expected);
        assert!(bounds.0 < -12.0 && bounds.2 > 12.0);
        assert!(bounds.3 > 12.0 + InstanceLabel::Value.size(&config));
    }

    #[test]
    fn export_viewbox_contains_a_long_resolution_diagnostic_at_sheet_edge() {
        let mut component =
            Component::new(1, ComponentType::BehavioralSource, Point::new(-400, -300));
        component.symbol_variant = Some("missing-authored-variant".to_owned());
        let mut schematic = Schematic::default();
        schematic.document_mut_for_test().components.push(component);
        let config = SvgExportConfig {
            units_per_world: 1.0,
            margin: 0.0,
            ..SvgExportConfig::default()
        };
        let expected = symbol_resolution_error_world_bounds(
            schematic.document().components.first().expect("component"),
            &config,
            "missing canonical SVG",
        );
        let diagnostic = symbol_resolution_error_label(
            schematic.document().components.first().expect("component"),
        );
        let diagnostic = diagnostic.as_str();
        let svg = export_to_svg(schematic.document(), &config);
        let width = expected.2 - expected.0;
        let height = expected.3 - expected.1;

        assert!(width >= estimated_text_width(diagnostic, InstanceLabel::Value.size(&config)));
        assert!(expected.3 > -300.0 + 12.0 + InstanceLabel::Value.size(&config));
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
        let mut schematic = Schematic::default();
        schematic.add_library_cell_component(placement(Point::origin()), binding);
        let libraries = LibraryCatalog::default();
        let buffers: HashMap<String, Schematic> = HashMap::new();
        let resolver = SymbolResolver::new(&libraries, &buffers);

        let svg = export_to_svg_with_symbol_resolver(
            schematic.document(),
            &SvgExportConfig::default(),
            &resolver,
        );

        assert!(svg.contains("symbol-resolution-error"));
        assert!(svg.contains("unresolved cell"));
        assert!(!svg.contains("symbol-pin-label"));
    }

    #[test]
    fn svg_port_uses_canonical_body_and_transformed_direction_overlay() {
        let config = SvgExportConfig {
            units_per_world: 1.0,
            ..SvgExportConfig::default()
        };
        let component = Component::new(1, ComponentType::Port, Point::new(100, 80))
            .with_rotation(rspice_design::schematic::rotation::Rotation::R90)
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
                overlay.matches(r#"<line class="symbol""#).count(),
                segment_count
            );
        }

        let library = crate::SymbolLibrary::load_embedded().expect("canonical symbols");
        let body = library
            .get_with_rotation_variant(ComponentType::Port, 90, None)
            .expect("canonical port body")
            .0;
        assert_eq!(body.paths.len(), 1);
        assert!(!body.paths[0].filled);
        assert!(!body.paths[0].commands.is_empty());

        let mut scalar_schematic = Schematic::default();
        scalar_schematic
            .document_mut_for_test()
            .components
            .push(component.clone());
        let scalar_svg = export_to_svg(scalar_schematic.document(), &config);
        assert!(!scalar_svg.contains(r#"style="stroke-width:1.2""#));

        let mut vector = component;
        vector.value = "DATA[7:0]".to_owned();
        let mut schematic = Schematic::default();
        schematic.document_mut_for_test().components.push(vector);
        let svg = export_to_svg(schematic.document(), &config);
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

        let mut schematic = Schematic::default();
        let id =
            schematic.add_library_cell_component(placement(Point::new(100, 100)), binding.clone());
        let component = schematic
            .document_mut_for_test()
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

        let mut libraries = LibraryCatalog::default();
        let mut library = Library::new("work");
        let mut cell = Cell::new("amp");
        let mut symbol_view = View::new("symbol", ViewType::Symbol);
        document
            .store_in_view(&mut symbol_view)
            .expect("symbol stores");
        cell.add_view(symbol_view);
        library.add_cell(cell);
        libraries.add_library(library);
        let resolved = SymbolResolver::<rspice_design::schematic::owned::Schematic>::new(
            &libraries,
            &HashMap::new(),
        )
        .resolve_binding(&binding)
        .expect("authored symbol resolves");

        let svg = export_to_svg_with_resolved_symbols(
            schematic.document(),
            &SvgExportConfig::default(),
            &[(id, resolved)],
        );

        assert!(
            svg.contains(r#"<path class="symbol" d="M 900 900 L 1100 900 L 1100 1100"/>"#),
            "authored body polyline should be exported"
        );
        // The anchors are (800, 700) and (800, 1300); an unturned run starts
        // there, centred on it: the baseline sits 0.375 em below.
        assert!(
            svg.contains(r#"<text class="name" x="800" y="737.5" text-anchor="start""#)
                && svg.contains(">XAMP</text>"),
            "instance name should use the authored name anchor: {svg}"
        );
        assert!(
            svg.contains(r#"<text class="value" x="800" y="1333.75" text-anchor="start""#)
                && svg.contains(">gain_stage</text>"),
            "instance value should use the authored value anchor: {svg}"
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

        let mut schematic = Schematic::default();
        let first_id = schematic
            .add_library_cell_component(placement(Point::new(100, 100)), first_binding.clone());
        schematic
            .document_mut_for_test()
            .components
            .iter_mut()
            .find(|component| component.id == first_id)
            .expect("first component exists")
            .name = "XIN".to_owned();
        let second_id = schematic
            .add_library_cell_component(placement(Point::new(200, 100)), second_binding.clone());
        schematic
            .document_mut_for_test()
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
        let mut libraries = LibraryCatalog::default();
        let mut library = Library::new("work");
        let mut cell = Cell::new("amp");
        let mut symbol_view = View::new("symbol", ViewType::Symbol);
        document
            .store_in_view(&mut symbol_view)
            .expect("symbol stores");
        cell.add_view(symbol_view);
        library.add_cell(cell);
        libraries.add_library(library);
        let buffers: HashMap<String, Schematic> = HashMap::new();
        let resolver = SymbolResolver::new(&libraries, &buffers);

        let svg = export_to_svg_with_symbol_resolver(
            schematic.document(),
            &SvgExportConfig::default(),
            &resolver,
        );

        // Each lead runs from the saved terminal all the way to the body it
        // belongs to, so neither instance exports a pin left floating.
        assert!(
            svg.contains(r#"<line class="symbol" x1="600" y1="1000" x2="900" y2="1000"/>"#),
            "first instance should export its saved IN terminal"
        );
        assert!(
            svg.contains(r#"<line class="symbol" x1="2400" y1="1000" x2="2100" y2="1000"/>"#),
            "second instance should export its saved OUT terminal, not reuse the first interface"
        );
    }

    #[test]
    fn svg_export_escapes_canonical_component_labels() {
        let mut schematic = Schematic::default();
        let id =
            schematic.add_component(ComponentType::Resistor, placement(Point::new(10, 10)), None);
        let component = schematic
            .document_mut_for_test()
            .components
            .iter_mut()
            .find(|component| component.id == id)
            .expect("component exists");
        component.name = "R<&>".to_owned();
        component.value = "1k & 2k".to_owned();

        let svg = export_to_svg(schematic.document(), &SvgExportConfig::default());

        assert!(svg.contains("R&lt;&amp;&gt;"));
        assert!(svg.contains("1k &amp; 2k"));
        assert!(!svg.contains("R<&>"));
        assert!(!svg.contains("1k & 2k"));
    }

    /// A primitive's name and value export where the canvas puts them,
    /// through the canvas's own layout: centred above an unturned resistor,
    /// and ending left of a turned one, in the instance type. Before, both
    /// sat at fixed offsets above and below the origin whatever the turn.
    #[test]
    fn primitive_labels_export_through_the_canvas_layout() {
        let config = SvgExportConfig::default();
        let name_line = |rotation: Rotation| {
            let mut resistor = Component::new(1, ComponentType::Resistor, Point::new(100, 100));
            resistor.name = "R1".to_owned();
            resistor.value = "10k".to_owned();
            resistor.rotation = rotation;
            let mut svg = String::new();
            write_component(&mut svg, &resistor, &config, None, None);
            let layout = crate::view::drawing::compute_label_layout(
                egui::Pos2::new(1000.0, 1000.0),
                config.units_per_world as f32,
                &resistor,
            );
            let line = svg
                .lines()
                .find(|line| line.ends_with(">R1</text>"))
                .unwrap_or_else(|| panic!("no name in {svg}"))
                .to_owned();
            (line, layout)
        };
        let (flat, layout) = name_line(Rotation::R0);
        assert!(flat.contains(r#"text-anchor="middle""#), "{flat}");
        assert!(
            flat.contains(&format!(
                r#"x="{}" y="{}""#,
                f64::from(layout.name_pos.x),
                f64::from(layout.name_pos.y) - 0.275 * 100.0
            )),
            "the name stands on the canvas's anchor above the body: {flat}"
        );
        assert!(flat.starts_with(r#"<text class="name""#), "{flat}");
        let svg = export_to_svg(&SchematicDocument::default(), &config);
        assert!(
            svg.contains(".name { font-family: 'IBM Plex Sans', 'Helvetica Neue', Arial, sans-serif; font-size: 100px; font-weight: 500;"),
            "the name class carries the instance type in world units: {svg}"
        );
        let (turned, _) = name_line(Rotation::R90);
        assert!(turned.contains(r#"text-anchor="end""#), "{turned}");
    }

    /// A port's name exports in the net-name class, the type a connector's
    /// name takes (D1 section 1), and its box is the net name's. It was set
    /// in the instance value's class.
    #[test]
    fn a_port_name_exports_as_a_net_name() {
        let config = SvgExportConfig::default();
        let mut port = Component::new(1, ComponentType::Port, Point::new(100, 100));
        port.value = "IN".to_owned();
        port.params = "dir=in".to_owned();
        let mut svg = String::new();
        write_component(&mut svg, &port, &config, None, None);
        let line = svg
            .lines()
            .find(|line| line.ends_with(">IN</text>"))
            .unwrap_or_else(|| panic!("no port name in {svg}"));
        assert!(line.starts_with(r#"<text class="net-name""#), "{line}");
        let labels = primitive_labels(&port, &config);
        let name = labels
            .iter()
            .find(|label| label.text == "IN")
            .expect("the port's name is a label");
        assert!(
            (name.kind.size(&config)
                - f64::from(crate::view::net_labels::LABEL_FONT_WORLD_SIZE)
                    * config.units_per_world)
                .abs()
                < 1e-9
        );
    }

    #[test]
    fn svg_export_background_is_explicit_and_can_be_transparent() {
        let schematic = Schematic::default();
        let paper = export_to_svg(schematic.document(), &SvgExportConfig::default());
        assert!(
            paper.contains("fill=\"#FFFFFF\""),
            "the default page is paper"
        );
        let dark = export_to_svg(
            schematic.document(),
            &SvgExportConfig {
                background_color: Some(SvgColor::rgb(0x1A, 0x1A, 0x1A)),
                ..SvgExportConfig::default()
            },
        );
        assert!(dark.contains("fill=\"#1A1A1A\""));

        let transparent = export_to_svg(
            schematic.document(),
            &SvgExportConfig {
                background_color: None,
                ..SvgExportConfig::default()
            },
        );
        assert!(!transparent.contains("<rect"));

        let print_safe = export_to_svg(
            schematic.document(),
            &SvgExportConfig {
                background_color: Some(SvgColor::rgb(0xFF, 0xFF, 0xFF)),
                ..SvgExportConfig::default()
            },
        );
        assert!(print_safe.contains("fill=\"#FFFFFF\""));
    }

    #[test]
    fn svg_export_emits_explicit_junction_and_includes_marker_extents() {
        let mut schematic = Schematic::default();
        schematic
            .document_mut_for_test()
            .junctions
            .push(Junction::new(19, Point::new(7, 11)));
        let config = SvgExportConfig {
            margin: 0.0,
            ..SvgExportConfig::default()
        };

        let svg = export_to_svg(schematic.document(), &config);

        // D1 section 11: the light wire ink, and a dot of radius 1.5 world
        // units (2.7 wires across).
        assert!(svg.contains(r#".junction { fill: #4E3400; stroke: none; }"#));
        assert!(svg.contains(r#"<circle class="junction" cx="70" cy="110" r="15"/>"#));
        assert!(svg.contains(r#"width="30" height="30" viewBox="55 95 30 30""#));
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
        let mut schematic = Schematic::default();
        schematic.document_mut_for_test().buses.push(bus);
        schematic.document_mut_for_test().bus_taps.push(tap);
        let config = SvgExportConfig {
            margin: 0.0,
            ..SvgExportConfig::default()
        };

        let svg = export_to_svg(schematic.document(), &config);
        let (min_x, min_y, max_x, max_y) = calculate_bounds_with_resolved_symbols(
            schematic.document(),
            schematic.document(),
            &config,
            &[],
            None,
            SvgDesignContext::default(),
        );

        assert_eq!(svg.matches(r#"<path class="bus""#).count(), 3);
        assert!(svg.contains(r#"<path class="tap" d="M 100 0 L 100 100"/>"#));
        assert!(svg.contains(r#">DATA[7:0]</text>"#));
        assert!(svg.contains(r#">DATA[3]</text>"#));
        assert!(min_x <= -4.0 && min_y <= -20.0);
        assert!(max_x > 270.0 && max_y >= 100.0);
    }

    #[test]
    fn svg_export_preserves_documentation_shape_identity_layer_and_kind() {
        let mut schematic = Schematic::default();
        schematic.document_mut_for_test().documentation_shapes = vec![
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

        let first = export_to_svg(schematic.document(), &SvgExportConfig::default());
        let second = export_to_svg(schematic.document(), &SvgExportConfig::default());

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
