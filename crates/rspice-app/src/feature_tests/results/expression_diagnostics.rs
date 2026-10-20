//! The expression and unit diagnostics report through a whole application.

/// The report is the kit's dialog: modal, centred like every other one, and
/// Esc closes it. It was an egui window opened at the top left over the title
/// and menu bar, which Esc left open.
#[test]
fn the_report_is_a_kit_dialog_that_escape_closes() {
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let mut app = crate::workbench::RSpiceApp::test_instance();
    app.state.results.expression_diagnostics_dialog = true;
    let input = |events: Vec<egui::Event>| egui::RawInput {
        screen_rect: Some(egui::Rect::from_min_size(
            egui::Pos2::ZERO,
            egui::vec2(1440.0, 900.0),
        )),
        events,
        ..Default::default()
    };
    let mut frame = |events: Vec<egui::Event>| {
        let _ = ctx.run_ui(input(events), |ui| {
            app.prepare_frame(ui.ctx());
            app.render_frame_chrome(ui);
            app.render_frame_dialogs(ui.ctx());
        });
    };
    frame(Vec::new());
    frame(Vec::new());
    assert!(
        ctx.memory(|memory| memory.top_modal_layer()).is_some(),
        "the report is a modal dialog"
    );
    let escape = egui::Event::Key {
        key: egui::Key::Escape,
        physical_key: None,
        pressed: true,
        repeat: false,
        modifiers: egui::Modifiers::NONE,
    };
    frame(vec![escape]);
    assert!(!app.state.results.expression_diagnostics_dialog);
}
