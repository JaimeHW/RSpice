//! Full-project recovery checkpoints for governed project mutations.
//!
//! These artifacts are intentionally separate from schematic autosaves. A
//! checkpoint owns the exact validated `ProjectFile` bytes plus a small
//! integrity manifest. The snapshot is durably published before the manifest,
//! so an interrupted commit is never advertised as recoverable. Recovery
//! exports an independent project copy and never overwrites the live project.

#[cfg(not(target_arch = "wasm32"))]
use std::path::Path;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};
use sha2::{Digest as _, Sha256};
use uuid::Uuid;

use crate::common::app::AppState;
use crate::io::ProjectFile;
use crate::product::ContentDigest;

const MANIFEST_SCHEMA_VERSION: u16 = 1;
#[cfg(not(target_arch = "wasm32"))]
const CHECKPOINT_DIRECTORY: &str = ".rspice-recovery";
#[cfg(not(target_arch = "wasm32"))]
const MANIFEST_SUFFIX: &str = ".checkpoint.json";
const SNAPSHOT_SUFFIX: &str = ".rspiceproj";
const MAX_RETAINED_CHECKPOINTS: usize = 8;
#[cfg(not(target_arch = "wasm32"))]
const MAX_RETAINED_QUARANTINE_RECORDS: usize = 8;
#[cfg(target_arch = "wasm32")]
const BROWSER_KEY_PREFIX: &str = "rspice.project-recovery.v1";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum ProjectCheckpointReason {
    TechnologyAttachment,
}

impl ProjectCheckpointReason {
    pub(crate) const fn label(self) -> &'static str {
        match self {
            Self::TechnologyAttachment => "Before technology attachment",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct ProjectCheckpointManifest {
    schema_version: u16,
    checkpoint_id: Uuid,
    project_id: String,
    project_name: String,
    project_revision: u64,
    reason: ProjectCheckpointReason,
    created_unix_ms: u64,
    snapshot_digest: ContentDigest,
    snapshot_byte_len: u64,
    snapshot_name: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ProjectCheckpointLocator {
    #[cfg(not(target_arch = "wasm32"))]
    Native {
        manifest: PathBuf,
        snapshot: PathBuf,
    },
    #[cfg(target_arch = "wasm32")]
    Browser {
        manifest_key: String,
        snapshot_key: String,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ProjectCheckpointSummary {
    checkpoint_id: Uuid,
    project_id: String,
    project_name: String,
    project_revision: u64,
    reason: ProjectCheckpointReason,
    created_unix_ms: u64,
    snapshot_digest: ContentDigest,
    snapshot_byte_len: u64,
    locator: ProjectCheckpointLocator,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ProjectCheckpointQuarantine {
    label: String,
    reason: String,
    #[cfg(not(target_arch = "wasm32"))]
    artifacts: Vec<PathBuf>,
    #[cfg(target_arch = "wasm32")]
    artifact_keys: Vec<String>,
}

impl ProjectCheckpointQuarantine {
    pub(crate) fn label(&self) -> &str {
        &self.label
    }

    pub(crate) fn reason(&self) -> &str {
        &self.reason
    }

    pub(crate) fn artifact_count(&self) -> usize {
        #[cfg(not(target_arch = "wasm32"))]
        {
            self.artifacts.len()
        }
        #[cfg(target_arch = "wasm32")]
        {
            self.artifact_keys.len()
        }
    }
}

#[derive(Debug, Clone, Default)]
pub(crate) struct ProjectCheckpointCatalog {
    pub(crate) checkpoints: Vec<ProjectCheckpointSummary>,
    pub(crate) quarantined: Vec<ProjectCheckpointQuarantine>,
}

impl ProjectCheckpointSummary {
    pub(crate) const fn checkpoint_id(&self) -> Uuid {
        self.checkpoint_id
    }

    pub(crate) fn project_name(&self) -> &str {
        &self.project_name
    }

    pub(crate) const fn project_revision(&self) -> u64 {
        self.project_revision
    }

    pub(crate) const fn reason(&self) -> ProjectCheckpointReason {
        self.reason
    }

    pub(crate) const fn snapshot_byte_len(&self) -> u64 {
        self.snapshot_byte_len
    }
}

pub(crate) fn matches_current_state(
    summary: &ProjectCheckpointSummary,
    state: &AppState,
) -> Result<bool, String> {
    let project = crate::common::project_lifecycle::snapshot(state)
        .map_err(|error| format!("project checkpoint comparison failed: {error}"))?;
    project
        .validate()
        .map_err(|error| format!("current project is invalid: {error}"))?;
    let serialized = crate::io::project_io::serialize_project_file(&project)
        .map_err(|error| format!("current project comparison serialization failed: {error}"))?;
    Ok(digest(serialized.as_bytes()) == summary.snapshot_digest
        && serialized.len() as u64 == summary.snapshot_byte_len)
}

fn prepare_checkpoint(
    state: &AppState,
    reason: ProjectCheckpointReason,
) -> Result<(ProjectCheckpointManifest, String), String> {
    let project = crate::common::project_lifecycle::snapshot(state)
        .map_err(|error| format!("project checkpoint snapshot failed: {error}"))?;
    project
        .validate()
        .map_err(|error| format!("project checkpoint is invalid: {error}"))?;
    let serialized = crate::io::project_io::serialize_project_file(&project)
        .map_err(|error| format!("project checkpoint serialization failed: {error}"))?;
    let bytes = serialized.as_bytes();
    let created_unix_ms = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis()
        .try_into()
        .unwrap_or(u64::MAX);
    let project_id = project.workspace.project.id().to_string();
    let checkpoint_id = Uuid::new_v4();
    let snapshot_name = format!("{created_unix_ms}-{checkpoint_id}{SNAPSHOT_SUFFIX}");
    let manifest = ProjectCheckpointManifest {
        schema_version: MANIFEST_SCHEMA_VERSION,
        checkpoint_id,
        project_id: project_id.clone(),
        project_name: project.workspace.project.name().to_owned(),
        project_revision: project.workspace.project.revision().get(),
        reason,
        created_unix_ms,
        snapshot_digest: digest(bytes),
        snapshot_byte_len: bytes.len().try_into().unwrap_or(u64::MAX),
        snapshot_name,
    };

    Ok((manifest, serialized))
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn create(
    state: &AppState,
    reason: ProjectCheckpointReason,
) -> Result<ProjectCheckpointSummary, String> {
    let (manifest, serialized) = prepare_checkpoint(state, reason)?;
    let summary = publish_native(state, &manifest, serialized.as_bytes())?;
    // Retention runs only after the new manifest is committed and verified.
    // A cleanup failure never invalidates the newly created recovery point.
    if let Err(error) = prune_retention(state, &manifest.project_id) {
        log::warn!("Project checkpoint committed, but retention cleanup was incomplete: {error}");
    }
    Ok(summary)
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn start_create(
    state: &AppState,
    reason: ProjectCheckpointReason,
    complete: impl FnOnce(Result<ProjectCheckpointSummary, String>) + 'static,
) -> Result<(), String> {
    let (manifest, serialized) = prepare_checkpoint(state, reason)?;
    let (manifest_key, snapshot_key) = browser_keys(&manifest);
    let prefix = browser_project_prefix(&manifest.project_id);
    let manifest_json = serde_json::to_string(&manifest)
        .map_err(|error| format!("project checkpoint manifest serialization failed: {error}"))?;
    let manifest_for_completion = manifest.clone();
    let manifest_key_for_completion = manifest_key.clone();
    let snapshot_key_for_completion = snapshot_key.clone();
    crate::common::project_lifecycle::start_browser_checkpoint_publish(
        prefix,
        manifest_key,
        snapshot_key,
        serialized.into_bytes(),
        manifest_json,
        MAX_RETAINED_CHECKPOINTS,
        move |result| {
            complete(result.and_then(|()| {
                summary_from_manifest(
                    manifest_for_completion,
                    ProjectCheckpointLocator::Browser {
                        manifest_key: manifest_key_for_completion,
                        snapshot_key: snapshot_key_for_completion,
                    },
                )
            }));
        },
    );
    Ok(())
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn list(state: &AppState) -> Result<ProjectCheckpointCatalog, String> {
    let project_id = state.workspace.project.id().to_string();
    list_native(state, &project_id)
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn load(summary: &ProjectCheckpointSummary) -> Result<ProjectFile, String> {
    let bytes = read_snapshot_bytes(summary)?;
    validate_snapshot_bytes(summary, &bytes)
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn recovery_copy_bytes(
    summary: &ProjectCheckpointSummary,
    destination: &Path,
) -> Result<Vec<u8>, String> {
    let mut project = load(summary)?;
    project.workspace.project = project
        .workspace
        .project
        .fork_copy_at(destination.to_path_buf());
    let serialized = crate::io::project_io::serialize_project_file(&project)
        .map_err(|error| format!("recovery copy serialization failed: {error}"))?;
    Ok(serialized.into_bytes())
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn publish_recovery_copy(
    summary: &ProjectCheckpointSummary,
    destination: &Path,
) -> Result<(), String> {
    let bytes = recovery_copy_bytes(summary, destination)?;
    let expected = crate::io::durable_file::observe_expected_content(destination)
        .map_err(|error| format!("recovery destination could not be inspected: {error}"))?;
    if !matches!(expected, crate::io::durable_file::ExpectedContent::Missing) {
        return Err("recovery copies never overwrite an existing file".to_owned());
    }
    crate::io::durable_file::compare_exchange_bytes(destination, expected, &bytes)
        .map_err(|error| format!("recovery copy publication failed: {error}"))
}

fn validate_snapshot_bytes(
    summary: &ProjectCheckpointSummary,
    bytes: &[u8],
) -> Result<ProjectFile, String> {
    if bytes.len() as u64 != summary.snapshot_byte_len || digest(bytes) != summary.snapshot_digest {
        return Err("project checkpoint bytes do not match their integrity manifest".to_owned());
    }
    let text = std::str::from_utf8(bytes)
        .map_err(|error| format!("project checkpoint is not valid UTF-8: {error}"))?;
    let project = crate::io::project_io::load_project_text(text, None)
        .map_err(|error| format!("project checkpoint cannot be restored: {error}"))?;
    if project.workspace.project.id().to_string() != summary.project_id
        || project.workspace.project.revision().get() != summary.project_revision
        || project.workspace.project.name() != summary.project_name
    {
        return Err("project checkpoint identity does not match its manifest".to_owned());
    }
    Ok(project)
}

fn digest(bytes: &[u8]) -> ContentDigest {
    ContentDigest::from_bytes(Sha256::digest(bytes).into())
}

fn validate_manifest(manifest: &ProjectCheckpointManifest) -> Result<(), String> {
    if manifest.schema_version != MANIFEST_SCHEMA_VERSION {
        return Err(format!(
            "unsupported project checkpoint schema {}",
            manifest.schema_version
        ));
    }
    if manifest.checkpoint_id.is_nil() {
        return Err("project checkpoint identity must not be nil".to_owned());
    }
    if Uuid::parse_str(&manifest.project_id).is_err()
        || Uuid::parse_str(&manifest.project_id).is_ok_and(|identity| identity.is_nil())
    {
        return Err("project checkpoint has an invalid project identity".to_owned());
    }
    crate::state::ProjectDescriptor::validate_name(&manifest.project_name)
        .map_err(|error| format!("project checkpoint has an invalid project name: {error}"))?;
    if manifest.project_revision == 0 {
        return Err("project checkpoint has an invalid zero revision".to_owned());
    }
    if manifest.created_unix_ms == 0 {
        return Err("project checkpoint has an invalid creation time".to_owned());
    }
    if manifest.snapshot_byte_len == 0 {
        return Err("project checkpoint snapshot is empty".to_owned());
    }
    if manifest.snapshot_byte_len > crate::io::project_io::MAX_PROJECT_FILE_BYTES {
        return Err(
            "project checkpoint snapshot exceeds the supported project-size limit".to_owned(),
        );
    }
    let expected_name = format!(
        "{}-{}{}",
        manifest.created_unix_ms, manifest.checkpoint_id, SNAPSHOT_SUFFIX
    );
    if manifest.snapshot_name != expected_name {
        return Err("project checkpoint snapshot name is not canonical".to_owned());
    }
    Ok(())
}

fn summary_from_manifest(
    manifest: ProjectCheckpointManifest,
    locator: ProjectCheckpointLocator,
) -> Result<ProjectCheckpointSummary, String> {
    validate_manifest(&manifest)?;
    Ok(ProjectCheckpointSummary {
        checkpoint_id: manifest.checkpoint_id,
        project_id: manifest.project_id,
        project_name: manifest.project_name,
        project_revision: manifest.project_revision,
        reason: manifest.reason,
        created_unix_ms: manifest.created_unix_ms,
        snapshot_digest: manifest.snapshot_digest,
        snapshot_byte_len: manifest.snapshot_byte_len,
        locator,
    })
}

fn sort_newest_first(checkpoints: &mut [ProjectCheckpointSummary]) {
    checkpoints.sort_by(|left, right| {
        right
            .created_unix_ms
            .cmp(&left.created_unix_ms)
            .then_with(|| right.checkpoint_id.cmp(&left.checkpoint_id))
    });
}

#[cfg(not(target_arch = "wasm32"))]
fn checkpoint_directory(state: &AppState, project_id: &str) -> Result<PathBuf, String> {
    let _ = project_id;
    let base = state
        .workspace
        .project
        .path
        .as_deref()
        .and_then(Path::parent)
        .filter(|path| !path.as_os_str().is_empty())
        .map(Path::to_path_buf)
        .or_else(|| dirs::data_local_dir().map(|path| path.join("RSpice")))
        .ok_or_else(|| "no durable local recovery directory is available".to_owned())?;
    let base = if base.is_absolute() {
        base
    } else {
        std::env::current_dir()
            .map_err(|error| format!("current directory is unavailable: {error}"))?
            .join(base)
    };
    // Keep the publication parent shallow. Durable publication adds a hashed
    // lease and UUID staging suffix; nesting the project UUID here would push
    // otherwise ordinary Windows user-profile paths past legacy MAX_PATH.
    // Project identity remains in every authenticated manifest and catalog
    // filtering, so checkpoints still cannot cross project ownership.
    Ok(base.join(CHECKPOINT_DIRECTORY))
}

#[cfg(not(target_arch = "wasm32"))]
fn publish_native(
    state: &AppState,
    manifest: &ProjectCheckpointManifest,
    bytes: &[u8],
) -> Result<ProjectCheckpointSummary, String> {
    let directory = checkpoint_directory(state, &manifest.project_id)?;
    std::fs::create_dir_all(&directory).map_err(|error| {
        format!(
            "project checkpoint directory '{}' could not be created: {error}",
            directory.display()
        )
    })?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt as _;
        std::fs::set_permissions(&directory, std::fs::Permissions::from_mode(0o700)).map_err(
            |error| {
                format!(
                    "project checkpoint directory '{}' could not be made private: {error}",
                    directory.display()
                )
            },
        )?;
    }
    let snapshot = directory.join(&manifest.snapshot_name);
    let manifest_path = directory.join(format!("{}{}", manifest.checkpoint_id, MANIFEST_SUFFIX));
    if snapshot.exists() || manifest_path.exists() {
        return Err("project checkpoint identity collision; no state was changed".to_owned());
    }

    publish_private_file(&snapshot, bytes)
        .map_err(|error| format!("project checkpoint snapshot publication failed: {error}"))?;
    let observed = std::fs::read(&snapshot)
        .map_err(|error| format!("project checkpoint snapshot verification failed: {error}"))?;
    if observed.len() as u64 != manifest.snapshot_byte_len
        || digest(&observed) != manifest.snapshot_digest
    {
        return Err("published project checkpoint failed exact-byte verification".to_owned());
    }
    let mut manifest_bytes = serde_json::to_vec_pretty(manifest)
        .map_err(|error| format!("project checkpoint manifest serialization failed: {error}"))?;
    manifest_bytes.push(b'\n');
    publish_private_file(&manifest_path, &manifest_bytes)
        .map_err(|error| format!("project checkpoint manifest publication failed: {error}"))?;
    crate::io::durable_file::sync_parent_of(&manifest_path)
        .map_err(|error| format!("project checkpoint directory sync failed: {error}"))?;

    summary_from_manifest(
        manifest.clone(),
        ProjectCheckpointLocator::Native {
            manifest: manifest_path,
            snapshot,
        },
    )
}

#[cfg(not(target_arch = "wasm32"))]
fn publish_private_file(
    path: &Path,
    bytes: &[u8],
) -> Result<(), crate::io::durable_file::CompareExchangeError> {
    #[cfg(unix)]
    {
        crate::io::durable_file::compare_exchange_bytes_owner_only(
            path,
            crate::io::durable_file::ExpectedContent::Missing,
            bytes,
        )
    }
    #[cfg(not(unix))]
    {
        crate::io::durable_file::compare_exchange_bytes(
            path,
            crate::io::durable_file::ExpectedContent::Missing,
            bytes,
        )
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn list_native(state: &AppState, project_id: &str) -> Result<ProjectCheckpointCatalog, String> {
    let directory = checkpoint_directory(state, project_id)?;
    let entries = match std::fs::read_dir(&directory) {
        Ok(entries) => entries,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            return Ok(ProjectCheckpointCatalog::default());
        }
        Err(error) => {
            return Err(format!(
                "project recovery directory '{}' could not be read: {error}",
                directory.display()
            ));
        }
    };
    let mut checkpoints = Vec::new();
    let mut quarantined = Vec::new();
    for entry in entries.flatten() {
        let path = entry.path();
        if !path
            .file_name()
            .and_then(|name| name.to_str())
            .is_some_and(|name| name.ends_with(MANIFEST_SUFFIX))
        {
            continue;
        }
        let bytes = match std::fs::read(&path) {
            Ok(bytes) => bytes,
            Err(error) => {
                log::warn!(
                    "Could not read project checkpoint manifest '{}': {error}",
                    path.display()
                );
                continue;
            }
        };
        let manifest = match serde_json::from_slice::<ProjectCheckpointManifest>(&bytes) {
            Ok(manifest) if manifest.project_id == project_id => manifest,
            Ok(_) => continue,
            Err(error) => {
                log::warn!(
                    "Ignored invalid project checkpoint manifest '{}': {error}",
                    path.display()
                );
                continue;
            }
        };
        let snapshot = directory.join(&manifest.snapshot_name);
        match summary_from_manifest(
            manifest,
            ProjectCheckpointLocator::Native {
                manifest: path.clone(),
                snapshot,
            },
        ) {
            Ok(summary) => match load(&summary) {
                Ok(_) => checkpoints.push(summary),
                Err(error) => {
                    let ProjectCheckpointLocator::Native { manifest, snapshot } = &summary.locator;
                    quarantined.push(ProjectCheckpointQuarantine {
                        label: summary.checkpoint_id.to_string(),
                        reason: error,
                        artifacts: vec![manifest.clone(), snapshot.clone()],
                    });
                }
            },
            Err(error) => quarantined.push(ProjectCheckpointQuarantine {
                label: path
                    .file_name()
                    .map(|name| name.to_string_lossy().into_owned())
                    .unwrap_or_else(|| "checkpoint manifest".to_owned()),
                reason: error,
                artifacts: vec![path],
            }),
        }
    }
    sort_newest_first(&mut checkpoints);
    quarantined.sort_by(|left, right| right.label.cmp(&left.label));
    Ok(ProjectCheckpointCatalog {
        checkpoints,
        quarantined,
    })
}

#[cfg(not(target_arch = "wasm32"))]
fn read_snapshot_bytes(summary: &ProjectCheckpointSummary) -> Result<Vec<u8>, String> {
    let ProjectCheckpointLocator::Native { snapshot, .. } = &summary.locator;
    std::fs::read(snapshot).map_err(|error| {
        format!(
            "project checkpoint snapshot '{}' could not be read: {error}",
            snapshot.display()
        )
    })
}

#[cfg(not(target_arch = "wasm32"))]
fn prune_retention(state: &AppState, project_id: &str) -> Result<(), String> {
    let catalog = list_native(state, project_id)?;
    let mut failures = Vec::new();
    for checkpoint in catalog
        .checkpoints
        .into_iter()
        .skip(MAX_RETAINED_CHECKPOINTS)
    {
        let ProjectCheckpointLocator::Native { manifest, snapshot } = checkpoint.locator;
        for path in [manifest, snapshot] {
            match std::fs::remove_file(&path) {
                Ok(()) => {}
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
                Err(error) => failures.push(format!("'{}': {error}", path.display())),
            }
        }
    }
    for quarantine in catalog
        .quarantined
        .into_iter()
        .skip(MAX_RETAINED_QUARANTINE_RECORDS)
    {
        for path in quarantine.artifacts {
            match std::fs::remove_file(&path) {
                Ok(()) => {}
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
                Err(error) => failures.push(format!("'{}': {error}", path.display())),
            }
        }
    }
    if failures.is_empty() {
        Ok(())
    } else {
        Err(failures.join("; "))
    }
}

#[cfg(target_arch = "wasm32")]
fn browser_keys(manifest: &ProjectCheckpointManifest) -> (String, String) {
    let stem = format!(
        "{}.{:020}.{}",
        browser_project_prefix(&manifest.project_id),
        manifest.created_unix_ms,
        manifest.checkpoint_id
    );
    (format!("{stem}.manifest"), format!("{stem}.snapshot"))
}

#[cfg(target_arch = "wasm32")]
fn browser_project_prefix(project_id: &str) -> String {
    format!("{BROWSER_KEY_PREFIX}.{project_id}")
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn start_list(
    state: &AppState,
    complete: impl FnOnce(Result<ProjectCheckpointCatalog, String>) + 'static,
) {
    let project_id = state.workspace.project.id().to_string();
    let prefix = browser_project_prefix(&project_id);
    crate::common::project_lifecycle::start_browser_checkpoint_list(prefix, move |records| {
        complete(records.map(|(records, orphan_snapshots)| {
            let mut checkpoints = Vec::new();
            let mut quarantined = Vec::new();
            for (manifest_key, value, snapshot_key, snapshot) in records {
                let Some(value) = value else {
                    quarantined.push(ProjectCheckpointQuarantine {
                        label: manifest_key.clone(),
                        reason: "checkpoint manifest is not a JSON string".to_owned(),
                        artifact_keys: vec![manifest_key, snapshot_key],
                    });
                    continue;
                };
                let manifest = match serde_json::from_str::<ProjectCheckpointManifest>(&value) {
                    Ok(manifest) => manifest,
                    Err(error) => {
                        quarantined.push(ProjectCheckpointQuarantine {
                            label: manifest_key.clone(),
                            reason: format!("checkpoint manifest is invalid: {error}"),
                            artifact_keys: vec![manifest_key, snapshot_key],
                        });
                        continue;
                    }
                };
                if manifest.project_id != project_id {
                    quarantined.push(ProjectCheckpointQuarantine {
                        label: manifest_key.clone(),
                        reason: "checkpoint key ownership and manifest project identity disagree"
                            .to_owned(),
                        artifact_keys: vec![manifest_key, snapshot_key],
                    });
                    continue;
                }
                let (expected_manifest_key, expected_snapshot_key) = browser_keys(&manifest);
                if manifest_key != expected_manifest_key || snapshot_key != expected_snapshot_key {
                    quarantined.push(ProjectCheckpointQuarantine {
                        label: manifest.checkpoint_id.to_string(),
                        reason: "checkpoint manifest is not bound to its canonical IndexedDB keys"
                            .to_owned(),
                        artifact_keys: vec![manifest_key, snapshot_key],
                    });
                    continue;
                }
                let Some(snapshot) = snapshot else {
                    quarantined.push(ProjectCheckpointQuarantine {
                        label: manifest.checkpoint_id.to_string(),
                        reason: "checkpoint snapshot is missing".to_owned(),
                        artifact_keys: vec![manifest_key, snapshot_key],
                    });
                    continue;
                };
                let summary = match summary_from_manifest(
                    manifest,
                    ProjectCheckpointLocator::Browser {
                        manifest_key: manifest_key.clone(),
                        snapshot_key: snapshot_key.clone(),
                    },
                ) {
                    Ok(summary) => summary,
                    Err(error) => {
                        quarantined.push(ProjectCheckpointQuarantine {
                            label: manifest_key.clone(),
                            reason: error,
                            artifact_keys: vec![manifest_key, snapshot_key],
                        });
                        continue;
                    }
                };
                match validate_snapshot_bytes(&summary, &snapshot) {
                    Ok(_) => checkpoints.push(summary),
                    Err(error) => quarantined.push(ProjectCheckpointQuarantine {
                        label: summary.checkpoint_id.to_string(),
                        reason: error,
                        artifact_keys: vec![manifest_key, snapshot_key],
                    }),
                }
            }
            for snapshot_key in orphan_snapshots {
                quarantined.push(ProjectCheckpointQuarantine {
                    label: snapshot_key.clone(),
                    reason: "checkpoint snapshot has no committed manifest".to_owned(),
                    artifact_keys: vec![snapshot_key],
                });
            }
            sort_newest_first(&mut checkpoints);
            quarantined.sort_by(|left, right| right.label.cmp(&left.label));
            ProjectCheckpointCatalog {
                checkpoints,
                quarantined,
            }
        }));
    });
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn start_recovery_copy_bytes(
    summary: ProjectCheckpointSummary,
    destination: PathBuf,
    complete: impl FnOnce(Result<Vec<u8>, String>) + 'static,
) {
    let ProjectCheckpointLocator::Browser { snapshot_key, .. } = &summary.locator;
    crate::common::project_lifecycle::start_browser_checkpoint_read(
        snapshot_key.clone(),
        move |result| {
            complete(result.and_then(|bytes| {
                let mut project = validate_snapshot_bytes(&summary, &bytes)?;
                project.workspace.project = project.workspace.project.fork_copy_at(destination);
                crate::io::project_io::serialize_project_file(&project)
                    .map(String::into_bytes)
                    .map_err(|error| format!("recovery copy serialization failed: {error}"))
            }));
        },
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn manifest_rejects_noncanonical_snapshot_identity() {
        let manifest = ProjectCheckpointManifest {
            schema_version: MANIFEST_SCHEMA_VERSION,
            checkpoint_id: Uuid::nil(),
            project_id: Uuid::new_v4().to_string(),
            project_name: "Precision reference".to_owned(),
            project_revision: 7,
            reason: ProjectCheckpointReason::TechnologyAttachment,
            created_unix_ms: 42,
            snapshot_digest: ContentDigest::from_bytes([0x51; 32]),
            snapshot_byte_len: 128,
            snapshot_name: "wrong.rspiceproj".to_owned(),
        };
        assert!(validate_manifest(&manifest).is_err());
    }

    #[test]
    fn snapshot_integrity_rejects_any_byte_change() {
        let state = AppState::default();
        let project = crate::common::project_lifecycle::snapshot(&state).expect("snapshot");
        let bytes = crate::io::project_io::serialize_project_file(&project)
            .expect("serialize")
            .into_bytes();
        let summary = ProjectCheckpointSummary {
            checkpoint_id: Uuid::new_v4(),
            project_id: project.workspace.project.id().to_string(),
            project_name: project.workspace.project.name().to_owned(),
            project_revision: project.workspace.project.revision().get(),
            reason: ProjectCheckpointReason::TechnologyAttachment,
            created_unix_ms: 1,
            snapshot_digest: digest(&bytes),
            snapshot_byte_len: bytes.len() as u64,
            locator: ProjectCheckpointLocator::Native {
                manifest: PathBuf::from("manifest"),
                snapshot: PathBuf::from("snapshot"),
            },
        };
        validate_snapshot_bytes(&summary, &bytes).expect("exact bytes restore");
        let mut wrong_name = summary.clone();
        wrong_name.project_name = "Different project".to_owned();
        assert!(validate_snapshot_bytes(&wrong_name, &bytes).is_err());
        assert!(matches_current_state(&summary, &state).expect("state digest compares"));
        let mut changed = bytes;
        changed.push(b' ');
        assert!(validate_snapshot_bytes(&summary, &changed).is_err());

        let mut changed_state = state;
        changed_state.workspace.netlist_source = Some("R1 1 0 1k\n.end\n".to_owned());
        assert!(
            !matches_current_state(&summary, &changed_state)
                .expect("changed state digest compares")
        );
    }

    #[test]
    fn native_checkpoint_roundtrip_and_recovery_copy_are_non_destructive() {
        let directory =
            std::env::temp_dir().join(format!("rspice-project-checkpoint-{}", Uuid::new_v4()));
        std::fs::create_dir_all(&directory).expect("create fixture directory");
        let mut state = AppState::default();
        state
            .workspace
            .project
            .set_path(directory.join("source.rspiceproj"));
        let source_id = state.workspace.project.id();
        let source_revision = state.workspace.project.revision();

        let checkpoint = create(&state, ProjectCheckpointReason::TechnologyAttachment)
            .expect("checkpoint publishes");
        assert_eq!(checkpoint.project_revision(), source_revision.get());
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt as _;
            let ProjectCheckpointLocator::Native { manifest, snapshot } = &checkpoint.locator;
            assert_eq!(
                std::fs::metadata(manifest)
                    .expect("manifest metadata")
                    .permissions()
                    .mode()
                    & 0o777,
                0o600
            );
            assert_eq!(
                std::fs::metadata(snapshot)
                    .expect("snapshot metadata")
                    .permissions()
                    .mode()
                    & 0o777,
                0o600
            );
        }
        let listed = list(&state).expect("checkpoint catalog reads");
        assert_eq!(listed.checkpoints.len(), 1);
        assert!(listed.quarantined.is_empty());
        assert_eq!(
            listed.checkpoints[0].checkpoint_id(),
            checkpoint.checkpoint_id()
        );
        let restored = load(&listed.checkpoints[0]).expect("checkpoint bytes restore");
        assert_eq!(restored.workspace.project.id(), source_id);

        let destination = directory.join("recovered.rspiceproj");
        publish_recovery_copy(&listed.checkpoints[0], &destination).expect("copy publishes");
        let copy = crate::io::load_project_file(&destination).expect("copy loads");
        assert_ne!(copy.workspace.project.id(), source_id);
        assert_eq!(copy.workspace.project.revision().get(), 1);

        std::fs::write(&destination, b"external bytes").expect("replace fixture copy");
        assert!(publish_recovery_copy(&listed.checkpoints[0], &destination).is_err());
        assert_eq!(
            std::fs::read(&destination).expect("destination remains readable"),
            b"external bytes"
        );
        let _ = std::fs::remove_dir_all(&directory);
    }

    #[test]
    fn owned_corrupt_checkpoint_is_exposed_as_quarantine() {
        let directory =
            std::env::temp_dir().join(format!("rspice-project-quarantine-{}", Uuid::new_v4()));
        std::fs::create_dir_all(&directory).expect("create fixture directory");
        let mut state = AppState::default();
        state
            .workspace
            .project
            .set_path(directory.join("source.rspiceproj"));
        let checkpoint = create(&state, ProjectCheckpointReason::TechnologyAttachment)
            .expect("checkpoint publishes");
        let ProjectCheckpointLocator::Native { snapshot, .. } = &checkpoint.locator;
        std::fs::write(snapshot, b"corrupt checkpoint").expect("corrupt owned snapshot");

        let catalog = list(&state).expect("catalog returns quarantine diagnostics");
        assert!(catalog.checkpoints.is_empty());
        assert_eq!(catalog.quarantined.len(), 1);
        assert!(
            catalog.quarantined[0]
                .reason()
                .contains("integrity manifest")
        );
        let _ = std::fs::remove_dir_all(&directory);
    }
}
