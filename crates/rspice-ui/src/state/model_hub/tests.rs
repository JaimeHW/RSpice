//! The Model Hub runtime gate.
//!
//! Every fixture here is a *real* signed artifact: the packs are built by the
//! same `build_pack` the publisher tool runs, and the snapshots by the same
//! `encode_snapshot` the service runs, both under a key generated in the test
//! from a fixed seed. Nothing is stubbed but the network, so a test that
//! passes has exercised the actual signature, the actual archive container,
//! and the actual canonical manifest — not a mock of them.

use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
    sync::Mutex,
};

use rspice_pack::{
    License, ManifestTemplate, Part, PartKind, Requires, Snapshot, SnapshotPack, SnapshotPart,
    SnapshotRelease, SourceRef, build_pack, encode_snapshot, sha256_hex, signing_key,
};

use super::{
    ModelHub, ModelHubError,
    provider::{PartProvenance, PartState},
    store::{FilesystemModelHubStore, MemoryModelHubStore, ModelHubStore, STAGING_PREFIX},
    transport::{ArchiveHandoff, CatalogHandoff, ModelHubTransport, OfflineTransport},
    trust::TrustAnchor,
};

pub(crate) const PACK_ID: &str = "rspice-proving";
pub(crate) const VERSION: &str = "1.0.0";
/// The successor release. Every fixture that needs two generations of one pack
/// builds a real archive at this version rather than relabelling the first, so
/// the manifest inside the bytes agrees with the catalog that lists them.
pub(crate) const NEXT_VERSION: &str = "1.1.0";
pub(crate) const SOURCE_PATH: &str = "models/proving.lib";
pub(crate) const PART_ID: &str = "RSPICE_PROVING_DIV";

/// The deck the fixture pack publishes.
///
/// A subcircuit over primitives this engine already runs, so a test can attach
/// it and solve rather than merely assert that bytes landed.
const PROVING_LIB: &str = "* RSpice proving pack\n\
                           .subckt RSPICE_PROVING_DIV IN OUT\n\
                           R1 IN OUT 1k\n\
                           R2 OUT 0 1k\n\
                           .ends RSPICE_PROVING_DIV\n";

/// The same part, redesigned by the successor release.
///
/// The divider ratio is deliberately different — 1/4 rather than 1/2 — so a
/// project pinned to [`VERSION`] proves it kept its own bytes by *solving to
/// the old answer*, which no amount of correct bookkeeping could fake.
const PROVING_LIB_NEXT: &str = "* RSpice proving pack\n\
                                .subckt RSPICE_PROVING_DIV IN OUT\n\
                                R1 IN OUT 3k\n\
                                R2 OUT 0 1k\n\
                                .ends RSPICE_PROVING_DIV\n";

pub(crate) fn hub_signing_key() -> rspice_pack::SigningKey {
    signing_key(&[0x2A_u8; 32])
}

fn foreign_signing_key() -> rspice_pack::SigningKey {
    signing_key(&[0x5B_u8; 32])
}

pub(crate) fn anchor_for(key: &rspice_pack::SigningKey) -> TrustAnchor {
    TrustAnchor::from_verifying_key(key.verifying_key())
}

fn template(version: &str, capabilities: &[&str]) -> ManifestTemplate {
    ManifestTemplate {
        pack: rspice_pack::PackIdentity {
            id: PACK_ID.to_owned(),
            version: version.to_owned(),
            name: "RSpice proving pack".to_owned(),
            category: "proving".to_owned(),
        },
        license: License {
            spdx: "LicenseRef-RSpice-Models".to_owned(),
        },
        requires: Requires {
            capabilities: capabilities
                .iter()
                .map(|value| (*value).to_owned())
                .collect(),
        },
        parts: vec![Part {
            id: PART_ID.to_owned(),
            kind: PartKind::Subckt,
            device: "network".to_owned(),
            aliases: Vec::new(),
            source: SourceRef {
                path: SOURCE_PATH.to_owned(),
                line: 2,
            },
            terminals: vec!["IN".to_owned(), "OUT".to_owned()],
            symbol: None,
        }],
    }
}

/// Builds a real signed archive for the fixture pack at [`VERSION`].
pub(crate) fn signed_archive(key: &rspice_pack::SigningKey, capabilities: &[&str]) -> Vec<u8> {
    signed_archive_at(key, capabilities, VERSION)
}

/// Builds a real signed archive for one release of the fixture pack.
///
/// The deck follows the version, so `1.1.0` is a genuinely different pack and
/// not the same bytes wearing a newer label — which is the only way an update
/// can be told apart from a no-op.
pub(crate) fn signed_archive_at(
    key: &rspice_pack::SigningKey,
    capabilities: &[&str],
    version: &str,
) -> Vec<u8> {
    let deck = if version == VERSION {
        PROVING_LIB
    } else {
        PROVING_LIB_NEXT
    };
    build_pack(
        &[(SOURCE_PATH.to_owned(), deck.as_bytes().to_vec())],
        template(version, capabilities),
        key,
    )
    .expect("the fixture pack is well formed")
}

/// Builds a real signed snapshot listing one release of the fixture pack.
pub(crate) fn signed_snapshot(
    key: &rspice_pack::SigningKey,
    archive: &[u8],
    capabilities: &[&str],
    version: &str,
) -> Vec<u8> {
    signed_snapshot_of(key, &[(version, archive, capabilities)])
}

/// Builds a real signed snapshot listing every release it is handed.
///
/// A later catalog generation keeps publishing the releases an earlier one
/// did — withdrawing a version is a separate act — so a fixture that lists
/// both is the shape a real update is decided against.
pub(crate) fn signed_snapshot_of(
    key: &rspice_pack::SigningKey,
    releases: &[(&str, &[u8], &[&str])],
) -> Vec<u8> {
    let snapshot = Snapshot {
        schema: rspice_pack::SNAPSHOT_SCHEMA,
        generated_at: "2026-08-15T09:30:00Z".to_owned(),
        packs: vec![SnapshotPack {
            id: PACK_ID.to_owned(),
            name: "RSpice proving pack".to_owned(),
            category: "proving".to_owned(),
            releases: releases
                .iter()
                .map(|(version, archive, capabilities)| SnapshotRelease {
                    version: (*version).to_owned(),
                    archive_sha256: sha256_hex(archive),
                    archive_length: archive.len() as u64,
                    capabilities: capabilities
                        .iter()
                        .map(|value| (*value).to_owned())
                        .collect(),
                    spdx: "LicenseRef-RSpice-Models".to_owned(),
                    parts: vec![SnapshotPart {
                        id: PART_ID.to_owned(),
                        kind: PartKind::Subckt,
                        device: "network".to_owned(),
                        aliases: Vec::new(),
                        terminals: vec!["IN".to_owned(), "OUT".to_owned()],
                        symbol: None,
                    }],
                })
                .collect(),
        }],
    };
    encode_snapshot(&snapshot, key).expect("the fixture snapshot is well formed")
}

/// A transport serving exactly the bytes a test hands it.
#[derive(Debug, Default)]
pub(crate) struct StubTransport {
    snapshot: Option<Vec<u8>>,
    archives: BTreeMap<String, Vec<u8>>,
    /// Declared digests, when a test wants the handoff to disagree with the
    /// bytes or with the snapshot.
    declared: Mutex<BTreeMap<String, (u64, String)>>,
    fail_archive_fetch: bool,
    generation: u64,
    calls: Mutex<Vec<String>>,
}

impl StubTransport {
    pub(crate) fn with_snapshot(snapshot: Vec<u8>) -> Self {
        Self {
            snapshot: Some(snapshot),
            generation: 7,
            ..Self::default()
        }
    }

    /// Serves this snapshot as a later catalog generation.
    pub(crate) fn at_generation(mut self, generation: u64) -> Self {
        self.generation = generation;
        self
    }

    pub(crate) fn serving(mut self, version: &str, archive: Vec<u8>) -> Self {
        self.archives.insert(version.to_owned(), archive);
        self
    }

    fn declaring(self, version: &str, length: u64, digest: &str) -> Self {
        self.declared
            .lock()
            .expect("stub state")
            .insert(version.to_owned(), (length, digest.to_owned()));
        self
    }

    pub(crate) fn failing_archive_fetch(mut self) -> Self {
        self.fail_archive_fetch = true;
        self
    }

    pub(crate) fn calls(&self) -> Vec<String> {
        self.calls.lock().expect("stub state").clone()
    }

    fn record(&self, call: &str) {
        self.calls.lock().expect("stub state").push(call.to_owned());
    }
}

impl ModelHubTransport for StubTransport {
    fn catalog_handoff(&self) -> Result<CatalogHandoff, ModelHubError> {
        self.record("catalog_handoff");
        let snapshot = self.snapshot.as_ref().ok_or(ModelHubError::Offline)?;
        Ok(CatalogHandoff {
            generation: self.generation,
            content_length: snapshot.len() as u64,
            content_sha256: self
                .declared
                .lock()
                .expect("stub state")
                .get("catalog")
                .map(|(_, digest)| digest.clone())
                .unwrap_or_else(|| sha256_hex(snapshot)),
        })
    }

    fn fetch_catalog(&self, _handoff: &CatalogHandoff) -> Result<Vec<u8>, ModelHubError> {
        self.record("fetch_catalog");
        self.snapshot.clone().ok_or(ModelHubError::Offline)
    }

    fn archive_handoff(
        &self,
        _pack_id: &str,
        version: &str,
    ) -> Result<ArchiveHandoff, ModelHubError> {
        self.record("archive_handoff");
        let archive = self
            .archives
            .get(version)
            .ok_or_else(|| ModelHubError::Rejected("no such release".to_owned()))?;
        let declared = self
            .declared
            .lock()
            .expect("stub state")
            .get(version)
            .cloned();
        Ok(match declared {
            Some((length, digest)) => ArchiveHandoff {
                content_length: length,
                content_sha256: digest,
            },
            None => ArchiveHandoff {
                content_length: archive.len() as u64,
                content_sha256: sha256_hex(archive),
            },
        })
    }

    fn fetch_archive(&self, handoff: &ArchiveHandoff) -> Result<Vec<u8>, ModelHubError> {
        self.record("fetch_archive");
        if self.fail_archive_fetch {
            return Err(ModelHubError::Offline);
        }
        // Digest first, and only then length: two releases of one pack are
        // very nearly the same size, so matching on length alone would serve
        // whichever happened to sort first and turn a correct update into a
        // digest mismatch that says nothing about the code under test.
        self.archives
            .values()
            .find(|archive| sha256_hex(archive) == handoff.content_sha256)
            .or_else(|| {
                self.archives
                    .values()
                    .find(|archive| archive.len() as u64 == handoff.content_length)
            })
            .cloned()
            .ok_or(ModelHubError::HandoffExpired)
    }
}

/// A filesystem tree that removes itself.
struct TempTree {
    root: PathBuf,
}

impl TempTree {
    fn new(label: &str) -> Self {
        static COUNTER: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
        let ordinal = COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        let root = std::env::temp_dir().join(format!(
            "rspice-model-hub-{label}-{}-{ordinal}",
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(&root).expect("create the fixture tree");
        Self { root }
    }

    fn path(&self) -> &Path {
        &self.root
    }
}

impl Drop for TempTree {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.root);
    }
}

fn filesystem_hub(tree: &TempTree, key: &rspice_pack::SigningKey) -> ModelHub {
    ModelHub::open(
        anchor_for(key),
        Box::new(FilesystemModelHubStore::new(tree.path())),
        Some(tree.path().join("packs")),
    )
    .expect("the hub opens over an empty tree")
}

fn packs_root(tree: &TempTree) -> PathBuf {
    tree.path().join("packs")
}

fn staging_directories(tree: &TempTree) -> Vec<PathBuf> {
    let Ok(entries) = std::fs::read_dir(packs_root(tree)) else {
        return Vec::new();
    };
    entries
        .flatten()
        .map(|entry| entry.path())
        .filter(|path| {
            path.file_name()
                .and_then(|name| name.to_str())
                .is_some_and(|name| name.starts_with(STAGING_PREFIX))
        })
        .collect()
}

#[test]
fn a_signed_release_installs_and_its_part_solves_through_the_retained_path() {
    let key = hub_signing_key();
    let archive = signed_archive(&key, &["subckt", "resistor"]);
    let snapshot = signed_snapshot(&key, &archive, &["subckt", "resistor"], VERSION);
    let transport = StubTransport::with_snapshot(snapshot).serving(VERSION, archive.clone());
    let tree = TempTree::new("install");
    let mut hub = filesystem_hub(&tree, &key);

    assert_eq!(hub.refresh_catalog(&transport).expect("catalog"), 7);
    let installed = hub.install(&transport, PACK_ID, VERSION).expect("install");
    assert_eq!(installed.pack_id(), PACK_ID);
    assert_eq!(installed.archive_sha256, sha256_hex(&archive));

    // The pack is present as a verified installation, not merely as files.
    hub.verify_installed(PACK_ID, VERSION)
        .expect("the installed archive re-proves under the anchor");

    let rows = hub.part_index(&[]);
    let row = rows
        .iter()
        .find(|row| row.part_id == PART_ID)
        .expect("the installed part is indexed");
    assert_eq!(row.state, PartState::Installed);
    assert_eq!(
        row.provenance,
        PartProvenance::InstalledPack {
            pack_id: PACK_ID.to_owned(),
            version: VERSION.to_owned(),
        }
    );

    // The installed source goes into a project through the existing retained
    // import path, and the retained bytes still solve.
    let pin = hub
        .part_pin(PACK_ID, VERSION, PART_ID)
        .expect("a pin for an installed part");
    let mut manager = crate::state::model_library::ModelLibraryManager::new();
    let library_name = hub
        .add_part_to_project(&mut manager, PACK_ID, VERSION, PART_ID)
        .expect("the installed source is retained into the project");
    let library = manager
        .get_library(&library_name)
        .expect("the retained library exists");
    assert!(matches!(
        library.source_authority,
        crate::state::model_library::ModelSourceAuthority::RetainedImport { .. }
    ));
    assert_eq!(library.source_contents.len(), library.source_closure.len());
    assert_eq!(library.pack_pin.as_ref(), Some(&pin));
    assert_eq!(pin.archive_sha256, sha256_hex(&archive));

    // The 1.0.0 divider is equal-valued, so a solved half of the input is the
    // release's own behaviour rather than any release's.
    assert!((retained_divider_output(&manager) - 0.5).abs() < 1.0e-9);
}

#[test]
fn a_tampered_archive_a_foreign_key_and_a_lying_handoff_are_all_refused() {
    let key = hub_signing_key();
    let archive = signed_archive(&key, &["subckt", "resistor"]);
    let snapshot = signed_snapshot(&key, &archive, &["subckt", "resistor"], VERSION);

    // A flipped byte inside the archive. The snapshot still names the honest
    // digest, so the substitution is caught before the signature is even
    // consulted.
    let mut flipped = archive.clone();
    let last = flipped.len() - 1;
    flipped[last] ^= 0x01;
    let tree = TempTree::new("tamper-archive");
    let mut hub = filesystem_hub(&tree, &key);
    let transport = StubTransport::with_snapshot(snapshot.clone())
        .serving(VERSION, flipped.clone())
        .declaring(VERSION, flipped.len() as u64, &sha256_hex(&flipped));
    hub.refresh_catalog(&transport).expect("catalog");
    let refusal = hub
        .install(&transport, PACK_ID, VERSION)
        .expect_err("a flipped archive byte is refused");
    assert!(matches!(refusal, ModelHubError::DigestMismatch { .. }));
    assert!(hub.installed().is_empty());
    assert!(staging_directories(&tree).is_empty());
    assert!(!packs_root(&tree).join(PACK_ID).exists());

    // A pack signed by another key, listed by an otherwise honest catalog.
    let foreign = foreign_signing_key();
    let foreign_archive = signed_archive(&foreign, &["subckt", "resistor"]);
    let honest_snapshot = signed_snapshot(&key, &foreign_archive, &["subckt", "resistor"], VERSION);
    let tree = TempTree::new("tamper-key");
    let mut hub = filesystem_hub(&tree, &key);
    let transport = StubTransport::with_snapshot(honest_snapshot).serving(VERSION, foreign_archive);
    hub.refresh_catalog(&transport).expect("catalog");
    let refusal = hub
        .install(&transport, PACK_ID, VERSION)
        .expect_err("a foreign signature is refused");
    assert!(matches!(refusal, ModelHubError::Format(_)));
    assert!(hub.installed().is_empty());
    assert!(staging_directories(&tree).is_empty());
    assert!(!packs_root(&tree).join(PACK_ID).exists());

    // A catalog handoff whose digest does not describe the bytes it serves.
    let tree = TempTree::new("tamper-snapshot");
    let mut hub = filesystem_hub(&tree, &key);
    let transport = StubTransport::with_snapshot(snapshot.clone()).declaring(
        "catalog",
        snapshot.len() as u64,
        &"ab".repeat(32),
    );
    let refusal = hub
        .refresh_catalog(&transport)
        .expect_err("a snapshot digest mismatch is refused");
    assert!(matches!(refusal, ModelHubError::DigestMismatch { .. }));
    assert!(hub.snapshot().is_none());

    // A snapshot signed by a key this client does not trust.
    let tree = TempTree::new("tamper-snapshot-key");
    let mut hub = filesystem_hub(&tree, &key);
    let foreign_snapshot = signed_snapshot(&foreign, &archive, &["subckt", "resistor"], VERSION);
    let transport = StubTransport::with_snapshot(foreign_snapshot);
    let refusal = hub
        .refresh_catalog(&transport)
        .expect_err("a foreign catalog signature is refused");
    assert!(matches!(refusal, ModelHubError::Format(_)));
    assert!(hub.snapshot().is_none());
}

#[test]
fn a_kill_between_staging_and_rename_leaves_only_staging_for_the_sweep() {
    let key = hub_signing_key();
    let archive = signed_archive(&key, &["subckt", "resistor"]);
    let snapshot = signed_snapshot(&key, &archive, &["subckt", "resistor"], VERSION);
    let tree = TempTree::new("kill-install");
    let store = FilesystemModelHubStore::new(tree.path());
    let anchor = anchor_for(&key);

    // Stage the verified pack and then stop, which is exactly the state a
    // process killed between the expansion and the rename leaves behind.
    let verified = rspice_pack::Pack::verify(&archive, anchor.key(), anchor.limits())
        .expect("the fixture archive verifies");
    let staged = store.stage_pack(&verified, &archive).expect("stage");
    std::mem::forget(staged);
    assert_eq!(staging_directories(&tree).len(), 1);
    assert!(!packs_root(&tree).join(PACK_ID).exists());

    // Starting up sweeps it, and the pack root is untouched.
    let mut hub = ModelHub::open(
        anchor_for(&key),
        Box::new(FilesystemModelHubStore::new(tree.path())),
        Some(packs_root(&tree)),
    )
    .expect("the hub opens over an interrupted install");
    assert!(staging_directories(&tree).is_empty());
    assert!(hub.installed().is_empty());
    assert!(!packs_root(&tree).join(PACK_ID).exists());

    // A failure mid-download stages nothing at all.
    let transport = StubTransport::with_snapshot(snapshot)
        .serving(VERSION, archive)
        .failing_archive_fetch();
    hub.refresh_catalog(&transport).expect("catalog");
    let refusal = hub
        .install(&transport, PACK_ID, VERSION)
        .expect_err("a failed download installs nothing");
    assert_eq!(refusal, ModelHubError::Offline);
    assert!(staging_directories(&tree).is_empty());
    assert!(!packs_root(&tree).join(PACK_ID).exists());
    assert!(transport.calls().contains(&"fetch_archive".to_owned()));
}

#[test]
fn a_cached_catalog_and_an_installed_pack_serve_the_shelf_with_no_network() {
    let key = hub_signing_key();
    let archive = signed_archive(&key, &["subckt", "resistor"]);
    let snapshot = signed_snapshot(&key, &archive, &["subckt", "resistor"], VERSION);
    let tree = TempTree::new("offline");
    {
        let transport = StubTransport::with_snapshot(snapshot).serving(VERSION, archive.clone());
        let mut hub = filesystem_hub(&tree, &key);
        hub.refresh_catalog(&transport).expect("catalog");
        hub.install(&transport, PACK_ID, VERSION).expect("install");
    }

    // Reopen with no transport reachable at all.
    let mut hub = filesystem_hub(&tree, &key);
    assert!(
        hub.snapshot().is_some(),
        "the cached snapshot still verifies"
    );
    assert_eq!(hub.installed().len(), 1);
    let rows = hub.part_index(&[]);
    assert!(
        rows.iter()
            .any(|row| row.part_id == PART_ID && row.state == PartState::Installed)
    );
    hub.verify_installed(PACK_ID, VERSION)
        .expect("the installed pack re-proves offline");
    assert_eq!(
        hub.refresh_catalog(&OfflineTransport)
            .expect_err("no network"),
        ModelHubError::Offline
    );
    // Refusing to refresh does not discard what is already proved.
    assert!(hub.snapshot().is_some());
    assert_eq!(hub.installed().len(), 1);
}

#[test]
fn a_pack_requiring_an_unknown_capability_is_incompatible_and_refused() {
    let key = hub_signing_key();
    let capabilities = ["subckt", "nonexistent-capability"];
    let archive = signed_archive(&key, &capabilities);
    let snapshot = signed_snapshot(&key, &archive, &capabilities, VERSION);
    let tree = TempTree::new("capability");
    let mut hub = filesystem_hub(&tree, &key);
    let transport = StubTransport::with_snapshot(snapshot).serving(VERSION, archive);
    hub.refresh_catalog(&transport).expect("catalog");

    assert_eq!(
        hub.compatibility(PACK_ID, VERSION)
            .expect("a listed release"),
        vec!["nonexistent-capability".to_owned()]
    );
    let row = hub
        .part_index(&[])
        .into_iter()
        .find(|row| row.part_id == PART_ID)
        .expect("the catalog row is still indexed");
    assert_eq!(
        row.state,
        PartState::Incompatible {
            missing: vec!["nonexistent-capability".to_owned()],
        }
    );
    assert_eq!(
        row.provenance,
        PartProvenance::RemoteRelease {
            pack_id: PACK_ID.to_owned(),
            version: VERSION.to_owned(),
        }
    );

    let refusal = hub
        .install(&transport, PACK_ID, VERSION)
        .expect_err("an incompatible pack is refused");
    assert_eq!(
        refusal,
        ModelHubError::Incompatible {
            missing: vec!["nonexistent-capability".to_owned()],
        }
    );
    // Refused before anything was fetched, and nothing landed.
    assert!(!transport.calls().contains(&"archive_handoff".to_owned()));
    assert!(hub.installed().is_empty());
    assert!(!packs_root(&tree).join(PACK_ID).exists());
}

#[test]
fn a_newer_listed_release_marks_an_installed_pack_as_updatable() {
    let key = hub_signing_key();
    let archive = signed_archive(&key, &["subckt", "resistor"]);
    let tree = TempTree::new("update");
    let mut hub = filesystem_hub(&tree, &key);
    let installed_snapshot = signed_snapshot(&key, &archive, &["subckt", "resistor"], VERSION);
    let transport =
        StubTransport::with_snapshot(installed_snapshot).serving(VERSION, archive.clone());
    hub.refresh_catalog(&transport).expect("catalog");
    hub.install(&transport, PACK_ID, VERSION).expect("install");

    // A later catalog generation lists a genuinely newer release beside the
    // one this machine holds, which is the shape a real update is read from.
    let next_archive = signed_archive_at(&key, &["subckt", "resistor"], NEXT_VERSION);
    let newer_snapshot = signed_snapshot_of(
        &key,
        &[
            (VERSION, &archive, &["subckt", "resistor"]),
            (NEXT_VERSION, &next_archive, &["subckt", "resistor"]),
        ],
    );
    let newer = StubTransport::with_snapshot(newer_snapshot).at_generation(8);
    assert_eq!(hub.refresh_catalog(&newer).expect("catalog"), 8);
    let rows = hub.part_index(&[]);
    let row = rows
        .iter()
        .find(|row| {
            matches!(row.provenance, PartProvenance::InstalledPack { .. })
                && row.part_id == PART_ID
        })
        .expect("the installed row is indexed");
    assert_eq!(
        row.state,
        PartState::UpdateAvailable {
            installed: VERSION.to_owned(),
            latest: NEXT_VERSION.to_owned(),
        }
    );
    // The release still listed at the installed version is not offered twice:
    // it is reported once, as the installation it is.
    assert_eq!(
        rows.iter()
            .filter(|row| row.part_id == PART_ID
                && row.provenance
                    == PartProvenance::RemoteRelease {
                        pack_id: PACK_ID.to_owned(),
                        version: NEXT_VERSION.to_owned(),
                    })
            .count(),
        1
    );
    assert!(!rows.iter().any(|row| {
        row.provenance
            == PartProvenance::RemoteRelease {
                pack_id: PACK_ID.to_owned(),
                version: VERSION.to_owned(),
            }
    }));
}

#[test]
fn uninstalling_removes_the_hub_copy_and_nothing_a_project_retained() {
    let key = hub_signing_key();
    let archive = signed_archive(&key, &["subckt", "resistor"]);
    let snapshot = signed_snapshot(&key, &archive, &["subckt", "resistor"], VERSION);
    let tree = TempTree::new("uninstall");
    let mut hub = filesystem_hub(&tree, &key);
    let transport = StubTransport::with_snapshot(snapshot).serving(VERSION, archive);
    hub.refresh_catalog(&transport).expect("catalog");
    hub.install(&transport, PACK_ID, VERSION).expect("install");

    // The hub's own expanded copy, named from the tree rather than from the
    // hub, so the assertion below still means "the files are gone" after the
    // hub has stopped believing the release exists.
    let installed_source = packs_root(&tree)
        .join(PACK_ID)
        .join(VERSION)
        .join("files")
        .join("models")
        .join("proving.lib");
    assert!(installed_source.is_file());
    let mut manager = crate::state::model_library::ModelLibraryManager::new();
    let library_name = hub
        .add_part_to_project(&mut manager, PACK_ID, VERSION, PART_ID)
        .expect("retain into a project");

    assert!(hub.uninstall(PACK_ID, VERSION).expect("uninstall"));
    assert!(hub.installed().is_empty());
    assert!(!installed_source.exists());
    assert!(!hub.uninstall(PACK_ID, VERSION).expect("already removed"));

    // The project still holds its own authenticated bytes.
    let library = manager.get_library(&library_name).expect("library");
    assert!(!library.source_contents.is_empty());
    manager
        .reference_process_model_cards(crate::simulation::dialog::corner::ProcessCorner::TT)
        .expect("the retained snapshot survives uninstalling the pack");
}

#[test]
fn a_project_without_a_pack_pin_round_trips_unchanged() {
    // The exact shape a project written before pack distribution carries: no
    // `pack_pin` key anywhere in the document.
    let legacy = serde_json::json!({
        "libraries": {
            "legacy": {
                "name": "legacy",
                "pdk_name": "",
                "technology_node": "",
                "root_path": null,
                "models": {},
                "corners": {},
                "selected_corner": null,
                "version": "1.0",
                "expanded": false,
            }
        },
        "selected_library": null,
        "filter_text": "",
        "filter_type": null,
    });
    let text = serde_json::to_string(&legacy).expect("fixture serializes");
    assert!(!text.contains("pack_pin"));

    let manager: crate::state::model_library::ModelLibraryManager =
        serde_json::from_str(&text).expect("a project without pins still loads");
    let library = manager
        .get_library("legacy")
        .expect("the legacy library survives the round trip");
    assert_eq!(library.pack_pin, None);
    assert_eq!(library.pack_id, None);
    assert_eq!(
        library.source_authority,
        crate::state::model_library::ModelSourceAuthority::BuiltIn
    );

    // Re-serializing and reloading is stable, and the absent pin stays absent.
    let round_tripped = serde_json::to_string(&manager).expect("serializes");
    let reloaded: crate::state::model_library::ModelLibraryManager =
        serde_json::from_str(&round_tripped).expect("reloads");
    assert_eq!(
        reloaded
            .get_library("legacy")
            .and_then(|library| library.pack_pin.clone()),
        None
    );
}

#[test]
fn the_memory_store_runs_the_same_pipeline_the_browser_build_uses() {
    let key = hub_signing_key();
    let archive = signed_archive(&key, &["subckt", "resistor"]);
    let snapshot = signed_snapshot(&key, &archive, &["subckt", "resistor"], VERSION);
    let mut hub = ModelHub::open(anchor_for(&key), Box::new(MemoryModelHubStore::new()), None)
        .expect("the hub opens over an empty store");
    let transport = StubTransport::with_snapshot(snapshot).serving(VERSION, archive.clone());

    hub.refresh_catalog(&transport).expect("catalog");
    let installed = hub.install(&transport, PACK_ID, VERSION).expect("install");
    assert_eq!(installed.archive_sha256, sha256_hex(&archive));
    hub.verify_installed(PACK_ID, VERSION)
        .expect("the stored archive re-proves");
    assert_eq!(
        hub.installed_file(PACK_ID, VERSION, SOURCE_PATH)
            .expect("the expanded source is readable"),
        PROVING_LIB.as_bytes()
    );
    // A browser store has no filesystem, so no row claims a source path.
    assert!(hub.part_index(&[]).iter().all(|row| row.source.is_none()));

    // And a part still reaches the project. Retention is over bytes this hub
    // just proved, never over an expanded directory, so a store with no disk
    // completes the whole gesture rather than refusing the last step of it.
    let mut manager = crate::state::model_library::ModelLibraryManager::new();
    let library_name = hub
        .add_part_to_project(&mut manager, PACK_ID, VERSION, PART_ID)
        .expect("a memory store retains a part into the project");
    let library = manager
        .get_library(&library_name)
        .expect("the retained library exists");
    assert!(matches!(
        library.source_authority,
        crate::state::model_library::ModelSourceAuthority::RetainedImport { .. }
    ));
    assert_eq!(library.source_contents.len(), library.source_closure.len());
    assert_eq!(
        library.pack_pin.as_ref().map(|pin| pin.part_id.as_str()),
        Some(PART_ID)
    );
    assert!((retained_divider_output(&manager) - 0.5).abs() < 1.0e-9);
}

/// The browser and the desktop must produce the same project, not merely two
/// projects that both work.
///
/// The two stores keep pack bytes in completely different places — one in a
/// directory tree, one in a map — so the only way to know that difference does
/// not leak into saved designs is to build a project from each and compare the
/// documents. The retained-source identity is minted fresh per import by
/// design and is fixed here before the comparison; everything else, including
/// every retained path, digest, and edge, must agree byte for byte.
#[test]
fn both_stores_retain_the_same_project_document_for_the_same_release() {
    let key = hub_signing_key();
    let capabilities = ["subckt", "resistor"];
    let archive = signed_archive(&key, &capabilities);
    let snapshot = signed_snapshot(&key, &archive, &capabilities, VERSION);

    let tree = TempTree::new("store-parity");
    let mut filesystem = filesystem_hub(&tree, &key);
    let transport =
        StubTransport::with_snapshot(snapshot.clone()).serving(VERSION, archive.clone());
    filesystem.refresh_catalog(&transport).expect("catalog");
    filesystem
        .install(&transport, PACK_ID, VERSION)
        .expect("install");
    let mut from_filesystem = crate::state::model_library::ModelLibraryManager::new();
    let filesystem_library = filesystem
        .add_part_to_project(&mut from_filesystem, PACK_ID, VERSION, PART_ID)
        .expect("a filesystem store retains the part");

    let mut memory = ModelHub::open(anchor_for(&key), Box::new(MemoryModelHubStore::new()), None)
        .expect("the hub opens over an empty store");
    let transport = StubTransport::with_snapshot(snapshot).serving(VERSION, archive);
    memory.refresh_catalog(&transport).expect("catalog");
    memory.install(&transport, PACK_ID, VERSION).expect("install");
    let mut from_memory = crate::state::model_library::ModelLibraryManager::new();
    let memory_library = memory
        .add_part_to_project(&mut from_memory, PACK_ID, VERSION, PART_ID)
        .expect("a memory store retains the part");

    assert_eq!(filesystem_library, memory_library);
    let shared = retained_source_id(&from_filesystem, &filesystem_library);
    fix_retained_source_ids(&mut from_memory, shared);
    assert_eq!(
        serde_json::to_string(&from_filesystem).expect("the filesystem project serializes"),
        serde_json::to_string(&from_memory).expect("the memory project serializes"),
        "the store a pack was installed through is not part of the project it produces"
    );
    assert!((retained_divider_output(&from_memory) - 0.5).abs() < 1.0e-9);
}

/// Solves the retained divider and returns V(OUT) for a one-volt input.
///
/// The answer is the point: bytes that merely *exist* prove nothing, and the
/// two releases of the fixture pack divide differently, so the ratio names
/// which release's source the project is actually running.
fn retained_divider_output(manager: &crate::state::model_library::ModelLibraryManager) -> f64 {
    let cards = manager
        .reference_process_model_cards(crate::simulation::dialog::corner::ProcessCorner::TT)
        .expect("the retained snapshot materializes without its source file");
    let deck = format!(
        "model hub proving deck\n{}\nV1 IN 0 1\nX1 IN OUT {PART_ID}\n.op\n.end\n",
        cards.join("\n")
    );
    let netlist = rspice_core::Netlist::parse(&deck).expect("the retained deck parses");
    rspice_core::Engine::new(rspice_core::SimulationConfig::default())
        .run_dc_op(&netlist)
        .expect("the pack's part solves through the retained path")
        .try_voltage_named("OUT")
        .expect("the divider output is a solved node")
}

/// The retained-source identity one library was given.
fn retained_source_id(
    manager: &crate::state::model_library::ModelLibraryManager,
    library: &str,
) -> crate::product::ModelSourceId {
    match manager
        .get_library(library)
        .expect("the retained library exists")
        .source_authority
    {
        crate::state::model_library::ModelSourceAuthority::RetainedImport { source_id, .. } => {
            source_id
        }
        other => panic!("a retained pack part carries retained authority, not {other:?}"),
    }
}

/// Replaces every retained-source identity, so two independently built
/// projects can be compared on everything a fresh identity would mask.
fn fix_retained_source_ids(
    manager: &mut crate::state::model_library::ModelLibraryManager,
    replacement: crate::product::ModelSourceId,
) {
    let names = manager
        .libraries_sorted()
        .iter()
        .map(|library| library.name.clone())
        .collect::<Vec<_>>();
    for name in names {
        if let Some(library) = manager.get_library_mut(&name)
            && let crate::state::model_library::ModelSourceAuthority::RetainedImport {
                source_id,
                ..
            } = &mut library.source_authority
        {
            *source_id = replacement;
        }
    }
}
