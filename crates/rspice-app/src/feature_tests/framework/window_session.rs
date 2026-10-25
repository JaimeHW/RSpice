//! The window session dialog's Windows page carries no constant paragraphs.

use crate::workbench::RSpiceApp;
use crate::workbench::app::dialogs::window_session::*;
use egui::Context;

/// The Windows page lists the windows and the actions on them. Two
/// paragraphs restating, on every opening, what detaching does and how
/// geometry is restored were constant boilerplate under the table.
#[test]
fn the_windows_page_carries_no_constant_paragraphs() {
    let ctx = Context::default();
    ctx.enable_accesskit();
    crate::ui::Theme::default().apply(&ctx);
    let mut app = RSpiceApp::test_instance();
    app.state
        .workbench
        .window_session_dialog
        .open(WindowSessionPage::Windows);
    // Access updates are incremental: keep the latest of every node.
    let mut nodes = std::collections::HashMap::new();
    for _ in 0..3 {
        let output = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(1440.0, 900.0),
                )),
                ..egui::RawInput::default()
            },
            |ctx| render_window_session_dialog(&mut app, ctx),
        );
        if let Some(update) = output.platform_output.accesskit_update {
            nodes.extend(update.nodes);
        }
    }
    let says = |text: &str| {
        nodes.values().any(|node| {
            node.label().is_some_and(|label| label.contains(text))
                || node.value().is_some_and(|value| value.contains(text))
        })
    };
    assert!(says("Application window"), "the windows table is missing");
    assert!(!says("Detaching changes presentation only"));
    assert!(!says("Restored geometry is clamped"));
}
