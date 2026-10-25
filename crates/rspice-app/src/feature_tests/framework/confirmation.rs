//! The confirmation dialog on its narrow surface: the close-project review and
//! the save prompt each ask one question, stack their verbs on a phone and paint
//! no fill band, and a review raised during a run offers Stop and no save verbs.

use crate::workbench::RSpiceApp;
use crate::workbench::app::dialogs::confirmation::*;
use crate::workbench::lifecycle::project_lifecycle::ProjectCloseDestination;
use egui::Context;

fn render_close_review(app: &mut RSpiceApp, size: egui::Vec2) -> egui::FullOutput {
    let ctx = Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let mut output = None;
    for _ in 0..3 {
        output = Some(ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, size)),
                ..Default::default()
            },
            |ui| render_confirmation_dialog(app, ui.ctx()),
        ));
    }
    output.expect("three passes ran")
}

fn visit_shapes(shape: &egui::epaint::Shape, visit: &mut impl FnMut(&egui::epaint::Shape)) {
    if let egui::epaint::Shape::Vec(shapes) = shape {
        for shape in shapes {
            visit_shapes(shape, visit);
        }
    } else {
        visit(shape);
    }
}

fn painted_text(output: &egui::FullOutput) -> Vec<(String, egui::Rect)> {
    let mut painted = Vec::new();
    for clipped in &output.shapes {
        visit_shapes(&clipped.shape, &mut |shape| {
            if let egui::epaint::Shape::Text(text) = shape {
                painted.push((
                    text.galley.text().to_owned(),
                    egui::Rect::from_min_size(text.pos, text.galley.size()),
                ));
            }
        });
    }
    painted
}

fn find(painted: &[(String, egui::Rect)], wanted: &str) -> egui::Rect {
    painted
        .iter()
        .find(|(text, _)| text == wanted)
        .map(|(_, rect)| *rect)
        .unwrap_or_else(|| panic!("{wanted:?} was not painted: {painted:?}"))
}

/// A never-saved project named "lna-frontend", asked to close.
fn unsaved_project_closing() -> RSpiceApp {
    let mut app = RSpiceApp::test_instance();
    app.state
        .model
        .workspace
        .content
        .project
        .rename("lna-frontend")
        .expect("valid project name");
    app.state
        .workbench
        .project_review_dialog
        .show_close_project(ProjectCloseDestination::EmptyWorkbench);
    app
}

/// A save prompt sits on the narrow confirmation surface with its
/// question, one sentence, and "Don't save" set apart from Cancel and the
/// primary on a single footer row.
fn assert_narrow_save_prompt(painted: &[(String, egui::Rect)], title: &str, sentence: &str) {
    let screen = egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(1_280.0, 800.0));
    let surface = egui::Rect::from_center_size(screen.center(), egui::vec2(480.0, screen.height()));
    let [dont_save, cancel, save] =
        ["Don't save", "Cancel", "Save"].map(|verb| find(painted, verb));
    for rect in [
        find(painted, title),
        find(painted, sentence),
        dont_save,
        cancel,
        save,
    ] {
        assert!(surface.contains_rect(rect), "{rect:?} escapes {surface:?}");
    }
    for rect in [cancel, save] {
        assert!(
            (rect.center().y - dont_save.center().y).abs() < 1.0,
            "the verbs share one footer row"
        );
    }
    assert!(dont_save.right() < surface.center().x, "Don't save leads");
    assert!(
        surface.center().x < cancel.left() && cancel.right() < save.left(),
        "Cancel, then the primary, trail"
    );
}

/// Closing a never-saved project asks the standard save-changes question
/// and says the whole project is unsaved, rather than naming the
/// configuration record that stands in for it.
#[test]
fn close_project_review_renders_on_the_narrow_confirmation_surface() {
    let painted = painted_text(&render_close_review(
        &mut unsaved_project_closing(),
        egui::vec2(1_280.0, 800.0),
    ));
    assert_narrow_save_prompt(
        &painted,
        "Save changes to \u{201c}lna-frontend\u{201d}?",
        "This project has never been saved.",
    );
    for retired in [
        "SAFE SHUTDOWN",
        "Local simulation",
        "Unsaved:",
        "Project configuration",
    ] {
        assert!(
            !painted.iter().any(|(text, _)| text.contains(retired)),
            "{retired:?} painted"
        );
    }
}

/// The save prompt in front of a destructive action is the same narrow
/// question: no kicker, one sentence, and "Don't save" set apart.
#[test]
fn save_prompt_asks_one_question_on_the_narrow_confirmation_surface() {
    let mut app = RSpiceApp::test_instance();
    app.state
        .model
        .workspace
        .content
        .project
        .rename("lna-frontend")
        .expect("valid project name");
    app.state.workbench.confirmation_dialog.visible = true;
    app.state.workbench.confirmation_dialog.pending_action =
        Some(crate::workbench::app::ConfirmationAction::ProjectOpen);

    let painted = painted_text(&render_close_review(&mut app, egui::vec2(1_280.0, 800.0)));

    assert_narrow_save_prompt(
        &painted,
        "Save changes to \u{201c}lna-frontend\u{201d}?",
        "Your changes will be lost if you don't save them.",
    );
    for retired in [
        "SCHEMATIC",
        "SAFE EXIT",
        "Save / retain",
        "Do you want to save",
    ] {
        assert!(
            !painted.iter().any(|(text, _)| text.contains(retired)),
            "{retired:?} painted"
        );
    }
}

/// On a phone the verbs stack full-width, primary first.
#[test]
fn close_project_review_stacks_its_verbs_on_a_phone() {
    let painted = painted_text(&render_close_review(
        &mut unsaved_project_closing(),
        egui::vec2(390.0, 844.0),
    ));
    let [save, dont_save, cancel] =
        ["Save", "Don't save", "Cancel"].map(|verb| find(&painted, verb));
    assert!(
        save.center().y < dont_save.center().y && dont_save.center().y < cancel.center().y,
        "stacked primary, Don't save, Cancel: {save:?} {dont_save:?} {cancel:?}"
    );
    for rect in [save, dont_save, cancel] {
        assert!(
            (rect.center().x - 195.0).abs() < 1.5,
            "a full-width verb centres its label on the card: {rect:?}"
        );
    }
}

/// The body is one surface. Its frame used to repeat the surface fill
/// over a rect as wide as the text, which showed as a band while the modal
/// faded in; nothing between the header and the footer paints a fill now.
#[test]
fn close_project_review_body_paints_no_fill_band() {
    let output = render_close_review(&mut unsaved_project_closing(), egui::vec2(1_280.0, 800.0));
    let painted = painted_text(&output);
    let summary = find(&painted, "This project has never been saved.");
    let body = egui::Rect::from_min_max(
        egui::pos2(0.0, summary.top() - 20.0),
        egui::pos2(1_280.0, summary.bottom() + 20.0),
    );
    let mut bands = Vec::new();
    for clipped in &output.shapes {
        visit_shapes(&clipped.shape, &mut |shape| {
            if let egui::epaint::Shape::Rect(rect) = shape
                && rect.fill != egui::Color32::TRANSPARENT
                && rect.rect.width() > 24.0
                && body.contains_rect(rect.rect)
            {
                bands.push(rect.rect);
            }
        });
    }
    assert!(
        bands.is_empty(),
        "the body painted a fill of its own: {bands:?}"
    );
}

/// While a run owns the project the prompt asks for it to stop, and
/// offers nothing that would save or discard.
#[test]
fn close_project_review_during_a_run_offers_stop_and_no_save_verbs() {
    let mut app = unsaved_project_closing();
    let run = app.state.model.simulation.start_run();
    run.mark_running().expect("fixture enters running state");
    let identity = run
        .execution_identity()
        .expect("running fixture has an identity");
    app.state.model.simulation.execution.active_execution = Some(identity);

    let painted = painted_text(&render_close_review(&mut app, egui::vec2(1_280.0, 800.0)));
    let shows = |wanted: &str| painted.iter().any(|(text, _)| text == wanted);

    for expected in [
        "Stop the simulation to close this project",
        "Stop simulation",
        "Cancel",
    ] {
        assert!(shows(expected), "{expected:?} missing: {painted:?}");
    }
    for verb in ["Save", "Save all", "Don't save"] {
        assert!(!shows(verb), "{verb:?} offered during a run");
    }
}
