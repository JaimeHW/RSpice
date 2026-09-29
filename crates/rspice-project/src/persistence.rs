//! Canonical project byte identities and persisted binding receipts.

mod bytes;
pub use bytes::ProjectBytes;

pub mod browser;
#[cfg(not(target_arch = "wasm32"))]
pub mod native;

use crate::{ProjectFile, ProjectIoError};
use rspice_app_types::product::ContentDigest;
use sha2::{Digest as _, Sha256};
use std::path::PathBuf;

/// Storage surface that owns the canonical browser bytes.  The opaque
/// binding UUID is deliberately independent of the logical project UUID so
/// forks, duplicate projects, and multiple browser tabs cannot alias one
/// another's persistence authority.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum BrowserBindingBackend {
    ExternalFile,
    Opfs,
}

/// Durable session receipt for a browser canonical binding.  A restored
/// IndexedDB/OPFS record must match every field before it may become an
/// accepted baseline; a different generation or digest is a conflict, not a
/// silently upgraded baseline.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct BrowserBindingReceipt {
    pub binding_id: uuid::Uuid,
    pub project_id: String,
    pub accepted_generation: u64,
    pub accepted_digest: ContentDigest,
    pub backend: BrowserBindingBackend,
}

/// Exact native canonical-file authority persisted with the working session.
/// A remembered pathname is only a convenience; restart restoration requires
/// this path, logical project identity, and content digest to match together.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct NativeBindingReceipt {
    pub canonical_path: PathBuf,
    pub project_id: String,
    pub accepted_digest: ContentDigest,
}

#[cfg(not(target_arch = "wasm32"))]
impl NativeBindingReceipt {
    pub fn validate_session_project(
        &self,
        session_project_id: &str,
    ) -> Result<(), PersistenceError> {
        if self.project_id != session_project_id {
            return Err(PersistenceError::NativeReceiptMismatch(
                "logical project identity differs from the restored session".to_owned(),
            ));
        }
        Ok(())
    }

    pub fn validate_canonical_path(
        &self,
        canonical_path: &std::path::Path,
    ) -> Result<(), PersistenceError> {
        if canonical_path != self.canonical_path {
            return Err(PersistenceError::NativeReceiptMismatch(
                "canonical pathname differs from the accepted pathname".to_owned(),
            ));
        }
        Ok(())
    }

    pub fn validate_loaded_project(&self, project: &ProjectFile) -> Result<(), PersistenceError> {
        if project.workspace.project.id().to_string() != self.project_id {
            return Err(PersistenceError::NativeReceiptMismatch(
                "project file identity differs from the accepted logical project".to_owned(),
            ));
        }
        Ok(())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum PersistenceError {
    #[cfg(not(target_arch = "wasm32"))]
    #[error(
        "the project changed outside RSpice; reload it or save a project copy before overwriting external changes"
    )]
    ExternalChange,
    #[cfg(not(target_arch = "wasm32"))]
    #[error("native canonical binding receipt mismatch: {0}")]
    NativeReceiptMismatch(String),
    #[cfg(not(target_arch = "wasm32"))]
    #[error(
        "project publication outcome is uncertain: {message}; preserved recovery files: {recovery_paths:?}"
    )]
    PublicationUncertain {
        message: String,
        recovery_paths: Vec<PathBuf>,
    },
    #[cfg(not(target_arch = "wasm32"))]
    #[error("another RSpice writer owns the destination lease: {0}")]
    LeaseBusy(PathBuf),
    #[cfg(not(target_arch = "wasm32"))]
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Project(#[from] ProjectIoError),
    #[cfg(not(target_arch = "wasm32"))]
    #[error("{0}")]
    Platform(String),
}

pub fn serialized_project(
    project: &ProjectFile,
) -> Result<(Vec<u8>, ContentDigest), PersistenceError> {
    let contents = crate::serialize_project_file(project)?;
    let bytes = contents.into_bytes();
    let digest = digest_bytes(&bytes);
    Ok((bytes, digest))
}

pub fn digest_bytes(bytes: &[u8]) -> ContentDigest {
    ContentDigest::from_bytes(Sha256::digest(bytes).into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn project_digest_is_exact_over_published_bytes() {
        let bytes = b"project bytes\n";
        assert_ne!(digest_bytes(bytes), digest_bytes(b"project bytes"));
    }
}
