//! License key parsing, verification, and local storage.
//!
//! Implements the internal license-key spec end to end:
//!
//! - Wire format `RSPICE-K1.<base32(payload)>.<base32(signature)>` in
//!   Crockford base32 (uppercase, no padding; grouping dashes and
//!   whitespace are cosmetic).
//! - Little-endian binary payload: version, key id, license id, tier,
//!   seats, issued/expires (unix days), feature bitfield, then TLVs
//!   (0x01 licensee name, 0x02 email hash; unknown types ignored).
//! - Ed25519 signature over `"rspice-license-v1" || payload` — the
//!   domain-separation tag prevents cross-protocol reuse.
//! - Verification against public keys compiled into the binary, selected
//!   by the payload's key id, then a denylist check.
//! - **Perpetual-fallback semantics**: `expires` is an updates-until
//!   date, not a kill switch — a key activates any build whose release
//!   date is within term, and later builds keep the last in-term feature
//!   set. Comparison is against the compiled-in release date, never the
//!   wall clock.
//!
//! Key material: key id `0x01` is the **development** signer used for
//! internal builds and the sample-key walkthrough. Production releases
//! add ceremony keys (new ids) from the platform backend and drop the
//! dev id. Keys are generated/issued with the `license_tool` example:
//! `cargo run -p rspice-ui --example license_tool -- gen|issue …`.

use ed25519_dalek::{Signature, Verifier, VerifyingKey};
use serde::{Deserialize, Serialize};

/// The version tag every key starts with.
const KEY_TAG: &str = "RSPICE-K1.";

/// Domain-separation tag prepended to the payload before signing
/// (public so the issuing tool signs exactly what we verify).
pub const SIGNING_DOMAIN: &[u8] = b"rspice-license-v1";

/// This build's release date in unix days — the reference point for the
/// perpetual-fallback updates window. Bump per release.
pub const RELEASE_UNIX_DAYS: u32 = 20_615; // 2026-06-11

/// Signer public keys compiled into the binary, by key id.
///
/// Id `0x01` is the development signer (see module docs); its secret is
/// held locally by the maintainer, never in the repository.
const VERIFYING_KEYS: &[(u8, [u8; 32])] = &[(
    0x01,
    [
        0xbd, 0x8c, 0x6d, 0x6a, 0xa9, 0xbb, 0x49, 0x1c, 0xb5, 0xcf, 0x8f, 0x08, 0xa1, 0x50, 0x8a,
        0x4e, 0x49, 0x4d, 0x43, 0x48, 0xa0, 0x9d, 0x52, 0xbe, 0x1e, 0xe0, 0x83, 0xb2, 0xd9, 0xca,
        0x44, 0xc9,
    ],
)];

/// Revoked license ids (kept tiny; updated per release).
const DENYLIST: &[u64] = &[];

/// A well-formed sample key for the dialog's "Paste sample key"
/// walkthrough — genuinely signed by the development key (id 0x01), so it
/// exercises the full verification path.
pub const SAMPLE_KEY: &str = "RSPICE-K1.040MW2YJEZ0NGEMZ040011TG000F6M80003G0000047MMRB9DNJJ0NV8D5T6CTB5DHJ0.12FX61MG7FWYV6CJSBQFTGHG954N0K1FE2EQ93FG33HM6R9CPEF35VT55XFX6VYZ2SJ3WW2K34P97SRS8QPPCHM474C0M3DCNW6VW30";

/// Feature bitfield labels, LSB first (spec §payload).
const FEATURE_LABELS: &[(u32, &str)] = &[
    (1, "RF suite — HB · PSS · PNoise · PAC"),
    (2, "Python API"),
    (4, "Encrypted models"),
    (8, "Cloud runners"),
];

/// A verified license grant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LicenseInfo {
    pub licensed_to: String,
    pub tier: String,
    pub updates_until: String,
    pub license_id: String,
    pub features: Vec<String>,
    /// True when this build postdates the updates window — the key still
    /// activates (possession token), with the last in-term feature set.
    #[serde(default)]
    pub updates_expired: bool,
}

/// Why a key failed to verify, with the user-facing message the dialog
/// shows under the field.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LicenseError {
    /// Doesn't even look like a key.
    Malformed(&'static str),
    /// Looks like a key, signature doesn't verify.
    BadSignature,
    /// Properly signed but revoked.
    Revoked,
}

impl LicenseError {
    pub fn message(&self) -> &'static str {
        match self {
            LicenseError::Malformed(message) => message,
            LicenseError::BadSignature => {
                "Signature check failed — the key looks truncated or altered. Paste the RSPICE-K1.… string in full, including both dot-separated parts."
            }
            LicenseError::Revoked => {
                "This license id has been revoked. Contact support with the id from your purchase email."
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Crockford base32
// ---------------------------------------------------------------------------

const CROCKFORD: &[u8; 32] = b"0123456789ABCDEFGHJKMNPQRSTVWXYZ";

/// Encode bytes as uppercase Crockford base32, no padding.
pub fn crockford_encode(data: &[u8]) -> String {
    let mut out = String::with_capacity(data.len().div_ceil(5) * 8);
    let mut acc: u64 = 0;
    let mut bits = 0u32;
    for &byte in data {
        acc = (acc << 8) | u64::from(byte);
        bits += 8;
        while bits >= 5 {
            bits -= 5;
            out.push(CROCKFORD[((acc >> bits) & 0x1f) as usize] as char);
        }
    }
    if bits > 0 {
        out.push(CROCKFORD[((acc << (5 - bits)) & 0x1f) as usize] as char);
    }
    out
}

/// Decode Crockford base32 (case-insensitive; I/L→1, O→0 per Crockford).
pub fn crockford_decode(text: &str) -> Option<Vec<u8>> {
    let mut out = Vec::with_capacity(text.len() * 5 / 8);
    let mut acc: u64 = 0;
    let mut bits = 0u32;
    for ch in text.chars() {
        let ch = ch.to_ascii_uppercase();
        let value = match ch {
            'O' => 0,
            'I' | 'L' => 1,
            _ => {
                let index = CROCKFORD.iter().position(|&c| c as char == ch)?;
                index as u8
            }
        };
        acc = (acc << 5) | u64::from(value);
        bits += 5;
        if bits >= 8 {
            bits -= 8;
            out.push(((acc >> bits) & 0xff) as u8);
        }
    }
    // Leftover sub-byte bits are encoder padding; they must be zero.
    if bits > 0 && (acc & ((1 << bits) - 1)) != 0 {
        return None;
    }
    Some(out)
}

/// Render a base32 string in groups of 5 separated by dashes (cosmetic,
/// for human transcription).
pub fn group5(text: &str) -> String {
    text.as_bytes()
        .chunks(5)
        .map(|chunk| std::str::from_utf8(chunk).unwrap_or_default())
        .collect::<Vec<_>>()
        .join("-")
}

// ---------------------------------------------------------------------------
// Payload
// ---------------------------------------------------------------------------

/// The decoded binary payload (spec §payload, little-endian).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LicensePayload {
    pub version: u8,
    pub key_id: u8,
    pub license_id: u64,
    pub tier: u8,
    pub seats: u16,
    pub issued_days: u32,
    pub expires_days: u32,
    pub features: u32,
    pub licensee: Option<String>,
    pub email_hash: Option<[u8; 16]>,
}

/// Fixed-size header length before the TLVs.
const PAYLOAD_HEADER: usize = 1 + 1 + 8 + 1 + 2 + 4 + 4 + 4;

impl LicensePayload {
    /// Serialize to the wire layout (used by the issuing tool; kept next
    /// to `parse` so the two can never drift).
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(PAYLOAD_HEADER + 64);
        out.push(self.version);
        out.push(self.key_id);
        out.extend_from_slice(&self.license_id.to_le_bytes());
        out.push(self.tier);
        out.extend_from_slice(&self.seats.to_le_bytes());
        out.extend_from_slice(&self.issued_days.to_le_bytes());
        out.extend_from_slice(&self.expires_days.to_le_bytes());
        out.extend_from_slice(&self.features.to_le_bytes());
        if let Some(name) = &self.licensee {
            let bytes = name.as_bytes();
            let len = bytes.len().min(48);
            out.push(0x01);
            out.push(len as u8);
            out.extend_from_slice(&bytes[..len]);
        }
        if let Some(hash) = &self.email_hash {
            out.push(0x02);
            out.push(16);
            out.extend_from_slice(hash);
        }
        out
    }

    pub fn parse(data: &[u8]) -> Option<Self> {
        fn take<'a>(data: &'a [u8], at: &mut usize, n: usize) -> Option<&'a [u8]> {
            let slice = data.get(*at..*at + n)?;
            *at += n;
            Some(slice)
        }

        if data.len() < PAYLOAD_HEADER {
            return None;
        }
        let at = &mut 0usize;
        let version = take(data, at, 1)?[0];
        let key_id = take(data, at, 1)?[0];
        let license_id = u64::from_le_bytes(take(data, at, 8)?.try_into().ok()?);
        let tier = take(data, at, 1)?[0];
        let seats = u16::from_le_bytes(take(data, at, 2)?.try_into().ok()?);
        let issued_days = u32::from_le_bytes(take(data, at, 4)?.try_into().ok()?);
        let expires_days = u32::from_le_bytes(take(data, at, 4)?.try_into().ok()?);
        let features = u32::from_le_bytes(take(data, at, 4)?.try_into().ok()?);

        let mut licensee = None;
        let mut email_hash = None;
        while *at < data.len() {
            let tlv_type = take(data, at, 1)?[0];
            let len = usize::from(take(data, at, 1)?[0]);
            let value = take(data, at, len)?;
            match tlv_type {
                0x01 => licensee = Some(String::from_utf8_lossy(value).into_owned()),
                0x02 => email_hash = value.try_into().ok(),
                _ => {} // unknown TLVs are ignored by design
            }
        }

        Some(Self {
            version,
            key_id,
            license_id,
            tier,
            seats,
            issued_days,
            expires_days,
            features,
            licensee,
            email_hash,
        })
    }

    fn tier_label(&self) -> &'static str {
        match self.tier {
            1 => "PRO",
            2 => "TEAM",
            3 => "ENTERPRISE",
            _ => "UNKNOWN",
        }
    }

    fn feature_labels(&self) -> Vec<String> {
        FEATURE_LABELS
            .iter()
            .filter(|(bit, _)| self.features & bit != 0)
            .map(|(_, label)| (*label).to_owned())
            .collect()
    }

    fn license_id_display(&self) -> String {
        let b = self.license_id.to_be_bytes();
        format!(
            "{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}",
            b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7]
        )
    }
}

/// Civil date from unix days (Howard Hinnant's algorithm), as YYYY-MM-DD.
pub fn date_from_unix_days(days: u32) -> String {
    let z = i64::from(days) + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z.rem_euclid(146_097);
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let year = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = doy - (153 * mp + 2) / 5 + 1;
    let month = if mp < 10 { mp + 3 } else { mp - 9 };
    let year = if month <= 2 { year + 1 } else { year };
    format!("{year:04}-{month:02}-{day:02}")
}

// ---------------------------------------------------------------------------
// Verification
// ---------------------------------------------------------------------------

/// Strip cosmetic whitespace and grouping dashes from a key part.
fn canonical(part: &str) -> String {
    part.chars()
        .filter(|c| !c.is_whitespace() && *c != '-')
        .collect::<String>()
        .to_ascii_uppercase()
}

/// Parse and verify a pasted key: structure → signature → denylist →
/// perpetual-fallback window.
pub fn parse_and_verify(raw: &str) -> Result<LicenseInfo, LicenseError> {
    let cleaned: String = raw.split_whitespace().collect();
    if cleaned.is_empty() {
        return Err(LicenseError::Malformed("The key is empty."));
    }
    let Some(rest) = cleaned.strip_prefix(KEY_TAG) else {
        return Err(LicenseError::Malformed(
            "Keys start with RSPICE-K1. — paste the whole string from your purchase email.",
        ));
    };
    let mut parts = rest.split('.');
    let (Some(payload_part), Some(signature_part), None) =
        (parts.next(), parts.next(), parts.next())
    else {
        return Err(LicenseError::Malformed(
            "Expected two dot-separated parts after the RSPICE-K1. tag.",
        ));
    };

    let payload_bytes = crockford_decode(&canonical(payload_part)).ok_or(
        LicenseError::Malformed("The key contains characters outside the base32 alphabet."),
    )?;
    let signature_bytes = crockford_decode(&canonical(signature_part)).ok_or(
        LicenseError::Malformed("The key contains characters outside the base32 alphabet."),
    )?;
    let signature_bytes: [u8; 64] = signature_bytes
        .try_into()
        .map_err(|_| LicenseError::BadSignature)?;

    let payload = LicensePayload::parse(&payload_bytes).ok_or(LicenseError::BadSignature)?;
    if payload.version != 1 {
        return Err(LicenseError::Malformed(
            "This key uses a newer layout than this build understands — update RSpice.",
        ));
    }

    let Some((_, key_bytes)) = VERIFYING_KEYS.iter().find(|(id, _)| *id == payload.key_id) else {
        return Err(LicenseError::BadSignature);
    };
    let verifying_key =
        VerifyingKey::from_bytes(key_bytes).map_err(|_| LicenseError::BadSignature)?;

    let mut message = Vec::with_capacity(SIGNING_DOMAIN.len() + payload_bytes.len());
    message.extend_from_slice(SIGNING_DOMAIN);
    message.extend_from_slice(&payload_bytes);
    verifying_key
        .verify(&message, &Signature::from_bytes(&signature_bytes))
        .map_err(|_| LicenseError::BadSignature)?;

    if DENYLIST.contains(&payload.license_id) {
        return Err(LicenseError::Revoked);
    }

    Ok(LicenseInfo {
        licensed_to: payload.licensee.clone().unwrap_or_else(|| "—".to_owned()),
        tier: payload.tier_label().to_owned(),
        updates_until: date_from_unix_days(payload.expires_days),
        license_id: payload.license_id_display(),
        features: payload.feature_labels(),
        updates_expired: RELEASE_UNIX_DAYS > payload.expires_days,
    })
}

// ---------------------------------------------------------------------------
// Storage
// ---------------------------------------------------------------------------

/// Where the activated key lives on disk (native): `%APPDATA%\rspice\license.key`.
#[cfg(not(target_arch = "wasm32"))]
pub fn license_file_path() -> Option<std::path::PathBuf> {
    Some(dirs::config_dir()?.join("rspice").join("license.key"))
}

/// Persist the activated key (native). Browser builds rely on the session
/// storage carrying the key string instead.
#[cfg(not(target_arch = "wasm32"))]
pub fn store_key(raw: &str) -> std::io::Result<()> {
    let Some(path) = license_file_path() else {
        return Err(std::io::Error::other("no config directory"));
    };
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(path, raw.trim())
}

/// Load and verify a previously stored key (native).
#[cfg(not(target_arch = "wasm32"))]
pub fn load_stored() -> Option<(String, LicenseInfo)> {
    let path = license_file_path()?;
    let raw = std::fs::read_to_string(path).ok()?;
    let info = parse_and_verify(&raw).ok()?;
    Some((raw, info))
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crockford_roundtrip() {
        for data in [&b""[..], &b"f"[..], &b"foob"[..], &[0u8, 255, 7, 99][..]] {
            let encoded = crockford_encode(data);
            assert_eq!(crockford_decode(&encoded).as_deref(), Some(data));
        }
    }

    #[test]
    fn sample_key_verifies_with_expected_grant() {
        let info = parse_and_verify(SAMPLE_KEY).expect("sample key must verify");
        assert_eq!(info.tier, "PRO");
        assert_eq!(info.licensed_to, "Jaime Whitfield");
        assert_eq!(info.updates_until, "2027-06-10");
        assert!(!info.updates_expired);
        assert_eq!(info.features.len(), 3);
    }

    #[test]
    fn tampered_key_fails() {
        // Flip one payload character.
        let mut tampered = SAMPLE_KEY.to_owned();
        let at = KEY_TAG.len() + 3;
        let original = tampered.as_bytes()[at];
        let replacement = if original == b'A' { b'B' } else { b'A' };
        unsafe { tampered.as_bytes_mut()[at] = replacement };
        assert!(matches!(
            parse_and_verify(&tampered),
            Err(LicenseError::BadSignature) | Err(LicenseError::Malformed(_))
        ));
    }

    #[test]
    fn grouping_and_case_are_cosmetic() {
        let (payload, signature) = SAMPLE_KEY
            .strip_prefix(KEY_TAG)
            .unwrap()
            .split_once('.')
            .unwrap();
        let pretty = format!(
            "RSPICE-K1.{}.{}",
            group5(&payload.to_ascii_lowercase()),
            group5(signature)
        );
        assert!(parse_and_verify(&pretty).is_ok());
    }

    #[test]
    fn dates_render_civil() {
        assert_eq!(date_from_unix_days(0), "1970-01-01");
        assert_eq!(date_from_unix_days(20_615), "2026-06-11");
    }
}
