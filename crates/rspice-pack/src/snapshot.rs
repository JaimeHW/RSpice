use std::collections::{BTreeMap, BTreeSet};

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
        PartKind, Symbol, catalog_identity_key, is_reserved_catalog_identity, is_sha256_hex,
        validate_capabilities, validate_display_name, validate_identifier, validate_kebab,
        validate_pack_id, validate_presentation, validate_semver, validate_spdx, validate_symbol,
        validate_terminals,
    },
    signing::{
        SIGNATURE_BYTES, SigningKey, VerifyingKey, sign_manifest, verify_manifest_signature,
    },
};

/// The only catalog snapshot schema this crate reads or writes. The envelope
/// and the snapshot carry the same number so a consumer can reject a document
/// before decompressing what it cannot model.
pub const SNAPSHOT_SCHEMA: u32 = 2;

const DEFLATE_LEVEL: u8 = 6;
const MAX_PACKS: usize = 10_000;
const MAX_RELEASES: usize = 1_000;
const MAX_PARTS: usize = 100_000;
const MAX_REVOCATIONS: usize = 10_000;
const MAX_INSTANT_CHARS: usize = 40;

/// The signed catalog the service publishes and every client resolves against.
///
/// Parts ride on the release rather than in the pack archive so a client shelf
/// can search the whole catalog without downloading a single pack.
///
/// Three fields exist for the client's benefit rather than the shelf's, and
/// they are what make a stale or malicious catalog detectable: `serial` orders
/// two authentic catalogs so an older one cannot be replayed over a newer one,
/// `expires_at` bounds how long any one of them may be believed, and
/// `revocations` names releases that must be recalled even though their bytes
/// remain downloadable for a project that pinned them.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Snapshot {
    pub schema: u32,
    /// Strictly increasing catalog ordinal. A client keeps the highest serial
    /// it has ever accepted and refuses anything below it, so the ordinal —
    /// not a clock — is what defeats a rollback. Zero is never published, which
    /// leaves it free as a client's "no catalog yet" sentinel.
    pub serial: u64,
    /// RFC 3339 instant supplied by the caller. This crate never reads a
    /// clock, so a snapshot is reproducible from its inputs alone.
    pub generated_at: String,
    /// RFC 3339 instant after which this catalog must not be believed, from
    /// the same caller-supplied clock as `generated_at`. This crate validates
    /// its shape and signs it; deciding that the instant has passed needs a
    /// clock, so it belongs to the consumer, not here.
    pub expires_at: String,
    pub packs: Vec<SnapshotPack>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub revocations: Vec<Revocation>,
}

/// A recalled pack release.
///
/// Revocation is strictly stronger than delisting: a withdrawn release simply
/// stops appearing in `packs`, while a revoked one is named here so a client
/// that already resolved and cached it learns it must stop using it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Revocation {
    pub pack_id: String,
    pub version: String,
    /// Operator prose shown to whoever has to act on the recall.
    pub reason: String,
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
    /// One trimmed line of publisher prose for a catalog shelf. Absent means
    /// the publisher offers no summary — never that the part resists one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Headline specifications, each value a string carrying its own unit
    /// (`"5.1 V"`). Strings are the point: the canonical encoding is defined
    /// only over integers, so this map structurally cannot become a float, and
    /// a bare number is not a specification a reader can act on. Absent means
    /// none are published, never that the part has none.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub specs: BTreeMap<String, String>,
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
    if snapshot.serial == 0 {
        return Err(invalid(
            "serial",
            "a published catalog serial starts at 1, so zero stays a client sentinel",
        ));
    }
    validate_instant(&snapshot.generated_at, "generated_at")?;
    validate_instant(&snapshot.expires_at, "expires_at")?;
    validate_revocations(&snapshot.revocations)?;
    if snapshot.packs.len() > MAX_PACKS {
        return Err(invalid(
            "packs",
            format!("a snapshot lists at most {MAX_PACKS} packs"),
        ));
    }
    let mut identifiers = BTreeSet::new();
    let mut part_owners: BTreeMap<String, String> = BTreeMap::new();
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
            let mut release_names = BTreeSet::new();
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
                for name in std::iter::once(&part.id).chain(&part.aliases) {
                    let key = catalog_identity_key(name);
                    if !release_names.insert(key.clone()) {
                        return Err(invalid(
                            "packs[].releases[].parts",
                            format!(
                                "{name:?} is claimed more than once in pack {:?} release {:?}",
                                pack.id, release.version
                            ),
                        ));
                    }
                    if let Some(owner) = part_owners.get(&key) {
                        if owner != &pack.id {
                            return Err(invalid(
                                "packs[].releases[].parts",
                                format!(
                                    "{name:?} in pack {:?} collides with an id or alias published by pack {owner:?}",
                                    pack.id
                                ),
                            ));
                        }
                    } else {
                        part_owners.insert(key, pack.id.clone());
                    }
                }
            }
        }
    }
    Ok(())
}

fn validate_part(part: &SnapshotPart) -> Result<(), PackError> {
    validate_identifier(&part.id, "packs[].releases[].parts[].id")?;
    if is_reserved_catalog_identity(&part.id) {
        return Err(invalid(
            "packs[].releases[].parts[].id",
            format!("{:?} is reserved by the RSpice foundation library", part.id),
        ));
    }
    validate_kebab(&part.device, "packs[].releases[].parts[].device")?;
    for alias in &part.aliases {
        validate_identifier(alias, "packs[].releases[].parts[].aliases")?;
        if is_reserved_catalog_identity(alias) {
            return Err(invalid(
                "packs[].releases[].parts[].aliases",
                format!("{alias:?} is reserved by the RSpice foundation library"),
            ));
        }
    }
    validate_terminals(&part.terminals)?;
    if let Some(symbol) = &part.symbol {
        validate_symbol(symbol, &part.terminals)?;
    }
    validate_presentation(
        part.description.as_deref(),
        &part.specs,
        "packs[].releases[].parts[].description",
        "packs[].releases[].parts[].specs",
    )?;
    Ok(())
}

/// Proves each recall names one pack version once, in the same spellings the
/// catalog itself uses.
///
/// A revocation is deliberately not cross-checked against `packs`: the release
/// it recalls has usually been delisted already, and a recall that vanished
/// because its release did would be the exact failure this field exists to
/// prevent.
fn validate_revocations(revocations: &[Revocation]) -> Result<(), PackError> {
    if revocations.len() > MAX_REVOCATIONS {
        return Err(invalid(
            "revocations",
            format!("a snapshot carries at most {MAX_REVOCATIONS} revocations"),
        ));
    }
    let mut seen = BTreeSet::new();
    for revocation in revocations {
        validate_pack_id(&revocation.pack_id)?;
        validate_semver(&revocation.version, "revocations[].version")?;
        validate_display_name(&revocation.reason, "revocations[].reason")?;
        if !seen.insert((revocation.pack_id.as_str(), revocation.version.as_str())) {
            return Err(invalid(
                "revocations",
                format!(
                    "{:?} {:?} is revoked more than once",
                    revocation.pack_id, revocation.version
                ),
            ));
        }
    }
    Ok(())
}

/// Accepts `YYYY-MM-DDTHH:MM:SS[.fraction]Z`. The value is opaque to this
/// crate beyond its shape: it is signed evidence of when a publisher generated
/// the catalog and how long it meant it to stand, never a clock this code
/// reads or compares.
fn validate_instant(value: &str, field: &str) -> Result<(), PackError> {
    let refuse = || {
        invalid(
            field,
            "expected an RFC 3339 UTC instant such as 2026-08-15T09:30:00Z",
        )
    };
    if value.len() > MAX_INSTANT_CHARS {
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
            serial: 7,
            generated_at: "2026-08-15T09:30:00Z".to_owned(),
            expires_at: "2026-09-14T09:30:00Z".to_owned(),
            revocations: Vec::new(),
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
                        description: Some("Low-noise JFET-input dual op-amp".to_owned()),
                        specs: BTreeMap::from([
                            ("supply".to_owned(), "+/-15 V".to_owned()),
                            ("gbw".to_owned(), "3 MHz".to_owned()),
                        ]),
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
    fn both_instants_must_be_bounded_rfc3339_and_are_validated_alike() {
        for accepted in ["2026-08-15T09:30:00Z", "2026-08-15T09:30:00.123456Z"] {
            let mut value = snapshot();
            value.generated_at = accepted.to_owned();
            value.expires_at = accepted.to_owned();
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
            let mut generated = snapshot();
            generated.generated_at = refused.to_owned();
            assert!(
                matches!(
                    validate_snapshot(&generated),
                    Err(PackError::InvalidField { ref field, .. }) if field == "generated_at"
                ),
                "{refused:?} must be refused as generated_at"
            );

            let mut expires = snapshot();
            expires.expires_at = refused.to_owned();
            assert!(
                matches!(
                    validate_snapshot(&expires),
                    Err(PackError::InvalidField { ref field, .. }) if field == "expires_at"
                ),
                "{refused:?} must be refused as expires_at"
            );
        }
    }

    #[test]
    fn a_published_serial_is_never_zero() {
        let mut value = snapshot();
        value.serial = 0;
        assert!(matches!(
            validate_snapshot(&value),
            Err(PackError::InvalidField { field, .. }) if field == "serial"
        ));
        value.serial = 1;
        validate_snapshot(&value).expect("the first published catalog is serial 1");
        value.serial = u64::MAX;
        validate_snapshot(&value).expect("the ordinal is not otherwise bounded");
    }

    #[test]
    fn only_snapshot_schema_two_is_read_or_written() {
        assert_eq!(SNAPSHOT_SCHEMA, 2);
        let key = signing_key(&[5_u8; 32]);
        let mut value = snapshot();
        value.schema = 1;
        assert!(matches!(
            validate_snapshot(&value),
            Err(PackError::InvalidField { field, .. }) if field == "schema"
        ));

        // A schema-1 document is refused at the envelope, before anything
        // inside it is modelled.
        let encoded = encode_snapshot(&snapshot(), &key).expect("encodes");
        let expanded =
            decompress_to_vec_zlib_with_limit(&encoded, 1 << 20).expect("test fixture expands");
        let text = String::from_utf8(expanded).expect("UTF-8");
        let downgraded = text.replacen("\"schema\":2", "\"schema\":1", 1);
        assert_ne!(downgraded, text);
        let recompressed = compress_to_vec_zlib(downgraded.as_bytes(), DEFLATE_LEVEL);
        assert!(matches!(
            decode_snapshot(&recompressed, &key.verifying_key(), &Limits::default()),
            Err(PackError::MalformedSnapshot(_))
        ));
    }

    #[test]
    fn revocations_round_trip_and_name_one_release_once() {
        let key = signing_key(&[5_u8; 32]);
        let absent = encode_snapshot(&snapshot(), &key).expect("encodes");
        let decoded =
            decode_snapshot(&absent, &key.verifying_key(), &Limits::default()).expect("decodes");
        assert!(decoded.revocations.is_empty());
        let text = String::from_utf8(
            decompress_to_vec_zlib_with_limit(&absent, 1 << 20).expect("expands"),
        )
        .expect("UTF-8");
        assert!(
            !text.contains("revocations"),
            "an empty recall list is absent, not empty: {text}"
        );

        let mut value = snapshot();
        value.revocations = vec![
            Revocation {
                pack_id: "rspice-opamps".to_owned(),
                version: "1.0.0".to_owned(),
                reason: "Ships an unlicensed vendor model".to_owned(),
            },
            Revocation {
                pack_id: "rspice-opamps".to_owned(),
                version: "1.1.0".to_owned(),
                reason: "Supersedes the recalled 1.0.0 without fixing it".to_owned(),
            },
        ];
        let encoded = encode_snapshot(&value, &key).expect("encodes");
        assert_eq!(
            decode_snapshot(&encoded, &key.verifying_key(), &Limits::default()).expect("decodes"),
            value
        );
        // A recalled release keeps its catalog entry: the bytes stay
        // resolvable for a project that pinned them.
        assert_eq!(value.packs[0].releases[0].version, "1.0.0");

        let mut repeated = value.clone();
        repeated.revocations.push(repeated.revocations[0].clone());
        let error = validate_snapshot(&repeated).expect_err("one release is recalled once");
        assert!(
            error.to_string().contains("revoked more than once"),
            "{error}"
        );

        for (pack_id, version, reason) in [
            ("opamps", "1.0.0", "Recalled"),
            ("rspice-opamps", "1.0", "Recalled"),
            ("rspice-opamps", "1.0.0", ""),
            ("rspice-opamps", "1.0.0", " Recalled"),
        ] {
            let mut refused = snapshot();
            refused.revocations = vec![Revocation {
                pack_id: pack_id.to_owned(),
                version: version.to_owned(),
                reason: reason.to_owned(),
            }];
            assert!(
                validate_snapshot(&refused).is_err(),
                "{pack_id:?} {version:?} {reason:?} must be refused"
            );
        }
    }

    #[test]
    fn part_presentation_is_bounded_at_its_published_limits() {
        let key = signing_key(&[5_u8; 32]);
        let mut value = snapshot();
        value.packs[0].releases[0].parts[0].specs = (0..6)
            .map(|index| (format!("spec{index}"), "x".repeat(32)))
            .collect();
        value.packs[0].releases[0].parts[0].description = Some("d".repeat(128));
        let encoded = encode_snapshot(&value, &key).expect("the published limits are signable");
        assert_eq!(
            decode_snapshot(&encoded, &key.verifying_key(), &Limits::default()).expect("decodes"),
            value
        );

        let mut seventh = value.clone();
        seventh.packs[0].releases[0].parts[0]
            .specs
            .insert("spec6".to_owned(), "1 V".to_owned());
        assert!(matches!(
            validate_snapshot(&seventh),
            Err(PackError::InvalidField { field, .. })
                if field == "packs[].releases[].parts[].specs"
        ));

        for (key_name, spec_value) in [
            ("spec0", "x".repeat(33)),
            ("spec0", String::new()),
            ("spec0", " 5 V".to_owned()),
            ("has space", "5 V".to_owned()),
            ("quo\"ted", "5 V".to_owned()),
            ("", "5 V".to_owned()),
        ] {
            let mut refused = snapshot();
            refused.packs[0].releases[0].parts[0].specs =
                BTreeMap::from([(key_name.to_owned(), spec_value.clone())]);
            assert!(
                validate_snapshot(&refused).is_err(),
                "{key_name:?} => {spec_value:?} must be refused"
            );
        }

        for refused_description in ["", " ", "d".repeat(129).as_str(), " trimmed"] {
            let mut refused = snapshot();
            refused.packs[0].releases[0].parts[0].description =
                Some(refused_description.to_owned());
            assert!(
                matches!(
                    validate_snapshot(&refused),
                    Err(PackError::InvalidField { ref field, .. })
                        if field == "packs[].releases[].parts[].description"
                ),
                "{refused_description:?} must be refused"
            );
        }
    }

    #[test]
    fn absent_presentation_cannot_be_smuggled_back_as_empty_or_null() {
        let key = signing_key(&[5_u8; 32]);
        let mut value = snapshot();
        value.packs[0].releases[0].parts[0].description = None;
        value.packs[0].releases[0].parts[0].specs = BTreeMap::new();
        let encoded = encode_snapshot(&value, &key).expect("encodes");
        let text = String::from_utf8(
            decompress_to_vec_zlib_with_limit(&encoded, 1 << 20).expect("expands"),
        )
        .expect("UTF-8");
        assert!(!text.contains("description") && !text.contains("specs"));

        // The signature covers the snapshot re-canonicalized from its typed
        // projection, so an encoding that carries what the schema models as
        // absent decodes to exactly the signed content: the smuggled keys
        // cannot become facts a client acts on.
        for (needle, replacement) in [
            ("\"device\"", "\"description\":null,\"device\""),
            ("\"terminals\"", "\"specs\":{},\"terminals\""),
        ] {
            let perturbed = text.replacen(needle, replacement, 1);
            assert_ne!(perturbed, text);
            let recompressed = compress_to_vec_zlib(perturbed.as_bytes(), DEFLATE_LEVEL);
            let decoded = decode_snapshot(&recompressed, &key.verifying_key(), &Limits::default())
                .expect("a canonically absent key is inert under the signature");
            assert_eq!(decoded, value);
        }

        // Content, by contrast, changes the bytes the signature covers.
        let smuggled = text.replacen(
            "\"terminals\"",
            "\"specs\":{\"supply\":\"5 V\"},\"terminals\"",
            1,
        );
        assert_ne!(smuggled, text);
        let recompressed = compress_to_vec_zlib(smuggled.as_bytes(), DEFLATE_LEVEL);
        assert!(matches!(
            decode_snapshot(&recompressed, &key.verifying_key(), &Limits::default()),
            Err(PackError::BadSignature)
        ));
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

    #[test]
    fn part_identities_may_repeat_across_versions_but_not_across_packs() {
        let mut value = snapshot();
        let newer = value.packs[0].releases[0].clone();
        value.packs[0].releases.push(SnapshotRelease {
            version: "1.1.0".to_owned(),
            ..newer
        });
        validate_snapshot(&value).expect("one pack may release the same part again");

        let mut competing = value.packs[0].clone();
        competing.id = "rspice-another-opamps".to_owned();
        competing.releases.truncate(1);
        competing.releases[0].version = "2.0.0".to_owned();
        competing.releases[0].parts[0].id = "tl-072".to_owned();
        competing.releases[0].parts[0].aliases.clear();
        value.packs.push(competing);
        let error = validate_snapshot(&value).expect_err("another pack cannot claim the part");
        assert!(error.to_string().contains("collides"), "{error}");
    }

    #[test]
    fn snapshots_may_not_shadow_foundation_models() {
        let mut value = snapshot();
        value.packs[0].releases[0].parts[0].aliases = vec!["rspice-opamp".to_owned()];
        let error = validate_snapshot(&value).expect_err("foundation names are reserved");
        assert!(error.to_string().contains("foundation library"), "{error}");
    }
}
