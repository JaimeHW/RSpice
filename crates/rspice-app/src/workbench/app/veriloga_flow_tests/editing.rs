//! Undo and compile bookkeeping at the Verilog-A editor, through whole
//! application frames.

use egui::Key;

use super::language_tools::{CTRL, EDITOR, caret_after, op_amp, source, type_text};
use super::*;
use crate::state::ProjectSourceRole;
use crate::workbench::documents::code_workspace;
use crate::workbench::lifecycle::project_lifecycle::{
    DestinationAuthority, SaveScope, active_document_is_dirty, save_native,
};

fn undo(user: &mut User) {
    user.key(Key::Z, CTRL);
    user.settle();
}

fn redo(user: &mut User) {
    user.key(Key::Y, CTRL);
    user.settle();
}

fn format(user: &mut User) {
    Command::FormatDocument.execute(&mut user.app);
    user.settle();
}

fn replace_source_with(user: &mut User, text: &str) {
    super::super::veriloga_interface_tests::edit_source(&mut user.app, text);
    user.settle();
}

#[test]
fn format_is_one_undo_and_the_typing_before_it_is_the_next() {
    let mut user = op_amp();
    let original = source(&user);
    caret_after(&mut user, "vtarget = vslew;");
    // A trailing blank, which Format takes back out.
    type_text(&mut user, " ");
    let typed = source(&user);
    assert_ne!(typed, original);

    format(&mut user);
    let formatted = source(&user);
    assert_ne!(formatted, typed, "the blank is formatted away");

    undo(&mut user);
    assert_eq!(source(&user), typed, "one Undo takes back the whole format");
    undo(&mut user);
    assert_eq!(source(&user), original, "the next takes back the typing");

    redo(&mut user);
    assert_eq!(source(&user), typed);
    redo(&mut user);
    assert_eq!(source(&user), formatted);
}

#[test]
fn rename_is_one_undo_across_the_module() {
    let mut user = op_amp();
    let original = source(&user);
    caret_after(&mut user, "vtarget = vslew");
    let selected = code_workspace::selected_veriloga_source(&user.app).expect("the open view");
    let path = selected.document().logical_path().to_owned();
    let caret = original.find("vslew").expect("the name");
    code_workspace::open_language_tools_in_bundle(
        &mut user.app,
        crate::state::ProjectSourceLanguage::VerilogA,
        selected.bundle().id(),
        &path,
        original[..caret].chars().count(),
    )
    .expect("the language tools open");
    let tools = user
        .app
        .state
        .code
        .code_workspace
        .language_tools
        .as_mut()
        .expect("the form is open");
    tools.query = "vslew".to_owned();
    tools.replacement = "limited".to_owned();
    code_workspace::commit_language_rename(&mut user.app).expect("the rename applies");
    user.settle();
    let renamed = source(&user);
    assert!(!renamed.contains("vslew"));
    assert!(renamed.matches("limited").count() > 2);

    user.click(EDITOR);
    undo(&mut user);
    assert_eq!(source(&user), original, "one Undo takes back every use");
}

/// An empty source is drawn by the same editor as any other: the one screen
/// readers name, whose history holds the first thing typed into it.
#[test]
fn an_empty_source_stays_in_the_source_editor() {
    let mut user = op_amp();
    replace_source_with(&mut user, "");

    user.click(EDITOR);
    type_text(&mut user, "module");
    assert_eq!(source(&user), "module");
    undo(&mut user);
    assert_eq!(source(&user), "");
}

fn open_tab_dirty(user: &User, view: &CellViewRef) -> bool {
    user.app
        .state
        .model
        .workspace
        .content
        .open_views
        .iter()
        .find(|open| &open.reference == view)
        .expect("the view has a tab")
        .dirty
}

/// Compiling proves something about the source and changes nothing in it.
/// The tab stays as clean as the save left it, the attempt is kept, and a
/// source on the compiler's defaults is not given a build profile file.
#[test]
fn compiling_a_saved_source_leaves_its_tab_clean_and_writes_no_profile() {
    let (mut user, _, view) = new_device(CELL, PORTS);
    replace_source_with(&mut user, MODULE);
    // The first compile writes the module's ports to the cell.
    compile(&mut user);

    let path = std::env::temp_dir().join(format!(
        "rspice-va-editing-{}-{}.rspiceproj",
        std::process::id(),
        uuid::Uuid::new_v4().simple()
    ));
    save_native(
        &mut user.app.state,
        SaveScope::AllDocuments,
        &path,
        DestinationAuthority::UserSelected,
    )
    .expect("the project saves");
    user.settle();
    assert!(!open_tab_dirty(&user, &view), "the save cleaned the tab");
    let attempts = |user: &User| {
        code_workspace::selected_veriloga_source(&user.app)
            .expect("the open view")
            .bundle()
            .qualifications()
            .len()
    };
    let before = attempts(&user);

    compile(&mut user);

    assert_eq!(attempts(&user), before + 1, "the attempt is kept");
    assert!(!open_tab_dirty(&user, &view), "a compile is not an edit");
    assert!(!active_document_is_dirty(&user.app.state));
    let selected = code_workspace::selected_veriloga_source(&user.app).expect("the open view");
    assert!(
        selected
            .bundle()
            .paths_for_role(ProjectSourceRole::VerilogABuildProfile)
            .next()
            .is_none(),
        "nothing changed a setting, so no build profile exists"
    );
    let _ = std::fs::remove_file(path);
}
