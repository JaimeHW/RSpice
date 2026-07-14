//! Durable native autosave checkpoint ownership and exact-byte identity.
//!
//! A checkpoint is published before its manifest. If the process stops in
//! between, the complete checkpoint remains discoverable as a conservative
//! legacy candidate. A per-checkpoint OS lease distinguishes a dead owner from
//! another running RSpice process and serializes destructive cleanup.

use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::Read;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, OnceLock};
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};
use sha2::{Digest as _, Sha256};
use uuid::Uuid;

use crate::product::ContentDigest;
use crate::state::SchematicState;

const CHECKPOINT_MANIFEST_SCHEMA_VERSION: u32 = 1;
const CHECKPOINT_GENERATION_MARKER: &str = ".generation-";
const GENERATION_RESERVATION_ATTEMPTS: usize = 16;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct CheckpointIdentity {
    pub(crate) content_digest: ContentDigest,
    pub(crate) byte_len: u64,
    pub(crate) modified_unix_ms: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum CheckpointOwnership {
    Managed {
        session_id: Uuid,
        process_id: u32,
    },
    /// A checkpoint written before ownership manifests were introduced (or a
    /// fully committed checkpoint whose manifest publication was interrupted).
    /// It may be opened non-destructively but is never deleted by Recovery.
    Legacy,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CheckpointBinding {
    pub(crate) identity: CheckpointIdentity,
    pub(crate) ownership: CheckpointOwnership,
    pub(crate) source_snapshot: Option<CheckpointIdentity>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SourceSnapshotRelation {
    Exact,
    Changed,
    Unrecorded,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct AutosaveRestoreCandidate {
    pub(crate) source: PathBuf,
    pub(crate) checkpoint: PathBuf,
    pub(crate) binding: CheckpointBinding,
}

impl AutosaveRestoreCandidate {
    pub(crate) fn can_discard(&self) -> bool {
        matches!(self.binding.ownership, CheckpointOwnership::Managed { .. })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum CheckpointInspection {
    /// Owned by this process or another process whose lease is still held.
    ActiveOwner,
    Candidate(CheckpointBinding),
    Unsafe {
        identity: Option<CheckpointIdentity>,
        message: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct CheckpointManifest {
    schema_version: u32,
    session_id: Uuid,
    process_id: u32,
    source_path_digest: ContentDigest,
    checkpoint_path_digest: ContentDigest,
    #[serde(default)]
    source_snapshot: Option<CheckpointIdentity>,
    checkpoint: CheckpointIdentity,
    committed_unix_ms: u64,
}

static PROCESS_SESSION_ID: OnceLock<Uuid> = OnceLock::new();
static WRITER_LEASES: OnceLock<Mutex<HashMap<PathBuf, File>>> = OnceLock::new();

fn process_session_id() -> Uuid {
    *PROCESS_SESSION_ID.get_or_init(Uuid::new_v4)
}

fn writer_leases() -> &'static Mutex<HashMap<PathBuf, File>> {
    WRITER_LEASES.get_or_init(|| Mutex::new(HashMap::new()))
}

fn writer_lease_key(checkpoint: &Path) -> Result<PathBuf, String> {
    resolved_path(checkpoint).map_err(|error| {
        format!(
            "checkpoint lease identity '{}' could not be resolved: {error}",
            checkpoint.display()
        )
    })
}

fn checkpoint_manifest_path(checkpoint: &Path) -> PathBuf {
    append_suffix(checkpoint, ".manifest.json")
}

fn checkpoint_lease_path(checkpoint: &Path) -> PathBuf {
    append_suffix(checkpoint, ".lease")
}

fn append_suffix(path: &Path, suffix: &str) -> PathBuf {
    let mut value = path.as_os_str().to_owned();
    value.push(suffix);
    PathBuf::from(value)
}

fn checkpoint_generation_path(source: &Path, generation_id: Uuid) -> PathBuf {
    append_suffix(
        &super::file_workflow::autosave_checkpoint_path(source),
        &format!("{CHECKPOINT_GENERATION_MARKER}{generation_id}"),
    )
}

/// Return every exact recovery generation associated with a source. The
/// unsuffixed path remains readable for checkpoints written by older builds;
/// current writers publish immutable UUID-named generations beside it.
pub(crate) fn checkpoint_paths_for_source(source: &Path) -> Vec<PathBuf> {
    let legacy = super::file_workflow::autosave_checkpoint_path(source);
    let mut checkpoints = Vec::new();
    if legacy.exists() {
        checkpoints.push(legacy.clone());
    }

    let parent = legacy
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));
    let Some(legacy_name) = legacy.file_name() else {
        return checkpoints;
    };
    let mut prefix = legacy_name.to_os_string();
    prefix.push(CHECKPOINT_GENERATION_MARKER);
    let Ok(entries) = std::fs::read_dir(parent) else {
        return checkpoints;
    };
    for entry in entries.flatten() {
        if let Some(generation_id) = generation_id_after_prefix(&entry.file_name(), &prefix) {
            checkpoints.push(checkpoint_generation_path(source, generation_id));
        }
    }
    checkpoints.sort();
    checkpoints.dedup();
    checkpoints
}

#[cfg(windows)]
fn generation_id_after_prefix(name: &std::ffi::OsStr, prefix: &std::ffi::OsStr) -> Option<Uuid> {
    use std::os::windows::ffi::OsStrExt as _;
    let name = name.encode_wide().collect::<Vec<_>>();
    let prefix = prefix.encode_wide().collect::<Vec<_>>();
    let suffix = name.strip_prefix(prefix.as_slice())?;
    Uuid::parse_str(&String::from_utf16(suffix).ok()?).ok()
}

#[cfg(unix)]
fn generation_id_after_prefix(name: &std::ffi::OsStr, prefix: &std::ffi::OsStr) -> Option<Uuid> {
    use std::os::unix::ffi::OsStrExt as _;
    let suffix = name.as_bytes().strip_prefix(prefix.as_bytes())?;
    Uuid::parse_str(std::str::from_utf8(suffix).ok()?).ok()
}

#[cfg(not(any(unix, windows)))]
fn generation_id_after_prefix(name: &std::ffi::OsStr, prefix: &std::ffi::OsStr) -> Option<Uuid> {
    let name = name.to_string_lossy();
    let prefix = prefix.to_string_lossy();
    Uuid::parse_str(name.strip_prefix(prefix.as_ref())?).ok()
}

fn resolved_path(path: &Path) -> std::io::Result<PathBuf> {
    if path.exists() {
        return std::fs::canonicalize(path);
    }
    let absolute = if path.is_absolute() {
        path.to_path_buf()
    } else {
        std::env::current_dir()?.join(path)
    };
    let parent = absolute.parent().unwrap_or_else(|| Path::new("."));
    let parent = std::fs::canonicalize(parent)?;
    Ok(match absolute.file_name() {
        Some(name) => parent.join(name),
        None => parent,
    })
}

fn path_digest(path: &Path) -> Result<ContentDigest, String> {
    let path = resolved_path(path).map_err(|error| {
        format!(
            "path identity '{}' could not be resolved: {error}",
            path.display()
        )
    })?;
    let mut hasher = Sha256::new();
    #[cfg(windows)]
    {
        use std::os::windows::ffi::OsStrExt as _;
        for unit in path.as_os_str().encode_wide() {
            hasher.update(unit.to_le_bytes());
        }
    }
    #[cfg(unix)]
    {
        use std::os::unix::ffi::OsStrExt as _;
        hasher.update(path.as_os_str().as_bytes());
    }
    #[cfg(not(any(unix, windows)))]
    hasher.update(path.as_os_str().to_string_lossy().as_bytes());
    Ok(ContentDigest::from_bytes(hasher.finalize().into()))
}

fn digest(bytes: &[u8]) -> ContentDigest {
    ContentDigest::from_bytes(Sha256::digest(bytes).into())
}

fn modified_unix_ms(metadata: &std::fs::Metadata) -> u64 {
    metadata
        .modified()
        .ok()
        .and_then(|modified| modified.duration_since(UNIX_EPOCH).ok())
        .and_then(|duration| duration.as_millis().try_into().ok())
        .unwrap_or(0)
}

fn read_exact_bytes(path: &Path) -> Result<(Vec<u8>, CheckpointIdentity), String> {
    let mut file = File::open(path).map_err(|error| {
        format!(
            "checkpoint '{}' could not be opened: {error}",
            path.display()
        )
    })?;
    let before = file.metadata().map_err(|error| {
        format!(
            "checkpoint metadata '{}' could not be read: {error}",
            path.display()
        )
    })?;
    if !before.is_file() {
        return Err(format!(
            "checkpoint '{}' is not a regular file",
            path.display()
        ));
    }
    let mut bytes = Vec::with_capacity(before.len().try_into().unwrap_or(0));
    file.read_to_end(&mut bytes)
        .map_err(|error| format!("checkpoint '{}' could not be read: {error}", path.display()))?;
    let after = file.metadata().map_err(|error| {
        format!(
            "checkpoint metadata '{}' could not be revalidated: {error}",
            path.display()
        )
    })?;
    if before.len() != after.len()
        || before.modified().ok() != after.modified().ok()
        || bytes.len() as u64 != after.len()
    {
        return Err(format!(
            "checkpoint '{}' changed while it was being read",
            path.display()
        ));
    }
    let identity = CheckpointIdentity {
        content_digest: digest(&bytes),
        byte_len: after.len(),
        modified_unix_ms: modified_unix_ms(&after),
    };
    Ok((bytes, identity))
}

fn serialize_checkpoint(source: &Path, schematic: &SchematicState) -> Result<Vec<u8>, String> {
    let mut file = crate::io::SchematicFile::new(schematic.clone());
    file.metadata.modified_at = Some(
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs(),
    );
    file.metadata.title = source
        .file_stem()
        .map(|name| name.to_string_lossy().to_string());
    let mut bytes = serde_json::to_vec_pretty(&file)
        .map_err(|error| format!("checkpoint serialization failed: {error}"))?;
    bytes.push(b'\n');
    Ok(bytes)
}

fn atomic_write(path: &Path, bytes: &[u8]) -> Result<(), String> {
    crate::io::durable_file::atomic_write_bytes(path, bytes).map_err(|error| {
        format!(
            "checkpoint '{}' could not be durably published: {error}",
            path.display()
        )
    })
}

fn open_and_lock_lease(checkpoint: &Path, create: bool) -> Result<File, String> {
    let lease_path = checkpoint_lease_path(checkpoint);
    let mut options = OpenOptions::new();
    options.read(true).write(true).create(create);
    let file = options.open(&lease_path).map_err(|error| {
        format!(
            "checkpoint lease '{}' could not be opened: {error}",
            lease_path.display()
        )
    })?;
    file.try_lock().map_err(|error| {
        format!(
            "checkpoint lease '{}' is unavailable: {error}",
            lease_path.display()
        )
    })?;
    Ok(file)
}

/// Reserve a never-before-published checkpoint name for one writer generation.
/// `create_new` on the lease is the namespace CAS: even a UUID collision cannot
/// cause an existing checkpoint, manifest, or lease to be replaced.
fn reserve_writer_generation(source: &Path) -> Result<PathBuf, String> {
    let base = super::file_workflow::autosave_checkpoint_path(source);
    let parent = base
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));
    std::fs::create_dir_all(parent).map_err(|error| {
        format!(
            "checkpoint directory '{}' could not be created: {error}",
            parent.display()
        )
    })?;

    for _ in 0..GENERATION_RESERVATION_ATTEMPTS {
        let checkpoint = checkpoint_generation_path(source, Uuid::new_v4());
        let manifest = checkpoint_manifest_path(&checkpoint);
        if checkpoint.exists() || manifest.exists() {
            continue;
        }

        let lease_path = checkpoint_lease_path(&checkpoint);
        let file = match OpenOptions::new()
            .read(true)
            .write(true)
            .create_new(true)
            .open(&lease_path)
        {
            Ok(file) => file,
            Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => continue,
            Err(error) => {
                return Err(format!(
                    "checkpoint lease '{}' could not be reserved: {error}",
                    lease_path.display()
                ));
            }
        };
        if let Err(error) = file.try_lock() {
            drop(file);
            let _ = std::fs::remove_file(&lease_path);
            return Err(format!(
                "checkpoint lease '{}' could not be locked: {error}",
                lease_path.display()
            ));
        }

        // A non-cooperating process could still race the UUID path after the
        // lease reservation. Never overwrite anything that appeared there.
        if checkpoint.exists() || manifest.exists() {
            let _ = file.unlock();
            drop(file);
            continue;
        }

        let key = writer_lease_key(&checkpoint)?;
        let mut leases = writer_leases()
            .lock()
            .map_err(|_| "checkpoint lease registry is poisoned".to_owned())?;
        if leases.contains_key(&key) {
            let _ = file.unlock();
            continue;
        }
        leases.insert(key, file);
        return Ok(checkpoint);
    }

    Err(format!(
        "a unique recovery generation for '{}' could not be reserved after {GENERATION_RESERVATION_ATTEMPTS} attempts; no existing recovery bytes were changed",
        source.display()
    ))
}

fn writer_lease_owned(checkpoint: &Path) -> bool {
    let Ok(key) = writer_lease_key(checkpoint) else {
        return false;
    };
    writer_leases()
        .lock()
        .is_ok_and(|leases| leases.contains_key(&key))
}

fn release_writer_lease(checkpoint: &Path) -> Result<(), String> {
    let key = writer_lease_key(checkpoint)?;
    let file = writer_leases()
        .lock()
        .map_err(|_| "checkpoint lease registry is poisoned".to_owned())?
        .remove(&key);
    if let Some(file) = file {
        file.unlock().map_err(|error| {
            format!(
                "checkpoint lease '{}' could not be released: {error}",
                checkpoint_lease_path(checkpoint).display()
            )
        })?;
    }
    Ok(())
}

struct RecoveryLease(File);

impl Drop for RecoveryLease {
    fn drop(&mut self) {
        let _ = self.0.unlock();
    }
}

fn acquire_recovery_lease(checkpoint: &Path) -> Result<RecoveryLease, String> {
    open_and_lock_lease(checkpoint, false).map(RecoveryLease)
}

fn lease_is_active(checkpoint: &Path) -> Result<bool, String> {
    if writer_lease_owned(checkpoint) {
        return Ok(true);
    }
    let lease_path = checkpoint_lease_path(checkpoint);
    if !lease_path.exists() {
        return Ok(false);
    }
    match open_and_lock_lease(checkpoint, false) {
        Ok(file) => {
            file.unlock().map_err(|error| {
                format!(
                    "checkpoint lease '{}' could not be released: {error}",
                    lease_path.display()
                )
            })?;
            Ok(false)
        }
        Err(error) if error.contains("unavailable") => Ok(true),
        Err(error) => Err(error),
    }
}

fn read_manifest(checkpoint: &Path) -> Result<Option<CheckpointManifest>, String> {
    let path = checkpoint_manifest_path(checkpoint);
    let bytes = match std::fs::read(&path) {
        Ok(bytes) => bytes,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(error) => {
            return Err(format!(
                "checkpoint manifest '{}' could not be read: {error}",
                path.display()
            ));
        }
    };
    serde_json::from_slice(&bytes).map(Some).map_err(|error| {
        format!(
            "checkpoint manifest '{}' is invalid: {error}",
            path.display()
        )
    })
}

fn validate_manifest_paths(
    manifest: &CheckpointManifest,
    source: &Path,
    checkpoint: &Path,
) -> Result<(), String> {
    if manifest.schema_version != CHECKPOINT_MANIFEST_SCHEMA_VERSION {
        return Err(format!(
            "unsupported checkpoint manifest schema {}; expected {}",
            manifest.schema_version, CHECKPOINT_MANIFEST_SCHEMA_VERSION
        ));
    }
    if manifest.source_path_digest != path_digest(source)?
        || manifest.checkpoint_path_digest != path_digest(checkpoint)?
    {
        return Err("checkpoint manifest path identity does not match its saved source".to_owned());
    }
    Ok(())
}

pub(crate) fn write_checkpoint(
    source: &Path,
    schematic: &SchematicState,
) -> Result<PathBuf, String> {
    let bytes = serialize_checkpoint(source, schematic)?;
    let source_snapshot = read_exact_bytes(source).ok().map(|(_, identity)| identity);
    let checkpoint = reserve_writer_generation(source)?;
    let result = (|| {
        atomic_write(&checkpoint, &bytes)?;
        let (_, identity) = read_exact_bytes(&checkpoint)?;
        if identity.content_digest != digest(&bytes) || identity.byte_len != bytes.len() as u64 {
            return Err(
                "committed checkpoint bytes do not match the serialized snapshot".to_owned(),
            );
        }

        let manifest = CheckpointManifest {
            schema_version: CHECKPOINT_MANIFEST_SCHEMA_VERSION,
            session_id: process_session_id(),
            process_id: std::process::id(),
            source_path_digest: path_digest(source)?,
            checkpoint_path_digest: path_digest(&checkpoint)?,
            source_snapshot,
            checkpoint: identity,
            committed_unix_ms: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis()
                .try_into()
                .unwrap_or(u64::MAX),
        };
        let mut manifest_bytes = serde_json::to_vec_pretty(&manifest)
            .map_err(|error| format!("checkpoint manifest serialization failed: {error}"))?;
        manifest_bytes.push(b'\n');
        atomic_write(&checkpoint_manifest_path(&checkpoint), &manifest_bytes)?;
        Ok(())
    })();

    if let Err(error) = result {
        // A fully published checkpoint remains as a conservative legacy
        // generation if manifest publication or its durability receipt fails.
        let _ = release_writer_lease(&checkpoint);
        return Err(error);
    }
    for error in retire_prior_owned_generations(source, &checkpoint) {
        log::warn!(
            "Committed a new autosave generation but retained an earlier owned generation: {error}"
        );
    }
    Ok(checkpoint)
}

fn retire_prior_owned_generations(source: &Path, current: &Path) -> Vec<String> {
    checkpoint_paths_for_source(source)
        .into_iter()
        .filter(|checkpoint| checkpoint != current && writer_lease_owned(checkpoint))
        .filter_map(|checkpoint| cleanup_owned_checkpoint(source, &checkpoint).err())
        .collect()
}

pub(crate) fn inspect_checkpoint(source: &Path, checkpoint: &Path) -> CheckpointInspection {
    let snapshot = read_exact_bytes(checkpoint);
    let (identity, read_error) = match snapshot {
        Ok((_, identity)) => (Some(identity), None),
        Err(error) => (None, Some(error)),
    };
    let manifest = match read_manifest(checkpoint) {
        Ok(manifest) => manifest,
        Err(error) => {
            return CheckpointInspection::Unsafe {
                identity,
                message: error,
            };
        }
    };

    let lease_active = match lease_is_active(checkpoint) {
        Ok(active) => active,
        Err(error) => {
            return CheckpointInspection::Unsafe {
                identity,
                message: error,
            };
        }
    };
    if manifest
        .as_ref()
        .is_some_and(|manifest| manifest.session_id == process_session_id())
        || lease_active
    {
        return CheckpointInspection::ActiveOwner;
    }
    let Some(identity) = identity else {
        return CheckpointInspection::Unsafe {
            identity: None,
            message: read_error.unwrap_or_else(|| "checkpoint bytes are unavailable".to_owned()),
        };
    };

    let Some(manifest) = manifest else {
        return CheckpointInspection::Candidate(CheckpointBinding {
            identity,
            ownership: CheckpointOwnership::Legacy,
            source_snapshot: None,
        });
    };
    if let Err(error) = validate_manifest_paths(&manifest, source, checkpoint) {
        return CheckpointInspection::Unsafe {
            identity: Some(identity),
            message: error,
        };
    }
    if manifest.checkpoint != identity {
        return CheckpointInspection::Unsafe {
            identity: Some(identity),
            message: "checkpoint bytes or metadata do not match the committed manifest".to_owned(),
        };
    }
    CheckpointInspection::Candidate(CheckpointBinding {
        identity,
        ownership: CheckpointOwnership::Managed {
            session_id: manifest.session_id,
            process_id: manifest.process_id,
        },
        source_snapshot: manifest.source_snapshot,
    })
}

pub(crate) fn autosave_restore_candidate(
    source: &Path,
) -> Result<Option<AutosaveRestoreCandidate>, String> {
    let mut candidates = Vec::new();
    let mut unsafe_messages = Vec::new();
    for checkpoint in checkpoint_paths_for_source(source) {
        match inspect_checkpoint(source, &checkpoint) {
            CheckpointInspection::ActiveOwner => {}
            CheckpointInspection::Candidate(binding) => candidates.push((checkpoint, binding)),
            CheckpointInspection::Unsafe { message, .. } => {
                unsafe_messages.push(format!("'{}': {message}", checkpoint.display()))
            }
        }
    }
    if candidates.is_empty() {
        if !unsafe_messages.is_empty() {
            return Err(format!(
                "autosave checkpoint was retained but cannot be offered for restore: {}",
                unsafe_messages.join("; ")
            ));
        }
        return Ok(None);
    }

    let (_, source_identity) = read_exact_bytes(source).map_err(|error| {
        format!(
            "saved source could not be verified before offering its autosave checkpoint: {error}"
        )
    })?;
    candidates
        .retain(|(_, binding)| should_offer_checkpoint_on_source_open(binding, &source_identity));
    candidates.sort_by(|(left_path, left), (right_path, right)| {
        right
            .identity
            .modified_unix_ms
            .cmp(&left.identity.modified_unix_ms)
            .then_with(|| right_path.cmp(left_path))
    });
    Ok(candidates
        .into_iter()
        .next()
        .map(|(checkpoint, binding)| AutosaveRestoreCandidate {
            source: source.to_path_buf(),
            checkpoint,
            binding,
        }))
}

fn source_snapshot_relation(
    binding: &CheckpointBinding,
    source_identity: &CheckpointIdentity,
) -> SourceSnapshotRelation {
    match binding.source_snapshot.as_ref() {
        Some(expected) if expected == source_identity => SourceSnapshotRelation::Exact,
        Some(_) => SourceSnapshotRelation::Changed,
        None => SourceSnapshotRelation::Unrecorded,
    }
}

/// Decide whether opening a saved source should be interrupted by its restore
/// prompt. A manifest-bound exact source always receives the prompt, including
/// coarse-filesystem timestamp ties. Changed and legacy/unrecorded sources only
/// receive it when the checkpoint is strictly newer; stale evidence remains
/// available in Recovery without repeatedly blocking ordinary file opens.
fn should_offer_checkpoint_on_source_open(
    binding: &CheckpointBinding,
    source_identity: &CheckpointIdentity,
) -> bool {
    match source_snapshot_relation(binding, source_identity) {
        SourceSnapshotRelation::Exact => true,
        SourceSnapshotRelation::Changed | SourceSnapshotRelation::Unrecorded => {
            binding.identity.modified_unix_ms > source_identity.modified_unix_ms
        }
    }
}

pub(crate) fn read_source_snapshot(
    source: &Path,
    binding: &CheckpointBinding,
) -> Result<(Vec<u8>, SourceSnapshotRelation), String> {
    let (bytes, identity) = read_exact_bytes(source)?;
    let relation = source_snapshot_relation(binding, &identity);
    Ok((bytes, relation))
}

fn validate_binding(
    source: &Path,
    checkpoint: &Path,
    binding: &CheckpointBinding,
) -> Result<Option<RecoveryLease>, String> {
    let lease = match binding.ownership {
        CheckpointOwnership::Managed {
            session_id,
            process_id,
        } => {
            let lease = acquire_recovery_lease(checkpoint).map_err(|error| {
                format!("checkpoint is owned by a running process or cannot be locked: {error}")
            })?;
            let manifest = read_manifest(checkpoint)?
                .ok_or_else(|| "managed checkpoint manifest is missing".to_owned())?;
            validate_manifest_paths(&manifest, source, checkpoint)?;
            if manifest.session_id != session_id || manifest.process_id != process_id {
                return Err("checkpoint ownership changed after it was listed".to_owned());
            }
            if manifest.checkpoint != binding.identity {
                return Err("checkpoint manifest changed after it was listed".to_owned());
            }
            if manifest.source_snapshot != binding.source_snapshot {
                return Err("saved-source snapshot identity changed after it was listed".to_owned());
            }
            Some(lease)
        }
        CheckpointOwnership::Legacy => None,
    };
    Ok(lease)
}

pub(crate) fn read_bound_checkpoint(
    source: &Path,
    checkpoint: &Path,
    binding: &CheckpointBinding,
) -> Result<Vec<u8>, String> {
    let _lease = validate_binding(source, checkpoint, binding)?;
    let (bytes, identity) = read_exact_bytes(checkpoint)?;
    if identity != binding.identity {
        return Err(
            "checkpoint changed after it was listed; refresh Recovery before continuing".to_owned(),
        );
    }
    Ok(bytes)
}

pub(crate) fn discard_bound_checkpoint(
    source: &Path,
    checkpoint: &Path,
    binding: &CheckpointBinding,
) -> Result<(), String> {
    if matches!(binding.ownership, CheckpointOwnership::Legacy) {
        return Err(
            "legacy checkpoint ownership cannot be proven; open it non-destructively or use explicit recovery maintenance or migration"
                .to_owned(),
        );
    }
    let lease = validate_binding(source, checkpoint, binding)?
        .ok_or_else(|| "managed checkpoint lease is unavailable".to_owned())?;
    let (_, identity) = read_exact_bytes(checkpoint)?;
    if identity != binding.identity {
        return Err(
            "checkpoint changed after confirmation opened; nothing was deleted—refresh Recovery and review the replacement checkpoint"
                .to_owned(),
        );
    }

    // Manifest first: a crash during cleanup leaves a discoverable legacy
    // checkpoint, never bytes hidden behind a stale ownership record.
    let manifest = checkpoint_manifest_path(checkpoint);
    std::fs::remove_file(&manifest).map_err(|error| {
        format!(
            "checkpoint manifest '{}' could not be removed: {error}",
            manifest.display()
        )
    })?;
    crate::io::durable_file::sync_parent_of(&manifest).map_err(|error| {
        format!(
            "checkpoint manifest retirement for '{}' could not be synchronized; checkpoint bytes were retained as recovery evidence: {error}",
            checkpoint.display()
        )
    })?;
    std::fs::remove_file(checkpoint).map_err(|error| {
        format!(
            "checkpoint '{}' could not be removed: {error}",
            checkpoint.display()
        )
    })?;
    crate::io::durable_file::sync_parent_of(checkpoint).map_err(|error| {
        format!(
            "checkpoint deletion for '{}' completed, but its directory durability receipt failed and deletion may be replayed after a crash: {error}",
            checkpoint.display()
        )
    })?;
    drop(lease);
    // Lease files are durable lock identities, not recovery data. Keeping the
    // empty file avoids an unlock/unlink race where another process could lock
    // a newly created inode while a peer still holds the old one.
    Ok(())
}

pub(crate) fn cleanup_checkpoint(source: &Path) -> Result<(), String> {
    let checkpoints = checkpoint_paths_for_source(source);
    let mut foreign_retained = false;
    let mut cleanup_errors = Vec::new();
    for checkpoint in checkpoints {
        if writer_lease_owned(&checkpoint) {
            if let Err(error) = cleanup_owned_checkpoint(source, &checkpoint) {
                cleanup_errors.push(error);
            }
        } else {
            foreign_retained = true;
        }
    }
    if !cleanup_errors.is_empty() {
        return Err(cleanup_errors.join("; "));
    }
    if foreign_retained {
        return Err(
            "a recovery checkpoint owned by another or earlier session was retained; review it in Recovery before discarding it"
                .to_owned(),
        );
    }
    Ok(())
}

fn cleanup_owned_checkpoint(source: &Path, checkpoint: &Path) -> Result<(), String> {
    let manifest = read_manifest(&checkpoint)?.ok_or_else(|| {
        "the current autosave checkpoint has no committed ownership manifest and was retained for explicit Recovery review"
            .to_owned()
    })?;
    validate_manifest_paths(&manifest, source, &checkpoint)?;
    if manifest.session_id != process_session_id() || manifest.process_id != std::process::id() {
        return Err(
            "a recovery checkpoint owned by another session was retained; review it in Recovery before discarding it"
                .to_owned(),
        );
    }
    let (_, current_identity) = read_exact_bytes(&checkpoint).map_err(|error| {
        format!(
            "the current autosave checkpoint could not be revalidated and was retained: {error}"
        )
    })?;
    if current_identity != manifest.checkpoint {
        return Err(
            "the current autosave checkpoint no longer matches its committed manifest and was retained for explicit Recovery review"
                .to_owned(),
        );
    }
    let manifest = checkpoint_manifest_path(&checkpoint);
    match std::fs::remove_file(&manifest) {
        Ok(()) => {}
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
        Err(error) => {
            return Err(format!(
                "checkpoint manifest '{}' could not be removed: {error}",
                manifest.display()
            ));
        }
    }
    crate::io::durable_file::sync_parent_of(&manifest).map_err(|error| {
        format!(
            "checkpoint manifest retirement for '{}' could not be synchronized; checkpoint bytes were retained as recovery evidence: {error}",
            checkpoint.display()
        )
    })?;
    match std::fs::remove_file(&checkpoint) {
        Ok(()) => {}
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
        Err(error) => {
            return Err(format!(
                "checkpoint '{}' could not be removed: {error}",
                checkpoint.display()
            ));
        }
    }
    let deletion_sync = crate::io::durable_file::sync_parent_of(checkpoint).map_err(|error| {
        format!(
            "checkpoint deletion for '{}' completed, but its directory durability receipt failed and deletion may be replayed after a crash: {error}",
            checkpoint.display()
        )
    });
    let release = release_writer_lease(&checkpoint);
    deletion_sync?;
    release
}

#[cfg(test)]
pub(crate) fn release_test_writer_lease(checkpoint: &Path) -> Result<(), String> {
    release_writer_lease(checkpoint)
}

#[cfg(test)]
fn orphan_test_checkpoint(checkpoint: &Path) -> Result<(), String> {
    release_writer_lease(checkpoint)?;
    let mut manifest = read_manifest(checkpoint)?
        .ok_or_else(|| "test checkpoint manifest is missing".to_owned())?;
    manifest.session_id = Uuid::from_u128(process_session_id().as_u128() ^ 1);
    manifest.process_id = u32::MAX;
    let mut bytes = serde_json::to_vec_pretty(&manifest)
        .map_err(|error| format!("test manifest serialization failed: {error}"))?;
    bytes.push(b'\n');
    atomic_write(&checkpoint_manifest_path(checkpoint), &bytes)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{Component, ComponentType, Point};

    #[test]
    fn current_session_checkpoint_is_hidden_and_cleanup_is_complete() {
        let root = unique_temp_dir("current-session");
        let source = root.join("design.rsch");
        crate::io::save_schematic(&SchematicState::default(), &source).expect("save source");

        let checkpoint = write_checkpoint(&source, &SchematicState::default())
            .expect("write managed checkpoint");
        assert_eq!(
            inspect_checkpoint(&source, &checkpoint),
            CheckpointInspection::ActiveOwner
        );
        assert!(checkpoint_manifest_path(&checkpoint).is_file());

        cleanup_checkpoint(&source).expect("clean managed checkpoint");
        assert!(!checkpoint.exists());
        assert!(!checkpoint_manifest_path(&checkpoint).exists());
        std::fs::remove_dir_all(root).expect("remove fixture");
    }

    #[test]
    fn repeated_autosaves_publish_new_generation_before_retiring_owned_predecessor() {
        let root = unique_temp_dir("generation-rotation");
        let source = root.join("design.rsch");
        crate::io::save_schematic(&SchematicState::default(), &source).expect("save source");

        let first =
            write_checkpoint(&source, &SchematicState::default()).expect("write first generation");
        let mut changed = SchematicState::default();
        changed
            .components
            .push(Component::new(1, ComponentType::Resistor, Point::new(4, 8)));
        let second = write_checkpoint(&source, &changed).expect("write successor generation");

        assert_ne!(
            first, second,
            "autosave publication must never replace in place"
        );
        assert!(
            !first.exists(),
            "owned predecessor should rotate after commit"
        );
        assert!(!checkpoint_manifest_path(&first).exists());
        assert!(second.exists());
        assert!(checkpoint_manifest_path(&second).exists());
        assert_eq!(checkpoint_paths_for_source(&source), vec![second.clone()]);

        cleanup_checkpoint(&source).expect("clean current generation");
        assert!(!second.exists());
        std::fs::remove_dir_all(root).expect("remove fixture");
    }

    #[test]
    fn autosave_never_overwrites_or_retires_preexisting_legacy_bytes() {
        let root = unique_temp_dir("legacy-preservation");
        let source = root.join("design.rsch");
        crate::io::save_schematic(&SchematicState::default(), &source).expect("save source");
        let legacy = super::super::file_workflow::autosave_checkpoint_path(&source);
        let legacy_bytes = serialize_checkpoint(&source, &SchematicState::default())
            .expect("serialize legacy checkpoint");
        atomic_write(&legacy, &legacy_bytes).expect("publish legacy checkpoint");

        let mut changed = SchematicState::default();
        changed.components.push(Component::new(
            1,
            ComponentType::Capacitor,
            Point::new(9, 3),
        ));
        let current = write_checkpoint(&source, &changed).expect("publish owned generation");

        assert_ne!(current, legacy);
        assert_eq!(
            std::fs::read(&legacy).expect("legacy bytes remain readable"),
            legacy_bytes
        );
        let cleanup = cleanup_checkpoint(&source)
            .expect_err("clean save must disclose retained foreign evidence");
        assert!(cleanup.contains("another or earlier session"));
        assert!(!current.exists(), "owned generation should be retired");
        assert!(legacy.exists(), "legacy recovery evidence must survive");

        std::fs::remove_dir_all(root).expect("remove fixture");
    }

    #[test]
    fn successor_autosave_preserves_a_released_managed_recovery_generation() {
        let root = unique_temp_dir("managed-preservation");
        let source = root.join("design.rsch");
        crate::io::save_schematic(&SchematicState::default(), &source).expect("save source");
        let retained = write_checkpoint(&source, &SchematicState::default())
            .expect("write retained generation");
        orphan_test_checkpoint(&retained).expect("simulate prior owner exit");
        let retained_bytes = std::fs::read(&retained).expect("read retained generation");

        let mut changed = SchematicState::default();
        changed
            .components
            .push(Component::new(1, ComponentType::Inductor, Point::new(6, 7)));
        let current = write_checkpoint(&source, &changed).expect("write current generation");

        assert_ne!(current, retained);
        assert_eq!(
            std::fs::read(&retained).expect("retained generation survives"),
            retained_bytes
        );
        assert!(checkpoint_manifest_path(&retained).exists());
        assert!(current.exists());

        let cleanup =
            cleanup_checkpoint(&source).expect_err("clean save must retain prior-session recovery");
        assert!(cleanup.contains("another or earlier session"));
        assert!(!current.exists());
        assert!(retained.exists());
        std::fs::remove_dir_all(root).expect("remove fixture");
    }

    #[test]
    fn cleanup_fails_closed_while_another_owner_holds_the_lease() {
        let root = unique_temp_dir("cleanup-race");
        let source = root.join("design.rsch");
        let checkpoint = super::super::file_workflow::autosave_checkpoint_path(&source);
        std::fs::write(&source, "saved source").expect("write source");
        std::fs::write(&checkpoint, "recovery bytes").expect("write checkpoint");
        let lease = open_and_lock_lease(&checkpoint, true).expect("hold competing lease");

        let error = cleanup_checkpoint(&source).expect_err("foreign lease must block cleanup");
        assert!(error.contains("another or earlier session"));
        assert!(checkpoint.exists(), "racing cleanup must preserve bytes");

        lease.unlock().expect("release competing lease");
        assert!(
            cleanup_checkpoint(&source).is_err(),
            "a dead foreign owner still requires explicit Recovery review"
        );
        assert!(checkpoint.exists());
        std::fs::remove_dir_all(root).expect("remove fixture");
    }

    #[test]
    fn legacy_checkpoint_is_openable_but_never_destructively_discarded() {
        let root = unique_temp_dir("legacy");
        let source = root.join("design.rsch");
        crate::io::save_schematic(&SchematicState::default(), &source).expect("save source");
        let checkpoint = super::super::file_workflow::autosave_checkpoint_path(&source);
        let bytes = serialize_checkpoint(&source, &SchematicState::default())
            .expect("serialize legacy checkpoint");
        atomic_write(&checkpoint, &bytes).expect("write legacy checkpoint");

        let CheckpointInspection::Candidate(binding) = inspect_checkpoint(&source, &checkpoint)
        else {
            panic!("legacy checkpoint should be a candidate")
        };
        assert!(matches!(binding.ownership, CheckpointOwnership::Legacy));
        assert_eq!(
            read_bound_checkpoint(&source, &checkpoint, &binding).expect("read exact legacy bytes"),
            bytes
        );
        assert!(discard_bound_checkpoint(&source, &checkpoint, &binding).is_err());
        assert!(checkpoint.exists());

        std::fs::remove_dir_all(root).expect("remove fixture");
    }

    #[test]
    fn bound_candidate_rejects_checkpoint_replaced_after_listing() {
        let root = unique_temp_dir("checkpoint-race");
        let source = root.join("design.rsch");
        crate::io::save_schematic(&SchematicState::default(), &source).expect("save source");
        let checkpoint = write_checkpoint(&source, &SchematicState::default())
            .expect("write managed checkpoint");
        orphan_test_checkpoint(&checkpoint).expect("simulate prior process exit");
        let CheckpointInspection::Candidate(binding) = inspect_checkpoint(&source, &checkpoint)
        else {
            panic!("released managed checkpoint should be recoverable")
        };

        let mut changed = SchematicState::default();
        changed.components.push(
            Component::new(1, ComponentType::Resistor, Point::new(10, 10))
                .with_name_value("R1", "2k"),
        );
        let replacement = serialize_checkpoint(&source, &changed).expect("serialize replacement");
        atomic_write(&checkpoint, &replacement).expect("replace checkpoint after listing");

        let open_error = read_bound_checkpoint(&source, &checkpoint, &binding)
            .expect_err("changed bytes must not open under stale binding");
        assert!(open_error.contains("changed") || open_error.contains("manifest"));
        let discard_error = discard_bound_checkpoint(&source, &checkpoint, &binding)
            .expect_err("changed bytes must not be deleted under stale confirmation");
        assert!(discard_error.contains("changed") || discard_error.contains("manifest"));
        assert!(
            checkpoint.exists(),
            "replacement checkpoint must remain intact"
        );

        std::fs::remove_dir_all(root).expect("remove fixture");
    }

    #[test]
    fn manifest_binds_the_saved_source_snapshot() {
        let root = unique_temp_dir("source-binding");
        let source = root.join("design.rsch");
        crate::io::save_schematic(&SchematicState::default(), &source).expect("save source");
        let checkpoint = write_checkpoint(&source, &SchematicState::default())
            .expect("write managed checkpoint");
        orphan_test_checkpoint(&checkpoint).expect("simulate prior process exit");
        let CheckpointInspection::Candidate(binding) = inspect_checkpoint(&source, &checkpoint)
        else {
            panic!("released checkpoint should be recoverable")
        };
        assert_eq!(
            read_source_snapshot(&source, &binding)
                .expect("read exact source")
                .1,
            SourceSnapshotRelation::Exact
        );

        let mut changed = SchematicState::default();
        changed.components.push(Component::new(
            1,
            ComponentType::Capacitor,
            Point::new(5, 5),
        ));
        crate::io::save_schematic(&changed, &source).expect("replace saved source");
        assert_eq!(
            read_source_snapshot(&source, &binding)
                .expect("read changed source")
                .1,
            SourceSnapshotRelation::Changed
        );

        std::fs::remove_dir_all(root).expect("remove fixture");
    }

    #[test]
    fn clean_save_never_retires_a_dead_foreign_session_checkpoint() {
        let root = unique_temp_dir("foreign-clean-save");
        let source = root.join("design.rsch");
        crate::io::save_schematic(&SchematicState::default(), &source).expect("save source");
        let checkpoint = write_checkpoint(&source, &SchematicState::default())
            .expect("write managed checkpoint");
        orphan_test_checkpoint(&checkpoint).expect("simulate writer crash");

        let mut stale_buffer = SchematicState::default();
        stale_buffer
            .components
            .push(Component::new(1, ComponentType::Inductor, Point::new(7, 7)));
        crate::io::save_schematic(&stale_buffer, &source).expect("save stale buffer");
        let error = cleanup_checkpoint(&source)
            .expect_err("clean save must retain foreign recovery evidence");
        assert!(error.contains("another or earlier session"));
        assert!(checkpoint.exists());
        assert!(checkpoint_manifest_path(&checkpoint).exists());

        std::fs::remove_dir_all(root).expect("remove fixture");
    }

    #[test]
    fn cleanup_rejects_bytes_replaced_under_the_current_writer_lease() {
        let root = unique_temp_dir("current-writer-replaced");
        let source = root.join("design.rsch");
        crate::io::save_schematic(&SchematicState::default(), &source).expect("save source");
        let checkpoint = write_checkpoint(&source, &SchematicState::default())
            .expect("write managed checkpoint");

        let mut replacement_state = SchematicState::default();
        replacement_state.components.push(Component::new(
            1,
            ComponentType::VoltageSource,
            Point::new(11, 9),
        ));
        let replacement =
            serialize_checkpoint(&source, &replacement_state).expect("serialize replacement");
        atomic_write(&checkpoint, &replacement).expect("replace checkpoint behind writer lease");

        let error = cleanup_checkpoint(&source)
            .expect_err("cleanup must bind deletion to the committed checkpoint identity");
        assert!(error.contains("no longer matches"));
        assert_eq!(
            std::fs::read(&checkpoint).expect("replacement remains readable"),
            replacement
        );
        assert!(checkpoint_manifest_path(&checkpoint).exists());

        release_test_writer_lease(&checkpoint).expect("release writer lease");
        std::fs::remove_dir_all(root).expect("remove fixture");
    }

    #[test]
    fn source_open_prompt_uses_binding_before_timestamp_fallback() {
        let source = CheckpointIdentity {
            content_digest: digest(b"saved"),
            byte_len: 5,
            modified_unix_ms: 100,
        };
        let checkpoint = CheckpointIdentity {
            content_digest: digest(b"autosave"),
            byte_len: 8,
            modified_unix_ms: 100,
        };
        let exact = CheckpointBinding {
            identity: checkpoint.clone(),
            ownership: CheckpointOwnership::Managed {
                session_id: Uuid::nil(),
                process_id: 1,
            },
            source_snapshot: Some(source.clone()),
        };
        assert!(
            should_offer_checkpoint_on_source_open(&exact, &source),
            "an exact manifest binding must survive coarse timestamp ties"
        );

        let changed = CheckpointBinding {
            source_snapshot: Some(CheckpointIdentity {
                content_digest: digest(b"older-saved"),
                byte_len: 11,
                modified_unix_ms: 90,
            }),
            ..exact.clone()
        };
        assert!(
            !should_offer_checkpoint_on_source_open(&changed, &source),
            "a changed source must not be interrupted by a tied checkpoint"
        );

        let mut newer = changed;
        newer.identity.modified_unix_ms = 101;
        assert!(should_offer_checkpoint_on_source_open(&newer, &source));

        let stale_legacy = CheckpointBinding {
            identity: checkpoint,
            ownership: CheckpointOwnership::Legacy,
            source_snapshot: None,
        };
        assert!(
            !should_offer_checkpoint_on_source_open(&stale_legacy, &source),
            "stale legacy evidence belongs in Recovery, not every file open"
        );
    }

    fn unique_temp_dir(label: &str) -> PathBuf {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock")
            .as_nanos();
        let path = std::env::temp_dir().join(format!(
            "rspice-checkpoint-{label}-{}-{nonce}",
            std::process::id()
        ));
        std::fs::create_dir_all(&path).expect("create fixture directory");
        path
    }
}
