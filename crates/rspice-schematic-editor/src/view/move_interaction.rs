//! Transactional move gestures over a frozen selection and borrowed design.

use super::{
    coordinates::screen_to_schematic,
    design_view::DesignView,
    pointer_target::{PointerHit, PointerQuery, PointerTarget, pointer_target},
    snap_resolution::resolve_grid_pointer,
    symbol_context::SchematicSymbolContext,
    transform_input::{
        TransformInputTransition, consume_keyboard, retain_canvas_focus_from_pointer,
    },
    viewport::Viewport,
};
use crate::{
    requests::EditorRequestSource,
    session::{
        selection::SchematicSelectionFilter, snap::SnapEngine, transform::TransformCanvasSession,
    },
};
use egui::{Response, Ui};
use rspice_design::schematic::{movement::MoveSelectionMode, selection::Selection};
use rspice_design_model::Point;

#[derive(Debug, Clone)]
pub struct MoveInputRequest {
    pub source: EditorRequestSource,
    pub selection: Selection,
    pub generation: u64,
    pub mode: MoveSelectionMode,
    pub transition: TransformInputTransition,
}

pub struct MoveInputView<'a> {
    pub design: DesignView<'a>,
    pub canvas: &'a TransformCanvasSession,
    pub selection: &'a Selection,
    pub filter: SchematicSelectionFilter,
    pub snap_engine: &'a SnapEngine,
}

pub fn input(
    ui: &Ui,
    response: &Response,
    viewport: &Viewport,
    symbol_context: &SchematicSymbolContext,
    view: MoveInputView<'_>,
    view_path: impl Fn() -> String,
) -> Option<TransformInputTransition> {
    let mut next = view.canvas.clone();
    let commit = apply_input(
        ui,
        response,
        viewport,
        symbol_context,
        &view,
        &mut next,
        &view_path,
    );
    (commit || next != *view.canvas).then(|| TransformInputTransition {
        expected: view.canvas.clone(),
        next,
        commit,
    })
}

fn apply_input(
    ui: &Ui,
    response: &Response,
    viewport: &Viewport,
    symbol_context: &SchematicSymbolContext,
    view: &MoveInputView<'_>,
    draft: &mut TransformCanvasSession,
    view_path: &impl Fn() -> String,
) -> bool {
    retain_canvas_focus_from_pointer(response);
    let (keyboard_step, keyboard_commit) =
        consume_keyboard(ui, response.has_focus(), view.design.document.grid_size);
    if keyboard_step != Point::origin() {
        draft.anchor = None;
        draft.pointer_drag = false;
        draft.preview_error = None;
        draft.preview_delta = Point::new(
            draft.preview_delta.x.saturating_add(keyboard_step.x),
            draft.preview_delta.y.saturating_add(keyboard_step.y),
        );
    }
    if keyboard_commit {
        return true;
    }

    if response.drag_started_by(egui::PointerButton::Primary)
        && let Some(position) = ui
            .input(|input| input.pointer.press_origin())
            .or_else(|| response.interact_pointer_pos())
    {
        let anchor = resolve_grid_pointer(
            view.snap_engine,
            view.design.document.grid_size,
            viewport,
            position,
        )
        .snapped_position;
        if pointer_is_in_frozen_move_selection(
            view,
            ui.ctx(),
            viewport,
            symbol_context,
            position,
            anchor,
            view_path,
        ) {
            draft.anchor = Some(anchor);
            draft.preview_delta = Point::origin();
            draft.pointer_drag = true;
            draft.preview_error = None;
        }
    }

    if response.dragged_by(egui::PointerButton::Primary)
        && draft.pointer_drag
        && let (Some(anchor), Some(position)) = (
            draft.anchor,
            response
                .hover_pos()
                .or_else(|| response.interact_pointer_pos()),
        )
    {
        let destination = resolve_grid_pointer(
            view.snap_engine,
            view.design.document.grid_size,
            viewport,
            position,
        )
        .snapped_position;
        draft.preview_delta = Point::new(
            destination.x.saturating_sub(anchor.x),
            destination.y.saturating_sub(anchor.y),
        );
    }

    if response.drag_stopped_by(egui::PointerButton::Primary) && draft.pointer_drag {
        draft.pointer_drag = false;
        return true;
    }

    if response.clicked_by(egui::PointerButton::Primary)
        && let Some(position) = response.interact_pointer_pos()
    {
        let point = resolve_grid_pointer(
            view.snap_engine,
            view.design.document.grid_size,
            viewport,
            position,
        )
        .snapped_position;
        if let Some(anchor) = draft.anchor {
            draft.preview_delta = Point::new(
                point.x.saturating_sub(anchor.x),
                point.y.saturating_sub(anchor.y),
            );
            return true;
        } else if pointer_is_in_frozen_move_selection(
            view,
            ui.ctx(),
            viewport,
            symbol_context,
            position,
            point,
            view_path,
        ) {
            draft.anchor = Some(point);
            draft.preview_delta = Point::origin();
            draft.preview_error = None;
        }
    } else if !draft.pointer_drag
        && let (Some(anchor), Some(position)) = (draft.anchor, response.hover_pos())
    {
        let destination = resolve_grid_pointer(
            view.snap_engine,
            view.design.document.grid_size,
            viewport,
            position,
        )
        .snapped_position;
        draft.preview_delta = Point::new(
            destination.x.saturating_sub(anchor.x),
            destination.y.saturating_sub(anchor.y),
        );
    }
    false
}

fn pointer_is_in_frozen_move_selection(
    view: &MoveInputView<'_>,
    context: &egui::Context,
    viewport: &Viewport,
    symbol_context: &SchematicSymbolContext,
    position: egui::Pos2,
    grid_position: Point,
    view_path: impl FnOnce() -> String,
) -> bool {
    let hit_position = screen_to_schematic(viewport, position);
    let hit_radius = (6.0 / viewport.zoom.max(0.1)).ceil() as i32;
    let Some(target) = pointer_target(
        &view.design,
        PointerQuery {
            hit: PointerHit::new(grid_position, hit_position),
            radius: hit_radius,
            position,
        },
        view.filter,
        symbol_context,
        context,
        viewport,
        view_path,
    ) else {
        return false;
    };
    let selection = view.selection;
    match target {
        PointerTarget::Component(id) => selection.has_component(id),
        PointerTarget::Wire(id) => selection.has_wire(id),
        PointerTarget::Bus(id) => selection.has_bus(id),
        PointerTarget::BusTap(id) => selection.has_bus_tap(id),
        PointerTarget::NetLabel(id) => selection.has_net_label(id),
        PointerTarget::DesignNote(id) => selection.has_design_note(id),
        PointerTarget::DocumentationShape(id) => selection.has_documentation_shape(id),
        PointerTarget::Probe(id) => selection.has_probe(id),
        PointerTarget::Junction(_) => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rspice_design::schematic::{document::SchematicDocument, wire::Wire};

    #[test]
    fn real_pointer_move_requires_the_frozen_selection_and_retains_drag_or_two_click_pitch() {
        for drag in [false, true] {
            let ctx = egui::Context::default();
            let document = SchematicDocument {
                wires: vec![
                    Wire::segment(1, Point::new(40, 50), Point::new(100, 50)),
                    Wire::segment(2, Point::new(40, 100), Point::new(100, 100)),
                ],
                ..Default::default()
            };
            let mut selection = Selection::default();
            selection.select_wire(1);
            let mut canvas = TransformCanvasSession::default();
            let snap = SnapEngine::default();
            let symbols = SchematicSymbolContext::default();
            let mut frame = |events: Vec<egui::Event>| {
                let mut commit = false;
                let _ = ctx.run_ui(
                    egui::RawInput {
                        screen_rect: Some(egui::Rect::from_min_size(
                            egui::Pos2::ZERO,
                            egui::vec2(400.0, 300.0),
                        )),
                        events,
                        ..Default::default()
                    },
                    |root| {
                        egui::CentralPanel::default().show(root, |ui| {
                            let bounds = ui.max_rect();
                            let viewport = Viewport {
                                offset: egui::pos2(-bounds.min.x, -bounds.min.y),
                                zoom: 1.0,
                                bounds,
                            };
                            let response = ui.interact(
                                bounds,
                                egui::Id::new("move"),
                                egui::Sense::click_and_drag(),
                            );
                            if let Some(transition) = input(
                                ui,
                                &response,
                                &viewport,
                                &symbols,
                                MoveInputView {
                                    design: DesignView {
                                        document: &document,
                                        canvas_cache: None,
                                        sheet_catalog: None,
                                        review_markers: Default::default(),
                                    },
                                    canvas: &canvas,
                                    selection: &selection,
                                    filter: Default::default(),
                                    snap_engine: &snap,
                                },
                                || "user/top/schematic".to_owned(),
                            ) {
                                assert_eq!(transition.expected, canvas);
                                commit = transition.commit;
                                canvas = transition.next;
                            }
                        });
                    },
                );
                (commit, canvas.clone())
            };
            let button = |pos, pressed| {
                vec![
                    egui::Event::PointerMoved(pos),
                    egui::Event::PointerButton {
                        pos,
                        button: egui::PointerButton::Primary,
                        pressed,
                        modifiers: egui::Modifiers::NONE,
                    },
                ]
            };
            frame(Vec::new());
            frame(button(egui::pos2(70.0, 100.0), true));
            let (commit, draft) = frame(button(egui::pos2(70.0, 100.0), false));
            assert!(!commit);
            assert_eq!(draft.anchor, None, "unselected wire cannot start move");
            frame(button(egui::pos2(70.0, 50.0), true));
            if drag {
                let (commit, draft) =
                    frame(vec![egui::Event::PointerMoved(egui::pos2(120.0, 80.0))]);
                assert!(!commit);
                assert!(draft.pointer_drag);
            } else {
                let (commit, draft) = frame(button(egui::pos2(70.0, 50.0), false));
                assert!(!commit);
                assert_eq!(draft.anchor, Some(Point::new(70, 50)));
                frame(button(egui::pos2(120.0, 80.0), true));
            }
            let (commit, draft) = frame(button(egui::pos2(120.0, 80.0), false));
            assert!(commit);
            assert_eq!(draft.preview_delta, Point::new(50, 30));
            assert!(!draft.pointer_drag);
        }
    }
}
