//! Active-sheet scene painting over current document/session and prepared overlays.

use super::{
    design_notes::{
        conservative_world_bounds as design_note_world_bounds, design_note_at, draw_design_note,
    },
    design_view::{DesignView, design_note_visible},
    documentation_shapes::{
        documentation_shape_at, draw_documentation_shape,
        world_bounds as documentation_shape_bounds,
    },
    drawing::{
        ConductorInk, JunctionPaint, ProbeVisualStatus, chain_conductors, draw_bus, draw_bus_tap,
        draw_component, draw_conductor, draw_junction, draw_probe, probe_at_screen,
        probe_world_bounds,
    },
    lod,
    net_labels::{
        NetLabelPaint, draw_net_label_attached, net_label_at,
        world_bounds as net_label_world_bounds,
    },
    symbol_context::{SchematicSymbolContext, SelectionWindow},
    viewport::Viewport,
};
use crate::{
    session::{
        EditorSession,
        selection::SchematicSelectionFilter,
        tool::Tool,
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
mod parent;
pub use annotations::OperatingPointCanvasAnnotation;
use annotations::draw_operating_point_annotations;
pub use empty::draw_empty_hint;
pub use focus::{draw_keyboard_focus, keyboard_focus_matches_selection};
pub use parent::draw_parent_context;

const CULL_MARGIN: f32 = 160.0;

pub struct SceneView<'a> {
    pub design: DesignView<'a>,
    pub editor: &'a EditorSession,
    pub net_highlighting: SchematicNetHighlighting,
    pub parameter_labels: SchematicParameterLabelVisibility,
    pub hover_wire_vertex: Option<(i32, i32)>,
    /// What a rubber band may select, so its live preview honours it.
    pub selection_filter: SchematicSelectionFilter,
}

/// The host resolves run provenance and probe materialization. The canvas only
/// consumes display values and narrow probe-status/selected-trace lookups.
pub struct SceneOverlays<'a> {
    pub net_class_colors: HashMap<u64, egui::Color32>,
    pub operating_point: Vec<OperatingPointCanvasAnnotation>,
    pub probe_status: &'a dyn Fn(&SchematicProbe) -> ProbeVisualStatus,
    pub selected_trace_expression: &'a dyn Fn() -> Option<&'a str>,
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
            SchematicNetHighlighting::SelectedNet
                if view.editor.net_highlight.is_wire_highlighted(wire.id) =>
            {
                ConductorInk::Highlighted
            }
            SchematicNetHighlighting::NetClassColors => overlays
                .net_class_colors
                .get(&wire.id)
                .map_or(ConductorInk::Plain, |colour| ConductorInk::Class(*colour)),
            SchematicNetHighlighting::SelectedNet | SchematicNetHighlighting::Off => {
                ConductorInk::Plain
            }
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
                hovered: view.hover_wire_vertex == Some((junction.pos.x, junction.pos.y)),
                incident: incident_ink(view, cache, &inked_wires, junction.pos),
            },
        );
    }

    draw_operating_point_annotations(painter, available, viewport, overlays.operating_point);

    // Presentation geometry is a background documentation layer. It remains
    // selectable, but is intentionally painted below authored text and names.
    let hovered_shape = if view.editor.tool == Tool::Select {
        let shapes = view
            .design
            .objects_on_active_sheet(&view.design.document.documentation_shapes, |item| item.id);
        painter
            .ctx()
            .pointer_hover_pos()
            .filter(|position| available.contains(*position))
            .and_then(|position| documentation_shape_at(viewport, shapes.as_ref(), position))
    } else {
        None
    };
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
            hovered_shape == Some(shape.id),
        );
    }

    // Net labels are authored text, not derived annotations. Paint them after
    // junction and OP overlays so the source net name always remains legible.
    let hovered_label = if view.editor.tool == Tool::Select {
        let labels = view
            .design
            .objects_on_active_sheet(&view.design.document.net_labels, |item| item.id);
        painter
            .ctx()
            .pointer_hover_pos()
            .filter(|pointer| available.contains(*pointer))
            .and_then(|pointer| net_label_at(painter.ctx(), viewport, labels.as_ref(), pointer))
    } else {
        None
    };
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
                hovered: hovered_label == Some(label.id),
                preview: false,
                attachment: (attachment != ConductorInk::Plain).then_some(attachment),
            },
        );
    }

    // Documentation objects are painted above electrical names but below
    // validation markers. They never participate in conductor rendering.
    let hovered_note = if view.editor.tool == Tool::Select {
        let notes = view.design.visible_design_notes();
        painter
            .ctx()
            .pointer_hover_pos()
            .filter(|pointer| available.contains(*pointer))
            .and_then(|pointer| {
                design_note_at(
                    painter.ctx(),
                    viewport,
                    notes.as_ref(),
                    &DesignNoteRenderContext::for_document(view.design.document, &view_path()),
                    pointer,
                )
            })
    } else {
        None
    };
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
            hovered_note == Some(note.id),
        );
    }

    // Probe flags are durable output intent and remain visible above authored
    // conductor/text layers. Their reference is the exact bound expression
    // when resolved, otherwise the stable unbound P<n> marker identity.
    let hovered_probe = if view.editor.tool == Tool::Select {
        let probes = view
            .design
            .objects_on_active_sheet(&view.design.document.probes, |item| item.id);
        painter
            .ctx()
            .pointer_hover_pos()
            .filter(|pointer| available.contains(*pointer))
            .and_then(|pointer| probe_at_screen(viewport, probes.as_ref(), pointer))
    } else {
        None
    };
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
            hovered_probe == Some(probe.id),
        );
    }

    if let Some((hx, hy)) = view.hover_wire_vertex {
        let hover_pos = Point::new(hx, hy);
        let is_junction = view.design.active_junction_at(hover_pos).is_some();
        if !is_junction {
            let pos = viewport.schematic_to_screen(hover_pos);
            let radius = 3.0 * viewport.zoom;
            painter.circle_stroke(
                pos,
                radius,
                Stroke::new(
                    lod::mark_outline_pt(painter.pixels_per_point()),
                    rspice_ui_kit::tokens::active_palette().canvas_select,
                ),
            );
        }
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
