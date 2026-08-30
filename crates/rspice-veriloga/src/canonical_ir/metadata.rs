//! Provenance metadata and the reproducible digest used to stamp it.
//!
//! [`CanonicalMetadata`] records which source package a model was compiled
//! from; [`StableDigest`] provides the content hash each IR level is stamped
//! with. Note the digest's deliberately narrow contract: it is a fast FNV-1a
//! hash for change detection and reproducibility, explicitly not a
//! cryptographic one. Identity that has to resist tampering uses the BLAKE3
//! chain in [`crate::virtual_source`] instead.

use serde::{Deserialize, Serialize};
use smol_str::SmolStr;

/// Canonical HIR/MIR wire-format version emitted and accepted by this build.
///
/// This is a hard compatibility boundary: caches and external backends must
/// never deserialize a structurally different artifact merely because its HIR
/// and metadata happen to repeat the same stale version number.
pub const CANONICAL_IR_SCHEMA_VERSION: u32 = 9;

/// Stable deterministic digest for canonical IR metadata.
///
/// This uses a fixed 64-bit FNV-1a-style hash to make metadata reproducible
/// across runs and platforms. It is not a cryptographic digest and must not be
/// used for trust, authentication, or collision-resistant identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct StableDigest(u64);

impl StableDigest {
    pub fn from_text(text: &str) -> Self {
        let mut hash = 0xcbf29ce484222325u64;
        for byte in text.as_bytes() {
            hash ^= u64::from(*byte);
            hash = hash.wrapping_mul(0x100000001b3);
        }
        Self(hash)
    }

    pub fn as_hex(self) -> String {
        format!("{:016x}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CanonicalMetadata {
    pub schema_version: u32,
    pub source_package: SmolStr,
    pub source_digest: SmolStr,
    pub compiler_version: SmolStr,
    pub feature_flags: Vec<SmolStr>,
}

impl CanonicalMetadata {
    pub fn for_source(source_package: impl Into<SmolStr>, source_text: &str) -> Self {
        Self {
            schema_version: CANONICAL_IR_SCHEMA_VERSION,
            source_package: source_package.into(),
            source_digest: StableDigest::from_text(source_text).as_hex().into(),
            compiler_version: env!("CARGO_PKG_VERSION").into(),
            feature_flags: Vec::new(),
        }
    }
}
