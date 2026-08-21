//! The Model Hub client runtime: catalog, install, and the unified shelf.
//!
//! RSpice's model library is distributed as ed25519-signed `.rspicepack`
//! archives listed by an ed25519-signed catalog snapshot. This module is the
//! whole client side of that: it fetches, proves, caches, installs, and
//! indexes — and it does the proving itself, with the same crate the service
//! signs with, against one compiled-in key.
//!
//! # Nothing is believed because it arrived
//!
//! The service is not an authority here. A catalog handoff is only a claim
//! about which bytes to fetch; a downloaded snapshot means nothing until its
//! signature verifies; a release's `archive_sha256` is taken from the *signed
//! snapshot*, never from the download handoff, so a compromised handoff can
//! misdirect a download but cannot change which bytes are acceptable. Only
//! after `Pack::verify` proves an archive end to end does anything reach disk
//! under a pack's name.
//!
//! # A signature is not the whole of trust
//!
//! An authentic catalog can still be the *wrong* catalog. Three facts the
//! snapshot signs answer that, and each has a refusal here:
//!
//! - **Serial.** The store keeps the highest serial it ever accepted, and a
//!   snapshot below it is refused whatever it is signed with — so replaying an
//!   old catalog cannot undo a recall or hide a release.
//! - **Expiry.** A catalog states the instant after which it must not be
//!   believed. Past it, this client stops *offering* — browsing, installing,
//!   updating — and stops nothing else. Installed packs, retained project
//!   bytes and every local workflow are unaffected, because a machine that
//!   has been offline for a month is not a machine whose work should stop.
//! - **Revocation.** A recalled release is dropped from what is offered and
//!   refused for install, update, adoption and retention, with the publisher's
//!   own reason. Bytes a project already retained are never deleted and never
//!   blocked from simulating: a recall is a recall, not an erasure, and a
//!   design that solved yesterday still solves.
//!
//! # Failure has one shape
//!
//! Every refusal is a typed [`ModelHubError`] and leaves the pack root exactly
//! as it was. Installation is a staged expansion followed by a rename, so a
//! failure — including a killed process — can leave a `.staging-*` directory
//! and nothing else. [`ModelHub::open`] sweeps those, which makes recovery a
//! consequence of starting up rather than a repair anyone has to run.

pub(crate) mod placement;
pub(crate) mod provider;
pub(crate) mod release_diff;
pub(crate) mod store;
pub(crate) mod transport;
pub(crate) mod trust;

#[cfg(test)]
pub(crate) mod tests;

use std::collections::BTreeMap;

use rspice_pack::{PackError, Snapshot, VerifiedPack, decode_snapshot};

pub use placement::{PartPlacement, plan_library_placement, plan_part_placement, refusal_sentence};
pub(crate) use provider::precedence;
pub use provider::{ModelHubPartRow, PartProvenance, PartState, missing_capabilities};
pub use release_diff::{ChangedPart, PartFact, PartStanding, ReleaseDiff, ReleaseDiffKey};
pub(crate) use store::NO_CATALOG_SERIAL;
pub use store::{InstalledPack, MemoryModelHubStore, ModelHubStore};
pub(crate) use transport::require_exact_bytes;
pub use transport::{ModelHubTransport, OfflineTransport};
pub use trust::TrustAnchor;

#[cfg(not(target_arch = "wasm32"))]
pub use store::FilesystemModelHubStore;
#[cfg(target_arch = "wasm32")]
pub use transport::BrowserModelHubTransport;
#[cfg(not(target_arch = "wasm32"))]
pub use transport::CloudModelHubTransport;

/// Every reason the Model Hub runtime refuses to act.
///
/// The variants are the vocabulary a caller reasons about, not a restatement
/// of whatever the failing layer happened to say: an unreachable service, a
/// pack whose bytes do not match what the catalog signed, and a pack this
/// engine cannot run are three different situations with three different
/// remedies, and message text is never how they are told apart.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModelHubError {
    /// The service could not be reached. Cached state is still usable.
    Offline,
    /// The service answered, and refused. The text is its own presentation.
    Rejected(String),
    /// The transport failed in a way that is neither of the above.
    Transport(String),
    /// A download capability no longer matches the handoff it was issued for.
    HandoffExpired,
    /// A pack identifier or version is not one the format can express.
    MalformedRelease(&'static str),
    /// No catalog snapshot has been cached or fetched.
    NoCatalog,
    /// The service offered a catalog older than one already accepted here.
    ///
    /// Authenticity is not the question — the snapshot verified — so this is
    /// not a `Format` refusal. What failed is *freshness*: a valid catalog
    /// from before a recall would undo the recall, so the older one is
    /// discarded and the held one kept.
    CatalogRollback { held: u64, offered: u64 },
    /// The held catalog states an instant it stops being believable, and this
    /// clock is past it. Only hub offerings are refused; nothing local is.
    CatalogExpired { expires_at: String },
    /// The publisher recalled this release, in these words.
    ReleaseRevoked {
        pack_id: String,
        version: String,
        reason: String,
    },
    /// The catalog does not publish this release.
    ReleaseUnknown { pack_id: String, version: String },
    /// The release is not installed on this machine.
    NotInstalled { pack_id: String, version: String },
    /// Received bytes were not the length that was promised.
    LengthMismatch { expected: u64, actual: u64 },
    /// Received bytes did not hash to the digest that was promised.
    DigestMismatch { expected: String, actual: String },
    /// The bytes are internally consistent but describe a different release.
    IdentityMismatch { expected: String, actual: String },
    /// The pack requires capabilities this engine build does not offer.
    Incompatible { missing: Vec<String> },
    /// The pack format refused the archive, snapshot, or manifest.
    Format(String),
    /// Local storage refused.
    Storage(String),
    /// The compiled-in trust anchor is not a usable key. A unit test in
    /// [`trust`] proves the shipped constant is, so this is unreachable in a
    /// build that ran its own tests.
    TrustAnchorUnusable,
}

impl From<PackError> for ModelHubError {
    fn from(error: PackError) -> Self {
        Self::Format(error.to_string())
    }
}

impl std::fmt::Display for ModelHubError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Offline => formatter.write_str("the model hub could not be reached"),
            Self::Rejected(detail) => write!(formatter, "the model hub refused: {detail}"),
            Self::Transport(detail) => write!(formatter, "model hub transport failed: {detail}"),
            Self::HandoffExpired => {
                formatter.write_str("the download capability no longer matches its handoff")
            }
            Self::MalformedRelease(field) => {
                write!(formatter, "{field} is not a value the pack format defines")
            }
            Self::NoCatalog => formatter.write_str("no signed catalog snapshot is available"),
            // The wording carries the word each refusal is classified by:
            // `ModelsOperationalState::from_failure` reads the sentence, and
            // these three have to land on Stale, Stale and Recalled rather
            // than on the generic execution error every unmatched text gets.
            Self::CatalogRollback { held, offered } => write!(
                formatter,
                "the model hub offered catalog serial {offered}, which is stale beside serial \
                 {held} this machine has already accepted; the held catalog was kept"
            ),
            // "must be refreshed" would have read better and classified worse:
            // `from_failure` reads "must " as an input the operator got wrong,
            // and would have told them to correct a value rather than refresh.
            Self::CatalogExpired { expires_at } => write!(
                formatter,
                "the held catalog is stale: it expired at {expires_at}, so the hub offers nothing \
                 until it is refreshed"
            ),
            Self::ReleaseRevoked {
                pack_id,
                version,
                reason,
            } => write!(
                formatter,
                "{pack_id} {version} was recalled by its publisher: {reason}"
            ),
            Self::ReleaseUnknown { pack_id, version } => {
                write!(
                    formatter,
                    "the catalog does not publish {pack_id} {version}"
                )
            }
            Self::NotInstalled { pack_id, version } => {
                write!(formatter, "{pack_id} {version} is not installed")
            }
            Self::LengthMismatch { expected, actual } => {
                write!(formatter, "expected {expected} bytes and received {actual}")
            }
            Self::DigestMismatch { expected, actual } => write!(
                formatter,
                "expected the content digest {expected} and computed {actual}"
            ),
            Self::IdentityMismatch { expected, actual } => {
                write!(formatter, "expected {expected} and found {actual}")
            }
            Self::Incompatible { missing } => write!(
                formatter,
                "this build does not offer the required capabilities: {}",
                missing.join(", ")
            ),
            Self::Format(detail) => write!(formatter, "the pack format refused: {detail}"),
            Self::Storage(detail) => write!(formatter, "model hub storage failed: {detail}"),
            Self::TrustAnchorUnusable => {
                formatter.write_str("the compiled-in model hub key is not a usable ed25519 key")
            }
        }
    }
}

impl std::error::Error for ModelHubError {}

/// What the startup sweep concluded about one installed release's archive.
///
/// Discovery already digests every installed archive — that is what a project
/// pin records — and for a release the release compared that digest against
/// nothing at all. It is the cheapest evidence the hub has: the signed catalog
/// publishes the digest each release's archive must have, and an installed
/// archive that no longer hashes to it is a release whose bytes were replaced
/// after they were proved.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArchiveEvidence {
    /// The archive on disk hashes to what the signed catalog publishes.
    MatchesCatalog,
    /// It does not, so these are not the bytes this release was proved as.
    DiffersFromCatalog,
    /// The catalog publishes no such release, so there is nothing to compare
    /// against. A withdrawn release still on this machine reads this way, and
    /// so does every release while no catalog has been fetched.
    NotPublished,
}

/// What the held catalog is, beyond the packs it lists.
///
/// Every field is settled at the one instant the snapshot bytes are in hand
/// and proved, and then kept. Recomputing any of it later would mean hashing
/// the whole snapshot again for a reader who only wants to look at it, and a
/// surface that repaints sixty times a second is exactly such a reader.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CatalogIdentity {
    /// The service's monotonic snapshot identity, when this session is the one
    /// that fetched it.
    ///
    /// A snapshot restored from the on-disk cache carries none: the store
    /// keeps the exact signed bytes, and the generation is a handoff field
    /// that never travelled inside them. Reporting the last generation this
    /// build happened to see would be a claim about the service rather than
    /// about what is held, so absence is stated instead.
    pub generation: Option<u64>,
    /// Lowercase hexadecimal SHA-256 of the exact snapshot bytes this client
    /// verified — the same digest the handoff declared and
    /// [`require_exact_bytes`] checked.
    pub digest: String,
    pub schema: u32,
    /// The catalog's own ordinal, covered by the signature.
    ///
    /// Unlike `generation` this survives a restart, because it travelled
    /// inside the signed bytes rather than beside them — which is exactly why
    /// it, and not the handoff field, is what defeats a rollback.
    pub serial: u64,
    /// RFC 3339 instant the publisher generated the snapshot at, covered by
    /// the signature.
    pub generated_at: String,
    /// RFC 3339 instant after which this catalog must not be believed, covered
    /// by the signature.
    pub expires_at: String,
    /// `expires_at` as unix seconds, resolved once where the snapshot was
    /// proved.
    ///
    /// Kept because a repainting surface asks whether the catalog has expired
    /// on every frame, and re-parsing a date string sixty times a second to
    /// answer is work with a known answer. `None` means the instant did not
    /// parse — which the signed format's own shape rules make unreachable for
    /// a snapshot that decoded, and which is read as "no expiry known" rather
    /// than as "expired", because refusing every hub action over an unparsable
    /// field would be a client bricking itself on a field it misread.
    pub expires_at_seconds: Option<u64>,
}

/// The releases the held catalog recalls, and why.
///
/// Computed once with the snapshot rather than per query: a recall check runs
/// on every browse row, every ledger line and every install, and walking the
/// revocation list each time would make the cost quadratic in a catalog that
/// is allowed ten thousand of them.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Recalls(BTreeMap<String, String>);

impl Recalls {
    /// Projects a snapshot's revocation list into the lookup a client uses.
    fn of(snapshot: Option<&Snapshot>) -> Self {
        Self(
            snapshot
                .into_iter()
                .flat_map(|snapshot| snapshot.revocations.iter())
                .map(|recall| {
                    (
                        release_key(&recall.pack_id, &recall.version),
                        recall.reason.clone(),
                    )
                })
                .collect(),
        )
    }

    /// The publisher's reason, when this release is recalled.
    #[must_use]
    pub fn reason(&self, pack_id: &str, version: &str) -> Option<&str> {
        self.0
            .get(&release_key(pack_id, version))
            .map(String::as_str)
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

/// One library in the open project pinned to a release the catalog recalls.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecalledPin {
    /// The project library the pin sits on.
    pub library: String,
    pub pack_id: String,
    pub version: String,
    /// The publisher's own words.
    pub reason: String,
}

/// Every pack pin in a project that the held catalog recalls.
///
/// This is a *report*, never a refusal. Nothing here removes a library, edits
/// a pin, or stops a run: the project owns authenticated bytes it retained
/// when the part was added, those bytes still hash to what the pin recorded,
/// and a design built on them still solves. What the reader is owed is the
/// news, in time to plan a migration rather than in the middle of a tapeout.
#[must_use]
pub fn recalled_pins(
    recalls: &Recalls,
    manager: &crate::state::model_library::ModelLibraryManager,
) -> Vec<RecalledPin> {
    if recalls.is_empty() {
        return Vec::new();
    }
    manager
        .libraries_sorted()
        .into_iter()
        .filter_map(|library| {
            let pin = library.pack_pin.as_ref()?;
            let reason = recalls.reason(&pin.pack_id, &pin.pack_version)?;
            Some(RecalledPin {
                library: library.name.clone(),
                pack_id: pin.pack_id.clone(),
                version: pin.pack_version.clone(),
                reason: reason.to_owned(),
            })
        })
        .collect()
}

/// A content key over every pack pin a project holds.
///
/// The project half of the latch that decides whether a recall report is still
/// about what is in front of the reader. It is content and not a revision
/// counter for the reason the model catalogue's own identity key gives: a
/// project is replaced *wholesale* — opened, restored from history, rebuilt by
/// a hub operation — and a counter arrives carrying whatever value it was
/// saved with, which may be one this session has already reported against.
/// Two different projects would then share a verdict. A key derived from the
/// pins themselves cannot be presented by a project that did not earn it.
///
/// The pins rather than the whole catalogue, because the pins are exactly what
/// a recall is about: a library whose retained bytes changed has not changed
/// which release it was taken from, and re-reporting a recall because somebody
/// re-pinned an unrelated source would be noise.
#[must_use]
pub fn pack_pin_key(manager: &crate::state::model_library::ModelLibraryManager) -> u64 {
    use std::hash::{Hash as _, Hasher as _};

    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    "rspice.model-hub-pack-pins/v1".hash(&mut hasher);
    for library in manager.libraries_sorted() {
        let Some(pin) = library.pack_pin.as_ref() else {
            continue;
        };
        // The library name participates: the same release pinned by two
        // libraries is two commitments, and a project that dropped one of them
        // has changed even though the set of pinned releases has not.
        library.name.hash(&mut hasher);
        pin.pack_id.hash(&mut hasher);
        pin.pack_version.hash(&mut hasher);
        pin.archive_sha256.hash(&mut hasher);
        pin.part_id.hash(&mut hasher);
    }
    hasher.finish()
}

/// How a release is addressed in every map this module keys by one.
///
/// One spelling, because two would let the archive evidence, the recall
/// lookup and the re-proof verdict disagree about which release they are
/// talking about.
fn release_key(pack_id: &str, version: &str) -> String {
    format!("{pack_id}@{version}")
}

/// Proves an archive under the anchor and against its declared identity.
///
/// The manifest is the authority on what a pack requires, so its capability
/// set is checked here. A snapshot's projection of that set is checked
/// elsewhere for a different reason — to refuse before downloading — and
/// neither check makes the other redundant.
///
/// `declared` is the identity the *catalog* stated, when the caller has one to
/// bind the bytes to. A download does: the snapshot named a pack and a version
/// before a byte moved, and an archive that proves under the anchor while
/// describing some other release is a substitution the signature alone would
/// not catch. Restoring bytes this machine already accepted has no such second
/// source — the signature over the manifest is the whole of the claim — so it
/// passes `None` rather than a tautological comparison against the manifest it
/// has just this moment read out of the same bytes.
fn verify_archive(
    anchor: &TrustAnchor,
    archive: &[u8],
    declared: Option<(&str, &str)>,
) -> Result<VerifiedPack, ModelHubError> {
    let verified = rspice_pack::Pack::verify(archive, anchor.key(), anchor.limits())?;
    if let Some((pack_id, version)) = declared
        && (verified.manifest.pack.id != pack_id || verified.manifest.pack.version != version)
    {
        return Err(ModelHubError::IdentityMismatch {
            expected: format!("{pack_id}@{version}"),
            actual: format!(
                "{}@{}",
                verified.manifest.pack.id, verified.manifest.pack.version
            ),
        });
    }
    let missing = missing_capabilities(&verified.manifest.requires.capabilities);
    if !missing.is_empty() {
        return Err(ModelHubError::Incompatible { missing });
    }
    Ok(verified)
}

/// The one door bytes go through to become an installed release.
///
/// Everything that ends with a pack under its real name arrives here: a fresh
/// download, and a restore of bytes a previous browser session accepted. That
/// is the point of it being a function rather than a step inside
/// [`ModelHub::install`]. A restore path that staged and committed on its own
/// would be a second acceptance, and a second acceptance is where a check goes
/// missing — silently, and only on the host that has the second path.
///
/// Nothing here reads a catalog. Acceptance decides whether *these bytes* are
/// an authentic pack this engine can run; every question about whether the
/// catalog may be acted on — expiry, recall, which version is current — is
/// asked by the caller before it gets here, because the answers differ by
/// caller. An install must refuse an expired catalog. A restore of packs
/// already on this machine must not, because an expired catalog withholds
/// offers and stops no local work.
pub(crate) fn accept_archive(
    anchor: &TrustAnchor,
    store: &dyn ModelHubStore,
    archive: &[u8],
    declared: Option<(&str, &str)>,
) -> Result<InstalledPack, ModelHubError> {
    let verified = verify_archive(anchor, archive, declared)?;
    let staged = store.stage_pack(&verified, archive)?;
    match store.commit_pack(staged) {
        Ok(installed) => Ok(installed),
        Err(error) => {
            // `commit_pack` consumed the staging handle, so anything it left
            // behind is swept here rather than leaking to startup.
            let _ = store.sweep_staging();
            Err(error)
        }
    }
}

/// Seconds since the unix epoch for an RFC 3339 instant in UTC.
///
/// The snapshot format fixes the shape — `YYYY-MM-DDTHH:MM:SSZ` — so this
/// reads that shape and refuses anything else rather than pulling in a date
/// library to be lenient about a field the signature already constrains.
///
/// It lives here rather than in the service above because two callers need it
/// and both are about the same signed fields: this module decides whether the
/// catalog has expired, and the service decides how old it is.
pub(crate) fn rfc3339_seconds(value: &str) -> Option<u64> {
    let value = value.strip_suffix('Z')?;
    let (date, time) = value.split_once('T')?;
    let mut date = date.split('-');
    let year: i64 = date.next()?.parse().ok()?;
    let month: i64 = date.next()?.parse().ok()?;
    let day: i64 = date.next()?.parse().ok()?;
    if date.next().is_some() || !(1..=12).contains(&month) || !(1..=31).contains(&day) {
        return None;
    }
    let mut time = time.split(':');
    let hour: i64 = time.next()?.parse().ok()?;
    let minute: i64 = time.next()?.parse().ok()?;
    let second: i64 = time
        .next()?
        .split('.')
        .next()
        .unwrap_or_default()
        .parse()
        .ok()?;
    if time.next().is_some() || hour > 23 || minute > 59 || second > 60 {
        return None;
    }

    // Days from the civil calendar, by Howard Hinnant's algorithm: the shift
    // to a March-based year makes the leap day the last day of the year, which
    // is what removes every special case from the arithmetic.
    let year = if month <= 2 { year - 1 } else { year };
    let era = if year >= 0 { year } else { year - 399 } / 400;
    let year_of_era = year - era * 400;
    let day_of_year = (153 * (if month > 2 { month - 3 } else { month + 9 }) + 2) / 5 + day - 1;
    let day_of_era = year_of_era * 365 + year_of_era / 4 - year_of_era / 100 + day_of_year;
    let days = era * 146_097 + day_of_era - 719_468;
    u64::try_from(days * 86_400 + hour * 3600 + minute * 60 + second).ok()
}

impl CatalogIdentity {
    /// Settles everything about one proved snapshot, at the one instant its
    /// bytes and its typed projection are both in hand.
    fn of(snapshot: &Snapshot, digest: String, generation: Option<u64>) -> Self {
        Self {
            generation,
            digest,
            schema: snapshot.schema,
            serial: snapshot.serial,
            generated_at: snapshot.generated_at.clone(),
            expires_at_seconds: rfc3339_seconds(&snapshot.expires_at),
            expires_at: snapshot.expires_at.clone(),
        }
    }

    /// Whether this catalog had stopped being believable by `now`.
    ///
    /// An instant that did not parse is not an expiry: see the field's own
    /// note. The comparison is `>=` because `expires_at` is the instant after
    /// which the catalog must not be believed, and a boundary a publisher
    /// named is a boundary they meant.
    #[must_use]
    pub fn expired_at(&self, now: u64) -> bool {
        self.expires_at_seconds
            .is_some_and(|expires_at| now >= expires_at)
    }
}

/// The client-side Model Hub.
///
/// It owns the trust anchor, the store, and the two derived facts a shelf
/// reads: the decoded catalog snapshot and the set of installed releases.
/// Both are kept as values rather than re-read per query, because both change
/// only when this type changes them.
#[derive(Debug)]
pub struct ModelHub {
    anchor: TrustAnchor,
    store: Box<dyn ModelHubStore>,
    snapshot: Option<Snapshot>,
    /// Identity of the snapshot above, settled where its bytes were proved.
    catalog_identity: Option<CatalogIdentity>,
    /// The highest catalog serial this store has ever accepted, and the floor
    /// a refresh must reach. Read from the store rather than from the cached
    /// snapshot, so discarding or replacing the cache cannot lower it.
    last_seen_serial: u64,
    /// Releases the held catalog recalls, projected where it was decoded.
    recalls: Recalls,
    installed: Vec<InstalledPack>,
    /// Whether a cached catalog was on disk and did not verify. "Never
    /// fetched" and "fetched, and the cache no longer proves" are the same
    /// absence with different remedies, and the second one used to be silent.
    catalog_cache_discarded: bool,
    /// Startup archive comparison, keyed `<pack id>@<version>`.
    archive_evidence: std::collections::BTreeMap<String, ArchiveEvidence>,
    /// Where installed sources live, when they live on a filesystem. A
    /// browser store has none, and a remote row never has one either.
    pack_root: Option<std::path::PathBuf>,
}

impl ModelHub {
    /// Opens the hub over a store, sweeping anything a killed install left.
    ///
    /// A cached snapshot that no longer verifies — because the release key
    /// rotated, or because something rewrote the file — is dropped rather
    /// than trusted, which is the safe reading of "the cache is not evidence".
    /// That it was dropped is *recorded*, though: behaving exactly as a hub
    /// that has never fetched is the right behaviour and the wrong story, and
    /// a machine told "never fetched" about a catalog it did fetch has no
    /// reason to suspect its own storage.
    pub fn open(
        anchor: TrustAnchor,
        store: Box<dyn ModelHubStore>,
        pack_root: Option<std::path::PathBuf>,
    ) -> Result<Self, ModelHubError> {
        store.sweep_staging()?;
        let cached = store.read_snapshot()?;
        let recorded = store.read_catalog_serial()?;
        let decoded = cached
            .as_ref()
            .and_then(|bytes| decode_snapshot(bytes, anchor.key(), anchor.limits()).ok());
        // A cache below the recorded floor is a rollback, and it is refused
        // here for exactly the reason [`Self::refresh_catalog`] refuses one
        // over the network: an authentic catalog from before a recall would
        // undo the recall. Authenticity is not the question — these bytes just
        // proved — so the check has to be a *separate* one, and it has to
        // happen at load rather than only at refresh. Without it, replacing
        // the cache with an older authentic snapshot was enough to make this
        // client hold, offer from, and compute `recalls` out of a catalog it
        // had already superseded: the floor stopped the *next* refresh and
        // nothing at all stopped the substitution itself.
        //
        // It matters more in a browser than on a filesystem, which is what
        // brought it to light. Persisted browser storage is reachable by
        // anything running on the origin, so "something rewrote the cache" is
        // an ordinary event there rather than an unusual one — but the refusal
        // is written once, here, so both hosts get it from the same two lines.
        let rolled_back = decoded.as_ref().is_some_and(|held| held.serial < recorded);
        let snapshot = decoded.filter(|_| !rolled_back);
        // A cache that was rejected — for its signature or for its serial — is
        // reported as discarded rather than as absent, for the reason this
        // method's own note gives: behaving like a client that never fetched is
        // the right behaviour and the wrong story.
        let catalog_cache_discarded = cached.is_some() && snapshot.is_none();
        // One hash, of bytes this call already read, at the only moment they
        // are both present and proved. A reader asking later gets the answer
        // rather than the work.
        let catalog_identity = snapshot
            .as_ref()
            .zip(cached.as_ref())
            .map(|(snapshot, bytes)| {
                CatalogIdentity::of(snapshot, rspice_pack::sha256_hex(bytes), None)
            });
        // The floor is whichever is higher: what the store recorded, or what
        // the cached catalog turned out to carry. They agree on a store this
        // build wrote, and taking the maximum is what makes a floor file that
        // was lost, truncated, or never written by an older build recover to
        // the truth rather than to zero.
        let last_seen_serial = recorded.max(
            snapshot
                .as_ref()
                .map_or(NO_CATALOG_SERIAL, |held| held.serial),
        );
        if last_seen_serial > recorded {
            store.record_catalog_serial(last_seen_serial)?;
        }
        let recalls = Recalls::of(snapshot.as_ref());
        let installed = store.installed_packs()?;
        let mut hub = Self {
            anchor,
            store,
            snapshot,
            catalog_identity,
            last_seen_serial,
            recalls,
            installed,
            catalog_cache_discarded,
            archive_evidence: std::collections::BTreeMap::new(),
            pack_root,
        };
        hub.recompute_archive_evidence();
        Ok(hub)
    }

    /// Whether a cached catalog was present and failed verification.
    pub const fn catalog_cache_discarded(&self) -> bool {
        self.catalog_cache_discarded
    }

    /// What the archive of one installed release hashes to, against the
    /// catalog. `None` means that release is not installed here.
    pub fn archive_evidence(&self, pack_id: &str, version: &str) -> Option<ArchiveEvidence> {
        self.archive_evidence
            .get(&release_key(pack_id, version))
            .copied()
    }

    /// Compares every installed archive digest against the signed catalog.
    ///
    /// Free in the sense that matters: the digests were already computed by
    /// discovery, and the catalog is already decoded in memory. Nothing here
    /// reads a byte of an archive.
    fn recompute_archive_evidence(&mut self) {
        self.archive_evidence = self
            .installed
            .iter()
            .map(|pack| {
                let published = self.snapshot.as_ref().and_then(|snapshot| {
                    snapshot
                        .packs
                        .iter()
                        .find(|listed| listed.id == pack.pack_id())
                        .and_then(|listed| {
                            listed
                                .releases
                                .iter()
                                .find(|release| release.version == pack.version())
                        })
                });
                let evidence = match published {
                    None => ArchiveEvidence::NotPublished,
                    Some(release) if release.archive_sha256 == pack.archive_sha256 => {
                        ArchiveEvidence::MatchesCatalog
                    }
                    Some(_) => ArchiveEvidence::DiffersFromCatalog,
                };
                (release_key(pack.pack_id(), pack.version()), evidence)
            })
            .collect();
    }

    /// The current signed catalog, if one has been cached or fetched.
    pub fn snapshot(&self) -> Option<&Snapshot> {
        self.snapshot.as_ref()
    }

    /// What the held catalog is: its digest, its schema, its serial, when it
    /// was signed and until when, and the generation if this session fetched
    /// it.
    pub fn catalog_identity(&self) -> Option<&CatalogIdentity> {
        self.catalog_identity.as_ref()
    }

    /// The highest catalog serial this machine has ever accepted.
    ///
    /// [`NO_CATALOG_SERIAL`] means none, which is the only state in which any
    /// authentic catalog is acceptable.
    pub const fn last_seen_serial(&self) -> u64 {
        self.last_seen_serial
    }

    /// The releases the held catalog recalls.
    pub const fn recalls(&self) -> &Recalls {
        &self.recalls
    }

    /// The instant the held catalog stopped being believable, if it has.
    ///
    /// The clock is read here rather than passed in because this is the
    /// consumer the pack format defers the decision to, and
    /// [`crate::time_compat::unix_epoch`] answers identically on both targets.
    /// It costs a clock read and an integer comparison — no parse, no hash —
    /// which is what lets a repainting surface ask it per frame.
    pub fn catalog_expired(&self) -> Option<&str> {
        let identity = self.catalog_identity.as_ref()?;
        identity
            .expired_at(crate::time_compat::unix_epoch().as_secs())
            .then_some(identity.expires_at.as_str())
    }

    /// The catalog, when it may still be believed as an *offer*.
    ///
    /// `None` past the expiry the publisher signed. Everything reading this is
    /// asking "what can be installed or browsed from the hub"; everything
    /// asking about bytes already on this machine reads [`Self::snapshot`],
    /// which never stops answering. That split is the whole of D-D: an expired
    /// catalog silences the shop, not the workshop.
    pub fn offered_snapshot(&self) -> Option<&Snapshot> {
        if self.catalog_expired().is_some() {
            return None;
        }
        self.snapshot.as_ref()
    }

    /// Refuses when the held catalog may no longer be offered from.
    pub fn require_current_catalog(&self) -> Result<(), ModelHubError> {
        match self.catalog_expired() {
            Some(expires_at) => Err(ModelHubError::CatalogExpired {
                expires_at: expires_at.to_owned(),
            }),
            None => Ok(()),
        }
    }

    /// Refuses when the held catalog recalls this release.
    ///
    /// Separate from [`Self::require_current_catalog`] because the two refuse
    /// different things: an expired catalog stops the hub offering anything,
    /// while a recall follows one release wherever it is named — including
    /// into retention from a copy already on this machine, which no expiry
    /// check reaches.
    pub fn require_not_recalled(&self, pack_id: &str, version: &str) -> Result<(), ModelHubError> {
        match self.recalls.reason(pack_id, version) {
            Some(reason) => Err(ModelHubError::ReleaseRevoked {
                pack_id: pack_id.to_owned(),
                version: version.to_owned(),
                reason: reason.to_owned(),
            }),
            None => Ok(()),
        }
    }

    /// Every installed release.
    pub fn installed(&self) -> &[InstalledPack] {
        &self.installed
    }

    pub fn anchor(&self) -> &TrustAnchor {
        &self.anchor
    }

    /// Fetches, proves, and caches the current catalog snapshot.
    ///
    /// The snapshot is proved three times on the way in: against the digest
    /// the handoff declared, which catches a truncated or substituted
    /// download; against the trust anchor, which decides whether the content
    /// is authentic; and against the serial floor, which decides whether an
    /// authentic catalog is the *current* one. Nothing is cached until all
    /// three pass, so a cached snapshot is always one this client verified and
    /// never one it has already superseded.
    ///
    /// An equal serial is accepted. It is the same catalog re-fetched — the
    /// ordinary outcome of a refresh on a quiet day — and refusing it would
    /// turn re-checking into an error.
    pub fn refresh_catalog(
        &mut self,
        transport: &dyn ModelHubTransport,
    ) -> Result<u64, ModelHubError> {
        let handoff = transport.catalog_handoff()?;
        let bytes = transport.fetch_catalog(&handoff)?;
        require_exact_bytes(&bytes, handoff.content_length, &handoff.content_sha256)?;
        let snapshot = decode_snapshot(&bytes, self.anchor.key(), self.anchor.limits())?;
        if snapshot.serial < self.last_seen_serial {
            // Refused before a byte of it is written, so the held catalog, the
            // installed packs and every project pin stand exactly as they did.
            return Err(ModelHubError::CatalogRollback {
                held: self.last_seen_serial,
                offered: snapshot.serial,
            });
        }
        self.store.write_snapshot(&bytes)?;
        // The floor is raised after the bytes land, so a store that refused
        // the write does not end up claiming a serial it does not hold.
        self.store.record_catalog_serial(snapshot.serial)?;
        self.last_seen_serial = self.last_seen_serial.max(snapshot.serial);
        // The handoff digest is the one `require_exact_bytes` just proved the
        // received bytes against, so recording it costs nothing and states
        // exactly what this client accepted.
        self.catalog_identity = Some(CatalogIdentity::of(
            &snapshot,
            handoff.content_sha256.clone(),
            Some(handoff.generation),
        ));
        self.recalls = Recalls::of(Some(&snapshot));
        self.snapshot = Some(snapshot);
        self.catalog_cache_discarded = false;
        self.recompute_archive_evidence();
        Ok(handoff.generation)
    }

    /// The release the catalog publishes for one pack and version.
    fn release(
        &self,
        pack_id: &str,
        version: &str,
    ) -> Result<&rspice_pack::SnapshotRelease, ModelHubError> {
        let snapshot = self.snapshot.as_ref().ok_or(ModelHubError::NoCatalog)?;
        snapshot
            .packs
            .iter()
            .find(|pack| pack.id == pack_id)
            .and_then(|pack| {
                pack.releases
                    .iter()
                    .find(|release| release.version == version)
            })
            .ok_or_else(|| ModelHubError::ReleaseUnknown {
                pack_id: pack_id.to_owned(),
                version: version.to_owned(),
            })
    }

    /// What the held catalog states one release changes about another.
    ///
    /// Both releases are looked up through the same [`Self::release`] the
    /// install path uses, so a version the catalog does not publish refuses
    /// with [`ModelHubError::ReleaseUnknown`] rather than producing a diff
    /// against nothing — and a client with no catalog at all cannot produce
    /// one either.
    ///
    /// The key names the exact snapshot both records were read from, which is
    /// what lets a caller cache the result on content instead of recomputing
    /// it, or worse, on a counter that a wholesale catalog replacement carries
    /// over unchanged.
    pub fn release_diff(
        &self,
        pack_id: &str,
        from: &str,
        to: &str,
    ) -> Result<ReleaseDiff, ModelHubError> {
        let digest = self
            .catalog_identity
            .as_ref()
            .ok_or(ModelHubError::NoCatalog)?
            .digest
            .clone();
        let older = self.release(pack_id, from)?;
        let newer = self.release(pack_id, to)?;
        Ok(release_diff::release_diff(
            ReleaseDiffKey {
                catalog_digest: digest,
                pack_id: pack_id.to_owned(),
                from: from.to_owned(),
                to: to.to_owned(),
            },
            older,
            newer,
        ))
    }

    /// Whether one release may be adopted from, or the reason it may not.
    ///
    /// Adoption re-runs the retention path over bytes this machine has already
    /// proved, so it demands exactly the standing a fresh install would have
    /// left behind and refuses in the vocabulary that install refuses in.
    /// Four ways it can fail, each already a named [`ModelHubError`]:
    /// no catalog to check against, a release the catalog no longer publishes
    /// — which is what a withdrawn or revoked one looks like from here — a
    /// release this build cannot run, and an archive on this machine that no
    /// longer hashes to the digest the signed catalog publishes for it.
    ///
    /// It is deliberately not a bool. "Adopt is unavailable" is not something
    /// a reader can act on; "the catalog no longer publishes 1.1.0" is.
    ///
    /// A fifth way it fails is a recall. That is checked here rather than left
    /// to the delisting above, because a recall is *stronger* than delisting:
    /// a recalled release may still be listed, and a client that only noticed
    /// the delisting would adopt onto bytes the publisher has withdrawn while
    /// naming no reason at all.
    pub fn adoptable(&self, pack_id: &str, version: &str) -> Result<(), ModelHubError> {
        self.require_not_recalled(pack_id, version)?;
        let release = self.release(pack_id, version)?;
        let missing = missing_capabilities(&release.capabilities);
        if !missing.is_empty() {
            return Err(ModelHubError::Incompatible { missing });
        }
        let installed = self
            .installed
            .iter()
            .find(|pack| pack.pack_id() == pack_id && pack.version() == version)
            .ok_or_else(|| ModelHubError::NotInstalled {
                pack_id: pack_id.to_owned(),
                version: version.to_owned(),
            })?;
        match self.archive_evidence(pack_id, version) {
            Some(ArchiveEvidence::MatchesCatalog) => Ok(()),
            Some(ArchiveEvidence::DiffersFromCatalog) => Err(ModelHubError::DigestMismatch {
                expected: release.archive_sha256.clone(),
                actual: installed.archive_sha256.clone(),
            }),
            // The catalog published this release a moment ago, above. Reading
            // it as unpublished here means the evidence and the snapshot
            // disagree, and the safe reading of that is the stricter one.
            Some(ArchiveEvidence::NotPublished) | None => Err(ModelHubError::ReleaseUnknown {
                pack_id: pack_id.to_owned(),
                version: version.to_owned(),
            }),
        }
    }

    /// Whether this build can run a published release.
    pub fn compatibility(
        &self,
        pack_id: &str,
        version: &str,
    ) -> Result<Vec<String>, ModelHubError> {
        Ok(missing_capabilities(
            &self.release(pack_id, version)?.capabilities,
        ))
    }

    /// Downloads, proves, and installs one published release.
    ///
    /// The order is the argument. Capability compatibility is settled before
    /// a byte moves, because installing a pack this engine cannot run is a
    /// worse outcome than refusing it. The archive digest is then taken from
    /// the signed snapshot and checked against the download handoff *before*
    /// the download, so a handoff that points at other bytes is refused
    /// without fetching them — and checked again against the bytes that
    /// actually arrive, so the download itself cannot substitute anything.
    /// Only then is the archive verified end to end and staged.
    ///
    /// Two questions come before all of that, because they are about whether
    /// the catalog may be acted on at all: whether it has expired, and whether
    /// this release was recalled. Both refuse without touching the network,
    /// which is what makes an expired catalog a quiet workspace rather than a
    /// download that fails at the end.
    pub fn install(
        &mut self,
        transport: &dyn ModelHubTransport,
        pack_id: &str,
        version: &str,
    ) -> Result<InstalledPack, ModelHubError> {
        self.require_current_catalog()?;
        self.require_not_recalled(pack_id, version)?;
        let release = self.release(pack_id, version)?.clone();
        let missing = missing_capabilities(&release.capabilities);
        if !missing.is_empty() {
            return Err(ModelHubError::Incompatible { missing });
        }

        let handoff = transport.archive_handoff(pack_id, version)?;
        if handoff.content_sha256 != release.archive_sha256 {
            return Err(ModelHubError::DigestMismatch {
                expected: release.archive_sha256.clone(),
                actual: handoff.content_sha256,
            });
        }
        if handoff.content_length != release.archive_length {
            return Err(ModelHubError::LengthMismatch {
                expected: release.archive_length,
                actual: handoff.content_length,
            });
        }

        let archive = transport.fetch_archive(&handoff)?;
        require_exact_bytes(&archive, release.archive_length, &release.archive_sha256)?;
        let installed = accept_archive(
            &self.anchor,
            self.store.as_ref(),
            &archive,
            Some((pack_id, version)),
        )?;
        self.installed = self.store.installed_packs()?;
        self.recompute_archive_evidence();
        Ok(installed)
    }

    /// Removes one installed release.
    ///
    /// Only the hub's own copy is removed. A project that added a part from
    /// this pack retained its own authenticated bytes at that moment, so it
    /// keeps opening and keeps solving with nothing installed at all.
    pub fn uninstall(&mut self, pack_id: &str, version: &str) -> Result<bool, ModelHubError> {
        let removed = self.store.remove_pack(pack_id, version)?;
        self.installed = self.store.installed_packs()?;
        self.recompute_archive_evidence();
        Ok(removed)
    }

    /// Re-proves an installed release end to end under the anchor.
    pub fn verify_installed(
        &self,
        pack_id: &str,
        version: &str,
    ) -> Result<VerifiedPack, ModelHubError> {
        self.store.verify_installed(pack_id, version, &self.anchor)
    }

    /// The unified part index over foundation, installed, catalog, and
    /// project-retained parts.
    ///
    /// It is built from [`Self::offered_snapshot`] and the recall list, so an
    /// expired catalog contributes no remote rows and a recalled release
    /// contributes none either. What this machine holds is untouched by both:
    /// the foundation, the project's retained libraries and every installed
    /// pack are indexed exactly as they were, which is what "never bricks
    /// offline" means at the level a reader sees.
    pub fn part_index(
        &self,
        libraries: &[&crate::state::model_library::ModelLibrary],
    ) -> Vec<ModelHubPartRow> {
        provider::part_index(
            libraries,
            &self.installed,
            self.offered_snapshot(),
            &self.recalls,
            self.pack_root.as_deref(),
        )
    }

    /// Adds one part from an installed release into a project's libraries.
    ///
    /// This is the join between the hub and the project, and it deliberately
    /// goes through the same retained-import path an uploaded source uses:
    /// the project ends up owning authenticated bytes and their closure, not
    /// a reference to the hub. Uninstalling the release afterwards therefore
    /// cannot change what the project simulates. What the pin adds is the
    /// record of where those bytes came from.
    ///
    /// The bytes handed over are the ones [`Self::verify_installed`] just
    /// proved end to end under the anchor, not whatever is expanded beside the
    /// archive. That is the stronger reading of "the project retains
    /// authenticated bytes" — a source edited on disk after installation is
    /// refused here rather than retained forever — and it is what makes this
    /// work identically on a store that has no disk at all.
    pub fn add_part_to_project(
        &self,
        manager: &mut crate::state::model_library::ModelLibraryManager,
        pack_id: &str,
        version: &str,
        part_id: &str,
    ) -> Result<String, ModelHubError> {
        let pin = self.part_pin(pack_id, version, part_id)?;
        let verified = self.verify_installed(pack_id, version)?;
        let relative = verified
            .manifest
            .parts
            .iter()
            .find(|part| part.id == part_id || part.aliases.iter().any(|alias| alias == part_id))
            .map(|part| part.source.path.clone())
            .ok_or_else(|| ModelHubError::IdentityMismatch {
                expected: format!("a part named {part_id}"),
                actual: format!("{pack_id}@{version} publishes no such part"),
            })?;
        manager
            .add_pack_release_part(verified.files.into_iter().collect(), &relative, pin)
            .map_err(ModelHubError::Storage)
    }

    /// The pin a project records when it adds a part from an installed pack.
    ///
    /// The pin names the exact published bytes — pack, version, archive
    /// digest, part — beside the retained source closure the project already
    /// keeps. The closure is what makes the project reproducible; the pin is
    /// what makes it *attributable*, so a saved design can still say which
    /// release its models came from after that release is uninstalled,
    /// superseded, or withdrawn.
    ///
    /// A recalled release refuses here, which is the narrowest place that
    /// covers every route into a project: retention is the one step every
    /// gesture that puts pack bytes into a design has to pass through. It is
    /// deliberately the *only* thing a recall stops. The release stays
    /// installed, the bytes a project retained before the recall stay retained,
    /// and both keep simulating — a recall tells a reader to stop reaching for
    /// something, not to throw away work already built on it.
    pub fn part_pin(
        &self,
        pack_id: &str,
        version: &str,
        part_id: &str,
    ) -> Result<crate::state::model_library::PackPartPin, ModelHubError> {
        self.require_not_recalled(pack_id, version)?;
        let installed = self
            .installed
            .iter()
            .find(|pack| pack.pack_id() == pack_id && pack.version() == version)
            .ok_or_else(|| ModelHubError::NotInstalled {
                pack_id: pack_id.to_owned(),
                version: version.to_owned(),
            })?;
        if !installed
            .manifest
            .parts
            .iter()
            .any(|part| part.id == part_id || part.aliases.iter().any(|alias| alias == part_id))
        {
            return Err(ModelHubError::IdentityMismatch {
                expected: format!("a part named {part_id}"),
                actual: format!("{pack_id}@{version} publishes no such part"),
            });
        }
        Ok(crate::state::model_library::PackPartPin {
            pack_id: pack_id.to_owned(),
            pack_version: version.to_owned(),
            archive_sha256: installed.archive_sha256.clone(),
            part_id: part_id.to_owned(),
        })
    }
}
