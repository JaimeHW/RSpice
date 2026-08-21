//! Where verified Model Hub bytes live, and how they arrive there safely.
//!
//! Two shapes of state are persisted: the raw signed catalog snapshot, and one
//! directory per installed pack release. Both are byte-for-byte what the
//! service published, so the trust anchor can re-reach the same verdict at any
//! later moment without asking the network again.
//!
//! # Installing is a rename
//!
//! A pack is expanded into `packs/.staging-<nonce>/`, every file is flushed to
//! the device, and only then is the staging directory renamed onto
//! `packs/<pack id>/<version>/`. Nothing partially written is ever reachable
//! under a pack's real name, so the failure modes collapse to two: the rename
//! happened, or it did not. A process killed mid-install therefore leaves a
//! `.staging-*` directory and nothing else, and [`ModelHubStore::sweep_staging`]
//! removes those at startup.
//!
//! # The installed layout
//!
//! ```text
//! packs/<pack id>/<version>/manifest.json        canonical signed manifest
//! packs/<pack id>/<version>/archive.rspicepack   the exact verified archive
//! packs/<pack id>/<version>/files/<path>         expanded model sources
//! ```
//!
//! The archive is kept rather than discarded because it is the evidence: with
//! it, [`ModelHubStore::verify_installed`] re-proves the whole pack under the
//! anchor whenever a caller wants proof rather than a record. Discovery itself
//! does the cheap half — parse the canonical manifest, match it to the
//! directory identity, digest the archive — so startup cost stays proportional
//! to the number of packs rather than to their size.
//!
//! # The serial floor outlives the catalog it came from
//!
//! A signed snapshot carries a `serial`, and a client that accepted serial 41
//! must never afterwards believe serial 40 — otherwise a service, or anything
//! that can answer as one, replays a catalog from before a release was recalled
//! and the recall is undone. The floor is therefore recorded *beside* the
//! cached snapshot rather than read out of it: replacing the cached catalog
//! wholesale, with anything at all, cannot lower it, and deleting the cache
//! does not reset it either. Only [`ModelHubStore::record_catalog_serial`]
//! moves it, and only upward.

use std::collections::BTreeMap;

use rspice_pack::{Manifest, Pack, VerifiedPack, parse_manifest, sha256_hex};

use super::{ModelHubError, trust::TrustAnchor};

/// The reserved prefix of an in-progress install.
///
/// A pack identifier is `rspice-<family>` in lowercase kebab case, so it can
/// never begin with a dot: staging directories and installed packs cannot
/// collide by construction.
pub(crate) const STAGING_PREFIX: &str = ".staging-";

const MANIFEST_FILE: &str = "manifest.json";
const ARCHIVE_FILE: &str = "archive.rspicepack";
const FILES_DIR: &str = "files";

/// The serial a store reports before it has ever accepted a catalog.
///
/// The format reserves it: `validate_snapshot` refuses a published serial of
/// zero precisely so a client has a sentinel that no authentic catalog can
/// collide with.
pub(crate) const NO_CATALOG_SERIAL: u64 = 0;

/// One pack release present on this machine.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InstalledPack {
    pub manifest: Manifest,
    /// Lowercase hexadecimal SHA-256 of the archive this release was
    /// installed from. This is the value a project pin records, so a pinned
    /// part identifies exactly one published release forever.
    pub archive_sha256: String,
}

impl InstalledPack {
    pub fn pack_id(&self) -> &str {
        &self.manifest.pack.id
    }

    pub fn version(&self) -> &str {
        &self.manifest.pack.version
    }
}

/// A verified pack expanded but not yet visible under its real name.
///
/// Dropping one without committing leaves the staging directory behind for
/// the sweep, which is the same state a killed process leaves; the pipeline
/// discards it explicitly so a recoverable failure does not wait for the next
/// startup to tidy up.
#[derive(Debug)]
pub struct StagedPack {
    pack_id: String,
    version: String,
    handle: StagingHandle,
}

impl StagedPack {
    /// Which release these staged bytes will be published as.
    ///
    /// A store that wraps another one needs this to keep its own record of the
    /// bytes beside the wrapped store's, and to drop that record when the
    /// stage is discarded rather than committed. The handle stays private:
    /// where the bytes are is the wrapped store's business, and *which release
    /// they are* is the only thing a wrapper has any claim on.
    pub(crate) fn pack_id(&self) -> &str {
        &self.pack_id
    }

    pub(crate) fn version(&self) -> &str {
        &self.version
    }
}

#[derive(Debug)]
enum StagingHandle {
    #[cfg(not(target_arch = "wasm32"))]
    Directory(std::path::PathBuf),
    Memory(String),
}

/// Durable storage for verified Model Hub bytes.
pub trait ModelHubStore: std::fmt::Debug {
    /// The cached raw snapshot blob, exactly as it was published.
    fn read_snapshot(&self) -> Result<Option<Vec<u8>>, ModelHubError>;

    /// Replaces the cached snapshot blob.
    fn write_snapshot(&self, bytes: &[u8]) -> Result<(), ModelHubError>;

    /// The highest catalog serial this store has ever accepted.
    ///
    /// [`NO_CATALOG_SERIAL`] means none has been, which is not the same as
    /// holding a catalog: a cache that failed verification leaves the floor
    /// standing, and that is the point — a rollback must stay refused even
    /// after the evidence for it was discarded.
    fn read_catalog_serial(&self) -> Result<u64, ModelHubError>;

    /// Raises the recorded floor to `serial`. It never lowers it.
    fn record_catalog_serial(&self, serial: u64) -> Result<(), ModelHubError>;

    /// Expands a verified pack into staging. No pack name is claimed yet.
    fn stage_pack(
        &self,
        verified: &VerifiedPack,
        archive: &[u8],
    ) -> Result<StagedPack, ModelHubError>;

    /// Publishes staged bytes under the pack's real name, atomically.
    fn commit_pack(&self, staged: StagedPack) -> Result<InstalledPack, ModelHubError>;

    /// Removes staged bytes without publishing them.
    fn discard_staged(&self, staged: StagedPack);

    /// Removes every abandoned staging directory. Returns how many.
    fn sweep_staging(&self) -> Result<usize, ModelHubError>;

    /// Every installed release, discovered from its on-disk manifest.
    fn installed_packs(&self) -> Result<Vec<InstalledPack>, ModelHubError>;

    /// Removes one installed release. Returns whether it was present.
    ///
    /// This touches nothing but the pack's own directory. Bytes a project
    /// retained from a pack live in the project, not here, so uninstalling
    /// can never break a saved design.
    fn remove_pack(&self, pack_id: &str, version: &str) -> Result<bool, ModelHubError>;

    /// Re-proves an installed release end to end under the anchor.
    fn verify_installed(
        &self,
        pack_id: &str,
        version: &str,
        anchor: &TrustAnchor,
    ) -> Result<VerifiedPack, ModelHubError>;
}

/// A shared store is a store.
///
/// The browser keeps its packs in memory, so a second [`ModelHub`] opened over
/// a *new* [`MemoryModelHubStore`] would see an empty machine. Sharing one
/// store behind an `Arc` is what lets a background task open a hub over the
/// same bytes the session already holds, without either side learning that the
/// other exists. A filesystem store needs none of this — its state is the
/// directory, and two handles to the same path already agree.
///
/// [`ModelHub`]: super::ModelHub
impl<T: ModelHubStore + ?Sized> ModelHubStore for std::sync::Arc<T> {
    fn read_snapshot(&self) -> Result<Option<Vec<u8>>, ModelHubError> {
        (**self).read_snapshot()
    }

    fn write_snapshot(&self, bytes: &[u8]) -> Result<(), ModelHubError> {
        (**self).write_snapshot(bytes)
    }

    fn read_catalog_serial(&self) -> Result<u64, ModelHubError> {
        (**self).read_catalog_serial()
    }

    fn record_catalog_serial(&self, serial: u64) -> Result<(), ModelHubError> {
        (**self).record_catalog_serial(serial)
    }

    fn stage_pack(
        &self,
        verified: &VerifiedPack,
        archive: &[u8],
    ) -> Result<StagedPack, ModelHubError> {
        (**self).stage_pack(verified, archive)
    }

    fn commit_pack(&self, staged: StagedPack) -> Result<InstalledPack, ModelHubError> {
        (**self).commit_pack(staged)
    }

    fn discard_staged(&self, staged: StagedPack) {
        (**self).discard_staged(staged);
    }

    fn sweep_staging(&self) -> Result<usize, ModelHubError> {
        (**self).sweep_staging()
    }

    fn installed_packs(&self) -> Result<Vec<InstalledPack>, ModelHubError> {
        (**self).installed_packs()
    }

    fn remove_pack(&self, pack_id: &str, version: &str) -> Result<bool, ModelHubError> {
        (**self).remove_pack(pack_id, version)
    }

    fn verify_installed(
        &self,
        pack_id: &str,
        version: &str,
        anchor: &TrustAnchor,
    ) -> Result<VerifiedPack, ModelHubError> {
        (**self).verify_installed(pack_id, version, anchor)
    }
}

/// Checks a manifest is the document its directory claims to hold.
fn require_identity(
    manifest: &Manifest,
    pack_id: &str,
    version: &str,
) -> Result<(), ModelHubError> {
    if manifest.pack.id != pack_id || manifest.pack.version != version {
        return Err(ModelHubError::IdentityMismatch {
            expected: format!("{pack_id}@{version}"),
            actual: format!("{}@{}", manifest.pack.id, manifest.pack.version),
        });
    }
    Ok(())
}

/// The in-memory store.
///
/// It is the browser build's store and every test's store. Nothing about the
/// pipeline changes with it in place: the same staging, the same rename
/// semantics, the same sweep. What it does not do is survive the process,
/// which is exactly the difference between a browser session and a desktop
/// installation today.
#[derive(Debug, Default)]
pub struct MemoryModelHubStore {
    state: std::sync::Mutex<MemoryState>,
}

#[derive(Debug, Default)]
struct MemoryState {
    snapshot: Option<Vec<u8>>,
    /// The highest catalog serial this store has accepted. Its own field
    /// rather than a reading of `snapshot`, for the reason the module header
    /// gives: replacing the cached catalog must not be able to lower it.
    catalog_serial: u64,
    /// Committed releases, keyed `<pack id>@<version>`.
    packs: BTreeMap<String, MemoryPack>,
    /// Staged releases, keyed by nonce.
    staging: BTreeMap<String, MemoryPack>,
    nonce: u64,
}

/// A committed or staged release, held as the two things the store is asked
/// for: the manifest it reports and the archive it re-proves. The expanded
/// sources are not kept — nothing reads bytes from a store without proving
/// them first, and a second copy of them here would be the unproved one.
#[derive(Debug, Clone)]
struct MemoryPack {
    manifest_bytes: Vec<u8>,
    archive: Vec<u8>,
}

impl MemoryModelHubStore {
    pub fn new() -> Self {
        Self::default()
    }

    fn locked(&self) -> Result<std::sync::MutexGuard<'_, MemoryState>, ModelHubError> {
        self.state
            .lock()
            .map_err(|_| ModelHubError::Storage("the model-hub store is poisoned".to_owned()))
    }
}

fn manifest_bytes_of(verified: &VerifiedPack) -> Result<Vec<u8>, ModelHubError> {
    rspice_pack::canonical_manifest_bytes(&verified.manifest).map_err(ModelHubError::from)
}

impl ModelHubStore for MemoryModelHubStore {
    fn read_snapshot(&self) -> Result<Option<Vec<u8>>, ModelHubError> {
        Ok(self.locked()?.snapshot.clone())
    }

    fn write_snapshot(&self, bytes: &[u8]) -> Result<(), ModelHubError> {
        self.locked()?.snapshot = Some(bytes.to_vec());
        Ok(())
    }

    fn read_catalog_serial(&self) -> Result<u64, ModelHubError> {
        Ok(self.locked()?.catalog_serial)
    }

    fn record_catalog_serial(&self, serial: u64) -> Result<(), ModelHubError> {
        let mut state = self.locked()?;
        state.catalog_serial = state.catalog_serial.max(serial);
        Ok(())
    }

    fn stage_pack(
        &self,
        verified: &VerifiedPack,
        archive: &[u8],
    ) -> Result<StagedPack, ModelHubError> {
        let staged = MemoryPack {
            manifest_bytes: manifest_bytes_of(verified)?,
            archive: archive.to_vec(),
        };
        let mut state = self.locked()?;
        state.nonce += 1;
        let nonce = format!("{STAGING_PREFIX}{}", state.nonce);
        state.staging.insert(nonce.clone(), staged);
        Ok(StagedPack {
            pack_id: verified.manifest.pack.id.clone(),
            version: verified.manifest.pack.version.clone(),
            handle: StagingHandle::Memory(nonce),
        })
    }

    fn commit_pack(&self, staged: StagedPack) -> Result<InstalledPack, ModelHubError> {
        // Exhaustive rather than a `let ... else`: the browser build compiles
        // only the memory variant, where a fallible binding would be dead.
        let nonce = match &staged.handle {
            StagingHandle::Memory(nonce) => nonce,
            #[cfg(not(target_arch = "wasm32"))]
            StagingHandle::Directory(_) => {
                return Err(ModelHubError::Storage(
                    "staged bytes belong to a different store".to_owned(),
                ));
            }
        };
        let mut state = self.locked()?;
        let pack = state
            .staging
            .remove(nonce.as_str())
            .ok_or_else(|| ModelHubError::Storage("staged bytes are gone".to_owned()))?;
        let manifest = parse_manifest(&pack.manifest_bytes)?;
        require_identity(&manifest, &staged.pack_id, &staged.version)?;
        let archive_sha256 = sha256_hex(&pack.archive);
        state
            .packs
            .insert(format!("{}@{}", staged.pack_id, staged.version), pack);
        Ok(InstalledPack {
            manifest,
            archive_sha256,
        })
    }

    fn discard_staged(&self, staged: StagedPack) {
        if let (StagingHandle::Memory(nonce), Ok(mut state)) = (&staged.handle, self.locked()) {
            state.staging.remove(nonce.as_str());
        }
    }

    fn sweep_staging(&self) -> Result<usize, ModelHubError> {
        let mut state = self.locked()?;
        let swept = state.staging.len();
        state.staging.clear();
        Ok(swept)
    }

    fn installed_packs(&self) -> Result<Vec<InstalledPack>, ModelHubError> {
        let state = self.locked()?;
        let mut installed = Vec::with_capacity(state.packs.len());
        for pack in state.packs.values() {
            installed.push(InstalledPack {
                manifest: parse_manifest(&pack.manifest_bytes)?,
                archive_sha256: sha256_hex(&pack.archive),
            });
        }
        Ok(installed)
    }

    fn remove_pack(&self, pack_id: &str, version: &str) -> Result<bool, ModelHubError> {
        Ok(self
            .locked()?
            .packs
            .remove(&format!("{pack_id}@{version}"))
            .is_some())
    }

    fn verify_installed(
        &self,
        pack_id: &str,
        version: &str,
        anchor: &TrustAnchor,
    ) -> Result<VerifiedPack, ModelHubError> {
        let archive = {
            let state = self.locked()?;
            state
                .packs
                .get(&format!("{pack_id}@{version}"))
                .map(|pack| pack.archive.clone())
                .ok_or_else(|| ModelHubError::NotInstalled {
                    pack_id: pack_id.to_owned(),
                    version: version.to_owned(),
                })?
        };
        let verified = Pack::verify(&archive, anchor.key(), anchor.limits())?;
        require_identity(&verified.manifest, pack_id, version)?;
        Ok(verified)
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub use native::FilesystemModelHubStore;

#[cfg(not(target_arch = "wasm32"))]
mod native {
    use std::{
        fs::{self, File},
        io::Write as _,
        path::{Path, PathBuf},
    };

    use rspice_pack::{Pack, VerifiedPack, parse_manifest, sha256_hex};

    use super::{
        ARCHIVE_FILE, FILES_DIR, InstalledPack, MANIFEST_FILE, ModelHubError, ModelHubStore,
        NO_CATALOG_SERIAL, STAGING_PREFIX, StagedPack, StagingHandle, TrustAnchor,
        manifest_bytes_of, require_identity,
    };

    /// Where a desktop installation keeps its Model Hub state.
    ///
    /// `<local data>/rspice/model-hub/`, beside the diagnostics directory this
    /// application already writes there. Local application data rather than
    /// configuration: these are re-downloadable bytes owned by the machine,
    /// not settings a user would migrate or back up, and a pack corpus is far
    /// too large for a configuration directory.
    pub(crate) fn default_model_hub_root() -> Option<PathBuf> {
        Some(
            dirs::data_local_dir()
                .or_else(dirs::config_dir)?
                .join("rspice")
                .join("model-hub"),
        )
    }

    /// The desktop store: real files, fsync before rename, rename to publish.
    #[derive(Debug, Clone)]
    pub struct FilesystemModelHubStore {
        root: PathBuf,
    }

    impl FilesystemModelHubStore {
        pub fn new(root: impl Into<PathBuf>) -> Self {
            Self { root: root.into() }
        }

        /// The store a shipped desktop build uses.
        pub fn production() -> Result<Self, ModelHubError> {
            default_model_hub_root().map(Self::new).ok_or_else(|| {
                ModelHubError::Storage(
                    "the operating system offered no local application-data directory".to_owned(),
                )
            })
        }

        fn snapshot_path(&self) -> PathBuf {
            self.root.join("catalog").join("snapshot.rspicecat")
        }

        /// The serial floor, beside the cached catalog rather than inside it.
        ///
        /// A separate file is the whole mechanism: `write_snapshot` replaces
        /// the catalog and never touches this, so no catalog — authentic,
        /// forged, truncated or absent — can lower the floor by arriving.
        fn serial_path(&self) -> PathBuf {
            self.root.join("catalog").join("accepted-serial")
        }

        /// Where installed releases are expanded.
        ///
        /// A hub needs this to hand a project an importable path to a pack's
        /// sources, so it is part of the store's contract rather than an
        /// internal detail.
        pub fn pack_root(&self) -> PathBuf {
            self.packs_root()
        }

        fn packs_root(&self) -> PathBuf {
            self.root.join("packs")
        }

        fn release_dir(&self, pack_id: &str, version: &str) -> PathBuf {
            self.packs_root().join(pack_id).join(version)
        }
    }

    fn storage_error(context: &str, error: &std::io::Error) -> ModelHubError {
        ModelHubError::Storage(format!("{context}: {error}"))
    }

    /// Writes a file and flushes it to the device.
    fn write_durable(path: &Path, bytes: &[u8]) -> Result<(), ModelHubError> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .map_err(|error| storage_error(&format!("create {}", parent.display()), &error))?;
        }
        let mut file = File::create(path)
            .map_err(|error| storage_error(&format!("create {}", path.display()), &error))?;
        file.write_all(bytes)
            .map_err(|error| storage_error(&format!("write {}", path.display()), &error))?;
        file.sync_all()
            .map_err(|error| storage_error(&format!("flush {}", path.display()), &error))?;
        Ok(())
    }

    /// Flushes a directory entry so a rename survives a power loss.
    ///
    /// Windows has no directory handle to synchronize; NTFS orders metadata
    /// through its own journal, so the call is a no-op there rather than a
    /// missing guarantee.
    fn sync_dir(path: &Path) -> Result<(), ModelHubError> {
        #[cfg(windows)]
        {
            let _ = path;
            Ok(())
        }
        #[cfg(not(windows))]
        {
            File::open(path)
                .and_then(|handle| handle.sync_all())
                .map_err(|error| storage_error(&format!("flush {}", path.display()), &error))
        }
    }

    /// A staging name no concurrent install can collide with.
    fn staging_name() -> String {
        let mut nonce = [0_u8; 16];
        // A failure here would only weaken uniqueness, never correctness: the
        // directory is created exclusively below, so a collision is refused
        // rather than silently shared.
        if getrandom::fill(&mut nonce).is_err() {
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|elapsed| elapsed.as_nanos())
                .unwrap_or_default();
            nonce[..16].copy_from_slice(&now.to_le_bytes());
        }
        format!("{STAGING_PREFIX}{}", rspice_pack::encode_hex(&nonce))
    }

    impl ModelHubStore for FilesystemModelHubStore {
        fn read_snapshot(&self) -> Result<Option<Vec<u8>>, ModelHubError> {
            match fs::read(self.snapshot_path()) {
                Ok(bytes) => Ok(Some(bytes)),
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(None),
                Err(error) => Err(storage_error("read the cached catalog", &error)),
            }
        }

        fn write_snapshot(&self, bytes: &[u8]) -> Result<(), ModelHubError> {
            let path = self.snapshot_path();
            let staging = path.with_extension("rspicecat-incoming");
            write_durable(&staging, bytes)?;
            fs::rename(&staging, &path)
                .map_err(|error| storage_error("publish the cached catalog", &error))?;
            if let Some(parent) = path.parent() {
                sync_dir(parent)?;
            }
            Ok(())
        }

        /// Reads the recorded floor.
        ///
        /// A file that is missing, unreadable, or does not hold a decimal
        /// integer answers [`NO_CATALOG_SERIAL`] rather than failing: a
        /// damaged floor must not stop a machine fetching a catalog, and the
        /// floor is re-established by the first snapshot it accepts. The one
        /// thing that must never happen — believing a *lower* serial than one
        /// already accepted — needs the file to be intact, and a torn write is
        /// impossible because it is written whole and renamed into place.
        fn read_catalog_serial(&self) -> Result<u64, ModelHubError> {
            Ok(fs::read_to_string(self.serial_path())
                .ok()
                .and_then(|text| text.trim().parse::<u64>().ok())
                .unwrap_or(NO_CATALOG_SERIAL))
        }

        fn record_catalog_serial(&self, serial: u64) -> Result<(), ModelHubError> {
            let raised = self.read_catalog_serial()?.max(serial);
            let path = self.serial_path();
            let staging = path.with_extension("incoming");
            write_durable(&staging, raised.to_string().as_bytes())?;
            fs::rename(&staging, &path)
                .map_err(|error| storage_error("publish the accepted catalog serial", &error))?;
            if let Some(parent) = path.parent() {
                sync_dir(parent)?;
            }
            Ok(())
        }

        fn stage_pack(
            &self,
            verified: &VerifiedPack,
            archive: &[u8],
        ) -> Result<StagedPack, ModelHubError> {
            let packs = self.packs_root();
            fs::create_dir_all(&packs)
                .map_err(|error| storage_error("create the pack root", &error))?;
            let staging = packs.join(staging_name());
            fs::create_dir(&staging)
                .map_err(|error| storage_error("create a staging directory", &error))?;

            let staged = StagedPack {
                pack_id: verified.manifest.pack.id.clone(),
                version: verified.manifest.pack.version.clone(),
                handle: StagingHandle::Directory(staging.clone()),
            };
            // Every fallible step below leaves the staging directory behind
            // for the caller to discard, and nothing under a real pack name.
            let write = (|| -> Result<(), ModelHubError> {
                write_durable(&staging.join(MANIFEST_FILE), &manifest_bytes_of(verified)?)?;
                write_durable(&staging.join(ARCHIVE_FILE), archive)?;
                for (path, content) in &verified.files {
                    // The pack format has already proved every path is
                    // relative, forward-slashed, and free of traversal, so
                    // the segments are joined rather than re-sanitized here.
                    let mut target = staging.join(FILES_DIR);
                    for segment in path.split('/') {
                        target.push(segment);
                    }
                    write_durable(&target, content)?;
                }
                sync_dir(&staging)
            })();
            match write {
                Ok(()) => Ok(staged),
                Err(error) => {
                    self.discard_staged(staged);
                    Err(error)
                }
            }
        }

        fn commit_pack(&self, staged: StagedPack) -> Result<InstalledPack, ModelHubError> {
            let StagingHandle::Directory(staging) = &staged.handle else {
                return Err(ModelHubError::Storage(
                    "staged bytes belong to a different store".to_owned(),
                ));
            };
            let target = self.release_dir(&staged.pack_id, &staged.version);
            let Some(parent) = target.parent() else {
                return Err(ModelHubError::Storage(
                    "the pack directory has no parent".to_owned(),
                ));
            };
            fs::create_dir_all(parent)
                .map_err(|error| storage_error("create the pack directory", &error))?;
            if target.exists() {
                // An installed release is immutable, so a second install of
                // the same version is already satisfied. Refuse to overwrite
                // it and drop the staged copy.
                let installed = read_installed(&target, &staged.pack_id, &staged.version);
                let _ = fs::remove_dir_all(staging);
                return installed;
            }
            fs::rename(staging, &target)
                .map_err(|error| storage_error("publish the staged pack", &error))?;
            sync_dir(parent)?;
            read_installed(&target, &staged.pack_id, &staged.version)
        }

        fn discard_staged(&self, staged: StagedPack) {
            if let StagingHandle::Directory(staging) = &staged.handle {
                let _ = fs::remove_dir_all(staging);
            }
        }

        fn sweep_staging(&self) -> Result<usize, ModelHubError> {
            let packs = self.packs_root();
            let entries = match fs::read_dir(&packs) {
                Ok(entries) => entries,
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(0),
                Err(error) => return Err(storage_error("read the pack root", &error)),
            };
            let mut swept = 0;
            for entry in entries.flatten() {
                let name = entry.file_name();
                let Some(name) = name.to_str() else {
                    continue;
                };
                if name.starts_with(STAGING_PREFIX) {
                    fs::remove_dir_all(entry.path()).map_err(|error| {
                        storage_error("remove an abandoned staging directory", &error)
                    })?;
                    swept += 1;
                }
            }
            Ok(swept)
        }

        fn installed_packs(&self) -> Result<Vec<InstalledPack>, ModelHubError> {
            let packs = self.packs_root();
            let entries = match fs::read_dir(&packs) {
                Ok(entries) => entries,
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
                Err(error) => return Err(storage_error("read the pack root", &error)),
            };
            let mut installed = Vec::new();
            for pack_entry in entries.flatten() {
                let pack_name = pack_entry.file_name();
                let Some(pack_id) = pack_name.to_str() else {
                    continue;
                };
                if pack_id.starts_with(STAGING_PREFIX) {
                    continue;
                }
                let Ok(versions) = fs::read_dir(pack_entry.path()) else {
                    continue;
                };
                for version_entry in versions.flatten() {
                    let version_name = version_entry.file_name();
                    let Some(version) = version_name.to_str() else {
                        continue;
                    };
                    // A directory that does not hold a manifest matching its
                    // own name is not an installation; skipping it keeps one
                    // damaged release from hiding every other one.
                    if let Ok(pack) = read_installed(&version_entry.path(), pack_id, version) {
                        installed.push(pack);
                    }
                }
            }
            Ok(installed)
        }

        fn remove_pack(&self, pack_id: &str, version: &str) -> Result<bool, ModelHubError> {
            let target = self.release_dir(pack_id, version);
            if !target.exists() {
                return Ok(false);
            }
            fs::remove_dir_all(&target)
                .map_err(|error| storage_error("remove an installed pack", &error))?;
            // An empty pack directory is not an installation; leaving it would
            // make the pack look present to a directory listing.
            let pack_root = self.packs_root().join(pack_id);
            if fs::read_dir(&pack_root).is_ok_and(|mut entries| entries.next().is_none()) {
                let _ = fs::remove_dir(&pack_root);
            }
            Ok(true)
        }

        fn verify_installed(
            &self,
            pack_id: &str,
            version: &str,
            anchor: &TrustAnchor,
        ) -> Result<VerifiedPack, ModelHubError> {
            let archive =
                fs::read(self.release_dir(pack_id, version).join(ARCHIVE_FILE)).map_err(|_| {
                    ModelHubError::NotInstalled {
                        pack_id: pack_id.to_owned(),
                        version: version.to_owned(),
                    }
                })?;
            let verified = Pack::verify(&archive, anchor.key(), anchor.limits())?;
            require_identity(&verified.manifest, pack_id, version)?;
            Ok(verified)
        }
    }

    /// Reads one installed release from its directory.
    fn read_installed(
        directory: &Path,
        pack_id: &str,
        version: &str,
    ) -> Result<InstalledPack, ModelHubError> {
        let manifest_bytes =
            fs::read(directory.join(MANIFEST_FILE)).map_err(|_| ModelHubError::NotInstalled {
                pack_id: pack_id.to_owned(),
                version: version.to_owned(),
            })?;
        let manifest = parse_manifest(&manifest_bytes)?;
        require_identity(&manifest, pack_id, version)?;
        let archive =
            fs::read(directory.join(ARCHIVE_FILE)).map_err(|_| ModelHubError::NotInstalled {
                pack_id: pack_id.to_owned(),
                version: version.to_owned(),
            })?;
        Ok(InstalledPack {
            manifest,
            archive_sha256: sha256_hex(&archive),
        })
    }
}
