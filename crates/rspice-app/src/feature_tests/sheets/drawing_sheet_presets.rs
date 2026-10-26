//! The custom sheet size dialogs over a whole application.

use base64::{Engine as _, engine::general_purpose::STANDARD};
use ed25519_dalek::{Signer as _, SigningKey};

use rspice_design_model::sheet_authoring::StartingFrame;
use rspice_design_model::sheet_package::{
    DrawingSheetPackageEncoding, DrawingSheetPresetPackage, DrawingSheetPresetPublisherSignature,
    build_package_with_options, encode_package_with_format, package_digest,
    package_signature_message,
};

use crate::state::pdk_config::{PdkPublisherTrustStore, TrustedPdkPublisherKey};
use crate::state::{
    AuthoredDrawingSheetSize, DesignManagementCatalog, DrawingSheetPreset,
    DrawingSheetPresetImportMappingKind, DrawingSheetPresetImportSkipReason,
    DrawingSheetPresetScope,
};
use crate::workbench::DrawingSheetPersonalPreferences;
use crate::workbench::app::RSpiceApp;
use crate::workbench::app::dialogs::drawing_sheet_presets::model::{
    ImportResolution, all_visible_presets,
};
use crate::workbench::app::dialogs::drawing_sheet_presets::render::{LibraryBodyAction, PresetKey};
use crate::workbench::app::dialogs::drawing_sheet_presets::*;
use crate::workbench::app_state::AppState;

/// The custom sheet size dialogs over the parts of `state`.
fn preset_dialogs(state: &mut AppState) -> PresetDialogs<'_> {
    PresetDialogs {
        dialogs: &mut state.sheets.drawing_sheet_presets,
        model: &mut state.model,
        workbench: &mut state.workbench,
        reveal_sheet_section: &mut state.sheets.reveal_sheet_section,
    }
}

/// Using a custom size from the library puts the open sheet on it, as one
/// sheet edit, closes the library and shows the sheet's properties.
#[test]
fn using_a_custom_size_puts_the_sheet_on_it_and_shows_its_properties() {
    let mut app = RSpiceApp::test_instance();
    let preset = DrawingSheetPreset {
        id: "lab-panel".to_owned(),
        name: "Lab panel".to_owned(),
        scope: DrawingSheetPresetScope::Project,
        format: model::custom_format("Lab panel", 250_000, 400_000, StartingFrame::IsoA).unwrap(),
    };
    let mut seeded = (*app.state.model.workspace.content.design_management).clone();
    seeded
        .publish_drawing_sheet_preset(seeded.revision(), preset.clone())
        .unwrap();
    app.state
        .model
        .workspace
        .content
        .replace_design_management(seeded)
        .unwrap();
    assert!(open_custom_sheet_size_library(
        &mut app.state.sheets.drawing_sheet_presets,
        &app.state.model
    ));

    preset_dialogs(&mut app.state)
        .handle_preset_library_action(LibraryBodyAction::Use(PresetKey::from(&preset)));

    let sheet = app.state.model.active_drawing_sheet();
    assert_eq!(sheet.format.authored_size.label(), "Lab panel");
    assert!(!app.state.sheets.drawing_sheet_presets.any_open());
    assert!(app.state.sheets.reveal_sheet_section);
    assert!(app.state.model.can_undo_project_design());
}

fn build_package(
    presets: impl IntoIterator<Item = DrawingSheetPreset>,
) -> Result<DrawingSheetPresetPackage, String> {
    build_package_with_options(presets, true, true)
}

fn encode_package(package: &DrawingSheetPresetPackage) -> Result<String, String> {
    encode_package_with_format(package, DrawingSheetPackageEncoding::CanonicalSchema1)
}

/// Sign a package and return the trust store that accepts it, so an
/// import test can exercise the real application path rather than a
/// bypass. The signature is produced through the shared contract, so a
/// change to what gets signed fails this test too.
fn authenticate_package_for_test(
    mut package: DrawingSheetPresetPackage,
) -> (DrawingSheetPresetPackage, PdkPublisherTrustStore) {
    let signing_key = SigningKey::from_bytes(&[0x53; 32]);
    package.publisher_signature = Some(DrawingSheetPresetPublisherSignature {
        publisher_id: "rspice-test-publisher".to_owned(),
        signing_key_id: "sheet-formats-2026".to_owned(),
        signature_base64: String::new(),
    });
    package.source_digest_sha256 =
        package_digest(&package).expect("test package contract is canonical");
    let signature = signing_key.sign(
        &package_signature_message(&package).expect("test package signature message is canonical"),
    );
    package
        .publisher_signature
        .as_mut()
        .expect("test publisher identity is present")
        .signature_base64 = STANDARD.encode(signature.to_bytes());
    let mut trust = PdkPublisherTrustStore::default();
    trust.keys.push(TrustedPdkPublisherKey {
        publisher_id: "rspice-test-publisher".to_owned(),
        key_id: "sheet-formats-2026".to_owned(),
        verifying_key: signing_key.verifying_key().to_bytes(),
        revoked: false,
    });
    (package, trust)
}

#[test]
fn transfer_identity_keeps_equal_ids_in_distinct_authorities() {
    let format =
        model::custom_format("Review strip", 210_000, 594_000, StartingFrame::IsoA).unwrap();
    let project = DrawingSheetPreset {
        id: "review-strip".to_owned(),
        name: "Review strip".to_owned(),
        scope: DrawingSheetPresetScope::Project,
        format: format.clone(),
    };
    let personal = DrawingSheetPreset {
        scope: DrawingSheetPresetScope::User,
        ..project.clone()
    };
    assert_ne!(transfer_identity(&project), transfer_identity(&personal));
}

#[test]
fn package_names_always_receive_json_extension() {
    assert_eq!(normalized_package_name("lab-sheets"), "lab-sheets.json");
    assert_eq!(
        normalized_package_name("LAB-SHEETS.JSON"),
        "LAB-SHEETS.JSON"
    );
}

#[test]
fn personal_default_counts_as_a_preset_reference() {
    let id = "personal-default-panel";
    let mut format = model::custom_format(
        "Personal default panel",
        250_000,
        400_000,
        StartingFrame::IsoA,
    )
    .unwrap();
    format = format
        .try_update(|draft| {
            if let AuthoredDrawingSheetSize::Custom { snapshot } = &mut draft.authored_size {
                snapshot.preset_id = Some(id.to_owned());
            }
        })
        .unwrap();
    let preset = DrawingSheetPreset {
        id: id.to_owned(),
        name: "Personal default panel".to_owned(),
        scope: DrawingSheetPresetScope::User,
        format: format.clone(),
    };
    let mut personal = DrawingSheetPersonalPreferences {
        default_format: format
            .try_update(|draft| {
                draft.inheritance = crate::state::DrawingSheetInheritance::UserDefault;
            })
            .unwrap(),
        presets: vec![preset],
    };
    personal.validate().unwrap();

    assert!(personal_default_references_preset(&personal, id));
    assert_eq!(
        usage_count(
            &DesignManagementCatalog::default(),
            &personal,
            DrawingSheetPresetScope::User,
            id,
        ),
        1
    );

    personal.presets.clear();
    assert!(personal.validate().is_err());
}

#[test]
fn drawing_sheet_preset_import_receipt_retains_reviewed_outcomes() {
    let mut app = RSpiceApp::test_instance();
    let existing = DrawingSheetPreset {
        id: "existing-review-strip".to_owned(),
        name: "Existing review strip".to_owned(),
        scope: DrawingSheetPresetScope::Project,
        format: model::custom_format(
            "Existing review strip",
            210_001,
            594_002,
            StartingFrame::IsoA,
        )
        .unwrap(),
    };
    let mut seeded = (*app.state.model.workspace.content.design_management).clone();
    seeded
        .publish_drawing_sheet_preset(seeded.revision(), existing.clone())
        .unwrap();
    app.state
        .model
        .workspace
        .content
        .replace_design_management(seeded)
        .unwrap();

    let imported = DrawingSheetPreset {
        id: "new-review-panel".to_owned(),
        name: "New review panel".to_owned(),
        scope: DrawingSheetPresetScope::User,
        format: model::custom_format("New review panel", 220_001, 610_002, StartingFrame::IsoA)
            .unwrap(),
    };
    let not_selected = DrawingSheetPreset {
        id: "not-selected-panel".to_owned(),
        name: "Not selected panel".to_owned(),
        scope: DrawingSheetPresetScope::User,
        format: model::custom_format("Not selected panel", 230_001, 620_002, StartingFrame::IsoA)
            .unwrap(),
    };
    let explicitly_skipped = DrawingSheetPreset {
        id: "explicitly-skipped-panel".to_owned(),
        name: "Explicitly skipped panel".to_owned(),
        scope: DrawingSheetPresetScope::Organization,
        format: model::custom_format(
            "Explicitly skipped panel",
            240_001,
            630_002,
            StartingFrame::IsoA,
        )
        .unwrap(),
    };
    let package = build_package([
        existing.clone(),
        imported.clone(),
        not_selected.clone(),
        explicitly_skipped.clone(),
    ])
    .unwrap();
    let (package, trust) = authenticate_package_for_test(package);
    app.state.model.pdk_config.publisher_trust_store = trust;
    let source = encode_package(&package).unwrap();

    assert!(open_custom_sheet_size_library(
        &mut app.state.sheets.drawing_sheet_presets,
        &app.state.model
    ));
    let personal = app
        .state
        .model
        .preferences
        .drawing_sheet_personal_preferences();
    let visible = all_visible_presets(
        &app.state.model.workspace.content.design_management,
        &personal,
    );
    preset_dialogs(&mut app.state)
        .review_import_source(&source, &visible)
        .unwrap();
    {
        let candidates = &mut app
            .state
            .sheets
            .drawing_sheet_presets
            .transfer
            .import_candidates;
        candidates
            .iter_mut()
            .find(|candidate| candidate.portable.stable_id == not_selected.id)
            .unwrap()
            .selected = false;
        let explicit = candidates
            .iter_mut()
            .find(|candidate| candidate.portable.stable_id == explicitly_skipped.id)
            .unwrap();
        explicit.selected = true;
        explicit.resolution = ImportResolution::Skip;
    }
    app.state.sheets.drawing_sheet_presets.transfer.json = source;

    let message = preset_dialogs(&mut app.state)
        .apply_preset_import()
        .unwrap()
        .unwrap();
    assert!(message.contains("durable receipt retained"));
    let settings = app
        .state
        .model
        .workspace
        .content
        .design_management
        .drawing_sheet_settings();
    assert!(settings.find_preset(&existing.id).is_some());
    assert!(settings.find_preset(&imported.id).is_some());
    assert!(settings.find_preset(&not_selected.id).is_none());
    assert!(settings.find_preset(&explicitly_skipped.id).is_none());

    let receipt = settings.preset_import_receipts.last().unwrap();
    assert_eq!(receipt.source_digest_sha256, package.source_digest_sha256);
    assert_eq!(receipt.source_schema, package.schema);
    assert_eq!(receipt.source_schema_version, package.version);
    assert_eq!(receipt.reviewed_candidate_count, 4);
    assert_eq!(receipt.selected_candidates.len(), 3);
    assert_eq!(receipt.mappings.len(), 2);
    assert_eq!(receipt.conflicts.len(), 1);
    assert_eq!(receipt.skipped_candidates.len(), 2);
    assert!(receipt.mappings.iter().any(|mapping| {
        mapping.source.preset_id == existing.id
            && mapping.target.preset_id == existing.id
            && mapping.kind == DrawingSheetPresetImportMappingKind::ExistingPreset
    }));
    assert!(receipt.mappings.iter().any(|mapping| {
        mapping.source.preset_id == imported.id
            && mapping.target.preset_id == imported.id
            && mapping.kind == DrawingSheetPresetImportMappingKind::CreatedProjectPreset
    }));
    assert!(receipt.skipped_candidates.iter().any(|skip| {
        skip.source.preset_id == not_selected.id
            && skip.reason == DrawingSheetPresetImportSkipReason::NotSelected
    }));
    assert!(receipt.skipped_candidates.iter().any(|skip| {
        skip.source.preset_id == explicitly_skipped.id
            && skip.reason == DrawingSheetPresetImportSkipReason::ExplicitlySkipped
    }));
}
