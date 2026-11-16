//! The canvas's queued input answered through the whole application: a shape gesture
//! the shell has since replaced edits nothing, and a requirement link opens only the
//! results' own specifications or a safe external address.

use crate::workbench::app_state::AppState;
use rspice_app_design::schematic::view::interaction::*;
use rspice_app_state::state::Point;
use rspice_app_state::state::{
    DesignNoteKind, DocumentationShapeKind, PendingDocumentationShapePlacement, Tool,
};
use rspice_schematic_editor::view::documentation_shape_input::ShapeInputAction;
use rspice_schematic_editor::view::documentation_shape_input::ShapeInputTransition;

#[test]
fn queued_shape_input_cannot_edit_a_replaced_gesture_or_changed_context() {
    for change in [
        "none", "rearmed", "document", "draft", "kind", "tool", "modal",
    ] {
        let mut state = AppState::default();
        state
            .model
            .schematic
            .session
            .editor
            .pending_documentation_shape = Some(
            PendingDocumentationShapePlacement::new(
                DocumentationShapeKind::Line,
                state.model.schematic.topology_version(),
                &[],
            )
            .with_source(
                rspice_app_design::schematic::view::requests::editor_request_source(&state),
            ),
        );
        state.model.schematic.session.editor.tool = Tool::DocumentationShape;
        let drawing = &mut state
            .model
            .schematic
            .session
            .editor
            .documentation_shape_drawing;
        drawing.points.push(Point::origin());
        drawing.keyboard_cursor = Some(Point::origin());
        let expected = drawing.clone();
        let mut next = expected.clone();
        next.keyboard_cursor = Some(Point::new(10, 0));
        next.keyboard_active = true;
        let request = capture_shape_input(
            &state.canvas_parts(),
            ShapeInputTransition {
                expected,
                next,
                action: Some(ShapeInputAction::PlacePoint(Point::new(10, 0))),
            },
        );
        match change {
            "none" => {}
            "rearmed" => {
                // Same source, kind and visible draft, but a different gesture lifetime.
                let drawing = &mut state
                    .model
                    .schematic
                    .session
                    .editor
                    .documentation_shape_drawing;
                drawing.clear();
                drawing.points.push(Point::origin());
                drawing.keyboard_cursor = Some(Point::origin());
            }
            "document" => state.model.active_schematic_epoch += 1,
            "draft" => state
                .model
                .schematic
                .session
                .editor
                .documentation_shape_drawing
                .points
                .push(Point::new(20, 20)),
            "kind" => {
                state
                    .model
                    .schematic
                    .session
                    .editor
                    .pending_documentation_shape
                    .as_mut()
                    .unwrap()
                    .kind = DocumentationShapeKind::Rectangle
            }
            "tool" => state.model.schematic.session.editor.tool = Tool::Select,
            "modal" => state.shell.about = true,
            _ => unreachable!(),
        }
        let drawing_before = state
            .model
            .schematic
            .session
            .editor
            .documentation_shape_drawing
            .clone();
        let pending_before = state
            .model
            .schematic
            .session
            .editor
            .pending_documentation_shape
            .clone();
        apply_shape_input(&mut state.canvas_parts(), request.clone());
        if change == "none" {
            assert_eq!(
                state.model.schematic.document().documentation_shapes.len(),
                1
            );
            assert!(state.model.schematic.undo());
        } else {
            assert_eq!(
                state
                    .model
                    .schematic
                    .session
                    .editor
                    .documentation_shape_drawing,
                drawing_before,
                "{change}"
            );
            assert_eq!(
                state
                    .model
                    .schematic
                    .session
                    .editor
                    .pending_documentation_shape,
                pending_before,
                "{change}"
            );
        }
        assert!(
            state
                .model
                .schematic
                .document()
                .documentation_shapes
                .is_empty(),
            "{change}"
        );
        assert!(!state.model.schematic.can_undo(), "{change}");
    }
}

#[test]
fn requirement_link_activation_uses_owned_specifications_or_safe_external_url() {
    let mut state = AppState::default();
    state.simulate.verification_page = rspice_app_state::state::VerificationPage::Corners;
    state
        .model
        .schematic
        .document_mut_for_test()
        .design_notes
        .push(
            rspice_app_state::state::DesignNote::new(
                32,
                Point::new(20, 20),
                DesignNoteKind::RequirementLink,
                "REQ-19",
            )
            .unwrap(),
        );
    let ctx = egui::Context::default();
    assert!(activate_requirement_link(
        &mut state.canvas_parts(),
        32,
        &ctx
    ));
    crate::workbench::commands::answer_surface_requests(&mut state);
    assert_eq!(
        state.workbench.workspace,
        rspice_app_state::workbench::workspace::Workspace::Verify
    );
    assert_eq!(
        state.simulate.verification_page,
        rspice_app_state::state::VerificationPage::Specifications
    );

    state
        .model
        .schematic
        .document_mut_for_test()
        .design_notes
        .push(
            rspice_app_state::state::DesignNote::new(
                33,
                Point::new(30, 20),
                DesignNoteKind::RequirementLink,
                "https://tracker.example/item?id=19&from=schematic%20note",
            )
            .unwrap(),
        );
    let output = ctx.run_ui(egui::RawInput::default(), |ctx| {
        assert!(activate_requirement_link(
            &mut state.canvas_parts(),
            33,
            ctx
        ));
    });
    assert!(
        output
            .platform_output
            .commands
            .iter()
            .any(|command| matches!(
                command,
                egui::OutputCommand::OpenUrl(open)
                    if open.url == "https://tracker.example/item?id=19&from=schematic%20note"
                        && open.new_tab
            ))
    );
}
