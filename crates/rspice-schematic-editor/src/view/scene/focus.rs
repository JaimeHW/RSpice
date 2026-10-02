//! Selection-bound keyboard focus geometry on the active sheet.

use super::super::{
    design_notes::conservative_world_bounds as design_note_world_bounds, design_view::DesignView,
    documentation_shapes::world_bounds as documentation_shape_bounds, drawing::probe_world_bounds,
    net_labels::world_bounds as net_label_world_bounds, symbol_context::SchematicSymbolContext,
    viewport::Viewport,
};
use crate::session::selection::SchematicKeyboardFocus;
use egui::{Painter, Rect, Stroke};
use rspice_design::schematic::selection::Selection;
use rspice_design_model::Point;

pub fn draw_keyboard_focus(
    painter: &Painter,
    viewport: &Viewport,
    view: &DesignView<'_>,
    selection: &Selection,
    focus: Option<SchematicKeyboardFocus>,
    symbol_context: &SchematicSymbolContext,
) {
    let Some(focus) = focus else {
        return;
    };
    if !keyboard_focus_matches_selection(view, selection, focus) {
        return;
    }
    let Some((min, max)) = keyboard_focus_bounds(view, symbol_context, focus) else {
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
    let palette = rspice_ui_kit::tokens::active_palette();
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

pub fn keyboard_focus_matches_selection(
    view: &DesignView<'_>,
    selection: &Selection,
    focus: SchematicKeyboardFocus,
) -> bool {
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
                view.document
                    .junctions
                    .iter()
                    .any(|junction| junction.id == id && junction.pos == position)
            })
        }
        SchematicKeyboardFocus::NetLabel(id) => selection.single_net_label() == Some(id),
        SchematicKeyboardFocus::Probe(id) => selection.single_probe() == Some(id),
        SchematicKeyboardFocus::DesignNote(id) => selection.single_design_note() == Some(id),
        SchematicKeyboardFocus::DocumentationShape(id) => {
            selection.single_documentation_shape() == Some(id)
        }
    }
}

fn keyboard_focus_bounds(
    view: &DesignView<'_>,
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
    if !view.object_is_visible(id) {
        return None;
    }
    match focus {
        SchematicKeyboardFocus::Component(id) => view
            .document
            .components
            .iter()
            .find(|object| object.id == id)
            .map(|object| symbol_context.component_bounds(object)),
        SchematicKeyboardFocus::Wire(id) => view
            .document
            .wires
            .iter()
            .find(|object| object.id == id)
            .and_then(|object| points_bounds(&object.points)),
        SchematicKeyboardFocus::Bus(id) => view
            .document
            .buses
            .iter()
            .find(|object| object.id == id)
            .and_then(|object| points_bounds(&object.points)),
        SchematicKeyboardFocus::BusTap(id) => view
            .document
            .bus_taps
            .iter()
            .find(|object| object.id == id)
            .and_then(|object| points_bounds(&crate::bus_geometry::bus_tap_route_points(object))),
        SchematicKeyboardFocus::Junction(id) => view
            .document
            .junctions
            .iter()
            .find(|object| object.id == id)
            .map(|object| (object.pos, object.pos)),
        SchematicKeyboardFocus::NetLabel(id) => view
            .document
            .net_labels
            .iter()
            .find(|object| object.id == id)
            .map(net_label_world_bounds),
        SchematicKeyboardFocus::Probe(id) => view
            .document
            .probes
            .iter()
            .find(|object| object.id == id)
            .map(probe_world_bounds),
        SchematicKeyboardFocus::DesignNote(id) => view
            .visible_design_notes()
            .iter()
            .find(|object| object.id == id)
            .map(design_note_world_bounds),
        SchematicKeyboardFocus::DocumentationShape(id) => view
            .document
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
