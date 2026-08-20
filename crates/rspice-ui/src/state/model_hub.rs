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

use rspice_pack::{PackError, Snapshot, VerifiedPack, decode_snapshot};

pub use placement::{PartPlacement, plan_library_placement, plan_part_placement, refusal_sentence};
pub(crate) use provider::precedence;
pub use provider::{ModelHubPartRow, PartProvenance, PartState, missing_capabilities};
pub use release_diff::{PartFact, PartStanding, ReleaseDiff, ReleaseDiffKey};
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
    /// RFC 3339 instant the publisher generated the snapshot at, covered by
    /// the signature.
    pub generated_at: String,
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
        let snapshot = cached
            .as_ref()
            .and_then(|bytes| decode_snapshot(bytes, anchor.key(), anchor.limits()).ok());
        let catalog_cache_discarded = cached.is_some() && snapshot.is_none();
        // One hash, of bytes this call already read, at the only moment they
        // are both present and proved. A reader asking later gets the answer
        // rather than the work.
        let catalog_identity = snapshot
            .as_ref()
            .zip(cached.as_ref())
            .map(|(snapshot, bytes)| CatalogIdentity {
                generation: None,
                digest: rspice_pack::sha256_hex(bytes),
                schema: snapshot.schema,
                generated_at: snapshot.generated_at.clone(),
            });
        let installed = store.installed_packs()?;
        let mut hub = Self {
            anchor,
            store,
            snapshot,
            catalog_identity,
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
            .get(&format!("{pack_id}@{version}"))
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
                (format!("{}@{}", pack.pack_id(), pack.version()), evidence)
            })
            .collect();
    }

    /// The current signed catalog, if one has been cached or fetched.
    pub fn snapshot(&self) -> Option<&Snapshot> {
        self.snapshot.as_ref()
    }

    /// What the held catalog is: its digest, its schema, when it was signed,
    /// and the generation, if this session is the one that fetched it.
    pub fn catalog_identity(&self) -> Option<&CatalogIdentity> {
        self.catalog_identity.as_ref()
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
    /// The snapshot is proved twice on the way in: against the digest the
    /// handoff declared, which catches a truncated or substituted download,
    /// and against the trust anchor, which is the only check that decides
    /// whether the content is believed. Nothing is cached until both pass, so
    /// a cached snapshot is always one this client verified.
    pub fn refresh_catalog(
        &mut self,
        transport: &dyn ModelHubTransport,
    ) -> Result<u64, ModelHubError> {
        let handoff = transport.catalog_handoff()?;
        let bytes = transport.fetch_catalog(&handoff)?;
        require_exact_bytes(&bytes, handoff.content_length, &handoff.content_sha256)?;
        let snapshot = decode_snapshot(&bytes, self.anchor.key(), self.anchor.limits())?;
        self.store.write_snapshot(&bytes)?;
        // The handoff digest is the one `require_exact_bytes` just proved the
        // received bytes against, so recording it costs nothing and states
        // exactly what this client accepted.
        self.catalog_identity = Some(CatalogIdentity {
            generation: Some(handoff.generation),
            digest: handoff.content_sha256.clone(),
            schema: snapshot.schema,
            generated_at: snapshot.generated_at.clone(),
        });
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
    pub fn adoptable(&self, pack_id: &str, version: &str) -> Result<(), ModelHubError> {
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
    pub fn install(
        &mut self,
        transport: &dyn ModelHubTransport,
        pack_id: &str,
        version: &str,
    ) -> Result<InstalledPack, ModelHubError> {
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
        let verified = self.verify_archive(&archive, pack_id, version)?;

        let staged = self.store.stage_pack(&verified, &archive)?;
        let installed = match self.store.commit_pack(staged) {
            Ok(installed) => installed,
            Err(error) => {
                // `commit_pack` consumed the staging handle, so anything it
                // left behind is swept here rather than leaking to startup.
                let _ = self.store.sweep_staging();
                return Err(error);
            }
        };
        self.installed = self.store.installed_packs()?;
        self.recompute_archive_evidence();
        Ok(installed)
    }

    /// Proves an archive under the anchor and against its declared identity.
    ///
    /// The manifest is the authority on what a pack requires, so its
    /// capability set is checked here too. The snapshot's projection of that
    /// set was checked earlier for a different reason — to refuse before
    /// downloading — and neither check makes the other redundant.
    fn verify_archive(
        &self,
        archive: &[u8],
        pack_id: &str,
        version: &str,
    ) -> Result<VerifiedPack, ModelHubError> {
        let verified = rspice_pack::Pack::verify(archive, self.anchor.key(), self.anchor.limits())?;
        if verified.manifest.pack.id != pack_id || verified.manifest.pack.version != version {
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
    pub fn part_index(
        &self,
        libraries: &[&crate::state::model_library::ModelLibrary],
    ) -> Vec<ModelHubPartRow> {
        provider::part_index(
            libraries,
            &self.installed,
            self.snapshot.as_ref(),
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
    pub fn part_pin(
        &self,
        pack_id: &str,
        version: &str,
        part_id: &str,
    ) -> Result<crate::state::model_library::PackPartPin, ModelHubError> {
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
