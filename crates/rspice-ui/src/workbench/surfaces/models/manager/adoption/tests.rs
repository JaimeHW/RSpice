//! The latch gate: one answer per question, and never the previous one.
//!
//! Two claims, and both have to hold or the projection is worse than absent.
//! A question already answered must not be recomputed, or the paint path walks
//! two release part lists sixty times a second. A question that only *looks*
//! the same — same pack, same two version strings, different catalog — must not
//! be handed the earlier answer, or a reader is told what a catalog that is no
//! longer held used to say.

use super::*;

use crate::services::model_hub::{ModelHubService, ModelHubStoreHandle};
use crate::state::model_hub::tests::{
    BIAS_PART_ID, NEXT_VERSION, PACK_ID, PART_ID, StubTransport, VERSION, anchor_for,
    hub_signing_key, signed_snapshot_projecting, two_part_archive,
};
use crate::state::model_hub::{MemoryModelHubStore, ModelHub};
use crate::workbench::app_state::AppState;

const CAPABILITIES: [&str; 2] = ["subckt", "resistor"];

/// The store a session's hubs are opened over, kept so a test can open
/// another under the *fixture* anchor. `ModelHubStoreHandle::open` opens under
/// the shipped release key, which no fixture snapshot is signed with.
type Store = std::sync::Arc<MemoryModelHubStore>;

/// A hub over the shared store, under the fixture anchor.
fn open(store: &Store) -> ModelHub {
    ModelHub::open(
        anchor_for(&hub_signing_key()),
        Box::new(std::sync::Arc::clone(store)),
        None,
    )
    .expect("a memory hub opens")
}

/// A session holding 1.0.0 of the two-part pack, with a project pinned to it.
fn session(catalog: Vec<u8>, archive: &[u8]) -> (ModelHub, Store, AppState) {
    let store = Store::new(MemoryModelHubStore::new());
    let mut hub = open(&store);
    let transport = StubTransport::with_snapshot(catalog).serving(VERSION, archive.to_vec());
    hub.refresh_catalog(&transport).expect("catalog");
    hub.install(&transport, PACK_ID, VERSION).expect("install");

    let mut state = AppState::default();
    state.model_library_manager.clear();
    hub.add_part_to_project(&mut state.model_library_manager, PACK_ID, VERSION, PART_ID)
        .expect("the project retains the divider");
    hub.add_part_to_project(
        &mut state.model_library_manager,
        PACK_ID,
        VERSION,
        BIAS_PART_ID,
    )
    .expect("the project retains the bias network");
    state.workbench.models_view.selected_pack = Some(PACK_ID.to_owned());
    (hub, store, state)
}

/// Fetches and proves a later catalog, as opening the workspace would.
fn hold(hub: &mut ModelHub, catalog: Vec<u8>, archives: &[(&str, &[u8])]) {
    let mut transport = StubTransport::with_snapshot(catalog).at_generation(8);
    for (version, archive) in archives {
        transport = transport.serving(version, (*archive).to_vec());
    }
    hub.refresh_catalog(&transport)
        .expect("the later catalog proves");
}

/// The session service a workspace frame reads through.
fn service_of(store: &Store, hub: ModelHub) -> ModelHubService {
    ModelHubService::with_store(
        ModelHubStoreHandle::Memory(std::sync::Arc::clone(store)),
        hub,
    )
}

/// A catalog listing 1.0.0, and one listing both releases.
fn catalogs() -> (Vec<u8>, Vec<u8>, Vec<u8>, Vec<u8>) {
    let key = hub_signing_key();
    let archive = two_part_archive(&key, &CAPABILITIES, VERSION);
    let next = two_part_archive(&key, &CAPABILITIES, NEXT_VERSION);
    let held = signed_snapshot_projecting(&key, &[(VERSION, &archive, &CAPABILITIES)]);
    let offering = signed_snapshot_projecting(
        &key,
        &[
            (VERSION, &archive, &CAPABILITIES),
            (NEXT_VERSION, &next, &CAPABILITIES),
        ],
    );
    (archive, next, held, offering)
}

#[test]
fn an_unchanged_question_is_answered_from_the_latch_rather_than_recomputed() {
    let (archive, next, held, offering) = catalogs();
    let (mut hub, store, mut state) = session(held, &archive);
    hold(
        &mut hub,
        offering,
        &[(VERSION, &archive), (NEXT_VERSION, &next)],
    );
    let service = service_of(&store, hub);

    let catalog = hub::hub_catalog(&service, &state);
    refresh_release_diff(&service, &mut state, &catalog);
    let first = state
        .workbench
        .models_view
        .release_diff
        .clone()
        .expect("an update is on offer, so there is a diff");
    assert_eq!(first.key.from, VERSION);
    assert_eq!(first.key.to, NEXT_VERSION);

    // A value only a recomputation would destroy.
    state
        .workbench
        .models_view
        .release_diff
        .as_mut()
        .expect("the diff is held")
        .added
        .push("SENTINEL".to_owned());
    for _ in 0..3 {
        refresh_release_diff(&service, &mut state, &catalog);
    }
    assert_eq!(
        state
            .workbench
            .models_view
            .release_diff
            .as_ref()
            .expect("the diff is still held")
            .added,
        vec!["SENTINEL".to_owned()],
        "a question already answered must be read, not walked again"
    );
}

/// The collision this latch exists to survive.
///
/// Two catalogs publish the same pack at the same two versions and disagree
/// about what those releases contain. Nothing about the pack identity or the
/// version strings can tell them apart — only the content can, which is why the
/// key carries the snapshot digest and not a generation counter that a
/// wholesale replacement would simply carry over.
#[test]
fn a_catalog_replaced_wholesale_cannot_reuse_a_diff_it_did_not_earn() {
    let key = hub_signing_key();
    let (archive, next, held, offering) = catalogs();
    let (mut hub, store, mut state) = session(held, &archive);
    hold(
        &mut hub,
        offering,
        &[(VERSION, &archive), (NEXT_VERSION, &next)],
    );
    let service = service_of(&store, hub);

    let catalog = hub::hub_catalog(&service, &state);
    refresh_release_diff(&service, &mut state, &catalog);
    let before = state
        .workbench
        .models_view
        .release_diff
        .clone()
        .expect("the first catalog answers");
    assert!(
        before.added.is_empty(),
        "1.1.0 adds no part in the first catalog"
    );

    // A different catalog, publishing the same pack at the same two versions
    // and saying something else about 1.1.0.
    let extra = two_part_archive(&key, &CAPABILITIES, "1.2.0");
    let replacement = signed_snapshot_projecting(
        &key,
        &[
            (VERSION, &archive, &CAPABILITIES),
            (NEXT_VERSION, &extra, &CAPABILITIES),
        ],
    );
    let mut reopened = open(&store);
    hold(
        &mut reopened,
        replacement,
        &[(VERSION, &archive), (NEXT_VERSION, &extra)],
    );
    let service = service_of(&store, reopened);
    let replaced_catalog = hub::hub_catalog(&service, &state);
    let after_key = replaced_catalog
        .identity
        .as_ref()
        .expect("the replacement is held")
        .digest
        .clone();
    assert_ne!(
        after_key, before.key.catalog_digest,
        "the two catalogs must be indistinguishable by pack and version, or this test proves \
         nothing about the content half of the key"
    );

    refresh_release_diff(&service, &mut state, &replaced_catalog);
    let after = state
        .workbench
        .models_view
        .release_diff
        .as_ref()
        .expect("the replacement answers too");
    assert_eq!(
        after.key.pack_id, before.key.pack_id,
        "same pack, same two releases"
    );
    assert_eq!(after.key.from, before.key.from);
    assert_eq!(after.key.to, before.key.to);
    assert_eq!(
        after.key.catalog_digest, after_key,
        "and the answer names the catalog it was actually read from"
    );
}

/// A pack the project pinned nothing from is offered no adoption, and the
/// pane says only what the catalog says.
#[test]
fn a_pack_this_project_pinned_nothing_from_offers_no_adoption() {
    let (archive, next, held, offering) = catalogs();
    let (mut hub, store, mut state) = session(held, &archive);
    hold(
        &mut hub,
        offering,
        &[(VERSION, &archive), (NEXT_VERSION, &next)],
    );
    let service = service_of(&store, hub);
    state.model_library_manager.clear();

    let catalog = hub::hub_catalog(&service, &state);
    let row = hub::selected_row(&catalog, &state).expect("the pack is listed");
    assert!(
        row.pins.is_empty(),
        "nothing is pinned, so nothing can be adopted"
    );
    refresh_release_diff(&service, &mut state, &catalog);
    assert!(
        state.workbench.models_view.release_diff.is_some(),
        "the diff is still worth stating: an update is on offer"
    );
}
