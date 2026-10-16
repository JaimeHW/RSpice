//! Whether the engineering canvas holds focus.
//!
//! Shortcut scoping depends on it: the same key means one thing over the
//! canvas and another in a text field.

use egui::{Id, Response};

use crate::state::ViewType;
use crate::workbench::state::Workspace;

const CANVAS_FOCUS_RECORD_ID: &str = "rspice-engineering-canvas-focus-owner";
const CANVAS_FOCUS_REQUEST_ID: &str = "rspice-engineering-canvas-focus-request";
const ESCAPE_OWNER_RECORD_ID: &str = "rspice-escape-owner";

/// How many passes a request for canvas focus waits for a canvas to be drawn.
///
/// A route is applied at the end of the pass that asked for it, after the
/// central surface was drawn, and the canvas it lands on is drawn on the
/// next; the margin covers a workspace switch that takes a pass to settle. A
/// request no canvas answers in that time was for a destination that is not a
/// canvas, and is dropped rather than left to take focus from whatever the
/// reader does next.
const CANVAS_FOCUS_REQUEST_PASSES: u64 = 4;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct CanvasFocusRecord {
    response_id: Id,
    view_type: ViewType,
}

/// Ask that the next engineering canvas drawn take keyboard focus.
///
/// For a surface that sends the reader to a place on a canvas: the canvas's
/// single-key shortcuts act only while it holds focus, so arriving there
/// without it would leave Delete, W or R doing nothing until the reader
/// clicked the canvas they had just been taken to.
pub(crate) fn request_engineering_canvas_focus(ctx: &egui::Context) {
    let pass = ctx.cumulative_pass_nr();
    ctx.data_mut(|data| data.insert_temp(Id::new(CANVAS_FOCUS_REQUEST_ID), pass));
}

/// Report the exact egui response that owns keyboard focus for a schematic,
/// testbench, or symbol canvas. Pointer activation explicitly transfers focus
/// from any prior text/control owner to the engineering canvas, and so does a
/// request a surface made by sending the reader here.
pub(crate) fn report_engineering_canvas_focus(response: &Response, view_type: ViewType) {
    let now = response.ctx.cumulative_pass_nr();
    // The canvas drawn in the pass that asked is the one the reader is leaving:
    // the route is applied after it. The request is answered from the next
    // pass on, by whichever canvas the route put in front of the reader.
    let requested = response.ctx.data_mut(|data| {
        let id = Id::new(CANVAS_FOCUS_REQUEST_ID);
        match data.get_temp::<u64>(id) {
            Some(pass) if pass < now => {
                data.remove::<u64>(id);
                now <= pass + CANVAS_FOCUS_REQUEST_PASSES
            }
            Some(_) | None => false,
        }
    });
    if response.clicked() || response.drag_started() || requested {
        response.request_focus();
    }
    response.ctx.data_mut(|data| {
        data.insert_temp(
            Id::new(CANVAS_FOCUS_RECORD_ID),
            CanvasFocusRecord {
                response_id: response.id,
                view_type,
            },
        );
    });
}

/// Record a widget that takes Escape itself while it holds keyboard focus.
///
/// egui lets go of a focused widget on Escape at the start of the pass unless
/// the widget's lock filter claims the key, and a filter is in force only from
/// the second pass the widget holds focus. An Escape on the first pass would
/// then reach the shortcut resolver with nothing focused and resolve to
/// Cancel, although it was meant for the widget; the record is what lets the
/// resolver tell.
pub(crate) fn report_escape_owner(response: &Response) {
    if response.has_focus() {
        response.ctx.data_mut(|data| {
            data.insert_temp(Id::new(ESCAPE_OWNER_RECORD_ID), response.id);
        });
    }
}

/// Whether this pass's Escape let go of a widget that takes Escape itself, so
/// the key is that widget's to act on when it is drawn, not a shortcut's.
///
/// Asked before any widget is drawn. Focus held at the end of the last pass
/// and gone at the start of this one was taken by the Escape itself.
pub(crate) fn escape_owner_was_let_go(ctx: &egui::Context) -> bool {
    let Some(owner) = ctx.data(|data| data.get_temp::<Id>(Id::new(ESCAPE_OWNER_RECORD_ID))) else {
        return false;
    };
    ctx.input(|input| input.key_pressed(egui::Key::Escape))
        && ctx.memory(|memory| memory.had_focus_last_frame(owner) && memory.focused().is_none())
}

pub(crate) fn engineering_canvas_has_focus(
    ctx: &egui::Context,
    workspace: Workspace,
    active_view: ViewType,
) -> bool {
    if !matches!(workspace, Workspace::Design | Workspace::Models)
        || !matches!(
            active_view,
            ViewType::Schematic | ViewType::Testbench | ViewType::Symbol
        )
    {
        return false;
    }
    let record =
        ctx.data(|data| data.get_temp::<CanvasFocusRecord>(Id::new(CANVAS_FOCUS_RECORD_ID)));
    record.is_some_and(|record| {
        record.view_type == active_view
            && ctx.memory(|memory| memory.focused() == Some(record.response_id))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn focus_requires_exact_canvas_id_view_and_workspace() {
        let ctx = egui::Context::default();
        let canvas = Id::new("canvas");
        ctx.data_mut(|data| {
            data.insert_temp(
                Id::new(CANVAS_FOCUS_RECORD_ID),
                CanvasFocusRecord {
                    response_id: canvas,
                    view_type: ViewType::Schematic,
                },
            );
        });
        ctx.memory_mut(|memory| memory.request_focus(canvas));

        assert!(engineering_canvas_has_focus(
            &ctx,
            Workspace::Design,
            ViewType::Schematic
        ));
        assert!(!engineering_canvas_has_focus(
            &ctx,
            Workspace::Design,
            ViewType::Symbol
        ));
        assert!(!engineering_canvas_has_focus(
            &ctx,
            Workspace::Models,
            ViewType::Symbol
        ));
        assert!(!engineering_canvas_has_focus(
            &ctx,
            Workspace::Results,
            ViewType::Schematic
        ));

        ctx.memory_mut(|memory| memory.request_focus(Id::new("field")));
        assert!(!engineering_canvas_has_focus(
            &ctx,
            Workspace::Design,
            ViewType::Schematic
        ));
    }

    /// A request is answered by the canvas drawn on a later pass than the one
    /// that asked — the one drawn in the same pass is the canvas being left —
    /// once, and only while it is fresh.
    #[test]
    fn a_focus_request_is_answered_once_by_the_canvas_the_reader_arrives_at() {
        let ctx = egui::Context::default();
        let canvas = Id::new("canvas");
        let pass = |request: bool, draw: bool| {
            let _ = ctx.run_ui(egui::RawInput::default(), |ui| {
                if request {
                    request_engineering_canvas_focus(ui.ctx());
                }
                if draw {
                    let rect =
                        egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(100.0, 100.0));
                    let response = ui.interact(rect, canvas, egui::Sense::click());
                    report_engineering_canvas_focus(&response, ViewType::Schematic);
                }
            });
        };
        let focused = || ctx.memory(|memory| memory.focused() == Some(canvas));

        pass(true, true);
        assert!(
            !focused(),
            "the canvas drawn as the request is made is left"
        );
        pass(false, true);
        assert!(focused(), "the next canvas drawn takes the keyboard");

        ctx.memory_mut(|memory| memory.surrender_focus(canvas));
        pass(false, true);
        assert!(!focused(), "a request is answered once");

        pass(true, false);
        for _ in 1..CANVAS_FOCUS_REQUEST_PASSES {
            pass(false, false);
        }
        pass(false, true);
        assert!(focused(), "a canvas drawn within the margin answers it");

        ctx.memory_mut(|memory| memory.surrender_focus(canvas));
        pass(true, false);
        for _ in 0..CANVAS_FOCUS_REQUEST_PASSES {
            pass(false, false);
        }
        pass(false, true);
        assert!(
            !focused(),
            "a request no canvas answered in time takes nothing from the reader",
        );
    }
}
