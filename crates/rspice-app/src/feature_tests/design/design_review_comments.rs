//! The design review comments dialog drawn through a whole application.

use crate::workbench::app::RSpiceApp;
use crate::workbench::app::dialogs::design_review_comments::tests::review_note;
use crate::workbench::app::dialogs::design_review_comments::*;
use egui::Context;

#[test]
fn initial_focus_accepts_review_reply_typing() {
    let ctx = Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx.options_mut(|options| options.max_passes = std::num::NonZeroUsize::new(1).unwrap());
    let mut app = RSpiceApp::test_instance();
    app.state
        .model
        .schematic
        .document_mut_for_test()
        .design_notes
        .push(review_note(1));
    app.execute_command_key(
        crate::workbench::design::commands::DesignCommand::ReviewComments.key(),
    );
    let notes = app.state.model.schematic.document().design_notes.clone();
    let _ = ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(1280.0, 800.0),
            )),
            events: vec![egui::Event::Paste("Check the bias point".to_owned())],
            ..Default::default()
        },
        |ui| app.render_frame_dialogs(ui),
    );
    assert_eq!(
        app.state.design.design_review_comments.reply,
        "Check the bias point"
    );
    assert_eq!(app.state.model.schematic.document().design_notes, notes);
}

#[test]
fn initial_focus_rejects_unavailable_review_reply() {
    for case in ["read-only", "stale", "empty"] {
        let ctx = Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.options_mut(|options| options.max_passes = std::num::NonZeroUsize::new(1).unwrap());
        let mut app = RSpiceApp::test_instance();
        if case != "empty" {
            app.state
                .model
                .schematic
                .document_mut_for_test()
                .design_notes
                .push(review_note(1));
        }
        open_design_review_comments(
            &mut app.state.design.design_review_comments,
            &app.state.model,
            &app.state.platform,
        );
        if case == "read-only" {
            app.state.model.safe_mode.activate(
                crate::state::write_policy::LocalSafeModeOptions {
                    open_project_read_only: true,
                    ..Default::default()
                },
                "UI focus qualification".to_owned(),
            );
        }
        if case == "stale" {
            app.state
                .model
                .schematic
                .document_mut_for_test()
                .design_notes
                .push(review_note(2));
        }
        let notes = app.state.model.schematic.document().design_notes.clone();
        let _ = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(1280.0, 800.0),
                )),
                events: vec![egui::Event::Paste("Must not become a reply".to_owned())],
                ..Default::default()
            },
            |ui| app.render_frame_dialogs(ui),
        );
        assert!(
            app.state.design.design_review_comments.reply.is_empty(),
            "{case}"
        );
        assert!(!ctx.memory(|memory| memory.has_focus(reply_id())), "{case}");
        assert_eq!(app.state.model.schematic.document().design_notes, notes);
    }
}

/// New comment opens the note form with the caret already in its field,
/// so the reader types rather than hunting for where to.
///
/// The caret lands on the frame after the click, not the frame of it: a
/// command form asks for focus once its rows exist, and the click is what
/// builds them. One frame is nothing to a reader and everything to a test.
#[test]
fn opening_input_follows_a_new_comment_click_onto_the_next_frame() {
    fn render(app: &mut RSpiceApp, ctx: &Context, events: Vec<egui::Event>) -> egui::FullOutput {
        ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(1280.0, 800.0),
                )),
                events,
                ..Default::default()
            },
            |ui| app.render_frame_dialogs(ui),
        )
    }
    let ctx = Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    ctx.options_mut(|options| options.max_passes = std::num::NonZeroUsize::new(1).unwrap());
    let mut app = RSpiceApp::test_instance();
    open_design_review_comments(
        &mut app.state.design.design_review_comments,
        &app.state.model,
        &app.state.platform,
    );
    let _ = render(&mut app, &ctx, Vec::new());
    let output = render(&mut app, &ctx, Vec::new());
    let tree = output
        .platform_output
        .accesskit_update
        .expect("review controls");
    let button = tree
        .nodes
        .iter()
        .find(|(_, node)| {
            node.role() == egui::accesskit::Role::Button && node.label() == Some("New comment")
        })
        .expect("New comment button")
        .1
        .bounds()
        .unwrap();
    let pos = egui::pos2(
        ((button.x0 + button.x1) / 2.0) as f32,
        ((button.y0 + button.y1) / 2.0) as f32,
    );
    let _ = render(
        &mut app,
        &ctx,
        vec![
            egui::Event::PointerMoved(pos),
            egui::Event::PointerButton {
                pos,
                button: egui::PointerButton::Primary,
                pressed: true,
                modifiers: egui::Modifiers::NONE,
            },
        ],
    );
    // The release opens the note form and builds its rows. A command form
    // asks for the caret once its rows exist, so this is the frame before
    // the one that can be typed into.
    let _ = render(
        &mut app,
        &ctx,
        vec![egui::Event::PointerButton {
            pos,
            button: egui::PointerButton::Primary,
            pressed: false,
            modifiers: egui::Modifiers::NONE,
        }],
    );
    let _ = render(
        &mut app,
        &ctx,
        vec![
            egui::Event::Key {
                key: egui::Key::A,
                physical_key: Some(egui::Key::A),
                pressed: true,
                repeat: false,
                modifiers: egui::Modifiers::CTRL | egui::Modifiers::COMMAND,
            },
            egui::Event::Paste("Browser recovery qualification".to_owned()),
        ],
    );
    assert!(!app.state.design.design_review_comments.open);
    assert!(app.state.design.design_note.open);
    assert_eq!(
        app.state.design.design_note.text,
        "Browser recovery qualification"
    );
    assert!(app.state.model.schematic.document().design_notes.is_empty());
}
