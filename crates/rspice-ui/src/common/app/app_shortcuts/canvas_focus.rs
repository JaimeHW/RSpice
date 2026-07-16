use egui::{Id, Response};

use crate::state::ViewType;
use crate::workbench::state::Workspace;

const CANVAS_FOCUS_RECORD_ID: &str = "rspice-engineering-canvas-focus-owner";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct CanvasFocusRecord {
    response_id: Id,
    view_type: ViewType,
}

/// Report the exact egui response that owns keyboard focus for a schematic,
/// testbench, or symbol canvas. Pointer activation explicitly transfers focus
/// from any prior text/control owner to the engineering canvas.
pub(crate) fn report_engineering_canvas_focus(response: &Response, view_type: ViewType) {
    if response.clicked() || response.drag_started() {
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
}
