use std::fmt;

use thiserror::Error;

/// Every reason this crate refuses a pack archive, manifest, or catalog
/// snapshot.
///
/// Rejection is always a typed value: nothing in this crate panics, unwraps,
/// or indexes on attacker-controlled input, so a caller can classify a refusal
/// without parsing message text.
#[derive(Debug, Error)]
pub enum PackError {
    #[error("archive is {actual} bytes, above the {limit}-byte compressed limit")]
    ArchiveTooLarge { limit: usize, actual: usize },
    #[error("expanded archive content exceeds the {limit}-byte limit")]
    ExpandedTooLarge { limit: usize },
    #[error("archive holds {actual} entries, above the {limit}-entry limit")]
    TooManyFiles { limit: usize, actual: usize },
    #[error("malformed archive: {0}")]
    MalformedArchive(&'static str),
    #[error("unsupported archive feature: {0}")]
    UnsupportedArchiveFeature(ArchiveFeature),
    #[error("archive entry {0:?} appears more than once")]
    DuplicateEntry(String),
    #[error("archive path {0:?} violates the pack path rules")]
    IllegalPath(String),
    #[error("archive is missing its required {0} entry")]
    MissingReservedEntry(&'static str),
    #[error("signature.ed25519 must hold exactly {expected} bytes, found {actual}")]
    MalformedSignature { expected: usize, actual: usize },
    #[error("the manifest signature does not verify against the supplied key")]
    BadSignature,
    #[error("the supplied ed25519 verifying key is not a canonical curve point")]
    InvalidVerifyingKey,
    #[error("manifest.json is not the canonical encoding of its own content")]
    NonCanonicalManifest,
    #[error("malformed manifest: {0}")]
    MalformedManifest(String),
    #[error("manifest holds a non-integer number at {0}")]
    FloatInManifest(String),
    #[error("manifest object key {0:?} is outside the canonical ASCII key domain")]
    NonAsciiKey(String),
    #[error("{field} is invalid: {detail}")]
    InvalidField { field: String, detail: String },
    #[error("manifest lists the reserved archive name {0:?}")]
    ReservedFileName(String),
    #[error("archive entry {0:?} is not listed by the manifest")]
    UnlistedEntry(String),
    #[error("manifest lists {0:?} but the archive does not carry it")]
    MissingListedFile(String),
    #[error("{path:?} holds {actual} bytes where the manifest declares {expected}")]
    LengthMismatch {
        path: String,
        expected: u64,
        actual: u64,
    },
    #[error("{path:?} content does not match its manifest digest")]
    DigestMismatch { path: String },
    #[error("catalog snapshot exceeds the {limit}-byte limit")]
    SnapshotTooLarge { limit: usize },
    #[error("malformed catalog snapshot: {0}")]
    MalformedSnapshot(String),
    #[error("canonical encoding failed: {0}")]
    Encoding(String),
    /// Filesystem refusal raised only by the `packer` feature; the portable
    /// verification path never touches a filesystem.
    #[error("{path}: {detail}")]
    Io { path: String, detail: String },
}

/// A ZIP capability this crate deliberately does not implement.
///
/// The pack container is a fixed subset of ZIP, so every one of these is a
/// refusal rather than a fallback: an archive that needs any of them was not
/// produced by the reviewed writer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArchiveFeature {
    Zip64,
    Encryption,
    StrongEncryption,
    MaskedLocalHeaders,
    DataDescriptor,
    GeneralPurposeFlags,
    CompressionMethod(u16),
    ExtraField,
    EntryComment,
    ArchiveComment,
    MultipleDisks,
    ExternalAttributes,
}

impl fmt::Display for ArchiveFeature {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Zip64 => formatter.write_str("zip64 extensions"),
            Self::Encryption => formatter.write_str("entry encryption"),
            Self::StrongEncryption => formatter.write_str("strong entry encryption"),
            Self::MaskedLocalHeaders => formatter.write_str("masked local headers"),
            Self::DataDescriptor => formatter.write_str("trailing data descriptors"),
            Self::GeneralPurposeFlags => formatter.write_str("general purpose bit flags"),
            Self::CompressionMethod(method) => write!(formatter, "compression method {method}"),
            Self::ExtraField => formatter.write_str("extra fields"),
            Self::EntryComment => formatter.write_str("entry comments"),
            Self::ArchiveComment => formatter.write_str("archive comments"),
            Self::MultipleDisks => formatter.write_str("multi-disk archives"),
            Self::ExternalAttributes => formatter.write_str("external file attributes"),
        }
    }
}

pub(crate) fn invalid(field: &str, detail: impl Into<String>) -> PackError {
    PackError::InvalidField {
        field: field.to_owned(),
        detail: detail.into(),
    }
}
