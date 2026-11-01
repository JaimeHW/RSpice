//! The window session dialog: its Windows page and its window workflows.

use crate::workbench::RSpiceApp;
use crate::workbench::app::dialogs::window_session::*;
use crate::workbench::state::{Workspace, WorkspaceLayoutState};
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

/// The window the manager is open in already has the focus, so its row
/// offers no Focus; with no other window there is no Action column.
#[test]
fn the_active_window_offers_no_focus() {
    let ctx = Context::default();
    ctx.enable_accesskit();
    crate::ui::Theme::default().apply(&ctx);
    let mut app = RSpiceApp::test_instance();
    app.state
        .workbench
        .window_session_dialog
        .open(WindowSessionPage::Windows);
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
        nodes
            .values()
            .any(|node| node.label() == Some(text) || node.value() == Some(text))
    };
    assert!(says("active"), "the main window's row is missing");
    assert!(!says("Focus"), "the active window offers Focus");
    assert!(!says("Action"), "an Action column with no actions");
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

/// Two detached windows, the second one off screen, with `workflow` open.
fn app_with_detached_windows(workflow: WindowWorkflow) -> RSpiceApp {
    let mut app = RSpiceApp::test_instance();
    let session = &mut app.state.workbench.window_session;
    for title in ["Review bench", "Plots"] {
        let id = session.create_window(
            title,
            Workspace::Design,
            WorkspaceLayoutState::default(),
            false,
        );
        if title == "Plots" {
            session
                .state_mut(id)
                .expect("the window just created")
                .bounds
                .recovery_pending = true;
        }
    }
    open_window_workflow(&mut app, workflow);
    app
}

/// Every label and value the window session dialog exposes after three
/// frames at the desktop size.
fn window_dialog_texts(ctx: &Context, app: &mut RSpiceApp) -> Vec<String> {
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
            |ctx| render_window_session_dialog(app, ctx),
        );
        if let Some(update) = output.platform_output.accesskit_update {
            nodes.extend(update.nodes);
        }
    }
    nodes
        .values()
        .flat_map(|node| [node.label(), node.value()])
        .flatten()
        .map(str::to_owned)
        .collect()
}

/// Consolidate and monitor recovery are command forms over the detached
/// windows. Each opened on a paragraph that said the same thing every
/// time, over a striped table with a constant "remove detached window"
/// column, and a refusal printed under the table in red.
#[test]
fn consolidate_and_recovery_are_command_forms_without_explainers() {
    for (workflow, rows) in [
        (
            WindowWorkflow::ConsolidateWindows,
            ["Windows", "Review bench, Plots", "Documents"],
        ),
        (
            WindowWorkflow::MonitorRecovery,
            [
                "Review bench",
                "Plots \u{00b7} off screen",
                "Keep restored windows on screen",
            ],
        ),
    ] {
        let ctx = Context::default();
        ctx.enable_accesskit();
        crate::ui::Theme::default().apply(&ctx);
        let mut app = app_with_detached_windows(workflow);
        let texts = window_dialog_texts(&ctx, &mut app);
        for row in rows {
            assert!(
                texts.iter().any(|text| text == row),
                "{workflow:?} has no {row}: {texts:?}"
            );
        }
        assert!(texts.iter().any(|text| text == "Cancel"));
        for explainer in [
            "All detached documents return",
            "Recovery changes window geometry only",
            "remove detached window",
        ] {
            assert!(
                !texts.iter().any(|text| text.contains(explainer)),
                "{workflow:?} explains itself: {explainer}"
            );
        }

        let escape = egui::Event::Key {
            key: egui::Key::Escape,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: egui::Modifiers::NONE,
        };
        let _ = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(1440.0, 900.0),
                )),
                events: vec![escape],
                ..egui::RawInput::default()
            },
            |ctx| render_window_session_dialog(&mut app, ctx),
        );
        assert!(
            !app.state.workbench.window_session_dialog.open,
            "Esc closes {workflow:?}"
        );
    }
}

/// With nothing detached there is nothing to consolidate, and the footer
/// says so before the primary runs into an error.
#[test]
fn consolidate_with_nothing_detached_refuses_before_running() {
    let ctx = Context::default();
    ctx.enable_accesskit();
    crate::ui::Theme::default().apply(&ctx);
    let mut app = RSpiceApp::test_instance();
    open_window_workflow(&mut app, WindowWorkflow::ConsolidateWindows);
    let texts = window_dialog_texts(&ctx, &mut app);
    assert!(
        texts.iter().any(|text| text == "No window is detached."),
        "no refusal in the footer: {texts:?}"
    );
    // The body states the fact the refusal is about rather than standing empty.
    assert!(texts.iter().any(|text| text == "None"), "{texts:?}");
}
