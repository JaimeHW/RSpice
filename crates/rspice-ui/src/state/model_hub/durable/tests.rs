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
    NEXT_VERSION, PACK_ID, VERSION, anchor_for, hub_signing_key, signed_archive, signed_archive_at,
    signed_snapshot,
};
use super::super::{ModelHub, ModelHubError, ModelHubStore, NO_CATALOG_SERIAL};
use super::{
    DurableHubMirror, HydrationReport, MirroredModelHubStore, PackStorageStanding,
    PersistedHubState, StoredArchive, hydrate,
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

/// An archive filed under the digest of its own bytes, as the mirror files it.
fn stored(archive: &[u8]) -> StoredArchive {
    StoredArchive {
        digest: rspice_pack::sha256_hex(archive),
        bytes: archive.to_vec(),
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
            archives: vec![stored(&archive)],
        },
    );

    assert_eq!(report.restored, 1);
    assert!(report.rejected.is_empty());
    assert!(report.refusal().is_none());
    let installed = store.installed_packs().expect("the restored store lists");
    assert_eq!(installed.len(), 1);
    assert_eq!(installed[0].pack_id(), PACK_ID);
    assert_eq!(installed[0].version(), VERSION);
    assert_eq!(
        installed[0].archive_sha256,
        rspice_pack::sha256_hex(&archive)
    );
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
            archives: vec![stored(&archive)],
        },
    );

    assert_eq!(report.restored, 0);
    assert_eq!(report.rejected.len(), 1);
    assert_eq!(report.rejected[0].digest, tampered_digest);
    assert!(
        store.installed_packs().expect("the store lists").is_empty(),
        "nothing that failed the proof may reach the installed set"
    );
    // The refusal carries the word the workspace's own attention ladder reads
    // to reach the Corrupted rung. That the ladder does reach it is asserted
    // where the ladder lives — this layer may not name the shell — and the
    // contract this layer owns is that the sentence says it.
    let refusal = report.refusal().expect("a discarded archive is stated");
    assert!(
        refusal.to_ascii_lowercase().contains("corrupted"),
        "the refusal has to carry the word it is classified by: {refusal}"
    );
    assert!(refusal.contains("Reinstall from the Model Hub"));
}

/// A validly signed archive written over another's key is refused.
///
/// The attack a signature is blind to by construction. Both releases here are
/// real, both are signed by the anchor's own key, and both would pass
/// `Pack::verify` without complaint — so a restore that trusted the signature
/// alone would install 1.1.0 while the reader, their project pins and the
/// ledger all believed 1.0.0 was what this origin had kept. What catches it is
/// the digest the bytes were *filed* under, checked before the container is
/// proved, exactly as a download checks the digest its catalog published before
/// proving the container it fetched.
#[test]
fn one_signed_archive_written_over_another_key_is_refused_before_it_proves() {
    let key = hub_signing_key();
    let stored_release = signed_archive_at(&key, RUNNABLE, VERSION);
    let substituted = signed_archive_at(&key, RUNNABLE, NEXT_VERSION);
    let anchor = anchor_for(&key);
    assert!(
        rspice_pack::Pack::verify(&substituted, anchor.key(), anchor.limits()).is_ok(),
        "the substituted archive is genuinely valid; the signature is not what refuses it"
    );

    let (store, _mirror) = mirrored_store(PackStorageStanding::Persistent);
    let report = hydrate(
        &anchor,
        &store,
        PersistedHubState {
            serial: NO_CATALOG_SERIAL,
            snapshot: None,
            // 1.1.0's bytes, filed under 1.0.0's name.
            archives: vec![StoredArchive {
                digest: rspice_pack::sha256_hex(&stored_release),
                bytes: substituted,
            }],
        },
    );

    assert_eq!(report.restored, 0);
    assert_eq!(report.rejected.len(), 1);
    assert!(
        matches!(
            report.rejected[0].error,
            ModelHubError::DigestMismatch { .. }
        ),
        "refused for being the wrong bytes, not for being unsigned: {:?}",
        report.rejected[0].error
    );
    assert!(
        store.installed_packs().expect("the store lists").is_empty(),
        "nothing may be installed from a record that does not hold what it claims"
    );
    // And the digest reported is the *key*, so the caller drops the record
    // that was tampered with rather than one named by the bytes that replaced
    // it — which would have left the substituted archive in place.
    assert_eq!(
        report.rejected[0].digest,
        rspice_pack::sha256_hex(&stored_release)
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
            archives: vec![stored(&archive)],
        },
    );

    assert_eq!(report.restored, 0);
    assert_eq!(report.rejected.len(), 1);
    assert!(matches!(report.rejected[0].error, ModelHubError::Format(_)));
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
            archives: vec![stored(&bad), stored(&good), stored(&also_good)],
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
            archives: vec![stored(&archive)],
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
            archives: vec![stored(&archive)],
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
            archives: vec![stored(&archive)],
        },
    );
    assert_eq!(
        report.restored, 1,
        "an expiry is not an opinion about bytes"
    );

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
            archives: vec![stored(&archive)],
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
    assert!(
        refusal.to_string().contains("recalled by its publisher"),
        "the refusal carries the phrase the recall rung is reached by: {refusal}"
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
    store
        .record_catalog_serial(4)
        .expect("a lower floor is a no-op");
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

/// The restore latch belongs to the store, and one store restores once.
///
/// The hazard it is placed against is the one that has bitten this codebase
/// repeatedly: the *service* holding a hub is replaced wholesale — a project
/// opening, a session restoring, a history entry being applied — and a flag
/// living on one of those comes back cleared. Two services over one shared
/// store therefore stand in here for the same session before and after such a
/// replacement, and the second one must find the claim already taken.
#[test]
fn one_store_grants_exactly_one_restore_however_many_callers_ask() {
    let key = hub_signing_key();
    let anchor = anchor_for(&key);
    let (store, _mirror) = mirrored_store(PackStorageStanding::Persistent);
    let store = std::sync::Arc::new(store);
    let persisted = PersistedHubState {
        serial: NO_CATALOG_SERIAL,
        snapshot: None,
        archives: vec![
            stored(&signed_archive_at(&key, RUNNABLE, VERSION)),
            stored(&signed_archive_at(&key, RUNNABLE, NEXT_VERSION)),
        ],
    };

    let first = std::sync::Arc::clone(&store);
    let second = std::sync::Arc::clone(&store);
    let report = first
        .restore_from(&anchor, persisted.clone())
        .expect("the first caller restores");
    assert_eq!(report.restored, 2);
    assert_eq!(store.archives_proved(), 2);

    assert!(
        second.restore_from(&anchor, persisted.clone()).is_none(),
        "a service replaced wholesale must not re-read storage over a store that has \
         already been restored"
    );
    assert!(store.restore_from(&anchor, persisted.clone()).is_none());
    // The counter is the part a reader could never observe: a second pass
    // reaches the same verdict, so nothing on screen would reveal that every
    // signature in the corpus had been checked twice.
    assert_eq!(
        store.archives_proved(),
        2,
        "a refused restore re-proved the corpus; the latch is not holding"
    );

    // And a genuinely different store — a second tab, a fresh session — is
    // entitled to its own, because the latch is about one allocation and not
    // about a value anything could present.
    let (other, _other_mirror) = mirrored_store(PackStorageStanding::Persistent);
    assert!(other.restore_from(&anchor, persisted).is_some());
    assert_eq!(other.archives_proved(), 2);
}

/// A restore copies nothing back to storage except the deletions it decided.
///
/// Every byte a restore writes into the store came *out* of storage moments
/// earlier, so mirroring those writes would copy the whole corpus back on every
/// boot — every archive, every snapshot, every session, to answer a question
/// storage had already answered. On a reader with a dozen packs that is tens of
/// megabytes of pointless writes per launch, and it is precisely the shape of
/// write that meets a quota refusal.
///
/// The one write a restore *should* make is the other half of the assertion:
/// an archive that failed the proof is deleted, because bytes that do not prove
/// are not evidence of anything and keeping them costs quota the reader cannot
/// see.
#[test]
fn a_restore_writes_nothing_back_but_the_records_it_discarded() {
    let key = hub_signing_key();
    let anchor = anchor_for(&key);
    let good = signed_archive_at(&key, RUNNABLE, VERSION);
    let mut bad = signed_archive_at(&key, RUNNABLE, NEXT_VERSION);
    bad[24] ^= 0x08;
    let bad_key = rspice_pack::sha256_hex(&bad);
    let (store, mirror) = mirrored_store(PackStorageStanding::Persistent);

    let report = store
        .restore_from(
            &anchor,
            PersistedHubState {
                serial: 7,
                snapshot: Some(signed_snapshot(&key, &good, RUNNABLE, VERSION)),
                archives: vec![stored(&good), stored(&bad)],
            },
        )
        .expect("the first restore is granted");
    assert_eq!(report.restored, 1);
    assert_eq!(report.rejected.len(), 1);

    assert_eq!(
        mirror.ops(),
        vec![MirrorOp::DeleteArchive(bad_key)],
        "a restore re-wrote what it had just read; every boot would copy the corpus back"
    );

    // And the suppression is a window, not a mode: an install *after* the
    // restore still keeps its copy, which is the whole point of the mirror.
    let fresh = signed_archive_at(&key, RUNNABLE, NEXT_VERSION);
    let verified = rspice_pack::Pack::verify(&fresh, anchor.key(), anchor.limits())
        .expect("the fixture proves");
    let staged = store.stage_pack(&verified, &fresh).expect("staged");
    store.commit_pack(staged).expect("committed");
    assert!(
        mirror
            .ops()
            .contains(&MirrorOp::PutArchive(rspice_pack::sha256_hex(&fresh))),
        "a pack installed after the restore must still be kept"
    );
}

/// Storage that will not open carries its own reason, and the default promises
/// nothing.
///
/// Which rung that reason lands on is asserted where the ladder lives; what
/// this layer owns is that there is a reason at all, and that a standing nobody
/// has set yet is the modest one rather than a confident one.
#[test]
fn storage_that_is_unavailable_states_why_and_promises_no_durability() {
    let denied = PackStorageStanding::Unavailable(
        "this browser is unavailable for storage in a private window".to_owned(),
    );
    assert_eq!(
        PackStorageStanding::default(),
        PackStorageStanding::NotApplicable
    );
    let PackStorageStanding::Unavailable(reason) = &denied else {
        panic!("the denied standing carries its reason")
    };
    assert!(!reason.is_empty());
}
