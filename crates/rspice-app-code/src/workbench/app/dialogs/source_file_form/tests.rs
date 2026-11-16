//! The file forms' judgement of a name, the stem a rename selects and the
//! files the bundle refuses to delete.

use super::*;
use crate::state::{ProjectSourceOwner, project_sources::ProjectSourceError};
use crate::workbench::code::{
    CodeFeature,
    tests::{CodeHarness, harness},
};

fn english() -> MessageCatalog {
    MessageCatalog::new(crate::workbench::UiTextLocale::EnglishUnitedStates)
}

fn reference() -> crate::state::CellViewRef {
    crate::state::CellViewRef::new("behavioral", "gain", "veriloga")
}

/// The Verilog-A view `behavioral/gain/veriloga` open, its bundle the main
/// file `gain.va` and `stage.va`, which the main file includes.
fn two_files() -> CodeHarness {
    let mut harness = harness();
    let reference = reference();
    let mut view = crate::state::View::new("veriloga", crate::state::ViewType::VerilogA);
    view.metadata
        .insert("veriloga.module".to_owned(), "gain".to_owned());
    let mut cell = crate::state::Cell::new(&reference.cell);
    cell.add_view(view);
    let mut library = crate::state::Library::new(&reference.library);
    library.add_cell(cell);
    harness.model.library_manager.add_library(library);
    let bundle = ProjectSourceBundle::try_new(
        ProjectSourceOwner::cell_view(reference.clone()),
        ProjectSourceLanguage::VerilogA,
        "gain.va",
        "module gain(p, n);\n  inout p, n;\n  electrical p, n;\n  analog I(p, n) <+ V(p, n);\nendmodule\n",
        [],
        [],
    )
    .expect("a valid cell-view bundle");
    let id = bundle.id();
    harness
        .model
        .workspace
        .content
        .project_sources
        .insert_bundle(bundle)
        .expect("one bundle per cell view");
    harness
        .model
        .workspace
        .content
        .add_project_source_bundle_file(
            id,
            "gain.va",
            ProjectSourceFile::try_new("stage.va", "// stage\n").expect("a valid file"),
        )
        .expect("the main file includes the stage");
    harness
        .model
        .workspace
        .open_view(reference, crate::state::ViewType::VerilogA);
    harness
}

fn bundle(harness: &CodeHarness) -> &ProjectSourceBundle {
    harness
        .model
        .workspace
        .content
        .project_sources
        .bundle_for_owner(&ProjectSourceOwner::cell_view(reference()))
        .expect("the view's bundle")
}

fn open(harness: &mut CodeHarness, action: CodeSourceFileAction, path: &str) {
    let id = bundle(harness).id();
    code_workspace::open_source_file_dialog_in_bundle(
        &mut CodeFeature::workspace_parts(&mut harness.cx()),
        ProjectSourceLanguage::VerilogA,
        id,
        action,
        path,
        path,
    )
    .expect("the form opens");
}

/// What the open draft's name, set to `name`, is judged as.
fn judged(harness: &mut CodeHarness, name: &str) -> NameCheck {
    let draft = harness
        .state
        .code_workspace
        .source_file_dialog
        .as_mut()
        .expect("a form is open");
    draft.proposed_path = name.to_owned();
    let draft = draft.clone();
    check_name(Some(bundle(harness)), &draft, english())
}

fn refused(text: &str) -> NameCheck {
    NameCheck::Refused(text.to_owned())
}

#[test]
fn a_rename_selects_the_leafs_stem_and_keeps_its_folder_and_extension() {
    for (path, selected) in [
        ("untitled.va", "untitled"),
        ("models/gain.va", "gain"),
        ("a.b.va", "a.b"),
        ("README", "README"),
        (".hidden", ".hidden"),
        ("models/.env", ".env"),
        ("étage.vams", "étage"),
    ] {
        let (start, end) = stem_range(path);
        let chosen: String = path.chars().skip(start).take(end - start).collect();
        assert_eq!(chosen, selected, "{path}");
    }
}

#[test]
fn a_malformed_path_is_refused_in_the_readers_words() {
    let said = |name: &str| {
        let error = ProjectSourceFile::try_new(name, String::new()).expect_err(name);
        name_error(&error, name, english())
    };
    for (name, words) in [
        ("models\\gain.va", "Use / to separate folders."),
        ("/gain.va", "A path can't start with / or a drive."),
        ("C:/gain.va", "A path can't start with / or a drive."),
        (
            "models//gain.va",
            "Each folder and file needs a name; '.' and '..' aren't allowed.",
        ),
        (
            "../gain.va",
            "Each folder and file needs a name; '.' and '..' aren't allowed.",
        ),
        ("models /gain.va", "A name can't begin or end with a space."),
        ("models./gain.va", "A name can't end with a period."),
        ("gain.va.", "A name can't end with a period."),
        ("ga*in.va", "A name can't contain \" ' : * ? < > |"),
        ("con.va", "Windows reserves this name; choose another."),
        (
            "models/LPT1.va",
            "Windows reserves this name; choose another.",
        ),
    ] {
        assert_eq!(said(name), words, "{name}");
    }
    let long = format!("{}.va", "a".repeat(5000));
    assert!(
        said(&long).starts_with("The path is longer than "),
        "{}",
        said(&long)
    );
}

#[test]
fn the_bundles_other_refusals_are_worded_and_the_rest_fall_back_to_its_own() {
    let main = ProjectSourceDocument::try_new("gain.txt", ProjectSourceLanguage::VerilogA, "")
        .expect_err("the main file needs a Verilog-A extension");
    assert_eq!(
        name_error(&main, "gain.txt", english()),
        "The main file must end in '.va', '.vams' or '.v'."
    );
    let duplicate = ProjectSourceError::DuplicateLogicalPath {
        logical_path: "gain.va".to_owned(),
    };
    assert_eq!(
        name_error(&duplicate, "GAIN.VA", english()),
        "A file named gain.va already exists."
    );
    let other = ProjectSourceError::MissingFile {
        logical_path: "gone.va".to_owned(),
    };
    assert_eq!(name_error(&other, "gone.va", english()), other.to_string());
}

#[test]
fn a_new_name_is_judged_before_the_commit() {
    let mut harness = two_files();
    open(&mut harness, CodeSourceFileAction::New, "gain.va");
    assert_eq!(judged(&mut harness, "  "), NameCheck::Empty);
    assert_eq!(
        judged(&mut harness, "STAGE.VA"),
        refused("A file named stage.va already exists."),
        "the bundle compares names without case"
    );
    assert_eq!(
        judged(&mut harness, "gain.va"),
        refused("A file named gain.va already exists.")
    );
    assert_eq!(
        judged(&mut harness, "ga?in.va"),
        refused("A name can't contain \" ' : * ? < > |")
    );
    assert_eq!(judged(&mut harness, " bias.va "), NameCheck::Ready);
}

#[test]
fn renaming_the_main_file_keeps_a_verilog_a_extension_and_allows_a_case_change() {
    let mut harness = two_files();
    open(&mut harness, CodeSourceFileAction::Rename, "gain.va");
    assert_eq!(judged(&mut harness, "gain.va"), NameCheck::Unchanged);
    assert_eq!(judged(&mut harness, "Gain.va"), NameCheck::Ready);
    assert_eq!(
        judged(&mut harness, "gain.txt"),
        refused("The main file must end in '.va', '.vams' or '.v'.")
    );
    assert_eq!(
        judged(&mut harness, "stage.va"),
        refused("A file named stage.va already exists.")
    );

    // Any other file may be named anything the bundle can hold.
    open(&mut harness, CodeSourceFileAction::Rename, "stage.va");
    assert_eq!(judged(&mut harness, "notes.txt"), NameCheck::Ready);
}

#[test]
fn the_main_file_a_role_bound_file_and_an_includer_refuse_deletion() {
    let mut harness = two_files();
    let id = bundle(&harness).id();
    harness
        .model
        .workspace
        .content
        .add_project_source_bundle_file(
            id,
            "stage.va",
            ProjectSourceFile::try_new("bias.va", "// bias\n").expect("a valid file"),
        )
        .expect("the stage includes the bias");
    harness
        .model
        .workspace
        .content
        .add_project_source_bundle_file_with_role(
            id,
            "gain.va",
            ProjectSourceFile::try_new("build.toml", "").expect("a valid file"),
            crate::state::ProjectSourceRole::VerilogABuildProfile,
        )
        .expect("the bundle takes a build profile");
    let bundle = bundle(&harness);
    let refusal = |path: &str| delete_refusal(bundle, path, english());
    assert_eq!(
        refusal("gain.va").as_deref(),
        Some("The main file can't be deleted. Rename it or replace its source instead.")
    );
    assert_eq!(
        refusal("build.toml").as_deref(),
        Some("build.toml is the Verilog-A build profile; it can't be deleted.")
    );
    assert_eq!(
        refusal("stage.va").as_deref(),
        Some("stage.va includes bias.va. Delete bias.va first.")
    );
    assert_eq!(refusal("bias.va"), None);
}
