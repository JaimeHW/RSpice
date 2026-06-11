//! License key parsing, verification, and local storage.
//!
//! Key format (per `design/volta-license-dialog.html` and the license-key
//! spec): `RSPICE-K1.<payload>.<signature>` — a version tag and two
//! dot-separated base32 groups; whitespace and grouping dashes inside the
//! parts are cosmetic. Verification is fully offline.
//!
//! The structural checks (tag, part shape) are final. The cryptographic
//! signature check is a placeholder pending the ed25519 signing backend
//! from the commercialization track — today the well-formed sample key
//! resolves to the demo PRO grant and any other well-formed key reports a
//! signature failure, which is exactly the honest behavior until real
//! keys exist.

use serde::{Deserialize, Serialize};

/// The version tag every key starts with.
const KEY_TAG: &str = "RSPICE-K1.";

/// The well-formed (cryptographically meaningless) sample key used by the
/// dialog's "Paste sample key" walkthrough. Kept in sync with the design.
pub const SAMPLE_KEY: &str = "RSPICE-K1.01A4G-K9PWD-3RT7M-ZB5RW-Q2XJH-66YVN-08CEF-LMA9S-D27TC-4UK3B.7HQGX-E5NMW-RA2VK-9JYPB-TC81D-ZS4FU-WQ6LH-XK0MR-G3VEC-N5JAT-PB7YW-D9QZK-2FHSL-M8XVG";

/// A verified license grant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LicenseInfo {
    pub licensed_to: String,
    pub tier: String,
    pub updates_until: String,
    pub license_id: String,
    pub features: Vec<String>,
}

/// Why a key failed to verify, with the user-facing message the dialog
/// shows under the field.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LicenseError {
    /// Doesn't even look like a key.
    Malformed(&'static str),
    /// Looks like a key, signature doesn't verify.
    BadSignature,
}

impl LicenseError {
    pub fn message(&self) -> &'static str {
        match self {
            LicenseError::Malformed(message) => message,
            LicenseError::BadSignature => {
                "Signature check failed — the key looks truncated. Paste the RSPICE-K1.… string in full, including both dot-separated parts."
            }
        }
    }
}

/// Strip cosmetic whitespace and grouping dashes from a key part.
fn canonical(part: &str) -> String {
    part.chars()
        .filter(|c| !c.is_whitespace() && *c != '-')
        .collect::<String>()
        .to_ascii_uppercase()
}

/// Parse and verify a pasted key.
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
    let (Some(payload), Some(signature), None) = (parts.next(), parts.next(), parts.next())
    else {
        return Err(LicenseError::Malformed(
            "Expected two dot-separated parts after the RSPICE-K1. tag.",
        ));
    };
    let payload = canonical(payload);
    let signature = canonical(signature);
    if payload.len() < 32 || signature.len() < 48 {
        return Err(LicenseError::BadSignature);
    }
    if !payload.chars().chain(signature.chars()).all(|c| c.is_ascii_alphanumeric()) {
        return Err(LicenseError::Malformed(
            "Keys contain only letters and digits between the separators.",
        ));
    }

    // ed25519 verification slots in here; until the signing backend lands,
    // only the documented sample key verifies.
    let sample = SAMPLE_KEY.strip_prefix(KEY_TAG).expect("sample key tag");
    let mut sample_parts = sample.split('.');
    let sample_payload = canonical(sample_parts.next().unwrap_or_default());
    let sample_signature = canonical(sample_parts.next().unwrap_or_default());
    if payload == sample_payload && signature == sample_signature {
        return Ok(LicenseInfo {
            licensed_to: "Jaime Whitfield".to_owned(),
            tier: "PRO".to_owned(),
            updates_until: "2027-06-10".to_owned(),
            license_id: "9f3a-58c1-77d2-0b4e".to_owned(),
            features: vec![
                "RF suite — HB · PSS · PNoise · PAC".to_owned(),
                "Python API".to_owned(),
                "Encrypted models".to_owned(),
            ],
        });
    }
    Err(LicenseError::BadSignature)
}

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
