//! Placement preview.
//!
//! Draws the ghost of an object being placed, following the cursor at the
//! current grid snap and orientation, before the placement is committed.

use egui::{Painter, Rect, Response, Stroke, Vec2};

use crate::state::{
    Bus, BusTap, Component, ComponentType, DesignNote, NetLabel, Point, PortDirection,
    ResolvedCellSymbol, SchematicArrayKind, SchematicArrayPlacement, SnapResult, SnapTarget,
    SnapTargetType, SymbolResolver, Tool, geometry_from_points,
};
use crate::workbench::app_state::AppState;

use super::super::symbols::{SymbolLibrary, draw_symbol};
use super::SchematicShelfDragPayload;
use super::SchematicSymbolContext;
use super::array_interaction::array_placement;
use super::bus_interaction::resolve_bus_tap_candidate_on_active_sheet;
use super::coordinates::screen_to_schematic;
use super::design_notes::draw_design_note;
use super::documentation_shapes::{
    draw_documentation_shape, draw_geometry, preview_anchor_color, preview_stroke,
};
use super::drawing::{
    draw_bus, draw_bus_tap, draw_component, draw_junction, draw_port_symbol, draw_wire,
    nearest_wire_screen_hit,
};
use super::net_labels::draw_net_label;
use super::resolved_symbol_render::draw_resolved_symbol;
use super::sheet_visibility::{
    active_junction_at, object_is_on_active_sheet, objects_on_active_sheet,
};
use super::snap_resolution::{resolve_grid_pointer, resolve_target_pointer};
use super::symbol_primitives::{
    draw_capacitor_symbol, draw_diode_symbol, draw_ground_symbol, draw_inductor_symbol,
    draw_isource_symbol, draw_nmos_symbol, draw_npn_symbol, draw_pmos_symbol, draw_pnp_symbol,
    draw_resistor_symbol, draw_vsource_symbol, rotation_to_index,
};
use super::viewport::Viewport;

const WIRE_PREVIEW_STROKE_WIDTH: f32 = 1.5;
const COMPONENT_PREVIEW_GHOST_ALPHA: f32 = 0.55;

pub(super) fn draw_interaction_previews(
    painter: &Painter,
    response: &Response,
    state: &mut AppState,
    viewport: &Viewport,
    symbol_context: &SchematicSymbolContext,
    symbol_library: Option<&SymbolLibrary>,
) {
    draw_move_selection_preview(
        painter,
        response,
        state,
        viewport,
        symbol_context,
        symbol_library,
    );
    draw_stretch_selection_preview(painter, response, state, viewport, symbol_context);
    draw_array_selection_preview(
        painter,
        response,
        state,
        viewport,
        symbol_context,
        symbol_library,
    );
    draw_bus_preview(painter, response, state, viewport);
    draw_wire_preview(painter, response, state, viewport, symbol_context);
    draw_bus_tap_preview(painter, response, state, viewport);
    draw_junction_preview(painter, response, state, viewport);
    draw_net_label_preview(painter, response, state, viewport, symbol_context);
    draw_design_note_preview(painter, response, state, viewport);
    draw_documentation_shape_preview(painter, response, state, viewport);
    draw_component_preview(
        painter,
        response,
        state,
        viewport,
        symbol_context,
        symbol_library,
    );
    draw_selection_rect(painter, state, viewport);
}

fn draw_move_selection_preview(
    painter: &Painter,
    response: &Response,
    state: &mut AppState,
    viewport: &Viewport,
    symbol_context: &SchematicSymbolContext,
    symbol_library: Option<&SymbolLibrary>,
) {
    if state.schematic_edit_read_only()
        || state.schematic.tool != Tool::MoveSelection
        || !state.dialogs.move_selection.armed
    {
        return;
    }
    let delta = state.dialogs.move_selection.preview_delta;
    if delta == Point::origin() {
        return;
    }
    let mode = state.dialogs.move_selection.mode;
    let mut candidate = state.schematic.clone();
    let result = candidate.move_selection_with_mode_resolved(delta, mode, |component| {
        symbol_context.terminal_points(component)
    });
    let valid = match result {
        Ok(true) => {
            state.dialogs.move_selection.preview_error = None;
            true
        }
        Ok(false) => return,
        Err(error) => {
            state.dialogs.move_selection.preview_error = Some(error.to_string());
            false
        }
    };

    if valid {
        for wire in candidate.wires.iter().filter(|candidate_wire| {
            state
                .schematic
                .wires
                .iter()
                .find(|wire| wire.id == candidate_wire.id)
                != Some(*candidate_wire)
        }) {
            draw_wire(painter, viewport, wire, true, None);
        }
        for bus in candidate.buses.iter().filter(|candidate_bus| {
            state
                .schematic
                .buses
                .iter()
                .find(|bus| bus.id == candidate_bus.id)
                != Some(*candidate_bus)
        }) {
            draw_bus(painter, viewport, bus, true);
        }
        for tap in candidate.bus_taps.iter().filter(|candidate_tap| {
            state
                .schematic
                .bus_taps
                .iter()
                .find(|tap| tap.id == candidate_tap.id)
                != Some(*candidate_tap)
        }) {
            draw_bus_tap(painter, viewport, tap, true);
        }
        for component in candidate.components.iter().filter(|candidate_component| {
            state
                .schematic
                .components
                .iter()
                .find(|component| component.id == candidate_component.id)
                != Some(*candidate_component)
        }) {
            draw_component(
                painter,
                viewport,
                component,
                true,
                symbol_library,
                symbol_context,
                state.ui.schematic_visibility.parameter_labels,
            );
        }
        for junction in candidate.junctions.iter().filter(|candidate_junction| {
            !state
                .schematic
                .junctions
                .iter()
                .any(|junction| junction == *candidate_junction)
        }) {
            draw_junction(painter, viewport, junction.pos, state);
        }
        for label in candidate.net_labels.iter().filter(|candidate_label| {
            state
                .schematic
                .net_labels
                .iter()
                .find(|label| label.id == candidate_label.id)
                != Some(*candidate_label)
        }) {
            draw_net_label(painter, viewport, label, true, false, true);
        }
        for note in candidate.design_notes.iter().filter(|candidate_note| {
            state
                .schematic
                .design_notes
                .iter()
                .find(|note| note.id == candidate_note.id)
                != Some(*candidate_note)
        }) {
            draw_design_note(painter, viewport, note, state, true, false);
        }
        for shape in candidate
            .documentation_shapes
            .iter()
            .filter(|candidate_shape| {
                state
                    .schematic
                    .documentation_shapes
                    .iter()
                    .find(|shape| shape.id == candidate_shape.id)
                    != Some(*candidate_shape)
            })
        {
            draw_documentation_shape(painter, viewport, shape, true, false);
        }
    }

    let detail = state
        .dialogs
        .move_selection
        .preview_error
        .as_deref()
        .map(str::to_owned)
        .unwrap_or_else(|| format!("\u{0394} {}, {} \u{b7} {}", delta.x, delta.y, mode.label()));
    draw_transform_feedback(painter, response, valid, detail);
}

fn draw_stretch_selection_preview(
    painter: &Painter,
    response: &Response,
    state: &mut AppState,
    viewport: &Viewport,
    symbol_context: &SchematicSymbolContext,
) {
    if state.schematic_edit_read_only()
        || state.schematic.tool != Tool::StretchSelection
        || !state.dialogs.stretch_selection.armed
    {
        return;
    }
    let delta = state.dialogs.stretch_selection.preview_delta;
    let Some(target) = state.dialogs.stretch_selection.target else {
        return;
    };
    if delta == Point::origin() {
        if let Some(detail) = state.dialogs.stretch_selection.preview_error.clone() {
            draw_transform_feedback(painter, response, false, detail);
        }
        return;
    }
    let policy = state.dialogs.stretch_selection.policy;
    let candidate = match state.schematic.preview_stretch_target_resolved(
        delta,
        target,
        policy,
        |component| symbol_context.terminal_points(component),
        |component| symbol_context.component_bounds_tuple(component),
    ) {
        Ok(Some(candidate)) => {
            state.dialogs.stretch_selection.preview_error = None;
            candidate
        }
        Ok(None) => return,
        Err(error) => {
            state.dialogs.stretch_selection.preview_error = Some(error.to_string());
            let detail = error.to_string();
            draw_transform_feedback(painter, response, false, detail);
            return;
        }
    };

    for wire in candidate.wires.iter().filter(|candidate_wire| {
        state
            .schematic
            .wires
            .iter()
            .find(|wire| wire.id == candidate_wire.id)
            != Some(*candidate_wire)
    }) {
        draw_wire(painter, viewport, wire, true, None);
    }
    for bus in candidate.buses.iter().filter(|candidate_bus| {
        state
            .schematic
            .buses
            .iter()
            .find(|bus| bus.id == candidate_bus.id)
            != Some(*candidate_bus)
    }) {
        draw_bus(painter, viewport, bus, true);
    }
    for tap in candidate.bus_taps.iter().filter(|candidate_tap| {
        state
            .schematic
            .bus_taps
            .iter()
            .find(|tap| tap.id == candidate_tap.id)
            != Some(*candidate_tap)
    }) {
        draw_bus_tap(painter, viewport, tap, true);
    }
    for shape in candidate
        .documentation_shapes
        .iter()
        .filter(|candidate_shape| {
            state
                .schematic
                .documentation_shapes
                .iter()
                .find(|shape| shape.id == candidate_shape.id)
                != Some(*candidate_shape)
        })
    {
        draw_documentation_shape(painter, viewport, shape, true, false);
    }

    let detail = state
        .dialogs
        .stretch_selection
        .preview_error
        .as_deref()
        .map(str::to_owned)
        .unwrap_or_else(|| {
            format!(
                "\u{0394} {}, {} \u{b7} {}",
                delta.x,
                delta.y,
                policy.label()
            )
        });
    draw_transform_feedback(painter, response, true, detail);
}

fn draw_array_selection_preview(
    painter: &Painter,
    response: &Response,
    state: &mut AppState,
    viewport: &Viewport,
    symbol_context: &SchematicSymbolContext,
    symbol_library: Option<&SymbolLibrary>,
) {
    if state.schematic_edit_read_only()
        || state.schematic.tool != Tool::ArraySelection
        || !state.dialogs.array_selection.armed
    {
        return;
    }
    let draft = &state.dialogs.array_selection;
    if draft.kind != SchematicArrayKind::RadialDocumentation
        && draft.preview_delta == Point::origin()
    {
        if let Some(detail) = draft.preview_error.clone() {
            draw_transform_feedback(painter, response, false, detail);
        }
        return;
    }
    let placement = match array_placement(state) {
        Ok(placement) => placement,
        Err(message) => {
            state.dialogs.array_selection.preview_error = Some(message.to_owned());
            draw_transform_feedback(painter, response, false, message.to_owned());
            return;
        }
    };
    let plan = match crate::workbench::app::armed_array_selection_plan(state, placement) {
        Ok(plan) => plan,
        Err(message) => {
            state.dialogs.array_selection.preview_error = Some(message.clone());
            draw_transform_feedback(painter, response, false, message);
            return;
        }
    };
    let library_revision = state.library_manager.revision();
    let symbol_revision = symbol_context.revision();
    let identity_cursor = state.schematic.identity_cursor();
    let mut cache = state.dialogs.array_selection.preview_cache.take();
    let cache_matches = cache.as_ref().is_some_and(|cached| {
        cached.plan == plan
            && cached.library_revision == library_revision
            && cached.symbol_revision == symbol_revision
            && cached.identity_cursor == identity_cursor
    });
    if !cache_matches {
        let preview = state
            .schematic
            .preview_array_selection_resolved(
                &plan,
                |component| symbol_context.named_terminal_points(component),
                |component| symbol_context.component_bounds_tuple(component),
            )
            .map_err(|error| error.to_string());
        cache = Some(crate::workbench::app::ArraySelectionPreviewCache {
            plan,
            library_revision,
            symbol_revision,
            identity_cursor,
            preview,
        });
    }
    let cache = cache.expect("array preview cache was populated");
    let preview = match &cache.preview {
        Ok(preview) => {
            state.dialogs.array_selection.preview_error = None;
            preview
        }
        Err(detail) => {
            let detail = detail.clone();
            state.dialogs.array_selection.preview_error = Some(detail.clone());
            state.dialogs.array_selection.preview_cache = Some(cache);
            draw_transform_feedback(painter, response, false, detail);
            return;
        }
    };

    for wire in preview.wires() {
        draw_wire(painter, viewport, wire, true, None);
    }
    for bus in preview.buses() {
        draw_bus(painter, viewport, bus, true);
    }
    for tap in preview.bus_taps() {
        draw_bus_tap(painter, viewport, tap, true);
    }
    for component in preview.components() {
        draw_component(
            painter,
            viewport,
            component,
            true,
            symbol_library,
            symbol_context,
            state.ui.schematic_visibility.parameter_labels,
        );
    }
    for junction in preview.junctions() {
        draw_junction(painter, viewport, junction.pos, state);
    }
    for label in preview.net_labels() {
        draw_net_label(painter, viewport, label, true, false, true);
    }
    for note in preview.design_notes() {
        draw_design_note(painter, viewport, note, state, true, false);
    }
    for shape in preview.documentation_shapes() {
        draw_documentation_shape(painter, viewport, shape, true, false);
    }

    let impact = preview.impact();
    let detail = match placement {
        SchematicArrayPlacement::Pitch(delta) => format!(
            "{} replicas \u{00b7} pitch \u{0394} {}, {}",
            impact.replicas, delta.x, delta.y
        ),
        SchematicArrayPlacement::Center(center) => format!(
            "{} radial replicas \u{00b7} center {}, {}",
            impact.replicas, center.x, center.y
        ),
    };
    state.dialogs.array_selection.preview_cache = Some(cache);
    draw_transform_feedback(painter, response, true, detail);
}

fn draw_transform_feedback(painter: &Painter, response: &Response, valid: bool, detail: String) {
    let palette = crate::ui::tokens::active_palette();
    let color = if valid { palette.ok } else { palette.err };
    let clip = painter.clip_rect();
    let tooltip_width = (clip.width() * 0.7).clamp(80.0, 300.0);
    let galley = painter.layout(
        detail,
        crate::ui::theme::mono(
            crate::ui::tokens::FS_0,
            crate::ui::theme::FontWeight::Medium,
        ),
        color,
        tooltip_width,
    );
    let requested = response
        .hover_pos()
        .or_else(|| response.interact_pointer_pos())
        .unwrap_or_else(|| clip.center())
        + Vec2::new(12.0, 12.0);
    let background_size = galley.size() + Vec2::splat(10.0);
    let inset = clip.shrink(4.0);
    let background_min = egui::pos2(
        (requested.x - 5.0).clamp(
            inset.left(),
            (inset.right() - background_size.x).max(inset.left()),
        ),
        (requested.y - 5.0).clamp(
            inset.top(),
            (inset.bottom() - background_size.y).max(inset.top()),
        ),
    );
    let background = Rect::from_min_size(background_min, background_size);
    let position = background.min + Vec2::splat(5.0);
    painter.rect_filled(background, 5.0, palette.bg_inset.gamma_multiply(0.96));
    painter.rect_stroke(
        background,
        5.0,
        Stroke::new(1.0, color.gamma_multiply(0.75)),
        egui::StrokeKind::Inside,
    );
    painter.galley(position, galley, color);
}

fn draw_documentation_shape_preview(
    painter: &Painter,
    response: &Response,
    state: &AppState,
    viewport: &Viewport,
) {
    if state.schematic_edit_read_only() || state.schematic.tool != Tool::DocumentationShape {
        return;
    }
    let Some(pending) = state.schematic.pending_documentation_shape.as_ref() else {
        return;
    };
    let drawing = &state.schematic.documentation_shape_drawing;
    let hover_point = if drawing.keyboard_active {
        drawing.keyboard_cursor
    } else {
        response
            .hover_pos()
            .map(|position| resolve_grid_pointer(state, viewport, position).snapped_position)
    };
    let Some(hover_point) = hover_point else {
        return;
    };
    let hover = viewport.schematic_to_screen(hover_point);
    let mut points = state.schematic.documentation_shape_drawing.points.clone();
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
        crate::ui::theme::mono(
            crate::ui::tokens::FS_0,
            crate::ui::theme::FontWeight::Regular,
        ),
        color,
    );
    painter.galley(hover + egui::vec2(10.0, 10.0), galley, color);
}

fn draw_design_note_preview(
    painter: &Painter,
    response: &Response,
    state: &AppState,
    viewport: &Viewport,
) {
    if state.schematic_edit_read_only() || state.schematic.tool != Tool::DesignNote {
        return;
    }
    let (Some(hover), Some(pending)) = (
        response.hover_pos(),
        state.schematic.pending_design_note.as_ref(),
    ) else {
        return;
    };
    let position = resolve_grid_pointer(state, viewport, hover).snapped_position;
    let Ok(note) = DesignNote::new(0, position, pending.kind, pending.text.clone()) else {
        return;
    };
    draw_design_note(painter, viewport, &note, state, false, false);
}

fn draw_net_label_preview(
    painter: &Painter,
    response: &Response,
    state: &AppState,
    viewport: &Viewport,
    symbol_context: &SchematicSymbolContext,
) {
    if state.schematic_edit_read_only() || state.schematic.tool != Tool::Label {
        return;
    }
    let Some(hover) = response.hover_pos() else {
        return;
    };
    let position = resolve_target_pointer(state, symbol_context, viewport, hover).snapped_position;
    draw_net_label(
        painter,
        viewport,
        &NetLabel::new(0, position, "click to name"),
        false,
        false,
        true,
    );
}

fn draw_junction_preview(
    painter: &Painter,
    response: &Response,
    state: &AppState,
    viewport: &Viewport,
) {
    if state.schematic_edit_read_only() || state.schematic.tool != Tool::Junction {
        return;
    }
    let Some(hover_pos) = response.hover_pos() else {
        return;
    };

    let requested = resolve_grid_pointer(state, viewport, hover_pos).snapped_position;
    let active_wires = objects_on_active_sheet(state, &state.schematic.wires, |item| item.id);
    let mut hit_schematic = crate::state::SchematicState::default();
    hit_schematic.wires = active_wires.into_owned();
    let candidate = hit_schematic.nearest_junction_candidate(requested, state.schematic.grid_size);
    let preview = candidate.unwrap_or(requested);
    let pos = viewport.schematic_to_screen(preview);
    let palette = crate::ui::tokens::active_palette();
    let mixed_bus = candidate.is_some_and(|point| {
        state
            .schematic
            .buses
            .iter()
            .any(|bus| object_is_on_active_sheet(state, bus.id) && bus.contains_point(point))
    });
    let color = match candidate {
        Some(_) if mixed_bus => palette.err,
        Some(point) if active_junction_at(state, point).is_some() => palette.warn,
        Some(_) => palette.accent,
        None => palette.err,
    };
    let radius = (4.0 * viewport.zoom).max(3.0);
    painter.circle_stroke(pos, radius, Stroke::new(1.0, color));
    if !mixed_bus && candidate.is_some_and(|point| active_junction_at(state, point).is_none()) {
        painter.circle_filled(pos, (1.75 * viewport.zoom).max(1.5), color);
    }
}

fn draw_bus_preview(
    painter: &Painter,
    response: &Response,
    state: &mut AppState,
    viewport: &Viewport,
) {
    if state.schematic_edit_read_only()
        || state.schematic.tool != Tool::Bus
        || !state.schematic.bus_drawing.active
    {
        return;
    }
    if let Some(hover) = response.hover_pos() {
        let position = resolve_grid_pointer(state, viewport, hover).snapped_position;
        state.schematic.update_bus_preview(position);
    }

    let mut points = state.schematic.bus_drawing.points.clone();
    let preview = state.schematic.bus_drawing.preview_path();
    points.extend(preview.into_iter().skip(1));
    if points.len() < 2 {
        if let Some(start) = points.first() {
            painter.circle_stroke(
                viewport.schematic_to_screen(*start),
                (5.0 * viewport.zoom).max(3.0),
                Stroke::new(1.0, crate::ui::tokens::active_palette().accent),
            );
        }
        return;
    }
    let bus = Bus {
        id: 0,
        points,
        declaration: state.schematic.bus_drawing.declaration.clone(),
    };
    draw_bus(painter, viewport, &bus, true);
}

fn draw_bus_tap_preview(
    painter: &Painter,
    response: &Response,
    state: &AppState,
    viewport: &Viewport,
) {
    if state.schematic_edit_read_only() || state.schematic.tool != Tool::BusTap {
        return;
    }
    let Some(hover) = response.hover_pos() else {
        return;
    };
    let requested = screen_to_schematic(viewport, hover);
    let hit_radius = (6.0 / viewport.zoom.max(0.1)).ceil() as i32;
    match resolve_bus_tap_candidate_on_active_sheet(state, requested, hit_radius) {
        Ok(candidate) => {
            let Some(pending) = state.schematic.pending_bus_tap.as_ref() else {
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
                Stroke::new(1.0, crate::ui::tokens::active_palette().err),
            );
        }
    }
}

fn draw_wire_preview(
    painter: &Painter,
    response: &Response,
    state: &mut AppState,
    viewport: &Viewport,
    symbol_context: &SchematicSymbolContext,
) {
    if state.schematic_edit_read_only() {
        return;
    }
    let wire_active = state.schematic.wire_drawing.active;

    let snap_feedback = if wire_active {
        response.hover_pos().and_then(|hover_pos| {
            let result = resolve_wire_preview_snap(state, symbol_context, viewport, hover_pos);
            if let Some(result) = result.as_ref() {
                state.schematic.update_wire_preview(result.snapped_position);
            } else {
                state.schematic.wire_drawing.preview_pos = None;
            }
            result
        })
    } else {
        None
    };

    if wire_active {
        let wire_points: Vec<Point> = state.schematic.wire_drawing.points.clone();
        let preview_pos_opt = state.schematic.wire_drawing.preview_pos;

        if !wire_points.is_empty() {
            let wire_color = crate::ui::tokens::active_palette().accent;
            let stroke = Stroke::new(WIRE_PREVIEW_STROKE_WIDTH * viewport.zoom, wire_color);

            for segment in wire_points.windows(2) {
                let p1 = viewport.schematic_to_screen(segment[0]);
                let p2 = viewport.schematic_to_screen(segment[1]);
                painter.line_segment([p1, p2], stroke);
            }

            if let Some(preview) = preview_pos_opt
                && let Some(last) = wire_points.last()
            {
                let p1 = viewport.schematic_to_screen(*last);
                let p2 = viewport.schematic_to_screen(preview);
                painter.line_segment(
                    [p1, p2],
                    Stroke::new(
                        WIRE_PREVIEW_STROKE_WIDTH * viewport.zoom,
                        wire_color.gamma_multiply(0.6),
                    ),
                );
            }

            if let Some(start) = wire_points.first() {
                let start_screen = viewport.schematic_to_screen(*start);
                painter.circle_filled(start_screen, 4.0 * viewport.zoom, wire_color);
            }
        }
    }

    if let Some(result) = snap_feedback.as_ref() {
        draw_wire_snap_feedback(painter, state, viewport, result);
    }
}

/// Resolve exactly what a wire click would commit. Visual conductor
/// acquisition owns the gesture before generic target priority, and a
/// non-representable diagonal acquisition fails closed instead of showing a
/// preview that cannot be committed.
fn resolve_wire_preview_snap(
    state: &AppState,
    symbol_context: &SchematicSymbolContext,
    viewport: &Viewport,
    pointer: egui::Pos2,
) -> Option<SnapResult> {
    if !state.schematic.snap_engine.enabled {
        return Some(resolve_target_pointer(
            state,
            symbol_context,
            viewport,
            pointer,
        ));
    }
    let active_wires = objects_on_active_sheet(state, &state.schematic.wires, |wire| wire.id);
    let Some(hit) = nearest_wire_screen_hit(viewport, active_wires.as_ref(), pointer, 6.0) else {
        return Some(resolve_target_pointer(
            state,
            symbol_context,
            viewport,
            pointer,
        ));
    };
    let attachment = hit.attachment?;
    let wire = active_wires.iter().find(|wire| wire.id == hit.wire_id)?;
    let raw = screen_to_schematic(viewport, pointer);
    let distance = (f64::from(raw.x) - f64::from(attachment.x))
        .hypot(f64::from(raw.y) - f64::from(attachment.y));
    let target = if wire.points.first() == Some(&attachment) {
        state
            .schematic
            .snap_engine
            .snap_to_wire_endpoints
            .then(|| SnapTarget::wire_endpoint(attachment, wire.id, true, distance))
    } else if wire.points.last() == Some(&attachment) {
        state
            .schematic
            .snap_engine
            .snap_to_wire_endpoints
            .then(|| SnapTarget::wire_endpoint(attachment, wire.id, false, distance))
    } else {
        let segment_index = wire
            .segments()
            .position(|segment| segment.contains_point(attachment))
            .unwrap_or_default();
        state
            .schematic
            .snap_engine
            .snap_to_wire_segments
            .then(|| SnapTarget::wire_segment(attachment, wire.id, segment_index, distance))
    };
    Some(match target {
        Some(target) => SnapResult::with_target(target, raw),
        None => resolve_target_pointer(state, symbol_context, viewport, pointer),
    })
}

fn draw_wire_snap_feedback(
    painter: &Painter,
    state: &AppState,
    viewport: &Viewport,
    result: &SnapResult,
) {
    if !result.show_indicator {
        return;
    }
    let Some(copy) = wire_snap_feedback_copy(state, result) else {
        return;
    };

    let palette = crate::ui::tokens::active_palette();
    let center = viewport.schematic_to_screen(result.snapped_position);
    painter.circle_stroke(center, 5.0, Stroke::new(1.25, palette.accent));
    painter.line_segment(
        [center - egui::vec2(2.5, 0.0), center + egui::vec2(2.5, 0.0)],
        Stroke::new(1.0, palette.accent),
    );
    painter.line_segment(
        [center - egui::vec2(0.0, 2.5), center + egui::vec2(0.0, 2.5)],
        Stroke::new(1.0, palette.accent),
    );

    let galley = painter.layout_no_wrap(
        copy,
        crate::ui::theme::mono(
            crate::ui::tokens::FS_0,
            crate::ui::theme::FontWeight::Regular,
        ),
        palette.text,
    );
    let requested = center + egui::vec2(10.0, -galley.size().y * 0.5);
    let background_size = galley.size() + egui::vec2(8.0, 5.0);
    let clip = painter.clip_rect().shrink(3.0);
    let origin = egui::pos2(
        requested.x.clamp(
            clip.left(),
            (clip.right() - background_size.x).max(clip.left()),
        ),
        requested.y.clamp(
            clip.top(),
            (clip.bottom() - background_size.y).max(clip.top()),
        ),
    );
    let background = Rect::from_min_size(origin, background_size);
    painter.rect_filled(background, 3.0, palette.bg_elevated.gamma_multiply(0.96));
    painter.rect_stroke(
        background,
        3.0,
        Stroke::new(1.0, palette.border_strong),
        egui::StrokeKind::Inside,
    );
    painter.galley(origin + egui::vec2(4.0, 2.5), galley, palette.text);
}

/// Copy shown beside the live acquisition marker. Net names are included only
/// when an authored label proves them; unknown nets are never guessed from IDs
/// or a potentially stale generated-net cache.
fn wire_snap_feedback_copy(state: &AppState, result: &SnapResult) -> Option<String> {
    if !result.show_indicator {
        return None;
    }
    let target = result.target.as_ref()?;
    let target_copy = match &target.target_type {
        SnapTargetType::Terminal {
            component_id,
            terminal_name,
        } => {
            let component = state
                .schematic
                .components
                .iter()
                .find(|component| component.id == *component_id);
            let owner = component
                .filter(|component| !component.name.trim().is_empty())
                .map(|component| component.name.as_str())
                .map(str::to_owned)
                .unwrap_or_else(|| format!("component #{component_id}"));
            format!("Pin {owner}.{terminal_name}")
        }
        SnapTargetType::Junction => "Junction".to_owned(),
        SnapTargetType::WireEndpoint { wire_id, is_start } => format!(
            "Wire #{wire_id} {} endpoint",
            if *is_start { "start" } else { "end" }
        ),
        SnapTargetType::WireSegment {
            wire_id,
            segment_index,
        } => format!("Wire #{wire_id} segment {}", segment_index + 1),
        SnapTargetType::Grid => return None,
    };

    Some(match net_name_at_snap_target(state, target.position) {
        Some(net_name) => format!("{target_copy} | net {net_name}"),
        None => target_copy,
    })
}

fn net_name_at_snap_target(state: &AppState, target: Point) -> Option<&str> {
    state
        .schematic
        .net_labels
        .iter()
        .find(|label| {
            object_is_on_active_sheet(state, label.id)
                && label.pos == target
                && !label.name.trim().is_empty()
        })
        .map(|label| label.name.as_str())
        .or_else(|| {
            state
                .schematic
                .wires
                .iter()
                .filter(|wire| {
                    object_is_on_active_sheet(state, wire.id) && wire.contains_point(target)
                })
                .find_map(|wire| {
                    state
                        .schematic
                        .net_labels
                        .iter()
                        .find(|label| {
                            object_is_on_active_sheet(state, label.id)
                                && !label.name.trim().is_empty()
                                && wire.contains_point(label.pos)
                        })
                        .map(|label| label.name.as_str())
                })
        })
}

fn pending_library_cell_preview<'a>(
    state: &AppState,
    symbol_context: &'a SchematicSymbolContext,
    grid_pos: Point,
) -> Option<(Component, &'a ResolvedCellSymbol)> {
    let binding = state.schematic.pending_library_cell.clone()?;
    let symbol = symbol_context.pending_library_symbol()?;
    let component = Component::new(0, ComponentType::CellInstance, grid_pos)
        .with_rotation(state.schematic.preview_rotation)
        .with_mirror_h(state.schematic.preview_mirror_h)
        .with_library_cell(binding);
    Some((component, symbol))
}

fn draw_component_preview(
    painter: &Painter,
    response: &Response,
    state: &AppState,
    viewport: &Viewport,
    symbol_context: &SchematicSymbolContext,
    symbol_library: Option<&SymbolLibrary>,
) {
    if !component_preview_enabled(state.schematic_edit_read_only()) {
        return;
    }

    let preview_tool = state.schematic.tool;
    let preview_rotation_degrees = state.schematic.preview_rotation.degrees();
    let preview_rotation_index = rotation_to_index(state.schematic.preview_rotation);
    let preview_mirror_h = state.schematic.preview_mirror_h;

    if let Tool::Place(component_type) = preview_tool
        && let Some(hover_pos) = response.hover_pos()
    {
        let grid_pos = resolve_grid_pointer(state, viewport, hover_pos).snapped_position;
        let preview_pos = viewport.schematic_to_screen(grid_pos);

        // Ghost the symbol in dimmed accent until it is placed.
        let preview_stroke = Stroke::new(
            1.0 * viewport.zoom,
            crate::ui::tokens::active_palette()
                .accent
                .gamma_multiply(COMPONENT_PREVIEW_GHOST_ALPHA),
        );

        if component_type == ComponentType::CellInstance
            && let Some((preview_component, symbol)) =
                pending_library_cell_preview(state, symbol_context, grid_pos)
        {
            draw_resolved_symbol(
                painter,
                preview_pos,
                viewport.zoom,
                &preview_component,
                symbol,
                preview_stroke,
            );
            return;
        }

        if component_type == ComponentType::Port {
            let direction = state
                .schematic
                .pending_port
                .as_ref()
                .map(|pending| pending.contract.direction)
                .unwrap_or_default();
            draw_port_symbol(
                painter,
                preview_pos,
                viewport.zoom,
                preview_rotation_index,
                (preview_mirror_h, false),
                direction,
                preview_stroke,
            );
            return;
        }

        let svg_rendered = if let Some(library) = symbol_library {
            if let Some((symbol, adjusted_rotation)) =
                library.get_with_rotation_variant(component_type, preview_rotation_degrees, None)
            {
                draw_symbol(
                    painter,
                    symbol,
                    preview_pos,
                    viewport.zoom,
                    adjusted_rotation,
                    preview_mirror_h,
                    false,
                    preview_stroke,
                );
                true
            } else {
                false
            }
        } else {
            false
        };

        if !svg_rendered {
            draw_procedural_component_preview(
                painter,
                component_type,
                preview_pos,
                viewport.zoom,
                preview_rotation_index,
                preview_stroke,
            );
        }
    }
}

/// Paint the ephemeral component-shelf payload at its snapped drop point.
///
/// The payload never enters application state before release, so an aborted or
/// invalid drag cannot leak an armed tool or pending library-cell binding.
pub(super) fn draw_shelf_drag_preview(
    painter: &Painter,
    state: &AppState,
    viewport: &Viewport,
    payload: &SchematicShelfDragPayload,
    pointer_pos: egui::Pos2,
    symbol_library: Option<&SymbolLibrary>,
) {
    if !component_preview_enabled(state.schematic_edit_read_only()) {
        return;
    }
    let grid_pos = resolve_grid_pointer(state, viewport, pointer_pos).snapped_position;
    let preview_pos = viewport.schematic_to_screen(grid_pos);
    let rotation = state.schematic.preview_rotation;
    let rotation_degrees = rotation.degrees();
    let rotation_index = rotation_to_index(rotation);
    let mirror_h = state.schematic.preview_mirror_h;
    let preview_stroke = Stroke::new(
        viewport.zoom,
        crate::ui::tokens::active_palette()
            .accent
            .gamma_multiply(COMPONENT_PREVIEW_GHOST_ALPHA),
    );

    if let Some(binding) = payload.binding() {
        let resolver =
            SymbolResolver::new(&state.library_manager, &state.workspace.schematic_buffers);
        if let Some(symbol) = resolver.resolve_binding(binding) {
            let component = Component::new(0, ComponentType::CellInstance, grid_pos)
                .with_rotation(rotation)
                .with_mirror_h(mirror_h)
                .with_library_cell(binding.clone());
            draw_resolved_symbol(
                painter,
                preview_pos,
                viewport.zoom,
                &component,
                &symbol,
                preview_stroke,
            );
            return;
        }
    }

    let component_type = payload.component_type();
    let svg_rendered = symbol_library.is_some_and(|library| {
        library
            .get_with_rotation_variant(component_type, rotation_degrees, None)
            .is_some_and(|(symbol, adjusted_rotation)| {
                draw_symbol(
                    painter,
                    symbol,
                    preview_pos,
                    viewport.zoom,
                    adjusted_rotation,
                    mirror_h,
                    false,
                    preview_stroke,
                );
                true
            })
    });
    if !svg_rendered {
        draw_procedural_component_preview(
            painter,
            component_type,
            preview_pos,
            viewport.zoom,
            rotation_index,
            preview_stroke,
        );
    }
}

pub(super) fn draw_procedural_component_preview(
    painter: &Painter,
    component_type: ComponentType,
    preview_pos: egui::Pos2,
    zoom: f32,
    rotation_index: i32,
    preview_stroke: Stroke,
) {
    match component_type {
        ComponentType::Resistor => {
            draw_resistor_symbol(painter, preview_pos, zoom, rotation_index, preview_stroke)
        }
        ComponentType::Capacitor => {
            draw_capacitor_symbol(painter, preview_pos, zoom, rotation_index, preview_stroke)
        }
        ComponentType::Inductor => {
            draw_inductor_symbol(painter, preview_pos, zoom, rotation_index, preview_stroke)
        }
        ComponentType::VoltageSource => {
            draw_vsource_symbol(painter, preview_pos, zoom, rotation_index, preview_stroke)
        }
        ComponentType::CurrentSource => {
            draw_isource_symbol(painter, preview_pos, zoom, rotation_index, preview_stroke)
        }
        ComponentType::Ground => draw_ground_symbol(painter, preview_pos, zoom, preview_stroke),
        ComponentType::Port => draw_port_symbol(
            painter,
            preview_pos,
            zoom,
            rotation_index,
            (false, false),
            PortDirection::default(),
            preview_stroke,
        ),
        ComponentType::Diode => {
            draw_diode_symbol(painter, preview_pos, zoom, rotation_index, preview_stroke)
        }
        ComponentType::Nmos => {
            draw_nmos_symbol(painter, preview_pos, zoom, rotation_index, preview_stroke)
        }
        ComponentType::Pmos => {
            draw_pmos_symbol(painter, preview_pos, zoom, rotation_index, preview_stroke)
        }
        ComponentType::NpnBjt => {
            draw_npn_symbol(painter, preview_pos, zoom, rotation_index, preview_stroke)
        }
        ComponentType::PnpBjt => {
            draw_pnp_symbol(painter, preview_pos, zoom, rotation_index, preview_stroke)
        }
        _ => {
            let rect = Rect::from_center_size(preview_pos, Vec2::splat(30.0 * zoom));
            painter.rect_stroke(rect, 2.0, preview_stroke, egui::StrokeKind::Inside);
        }
    }
}

fn component_preview_enabled(read_only: bool) -> bool {
    !read_only
}

fn draw_selection_rect(painter: &Painter, state: &AppState, tool_viewport: &Viewport) {
    if state.schematic.selection_rect.is_active() {
        let (min_x, min_y, max_x, max_y) = state.schematic.selection_rect.bounds();
        let top_left = tool_viewport.schematic_to_screen(Point::new(min_x, min_y));
        let bottom_right = tool_viewport.schematic_to_screen(Point::new(max_x, max_y));

        let selection_rect = Rect::from_min_max(top_left, bottom_right);

        let accent = crate::ui::tokens::active_palette().accent;
        painter.rect_filled(selection_rect, 0.0, accent.gamma_multiply(0.14));
        painter.rect_stroke(
            selection_rect,
            0.0,
            Stroke::new(1.0, accent),
            egui::StrokeKind::Inside,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn component_preview_ghost_uses_design_alpha_and_hides_on_read_only() {
        assert!((COMPONENT_PREVIEW_GHOST_ALPHA - 0.55).abs() < f32::EPSILON);
        assert!(component_preview_enabled(false));
        assert!(!component_preview_enabled(true));
    }

    #[test]
    fn wire_preview_stroke_width_matches_live_preview_spec() {
        assert!((WIRE_PREVIEW_STROKE_WIDTH - 1.5).abs() < f32::EPSILON);
    }

    #[test]
    fn wire_preview_snap_resolution_uses_resolved_cell_terminals() {
        let mut state = AppState::default();
        let mut binding = crate::state::LibraryCellInstance::new("work", "amp", "schematic");
        binding.bind_interface(&[crate::state::PortSpec {
            name: "IN".to_string(),
            direction: crate::state::PortDirection::In,
        }]);
        state.schematic.components.push(
            crate::state::Component::new(
                1,
                ComponentType::CellInstance,
                crate::state::Point::new(40, 40),
            )
            .with_library_cell(binding),
        );
        let symbol_context = crate::schematic::view::SchematicSymbolContext::from_state(&state);
        let component = &state.schematic.components[0];
        let terminal =
            component.terminal_positions_resolved(symbol_context.resolved_symbol(component))[0].1;
        let near_terminal = crate::state::Point::new(terminal.x + 1, terminal.y);
        let viewport = Viewport {
            offset: egui::Pos2::ZERO,
            zoom: 1.0,
            bounds: egui::Rect::from_min_size(egui::Pos2::ZERO, egui::Vec2::splat(400.0)),
        };

        assert_eq!(
            resolve_target_pointer(
                &state,
                &symbol_context,
                &viewport,
                egui::pos2(near_terminal.x as f32, near_terminal.y as f32),
            )
            .snapped_position,
            terminal
        );
    }

    #[test]
    fn wire_preview_matches_exact_visual_conductor_attachment() {
        let mut state = AppState::default();
        state.schematic.wires.push(crate::state::Wire::segment(
            5,
            Point::new(0, 10),
            Point::new(20, 10),
        ));
        let symbol_context = SchematicSymbolContext::from_state(&state);
        let viewport = Viewport {
            offset: egui::Pos2::ZERO,
            zoom: 2.0,
            bounds: egui::Rect::from_min_size(egui::Pos2::ZERO, egui::Vec2::splat(400.0)),
        };
        let pointer = viewport.schematic_to_screen(Point::new(7, 10)) + egui::vec2(0.0, 2.0);
        let result = resolve_wire_preview_snap(&state, &symbol_context, &viewport, pointer)
            .expect("representable conductor acquisition");

        assert_eq!(result.snapped_position, Point::new(7, 10));
        assert_eq!(
            result.target_type(),
            Some(&SnapTargetType::WireSegment {
                wire_id: 5,
                segment_index: 0,
            })
        );
        assert!(result.show_indicator);
    }

    #[test]
    fn wire_snap_feedback_uses_retained_net_name_and_never_invents_one() {
        let mut state = AppState::default();
        let component = Component::new(7, ComponentType::Resistor, Point::new(20, 20))
            .with_name_value("R7", "1k");
        let terminal_position = component.terminal_positions()[0].1;
        state.schematic.components.push(component);
        state
            .schematic
            .net_labels
            .push(NetLabel::new(1, terminal_position, "VOUT"));
        let terminal = state.schematic.snap_engine.find_snap_target(
            terminal_position,
            &state.schematic.components,
            &[],
            &[],
        );
        let expected = format!(
            "Pin R7.{} | net VOUT",
            terminal.terminal_name().expect("terminal target")
        );
        assert_eq!(
            wire_snap_feedback_copy(&state, &terminal).as_deref(),
            Some(expected.as_str())
        );

        let wire = crate::state::Wire::segment(41, Point::new(60, 20), Point::new(80, 20));
        let unknown_wire = state.schematic.snap_engine.find_snap_target(
            Point::new(80, 20),
            &[],
            std::slice::from_ref(&wire),
            &[],
        );
        let copy = wire_snap_feedback_copy(&state, &unknown_wire).expect("target copy");
        assert_eq!(copy, "Wire #41 end endpoint");
        assert!(!copy.contains("net"));
    }

    #[test]
    fn pending_library_cell_preview_uses_selected_authored_symbol() {
        let mut state = AppState::default();
        let mut library = crate::state::Library::new("work");
        let mut cell = crate::state::Cell::new("amp");
        cell.add_view(crate::state::View::new(
            "schematic",
            crate::state::ViewType::Schematic,
        ));
        let mut symbol_view = crate::state::View::new("symbol", crate::state::ViewType::Symbol);
        crate::state::SymbolDocument {
            pins: vec![crate::state::SymbolPin::new(
                "OUT",
                crate::state::PortDirection::Out,
                Some(Point::new(40, 0)),
            )],
            ..crate::state::SymbolDocument::default()
        }
        .store_in_view(&mut symbol_view)
        .expect("symbol stores");
        cell.add_view(symbol_view);
        library.add_cell(cell);
        state.library_manager.add_library(library);

        let mut binding = crate::state::LibraryCellInstance::new("work", "amp", "schematic");
        binding.bind_interface(&[crate::state::PortSpec {
            name: "OUT".to_owned(),
            direction: crate::state::PortDirection::Out,
        }]);
        state.schematic.pending_library_cell = Some(binding);
        state.schematic.preview_rotation = crate::state::Rotation::R90;
        state.schematic.preview_mirror_h = true;
        let context = SchematicSymbolContext::from_state(&state);

        let (component, symbol) =
            pending_library_cell_preview(&state, &context, Point::new(100, 50))
                .expect("pending library cell has preview symbol");

        assert_eq!(component.kind, ComponentType::CellInstance);
        assert_eq!(component.pos, Point::new(100, 50));
        assert_eq!(component.rotation, crate::state::Rotation::R90);
        assert!(component.mirror_h);
        assert_eq!(
            component
                .library_cell
                .as_ref()
                .map(|binding| (binding.library.as_str(), binding.cell.as_str())),
            Some(("work", "amp"))
        );
        assert_eq!(symbol.connectable_pins().count(), 1);
    }
}
