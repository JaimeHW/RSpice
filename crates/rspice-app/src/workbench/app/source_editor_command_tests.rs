//! The application's commands acting on the source editor in front of the
//! reader: the Edit menu, the shortcuts and the palette reach the text rather
//! than a schematic hidden behind it, and no page carries a menu of its own.

use super::veriloga_design_tests::{
    DESKTOP, add_veriloga_cell, frame, open_in_design, open_project_app, themed_context,
};
use super::*;
use crate::state::{Point, ProjectSourceId, ProjectSourceLanguage, Wire};
use crate::workbench::commands::vocabulary::Command;
use crate::workbench::documents::code_workspace::{self, CodeWorkspacePage};
use crate::workbench::state::Workspace;

const FIRST_LINE: &str = "// Behavioral conductance.";

fn root_text(app: &RSpiceApp, bundle_id: ProjectSourceId) -> String {
    app.state
        .workspace
        .content
        .project_sources
        .get_bundle(bundle_id)
        .expect("the bundle")
        .root()
        .content()
        .to_owned()
}

fn frame_with(
    ctx: &egui::Context,
    app: &mut RSpiceApp,
    events: Vec<egui::Event>,
) -> egui::FullOutput {
    ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, DESKTOP)),
            events,
            ..Default::default()
        },
        |ui| {
            app.prepare_frame(ui.ctx());
            app.render_frame_chrome(ui);
            app.render_frame_dialogs(ui.ctx());
        },
    )
}

fn key(key: egui::Key, modifiers: egui::Modifiers) -> egui::Event {
    egui::Event::Key {
        key,
        physical_key: Some(key),
        pressed: true,
        repeat: false,
        modifiers,
    }
}

/// Whether the frame exposed a node named `label`; a custom editor states its
/// accessible name as its value.
fn has_label(output: &egui::FullOutput, label: &str) -> bool {
    output
        .platform_output
        .accesskit_update
        .as_ref()
        .is_some_and(|update| {
            update
                .nodes
                .iter()
                .any(|(_, node)| node.label() == Some(label) || node.value() == Some(label))
        })
}

/// A Verilog-A cell view open in Design with the first line of its source
/// selected in a focused editor, and a wire selected on the schematic the
/// Design workspace keeps behind it.
fn veriloga_with_selected_first_line() -> (egui::Context, RSpiceApp, ProjectSourceId) {
    let mut app = open_project_app();
    let (reference, bundle_id) = add_veriloga_cell(&mut app.state, "gain", "model.va", "gain");
    open_in_design(&mut app.state, &reference);
    let ctx = themed_context();
    frame(&ctx, &mut app, DESKTOP);
    frame(&ctx, &mut app, DESKTOP);
    app.state
        .schematic
        .document_mut_for_test()
        .wires
        .push(Wire::segment(7, Point::new(0, 0), Point::new(100, 0)));
    app.state.schematic.session.editor.selection.select_wire(7);
    let editor = code_workspace::source_editor_id(ProjectSourceLanguage::VerilogA, bundle_id, "model.va");
    ctx.memory_mut(|memory| memory.request_focus(editor));
    frame(&ctx, &mut app, DESKTOP);
    frame_with(
        &ctx,
        &mut app,
        vec![key(egui::Key::End, egui::Modifiers::SHIFT)],
    );
    (ctx, app, bundle_id)
}

#[test]
fn no_source_page_carries_an_editor_menu_of_its_own() {
    // Verilog-A, in Design.
    let mut app = open_project_app();
    let (reference, _) = add_veriloga_cell(&mut app.state, "gain", "model.va", "gain");
    open_in_design(&mut app.state, &reference);
    let ctx = themed_context();
    frame(&ctx, &mut app, DESKTOP);
    let output = frame(&ctx, &mut app, DESKTOP);
    assert!(
        has_label(&output, "Verilog-A source editor"),
        "the Verilog-A page was drawn"
    );
    assert!(!has_label(&output, "Editor"), "Verilog-A page");

    // The netlist and automation pages of the Netlist workspace, each known
    // to be drawn by a control it does keep. The project's own automation
    // bundle is what the automation page shows.
    for (page, drawn) in [
        (
            CodeWorkspacePage::Netlist,
            &["Find in active netlist document"][..],
        ),
        (
            CodeWorkspacePage::Automation,
            &["Python source editor", "RSpice Automation source editor"][..],
        ),
    ] {
        let mut app = open_project_app();
        app.state.workbench.activate(Workspace::Netlist);
        app.state.ui.code_workspace.page = page;
        let ctx = themed_context();
        frame(&ctx, &mut app, DESKTOP);
        let output = frame(&ctx, &mut app, DESKTOP);
        assert!(
            drawn.iter().any(|label| has_label(&output, label)),
            "the {page:?} page was drawn"
        );
        assert!(!has_label(&output, "Editor"), "{page:?} page");
    }
}

#[test]
fn edit_cut_cuts_the_source_selection_and_leaves_the_schematic_alone() {
    let (ctx, mut app, bundle_id) = veriloga_with_selected_first_line();
    let original = root_text(&app, bundle_id);
    assert!(original.starts_with(FIRST_LINE));

    assert!(
        Command::Cut.is_enabled(&app),
        "Cut is offered for the source editor's selection"
    );
    Command::Cut.execute(&mut app);
    let output = frame(&ctx, &mut app, DESKTOP);

    assert_eq!(
        root_text(&app, bundle_id),
        original.replacen(FIRST_LINE, "", 1),
        "Edit > Cut removed the selected source text"
    );
    assert!(
        output.platform_output.commands.iter().any(|command| matches!(
            command,
            egui::OutputCommand::CopyText(text) if text == FIRST_LINE
        )),
        "the cut text went to the clipboard"
    );
    assert!(
        app.state.schematic.session.editor.selection.has_wire(7),
        "the schematic selection behind the source page is untouched"
    );
    assert!(
        app.state
            .schematic
            .document()
            .wires
            .iter()
            .any(|wire| wire.id == 7),
        "the schematic behind the source page lost nothing"
    );

    Command::Undo.execute(&mut app);
    frame(&ctx, &mut app, DESKTOP);
    assert_eq!(
        root_text(&app, bundle_id),
        original,
        "Edit > Undo after the cut restored the text"
    );
}

#[test]
fn edit_select_all_then_copy_reach_the_source_editor() {
    let (ctx, mut app, bundle_id) = veriloga_with_selected_first_line();
    let original = root_text(&app, bundle_id);

    Command::SelectAll.execute(&mut app);
    frame(&ctx, &mut app, DESKTOP);
    Command::Copy.execute(&mut app);
    let output = frame(&ctx, &mut app, DESKTOP);

    assert!(
        output.platform_output.commands.iter().any(|command| matches!(
            command,
            egui::OutputCommand::CopyText(text) if *text == original
        )),
        "Select all then Copy put the whole source on the clipboard"
    );
    assert!(
        app.state.schematic.session.editor.clipboard.is_empty(),
        "Copy did not copy the schematic selection"
    );
    assert_eq!(root_text(&app, bundle_id), original);
}

/// The Verilog-A source with its caret on the first line, in a focused
/// editor.
fn veriloga_with_focused_editor() -> (egui::Context, RSpiceApp, ProjectSourceId) {
    let mut app = open_project_app();
    let (reference, bundle_id) = add_veriloga_cell(&mut app.state, "gain", "model.va", "gain");
    open_in_design(&mut app.state, &reference);
    let ctx = themed_context();
    frame(&ctx, &mut app, DESKTOP);
    frame(&ctx, &mut app, DESKTOP);
    let editor =
        code_workspace::source_editor_id(ProjectSourceLanguage::VerilogA, bundle_id, "model.va");
    ctx.memory_mut(|memory| memory.request_focus(editor));
    frame(&ctx, &mut app, DESKTOP);
    (ctx, app, bundle_id)
}

#[test]
fn ctrl_d_in_a_focused_source_editor_duplicates_the_caret_line_once() {
    let (ctx, mut app, bundle_id) = veriloga_with_focused_editor();
    let original = root_text(&app, bundle_id);
    frame_with(
        &ctx,
        &mut app,
        vec![key(
            egui::Key::D,
            egui::Modifiers::CTRL | egui::Modifiers::COMMAND,
        )],
    );
    frame(&ctx, &mut app, DESKTOP);
    assert_eq!(
        root_text(&app, bundle_id),
        format!("{FIRST_LINE}\n{original}"),
        "Ctrl+D duplicated the caret's line, exactly once"
    );
}

#[test]
fn edit_toggle_line_comment_comments_the_caret_line_and_source_items_hide_elsewhere() {
    let (ctx, mut app, bundle_id) = veriloga_with_focused_editor();
    let original = root_text(&app, bundle_id);
    let second_line = original.lines().nth(1).expect("a second line").to_owned();
    frame_with(
        &ctx,
        &mut app,
        vec![key(egui::Key::ArrowDown, egui::Modifiers::NONE)],
    );

    assert!(Command::ToggleLineComment.availability(&app).is_available());
    Command::ToggleLineComment.execute(&mut app);
    frame(&ctx, &mut app, DESKTOP);
    assert_eq!(
        root_text(&app, bundle_id),
        original.replacen(&second_line, &format!("// {second_line}"), 1),
        "Toggle line comment commented the caret's line"
    );

    // On a drawing there is no source editor: the source commands are not
    // offered at all, rather than offered disabled.
    app.state.workbench.activate(Workspace::Project);
    frame(&ctx, &mut app, DESKTOP);
    for command in [
        Command::ToggleLineComment,
        Command::DuplicateLine,
        Command::GoToLine,
        Command::FoldAll,
    ] {
        assert_eq!(
            command.availability(&app),
            crate::workbench::commands::CommandAvailability::Hidden,
            "{command:?} off a source page"
        );
    }
}

/// The palette row's shortcut, as a screen reader is told it.
fn palette_shortcut(output: &egui::FullOutput, label: &str) -> Option<String> {
    output
        .platform_output
        .accesskit_update
        .as_ref()?
        .nodes
        .iter()
        .find(|(_, node)| {
            node.role() == egui::accesskit::Role::ListBoxOption && node.label() == Some(label)
        })
        .and_then(|(_, node)| node.keyboard_shortcut().map(str::to_owned))
}

#[test]
fn the_palette_lists_toggle_line_comment_with_the_platform_shortcut() {
    for (os, expected) in [
        (egui::os::OperatingSystem::Windows, "Ctrl+/"),
        (egui::os::OperatingSystem::Mac, "Cmd+/"),
    ] {
        let (ctx, mut app, _) = veriloga_with_focused_editor();
        ctx.set_os(os);
        Command::CommandPalette.execute(&mut app);
        app.state.dialogs.command_palette.query = "Toggle line comment".to_owned();
        let mut output = frame(&ctx, &mut app, DESKTOP);
        for _ in 0..6 {
            output = frame(&ctx, &mut app, DESKTOP);
        }
        assert_eq!(
            palette_shortcut(&output, "Toggle line comment").as_deref(),
            Some(expected),
            "{os:?}"
        );
    }
}
