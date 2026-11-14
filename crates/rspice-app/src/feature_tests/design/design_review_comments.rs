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

/// Review comments over the application's parts, as the dialog's step
/// lends them.
fn comments(app: &mut RSpiceApp) -> ReviewComments<'_> {
    let state = &mut app.state;
    ReviewComments {
        dialog: &mut state.design.design_review_comments,
        design_note: &mut state.design.design_note,
        model: &mut state.model,
        workbench: &mut state.workbench,
        cloud: &*state.cloud,
        session: &app.cloud,
    }
}

#[test]
fn failed_review_clock_retains_the_draft_and_durable_thread() {
    for action in [ReviewAction::Reply, ReviewAction::Resolve] {
        let mut app = RSpiceApp::test_instance();
        app.state
            .model
            .schematic
            .document_mut_for_test()
            .design_notes
            .push(review_note(1));
        open_design_review_comments(
            &mut app.state.design.design_review_comments,
            &app.state.model,
            &app.state.platform,
        );
        app.state.design.design_review_comments.reply = "Check the bias point".to_owned();
        let notes = app.state.model.schematic.document().design_notes.clone();
        let dirty = app.state.model.schematic.session.is_dirty;
        for epoch in [
            Err("clock unavailable"),
            Ok(std::time::Duration::ZERO),
            Ok(std::time::Duration::MAX),
        ] {
            crate::time_compat::with_unix_epoch(epoch, || {
                comments(&mut app).handle_design_review_action(action.clone())
            });
            assert_eq!(app.state.model.schematic.document().design_notes, notes);
            assert_eq!(app.state.model.schematic.session.is_dirty, dirty);
            assert_eq!(
                app.state.design.design_review_comments.reply,
                "Check the bias point"
            );
            assert!(
                app.state
                    .design
                    .design_review_comments
                    .error
                    .as_deref()
                    .unwrap()
                    .contains("timestamped")
            );
        }
        comments(&mut app).handle_design_review_action(action);
        assert!(app.state.design.design_review_comments.error.is_none());
        assert!(app.state.design.design_review_comments.reply.is_empty());
        assert_eq!(
            app.state.model.schematic.document().design_notes[0]
                .review
                .as_ref()
                .unwrap()
                .messages
                .len(),
            1
        );
    }
}

/// A reply is signed with the name the application knows the user by,
/// here the license holder, and never a placeholder role.
#[test]
fn a_posted_reply_is_signed_with_the_users_name() {
    let mut app = RSpiceApp::test_instance();
    let license = crate::services::license::parse_and_verify(crate::services::license::SAMPLE_KEY)
        .expect("signed sample license");
    let holder = license.licensed_to.clone();
    app.state.cloud.license = Some(license);
    app.state
        .model
        .schematic
        .document_mut_for_test()
        .design_notes
        .push(review_note(1));
    open_design_review_comments(
        &mut app.state.design.design_review_comments,
        &app.state.model,
        &app.state.platform,
    );
    app.state.design.design_review_comments.reply = "Bias point checked.".to_owned();

    comments(&mut app).handle_design_review_action(ReviewAction::Reply);

    let review = app.state.model.schematic.document().design_notes[0]
        .review
        .clone()
        .expect("a review thread");
    let author = &review.messages.last().expect("the posted reply").author;
    assert_eq!(author, &holder);
    assert_ne!(author, "Local project editor");
}

/// With no thread to show, the body says so in one block and the dialog
/// is as tall as that block, not a list pane and a detail pane both
/// empty; and the disabled Post reply says why.
#[test]
fn an_empty_review_is_one_block_and_post_reply_says_why() {
    let ctx = Context::default();
    ctx.enable_accesskit();
    crate::ui::Theme::default().apply(&ctx);
    let mut app = RSpiceApp::test_instance();
    open_design_review_comments(
        &mut app.state.design.design_review_comments,
        &app.state.model,
        &app.state.platform,
    );
    // Access updates are incremental: keep the latest of every node.
    let mut nodes = std::collections::HashMap::new();
    for _ in 0..4 {
        let output = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(1440.0, 900.0),
                )),
                ..egui::RawInput::default()
            },
            |ctx| comments(&mut app).render_design_review_comments_dialog(ctx),
        );
        if let Some(update) = output.platform_output.accesskit_update {
            nodes.extend(update.nodes);
        }
    }
    // The modal's own node is the whole screen. The surface is centred
    // on it, so twice its footer's distance below the centre bounds it.
    // The header's close button shares the footer button's name; the
    // footer's is the lower.
    let footer = nodes
        .values()
        .filter(|node| {
            node.role() == egui::accesskit::Role::Button && node.label() == Some("Close")
        })
        .filter_map(egui::accesskit::Node::bounds)
        .map(|bounds| bounds.y1)
        .fold(f64::MIN, f64::max);
    let height = 2.0 * (footer - 450.0);
    assert!(height < 320.0, "an empty review is about {height} pt tall");
    let says = |text: &str| {
        nodes.values().any(|node| {
            node.label().is_some_and(|label| label.contains(text))
                || node.value().is_some_and(|value| value.contains(text))
        })
    };
    assert!(says(NO_THREADS));
    assert!(says(NO_THREAD_SELECTED), "Post reply does not say why");
}

/// The filter selects show their whole choice. The revision scope was a
/// 220 pt select that read "Current revisio…carried forward".
#[test]
fn the_filter_selects_show_their_whole_choice() {
    let ctx = Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let mut app = RSpiceApp::test_instance();
    open_design_review_comments(
        &mut app.state.design.design_review_comments,
        &app.state.model,
        &app.state.platform,
    );
    let mut shapes = Vec::new();
    for _ in 0..3 {
        shapes = ctx
            .run_ui(
                egui::RawInput {
                    screen_rect: Some(egui::Rect::from_min_size(
                        egui::Pos2::ZERO,
                        egui::vec2(1440.0, 900.0),
                    )),
                    ..egui::RawInput::default()
                },
                |ctx| comments(&mut app).render_design_review_comments_dialog(ctx),
            )
            .shapes;
    }
    let painted = shapes
        .iter()
        .filter_map(|clipped| match &clipped.shape {
            egui::Shape::Text(text) => Some(text.galley.text().to_owned()),
            _ => None,
        })
        .collect::<Vec<_>>();
    let scope = DesignReviewRevisionScope::CurrentAndCarried.label();
    assert!(
        painted.iter().any(|text| text == scope),
        "the scope select does not read {scope:?}: {painted:?}"
    );
}
