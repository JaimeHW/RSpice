//! The application's commands acting on the source editor in front of the
//! reader: the Edit menu, the shortcuts and the palette reach the text rather
//! than a schematic hidden behind it, and no page carries a menu of its own.

use super::veriloga_design_tests::{
    DESKTOP, add_veriloga_cell, frame, frame_with, open_in_design, open_project_app, themed_context,
};
use super::*;
use crate::state::{Point, ProjectSourceId, ProjectSourceLanguage, Wire};
use crate::workbench::code::commands::CodeCommand;
use crate::workbench::commands::vocabulary::Command;
use crate::workbench::documents::code_workspace::{self, CodeWorkspacePage};
use crate::workbench::state::Workspace;

const FIRST_LINE: &str = "// Behavioral conductance.";

fn root_text(app: &RSpiceApp, bundle_id: ProjectSourceId) -> String {
    app.state
        .model
        .workspace
        .content
        .project_sources
        .get_bundle(bundle_id)
        .expect("the bundle")
        .root()
        .content()
        .to_owned()
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
        .model
        .schematic
        .document_mut_for_test()
        .wires
        .push(Wire::segment(7, Point::new(0, 0), Point::new(100, 0)));
    app.state
        .model
        .schematic
        .session
        .editor
        .selection
        .select_wire(7);
    let editor =
        code_workspace::source_editor_id(ProjectSourceLanguage::VerilogA, bundle_id, "model.va");
    ctx.memory_mut(|memory| memory.request_focus(editor));
    frame(&ctx, &mut app, DESKTOP);
    frame_with(
        &ctx,
        &mut app,
        DESKTOP,
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
        has_label(&output, "Verilog-A/AMS source editor"),
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
        app.state.code.code_workspace.page = page;
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
        output
            .platform_output
            .commands
            .iter()
            .any(|command| matches!(
                command,
                egui::OutputCommand::CopyText(text) if text == FIRST_LINE
            )),
        "the cut text went to the clipboard"
    );
    assert!(
        app.state
            .model
            .schematic
            .session
            .editor
            .selection
            .has_wire(7),
        "the schematic selection behind the source page is untouched"
    );
    assert!(
        app.state
            .model
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
        output
            .platform_output
            .commands
            .iter()
            .any(|command| matches!(
                command,
                egui::OutputCommand::CopyText(text) if *text == original
            )),
        "Select all then Copy put the whole source on the clipboard"
    );
    assert!(
        app.state.design.schematic_clipboard.is_empty(),
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
        DESKTOP,
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
        DESKTOP,
        vec![key(egui::Key::ArrowDown, egui::Modifiers::NONE)],
    );

    assert!(
        app.command_availability(CodeCommand::ToggleLineComment.key())
            .is_available()
    );
    app.execute_command_key(CodeCommand::ToggleLineComment.key());
    frame(&ctx, &mut app, DESKTOP);
    assert_eq!(
        root_text(&app, bundle_id),
        original.replacen(&second_line, &format!("// {second_line}"), 1),
        "Toggle line comment commented the caret's line"
    );

    // Edit's Find is the source's own find, with the key that opens it from
    // the focused editor; the design's find is not offered over a source.
    let output = frame(&ctx, &mut app, DESKTOP);
    let (trigger, _) = menu_item(&output, "Edit").expect("the Edit menu");
    press(
        &ctx,
        &mut app,
        trigger.center(),
        egui::PointerButton::Primary,
    );
    let mut output = frame(&ctx, &mut app, DESKTOP);
    for _ in 0..3 {
        output = frame(&ctx, &mut app, DESKTOP);
    }
    let find = CodeCommand::FindCodeDocument.declared_spec().label;
    assert_eq!(menu_shortcut(&output, find).as_deref(), Some("Ctrl+F"));
    assert!(menu_item(&output, "Find in design…").is_none());
    frame_with(
        &ctx,
        &mut app,
        DESKTOP,
        vec![key(egui::Key::Escape, egui::Modifiers::NONE)],
    );
    let editor =
        code_workspace::source_editor_id(ProjectSourceLanguage::VerilogA, bundle_id, "model.va");
    ctx.memory_mut(|memory| memory.request_focus(editor));
    frame(&ctx, &mut app, DESKTOP);
    assert!(app.state.code.code_workspace.source_search.is_none());
    frame_with(
        &ctx,
        &mut app,
        DESKTOP,
        vec![key(
            egui::Key::F,
            egui::Modifiers::CTRL | egui::Modifiers::COMMAND,
        )],
    );
    frame(&ctx, &mut app, DESKTOP);
    assert!(
        app.state.code.code_workspace.source_search.is_some(),
        "Ctrl+F in the focused editor opened the source's find"
    );

    // On a drawing there is no source editor: the source commands are not
    // offered at all, rather than offered disabled.
    app.state.workbench.activate(Workspace::Project);
    frame(&ctx, &mut app, DESKTOP);
    for command in [
        CodeCommand::ToggleLineComment,
        CodeCommand::DuplicateLine,
        CodeCommand::GoToLine,
        CodeCommand::FoldAll,
    ] {
        assert_eq!(
            app.command_availability(command.key()),
            crate::workbench::framework::command_registry::CommandAvailability::Hidden,
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
        (egui::os::OperatingSystem::Mac, "\u{2318}/"),
    ] {
        let (ctx, mut app, _) = veriloga_with_focused_editor();
        ctx.set_os(os);
        Command::CommandPalette.execute(&mut app);
        app.state.workbench.command_palette.query = "Toggle line comment".to_owned();
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

/// A module whose last statement continues onto a line that starts with a
/// use of the parameter `g`, so a press anywhere left of the code on that
/// line puts the caret on it.
const NAVIGABLE_MODULE: &str = "module gain(p, n);\n  inout p, n;\n  electrical p, n;\n  parameter real g = 1m;\n  analog I(p, n) <+\ng * V(p, n);\nendmodule\n";

/// The Verilog-A editor's name, as a screen reader is told it.
const VERILOGA_EDITOR: &str = "Verilog-A/AMS source editor";

/// The Automation page's editor's name, on a Python file.
const PYTHON_EDITOR: &str = "Python source editor";

/// Its name on a Python file it may not write.
const PYTHON_VIEWER: &str = "Python source viewer";

/// The top-left corner of the editor named `name`, as a screen reader is
/// told it.
fn named_editor_origin(output: &egui::FullOutput, name: &str) -> egui::Pos2 {
    output
        .platform_output
        .accesskit_update
        .as_ref()
        .and_then(|update| {
            update
                .nodes
                .iter()
                .find(|(_, node)| node.value() == Some(name) || node.label() == Some(name))
        })
        .and_then(|(_, node)| node.bounds())
        .map(|bounds| egui::pos2(bounds.x0 as f32, bounds.y0 as f32))
        .unwrap_or_else(|| panic!("the {name} is on screen"))
}

/// Where the gutter of the one-based `line` is: a press there lands on the
/// line's first column.
fn gutter_of_line(origin: egui::Pos2, line: usize) -> egui::Pos2 {
    // The code document's top padding and line height.
    egui::pos2(origin.x + 6.0, origin.y + 8.0 + 17.05 * (line as f32 - 0.5))
}

fn menu_item(output: &egui::FullOutput, label: &str) -> Option<(egui::Rect, bool)> {
    output
        .platform_output
        .accesskit_update
        .as_ref()?
        .nodes
        .iter()
        .find(|(_, node)| {
            node.role() == egui::accesskit::Role::MenuItem && node.label() == Some(label)
        })
        .and_then(|(_, node)| {
            node.bounds().map(|bounds| {
                (
                    egui::Rect::from_min_max(
                        egui::pos2(bounds.x0 as f32, bounds.y0 as f32),
                        egui::pos2(bounds.x1 as f32, bounds.y1 as f32),
                    ),
                    !node.is_disabled(),
                )
            })
        })
}

/// The menu row's shortcut, as a screen reader is told it.
fn menu_shortcut(output: &egui::FullOutput, label: &str) -> Option<String> {
    output
        .platform_output
        .accesskit_update
        .as_ref()?
        .nodes
        .iter()
        .find(|(_, node)| {
            node.role() == egui::accesskit::Role::MenuItem && node.label() == Some(label)
        })
        .and_then(|(_, node)| node.keyboard_shortcut().map(str::to_owned))
}

fn press(
    ctx: &egui::Context,
    app: &mut RSpiceApp,
    at: egui::Pos2,
    button: egui::PointerButton,
) -> egui::FullOutput {
    frame_with(ctx, app, DESKTOP, vec![egui::Event::PointerMoved(at)]);
    let mut output = None;
    for pressed in [true, false] {
        output = Some(frame_with(
            ctx,
            app,
            DESKTOP,
            vec![egui::Event::PointerButton {
                pos: at,
                button,
                pressed,
                modifiers: egui::Modifiers::NONE,
            }],
        ));
    }
    output.expect("a frame")
}

/// Right-click the gutter of the Verilog-A source's one-based `line` and let
/// the menu settle.
fn right_click_line(ctx: &egui::Context, app: &mut RSpiceApp, line: usize) -> egui::FullOutput {
    right_click_line_of(ctx, app, VERILOGA_EDITOR, line)
}

/// Right-click the gutter of the one-based `line` of the editor named
/// `name` and let the menu settle.
fn right_click_line_of(
    ctx: &egui::Context,
    app: &mut RSpiceApp,
    name: &str,
    line: usize,
) -> egui::FullOutput {
    let origin = named_editor_origin(&frame(ctx, app, DESKTOP), name);
    press(
        ctx,
        app,
        gutter_of_line(origin, line),
        egui::PointerButton::Secondary,
    );
    let mut output = frame(ctx, app, DESKTOP);
    for _ in 0..4 {
        output = frame(ctx, app, DESKTOP);
    }
    output
}

const CONTEXT_MENU: [&str; 8] = [
    "Go to definition",
    "Find references",
    "Rename symbol…",
    "Cut",
    "Copy",
    "Paste",
    "Toggle line comment",
    "Format document",
];

#[test]
fn right_click_opens_the_source_menu_and_toggle_line_comment_acts_on_the_clicked_line() {
    let (ctx, mut app, bundle_id) = veriloga_with_focused_editor();
    let original = root_text(&app, bundle_id);
    let second_line = original.lines().nth(1).expect("a second line").to_owned();

    let output = right_click_line(&ctx, &mut app, 2);
    let rows = CONTEXT_MENU.map(|label| {
        menu_item(&output, label).unwrap_or_else(|| panic!("the menu offers {label}"))
    });
    assert!(
        rows.windows(2)
            .all(|pair| pair[0].0.center().y < pair[1].0.center().y),
        "the menu lists its rows in order"
    );
    assert!(
        rows.iter().all(|(_, enabled)| *enabled),
        "every row is live on an editable source"
    );

    press(
        &ctx,
        &mut app,
        rows[6].0.center(),
        egui::PointerButton::Primary,
    );
    frame(&ctx, &mut app, DESKTOP);
    frame(&ctx, &mut app, DESKTOP);
    assert_eq!(
        root_text(&app, bundle_id),
        original.replacen(&second_line, &format!("// {second_line}"), 1),
        "Toggle line comment commented the line the menu was opened on"
    );
}

#[test]
fn on_a_read_only_source_the_menu_disables_cut_and_paste() {
    let (ctx, mut app, _) = veriloga_with_focused_editor();
    app.state.model.safe_mode.activate(
        crate::state::write_policy::LocalSafeModeOptions {
            open_project_read_only: true,
            ..crate::state::write_policy::LocalSafeModeOptions::default()
        },
        String::new(),
    );
    let output = right_click_line(&ctx, &mut app, 2);
    let enabled = |label| {
        menu_item(&output, label)
            .unwrap_or_else(|| panic!("the menu offers {label}"))
            .1
    };
    assert!(!enabled("Cut"));
    assert!(!enabled("Paste"));
    assert!(!enabled("Toggle line comment"));
    assert!(!enabled("Rename symbol…"));
    assert!(enabled("Copy"), "reading stays possible");
    assert!(enabled("Go to definition"));
}

#[test]
fn go_to_definition_from_the_menu_moves_the_caret_to_the_definition() {
    let (ctx, mut app, bundle_id) = veriloga_with_focused_editor();
    app.state
        .model
        .workspace
        .content
        .replace_project_source_bundle_file(bundle_id, "model.va", NAVIGABLE_MODULE.to_owned())
        .expect("the module is replaced");
    frame(&ctx, &mut app, DESKTOP);

    let output = right_click_line(&ctx, &mut app, 6);
    let caret = |app: &RSpiceApp| {
        app.state
            .code
            .code_workspace
            .source_carets
            .get(&(bundle_id, "model.va".to_owned()))
            .copied()
    };
    let line_start = |line: usize| {
        NAVIGABLE_MODULE
            .split_inclusive('\n')
            .take(line - 1)
            .map(|text| text.chars().count())
            .sum::<usize>()
    };
    assert_eq!(
        caret(&app),
        Some(line_start(6)),
        "the right-click put the caret on the use of g"
    );
    let (row, enabled) = menu_item(&output, "Go to definition").expect("Go to definition");
    assert!(enabled);
    press(&ctx, &mut app, row.center(), egui::PointerButton::Primary);
    for _ in 0..3 {
        frame(&ctx, &mut app, DESKTOP);
    }
    assert_eq!(
        caret(&app),
        Some(line_start(4)),
        "the caret moved to the parameter's definition"
    );
    assert!(
        app.state.code.code_workspace.language_tools.is_none(),
        "Go to definition jumps rather than opening the tools dialog"
    );
}

/// The Automation page with its entry script's caret on the first line, in
/// a focused editor.
fn automation_with_focused_editor() -> (egui::Context, RSpiceApp, ProjectSourceId) {
    let mut app = open_project_app();
    app.state.workbench.activate(Workspace::Netlist);
    app.state.code.code_workspace.page = CodeWorkspacePage::Automation;
    let ctx = themed_context();
    frame(&ctx, &mut app, DESKTOP);
    frame(&ctx, &mut app, DESKTOP);
    let bundle = app
        .state
        .model
        .workspace
        .content
        .project_sources
        .bundle_for_owner(&crate::state::ProjectSourceOwner::code_workspace(
            ProjectSourceLanguage::RSpiceAutomation,
        ))
        .expect("the project's Automation workspace");
    let (bundle_id, root) = (bundle.id(), bundle.root().logical_path().to_owned());
    let editor =
        code_workspace::source_editor_id(ProjectSourceLanguage::RSpiceAutomation, bundle_id, &root);
    ctx.memory_mut(|memory| memory.request_focus(editor));
    frame(&ctx, &mut app, DESKTOP);
    (ctx, app, bundle_id)
}

/// The Automation page's editor offers the source editor's menu, every row
/// live on its editable entry script, and Toggle line comment, chosen
/// there, comments the line the menu was opened on once the page has drawn.
#[test]
fn the_automation_pages_menu_comments_the_line_it_was_opened_on() {
    let (ctx, mut app, bundle_id) = automation_with_focused_editor();
    let original = root_text(&app, bundle_id);
    let second_line = original.lines().nth(1).expect("a second line").to_owned();

    let output = right_click_line_of(&ctx, &mut app, PYTHON_EDITOR, 2);
    let rows = CONTEXT_MENU.map(|label| {
        menu_item(&output, label).unwrap_or_else(|| panic!("the menu offers {label}"))
    });
    assert!(
        rows.windows(2)
            .all(|pair| pair[0].0.center().y < pair[1].0.center().y),
        "the menu lists its rows in order"
    );
    assert!(
        rows.iter().all(|(_, enabled)| *enabled),
        "every row is live on an editable source"
    );

    press(
        &ctx,
        &mut app,
        rows[6].0.center(),
        egui::PointerButton::Primary,
    );
    frame(&ctx, &mut app, DESKTOP);
    frame(&ctx, &mut app, DESKTOP);
    assert_eq!(
        root_text(&app, bundle_id),
        original.replacen(&second_line, &format!("# {second_line}"), 1),
        "Toggle line comment commented the line the menu was opened on"
    );
}

/// In a project opened read-only, the Automation page's menu greys what
/// would write the script and keeps what only reads it.
#[test]
fn on_a_read_only_project_the_automation_pages_menu_disables_cut_and_paste() {
    let (ctx, mut app, _) = automation_with_focused_editor();
    app.state.model.safe_mode.activate(
        crate::state::write_policy::LocalSafeModeOptions {
            open_project_read_only: true,
            ..crate::state::write_policy::LocalSafeModeOptions::default()
        },
        String::new(),
    );
    let output = right_click_line_of(&ctx, &mut app, PYTHON_VIEWER, 2);
    let enabled = |label| {
        menu_item(&output, label)
            .unwrap_or_else(|| panic!("the menu offers {label}"))
            .1
    };
    assert!(!enabled("Cut"));
    assert!(!enabled("Paste"));
    assert!(!enabled("Toggle line comment"));
    assert!(enabled("Copy"), "reading stays possible");
}
