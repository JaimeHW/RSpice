use egui::{Painter, Rect, Stroke};

use crate::common::app::{AppState, SchematicKeyboardFocus};
use crate::state::{
    Component, DesignNote, DesignNoteKind, DesignReviewState, NetGraph, Point,
    SchematicAnnotationVisibility, SchematicBackAnnotationContent, SchematicNetHighlighting,
    SchematicReviewMarkerVisibility,
};

use super::super::symbols::SymbolLibrary;
use super::SchematicSymbolContext;
use super::design_notes::{
    conservative_world_bounds as design_note_world_bounds, design_note_at, draw_design_note,
};
use super::documentation_shapes::{
    documentation_shape_at, draw_documentation_shape, world_bounds as documentation_shape_bounds,
};
use super::drawing::{
    draw_bus, draw_bus_tap, draw_component, draw_junction, draw_probe, draw_wire,
    probe_world_bounds,
};
use super::grid::draw_grid;
use super::net_labels::{draw_net_label, net_label_at, world_bounds as net_label_world_bounds};
use super::sheet_visibility::{
    active_junction_at, active_sheet_has_objects, object_is_on_active_sheet,
    objects_on_active_sheet,
};
use super::viewport::Viewport;

/// Culling margin in world units: symbols extend up to ~40 units from their
/// anchor and labels overhang further; generous slack keeps pop-in impossible
/// while still rejecting everything genuinely off-screen.
const CULL_MARGIN: f32 = 160.0;
const EMPTY_HINT_MOBILE_BREAKPOINT: f32 = 460.0;
const EMPTY_HINT_DESKTOP_LINES: [&str; 3] = [
    "Empty schematic",
    "Pick a part from the left panel to place a device or source",
    "File > Open project loads an existing design",
];
const EMPTY_HINT_MOBILE_LINES: [&str; 4] = [
    "Empty schematic",
    "Use Library to place devices and sources",
    "The toolbar provides wiring, labels, and probes",
    "File > Open project loads an existing design",
];

pub(super) fn component_cull_bounds(
    component: &Component,
    symbol_context: &SchematicSymbolContext,
) -> (Point, Point) {
    symbol_context.component_bounds(component)
}

pub(super) fn draw_scene(
    painter: &Painter,
    available: Rect,
    viewport: &Viewport,
    state: &AppState,
    symbol_library: Option<&SymbolLibrary>,
    symbol_context: &SchematicSymbolContext,
) {
    painter.rect_filled(
        available,
        0.0,
        crate::ui::tokens::active_palette().canvas_bg,
    );
    draw_grid(painter, available, state);

    // First-run guidance: an empty sheet says what to do next instead of
    // presenting a silent dot field.
    if !active_sheet_has_objects(state) {
        draw_empty_hint(painter, available);
    }

    let preview_bounds = if state.schematic.selection_rect.is_active() {
        let (min_x, min_y, max_x, max_y) = state.schematic.selection_rect.bounds();
        Some((min_x, min_y, max_x, max_y))
    } else {
        None
    };

    // Viewport culling: only elements whose bounds intersect the visible
    // world rect are transformed and tessellated.
    let (wx0, wy0, wx1, wy1) = viewport.visible_world_rect(CULL_MARGIN);
    let cache = state.schematic.canvas_cache();
    let net_class_colors = if state.ui.schematic_visibility.net_highlighting
        == SchematicNetHighlighting::NetClassColors
    {
        net_class_colors(state)
    } else {
        std::collections::HashMap::new()
    };

    for bus in &state.schematic.buses {
        if !object_is_on_active_sheet(state, bus.id) {
            continue;
        }
        if !polyline_intersects_view(&bus.points, wx0, wy0, wx1, wy1) {
            continue;
        }
        let mut selected = state.schematic.selection.has_bus(bus.id);
        if !selected && let Some((min_x, min_y, max_x, max_y)) = preview_bounds {
            selected = bus.points.windows(2).any(|segment| {
                super::segment_intersects_rect(segment[0], segment[1], min_x, min_y, max_x, max_y)
            });
        }
        draw_bus(painter, viewport, bus, selected);
    }

    for (index, wire) in state.schematic.wires.iter().enumerate() {
        if !object_is_on_active_sheet(state, wire.id) {
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
        let mut is_selected = state.schematic.selection.wires.contains(&wire.id);

        if !is_selected && let Some((min_x, min_y, max_x, max_y)) = preview_bounds {
            is_selected = wire
                .points
                .iter()
                .any(|p| p.x >= min_x && p.x <= max_x && p.y >= min_y && p.y <= max_y);
        }

        let highlight_color = match state.ui.schematic_visibility.net_highlighting {
            SchematicNetHighlighting::SelectedAcrossHierarchy => state
                .schematic
                .net_highlight
                .is_wire_highlighted(wire.id)
                .then_some(crate::ui::tokens::active_palette().warn),
            SchematicNetHighlighting::NetClassColors => net_class_colors.get(&wire.id).copied(),
            SchematicNetHighlighting::Off => None,
        };
        draw_wire(painter, viewport, wire, is_selected, highlight_color);
    }

    for tap in &state.schematic.bus_taps {
        if !object_is_on_active_sheet(state, tap.id) {
            continue;
        }
        let route = crate::schematic::bus_geometry::bus_tap_route_points(tap);
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
        let mut selected = state.schematic.selection.has_bus_tap(tap.id);
        if !selected && let Some((rx0, ry0, rx1, ry1)) = preview_bounds {
            selected = route.windows(2).any(|segment| {
                super::segment_intersects_rect(segment[0], segment[1], rx0, ry0, rx1, ry1)
            });
        }
        draw_bus_tap(painter, viewport, tap, selected);
    }

    for component in &state.schematic.components {
        if !object_is_on_active_sheet(state, component.id) {
            continue;
        }
        let (min, max) = component_cull_bounds(component, symbol_context);
        if (max.x as f32) < wx0
            || (min.x as f32) > wx1
            || (max.y as f32) < wy0
            || (min.y as f32) > wy1
        {
            continue;
        }
        let mut is_selected = state.schematic.selection.components.contains(&component.id);

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
            state.ui.schematic_visibility.parameter_labels,
        );
    }

    for junction in &state.schematic.junctions {
        if !object_is_on_active_sheet(state, junction.id) {
            continue;
        }
        let (jx, jy) = (junction.pos.x as f32, junction.pos.y as f32);
        if jx < wx0 || jx > wx1 || jy < wy0 || jy > wy1 {
            continue;
        }
        draw_junction(painter, viewport, junction.pos, state);
    }

    draw_operating_point_annotations(painter, available, viewport, state);

    // Presentation geometry is a background documentation layer. It remains
    // selectable, but is intentionally painted below authored text and names.
    let hovered_shape = if state.schematic.tool == crate::state::Tool::Select {
        let shapes =
            objects_on_active_sheet(state, &state.schematic.documentation_shapes, |item| item.id);
        painter
            .ctx()
            .pointer_hover_pos()
            .filter(|position| available.contains(*position))
            .and_then(|position| documentation_shape_at(viewport, shapes.as_ref(), position))
    } else {
        None
    };
    for shape in &state.schematic.documentation_shapes {
        if !object_is_on_active_sheet(state, shape.id) {
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
        let mut selected = state.schematic.selection.has_documentation_shape(shape.id);
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
    let hovered_label = if state.schematic.tool == crate::state::Tool::Select {
        let labels = objects_on_active_sheet(state, &state.schematic.net_labels, |item| item.id);
        painter
            .ctx()
            .pointer_hover_pos()
            .filter(|pointer| available.contains(*pointer))
            .and_then(|pointer| net_label_at(painter.ctx(), viewport, labels.as_ref(), pointer))
    } else {
        None
    };
    for label in &state.schematic.net_labels {
        if !object_is_on_active_sheet(state, label.id) {
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
        let mut selected = state.schematic.selection.has_net_label(label.id);
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
    let hovered_note = if state.schematic.tool == crate::state::Tool::Select {
        let notes = visible_design_notes(state);
        painter
            .ctx()
            .pointer_hover_pos()
            .filter(|pointer| available.contains(*pointer))
            .and_then(|pointer| {
                design_note_at(painter.ctx(), viewport, notes.as_ref(), state, pointer)
            })
    } else {
        None
    };
    for note in &state.schematic.design_notes {
        if !object_is_on_active_sheet(state, note.id) {
            continue;
        }
        if !design_note_visible(note, state.ui.schematic_visibility.review_markers) {
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
        let mut selected = state.schematic.selection.has_design_note(note.id);
        if !selected && let Some((min_x, min_y, max_x, max_y)) = preview_bounds {
            selected = max.x >= min_x && min.x <= max_x && max.y >= min_y && min.y <= max_y;
        }
        draw_design_note(
            painter,
            viewport,
            note,
            state,
            selected,
            hovered_note == Some(note.id),
        );
    }

    // Probe flags are durable output intent and remain visible above authored
    // conductor/text layers. Their reference is the exact bound expression
    // when resolved, otherwise the stable unbound P<n> marker identity.
    for probe in &state.schematic.probes {
        if !object_is_on_active_sheet(state, probe.id) {
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
        draw_probe(painter, viewport, probe);
    }

    if let Some((hx, hy)) = state.dialogs.interaction.hover_wire_vertex {
        let hover_pos = Point::new(hx, hy);
        let is_junction = active_junction_at(state, hover_pos).is_some();
        if !is_junction {
            let pos = viewport.schematic_to_screen(hover_pos);
            let radius = 3.0 * viewport.zoom;
            painter.circle_stroke(
                pos,
                radius,
                Stroke::new(
                    1.0 * viewport.zoom,
                    crate::ui::tokens::active_palette().accent,
                ),
            );
        }
    }

    // Check results last — violation badges annotate everything below.
    if state.ui.schematic_visibility.annotations == SchematicAnnotationVisibility::ViolationsOnly {
        super::violations::draw_violation_markers(painter, viewport, state);
    }

    draw_keyboard_focus(painter, viewport, state, symbol_context);
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

fn draw_keyboard_focus(
    painter: &Painter,
    viewport: &Viewport,
    state: &AppState,
    symbol_context: &SchematicSymbolContext,
) {
    let Some(focus) = state.dialogs.interaction.schematic_keyboard_focus else {
        return;
    };
    if !keyboard_focus_matches_selection(state, focus) {
        return;
    }
    let Some((min, max)) = keyboard_focus_bounds(state, symbol_context, focus) else {
        return;
    };
    let mut rect = Rect::from_two_pos(
        viewport.schematic_to_screen(min),
        viewport.schematic_to_screen(max),
    )
    .expand(4.0);
    rect = Rect::from_center_size(
        rect.center(),
        egui::vec2(rect.width().max(16.0), rect.height().max(16.0)),
    );
    let palette = crate::ui::tokens::active_palette();
    painter.rect_stroke(
        rect,
        3.0,
        Stroke::new(3.5, palette.canvas_bg),
        egui::StrokeKind::Outside,
    );
    painter.rect_stroke(
        rect,
        3.0,
        Stroke::new(1.5, palette.accent),
        egui::StrokeKind::Outside,
    );
}

pub(super) fn keyboard_focus_matches_selection(
    state: &AppState,
    focus: SchematicKeyboardFocus,
) -> bool {
    let selection = &state.schematic.selection;
    match focus {
        SchematicKeyboardFocus::Component(id) => selection.single_component() == Some(id),
        SchematicKeyboardFocus::Wire(id) => {
            selection.single_wire() == Some(id)
                || selection
                    .single_wire_segment()
                    .is_some_and(|selected| selected.wire_id == id)
                || selection
                    .single_wire_vertex()
                    .is_some_and(|selected| selected.wire_id == id)
        }
        SchematicKeyboardFocus::Bus(id) => selection.single_bus() == Some(id),
        SchematicKeyboardFocus::BusTap(id) => selection.single_bus_tap() == Some(id),
        SchematicKeyboardFocus::Junction(id) => {
            selection.single_junction().is_some_and(|position| {
                state
                    .schematic
                    .junctions
                    .iter()
                    .any(|junction| junction.id == id && junction.pos == position)
            })
        }
        SchematicKeyboardFocus::NetLabel(id) => selection.single_net_label() == Some(id),
        SchematicKeyboardFocus::Probe(_) => selection.is_empty(),
        SchematicKeyboardFocus::DesignNote(id) => selection.single_design_note() == Some(id),
        SchematicKeyboardFocus::DocumentationShape(id) => {
            selection.single_documentation_shape() == Some(id)
        }
    }
}

fn keyboard_focus_bounds(
    state: &AppState,
    symbol_context: &SchematicSymbolContext,
    focus: SchematicKeyboardFocus,
) -> Option<(Point, Point)> {
    let id = match focus {
        SchematicKeyboardFocus::Component(id)
        | SchematicKeyboardFocus::Wire(id)
        | SchematicKeyboardFocus::Bus(id)
        | SchematicKeyboardFocus::BusTap(id)
        | SchematicKeyboardFocus::Junction(id)
        | SchematicKeyboardFocus::NetLabel(id)
        | SchematicKeyboardFocus::Probe(id)
        | SchematicKeyboardFocus::DesignNote(id)
        | SchematicKeyboardFocus::DocumentationShape(id) => id,
    };
    if !object_is_on_active_sheet(state, id) {
        return None;
    }
    match focus {
        SchematicKeyboardFocus::Component(id) => state
            .schematic
            .components
            .iter()
            .find(|object| object.id == id)
            .map(|object| symbol_context.component_bounds(object)),
        SchematicKeyboardFocus::Wire(id) => state
            .schematic
            .wires
            .iter()
            .find(|object| object.id == id)
            .and_then(|object| points_bounds(&object.points)),
        SchematicKeyboardFocus::Bus(id) => state
            .schematic
            .buses
            .iter()
            .find(|object| object.id == id)
            .and_then(|object| points_bounds(&object.points)),
        SchematicKeyboardFocus::BusTap(id) => state
            .schematic
            .bus_taps
            .iter()
            .find(|object| object.id == id)
            .and_then(|object| {
                points_bounds(&crate::schematic::bus_geometry::bus_tap_route_points(
                    object,
                ))
            }),
        SchematicKeyboardFocus::Junction(id) => state
            .schematic
            .junctions
            .iter()
            .find(|object| object.id == id)
            .map(|object| (object.pos, object.pos)),
        SchematicKeyboardFocus::NetLabel(id) => state
            .schematic
            .net_labels
            .iter()
            .find(|object| object.id == id)
            .map(net_label_world_bounds),
        SchematicKeyboardFocus::Probe(id) => state
            .schematic
            .probes
            .iter()
            .find(|object| object.id == id)
            .map(probe_world_bounds),
        SchematicKeyboardFocus::DesignNote(id) => visible_design_notes(state)
            .iter()
            .find(|object| object.id == id)
            .map(design_note_world_bounds),
        SchematicKeyboardFocus::DocumentationShape(id) => state
            .schematic
            .documentation_shapes
            .iter()
            .find(|object| object.id == id)
            .map(documentation_shape_bounds),
    }
}

fn points_bounds(points: &[Point]) -> Option<(Point, Point)> {
    let first = *points.first()?;
    let (mut min, mut max) = (first, first);
    for point in &points[1..] {
        min.x = min.x.min(point.x);
        min.y = min.y.min(point.y);
        max.x = max.x.max(point.x);
        max.y = max.y.max(point.y);
    }
    Some((min, max))
}

pub(super) fn visible_design_notes(state: &AppState) -> std::borrow::Cow<'_, [DesignNote]> {
    let active = objects_on_active_sheet(state, &state.schematic.design_notes, |note| note.id);
    if state.ui.schematic_visibility.review_markers == SchematicReviewMarkerVisibility::All {
        return active;
    }
    std::borrow::Cow::Owned(
        active
            .iter()
            .filter(|note| design_note_visible(note, state.ui.schematic_visibility.review_markers))
            .cloned()
            .collect(),
    )
}

fn design_note_visible(note: &DesignNote, visibility: SchematicReviewMarkerVisibility) -> bool {
    if note.kind != DesignNoteKind::ReviewNote {
        return true;
    }
    match visibility {
        SchematicReviewMarkerVisibility::Hidden => false,
        SchematicReviewMarkerVisibility::All => true,
        SchematicReviewMarkerVisibility::OpenAndAssigned => {
            note.review.as_ref().is_some_and(|review| {
                review.state == DesignReviewState::Open || review.assignee.is_some()
            })
        }
    }
}

fn net_class_colors(state: &AppState) -> std::collections::HashMap<u64, egui::Color32> {
    let wires = state
        .schematic
        .wires
        .iter()
        .filter(|wire| object_is_on_active_sheet(state, wire.id))
        .cloned()
        .collect::<Vec<_>>();
    let junctions = state
        .schematic
        .junctions
        .iter()
        .filter(|junction| object_is_on_active_sheet(state, junction.id))
        .cloned()
        .collect::<Vec<_>>();
    let graph = NetGraph::build(&wires, &junctions);
    let mut colors = std::collections::HashMap::new();
    for label in state
        .schematic
        .net_labels
        .iter()
        .filter(|label| object_is_on_active_sheet(state, label.id))
    {
        let color = named_net_class_color(&label.name);
        for wire_id in graph.find_connected_wires(label.pos) {
            colors.entry(wire_id).or_insert(color);
        }
    }
    colors
}

fn named_net_class_color(name: &str) -> egui::Color32 {
    let palette = crate::ui::tokens::active_palette();
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

#[derive(Debug, Clone, PartialEq)]
struct OperatingPointCanvasAnnotation {
    position: Point,
    label: String,
    selected_current: bool,
}

fn device_op_param_unit(name: &str) -> &'static str {
    match name {
        "id" | "ic" | "ib" => "A",
        "vgs" | "vds" | "vbs" | "vth" | "vbe" | "vce" | "vd" => "V",
        "gm" | "gds" | "gmb" | "gd" => "S",
        _ => "",
    }
}

fn device_op_annotation_label(entry: &rspice_core::circuit::DeviceOpEntry) -> String {
    let identity = entry.region.map_or_else(
        || format!("{} [{}]", entry.name, entry.device_kind),
        |region| format!("{} [{} · {region}]", entry.name, entry.device_kind),
    );
    let values = entry
        .params
        .iter()
        .take(3)
        .map(|(name, value)| {
            let unit = device_op_param_unit(name);
            format!(
                "{name}={}{}",
                crate::state::format_engineering(*value),
                unit
            )
        })
        .collect::<Vec<_>>()
        .join(" · ");
    if values.is_empty() {
        identity
    } else {
        format!("{identity} · {values}")
    }
}

/// The name inside a `V(...)` or `I(...)` wrapper, or `None` when the text
/// is anything else.
///
/// An expression such as `V(out)-V(in)` is deliberately rejected: it wraps
/// two signals, not one, and treating its interior as a net name would name
/// a conductor that does not exist.
pub(crate) fn wrapped_signal_name(name: &str, prefix: char) -> Option<&str> {
    let name = name.trim();
    let (head, tail) = name.split_once('(')?;
    if !head.eq_ignore_ascii_case(&prefix.to_string()) || !tail.ends_with(')') {
        return None;
    }
    let inner = tail[..tail.len() - 1].trim();
    (!inner.is_empty() && !inner.contains(['(', ')'])).then_some(inner)
}

/// Produce annotations only from the explicitly selected analysis. The
/// retained cross-probe point map is replaced for each dispatch, so pairing
/// it with any other result would falsely attach values to a different solve.
fn operating_point_annotations(state: &AppState) -> Vec<OperatingPointCanvasAnnotation> {
    if state.ui.schematic_visibility.annotations != SchematicAnnotationVisibility::OperatingPoint
        || !state.simulation.cross_probe.is_populated()
        || !state.simulation.cross_probe.is_current_for(
            &state.workspace.active_view,
            state.schematic.topology_version(),
        )
    {
        return Vec::new();
    }
    let Some((dc_op, retained_annotation, device_op)) =
        state.simulation.active_analysis().and_then(|analysis| {
            analysis.dc_op.as_ref().map(|dc_op| {
                let annotation = match analysis.result_payload.as_ref() {
                    Some(crate::state::AnalysisResultPayload::OperatingPoint {
                        annotation,
                        ..
                    }) => Some(*annotation),
                    _ => None,
                };
                (dc_op, annotation, analysis.device_op.as_ref())
            })
        })
    else {
        return Vec::new();
    };
    if retained_annotation == Some(crate::state::OperatingPointAnnotationEvidence::None) {
        return Vec::new();
    }

    let mut annotations = Vec::new();
    let back_annotation = state.ui.schematic_visibility.back_annotation;
    let mut annotated_devices = std::collections::HashSet::new();
    for voltage in &dc_op.node_voltages {
        if !voltage.value.is_finite() {
            continue;
        }
        let Some(net_name) = wrapped_signal_name(&voltage.name, 'V') else {
            continue;
        };
        let points = state
            .simulation
            .cross_probe
            .net_to_points
            .iter()
            .find(|(name, _)| name.eq_ignore_ascii_case(net_name))
            .map(|(_, points)| points);
        let Some(position) = points.and_then(|points| {
            points
                .iter()
                .copied()
                .filter(|point| {
                    super::sheet_visibility::active_wire_at(state, *point).is_some()
                        || active_junction_at(state, *point).is_some()
                        || state.schematic.components.iter().any(|component| {
                            object_is_on_active_sheet(state, component.id)
                                && component
                                    .terminal_positions()
                                    .iter()
                                    .any(|(_, terminal)| terminal == point)
                        })
                })
                .min_by_key(|point| (point.y, point.x))
        }) else {
            continue;
        };
        annotations.push(OperatingPointCanvasAnnotation {
            position,
            label: format!(
                "{} = {} {}",
                voltage.name,
                crate::state::format_engineering(voltage.value),
                voltage.unit
            ),
            selected_current: false,
        });
    }

    let retained_currents = retained_annotation.is_none_or(|annotation| {
        annotation == crate::state::OperatingPointAnnotationEvidence::VoltagesAndCurrents
    });
    if retained_currents && back_annotation != SchematicBackAnnotationContent::VoltagesOnly {
        for component in state.schematic.components.iter().filter(|component| {
            state.schematic.selection.has_component(component.id)
                && object_is_on_active_sheet(state, component.id)
        }) {
            let Some(current) = dc_op.branch_currents.iter().find(|current| {
                current.value.is_finite()
                    && wrapped_signal_name(&current.name, 'I')
                        .is_some_and(|name| name.eq_ignore_ascii_case(&component.name))
            }) else {
                continue;
            };
            let mut label = format!(
                "{} = {} {}",
                current.name,
                crate::state::format_engineering(current.value),
                current.unit
            );
            if back_annotation == SchematicBackAnnotationContent::VoltagesCurrentsAndPower
                && let Some(power) = device_power(dc_op, &component.name)
            {
                label.push_str(&format!(
                    " \u{00b7} {} = {} {}",
                    power.name,
                    crate::state::format_engineering(power.value),
                    power.unit
                ));
            }
            annotations.push(OperatingPointCanvasAnnotation {
                position: component.pos,
                label,
                selected_current: true,
            });
            annotated_devices.insert(component.id);
        }
    }
    if retained_annotation
        == Some(crate::state::OperatingPointAnnotationEvidence::VoltagesAndDeviceOp)
        && back_annotation != SchematicBackAnnotationContent::VoltagesOnly
        && let Some(report) = device_op
    {
        for component in state.schematic.components.iter().filter(|component| {
            state.schematic.selection.has_component(component.id)
                && object_is_on_active_sheet(state, component.id)
        }) {
            let Some(entry) = report
                .entries
                .iter()
                .find(|entry| entry.name.eq_ignore_ascii_case(&component.name))
            else {
                continue;
            };
            let mut label = device_op_annotation_label(entry);
            if back_annotation == SchematicBackAnnotationContent::VoltagesCurrentsAndPower
                && let Some(power) = device_power(dc_op, &component.name)
            {
                label.push_str(&format!(
                    " \u{00b7} {} = {} {}",
                    power.name,
                    crate::state::format_engineering(power.value),
                    power.unit
                ));
            }
            annotations.push(OperatingPointCanvasAnnotation {
                position: component.pos,
                label,
                selected_current: true,
            });
            annotated_devices.insert(component.id);
        }
    }
    if back_annotation == SchematicBackAnnotationContent::VoltagesCurrentsAndPower {
        for component in state.schematic.components.iter().filter(|component| {
            state.schematic.selection.has_component(component.id)
                && object_is_on_active_sheet(state, component.id)
                && !annotated_devices.contains(&component.id)
        }) {
            let Some(power) = device_power(dc_op, &component.name) else {
                continue;
            };
            annotations.push(OperatingPointCanvasAnnotation {
                position: component.pos,
                label: format!(
                    "{} = {} {}",
                    power.name,
                    crate::state::format_engineering(power.value),
                    power.unit
                ),
                selected_current: true,
            });
        }
    }
    annotations
}

fn device_power<'a>(
    dc_op: &'a crate::state::DcOpResult,
    component_name: &str,
) -> Option<&'a crate::state::OperatingPointValue> {
    dc_op.power_dissipation.iter().find(|power| {
        power.value.is_finite()
            && wrapped_signal_name(&power.name, 'P')
                .or_else(|| wrapped_signal_name(&power.name, 'W'))
                .is_some_and(|name| name.eq_ignore_ascii_case(component_name))
    })
}

fn draw_operating_point_annotations(
    painter: &Painter,
    available: Rect,
    viewport: &Viewport,
    state: &AppState,
) {
    use crate::ui::theme::{self, FontWeight};

    let palette = crate::ui::tokens::active_palette();
    for annotation in operating_point_annotations(state) {
        let anchor = viewport.schematic_to_screen(annotation.position);
        if !available.expand(12.0).contains(anchor) {
            continue;
        }
        let galley = painter.layout_no_wrap(
            annotation.label,
            theme::mono(crate::ui::tokens::FS_0, FontWeight::Medium),
            if annotation.selected_current {
                palette.info
            } else {
                palette.net_label
            },
        );
        let offset = if annotation.selected_current {
            egui::vec2(8.0, 12.0)
        } else {
            egui::vec2(7.0, -galley.size().y - 7.0)
        };
        let mut text_pos = anchor + offset;
        text_pos.x = text_pos.x.clamp(
            available.left() + 3.0,
            available.right() - galley.size().x - 3.0,
        );
        text_pos.y = text_pos.y.clamp(
            available.top() + 3.0,
            available.bottom() - galley.size().y - 3.0,
        );
        let background = Rect::from_min_size(text_pos, galley.size()).expand2(egui::vec2(4.0, 2.0));
        painter.rect_filled(background, 2.0, palette.bg_elevated);
        painter.rect_stroke(
            background,
            2.0,
            Stroke::new(1.0, palette.border),
            egui::StrokeKind::Inside,
        );
        painter.galley(text_pos, galley, palette.text);
    }
}

/// Centered get-started hint for an empty sheet.
fn draw_empty_hint(painter: &Painter, available: Rect) {
    use crate::ui::theme::{self, FontWeight};

    let palette = crate::ui::tokens::active_palette();
    let center = available.center();
    if available.width() < EMPTY_HINT_MOBILE_BREAKPOINT {
        draw_empty_hint_mobile(painter, center);
        return;
    }
    painter.text(
        center - egui::vec2(0.0, 22.0),
        egui::Align2::CENTER_CENTER,
        EMPTY_HINT_DESKTOP_LINES[0],
        theme::sans(15.0, FontWeight::Medium),
        palette.text_dim,
    );
    painter.text(
        center + egui::vec2(0.0, 2.0),
        egui::Align2::CENTER_CENTER,
        EMPTY_HINT_DESKTOP_LINES[1],
        theme::sans(12.0, FontWeight::Regular),
        palette.text_faint,
    );
    painter.text(
        center + egui::vec2(0.0, 22.0),
        egui::Align2::CENTER_CENTER,
        EMPTY_HINT_DESKTOP_LINES[2],
        theme::sans(12.0, FontWeight::Regular),
        palette.text_faint,
    );
}

fn draw_empty_hint_mobile(painter: &Painter, center: egui::Pos2) {
    use crate::ui::theme::{self, FontWeight};

    let palette = crate::ui::tokens::active_palette();
    let lines = empty_hint_lines_for_width(EMPTY_HINT_MOBILE_BREAKPOINT - 1.0);
    let line_height = 20.0;
    let first_y = center.y - (lines.len().saturating_sub(1) as f32 * line_height) * 0.5;
    for (index, line) in lines.iter().enumerate() {
        let title = index == 0;
        painter.text(
            egui::pos2(center.x, first_y + index as f32 * line_height),
            egui::Align2::CENTER_CENTER,
            *line,
            theme::sans(
                if title { 15.0 } else { 12.0 },
                if title {
                    FontWeight::Medium
                } else {
                    FontWeight::Regular
                },
            ),
            if title {
                palette.text_dim
            } else {
                palette.text_faint
            },
        );
    }
}

fn empty_hint_lines_for_width(width: f32) -> &'static [&'static str] {
    if width < EMPTY_HINT_MOBILE_BREAKPOINT {
        &EMPTY_HINT_MOBILE_LINES
    } else {
        &EMPTY_HINT_DESKTOP_LINES
    }
}

#[cfg(test)]
fn empty_hint_estimated_width(line: &str) -> f32 {
    line.chars().count() as f32 * 7.0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::app::AppState;
    use crate::state::{
        AnalysisResult, AnalysisType, ComponentType, DcOpResult, Junction, OperatingPointValue,
        PortDirection, PortSpec, ResolvedCellSymbol, SimulationRun, SymbolDocument, SymbolPin,
        SymbolShape,
    };
    use std::collections::HashMap;

    fn port(name: &str, direction: PortDirection) -> PortSpec {
        PortSpec {
            name: name.to_owned(),
            direction,
        }
    }

    #[test]
    fn component_cull_bounds_use_resolved_symbol_bounds() {
        let component = Component::new(1, ComponentType::CellInstance, Point::new(100, 50));
        let symbol = ResolvedCellSymbol::from_authored_document(
            SymbolDocument {
                body: vec![SymbolShape::Polyline {
                    points: vec![Point::new(80, -10), Point::new(120, 10)],
                    closed: false,
                }],
                pins: vec![SymbolPin::new(
                    "OUT",
                    PortDirection::Out,
                    Some(Point::new(120, 0)),
                )],
                ..SymbolDocument::default()
            },
            &[port("OUT", PortDirection::Out)],
        );
        let mut resolved_by_component_id = HashMap::new();
        resolved_by_component_id.insert(component.id, symbol);
        let context = SchematicSymbolContext {
            resolved_by_component_id,
            resolved_by_binding: Vec::new(),
            pending_library_symbol: None,
            revision: 0,
        };

        assert_eq!(
            component_cull_bounds(&component, &context),
            (Point::new(80, 10), Point::new(220, 90))
        );
    }

    #[test]
    fn phone_width_empty_hint_lines_fit_canvas() {
        let lines = empty_hint_lines_for_width(390.0);
        let safe_width = 390.0 - 32.0;

        for line in lines {
            assert!(
                empty_hint_estimated_width(line) <= safe_width,
                "{line:?} should fit within {safe_width}px"
            );
        }
    }

    fn state_with_operating_point() -> AppState {
        let mut state = AppState::default();
        let mut component = Component::new(1, ComponentType::VoltageSource, Point::new(40, 30));
        component.name = "VBIAS".to_owned();
        state.schematic.components.push(component);
        state.schematic.selection.select_component(1);

        let point = Point::new(20, 10);
        state.schematic.junctions.push(Junction::new(2, point));
        state.simulation.cross_probe.update(
            state.workspace.active_view.clone(),
            HashMap::from([(point, "OUT".to_owned())]),
            HashMap::from([("OUT".to_owned(), vec![Point::new(30, 10), point])]),
            HashMap::new(),
            state.schematic.topology_version(),
        );
        let mut run = SimulationRun::new(1);
        run.add_analysis(
            AnalysisResult::new(1, AnalysisType::DcOp, "OP").with_dc_op(DcOpResult {
                node_voltages: vec![OperatingPointValue {
                    name: "V(out)".to_owned(),
                    value: 1.25,
                    unit: "V".to_owned(),
                }],
                branch_currents: vec![OperatingPointValue {
                    name: "I(vbias)".to_owned(),
                    value: 2.0e-3,
                    unit: "A".to_owned(),
                }],
                power_dissipation: Vec::new(),
            }),
        );
        state.simulation.runs.insert(0, run);
        state.simulation.active_run_idx = Some(0);
        state.simulation.active_analysis_idx = Some(0);
        state
    }

    #[test]
    fn operating_point_policy_maps_latest_values_to_exact_schematic_points() {
        let state = state_with_operating_point();
        let annotations = operating_point_annotations(&state);

        assert_eq!(annotations.len(), 2);
        assert_eq!(annotations[0].position, Point::new(20, 10));
        assert!(annotations[0].label.starts_with("V(out) = 1.25"));
        assert_eq!(annotations[1].position, Point::new(40, 30));
        assert!(annotations[1].selected_current);
    }

    #[test]
    fn hidden_or_voltage_only_policy_enforces_annotation_detail() {
        let mut state = state_with_operating_point();
        state.ui.schematic_visibility.back_annotation =
            SchematicBackAnnotationContent::VoltagesOnly;
        assert_eq!(operating_point_annotations(&state).len(), 1);

        state.ui.schematic_visibility.annotations = SchematicAnnotationVisibility::Hidden;
        assert!(operating_point_annotations(&state).is_empty());
    }

    #[test]
    fn power_detail_is_rendered_only_when_the_session_requests_it() {
        let mut state = state_with_operating_point();
        state.simulation.runs[0].analyses[0]
            .dc_op
            .as_mut()
            .expect("fixture dc op")
            .power_dissipation
            .push(OperatingPointValue {
                name: "P(vbias)".to_owned(),
                value: 2.5e-3,
                unit: "W".to_owned(),
            });
        state.ui.schematic_visibility.back_annotation =
            SchematicBackAnnotationContent::VoltagesCurrentsAndPower;

        let annotations = operating_point_annotations(&state);

        assert!(annotations[1].label.contains("P(vbias)"));
        assert!(annotations[1].label.ends_with('W'));
    }

    #[test]
    fn review_marker_visibility_preserves_non_review_documentation() {
        let plain = crate::state::DesignNote::new(
            10,
            Point::origin(),
            DesignNoteKind::PlainText,
            "Bias network",
        )
        .expect("plain note");
        let mut review = crate::state::DesignNote::new(
            11,
            Point::origin(),
            DesignNoteKind::ReviewNote,
            "Check startup margin",
        )
        .expect("review note");

        assert!(design_note_visible(
            &plain,
            SchematicReviewMarkerVisibility::Hidden
        ));
        assert!(design_note_visible(
            &review,
            SchematicReviewMarkerVisibility::OpenAndAssigned
        ));
        review
            .assign_review(Some("Analog design"))
            .expect("assign note");
        review
            .set_review_state(DesignReviewState::Resolved)
            .expect("resolve note");
        assert!(design_note_visible(
            &review,
            SchematicReviewMarkerVisibility::OpenAndAssigned
        ));
        review
            .assign_review(None::<String>)
            .expect("clear assignment");
        assert!(!design_note_visible(
            &review,
            SchematicReviewMarkerVisibility::OpenAndAssigned
        ));
        assert!(design_note_visible(
            &review,
            SchematicReviewMarkerVisibility::All
        ));
        assert!(!design_note_visible(
            &review,
            SchematicReviewMarkerVisibility::Hidden
        ));
    }

    #[test]
    fn retained_device_op_mode_annotates_only_selected_authoritative_devices() {
        use crate::state::*;

        let mut state = state_with_operating_point();
        let analysis = &mut state.simulation.runs[0].analyses[0];
        analysis.result_payload = Some(AnalysisResultPayload::OperatingPoint {
            temperature_mode: OperatingPointTemperatureEvidence::PvtRunSet,
            temperature_celsius: 27.0,
            initial_guess: OperatingPointInitialGuessEvidence::Automatic,
            node_initialization: OperatingPointNodeInitializationEvidence::UseIcAndNodeset,
            homotopy: OperatingPointHomotopyEvidence::Adaptive,
            annotation: OperatingPointAnnotationEvidence::VoltagesAndDeviceOp,
            device_detail: OperatingPointDeviceDetailEvidence::SelectedAndViolations,
            save_device_op: OperatingPointSaveDeviceEvidence::Enabled,
            accuracy: OperatingPointAccuracyEvidence::Balanced,
            selected_devices: vec!["VBIAS".to_owned()],
            violation_devices: Vec::new(),
            violation_source_content_digest: None,
            validated_startup_directives: 0,
            mna_node_names: vec!["OUT".to_owned()],
            mna_branch_names: vec!["VBIAS".to_owned()],
            mna_solution: vec![1.25, 2.0e-3],
            effective_source_content_digest: None,
            run_point_index: 0,
            run_point_count: 1,
            run_point_process: OperatingPointProcessEvidence::TT,
            run_point_supply_voltage: None,
            run_point_nominal_supply_voltage: None,
        });
        analysis.device_op = Some(rspice_core::circuit::DeviceOpReport {
            entries: vec![rspice_core::circuit::DeviceOpEntry {
                name: "VBIAS".to_owned(),
                device_kind: "SOURCE",
                region: None,
                params: vec![("id", 2.0e-3)],
            }],
        });

        let annotations = operating_point_annotations(&state);
        assert_eq!(annotations.len(), 2, "voltage plus selected device OP");
        assert!(annotations[1].label.contains("VBIAS [SOURCE]"));
        assert!(annotations[1].label.contains("id=2mA"));
    }

    #[test]
    fn topology_edit_invalidates_retained_operating_point_positions() {
        let mut state = state_with_operating_point();
        assert!(!operating_point_annotations(&state).is_empty());

        state.schematic.bump_topology_version();

        assert!(operating_point_annotations(&state).is_empty());
    }

    #[test]
    fn annotations_follow_only_the_explicitly_selected_operating_point() {
        let mut state = state_with_operating_point();
        state.simulation.runs[0].add_analysis(
            AnalysisResult::new(2, AnalysisType::DcOp, "OP hot").with_dc_op(DcOpResult {
                node_voltages: vec![OperatingPointValue {
                    name: "V(out)".to_owned(),
                    value: 2.5,
                    unit: "V".to_owned(),
                }],
                branch_currents: vec![OperatingPointValue {
                    name: "I(vbias)".to_owned(),
                    value: 4.0e-3,
                    unit: "A".to_owned(),
                }],
                power_dissipation: Vec::new(),
            }),
        );

        state.simulation.active_analysis_idx = Some(1);
        let hot = operating_point_annotations(&state);
        assert!(hot[0].label.contains("2.5"), "{hot:?}");
        assert!(hot[1].label.contains("4m"), "{hot:?}");

        state.simulation.active_analysis_idx = Some(0);
        let nominal = operating_point_annotations(&state);
        assert!(nominal[0].label.contains("1.25"));
        assert!(nominal[1].label.contains("2m"));
    }
}
