//! New, Rename and Delete from the Verilog-A Files rail, through whole
//! frames: each opens its own compact form, never Manage source document.

use egui::Key;

use super::language_tools::{NONE, op_amp, type_text};
use super::*;
use rspice_app_code::workbench::documents::code_workspace;
use rspice_app_workbench::workbench::localization::MessageId;

const MANAGE: &str = "Manage source document";
const NEW_FORM: &str = "New Verilog-A file";

/// The bundle of the view in front.
fn bundle(user: &User) -> rspice_app_state::state::ProjectSourceBundle {
    code_workspace::selected_veriloga_source_in(&user.app.state.model)
        .expect("the open view")
        .bundle()
        .clone()
}

/// The file the rail and the editor show: the one chosen, else the main file.
fn selected(user: &User) -> String {
    user.app
        .state
        .code
        .code_workspace
        .veriloga
        .selected_file
        .as_ref()
        .map(|selection| selection.logical_path.clone())
        .filter(|path| bundle(user).contains_file(path))
        .unwrap_or_else(|| main_file(user))
}

fn main_file(user: &User) -> String {
    bundle(user).root().logical_path().to_owned()
}

fn first_line(user: &User, path: &str) -> String {
    bundle(user)
        .file_content(path)
        .expect("the file")
        .lines()
        .next()
        .unwrap_or_default()
        .to_owned()
}

/// The rail's New file icon, then `stem` typed over the suggestion.
fn new_file(user: &mut User, stem: &str) {
    let label = user
        .app
        .state
        .workbench
        .messages()
        .text(MessageId::CodeSourceNewFile);
    user.click(&label);
    type_text(user, stem);
}

#[test]
fn new_file_on_the_rail_is_a_compact_form_and_enter_creates_the_file() {
    let mut user = op_amp();
    let main = main_file(&user);
    new_file(&mut user, "stage");
    user.expect(NEW_FORM);
    assert!(
        user.find(MANAGE).is_none(),
        "Manage source document stays shut"
    );
    user.expect(&format!("Adds an `include at the top of {main}"));

    user.key(Key::Enter, NONE);
    user.settle();
    assert!(user.find(NEW_FORM).is_none(), "the form closes on Create");
    assert!(
        bundle(&user).contains_file("stage.va"),
        "the stem was typed over"
    );
    assert_eq!(selected(&user), "stage.va", "the new file is the one shown");
    assert_eq!(first_line(&user, &main), "`include \"stage.va\"");
}

#[test]
fn f2_on_a_file_row_renames_it_with_the_stem_selected() {
    let mut user = op_amp();
    let main = main_file(&user);
    user.focus_on(&main);
    user.key(Key::F2, NONE);
    user.settle();
    user.expect("Rename file");
    assert!(user.find(MANAGE).is_none());

    type_text(&mut user, "amp");
    user.key(Key::Enter, NONE);
    user.settle();
    assert_eq!(main_file(&user), "amp.va", "typing replaced the stem only");
    assert_eq!(selected(&user), "amp.va");
}

#[test]
fn a_name_the_bundle_holds_is_refused_and_create_does_nothing() {
    let mut user = op_amp();
    let main = main_file(&user);
    let stem = main
        .rsplit_once('.')
        .map_or(main.as_str(), |(stem, _)| stem);
    let files = bundle(&user).files().len();
    new_file(&mut user, &stem.to_uppercase());
    user.expect(&format!("A file named {main} already exists."));

    user.key(Key::Enter, NONE);
    user.settle();
    user.click("Create");
    user.expect(NEW_FORM);
    assert_eq!(bundle(&user).files().len(), files, "nothing was created");
}

#[test]
fn deleting_a_leaf_removes_its_include() {
    let mut user = op_amp();
    let main = main_file(&user);
    new_file(&mut user, "stage");
    user.key(Key::Enter, NONE);
    user.settle();
    assert_eq!(first_line(&user, &main), "`include \"stage.va\"");

    // The new file is the one shown, so its row is the rail's only one by
    // that name: the main file's includes are not listed.
    user.focus_on("stage.va");
    user.key(Key::Delete, NONE);
    user.settle();
    user.expect("Delete file");
    user.expect(&format!("Removes its `include from {main}"));
    user.click("Delete");
    assert!(!bundle(&user).contains_file("stage.va"));
    assert!(
        !bundle(&user)
            .file_content(&main)
            .expect("the main file")
            .contains("stage.va"),
        "the include went with the file"
    );
    assert_eq!(selected(&user), main, "the reader is left on its includer");
}

#[test]
fn the_main_file_cannot_be_deleted_and_says_why() {
    let mut user = op_amp();
    let main = main_file(&user);
    let messages = user.app.state.workbench.messages();
    let reason = messages.text(MessageId::CodeSourceRootDeleteBlocked);

    // The header's Delete acts on the file in front, the main file: held.
    user.click(&messages.text(MessageId::CodeSourceDelete));
    assert!(
        user.find("Delete file").is_none(),
        "the held icon opens nothing"
    );

    // The key on its row says why instead of doing nothing.
    user.focus_on(&main);
    user.key(Key::Delete, NONE);
    user.settle();
    assert!(user.find("Delete file").is_none());
    assert_eq!(
        user.app
            .state
            .workbench
            .prompt
            .as_ref()
            .map(|prompt| prompt.text.trim_end_matches('\u{a0}')),
        Some(reason.as_str())
    );
    assert!(bundle(&user).contains_file(&main));
}
