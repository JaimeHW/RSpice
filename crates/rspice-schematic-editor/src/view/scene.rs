//! Active-sheet scene painting over current document/session and prepared overlays.

use super::{
    design_notes::{conservative_world_bounds as design_note_world_bounds, draw_design_note},
    design_view::{DesignView, design_note_visible},
    documentation_shapes::{draw_documentation_shape, world_bounds as documentation_shape_bounds},
    drawing::{
        ConductorInk, JunctionPaint, ProbeVisualStatus, bus_band_width_pt, bus_tap_width_pt,
        chain_conductors, draw_bus, draw_bus_tap, draw_component, draw_conductor, draw_hover_halo,
        draw_hover_outline, draw_junction, draw_open_pin_marks, draw_probe, probe_world_bounds,
    },
    net_labels::{NetLabelPaint, draw_net_label_attached, world_bounds as net_label_world_bounds},
    pointer_target::PointerTarget,
    symbol_context::{SchematicSymbolContext, SelectionWindow},
    viewport::Viewport,
};
use crate::{
    session::{
        EditorSession,
        selection::SchematicSelectionFilter,
        visibility::{SchematicNetHighlighting, SchematicParameterLabelVisibility},
    },
    symbols::SymbolLibrary,
};
use egui::{Painter, Rect, Stroke};
use rspice_design::schematic::design_note::DesignNoteRenderContext;
use rspice_design::schematic::probe::SchematicProbe;
use rspice_design_model::Point;
use std::collections::{BTreeMap, HashMap};

mod annotations;
mod empty;
mod focus;
use annotations::draw_operating_point_annotations;
pub use annotations::{BadgeSubject, OperatingPointCanvasAnnotation};
pub use empty::draw_empty_hint;
pub use focus::{draw_keyboard_focus, keyboard_focus_matches_selection};

const CULL_MARGIN: f32 = 160.0;

pub struct SceneView<'a> {
    pub design: DesignView<'a>,
    pub editor: &'a EditorSession,
    pub net_highlighting: SchematicNetHighlighting,
    pub parameter_labels: SchematicParameterLabelVisibility,
    pub hover_wire_vertex: Option<(i32, i32)>,
    /// What a rubber band may select, so its live preview honours it.
    pub selection_filter: SchematicSelectionFilter,
    /// What a click here would take, pre-highlighted. The host resolves it
    /// once per frame through the same pointer target a click uses, and
    /// leaves it empty during a drag, a band, a popup or touch input.
    pub hover: Option<PointerTarget>,
}

/// The host resolves run provenance and probe materialization. The canvas only
/// consumes display values and narrow probe-status/selected-trace lookups.
pub struct SceneOverlays<'a> {
    pub net_class_colors: HashMap<u64, egui::Color32>,
    pub operating_point: Vec<OperatingPointCanvasAnnotation>,
    pub probe_status: &'a dyn Fn(&SchematicProbe) -> ProbeVisualStatus,
    pub selected_trace_expression: &'a dyn Fn() -> Option<&'a str>,
    /// Each instance's open terminals (nothing meets them), which take the
    /// open-pin mark.
    pub open_pins: &'a HashMap<u64, Vec<Point>>,
}

pub struct SceneSymbols<'a> {
    pub library: Option<&'a SymbolLibrary>,
    pub context: &'a SchematicSymbolContext,
}

pub fn draw_content(
    painter: &Painter,
    available: Rect,
    viewport: &Viewport,
    view: &SceneView<'_>,
    symbols: SceneSymbols<'_>,
    overlays: SceneOverlays<'_>,
    view_path: impl Fn() -> String,
) {
    let symbol_library = symbols.library;
    let symbol_context = symbols.context;
    // A live rubber band draws what its release will select, through the
    // same window and the same selection filter.
    let band = view.editor.selection_rect.is_active().then(|| {
        SelectionWindow::from_band(
            view.editor.selection_rect.start,
            view.editor.selection_rect.current,
            view.design.document.document_policy.selection_crossing,
        )
    });
    let filter = view.selection_filter;
    let band_for = |family: bool| band.filter(|_| family);
    let (band_wires, band_instances, band_labels, band_annotations) = (
        band_for(filter.wires),
        band_for(filter.instances),
        band_for(filter.labels),
        band_for(filter.annotations),
    );

    // Viewport culling: only elements whose bounds intersect the visible
    // world rect are transformed and tessellated.
    let (wx0, wy0, wx1, wy1) = viewport.visible_world_rect(CULL_MARGIN);
    let cache = view.design.canvas_cache;
    let visible_wire_indices = cache
        .map(|cache| cache.wire_indices_in_world_rect(wx0, wy0, wx1, wy1))
        .unwrap_or_else(|| (0..view.design.document.wires.len()).collect());

    // A hovered conductor's halo goes beneath every conductor.
    let selection = &view.editor.selection;
    draw_conductor_hover(painter, viewport, view, |target| match target {
        PointerTarget::Wire(id) => selection.wires.contains(&id),
        PointerTarget::Bus(id) => selection.has_bus(id),
        PointerTarget::BusTap(id) => selection.has_bus_tap(id),
        _ => false,
    });

    for bus in &view.design.document.buses {
        if !view.design.object_is_visible(bus.id) {
            continue;
        }
        if !polyline_intersects_view(&bus.points, wx0, wy0, wx1, wy1) {
            continue;
        }
        let selected = view.editor.selection.has_bus(bus.id)
            || band_wires.is_some_and(|band| band.takes_polyline(&bus.points));
        draw_bus(painter, viewport, bus, selected);
    }

    // Wires of one style that meet end to end are painted as one path, so the
    // corner two wires form is a mitered join. Plain wires go down first,
    // then class colours, highlighted nets and the selection, so emphasis
    // stays on top.
    type ConductorStyle = (u8, [u8; 4]);
    let mut conductor_groups: BTreeMap<ConductorStyle, (ConductorInk, Vec<&[Point]>)> =
        BTreeMap::new();
    // The ink of every wire drawn other than plain, by wire index, so a
    // junction takes the colour and the width of what it joins.
    let mut inked_wires: HashMap<usize, ConductorInk> = HashMap::new();
    for index in visible_wire_indices {
        let Some(wire) = view.design.document.wires.get(index) else {
            continue;
        };
        if !view.design.object_is_visible(wire.id) {
            continue;
        }
        if let Some((min, max)) = cache.and_then(|c| c.wire_bounds.get(index))
            && ((max.x as f32) < wx0
                || (min.x as f32) > wx1
                || (max.y as f32) < wy0
                || (min.y as f32) > wy1)
        {
            continue;
        }
        let is_selected = view.editor.selection.wires.contains(&wire.id)
            || band_wires.is_some_and(|band| band.takes_polyline(&wire.points));

        let net_ink = match view.net_highlighting {
            policy
                if policy.lights_selected_net()
                    && view.editor.net_highlight.is_wire_highlighted(wire.id) =>
            {
                ConductorInk::Highlighted
            }
            SchematicNetHighlighting::NetClassColors => overlays
                .net_class_colors
                .get(&wire.id)
                .map_or(ConductorInk::Plain, |colour| ConductorInk::Class(*colour)),
            SchematicNetHighlighting::SelectedNet
            | SchematicNetHighlighting::SelectedNetAcrossHierarchy
            | SchematicNetHighlighting::Off => ConductorInk::Plain,
        };
        let ink = if is_selected {
            ConductorInk::Selected
        } else {
            net_ink
        };
        if ink != ConductorInk::Plain {
            inked_wires.insert(index, ink);
        }
        let colour_key = match ink {
            ConductorInk::Class(colour) => colour.to_array(),
            ConductorInk::Plain | ConductorInk::Highlighted | ConductorInk::Selected => [0; 4],
        };
        conductor_groups
            .entry((ink.rank(), colour_key))
            .or_insert_with(|| (ink, Vec::new()))
            .1
            .push(wire.points.as_slice());
    }
    for (_, (ink, polylines)) in conductor_groups {
        for chain in chain_conductors(polylines) {
            draw_conductor(painter, viewport, &chain, ink);
        }
    }

    for tap in &view.design.document.bus_taps {
        if !view.design.object_is_visible(tap.id) {
            continue;
        }
        let route = crate::bus_geometry::bus_tap_route_points(tap);
        let Some(first) = route.first() else {
            continue;
        };
        let (mut min_x, mut max_x, mut min_y, mut max_y) = (first.x, first.x, first.y, first.y);
        for point in &route[1..] {
            min_x = min_x.min(point.x);
            max_x = max_x.max(point.x);
            min_y = min_y.min(point.y);
            max_y = max_y.max(point.y);
        }
        if (max_x as f32) < wx0
            || (min_x as f32) > wx1
            || (max_y as f32) < wy0
            || (min_y as f32) > wy1
        {
            continue;
        }
        let selected = view.editor.selection.has_bus_tap(tap.id)
            || band_wires.is_some_and(|band| band.takes_polyline(&route));
        draw_bus_tap(painter, viewport, tap, selected);
    }

    for component in &view.design.document.components {
        if !view.design.object_is_visible(component.id) {
            continue;
        }
        let (min, max) = symbol_context.component_bounds(component);
        if (max.x as f32) < wx0
            || (min.x as f32) > wx1
            || (max.y as f32) < wy0
            || (min.y as f32) > wy1
        {
            continue;
        }
        let is_selected = view.editor.selection.components.contains(&component.id)
            || band_instances.is_some_and(|band| band.takes_bounds(min, max));

        draw_component(
            painter,
            viewport,
            component,
            is_selected,
            symbol_library,
            symbol_context,
            view.parameter_labels,
        );
        if let Some(open) = overlays.open_pins.get(&component.id) {
            let palette = rspice_ui_kit::tokens::active_palette();
            draw_open_pin_marks(
                painter,
                viewport,
                open,
                if is_selected {
                    palette.canvas_select
                } else {
                    palette.symbol
                },
            );
        }
        // The hover outline is the pick bounds the click tests.
        if !is_selected && view.hover == Some(PointerTarget::Component(component.id)) {
            draw_hover_outline(
                painter,
                Rect::from_two_pos(
                    viewport.schematic_to_screen(min),
                    viewport.schematic_to_screen(max),
                ),
            );
        }
    }

    for junction in &view.design.document.junctions {
        if !view.design.object_is_visible(junction.id) {
            continue;
        }
        let (jx, jy) = (junction.pos.x as f32, junction.pos.y as f32);
        if jx < wx0 || jx > wx1 || jy < wy0 || jy > wy1 {
            continue;
        }
        draw_junction(
            painter,
            viewport,
            junction.pos,
            JunctionPaint {
                selected: view.editor.selection.has_junction(junction.pos)
                    || band_wires.is_some_and(|band| band.takes_point(junction.pos)),
                hovered: view.hover == Some(PointerTarget::Junction(junction.pos))
                    || view.hover_wire_vertex == Some((junction.pos.x, junction.pos.y)),
                incident: incident_ink(view, cache, &inked_wires, junction.pos),
            },
        );
    }

    draw_operating_point_annotations(
        painter,
        available,
        viewport,
        &view.design,
        symbol_context,
        &overlays.operating_point,
    );

    // Presentation geometry is a background documentation layer. It remains
    // selectable, but is intentionally painted below authored text and names.
    for shape in &view.design.document.documentation_shapes {
        if !view.design.object_is_visible(shape.id) {
            continue;
        }
        let (min, max) = documentation_shape_bounds(shape);
        if (max.x as f32) < wx0
            || (min.x as f32) > wx1
            || (max.y as f32) < wy0
            || (min.y as f32) > wy1
        {
            continue;
        }
        let selected = view.editor.selection.has_documentation_shape(shape.id)
            || band_annotations.is_some_and(|band| band.takes_documentation_shape(shape));
        draw_documentation_shape(
            painter,
            viewport,
            shape,
            selected,
            view.hover == Some(PointerTarget::DocumentationShape(shape.id)),
        );
    }

    // Net labels are authored text, not derived annotations. Paint them after
    // junction and OP overlays so the source net name always remains legible.
    for label in &view.design.document.net_labels {
        if !view.design.object_is_visible(label.id) {
            continue;
        }
        let (min, max) = net_label_world_bounds(label);
        if (max.x as f32) < wx0
            || (min.x as f32) > wx1
            || (max.y as f32) < wy0
            || (min.y as f32) > wy1
        {
            continue;
        }
        let selected = view.editor.selection.has_net_label(label.id)
            || band_labels.is_some_and(|band| band.takes_bounds(min, max));
        // The attachment diamond follows the junction precedence; the text
        // keeps its own colour.
        let attachment = if selected {
            ConductorInk::Selected
        } else {
            incident_ink(view, cache, &inked_wires, label.pos)
        };
        draw_net_label_attached(
            painter,
            viewport,
            label,
            NetLabelPaint {
                selected,
                hovered: view.hover == Some(PointerTarget::NetLabel(label.id)),
                preview: false,
                attachment: (attachment != ConductorInk::Plain).then_some(attachment),
            },
        );
    }

    // Documentation objects are painted above electrical names but below
    // validation markers. They never participate in conductor rendering.
    for note in &view.design.document.design_notes {
        if !view.design.object_is_visible(note.id) {
            continue;
        }
        if !design_note_visible(note, view.design.review_markers) {
            continue;
        }
        let (min, max) = design_note_world_bounds(note);
        if (max.x as f32) < wx0
            || (min.x as f32) > wx1
            || (max.y as f32) < wy0
            || (min.y as f32) > wy1
        {
            continue;
        }
        let selected = view.editor.selection.has_design_note(note.id)
            || band_annotations.is_some_and(|band| band.takes_bounds(min, max));
        draw_design_note(
            painter,
            viewport,
            note,
            &DesignNoteRenderContext::for_document(view.design.document, &view_path()),
            selected,
            view.hover == Some(PointerTarget::DesignNote(note.id)),
        );
    }

    // Probe flags are durable output intent and remain visible above authored
    // conductor/text layers. Their reference is the exact bound expression
    // when resolved, otherwise the stable unbound P<n> marker identity.
    for probe in &view.design.document.probes {
        if !view.design.object_is_visible(probe.id) {
            continue;
        }
        let (min, max) = probe_world_bounds(probe);
        if (max.x as f32) < wx0
            || (min.x as f32) > wx1
            || (max.y as f32) < wy0
            || (min.y as f32) > wy1
        {
            continue;
        }
        let mut selected = view.editor.selection.has_probe(probe.id);
        if !selected {
            selected = (overlays.selected_trace_expression)()
                .and_then(|trace| {
                    probe
                        .source_expression
                        .as_deref()
                        .map(|expression| (trace, expression))
                })
                .is_some_and(|(trace, expression)| {
                    normalized_probe_expression(trace) == normalized_probe_expression(expression)
                });
        }
        selected = selected || band_annotations.is_some_and(|band| band.takes_bounds(min, max));
        draw_probe(
            painter,
            viewport,
            probe,
            (overlays.probe_status)(probe),
            selected,
            view.hover == Some(PointerTarget::Probe(probe.id)),
        );
    }

    // The wire-vertex drag handle: a fixed-size ring, the same at every zoom.
    if let Some((hx, hy)) = view.hover_wire_vertex {
        let hover_pos = Point::new(hx, hy);
        let is_junction = view.design.active_junction_at(hover_pos).is_some();
        if !is_junction {
            painter.circle_stroke(
                viewport.schematic_to_screen(hover_pos),
                VERTEX_HANDLE_RADIUS,
                Stroke::new(
                    1.0 / painter.pixels_per_point(),
                    rspice_ui_kit::tokens::active_palette().canvas_select,
                ),
            );
        }
    }
}

/// The wire-vertex drag handle's ring, in points.
const VERTEX_HANDLE_RADIUS: f32 = 3.0;

/// The hover pre-highlight for a conductor or an instance, drawn beneath the
/// conductor it outlines or after the instance family. A hovered object
/// that is already selected draws nothing extra.
fn draw_conductor_hover(
    painter: &Painter,
    viewport: &Viewport,
    view: &SceneView<'_>,
    selected: impl Fn(PointerTarget) -> bool,
) {
    let Some(target) = view.hover.filter(|target| !selected(*target)) else {
        return;
    };
    let document = view.design.document;
    let ppp = painter.pixels_per_point();
    match target {
        PointerTarget::Wire(id) => {
            if let Some(wire) = document.wires.iter().find(|wire| wire.id == id) {
                let width = super::drawing::conductor_width_pt(viewport.zoom, ppp, false);
                draw_hover_halo(painter, viewport, &wire.points, width);
            }
        }
        PointerTarget::Bus(id) => {
            if let Some(bus) = document.buses.iter().find(|bus| bus.id == id) {
                let width = bus_band_width_pt(viewport.zoom, ppp);
                draw_hover_halo(painter, viewport, &bus.points, width);
            }
        }
        PointerTarget::BusTap(id) => {
            if let Some(tap) = document.bus_taps.iter().find(|tap| tap.id == id) {
                let route = crate::bus_geometry::bus_tap_route_points(tap);
                draw_hover_halo(
                    painter,
                    viewport,
                    &route,
                    bus_tap_width_pt(viewport.zoom, ppp),
                );
            }
        }
        PointerTarget::Component(_)
        | PointerTarget::DesignNote(_)
        | PointerTarget::DocumentationShape(_)
        | PointerTarget::Probe(_)
        | PointerTarget::NetLabel(_)
        | PointerTarget::Junction(_) => {}
    }
}

/// The most emphasized ink among the drawn wires that pass through `point`.
/// Candidates come from the cache's spatial bins when it is fresh.
fn incident_ink(
    view: &SceneView<'_>,
    cache: Option<&crate::session::canvas_cache::CanvasCache>,
    inked_wires: &HashMap<usize, ConductorInk>,
    point: Point,
) -> ConductorInk {
    if inked_wires.is_empty() {
        return ConductorInk::Plain;
    }
    let wires = &view.design.document.wires;
    let candidates: Vec<usize> = cache.map_or_else(
        || inked_wires.keys().copied().collect(),
        |cache| cache.wire_indices_at_point(point),
    );
    candidates
        .into_iter()
        .filter_map(|index| inked_wires.get(&index).map(|ink| (index, *ink)))
        .filter(|(index, _)| {
            wires
                .get(*index)
                .is_some_and(|wire| wire.contains_point(point))
        })
        .fold(ConductorInk::Plain, |ink, (_, other)| ink.max(other))
}

fn polyline_intersects_view(points: &[Point], wx0: f32, wy0: f32, wx1: f32, wy1: f32) -> bool {
    let Some(first) = points.first() else {
        return false;
    };
    let (mut min_x, mut min_y, mut max_x, mut max_y) = (first.x, first.y, first.x, first.y);
    for point in &points[1..] {
        min_x = min_x.min(point.x);
        min_y = min_y.min(point.y);
        max_x = max_x.max(point.x);
        max_y = max_y.max(point.y);
    }
    (max_x as f32) >= wx0 && (min_x as f32) <= wx1 && (max_y as f32) >= wy0 && (min_y as f32) <= wy1
}

pub fn normalized_probe_expression(expression: &str) -> String {
    expression
        .chars()
        .filter(|character| !character.is_ascii_whitespace())
        .flat_map(char::to_lowercase)
        .collect()
}

/// The class colour of a named net: ground, supply or clock. Any other name
/// declares no class and is painted as a wire: a net colour is an explicit
/// assignment, and a hash of the name lands on the wire, selection and class
/// hues as often as not.
pub fn named_net_class_color(name: &str) -> Option<egui::Color32> {
    let palette = rspice_ui_kit::tokens::active_palette();
    let normalized = name.trim().to_ascii_lowercase();
    if matches!(
        normalized.as_str(),
        "0" | "gnd" | "ground" | "vss" | "vssa" | "vssd"
    ) {
        return Some(palette.net_class_ground);
    }
    if normalized.starts_with("vdd")
        || normalized.starts_with("vcc")
        || normalized.starts_with("vee")
        || normalized.starts_with("supply")
    {
        return Some(palette.net_class_supply);
    }
    if normalized.contains("clk") || normalized.contains("clock") {
        return Some(palette.net_class_clock);
    }
    None
}

#[cfg(test)]
mod net_class_tests {
    use super::named_net_class_color;

    #[test]
    fn only_ground_supply_and_clock_names_carry_a_class_colour() {
        let palette = rspice_ui_kit::tokens::active_palette();
        assert_eq!(named_net_class_color("GND"), Some(palette.net_class_ground));
        assert_eq!(
            named_net_class_color("vdd_core"),
            Some(palette.net_class_supply)
        );
        assert_eq!(
            named_net_class_color("sys_clk"),
            Some(palette.net_class_clock)
        );
        for unclassified in ["VOUT", "afe_in", "bias_n", "D3"] {
            assert_eq!(named_net_class_color(unclassified), None, "{unclassified}");
        }
    }
}

#[cfg(test)]
mod band_tests {
    use super::*;
    use crate::session::visibility::SchematicReviewMarkerVisibility;
    use rspice_design::schematic::{
        document::SchematicDocument, document_policy::SelectionCrossingPolicy,
        selection::Selection, wire::Wire,
    };

    /// The ids of the wires a scene paints in the selection colour.
    fn painted_selected(document: &SchematicDocument, editor: &EditorSession) -> usize {
        let ctx = egui::Context::default();
        let bounds = Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(400.0, 200.0));
        let painter = Painter::new(
            ctx,
            egui::LayerId::new(egui::Order::Foreground, egui::Id::new("band-test")),
            bounds,
        );
        let viewport = Viewport {
            offset: egui::Pos2::new(50.0, 100.0),
            zoom: 1.0,
            bounds,
        };
        let context = SchematicSymbolContext::default();
        draw_content(
            &painter,
            bounds,
            &viewport,
            &SceneView {
                design: DesignView {
                    document,
                    canvas_cache: None,
                    sheet_catalog: None,
                    review_markers: SchematicReviewMarkerVisibility::All,
                },
                editor,
                net_highlighting: SchematicNetHighlighting::Off,
                parameter_labels: SchematicParameterLabelVisibility::NamesAndValues,
                hover_wire_vertex: None,
                selection_filter: SchematicSelectionFilter::default(),
                hover: None,
            },
            SceneSymbols {
                library: None,
                context: &context,
            },
            SceneOverlays {
                net_class_colors: HashMap::new(),
                operating_point: Vec::new(),
                probe_status: &|_| ProbeVisualStatus::Materialized,
                selected_trace_expression: &|| None,
                open_pins: &HashMap::new(),
            },
            String::new,
        );
        let select = rspice_ui_kit::tokens::active_palette().canvas_select;
        let mut count = 0;
        painter.for_each_shape(|shape| {
            if let egui::Shape::Path(path) = &shape.shape
                && path.stroke.color == egui::epaint::ColorMode::Solid(select)
            {
                count += 1;
            }
        });
        count
    }

    /// One long wire, and the selection a release of `band` would make.
    fn drag(start: Point, current: Point) -> (usize, Selection) {
        let mut document = SchematicDocument {
            wires: vec![Wire::segment(1, Point::new(0, 0), Point::new(200, 0))],
            ..SchematicDocument::default()
        };
        document.document_policy.selection_crossing = SelectionCrossingPolicy::Directional;
        let mut editor = EditorSession::default();
        editor.selection_rect.start_at(start);
        editor.selection_rect.update(current);
        let mut released = Selection::default();
        SchematicSymbolContext::default().select_in_rect(
            &document,
            &mut released,
            SelectionWindow::from_band(start, current, document.document_policy.selection_crossing),
            false,
        );
        (painted_selected(&document, &editor), released)
    }

    /// A left-to-right band is a window: over one end of a long wire it
    /// neither shows the wire selected nor selects it. A right-to-left band
    /// is a crossing: across the middle of the wire, with no vertex inside,
    /// it shows the wire selected and selects it. Before, the preview took
    /// any vertex inside the box in both modes.
    #[test]
    fn the_live_band_shows_exactly_what_its_release_selects() {
        let (painted, released) = drag(Point::new(-10, -10), Point::new(50, 10));
        assert_eq!(painted, 0, "a window over one end shows nothing selected");
        assert!(!released.has_wire(1));

        let (painted, released) = drag(Point::new(110, 10), Point::new(90, -10));
        assert!(
            painted > 0,
            "a crossing band over the middle shows the wire"
        );
        assert!(released.has_wire(1));
    }
}

#[cfg(test)]
mod hover_tests {
    use super::*;
    use crate::session::visibility::SchematicReviewMarkerVisibility;
    use rspice_design::schematic::{
        component::Component, component_type::ComponentType, document::SchematicDocument,
        wire::Wire,
    };

    /// Every shape a scene paints, in paint order.
    fn paint(
        document: &SchematicDocument,
        editor: &EditorSession,
        hover: Option<PointerTarget>,
    ) -> Vec<egui::Shape> {
        paint_with_open_pins(document, editor, hover, &HashMap::new())
    }

    fn paint_with_open_pins(
        document: &SchematicDocument,
        editor: &EditorSession,
        hover: Option<PointerTarget>,
        open_pins: &HashMap<u64, Vec<Point>>,
    ) -> Vec<egui::Shape> {
        let ctx = egui::Context::default();
        // Instance labels lay out text, which needs the fonts a first pass
        // loads.
        let _ = ctx.run_ui(egui::RawInput::default(), |_| {});
        let bounds = Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(400.0, 200.0));
        let painter = Painter::new(
            ctx,
            egui::LayerId::new(egui::Order::Foreground, egui::Id::new("hover-test")),
            bounds,
        );
        let viewport = Viewport {
            offset: egui::Pos2::new(50.0, 100.0),
            zoom: 1.0,
            bounds,
        };
        let context = SchematicSymbolContext::default();
        draw_content(
            &painter,
            bounds,
            &viewport,
            &SceneView {
                design: DesignView {
                    document,
                    canvas_cache: None,
                    sheet_catalog: None,
                    review_markers: SchematicReviewMarkerVisibility::All,
                },
                editor,
                net_highlighting: SchematicNetHighlighting::Off,
                parameter_labels: SchematicParameterLabelVisibility::NamesAndValues,
                hover_wire_vertex: None,
                selection_filter: SchematicSelectionFilter::default(),
                hover,
            },
            SceneSymbols {
                library: None,
                context: &context,
            },
            SceneOverlays {
                net_class_colors: HashMap::new(),
                operating_point: Vec::new(),
                probe_status: &|_| ProbeVisualStatus::Materialized,
                selected_trace_expression: &|| None,
                open_pins,
            },
            String::new,
        );
        let mut shapes = Vec::new();
        painter.for_each_shape(|shape| shapes.push(shape.shape.clone()));
        shapes
    }

    /// The dashes of a hover outline: 1-dpx segments in the selection
    /// colour at alpha 0.85.
    fn outline_dashes(shapes: &[egui::Shape]) -> Vec<[egui::Pos2; 2]> {
        let ink = rspice_ui_kit::tokens::active_palette()
            .canvas_select
            .gamma_multiply(0.85);
        shapes
            .iter()
            .filter_map(|shape| match shape {
                egui::Shape::LineSegment { points, stroke }
                    if stroke.color == ink && stroke.width == 1.0 =>
                {
                    Some(*points)
                }
                _ => None,
            })
            .collect()
    }

    fn resistor() -> SchematicDocument {
        SchematicDocument {
            components: vec![Component::new(
                1,
                ComponentType::Resistor,
                Point::new(100, 0),
            )],
            ..SchematicDocument::default()
        }
    }

    /// A hovered instance draws a dashed outline 3 pt outside the pick
    /// bounds the click tests; an unhovered one, and a hovered one that is
    /// already selected, draw none.
    #[test]
    fn a_hovered_instance_draws_a_dashed_outline_unless_selected() {
        let document = resistor();
        let editor = EditorSession::default();
        let dashes = outline_dashes(&paint(
            &document,
            &editor,
            Some(PointerTarget::Component(1)),
        ));
        assert!(
            dashes.len() >= 8,
            "a dashed outline, not a solid rect: {} dashes",
            dashes.len()
        );
        let (min, max) =
            SchematicSymbolContext::default().component_bounds(&document.components[0]);
        let viewport_offset = egui::vec2(50.0, 100.0);
        let pick = Rect::from_min_max(
            egui::pos2(min.x as f32, min.y as f32) + viewport_offset,
            egui::pos2(max.x as f32, max.y as f32) + viewport_offset,
        );
        let drawn = dashes
            .iter()
            .flat_map(|points| points.iter().copied())
            .fold(Rect::NOTHING, |rect, point| {
                rect.union(Rect::from_min_max(point, point))
            });
        assert!(
            (drawn.min.x - (pick.min.x - 3.0)).abs() <= 1.0
                && (drawn.max.x - (pick.max.x + 3.0)).abs() <= 1.0
                && (drawn.min.y - (pick.min.y - 3.0)).abs() <= 1.0
                && (drawn.max.y - (pick.max.y + 3.0)).abs() <= 1.0,
            "the outline stands 3 pt off the pick bounds {pick:?}: drawn {drawn:?}"
        );

        assert!(outline_dashes(&paint(&document, &editor, None)).is_empty());

        let mut selected = EditorSession::default();
        selected.selection.components.insert(1);
        assert!(
            outline_dashes(&paint(
                &document,
                &selected,
                Some(PointerTarget::Component(1))
            ))
            .is_empty(),
            "a selected instance draws nothing extra on hover"
        );
    }

    /// An open terminal takes a hollow square centred on it, 5 pt a side at
    /// zoom 1 (3 wu is under the 4-pt floor, and an odd pixel count keeps
    /// the 1-pt outline crisp and square about the lead), outlined 1 pt in
    /// the symbol colour, or the selection colour when the instance is
    /// selected. A connected terminal takes nothing.
    #[test]
    fn an_open_pin_takes_a_hollow_square_in_the_instance_colour() {
        let document = resistor();
        let terminal =
            SchematicSymbolContext::default().terminal_points(&document.components[0])[0];
        let open = HashMap::from([(1, vec![terminal])]);
        let squares = |shapes: &[egui::Shape]| -> Vec<(Rect, egui::Color32, f32)> {
            shapes
                .iter()
                .filter_map(|shape| match shape {
                    egui::Shape::Rect(mark)
                        if mark.fill == egui::Color32::TRANSPARENT
                            && mark.rect.size() == egui::Vec2::splat(5.0) =>
                    {
                        Some((mark.rect, mark.stroke.color, mark.stroke.width))
                    }
                    _ => None,
                })
                .collect()
        };
        let palette = rspice_ui_kit::tokens::active_palette();
        let editor = EditorSession::default();
        let marks = squares(&paint_with_open_pins(&document, &editor, None, &open));
        assert_eq!(marks.len(), 1, "one open terminal, one mark");
        let (mark, colour, width) = marks[0];
        assert_eq!((colour, width), (palette.symbol, 1.0));
        let centre = egui::pos2(terminal.x as f32 + 50.0, terminal.y as f32 + 100.0);
        assert_eq!(mark.center(), centre, "centred on the terminal");
        assert!(
            squares(&paint(&document, &editor, None)).is_empty(),
            "a connected pin has no mark"
        );
        let mut selected = EditorSession::default();
        selected.selection.components.insert(1);
        assert_eq!(
            squares(&paint_with_open_pins(&document, &selected, None, &open))[0].1,
            palette.canvas_select
        );
    }

    /// A hovered wire draws a halo beneath it: the same path, 4 pt wider, in
    /// the selection colour at alpha 0.35, painted before the wire.
    #[test]
    fn a_hovered_wire_draws_a_halo_beneath_it() {
        let document = SchematicDocument {
            wires: vec![Wire::segment(1, Point::new(0, 0), Point::new(200, 0))],
            ..SchematicDocument::default()
        };
        let editor = EditorSession::default();
        let halo_ink = rspice_ui_kit::tokens::active_palette()
            .canvas_select
            .gamma_multiply(0.35);
        let strokes = |shapes: &[egui::Shape]| -> Vec<(egui::Color32, f32)> {
            shapes
                .iter()
                .filter_map(|shape| match shape {
                    egui::Shape::Path(path) => match path.stroke.color {
                        egui::epaint::ColorMode::Solid(color) => Some((color, path.stroke.width)),
                        egui::epaint::ColorMode::UV(_) => None,
                    },
                    egui::Shape::LineSegment { stroke, .. } => Some((stroke.color, stroke.width)),
                    _ => None,
                })
                .collect()
        };
        let plain = strokes(&paint(&document, &editor, None));
        assert!(plain.iter().all(|(color, _)| *color != halo_ink));
        let (_, wire_width) = plain[0];

        let hovered = strokes(&paint(&document, &editor, Some(PointerTarget::Wire(1))));
        assert_eq!(
            hovered.first(),
            Some(&(halo_ink, wire_width + 4.0)),
            "the halo is painted first, 4 pt wider than the wire"
        );
        assert_eq!(
            &hovered[1..],
            plain.as_slice(),
            "the wire itself is unchanged"
        );

        let mut selected = EditorSession::default();
        selected.selection.wires.insert(1);
        assert!(
            strokes(&paint(&document, &selected, Some(PointerTarget::Wire(1))))
                .iter()
                .all(|(color, _)| *color != halo_ink),
            "a selected wire draws no halo"
        );
    }
}
