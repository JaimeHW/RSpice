//! App availability and command dispatch for source-bound touch viewport requests.

use crate::diagnostics::ConsoleMessage;
use crate::workbench::RSpiceApp;
use crate::workbench::commands::vocabulary::Command;
use crate::workbench::{
    CapabilityWorkflowId, RouteTransitionSource, SurfaceId, SurfaceRoute, route_availability,
};
use egui::{Context, Rect};
use rspice_schematic_editor::view::mobile_controls::{
    self, MobileCanvasAction, MobileCanvasCapabilities, MobileCanvasRequest,
};

pub(crate) fn show(ctx: &Context, app: &mut RSpiceApp, content_rect: Rect) {
    if mobile_controls::mobile_canvas_controls_rect(content_rect, ctx.content_rect().width())
        .is_none()
    {
        return;
    }
    let source = super::requests::editor_request_source(&app.state);
    let capabilities = MobileCanvasCapabilities {
        zoom_out: action_enabled(app, MobileCanvasAction::ZoomOut),
        zoom_fit: action_enabled(app, MobileCanvasAction::ZoomFit),
        zoom_in: action_enabled(app, MobileCanvasAction::ZoomIn),
        touch_edit_guide: action_enabled(app, MobileCanvasAction::TouchEditGuide),
    };
    if let Some(request) = mobile_controls::show(ctx, content_rect, &source, capabilities) {
        execute_action(app, request);
    }
}

fn product_command(action: MobileCanvasAction) -> Option<Command> {
    match action {
        MobileCanvasAction::ZoomOut => Some(Command::ZoomOut),
        MobileCanvasAction::ZoomFit => Some(Command::ZoomFit),
        MobileCanvasAction::ZoomIn => Some(Command::ZoomIn),
        MobileCanvasAction::TouchEditGuide => None,
    }
}

fn action_enabled(app: &RSpiceApp, action: MobileCanvasAction) -> bool {
    if let Some(command) = product_command(action) {
        command.is_enabled(app)
    } else {
        route_availability(SurfaceRoute::capability_workflow(
            CapabilityWorkflowId::TouchEditGuide,
        ))
        .can_open()
    }
}

fn execute_action(app: &mut RSpiceApp, request: MobileCanvasRequest) {
    if request.source != super::requests::editor_request_source(&app.state)
        || app.state.workbench.current_route().surface_id() != SurfaceId::Design
        || !matches!(
            app.state.workspace.content.active_view_type(),
            crate::state::ViewType::Schematic | crate::state::ViewType::Testbench
        )
        || app.state.application_modal_open()
        || !action_enabled(app, request.action)
    {
        return;
    }
    if let Some(command) = product_command(request.action) {
        command.execute(app);
    } else {
        let route = SurfaceRoute::capability_workflow(CapabilityWorkflowId::TouchEditGuide);
        if let Err(error) = app
            .state
            .workbench
            .navigate(route, RouteTransitionSource::User)
        {
            app.state
                .push_user_message(ConsoleMessage::warning(error.to_string()));
        }
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;
    use crate::workbench::state::Workspace;

    fn execute_current(app: &mut RSpiceApp, action: MobileCanvasAction) {
        let request = MobileCanvasRequest {
            source: super::super::requests::editor_request_source(&app.state),
            action,
        };
        execute_action(app, request);
    }

    #[test]
    fn touch_requests_recheck_document_route_and_modal_state() {
        for change in ["document", "route", "modal", "read-only", "none"] {
            let mut app = RSpiceApp::test_instance();
            app.state.workbench.activate(Workspace::Design);
            app.state.schematic.session.editor.zoom = 1.0;
            let request = MobileCanvasRequest {
                source: super::super::requests::editor_request_source(&app.state),
                action: MobileCanvasAction::ZoomIn,
            };
            match change {
                "document" => app.state.bump_active_schematic_epoch(),
                "route" => app.state.workbench.activate(Workspace::Results),
                "modal" => app.state.dialogs.preferences_open = true,
                "read-only" => app.state.schematic.session.read_only = true,
                "none" => {}
                _ => unreachable!(),
            }
            execute_action(&mut app, request);
            let expected_zoom = if matches!(change, "none" | "read-only") {
                1.2
            } else {
                1.0
            };
            assert_eq!(
                app.state.schematic.session.editor.zoom, expected_zoom,
                "{change}"
            );
        }
    }

    #[test]
    fn actions_execute_the_schematic_camera_and_touch_guide_route() {
        let mut app = RSpiceApp::test_instance();
        app.state.workbench.activate(Workspace::Design);
        assert!(matches!(
            app.state.workspace.content.active_view_type(),
            crate::state::ViewType::Schematic | crate::state::ViewType::Testbench
        ));
        app.state.schematic.session.editor.zoom = 1.0;

        // The touch route dispatches the same Command as every other surface,
        // so it steps by COMMAND_ZOOM_FACTOR (1.2) rather than a rate of its
        // own. Asserting the shared factor is the point: a mobile-only zoom
        // step would mean the same command zoomed differently by input device.
        execute_current(&mut app, MobileCanvasAction::ZoomOut);
        assert!((app.state.schematic.session.editor.zoom - (1.0 / 1.2)).abs() < f64::EPSILON);
        execute_current(&mut app, MobileCanvasAction::ZoomIn);
        assert!((app.state.schematic.session.editor.zoom - 1.0).abs() < f64::EPSILON);

        // ZoomFit frames the drawing sheet; FitSchematicContent frames the
        // content. The touch route must not collapse that distinction, so
        // assert each sets its own flag and clears the other.
        assert!(!app.state.schematic.session.editor.needs_drawing_sheet_fit);
        execute_current(&mut app, MobileCanvasAction::ZoomFit);
        assert!((app.state.schematic.session.editor.zoom - 1.0).abs() < f64::EPSILON);
        assert_eq!(app.state.schematic.session.editor.pan, (0.0, 0.0));
        assert!(app.state.schematic.session.editor.needs_drawing_sheet_fit);
        assert!(!app.state.schematic.session.editor.needs_fit);

        Command::FitSchematicContent.execute(&mut app);
        assert!(app.state.schematic.session.editor.needs_fit);
        assert!(!app.state.schematic.session.editor.needs_drawing_sheet_fit);

        execute_current(&mut app, MobileCanvasAction::TouchEditGuide);
        assert_eq!(
            app.state.workbench.current_route(),
            SurfaceRoute::capability_workflow(CapabilityWorkflowId::TouchEditGuide)
        );
        assert_eq!(
            app.state.workbench.current_route().surface_id(),
            SurfaceId::FeatureAvailability
        );
    }
}
