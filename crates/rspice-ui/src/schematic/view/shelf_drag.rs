//! Component-shelf drag-and-drop contract for the schematic canvas.
//!
//! Shelf drags are deliberately ephemeral egui payloads. They do not arm a
//! placement tool or alter the open document until the pointer is released
//! over an editable schematic canvas. Releasing elsewhere or pressing Escape
//! is therefore a true cancellation with no state to roll back.

use egui::{InputState, Key, Modifiers, Popup, Response};

use crate::diagnostics::ConsoleMessage;
use crate::state::{ComponentType, LibraryCellInstance, Point};
use crate::workbench::app_state::AppState;

/// Typed payload shared by component-shelf drag sources and the schematic
/// canvas drop target.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum SchematicShelfDragPayload {
    Primitive(ComponentType),
    LibraryCell(LibraryCellInstance),
}

impl SchematicShelfDragPayload {
    /// Build a directly placeable primitive payload.
    ///
    /// Interface ports are intentionally excluded: their direction,
    /// discipline, name, netlist order, and document authority must first be
    /// committed by the Place pin or port transaction.
    pub(crate) fn primitive(kind: ComponentType) -> Option<Self> {
        (!matches!(kind, ComponentType::Port | ComponentType::CellInstance))
            .then_some(Self::Primitive(kind))
    }

    pub(crate) fn library_cell(binding: LibraryCellInstance) -> Self {
        Self::LibraryCell(binding)
    }

    pub(crate) fn component_type(&self) -> ComponentType {
        match self {
            Self::Primitive(kind) => *kind,
            Self::LibraryCell(_) => ComponentType::CellInstance,
        }
    }

    pub(crate) fn binding(&self) -> Option<&LibraryCellInstance> {
        match self {
            Self::Primitive(_) => None,
            Self::LibraryCell(binding) => Some(binding),
        }
    }

    fn placement_label(&self) -> String {
        match self {
            Self::Primitive(kind) => kind.display_name().to_owned(),
            Self::LibraryCell(binding) => format!("{}/{}", binding.library, binding.cell),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum ShelfDropOutcome {
    Placed,
    ReadOnly,
    RequiresConfiguration,
}

pub(super) fn can_accept_shelf_drop(state: &AppState, payload: &SchematicShelfDragPayload) -> bool {
    !state.schematic_edit_read_only()
        && !matches!(
            payload,
            SchematicShelfDragPayload::Primitive(ComponentType::Port | ComponentType::CellInstance)
        )
}

/// Commit one shelf payload at an already grid-snapped point.
///
/// This is a single undo transaction and does not replace the user's current
/// non-placement tool. If a click-to-arm placement tool was already active,
/// the one-shot shelf drop retires it exactly as the mockup does.
pub(super) fn commit_shelf_drop(
    state: &mut AppState,
    payload: &SchematicShelfDragPayload,
    grid_pos: Point,
) -> ShelfDropOutcome {
    if state.deny_read_only_edit() {
        return ShelfDropOutcome::ReadOnly;
    }
    if matches!(
        payload,
        SchematicShelfDragPayload::Primitive(ComponentType::Port | ComponentType::CellInstance)
    ) {
        return ShelfDropOutcome::RequiresConfiguration;
    }

    let label = payload.placement_label();
    let changed = match payload {
        SchematicShelfDragPayload::Primitive(kind) => {
            state
                .schematic
                .with_undo(format!("drop {}", kind.display_name()), |schematic| {
                    schematic.add_component(*kind, grid_pos);
                })
        }
        SchematicShelfDragPayload::LibraryCell(binding) => {
            let binding = binding.clone();
            state.schematic.with_undo("drop library cell", |schematic| {
                schematic.add_library_cell_component(grid_pos, binding);
            })
        }
    };
    if !changed {
        return ShelfDropOutcome::ReadOnly;
    }

    if state.schematic.tool.is_place_tool() {
        state.schematic.cancel_tool();
        state.schematic.pending_library_cell = None;
    }
    state.push_user_message(ConsoleMessage::info(format!(
        "Placed {label} at ({}, {}) from the component shelf.",
        grid_pos.x, grid_pos.y
    )));
    ShelfDropOutcome::Placed
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PlacementTransform {
    RotateClockwise,
    ToggleHorizontalMirror,
}

fn consume_placement_transform(input: &mut InputState) -> Option<PlacementTransform> {
    if input.consume_key(Modifiers::NONE, Key::R) {
        Some(PlacementTransform::RotateClockwise)
    } else if input.consume_key(Modifiers::NONE, Key::M) {
        Some(PlacementTransform::ToggleHorizontalMirror)
    } else {
        None
    }
}

fn apply_placement_transform(
    schematic: &mut crate::state::SchematicState,
    transform: PlacementTransform,
) {
    match transform {
        PlacementTransform::RotateClockwise => {
            schematic.preview_rotation = schematic.preview_rotation.rotate_cw();
        }
        PlacementTransform::ToggleHorizontalMirror => {
            schematic.preview_mirror_h = !schematic.preview_mirror_h;
        }
    }
}

/// Consume an R/M transform before the application-wide shortcut resolver.
///
/// Shortcut resolution runs before the schematic canvas is rendered. Without
/// this boundary, the global `R` placement shortcut can replace an active
/// ghost with a resistor before the canvas gets a chance to rotate it.
pub(crate) fn handle_pre_render_placement_transform(
    ctx: &egui::Context,
    state: &mut AppState,
    canvas_has_focus: bool,
) -> bool {
    let pointer_over_canvas = super::schematic_canvas_contains_pointer(ctx);
    let click_placement_active =
        state.schematic.tool.is_place_tool() && (canvas_has_focus || pointer_over_canvas);
    let shelf_drag_over_canvas = super::shelf_drag_over_schematic_canvas(ctx);
    if !click_placement_active && !shelf_drag_over_canvas {
        return false;
    }
    let Some(transform) = ctx.input_mut(consume_placement_transform) else {
        return false;
    };
    apply_placement_transform(&mut state.schematic, transform);
    ctx.request_repaint();
    true
}

/// Handle the mockup's direct R/M placement gestures.
///
/// A click-armed placement owns these keys while the canvas is focused or the
/// pointer is over it. An active shelf drag owns them while it is over the
/// canvas, even though the drag source necessarily owns pointer focus.
pub(super) fn handle_placement_transform_keys(
    response: &Response,
    state: &mut AppState,
    shelf_drag_over_canvas: bool,
) -> bool {
    let click_placement_active = state.schematic.tool.is_place_tool()
        && (response.has_focus() || response.contains_pointer());
    if (!click_placement_active && !shelf_drag_over_canvas)
        || state.application_modal_open()
        || Popup::is_any_open(&response.ctx)
    {
        return false;
    }

    let transform = response.ctx.input_mut(consume_placement_transform);
    let Some(transform) = transform else {
        return false;
    };
    apply_placement_transform(&mut state.schematic, transform);
    response.ctx.request_repaint();
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{Rotation, SchematicState, Tool};
    use egui::{Context, Event, Id, RawInput, Rect, Sense, pos2, vec2};

    fn placement_key_input(key: Key, modifiers: Modifiers) -> RawInput {
        RawInput {
            screen_rect: Some(Rect::from_min_size(pos2(0.0, 0.0), vec2(640.0, 480.0))),
            events: vec![Event::Key {
                key,
                physical_key: Some(key),
                pressed: true,
                repeat: false,
                modifiers,
            }],
            ..RawInput::default()
        }
    }

    fn run_transform_frame(
        ctx: &Context,
        input: RawInput,
        state: &mut AppState,
        shelf_drag_over_canvas: bool,
    ) -> (bool, bool) {
        let mut outcome = (false, false);
        let _ = ctx.run_ui(input, |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                let response =
                    ui.interact(ui.max_rect(), Id::new("placement-canvas"), Sense::click());
                response.request_focus();
                let handled =
                    handle_placement_transform_keys(&response, state, shelf_drag_over_canvas);
                let m_still_available =
                    ui.input_mut(|input| input.consume_key(Modifiers::NONE, Key::M));
                outcome = (handled, m_still_available);
            });
        });
        outcome
    }

    #[test]
    fn typed_payload_excludes_ports_that_require_a_configuration_transaction() {
        assert_eq!(
            SchematicShelfDragPayload::primitive(ComponentType::Resistor),
            Some(SchematicShelfDragPayload::Primitive(
                ComponentType::Resistor
            ))
        );
        assert_eq!(
            SchematicShelfDragPayload::primitive(ComponentType::Port),
            None
        );
        assert_eq!(
            SchematicShelfDragPayload::primitive(ComponentType::CellInstance),
            None
        );
    }

    #[test]
    fn primitive_drop_commits_rotation_and_mirror_as_one_undoable_placement() {
        let mut state = AppState::default();
        state.schematic.preview_rotation = Rotation::R90;
        state.schematic.preview_mirror_h = true;
        state.schematic.tool = Tool::Place(ComponentType::Capacitor);
        state.schematic.pending_library_cell =
            Some(LibraryCellInstance::new("work", "stale", "schematic"));
        let payload =
            SchematicShelfDragPayload::primitive(ComponentType::Resistor).expect("placeable");

        assert_eq!(
            commit_shelf_drop(&mut state, &payload, Point::new(30, 40)),
            ShelfDropOutcome::Placed
        );

        assert_eq!(state.schematic.components.len(), 1);
        let placed = &state.schematic.components[0];
        assert_eq!(placed.kind, ComponentType::Resistor);
        assert_eq!(placed.pos, Point::new(30, 40));
        assert_eq!(placed.rotation, Rotation::R90);
        assert!(placed.mirror_h);
        assert_eq!(state.schematic.tool, Tool::Select);
        assert!(state.schematic.pending_library_cell.is_none());
        assert!(state.schematic.can_undo());
        state.schematic.undo();
        assert!(state.schematic.components.is_empty());
    }

    #[test]
    fn library_drop_uses_payload_binding_and_preserves_a_non_placement_tool() {
        let mut state = AppState::default();
        state.schematic.tool = Tool::Wire;
        let binding = LibraryCellInstance::new("work", "ota", "schematic");
        let payload = SchematicShelfDragPayload::library_cell(binding.clone());

        assert_eq!(
            commit_shelf_drop(&mut state, &payload, Point::new(10, 20)),
            ShelfDropOutcome::Placed
        );

        assert_eq!(
            state.schematic.components[0].library_cell.as_ref(),
            Some(&binding)
        );
        assert_eq!(state.schematic.tool, Tool::Wire);
    }

    #[test]
    fn read_only_drop_is_a_no_op_and_preserves_the_active_tool() {
        let mut state = AppState::default();
        state.schematic.read_only = true;
        state.schematic.tool = Tool::Place(ComponentType::Resistor);
        let payload =
            SchematicShelfDragPayload::primitive(ComponentType::Capacitor).expect("placeable");

        assert_eq!(
            commit_shelf_drop(&mut state, &payload, Point::origin()),
            ShelfDropOutcome::ReadOnly
        );
        assert!(state.schematic.components.is_empty());
        assert_eq!(state.schematic.tool, Tool::Place(ComponentType::Resistor));
        assert!(!state.schematic.can_undo());
    }

    #[test]
    fn placement_transforms_update_only_ephemeral_orientation_state() {
        let mut schematic = SchematicState::default();
        apply_placement_transform(&mut schematic, PlacementTransform::RotateClockwise);
        apply_placement_transform(&mut schematic, PlacementTransform::ToggleHorizontalMirror);

        assert_eq!(schematic.preview_rotation, Rotation::R90);
        assert!(schematic.preview_mirror_h);
        assert!(schematic.components.is_empty());
        assert!(!schematic.is_dirty);
        assert!(!schematic.can_undo());
    }

    #[test]
    fn focused_click_placement_consumes_direct_r_and_m_canvas_gestures() {
        let ctx = Context::default();
        let mut state = AppState::default();
        state.schematic.tool = Tool::Place(ComponentType::Resistor);

        let (handled, _) = run_transform_frame(
            &ctx,
            placement_key_input(Key::R, Modifiers::NONE),
            &mut state,
            false,
        );
        assert!(handled);
        assert_eq!(state.schematic.preview_rotation, Rotation::R90);

        let (handled, m_still_available) = run_transform_frame(
            &ctx,
            placement_key_input(Key::M, Modifiers::NONE),
            &mut state,
            false,
        );
        assert!(handled);
        assert!(!m_still_available);
        assert!(state.schematic.preview_mirror_h);
    }

    #[test]
    fn shelf_drag_owns_transform_keys_without_canvas_pointer_focus() {
        let ctx = Context::default();
        let mut state = AppState::default();
        state.schematic.tool = Tool::Select;

        let (handled, _) = run_transform_frame(
            &ctx,
            placement_key_input(Key::M, Modifiers::NONE),
            &mut state,
            true,
        );

        assert!(handled);
        assert!(state.schematic.preview_mirror_h);
        assert_eq!(state.schematic.tool, Tool::Select);
    }

    #[test]
    fn pre_render_transform_wins_before_the_global_r_shortcut() {
        let ctx = Context::default();
        let mut state = AppState::default();
        state.schematic.tool = Tool::Place(ComponentType::Capacitor);

        let mut handled = false;
        let _ = ctx.run_ui(placement_key_input(Key::R, Modifiers::NONE), |ctx| {
            handled = handle_pre_render_placement_transform(ctx, &mut state, true);
        });

        assert!(handled);
        assert_eq!(state.schematic.tool, Tool::Place(ComponentType::Capacitor));
        assert_eq!(state.schematic.preview_rotation, Rotation::R90);
        assert!(!ctx.input_mut(|input| input.consume_key(Modifiers::NONE, Key::R)));
    }
}
