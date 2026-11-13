//! Portable PDK technology validation vocabulary.
//!
//! Installation and source compilation remain with their consuming services.

use rspice_vocabulary::product::ContentDigest;
use sha2::{Digest as _, Sha256};

mod authority;
pub use authority::PdkAdministrativeAuthority;
pub mod contracts;
pub mod diff;
mod library_config;
pub use library_config::{DiscoveredFile, LibraryPathEntry};
pub mod manifest;
pub mod package;

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum PdkTechnologyError {
    #[error("technology archive is {actual} bytes; maximum is {maximum}")]
    ArchiveTooLarge { actual: usize, maximum: usize },
    #[error("technology archive JSON is invalid: {0}")]
    ArchiveParse(String),
    #[error("technology manifest JSON is invalid: {0}")]
    ManifestParse(String),
    #[error("unsupported {object} schema {actual}; newest supported schema is {supported}")]
    UnsupportedSchema {
        object: &'static str,
        actual: u32,
        supported: u32,
    },
    #[error("invalid field: {0}")]
    InvalidField(String),
    #[error("invalid reference: {0}")]
    InvalidReference(String),
    #[error("duplicate identity: {0}")]
    Duplicate(String),
    #[error("missing stream mapping for {0}")]
    MissingMapping(String),
    #[error("limit exceeded: {0}")]
    LimitExceeded(String),
    #[error("{field} is not valid base64: {detail}")]
    InvalidBase64 { field: String, detail: String },
    #[error("manifest declares missing artifact '{0}'")]
    MissingArtifact(String),
    #[error("archive contains undeclared artifact '{0}'")]
    UndeclaredArtifact(String),
    #[error("artifact '{path}' declares {declared} bytes but contains {actual}")]
    ArtifactSizeMismatch {
        path: String,
        declared: u64,
        actual: usize,
    },
    #[error("artifact '{path}' digest mismatch: declared {declared}, actual {actual}")]
    ArtifactDigestMismatch {
        path: String,
        declared: ContentDigest,
        actual: ContentDigest,
    },
    #[error("PDK model-source materialization failed: {0}")]
    ModelMaterialization(String),
    #[error("immutable technology revision conflict: {0}")]
    ImmutableRevision(String),
    #[error("technology package is not runtime-validated: {0}")]
    NotRuntimeValidated(String),
    #[error("invalid technology transition: {0}")]
    InvalidTransition(String),
    #[error("incompatible technology runtime: {0}")]
    IncompatibleRuntime(String),
    #[error("technology audit chain is corrupted: {0}")]
    AuditCorrupted(String),
    #[error("technology administration serialization failed: {0}")]
    Serialization(String),
}

pub fn content_digest(bytes: &[u8]) -> ContentDigest {
    ContentDigest::from_bytes(Sha256::digest(bytes).into())
}

pub fn validate_identifier(path: &str, value: &str) -> Result<(), PdkTechnologyError> {
    validate_text(path, value, 128)?;
    if !value.bytes().all(|byte| {
        byte.is_ascii_lowercase()
            || byte.is_ascii_digit()
            || matches!(byte, b'-' | b'_' | b'.' | b':' | b'@')
    }) {
        return Err(PdkTechnologyError::InvalidField(format!(
            "{path} must use lowercase ASCII identifier characters"
        )));
    }
    Ok(())
}

pub fn validate_text(path: &str, value: &str, maximum: usize) -> Result<(), PdkTechnologyError> {
    if value.trim() != value || value.is_empty() || value.len() > maximum {
        return Err(PdkTechnologyError::InvalidField(format!(
            "{path} must contain 1..={maximum} bytes without surrounding whitespace"
        )));
    }
    if value.chars().any(char::is_control) {
        return Err(PdkTechnologyError::InvalidField(format!(
            "{path} contains a control character"
        )));
    }
    Ok(())
}
