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
        ProbeVisualStatus, chain_conductors, draw_bus, draw_bus_tap, draw_component,
        draw_conductor, draw_junction, draw_probe, probe_at_screen, probe_world_bounds,
    },
    net_labels::{draw_net_label, net_label_at, world_bounds as net_label_world_bounds},
    symbol_context::SchematicSymbolContext,
    viewport::Viewport,
};
use crate::{
    session::{
        EditorSession,
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
    let preview_bounds = if view.editor.selection_rect.is_active() {
        let (min_x, min_y, max_x, max_y) = view.editor.selection_rect.bounds();
        Some((min_x, min_y, max_x, max_y))
    } else {
        None
    };

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
        let mut selected = view.editor.selection.has_bus(bus.id);
        if !selected && let Some((min_x, min_y, max_x, max_y)) = preview_bounds {
            selected = bus.points.windows(2).any(|segment| {
                super::geometry::segment_intersects_rect(
                    segment[0], segment[1], min_x, min_y, max_x, max_y,
                )
            });
        }
        draw_bus(painter, viewport, bus, selected);
    }

    // Wires of one style that meet end to end are painted as one path, so the
    // corner two wires form is a mitered join. Plain wires go down first,
    // then highlighted nets, then the selection, so emphasis stays on top.
    type ConductorStyle = (bool, Option<[u8; 4]>);
    let mut conductor_groups: BTreeMap<ConductorStyle, (Option<egui::Color32>, Vec<&[Point]>)> =
        BTreeMap::new();
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
        let mut is_selected = view.editor.selection.wires.contains(&wire.id);

        if !is_selected && let Some((min_x, min_y, max_x, max_y)) = preview_bounds {
            is_selected = wire
                .points
                .iter()
                .any(|p| p.x >= min_x && p.x <= max_x && p.y >= min_y && p.y <= max_y);
        }

        let highlight_color = match view.net_highlighting {
            SchematicNetHighlighting::SelectedAcrossHierarchy => view
                .editor
                .net_highlight
                .is_wire_highlighted(wire.id)
                .then_some(rspice_ui_kit::tokens::active_palette().warn),
            SchematicNetHighlighting::NetClassColors => {
                overlays.net_class_colors.get(&wire.id).copied()
            }
            SchematicNetHighlighting::Off => None,
        };
        conductor_groups
            .entry((is_selected, highlight_color.map(|color| color.to_array())))
            .or_insert_with(|| (highlight_color, Vec::new()))
            .1
            .push(wire.points.as_slice());
    }
    for ((is_selected, _), (highlight_color, polylines)) in conductor_groups {
        for chain in chain_conductors(polylines) {
            draw_conductor(painter, viewport, &chain, is_selected, highlight_color);
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
        let mut selected = view.editor.selection.has_bus_tap(tap.id);
        if !selected && let Some((rx0, ry0, rx1, ry1)) = preview_bounds {
            selected = route.windows(2).any(|segment| {
                super::geometry::segment_intersects_rect(segment[0], segment[1], rx0, ry0, rx1, ry1)
            });
        }
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
        let mut is_selected = view.editor.selection.components.contains(&component.id);

        if !is_selected && let Some((min_x, min_y, max_x, max_y)) = preview_bounds {
            is_selected = max.x >= min_x && min.x <= max_x && max.y >= min_y && min.y <= max_y;
        }

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
            view.editor.selection.has_junction(junction.pos),
            view.hover_wire_vertex == Some((junction.pos.x, junction.pos.y)),
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
        let mut selected = view.editor.selection.has_documentation_shape(shape.id);
        if !selected && let Some((min_x, min_y, max_x, max_y)) = preview_bounds {
            selected = super::documentation_shapes::shape_intersects_rect(
                shape, min_x, min_y, max_x, max_y, false,
            );
        }
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
        let mut selected = view.editor.selection.has_net_label(label.id);
        if !selected && let Some((min_x, min_y, max_x, max_y)) = preview_bounds {
            selected = max.x >= min_x && min.x <= max_x && max.y >= min_y && min.y <= max_y;
        }
        draw_net_label(
            painter,
            viewport,
            label,
            selected,
            hovered_label == Some(label.id),
            false,
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
        let mut selected = view.editor.selection.has_design_note(note.id);
        if !selected && let Some((min_x, min_y, max_x, max_y)) = preview_bounds {
            selected = max.x >= min_x && min.x <= max_x && max.y >= min_y && min.y <= max_y;
        }
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
        if !selected && let Some((min_x, min_y, max_x, max_y)) = preview_bounds {
            selected = max.x >= min_x && min.x <= max_x && max.y >= min_y && min.y <= max_y;
        }
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
                    1.0 * viewport.zoom,
                    rspice_ui_kit::tokens::active_palette().accent,
                ),
            );
        }
    }
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

pub fn named_net_class_color(name: &str) -> egui::Color32 {
    let palette = rspice_ui_kit::tokens::active_palette();
    let normalized = name.trim().to_ascii_lowercase();
    if matches!(
        normalized.as_str(),
        "0" | "gnd" | "ground" | "vss" | "vssa" | "vssd"
    ) {
        return palette.ok;
    }
    if normalized.starts_with("vdd")
        || normalized.starts_with("vcc")
        || normalized.starts_with("vee")
        || normalized.starts_with("supply")
    {
        return palette.warn;
    }
    if normalized.contains("clk") || normalized.contains("clock") {
        return palette.info;
    }
    let hash = normalized.bytes().fold(0_u64, |hash, byte| {
        hash.wrapping_mul(109).wrapping_add(u64::from(byte))
    });
    palette.traces[hash as usize % palette.traces.len()]
}
