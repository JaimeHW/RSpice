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
//! internal builds and the sample-key walkthrough. Its verifier and the
//! signed sample are compiled only for debug and test builds. Production
//! releases fail closed and accept only ceremony keys in
//! `PRODUCTION_VERIFYING_KEYS`. Keys are generated/issued with the
//! `license_tool` example:
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

/// Production ceremony public keys compiled into release binaries, by key id.
///
/// Keep development signers out of this table. An empty table intentionally
/// fails closed until the production issuing ceremony publishes its public
/// verifier.
const PRODUCTION_VERIFYING_KEYS: &[(u8, [u8; 32])] = &[];

/// Development signer public keys, available only to debug and test builds.
///
/// Id `0x01` is the development signer (see module docs); its secret is held
/// locally by the maintainer, never in the repository. The entire constant is
/// absent from optimized release binaries, rather than relying on a runtime
/// check that could accidentally be bypassed.
#[cfg(any(test, rspice_development_build))]
const DEVELOPMENT_VERIFYING_KEYS: &[(u8, [u8; 32])] = &[(
    0x01,
    [
        0xbd, 0x8c, 0x6d, 0x6a, 0xa9, 0xbb, 0x49, 0x1c, 0xb5, 0xcf, 0x8f, 0x08, 0xa1, 0x50, 0x8a,
        0x4e, 0x49, 0x4d, 0x43, 0x48, 0xa0, 0x9d, 0x52, 0xbe, 0x1e, 0xe0, 0x83, 0xb2, 0xd9, 0xca,
        0x44, 0xc9,
    ],
)];

/// Revoked license ids (kept tiny; updated per release).
const DENYLIST: &[u64] = &[];

/// A well-formed sample key for the debug dialog's "Paste sample key"
/// walkthrough — genuinely signed by the development key (id 0x01), so it
/// exercises the full verification path. It is not present in release builds.
#[cfg(any(test, rspice_development_build))]
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

/// Which signer set may authorize a key.
///
/// The production variant is always available. The development variant and
/// its corresponding key material do not exist in release builds.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(all(rspice_development_build, not(test)), allow(dead_code))]
enum VerificationPolicy {
    Production,
    #[cfg(any(test, rspice_development_build))]
    Development,
}

const fn build_verification_policy() -> VerificationPolicy {
    #[cfg(any(test, rspice_development_build))]
    {
        VerificationPolicy::Development
    }
    #[cfg(not(any(test, rspice_development_build)))]
    {
        VerificationPolicy::Production
    }
}

fn verifying_key_bytes(key_id: u8, policy: VerificationPolicy) -> Option<&'static [u8; 32]> {
    if let Some((_, key)) = PRODUCTION_VERIFYING_KEYS
        .iter()
        .find(|(id, _)| *id == key_id)
    {
        return Some(key);
    }

    #[cfg(any(test, rspice_development_build))]
    if policy == VerificationPolicy::Development {
        return DEVELOPMENT_VERIFYING_KEYS
            .iter()
            .find(|(id, _)| *id == key_id)
            .map(|(_, key)| key);
    }

    #[cfg(not(any(test, rspice_development_build)))]
    let _ = policy;
    None
}

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
    parse_and_verify_with_policy(raw, build_verification_policy())
}

fn parse_and_verify_with_policy(
    raw: &str,
    policy: VerificationPolicy,
) -> Result<LicenseInfo, LicenseError> {
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

    let Some(key_bytes) = verifying_key_bytes(payload.key_id, policy) else {
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
    store_key_to_path(&path, raw)
}

#[cfg(not(target_arch = "wasm32"))]
fn store_key_to_path(path: &std::path::Path, raw: &str) -> std::io::Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    #[cfg(unix)]
    tighten_existing_license_permissions(path)?;
    let expected = crate::io::durable_file::observe_expected_content(path)
        .map_err(|error| std::io::Error::other(error.to_string()))?;
    let encoded = encode_stored_key(raw.trim())?;
    publish_stored_key(path, expected, &encoded)
}

/// Load and verify a previously stored key (native).
#[cfg(not(target_arch = "wasm32"))]
pub fn load_stored() -> Option<(String, LicenseInfo)> {
    let path = license_file_path()?;
    load_stored_from_path(&path)
}

#[cfg(not(target_arch = "wasm32"))]
fn load_stored_from_path(path: &std::path::Path) -> Option<(String, LicenseInfo)> {
    #[cfg(unix)]
    {
        tighten_existing_license_permissions(path).ok()?;
        crate::io::durable_file::reconcile_publication(path).ok()?;
    }
    #[cfg(windows)]
    let expected = crate::io::durable_file::observe_expected_content(path).ok()?;
    let stored = std::fs::read(path).ok()?;
    let (raw, legacy_plaintext) = decode_stored_key(&stored).ok()?;
    let info = parse_and_verify(&raw).ok()?;
    #[cfg(windows)]
    if legacy_plaintext {
        let encrypted = encode_stored_key(raw.trim()).ok()?;
        publish_stored_key(path, expected, &encrypted).ok()?;
    }
    #[cfg(not(windows))]
    let _ = legacy_plaintext;
    Some((raw, info))
}

#[cfg(not(target_arch = "wasm32"))]
fn publish_stored_key(
    path: &std::path::Path,
    expected: crate::io::durable_file::ExpectedContent,
    encoded: &[u8],
) -> std::io::Result<()> {
    #[cfg(unix)]
    {
        crate::io::durable_file::compare_exchange_bytes_owner_only(path, expected, encoded)
            .map_err(|error| std::io::Error::other(error.to_string()))
    }
    #[cfg(windows)]
    {
        // DPAPI ciphertext is safe to place in the generic durable staging
        // and recovery machinery; plaintext never reaches those files.
        crate::io::durable_file::compare_exchange_bytes(path, expected, encoded)
            .map_err(|error| std::io::Error::other(error.to_string()))
    }
}

#[cfg(unix)]
fn tighten_existing_license_permissions(path: &std::path::Path) -> std::io::Result<()> {
    use std::os::unix::fs::{MetadataExt as _, PermissionsExt as _};

    let named = match std::fs::symlink_metadata(path) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(()),
        Err(error) => return Err(error),
    };
    if !named.file_type().is_file() || named.nlink() != 1 {
        return Err(std::io::Error::other(format!(
            "license path '{}' is not one private regular file",
            path.display()
        )));
    }
    let file = std::fs::File::open(path)?;
    let opened = file.metadata()?;
    if opened.dev() != named.dev() || opened.ino() != named.ino() || opened.nlink() != 1 {
        return Err(std::io::Error::other(format!(
            "license path '{}' changed while permissions were checked",
            path.display()
        )));
    }
    file.set_permissions(std::fs::Permissions::from_mode(0o600))?;
    file.sync_all()
}

#[cfg(all(not(target_arch = "wasm32"), not(windows)))]
fn encode_stored_key(raw: &str) -> std::io::Result<Vec<u8>> {
    Ok(raw.as_bytes().to_vec())
}

#[cfg(all(not(target_arch = "wasm32"), not(windows)))]
fn decode_stored_key(stored: &[u8]) -> std::io::Result<(String, bool)> {
    String::from_utf8(stored.to_vec())
        .map(|raw| (raw, false))
        .map_err(|error| std::io::Error::new(std::io::ErrorKind::InvalidData, error))
}

#[cfg(windows)]
const WINDOWS_DPAPI_PREFIX: &[u8] = b"RSPICE-LICENSE-DPAPI\0\x01";

#[cfg(windows)]
fn encode_stored_key(raw: &str) -> std::io::Result<Vec<u8>> {
    let ciphertext = windows_dpapi_protect(raw.as_bytes())?;
    let total = WINDOWS_DPAPI_PREFIX
        .len()
        .checked_add(ciphertext.len())
        .ok_or_else(|| std::io::Error::other("encrypted license size overflow"))?;
    let mut encoded = Vec::new();
    encoded.try_reserve_exact(total).map_err(|error| {
        std::io::Error::other(format!("could not allocate encrypted license: {error}"))
    })?;
    encoded.extend_from_slice(WINDOWS_DPAPI_PREFIX);
    encoded.extend_from_slice(&ciphertext);
    Ok(encoded)
}

#[cfg(windows)]
fn decode_stored_key(stored: &[u8]) -> std::io::Result<(String, bool)> {
    let Some(ciphertext) = stored.strip_prefix(WINDOWS_DPAPI_PREFIX) else {
        return String::from_utf8(stored.to_vec())
            .map(|raw| (raw, true))
            .map_err(|error| std::io::Error::new(std::io::ErrorKind::InvalidData, error));
    };
    let plaintext = windows_dpapi_unprotect(ciphertext)?;
    String::from_utf8(plaintext)
        .map(|raw| (raw, false))
        .map_err(|error| std::io::Error::new(std::io::ErrorKind::InvalidData, error))
}

#[cfg(windows)]
fn windows_dpapi_protect(plaintext: &[u8]) -> std::io::Result<Vec<u8>> {
    use windows_sys::Win32::Security::Cryptography::{
        CRYPT_INTEGER_BLOB, CRYPTPROTECT_UI_FORBIDDEN, CryptProtectData,
    };

    let input_len = u32::try_from(plaintext.len())
        .map_err(|_| std::io::Error::other("license key is too large for DPAPI"))?;
    let input = CRYPT_INTEGER_BLOB {
        cbData: input_len,
        pbData: plaintext.as_ptr().cast_mut(),
    };
    let mut output = CRYPT_INTEGER_BLOB::default();
    let succeeded = unsafe {
        CryptProtectData(
            &input,
            std::ptr::null(),
            std::ptr::null(),
            std::ptr::null(),
            std::ptr::null(),
            CRYPTPROTECT_UI_FORBIDDEN,
            &mut output,
        )
    };
    if succeeded == 0 {
        return Err(std::io::Error::last_os_error());
    }
    copy_and_free_dpapi_blob(output)
}

#[cfg(windows)]
fn windows_dpapi_unprotect(ciphertext: &[u8]) -> std::io::Result<Vec<u8>> {
    use windows_sys::Win32::Security::Cryptography::{
        CRYPT_INTEGER_BLOB, CRYPTPROTECT_UI_FORBIDDEN, CryptUnprotectData,
    };

    let input_len = u32::try_from(ciphertext.len())
        .map_err(|_| std::io::Error::other("encrypted license is too large for DPAPI"))?;
    let input = CRYPT_INTEGER_BLOB {
        cbData: input_len,
        pbData: ciphertext.as_ptr().cast_mut(),
    };
    let mut output = CRYPT_INTEGER_BLOB::default();
    let succeeded = unsafe {
        CryptUnprotectData(
            &input,
            std::ptr::null_mut(),
            std::ptr::null(),
            std::ptr::null(),
            std::ptr::null(),
            CRYPTPROTECT_UI_FORBIDDEN,
            &mut output,
        )
    };
    if succeeded == 0 {
        return Err(std::io::Error::last_os_error());
    }
    copy_and_free_dpapi_blob(output)
}

#[cfg(windows)]
fn copy_and_free_dpapi_blob(
    blob: windows_sys::Win32::Security::Cryptography::CRYPT_INTEGER_BLOB,
) -> std::io::Result<Vec<u8>> {
    use windows_sys::Win32::Foundation::LocalFree;

    struct LocalBlob(*mut u8);
    impl Drop for LocalBlob {
        fn drop(&mut self) {
            if !self.0.is_null() {
                unsafe {
                    LocalFree(self.0.cast());
                }
            }
        }
    }

    let allocation = LocalBlob(blob.pbData);
    if blob.cbData > 0 && allocation.0.is_null() {
        return Err(std::io::Error::other(
            "DPAPI returned a null output allocation",
        ));
    }
    let bytes = if blob.cbData == 0 {
        &[]
    } else {
        unsafe { std::slice::from_raw_parts(allocation.0, blob.cbData as usize) }
    };
    Ok(bytes.to_vec())
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
    fn production_policy_rejects_development_signer() {
        assert_eq!(
            parse_and_verify_with_policy(SAMPLE_KEY, VerificationPolicy::Production),
            Err(LicenseError::BadSignature)
        );
    }

    #[test]
    fn development_policy_accepts_development_signer() {
        let info =
            parse_and_verify_with_policy(SAMPLE_KEY, VerificationPolicy::Development).unwrap();
        assert_eq!(info.tier, "PRO");
    }

    #[test]
    fn build_policy_matches_debug_or_test_configuration() {
        assert_eq!(build_verification_policy(), VerificationPolicy::Development);
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

    #[test]
    fn stored_key_round_trips_through_secure_publication() {
        let root = unique_temp_dir("round-trip");
        let path = root.join("license.key");

        store_key_to_path(&path, SAMPLE_KEY).expect("store key");
        let (raw, info) = load_stored_from_path(&path).expect("load key");

        assert_eq!(raw, SAMPLE_KEY);
        assert_eq!(info.tier, "PRO");
        std::fs::remove_dir_all(root).expect("remove fixture");
    }

    #[test]
    fn key_publication_rejects_late_external_change() {
        let root = unique_temp_dir("late-change");
        let path = root.join("license.key");
        store_key_to_path(&path, SAMPLE_KEY).expect("store predecessor");
        let expected =
            crate::io::durable_file::observe_expected_content(&path).expect("observe destination");
        let encoded = encode_stored_key(SAMPLE_KEY).expect("encode key");
        std::fs::write(&path, b"late external edit").expect("race destination");

        let result = publish_stored_key(&path, expected, &encoded);

        assert!(result.is_err());
        assert_eq!(std::fs::read(&path).unwrap(), b"late external edit");
        std::fs::remove_dir_all(root).expect("remove fixture");
    }

    #[cfg(unix)]
    #[test]
    fn unix_key_and_successor_stay_owner_only() {
        use std::os::unix::fs::PermissionsExt as _;

        let root = unique_temp_dir("owner-only");
        let path = root.join("license.key");
        std::fs::write(&path, SAMPLE_KEY).expect("write permissive predecessor");
        std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o644))
            .expect("make predecessor permissive");

        store_key_to_path(&path, SAMPLE_KEY).expect("store key");

        assert_eq!(
            std::fs::metadata(&path).unwrap().permissions().mode() & 0o777,
            0o600
        );
        std::fs::remove_dir_all(root).expect("remove fixture");
    }

    #[cfg(windows)]
    #[test]
    fn windows_key_file_contains_only_dpapi_ciphertext() {
        let root = unique_temp_dir("dpapi");
        let path = root.join("license.key");

        store_key_to_path(&path, SAMPLE_KEY).expect("store key");
        let stored = std::fs::read(&path).expect("read encrypted key");

        assert!(stored.starts_with(WINDOWS_DPAPI_PREFIX));
        assert!(
            !stored
                .windows(SAMPLE_KEY.len())
                .any(|window| window == SAMPLE_KEY.as_bytes())
        );
        assert!(load_stored_from_path(&path).is_some());
        std::fs::remove_dir_all(root).expect("remove fixture");
    }

    fn unique_temp_dir(label: &str) -> std::path::PathBuf {
        let root =
            std::env::temp_dir().join(format!("rspice-license-{label}-{}", uuid::Uuid::new_v4()));
        std::fs::create_dir_all(&root).expect("create fixture");
        root
    }
}
