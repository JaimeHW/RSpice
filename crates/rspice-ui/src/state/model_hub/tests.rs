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
    License, ManifestTemplate, Part, PartKind, Requires, Revocation, Snapshot, SnapshotPack,
    SnapshotPart, SnapshotRelease, SourceRef, build_pack, encode_snapshot, sha256_hex, signing_key,
};

use super::{
    ModelHub, ModelHubError,
    provider::{PartProvenance, PartState},
    store::{FilesystemModelHubStore, MemoryModelHubStore, ModelHubStore, STAGING_PREFIX},
    transport::{ArchiveHandoff, CatalogHandoff, ModelHubTransport, OfflineTransport},
    trust::TrustAnchor,
};

/// When the fixture catalogs claim to have been signed, and until when.
///
/// The horizon is far enough out that a build running in 2099 is the only way
/// these tests start failing for a reason nobody changed — and near enough to
/// be a real RFC 3339 instant the format accepts rather than a sentinel.
pub(crate) const SIGNED_AT: &str = "2026-08-15T09:30:00Z";
pub(crate) const STANDS_UNTIL: &str = "2099-12-31T23:59:59Z";
/// A horizon every clock this code can run under is already past.
pub(crate) const EXPIRED_AT: &str = "2020-01-01T00:00:00Z";

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
            description: Some("Proving-pack resistive divider".to_owned()),
            // Two keys the shelf declares a column for under one class facet,
            // and one it declares under none — so a fixture proves both that a
            // declared column lights and that an undeclared key stays off the
            // table without being lost.
            specs: BTreeMap::from([
                ("VR".to_owned(), "50 V".to_owned()),
                ("IF".to_owned(), "150 mA".to_owned()),
                ("ratio".to_owned(), "1:2".to_owned()),
            ]),
        }],
    }
}

/// A second part, in a second source file.
///
/// Two files rather than two subcircuits in one, because the project retains a
/// part's *reachable closure* as one library: two parts sharing a file are one
/// pin, and a lane proving that adopting one part leaves another alone needs
/// them to be two.
pub(crate) const BIAS_SOURCE_PATH: &str = "models/proving-bias.lib";
pub(crate) const BIAS_PART_ID: &str = "RSPICE_PROVING_BIAS";

/// The alias the successor release adds to the divider part.
///
/// It is the difference the *catalog* can state about a part both releases
/// publish. Schema 1 has no per-part digest, so without a published field
/// actually moving, a diff between two releases could only ever compare part
/// lists — and a fixture that never exercises the other half would let the
/// comparison rot unnoticed.
pub(crate) const DIVIDER_ALIAS_NEXT: &str = "RSPICE_PROVING_DIVIDER";

/// The second part's deck, identical in both releases.
const PROVING_BIAS_LIB: &str = "* RSpice proving pack\n\
                                .subckt RSPICE_PROVING_BIAS IN OUT\n\
                                R1 IN OUT 4k\n\
                                R2 OUT 0 1k\n\
                                .ends RSPICE_PROVING_BIAS\n";

/// Builds a real signed archive for the two-part fixture pack.
///
/// Between the two releases the divider's *source* changes and its catalog
/// listing gains an alias, while the bias part changes in neither. That is
/// exactly the pair a per-part adoption has to tell apart.
pub(crate) fn two_part_archive(
    key: &rspice_pack::SigningKey,
    capabilities: &[&str],
    version: &str,
) -> Vec<u8> {
    let base = template(version, capabilities);
    let mut parts = base.parts.clone();
    if version != VERSION {
        parts[0].aliases = vec![DIVIDER_ALIAS_NEXT.to_owned()];
    }
    parts.push(Part {
        id: BIAS_PART_ID.to_owned(),
        kind: PartKind::Subckt,
        device: "network".to_owned(),
        aliases: Vec::new(),
        source: SourceRef {
            path: BIAS_SOURCE_PATH.to_owned(),
            line: 2,
        },
        terminals: vec!["IN".to_owned(), "OUT".to_owned()],
        symbol: None,
        description: None,
        specs: BTreeMap::new(),
    });
    let divider = if version == VERSION {
        PROVING_LIB
    } else {
        PROVING_LIB_NEXT
    };
    build_pack(
        &[
            (SOURCE_PATH.to_owned(), divider.as_bytes().to_vec()),
            (
                BIAS_SOURCE_PATH.to_owned(),
                PROVING_BIAS_LIB.as_bytes().to_vec(),
            ),
        ],
        ManifestTemplate { parts, ..base },
        key,
    )
    .expect("the two-part fixture pack is well formed")
}

/// Builds a signed snapshot whose part lists are read out of the archives.
///
/// Projecting each manifest rather than restating it is what stops a fixture
/// catalog from publishing something the bytes it points at do not say — which
/// is precisely the defect a diff computed over the catalog would inherit and
/// present as fact.
pub(crate) fn signed_snapshot_projecting(
    key: &rspice_pack::SigningKey,
    releases: &[(&str, &[u8], &[&str])],
) -> Vec<u8> {
    let anchor = anchor_for(key);
    let snapshot = Snapshot {
        schema: rspice_pack::SNAPSHOT_SCHEMA,
        serial: 1,
        generated_at: SIGNED_AT.to_owned(),
        expires_at: STANDS_UNTIL.to_owned(),
        revocations: Vec::new(),
        packs: vec![SnapshotPack {
            id: PACK_ID.to_owned(),
            name: "RSpice proving pack".to_owned(),
            category: "proving".to_owned(),
            releases: releases
                .iter()
                .map(|(version, archive, capabilities)| {
                    let verified =
                        rspice_pack::Pack::verify(archive, anchor.key(), anchor.limits())
                            .expect("the fixture archive proves under its own key");
                    SnapshotRelease {
                        version: (*version).to_owned(),
                        archive_sha256: sha256_hex(archive),
                        archive_length: archive.len() as u64,
                        capabilities: capabilities
                            .iter()
                            .map(|value| (*value).to_owned())
                            .collect(),
                        spdx: "LicenseRef-RSpice-Models".to_owned(),
                        parts: verified
                            .manifest
                            .parts
                            .iter()
                            .map(|part| SnapshotPart {
                                id: part.id.clone(),
                                kind: part.kind,
                                device: part.device.clone(),
                                aliases: part.aliases.clone(),
                                terminals: part.terminals.clone(),
                                symbol: part.symbol.clone(),
                                description: part.description.clone(),
                                specs: part.specs.clone(),
                            })
                            .collect(),
                    }
                })
                .collect(),
        }],
    };
    encode_snapshot(&snapshot, key).expect("the projected fixture snapshot is well formed")
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
    signed_snapshot_on(key, releases, &CatalogTerms::default())
}

/// What a fixture catalog says about itself beyond the releases it lists.
///
/// The three schema-2 trust fields, gathered so a test states only the one it
/// is about. Every default is the healthy value, which is what keeps a test
/// that says nothing about expiry from accidentally being a test about it.
#[derive(Debug, Clone)]
pub(crate) struct CatalogTerms {
    pub(crate) serial: u64,
    pub(crate) expires_at: String,
    pub(crate) revocations: Vec<Revocation>,
}

impl Default for CatalogTerms {
    fn default() -> Self {
        Self {
            serial: 1,
            expires_at: STANDS_UNTIL.to_owned(),
            revocations: Vec::new(),
        }
    }
}

impl CatalogTerms {
    /// The same catalog published at a later ordinal.
    pub(crate) fn at_serial(serial: u64) -> Self {
        Self {
            serial,
            ..Self::default()
        }
    }

    /// A catalog whose horizon has already passed.
    pub(crate) fn expired() -> Self {
        Self {
            expires_at: EXPIRED_AT.to_owned(),
            ..Self::default()
        }
    }

    /// A catalog recalling one release of the fixture pack.
    pub(crate) fn recalling(version: &str, reason: &str) -> Self {
        Self {
            revocations: vec![Revocation {
                pack_id: PACK_ID.to_owned(),
                version: version.to_owned(),
                reason: reason.to_owned(),
            }],
            ..Self::default()
        }
    }
}

/// Builds a real signed snapshot on stated terms.
pub(crate) fn signed_snapshot_on(
    key: &rspice_pack::SigningKey,
    releases: &[(&str, &[u8], &[&str])],
    terms: &CatalogTerms,
) -> Vec<u8> {
    let snapshot = Snapshot {
        schema: rspice_pack::SNAPSHOT_SCHEMA,
        serial: terms.serial,
        generated_at: SIGNED_AT.to_owned(),
        expires_at: terms.expires_at.clone(),
        revocations: terms.revocations.clone(),
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
                        description: Some("Proving-pack resistive divider".to_owned()),
                        specs: BTreeMap::from([
                            ("VR".to_owned(), "50 V".to_owned()),
                            ("IF".to_owned(), "150 mA".to_owned()),
                            ("ratio".to_owned(), "1:2".to_owned()),
                        ]),
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
            matches!(row.provenance, PartProvenance::InstalledPack { .. }) && row.part_id == PART_ID
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
        .reference_process_model_cards(crate::product::ProcessCorner::TT)
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

/// Discovery already hashes every installed archive; now it compares.
///
/// The digest read at startup is the value a project pin records, and for a
/// release it was compared against nothing at all — a machine whose archive
/// had been replaced looked exactly like one whose had not. The signed catalog
/// publishes the digest each release must have, so the comparison is free and
/// the only thing that was missing was making it.
#[test]
fn the_startup_sweep_compares_each_archive_against_the_catalog_that_signed_it() {
    use super::ArchiveEvidence;

    let key = hub_signing_key();
    let archive = signed_archive(&key, &["subckt", "resistor"]);
    let snapshot = signed_snapshot(&key, &archive, &["subckt", "resistor"], VERSION);
    let store = std::sync::Arc::new(MemoryModelHubStore::new());
    let mut hub = ModelHub::open(anchor_for(&key), Box::new(store.clone()), None)
        .expect("the hub opens over an empty store");
    let transport =
        StubTransport::with_snapshot(snapshot.clone()).serving(VERSION, archive.clone());
    hub.refresh_catalog(&transport).expect("catalog");
    hub.install(&transport, PACK_ID, VERSION).expect("install");
    assert_eq!(
        hub.archive_evidence(PACK_ID, VERSION),
        Some(ArchiveEvidence::MatchesCatalog)
    );
    assert!(!hub.catalog_cache_discarded());

    // A hub opened over the same store, but under a catalog that publishes a
    // different archive for this release, is looking at bytes that are not the
    // ones that release was proved as.
    let other = signed_archive_at(&key, &["subckt", "resistor"], NEXT_VERSION);
    let disagreeing = signed_snapshot(&key, &other, &["subckt", "resistor"], VERSION);
    store
        .write_snapshot(&disagreeing)
        .expect("cache the disagreeing catalog");
    let reopened = ModelHub::open(anchor_for(&key), Box::new(store.clone()), None)
        .expect("the hub reopens over the same store");
    assert_eq!(
        reopened.archive_evidence(PACK_ID, VERSION),
        Some(ArchiveEvidence::DiffersFromCatalog)
    );

    // A cached catalog that no longer verifies is dropped — and *says* it was
    // dropped, rather than presenting as a client that has never fetched.
    store
        .write_snapshot(b"not a snapshot")
        .expect("cache an unverifiable catalog");
    let discarded = ModelHub::open(anchor_for(&key), Box::new(store.clone()), None)
        .expect("an unverifiable cache is a state, not a failure to open");
    assert!(discarded.snapshot().is_none());
    assert!(discarded.catalog_cache_discarded());
    assert_eq!(
        discarded.archive_evidence(PACK_ID, VERSION),
        Some(ArchiveEvidence::NotPublished),
        "with no catalog there is nothing to compare against, which is its own answer"
    );

    // And a hub that never had a cache does not claim one was thrown away.
    let fresh = ModelHub::open(anchor_for(&key), Box::new(MemoryModelHubStore::new()), None)
        .expect("the hub opens over an empty store");
    assert!(!fresh.catalog_cache_discarded());
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
    memory
        .install(&transport, PACK_ID, VERSION)
        .expect("install");
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
        .reference_process_model_cards(crate::product::ProcessCorner::TT)
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

/// The store behind a filesystem hub, reachable without the hub.
///
/// The anti-rollback floor is a property of the *store*: a test that could only
/// reach it through the hub that wrote it could not tell a durable floor from a
/// field in memory.
fn filesystem_store(tree: &TempTree) -> FilesystemModelHubStore {
    FilesystemModelHubStore::new(tree.path())
}

/// The fixture pack's one release, listed at a stated catalog serial.
fn catalog_at(key: &rspice_pack::SigningKey, archive: &[u8], serial: u64) -> Vec<u8> {
    signed_snapshot_on(
        key,
        &[(VERSION, archive, &["subckt", "resistor"])],
        &CatalogTerms::at_serial(serial),
    )
}

#[test]
fn a_catalog_serial_below_the_one_already_accepted_is_refused_and_changes_nothing() {
    let key = hub_signing_key();
    let archive = signed_archive(&key, &["subckt", "resistor"]);
    let tree = TempTree::new("rollback");
    let mut hub = filesystem_hub(&tree, &key);

    let current = StubTransport::with_snapshot(catalog_at(&key, &archive, 5))
        .serving(VERSION, archive.clone());
    hub.refresh_catalog(&current).expect("serial 5 is accepted");
    hub.install(&current, PACK_ID, VERSION).expect("install");
    let held = hub
        .catalog_identity()
        .expect("a catalog is held")
        .digest
        .clone();
    assert_eq!(hub.last_seen_serial(), 5);

    // An authentic catalog, signed by the same key, from before the floor.
    let replay = StubTransport::with_snapshot(catalog_at(&key, &archive, 4));
    let refusal = hub
        .refresh_catalog(&replay)
        .expect_err("a serial below the floor is refused");
    assert_eq!(
        refusal,
        ModelHubError::CatalogRollback {
            held: 5,
            offered: 4
        }
    );
    // Refused before anything was written: the held catalog, the installed
    // release and the floor are all exactly where they were.
    assert_eq!(hub.catalog_identity().expect("still held").digest, held);
    assert_eq!(hub.catalog_identity().expect("still held").serial, 5);
    assert_eq!(hub.last_seen_serial(), 5);
    assert_eq!(hub.installed().len(), 1);
    assert_eq!(
        filesystem_store(&tree)
            .read_catalog_serial()
            .expect("the store reports its floor"),
        5
    );

    // The refusal names both sides. Which banner rung its sentence classifies
    // onto is asserted where the banner lives, in
    // `workbench::app::dialogs::pdk_workflow::model_hub::tests`: this module
    // sits below the workspace and may not reach up into it.
    let sentence = refusal.to_string();
    assert!(
        sentence.contains("serial 4") && sentence.contains("serial 5"),
        "the refusal names what was offered and what is held: {sentence}"
    );
}

#[test]
fn an_equal_serial_is_a_re_fetch_and_a_later_one_supersedes_it() {
    let key = hub_signing_key();
    let archive = signed_archive(&key, &["subckt", "resistor"]);
    let tree = TempTree::new("serial-forward");
    let mut hub = filesystem_hub(&tree, &key);

    let fifth = StubTransport::with_snapshot(catalog_at(&key, &archive, 5));
    hub.refresh_catalog(&fifth).expect("serial 5");
    // The same catalog fetched again is the ordinary outcome of a refresh on a
    // quiet day, and must not be an error.
    hub.refresh_catalog(&fifth)
        .expect("an equal serial is the same catalog re-fetched");
    assert_eq!(hub.last_seen_serial(), 5);

    let sixth = StubTransport::with_snapshot(catalog_at(&key, &archive, 6));
    hub.refresh_catalog(&sixth).expect("a later serial");
    assert_eq!(hub.last_seen_serial(), 6);
    assert_eq!(hub.catalog_identity().expect("held").serial, 6);
}

/// The floor is durable, and no catalog can lower it by arriving.
///
/// Two halves, because the mechanism makes two claims. It survives closing the
/// application, which a field in memory would not; and it survives the cached
/// catalog being replaced wholesale with an older one, which reading the floor
/// out of the cache would not.
#[test]
fn the_accepted_serial_survives_a_reopen_and_a_wholesale_catalog_replacement() {
    let key = hub_signing_key();
    let archive = signed_archive(&key, &["subckt", "resistor"]);
    let tree = TempTree::new("serial-durable");
    let older = catalog_at(&key, &archive, 4);

    {
        let mut hub = filesystem_hub(&tree, &key);
        let current = StubTransport::with_snapshot(catalog_at(&key, &archive, 9));
        hub.refresh_catalog(&current).expect("serial 9");
        assert_eq!(hub.last_seen_serial(), 9);
    }

    // Reopened over the same directory, as a second launch would.
    let reopened = filesystem_hub(&tree, &key);
    assert_eq!(reopened.last_seen_serial(), 9);
    drop(reopened);

    // Now the cached catalog is replaced wholesale with an authentic older
    // one, which is what anything with write access to the cache produces —
    // including a restored backup. The floor is a separate file and untouched.
    filesystem_store(&tree)
        .write_snapshot(&older)
        .expect("the cache is replaced");
    let mut hub = filesystem_hub(&tree, &key);
    assert_eq!(
        hub.last_seen_serial(),
        9,
        "the floor outlives the catalog it was learned from"
    );
    // And the substituted cache is not what the hub ends up holding. It is
    // authentic, so it decodes; it is below the floor, so it is a replay, and
    // a replay that this client *held* would undo every recall published
    // between serial 4 and serial 9 — the exact outcome the floor exists to
    // prevent. Refusing it only at the next refresh would have left the
    // window between the substitution and that refresh wide open.
    assert!(
        hub.catalog_identity().is_none(),
        "a cache below the floor is a replay, and holding it would undo a recall"
    );
    assert!(
        hub.catalog_cache_discarded(),
        "and the reader is told the cache was rejected rather than never written"
    );
    let replay = StubTransport::with_snapshot(older);
    assert_eq!(
        hub.refresh_catalog(&replay)
            .expect_err("the replay is still refused"),
        ModelHubError::CatalogRollback {
            held: 9,
            offered: 4
        }
    );
}

/// An expired catalog silences the hub and nothing else.
///
/// The offline half is the one that matters: a machine that has not reached the
/// service since the catalog lapsed must still open its projects, keep its
/// packs, and *solve*. Proving that with a real engine run rather than with a
/// row count is the difference between a claim and evidence.
#[test]
fn an_expired_catalog_withholds_every_offer_and_blocks_no_local_work() {
    let key = hub_signing_key();
    let archive = signed_archive(&key, &["subckt", "resistor"]);
    let tree = TempTree::new("expiry");
    let mut hub = filesystem_hub(&tree, &key);

    let current = StubTransport::with_snapshot(catalog_at(&key, &archive, 1))
        .serving(VERSION, archive.clone());
    hub.refresh_catalog(&current)
        .expect("a catalog that stands");
    hub.install(&current, PACK_ID, VERSION).expect("install");
    assert!(hub.catalog_expired().is_none());

    // The service now publishes a catalog whose horizon has passed. Fetching
    // one is not itself an error — a lapsed catalog is still the best evidence
    // this client has — so it caches, and only offering from it stops.
    let lapsed = signed_snapshot_on(
        &key,
        &[(VERSION, &archive, &["subckt", "resistor"])],
        &CatalogTerms {
            serial: 2,
            ..CatalogTerms::expired()
        },
    );
    let stale = StubTransport::with_snapshot(lapsed).serving(VERSION, archive.clone());
    hub.refresh_catalog(&stale)
        .expect("an expired catalog caches");
    assert_eq!(hub.catalog_expired(), Some(EXPIRED_AT));
    assert!(hub.offered_snapshot().is_none());
    assert!(
        hub.snapshot().is_some(),
        "the held catalog is still readable — only offering from it stops"
    );

    // Every hub offer refuses, naming the instant it is refusing against.
    let refusal = hub
        .install(&stale, PACK_ID, VERSION)
        .expect_err("installing from an expired catalog refuses");
    assert_eq!(
        refusal,
        ModelHubError::CatalogExpired {
            expires_at: EXPIRED_AT.to_owned(),
        }
    );

    // Browsing offers nothing remote, and keeps every row for what is here.
    let rows = hub.part_index(&[]);
    assert!(
        rows.iter()
            .all(|row| !matches!(row.provenance, PartProvenance::RemoteRelease { .. })),
        "an expired catalog offers no remote row"
    );
    assert!(
        rows.iter().any(|row| row.part_id == PART_ID
            && matches!(row.provenance, PartProvenance::InstalledPack { .. })),
        "and the installed release is still indexed"
    );

    // Local work is untouched: the installed release re-proves, a part still
    // retains into a project, and the retained deck still solves.
    hub.verify_installed(PACK_ID, VERSION)
        .expect("an expired catalog does not stop a release re-proving");
    let mut manager = crate::state::model_library::ModelLibraryManager::new();
    hub.add_part_to_project(&mut manager, PACK_ID, VERSION, PART_ID)
        .expect("retention is local work and keeps working");
    assert!((retained_divider_output(&manager) - 0.5).abs() < 1.0e-9);

    // And refreshing into a catalog that stands restores every offer.
    let restored = StubTransport::with_snapshot(catalog_at(&key, &archive, 3));
    hub.refresh_catalog(&restored).expect("a fresh catalog");
    assert!(hub.catalog_expired().is_none());
    assert!(hub.offered_snapshot().is_some());
    hub.require_current_catalog()
        .expect("offers are open again");
}

/// A recall is a recall, not an erasure.
///
/// Both halves: the release leaves every offer and refuses every way of taking
/// something new from it, and the bytes a project already retained keep solving
/// to the same answer they always did.
#[test]
fn a_recalled_release_is_withheld_and_refused_everywhere_it_is_named() {
    const REASON: &str = "the divider ratio was published against the wrong reference.";

    let key = hub_signing_key();
    let first = signed_archive_at(&key, &["subckt", "resistor"], VERSION);
    let second = signed_archive_at(&key, &["subckt", "resistor"], NEXT_VERSION);
    let releases: [(&str, &[u8], &[&str]); 2] = [
        (VERSION, &first, &["subckt", "resistor"]),
        (NEXT_VERSION, &second, &["subckt", "resistor"]),
    ];
    let tree = TempTree::new("recall");
    let mut hub = filesystem_hub(&tree, &key);

    let listing = StubTransport::with_snapshot(signed_snapshot_on(
        &key,
        &releases,
        &CatalogTerms::at_serial(1),
    ))
    .serving(VERSION, first.clone())
    .serving(NEXT_VERSION, second.clone());
    hub.refresh_catalog(&listing).expect("catalog");
    hub.install(&listing, PACK_ID, VERSION).expect("install");
    let mut project = crate::state::model_library::ModelLibraryManager::new();
    hub.add_part_to_project(&mut project, PACK_ID, VERSION, PART_ID)
        .expect("the project retains a part before the recall");
    assert!((retained_divider_output(&project) - 0.5).abs() < 1.0e-9);
    assert!(
        hub.part_index(&[]).iter().any(
            |row| matches!(&row.provenance, PartProvenance::RemoteRelease { version, .. }
                if version == NEXT_VERSION)
        ),
        "1.1.0 is on offer before it is recalled"
    );

    // The publisher recalls the successor.
    let recalled = StubTransport::with_snapshot(signed_snapshot_on(
        &key,
        &releases,
        &CatalogTerms {
            serial: 2,
            ..CatalogTerms::recalling(NEXT_VERSION, REASON)
        },
    ))
    .serving(NEXT_VERSION, second.clone());
    hub.refresh_catalog(&recalled).expect("the recall arrives");
    assert_eq!(hub.recalls().reason(PACK_ID, NEXT_VERSION), Some(REASON));
    assert_eq!(
        hub.recalls().reason(PACK_ID, VERSION),
        None,
        "a recall names one release, not a pack"
    );

    let revoked = ModelHubError::ReleaseRevoked {
        pack_id: PACK_ID.to_owned(),
        version: NEXT_VERSION.to_owned(),
        reason: REASON.to_owned(),
    };
    // Install, update — which installs the newer release — and adoption all
    // refuse, each carrying the publisher's recorded reason rather than a bare
    // no.
    assert_eq!(
        hub.install(&recalled, PACK_ID, NEXT_VERSION)
            .expect_err("installing a recalled release refuses"),
        revoked
    );
    assert_eq!(
        hub.adoptable(PACK_ID, NEXT_VERSION)
            .expect_err("adopting onto a recalled release refuses"),
        revoked
    );
    assert_eq!(
        hub.part_pin(PACK_ID, NEXT_VERSION, PART_ID)
            .expect_err("retaining from a recalled release refuses"),
        revoked
    );
    assert!(
        revoked.to_string().contains(REASON),
        "the refusal carries the reason: {revoked}"
    );

    // It leaves the offer, and stops being offered as an update.
    let rows = hub.part_index(&[]);
    assert!(
        rows.iter().all(
            |row| !matches!(&row.provenance, PartProvenance::RemoteRelease { version, .. }
                if version == NEXT_VERSION)
        ),
        "a recalled release is dropped from browse rather than shown refused"
    );
    assert!(
        rows.iter()
            .filter(|row| matches!(row.provenance, PartProvenance::InstalledPack { .. }))
            .all(|row| row.state == PartState::Installed),
        "and no row offers an update onto it"
    );

    // Nothing local moved. The installed release is still installed, still
    // re-proves, and the retained project still solves to its own answer.
    assert_eq!(hub.installed().len(), 1);
    hub.verify_installed(PACK_ID, VERSION)
        .expect("a recall elsewhere does not disturb what is installed");
    assert!((retained_divider_output(&project) - 0.5).abs() < 1.0e-9);

    // Now the release this machine holds is recalled too. Retention from it
    // refuses; the bytes already retained are neither deleted nor blocked.
    let held_recalled = StubTransport::with_snapshot(signed_snapshot_on(
        &key,
        &releases,
        &CatalogTerms {
            serial: 3,
            ..CatalogTerms::recalling(VERSION, REASON)
        },
    ));
    hub.refresh_catalog(&held_recalled)
        .expect("the second recall arrives");
    let mut fresh = crate::state::model_library::ModelLibraryManager::new();
    assert!(matches!(
        hub.add_part_to_project(&mut fresh, PACK_ID, VERSION, PART_ID),
        Err(ModelHubError::ReleaseRevoked { .. })
    ));
    assert!(fresh.libraries_sorted().is_empty(), "and nothing was added");
    assert_eq!(hub.installed().len(), 1, "the release is not uninstalled");
    assert!(
        (retained_divider_output(&project) - 0.5).abs() < 1.0e-9,
        "the project that retained it before the recall still solves"
    );

    // A project pinned to it is reported by name, with the reason.
    let pinned = super::recalled_pins(hub.recalls(), &project);
    assert_eq!(pinned.len(), 1);
    assert_eq!(pinned[0].pack_id, PACK_ID);
    assert_eq!(pinned[0].version, VERSION);
    assert_eq!(pinned[0].reason, REASON);
    assert!(super::recalled_pins(hub.recalls(), &fresh).is_empty());
}

/// The pin key is content, so no project can present one it did not earn.
#[test]
fn the_pack_pin_key_moves_with_every_commitment_it_names() {
    let key = hub_signing_key();
    let archive = signed_archive(&key, &["subckt", "resistor"]);
    let tree = TempTree::new("pin-key");
    let mut hub = filesystem_hub(&tree, &key);
    let transport = StubTransport::with_snapshot(catalog_at(&key, &archive, 1))
        .serving(VERSION, archive.clone());
    hub.refresh_catalog(&transport).expect("catalog");
    hub.install(&transport, PACK_ID, VERSION).expect("install");

    let empty = crate::state::model_library::ModelLibraryManager::new();
    let mut pinned = crate::state::model_library::ModelLibraryManager::new();
    let library = hub
        .add_part_to_project(&mut pinned, PACK_ID, VERSION, PART_ID)
        .expect("retained");
    assert_ne!(
        super::pack_pin_key(&empty),
        super::pack_pin_key(&pinned),
        "a project that pinned something is not the project that did not"
    );
    assert_eq!(
        super::pack_pin_key(&pinned),
        super::pack_pin_key(&pinned.clone()),
        "and the key depends on content rather than on this instance"
    );

    // Each field of the pin participates: two projects differing in any one of
    // them are two different commitments, and a latch that could not tell them
    // apart would report one project's recall against the other's pins.
    let mutations: [fn(&mut crate::state::model_library::PackPartPin); 4] = [
        |pin| pin.pack_id.push_str("-other"),
        |pin| pin.pack_version = "9.9.9".to_owned(),
        |pin| pin.archive_sha256 = "cd".repeat(32),
        |pin| pin.part_id.push_str("_ALT"),
    ];
    for mutate in mutations {
        let mut altered = pinned.clone();
        let held = altered
            .get_library_mut(&library)
            .expect("the retained library exists");
        let mut pin = held.pack_pin.clone().expect("it carries a pin");
        mutate(&mut pin);
        held.pack_pin = Some(pin);
        assert_ne!(
            super::pack_pin_key(&pinned),
            super::pack_pin_key(&altered),
            "changing one field of a pin moves the key"
        );
    }
}

/// The presentation schema 2 added survives a real signed round trip.
#[test]
fn a_schema_two_snapshot_round_trips_its_description_and_specifications() {
    let key = hub_signing_key();
    let archive = signed_archive(&key, &["subckt", "resistor"]);
    let tree = TempTree::new("schema-two");
    let mut hub = filesystem_hub(&tree, &key);
    // Projected out of the archive's own manifest rather than restated, so the
    // fixture catalog cannot publish presentation the signed bytes lack.
    let transport = StubTransport::with_snapshot(signed_snapshot_projecting(
        &key,
        &[(VERSION, &archive, &["subckt", "resistor"])],
    ))
    .serving(VERSION, archive);
    hub.refresh_catalog(&transport).expect("catalog");

    let identity = hub.catalog_identity().expect("held");
    assert_eq!(identity.schema, rspice_pack::SNAPSHOT_SCHEMA);
    assert_eq!(identity.serial, 1);
    assert_eq!(identity.expires_at, STANDS_UNTIL);
    assert_eq!(
        identity.expires_at_seconds,
        super::rfc3339_seconds(STANDS_UNTIL)
    );

    let part = hub
        .snapshot()
        .expect("held")
        .packs
        .iter()
        .flat_map(|pack| &pack.releases)
        .flat_map(|release| &release.parts)
        .find(|part| part.id == PART_ID)
        .expect("the projected part")
        .clone();
    assert_eq!(
        part.description.as_deref(),
        Some("Proving-pack resistive divider")
    );
    assert_eq!(part.specs.get("VR").map(String::as_str), Some("50 V"));
    assert_eq!(part.specs.get("ratio").map(String::as_str), Some("1:2"));

    // The installed manifest carries the same two facts, which is what the
    // shelf reads: catalog projection and archive agree by construction,
    // because one was built from the other.
    hub.install(&transport, PACK_ID, VERSION).expect("install");
    let installed = &hub.installed()[0].manifest.parts[0];
    assert_eq!(installed.description, part.description);
    assert_eq!(installed.specs, part.specs);
}

/// Every instant this module reads is one the format's shape rules admit.
#[test]
fn civil_instants_convert_to_the_unix_epoch_seconds_they_name() {
    use super::rfc3339_seconds;

    assert_eq!(rfc3339_seconds("1970-01-01T00:00:00Z"), Some(0));
    assert_eq!(rfc3339_seconds("2026-08-15T09:30:00Z"), Some(1_786_786_200));
    // A leap day, and the instant immediately after it.
    assert_eq!(rfc3339_seconds("2024-02-29T00:00:00Z"), Some(1_709_164_800));
    assert_eq!(rfc3339_seconds("2024-03-01T00:00:00Z"), Some(1_709_251_200));
    assert_eq!(
        rfc3339_seconds("2026-08-15T09:30:00.123Z"),
        Some(1_786_786_200)
    );
}

#[test]
fn a_malformed_instant_is_refused_rather_than_guessed() {
    use super::{CatalogIdentity, rfc3339_seconds};

    for malformed in [
        "",
        "2026-08-15T09:30:00",
        "2026-08-15 09:30:00Z",
        "2026-13-15T09:30:00Z",
        "2026-08-15T25:30:00Z",
        "not-a-date",
    ] {
        assert_eq!(
            rfc3339_seconds(malformed),
            None,
            "{malformed:?} must not parse"
        );
    }
    // An expiry that did not parse is read as "no expiry known", never as
    // expired: a client bricking its own hub over a field it misread would be
    // the failure this reading exists to prevent.
    let identity = CatalogIdentity {
        generation: None,
        digest: "ab".repeat(32),
        schema: rspice_pack::SNAPSHOT_SCHEMA,
        serial: 1,
        generated_at: SIGNED_AT.to_owned(),
        expires_at: "not-an-instant".to_owned(),
        expires_at_seconds: None,
    };
    assert!(!identity.expired_at(u64::MAX));
}
