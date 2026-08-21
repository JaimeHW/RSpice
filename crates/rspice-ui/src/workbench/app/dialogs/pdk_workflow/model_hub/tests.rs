//! The Model Hub operation-machine gate.
//!
//! Every test here drives a real request end to end against a real signed
//! archive and a real signed snapshot — the M3 fixtures, built by the same
//! packer and signer the service uses — and then publishes the result into a
//! real [`AppState`]. Only the network is a stub. A test that passes has
//! therefore exercised the actual signature check, the actual staging and
//! rename, the actual project retention, and the actual placement arming.

use crate::state::model_hub::tests::{
    NEXT_VERSION, PACK_ID, StubTransport, VERSION, anchor_for, hub_signing_key, signed_archive,
    signed_archive_at, signed_snapshot, signed_snapshot_of,
};
use crate::state::model_hub::{
    MemoryModelHubStore, ModelHub, ModelHubStore, PartPlacement, PartProvenance, PartState,
};
use crate::state::model_library::ModelLibraryManager;
use crate::state::{ComponentType, Tool};
use crate::workbench::app_state::AppState;
use crate::workbench::state::ModelsOperationalState;

use super::{ModelHubOutput, ModelHubRequest, execute, publish_model_hub_output};

const PART: &str = "RSPICE_PROVING_DIV";
const CAPABILITIES: [&str; 2] = ["subckt", "resistor"];

/// A hub over shared memory, plus the handle a session would hold.
///
/// The store is shared rather than copied so a reload sees what an operation
/// wrote, which is exactly the relationship the session and its workers have.
fn fixture_hub(
    key: &rspice_pack::SigningKey,
) -> (
    ModelHub,
    crate::services::model_hub::ModelHubStoreHandle,
    std::sync::Arc<MemoryModelHubStore>,
) {
    let store = std::sync::Arc::new(MemoryModelHubStore::new());
    let hub = ModelHub::open(
        anchor_for(key),
        Box::new(std::sync::Arc::clone(&store)),
        None,
    )
    .expect("a memory hub opens");
    let handle =
        crate::services::model_hub::ModelHubStoreHandle::Memory(std::sync::Arc::clone(&store));
    (hub, handle, store)
}

/// A hub whose catalog is already cached and whose pack is already installed.
fn installed_hub(
    key: &rspice_pack::SigningKey,
    transport: &StubTransport,
) -> (
    ModelHub,
    crate::services::model_hub::ModelHubStoreHandle,
    std::sync::Arc<MemoryModelHubStore>,
) {
    let (mut hub, handle, store) = fixture_hub(key);
    hub.refresh_catalog(transport).expect("catalog");
    hub.install(transport, PACK_ID, VERSION).expect("install");
    (hub, handle, store)
}

fn open_project() -> AppState {
    let mut state = AppState::default();
    state.project_lifecycle.project_open = true;
    state.model_library_manager.clear();
    state
}

/// Publishes a completed operation into a session, as the frame loop does.
fn publish(
    state: &mut AppState,
    handle: crate::services::model_hub::ModelHubStoreHandle,
    hub: ModelHub,
    request: &ModelHubRequest,
    output: ModelHubOutput,
) {
    let ctx = egui::Context::default();
    let mut service = crate::services::model_hub::ModelHubService::with_store(handle, hub);
    publish_model_hub_output(&ctx, state, &mut service, request, output);
}

#[test]
fn fetching_the_catalog_proves_it_and_changes_no_project_state() {
    let key = hub_signing_key();
    let archive = signed_archive(&key, &CAPABILITIES);
    let snapshot = signed_snapshot(&key, &archive, &CAPABILITIES, VERSION);
    let transport = StubTransport::with_snapshot(snapshot);
    let (mut hub, handle, _store) = fixture_hub(&key);
    let mut state = open_project();
    let revision_before = state.workspace.project.revision().get();

    let request = ModelHubRequest::FetchSnapshot;
    let output = execute(
        &mut hub,
        state.model_library_manager.clone(),
        &request,
        &transport,
    )
    .expect("the catalog fetch succeeds");
    assert!(output.part.is_none(), "a refresh retains no part");
    assert!(
        output.receipt.contains("generation 7"),
        "the receipt names the generation it proved: {}",
        output.receipt
    );
    assert!(hub.snapshot().is_some());

    publish(&mut state, handle, hub, &request, output);
    assert_eq!(
        state.workbench.models_view.operational_state,
        ModelsOperationalState::Ready
    );
    assert!(
        state
            .workbench
            .models_view
            .action_receipt
            .as_ref()
            .unwrap()
            .is_ok()
    );
    assert_eq!(
        state.workspace.project.revision().get(),
        revision_before,
        "proving a catalog is not a project edit"
    );
}

#[test]
fn installing_a_release_retains_its_part_pins_it_and_arms_the_placement() {
    let key = hub_signing_key();
    let archive = signed_archive(&key, &CAPABILITIES);
    let snapshot = signed_snapshot(&key, &archive, &CAPABILITIES, VERSION);
    let transport = StubTransport::with_snapshot(snapshot).serving(VERSION, archive.clone());
    let (mut hub, memory_handle, _store) = fixture_hub(&key);
    hub.refresh_catalog(&transport).expect("catalog");
    let mut state = open_project();

    let request = ModelHubRequest::InstallPack {
        pack_id: PACK_ID.to_owned(),
        version: VERSION.to_owned(),
        part: Some(PART.to_owned()),
    };
    // The browser's store keeps pack sources in memory and has no importable
    // path at all, so this request completing is the whole of what makes the
    // shelf's one gesture available on that build.
    let browser = execute(
        &mut hub,
        state.model_library_manager.clone(),
        &request,
        &transport,
    )
    .expect("a memory store completes the install and the retention");
    let browser_part = browser.part.as_ref().expect("the part was retained");
    assert_eq!(browser_part.part_id, PART);
    let browser_placement = browser_part.placement.clone();
    assert_eq!(hub.installed().len(), 1);
    let mut browser_state = open_project();
    publish(&mut browser_state, memory_handle, hub, &request, browser);
    assert_eq!(
        browser_state
            .model_library_manager
            .libraries_sorted()
            .into_iter()
            .find_map(|library| library.pack_pin.clone())
            .expect("the browser project holds a pinned library")
            .archive_sha256,
        rspice_pack::sha256_hex(&archive)
    );
    assert_eq!(
        browser_state.schematic.tool,
        Tool::Place(ComponentType::CellInstance),
        "the browser arms the same placement the desktop does"
    );

    // The same request against a filesystem store completes the whole gesture.
    let tree = std::env::temp_dir().join(format!("rspice-m4-install-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&tree);
    let store = crate::state::model_hub::FilesystemModelHubStore::new(&tree);
    let mut hub = ModelHub::open(
        anchor_for(&key),
        Box::new(store.clone()),
        Some(tree.join("packs")),
    )
    .expect("a filesystem hub opens");
    hub.refresh_catalog(&transport).expect("catalog");
    let handle = crate::services::model_hub::ModelHubStoreHandle::Filesystem {
        store,
        root: tree.join("packs"),
    };
    let output = execute(
        &mut hub,
        state.model_library_manager.clone(),
        &request,
        &transport,
    )
    .expect("the install and retention succeed");
    let part = output.part.as_ref().expect("the part was retained");
    assert_eq!(part.part_id, PART);
    let placement = part
        .placement
        .as_ref()
        .expect("a part added for placement is armed");
    assert_eq!(
        placement.symbol_reference(),
        "rspice.library.cell_instance",
        "a subcircuit part is drawn as a cell instance"
    );
    assert_eq!(
        part.placement, browser_placement,
        "the store a pack came through does not change what is placed"
    );

    publish(&mut state, handle, hub, &request, output);
    let library = state
        .model_library_manager
        .libraries_sorted()
        .into_iter()
        .find(|library| library.pack_pin.is_some())
        .expect("the project holds a pinned library");
    let pin = library.pack_pin.as_ref().expect("the pin");
    assert_eq!(pin.pack_id, PACK_ID);
    assert_eq!(pin.pack_version, VERSION);
    assert_eq!(pin.part_id, PART);
    assert_eq!(pin.archive_sha256, rspice_pack::sha256_hex(&archive));

    assert_eq!(
        state.schematic.tool,
        Tool::Place(ComponentType::CellInstance),
        "the placement is armed on the cursor"
    );
    let armed = state
        .schematic
        .pending_library_cell
        .as_ref()
        .expect("an armed cell binding");
    assert_eq!(armed.cell, PART);
    assert_eq!(armed.terminal_order, ["IN".to_owned(), "OUT".to_owned()]);
    assert!(armed.interface_bound, "the pins come from the manifest");
    assert_eq!(
        state.workbench.models_view.operational_state,
        ModelsOperationalState::Ready
    );
    let _ = std::fs::remove_dir_all(&tree);
}

/// The update flow, all the way through, on a store whose removal is visible.
///
/// A filesystem store is used deliberately: "the older release was removed"
/// is a claim about a directory, and only a store with directories can be
/// asked whether one is gone. The project pinned to the older release is
/// carried through the update and then *solved*, because the divider ratio is
/// the one fact no bookkeeping error could produce by accident — 1/2 is the
/// 1.0.0 source and 1/4 is the 1.1.0 source, and the project must still be
/// running the bytes it retained.
#[test]
fn updating_installs_the_newer_release_and_removes_the_older() {
    let key = hub_signing_key();
    let archive = signed_archive(&key, &CAPABILITIES);
    let next_archive = signed_archive_at(&key, &CAPABILITIES, NEXT_VERSION);
    let snapshot = signed_snapshot(&key, &archive, &CAPABILITIES, VERSION);
    let transport = StubTransport::with_snapshot(snapshot).serving(VERSION, archive.clone());

    let tree = std::env::temp_dir().join(format!(
        "rspice-m4-update-{}-{:?}",
        std::process::id(),
        std::thread::current().id()
    ));
    let _ = std::fs::remove_dir_all(&tree);
    let packs = tree.join("packs");
    let store = crate::state::model_hub::FilesystemModelHubStore::new(&tree);
    let mut hub = ModelHub::open(
        anchor_for(&key),
        Box::new(store.clone()),
        Some(packs.clone()),
    )
    .expect("a filesystem hub opens");
    let handle = crate::services::model_hub::ModelHubStoreHandle::Filesystem {
        store,
        root: packs.clone(),
    };
    hub.refresh_catalog(&transport).expect("catalog");
    hub.install(&transport, PACK_ID, VERSION).expect("install");
    assert!(packs.join(PACK_ID).join(VERSION).is_dir());

    // A project takes the part before the update, so the pin under test names
    // a release this machine is about to stop holding.
    let mut pinned = ModelLibraryManager::new();
    hub.add_part_to_project(&mut pinned, PACK_ID, VERSION, PART)
        .expect("the project retains the 1.0.0 part");

    // A later catalog generation publishes 1.1.0 beside it.
    let newer_snapshot = signed_snapshot_of(
        &key,
        &[
            (VERSION, &archive, &CAPABILITIES),
            (NEXT_VERSION, &next_archive, &CAPABILITIES),
        ],
    );
    let newer = StubTransport::with_snapshot(newer_snapshot)
        .at_generation(8)
        .serving(VERSION, archive.clone())
        .serving(NEXT_VERSION, next_archive.clone());
    hub.refresh_catalog(&newer).expect("the newer catalog");
    assert_eq!(
        hub.part_index(&[])
            .into_iter()
            .find(|row| matches!(row.provenance, PartProvenance::InstalledPack { .. }))
            .expect("the installed row")
            .state,
        PartState::UpdateAvailable {
            installed: VERSION.to_owned(),
            latest: NEXT_VERSION.to_owned(),
        }
    );

    let request = ModelHubRequest::UpdatePack {
        pack_id: PACK_ID.to_owned(),
        installed: VERSION.to_owned(),
        latest: NEXT_VERSION.to_owned(),
    };
    let output = execute(&mut hub, ModelLibraryManager::new(), &request, &newer)
        .expect("the update installs the newer release");
    assert!(output.part.is_none(), "an update retains no part");
    assert!(
        output.receipt.contains(NEXT_VERSION) && output.receipt.contains("was removed"),
        "the receipt names what it installed and what it removed: {}",
        output.receipt
    );

    // Installed 1.1.0, and the 1.0.0 directory is gone rather than orphaned.
    assert_eq!(hub.installed().len(), 1);
    assert_eq!(hub.installed()[0].version(), NEXT_VERSION);
    assert_eq!(
        hub.installed()[0].archive_sha256,
        rspice_pack::sha256_hex(&next_archive)
    );
    assert!(packs.join(PACK_ID).join(NEXT_VERSION).is_dir());
    assert!(!packs.join(PACK_ID).join(VERSION).exists());
    hub.verify_installed(PACK_ID, NEXT_VERSION)
        .expect("the newer release re-proves under the anchor");

    // The provider row flips to the release this machine now holds, and the
    // superseded version is not still offered as something to install.
    let rows = hub.part_index(&[]);
    let row = rows
        .iter()
        .find(|row| matches!(row.provenance, PartProvenance::InstalledPack { .. }))
        .expect("the installed row");
    assert_eq!(
        row.provenance,
        PartProvenance::InstalledPack {
            pack_id: PACK_ID.to_owned(),
            version: NEXT_VERSION.to_owned(),
        }
    );
    assert_eq!(row.state, PartState::Installed);
    assert!(rows.iter().any(|row| {
        row.provenance
            == PartProvenance::RemoteRelease {
                pack_id: PACK_ID.to_owned(),
                version: VERSION.to_owned(),
            }
    }));

    let mut state = open_project();
    publish(&mut state, handle, hub, &request, output);
    assert_eq!(
        state.workbench.models_view.operational_state,
        ModelsOperationalState::Ready
    );
    assert!(state.schematic.pending_library_cell.is_none());

    // The project pinned to 1.0.0 reopens and solves the 1.0.0 circuit, with
    // only 1.1.0 installed anywhere on this machine.
    let reloaded: ModelLibraryManager =
        serde_json::from_str(&serde_json::to_string(&pinned).expect("the pinned project saves"))
            .expect("the pinned project reopens");
    let pin = reloaded
        .libraries_sorted()
        .into_iter()
        .find_map(|library| library.pack_pin.clone())
        .expect("the pin survives the round trip");
    assert_eq!(pin.pack_version, VERSION);
    assert_eq!(pin.archive_sha256, rspice_pack::sha256_hex(&archive));
    let cards = reloaded
        .reference_process_model_cards(crate::product::ProcessCorner::TT)
        .expect("the retained snapshot materializes after the update");
    let deck = format!(
        "model hub update deck\n{}\nV1 IN 0 1\nX1 IN OUT {PART}\n.op\n.end\n",
        cards.join("\n")
    );
    let netlist = rspice_core::Netlist::parse(&deck).expect("the retained deck parses");
    let solved = rspice_core::Engine::new(rspice_core::SimulationConfig::default())
        .run_dc_op(&netlist)
        .expect("the retained bytes still solve after their release was replaced");
    let out = solved
        .try_voltage_named("OUT")
        .expect("the divider output is a solved node");
    assert!(
        (out - 0.5).abs() < 1.0e-9,
        "the pinned project runs the 1.0.0 divider, not the 1.1.0 one: V(OUT)={out}"
    );
    let _ = std::fs::remove_dir_all(&tree);
}

/// A failed update leaves the release this machine already holds.
#[test]
fn a_refused_update_never_removes_what_it_could_not_replace() {
    let key = hub_signing_key();
    let archive = signed_archive(&key, &CAPABILITIES);
    let next_archive = signed_archive_at(&key, &CAPABILITIES, NEXT_VERSION);
    let snapshot = signed_snapshot(&key, &archive, &CAPABILITIES, VERSION);
    let transport = StubTransport::with_snapshot(snapshot).serving(VERSION, archive.clone());
    let (mut hub, handle, _store) = installed_hub(&key, &transport);
    assert_eq!(hub.installed().len(), 1);

    // The catalog lists 1.1.0 honestly, and the service serves the 1.0.0
    // bytes under it. The archive digest is settled against the *signed
    // snapshot*, so the substitution is refused before anything is removed.
    let newer_snapshot = signed_snapshot_of(
        &key,
        &[
            (VERSION, &archive, &CAPABILITIES),
            (NEXT_VERSION, &next_archive, &CAPABILITIES),
        ],
    );
    let liar = StubTransport::with_snapshot(newer_snapshot)
        .at_generation(8)
        .serving(NEXT_VERSION, archive.clone());
    hub.refresh_catalog(&liar).expect("the newer catalog");

    let request = ModelHubRequest::UpdatePack {
        pack_id: PACK_ID.to_owned(),
        installed: VERSION.to_owned(),
        latest: NEXT_VERSION.to_owned(),
    };
    let refusal = execute(&mut hub, ModelLibraryManager::new(), &request, &liar)
        .expect_err("bytes that are not the release they are served as are refused");
    assert!(
        refusal.contains("expected"),
        "the refusal names the mismatch: {refusal}"
    );
    assert_eq!(
        hub.installed().len(),
        1,
        "the installed release survives a failed update"
    );
    assert_eq!(hub.installed()[0].version(), VERSION);

    let mut state = open_project();
    publish(
        &mut state,
        handle,
        hub,
        &ModelHubRequest::FetchSnapshot,
        ModelHubOutput {
            receipt: "unchanged".to_owned(),
            part: None,
        },
    );
    assert!(state.schematic.pending_library_cell.is_none());
}

#[test]
fn removing_a_release_leaves_a_project_that_retained_a_part_working() {
    let key = hub_signing_key();
    let archive = signed_archive(&key, &CAPABILITIES);
    let snapshot = signed_snapshot(&key, &archive, &CAPABILITIES, VERSION);
    let transport = StubTransport::with_snapshot(snapshot).serving(VERSION, archive);
    let (mut hub, handle, _store) = installed_hub(&key, &transport);

    let request = ModelHubRequest::RemovePack {
        pack_id: PACK_ID.to_owned(),
        version: VERSION.to_owned(),
    };
    let output = execute(&mut hub, ModelLibraryManager::new(), &request, &transport)
        .expect("the removal succeeds");
    assert!(output.part.is_none());
    assert!(hub.installed().is_empty());
    assert!(
        output.receipt.contains("Projects keep the bytes"),
        "the receipt says what removal does not touch: {}",
        output.receipt
    );
    // The catalog still publishes it, so the row becomes available again.
    assert!(
        hub.part_index(&[])
            .iter()
            .any(|row| row.part_id == PART && row.state == PartState::Available)
    );

    let mut state = open_project();
    publish(&mut state, handle, hub, &request, output);
    assert_eq!(
        state.workbench.models_view.operational_state,
        ModelsOperationalState::Ready
    );

    // Removing something that is not there is a refusal, not a silent success.
    let (mut empty, _handle, _store) = fixture_hub(&key);
    assert!(
        execute(&mut empty, ModelLibraryManager::new(), &request, &transport)
            .expect_err("removing an absent release is refused")
            .contains("already not installed")
    );
}

#[test]
fn a_failed_transfer_leaves_no_partial_state_and_reports_the_failure() {
    let key = hub_signing_key();
    let archive = signed_archive(&key, &CAPABILITIES);
    let snapshot = signed_snapshot(&key, &archive, &CAPABILITIES, VERSION);
    let transport = StubTransport::with_snapshot(snapshot)
        .serving(VERSION, archive)
        .failing_archive_fetch();
    let (mut hub, _handle, store) = fixture_hub(&key);
    hub.refresh_catalog(&transport).expect("catalog");
    let mut state = open_project();
    let revision_before = state.workspace.project.revision().get();

    let request = ModelHubRequest::InstallPack {
        pack_id: PACK_ID.to_owned(),
        version: VERSION.to_owned(),
        part: Some(PART.to_owned()),
    };
    let failure = execute(
        &mut hub,
        state.model_library_manager.clone(),
        &request,
        &transport,
    )
    .expect_err("a failed download installs nothing");
    assert!(
        failure.contains("could not be reached"),
        "the failure is reported as offline: {failure}"
    );
    assert!(hub.installed().is_empty(), "nothing was installed");
    assert_eq!(
        store.installed_packs().expect("store reads").len(),
        0,
        "and nothing reached the store either"
    );

    // The frame loop turns that into a toast, a console error, and a typed
    // operational state — and touches no project state on the way.
    let ctx = egui::Context::default();
    super::emit_model_hub_errors(&ctx, &mut state, vec![failure]);
    assert_eq!(
        state.workbench.models_view.operational_state,
        ModelsOperationalState::Offline
    );
    assert!(
        state
            .workbench
            .models_view
            .action_receipt
            .as_ref()
            .unwrap()
            .is_err()
    );
    assert_eq!(state.workspace.project.revision().get(), revision_before);
    assert!(state.schematic.pending_library_cell.is_none());
    assert!(state.schematic.pending_part_model.is_none());
    assert!(
        state
            .model_library_manager
            .libraries_sorted()
            .iter()
            .all(|library| library.pack_pin.is_none()),
        "no project library was pinned to a release that never installed"
    );
}

/// The whole shelf gesture, once, in the order a user performs it, on the
/// store a desktop session has.
#[test]
fn the_acceptance_sequence_runs_end_to_end_and_survives_uninstalling_the_pack() {
    let key = hub_signing_key();
    let tree = std::env::temp_dir().join(format!(
        "rspice-m4-acceptance-{}-{:?}",
        std::process::id(),
        std::thread::current().id()
    ));
    let _ = std::fs::remove_dir_all(&tree);
    let packs = tree.join("packs");
    let store = crate::state::model_hub::FilesystemModelHubStore::new(&tree);
    let handle = crate::services::model_hub::ModelHubStoreHandle::Filesystem {
        store: store.clone(),
        root: packs.clone(),
    };
    acceptance_sequence(&key, handle, &|| {
        ModelHub::open(
            anchor_for(&key),
            Box::new(store.clone()),
            Some(packs.clone()),
        )
        .expect("a filesystem hub opens")
    });
    let _ = std::fs::remove_dir_all(&tree);
}

/// The same gesture on the store the browser has.
///
/// This is the acceptance requirement the browser build has to meet, and it is
/// run rather than argued: a memory store keeps pack sources in a map with no
/// path anywhere, so every step from installing to solving offline has to work
/// over bytes alone.
#[test]
fn the_acceptance_sequence_runs_end_to_end_on_the_browser_store() {
    let key = hub_signing_key();
    let store = std::sync::Arc::new(MemoryModelHubStore::new());
    let handle =
        crate::services::model_hub::ModelHubStoreHandle::Memory(std::sync::Arc::clone(&store));
    acceptance_sequence(&key, handle, &|| {
        ModelHub::open(
            anchor_for(&key),
            Box::new(std::sync::Arc::clone(&store)),
            None,
        )
        .expect("a memory hub opens")
    });
}

/// Search the catalog, confirm the install, place what it armed, netlist the
/// sheet, save and reload the project, and then reopen it with the pack gone
/// from this machine entirely.
///
/// The last step is the one that matters: a design must keep solving from the
/// bytes it retained, not from a pack that happens to still be installed. The
/// sequence takes its store as an argument because the whole point of the
/// requirement is that the answer does not depend on which store it is —
/// `open` reopens a hub over the same one, which is what a session's worker
/// does.
fn acceptance_sequence(
    key: &rspice_pack::SigningKey,
    handle: crate::services::model_hub::ModelHubStoreHandle,
    open: &dyn Fn() -> ModelHub,
) {
    let archive = signed_archive(key, &CAPABILITIES);
    let snapshot = signed_snapshot(key, &archive, &CAPABILITIES, VERSION);
    let transport = StubTransport::with_snapshot(snapshot).serving(VERSION, archive.clone());
    let mut hub = open();
    let mut state = open_project();

    // 1. Search. Before any catalog is fetched the shelf offers nothing, which
    //    is the honest answer rather than an empty list of "available" parts.
    assert!(hub.part_index(&[]).is_empty());
    hub.refresh_catalog(&transport).expect("catalog");
    let found = hub
        .part_index(&[])
        .into_iter()
        .find(|row| row.part_id.contains("PROVING"))
        .expect("the search finds the published part");
    assert_eq!(found.state, PartState::Available);

    // 2. Confirm and install, retaining the part into the project.
    let request = ModelHubRequest::InstallPack {
        pack_id: PACK_ID.to_owned(),
        version: VERSION.to_owned(),
        part: Some(PART.to_owned()),
    };
    let output = execute(
        &mut hub,
        state.model_library_manager.clone(),
        &request,
        &transport,
    )
    .expect("the install succeeds");
    publish(&mut state, handle, hub, &request, output);

    // 3. Place what it armed.
    assert_eq!(
        state.schematic.tool,
        Tool::Place(ComponentType::CellInstance)
    );
    let binding = state
        .schematic
        .pending_library_cell
        .clone()
        .expect("an armed binding");
    let id = state
        .schematic
        .add_library_cell_component(crate::state::Point::new(100, 100), binding);
    assert!(
        state
            .schematic
            .components
            .iter()
            .any(|component| component.id == id)
    );

    // 4. The netlist names the part as the instance master.
    let deck = crate::simulation::netlist_gen::generate_netlist(&state.schematic).netlist;
    assert!(
        deck.contains(PART),
        "the emitted deck instantiates the pack part:\n{deck}"
    );

    // 5. The project round-trips with the pin intact.
    let serialized =
        serde_json::to_string(&state.model_library_manager).expect("the catalog serializes");
    let reloaded: ModelLibraryManager =
        serde_json::from_str(&serialized).expect("the catalog reloads");
    let pin = reloaded
        .libraries_sorted()
        .into_iter()
        .find_map(|library| library.pack_pin.clone())
        .expect("the pin survives the round trip");
    assert_eq!(pin.part_id, PART);
    assert_eq!(pin.archive_sha256, rspice_pack::sha256_hex(&archive));

    // 6. Uninstall the pack outright, then serve the design from what the
    //    project retained. Nothing about execution consults the hub.
    let mut hub = open();
    assert!(hub.uninstall(PACK_ID, VERSION).expect("uninstall"));
    assert!(hub.installed().is_empty());

    let cards = reloaded
        .reference_process_model_cards(crate::product::ProcessCorner::TT)
        .expect("the retained snapshot materializes with the pack uninstalled");
    let offline_deck = format!(
        "model hub acceptance deck\n{}\nV1 IN 0 1\nX1 IN OUT {PART}\n.op\n.end\n",
        cards.join("\n")
    );
    let netlist = rspice_core::Netlist::parse(&offline_deck).expect("the retained deck parses");
    let out = rspice_core::Engine::new(rspice_core::SimulationConfig::default())
        .run_dc_op(&netlist)
        .expect("the retained bytes still solve with nothing installed")
        .try_voltage_named("OUT")
        .expect("the divider output is a solved node");
    assert!(
        (out - 0.5).abs() < 1.0e-9,
        "the retained release's own divider is what solves: V(OUT)={out}"
    );
}

#[test]
fn a_model_card_part_arms_its_native_device_and_symbol_skin() {
    // The placement half of the shelf gesture, driven from the same arming
    // entry point the operation machine uses.
    let mut state = open_project();
    let armed = state.schematic.arm_pack_part(PartPlacement::NativeDevice {
        component_type: ComponentType::Diode,
        variant: Some("zener".to_owned()),
        model: "1N4728A".to_owned(),
    });
    assert_eq!(armed, "1N4728A");
    assert_eq!(state.schematic.tool, Tool::Place(ComponentType::Diode));
    assert!(state.schematic.pending_library_cell.is_none());

    let id = state
        .schematic
        .add_component(ComponentType::Diode, crate::state::Point::new(100, 100));
    let placed = state
        .schematic
        .components
        .iter()
        .find(|component| component.id == id)
        .expect("the placed diode");
    assert_eq!(placed.value, "1N4728A", "the card rides the instance value");
    assert_eq!(placed.symbol_variant.as_deref(), Some("zener"));

    // Arming any other device retires the card, so a resistor can never be
    // placed still carrying it.
    state
        .schematic
        .arm_tool(Tool::Place(ComponentType::Resistor));
    assert!(state.schematic.pending_part_model.is_none());
    let id = state
        .schematic
        .add_component(ComponentType::Resistor, crate::state::Point::new(200, 200));
    let placed = state
        .schematic
        .components
        .iter()
        .find(|component| component.id == id)
        .expect("the placed resistor");
    assert_ne!(placed.value, "1N4728A");
    assert!(placed.symbol_variant.is_none());
}

#[test]
fn a_failed_catalog_refresh_lets_a_later_visit_try_again() {
    // Opening the Models workspace refreshes a stale catalog once, latched so
    // it does not fire every frame the workspace is visible. The latch was
    // never released, so a session that happened to be offline the first time
    // the workspace opened spent its only attempt and never retried — the
    // workspace went on reporting a week-old catalog for the rest of the day.
    let mut state = AppState::default();
    state.workbench.models_view.catalog_refresh_requested = true;
    let ctx = egui::Context::default();

    super::emit_model_hub_errors(
        &ctx,
        &mut state,
        vec!["the model hub could not be reached".to_owned()],
    );

    assert_eq!(
        state.workbench.models_view.operational_state,
        ModelsOperationalState::Offline
    );
    assert!(
        !state.workbench.models_view.catalog_refresh_requested,
        "a failed refresh must not consume the session's only attempt"
    );
}

/// The M4b round trip, end to end: pinned at one release, offered another,
/// told exactly what the catalog says changed, and moving one part onto it.
///
/// Every claim here is one a surface makes. The diff is the projection the
/// inspector's Releases pane renders; the adoption is the request its Adopt
/// control queues; the pins afterwards are what the ledger's Project column
/// reads. The ratio is the part no bookkeeping error could produce by
/// accident: 1/2 is the 1.0.0 divider and 1/4 is the 1.1.0 one, so a project
/// that adopted solves to the new answer and a project that did not solves to
/// the old one — with both releases on the machine the whole time.
#[test]
fn adopting_one_part_moves_its_pin_and_leaves_every_other_pin_where_it_was() {
    use crate::state::model_hub::tests::{
        BIAS_PART_ID, DIVIDER_ALIAS_NEXT, signed_snapshot_projecting, two_part_archive,
    };
    use crate::state::model_hub::{PartFact, PartStanding};

    let key = hub_signing_key();
    let archive = two_part_archive(&key, &CAPABILITIES, VERSION);
    let next_archive = two_part_archive(&key, &CAPABILITIES, NEXT_VERSION);
    let held_catalog = signed_snapshot_projecting(&key, &[(VERSION, &archive, &CAPABILITIES)]);
    let transport = StubTransport::with_snapshot(held_catalog).serving(VERSION, archive.clone());
    let (mut hub, handle, _store) = fixture_hub(&key);
    hub.refresh_catalog(&transport).expect("catalog");
    hub.install(&transport, PACK_ID, VERSION).expect("install");

    // A project takes both parts at 1.0.0. They land as two libraries, so the
    // project holds two pins and can move one without the other.
    let mut state = open_project();
    hub.add_part_to_project(&mut state.model_library_manager, PACK_ID, VERSION, PART)
        .expect("the project retains the 1.0.0 divider");
    hub.add_part_to_project(
        &mut state.model_library_manager,
        PACK_ID,
        VERSION,
        BIAS_PART_ID,
    )
    .expect("the project retains the 1.0.0 bias network");
    // A second project that adopts nothing, kept for the end.
    let untouched = state.model_library_manager.clone();

    // A later generation publishes 1.1.0 beside 1.0.0.
    let offered_catalog = signed_snapshot_projecting(
        &key,
        &[
            (VERSION, &archive, &CAPABILITIES),
            (NEXT_VERSION, &next_archive, &CAPABILITIES),
        ],
    );
    let newer = StubTransport::with_snapshot(offered_catalog)
        .at_generation(8)
        .serving(VERSION, archive.clone())
        .serving(NEXT_VERSION, next_archive.clone());
    hub.refresh_catalog(&newer).expect("the newer catalog");

    // What the Releases pane shows: a comparison of two signed records, and
    // an explicit refusal to overstate the part the catalog is silent about.
    let diff = hub
        .release_diff(PACK_ID, VERSION, NEXT_VERSION)
        .expect("the catalog publishes both releases");
    assert!(diff.added.is_empty() && diff.removed.is_empty());
    let changed = diff
        .changed
        .iter()
        .find(|part| part.part_id == PART)
        .expect("the divider is published by both releases and listed differently");
    assert_eq!(
        changed
            .facts
            .iter()
            .map(PartFact::describe)
            .collect::<Vec<_>>(),
        vec![format!("aliases +{DIVIDER_ALIAS_NEXT}")]
    );
    assert_eq!(
        diff.part_standing(BIAS_PART_ID),
        PartStanding::Relisted,
        "the catalog states no difference about the bias part"
    );
    assert_eq!(diff.relisted, 1);
    assert!(
        diff.archive_differs,
        "and re-listed is still not a claim that the archives agree"
    );

    // Adopt exactly one part.
    let request = ModelHubRequest::AdoptPart {
        pack_id: PACK_ID.to_owned(),
        from_version: VERSION.to_owned(),
        to_version: NEXT_VERSION.to_owned(),
        part_id: PART.to_owned(),
    };
    let output = execute(
        &mut hub,
        state.model_library_manager.clone(),
        &request,
        &newer,
    )
    .expect("the adoption succeeds");
    let part = output.part.as_ref().expect("adoption retains a part");
    assert!(
        part.placement.is_none(),
        "a part already in the design is not armed for placement again"
    );
    assert_eq!(
        pinned_versions(&part.candidate),
        vec![
            (BIAS_PART_ID.to_owned(), VERSION.to_owned()),
            (PART.to_owned(), NEXT_VERSION.to_owned()),
        ],
        "one pin moved and the other did not"
    );
    assert!(
        output.receipt.contains(NEXT_VERSION) && output.receipt.contains(VERSION),
        "the receipt names what moved and what it moved off: {}",
        output.receipt
    );

    // Both releases are still on this machine: adoption installs, it never
    // replaces, because other parts are still pinned to the older one.
    assert_eq!(hub.installed().len(), 2);

    publish(&mut state, handle, hub, &request, output);
    assert_eq!(
        state.workbench.models_view.operational_state,
        ModelsOperationalState::Ready,
        "the adoption published: {:?}",
        state.workbench.models_view.action_receipt
    );
    assert!(
        state.schematic.pending_library_cell.is_none(),
        "adoption places nothing"
    );
    assert_eq!(
        pinned_versions(&state.model_library_manager),
        vec![
            (BIAS_PART_ID.to_owned(), VERSION.to_owned()),
            (PART.to_owned(), NEXT_VERSION.to_owned()),
        ],
        "and the published project records the same two pins"
    );
    let moved = state
        .model_library_manager
        .libraries_sorted()
        .into_iter()
        .find_map(|library| library.pack_pin.clone())
        .filter(|pin| pin.part_id == PART)
        .expect("the moved pin is recorded");
    assert_eq!(
        moved.archive_sha256,
        rspice_pack::sha256_hex(&next_archive),
        "the pin names the archive the adopted bytes were proved as"
    );

    // The adopted project solves the 1.1.0 divider — the bytes moved, not only
    // the label — and the project that adopted nothing still solves the 1.0.0
    // one, with both releases installed the whole time.
    let adopted = divider_output(&state.model_library_manager);
    assert!(
        (adopted - 0.25).abs() < 1.0e-9,
        "the adopted project runs the 1.1.0 divider: V(OUT)={adopted}"
    );
    let reloaded: ModelLibraryManager = serde_json::from_str(
        &serde_json::to_string(&untouched).expect("the untouched project saves"),
    )
    .expect("the untouched project reopens");
    let out = divider_output(&reloaded);
    assert!(
        (out - 0.5).abs() < 1.0e-9,
        "a project pinned wholly at 1.0.0 still runs the 1.0.0 divider: V(OUT)={out}"
    );
}

/// `(part, release)` for every pack pin a project records, part-sorted.
fn pinned_versions(manager: &ModelLibraryManager) -> Vec<(String, String)> {
    let mut pins = manager
        .libraries_sorted()
        .into_iter()
        .filter_map(|library| library.pack_pin.clone())
        .map(|pin| (pin.part_id, pin.pack_version))
        .collect::<Vec<_>>();
    pins.sort();
    pins
}

/// A release the catalog stopped publishing cannot be adopted from.
///
/// The bytes are on this machine and they still prove under the release key,
/// which is exactly why this needs its own guard: everything an install checks
/// has already passed, and the one thing that changed is the catalog's word
/// that this release is still published. A withdrawn release is what a revoked
/// one looks like from a client, and adoption refuses it in the vocabulary
/// every other hub refusal uses.
#[test]
fn a_release_the_catalog_no_longer_publishes_refuses_adoption_with_the_reason() {
    use crate::state::model_hub::tests::{signed_snapshot_projecting, two_part_archive};

    let key = hub_signing_key();
    let archive = two_part_archive(&key, &CAPABILITIES, VERSION);
    let next_archive = two_part_archive(&key, &CAPABILITIES, NEXT_VERSION);
    let both = signed_snapshot_projecting(
        &key,
        &[
            (VERSION, &archive, &CAPABILITIES),
            (NEXT_VERSION, &next_archive, &CAPABILITIES),
        ],
    );
    let transport = StubTransport::with_snapshot(both)
        .serving(VERSION, archive.clone())
        .serving(NEXT_VERSION, next_archive.clone());
    let (mut hub, _handle, _store) = fixture_hub(&key);
    hub.refresh_catalog(&transport).expect("catalog");
    hub.install(&transport, PACK_ID, VERSION).expect("install");
    hub.install(&transport, PACK_ID, NEXT_VERSION)
        .expect("the newer release is on this machine too");

    let mut project = ModelLibraryManager::new();
    hub.add_part_to_project(&mut project, PACK_ID, VERSION, PART)
        .expect("the project retains the 1.0.0 divider");

    // The publisher withdraws 1.1.0. Nothing on this machine moved.
    let withdrawn = signed_snapshot_projecting(&key, &[(VERSION, &archive, &CAPABILITIES)]);
    let later = StubTransport::with_snapshot(withdrawn)
        .at_generation(9)
        .serving(VERSION, archive.clone());
    hub.refresh_catalog(&later).expect("the later catalog");
    hub.verify_installed(PACK_ID, NEXT_VERSION)
        .expect("the withdrawn release's bytes still prove under the release key");

    let error = execute(
        &mut hub,
        project.clone(),
        &ModelHubRequest::AdoptPart {
            pack_id: PACK_ID.to_owned(),
            from_version: VERSION.to_owned(),
            to_version: NEXT_VERSION.to_owned(),
            part_id: PART.to_owned(),
        },
        &later,
    )
    .expect_err("a withdrawn release is not adoptable");
    assert!(
        error.contains("does not publish") && error.contains(NEXT_VERSION),
        "the refusal names what is no longer published: {error}"
    );
    assert_eq!(
        pinned_versions(&project),
        vec![(PART.to_owned(), VERSION.to_owned())],
        "and the pin it refused to move is exactly where it was"
    );
}

/// A recall refuses on the operation machine, in the banner's own words.
///
/// The unit gate next door proves the runtime refuses. This proves the refusal
/// survives the trip a reader actually takes: the request runs on the machine
/// every model-catalog operation runs on, the error becomes a receipt, and the
/// receipt classifies onto the rung whose consequence line is true of it.
#[test]
fn an_update_onto_a_recalled_release_refuses_on_the_operation_machine() {
    use crate::state::model_hub::tests::{CatalogTerms, signed_snapshot_on};

    const REASON: &str = "the divider ratio was published against the wrong reference.";

    let key = hub_signing_key();
    let archive = signed_archive_at(&key, &CAPABILITIES, VERSION);
    let next_archive = signed_archive_at(&key, &CAPABILITIES, NEXT_VERSION);
    let releases: [(&str, &[u8], &[&str]); 2] = [
        (VERSION, &archive, &CAPABILITIES),
        (NEXT_VERSION, &next_archive, &CAPABILITIES),
    ];
    let listing = StubTransport::with_snapshot(signed_snapshot_on(
        &key,
        &releases,
        &CatalogTerms::at_serial(1),
    ))
    .serving(VERSION, archive.clone())
    .serving(NEXT_VERSION, next_archive.clone());
    let (mut hub, handle, _store) = fixture_hub(&key);
    hub.refresh_catalog(&listing).expect("catalog");
    hub.install(&listing, PACK_ID, VERSION).expect("install");

    let recalled = StubTransport::with_snapshot(signed_snapshot_on(
        &key,
        &releases,
        &CatalogTerms {
            serial: 2,
            ..CatalogTerms::recalling(NEXT_VERSION, REASON)
        },
    ))
    .at_generation(9)
    .serving(NEXT_VERSION, next_archive);
    hub.refresh_catalog(&recalled).expect("the recall arrives");

    let request = ModelHubRequest::UpdatePack {
        pack_id: PACK_ID.to_owned(),
        installed: VERSION.to_owned(),
        latest: NEXT_VERSION.to_owned(),
    };
    let error = execute(&mut hub, ModelLibraryManager::new(), &request, &recalled)
        .expect_err("updating onto a recalled release refuses");
    assert!(
        error.contains(REASON) && error.contains(NEXT_VERSION),
        "the refusal names the release and the publisher's reason: {error}"
    );
    assert_eq!(
        ModelsOperationalState::from_failure(&error),
        ModelsOperationalState::Recalled,
        "and the banner says Recalled rather than blaming the operator for a \
         reason that happens to mention a licence or an invalid card"
    );
    assert_eq!(
        hub.installed().len(),
        1,
        "the release that was already here is untouched"
    );
    assert_eq!(hub.installed()[0].version(), VERSION);

    // The same refusal reaching the session leaves the banner the workspace
    // already paints, with no new vocabulary of its own.
    let ctx = egui::Context::default();
    let mut state = open_project();
    super::emit_model_hub_errors(&ctx, &mut state, vec![error]);
    assert_eq!(
        state.workbench.models_view.operational_state,
        ModelsOperationalState::Recalled
    );
    assert!(
        state
            .workbench
            .models_view
            .action_receipt
            .as_ref()
            .is_some_and(Result::is_err)
    );
    let _ = handle;
}

/// Opening a project pinned to a recalled release warns, once, and blocks
/// nothing.
///
/// The warning arrives from the frame loop rather than from the Models
/// workspace, because a project opens into Design and a recall a reader has to
/// walk somewhere else to find is one they find after the work is done.
#[test]
fn a_project_pinned_to_a_recalled_release_is_warned_on_open_and_still_solves() {
    use crate::state::model_hub::tests::{CatalogTerms, signed_snapshot_on};
    use crate::workbench::RSpiceApp;

    const REASON: &str = "the divider ratio was published against the wrong reference.";

    let key = hub_signing_key();
    let archive = signed_archive(&key, &CAPABILITIES);
    let listing = StubTransport::with_snapshot(signed_snapshot_on(
        &key,
        &[(VERSION, &archive, &CAPABILITIES)],
        &CatalogTerms::at_serial(1),
    ))
    .serving(VERSION, archive.clone());
    let (mut hub, handle, store) = fixture_hub(&key);
    hub.refresh_catalog(&listing).expect("catalog");
    hub.install(&listing, PACK_ID, VERSION).expect("install");
    let mut project = ModelLibraryManager::new();
    hub.add_part_to_project(&mut project, PACK_ID, VERSION, PART)
        .expect("the project retained the part before the recall");
    let before = divider_output(&project);

    // The publisher recalls the release this project is pinned to.
    let recall = StubTransport::with_snapshot(signed_snapshot_on(
        &key,
        &[(VERSION, &archive, &CAPABILITIES)],
        &CatalogTerms {
            serial: 2,
            ..CatalogTerms::recalling(VERSION, REASON)
        },
    ));
    hub.refresh_catalog(&recall).expect("the recall arrives");
    drop(hub);

    // A session opens that project, over a store that already knows.
    let ctx = egui::Context::default();
    let mut app = RSpiceApp::test_instance();
    app.model_hub = crate::services::model_hub::ModelHubService::with_store(
        handle,
        ModelHub::open(anchor_for(&key), Box::new(store), None).expect("the session hub opens"),
    );
    app.state = open_project();
    app.state.model_library_manager = project.clone();

    app.pump_model_catalog_operations(&ctx);
    let warnings = |app: &RSpiceApp| {
        app.state
            .log_buffer
            .entries()
            .filter(|entry| entry.message.contains("which the publisher recalled"))
            .count()
    };
    assert_eq!(warnings(&app), 1, "the reader is told once");
    assert!(
        app.state
            .log_buffer
            .entries()
            .any(|entry| entry.message.contains(REASON)),
        "and told why"
    );

    // Every later frame is silent about a fact that has not changed.
    app.pump_model_catalog_operations(&ctx);
    app.pump_model_catalog_operations(&ctx);
    assert_eq!(warnings(&app), 1, "and not once per frame");

    // Nothing was blocked. The retained bytes are the same bytes and solve to
    // the same answer they did before the recall.
    assert!(
        (divider_output(&app.state.model_library_manager) - before).abs() < 1.0e-12,
        "a recall never touches what a project already retained"
    );
    assert_eq!(
        app.state.model_library_manager.libraries_sorted().len(),
        project.libraries_sorted().len(),
        "and no library was removed"
    );
}

/// Solves the divider out of whatever a project retained for it.
fn divider_output(manager: &ModelLibraryManager) -> f64 {
    let cards = manager
        .reference_process_model_cards(crate::product::ProcessCorner::TT)
        .expect("the retained snapshot materializes");
    let deck = format!(
        "model hub adoption deck\n{}\nV1 IN 0 1\nX1 IN OUT {PART}\n.op\n.end\n",
        cards.join("\n")
    );
    let netlist = rspice_core::Netlist::parse(&deck).expect("the retained deck parses");
    rspice_core::Engine::new(rspice_core::SimulationConfig::default())
        .run_dc_op(&netlist)
        .expect("the retained bytes solve")
        .try_voltage_named("OUT")
        .expect("the divider output is a solved node")
}
