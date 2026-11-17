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

use serde::{Deserialize, Serialize};
use sha2::{Digest as _, Sha256};
use uuid::Uuid;

use crate::io::ProjectSnapshot;
#[cfg(not(target_arch = "wasm32"))]
use crate::io::durable_file::ExpectedContent;
use crate::product::ContentDigest;
#[cfg(not(target_arch = "wasm32"))]
use crate::workbench::lifecycle::project_lifecycle::LifecycleSourceMut;
use crate::workbench::lifecycle::project_lifecycle::{LifecycleSource, Model};
#[cfg(not(target_arch = "wasm32"))]
use rspice_formats::project_results::segment_folder;

const MANIFEST_SCHEMA_VERSION: u16 = 2;
/// A saved project's checkpoints are kept in a folder beside its file, named
/// for it: `amp.rspiceproj` keeps them in `amp.rspice-recovery`.
#[cfg(not(target_arch = "wasm32"))]
const CHECKPOINT_DIRECTORY_SUFFIX: &str = ".rspice-recovery";
#[cfg(not(target_arch = "wasm32"))]
const MANIFEST_SUFFIX: &str = ".checkpoint.json";
const SNAPSHOT_SUFFIX: &str = ".rspiceproj";
#[cfg(target_arch = "wasm32")]
const BROWSER_KEY_PREFIX: &str = "rspice.project-recovery.v1";
const MIB: u64 = 1024 * 1024;

/// Why a checkpoint was taken: by the reader, as an autosave of unsaved
/// work, or by RSpice just before the change each other reason names.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProjectCheckpointReason {
    #[serde(rename = "manual")]
    Manual,
    #[serde(rename = "before-save")]
    Save,
    #[serde(rename = "before-restore")]
    Restore,
    #[serde(rename = "before-technology-change")]
    TechnologyChange,
    #[serde(rename = "before-library-deletion")]
    LibraryDeletion,
    #[serde(rename = "before-publication-restore")]
    PublicationRestore,
    #[serde(rename = "autosave")]
    Autosave,
}

impl ProjectCheckpointReason {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Manual => "Manual checkpoint",
            Self::Save => "Before save",
            Self::Restore => "Before restore",
            Self::TechnologyChange => "Before technology change",
            Self::LibraryDeletion => "Before library deletion",
            Self::PublicationRestore => "Before restore from publication",
            Self::Autosave => "Autosave",
        }
    }
}

/// How long retention keeps a checkpoint.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CheckpointRetention {
    /// Manual and pinned checkpoints: never removed automatically.
    UntilDeleted,
    /// An autosave: only the newest is kept, and a save removes it.
    UntilNextSave,
    /// Every other automatic checkpoint: kept inside the disk budget the
    /// Preferences set, oldest dropped first.
    WithinBudget,
}

impl CheckpointRetention {
    /// The rule in the words the Recovery page and its inspector use.
    pub const fn label(self) -> &'static str {
        match self {
            Self::UntilDeleted => "Until you delete it",
            Self::UntilNextSave => "Until the next save",
            Self::WithinBudget => "While it fits the automatic space",
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
    /// What the reader wrote about the checkpoint, one line.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    note: Option<String>,
    /// Kept until the reader deletes it, whatever its reason.
    pinned: bool,
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
pub struct ProjectCheckpointSummary {
    checkpoint_id: Uuid,
    project_id: String,
    project_name: String,
    project_revision: u64,
    reason: ProjectCheckpointReason,
    created_unix_ms: u64,
    note: Option<String>,
    pinned: bool,
    snapshot_digest: ContentDigest,
    snapshot_byte_len: u64,
    locator: ProjectCheckpointLocator,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectCheckpointQuarantine {
    label: String,
    reason: String,
    #[cfg(not(target_arch = "wasm32"))]
    artifacts: Vec<PathBuf>,
    #[cfg(target_arch = "wasm32")]
    artifact_keys: Vec<String>,
}

impl ProjectCheckpointQuarantine {
    /// The artifact that failed verification: its checkpoint identity when its
    /// manifest could be read, otherwise the file or key it was found under.
    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn reason(&self) -> &str {
        &self.reason
    }
}

#[derive(Debug, Clone, Default)]
pub struct ProjectCheckpointCatalog {
    pub checkpoints: Vec<ProjectCheckpointSummary>,
    pub quarantined: Vec<ProjectCheckpointQuarantine>,
}

impl ProjectCheckpointSummary {
    pub const fn checkpoint_id(&self) -> Uuid {
        self.checkpoint_id
    }

    pub fn project_name(&self) -> &str {
        &self.project_name
    }

    pub const fn project_revision(&self) -> u64 {
        self.project_revision
    }

    pub const fn reason(&self) -> ProjectCheckpointReason {
        self.reason
    }

    pub const fn snapshot_byte_len(&self) -> u64 {
        self.snapshot_byte_len
    }

    pub const fn created_unix_ms(&self) -> u64 {
        self.created_unix_ms
    }

    pub const fn snapshot_digest(&self) -> ContentDigest {
        self.snapshot_digest
    }

    /// What the reader wrote about the checkpoint.
    pub fn note(&self) -> Option<&str> {
        self.note.as_deref()
    }

    pub const fn pinned(&self) -> bool {
        self.pinned
    }

    /// The checkpoint's name in a list: the reader's note when it has one,
    /// otherwise its reason.
    pub fn title(&self) -> &str {
        self.note().unwrap_or(self.reason.label())
    }

    pub const fn retention(&self) -> CheckpointRetention {
        if self.pinned || matches!(self.reason, ProjectCheckpointReason::Manual) {
            CheckpointRetention::UntilDeleted
        } else if matches!(self.reason, ProjectCheckpointReason::Autosave) {
            CheckpointRetention::UntilNextSave
        } else {
            CheckpointRetention::WithinBudget
        }
    }
}

/// A catalog entry for a checkpoint that exists only in a test, so a surface
/// can be drawn over a populated catalog without publishing one.
#[cfg(all(any(test, feature = "test-support"), not(target_arch = "wasm32")))]
pub fn fixture_summary(
    project_name: &str,
    project_revision: u64,
    reason: ProjectCheckpointReason,
    created_unix_ms: u64,
    snapshot_byte_len: u64,
) -> ProjectCheckpointSummary {
    ProjectCheckpointSummary {
        checkpoint_id: Uuid::new_v4(),
        project_id: Uuid::new_v4().to_string(),
        project_name: project_name.to_owned(),
        project_revision,
        reason,
        created_unix_ms,
        note: None,
        pinned: false,
        snapshot_digest: ContentDigest::from_bytes([0x5a; 32]),
        snapshot_byte_len,
        locator: ProjectCheckpointLocator::Native {
            manifest: PathBuf::from("fixture.checkpoint.json"),
            snapshot: PathBuf::from("fixture.rspiceproj"),
        },
    }
}

#[cfg(all(any(test, feature = "test-support"), not(target_arch = "wasm32")))]
impl ProjectCheckpointSummary {
    /// The fixture with a note written on it.
    #[must_use]
    pub fn with_note(mut self, note: &str) -> Self {
        self.note = Some(note.to_owned());
        self
    }

    /// The fixture pinned.
    #[must_use]
    pub const fn pinned_fixture(mut self) -> Self {
        self.pinned = true;
        self
    }
}

/// A set-aside artifact that exists only in a test.
#[cfg(all(any(test, feature = "test-support"), not(target_arch = "wasm32")))]
pub fn fixture_quarantine(label: &str, reason: &str) -> ProjectCheckpointQuarantine {
    ProjectCheckpointQuarantine {
        label: label.to_owned(),
        reason: reason.to_owned(),
        artifacts: Vec::new(),
    }
}

pub fn matches_current_state(
    summary: &ProjectCheckpointSummary,
    state: &dyn LifecycleSource,
) -> Result<bool, String> {
    capture_current_state(state)?.matches(summary)
}

/// The open project as a checkpoint of it would hold it, read off the model.
/// Comparing it with a checkpoint is work for a background thread.
#[derive(Debug)]
pub struct CurrentProjectState {
    project: ProjectSnapshot,
}

pub fn capture_current_state(state: &dyn LifecycleSource) -> Result<CurrentProjectState, String> {
    crate::workbench::lifecycle::project_lifecycle::snapshot(state)
        .map(|project| CurrentProjectState { project })
        .map_err(|error| format!("project checkpoint comparison failed: {error}"))
}

impl CurrentProjectState {
    /// Whether `summary` holds exactly this project, by its manifest: the
    /// checkpoint's snapshot is not read.
    pub fn matches(&self, summary: &ProjectCheckpointSummary) -> Result<bool, String> {
        self.project
            .file
            .validate()
            .map_err(|error| format!("current project is invalid: {error}"))?;
        let serialized = crate::io::project_io::serialize_project_file(&self.project)
            .map_err(|error| format!("current project comparison serialization failed: {error}"))?;
        Ok(digest(serialized.as_bytes()) == summary.snapshot_digest
            && serialized.len() as u64 == summary.snapshot_byte_len)
    }
}

#[cfg(any(test, target_arch = "wasm32"))]
fn prepare_checkpoint(
    state: &dyn LifecycleSource,
    reason: ProjectCheckpointReason,
) -> Result<(ProjectCheckpointManifest, String), String> {
    let (project, created_unix_ms) = snapshot_now(state)?;
    seal_checkpoint(&project, reason, created_unix_ms)
}

/// The open project read off the model, and when: a clock that cannot date
/// the checkpoint stops it before anything is read or written.
fn snapshot_now(state: &dyn LifecycleSource) -> Result<(ProjectSnapshot, u64), String> {
    let created_unix_ms = crate::time_compat::checked_unix_time_ms()
        .map_err(|error| format!("Project checkpoint could not be timestamped: {error}"))?;
    let project = crate::workbench::lifecycle::project_lifecycle::snapshot(state)
        .map_err(|error| format!("project checkpoint snapshot failed: {error}"))?;
    Ok((project, created_unix_ms))
}

/// The manifest and exact bytes a checkpoint of `project` is written as.
fn seal_checkpoint(
    project: &ProjectSnapshot,
    reason: ProjectCheckpointReason,
    created_unix_ms: u64,
) -> Result<(ProjectCheckpointManifest, String), String> {
    project
        .file
        .validate()
        .map_err(|error| format!("project checkpoint is invalid: {error}"))?;
    let serialized = crate::io::project_io::serialize_project_file(project)
        .map_err(|error| format!("project checkpoint serialization failed: {error}"))?;
    let bytes = serialized.as_bytes();
    let project_id = project.file.workspace.project.id().to_string();
    let checkpoint_id = Uuid::new_v4();
    let snapshot_name = format!("{created_unix_ms}-{checkpoint_id}{SNAPSHOT_SUFFIX}");
    let manifest = ProjectCheckpointManifest {
        schema_version: MANIFEST_SCHEMA_VERSION,
        checkpoint_id,
        project_id: project_id.clone(),
        project_name: project.file.workspace.project.name().to_owned(),
        project_revision: project.file.workspace.project.revision().get(),
        reason,
        created_unix_ms,
        note: None,
        pinned: false,
        snapshot_digest: digest(bytes),
        snapshot_byte_len: bytes.len().try_into().unwrap_or(u64::MAX),
        snapshot_name,
    };

    validate_manifest(&manifest)?;
    Ok((manifest, serialized))
}

#[cfg(not(target_arch = "wasm32"))]
pub fn create(
    state: &mut dyn LifecycleSourceMut,
    reason: ProjectCheckpointReason,
) -> Result<ProjectCheckpointSummary, String> {
    let state = &mut state.lifecycle_parts();
    let created = capture(state, reason)?.publish()?;
    if let Some(warning) = created.retention_warning {
        state.model.record(
            crate::diagnostics::LogSeverity::Warning,
            crate::diagnostics::LogSource::Studio,
            warning,
        );
    }
    Ok(created.summary)
}

/// A checkpoint of the open project read off the model. Writing it is work
/// for a background thread: nothing here holds the model.
#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug)]
pub struct CheckpointCapture {
    project: ProjectSnapshot,
    reason: ProjectCheckpointReason,
    created_unix_ms: u64,
    directory: PathBuf,
    budget_bytes: u64,
}

/// What writing a checkpoint left: the checkpoint, the project's checkpoints
/// as read back after it, and a retention failure that did not undo it.
#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug)]
pub struct CheckpointCreated {
    pub summary: ProjectCheckpointSummary,
    pub catalog: Result<ProjectCheckpointCatalog, String>,
    pub retention_warning: Option<String>,
}

#[cfg(not(target_arch = "wasm32"))]
pub fn capture(
    state: &dyn LifecycleSource,
    reason: ProjectCheckpointReason,
) -> Result<CheckpointCapture, String> {
    let (project, created_unix_ms) = snapshot_now(state)?;
    let model = state.lifecycle_view().model;
    Ok(CheckpointCapture {
        project,
        reason,
        created_unix_ms,
        directory: checkpoint_directory(model)?,
        budget_bytes: budget_bytes(model),
    })
}

/// The disk space automatic checkpoints are kept inside, as the Preferences
/// set it.
pub fn budget_bytes(model: &Model) -> u64 {
    u64::from(
        model.preferences.scalar(
            rspice_app_state::workbench::preferences::ScalarPreference::CheckpointBudgetMib,
        ),
    ) * MIB
}

#[cfg(not(target_arch = "wasm32"))]
impl CheckpointCapture {
    /// Write the checkpoint and read it back, then drop what retention no
    /// longer keeps and list what is left.
    pub fn publish(self) -> Result<CheckpointCreated, String> {
        let (manifest, serialized) =
            seal_checkpoint(&self.project, self.reason, self.created_unix_ms)?;
        // The results the checkpoint names are kept beside it first, as a save keeps a
        // project's: it outlives the run's own store.
        crate::io::result_segments::keep_segments_now(
            crate::io::result_segments::file_segments(
                &self.project.file.simulation_results,
                &self.project.file.workspace.visualization_documents,
            ),
            &self.directory.join(&manifest.snapshot_name),
        )?;
        let summary = publish_native(&self.directory, &manifest, serialized.as_bytes())?;
        // Retention runs only after the new manifest is committed and
        // verified. A cleanup failure never invalidates the new checkpoint.
        let retention_warning =
            prune_retention(&self.directory, &manifest.project_id, self.budget_bytes)
                .err()
                .map(|error| {
                    format!(
                        "The project checkpoint was created, but older checkpoints were not all removed: {error}"
                    )
                });
        let catalog = list_directory(&self.directory, &manifest.project_id);
        Ok(CheckpointCreated {
            summary,
            catalog,
            retention_warning,
        })
    }
}

/// Where the open project's checkpoints are listed from, read off the
/// model. Listing them is work for a background thread.
#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug, Clone)]
pub struct CheckpointListing {
    directory: PathBuf,
    project_id: String,
}

#[cfg(not(target_arch = "wasm32"))]
pub fn listing(model: &Model) -> Result<CheckpointListing, String> {
    Ok(CheckpointListing {
        directory: checkpoint_directory(model)?,
        project_id: model.workspace.content.project.id().to_string(),
    })
}

#[cfg(not(target_arch = "wasm32"))]
impl CheckpointListing {
    /// The project's checkpoints, each from its manifest.
    pub fn list(&self) -> Result<ProjectCheckpointCatalog, String> {
        list_directory(&self.directory, &self.project_id)
    }
}

#[cfg(target_arch = "wasm32")]
pub fn start_create(
    state: &dyn LifecycleSource,
    reason: ProjectCheckpointReason,
    complete: impl FnOnce(Result<ProjectCheckpointSummary, String>) + 'static,
) -> Result<(), String> {
    let (manifest, serialized) = prepare_checkpoint(state, reason)?;
    let budget = budget_bytes(state.lifecycle_view().model);
    let (manifest_key, snapshot_key) = browser_keys(&manifest);
    let prefix = browser_project_prefix(&manifest.project_id);
    let manifest_json = serde_json::to_string(&manifest)
        .map_err(|error| format!("project checkpoint manifest serialization failed: {error}"))?;
    let manifest_for_completion = manifest.clone();
    let manifest_key_for_completion = manifest_key.clone();
    let snapshot_key_for_completion = snapshot_key.clone();
    crate::workbench::lifecycle::project_lifecycle::start_browser_checkpoint_publish(
        prefix,
        manifest_key,
        snapshot_key,
        serialized.into_bytes(),
        manifest_json,
        move |stored| browser_retention_removals(stored, budget, false),
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
pub fn list(model: &Model) -> Result<ProjectCheckpointCatalog, String> {
    listing(model)?.list()
}

#[cfg(not(target_arch = "wasm32"))]
pub fn load(summary: &ProjectCheckpointSummary) -> Result<ProjectSnapshot, String> {
    let ProjectCheckpointLocator::Native { snapshot, .. } = &summary.locator;
    rspice_results::store_reader::keep_segments_in(&segment_folder(snapshot));
    let bytes = read_snapshot_bytes(summary)?;
    validate_snapshot_bytes(summary, &bytes)
}

#[cfg(not(target_arch = "wasm32"))]
pub fn recovery_copy_bytes(
    summary: &ProjectCheckpointSummary,
    destination: &Path,
) -> Result<Vec<u8>, String> {
    recovery_copy_text(&recovery_copy(summary, destination)?)
}

/// The checkpoint's project as a new project at `destination`.
#[cfg(not(target_arch = "wasm32"))]
fn recovery_copy(
    summary: &ProjectCheckpointSummary,
    destination: &Path,
) -> Result<ProjectSnapshot, String> {
    let mut project = load(summary)?;
    project.file.workspace.project = project
        .file
        .workspace
        .project
        .fork_copy_at(destination.to_path_buf());
    Ok(project)
}

#[cfg(not(target_arch = "wasm32"))]
fn recovery_copy_text(project: &ProjectSnapshot) -> Result<Vec<u8>, String> {
    let serialized = crate::io::project_io::serialize_project_file(project)
        .map_err(|error| format!("recovery copy serialization failed: {error}"))?;
    Ok(serialized.into_bytes())
}

#[cfg(not(target_arch = "wasm32"))]
pub fn publish_recovery_copy(
    summary: &ProjectCheckpointSummary,
    destination: &Path,
) -> Result<(), String> {
    let project = recovery_copy(summary, destination)?;
    let bytes = recovery_copy_text(&project)?;
    let expected = crate::io::durable_file::observe_expected_content(destination)
        .map_err(|error| format!("recovery destination could not be inspected: {error}"))?;
    if !matches!(expected, crate::io::durable_file::ExpectedContent::Missing) {
        return Err("recovery copies never overwrite an existing file".to_owned());
    }
    // The copy reads its results from beside it, as a saved project does.
    crate::io::result_segments::keep_segments_now(
        crate::io::result_segments::file_segments(
            &project.file.simulation_results,
            &project.file.workspace.visualization_documents,
        ),
        destination,
    )?;
    crate::io::durable_file::compare_exchange_bytes(destination, expected, &bytes)
        .map_err(|error| format!("recovery copy publication failed: {error}"))
}

fn validate_snapshot_bytes(
    summary: &ProjectCheckpointSummary,
    bytes: &[u8],
) -> Result<ProjectSnapshot, String> {
    if bytes.len() as u64 != summary.snapshot_byte_len || digest(bytes) != summary.snapshot_digest {
        return Err("project checkpoint bytes do not match their integrity manifest".to_owned());
    }
    let text = std::str::from_utf8(bytes)
        .map_err(|error| format!("project checkpoint is not valid UTF-8: {error}"))?;
    let project = crate::io::project_io::load_project_text(text, None)
        .map_err(|error| format!("project checkpoint cannot be restored: {error}"))?;
    if project.file.workspace.project.id().to_string() != summary.project_id
        || project.file.workspace.project.revision().get() != summary.project_revision
        || project.file.workspace.project.name() != summary.project_name
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
    if let Some(note) = &manifest.note
        && normalized_note(note)?.as_ref() != Some(note)
    {
        return Err("project checkpoint note is not one trimmed line of text".to_owned());
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
        return Err("project checkpoint snapshot name does not match its manifest".to_owned());
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
        note: manifest.note,
        pinned: manifest.pinned,
        snapshot_digest: manifest.snapshot_digest,
        snapshot_byte_len: manifest.snapshot_byte_len,
        locator,
    })
}

/// A note as it is stored: one line, trimmed, or none when it is blank.
pub fn normalized_note(note: &str) -> Result<Option<String>, String> {
    let note = note.trim();
    if note.chars().any(char::is_control) {
        return Err("a checkpoint note is one line of text".to_owned());
    }
    Ok((!note.is_empty()).then(|| note.to_owned()))
}

/// The positions in `checkpoints`, sorted newest first, that retention
/// removes. Manual and pinned checkpoints are never among them. Of the
/// autosaves only the newest stays. The other automatic checkpoints are kept
/// newest first while they fit in `budget_bytes`, the newest of them always,
/// and once one does not fit it goes with everything older.
fn retention_removals(checkpoints: &[ProjectCheckpointSummary], budget_bytes: u64) -> Vec<usize> {
    let mut removals = Vec::new();
    let mut autosave_kept = false;
    let mut budget_used = 0_u64;
    let mut budget_kept_any = false;
    let mut budget_full = false;
    for (index, checkpoint) in checkpoints.iter().enumerate() {
        match checkpoint.retention() {
            CheckpointRetention::UntilDeleted => {}
            CheckpointRetention::UntilNextSave => {
                if autosave_kept {
                    removals.push(index);
                }
                autosave_kept = true;
            }
            CheckpointRetention::WithinBudget => {
                let used = budget_used.saturating_add(checkpoint.snapshot_byte_len);
                if budget_full || (budget_kept_any && used > budget_bytes) {
                    budget_full = true;
                    removals.push(index);
                } else {
                    budget_used = used;
                    budget_kept_any = true;
                }
            }
        }
    }
    removals
}

/// A budget in the whole units it is chosen in: "512 MiB", "2 GiB".
pub fn budget_label(bytes: u64) -> String {
    let mib = bytes / MIB;
    if mib >= 1024 && mib % 1024 == 0 {
        format!("{} GiB", mib / 1024)
    } else {
        format!("{mib} MiB")
    }
}

/// What the reader changes about a checkpoint.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CheckpointEdit {
    /// Write this note on it, or clear it.
    Note(Option<String>),
    /// Keep it until the reader deletes it, or let retention have it again.
    Pin(bool),
}

/// The manifest in `stored` with `edit` made, when it is still the one
/// `summary` was listed from.
fn edited_manifest(
    summary: &ProjectCheckpointSummary,
    stored: &[u8],
    edit: &CheckpointEdit,
) -> Result<ProjectCheckpointManifest, String> {
    let mut manifest = serde_json::from_slice::<ProjectCheckpointManifest>(stored)
        .map_err(|error| format!("checkpoint manifest is invalid: {error}"))?;
    if manifest.checkpoint_id != summary.checkpoint_id
        || manifest.snapshot_digest != summary.snapshot_digest
    {
        return Err("the checkpoint changed on disk after it was listed".to_owned());
    }
    match edit {
        CheckpointEdit::Note(note) => manifest.note.clone_from(note),
        CheckpointEdit::Pin(pinned) => manifest.pinned = *pinned,
    }
    validate_manifest(&manifest)?;
    Ok(manifest)
}

/// Make `edit` to the checkpoint's manifest, in place of the bytes it was
/// read from.
#[cfg(not(target_arch = "wasm32"))]
pub fn edit(summary: &ProjectCheckpointSummary, edit: &CheckpointEdit) -> Result<(), String> {
    let ProjectCheckpointLocator::Native { manifest: path, .. } = &summary.locator;
    let stored = std::fs::read(path)
        .map_err(|error| format!("checkpoint manifest could not be read: {error}"))?;
    let manifest = edited_manifest(summary, &stored, edit)?;
    let mut bytes = serde_json::to_vec_pretty(&manifest)
        .map_err(|error| format!("project checkpoint manifest serialization failed: {error}"))?;
    bytes.push(b'\n');
    publish_private_file(
        path,
        ExpectedContent::Digest(Sha256::digest(&stored).into()),
        &bytes,
    )
    .map_err(|error| format!("checkpoint manifest could not be written: {error}"))
}

/// Delete a checkpoint: its manifest first, so it is no longer listed, then
/// its snapshot and the results only it kept.
#[cfg(not(target_arch = "wasm32"))]
pub fn delete(summary: &ProjectCheckpointSummary) -> Result<(), String> {
    let ProjectCheckpointLocator::Native { manifest, snapshot } = &summary.locator;
    let mut failures = Vec::new();
    remove_checkpoint(manifest, snapshot, &mut failures);
    joined(failures)
}

/// Delete the files an artifact that failed verification was found as.
#[cfg(not(target_arch = "wasm32"))]
pub fn delete_set_aside(artifact: &ProjectCheckpointQuarantine) -> Result<(), String> {
    let mut failures = Vec::new();
    remove_artifacts(
        artifact.artifacts.iter().map(PathBuf::as_path),
        &mut failures,
    );
    joined(failures)
}

/// Bring the project's checkpoints up to a save that has been written.
///
/// `model` is the project as saved, `previous_path` where it was saved before
/// (none for a first save) and `replaced` the saved version this save wrote
/// over, when the save changed it. The checkpoints kept for the old place
/// follow the project: moved out of the application's data folder on a first
/// save, copied when the old file is still there. The replaced version is
/// kept as a "Before save" checkpoint. A save of every document leaves no
/// unsaved work, so the autosave that held it goes. Returns what could not
/// be done; the save stands either way.
#[cfg(not(target_arch = "wasm32"))]
pub fn after_save(
    model: &Model,
    previous_path: Option<&Path>,
    replaced: Option<&ProjectSnapshot>,
    saved_everything: bool,
) -> Result<(), String> {
    let project_id = model.workspace.content.project.id().to_string();
    let directory = checkpoint_directory(model)?;
    let mut failures = Vec::new();
    // A folder that cannot be named (no application data folder) never
    // received a checkpoint, so there is nothing to bring from it.
    if let Ok(previous) = directory_for(previous_path, &project_id)
        && previous != directory
        && let Err(error) = carry(&previous, &directory, &project_id, previous_path.is_none())
    {
        failures.push(format!(
            "its checkpoints were not all brought beside it: {error}"
        ));
    }
    if let Some(replaced) = replaced {
        let kept = crate::time_compat::checked_unix_time_ms()
            .map_err(|error| format!("the time could not be read: {error}"))
            .and_then(|now| seal_checkpoint(replaced, ProjectCheckpointReason::Save, now))
            .and_then(|(manifest, serialized)| {
                // Its results are kept beside it first, as any checkpoint's.
                crate::io::result_segments::keep_segments_now(
                    crate::io::result_segments::file_segments(
                        &replaced.file.simulation_results,
                        &replaced.file.workspace.visualization_documents,
                    ),
                    &directory.join(&manifest.snapshot_name),
                )?;
                publish_native(&directory, &manifest, serialized.as_bytes())
            });
        if let Err(error) = kept {
            failures.push(format!("the version it replaced was not kept: {error}"));
        }
    }
    if saved_everything {
        match list_directory(&directory, &project_id) {
            Ok(catalog) => {
                for checkpoint in catalog
                    .checkpoints
                    .iter()
                    .filter(|checkpoint| checkpoint.reason == ProjectCheckpointReason::Autosave)
                {
                    let ProjectCheckpointLocator::Native { manifest, snapshot } =
                        &checkpoint.locator;
                    remove_checkpoint(manifest, snapshot, &mut failures);
                }
            }
            Err(error) => failures.push(error),
        }
    }
    if let Err(error) = prune_retention(&directory, &project_id, budget_bytes(model)) {
        failures.push(format!("older checkpoints were not all removed: {error}"));
    }
    joined(failures)
}

/// Copy the project's checkpoints in `from` into `to`, each snapshot and the
/// results it reads before its manifest so a checkpoint is never listed
/// before what it holds is there, and when `remove` is set delete each from
/// `from` once its copy is in place. A checkpoint whose bytes no longer match
/// its manifest stays where it is.
#[cfg(not(target_arch = "wasm32"))]
fn carry(from: &Path, to: &Path, project_id: &str, remove: bool) -> Result<(), String> {
    let catalog = list_directory(from, project_id)?;
    if catalog.checkpoints.is_empty() {
        return Ok(());
    }
    prepare_directory(to)?;
    let mut failures = Vec::new();
    for checkpoint in &catalog.checkpoints {
        let ProjectCheckpointLocator::Native { manifest, snapshot } = &checkpoint.locator;
        let copied = (|| {
            let bytes = read_snapshot_bytes(checkpoint)?;
            if bytes.len() as u64 != checkpoint.snapshot_byte_len
                || digest(&bytes) != checkpoint.snapshot_digest
            {
                return Err("its bytes do not match its integrity manifest".to_owned());
            }
            let manifest_bytes = std::fs::read(manifest)
                .map_err(|error| format!("its manifest could not be read: {error}"))?;
            let (Some(snapshot_name), Some(manifest_name)) =
                (snapshot.file_name(), manifest.file_name())
            else {
                return Err("its files have no names".to_owned());
            };
            let target_manifest = to.join(manifest_name);
            if target_manifest.exists() {
                return Ok(());
            }
            let target_snapshot = to.join(snapshot_name);
            if !target_snapshot.exists() {
                publish_new_private_file(&target_snapshot, &bytes)
                    .map_err(|error| format!("its snapshot could not be copied: {error}"))?;
            }
            carry_results(snapshot, &target_snapshot)?;
            publish_new_private_file(&target_manifest, &manifest_bytes)
                .map_err(|error| format!("its manifest could not be copied: {error}"))
        })();
        match copied {
            Ok(()) if remove => remove_checkpoint(manifest, snapshot, &mut failures),
            Ok(()) => {}
            Err(error) => {
                failures.push(format!("checkpoint {}: {error}", checkpoint.checkpoint_id))
            }
        }
    }
    crate::io::durable_file::sync_parent_of(&to.join("checkpoint"))
        .map_err(|error| format!("the checkpoint folder could not be synchronized: {error}"))?;
    if remove {
        // Only an emptied folder goes; anything set aside in it stays.
        if let Some(results) = segment_folder(&from.join("checkpoint")).parent() {
            let _ = std::fs::remove_dir(results);
        }
        let _ = std::fs::remove_dir(from);
    }
    joined(failures)
}

/// Keep the results the checkpoint at `snapshot` reads beside its copy at
/// `target` too, each segment by the cheapest way the volume allows.
#[cfg(not(target_arch = "wasm32"))]
fn carry_results(snapshot: &Path, target: &Path) -> Result<(), String> {
    let entries = match std::fs::read_dir(segment_folder(snapshot)) {
        Ok(entries) => entries,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(()),
        Err(error) => return Err(format!("its results could not be read: {error}")),
    };
    let segments = entries
        .flatten()
        .filter_map(|entry| {
            let path = entry.path();
            let digest =
                rspice_results::store_reader::parse_digest(path.file_stem()?.to_str()?).ok()?;
            // A partial copy is not a segment.
            let named = rspice_results::store_reader::kept_segment_name(&digest);
            (path.file_name()? == named.as_str()).then_some(
                rspice_formats::project_results::StoredSegment {
                    digest,
                    source: Some(path),
                },
            )
        })
        .collect();
    crate::io::result_segments::keep_segments_now(segments, target)
}

/// Remove a checkpoint: its manifest first, so it stops being listed, then
/// its snapshot, then the results only it kept. A segment a reader still has
/// open is left for a later removal.
#[cfg(not(target_arch = "wasm32"))]
fn remove_checkpoint(manifest: &Path, snapshot: &Path, failures: &mut Vec<String>) {
    remove_artifacts([manifest, snapshot], failures);
    let _ = std::fs::remove_dir_all(segment_folder(snapshot));
}

#[cfg(not(target_arch = "wasm32"))]
fn joined(failures: Vec<String>) -> Result<(), String> {
    if failures.is_empty() {
        Ok(())
    } else {
        Err(failures.join("; "))
    }
}

/// The disk space the checkpoints in `checkpoints` that the budget governs
/// take up.
pub fn budget_use(checkpoints: &[ProjectCheckpointSummary]) -> u64 {
    checkpoints
        .iter()
        .filter(|checkpoint| checkpoint.retention() == CheckpointRetention::WithinBudget)
        .map(ProjectCheckpointSummary::snapshot_byte_len)
        .sum()
}

fn sort_newest_first(checkpoints: &mut [ProjectCheckpointSummary]) {
    checkpoints.sort_by(|left, right| {
        right
            .created_unix_ms
            .cmp(&left.created_unix_ms)
            .then_with(|| right.checkpoint_id.cmp(&left.checkpoint_id))
    });
}

/// Where this project's checkpoints are kept, as the reader would look for them.
#[cfg(not(target_arch = "wasm32"))]
pub fn storage_location(model: &Model) -> Result<PathBuf, String> {
    checkpoint_directory(model)
}

#[cfg(not(target_arch = "wasm32"))]
fn checkpoint_directory(model: &Model) -> Result<PathBuf, String> {
    let project = &model.workspace.content.project;
    directory_for(project.path.as_deref(), &project.id().to_string())
}

/// The one folder a project's checkpoints are kept in: beside its file and
/// named for it once it is saved, and under the application's data folder,
/// named for the project, until then. Every manifest in it still names its
/// project, so a folder shared by mistake never mixes two projects' lists.
#[cfg(not(target_arch = "wasm32"))]
fn directory_for(project_path: Option<&Path>, project_id: &str) -> Result<PathBuf, String> {
    let saved = project_path.and_then(|path| {
        let stem = path.file_stem().filter(|stem| !stem.is_empty())?;
        let mut name = stem.to_os_string();
        name.push(CHECKPOINT_DIRECTORY_SUFFIX);
        Some(path.parent().unwrap_or_else(|| Path::new("")).join(name))
    });
    let directory = match saved {
        Some(directory) => directory,
        None => unsaved_root()?.join(project_id),
    };
    if directory.is_absolute() {
        Ok(directory)
    } else {
        Ok(std::env::current_dir()
            .map_err(|error| format!("current directory is unavailable: {error}"))?
            .join(directory))
    }
}

/// Where the checkpoints of projects not yet saved are kept.
#[cfg(not(target_arch = "wasm32"))]
fn unsaved_root() -> Result<PathBuf, String> {
    #[cfg(any(test, feature = "test-support"))]
    if let Some(root) = UNSAVED_ROOT.with(|root| root.borrow().clone()) {
        return Ok(root);
    }
    dirs::data_local_dir()
        .map(|path| path.join("RSpice").join("recovery"))
        .ok_or_else(|| "no durable local recovery directory is available".to_owned())
}

#[cfg(all(any(test, feature = "test-support"), not(target_arch = "wasm32")))]
thread_local! {
    static UNSAVED_ROOT: std::cell::RefCell<Option<PathBuf>> =
        const { std::cell::RefCell::new(None) };
}

/// Keep this thread's unsaved projects' checkpoints under `root` rather than
/// the machine's application data folder, until the guard is dropped.
#[cfg(all(any(test, feature = "test-support"), not(target_arch = "wasm32")))]
#[must_use]
pub fn keep_unsaved_checkpoints_in(root: PathBuf) -> UnsavedCheckpointRoot {
    UnsavedCheckpointRoot(UNSAVED_ROOT.with(|slot| slot.replace(Some(root))))
}

/// Restores where unsaved projects' checkpoints are kept when dropped.
#[cfg(all(any(test, feature = "test-support"), not(target_arch = "wasm32")))]
pub struct UnsavedCheckpointRoot(Option<PathBuf>);

#[cfg(all(any(test, feature = "test-support"), not(target_arch = "wasm32")))]
impl Drop for UnsavedCheckpointRoot {
    fn drop(&mut self) {
        let previous = self.0.take();
        UNSAVED_ROOT.with(|slot| *slot.borrow_mut() = previous);
    }
}

/// Create the checkpoint folder, private to its owner.
#[cfg(not(target_arch = "wasm32"))]
fn prepare_directory(directory: &Path) -> Result<(), String> {
    std::fs::create_dir_all(directory).map_err(|error| {
        format!(
            "project checkpoint directory '{}' could not be created: {error}",
            directory.display()
        )
    })?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt as _;
        std::fs::set_permissions(directory, std::fs::Permissions::from_mode(0o700)).map_err(
            |error| {
                format!(
                    "project checkpoint directory '{}' could not be made private: {error}",
                    directory.display()
                )
            },
        )?;
    }
    Ok(())
}

#[cfg(not(target_arch = "wasm32"))]
fn publish_native(
    directory: &Path,
    manifest: &ProjectCheckpointManifest,
    bytes: &[u8],
) -> Result<ProjectCheckpointSummary, String> {
    prepare_directory(directory)?;
    let snapshot = directory.join(&manifest.snapshot_name);
    let manifest_path = directory.join(format!("{}{}", manifest.checkpoint_id, MANIFEST_SUFFIX));
    if snapshot.exists() || manifest_path.exists() {
        return Err("project checkpoint identity collision; no state was changed".to_owned());
    }

    publish_new_private_file(&snapshot, bytes)
        .map_err(|error| format!("project checkpoint snapshot publication failed: {error}"))?;
    let observed = std::fs::read(&snapshot)
        .map_err(|error| format!("project checkpoint snapshot verification failed: {error}"))?;
    if observed.len() as u64 != manifest.snapshot_byte_len
        || digest(&observed) != manifest.snapshot_digest
    {
        return Err("the written project checkpoint did not match when read back".to_owned());
    }
    let mut manifest_bytes = serde_json::to_vec_pretty(manifest)
        .map_err(|error| format!("project checkpoint manifest serialization failed: {error}"))?;
    manifest_bytes.push(b'\n');
    publish_new_private_file(&manifest_path, &manifest_bytes)
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

/// Publish `bytes` as a new file at `path`, readable by its owner only. A
/// checkpoint's files have unique names, so nothing else is written there and
/// nothing but the file is left in the folder.
#[cfg(not(target_arch = "wasm32"))]
fn publish_new_private_file(path: &Path, bytes: &[u8]) -> std::io::Result<()> {
    #[cfg(unix)]
    {
        crate::io::durable_file::publish_new_bytes_owner_only(path, bytes)
    }
    #[cfg(not(unix))]
    {
        crate::io::durable_file::publish_new_bytes(path, bytes)
    }
}

/// Publish `bytes` at `path` while it holds what `expected` says, readable by
/// its owner only.
#[cfg(not(target_arch = "wasm32"))]
fn publish_private_file(
    path: &Path,
    expected: ExpectedContent,
    bytes: &[u8],
) -> Result<(), crate::io::durable_file::CompareExchangeError> {
    #[cfg(unix)]
    {
        crate::io::durable_file::compare_exchange_bytes_owner_only(path, expected, bytes)
    }
    #[cfg(not(unix))]
    {
        crate::io::durable_file::compare_exchange_bytes(path, expected, bytes)
    }
}

/// The checkpoints in `directory` that belong to the project, each from its
/// manifest. A snapshot is only looked at, for being there at the length its
/// manifest records; its bytes are verified when it is used.
#[cfg(not(target_arch = "wasm32"))]
fn list_directory(directory: &Path, project_id: &str) -> Result<ProjectCheckpointCatalog, String> {
    super::recovery::disk_observation::checkpoint_directory();
    let entries = match std::fs::read_dir(directory) {
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
        // A manifest that cannot be read is quarantined, as the browser store
        // does, so the recovery page states it rather than the checkpoint
        // silently not being there.
        let unreadable = |reason: String| ProjectCheckpointQuarantine {
            label: path
                .file_name()
                .map(|name| name.to_string_lossy().into_owned())
                .unwrap_or_else(|| "checkpoint manifest".to_owned()),
            reason,
            artifacts: vec![path.clone()],
        };
        let bytes = match std::fs::read(&path) {
            Ok(bytes) => bytes,
            Err(error) => {
                quarantined.push(unreadable(format!(
                    "checkpoint manifest could not be read: {error}"
                )));
                continue;
            }
        };
        let manifest = match serde_json::from_slice::<ProjectCheckpointManifest>(&bytes) {
            Ok(manifest) if manifest.project_id == project_id => manifest,
            Ok(_) => continue,
            Err(error) => {
                quarantined.push(unreadable(format!(
                    "checkpoint manifest is invalid: {error}"
                )));
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
            Ok(summary) => match snapshot_present(&summary) {
                Ok(()) => checkpoints.push(summary),
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

/// Whether a listed checkpoint's snapshot is there at the length its
/// manifest records: what a listing can know without reading it.
#[cfg(not(target_arch = "wasm32"))]
fn snapshot_present(summary: &ProjectCheckpointSummary) -> Result<(), String> {
    let ProjectCheckpointLocator::Native { snapshot, .. } = &summary.locator;
    super::recovery::disk_observation::file_stat();
    let metadata = std::fs::metadata(snapshot).map_err(|error| {
        format!(
            "project checkpoint snapshot '{}' could not be found: {error}",
            snapshot.display()
        )
    })?;
    if metadata.len() != summary.snapshot_byte_len {
        return Err("project checkpoint bytes do not match their integrity manifest".to_owned());
    }
    Ok(())
}

#[cfg(not(target_arch = "wasm32"))]
fn read_snapshot_bytes(summary: &ProjectCheckpointSummary) -> Result<Vec<u8>, String> {
    let ProjectCheckpointLocator::Native { snapshot, .. } = &summary.locator;
    super::recovery::disk_observation::project_file();
    std::fs::read(snapshot).map_err(|error| {
        format!(
            "project checkpoint snapshot '{}' could not be read: {error}",
            snapshot.display()
        )
    })
}

/// Remove what retention no longer keeps of the project's checkpoints in
/// `directory`. Artifacts that failed verification are left for the reader:
/// the page lists them and nothing restores from them.
#[cfg(not(target_arch = "wasm32"))]
fn prune_retention(directory: &Path, project_id: &str, budget_bytes: u64) -> Result<(), String> {
    let catalog = list_directory(directory, project_id)?;
    let mut failures = Vec::new();
    for index in retention_removals(&catalog.checkpoints, budget_bytes) {
        let ProjectCheckpointLocator::Native { manifest, snapshot } =
            &catalog.checkpoints[index].locator;
        remove_checkpoint(manifest, snapshot, &mut failures);
    }
    joined(failures)
}

/// Remove each file, the manifest first so a checkpoint stops being listed
/// before its snapshot goes, with the lease an edit of it left; a file
/// already gone is not a failure.
#[cfg(not(target_arch = "wasm32"))]
fn remove_artifacts<'a>(paths: impl IntoIterator<Item = &'a Path>, failures: &mut Vec<String>) {
    for path in paths {
        if let Err(error) = crate::io::durable_file::remove_published(path) {
            failures.push(format!("'{}': {error}", path.display()));
        }
    }
}

/// The manifest keys among `stored`, the project's manifests by key, whose
/// checkpoints retention removes, every autosave first when `drop_autosaves`
/// is set. A manifest that cannot be read is left where it is: the page lists
/// it as set aside.
#[cfg(target_arch = "wasm32")]
fn browser_retention_removals(
    stored: Vec<(String, Option<String>)>,
    budget: u64,
    drop_autosaves: bool,
) -> Vec<String> {
    let manifest_key = |checkpoint: &ProjectCheckpointSummary| {
        let ProjectCheckpointLocator::Browser { manifest_key, .. } = &checkpoint.locator;
        manifest_key.clone()
    };
    let mut checkpoints = stored
        .into_iter()
        .filter_map(|(manifest_key, text)| {
            let manifest = serde_json::from_str::<ProjectCheckpointManifest>(&text?).ok()?;
            let snapshot_key = manifest_key
                .strip_suffix(".manifest")
                .map(|stem| format!("{stem}.snapshot"))?;
            summary_from_manifest(
                manifest,
                ProjectCheckpointLocator::Browser {
                    manifest_key,
                    snapshot_key,
                },
            )
            .ok()
        })
        .collect::<Vec<_>>();
    sort_newest_first(&mut checkpoints);
    let mut removals = Vec::new();
    if drop_autosaves {
        checkpoints.retain(|checkpoint| {
            let autosave = checkpoint.reason == ProjectCheckpointReason::Autosave;
            if autosave {
                removals.push(manifest_key(checkpoint));
            }
            !autosave
        });
    }
    removals.extend(
        retention_removals(&checkpoints, budget)
            .into_iter()
            .map(|index| manifest_key(&checkpoints[index])),
    );
    removals
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
struct BrowserCheckpointAccumulator {
    project_id: String,
    checkpoints: Vec<ProjectCheckpointSummary>,
    quarantined: Vec<ProjectCheckpointQuarantine>,
}

#[cfg(target_arch = "wasm32")]
fn visit_browser_checkpoint_record(
    accumulator: &mut BrowserCheckpointAccumulator,
    manifest_key: String,
    value: Option<String>,
    snapshot_key: String,
    snapshot: Option<Vec<u8>>,
) {
    let Some(value) = value else {
        accumulator.quarantined.push(ProjectCheckpointQuarantine {
            label: manifest_key.clone(),
            reason: "checkpoint manifest is not a JSON string".to_owned(),
            artifact_keys: vec![manifest_key, snapshot_key],
        });
        return;
    };
    let manifest = match serde_json::from_str::<ProjectCheckpointManifest>(&value) {
        Ok(manifest) => manifest,
        Err(error) => {
            accumulator.quarantined.push(ProjectCheckpointQuarantine {
                label: manifest_key.clone(),
                reason: format!("checkpoint manifest is invalid: {error}"),
                artifact_keys: vec![manifest_key, snapshot_key],
            });
            return;
        }
    };
    if manifest.project_id != accumulator.project_id {
        accumulator.quarantined.push(ProjectCheckpointQuarantine {
            label: manifest_key.clone(),
            reason: "checkpoint key ownership and manifest project identity disagree".to_owned(),
            artifact_keys: vec![manifest_key, snapshot_key],
        });
        return;
    }
    let (expected_manifest_key, expected_snapshot_key) = browser_keys(&manifest);
    if manifest_key != expected_manifest_key || snapshot_key != expected_snapshot_key {
        accumulator.quarantined.push(ProjectCheckpointQuarantine {
            label: manifest.checkpoint_id.to_string(),
            reason: "checkpoint manifest is stored under the wrong browser storage keys".to_owned(),
            artifact_keys: vec![manifest_key, snapshot_key],
        });
        return;
    }
    let Some(snapshot) = snapshot else {
        accumulator.quarantined.push(ProjectCheckpointQuarantine {
            label: manifest.checkpoint_id.to_string(),
            reason: "checkpoint snapshot is missing".to_owned(),
            artifact_keys: vec![manifest_key, snapshot_key],
        });
        return;
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
            accumulator.quarantined.push(ProjectCheckpointQuarantine {
                label: manifest_key.clone(),
                reason: error,
                artifact_keys: vec![manifest_key, snapshot_key],
            });
            return;
        }
    };
    match validate_snapshot_bytes(&summary, &snapshot) {
        Ok(_) => accumulator.checkpoints.push(summary),
        Err(error) => accumulator.quarantined.push(ProjectCheckpointQuarantine {
            label: summary.checkpoint_id.to_string(),
            reason: error,
            artifact_keys: vec![manifest_key, snapshot_key],
        }),
    }
}

#[cfg(target_arch = "wasm32")]
pub fn start_list(
    model: &Model,
    complete: impl FnOnce(Result<ProjectCheckpointCatalog, String>) + 'static,
) {
    let project_id = model.workspace.content.project.id().to_string();
    let prefix = browser_project_prefix(&project_id);
    let accumulator = std::rc::Rc::new(std::cell::RefCell::new(BrowserCheckpointAccumulator {
        project_id,
        checkpoints: Vec::new(),
        quarantined: Vec::new(),
    }));
    let visit_accumulator = std::rc::Rc::clone(&accumulator);
    crate::workbench::lifecycle::project_lifecycle::start_browser_checkpoint_list(
        prefix,
        move |manifest_key, value, snapshot_key, snapshot| {
            visit_browser_checkpoint_record(
                &mut visit_accumulator.borrow_mut(),
                manifest_key,
                value,
                snapshot_key,
                snapshot,
            );
        },
        move |result| {
            complete(result.map(|orphan_snapshots| {
                let mut accumulator = accumulator.borrow_mut();
                for snapshot_key in orphan_snapshots {
                    accumulator.quarantined.push(ProjectCheckpointQuarantine {
                        label: snapshot_key.clone(),
                        reason: "checkpoint snapshot has no committed manifest".to_owned(),
                        artifact_keys: vec![snapshot_key],
                    });
                }
                sort_newest_first(&mut accumulator.checkpoints);
                accumulator
                    .quarantined
                    .sort_by(|left, right| right.label.cmp(&left.label));
                ProjectCheckpointCatalog {
                    checkpoints: std::mem::take(&mut accumulator.checkpoints),
                    quarantined: std::mem::take(&mut accumulator.quarantined),
                }
            }));
        },
    );
}

#[cfg(target_arch = "wasm32")]
pub fn start_recovery_copy_bytes(
    summary: ProjectCheckpointSummary,
    destination: PathBuf,
    complete: impl FnOnce(Result<Vec<u8>, String>) + 'static,
) {
    let ProjectCheckpointLocator::Browser { snapshot_key, .. } = &summary.locator;
    crate::workbench::lifecycle::project_lifecycle::start_browser_checkpoint_read(
        snapshot_key.clone(),
        move |result| {
            complete(result.and_then(|bytes| {
                let mut project = validate_snapshot_bytes(&summary, &bytes)?;
                project.file.workspace.project =
                    project.file.workspace.project.fork_copy_at(destination);
                crate::io::project_io::serialize_project_file(&project)
                    .map(String::into_bytes)
                    .map_err(|error| format!("recovery copy serialization failed: {error}"))
            }));
        },
    );
}

#[cfg(target_arch = "wasm32")]
thread_local! {
    /// The browser store's changes that finish after the frame that started
    /// them, with no page waiting on them: how many have finished, and what
    /// each that failed could not do.
    static BROWSER_STORE_CHANGES: std::cell::RefCell<(u64, Vec<String>)> =
        const { std::cell::RefCell::new((0, Vec::new())) };
}

#[cfg(target_arch = "wasm32")]
fn note_browser_store_change(failure: Option<String>) {
    BROWSER_STORE_CHANGES.with(|changes| {
        let mut changes = changes.borrow_mut();
        changes.0 = changes.0.wrapping_add(1);
        changes.1.extend(failure);
    });
}

/// How many changes to the browser store have finished on their own; the
/// Recovery page lists again when the count moves.
#[cfg(target_arch = "wasm32")]
pub fn browser_store_changes() -> u64 {
    BROWSER_STORE_CHANGES.with(|changes| changes.borrow().0)
}

/// What the browser store's own changes could not do, once.
#[cfg(target_arch = "wasm32")]
pub fn take_browser_store_failures() -> Vec<String> {
    BROWSER_STORE_CHANGES.with(|changes| std::mem::take(&mut changes.borrow_mut().1))
}

/// Make `edit` to the checkpoint's stored manifest, while it is still the
/// one the checkpoint was listed from.
#[cfg(target_arch = "wasm32")]
pub fn start_edit(
    summary: &ProjectCheckpointSummary,
    edit: CheckpointEdit,
    complete: impl FnOnce(Result<(), String>) + 'static,
) {
    let ProjectCheckpointLocator::Browser { manifest_key, .. } = &summary.locator;
    let listed = summary.clone();
    crate::workbench::lifecycle::project_lifecycle::start_browser_checkpoint_manifest_update(
        manifest_key.clone(),
        move |stored| {
            let stored = stored.ok_or_else(|| "the checkpoint is no longer stored".to_owned())?;
            let manifest = edited_manifest(&listed, stored.as_bytes(), &edit)?;
            serde_json::to_string(&manifest).map_err(|error| {
                format!("project checkpoint manifest serialization failed: {error}")
            })
        },
        complete,
    );
}

/// Delete a checkpoint: its manifest first, so it is no longer listed, then
/// its snapshot.
#[cfg(target_arch = "wasm32")]
pub fn start_delete(
    summary: &ProjectCheckpointSummary,
    complete: impl FnOnce(Result<(), String>) + 'static,
) {
    let ProjectCheckpointLocator::Browser {
        manifest_key,
        snapshot_key,
    } = &summary.locator;
    crate::workbench::lifecycle::project_lifecycle::start_browser_checkpoint_delete(
        vec![manifest_key.clone(), snapshot_key.clone()],
        complete,
    );
}

/// Delete the records an artifact that failed verification was found as.
#[cfg(target_arch = "wasm32")]
pub fn start_delete_set_aside(
    artifact: &ProjectCheckpointQuarantine,
    complete: impl FnOnce(Result<(), String>) + 'static,
) {
    crate::workbench::lifecycle::project_lifecycle::start_browser_checkpoint_delete(
        artifact.artifact_keys.clone(),
        complete,
    );
}

/// Bring the browser store up to a save that has been written: the version
/// it replaced, when it changed one, is kept as a "Before save" checkpoint;
/// a save of every document removes the autosave; retention runs. The store
/// finishes after this returns, and what it could not do is taken with
/// [`take_browser_store_failures`].
#[cfg(target_arch = "wasm32")]
pub fn start_after_save(model: &Model, replaced: Option<&ProjectSnapshot>, saved_everything: bool) {
    let project_id = model.workspace.content.project.id().to_string();
    let budget = budget_bytes(model);
    let prefix = browser_project_prefix(&project_id);
    let removals = move |stored| browser_retention_removals(stored, budget, saved_everything);
    let sealed = replaced.map(|replaced| {
        crate::time_compat::checked_unix_time_ms()
            .map_err(|error| format!("the time could not be read: {error}"))
            .and_then(|now| seal_checkpoint(replaced, ProjectCheckpointReason::Save, now))
            .and_then(|(manifest, serialized)| {
                serde_json::to_string(&manifest)
                    .map(|manifest_json| (manifest, serialized, manifest_json))
                    .map_err(|error| {
                        format!("project checkpoint manifest serialization failed: {error}")
                    })
            })
    });
    match sealed {
        Some(Ok((manifest, serialized, manifest_json))) => {
            let (manifest_key, snapshot_key) = browser_keys(&manifest);
            crate::workbench::lifecycle::project_lifecycle::start_browser_checkpoint_publish(
                prefix,
                manifest_key,
                snapshot_key,
                serialized.into_bytes(),
                manifest_json,
                removals,
                |result| {
                    note_browser_store_change(
                        result
                            .err()
                            .map(|error| format!("the version it replaced was not kept: {error}")),
                    );
                },
            );
        }
        sealed => {
            if let Some(Err(error)) = sealed {
                note_browser_store_change(Some(format!(
                    "the version it replaced was not kept: {error}"
                )));
            }
            crate::workbench::lifecycle::project_lifecycle::start_browser_checkpoint_prune(
                prefix,
                removals,
                |result| {
                    note_browser_store_change(
                        result.err().map(|error| {
                            format!("older checkpoints were not all removed: {error}")
                        }),
                    );
                },
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::workbench::lifecycle::project_lifecycle::test_host::ServicesHost;

    #[test]
    fn checkpoint_clock_failure_precedes_publication() {
        let state = ServicesHost::default();
        for reason in [
            ProjectCheckpointReason::Manual,
            ProjectCheckpointReason::TechnologyChange,
        ] {
            for epoch in [
                Err("clock unavailable"),
                Ok(std::time::Duration::ZERO),
                Ok(std::time::Duration::MAX),
            ] {
                crate::time_compat::with_unix_epoch(epoch, || {
                    let error = prepare_checkpoint(&state, reason).unwrap_err();
                    assert!(error.contains("timestamped"), "{error}");
                });
            }
        }
    }

    #[test]
    fn manifest_rejects_noncanonical_snapshot_identity() {
        let manifest = ProjectCheckpointManifest {
            schema_version: MANIFEST_SCHEMA_VERSION,
            checkpoint_id: Uuid::nil(),
            project_id: Uuid::new_v4().to_string(),
            project_name: "Precision reference".to_owned(),
            project_revision: 7,
            reason: ProjectCheckpointReason::TechnologyChange,
            created_unix_ms: 42,
            note: None,
            pinned: false,
            snapshot_digest: ContentDigest::from_bytes([0x51; 32]),
            snapshot_byte_len: 128,
            snapshot_name: "wrong.rspiceproj".to_owned(),
        };
        assert!(validate_manifest(&manifest).is_err());
    }

    #[test]
    fn snapshot_integrity_rejects_any_byte_change() {
        let state = ServicesHost::default();
        let project =
            crate::workbench::lifecycle::project_lifecycle::snapshot(&state).expect("snapshot");
        let bytes = crate::io::project_io::serialize_project_file(&project)
            .expect("serialize")
            .into_bytes();
        let summary = ProjectCheckpointSummary {
            checkpoint_id: Uuid::new_v4(),
            project_id: project.file.workspace.project.id().to_string(),
            project_name: project.file.workspace.project.name().to_owned(),
            project_revision: project.file.workspace.project.revision().get(),
            reason: ProjectCheckpointReason::TechnologyChange,
            created_unix_ms: 1,
            note: None,
            pinned: false,
            snapshot_digest: digest(&bytes),
            snapshot_byte_len: bytes.len() as u64,
            #[cfg(not(target_arch = "wasm32"))]
            locator: ProjectCheckpointLocator::Native {
                manifest: PathBuf::from("manifest"),
                snapshot: PathBuf::from("snapshot"),
            },
            #[cfg(target_arch = "wasm32")]
            locator: ProjectCheckpointLocator::Browser {
                manifest_key: "manifest".to_owned(),
                snapshot_key: "snapshot".to_owned(),
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
        changed_state.model.workspace.content.netlist_source = Some("R1 1 0 1k\n.end\n".to_owned());
        assert!(
            !matches_current_state(&summary, &changed_state)
                .expect("changed state digest compares")
        );
    }

    #[test]
    fn checkpoint_closes_over_arbitrary_netlist_veriloga_and_automation_sources() {
        use crate::state::{
            ProjectSourceBundle, ProjectSourceDependency, ProjectSourceFile, ProjectSourceLanguage,
            ProjectSourceOwner, ProjectSourceRegistry, ProjectSourceRole, ProjectSourceRoleBinding,
        };

        fn file(path: &str, content: &str) -> ProjectSourceFile {
            ProjectSourceFile::try_new(path, content).expect("source file fixture is valid")
        }

        fn dependency(importer: &str, imported: &str) -> ProjectSourceDependency {
            ProjectSourceDependency::try_new(importer, imported)
                .expect("source dependency fixture is valid")
        }

        fn role(path: &str, source_role: ProjectSourceRole) -> ProjectSourceRoleBinding {
            ProjectSourceRoleBinding::try_new(path, source_role)
                .expect("source role fixture is valid")
        }

        let netlist = "* production-owned deck\r\nVBIAS bias 0 1.250\r\n.op\r\n.end\r\n";
        let veriloga_root =
            "`include \"physics/thermal_constants.vams\"\nmodule sensor_core; endmodule\n";
        let veriloga_dependency = "`define ROOM_TEMPERATURE 300.15\n";
        let automation_entry =
            "from automation_helpers.naming import run_label\nprint(run_label('tt'))\n";
        let automation_helper = "def run_label(corner):\n    return f'production-{corner}'\n";
        let run_plan = "schema: rspice.run-plan/v1\nanalyses: [op]\n";
        let environment_lock =
            "format = \"rspice-python-lock/v3\"\npython = \">=3.14.0,<3.15.0\"\n";
        let permission_manifest = "schema = \"rspice.permissions/v1\"\nnetwork = \"deny\"\n";

        let veriloga = ProjectSourceBundle::try_new(
            ProjectSourceOwner::code_workspace(ProjectSourceLanguage::VerilogA),
            ProjectSourceLanguage::VerilogA,
            "devices/sensor_core.va",
            veriloga_root,
            [file("physics/thermal_constants.vams", veriloga_dependency)],
            [dependency(
                "devices/sensor_core.va",
                "physics/thermal_constants.vams",
            )],
        )
        .expect("Verilog-A source closure is valid");
        let veriloga_id = veriloga.id();

        let automation = ProjectSourceBundle::try_new_with_roles(
            ProjectSourceOwner::code_workspace(ProjectSourceLanguage::RSpiceAutomation),
            ProjectSourceLanguage::RSpiceAutomation,
            "flows/production_sweep.py",
            automation_entry,
            [
                file("automation_helpers/naming.py", automation_helper),
                file("plans/qualified-release.plan", run_plan),
                file("environment/native-runtime.lock", environment_lock),
                file("security/least-authority.policy", permission_manifest),
            ],
            [
                dependency("flows/production_sweep.py", "automation_helpers/naming.py"),
                dependency("flows/production_sweep.py", "plans/qualified-release.plan"),
                dependency(
                    "flows/production_sweep.py",
                    "environment/native-runtime.lock",
                ),
                dependency(
                    "flows/production_sweep.py",
                    "security/least-authority.policy",
                ),
            ],
            [
                role(
                    "flows/production_sweep.py",
                    ProjectSourceRole::AutomationEntry,
                ),
                role(
                    "plans/qualified-release.plan",
                    ProjectSourceRole::AutomationRunPlan,
                ),
                role(
                    "environment/native-runtime.lock",
                    ProjectSourceRole::AutomationEnvironmentLock,
                ),
                role(
                    "security/least-authority.policy",
                    ProjectSourceRole::AutomationPermissionManifest,
                ),
            ],
        )
        .expect("Automation source closure is valid");
        let automation_id = automation.id();

        let mut state = ServicesHost::default();
        assert!(
            state
                .model
                .workspace
                .content
                .make_netlist_editable_copy(netlist)
        );
        state.model.workspace.content.project_sources =
            ProjectSourceRegistry::try_from_bundles([veriloga, automation])
                .expect("source registry is valid");
        state.model.workspace.content.project_sources_dirty = true;

        let (manifest, serialized) = prepare_checkpoint(&state, ProjectCheckpointReason::Manual)
            .expect("complete Code workspace checkpoint prepares");
        assert_eq!(manifest.snapshot_digest, digest(serialized.as_bytes()));
        assert_eq!(manifest.snapshot_byte_len, serialized.len() as u64);

        let restored = crate::io::project_io::load_project_text(&serialized, None)
            .expect("checkpoint project restores");
        assert_eq!(
            restored.file.workspace.netlist_source.as_deref(),
            Some(netlist)
        );

        let restored_veriloga = restored
            .file
            .workspace
            .project_sources
            .get_bundle(veriloga_id)
            .expect("Verilog-A closure restores by stable identity");
        assert_eq!(
            restored_veriloga.root().logical_path(),
            "devices/sensor_core.va"
        );
        assert_eq!(restored_veriloga.root().content(), veriloga_root);
        assert_eq!(
            restored_veriloga.file_content("physics/thermal_constants.vams"),
            Some(veriloga_dependency)
        );

        let restored_automation = restored
            .file
            .workspace
            .project_sources
            .get_bundle(automation_id)
            .expect("Automation closure restores by stable identity");
        assert_eq!(
            restored_automation.root().logical_path(),
            "flows/production_sweep.py"
        );
        assert_eq!(restored_automation.root().content(), automation_entry);
        assert_eq!(
            restored_automation.file_content("automation_helpers/naming.py"),
            Some(automation_helper)
        );
        assert_eq!(
            restored_automation.role_for_path("plans/qualified-release.plan"),
            Some(ProjectSourceRole::AutomationRunPlan)
        );
        assert_eq!(
            restored_automation.role_for_path("environment/native-runtime.lock"),
            Some(ProjectSourceRole::AutomationEnvironmentLock)
        );
        assert_eq!(
            restored_automation.role_for_path("security/least-authority.policy"),
            Some(ProjectSourceRole::AutomationPermissionManifest)
        );

        assert!(
            state
                .model
                .workspace
                .content
                .replace_editable_netlist_source("* changed after checkpoint\n.end\n".to_owned())
        );
        assert_ne!(
            crate::workbench::lifecycle::project_lifecycle::snapshot(&state)
                .expect("changed project snapshots")
                .file
                .workspace
                .netlist_source,
            restored.file.workspace.netlist_source
        );
        assert_eq!(manifest.snapshot_digest, digest(serialized.as_bytes()));
    }

    #[test]
    #[cfg(not(target_arch = "wasm32"))]
    fn native_checkpoint_roundtrip_and_recovery_copy_are_non_destructive() {
        let directory =
            std::env::temp_dir().join(format!("rspice-project-checkpoint-{}", Uuid::new_v4()));
        std::fs::create_dir_all(&directory).expect("create fixture directory");
        let mut state = ServicesHost::default();
        state
            .model
            .workspace
            .content
            .project
            .set_path(directory.join("source.rspiceproj"));
        let source_id = state.model.workspace.content.project.id();
        let source_revision = state.model.workspace.content.project.revision();

        let checkpoint = create(&mut state, ProjectCheckpointReason::TechnologyChange)
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
        let listed = list(&state.model).expect("checkpoint catalog reads");
        assert_eq!(listed.checkpoints.len(), 1);
        assert!(listed.quarantined.is_empty());
        assert_eq!(
            listed.checkpoints[0].checkpoint_id(),
            checkpoint.checkpoint_id()
        );
        let restored = load(&listed.checkpoints[0]).expect("checkpoint bytes restore");
        assert_eq!(restored.file.workspace.project.id(), source_id);

        let destination = directory.join("recovered.rspiceproj");
        publish_recovery_copy(&listed.checkpoints[0], &destination).expect("copy publishes");
        let copy = crate::io::load_project_file(&destination).expect("copy loads");
        assert_ne!(copy.file.workspace.project.id(), source_id);
        assert_eq!(copy.file.workspace.project.revision().get(), 1);

        std::fs::write(&destination, b"external bytes").expect("replace fixture copy");
        assert!(publish_recovery_copy(&listed.checkpoints[0], &destination).is_err());
        assert_eq!(
            std::fs::read(&destination).expect("destination remains readable"),
            b"external bytes"
        );
        let _ = std::fs::remove_dir_all(&directory);
    }

    #[test]
    #[cfg(not(target_arch = "wasm32"))]
    fn owned_corrupt_checkpoint_is_exposed_as_quarantine() {
        let directory =
            std::env::temp_dir().join(format!("rspice-project-quarantine-{}", Uuid::new_v4()));
        std::fs::create_dir_all(&directory).expect("create fixture directory");
        let mut state = ServicesHost::default();
        state
            .model
            .workspace
            .content
            .project
            .set_path(directory.join("source.rspiceproj"));
        let checkpoint = create(&mut state, ProjectCheckpointReason::TechnologyChange)
            .expect("checkpoint publishes");
        let ProjectCheckpointLocator::Native { snapshot, .. } = &checkpoint.locator;
        std::fs::write(snapshot, b"corrupt checkpoint").expect("corrupt owned snapshot");

        let catalog = list(&state.model).expect("catalog returns quarantine diagnostics");
        assert!(catalog.checkpoints.is_empty());
        assert_eq!(catalog.quarantined.len(), 1);
        assert!(
            catalog.quarantined[0]
                .reason()
                .contains("integrity manifest")
        );
        let _ = std::fs::remove_dir_all(&directory);
    }

    #[test]
    #[cfg(not(target_arch = "wasm32"))]
    fn a_checkpoint_listed_from_its_manifest_is_verified_when_restored() {
        let directory =
            std::env::temp_dir().join(format!("rspice-project-verify-{}", Uuid::new_v4()));
        std::fs::create_dir_all(&directory).expect("create fixture directory");
        let mut state = ServicesHost::default();
        state
            .model
            .workspace
            .content
            .project
            .set_path(directory.join("source.rspiceproj"));
        let checkpoint =
            create(&mut state, ProjectCheckpointReason::Manual).expect("checkpoint publishes");
        let ProjectCheckpointLocator::Native { snapshot, .. } = &checkpoint.locator;
        let mut bytes = std::fs::read(snapshot).expect("snapshot reads");
        let last = bytes.len() - 1;
        bytes[last] ^= 0x01;
        std::fs::write(snapshot, &bytes).expect("corrupt owned snapshot in place");

        let catalog = list(&state.model).expect("catalog reads");
        assert_eq!(catalog.checkpoints.len(), 1);
        assert!(catalog.quarantined.is_empty());
        let refused = load(&catalog.checkpoints[0]).expect_err("restore verifies the bytes");
        assert!(refused.contains("integrity manifest"), "{refused}");
        let _ = std::fs::remove_dir_all(&directory);
    }

    /// Publish one checkpoint of `state` into `directory`, dated `created`,
    /// through the store's own write, then retention under `budget_bytes`.
    #[cfg(not(target_arch = "wasm32"))]
    fn publish_dated(
        state: &ServicesHost,
        directory: &Path,
        reason: ProjectCheckpointReason,
        created: u64,
        budget_bytes: u64,
    ) -> ProjectCheckpointSummary {
        CheckpointCapture {
            project: crate::workbench::lifecycle::project_lifecycle::snapshot(state)
                .expect("snapshot"),
            reason,
            created_unix_ms: created,
            directory: directory.to_path_buf(),
            budget_bytes,
        }
        .publish()
        .expect("checkpoint publishes")
        .summary
    }

    /// Rewrite a published checkpoint's manifest with `pinned` and `note`, as
    /// the reader's edit leaves it.
    #[cfg(not(target_arch = "wasm32"))]
    fn mark(summary: &ProjectCheckpointSummary, pinned: bool, note: Option<&str>) {
        let ProjectCheckpointLocator::Native { manifest, .. } = &summary.locator;
        let mut stored: ProjectCheckpointManifest =
            serde_json::from_slice(&std::fs::read(manifest).expect("manifest reads"))
                .expect("manifest parses");
        stored.pinned = pinned;
        stored.note = note.map(str::to_owned);
        std::fs::write(
            manifest,
            serde_json::to_vec_pretty(&stored).expect("manifest encodes"),
        )
        .expect("manifest writes");
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn fixture_directory(label: &str) -> (ServicesHost, PathBuf) {
        let directory = std::env::temp_dir().join(format!("rspice-{label}-{}", Uuid::new_v4()));
        std::fs::create_dir_all(&directory).expect("create fixture directory");
        let mut state = ServicesHost::default();
        state
            .model
            .workspace
            .content
            .project
            .set_path(directory.join("source.rspiceproj"));
        (state, directory)
    }

    /// Nine checkpoints of every kind, oldest first, under a budget that
    /// holds two snapshots: retention keeps both manual checkpoints and the
    /// pinned one, the newest autosave, and the two newest of the others,
    /// and removes the rest from disk.
    #[test]
    #[cfg(not(target_arch = "wasm32"))]
    fn retention_keeps_manual_and_pinned_checkpoints_and_drops_the_oldest_automatic_ones() {
        use ProjectCheckpointReason as Reason;
        let (state, directory) = fixture_directory("checkpoint-retention");
        let recovery = checkpoint_directory(&state.model).expect("directory");
        let project =
            crate::workbench::lifecycle::project_lifecycle::snapshot(&state).expect("snapshot");
        let plan = [
            (Reason::Manual, false, None),
            (Reason::TechnologyChange, false, None),
            (Reason::Save, true, None),
            (Reason::Autosave, false, None),
            (Reason::LibraryDeletion, false, None),
            (Reason::Manual, false, Some("before rewiring the bias")),
            (Reason::Autosave, false, None),
            (Reason::Save, false, None),
            (Reason::PublicationRestore, false, None),
        ];
        let mut published = Vec::new();
        for (index, (reason, pinned, note)) in plan.into_iter().enumerate() {
            // Written without retention, so all nine are there to prune.
            let (manifest, serialized) =
                seal_checkpoint(&project, reason, 1_000 + index as u64).expect("seals");
            let summary =
                publish_native(&recovery, &manifest, serialized.as_bytes()).expect("publishes");
            if pinned || note.is_some() {
                mark(&summary, pinned, note);
            }
            published.push(summary);
        }
        assert_eq!(list(&state.model).expect("lists").checkpoints.len(), 9);

        let snapshot = published[0].snapshot_byte_len();
        prune_retention(&recovery, &published[0].project_id, 2 * snapshot).expect("prunes");

        let kept = list(&state.model).expect("lists");
        assert!(kept.quarantined.is_empty());
        let kept = kept
            .checkpoints
            .iter()
            .map(|checkpoint| (checkpoint.created_unix_ms(), checkpoint.title().to_owned()))
            .collect::<Vec<_>>();
        assert_eq!(
            kept,
            [
                (1_008, "Before restore from publication".to_owned()),
                (1_007, "Before save".to_owned()),
                (1_006, "Autosave".to_owned()),
                (1_005, "before rewiring the bias".to_owned()),
                (1_002, "Before save".to_owned()),
                (1_000, "Manual checkpoint".to_owned()),
            ]
        );
        for removed in [1, 3, 4] {
            let ProjectCheckpointLocator::Native { manifest, snapshot } =
                &published[removed].locator;
            assert!(
                !manifest.exists() && !snapshot.exists(),
                "{removed} is on disk"
            );
        }
        let _ = std::fs::remove_dir_all(&directory);
    }

    /// However many automatic checkpoints follow it, a manual one is never
    /// what retention removes.
    #[test]
    #[cfg(not(target_arch = "wasm32"))]
    fn a_manual_checkpoint_survives_any_number_of_automatic_ones() {
        use ProjectCheckpointReason as Reason;
        let (state, directory) = fixture_directory("checkpoint-manual-survives");
        let recovery = checkpoint_directory(&state.model).expect("directory");
        let manual = publish_dated(&state, &recovery, Reason::Manual, 1_000, 1);
        let automatic = [
            Reason::Save,
            Reason::Autosave,
            Reason::TechnologyChange,
            Reason::LibraryDeletion,
            Reason::Restore,
            Reason::PublicationRestore,
        ];
        for index in 0..30_u64 {
            let reason = automatic[usize::try_from(index).expect("small") % automatic.len()];
            publish_dated(&state, &recovery, reason, 2_000 + index, 1);
        }
        let kept = list(&state.model).expect("lists").checkpoints;
        assert!(
            kept.iter()
                .any(|checkpoint| checkpoint.checkpoint_id() == manual.checkpoint_id()),
            "the manual checkpoint was removed"
        );
        // Under a one-byte budget: the manual one, the newest autosave and
        // the newest of the others.
        assert_eq!(
            kept.iter()
                .map(ProjectCheckpointSummary::reason)
                .collect::<Vec<_>>(),
            [Reason::PublicationRestore, Reason::Autosave, Reason::Manual]
        );
        let _ = std::fs::remove_dir_all(&directory);
    }

    /// A saved project's checkpoints are kept beside its file in a folder
    /// named for it, so two projects in one folder never share one; an
    /// unsaved project's are kept in a folder named for the project.
    #[test]
    #[cfg(not(target_arch = "wasm32"))]
    fn each_project_keeps_its_checkpoints_in_a_folder_of_its_own() {
        let root = std::env::temp_dir().join(format!("rspice-checkpoint-dirs-{}", Uuid::new_v4()));
        let id = Uuid::new_v4().to_string();
        assert_eq!(
            directory_for(Some(&root.join("amp.rspiceproj")), &id).expect("saved"),
            root.join("amp.rspice-recovery")
        );
        assert_ne!(
            directory_for(Some(&root.join("amp.rspiceproj")), &id).expect("saved"),
            directory_for(Some(&root.join("bias.rspiceproj")), &id).expect("saved")
        );
        let _root = keep_unsaved_checkpoints_in(root.join("unsaved"));
        assert_eq!(
            directory_for(None, &id).expect("unsaved"),
            root.join("unsaved").join(&id)
        );
    }

    /// The automatic budget is the Preferences' figure, 512 MiB as shipped.
    #[test]
    fn the_automatic_budget_is_read_from_the_preferences() {
        let mut state = ServicesHost::default();
        assert_eq!(budget_bytes(&state.model), 512 * MIB);
        state
            .model
            .preferences
            .set_scalar(
                rspice_app_state::workbench::preferences::ScalarPreference::CheckpointBudgetMib,
                64,
            )
            .expect("in range");
        assert_eq!(budget_bytes(&state.model), 64 * MIB);
    }
}
