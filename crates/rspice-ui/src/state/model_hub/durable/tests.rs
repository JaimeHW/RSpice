//! What a restored browser session may and may not believe.
//!
//! Every fixture is a real signed artifact built by the same helpers the rest
//! of the Model Hub gate uses, so a restore that passes here has been through
//! the actual ed25519 signature and the actual archive container. The mirror
//! is a fake, and only the mirror: it records what a durable store would have
//! been asked to keep, which is the one thing a desktop test cannot observe
//! through IndexedDB.

use std::sync::Mutex;

use super::super::tests::{
    NEXT_VERSION, PACK_ID, VERSION, anchor_for, hub_signing_key, signed_archive,
    signed_archive_at, signed_snapshot,
};
use super::super::{ModelHub, ModelHubError, ModelHubStore, NO_CATALOG_SERIAL};
use super::{
    DurableHubMirror, HydrationReport, MirroredModelHubStore, PackStorageStanding,
    PersistedHubState, hydrate,
};

/// The capabilities the fixture pack declares, which this build offers.
const RUNNABLE: &[&str] = &["subckt", "resistor"];

/// One thing a durable store was asked to do.
#[derive(Debug, Clone, PartialEq, Eq)]
enum MirrorOp {
    Snapshot(Vec<u8>),
    Serial(u64),
    PutArchive(String),
    DeleteArchive(String),
}

/// A mirror that records instead of storing.
///
/// It answers `standing` with whatever it was built with, so a test about what
/// the workspace says when storage is denied does not need a browser to deny
/// it.
#[derive(Debug)]
struct RecordingMirror {
    ops: Mutex<Vec<MirrorOp>>,
    standing: PackStorageStanding,
}

impl RecordingMirror {
    fn new(standing: PackStorageStanding) -> std::sync::Arc<Self> {
        std::sync::Arc::new(Self {
            ops: Mutex::new(Vec::new()),
            standing,
        })
    }

    fn ops(&self) -> Vec<MirrorOp> {
        self.ops.lock().expect("mirror log").clone()
    }

    fn record(&self, op: MirrorOp) {
        self.ops.lock().expect("mirror log").push(op);
    }
}

/// Shared so a test can hold the log while the store holds the mirror.
impl DurableHubMirror for std::sync::Arc<RecordingMirror> {
    fn put_snapshot(&self, bytes: &[u8]) {
        self.record(MirrorOp::Snapshot(bytes.to_vec()));
    }

    fn put_serial(&self, serial: u64) {
        self.record(MirrorOp::Serial(serial));
    }

    fn put_archive(&self, digest: &str, _bytes: &[u8]) {
        self.record(MirrorOp::PutArchive(digest.to_owned()));
    }

    fn delete_archive(&self, digest: &str) {
        self.record(MirrorOp::DeleteArchive(digest.to_owned()));
    }

    fn standing(&self) -> PackStorageStanding {
        self.standing.clone()
    }
}

fn mirrored_store(
    standing: PackStorageStanding,
) -> (MirroredModelHubStore, std::sync::Arc<RecordingMirror>) {
    let mirror = RecordingMirror::new(standing);
    (
        MirroredModelHubStore::new(Box::new(std::sync::Arc::clone(&mirror))),
        mirror,
    )
}

/// A restored archive is proved before it is anything else.
///
/// The pack is real and signed by the anchor's own key, so it lands; the whole
/// point of the test is that it landed *through* the proof rather than around
/// it, which the tampered case below establishes by changing one byte.
#[test]
fn a_restored_archive_is_installed_only_after_it_proves_under_the_anchor() {
    let key = hub_signing_key();
    let archive = signed_archive(&key, RUNNABLE);
    let (store, _mirror) = mirrored_store(PackStorageStanding::Persistent);

    let report = hydrate(
        &anchor_for(&key),
        &store,
        PersistedHubState {
            serial: NO_CATALOG_SERIAL,
            snapshot: None,
            archives: vec![archive.clone()],
        },
    );

    assert_eq!(report.restored, 1);
    assert!(report.rejected.is_empty());
    assert!(report.refusal().is_none());
    let installed = store.installed_packs().expect("the restored store lists");
    assert_eq!(installed.len(), 1);
    assert_eq!(installed[0].pack_id(), PACK_ID);
    assert_eq!(installed[0].version(), VERSION);
    assert_eq!(installed[0].archive_sha256, rspice_pack::sha256_hex(&archive));
}

/// Bytes that no longer prove are discarded, named, and not installed.
///
/// This is the whole reason a restore re-proves rather than trusting its own
/// storage. One flipped byte in the middle of the archive is what anything
/// with write access to the origin's storage can produce, and the session that
/// reads it must end up with nothing rather than with a pack.
#[test]
fn a_rewritten_archive_is_discarded_rather_than_restored() {
    let key = hub_signing_key();
    let mut archive = signed_archive(&key, RUNNABLE);
    let tampered_digest = {
        let middle = archive.len() / 2;
        archive[middle] ^= 0x40;
        rspice_pack::sha256_hex(&archive)
    };
    let (store, _mirror) = mirrored_store(PackStorageStanding::BestEffort);

    let report = hydrate(
        &anchor_for(&key),
        &store,
        PersistedHubState {
            serial: NO_CATALOG_SERIAL,
            snapshot: None,
            archives: vec![archive],
        },
    );

    assert_eq!(report.restored, 0);
    assert_eq!(report.rejected.len(), 1);
    assert_eq!(report.rejected[0].digest, tampered_digest);
    assert!(
        store
            .installed_packs()
            .expect("the store lists")
            .is_empty(),
        "nothing that failed the proof may reach the installed set"
    );
    // The refusal lands on the Corrupted rung rather than on the generic
    // execution error every unclassified sentence gets.
    let refusal = report.refusal().expect("a discarded archive is stated");
    assert_eq!(
        crate::workbench::state::ModelsOperationalState::from_failure(&refusal),
        crate::workbench::state::ModelsOperationalState::Corrupted
    );
}

/// An archive signed by a key this build does not carry is refused.
///
/// Distinct from tampering: these bytes are internally perfect and were signed
/// by *somebody*. The anchor is the whole of what makes them unacceptable.
#[test]
fn an_archive_signed_by_a_foreign_key_does_not_survive_a_restore() {
    let key = hub_signing_key();
    let foreign = rspice_pack::signing_key(&[0x77_u8; 32]);
    let archive = signed_archive(&foreign, RUNNABLE);
    let (store, _mirror) = mirrored_store(PackStorageStanding::Persistent);

    let report = hydrate(
        &anchor_for(&key),
        &store,
        PersistedHubState {
            serial: NO_CATALOG_SERIAL,
            snapshot: None,
            archives: vec![archive],
        },
    );

    assert_eq!(report.restored, 0);
    assert_eq!(report.rejected.len(), 1);
    assert!(matches!(
        report.rejected[0].error,
        ModelHubError::Format(_)
    ));
    assert!(store.installed_packs().expect("the store lists").is_empty());
}

/// One bad archive does not cost a reader the good ones.
#[test]
fn a_restore_keeps_every_archive_that_proves_and_only_drops_the_ones_that_do_not() {
    let key = hub_signing_key();
    let good = signed_archive_at(&key, RUNNABLE, VERSION);
    let also_good = signed_archive_at(&key, RUNNABLE, NEXT_VERSION);
    let mut bad = signed_archive_at(&key, RUNNABLE, VERSION);
    bad[16] ^= 0x01;
    let (store, _mirror) = mirrored_store(PackStorageStanding::Persistent);

    let report = hydrate(
        &anchor_for(&key),
        &store,
        PersistedHubState {
            serial: NO_CATALOG_SERIAL,
            snapshot: None,
            archives: vec![bad, good, also_good],
        },
    );

    assert_eq!(report.restored, 2);
    assert_eq!(report.rejected.len(), 1);
    let mut versions = store
        .installed_packs()
        .expect("the store lists")
        .into_iter()
        .map(|pack| pack.version().to_owned())
        .collect::<Vec<_>>();
    versions.sort();
    assert_eq!(versions, vec![VERSION.to_owned(), NEXT_VERSION.to_owned()]);
}

/// A pack this build cannot run is refused on restore, as it is on install.
///
/// The check lives in the shared acceptance owner, so this is really a test
/// that the restore path goes through it — an engine that dropped a device
/// family between two releases must not silently keep serving packs that need
/// it.
#[test]
fn a_restored_pack_this_build_cannot_run_is_refused_the_way_an_install_would_be() {
    let key = hub_signing_key();
    let archive = signed_archive(&key, &["subckt", "a-capability-this-build-does-not-offer"]);
    let (store, _mirror) = mirrored_store(PackStorageStanding::Persistent);

    let report = hydrate(
        &anchor_for(&key),
        &store,
        PersistedHubState {
            serial: NO_CATALOG_SERIAL,
            snapshot: None,
            archives: vec![archive],
        },
    );

    assert_eq!(report.restored, 0);
    assert!(matches!(
        report.rejected[0].error,
        ModelHubError::Incompatible { .. }
    ));
}

/// The restored floor is in place before the restored catalog is measured.
///
/// A session that had accepted serial 9 and whose stored catalog was swapped
/// for an authentic serial-4 one must not come back holding serial 4. Both
/// halves are restored by the same call, so the order inside it is the only
/// thing standing between the reader and a replayed catalog — which would
/// carry a recall list from before every recall published since.
#[test]
fn a_restored_catalog_below_the_restored_floor_is_refused_rather_than_held() {
    let key = hub_signing_key();
    let archive = signed_archive(&key, RUNNABLE);
    let older = signed_snapshot(&key, &archive, RUNNABLE, VERSION);
    let (store, _mirror) = mirrored_store(PackStorageStanding::Persistent);

    let report = hydrate(
        &anchor_for(&key),
        &store,
        PersistedHubState {
            // The fixture snapshot is published at serial 1; this origin has
            // already accepted 9.
            serial: 9,
            snapshot: Some(older),
            archives: vec![archive],
        },
    );
    assert_eq!(report.restored, 1);

    let hub = ModelHub::open(anchor_for(&key), Box::new(store), None)
        .expect("a hub opens over the restored store");
    assert_eq!(hub.last_seen_serial(), 9, "the restored floor stands");
    assert!(
        hub.catalog_identity().is_none(),
        "a stored catalog below the stored floor is a replay"
    );
    assert!(
        hub.catalog_cache_discarded(),
        "and the reader is told it was rejected, not that none was ever fetched"
    );
    assert_eq!(
        hub.installed().len(),
        1,
        "a refused catalog costs the reader no packs: they proved on their own bytes"
    );
}

/// An expired catalog does not cost a restored session its packs.
///
/// The catalog is restored, and the packs with it, and the hub then withholds
/// offers exactly as it does on the desktop. A restore that refused packs for
/// a lapsed catalog would brick a workspace for being offline.
#[test]
fn an_expired_stored_catalog_withholds_offers_and_keeps_every_restored_pack() {
    use super::super::tests::{CatalogTerms, EXPIRED_AT, signed_snapshot_on};

    let key = hub_signing_key();
    let archive = signed_archive(&key, RUNNABLE);
    let lapsed = signed_snapshot_on(
        &key,
        &[(VERSION, &archive, RUNNABLE)],
        &CatalogTerms {
            expires_at: EXPIRED_AT.to_owned(),
            ..CatalogTerms::default()
        },
    );
    let (store, _mirror) = mirrored_store(PackStorageStanding::Persistent);

    let report = hydrate(
        &anchor_for(&key),
        &store,
        PersistedHubState {
            serial: 1,
            snapshot: Some(lapsed),
            archives: vec![archive],
        },
    );
    assert_eq!(report.restored, 1, "an expiry is not an opinion about bytes");

    let hub = ModelHub::open(anchor_for(&key), Box::new(store), None)
        .expect("a hub opens over the restored store");
    assert!(hub.catalog_expired().is_some());
    assert!(
        hub.offered_snapshot().is_none(),
        "an expired catalog offers nothing"
    );
    assert_eq!(
        hub.installed().len(),
        1,
        "and stops nothing already on this machine"
    );
}

/// A recalled release keeps its restored bytes and loses its offer.
///
/// The desktop does not uninstall a recalled pack, so neither may a restore.
/// What the recall costs is retention — the one step every route into a
/// project passes through — and that refusal is reached over a restored pack
/// exactly as over an installed one, because by then they are the same thing.
#[test]
fn a_recalled_release_is_restored_and_then_refused_where_a_recall_refuses() {
    use super::super::tests::{CatalogTerms, signed_snapshot_on};

    let key = hub_signing_key();
    let archive = signed_archive(&key, RUNNABLE);
    let recalling = signed_snapshot_on(
        &key,
        &[(VERSION, &archive, RUNNABLE)],
        &CatalogTerms {
            revocations: vec![rspice_pack::Revocation {
                pack_id: PACK_ID.to_owned(),
                version: VERSION.to_owned(),
                reason: "a bin boundary was wrong".to_owned(),
            }],
            ..CatalogTerms::default()
        },
    );
    let (store, _mirror) = mirrored_store(PackStorageStanding::Persistent);

    let report = hydrate(
        &anchor_for(&key),
        &store,
        PersistedHubState {
            serial: 1,
            snapshot: Some(recalling),
            archives: vec![archive],
        },
    );
    assert_eq!(
        report.restored, 1,
        "a recall is not an erasure, on either host"
    );

    let hub = ModelHub::open(anchor_for(&key), Box::new(store), None)
        .expect("a hub opens over the restored store");
    assert_eq!(hub.installed().len(), 1);
    assert_eq!(
        hub.recalls().reason(PACK_ID, VERSION),
        Some("a bin boundary was wrong")
    );
    // The recall rung, reached over a pack that arrived by restore.
    let refusal = hub
        .part_pin(PACK_ID, VERSION, super::super::tests::PART_ID)
        .expect_err("retention from a recalled release is refused");
    assert!(matches!(refusal, ModelHubError::ReleaseRevoked { .. }));
    assert_eq!(
        crate::workbench::state::ModelsOperationalState::from_failure(&refusal.to_string()),
        crate::workbench::state::ModelsOperationalState::Recalled
    );
}

/// Committing a pack writes the durable copy; staging alone does not.
///
/// Staging is the step that is allowed to be abandoned, so a mirror written
/// there would leave the next session restoring a pack this one never
/// installed.
#[test]
fn the_durable_copy_is_written_on_commit_and_never_on_a_stage_that_is_discarded() {
    let key = hub_signing_key();
    let anchor = anchor_for(&key);
    let archive = signed_archive(&key, RUNNABLE);
    let digest = rspice_pack::sha256_hex(&archive);
    let (store, mirror) = mirrored_store(PackStorageStanding::Persistent);

    let verified = rspice_pack::Pack::verify(&archive, anchor.key(), anchor.limits())
        .expect("the fixture proves");
    let staged = store.stage_pack(&verified, &archive).expect("staged");
    assert!(
        mirror.ops().is_empty(),
        "staging claims no name and writes no durable copy"
    );
    store.discard_staged(staged);
    assert!(mirror.ops().is_empty());

    let staged = store.stage_pack(&verified, &archive).expect("staged again");
    store.commit_pack(staged).expect("committed");
    assert_eq!(mirror.ops(), vec![MirrorOp::PutArchive(digest.clone())]);

    // And uninstalling drops the durable copy, so the next session does not
    // restore a release the reader removed.
    assert!(store.remove_pack(PACK_ID, VERSION).expect("removed"));
    assert_eq!(
        mirror.ops(),
        vec![
            MirrorOp::PutArchive(digest.clone()),
            MirrorOp::DeleteArchive(digest)
        ]
    );
}

/// The mirrored floor is the raised one, never the one that was offered.
///
/// The wrapped store only moves the floor upward. Mirroring the argument
/// rather than the result would let a stale call persist a floor lower than
/// the one this session holds — the single thing a serial floor may never do.
#[test]
fn the_durable_floor_only_ever_rises() {
    let (store, mirror) = mirrored_store(PackStorageStanding::Persistent);
    store.record_catalog_serial(9).expect("floor recorded");
    store.record_catalog_serial(4).expect("a lower floor is a no-op");
    assert_eq!(
        mirror.ops(),
        vec![MirrorOp::Serial(9), MirrorOp::Serial(9)],
        "the durable copy never learns the lower number"
    );
    assert_eq!(store.read_catalog_serial().expect("floor"), 9);
}

/// Restoring nothing is a state, not a failure.
#[test]
fn an_origin_that_has_kept_nothing_restores_nothing_and_says_nothing() {
    let key = hub_signing_key();
    let (store, _mirror) = mirrored_store(PackStorageStanding::NotApplicable);
    let report = hydrate(&anchor_for(&key), &store, PersistedHubState::default());
    assert!(report.is_empty());
    assert!(report.refusal().is_none());
    assert_eq!(report, HydrationReport::default());
}

/// Storage that will not open is a sentence a reader can act on, and it
/// reaches the Offline rung rather than the generic one.
#[test]
fn storage_that_is_unavailable_states_why_and_promises_no_durability() {
    let denied = PackStorageStanding::Unavailable(
        "this browser is unavailable for storage in a private window".to_owned(),
    );
    assert!(!denied.keeps_packs());
    assert!(PackStorageStanding::Persistent.keeps_packs());
    assert!(PackStorageStanding::BestEffort.keeps_packs());
    assert!(!PackStorageStanding::NotApplicable.keeps_packs());
    let PackStorageStanding::Unavailable(reason) = &denied else {
        panic!("the denied standing carries its reason")
    };
    assert_eq!(
        crate::workbench::state::ModelsOperationalState::from_failure(reason),
        crate::workbench::state::ModelsOperationalState::Offline
    );
}
