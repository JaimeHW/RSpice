//! Which files a source rail offers and what each row says about itself.
//!
//! A row states the role its bundle records, and only a role other than
//! ordinary source.

use super::*;

/// The defect this rail exists for: `selected_file` had no UI writer, so
/// every file in a bundle except the root was unreachable. Assert the
/// bundle's whole file set is offered, and that choosing one is what
/// changes the selection.
#[test]
fn every_file_in_the_bundle_is_offered_and_selectable() {
    let mut app = RSpiceApp::test_instance();
    let language = ProjectSourceLanguage::RSpiceAutomation;
    let rows = bundle_rows(&app.state, language);
    assert!(
        rows.len() > 1,
        "an automation bundle carries an entry, a run plan, a lock, and a manifest"
    );

    let root = active_path(&app.state, language).expect("bundle root is the default document");
    let other = rows
        .iter()
        .find(|row| row.logical_path != root)
        .expect("a non-root file to select");
    let target = other.logical_path.clone();

    select(&mut app.state, language, &target);

    assert_eq!(
        active_path(&app.state, language).as_deref(),
        Some(target.as_str())
    );
}

/// A row states the role the bundle recorded. Nothing here is inferred
/// from the filename.
#[test]
fn rows_state_the_recorded_role_rather_than_guessing_from_the_name() {
    let app = RSpiceApp::test_instance();
    let messages = MessageCatalog::new(crate::workbench::UiTextLocale::EnglishUnitedStates);
    let rows = bundle_rows(&app.state, ProjectSourceLanguage::RSpiceAutomation);
    let roles = rows
        .iter()
        .filter_map(|row| role_message(row.role))
        .map(|role| messages.text(role))
        .collect::<std::collections::BTreeSet<_>>();

    assert!(roles.contains("run plan"), "roles were {roles:?}");
    assert!(roles.contains("permissions"), "roles were {roles:?}");
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
