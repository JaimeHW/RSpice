//! Every operation on a Verilog-A file from its row on the Files rail,
//! through whole frames: the header's New file, the row's keys, and its
//! menu, whose forms and revision history each open on their own.

use egui::Key;

use super::language_tools::{NONE, op_amp, type_text};
use super::*;
use rspice_app_code::workbench::documents::source_editing;
use rspice_app_workbench::workbench::localization::MessageId;

const NEW_FORM: &str = "New Verilog-A file";

/// The bundle of the view in front.
fn bundle(user: &User) -> rspice_app_state::state::ProjectSourceBundle {
    source_editing::selected_veriloga_source_in(&user.app.state.model)
        .expect("the open view")
        .bundle()
        .clone()
}

/// The file the rail and the editor show: the one chosen, else the main file.
fn selected(user: &User) -> String {
    user.app
        .state
        .code
        .source_editing
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
        .text(MessageId::SourceNewFile);
    user.click(&label);
    type_text(user, stem);
}

#[test]
fn new_file_on_the_rail_is_a_compact_form_and_enter_creates_the_file() {
    let mut user = op_amp();
    let main = main_file(&user);
    new_file(&mut user, "stage");
    user.expect(NEW_FORM);
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
    let reason = messages.text(MessageId::SourceRootDeleteBlocked);

    // The header's Delete acts on the file in front, the main file: held.
    user.click(&messages.text(MessageId::SourceDelete));
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

/// Where the rail drew the row of `path`: the navigator's, left of the
/// editor's tab of the same name.
fn row(user: &User, path: &str) -> egui::Rect {
    user.named
        .iter()
        .filter(|(name, _)| name == path)
        .map(|(_, rect)| *rect)
        .min_by(|a, b| a.left().total_cmp(&b.left()))
        .unwrap_or_else(|| panic!("no row named {path:?}"))
}

/// Open the menu of `path`'s row, as a right-click does.
fn open_row_menu(user: &mut User, path: &str) -> egui::Rect {
    let at = row(user, path);
    user.press(at.center(), egui::PointerButton::Secondary);
    user.settle();
    at
}

/// Where the menu open under `at` drew `name`, which an item that names
/// its key says before the key: the header's icons of the same names are
/// above the rows, so the nearest below the row is the menu's.
fn menu_item(user: &User, at: egui::Rect, name: &str) -> Option<egui::Rect> {
    let keyed = format!("{name} ");
    user.named
        .iter()
        .filter(|(candidate, rect)| {
            (candidate == name || candidate.starts_with(&keyed)) && rect.top() >= at.top()
        })
        .map(|(_, rect)| *rect)
        .min_by(|a, b| a.top().total_cmp(&b.top()))
}

/// Press `name` on the menu open under `at`.
fn press_menu_item(user: &mut User, at: egui::Rect, name: &str) {
    let item = menu_item(user, at, name).unwrap_or_else(|| panic!("the menu offers {name:?}"));
    user.press(item.center(), egui::PointerButton::Primary);
    user.settle();
}

/// A file row's menu holds every operation on the file, New file first and
/// Delete last. The main file cannot be deleted, so its Delete is held and
/// opens nothing; the others open their forms.
#[test]
fn a_file_rows_menu_offers_every_operation_and_holds_a_refused_one() {
    let mut user = op_amp();
    let main = main_file(&user);
    let messages = user.app.state.workbench.messages();
    let at = open_row_menu(&mut user, &main);
    let items = [
        MessageId::SourceNewFile,
        MessageId::SourceRename,
        MessageId::SourceMove,
        MessageId::SourceDuplicate,
        MessageId::SourceRevisionHistory,
        MessageId::SourceReplaceFromFile,
        MessageId::SourceDelete,
    ]
    .map(|item| {
        menu_item(&user, at, &messages.text(item))
            .unwrap_or_else(|| panic!("the menu offers {item:?}"))
    });
    assert!(
        items.windows(2).all(|pair| pair[0].top() < pair[1].top()),
        "the menu lists them in order: {items:?}"
    );

    press_menu_item(&mut user, at, &messages.text(MessageId::SourceDelete));
    assert!(
        user.find("Delete file").is_none(),
        "the held Delete opens nothing"
    );
    assert!(bundle(&user).contains_file(&main));
    user.key(Key::Escape, NONE);
    user.settle();

    let at = open_row_menu(&mut user, &main);
    press_menu_item(&mut user, at, &messages.text(MessageId::SourceDuplicate));
    user.expect("Duplicate file");
}

/// Rename from the row's menu runs the whole transaction: the form opens on
/// the row's file, and committing it renames the file the bundle holds.
#[test]
fn rename_from_a_file_rows_menu_renames_the_file() {
    let mut user = op_amp();
    let main = main_file(&user);
    let messages = user.app.state.workbench.messages();
    let at = open_row_menu(&mut user, &main);
    press_menu_item(&mut user, at, &messages.text(MessageId::SourceRename));
    user.expect("Rename file");

    type_text(&mut user, "amp");
    user.key(Key::Enter, NONE);
    user.settle();
    assert_eq!(main_file(&user), "amp.va");
}

/// Revision history opens from the row's menu as a dialog of its own,
/// named for the file, and restores the revision chosen in it as a new one:
/// the file made since is gone again, and the dialog closes.
#[test]
fn revision_history_from_a_file_rows_menu_restores_the_revision_chosen() {
    let mut user = op_amp();
    let main = main_file(&user);
    new_file(&mut user, "stage");
    user.key(Key::Enter, NONE);
    user.settle();
    assert!(bundle(&user).contains_file("stage.va"));

    let messages = user.app.state.workbench.messages();
    let at = open_row_menu(&mut user, &main);
    press_menu_item(
        &mut user,
        at,
        &messages.text(MessageId::SourceRevisionHistory),
    );
    user.expect(&format!("Revision history \u{b7} {main}"));

    // Every revision's row announces itself; the earlier one, before the
    // new file, is the one not current.
    let earlier = user
        .named
        .iter()
        .find(|(name, _)| name.starts_with("Revision ") && name.ends_with("1 file."))
        .map(|(_, rect)| *rect)
        .expect("the revision before the new file is listed");
    user.press(earlier.center(), egui::PointerButton::Primary);
    user.settle();
    user.click(&messages.text(MessageId::SourceRestoreAsNewRevision));
    assert!(
        !bundle(&user).contains_file("stage.va"),
        "the revision restored has no stage.va"
    );
    assert!(
        user.app.state.code.source_editing.source_history.is_none(),
        "a restore closes the history"
    );
}

/// The rail's Includes lists both directions: what the file in front
/// includes, and the files that include it. Its count is the files included.
#[test]
fn the_rail_lists_the_files_that_include_the_one_in_front() {
    let mut user = op_amp();
    let main = main_file(&user);
    new_file(&mut user, "stage");
    user.key(Key::Enter, NONE);
    user.settle();
    assert_eq!(selected(&user), "stage.va");

    let messages = user.app.state.workbench.messages();
    let included = bundle(&user)
        .file_content("stage.va")
        .expect("the file")
        .lines()
        .filter(|line| line.trim_start().starts_with("`include"))
        .count();
    user.expect(&format!(
        "{}, {included}",
        messages.text(MessageId::SourceNavigatorIncludes)
    ));
    let included_by = messages.text(MessageId::SourceNavigatorIncludedBy);
    let caption = user.expect(&included_by);
    assert!(
        user.named
            .iter()
            .any(|(name, rect)| *name == main && rect.top() > caption.top()),
        "{main} is listed under {included_by}"
    );
}

/// The inspector states the file in front's line endings and where the
/// source stands with the compiler, as facts beside its module.
#[test]
fn the_inspector_states_the_line_endings_and_the_compile() {
    let user = op_amp();
    let messages = user.app.state.workbench.messages();
    user.expect(&messages.text(MessageId::SourceLineEndings));
    user.expect(&messages.text(MessageId::SourceCompile));
}
