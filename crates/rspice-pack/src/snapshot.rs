use std::collections::BTreeSet;

use base64::{Engine as _, engine::general_purpose::STANDARD};
use miniz_oxide::{
    deflate::compress_to_vec_zlib,
    inflate::{TINFLStatus, decompress_to_vec_zlib_with_limit},
};
use serde::{Deserialize, Serialize};

use crate::{
    canonical::canonical_bytes,
    error::{PackError, invalid},
    limits::Limits,
    manifest::{
        PartKind, Symbol, is_sha256_hex, validate_capabilities, validate_display_name,
        validate_identifier, validate_kebab, validate_pack_id, validate_semver, validate_spdx,
        validate_symbol, validate_terminals,
    },
    signing::{
        SIGNATURE_BYTES, SigningKey, VerifyingKey, sign_manifest, verify_manifest_signature,
    },
};

/// The only catalog snapshot schema this crate reads or writes. The envelope
/// and the snapshot carry the same number so a consumer can reject a document
/// before decompressing what it cannot model.
pub const SNAPSHOT_SCHEMA: u32 = 1;

const DEFLATE_LEVEL: u8 = 6;
const MAX_PACKS: usize = 10_000;
const MAX_RELEASES: usize = 1_000;
const MAX_PARTS: usize = 100_000;
const MAX_GENERATED_AT_CHARS: usize = 40;

/// The signed catalog the service publishes and every client resolves against.
///
/// Parts ride on the release rather than in the pack archive so a client shelf
/// can search the whole catalog without downloading a single pack.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Snapshot {
    pub schema: u32,
    /// RFC 3339 instant supplied by the caller. This crate never reads a
    /// clock, so a snapshot is reproducible from its inputs alone.
    pub generated_at: String,
    pub packs: Vec<SnapshotPack>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SnapshotPack {
    pub id: String,
    pub name: String,
    pub category: String,
    pub releases: Vec<SnapshotRelease>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SnapshotRelease {
    pub version: String,
    pub archive_sha256: String,
    pub archive_length: u64,
    pub capabilities: Vec<String>,
    pub spdx: String,
    pub parts: Vec<SnapshotPart>,
}

/// A catalog projection of a manifest part: everything a shelf needs to search
/// and preview, without the in-archive source location.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SnapshotPart {
    pub id: String,
    pub kind: PartKind,
    pub device: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub aliases: Vec<String>,
    pub terminals: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<Symbol>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct SnapshotEnvelope {
    schema: u32,
    signature: String,
    snapshot: Snapshot,
}

/// Encodes the canonical bytes a snapshot signature covers.
pub fn canonical_snapshot_bytes(snapshot: &Snapshot) -> Result<Vec<u8>, PackError> {
    validate_snapshot(snapshot)?;
    let value =
        serde_json::to_value(snapshot).map_err(|error| PackError::Encoding(error.to_string()))?;
    canonical_bytes(&value)
}

/// Signs a snapshot and compresses its envelope for transport.
pub fn encode_snapshot(snapshot: &Snapshot, key: &SigningKey) -> Result<Vec<u8>, PackError> {
    let signed = canonical_snapshot_bytes(snapshot)?;
    let envelope = SnapshotEnvelope {
        schema: SNAPSHOT_SCHEMA,
        signature: STANDARD.encode(sign_manifest(&signed, key)),
        snapshot: snapshot.clone(),
    };
    let value =
        serde_json::to_value(&envelope).map_err(|error| PackError::Encoding(error.to_string()))?;
    Ok(compress_to_vec_zlib(
        &canonical_bytes(&value)?,
        DEFLATE_LEVEL,
    ))
}

/// Decompresses, verifies, and validates a published catalog snapshot.
///
/// The signature is checked against the snapshot re-canonicalized from its
/// typed projection, so transport-level re-encoding is tolerated while any
/// change to the content the client will act on is not.
pub fn decode_snapshot(
    bytes: &[u8],
    key: &VerifyingKey,
    limits: &Limits,
) -> Result<Snapshot, PackError> {
    if bytes.len() > limits.max_snapshot_bytes {
        return Err(PackError::SnapshotTooLarge {
            limit: limits.max_snapshot_bytes,
        });
    }
    let expanded =
        decompress_to_vec_zlib_with_limit(bytes, limits.max_snapshot_bytes).map_err(|error| {
            if error.status == TINFLStatus::HasMoreOutput {
                PackError::SnapshotTooLarge {
                    limit: limits.max_snapshot_bytes,
                }
            } else {
                PackError::MalformedSnapshot(error.to_string())
            }
        })?;
    let envelope: SnapshotEnvelope = serde_json::from_slice(&expanded)
        .map_err(|error| PackError::MalformedSnapshot(error.to_string()))?;
    if envelope.schema != SNAPSHOT_SCHEMA {
        return Err(PackError::MalformedSnapshot(format!(
            "only envelope schema {SNAPSHOT_SCHEMA} is defined"
        )));
    }
    let raw = STANDARD
        .decode(&envelope.signature)
        .map_err(|error| PackError::MalformedSnapshot(error.to_string()))?;
    let signature: [u8; SIGNATURE_BYTES] =
        raw.as_slice()
            .try_into()
            .map_err(|_| PackError::MalformedSignature {
                expected: SIGNATURE_BYTES,
                actual: raw.len(),
            })?;
    let signed = canonical_snapshot_bytes(&envelope.snapshot)?;
    verify_manifest_signature(&signed, &signature, key)?;
    Ok(envelope.snapshot)
}

/// Proves a snapshot is internally consistent before it is signed or trusted.
pub fn validate_snapshot(snapshot: &Snapshot) -> Result<(), PackError> {
    if snapshot.schema != SNAPSHOT_SCHEMA {
        return Err(invalid(
            "schema",
            format!("only snapshot schema {SNAPSHOT_SCHEMA} is defined"),
        ));
    }
    validate_generated_at(&snapshot.generated_at)?;
    if snapshot.packs.len() > MAX_PACKS {
        return Err(invalid(
            "packs",
            format!("a snapshot lists at most {MAX_PACKS} packs"),
        ));
    }
    let mut identifiers = BTreeSet::new();
    let mut parts = 0_usize;
    for pack in &snapshot.packs {
        validate_pack_id(&pack.id)?;
        validate_display_name(&pack.name, "packs[].name")?;
        validate_kebab(&pack.category, "packs[].category")?;
        if !identifiers.insert(pack.id.as_str()) {
            return Err(invalid(
                "packs[].id",
                format!("{:?} appears more than once", pack.id),
            ));
        }
        if pack.releases.is_empty() || pack.releases.len() > MAX_RELEASES {
            return Err(invalid(
                "packs[].releases",
                format!("a pack publishes between 1 and {MAX_RELEASES} releases"),
            ));
        }
        let mut versions = BTreeSet::new();
        for release in &pack.releases {
            validate_semver(&release.version, "packs[].releases[].version")?;
            if !versions.insert(release.version.as_str()) {
                return Err(invalid(
                    "packs[].releases[].version",
                    format!("{:?} is published more than once", release.version),
                ));
            }
            if !is_sha256_hex(&release.archive_sha256) {
                return Err(invalid(
                    "packs[].releases[].archive_sha256",
                    "a digest is 64 lowercase hexadecimal characters",
                ));
            }
            if release.archive_length == 0 {
                return Err(invalid(
                    "packs[].releases[].archive_length",
                    "a published archive is never empty",
                ));
            }
            validate_capabilities(&release.capabilities)?;
            validate_spdx(&release.spdx)?;
            parts += release.parts.len();
            if parts > MAX_PARTS {
                return Err(invalid(
                    "packs[].releases[].parts",
                    format!("a snapshot carries at most {MAX_PARTS} parts"),
                ));
            }
            for part in &release.parts {
                validate_part(part)?;
            }
        }
    }
    Ok(())
}

fn validate_part(part: &SnapshotPart) -> Result<(), PackError> {
    validate_identifier(&part.id, "packs[].releases[].parts[].id")?;
    validate_kebab(&part.device, "packs[].releases[].parts[].device")?;
    for alias in &part.aliases {
        validate_identifier(alias, "packs[].releases[].parts[].aliases")?;
    }
    validate_terminals(&part.terminals)?;
    if let Some(symbol) = &part.symbol {
        validate_symbol(symbol, &part.terminals)?;
    }
    Ok(())
}

/// Accepts `YYYY-MM-DDTHH:MM:SS[.fraction]Z`. The value is opaque to this
/// crate beyond its shape: it is signed evidence of when a publisher generated
/// the catalog, never a clock this code reads or compares.
fn validate_generated_at(value: &str) -> Result<(), PackError> {
    let refuse = || {
        invalid(
            "generated_at",
            "expected an RFC 3339 UTC instant such as 2026-08-15T09:30:00Z",
        )
    };
    if value.len() > MAX_GENERATED_AT_CHARS {
        return Err(refuse());
    }
    let Some(body) = value.strip_suffix('Z') else {
        return Err(refuse());
    };
    let (instant, fraction) = match body.split_once('.') {
        Some((instant, fraction)) => (instant, Some(fraction)),
        None => (body, None),
    };
    let shaped = instant.len() == 19
        && instant
            .bytes()
            .enumerate()
            .all(|(index, byte)| match index {
                4 | 7 => byte == b'-',
                10 => byte == b'T',
                13 | 16 => byte == b':',
                _ => byte.is_ascii_digit(),
            });
    let fraction_shaped = fraction.is_none_or(|fraction| {
        !fraction.is_empty() && fraction.bytes().all(|byte| byte.is_ascii_digit())
    });
    if shaped && fraction_shaped {
        Ok(())
    } else {
        Err(refuse())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::signing::signing_key;

    fn snapshot() -> Snapshot {
        Snapshot {
            schema: SNAPSHOT_SCHEMA,
            generated_at: "2026-08-15T09:30:00Z".to_owned(),
            packs: vec![SnapshotPack {
                id: "rspice-opamps".to_owned(),
                name: "RSpice Op-Amps".to_owned(),
                category: "ic-analog".to_owned(),
                releases: vec![SnapshotRelease {
                    version: "1.0.0".to_owned(),
                    archive_sha256: "ab".repeat(32),
                    archive_length: 48_122,
                    capabilities: vec!["subckt".to_owned()],
                    spdx: "LicenseRef-RSpice-Models".to_owned(),
                    parts: vec![SnapshotPart {
                        id: "TL072".to_owned(),
                        kind: PartKind::Subckt,
                        device: "opamp".to_owned(),
                        aliases: vec!["TL072CP".to_owned()],
                        terminals: vec!["INP".to_owned(), "OUT".to_owned()],
                        symbol: None,
                    }],
                }],
            }],
        }
    }

    #[test]
    fn a_snapshot_round_trips_through_its_signed_envelope() {
        let key = signing_key(&[5_u8; 32]);
        let encoded = encode_snapshot(&snapshot(), &key).expect("encodes");
        let decoded =
            decode_snapshot(&encoded, &key.verifying_key(), &Limits::default()).expect("decodes");
        assert_eq!(decoded, snapshot());
        assert_eq!(
            encode_snapshot(&snapshot(), &key).expect("encodes"),
            encoded
        );
    }

    #[test]
    fn a_foreign_key_and_a_tampered_snapshot_are_both_refused() {
        let key = signing_key(&[5_u8; 32]);
        let foreign = signing_key(&[6_u8; 32]);
        let encoded = encode_snapshot(&snapshot(), &key).expect("encodes");
        assert!(matches!(
            decode_snapshot(&encoded, &foreign.verifying_key(), &Limits::default()),
            Err(PackError::BadSignature)
        ));

        let expanded =
            decompress_to_vec_zlib_with_limit(&encoded, 1 << 20).expect("test fixture expands");
        let text = String::from_utf8(expanded).expect("UTF-8");
        let tampered = text.replace("48122", "48123");
        assert_ne!(tampered, text);
        let recompressed = compress_to_vec_zlib(tampered.as_bytes(), DEFLATE_LEVEL);
        assert!(matches!(
            decode_snapshot(&recompressed, &key.verifying_key(), &Limits::default()),
            Err(PackError::BadSignature)
        ));
    }

    #[test]
    fn the_decompression_cap_is_enforced() {
        let key = signing_key(&[5_u8; 32]);
        let encoded = encode_snapshot(&snapshot(), &key).expect("encodes");
        let limits = Limits {
            max_snapshot_bytes: 32,
            ..Limits::default()
        };
        assert!(matches!(
            decode_snapshot(&encoded, &key.verifying_key(), &limits),
            Err(PackError::SnapshotTooLarge { limit: 32 })
        ));
    }

    #[test]
    fn generated_at_must_be_a_bounded_rfc3339_instant() {
        for accepted in ["2026-08-15T09:30:00Z", "2026-08-15T09:30:00.123456Z"] {
            let mut value = snapshot();
            value.generated_at = accepted.to_owned();
            validate_snapshot(&value).expect("accepted instant");
        }
        for refused in [
            "",
            "2026-08-15",
            "2026-08-15T09:30:00",
            "2026-08-15T09:30:00+01:00",
            "2026-08-15t09:30:00Z",
            "2026-08-15T09:30:00.Z",
        ] {
            let mut value = snapshot();
            value.generated_at = refused.to_owned();
            assert!(
                validate_snapshot(&value).is_err(),
                "{refused:?} must be refused"
            );
        }
    }

    #[test]
    fn a_snapshot_without_a_release_is_refused() {
        let mut value = snapshot();
        value.packs[0].releases.clear();
        assert!(matches!(
            validate_snapshot(&value),
            Err(PackError::InvalidField { .. })
        ));
    }
}
