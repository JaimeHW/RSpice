//! Read-only placement previews over the current design and editor session.

use super::{
    bus_interaction::resolve_bus_tap_candidate,
    coordinates::screen_to_schematic,
    design_notes::draw_design_note,
    design_view::DesignView,
    documentation_shapes::{draw_geometry, preview_anchor_color, preview_stroke},
    drawing::{draw_bus, draw_bus_tap},
    net_labels::draw_net_label,
    snap_resolution::{resolve_grid_pointer, resolve_target_pointer},
    symbol_context::SchematicSymbolContext,
    viewport::Viewport,
};
use crate::session::{EditorSession, tool::Tool};
use egui::{Painter, Rect, Response, Stroke};
use rspice_design::schematic::{
    bus::{Bus, BusTap},
    design_note::{DesignNote, DesignNoteRenderContext},
    documentation_shape::geometry_from_points,
    junction_candidates::{collect_junction_candidates, nearest_junction_candidate},
    net_label::NetLabel,
};
use rspice_design_model::{Point, design_management::CrossSheetPortDirection};

mod component;
mod wire;
pub use component::{
    ShelfPreview, draw_component_preview, draw_shelf_drag_preview, pending_library_cell_component,
};
pub use wire::{draw_wire_preview, resolve_wire_preview_snap};

/// Application permissions are evaluated by the caller for this frame.
#[derive(Clone, Copy)]
pub struct PreviewView<'a> {
    pub design: DesignView<'a>,
    pub editor: &'a EditorSession,
    pub can_edit: bool,
}

pub fn draw_documentation_shape_preview(
    painter: &Painter,
    response: &Response,
    view: &PreviewView<'_>,
    viewport: &Viewport,
) {
    if !view.can_edit || view.editor.tool != Tool::DocumentationShape {
        return;
    }
    let Some(pending) = view.editor.pending_documentation_shape.as_ref() else {
        return;
    };
    let drawing = &view.editor.documentation_shape_drawing;
    let hover_point = if drawing.keyboard_active {
        drawing.keyboard_cursor
    } else {
        response.hover_pos().map(|position| {
            resolve_grid_pointer(
                &view.editor.snap_engine,
                view.design.document.grid_size,
                viewport,
                position,
            )
            .snapped_position
        })
    };
    let Some(hover_point) = hover_point else {
        return;
    };
    let hover = viewport.schematic_to_screen(hover_point);
    let mut points = view.editor.documentation_shape_drawing.points.clone();
    if points.last() != Some(&hover_point) {
        points.push(hover_point);
    }
    let geometry = geometry_from_points(pending.kind, &points);
    let valid = geometry.is_ok();
    if let Ok(geometry) = geometry {
        draw_geometry(painter, viewport, &geometry, preview_stroke(true));
    } else if points.len() >= 2 {
        let screen_points = points
            .iter()
            .map(|point| viewport.schematic_to_screen(*point))
            .collect::<Vec<_>>();
        painter.add(egui::Shape::line(screen_points, preview_stroke(false)));
    }
    let color = preview_anchor_color(valid);
    for point in &points {
        painter.circle_stroke(
            viewport.schematic_to_screen(*point),
            3.0,
            Stroke::new(1.0, color),
        );
    }
    let label = format!(
        "{}, {} \u{b7} {} \u{b7} non-electrical",
        hover_point.x,
        hover_point.y,
        pending.kind.label()
    );
    let galley = painter.layout_no_wrap(
        label,
        rspice_ui_kit::theme::mono(
            rspice_ui_kit::tokens::FS_0,
            rspice_ui_kit::theme::FontWeight::Regular,
        ),
        color,
    );
    painter.galley(hover + egui::vec2(10.0, 10.0), galley, color);
}

pub fn draw_design_note_preview(
    painter: &Painter,
    response: &Response,
    view: &PreviewView<'_>,
    viewport: &Viewport,
    view_path: impl FnOnce() -> String,
) {
    if !view.can_edit || view.editor.tool != Tool::DesignNote {
        return;
    }
    let (Some(hover), Some(pending)) = (
        response.hover_pos(),
        view.editor.pending_design_note.as_ref(),
    ) else {
        return;
    };
    let position = resolve_grid_pointer(
        &view.editor.snap_engine,
        view.design.document.grid_size,
        viewport,
        hover,
    )
    .snapped_position;
    let Ok(note) = DesignNote::new(0, position, pending.kind, pending.text.clone()) else {
        return;
    };
    draw_design_note(
        painter,
        viewport,
        &note,
        &DesignNoteRenderContext::for_document(view.design.document, &view_path()),
        false,
        false,
    );
}

pub fn draw_net_label_preview(
    painter: &Painter,
    response: &Response,
    view: &PreviewView<'_>,
    viewport: &Viewport,
    symbol_context: &SchematicSymbolContext,
) {
    if !view.can_edit || !matches!(view.editor.tool, Tool::Label | Tool::OffSheetConnector) {
        return;
    }
    let Some(hover) = response.hover_pos() else {
        return;
    };
    let position = resolve_target_pointer(
        &view.design,
        &view.editor.snap_engine,
        symbol_context,
        viewport,
        hover,
    )
    .snapped_position;
    // The ghost carries the armed tool's kind, so a connector's direction tab
    // appears before the click rather than only after the transaction commits.
    let ghost = if view.editor.tool == Tool::OffSheetConnector {
        NetLabel::off_sheet(
            0,
            position,
            "click to name",
            CrossSheetPortDirection::default(),
        )
    } else {
        NetLabel::new(0, position, "click to name")
    };
    draw_net_label(painter, viewport, &ghost, false, false, true);
}

pub fn draw_junction_preview(
    painter: &Painter,
    response: &Response,
    view: &PreviewView<'_>,
    viewport: &Viewport,
) {
    if !view.can_edit || view.editor.tool != Tool::Junction {
        return;
    }
    let Some(hover_pos) = response.hover_pos() else {
        return;
    };

    let requested = resolve_grid_pointer(
        &view.editor.snap_engine,
        view.design.document.grid_size,
        viewport,
        hover_pos,
    )
    .snapped_position;
    let active_wires = view
        .design
        .objects_on_active_sheet(&view.design.document.wires, |item| item.id);
    let candidates = collect_junction_candidates(active_wires.as_ref());
    let candidate =
        nearest_junction_candidate(&candidates, requested, view.design.document.grid_size);
    let preview = candidate.unwrap_or(requested);
    let pos = viewport.schematic_to_screen(preview);
    let palette = rspice_ui_kit::tokens::active_palette();
    let mixed_bus = candidate.is_some_and(|point| {
        view.design
            .document
            .buses
            .iter()
            .any(|bus| view.design.object_is_visible(bus.id) && bus.contains_point(point))
    });
    let color = match candidate {
        Some(_) if mixed_bus => palette.err,
        Some(point) if view.design.active_junction_at(point).is_some() => palette.warn,
        Some(_) => palette.accent,
        None => palette.err,
    };
    let radius = (4.0 * viewport.zoom).max(3.0);
    painter.circle_stroke(pos, radius, Stroke::new(1.0, color));
    if !mixed_bus && candidate.is_some_and(|point| view.design.active_junction_at(point).is_none())
    {
        painter.circle_filled(pos, (1.75 * viewport.zoom).max(1.5), color);
    }
}

pub fn draw_bus_preview(painter: &Painter, view: &PreviewView<'_>, viewport: &Viewport) {
    if !view.can_edit || view.editor.tool != Tool::Bus || !view.editor.bus_drawing.active {
        return;
    }
    let mut points = view.editor.bus_drawing.points.clone();
    let preview = view.editor.bus_drawing.preview_path();
    points.extend(preview.into_iter().skip(1));
    if points.len() < 2 {
        if let Some(start) = points.first() {
            painter.circle_stroke(
                viewport.schematic_to_screen(*start),
                (5.0 * viewport.zoom).max(3.0),
                Stroke::new(1.0, rspice_ui_kit::tokens::active_palette().accent),
            );
        }
        return;
    }
    let bus = Bus {
        id: 0,
        points,
        declaration: view.editor.bus_drawing.declaration.clone(),
    };
    draw_bus(painter, viewport, &bus, true);
}

pub fn draw_bus_tap_preview(
    painter: &Painter,
    response: &Response,
    view: &PreviewView<'_>,
    viewport: &Viewport,
) {
    if !view.can_edit || view.editor.tool != Tool::BusTap {
        return;
    }
    let Some(hover) = response.hover_pos() else {
        return;
    };
    let requested = screen_to_schematic(viewport, hover);
    let hit_radius = (6.0 / viewport.zoom.max(0.1)).ceil() as i32;
    match resolve_bus_tap_candidate(
        &view.design,
        view.editor.pending_bus_tap.as_ref(),
        requested,
        hit_radius,
    ) {
        Ok(candidate) => {
            let Some(pending) = view.editor.pending_bus_tap.as_ref() else {
                return;
            };
            let tap = BusTap {
                id: 0,
                bus_id: candidate.bus_id,
                bus_point: candidate.bus_point,
                connection_point: candidate.connection_point,
                slice: pending.slice.clone(),
                orientation: candidate.orientation,
            };
            draw_bus_tap(painter, viewport, &tap, true);
        }
        Err(_) => {
            painter.circle_stroke(
                viewport.schematic_to_screen(requested),
                (4.0 * viewport.zoom).max(3.0),
                Stroke::new(1.0, rspice_ui_kit::tokens::active_palette().err),
            );
        }
    }
}

pub fn draw_selection_rect(painter: &Painter, view: &PreviewView<'_>, tool_viewport: &Viewport) {
    if view.editor.selection_rect.is_active() {
        let (min_x, min_y, max_x, max_y) = view.editor.selection_rect.bounds();
        let top_left = tool_viewport.schematic_to_screen(Point::new(min_x, min_y));
        let bottom_right = tool_viewport.schematic_to_screen(Point::new(max_x, max_y));

        let selection_rect = Rect::from_min_max(top_left, bottom_right);

        let accent = rspice_ui_kit::tokens::active_palette().accent;
        painter.rect_filled(selection_rect, 0.0, accent.gamma_multiply(0.14));
        painter.rect_stroke(
            selection_rect,
            0.0,
            Stroke::new(1.0, accent),
            egui::StrokeKind::Inside,
        );
    }
}
