//! Making a browser session's installed packs outlive the session.
//!
//! A desktop installation keeps its packs in a directory, and the directory is
//! still there next time. A browser session kept them in the tab's memory, so
//! every reload re-downloaded a corpus the reader had already accepted. What
//! this module adds is a *copy*: everything the store accepts is written to
//! durable browser storage as well, and a later session reads it back.
//!
//! # A restored archive is not an installed pack
//!
//! Nothing here trusts what it reads. Bytes recovered from storage are exactly
//! as trustworthy as bytes off the network — which is to say, not at all until
//! [`rspice_pack::Pack::verify`] proves them under the compiled-in anchor — and
//! they go through the same [`accept_archive`] a download does, for the same
//! reasons and with the same refusals. Storage is reachable by anything running
//! on the origin, so this is not a formality: an archive rewritten between two
//! sessions is caught here, discarded, and reported.
//!
//! A signature alone would not be enough, and the restore does not rely on one.
//! Every archive is filed under the digest of its own bytes, and that digest is
//! checked before the container is proved — the storage-layer counterpart of
//! the length and digest a download handoff declares. The attack it closes is
//! the one a signature is blind to by construction: writing pack B's bytes,
//! validly signed by the same publisher, over pack A's key. Both verify; only
//! one is what was stored.
//!
//! # What re-proving is not
//!
//! It is not a re-run of the *catalog* decisions. Three of those apply to a
//! restore and each lands somewhere else on purpose:
//!
//! - **Serial.** The floor is restored before the cached catalog is, so
//!   [`ModelHub::open`] sees a floor that is already at least as high as the
//!   one this origin had accepted, and refuses a cached snapshot below it.
//!   Restoring them the other way round would let a substituted catalog set
//!   the floor it wanted to be measured against.
//! - **Expiry.** Never consulted here. An expired catalog withholds *offers*
//!   and stops nothing local, and packs already on this machine are as local as
//!   it gets. A restore that refused them would brick a workspace for being
//!   offline over a long weekend.
//! - **Recall.** Also not consulted here, and deliberately: a recall does not
//!   uninstall anything on the desktop, so a browser that deleted recalled
//!   packs on restore would be a *different product* on the two hosts. The
//!   recall machinery runs where it already runs — the ledger stops offering
//!   the release, and [`ModelHub::part_pin`] refuses to retain from it — which
//!   it does over restored packs exactly as over installed ones, because by
//!   then they are the same thing. Restoring cannot consult it in any case:
//!   the recall list is projected from the catalog by `ModelHub::open`, which
//!   has not run yet.
//!
//! # What compiles where
//!
//! Only [`PackStorageStanding`] is built on every target. It is the vocabulary
//! a *host* answers in, and the desktop answers `NotApplicable` — a filesystem
//! promises what every desktop application promises by existing, and the
//! workspace says nothing about it. Everything else here is machinery for a
//! store that has to be told to keep things, so it is compiled for the browser
//! and for the tests that prove it, and is absent from a desktop binary rather
//! than present and unreachable in it.
//!
//! [`accept_archive`]: super::accept_archive
//! [`ModelHub::open`]: super::ModelHub::open
//! [`ModelHub::part_pin`]: super::ModelHub::part_pin

#[cfg(any(test, target_arch = "wasm32"))]
use std::collections::BTreeMap;
#[cfg(any(test, target_arch = "wasm32"))]
use std::sync::Mutex;

#[cfg(any(test, target_arch = "wasm32"))]
use rspice_pack::{VerifiedPack, sha256_hex};

#[cfg(any(test, target_arch = "wasm32"))]
use super::store::StagedPack;
#[cfg(any(test, target_arch = "wasm32"))]
use super::{
    InstalledPack, MemoryModelHubStore, ModelHubError, ModelHubStore, TrustAnchor, accept_archive,
    release_key,
};

#[cfg(target_arch = "wasm32")]
mod browser;
#[cfg(target_arch = "wasm32")]
pub(crate) use browser::{BrowserPackMirror, start_browser_pack_restore};

#[cfg(test)]
mod tests;

/// What a reader is told when storage would not open at all.
///
/// The browser's own error is deliberately *not* interpolated into it, and the
/// reason is not brevity. This sentence is read by
/// `ModelsOperationalState::from_failure`, which scans it for the word that
/// decides which rung it lands on — and an arbitrary foreign string spliced
/// into the middle can carry any of those words. The first draft said "could
/// not open storage" and landed on `ReadOnly`, whose consequence tells the
/// reader to open their project for editing: advice about something else
/// entirely, produced by a substring nobody wrote on purpose. The browser's
/// text goes to the log, where nothing decides anything from it.
///
/// It lives here rather than beside the browser mirror that publishes it so
/// the test asserting which rung it reaches — which is in the shell, because
/// that is where the ladder is — reads the same bytes the browser will, rather
/// than a copy of them that can drift.
#[cfg(any(test, target_arch = "wasm32"))]
pub(crate) const UNAVAILABLE_AT_OPEN: &str = "Storage for model packs is unavailable in this browser, so installed packs last only as long \
     as this tab.";

/// And when it opened, took an install, and then refused to keep it.
///
/// A distinct sentence because a distinct thing happened: this session has the
/// pack and goes on having it, and only the copy that would have outlived the
/// tab was lost. Both land on the same rung, whose consequence — nothing on
/// this machine changed — is exactly true of both.
#[cfg(any(test, target_arch = "wasm32"))]
pub(crate) const UNAVAILABLE_AFTER_WRITE: &str = "Durable storage for model packs became unavailable in this browser, so packs installed now \
     last only as long as this tab.";

/// Everything one origin has kept about this machine's Model Hub.
///
/// Untrusted, all of it. This is the shape read *out* of storage, before any
/// of it has been proved, which is why it is bytes and integers rather than a
/// decoded snapshot and a list of installed packs. Turning it into those is
/// [`hydrate`]'s job and involves refusing some of it.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
#[cfg(any(test, target_arch = "wasm32"))]
pub(crate) struct PersistedHubState {
    /// The highest catalog serial this origin recorded accepting.
    pub(crate) serial: u64,
    /// The cached catalog snapshot, exactly as it was published.
    pub(crate) snapshot: Option<Vec<u8>>,
    /// Pack archives, each beside the digest it was filed under.
    ///
    /// The digest is neither decoration nor a convenience. It is the storage
    /// layer's equivalent of the length and digest a download handoff
    /// declares, and [`hydrate`] checks it for exactly the reason
    /// `require_exact_bytes` checks those: a signature proves these bytes are
    /// *a* pack this publisher signed, and says nothing whatever about their
    /// being the pack that was stored here. Without the check, writing one
    /// validly signed archive over another's key would install a release the
    /// reader never asked for — and every signature would verify.
    pub(crate) archives: Vec<StoredArchive>,
}

/// One archive as storage handed it back: the bytes, and the name they were
/// filed under.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg(any(test, target_arch = "wasm32"))]
pub(crate) struct StoredArchive {
    /// The lowercase hexadecimal SHA-256 these bytes were stored under.
    pub(crate) digest: String,
    pub(crate) bytes: Vec<u8>,
}

/// One restored archive that no longer proves, and why.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg(any(test, target_arch = "wasm32"))]
pub(crate) struct RejectedArchive {
    /// The digest the bytes actually hash to, which is the key they are stored
    /// under and therefore the one a caller deletes them by.
    pub(crate) digest: String,
    pub(crate) error: ModelHubError,
}

/// What one hydration concluded.
///
/// Counting rather than listing what succeeded: a restored pack is
/// indistinguishable from an installed one the moment it lands, and the ledger
/// already lists those. What is worth carrying out of here is the part the
/// ledger *cannot* show, because the bytes are gone — the archives that were
/// discarded, and the digests to delete them by.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
#[cfg(any(test, target_arch = "wasm32"))]
pub(crate) struct HydrationReport {
    pub(crate) restored: usize,
    pub(crate) rejected: Vec<RejectedArchive>,
}

#[cfg(any(test, target_arch = "wasm32"))]
impl HydrationReport {
    /// Whether this session recovered nothing and had nothing to recover.
    pub(crate) fn is_empty(&self) -> bool {
        self.restored == 0 && self.rejected.is_empty()
    }

    /// The refusal a workspace states, when there is one to state.
    ///
    /// One sentence for the whole hydration rather than one per archive: a
    /// reader whose storage was rewritten does not have a per-pack decision to
    /// make, and a list of digests is not prose. The wording carries
    /// "corrupted" because that is the rung this belongs on — the bytes are
    /// authentic-looking and do not describe what they claim to — and
    /// `ModelsOperationalState::from_failure` reads that word to reach it.
    pub(crate) fn refusal(&self) -> Option<String> {
        let rejected = self.rejected.len();
        if rejected == 0 {
            return None;
        }
        let subject = if rejected == 1 {
            "1 stored model pack was".to_owned()
        } else {
            format!("{rejected} stored model packs were")
        };
        Some(format!(
            "{subject} discarded as corrupted: the archive no longer proves under this build's \
             signing key, so nothing from it was installed. Reinstall from the Model Hub."
        ))
    }
}

/// Restores what an origin kept, proving every byte of it first.
///
/// The order is the argument, and it is the same argument the module header
/// makes about the serial: the floor is recorded before the catalog is, so
/// nothing a substituted catalog says can lower the bar it is then measured
/// against. Archives come last, because whether they are acceptable does not
/// depend on the catalog at all.
///
/// It never fails as a whole. A store that refuses one archive has still
/// restored the others, and a session that recovered nine packs out of ten is
/// a better outcome than a session that threw all ten away because of one.
#[cfg(any(test, target_arch = "wasm32"))]
pub(crate) fn hydrate(
    anchor: &TrustAnchor,
    store: &dyn ModelHubStore,
    persisted: PersistedHubState,
) -> HydrationReport {
    if persisted.serial > super::NO_CATALOG_SERIAL {
        // Best-effort: a floor that could not be recorded means a rollback
        // this origin had already refused could be offered again, which is
        // worth a log line and is not worth discarding nine proved packs over.
        if let Err(error) = store.record_catalog_serial(persisted.serial) {
            log::warn!("the restored model-hub catalog serial could not be recorded: {error}");
        }
    }
    if let Some(snapshot) = persisted.snapshot.as_deref()
        && let Err(error) = store.write_snapshot(snapshot)
    {
        // Also best-effort, and safe to be: `ModelHub::open` treats a missing
        // cache as "never fetched", which withholds offers rather than
        // inventing them.
        log::warn!("the restored model-hub catalog could not be seeded: {error}");
    }

    let mut report = HydrationReport::default();
    for archive in &persisted.archives {
        // The bytes are bound to the name they were filed under before
        // anything else looks at them, which is the same order the download
        // path takes: `require_exact_bytes` runs against the digest the
        // catalog published *before* `accept_archive` proves the container.
        // Both are asking the one question a signature cannot answer — are
        // these the bytes that were promised, as opposed to some other bytes
        // the same publisher also signed.
        let actual = sha256_hex(&archive.bytes);
        if actual != archive.digest {
            report.rejected.push(RejectedArchive {
                digest: archive.digest.clone(),
                error: ModelHubError::DigestMismatch {
                    expected: archive.digest.clone(),
                    actual,
                },
            });
            continue;
        }
        // `None` is now the honest argument rather than a shortcut. The
        // identity binding a download gets from the catalog, this restore has
        // just had from the digest above — and had it *more* strictly, because
        // a digest pins the exact bytes where a pack identifier pins only a
        // name. Re-asserting the manifest's own identity against itself here
        // would add a comparison that cannot fail.
        match accept_archive(anchor, store, &archive.bytes, None) {
            Ok(_) => report.restored += 1,
            Err(error) => report.rejected.push(RejectedArchive {
                digest: archive.digest.clone(),
                error,
            }),
        }
    }
    report
}

/// Where a store writes the copy that outlives the session.
///
/// Every method is fire-and-forget and none of them returns a result, which is
/// deliberate rather than lazy: the wrapped store is authoritative for this
/// session and has already accepted the bytes by the time any of these is
/// called, so there is no decision left for a failure to change. What a
/// failure does change is what the *next* session will find, and that is
/// reported through the standing this mirror publishes rather than by
/// unwinding an install that already succeeded.
///
/// `Send + Sync` because the store this mirror belongs to is handed to the
/// worker that performs an install. The worker opens its own hub over the same
/// store, which is what makes two hubs agree, and it cannot do that with a
/// mirror bolted to one thread.
#[cfg(any(test, target_arch = "wasm32"))]
pub(crate) trait DurableHubMirror: std::fmt::Debug + Send + Sync {
    fn put_snapshot(&self, bytes: &[u8]);
    fn put_serial(&self, serial: u64);
    /// Keeps one archive under its own digest. Content-addressed, so writing
    /// the same release twice is writing the same key twice.
    fn put_archive(&self, digest: &str, bytes: &[u8]);
    fn delete_archive(&self, digest: &str);
    /// What this mirror is currently able to promise.
    fn standing(&self) -> PackStorageStanding;
}

/// What durable pack storage is doing, in the only terms honest about it.
///
/// Three states rather than a boolean, because a browser offers three
/// different promises and stating the middle one as either of the others is
/// the over-claim this whole module has to avoid. It is not a failure
/// vocabulary — a failure is a sentence that goes to the workspace's own
/// operational-state ladder — it is what the durability *note* is derived
/// from.
///
/// The three that are not `NotApplicable` are constructed only by a store that
/// has somewhere to keep things, which on a shipped desktop binary is no store
/// at all — while the projection that *reads* them compiles on every target,
/// because a desktop test and the raster harness both compose the browser
/// projection and look at it. The enum is therefore whole everywhere and
/// exercised in one place, which the dead-code gate cannot tell apart from an
/// enum nobody uses.
#[cfg_attr(
    not(any(test, target_arch = "wasm32")),
    allow(dead_code, reason = "only a browser store constructs the other three")
)]
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) enum PackStorageStanding {
    /// No durable store on this host, or none asked for yet.
    #[default]
    NotApplicable,
    /// Kept, and the browser has granted persistence: it will not evict this
    /// origin to reclaim space.
    Persistent,
    /// Kept, and the browser has promised nothing. Quota pressure, private
    /// browsing and the reader clearing site data all still remove it.
    BestEffort,
    /// Not kept at all. The sentence is the reason, in words a reader can act
    /// on.
    Unavailable(String),
}

/// A pack store that keeps a copy of everything it accepts.
///
/// The wrapped [`MemoryModelHubStore`] stays authoritative for this session and
/// stays entirely unaware of the mirror, which is what keeps the install
/// pipeline — staging, the rename semantics, the sweep — one implementation
/// rather than two. The mirror is a consequence of accepting bytes, never a
/// participant in deciding whether to.
///
/// Wrapping the store rather than mirroring from the service is not a detail.
/// An install runs on a worker that opens its *own* hub over this same store,
/// so the store is the only place both the session and the worker pass
/// through — and a mirror anywhere else would miss every pack installed in the
/// background, which is all of them.
#[derive(Debug)]
#[cfg(any(test, target_arch = "wasm32"))]
pub(crate) struct MirroredModelHubStore {
    inner: MemoryModelHubStore,
    mirror: Box<dyn DurableHubMirror>,
    /// Archive bytes handed to `stage_pack` and not yet resolved, keyed by the
    /// release they will be published as.
    ///
    /// The mirror is written on *commit*, never on stage, because staging is
    /// the step that is allowed to be abandoned. Keeping the bytes here in the
    /// meantime is what lets commit write them without asking the wrapped
    /// store to hand back something it was never asked to keep.
    staged: Mutex<BTreeMap<String, Vec<u8>>>,
    /// Whether this store has already been restored from durable storage.
    ///
    /// The latch lives here, on the shared store, and not on the service that
    /// asks for the restore. A service is replaced wholesale — a project
    /// opening, a session restoring, a test building a fresh application — and
    /// a flag on one of those comes back cleared, which would re-read storage
    /// and re-prove every archive on a machine that had already done it.
    ///
    /// It is not keyed on anything, and that is the point. Restoring happens
    /// once in the life of a store; there is no version of the input to
    /// compare, so there is nothing a key could be derived from that would not
    /// be invented. What a key would have bought is exactly what an `Arc` to
    /// one allocation already buys.
    restored: std::sync::atomic::AtomicBool,
    /// Stored archives this store has put through the proof.
    ///
    /// See [`Self::archives_proved`]. It is a plain running total and never a
    /// key: nothing decides anything from it, so there is no latch here for a
    /// rewound counter to unlatch.
    proved: std::sync::atomic::AtomicUsize,
}

#[cfg(any(test, target_arch = "wasm32"))]
impl MirroredModelHubStore {
    pub(crate) fn new(mirror: Box<dyn DurableHubMirror>) -> Self {
        Self {
            inner: MemoryModelHubStore::new(),
            mirror,
            staged: Mutex::new(BTreeMap::new()),
            restored: std::sync::atomic::AtomicBool::new(false),
            proved: std::sync::atomic::AtomicUsize::new(0),
        }
    }

    /// Restores what an origin kept, at most once in this store's life.
    ///
    /// `None` means the one restore was already taken. Reading storage twice
    /// would be slow rather than wrong — acceptance is idempotent, and a pack
    /// that proves once proves again — but "slow rather than wrong" over a
    /// corpus of signed archives is every ed25519 verification and every file
    /// digest repeated for a verdict that cannot have changed, and it is
    /// undetectable from the outside precisely *because* the answer is the
    /// same. [`Self::archives_proved`] is what makes it detectable.
    ///
    /// The latch and the counter are claimed and advanced here, together, so
    /// there is no arrangement of callers that gets one without the other.
    pub(crate) fn restore_from(
        &self,
        anchor: &TrustAnchor,
        persisted: PersistedHubState,
    ) -> Option<HydrationReport> {
        if self
            .restored
            .swap(true, std::sync::atomic::Ordering::SeqCst)
        {
            return None;
        }
        self.proved.fetch_add(
            persisted.archives.len(),
            std::sync::atomic::Ordering::Relaxed,
        );
        // `&self.inner`, not `self`, and this is the whole of how a restore
        // avoids copying storage back into storage. Every byte it writes came
        // *out* of the mirror moments earlier, so mirroring it would copy the
        // entire corpus back on every launch — tens of megabytes per session
        // on a reader with a dozen packs, and exactly the shape of write that
        // meets a quota refusal.
        //
        // The obvious alternative — a flag on this store saying "suppress the
        // mirror for now" — is wrong, and wrong in a way that loses a reader's
        // pack. This store is `Arc`-shared with the worker that performs an
        // install, which opens its own hub over it. A `commit_pack` landing
        // inside that window would see the flag, skip the copy, and report
        // success: the pack installs, the session shows it, and it is gone
        // next time with nothing having gone wrong anywhere a reader can see.
        // Addressing the wrapped store instead has no window to land in.
        let report = hydrate(anchor, &self.inner, persisted);
        // Through the mirror, and deliberately: bytes that did not prove are
        // not evidence of anything, keeping them costs a reader quota they
        // cannot see, and this is the one storage write a restore *should*
        // make.
        for rejected in &report.rejected {
            self.mirror.delete_archive(&rejected.digest);
        }
        Some(report)
    }

    /// How many stored archives this store has put through the proof.
    ///
    /// Per store rather than per process, so a test can assert the exact
    /// number without being at the mercy of whatever else is hydrating beside
    /// it.
    pub(crate) fn archives_proved(&self) -> usize {
        self.proved.load(std::sync::atomic::Ordering::Relaxed)
    }

    pub(crate) fn standing(&self) -> PackStorageStanding {
        self.mirror.standing()
    }

    fn staged_bytes(&self) -> std::sync::MutexGuard<'_, BTreeMap<String, Vec<u8>>> {
        self.staged
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
    }

    /// The digest one installed release was proved from, if it is here.
    fn installed_digest(&self, pack_id: &str, version: &str) -> Option<String> {
        self.inner
            .installed_packs()
            .ok()?
            .into_iter()
            .find_map(|pack| {
                (pack.pack_id() == pack_id && pack.version() == version)
                    .then_some(pack.archive_sha256)
            })
    }
}

#[cfg(any(test, target_arch = "wasm32"))]
impl ModelHubStore for MirroredModelHubStore {
    fn read_snapshot(&self) -> Result<Option<Vec<u8>>, ModelHubError> {
        self.inner.read_snapshot()
    }

    fn write_snapshot(&self, bytes: &[u8]) -> Result<(), ModelHubError> {
        self.inner.write_snapshot(bytes)?;
        self.mirror.put_snapshot(bytes);
        Ok(())
    }

    fn read_catalog_serial(&self) -> Result<u64, ModelHubError> {
        self.inner.read_catalog_serial()
    }

    fn record_catalog_serial(&self, serial: u64) -> Result<(), ModelHubError> {
        self.inner.record_catalog_serial(serial)?;
        // The *raised* value, read back, not the one that was offered. The
        // wrapped store only ever moves the floor upward, and mirroring the
        // argument instead would let a stale call write a lower floor than the
        // one this session is actually holding — which is the one thing the
        // floor may never do.
        self.mirror.put_serial(self.inner.read_catalog_serial()?);
        Ok(())
    }

    fn stage_pack(
        &self,
        verified: &VerifiedPack,
        archive: &[u8],
    ) -> Result<StagedPack, ModelHubError> {
        let staged = self.inner.stage_pack(verified, archive)?;
        self.staged_bytes().insert(
            release_key(staged.pack_id(), staged.version()),
            archive.to_vec(),
        );
        Ok(staged)
    }

    fn commit_pack(&self, staged: StagedPack) -> Result<InstalledPack, ModelHubError> {
        let key = release_key(staged.pack_id(), staged.version());
        let bytes = self.staged_bytes().remove(&key);
        let installed = self.inner.commit_pack(staged)?;
        // Only after the wrapped store published it. A mirror written first
        // would survive a commit that failed, and the next session would
        // restore a pack this one never installed.
        if let Some(bytes) = bytes {
            self.mirror.put_archive(&installed.archive_sha256, &bytes);
        }
        Ok(installed)
    }

    fn discard_staged(&self, staged: StagedPack) {
        self.staged_bytes()
            .remove(&release_key(staged.pack_id(), staged.version()));
        self.inner.discard_staged(staged);
    }

    fn sweep_staging(&self) -> Result<usize, ModelHubError> {
        self.staged_bytes().clear();
        self.inner.sweep_staging()
    }

    fn installed_packs(&self) -> Result<Vec<InstalledPack>, ModelHubError> {
        self.inner.installed_packs()
    }

    fn remove_pack(&self, pack_id: &str, version: &str) -> Result<bool, ModelHubError> {
        // Read the digest before the removal, because afterwards there is
        // nothing left to read it from and the durable copy would be orphaned
        // — present next session, restored, and belonging to a release the
        // reader uninstalled.
        let digest = self.installed_digest(pack_id, version);
        let removed = self.inner.remove_pack(pack_id, version)?;
        if removed && let Some(digest) = digest {
            self.mirror.delete_archive(&digest);
        }
        Ok(removed)
    }

    fn verify_installed(
        &self,
        pack_id: &str,
        version: &str,
        anchor: &TrustAnchor,
    ) -> Result<VerifiedPack, ModelHubError> {
        self.inner.verify_installed(pack_id, version, anchor)
    }
}
