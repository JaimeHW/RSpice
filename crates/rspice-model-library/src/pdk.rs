//! Portable signed-PDK validation vocabulary and publisher trust authority.
//!
//! Installation and source compilation remain with their consuming services.

use rspice_app_types::product::ContentDigest;
use sha2::{Digest as _, Sha256};

pub mod callback;
pub mod contracts;
pub mod diff;
pub mod manifest;
pub mod package;
mod trust;
pub use trust::{
    PdkAdministrativeAuthority, PdkPublisherTrustStore, PdkTrustAuditAction, PdkTrustAuditReceipt,
    TrustedPdkPublisherKey,
};

pub const MAX_PDK_TRUST_AUDIT_RECEIPTS: usize = 16_384;
pub const MAX_PDK_PUBLISHER_KEYS: usize = 4_096;

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
    #[error("signature contains {actual} bytes; exactly 64 are required")]
    InvalidSignatureLength { actual: usize },
    #[error("publisher '{publisher_id}' key '{key_id}' is not trusted")]
    UntrustedPublisher {
        publisher_id: String,
        key_id: String,
    },
    #[error("publisher '{publisher_id}' key '{key_id}' is revoked")]
    RevokedPublisherKey {
        publisher_id: String,
        key_id: String,
    },
    #[error("publisher signature verification failed for '{publisher_id}' key '{key_id}'")]
    InvalidSignature {
        publisher_id: String,
        key_id: String,
    },
    #[error("invalid publisher trust store: {0}")]
    InvalidTrustStore(String),
    #[error("immutable publisher trust-key conflict: {0}")]
    ImmutableTrustKey(String),
    #[error("publisher trust audit chain is corrupted: {0}")]
    TrustAuditCorrupted(String),
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
    #[error("signed PDK model-source materialization failed: {0}")]
    ModelMaterialization(String),
    #[error("forbidden callback capability: {0}")]
    ForbiddenCapability(String),
    #[error("signed PDK callback validation failed: {0}")]
    CallbackValidation(String),
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
