//! The Models workspace along the browser model-qualification job's path,
//! replayed through whole frames of the application: the palette's Library
//! manager, the catalog's rows pressed where their announcements say they
//! are and walked with the arrow keys, and the packs view of a build that
//! carries no cloud endpoints.

use std::sync::Arc;

use egui::accesskit::Role;
use egui::{Key, Modifiers};

use crate::feature_tests::replay::{Replay, only};
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

/// A catalog row pressed at the centre of its announced bounds takes the
/// selection and the keyboard, and the arrow keys then walk the catalog a row
/// at a time, past the rows the table first showed.
#[test]
fn the_arrow_keys_walk_the_catalog_from_a_pressed_row() {
    let mut replay = Replay::new();
    replay.frames(Vec::new());
    replay.choose_command("Library manager");
    only(
        &replay.frames(Vec::new()),
        "RSPICE_DIODE in RSpice Foundation",
        Role::Button,
    );
    let order: Vec<String> = replay
        .frames(Vec::new())
        .iter()
        .filter(|control| control.role == Role::Button)
        .filter_map(|control| control.label.strip_suffix(" in RSpice Foundation"))
        .map(str::to_owned)
        .collect();
    assert!(order.len() >= 3, "the catalog shows its rows: {order:?}");

    replay.click("RSPICE_NMOS in RSpice Foundation", Role::Button);
    assert_eq!(
        replay.app.state.models.selected_model.as_deref(),
        Some("RSPICE_NMOS"),
        "the pressed row did not take the selection"
    );
    let at = order
        .iter()
        .position(|model| model == "RSPICE_NMOS")
        .expect("the NMOS row is listed");
    replay.key(Key::ArrowDown, Modifiers::NONE);
    assert_eq!(
        replay.app.state.models.selected_model.as_deref(),
        Some(order[at + 1].as_str()),
        "the down arrow moved the selection to the next row"
    );
    replay.key(Key::ArrowUp, Modifiers::NONE);
    replay.key(Key::ArrowUp, Modifiers::NONE);
    assert_eq!(
        replay.app.state.models.selected_model.as_deref(),
        Some(order[at - 1].as_str()),
        "the up arrow moved it back past the pressed row"
    );

    // Walking on past the last row the table first showed scrolls the rest in.
    let models = replay
        .app
        .state
        .model
        .model_library_manager
        .get_library("RSpice Foundation")
        .map(|library| library.model_count())
        .expect("the foundation library is loaded");
    for _ in 0..models {
        replay.key(Key::ArrowDown, Modifiers::NONE);
    }
    let last = replay.app.state.models.selected_model.clone();
    let shown = replay.frames(Vec::new());
    assert!(
        last.as_deref().is_some_and(|model| {
            shown
                .iter()
                .any(|control| control.label == format!("{model} in RSpice Foundation"))
        }),
        "the selected row {last:?} is on screen"
    );
    assert!(
        !order.contains(last.as_ref().expect("a row is selected")),
        "the arrows stopped at {last:?}, inside the rows first shown {order:?}"
    );
}
