//! Shared focus and keyboard input for transactional schematic transforms.

use crate::session::transform::TransformCanvasSession;
use egui::{Response, Ui};
use rspice_design_model::Point;

#[derive(Debug, Clone)]
pub struct TransformInputTransition {
    pub expected: TransformCanvasSession,
    pub next: TransformCanvasSession,
    pub commit: bool,
}

pub(crate) fn retain_canvas_focus_from_pointer(response: &Response) {
    // Touch taps are reported as primary clicks by egui. A drag claims focus as
    // soon as it crosses the drag threshold so keyboard continuation works after
    // either pointer interaction without stealing focus from unrelated controls.
    if response.clicked_by(egui::PointerButton::Primary)
        || response.drag_started_by(egui::PointerButton::Primary)
    {
        response.request_focus();
    }
}

pub(crate) fn consume_keyboard(ui: &Ui, canvas_has_focus: bool, grid_size: i32) -> (Point, bool) {
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
    fn move_keyboard_input() -> egui::RawInput {
        egui::RawInput {
            events: [egui::Key::ArrowRight, egui::Key::Enter]
                .into_iter()
                .map(|key| egui::Event::Key {
                    key,
                    physical_key: None,
                    pressed: true,
                    repeat: false,
                    modifiers: egui::Modifiers::NONE,
                })
                .collect(),
            ..Default::default()
        }
    }
    #[test]
    fn armed_move_keyboard_leaves_keys_unconsumed_without_canvas_focus() {
        let ctx = egui::Context::default();
        let mut intent = None;
        let mut keys_remain = None;

        let _ = ctx.run_ui(move_keyboard_input(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                // accessibility-pointer-shim: test-only canvas focus harness.
                let response = ui.interact(
                    ui.max_rect(),
                    egui::Id::new("unfocused-move-canvas"),
                    egui::Sense::click_and_drag(),
                );
                assert!(!response.has_focus());
                intent = Some(consume_keyboard(ui, response.has_focus(), 10));
                keys_remain = Some(ui.input(|input| {
                    (
                        input.key_pressed(egui::Key::ArrowRight),
                        input.key_pressed(egui::Key::Enter),
                    )
                }));
            });
        });

        assert_eq!(intent, Some((Point::origin(), false)));
        assert_eq!(keys_remain, Some((true, true)));
    }
    #[test]
    fn armed_move_keyboard_consumes_keys_when_canvas_has_focus() {
        let ctx = egui::Context::default();
        let mut intent = None;
        let mut keys_remain = None;

        let _ = ctx.run_ui(move_keyboard_input(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                // accessibility-pointer-shim: test-only canvas focus harness.
                let response = ui.interact(
                    ui.max_rect(),
                    egui::Id::new("focused-move-canvas"),
                    egui::Sense::click_and_drag(),
                );
                response.request_focus();
                assert!(response.has_focus());
                intent = Some(consume_keyboard(ui, response.has_focus(), 10));
                keys_remain = Some(ui.input(|input| {
                    (
                        input.key_pressed(egui::Key::ArrowRight),
                        input.key_pressed(egui::Key::Enter),
                    )
                }));
            });
        });

        assert_eq!(intent, Some((Point::new(10, 0), true)));
        assert_eq!(keys_remain, Some((false, false)));
    }
}
