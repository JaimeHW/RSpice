//! Local symbol draft edits and their undo intent, without application state.

use super::*;

#[test]
fn finishing_pending_polyline_creates_one_selected_shape() {
    let mut session = SymbolEditorSession::default();
    let mut edit = CanvasEdit {
        session: &mut session,
        capabilities: SymbolEditCapabilities { edit: true },
        outcome: SymbolEditOutcome::default(),
    };
    let mut document = SymbolDocument::default();
    edit.session.pending_polyline = vec![Point::new(0, 0), Point::new(10, 0)];

    assert!(finish_pending_polyline(&mut edit, &mut document));

    assert_eq!(document.body.len(), 1);
    assert_eq!(edit.session.selected_shape, Some(0));
    assert!(edit.session.selection.shapes.contains(&0));
    assert!(edit.session.pending_polyline.is_empty());
}

#[test]
fn place_pin_without_available_pin_does_not_record_undo() {
    let mut session = SymbolEditorSession::default();
    let mut edit = CanvasEdit {
        session: &mut session,
        capabilities: SymbolEditCapabilities { edit: true },
        outcome: SymbolEditOutcome::default(),
    };
    let mut document = SymbolDocument::default();

    assert!(!place_selected_pin(
        &mut edit,
        &mut document,
        Point::new(10, 0)
    ));

    assert!(edit.outcome.undo_before.is_none());
    assert!(document.pins.is_empty());
}

/// A drag reports as started only past egui's threshold, by which time a
/// quick hand is well away from the pin it pressed on.
#[test]
fn a_drag_starts_where_the_press_was() {
    let ctx = egui::Context::default();
    let source = SymbolRequestSource {
        project: rspice_vocabulary::product::ProjectId::new(),
        document: rspice_design_model::cell_view::CellViewRef::new("work", "amp", "symbol"),
        occurrence: None,
        design_epoch: 3,
        document_epoch: 4,
        library_revision: 5,
        window: 1,
    };
    let session = SymbolEditorSession::default();
    let screen = egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(800.0, 600.0));
    let viewport = SymbolViewport::new(screen, 4.0, egui::Vec2::ZERO);
    let press = egui::pos2(100.0, 100.0);
    let frames = [
        vec![egui::Event::PointerMoved(press)],
        vec![egui::Event::PointerButton {
            pos: press,
            button: egui::PointerButton::Primary,
            pressed: true,
            modifiers: egui::Modifiers::NONE,
        }],
        vec![egui::Event::PointerMoved(egui::pos2(140.0, 100.0))],
        vec![egui::Event::PointerMoved(egui::pos2(150.0, 100.0))],
    ];
    let mut started = None;
    for events in frames {
        let _ = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(screen),
                events,
                ..Default::default()
            },
            |ui| {
                let response = ui.interact(
                    screen,
                    ui.id().with("canvas"),
                    egui::Sense::click_and_drag(),
                );
                if let Some(request) = canvas_request(&source, &session, viewport, &response)
                    && request.input.drag_started
                {
                    started = Some(request.input.pointer);
                }
            },
        );
    }
    assert_eq!(started, Some(press));
}
