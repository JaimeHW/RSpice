//! The canvas's touch controls driven through a whole application.

use crate::schematic::view::mobile_controls::tests::strip_cx;
use crate::schematic::view::mobile_controls::*;
use crate::workbench::RSpiceApp;
use crate::workbench::state::Workspace;
use egui::Context;
use rspice_schematic_editor::view::mobile_controls::{MobileCanvasAction, MobileCanvasRequest};
use rspice_schematic_editor::view::navigation::ZoomRequest;

#[test]
fn touch_requests_recheck_document_route_and_modal_state() {
    for change in ["document", "route", "modal", "read-only", "none"] {
        let mut app = RSpiceApp::test_instance();
        app.state.workbench.activate(Workspace::Design);
        app.state
            .model
            .schematic
            .session
            .editor
            .needs_drawing_sheet_fit = false;
        let request = MobileCanvasRequest {
            source: crate::schematic::view::requests::editor_request_source(&app.state),
            action: MobileCanvasAction::Fit,
        };
        match change {
            "document" => app.state.bump_active_schematic_epoch(),
            "route" => app.state.workbench.activate(Workspace::Results),
            "modal" => app.state.settings.preferences_open = true,
            "read-only" => app.state.model.schematic.session.read_only = true,
            "none" => {}
            _ => unreachable!(),
        }
        execute_action(&Context::default(), &mut strip_cx(&mut app), request);
        app.after_contribution();
        assert_eq!(
            app.state
                .model
                .schematic
                .session
                .editor
                .needs_drawing_sheet_fit,
            matches!(change, "none" | "read-only"),
            "{change}"
        );
    }
}

/// The view buttons dispatch the same `Command` as every other surface,
/// so a fit from the strip is the fit from the View menu and `F`.
#[test]
fn fit_executes_the_schematic_camera_command() {
    let mut app = RSpiceApp::test_instance();
    app.state.workbench.activate(Workspace::Design);
    app.state.model.schematic.session.editor.zoom = 2.0;
    app.state.model.schematic.session.editor.zoom_request = Some(ZoomRequest::By(1.2));
    let request = MobileCanvasRequest {
        source: crate::schematic::view::requests::editor_request_source(&app.state),
        action: MobileCanvasAction::Fit,
    };
    execute_action(&Context::default(), &mut strip_cx(&mut app), request);
    app.after_contribution();
    assert!(
        app.state
            .model
            .schematic
            .session
            .editor
            .needs_drawing_sheet_fit
    );
    assert!(!app.state.model.schematic.session.editor.needs_fit);
}
