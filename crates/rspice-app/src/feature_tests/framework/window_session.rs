//! The window session dialog: its Windows page and its window workflows.

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

/// Detach and New window are command forms: each fact and choice on the
/// label grid, the choices kit selects, Cancel and Esc close. They had a
/// Monitor dropdown whose one entry did nothing, labels to the right of
/// their dropdowns, and the document in a bordered box over a sentence that
/// was true of every detach.
#[test]
fn the_window_workflows_are_command_forms_without_a_monitor_row() {
    for (workflow, rows) in [
        (WindowWorkflow::DetachDocument, ["Document", "Destination"]),
        (WindowWorkflow::NewApplicationWindow, ["Content", "Layout"]),
    ] {
        let ctx = Context::default();
        ctx.enable_accesskit();
        crate::ui::Theme::default().apply(&ctx);
        let mut app = RSpiceApp::test_instance();
        open_window_workflow(&mut app, workflow);
        let input = |events: Vec<egui::Event>| egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(1440.0, 900.0),
            )),
            events,
            ..egui::RawInput::default()
        };
        let mut nodes = std::collections::HashMap::new();
        for _ in 0..3 {
            let output = ctx.run_ui(input(Vec::new()), |ctx| {
                render_window_session_dialog(&mut app, ctx);
            });
            if let Some(update) = output.platform_output.accesskit_update {
                nodes.extend(update.nodes);
            }
        }
        let says = |text: &str| {
            nodes.values().any(|node| {
                node.label().is_some_and(|label| label == text)
                    || node.value().is_some_and(|value| value == text)
            })
        };
        for row in rows {
            assert!(says(row), "{workflow:?} has no {row} row");
        }
        assert!(says("Cancel"), "{workflow:?} has no Cancel");
        assert!(
            !says("Monitor"),
            "{workflow:?} offers a Monitor that does nothing"
        );
        assert!(!says("Working state remains owned by this project."));

        let escape = egui::Event::Key {
            key: egui::Key::Escape,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: egui::Modifiers::NONE,
        };
        let _ = ctx.run_ui(input(vec![escape]), |ctx| {
            render_window_session_dialog(&mut app, ctx);
        });
        assert!(
            !app.state.workbench.window_session_dialog.open,
            "Esc closes {workflow:?}"
        );
    }
}
