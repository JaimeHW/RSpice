//! Publish to web over a whole application: the review build it scopes,
//! what an imported model discloses, the dialog the command opens, and a
//! failure said once its dialog has closed.

use crate::workbench::app::RSpiceApp;
use crate::workbench::app_state::AppState;
use rspice_app_cloud::services::cloud_account::{
    CLOUD_PUBLISHING_FEATURE, CloudAccountAvailability, PublishState,
};
use rspice_app_cloud::workbench::app::dialogs::publish_web::*;
use rspice_app_workbench::workbench::framework::feature::PublicationDraft;

/// What a review build reads, lent from the whole state.
fn snapshot_source(state: &AppState) -> SnapshotSource<'_> {
    SnapshotSource {
        hardcopy: &*state.hardcopy,
        design: &*state.design,
        model: &state.model,
        workbench: &state.workbench,
        results: &*state.results,
        runs: &state.runs,
    }
}

#[test]
fn empty_project_scope_reports_the_builder_refusal() {
    let state = AppState::default();
    let scope = build_scope(
        snapshot_source(&state),
        &PublicationDraft {
            title: "Untitled project".to_owned(),
            author_display: "Anonymous".to_owned(),
            created_utc: "2026-08-06T00:00:00Z".to_owned(),
            ..Default::default()
        },
    );
    let reason = scope.expect_err("empty project cannot publish");
    assert!(reason.contains("no schematic"), "{reason}");
}

/// A published design letters the part labels the author's canvas letters,
/// names and values, values only, or neither, as a print of it does.
#[test]
fn a_published_design_letters_the_part_labels_the_canvas_shows() {
    use rspice_design::schematic::component_display::SchematicParameterLabelVisibility;
    use rspice_publication_contract::Primitive;

    let mut state = AppState::default();
    state.model.schematic.add_component(
        rspice_app_state::state::ComponentType::Resistor,
        rspice_app_state::state::Point::new(0, 0),
    );
    let reference = state.model.workspace.content.active_view.clone();
    state.workbench.documents.activate(
        rspice_app_workbench::workbench::state::workbench_session::WorkspaceDocumentId::CellView(
            reference,
        ),
    );
    let part = state.model.schematic.document().components[0].clone();
    let letters = |state: &AppState| -> Vec<String> {
        let snapshot = snapshot_source(state)
            .build(&PublicationDraft {
                title: "Divider".to_owned(),
                author_display: "Ada".to_owned(),
                created_utc: "2026-08-06T00:00:00Z".to_owned(),
                include_schematic: true,
                ..Default::default()
            })
            .expect("a placed part publishes");
        snapshot
            .schematic
            .expect("the schematic publishes")
            .sheets
            .into_iter()
            .flat_map(|sheet| sheet.scene.groups)
            .flat_map(|group| group.primitives)
            .filter_map(|primitive| match primitive {
                Primitive::Text(text) => Some(text.text),
                Primitive::Path(_) => None,
            })
            .collect()
    };
    let shows = |letters: &[String], text: &str| letters.iter().any(|letter| letter == text);

    for (labels, name, value) in [
        (
            SchematicParameterLabelVisibility::NamesAndValues,
            true,
            true,
        ),
        (SchematicParameterLabelVisibility::ValuesOnly, false, true),
        (SchematicParameterLabelVisibility::Hidden, false, false),
    ] {
        state.design.schematic_visibility.parameter_labels = labels;
        let published = letters(&state);
        assert_eq!(
            shows(&published, &part.name),
            name,
            "{labels:?}: {published:?}"
        );
        assert_eq!(
            shows(&published, &part.value),
            value,
            "{labels:?}: {published:?}"
        );
    }
}

#[test]
fn retained_import_disclosure_only_applies_when_the_netlist_is_included() {
    let mut state = AppState::default();
    let mut library = rspice_app_state::state::model_library::ModelLibrary::new("uploaded");
    library.source_authority =
        rspice_app_state::state::model_library::ModelSourceAuthority::RetainedImport {
            source_id: rspice_app_state::product::ModelSourceId::new(),
            digest: rspice_app_state::product::ContentDigest::from_bytes([7; 32]),
        };
    state.model.model_library_manager.add_library(library);

    assert_eq!(imported_model_source_count(&state.model, true), 1);
    assert_eq!(imported_model_source_count(&state.model, false), 0);
}

/// Publish to web opens its dialog within the command's own call, over a
/// review build of the open project that the hardcopy feature makes, and
/// credits the signed-in account.
#[test]
fn publish_to_web_opens_its_dialog_over_a_review_build_of_the_project() {
    use rspice_app_cloud::services::cloud_account::{
        CloudAccountService, CloudSessionPhase, CloudSessionSnapshot, EntitlementSummary,
        PrincipalSummary,
    };
    use rspice_app_cloud::workbench::cloud::commands::CloudCommand;

    let mut app = RSpiceApp::test_instance();
    app.cloud.account = CloudAccountService::with_snapshot(
        CloudAccountAvailability::Native,
        CloudSessionSnapshot {
            phase: CloudSessionPhase::Active,
            principal: Some(PrincipalSummary {
                email: Some("ada@example.com".to_owned()),
                display_name: Some("Ada".to_owned()),
            }),
            entitlements: vec![EntitlementSummary {
                status: "active".to_owned(),
                granted_features: vec![CLOUD_PUBLISHING_FEATURE.to_owned()],
                valid_until: None,
            }],
            ..CloudSessionSnapshot::default()
        },
    );
    app.execute_command_key(CloudCommand::PublishToWeb.key());

    let dialog = &app.state.cloud.publish_web;
    assert!(dialog.open);
    assert!(!dialog.submitted);
    assert_eq!(dialog.author_credit, "Ada");
    let review = build_scope(snapshot_source(&app.state), &publication_draft(dialog));
    assert_eq!(dialog.scope, Some(review));
}

/// A publish that fails after its dialog was closed is said, once. One
/// the open dialog already showed is not said again.
#[test]
fn a_publish_that_fails_after_its_dialog_closed_is_reported_once() {
    use rspice_app_cloud::services::cloud_account::{CloudAccountService, CloudSessionSnapshot};
    use rspice_app_workbench::ui::widgets::ToastKind;

    let failed = |message: &str| {
        CloudAccountService::with_snapshot(
            CloudAccountAvailability::Native,
            CloudSessionSnapshot {
                publish: Some(PublishState::Failed {
                    message: message.to_owned(),
                }),
                ..CloudSessionSnapshot::default()
            },
        )
    };

    let ctx = egui::Context::default();
    let mut app = RSpiceApp::test_instance();
    app.cloud.account = failed("The page could not be rendered.");
    app.state.cloud.publish_web.open = true;
    app.state.cloud.publish_web.submitted = true;
    app.record_publish_receipt();
    app.state.drain_notices(&ctx);
    assert!(
        app.state.workbench.toasts.activity().is_empty(),
        "the open dialog shows the failure itself"
    );

    app.state.cloud.publish_web.open = false;
    app.record_publish_receipt();
    app.record_publish_receipt();
    app.state.drain_notices(&ctx);
    let activity = app.state.workbench.toasts.activity();
    assert_eq!(activity.len(), 1);
    assert_eq!(activity[0].title(), "Publish failed");
    assert_eq!(activity[0].message(), "The page could not be rendered.");
    assert_eq!(activity[0].kind(), ToastKind::Error);
}
