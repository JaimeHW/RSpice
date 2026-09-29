//! Native destination authority and project publication through host storage.

use super::{NativeBindingReceipt, PersistenceError, ProjectBytes, serialized_project};
use crate::lifecycle::ProjectLifecycleError;
use crate::registry::DocumentRegistry;
use crate::{DecodedProject, ProjectFile, ProjectIoError};
use rspice_app_types::product::ContentDigest;
use rspice_design::hierarchy::HierarchySourceFiles;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NativeBinding {
    pub canonical_path: PathBuf,
    pub accepted_digest: ContentDigest,
}

impl NativeBinding {
    pub fn native_receipt(&self, project_id: &str) -> NativeBindingReceipt {
        NativeBindingReceipt {
            canonical_path: self.canonical_path.clone(),
            project_id: project_id.to_owned(),
            accepted_digest: self.accepted_digest,
        }
    }
}

impl NativeBinding {
    /// Explicit opening accepts the exact bytes read after recovery completes.
    pub fn open(
        storage: &impl NativeProjectStorage,
        path: &Path,
        source_files: impl HierarchySourceFiles,
    ) -> Result<(DecodedProject, Self), PersistenceError> {
        let canonical_path = storage.normalize_path(path)?;
        storage.reconcile_publication(&canonical_path)?;
        let bytes = storage.read_project(&canonical_path)?;
        let accepted_digest = bytes.digest();
        let project = bytes.decode(Some(&canonical_path), source_files)?;
        Ok((
            project,
            Self {
                canonical_path,
                accepted_digest,
            },
        ))
    }

    /// A remembered path alone is not save authority. Admit the receipt before
    /// storage access, then its exact bytes before parsing and project identity.
    pub fn restore(
        storage: &impl NativeProjectStorage,
        path: &Path,
        session_project_id: &str,
        receipt: &NativeBindingReceipt,
        source_files: impl HierarchySourceFiles,
    ) -> Result<(DecodedProject, Self), PersistenceError> {
        receipt.validate_session_project(session_project_id)?;
        let canonical_path = storage.normalize_path(path)?;
        receipt.validate_canonical_path(&canonical_path)?;
        storage.reconcile_publication(&canonical_path)?;
        let Some(project) = storage.read_project(&canonical_path)?.decode_if_digest(
            receipt.accepted_digest,
            Some(&canonical_path),
            source_files,
        )?
        else {
            return Err(PersistenceError::ExternalChange);
        };
        receipt.validate_loaded_project(&project.file)?;
        Ok((
            project,
            Self {
                canonical_path,
                accepted_digest: receipt.accepted_digest,
            },
        ))
    }
}

#[derive(Debug, Clone)]
pub struct UnreadableNativeBinding {
    pub canonical_path: PathBuf,
    pub reason: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DestinationAuthority {
    /// Ordinary Save to an already accepted canonical binding.
    Canonical,
    /// A native picker selected this path and supplied the overwrite decision.
    UserSelected,
}

/// Filesystem operations remain in the host. Expected content is the host
/// writer's exact missing-or-digest condition, never unconditional overwrite.
pub trait NativeProjectStorage {
    type ExpectedContent;

    fn normalize_path(&self, path: &Path) -> Result<PathBuf, PersistenceError>;
    fn reconcile_publication(&self, path: &Path) -> Result<(), PersistenceError>;
    fn read_project(&self, path: &Path) -> Result<ProjectBytes, ProjectIoError>;
    fn same_file(&self, left: &Path, right: &Path) -> Result<bool, PersistenceError>;
    fn observe_destination(&self, path: &Path) -> Result<Self::ExpectedContent, PersistenceError>;
    fn accepted_content(&self, digest: ContentDigest) -> Self::ExpectedContent;

    /// Compare and publish under the writer's lease/recovery protocol. Errors
    /// must retain conflicts and publication uncertainty; success identifies
    /// the exact bytes durably published at this path.
    fn publish(
        &self,
        path: &Path,
        expected: Self::ExpectedContent,
        bytes: &[u8],
    ) -> Result<ContentDigest, PersistenceError>;
}

/// Destination authority captured before transaction entry or content capture.
pub struct NativeSaveDestination<E> {
    path: PathBuf,
    expected: E,
}

impl<E> NativeSaveDestination<E> {
    pub fn new(
        storage: &impl NativeProjectStorage<ExpectedContent = E>,
        path: &Path,
        authority: DestinationAuthority,
        binding: Option<&NativeBinding>,
        unreadable: Option<&UnreadableNativeBinding>,
    ) -> Result<Self, ProjectLifecycleError> {
        let path = storage.normalize_path(path)?;
        if let Some(unreadable) = unreadable
            && unreadable.canonical_path == path
        {
            return Err(ProjectLifecycleError::UnreadableCanonical(
                unreadable.reason.clone(),
            ));
        }
        let expected = match authority {
            DestinationAuthority::Canonical => binding
                .filter(|binding| binding.canonical_path == path)
                .map(|binding| storage.accepted_content(binding.accepted_digest))
                .ok_or_else(|| {
                    ProjectLifecycleError::UnreadableCanonical(
                        "no exact accepted byte baseline exists for this pathname".to_owned(),
                    )
                })?,
            DestinationAuthority::UserSelected => storage.observe_destination(&path)?,
        };
        Ok(Self { path, expected })
    }

    /// Prepare every fallible adoption comparison before serialization and
    /// publication. The returned candidate and registry can then be adopted
    /// together without a new fallible comparison of the current draft.
    pub fn publish(
        self,
        storage: &impl NativeProjectStorage<ExpectedContent = E>,
        mut candidate: ProjectFile,
        prepare_registry: impl FnOnce(&ProjectFile) -> Result<DocumentRegistry, ProjectLifecycleError>,
    ) -> Result<NativeSavePublication, ProjectLifecycleError> {
        candidate.workspace.project.set_path(self.path.clone());
        let registry = prepare_registry(&candidate)?;
        let (bytes, _) = serialized_project(&candidate)?;
        let accepted_digest = storage.publish(&self.path, self.expected, &bytes)?;
        Ok(NativeSavePublication {
            candidate,
            binding: NativeBinding {
                canonical_path: self.path,
                accepted_digest,
            },
            registry,
        })
    }
}

pub struct NativeSavePublication {
    pub candidate: ProjectFile,
    pub binding: NativeBinding,
    pub registry: DocumentRegistry,
}

/// A selected independent destination cannot become canonical save authority.
pub struct NativeCopyDestination<E> {
    path: PathBuf,
    expected: E,
}

impl<E> NativeCopyDestination<E> {
    pub fn new(
        storage: &impl NativeProjectStorage<ExpectedContent = E>,
        path: &Path,
        canonical_source: Option<&Path>,
        unreadable: Option<&UnreadableNativeBinding>,
    ) -> Result<Self, ProjectLifecycleError> {
        let path = storage.normalize_path(path)?;
        let unreadable_source = unreadable.map(|binding| binding.canonical_path.as_path());
        for source in canonical_source.into_iter().chain(unreadable_source) {
            if source == path || storage.same_file(source, &path)? {
                return Err(ProjectLifecycleError::CopyDestinationIsCanonical);
            }
        }
        let expected = storage.observe_destination(&path)?;
        Ok(Self { path, expected })
    }

    pub fn publish(
        self,
        storage: &impl NativeProjectStorage<ExpectedContent = E>,
        mut candidate: ProjectFile,
    ) -> Result<(), ProjectLifecycleError> {
        candidate.workspace.project = candidate.workspace.project.fork_copy_at(self.path.clone());
        let (bytes, _) = serialized_project(&candidate)?;
        // Retain picker-time expectations even for an independent copy.
        // No canonical binding or accepted baseline is returned to the caller.
        storage.publish(&self.path, self.expected, &bytes)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests;
