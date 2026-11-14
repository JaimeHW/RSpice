//! What the browser's accessibility mirror reads from whole application
//! frames: the editor's caret line, dialog fields, and the live regions that
//! carry a compile's result and the caret's arrival on a line.

use super::veriloga_design_tests::{
    DESKTOP, add_veriloga_cell, frame, frame_with, open_in_design, open_project_app, themed_context,
};
use super::*;
use crate::workbench::browser::accessible_mirror::{AccessibleProjection, MirrorState, project};
use crate::workbench::documents::virtual_text_editor::speak_caret_lines;
use egui::accesskit::Role;

const FIRST_LINE: &str = "// Behavioral conductance.";
const SECOND_LINE: &str = "module gain(p, n);";

fn projection(output: &egui::FullOutput) -> AccessibleProjection {
    project(
        output
            .platform_output
            .accesskit_update
            .as_ref()
            .expect("AccessKit is on"),
    )
}

/// A press and its release: egui reads a second press with no release between
/// as the key repeating.
fn key(key: egui::Key, modifiers: egui::Modifiers) -> Vec<egui::Event> {
    [true, false]
        .map(|pressed| egui::Event::Key {
            key,
            physical_key: Some(key),
            pressed,
            repeat: false,
            modifiers,
        })
        .to_vec()
}

/// A Verilog-A document with the keyboard in its source editor.
fn focused_editor() -> (egui::Context, RSpiceApp) {
    let mut app = open_project_app();
    let (reference, _) = add_veriloga_cell(&mut app.state, "gain", "model.va", "gain");
    open_in_design(&mut app.state, &reference);
    let ctx = themed_context();
    frame(&ctx, &mut app, DESKTOP);
    frame(&ctx, &mut app, DESKTOP);
    let editor = app
        .state
        .workbench
        .active_source_editor
        .expect("the document is a source editor")
        .id;
    ctx.memory_mut(|memory| memory.request_focus(editor));
    frame(&ctx, &mut app, DESKTOP);
    frame(&ctx, &mut app, DESKTOP);
    (ctx, app)
}

fn messages(app: &RSpiceApp) -> rspice_app_workbench::workbench::localization::MessageCatalog {
    app.state.workbench.messages()
}

#[test]
fn the_editor_s_current_line_is_read_from_the_tree() {
    let (ctx, mut app) = focused_editor();
    let output = frame(&ctx, &mut app, DESKTOP);
    let seen = projection(&output);
    let (_, focused) = seen.focused.expect("the editor holds the keyboard");
    assert_eq!(focused.role, Role::MultilineTextInput);
    assert!(
        focused.description.starts_with("Line 1 of"),
        "the position travels with it: {}",
        focused.description
    );
    assert_eq!(
        seen.caret_line
            .expect("a text document has a caret line")
            .text,
        FIRST_LINE
    );
}

#[test]
fn arriving_on_a_line_speaks_it_and_typing_on_it_does_not() {
    let (ctx, mut app) = focused_editor();
    speak_caret_lines(&ctx);
    let messages = messages(&app);
    let mut mirror = MirrorState::default();
    mirror.patch(projection(&frame(&ctx, &mut app, DESKTOP)), messages);

    frame_with(
        &ctx,
        &mut app,
        DESKTOP,
        key(egui::Key::ArrowDown, egui::Modifiers::NONE),
    );
    let moved = mirror.patch(projection(&frame(&ctx, &mut app, DESKTOP)), messages);
    assert_eq!(moved.announcement.as_deref(), Some(SECOND_LINE));

    let typed = frame_with(
        &ctx,
        &mut app,
        DESKTOP,
        vec![egui::Event::Text("x".to_owned())],
    );
    let after_typing = mirror.patch(projection(&typed), messages);
    assert_eq!(after_typing.announcement, None);

    let settled = frame(&ctx, &mut app, DESKTOP);
    let idle = mirror.patch(projection(&settled), messages);
    assert_eq!(idle.announcement, None);
}

#[test]
fn two_lines_reading_the_same_are_two_arrivals() {
    // A live node is told apart by its identity, so arriving on a second
    // blank line is still something to say.
    let (ctx, mut app) = focused_editor();
    speak_caret_lines(&ctx);
    let messages = messages(&app);
    let mut mirror = MirrorState::default();
    mirror.patch(projection(&frame(&ctx, &mut app, DESKTOP)), messages);
    for _ in 0..2 {
        frame_with(
            &ctx,
            &mut app,
            DESKTOP,
            key(egui::Key::Enter, egui::Modifiers::NONE),
        );
        frame(&ctx, &mut app, DESKTOP);
    }
    // The two returns left two blank lines above the caret.
    for _ in 0..2 {
        frame_with(
            &ctx,
            &mut app,
            DESKTOP,
            key(egui::Key::ArrowUp, egui::Modifiers::NONE),
        );
        let up = mirror.patch(projection(&frame(&ctx, &mut app, DESKTOP)), messages);
        assert_eq!(up.announcement.as_deref(), Some("blank"));
    }
}

#[test]
fn a_compile_result_reaches_the_live_region_even_when_it_reads_the_same() {
    let (ctx, mut app) = focused_editor();
    let messages = messages(&app);
    let mut mirror = MirrorState::default();
    mirror.patch(projection(&frame(&ctx, &mut app, DESKTOP)), messages);

    let result = "Compiled gain: 2 ports, 1 parameter";
    app.state.workbench.prompt(result);
    let first = mirror.patch(projection(&frame(&ctx, &mut app, DESKTOP)), messages);
    assert_eq!(first.announcement.as_deref(), Some(result));

    let idle = mirror.patch(projection(&frame(&ctx, &mut app, DESKTOP)), messages);
    assert!(idle.is_empty(), "nothing changed: {idle:?}");

    // The same result some time later is a new answer.
    let prompt = app.state.workbench.prompt.as_mut().expect("still shown");
    prompt.issued = prompt
        .issued
        .checked_sub(std::time::Duration::from_secs(1))
        .expect("the clock is past a second");
    app.state.workbench.prompt(result);
    let again = mirror.patch(projection(&frame(&ctx, &mut app, DESKTOP)), messages);
    assert_eq!(
        again
            .announcement
            .map(|text| text.trim().to_owned())
            .as_deref(),
        Some(result)
    );
}

#[test]
fn a_dialog_s_field_names_and_values_are_mirrored() {
    let mut app = open_project_app();
    crate::workbench::app::project_transactions::open_new_project_dialog(&mut app.state);
    let ctx = themed_context();
    for _ in 0..3 {
        frame(&ctx, &mut app, DESKTOP);
    }
    let seen = projection(&frame(&ctx, &mut app, DESKTOP));
    let dialog = seen.dialog.expect("the dialog is open");
    assert_eq!(dialog.title, "Create RSpice project");
    let inputs: Vec<(&str, &str)> = dialog
        .fields
        .iter()
        .filter(|field| field.role == Role::TextInput)
        .map(|field| (field.name.as_str(), field.value.as_str()))
        .collect();
    assert_eq!(
        inputs,
        vec![
            ("Project name", ""),
            ("Library", "user"),
            ("Top cell", "top")
        ]
    );
    let create = dialog
        .fields
        .iter()
        .find(|field| field.name == "Create project")
        .expect("the dialog's confirm button");
    assert!(create.disabled, "no name has been typed yet");
    assert!(
        !dialog.fields.iter().any(|field| field.name == dialog.title),
        "the heading is the title, not a field"
    );
}

#[test]
fn an_unchanged_frame_writes_nothing_to_the_page() {
    let (ctx, mut app) = focused_editor();
    let messages = messages(&app);
    let mut mirror = MirrorState::default();
    mirror.patch(projection(&frame(&ctx, &mut app, DESKTOP)), messages);
    for _ in 0..5 {
        let output = frame(&ctx, &mut app, DESKTOP);
        assert!(mirror.patch(projection(&output), messages).is_empty());
    }
}
