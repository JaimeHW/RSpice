//! Publish to web over a whole application: the review build it scopes,
//! what an imported model discloses, the dialog the command opens, and a
//! failure said once its dialog has closed.

use crate::services::cloud_account::{
    CLOUD_PUBLISHING_FEATURE, CloudAccountAvailability, PublishState,
};
use crate::workbench::app::RSpiceApp;
use crate::workbench::app::dialogs::publish_web::*;
use crate::workbench::app_state::AppState;
use crate::workbench::framework::feature::PublicationDraft;

/// What a review build reads, lent from the whole state.
fn snapshot_source(state: &AppState) -> SnapshotSource<'_> {
    SnapshotSource {
        hardcopy: &*state.hardcopy,
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

#[test]
fn retained_import_disclosure_only_applies_when_the_netlist_is_included() {
    let mut state = AppState::default();
    let mut library = crate::state::model_library::ModelLibrary::new("uploaded");
    library.source_authority = crate::state::model_library::ModelSourceAuthority::RetainedImport {
        source_id: crate::product::ModelSourceId::new(),
        digest: crate::product::ContentDigest::from_bytes([7; 32]),
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
    use crate::services::cloud_account::{
        CloudAccountService, CloudSessionPhase, CloudSessionSnapshot, EntitlementSummary,
        PrincipalSummary,
    };
    use crate::workbench::cloud::commands::CloudCommand;

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
    use crate::services::cloud_account::{CloudAccountService, CloudSessionSnapshot};
    use crate::ui::widgets::ToastKind;

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
