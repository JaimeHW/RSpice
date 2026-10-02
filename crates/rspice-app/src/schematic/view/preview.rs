//! App-owned preview orchestration and canonical transform candidates.

use super::super::symbols::SymbolLibrary;
use super::{
    SchematicShelfDragPayload, SchematicSymbolContext, array_interaction::array_placement,
    snap_resolution::resolve_grid_pointer, viewport::Viewport,
};
use crate::state::{Point, SchematicArrayKind, SchematicArrayPlacement, SymbolResolver, Tool};
use crate::workbench::app_state::AppState;
use egui::{Painter, Response};
use rspice_schematic_editor::view::preview::{self, PreviewView, TransformPreviewStyle};

fn preview_view(state: &AppState) -> PreviewView<'_> {
    PreviewView {
        design: super::schematic_design_view(state),
        editor: &state.schematic.session.editor,
        can_edit: !state.schematic_edit_read_only(),
    }
}

fn transform_preview_style<'a>(
    state: &AppState,
    symbol_context: &'a SchematicSymbolContext,
    symbol_library: Option<&'a SymbolLibrary>,
) -> TransformPreviewStyle<'a> {
    TransformPreviewStyle {
        symbol_library,
        symbol_context,
        parameter_labels: state.ui.schematic_visibility.parameter_labels,
        hover_wire_vertex: state.dialogs.interaction.hover_wire_vertex,
    }
}

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
    let view = preview_view(state);
    preview::draw_bus_tap_preview(painter, response, &view, viewport);
    preview::draw_junction_preview(painter, response, &view, viewport);
    preview::draw_net_label_preview(painter, response, &view, viewport, symbol_context);
    preview::draw_design_note_preview(painter, response, &view, viewport, || {
        state.workspace.content.active_view.display_path()
    });
    preview::draw_documentation_shape_preview(painter, response, &view, viewport);
    preview::draw_component_preview(
        painter,
        response,
        &view,
        viewport,
        symbol_context,
        symbol_library,
    );
    preview::draw_selection_rect(painter, &view, viewport);
    draw_placement_badge(painter, response, state);
}

/// Show the next sequenced placement and the model's current refusal, if any.
fn draw_placement_badge(painter: &Painter, response: &Response, state: &AppState) {
    let Some(sequence) = state
        .schematic
        .session
        .editor
        .pending_port_sequence
        .as_ref()
    else {
        return;
    };
    let (Some(hover), Some(name)) = (response.hover_pos(), sequence.next_name()) else {
        return;
    };
    let palette = crate::ui::tokens::active_palette();
    let refusal = sequence
        .next_placement(
            state.schematic.topology_version(),
            state.schematic.next_interface_order(),
        )
        .and_then(|pending| state.schematic.validate_pending_port(&pending).err());
    let (label, color) = match refusal {
        None => (
            format!("{name}  {}/{}", sequence.position(), sequence.total),
            palette.accent,
        ),
        Some(error) => (format!("{name}: {error}"), palette.err),
    };
    preview::draw_placement_badge(painter, hover, label, color);
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
        || state.schematic.session.editor.tool != Tool::MoveSelection
        || !state.dialogs.move_selection.armed
    {
        return;
    }
    let delta = state.dialogs.move_selection.canvas.preview_delta;
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
            state.dialogs.move_selection.canvas.preview_error = None;
            true
        }
        Ok(false) => return,
        Err(error) => {
            state.dialogs.move_selection.canvas.preview_error = Some(error.to_string());
            false
        }
    };

    if valid {
        preview::draw_move_candidate(
            painter,
            viewport,
            &preview_view(state),
            candidate.document(),
            &transform_preview_style(state, symbol_context, symbol_library),
            || state.workspace.content.active_view.display_path(),
        );
    }

    let detail = state
        .dialogs
        .move_selection
        .canvas
        .preview_error
        .as_deref()
        .map(str::to_owned)
        .unwrap_or_else(|| format!("\u{0394} {}, {} \u{b7} {}", delta.x, delta.y, mode.label()));
    preview::draw_transform_feedback(painter, response, valid, detail);
}

fn draw_stretch_selection_preview(
    painter: &Painter,
    response: &Response,
    state: &mut AppState,
    viewport: &Viewport,
    symbol_context: &SchematicSymbolContext,
) {
    if state.schematic_edit_read_only()
        || state.schematic.session.editor.tool != Tool::StretchSelection
        || !state.dialogs.stretch_selection.armed
    {
        return;
    }
    let delta = state.dialogs.stretch_selection.canvas.gesture.preview_delta;
    let Some(target) = state.dialogs.stretch_selection.canvas.target else {
        return;
    };
    if delta == Point::origin() {
        if let Some(detail) = state
            .dialogs
            .stretch_selection
            .canvas
            .gesture
            .preview_error
            .clone()
        {
            preview::draw_transform_feedback(painter, response, false, detail);
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
            state.dialogs.stretch_selection.canvas.gesture.preview_error = None;
            candidate
        }
        Ok(None) => return,
        Err(error) => {
            state.dialogs.stretch_selection.canvas.gesture.preview_error = Some(error.to_string());
            let detail = error.to_string();
            preview::draw_transform_feedback(painter, response, false, detail);
            return;
        }
    };

    preview::draw_stretch_candidate(painter, viewport, &preview_view(state), &candidate);

    let detail = state
        .dialogs
        .stretch_selection
        .canvas
        .gesture
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
    preview::draw_transform_feedback(painter, response, true, detail);
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
        || state.schematic.session.editor.tool != Tool::ArraySelection
        || !state.dialogs.array_selection.armed
    {
        return;
    }
    let draft = &state.dialogs.array_selection;
    if draft.kind != SchematicArrayKind::RadialDocumentation
        && draft.canvas.preview_delta == Point::origin()
    {
        if let Some(detail) = draft.canvas.preview_error.clone() {
            preview::draw_transform_feedback(painter, response, false, detail);
        }
        return;
    }
    let placement = match array_placement(state) {
        Ok(placement) => placement,
        Err(message) => {
            state.dialogs.array_selection.canvas.preview_error = Some(message.to_owned());
            preview::draw_transform_feedback(painter, response, false, message.to_owned());
            return;
        }
    };
    let plan = match crate::workbench::app::armed_array_selection_plan(state, placement) {
        Ok(plan) => plan,
        Err(message) => {
            state.dialogs.array_selection.canvas.preview_error = Some(message.clone());
            preview::draw_transform_feedback(painter, response, false, message);
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
            state.dialogs.array_selection.canvas.preview_error = None;
            preview
        }
        Err(detail) => {
            let detail = detail.clone();
            state.dialogs.array_selection.canvas.preview_error = Some(detail.clone());
            state.dialogs.array_selection.preview_cache = Some(cache);
            preview::draw_transform_feedback(painter, response, false, detail);
            return;
        }
    };

    preview::draw_array_candidate(
        painter,
        viewport,
        &preview_view(state),
        preview,
        &transform_preview_style(state, symbol_context, symbol_library),
        || state.workspace.content.active_view.display_path(),
    );

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
    preview::draw_transform_feedback(painter, response, true, detail);
}

fn draw_bus_preview(
    painter: &Painter,
    response: &Response,
    state: &mut AppState,
    viewport: &Viewport,
) {
    if state.schematic_edit_read_only()
        || state.schematic.session.editor.tool != Tool::Bus
        || !state.schematic.session.editor.bus_drawing.active
    {
        return;
    }
    if let Some(hover) = response.hover_pos() {
        let position = resolve_grid_pointer(state, viewport, hover).snapped_position;
        state.schematic.update_bus_preview(position);
    }

    preview::draw_bus_preview(painter, &preview_view(state), viewport);
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
    let wire_active = state.schematic.session.editor.wire_drawing.active;

    let snap_feedback = if wire_active {
        response.hover_pos().and_then(|hover_pos| {
            let result = preview::resolve_wire_preview_snap(
                &preview_view(state),
                symbol_context,
                viewport,
                hover_pos,
            );
            if let Some(result) = result.as_ref() {
                state.schematic.update_wire_preview(result.snapped_position);
            } else {
                state.schematic.session.editor.wire_drawing.preview_pos = None;
            }
            result
        })
    } else {
        None
    };

    preview::draw_wire_preview(
        painter,
        &preview_view(state),
        viewport,
        snap_feedback.as_ref(),
    );
}

/// Resolve a shelf payload lazily against current libraries and document buffers.
pub(super) fn draw_shelf_drag_preview(
    painter: &Painter,
    state: &AppState,
    viewport: &Viewport,
    payload: &SchematicShelfDragPayload,
    pointer_pos: egui::Pos2,
    symbol_library: Option<&SymbolLibrary>,
) {
    preview::draw_shelf_drag_preview(
        painter,
        &preview_view(state),
        viewport,
        preview::ShelfPreview {
            component_type: payload.component_type(),
            binding: payload.binding(),
        },
        pointer_pos,
        symbol_library,
        |binding| {
            SymbolResolver::new(
                &state.library_manager,
                &state.workspace.content.schematic_buffers,
            )
            .resolve_binding(binding)
        },
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schematic::view::schematic_symbol_context;
    use crate::state::ComponentType;

    #[test]
    fn wire_preview_snap_resolution_uses_resolved_cell_terminals() {
        let mut state = AppState::default();
        let mut binding = crate::state::LibraryCellInstance::new("work", "amp", "schematic");
        binding.bind_interface(&[crate::state::PortSpec {
            name: "IN".to_string(),
            direction: crate::state::PortDirection::In,
        }]);
        state.schematic.document_mut_for_test().components.push(
            crate::state::Component::new(
                1,
                ComponentType::CellInstance,
                crate::state::Point::new(40, 40),
            )
            .with_library_cell(binding),
        );
        let symbol_context = crate::schematic::view::schematic_symbol_context(&state);
        let component = &state.schematic.document().components[0];
        let terminal =
            component.terminal_positions_resolved(symbol_context.resolved_symbol(component))[0].1;
        let near_terminal = crate::state::Point::new(terminal.x + 1, terminal.y);
        let viewport = Viewport {
            offset: egui::Pos2::ZERO,
            zoom: 1.0,
            bounds: egui::Rect::from_min_size(egui::Pos2::ZERO, egui::Vec2::splat(400.0)),
        };

        assert_eq!(
            preview::resolve_wire_preview_snap(
                &preview_view(&state),
                &symbol_context,
                &viewport,
                egui::pos2(near_terminal.x as f32, near_terminal.y as f32),
            )
            .expect("resolved terminal acquisition")
            .snapped_position,
            terminal
        );
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
        state.schematic.session.editor.pending_library_cell = Some(binding);
        state.schematic.session.editor.preview_rotation = crate::state::Rotation::R90;
        state.schematic.session.editor.preview_mirror_h = true;
        let context = schematic_symbol_context(&state);

        let component = preview::pending_library_cell_component(
            &state.schematic.session.editor,
            Point::new(100, 50),
        )
        .expect("pending component");
        let symbol = context
            .pending_library_symbol()
            .expect("pending authored symbol");

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
