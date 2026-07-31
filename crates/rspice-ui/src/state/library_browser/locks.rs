//! Runtime collaboration-lock authority for project library mutations.
//!
//! A device-local project does not invent distributed locks. When a
//! collaboration service supplies an authoritative snapshot, every library
//! mutation is bound to that exact project/catalog revision and fails closed
//! if the snapshot becomes stale or a foreign lease intersects its target.

use std::collections::HashSet;

use serde::{Deserialize, Serialize};
use sha2::{Digest as _, Sha256};
use uuid::Uuid;

use crate::product::{ContentDigest, ObjectRevision, ProjectId};
use crate::state::{ProjectLibraryMutation, canonical_cell_view_owner_key};

const LOCK_SNAPSHOT_SCHEMA_VERSION: u16 = 1;
const MAX_LIBRARY_EDIT_LOCKS: usize = 4_096;
const MAX_LOCK_TEXT_SCALARS: usize = 240;

/// Hierarchical resource protected by one collaboration lease.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "scope", rename_all = "snake_case", deny_unknown_fields)]
pub enum ProjectLibraryEditLockScope {
    Library {
        library: String,
    },
    Cell {
        library: String,
        cell: String,
    },
    View {
        library: String,
        cell: String,
        view: String,
    },
}

impl ProjectLibraryEditLockScope {
    fn validate(&self) -> Result<(), String> {
        match self {
            Self::Library { library } => validate_segment("lock.library", library),
            Self::Cell { library, cell } => {
                validate_segment("lock.library", library)?;
                validate_segment("lock.cell", cell)
            }
            Self::View {
                library,
                cell,
                view,
            } => {
                validate_segment("lock.library", library)?;
                validate_segment("lock.cell", cell)?;
                validate_segment("lock.view", view)
            }
        }
    }

    #[must_use]
    pub fn display_path(&self) -> String {
        match self {
            Self::Library { library } => library.clone(),
            Self::Cell { library, cell } => format!("{library}/{cell}"),
            Self::View {
                library,
                cell,
                view,
            } => format!("{library}/{cell}/{view}"),
        }
    }

    fn canonical_parts(&self) -> (&str, Option<&str>, Option<&str>) {
        match self {
            Self::Library { library } => (library, None, None),
            Self::Cell { library, cell } => (library, Some(cell), None),
            Self::View {
                library,
                cell,
                view,
            } => (library, Some(cell), Some(view)),
        }
    }
}

/// One foreign or locally-held lease reported by the collaboration authority.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProjectLibraryEditLock {
    lock_id: Uuid,
    owner_id: String,
    authority_id: String,
    scope: ProjectLibraryEditLockScope,
    acquired_unix_ms: u64,
    lease_until_unix_ms: u64,
}

impl ProjectLibraryEditLock {
    #[must_use]
    pub fn new(
        lock_id: Uuid,
        owner_id: impl Into<String>,
        authority_id: impl Into<String>,
        scope: ProjectLibraryEditLockScope,
        acquired_unix_ms: u64,
        lease_until_unix_ms: u64,
    ) -> Self {
        Self {
            lock_id,
            owner_id: owner_id.into(),
            authority_id: authority_id.into(),
            scope,
            acquired_unix_ms,
            lease_until_unix_ms,
        }
    }

    fn validate(&self) -> Result<(), String> {
        if self.lock_id.is_nil() {
            return Err("library edit lock identity must not be nil".to_owned());
        }
        validate_text("lock.owner_id", &self.owner_id)?;
        validate_text("lock.authority_id", &self.authority_id)?;
        self.scope.validate()?;
        if self.lease_until_unix_ms <= self.acquired_unix_ms {
            return Err("library edit lock lease must end after acquisition".to_owned());
        }
        Ok(())
    }

    #[must_use]
    pub const fn lock_id(&self) -> Uuid {
        self.lock_id
    }

    #[must_use]
    pub fn owner_id(&self) -> &str {
        &self.owner_id
    }

    #[must_use]
    pub fn authority_id(&self) -> &str {
        &self.authority_id
    }

    #[must_use]
    pub fn scope(&self) -> &ProjectLibraryEditLockScope {
        &self.scope
    }

    #[must_use]
    pub const fn acquired_unix_ms(&self) -> u64 {
        self.acquired_unix_ms
    }

    #[must_use]
    pub const fn lease_until_unix_ms(&self) -> u64 {
        self.lease_until_unix_ms
    }
}

/// Exact, digest-sealed lock snapshot delivered by an external authority.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProjectLibraryLockSnapshot {
    schema_version: u16,
    project_id: ProjectId,
    generation: u64,
    project_revision: ObjectRevision,
    library_revision: u64,
    authority_id: String,
    locks: Vec<ProjectLibraryEditLock>,
    snapshot_digest: ContentDigest,
}

#[derive(Serialize)]
struct ProjectLibraryLockSnapshotPayload<'a> {
    schema_version: u16,
    project_id: ProjectId,
    generation: u64,
    project_revision: ObjectRevision,
    library_revision: u64,
    authority_id: &'a str,
    locks: &'a [ProjectLibraryEditLock],
}

impl ProjectLibraryLockSnapshot {
    pub fn try_new(
        project_id: ProjectId,
        generation: u64,
        project_revision: ObjectRevision,
        library_revision: u64,
        authority_id: impl Into<String>,
        mut locks: Vec<ProjectLibraryEditLock>,
    ) -> Result<Self, String> {
        locks.sort_by_key(ProjectLibraryEditLock::lock_id);
        let mut snapshot = Self {
            schema_version: LOCK_SNAPSHOT_SCHEMA_VERSION,
            project_id,
            generation,
            project_revision,
            library_revision,
            authority_id: authority_id.into(),
            locks,
            snapshot_digest: ContentDigest::from_bytes([0; 32]),
        };
        snapshot.snapshot_digest = snapshot.calculate_digest()?;
        snapshot.validate()?;
        Ok(snapshot)
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.schema_version != LOCK_SNAPSHOT_SCHEMA_VERSION {
            return Err(format!(
                "unsupported project library lock snapshot schema {}",
                self.schema_version
            ));
        }
        if self.project_id.as_uuid().is_nil() {
            return Err(
                "project library lock snapshot project identity must not be nil".to_owned(),
            );
        }
        if self.generation == 0 {
            return Err(
                "project library lock snapshot generation must be greater than zero".to_owned(),
            );
        }
        if self.project_revision.get() == 0 {
            return Err(
                "project library lock snapshot revision must be greater than zero".to_owned(),
            );
        }
        validate_text("lock_snapshot.authority_id", &self.authority_id)?;
        if self.locks.len() > MAX_LIBRARY_EDIT_LOCKS {
            return Err(format!(
                "project library lock snapshot exceeds {MAX_LIBRARY_EDIT_LOCKS} locks"
            ));
        }
        let mut identities = HashSet::with_capacity(self.locks.len());
        for lock in &self.locks {
            lock.validate()?;
            if !identities.insert(lock.lock_id) {
                return Err(format!(
                    "project library lock identity {} is duplicated",
                    lock.lock_id
                ));
            }
        }
        if !self
            .locks
            .windows(2)
            .all(|pair| pair[0].lock_id < pair[1].lock_id)
        {
            return Err("project library locks are not in canonical identity order".to_owned());
        }
        if self.calculate_digest()? != self.snapshot_digest {
            return Err(
                "project library lock snapshot digest does not match its content".to_owned(),
            );
        }
        Ok(())
    }

    fn calculate_digest(&self) -> Result<ContentDigest, String> {
        let bytes = serde_json::to_vec(&ProjectLibraryLockSnapshotPayload {
            schema_version: self.schema_version,
            project_id: self.project_id,
            generation: self.generation,
            project_revision: self.project_revision,
            library_revision: self.library_revision,
            authority_id: &self.authority_id,
            locks: &self.locks,
        })
        .map_err(|error| format!("project library lock snapshot serialization failed: {error}"))?;
        Ok(ContentDigest::from_bytes(Sha256::digest(bytes).into()))
    }

    #[must_use]
    pub const fn project_id(&self) -> ProjectId {
        self.project_id
    }

    #[must_use]
    pub const fn generation(&self) -> u64 {
        self.generation
    }

    #[must_use]
    pub const fn project_revision(&self) -> ObjectRevision {
        self.project_revision
    }

    #[must_use]
    pub const fn library_revision(&self) -> u64 {
        self.library_revision
    }

    #[must_use]
    pub fn authority_id(&self) -> &str {
        &self.authority_id
    }

    #[must_use]
    pub fn locks(&self) -> &[ProjectLibraryEditLock] {
        &self.locks
    }

    #[must_use]
    pub const fn snapshot_digest(&self) -> ContentDigest {
        self.snapshot_digest
    }
}

/// Runtime lock mode. Local projects remain explicitly unmanaged; a
/// collaboration session must install monotonically newer authority snapshots.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum ProjectLibraryLockAuthority {
    #[default]
    UnmanagedLocal,
    Authoritative(ProjectLibraryLockSnapshot),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProjectLibraryLockBlockReason {
    WrongProject {
        expected: ProjectId,
        observed: ProjectId,
    },
    StaleSnapshot {
        snapshot_project_revision: ObjectRevision,
        current_project_revision: ObjectRevision,
        snapshot_library_revision: u64,
        current_library_revision: u64,
    },
    Locked {
        lock: ProjectLibraryEditLock,
    },
}

impl std::fmt::Display for ProjectLibraryLockBlockReason {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::WrongProject { expected, observed } => write!(
                formatter,
                "collaboration lock authority belongs to project {observed}, not current project {expected}"
            ),
            Self::StaleSnapshot {
                snapshot_project_revision,
                current_project_revision,
                snapshot_library_revision,
                current_library_revision,
            } => write!(
                formatter,
                "collaboration lock snapshot is stale (project {} vs {}, library {} vs {}); synchronize before editing",
                snapshot_project_revision.get(),
                current_project_revision.get(),
                snapshot_library_revision,
                current_library_revision
            ),
            Self::Locked { lock } => write!(
                formatter,
                "{} is locked by {} under authority {} (lease {})",
                lock.scope.display_path(),
                lock.owner_id,
                lock.authority_id,
                lock.lock_id
            ),
        }
    }
}

impl ProjectLibraryLockAuthority {
    pub fn install_authoritative(
        &mut self,
        snapshot: ProjectLibraryLockSnapshot,
    ) -> Result<(), String> {
        snapshot.validate()?;
        if let Self::Authoritative(current) = self {
            if snapshot.project_id != current.project_id {
                return Err("a lock authority snapshot cannot switch projects in place".to_owned());
            }
            if snapshot.authority_id != current.authority_id {
                return Err(
                    "a lock authority snapshot cannot switch authorities in place".to_owned(),
                );
            }
            if snapshot.generation <= current.generation {
                return Err(format!(
                    "project library lock snapshot generation {} is not newer than {}",
                    snapshot.generation, current.generation
                ));
            }
        }
        *self = Self::Authoritative(snapshot);
        Ok(())
    }

    #[must_use]
    pub fn snapshot(&self) -> Option<&ProjectLibraryLockSnapshot> {
        match self {
            Self::UnmanagedLocal => None,
            Self::Authoritative(snapshot) => Some(snapshot),
        }
    }

    pub fn require_mutation_authority(
        &self,
        project_id: ProjectId,
        project_revision: ObjectRevision,
        library_revision: u64,
        mutation: &ProjectLibraryMutation,
    ) -> Result<(), ProjectLibraryLockBlockReason> {
        let Self::Authoritative(snapshot) = self else {
            return Ok(());
        };
        if snapshot.project_id != project_id {
            return Err(ProjectLibraryLockBlockReason::WrongProject {
                expected: project_id,
                observed: snapshot.project_id,
            });
        }
        if snapshot.project_revision != project_revision
            || snapshot.library_revision != library_revision
        {
            return Err(ProjectLibraryLockBlockReason::StaleSnapshot {
                snapshot_project_revision: snapshot.project_revision,
                current_project_revision: project_revision,
                snapshot_library_revision: snapshot.library_revision,
                current_library_revision: library_revision,
            });
        }
        if matches!(mutation, ProjectLibraryMutation::RollbackPublication { .. })
            && let Some(lock) = snapshot.locks.first()
        {
            return Err(ProjectLibraryLockBlockReason::Locked { lock: lock.clone() });
        }
        if let Some(lock) = snapshot
            .locks
            .iter()
            .find(|lock| scope_conflicts(&lock.scope, mutation))
        {
            return Err(ProjectLibraryLockBlockReason::Locked { lock: lock.clone() });
        }
        Ok(())
    }
}

fn scope_conflicts(scope: &ProjectLibraryEditLockScope, mutation: &ProjectLibraryMutation) -> bool {
    mutation_targets(mutation)
        .iter()
        .any(|target| scopes_intersect(scope, target))
}

fn mutation_targets(mutation: &ProjectLibraryMutation) -> Vec<ProjectLibraryEditLockScope> {
    match mutation {
        ProjectLibraryMutation::CreateCell { library, cell } => {
            vec![ProjectLibraryEditLockScope::Cell {
                library: library.clone(),
                cell: cell.clone(),
            }]
        }
        ProjectLibraryMutation::CreateView {
            library,
            cell,
            view,
        }
        | ProjectLibraryMutation::DeleteView {
            library,
            cell,
            view,
        } => vec![ProjectLibraryEditLockScope::View {
            library: library.clone(),
            cell: cell.clone(),
            view: view.clone(),
        }],
        ProjectLibraryMutation::CopyCell {
            target_library,
            target_cell,
            ..
        } => vec![ProjectLibraryEditLockScope::Cell {
            library: target_library.clone(),
            cell: target_cell.clone(),
        }],
        ProjectLibraryMutation::RenameCell {
            library,
            from_cell,
            to_cell,
        } => vec![
            ProjectLibraryEditLockScope::Cell {
                library: library.clone(),
                cell: from_cell.clone(),
            },
            ProjectLibraryEditLockScope::Cell {
                library: library.clone(),
                cell: to_cell.clone(),
            },
        ],
        ProjectLibraryMutation::DeleteCell { library, cell } => {
            vec![ProjectLibraryEditLockScope::Cell {
                library: library.clone(),
                cell: cell.clone(),
            }]
        }
        ProjectLibraryMutation::RollbackPublication { .. } => Vec::new(),
    }
}

fn scopes_intersect(
    left: &ProjectLibraryEditLockScope,
    right: &ProjectLibraryEditLockScope,
) -> bool {
    let (left_library, left_cell, left_view) = left.canonical_parts();
    let (right_library, right_cell, right_view) = right.canonical_parts();
    if !left_library.eq_ignore_ascii_case(right_library) {
        return false;
    }
    match (left_cell, right_cell) {
        (None, _) | (_, None) => true,
        (Some(left_cell), Some(right_cell)) => {
            if canonical_cell_view_owner_key(left_library, left_cell, "")
                != canonical_cell_view_owner_key(right_library, right_cell, "")
            {
                return false;
            }
            match (left_view, right_view) {
                (None, _) | (_, None) => true,
                (Some(left_view), Some(right_view)) => {
                    canonical_cell_view_owner_key(left_library, left_cell, left_view)
                        == canonical_cell_view_owner_key(right_library, right_cell, right_view)
                }
            }
        }
    }
}

fn validate_text(field: &str, value: &str) -> Result<(), String> {
    if value.is_empty() || value != value.trim() {
        return Err(format!("{field} is required and must be trimmed"));
    }
    if value.chars().count() > MAX_LOCK_TEXT_SCALARS {
        return Err(format!(
            "{field} exceeds {MAX_LOCK_TEXT_SCALARS} Unicode scalar values"
        ));
    }
    if value.chars().any(char::is_control) {
        return Err(format!("{field} contains a control character"));
    }
    Ok(())
}

fn validate_segment(field: &str, value: &str) -> Result<(), String> {
    validate_text(field, value)?;
    if value.contains(['/', '\\']) {
        return Err(format!("{field} must be one library path segment"));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn snapshot(
        project_id: ProjectId,
        project_revision: ObjectRevision,
        library_revision: u64,
        scope: ProjectLibraryEditLockScope,
    ) -> ProjectLibraryLockSnapshot {
        ProjectLibraryLockSnapshot::try_new(
            project_id,
            1,
            project_revision,
            library_revision,
            "org-lock-service",
            vec![ProjectLibraryEditLock::new(
                Uuid::new_v4(),
                "engineer@example.test",
                "org-lock-service",
                scope,
                10,
                20,
            )],
        )
        .expect("valid lock snapshot")
    }

    #[test]
    fn hierarchical_locks_block_exact_mutation_targets() {
        let project_id = ProjectId::new();
        let project_revision = ObjectRevision::INITIAL;
        let authority = ProjectLibraryLockAuthority::Authoritative(snapshot(
            project_id,
            project_revision,
            8,
            ProjectLibraryEditLockScope::Cell {
                library: "work".to_owned(),
                cell: "amp".to_owned(),
            },
        ));

        let blocked = authority.require_mutation_authority(
            project_id,
            project_revision,
            8,
            &ProjectLibraryMutation::CreateView {
                library: "WORK".to_owned(),
                cell: "AMP".to_owned(),
                view: "symbol".to_owned(),
            },
        );
        assert!(matches!(
            blocked,
            Err(ProjectLibraryLockBlockReason::Locked { .. })
        ));
        authority
            .require_mutation_authority(
                project_id,
                project_revision,
                8,
                &ProjectLibraryMutation::CreateCell {
                    library: "work".to_owned(),
                    cell: "other".to_owned(),
                },
            )
            .expect("an unrelated cell remains editable");
    }

    #[test]
    fn stale_or_tampered_lock_authority_fails_closed() {
        let project_id = ProjectId::new();
        let mut authority = ProjectLibraryLockAuthority::default();
        let first = snapshot(
            project_id,
            ObjectRevision::INITIAL,
            4,
            ProjectLibraryEditLockScope::Library {
                library: "work".to_owned(),
            },
        );
        authority
            .install_authoritative(first.clone())
            .expect("first snapshot installs");
        assert!(matches!(
            authority.require_mutation_authority(
                project_id,
                ObjectRevision::INITIAL,
                5,
                &ProjectLibraryMutation::CreateCell {
                    library: "other".to_owned(),
                    cell: "amp".to_owned(),
                },
            ),
            Err(ProjectLibraryLockBlockReason::StaleSnapshot { .. })
        ));
        assert!(authority.install_authoritative(first).is_err());

        let mut json = serde_json::to_value(authority.snapshot().unwrap()).unwrap();
        json["authority_id"] = serde_json::json!("attacker");
        let tampered: ProjectLibraryLockSnapshot = serde_json::from_value(json).unwrap();
        assert!(tampered.validate().is_err());
    }
}
