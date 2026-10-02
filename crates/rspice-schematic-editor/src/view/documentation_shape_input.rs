//! Documentation-shape cursor and keyboard input over a borrowed editor draft.

use super::{snap_resolution::resolve_grid_pointer, viewport::Viewport};
use crate::{
    requests::EditorRequestSource,
    session::{documentation_shape::DocumentationShapeDrawing, snap::SnapEngine},
};
use egui::{Response, Ui};
use rspice_design::schematic::documentation_shape::DocumentationShapeKind;
use rspice_design_model::Point;

pub struct ShapeInputView<'a> {
    pub drawing: &'a DocumentationShapeDrawing,
    pub kind: Option<DocumentationShapeKind>,
    pub snap_engine: &'a SnapEngine,
    pub grid_size: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShapeInputAction {
    PlacePoint(Point),
    FinishPolygon,
}

#[derive(Debug, Clone)]
pub struct ShapeInputTransition {
    pub expected: DocumentationShapeDrawing,
    pub next: DocumentationShapeDrawing,
    pub action: Option<ShapeInputAction>,
}

#[derive(Debug, Clone)]
pub struct ShapeInputRequest {
    pub source: EditorRequestSource,
    pub kind: Option<DocumentationShapeKind>,
    pub transition: ShapeInputTransition,
}

pub fn pointer_motion(
    ui: &Ui,
    response: &Response,
    viewport: &Viewport,
    view: ShapeInputView<'_>,
) -> Option<ShapeInputTransition> {
    if ui.input(|input| input.pointer.delta() == egui::Vec2::ZERO) {
        return None;
    }
    let position = resolve_grid_pointer(
        view.snap_engine,
        view.grid_size,
        viewport,
        response.hover_pos()?,
    )
    .snapped_position;
    if view.drawing.keyboard_cursor == Some(position) && !view.drawing.keyboard_active {
        return None;
    }
    let mut next = view.drawing.clone();
    next.keyboard_cursor = Some(position);
    next.keyboard_active = false;
    Some(ShapeInputTransition {
        expected: view.drawing.clone(),
        next,
        action: None,
    })
}

/// The host calls this only for the focused canvas after its modal/tool gates.
pub fn keyboard(
    ui: &Ui,
    response: &Response,
    viewport: &Viewport,
    view: ShapeInputView<'_>,
    step_grid_size: i32,
) -> Option<ShapeInputTransition> {
    let (left, right, up, down, place, finish, backspace) = ui.input_mut(|input| {
        (
            input.consume_key(egui::Modifiers::NONE, egui::Key::ArrowLeft),
            input.consume_key(egui::Modifiers::NONE, egui::Key::ArrowRight),
            input.consume_key(egui::Modifiers::NONE, egui::Key::ArrowUp),
            input.consume_key(egui::Modifiers::NONE, egui::Key::ArrowDown),
            input.consume_key(egui::Modifiers::NONE, egui::Key::Space),
            input.consume_key(egui::Modifiers::NONE, egui::Key::Enter),
            input.consume_key(egui::Modifiers::NONE, egui::Key::Backspace),
        )
    });
    let directional = left || right || up || down;
    if !directional && !place && !finish && !backspace {
        return None;
    }
    let hover_point = || {
        response.hover_pos().map(|position| {
            resolve_grid_pointer(view.snap_engine, view.grid_size, viewport, position)
                .snapped_position
        })
    };
    let mut next = view.drawing.clone();
    if directional {
        let fallback = hover_point()
            .or_else(|| next.points.last().copied())
            .unwrap_or_else(Point::origin);
        let step = if view.snap_engine.enabled && view.snap_engine.snap_to_grid {
            step_grid_size.max(1)
        } else {
            1
        };
        let mut cursor = next.keyboard_cursor.unwrap_or(fallback);
        if left {
            cursor.x = cursor.x.saturating_sub(step);
        }
        if right {
            cursor.x = cursor.x.saturating_add(step);
        }
        if up {
            cursor.y = cursor.y.saturating_sub(step);
        }
        if down {
            cursor.y = cursor.y.saturating_add(step);
        }
        next.keyboard_cursor = Some(cursor);
        next.keyboard_active = true;
    }
    if backspace {
        next.points.pop();
    }
    let action = next
        .keyboard_cursor
        .or_else(hover_point)
        .and_then(|cursor| {
            if place {
                next.keyboard_active = true;
                Some(ShapeInputAction::PlacePoint(cursor))
            } else if finish {
                if view.kind == Some(DocumentationShapeKind::Polygon) && next.points.len() >= 3 {
                    Some(ShapeInputAction::FinishPolygon)
                } else {
                    next.keyboard_active = true;
                    Some(ShapeInputAction::PlacePoint(cursor))
                }
            } else {
                None
            }
        });
    (next != *view.drawing || action.is_some()).then(|| ShapeInputTransition {
        expected: view.drawing.clone(),
        next,
        action,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn keys(
        drawing: &DocumentationShapeDrawing,
        keys: &[egui::Key],
    ) -> Option<ShapeInputTransition> {
        let ctx = egui::Context::default();
        let mut transition = None;
        let _ = ctx.run_ui(
            egui::RawInput {
                events: keys
                    .iter()
                    .map(|&key| egui::Event::Key {
                        key,
                        physical_key: None,
                        pressed: true,
                        repeat: false,
                        modifiers: egui::Modifiers::NONE,
                    })
                    .collect(),
                ..Default::default()
            },
            |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    let response =
                        ui.allocate_response(egui::vec2(200.0, 200.0), egui::Sense::click());
                    transition = keyboard(
                        ui,
                        &response,
                        &Viewport {
                            offset: egui::Pos2::ZERO,
                            zoom: 1.0,
                            bounds: response.rect,
                        },
                        ShapeInputView {
                            drawing,
                            kind: Some(DocumentationShapeKind::Polygon),
                            snap_engine: &SnapEngine::default(),
                            grid_size: 10,
                        },
                        10,
                    );
                });
            },
        );
        transition
    }

    #[test]
    fn keyboard_preserves_saturation_and_polygon_completion_order() {
        use egui::Key::*;
        let mut drawing = DocumentationShapeDrawing::default();
        drawing
            .points
            .extend([Point::origin(), Point::new(20, 0), Point::new(10, 10)]);
        assert!(keys(&drawing, &[]).is_none());
        assert!(
            keys(&drawing, &[Enter]).is_none(),
            "Enter needs a cursor or hover even for a polygon"
        );
        drawing.keyboard_cursor = Some(Point::new(i32::MAX, i32::MIN));
        let transition = keys(
            &drawing,
            &[
                ArrowLeft, ArrowRight, ArrowUp, ArrowDown, Backspace, Space, Enter,
            ],
        )
        .unwrap();
        assert_eq!(transition.expected, drawing);
        assert_eq!(transition.next.points.len(), 2);
        assert_eq!(
            transition.next.keyboard_cursor,
            Some(Point::new(i32::MAX, i32::MIN + 10))
        );
        assert_eq!(
            transition.action,
            Some(ShapeInputAction::PlacePoint(Point::new(
                i32::MAX,
                i32::MIN + 10
            )))
        );
        assert!(transition.next.keyboard_active);
        let transition = keys(&drawing, &[Enter]).unwrap();
        assert_eq!(transition.action, Some(ShapeInputAction::FinishPolygon));
        assert_eq!(transition.next, drawing);
        let transition = keys(&drawing, &[Backspace, Enter]).unwrap();
        assert_eq!(transition.next.points.len(), 2);
        assert_eq!(
            transition.action,
            Some(ShapeInputAction::PlacePoint(Point::new(i32::MAX, i32::MIN)))
        );
    }
}
