//! The Models workspace along the browser model-qualification job's path,
//! replayed through whole frames of the application: the palette's Library
//! manager and the packs view of a build that carries no cloud endpoints.

use std::sync::Arc;

use egui::accesskit::Role;

use crate::feature_tests::replay::Replay;
use crate::services::cloud_account::model_hub_transport::CloudModelHubTransportProvider;
use crate::services::model_hub::{ModelHubService, ModelHubStoreHandle};
use crate::state::model_hub::tests::{anchor_for, hub_signing_key};
use crate::state::model_hub::{MemoryModelHubStore, ModelHub};
use crate::workbench::RSpiceApp;

/// A session as a build made without cloud endpoints opens it: a pack store
/// that works and holds no catalog yet, behind the shell's own transport
/// provider, which has nowhere to fetch one from.
fn without_endpoints() -> RSpiceApp {
    let store = Arc::new(MemoryModelHubStore::new());
    let hub = ModelHub::open(
        anchor_for(&hub_signing_key()),
        Box::new(Arc::clone(&store)),
        None,
    )
    .expect("a memory hub opens");
    let mut app = RSpiceApp::test_instance();
    app.model_hub = ModelHubService::with_store(ModelHubStoreHandle::Memory(store), hub)
        .through(&CloudModelHubTransportProvider);
    app
}

/// Frames until the Models workspace has no model operation running, as the
/// browser job waits for the page to settle; what the last one published.
fn settled(replay: &mut Replay) -> Vec<crate::feature_tests::replay::Control> {
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(10);
    loop {
        let shown = replay.frames(Vec::new());
        if !replay.app.state.models.models_view.model_import_in_progress {
            return shown;
        }
        assert!(
            std::time::Instant::now() < deadline,
            "the model operation never finished"
        );
        std::thread::sleep(std::time::Duration::from_millis(20));
    }
}

/// Opening the Models workspace in a build that carries no cloud endpoints
/// starts no catalog fetch and raises no error, and the packs view says in
/// one quiet sentence that published packs are not available here.
///
/// The fetch could only fail, at the step that resolves the endpoints, so it
/// put an "Execution error" card above the catalog on every visit to the
/// browser IDE, whose build carries none.
#[test]
fn a_build_without_cloud_endpoints_fetches_no_catalog_and_reports_no_error() {
    let mut replay = Replay::of(without_endpoints());
    assert!(
        !replay.app.model_hub.reaches_catalog(),
        "a test build carries no cloud endpoints, as the browser IDE's build does not"
    );
    replay.frames(Vec::new());
    replay.choose_command("Library manager");
    let shown = settled(&mut replay);
    assert_eq!(
        replay.app.state.models.models_view.attempted_operation, None,
        "opening the catalog attempted a model operation"
    );
    assert!(
        !shown
            .iter()
            .any(|control| control.label == "Execution error"),
        "the catalog raised an error card"
    );

    replay.click("Installed packs  0", Role::RadioButton);
    let packs = settled(&mut replay);
    assert!(
        packs.iter().any(|control| {
            control
                .label
                .starts_with("Published model packs are not available in this build. ")
        }),
        "the packs view did not say why it is empty; it announces {:?}",
        packs
            .iter()
            .map(|control| control.label.as_str())
            .collect::<Vec<_>>()
    );
    assert!(
        !packs
            .iter()
            .any(|control| control.label == "Refresh catalog"),
        "a build with no catalog to reach offers to refresh one"
    );
    assert_eq!(
        replay.app.state.models.models_view.attempted_operation,
        None
    );
}
