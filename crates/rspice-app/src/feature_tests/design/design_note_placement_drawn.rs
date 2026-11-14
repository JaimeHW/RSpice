//! Placing a design note, drawn through a whole application.

use crate::workbench::app::RSpiceApp;
use crate::workbench::app::dialogs::placement::design_note::tests::render;
use crate::workbench::app::schematic_editor_request_source;
use egui::Context;

#[test]
fn opening_input_reaches_design_note_text() {
    let ctx = Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx.options_mut(|options| options.max_passes = std::num::NonZeroUsize::new(1).unwrap());
    let mut app = RSpiceApp::test_instance();
    let source = schematic_editor_request_source(&app.state);
    app.state.design.design_note.open(source);
    // The form asks for the caret once its rows exist, so the frame that
    // builds them is the frame before the one that can be typed into.
    let _ = ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(1280.0, 800.0),
            )),
            ..Default::default()
        },
        |ctx| render(&mut app, ctx),
    );
    let _ = ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(1280.0, 800.0),
            )),
            events: vec![
                egui::Event::Key {
                    key: egui::Key::A,
                    physical_key: Some(egui::Key::A),
                    pressed: true,
                    repeat: false,
                    modifiers: egui::Modifiers::CTRL | egui::Modifiers::COMMAND,
                },
                egui::Event::Paste("Browser recovery qualification".to_owned()),
            ],
            ..Default::default()
        },
        |ctx| render(&mut app, ctx),
    );
    assert_eq!(
        app.state.design.design_note.text,
        "Browser recovery qualification"
    );
    assert!(app.state.model.schematic.document().design_notes.is_empty());
}
