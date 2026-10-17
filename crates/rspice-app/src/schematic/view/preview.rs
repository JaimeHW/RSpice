//! App-owned preview orchestration and canonical transform candidates.

use super::super::symbols::SymbolLibrary;
use super::{
    SchematicShelfDragPayload, SchematicSymbolContext, array_interaction::array_placement,
    snap_resolution::resolve_grid_pointer, viewport::Viewport,
};
use crate::state::transform_preview_memo::{
    DrawingInputs, TransformPreviewKey, TransformPreviewOutcome,
};
use crate::state::{Point, SchematicArrayKind, SchematicArrayPlacement, SymbolResolver, Tool};
use crate::workbench::app_state::AppState;
use egui::{Painter, Response};
use rspice_schematic_editor::view::preview::{self, PreviewView, TransformPreviewStyle};
use std::sync::Arc;

pub(super) fn preview_view(state: &AppState) -> PreviewView<'_> {
    PreviewView {
        design: super::schematic_design_view(state),
        editor: &state.schematic.session.editor,
        can_edit: !state.schematic_edit_read_only(),
    }
}

pub(super) fn transform_preview_style<'a>(
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
    super::floating_paste::draw_floating_paste_preview(
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
    super::zoom_box::paint_zoom_box(painter, state, viewport);
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
    let outcome = move_preview_diff(state, symbol_context, delta, mode);
    let valid = match &outcome {
        Ok(Some(_)) => {
            state.dialogs.move_selection.canvas.preview_error = None;
            true
        }
        Ok(None) => return,
        Err(error) => {
            state.dialogs.move_selection.canvas.preview_error = Some(error.clone());
            false
        }
    };

    if let Ok(Some(diff)) = &outcome {
        preview::draw_move_diff(
            painter,
            viewport,
            &preview_view(state),
            diff,
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
    let diff = match stretch_preview_diff(state, symbol_context, delta, target, policy) {
        Ok(Some(diff)) => {
            state.dialogs.stretch_selection.canvas.gesture.preview_error = None;
            diff
        }
        Ok(None) => return,
        Err(error) => {
            state.dialogs.stretch_selection.canvas.gesture.preview_error = Some(error.clone());
            preview::draw_transform_feedback(painter, response, false, error);
            return;
        }
    };

    preview::draw_stretch_diff(painter, viewport, &diff);

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

/// What a canvas derivation reads from the open drawing besides its own
/// inputs.
pub(super) fn drawing_inputs(
    state: &AppState,
    symbol_context: &SchematicSymbolContext,
) -> DrawingInputs {
    let catalog = super::sheet_visibility::active_sheet_catalog(state);
    DrawingInputs {
        topology_version: state.schematic.topology_version(),
        content_version: state.schematic.content_version(),
        symbol_revision: symbol_context.revision(),
        active_sheet: catalog.and_then(crate::state::SheetCatalog::active_sheet_id),
        sheet_catalog_revision: catalog.map(crate::state::SheetCatalog::revision),
    }
}

/// The preview held for `key`, or the one `compute` finds, held from now on.
fn held_transform_preview(
    state: &mut AppState,
    key: TransformPreviewKey,
    compute: impl FnOnce(&AppState) -> TransformPreviewOutcome,
) -> TransformPreviewOutcome {
    if let Some(outcome) = state.schematic.session.transform_preview.current(&key) {
        return outcome;
    }
    let outcome = compute(state);
    state
        .schematic
        .session
        .transform_preview
        .store(key, outcome.clone());
    outcome
}

/// What the armed Move would change, computed once per delta, mode,
/// selection and drawing rather than once per frame.
pub(super) fn move_preview_diff(
    state: &mut AppState,
    symbol_context: &SchematicSymbolContext,
    delta: Point,
    mode: crate::state::MoveSelectionMode,
) -> TransformPreviewOutcome {
    let key = TransformPreviewKey::Move {
        delta,
        mode,
        selection: state.schematic.session.editor.selection.clone(),
        drawing: drawing_inputs(state, symbol_context),
    };
    held_transform_preview(state, key, |state| {
        let (candidate, result) = move_preview_candidate(state, symbol_context, delta, mode);
        match result {
            Ok(true) => Ok(Some(Arc::new(preview::TransformCandidateDiff::between(
                state.schematic.document(),
                candidate.document(),
            )))),
            Ok(false) => Ok(None),
            Err(error) => Err(error.to_string()),
        }
    })
}

/// What the armed Stretch would change, computed once per delta, target,
/// policy and drawing rather than once per frame.
pub(super) fn stretch_preview_diff(
    state: &mut AppState,
    symbol_context: &SchematicSymbolContext,
    delta: Point,
    target: crate::state::StretchTarget,
    policy: crate::state::StretchOrthogonalPolicy,
) -> TransformPreviewOutcome {
    let key = TransformPreviewKey::Stretch {
        delta,
        target,
        policy,
        drawing: drawing_inputs(state, symbol_context),
    };
    held_transform_preview(state, key, |state| {
        match stretch_preview_candidate(state, symbol_context, delta, target, policy) {
            Ok(Some(candidate)) => Ok(Some(Arc::new(preview::TransformCandidateDiff::between(
                state.schematic.document(),
                &candidate,
            )))),
            Ok(None) => Ok(None),
            Err(error) => Err(error.to_string()),
        }
    })
}

/// The document the armed Move would leave, and whether it would move.
pub(super) fn move_preview_candidate(
    state: &AppState,
    symbol_context: &SchematicSymbolContext,
    delta: Point,
    mode: crate::state::MoveSelectionMode,
) -> (
    crate::state::SchematicState,
    Result<bool, rspice_design::schematic::movement::MoveSelectionError>,
) {
    let mut candidate = state.schematic.clone();
    let result = super::sheet_visibility::with_active_wire_topology_in(
        &state.workspace,
        &mut candidate,
        |candidate, scope| {
            candidate.move_selection_with_mode_resolved(
                delta,
                mode,
                |component| symbol_context.terminal_points(component),
                scope,
            )
        },
    );
    (candidate, result)
}

/// The document the armed Stretch would leave.
pub(super) fn stretch_preview_candidate(
    state: &AppState,
    symbol_context: &SchematicSymbolContext,
    delta: Point,
    target: crate::state::StretchTarget,
    policy: crate::state::StretchOrthogonalPolicy,
) -> Result<
    Option<rspice_design::schematic::document::SchematicDocument>,
    rspice_design::schematic::stretch::StretchSelectionError,
> {
    state.schematic.preview_stretch_target_resolved(
        delta,
        target,
        policy,
        |component| symbol_context.terminal_points(component),
        |component| symbol_context.component_bounds_tuple(component),
        super::sheet_visibility::off_active_sheet(&state.workspace),
    )
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
                &|id| super::sheet_visibility::object_is_on_active_sheet(state, id),
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
        state
            .schematic
            .session
            .editor
            .bus_drawing
            .update_preview(position);
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
                state
                    .schematic
                    .session
                    .editor
                    .wire_drawing
                    .update_preview(result.snapped_position);
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

    /// Two resistors, the first selected, on the first of two sheets, with a
    /// wire off its terminal that a stretch can take.
    fn movable_state() -> AppState {
        let mut state = super::super::sheet_visibility::two_sheet_state(&[1, 2, 3], &[4]);
        let document = state.schematic.document_mut_for_test();
        document.components.push(crate::state::Component::new(
            1,
            ComponentType::Resistor,
            Point::new(0, 0),
        ));
        document.components.push(crate::state::Component::new(
            2,
            ComponentType::Resistor,
            Point::new(200, 0),
        ));
        document.wires.push(crate::state::Wire::new(
            3,
            vec![
                Point::new(0, 100),
                Point::new(0, 140),
                Point::new(60, 140),
                Point::new(60, 100),
            ],
        ));
        state.schematic.bump_topology_version();
        state.schematic.session.editor.selection.select_component(1);
        state
    }

    fn held_diff(outcome: TransformPreviewOutcome) -> Arc<preview::TransformCandidateDiff> {
        outcome
            .expect("the preview is valid")
            .expect("the preview changes something")
    }

    /// The move preview is computed once while its inputs stand still, and
    /// afresh when any one of them changes.
    #[test]
    fn the_move_preview_is_held_until_an_input_changes() {
        let mut state = movable_state();
        let symbols = schematic_symbol_context(&state);
        let mode = state.dialogs.move_selection.mode;
        let delta = Point::new(40, 0);
        let first = held_diff(move_preview_diff(&mut state, &symbols, delta, mode));
        assert!(Arc::ptr_eq(
            &first,
            &held_diff(move_preview_diff(&mut state, &symbols, delta, mode))
        ));

        let fresh = |state: &mut AppState, delta: Point, mode| {
            let symbols = schematic_symbol_context(state);
            let before = held_diff(move_preview_diff(state, &symbols, Point::new(40, 0), mode));
            let after = held_diff(move_preview_diff(state, &symbols, delta, mode));
            !Arc::ptr_eq(&before, &after)
        };
        assert!(fresh(&mut state, Point::new(60, 0), mode), "delta");
        let other_mode = if mode == crate::state::MoveSelectionMode::Connected {
            crate::state::MoveSelectionMode::BreakConnections
        } else {
            crate::state::MoveSelectionMode::Connected
        };
        let before = held_diff(move_preview_diff(&mut state, &symbols, delta, mode));
        let after = held_diff(move_preview_diff(&mut state, &symbols, delta, other_mode));
        assert!(!Arc::ptr_eq(&before, &after), "mode");

        let changes: [(&str, fn(&mut AppState)); 5] = [
            ("selection", |state| {
                state.schematic.session.editor.selection.select_component(2);
            }),
            ("topology", |state| state.schematic.bump_topology_version()),
            ("committed content", |state| {
                state.schematic.init_undo_history();
                state.schematic.begin_operation("rename");
                state.schematic.document_mut_for_test().components[1].name = "R9".to_owned();
                assert!(state.schematic.end_operation());
            }),
            ("symbols", |state| {
                state
                    .library_manager
                    .add_library(crate::state::Library::new("other"));
            }),
            ("sheet assignment", |state| {
                let key = state.workspace.content.active_schematic_reference().key();
                let catalog = state
                    .workspace
                    .content
                    .design_management
                    .sheet_catalog_mut(&key)
                    .expect("two sheets");
                let second = catalog.sheets()[1].id();
                catalog
                    .assign_objects(catalog.revision(), second, [2])
                    .expect("R2 moves to Sheet 2");
            }),
        ];
        for (input, change) in changes {
            let mut state = movable_state();
            let symbols = schematic_symbol_context(&state);
            let before = held_diff(move_preview_diff(&mut state, &symbols, delta, mode));
            change(&mut state);
            let symbols = schematic_symbol_context(&state);
            let after = held_diff(move_preview_diff(&mut state, &symbols, delta, mode));
            assert!(!Arc::ptr_eq(&before, &after), "{input}");
        }
    }

    /// The held stretch preview follows its delta and drawing too, and what it
    /// holds is the stretch's own change.
    #[test]
    fn the_stretch_preview_is_held_until_an_input_changes() {
        let mut state = movable_state();
        state.schematic.session.editor.selection.clear();
        state
            .schematic
            .session
            .editor
            .selection
            .select_only_wire_segment(3, 1);
        let target = state
            .schematic
            .default_stretch_target()
            .expect("a segment to stretch");
        let policy = crate::state::StretchOrthogonalPolicy::default();
        let symbols = schematic_symbol_context(&state);
        let diff = |state: &mut AppState, delta: Point| {
            held_diff(stretch_preview_diff(state, &symbols, delta, target, policy))
        };
        let first = diff(&mut state, Point::new(0, 20));
        assert!(Arc::ptr_eq(&first, &diff(&mut state, Point::new(0, 20))));
        assert_eq!(
            *first,
            preview::TransformCandidateDiff::between(
                state.schematic.document(),
                &stretch_preview_candidate(&state, &symbols, Point::new(0, 20), target, policy)
                    .expect("valid stretch")
                    .expect("it changes the wire"),
            )
        );
        assert!(!Arc::ptr_eq(&first, &diff(&mut state, Point::new(0, 40))));
        let held = diff(&mut state, Point::new(0, 40));
        state.schematic.bump_topology_version();
        assert!(!Arc::ptr_eq(&held, &diff(&mut state, Point::new(0, 40))));
    }

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
        state.schematic.session.editor.pending_library_cell = Some(
            rspice_schematic_editor::session::placement::PendingLibraryCellPlacement::new(
                binding,
                crate::workbench::app::schematic_editor_request_source(&state),
            ),
        );
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
