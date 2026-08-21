#![forbid(unsafe_code)]

//! The RSpice model pack format and its trust boundary.
//!
//! A `.rspicepack` is a signed, immutable archive of authored SPICE model
//! sources. This crate defines that container and is the only code allowed to
//! canonicalize, sign, or verify one: the desktop client, the browser client,
//! the cloud service, and the packer tool all reach the same verdict because
//! they run these exact functions.
//!
//! What it does not have is as deliberate as what it does. There is no HTTP
//! client, filesystem access outside the optional `packer` feature, clock,
//! async runtime, platform binding, general-purpose ZIP implementation, or
//! random-number source in the portable path — so the crate compiles for
//! `wasm32-unknown-unknown` and behaves identically there.
//!
//! # Container
//!
//! ```text
//! manifest.json          canonical JSON, the signed document
//! signature.ed25519      64 raw bytes over the exact manifest.json bytes
//! models/...             authored model sources
//! LICENSE, NOTICE        optional
//! ```
//!
//! # Verifying
//!
//! ```no_run
//! use rspice_pack::{Limits, Pack, verifying_key_from_hex};
//!
//! # fn main() -> Result<(), rspice_pack::PackError> {
//! # let archive: Vec<u8> = Vec::new();
//! let key = verifying_key_from_hex("00".repeat(32).as_str())?;
//! let pack = Pack::verify(&archive, &key, &Limits::default())?;
//! for part in &pack.manifest.parts {
//!     println!("{} from {}", part.id, part.source.path);
//! }
//! # Ok(())
//! # }
//! ```

mod canonical;
mod error;
mod limits;
mod manifest;
mod pack;
mod path;
mod signing;
mod snapshot;
#[cfg(test)]
mod testing;
mod zip;

#[cfg(feature = "packer")]
mod packer;

pub use crate::{
    error::{ArchiveFeature, PackError},
    limits::Limits,
    manifest::{
        FileEntry, License, MANIFEST_SCHEMA, Manifest, ManifestTemplate, PackIdentity, Part,
        PartKind, RESERVED_PART_IDENTITIES, Requires, SourceRef, Symbol, canonical_manifest_bytes,
        catalog_identity_key, is_reserved_catalog_identity, parse_manifest, validate_manifest,
    },
    pack::{InspectedPack, Pack, VerifiedPack, build_pack},
    path::{MANIFEST_ENTRY, MAX_PATH_BYTES, SIGNATURE_ENTRY, is_reserved_path, validate_path},
    signing::{
        SIGNATURE_BYTES, SIGNING_KEY_BYTES, SigningKey, VERIFYING_KEY_BYTES, VerifyingKey,
        encode_hex, sha256_hex, sign_manifest, signing_key, verify_manifest_signature,
        verifying_key, verifying_key_from_hex,
    },
    snapshot::{
        Revocation, SNAPSHOT_SCHEMA, Snapshot, SnapshotPack, SnapshotPart, SnapshotRelease,
        canonical_snapshot_bytes, decode_snapshot, encode_snapshot, validate_snapshot,
    },
};

#[cfg(feature = "packer")]
pub use crate::packer::{build_pack_from_dir, generate_signing_key};
