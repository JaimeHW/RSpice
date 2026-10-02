//! Array gesture interpretation over a borrowed canvas draft and snap policy.

use super::{snap_resolution::resolve_grid_pointer, viewport::Viewport};
use crate::{
    requests::EditorRequestSource,
    session::{array::ArrayCanvasSession, snap::SnapEngine},
};
use egui::{Response, Ui};
use rspice_design::schematic::{
    array::{SchematicArrayKind, SchematicArrayPlacement},
    selection::Selection,
};
use rspice_design_model::Point;

const DELTA_OVERFLOW: &str =
    "The requested array placement exceeds the schematic coordinate range.";

#[derive(Debug, Clone)]
pub struct ArrayInputTransition {
    pub expected: ArrayCanvasSession,
    pub next: ArrayCanvasSession,
    pub commit: bool,
}

/// The app rechecks the operation, its source and its draft before applying input.
#[derive(Debug, Clone)]
pub struct ArrayInputRequest {
    pub source: EditorRequestSource,
    pub selection: Selection,
    pub generation: u64,
    pub kind: SchematicArrayKind,
    pub count: String,
    pub naming: String,
    pub transition: ArrayInputTransition,
}

pub struct ArrayInputView<'a> {
    pub canvas: &'a ArrayCanvasSession,
    pub kind: SchematicArrayKind,
    pub grid_size: i32,
    pub snap_engine: &'a SnapEngine,
}

pub fn input(
    ui: &Ui,
    response: &Response,
    viewport: &Viewport,
    view: ArrayInputView<'_>,
) -> Option<ArrayInputTransition> {
    let mut next = view.canvas.clone();
    let commit = apply_input(
        ui,
        response,
        viewport,
        &mut next,
        view.kind,
        view.grid_size,
        view.snap_engine,
    );
    (commit || next != *view.canvas).then(|| ArrayInputTransition {
        expected: view.canvas.clone(),
        next,
        commit,
    })
}

fn apply_input(
    ui: &Ui,
    response: &Response,
    viewport: &Viewport,
    draft: &mut ArrayCanvasSession,
    kind: SchematicArrayKind,
    grid_size: i32,
    snap_engine: &SnapEngine,
) -> bool {
    retain_canvas_focus_from_pointer(response);
    let (keyboard_step, keyboard_commit) = consume_keyboard(ui, response.has_focus(), grid_size);
    if keyboard_step != Point::origin() {
        let requested = checked_accumulate_delta(draft.preview_delta, keyboard_step);
        draft.pointer_drag = false;
        if kind == SchematicArrayKind::RadialDocumentation && draft.anchor.is_none() {
            draft.anchor = Some(Point::origin());
        }
        update_preview_delta(draft, requested);
    }
    if keyboard_commit {
        return true;
    }

    if response.drag_started_by(egui::PointerButton::Primary)
        && let Some(position) = ui
            .input(|input| input.pointer.press_origin())
            .or_else(|| response.interact_pointer_pos())
    {
        let point =
            resolve_grid_pointer(snap_engine, grid_size, viewport, position).snapped_position;
        draft.anchor = Some(point);
        draft.preview_delta = Point::origin();
        draft.pointer_drag = true;
        draft.preview_error = None;
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
        let destination =
            resolve_grid_pointer(snap_engine, grid_size, viewport, position).snapped_position;
        if kind == SchematicArrayKind::RadialDocumentation {
            draft.anchor = Some(destination);
            update_preview_delta(draft, Ok(Point::origin()));
        } else {
            update_preview_delta(draft, checked_pointer_delta(anchor, destination));
        }
    }

    if response.drag_stopped_by(egui::PointerButton::Primary) && draft.pointer_drag {
        draft.pointer_drag = false;
        return true;
    }

    if response.clicked_by(egui::PointerButton::Primary)
        && let Some(position) = response.interact_pointer_pos()
    {
        let point =
            resolve_grid_pointer(snap_engine, grid_size, viewport, position).snapped_position;
        if kind == SchematicArrayKind::RadialDocumentation {
            draft.anchor = Some(point);
            draft.preview_delta = Point::origin();
            update_preview_delta(draft, Ok(Point::origin()));
            return true;
        } else if let Some(anchor) = draft.anchor {
            update_preview_delta(draft, checked_pointer_delta(anchor, point));
            return true;
        } else {
            draft.anchor = Some(point);
            draft.preview_delta = Point::origin();
            draft.preview_error = None;
        }
    } else if !draft.pointer_drag
        && let Some(position) = response.hover_pos()
    {
        let point =
            resolve_grid_pointer(snap_engine, grid_size, viewport, position).snapped_position;
        if kind == SchematicArrayKind::RadialDocumentation {
            draft.anchor = Some(point);
            update_preview_delta(draft, Ok(Point::origin()));
        } else if let Some(anchor) = draft.anchor {
            update_preview_delta(draft, checked_pointer_delta(anchor, point));
        }
    }
    false
}

fn checked_accumulate_delta(current: Point, step: Point) -> Result<Point, &'static str> {
    let Some(x) = current.x.checked_add(step.x) else {
        return Err(DELTA_OVERFLOW);
    };
    let Some(y) = current.y.checked_add(step.y) else {
        return Err(DELTA_OVERFLOW);
    };
    Ok(Point::new(x, y))
}

fn checked_pointer_delta(anchor: Point, destination: Point) -> Result<Point, &'static str> {
    let Some(x) = destination.x.checked_sub(anchor.x) else {
        return Err(DELTA_OVERFLOW);
    };
    let Some(y) = destination.y.checked_sub(anchor.y) else {
        return Err(DELTA_OVERFLOW);
    };
    Ok(Point::new(x, y))
}

pub fn array_placement(
    kind: SchematicArrayKind,
    draft: &ArrayCanvasSession,
) -> Result<SchematicArrayPlacement, &'static str> {
    match kind {
        SchematicArrayKind::RadialDocumentation => {
            let base = draft.anchor.unwrap_or_else(Point::origin);
            let Some(x) = base.x.checked_add(draft.preview_delta.x) else {
                return Err(DELTA_OVERFLOW);
            };
            let Some(y) = base.y.checked_add(draft.preview_delta.y) else {
                return Err(DELTA_OVERFLOW);
            };
            Ok(SchematicArrayPlacement::Center(Point::new(x, y)))
        }
        SchematicArrayKind::Linear | SchematicArrayKind::Rectangular => {
            Ok(SchematicArrayPlacement::Pitch(draft.preview_delta))
        }
    }
}

fn update_preview_delta(draft: &mut ArrayCanvasSession, requested: Result<Point, &'static str>) {
    let requested = match requested {
        Ok(requested) => requested,
        Err(message) => {
            draft.preview_delta = Point::origin();
            draft.preview_error = Some(message.to_owned());
            return;
        }
    };
    draft.preview_delta = requested;
    // The painter keys its retained immutable preview by the complete plan.
    // Commit independently rebuilds the exact final candidate, so a prior
    // plan's cached error can never reject a newly valid pointer position.
    draft.preview_error = None;
}

fn retain_canvas_focus_from_pointer(response: &Response) {
    if response.clicked_by(egui::PointerButton::Primary)
        || response.drag_started_by(egui::PointerButton::Primary)
    {
        response.request_focus();
    }
}

fn consume_keyboard(ui: &Ui, canvas_has_focus: bool, grid_size: i32) -> (Point, bool) {
    if !canvas_has_focus {
        return (Point::origin(), false);
    }
    ui.input_mut(|input| {
        let mut step = Point::origin();
        if input.consume_key(egui::Modifiers::NONE, egui::Key::ArrowLeft) {
            step.x = -grid_size;
        }
        if input.consume_key(egui::Modifiers::NONE, egui::Key::ArrowRight) {
            step.x = grid_size;
        }
        if input.consume_key(egui::Modifiers::NONE, egui::Key::ArrowUp) {
            step.y = -grid_size;
        }
        if input.consume_key(egui::Modifiers::NONE, egui::Key::ArrowDown) {
            step.y = grid_size;
        }
        let commit = input.consume_key(egui::Modifiers::NONE, egui::Key::Enter);
        (step, commit)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn pointer_and_keyboard_delta_arithmetic_rejects_overflow() {
        assert_eq!(
            checked_accumulate_delta(Point::new(i32::MAX, 0), Point::new(1, 0)),
            Err(DELTA_OVERFLOW)
        );
        assert_eq!(
            checked_pointer_delta(Point::new(i32::MIN, 0), Point::new(i32::MAX, 0)),
            Err(DELTA_OVERFLOW)
        );
    }
    struct Canvas {
        ctx: egui::Context,
        draft: ArrayCanvasSession,
        kind: SchematicArrayKind,
        snap: SnapEngine,
    }

    impl Canvas {
        fn frame(&mut self, events: Vec<egui::Event>) -> bool {
            let mut commit = false;
            let _ = self.ctx.run_ui(
                egui::RawInput {
                    screen_rect: Some(egui::Rect::from_min_size(
                        egui::Pos2::ZERO,
                        egui::vec2(400.0, 300.0),
                    )),
                    events,
                    ..Default::default()
                },
                |ctx| {
                    egui::CentralPanel::default().show(ctx, |ui| {
                        let bounds = ui.max_rect();
                        let response = ui.interact(
                            bounds,
                            egui::Id::new("array"),
                            egui::Sense::click_and_drag(),
                        );
                        let viewport = Viewport {
                            offset: egui::pos2(-bounds.min.x, -bounds.min.y),
                            zoom: 1.0,
                            bounds,
                        };
                        if let Some(transition) = input(
                            ui,
                            &response,
                            &viewport,
                            ArrayInputView {
                                canvas: &self.draft,
                                kind: self.kind,
                                grid_size: 10,
                                snap_engine: &self.snap,
                            },
                        ) {
                            assert_eq!(transition.expected, self.draft);
                            commit = transition.commit;
                            self.draft = transition.next;
                        }
                    });
                },
            );
            commit
        }

        fn pointer(&mut self, x: f32, y: f32, pressed: bool) -> bool {
            let pos = egui::pos2(x, y);
            self.frame(vec![
                egui::Event::PointerMoved(pos),
                egui::Event::PointerButton {
                    pos,
                    button: egui::PointerButton::Primary,
                    pressed,
                    modifiers: egui::Modifiers::NONE,
                },
            ])
        }
    }

    #[test]
    fn real_pointer_gestures_preserve_array_pitch_center_and_free_snap() {
        for (kind, drag, free) in [
            (SchematicArrayKind::Linear, false, false),
            (SchematicArrayKind::Rectangular, true, false),
            (SchematicArrayKind::RadialDocumentation, false, false),
            (SchematicArrayKind::RadialDocumentation, true, true),
        ] {
            let mut canvas = Canvas {
                ctx: egui::Context::default(),
                draft: ArrayCanvasSession::default(),
                kind,
                snap: SnapEngine::default(),
            };
            canvas.snap.enabled = true;
            canvas.snap.snap_to_grid = !free;
            assert!(!canvas.frame(Vec::new()));
            assert!(!canvas.pointer(43.0, 54.0, true));
            if drag {
                assert!(!canvas.frame(vec![egui::Event::PointerMoved(egui::pos2(153.0, 104.0))]));
                assert!(canvas.draft.pointer_drag);
                assert!(canvas.pointer(153.0, 104.0, false));
            } else if kind == SchematicArrayKind::RadialDocumentation {
                assert!(canvas.pointer(43.0, 54.0, false));
            } else {
                assert!(!canvas.pointer(43.0, 54.0, false));
                assert!(!canvas.pointer(153.0, 104.0, true));
                assert!(canvas.pointer(153.0, 104.0, false));
            }
            let expected = if kind == SchematicArrayKind::RadialDocumentation {
                SchematicArrayPlacement::Center(if free {
                    Point::new(153, 104)
                } else {
                    Point::new(40, 50)
                })
            } else {
                SchematicArrayPlacement::Pitch(Point::new(110, 50))
            };
            assert_eq!(array_placement(kind, &canvas.draft), Ok(expected));
            assert!(!canvas.draft.pointer_drag);
        }
    }
}
