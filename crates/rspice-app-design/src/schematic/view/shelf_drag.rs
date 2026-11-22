//! Component-shelf drag-and-drop contract for the schematic canvas.
//!
//! Shelf drags are deliberately ephemeral egui payloads. They do not arm a
//! placement tool or alter the open document until the pointer is released
//! over an editable schematic canvas. Releasing elsewhere or pressing Escape
//! is therefore a true cancellation with no state to roll back.

use egui::{InputState, Key, Modifiers, Popup, Response};

use super::interaction::connect_pins_inside_wires;
use super::parts::{CanvasFacts, CanvasParts};
use crate::state::{
    ComponentType, LibraryCellInstance, PendingStimulusPlacement, Point, functional_part,
};

use super::selection_transform::SelectionTransform;

/// Typed payload shared by component-shelf drag sources and the schematic
/// canvas drop target.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SchematicShelfDragPayload {
    Primitive(ComponentType),
    /// Boxed: a bound cell instance dwarfs a primitive's component type, and
    /// this payload rides in drag state alongside the primitive case.
    LibraryCell(Box<LibraryCellInstance>),
    /// A stimulus definition dragged off the Component shelf. Boxed for the
    /// same reason the cell binding is: it carries the copied card.
    Stimulus(Box<PendingStimulusPlacement>),
}

impl SchematicShelfDragPayload {
    /// Build a directly placeable primitive payload.
    ///
    /// Interface ports are intentionally excluded: a pin has a name, and only
    /// Create pins can supply one before the drop lands.
    pub fn primitive(kind: ComponentType) -> Option<Self> {
        (!matches!(kind, ComponentType::Port | ComponentType::CellInstance))
            .then_some(Self::Primitive(kind))
    }

    pub(crate) fn library_cell(binding: LibraryCellInstance) -> Self {
        Self::LibraryCell(Box::new(binding))
    }

    /// Carry one armed stimulus definition to the canvas.
    pub(crate) fn stimulus(placement: PendingStimulusPlacement) -> Self {
        Self::Stimulus(Box::new(placement))
    }

    pub(crate) fn component_type(&self) -> ComponentType {
        match self {
            Self::Primitive(kind) => *kind,
            Self::LibraryCell(_) => ComponentType::CellInstance,
            Self::Stimulus(placement) => placement.component_type,
        }
    }

    pub(crate) fn binding(&self) -> Option<&LibraryCellInstance> {
        match self {
            Self::Primitive(_) | Self::Stimulus(_) => None,
            Self::LibraryCell(binding) => Some(binding),
        }
    }

    fn placement_label(&self) -> String {
        match self {
            Self::Primitive(kind) => kind.display_name().to_owned(),
            Self::LibraryCell(binding) => binding
                .builtin_functional
                .as_ref()
                .and_then(|contract| functional_part(&contract.stable_id))
                .map_or_else(
                    || format!("{}/{}", binding.library, binding.cell),
                    |part| part.display_name.to_owned(),
                ),
            Self::Stimulus(placement) => {
                format!("{} r{}", placement.definition(), placement.revision())
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum ShelfDropOutcome {
    Placed,
    ReadOnly,
    RequiresConfiguration,
}

pub(super) fn can_accept_shelf_drop(
    state: &CanvasFacts<'_>,
    payload: &SchematicShelfDragPayload,
) -> bool {
    !state.model.schematic.session.read_only
        && !state.active_view_read_only()
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
    state: &mut CanvasParts<'_>,
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
    // A drop is a placement: a pin that lands inside a wire is connected
    // there exactly as a click with the armed tool connects it, so a part
    // dropped across a wire sits in series instead of shorting both pins.
    let symbols = state.model.schematic_symbol_context();
    let workspace = &state.model.workspace;
    let changed =
        match payload {
            SchematicShelfDragPayload::Primitive(kind) => state.model.schematic.with_undo(
                format!("drop {}", kind.display_name()),
                |schematic| {
                    let id = schematic.add_component(*kind, grid_pos);
                    connect_pins_inside_wires(workspace, schematic, id, &symbols);
                },
            ),
            SchematicShelfDragPayload::LibraryCell(binding) => {
                let binding = (**binding).clone();
                state
                    .model
                    .schematic
                    .with_undo("drop library cell", |schematic| {
                        let id = schematic.add_library_cell_component(grid_pos, binding);
                        connect_pins_inside_wires(workspace, schematic, id, &symbols);
                    })
            }
            SchematicShelfDragPayload::Stimulus(placement) => {
                let placement = (**placement).clone();
                state.model.schematic.with_undo(
                    format!("drop {}", placement.definition()),
                    |schematic| {
                        let id = schematic.add_stimulus_component(&placement, grid_pos);
                        connect_pins_inside_wires(workspace, schematic, id, &symbols);
                    },
                )
            }
        };
    if !changed {
        return ShelfDropOutcome::ReadOnly;
    }

    if state.model.schematic.session.editor.tool.is_place_tool() {
        state.model.schematic.cancel_tool();
        state.model.schematic.session.editor.pending_library_cell = None;
    }
    state.prompt(format!(
        "Placed {label} at ({}, {}).",
        grid_pos.x, grid_pos.y
    ));
    ShelfDropOutcome::Placed
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum PlacementTransform {
    RotateClockwise,
    RotateCounterClockwise,
    ToggleHorizontalMirror,
    /// Top to bottom. A ghost has only a left-to-right mirror, and a
    /// half turn with it is the same flip.
    ToggleVerticalMirror,
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
            schematic.session.editor.preview_rotation =
                schematic.session.editor.preview_rotation.rotate_cw();
        }
        PlacementTransform::RotateCounterClockwise => {
            schematic.session.editor.preview_rotation =
                schematic.session.editor.preview_rotation.rotate_ccw();
        }
        PlacementTransform::ToggleHorizontalMirror => {
            schematic.session.editor.preview_mirror_h = !schematic.session.editor.preview_mirror_h;
        }
        PlacementTransform::ToggleVerticalMirror => {
            let editor = &mut schematic.session.editor;
            editor.preview_rotation = editor.preview_rotation.rotate_cw().rotate_cw();
            editor.preview_mirror_h = !editor.preview_mirror_h;
        }
    }
}

/// Whether something floats under the pointer for R and M to turn: a
/// floating paste or duplicate, a selection Move is carrying, or the
/// placement ghost.
pub(super) fn something_floats(state: &CanvasFacts<'_>) -> bool {
    state.design.floating_paste.is_some()
        || state.design.move_selection.armed
        || state.model.schematic.session.editor.tool.is_place_tool()
}

/// Turn what floats under the pointer. R and M take this path, and so do
/// Rotate and Mirror on the touch strip and the canvas menu while something
/// floats, since a tablet has no R or M key. Returns whether anything did.
pub(super) fn turn_what_floats(state: &mut CanvasParts<'_>, transform: PlacementTransform) -> bool {
    if state.design.floating_paste.is_some() {
        crate::workbench::app::turn_floating_paste(
            &mut state.design.floating_paste,
            selection_transform(transform),
        )
    } else if state.design.move_selection.armed {
        turn_the_move(state, transform);
        true
    } else if state.model.schematic.session.editor.tool.is_place_tool() {
        apply_placement_transform(&mut state.model.schematic, transform);
        true
    } else {
        false
    }
}

const fn selection_transform(transform: PlacementTransform) -> SelectionTransform {
    match transform {
        PlacementTransform::RotateClockwise => SelectionTransform::RotateClockwise,
        PlacementTransform::RotateCounterClockwise => SelectionTransform::RotateCounterClockwise,
        PlacementTransform::ToggleHorizontalMirror => SelectionTransform::MirrorHorizontal,
        PlacementTransform::ToggleVerticalMirror => SelectionTransform::MirrorVertical,
    }
}

/// Consume an R/M transform before the application-wide shortcut resolver.
///
/// Shortcut resolution runs before the schematic canvas is rendered. Without
/// this boundary, the global `R` placement shortcut can replace an active
/// ghost with a resistor before the canvas gets a chance to rotate it.
pub fn handle_pre_render_placement_transform(
    ctx: &egui::Context,
    state: &mut CanvasParts<'_>,
    canvas_has_focus: bool,
) -> bool {
    if crate::schematic::symbol_editor::interaction::turn_pin_being_placed(
        ctx,
        state,
        canvas_has_focus,
    ) {
        return true;
    }
    let pointer_over_canvas = rspice_schematic_editor::view::canvas::contains_pointer(ctx);
    // A floating paste, or the selection Move carries, turns about its own
    // middle and keeps floating. Neither is on the sheet until it lands, and
    // a turn of what Move carries lands with the move.
    let carried = state.design.floating_paste.is_some() || state.design.move_selection.armed;
    if carried && (canvas_has_focus || pointer_over_canvas) {
        let Some(transform) = ctx.input_mut(consume_placement_transform) else {
            return false;
        };
        turn_what_floats(state, transform);
        ctx.request_repaint();
        return true;
    }
    let click_placement_active = state.model.schematic.session.editor.tool.is_place_tool()
        && (canvas_has_focus || pointer_over_canvas);
    let shelf_drag_over_canvas = super::shelf_drag_over_schematic_canvas(ctx);
    if !click_placement_active && !shelf_drag_over_canvas {
        return false;
    }
    let Some(transform) = ctx.input_mut(consume_placement_transform) else {
        return false;
    };
    apply_placement_transform(&mut state.model.schematic, transform);
    ctx.request_repaint();
    true
}

/// While Move carries the selection, a turn floats with it about its own
/// middle and the move carries on from where the pointer is. The sheet is
/// not touched: the turn lands with the move as one undo step, and Esc
/// drops it with the move.
fn turn_the_move(state: &mut CanvasParts<'_>, transform: PlacementTransform) {
    state
        .design
        .move_selection
        .turns
        .push(selection_transform(transform));
}

/// Handle the mockup's direct R/M placement gestures.
///
/// A click-armed placement owns these keys while the canvas is focused or the
/// pointer is over it. An active shelf drag owns them while it is over the
/// canvas, even though the drag source necessarily owns pointer focus.
pub(super) fn handle_placement_transform_keys(
    response: &Response,
    state: &mut CanvasParts<'_>,
    shelf_drag_over_canvas: bool,
) -> bool {
    let click_placement_active = state.model.schematic.session.editor.tool.is_place_tool()
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
    apply_placement_transform(&mut state.model.schematic, transform);
    response.ctx.request_repaint();
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{Component, Rotation, SchematicState, Tool, Wire};
    use crate::workbench::design::test_host::{DesignTestHost, canvas_parts};
    use egui::{Context, Event, Id, RawInput, Rect, Sense, pos2, vec2};
    use rspice_design::connectivity::extract_with_hierarchy;

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
        state: &mut DesignTestHost,
        shelf_drag_over_canvas: bool,
    ) -> (bool, bool) {
        let mut outcome = (false, false);
        let _ = ctx.run_ui(input, |root| {
            egui::CentralPanel::default().show(root, |ui| {
                // accessibility-pointer-shim: test-only placement canvas harness.
                let response =
                    ui.interact(ui.max_rect(), Id::new("placement-canvas"), Sense::click());
                response.request_focus();
                let handled = handle_placement_transform_keys(
                    &response,
                    &mut canvas_parts(state),
                    shelf_drag_over_canvas,
                );
                let m_still_available = ui
                    .ctx()
                    .input_mut(|input| input.consume_key(Modifiers::NONE, Key::M));
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
        let mut state = DesignTestHost::default();
        state.model.schematic.session.editor.preview_rotation = Rotation::R90;
        state.model.schematic.session.editor.preview_mirror_h = true;
        state.model.schematic.session.editor.tool = Tool::Place(ComponentType::Capacitor);
        state.model.schematic.session.editor.pending_library_cell = Some(
            rspice_schematic_editor::session::placement::PendingLibraryCellPlacement::new(
                LibraryCellInstance::new("work", "stale", "schematic"),
                crate::workbench::app::schematic_editor_request_source(&state),
            ),
        );
        let payload =
            SchematicShelfDragPayload::primitive(ComponentType::Resistor).expect("placeable");

        assert_eq!(
            commit_shelf_drop(&mut canvas_parts(&mut state), &payload, Point::new(30, 40)),
            ShelfDropOutcome::Placed
        );

        assert_eq!(state.model.schematic.document().components.len(), 1);
        let placed = &state.model.schematic.document().components[0];
        assert_eq!(placed.kind, ComponentType::Resistor);
        assert_eq!(placed.pos, Point::new(30, 40));
        assert_eq!(placed.rotation, Rotation::R90);
        assert!(placed.mirror_h);
        assert_eq!(state.model.schematic.session.editor.tool, Tool::Select);
        assert!(
            state
                .model
                .schematic
                .session
                .editor
                .pending_library_cell
                .is_none()
        );
        assert!(state.model.schematic.can_undo());
        state.model.schematic.undo();
        assert!(state.model.schematic.document().components.is_empty());
    }

    /// A resistor dropped across a wire is inserted into it: the wire splits
    /// at each pin, the piece under the body goes, and the two pins sit on
    /// two nets, as they do when the armed tool places the part by click.
    #[test]
    fn a_resistor_dropped_across_a_wire_sits_in_series_with_it() {
        let mut state = DesignTestHost::default();
        let pins = Component::new(0, ComponentType::Resistor, Point::origin()).terminal_positions();
        let (first, second) = (pins[0].1, pins[1].1);
        // A wire along the line through both pins, running on past each.
        let (start, end) = if first.y == second.y {
            (
                Point::new(first.x.min(second.x) - 100, first.y),
                Point::new(first.x.max(second.x) + 100, first.y),
            )
        } else {
            assert_eq!(first.x, second.x, "the pins share a row or a column");
            (
                Point::new(first.x, first.y.min(second.y) - 100),
                Point::new(first.x, first.y.max(second.y) + 100),
            )
        };
        state
            .model
            .schematic
            .document_mut_for_test()
            .wires
            .push(Wire::segment(1, start, end));
        state.model.schematic.init_undo_history();
        let payload =
            SchematicShelfDragPayload::primitive(ComponentType::Resistor).expect("placeable");

        assert_eq!(
            commit_shelf_drop(&mut canvas_parts(&mut state), &payload, Point::origin()),
            ShelfDropOutcome::Placed
        );

        let resistor = state.model.schematic.document().components[0].id;
        let connectivity = extract_with_hierarchy(&state.model.schematic, None);
        let nets: Vec<&str> = connectivity
            .terminals
            .iter()
            .filter(|terminal| terminal.component_id == resistor)
            .map(|terminal| {
                assert!(terminal.attached, "pin {} meets the wire", terminal.pin);
                terminal.net_name.as_str()
            })
            .collect();
        assert_eq!(nets.len(), 2);
        assert_ne!(nets[0], nets[1], "the resistor is not shorted by the wire");
        assert!(
            state
                .model
                .schematic
                .document()
                .wires
                .iter()
                .all(|wire| !wire.points.contains(&first) || !wire.points.contains(&second)),
            "no wire runs on under the body"
        );

        assert!(state.model.schematic.undo());
        assert!(state.model.schematic.document().components.is_empty());
        assert_eq!(
            state.model.schematic.document().wires.len(),
            1,
            "the drop and the split are one undo step"
        );
    }

    #[test]
    fn library_drop_uses_payload_binding_and_preserves_a_non_placement_tool() {
        let mut state = DesignTestHost::default();
        state.model.schematic.session.editor.tool = Tool::Wire;
        let binding = LibraryCellInstance::new("work", "ota", "schematic");
        let payload = SchematicShelfDragPayload::library_cell(binding.clone());

        assert_eq!(
            commit_shelf_drop(&mut canvas_parts(&mut state), &payload, Point::new(10, 20)),
            ShelfDropOutcome::Placed
        );

        assert_eq!(
            state.model.schematic.document().components[0]
                .library_cell
                .as_ref(),
            Some(&binding)
        );
        assert_eq!(state.model.schematic.session.editor.tool, Tool::Wire);
    }

    #[test]
    fn read_only_drop_is_a_no_op_and_preserves_the_active_tool() {
        let mut state = DesignTestHost::default();
        state.model.schematic.session.read_only = true;
        state.model.schematic.session.editor.tool = Tool::Place(ComponentType::Resistor);
        let payload =
            SchematicShelfDragPayload::primitive(ComponentType::Capacitor).expect("placeable");

        assert_eq!(
            commit_shelf_drop(&mut canvas_parts(&mut state), &payload, Point::origin()),
            ShelfDropOutcome::ReadOnly
        );
        assert!(state.model.schematic.document().components.is_empty());
        assert_eq!(
            state.model.schematic.session.editor.tool,
            Tool::Place(ComponentType::Resistor)
        );
        assert!(!state.model.schematic.can_undo());
    }

    #[test]
    fn placement_transforms_update_only_ephemeral_orientation_state() {
        let mut schematic = SchematicState::default();
        apply_placement_transform(&mut schematic, PlacementTransform::RotateClockwise);
        apply_placement_transform(&mut schematic, PlacementTransform::ToggleHorizontalMirror);

        assert_eq!(schematic.session.editor.preview_rotation, Rotation::R90);
        assert!(schematic.session.editor.preview_mirror_h);
        assert!(schematic.document().components.is_empty());
        assert!(!schematic.session.is_dirty);
        assert!(!schematic.can_undo());
    }

    #[test]
    fn focused_click_placement_consumes_direct_r_and_m_canvas_gestures() {
        let ctx = Context::default();
        let mut state = DesignTestHost::default();
        state.model.schematic.session.editor.tool = Tool::Place(ComponentType::Resistor);

        let (handled, _) = run_transform_frame(
            &ctx,
            placement_key_input(Key::R, Modifiers::NONE),
            &mut state,
            false,
        );
        assert!(handled);
        assert_eq!(
            state.model.schematic.session.editor.preview_rotation,
            Rotation::R90
        );

        let (handled, m_still_available) = run_transform_frame(
            &ctx,
            placement_key_input(Key::M, Modifiers::NONE),
            &mut state,
            false,
        );
        assert!(handled);
        assert!(!m_still_available);
        assert!(state.model.schematic.session.editor.preview_mirror_h);
    }

    #[test]
    fn shelf_drag_owns_transform_keys_without_canvas_pointer_focus() {
        let ctx = Context::default();
        let mut state = DesignTestHost::default();
        state.model.schematic.session.editor.tool = Tool::Select;

        let (handled, _) = run_transform_frame(
            &ctx,
            placement_key_input(Key::M, Modifiers::NONE),
            &mut state,
            true,
        );

        assert!(handled);
        assert!(state.model.schematic.session.editor.preview_mirror_h);
        assert_eq!(state.model.schematic.session.editor.tool, Tool::Select);
    }

    #[test]
    fn pre_render_transform_wins_before_the_global_r_shortcut() {
        let ctx = Context::default();
        let mut state = DesignTestHost::default();
        state.model.schematic.session.editor.tool = Tool::Place(ComponentType::Capacitor);

        let mut handled = false;
        let _ = ctx.run_ui(placement_key_input(Key::R, Modifiers::NONE), |ctx| {
            handled =
                handle_pre_render_placement_transform(ctx, &mut canvas_parts(&mut state), true);
        });

        assert!(handled);
        assert_eq!(
            state.model.schematic.session.editor.tool,
            Tool::Place(ComponentType::Capacitor)
        );
        assert_eq!(
            state.model.schematic.session.editor.preview_rotation,
            Rotation::R90
        );
        assert!(!ctx.input_mut(|input| input.consume_key(Modifiers::NONE, Key::R)));
    }
}
