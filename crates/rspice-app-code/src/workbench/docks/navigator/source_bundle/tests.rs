//! Which files a source rail offers and what each row says about itself.
//!
//! A row states the role its bundle records, and only a role other than
//! ordinary source.

use super::*;
use crate::workbench::app::dialogs::source_file_form::tests::two_files;

/// The defect this rail exists for: `selected_file` had no UI writer, so
/// every file in a bundle except the root was unreachable. Assert the
/// bundle's whole file set is offered, and that choosing one is what
/// changes the selection.
#[test]
fn every_file_in_the_bundle_is_offered_and_selectable() {
    let mut harness = two_files();
    let language = ProjectSourceLanguage::VerilogA;
    let rows = bundle_rows(&harness.model, language);
    assert!(
        rows.len() > 1,
        "the view's bundle carries its main file and the stage it includes"
    );

    let root = active_path(&harness.model, &harness.state.code_workspace, language)
        .expect("bundle root is the default document");
    let other = rows
        .iter()
        .find(|row| row.logical_path != root)
        .expect("a non-root file to select");
    let target = other.logical_path.clone();

    select(
        &harness.model,
        &mut harness.state.code_workspace,
        language,
        &target,
    );

    assert_eq!(
        active_path(&harness.model, &harness.state.code_workspace, language).as_deref(),
        Some(target.as_str())
    );
}

/// A row states the role the bundle recorded. Nothing here is inferred
/// from the filename.
#[test]
fn rows_state_the_recorded_role_rather_than_guessing_from_the_name() {
    let mut harness = two_files();
    let id = harness
        .model
        .workspace
        .active_source_bundle(ProjectSourceLanguage::VerilogA)
        .expect("the open view's bundle")
        .id();
    harness
        .model
        .workspace
        .content
        .add_project_source_bundle_file_with_role(
            id,
            "gain.va",
            crate::state::ProjectSourceFile::try_new("settings.toml", "").expect("a valid file"),
            ProjectSourceRole::VerilogABuildProfile,
        )
        .expect("the bundle takes a build profile");
    let messages = MessageCatalog::new(crate::workbench::UiTextLocale::EnglishUnitedStates);
    let rows = bundle_rows(&harness.model, ProjectSourceLanguage::VerilogA);
    let stated = rows
        .iter()
        .map(|row| {
            (
                row.label.clone(),
                role_message(row.role).map(|role| messages.text(role)),
            )
        })
        .collect::<Vec<_>>();

    assert!(
        stated.contains(&("settings.toml".to_owned(), Some("build profile".to_owned()))),
        "rows were {stated:?}"
    );
    assert!(
        stated.contains(&("stage.va".to_owned(), None)),
        "rows were {stated:?}"
    );
}

/// Ordinary source is what every file of a Verilog-A bundle is unless it is
/// the build profile, so it is the one role a row never states.
#[test]
fn only_a_role_other_than_ordinary_source_is_stated() {
    let messages = MessageCatalog::new(crate::workbench::UiTextLocale::EnglishUnitedStates);

    assert_eq!(role_message(None), None);
    assert_eq!(
        role_message(Some(ProjectSourceRole::VerilogABuildProfile)).map(|role| messages.text(role)),
        Some("build profile".to_owned())
    );
}
