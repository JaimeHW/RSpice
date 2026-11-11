//! A simulation plan package, exported from a real plan and imported back.
//!
//! An unknown field or another format or version is refused by name; the
//! refusal a preview states is the one the import commits; an empty draft is
//! refused before anything is parsed; the proposed file name is derived,
//! bounded and portable; and the previewed specification count is what the
//! import creates.

use crate::workbench::RSpiceApp;
use crate::workbench::surfaces::simulate::plan_manager::exchange::{
    PACKAGE_EXTENSION, PACKAGE_STEM_LIMIT, SIMULATION_PLAN_PACKAGE_FORMAT,
    SIMULATION_PLAN_PACKAGE_VERSION, commit_import_simulation_plan, envelope_label,
    export_simulation_plan_package, inspect_package, package_file_name, specification_count,
};
use crate::workbench::surfaces::simulate::plan_manager::records::plan_catalog_records;

/// A real export of a real plan, which is the only package these tests use.
/// A hand-authored JSON fixture would drift from the wire form the moment a
/// field moved, and would prove nothing about the format that ships.
fn exported_package() -> String {
    let app = RSpiceApp::test_instance();
    let id = app
        .state
        .model
        .sim_setup
        .stable_analysis_plan()
        .expect("the fixture has a stable plan")
        .id();
    export_simulation_plan_package(&app.state.model, id).expect("the fixture's plan exports")
}

/// An unknown field is refused rather than dropped. This is the behaviour
/// the export's third note claims, and it is `deny_unknown_fields` doing it:
/// without that attribute serde would ignore the field and import a package
/// whose extra content silently went missing.
#[test]
fn a_package_carrying_an_unknown_field_is_refused_by_name() {
    let json = exported_package();
    let mut value: serde_json::Value = serde_json::from_str(&json).expect("the export is JSON");
    value["governance_authority"] = serde_json::Value::String("release".to_owned());
    let doctored = serde_json::to_string(&value).expect("the doctored package serializes");

    let error = inspect_package(&doctored).expect_err("an unknown field is refused");
    assert!(
        error.contains("governance_authority"),
        "the refusal has to name the field it did not know: {error}"
    );
    assert!(
        inspect_package(&json).is_ok(),
        "the same package without the extra field is accepted"
    );
}

/// The tag and the version are the whole compatibility contract, and both
/// halves of it are enforced.
#[test]
fn the_envelope_admits_only_this_format_and_this_version() {
    let json = exported_package();
    let mut value: serde_json::Value = serde_json::from_str(&json).expect("the export is JSON");
    assert_eq!(value["format"], SIMULATION_PLAN_PACKAGE_FORMAT);
    assert_eq!(value["version"], SIMULATION_PLAN_PACKAGE_VERSION);

    value["version"] = serde_json::json!(SIMULATION_PLAN_PACKAGE_VERSION + 1);
    let future = serde_json::to_string(&value).expect("the future package serializes");
    assert!(
        inspect_package(&future)
            .expect_err("a later version is refused")
            .contains("version"),
        "a version refusal has to say so"
    );

    value["version"] = serde_json::json!(SIMULATION_PLAN_PACKAGE_VERSION);
    value["format"] = serde_json::Value::String("rspice.result-document".to_owned());
    let foreign = serde_json::to_string(&value).expect("the foreign package serializes");
    assert!(
        inspect_package(&foreign)
            .expect_err("a foreign format is refused")
            .contains("rspice.result-document"),
        "a format refusal has to name the format it was handed"
    );
}

/// The preview and the commit refuse the same package with the same
/// sentence, because they are one function. A second envelope check would
/// let the dialog show a package as sound that the commit then rejects.
#[test]
fn the_previewed_refusal_is_the_committed_refusal() {
    let mut app = RSpiceApp::test_instance();
    let mut value: serde_json::Value =
        serde_json::from_str(&exported_package()).expect("the export is JSON");
    value["version"] = serde_json::json!(SIMULATION_PLAN_PACKAGE_VERSION + 1);
    let future = serde_json::to_string(&value).expect("the future package serializes");

    let previewed = inspect_package(&future).expect_err("the preview refuses it");
    let committed = commit_import_simulation_plan(
        &mut crate::workbench::features::cx(&mut app),
        &future,
        "Imported plan",
    )
    .expect_err("the commit refuses it");
    assert_eq!(previewed, committed);
}

/// An empty draft is refused with the instruction that matches the route:
/// the reader chooses a file or pastes one, and no longer types into a
/// field, so a message telling them to paste one would name a control that
/// is not there.
#[test]
fn an_empty_draft_is_refused_before_anything_is_parsed() {
    let mut app = RSpiceApp::test_instance();
    let error = commit_import_simulation_plan(
        &mut crate::workbench::features::cx(&mut app),
        "   ",
        "Imported plan",
    )
    .expect_err("an empty draft is refused");
    assert!(error.contains("Choose"), "{error}");
}

/// The proposed file name is derived from the plan and is safe to hand a
/// file system: one extension, no separators, and bounded.
#[test]
fn the_proposed_file_name_is_derived_bounded_and_portable() {
    let app = RSpiceApp::test_instance();
    let records = plan_catalog_records(&app.state.model);
    let record = records.first().expect("the fixture projects a plan");
    let derived = package_file_name(Some(record));
    assert!(
        derived.ends_with(PACKAGE_EXTENSION),
        "the proposal carries the package extension: {derived}"
    );
    assert!(
        derived
            .chars()
            .all(|character| character.is_ascii_alphanumeric()
                || character == '-'
                || character == '.'),
        "a proposed name may only use characters every file system accepts: {derived}"
    );
    assert!(derived.len() <= PACKAGE_STEM_LIMIT + PACKAGE_EXTENSION.len() + 1);

    // No plan selected, and a name with nothing usable in it, both fall back
    // rather than proposing a file called ".rspice-plan".
    assert_eq!(package_file_name(None), "simulation-plan.rspice-plan");
}

/// The number the dialog previews is the number the import creates.
///
/// Checked against the real transaction rather than against
/// `specification_count`'s own arithmetic, because the claim is not that the
/// helper adds up — it is that it mirrors the migration
/// `Workspace::import_plan_data` performs. A payload carrying only the
/// legacy list becomes that many governed definitions, and a count taken off
/// the governed field alone would have previewed zero of them.
#[test]
fn the_previewed_specification_count_is_what_the_import_creates() {
    let mut app = RSpiceApp::test_instance();
    app.state.provision_test_project_technology_contract();
    let id = app
        .state
        .model
        .sim_setup
        .stable_analysis_plan()
        .expect("the fixture has a stable plan")
        .id();
    let json =
        export_simulation_plan_package(&app.state.model, id).expect("the fixture's plan exports");
    let previewed = specification_count(
        &inspect_package(&json)
            .expect("its own export is accepted")
            .payload,
    );

    let (imported_id, _) = commit_import_simulation_plan(
        &mut crate::workbench::features::cx(&mut app),
        &json,
        "Imported specification count",
    )
    .expect("the fixture's own package imports");
    let owned = app
        .state
        .model
        .workspace
        .content
        .plan_data(imported_id)
        .expect("the imported plan owns a payload")
        .specification_definitions
        .len();
    assert_eq!(
        previewed, owned,
        "the import created {owned} specification(s) where the dialog \
         previewed {previewed}"
    );
}

/// The envelope line is one owner, so the export's package row and the
/// import's envelope row cannot come to state different contracts.
#[test]
fn the_envelope_label_states_the_format_and_the_version() {
    let label = envelope_label(exported_package().len());
    assert!(label.contains(SIMULATION_PLAN_PACKAGE_FORMAT), "{label}");
    assert!(
        label.contains(&SIMULATION_PLAN_PACKAGE_VERSION.to_string()),
        "{label}"
    );
}
