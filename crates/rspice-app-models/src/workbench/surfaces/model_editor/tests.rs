//! Tests for the model editor surface's authoring behaviour.
//!
//! The cases pin how identities are elided and what the candidate header
//! announces.

use super::*;
use crate::workbench::framework::feature::harness::FeatureHarness;
use crate::workbench::models::tests::harness;

/// A project-owned candidate with enough declared parameters that the counts
/// the surface states are not all zero.
#[cfg(not(target_arch = "wasm32"))]
fn app_with_open_candidate() -> FeatureHarness<ModelsFeature> {
    use crate::state::model_library::ProjectModelDefinition;

    let mut app = harness();
    app.model
        .model_library_manager
        .create_project_model(
            "owned-models",
            &ProjectModelDefinition {
                name: "nch_owned".to_owned(),
                spice_type: "NMOS".to_owned(),
                description: "Project model".to_owned(),
                numeric_parameters: BTreeMap::from([
                    ("vto".to_owned(), 0.7),
                    ("kp".to_owned(), 120e-6),
                    ("gamma".to_owned(), 0.45),
                ]),
                string_parameters: BTreeMap::new(),
            },
        )
        .expect("create project model");
    let revision = app.model.workspace.content.project.revision();
    app.state
        .model_editor
        .open(
            &app.model.model_library_manager,
            "owned-models",
            "nch_owned",
            revision,
        )
        .expect("open model candidate");
    app
}

#[test]
fn identity_elision_preserves_both_collision_significant_ends() {
    assert_eq!(short_identity("short"), "short");
    assert_eq!(
        short_identity("0123456789abcdef0123456789abcdef"),
        "01234567…abcdef"
    );
}

/// A rejected persisted schema is stated once, by the header, as a failure
/// mark in front of neutral words. The counts read off it stay plain: a rail
/// entry painted red is the same rejection said a fourth time, and it names
/// no field the reader could repair.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn a_rejected_schema_is_announced_once_by_the_header() {
    let mut app = app_with_open_candidate();
    let mut records = persisted_records(&app.cx());
    records.metadata_error = Some("parameters[0]: out of declared bounds".to_owned());

    assert_eq!(
        candidate_status(&app.cx(), &records),
        Some((
            "metadata invalid · parameters[0]: out of declared bounds".to_owned(),
            Some(Problem::Failure)
        ))
    );
    for section in [ModelEditorSection::Parameters, ModelEditorSection::Sections] {
        let count = navigation_meta(&app.cx(), &records, section).expect("count");
        assert!(count.chars().all(|c| c.is_ascii_digit()), "{count}");
    }
}

/// A validated candidate says nothing at all, and an unvalidated one says so
/// without a mark: not having been validated yet is where every candidate
/// starts, not a problem the reader has to act on.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn the_header_announces_no_status_for_a_validated_candidate() {
    let mut app = app_with_open_candidate();
    let records = persisted_records(&app.cx());
    assert_eq!(
        candidate_status(&app.cx(), &records),
        Some(("Not validated".to_owned(), None))
    );

    let revision = app.model.workspace.content.project.revision();
    let evidence = app
        .state
        .model_editor
        .draft
        .as_ref()
        .expect("open candidate")
        .validate(&app.model.model_library_manager, revision)
        .expect("the fixture candidate validates");
    app.state.model_editor.validation = Some(evidence);
    assert_eq!(candidate_status(&app.cx(), &records), None);
}
