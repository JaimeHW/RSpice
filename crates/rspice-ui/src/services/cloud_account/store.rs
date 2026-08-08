//! Protected at-rest storage for the cloud session (native).
//!
//! One JSON record holds the refresh token, the stable device-binding secret
//! behind the lease fingerprint, the verified lease's compact token, and the
//! admitted license JWKS — everything offline startup needs. On Windows the
//! whole record is DPAPI-wrapped for the signed-in OS user; on Unix it is a
//! `0o600` file published through the owner-only durable-file path. This
//! mirrors the offline license key's storage in `services::license`, and the
//! record never reaches eframe's general app-state persistence.

use base64::Engine as _;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use sha2::Digest as _;

/// Version-tagged persisted session record.
#[derive(Clone, serde::Deserialize, serde::Serialize)]
pub(super) struct CloudSessionRecord {
    pub version: u32,
    /// OIDC refresh token, present while a session is signed in.
    pub refresh_token: Option<String>,
    /// 32 random bytes (base64url, unpadded): the stable, privacy-reviewed
    /// device-binding material behind the lease fingerprint. Per-install,
    /// never derived from hardware serials.
    pub device_binding_secret: String,
    /// The lease request UUID, retained so a retried issuance reuses it.
    pub lease_request_id: Option<String>,
    /// Verified compact lease token for offline startup.
    pub lease_token: Option<String>,
    /// Admitted license JWKS (serialized), verified before storage.
    pub license_jwks: Option<String>,
    /// Identity cache so the console can name the account while offline.
    pub principal_email: Option<String>,
    pub principal_display_name: Option<String>,
}

pub(super) const RECORD_VERSION: u32 = 1;

impl CloudSessionRecord {
    /// A fresh record with a newly generated device-binding secret.
    pub(super) fn new_with_binding_secret() -> std::io::Result<Self> {
        let mut secret = [0_u8; 32];
        getrandom::fill(&mut secret)
            .map_err(|_| std::io::Error::other("no entropy source for device binding"))?;
        Ok(Self {
            version: RECORD_VERSION,
            refresh_token: None,
            device_binding_secret: URL_SAFE_NO_PAD.encode(secret),
            lease_request_id: None,
            lease_token: None,
            license_jwks: None,
            principal_email: None,
            principal_display_name: None,
        })
    }

    /// Unpadded base64url SHA-256 digest of the binding secret — the exact
    /// `device_fingerprint_sha256` the lease contract requires.
    pub(super) fn device_fingerprint_sha256(&self) -> Option<String> {
        let secret = URL_SAFE_NO_PAD.decode(&self.device_binding_secret).ok()?;
        if secret.len() != 32 {
            return None;
        }
        Some(URL_SAFE_NO_PAD.encode(sha2::Sha256::digest(&secret)))
    }
}

/// Where the record lives: `<config>/rspice/cloud-session.dat`.
pub(super) fn record_path() -> Option<std::path::PathBuf> {
    Some(dirs::config_dir()?.join("rspice").join("cloud-session.dat"))
}

pub(super) fn load() -> Option<CloudSessionRecord> {
    load_from_path(&record_path()?)
}

pub(super) fn save(record: &CloudSessionRecord) -> std::io::Result<()> {
    let Some(path) = record_path() else {
        return Err(std::io::Error::other("no config directory"));
    };
    save_to_path(&path, record)
}

pub(super) fn load_from_path(path: &std::path::Path) -> Option<CloudSessionRecord> {
    #[cfg(unix)]
    {
        tighten_permissions(path).ok()?;
        crate::io::durable_file::reconcile_publication(path).ok()?;
    }
    let stored = std::fs::read(path).ok()?;
    let plaintext = decode_record_bytes(&stored).ok()?;
    let record: CloudSessionRecord = serde_json::from_slice(&plaintext).ok()?;
    if record.version != RECORD_VERSION {
        return None;
    }
    Some(record)
}

pub(super) fn save_to_path(
    path: &std::path::Path,
    record: &CloudSessionRecord,
) -> std::io::Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    #[cfg(unix)]
    tighten_permissions(path)?;
    let expected = crate::io::durable_file::observe_expected_content(path)
        .map_err(|error| std::io::Error::other(error.to_string()))?;
    let plaintext =
        serde_json::to_vec(record).map_err(|error| std::io::Error::other(error.to_string()))?;
    let encoded = encode_record_bytes(&plaintext)?;
    #[cfg(unix)]
    {
        crate::io::durable_file::compare_exchange_bytes_owner_only(path, expected, &encoded)
            .map_err(|error| std::io::Error::other(error.to_string()))
    }
    #[cfg(not(unix))]
    {
        // DPAPI ciphertext is safe in the generic durable staging machinery;
        // plaintext never reaches those files.
        crate::io::durable_file::compare_exchange_bytes(path, expected, &encoded)
            .map_err(|error| std::io::Error::other(error.to_string()))
    }
}

#[cfg(windows)]
const WINDOWS_DPAPI_PREFIX: &[u8] = b"RSPICE-CLOUD-DPAPI\0\x01";

#[cfg(windows)]
fn encode_record_bytes(plaintext: &[u8]) -> std::io::Result<Vec<u8>> {
    let ciphertext = crate::services::dpapi::protect(plaintext)?;
    let mut encoded = Vec::with_capacity(WINDOWS_DPAPI_PREFIX.len() + ciphertext.len());
    encoded.extend_from_slice(WINDOWS_DPAPI_PREFIX);
    encoded.extend_from_slice(&ciphertext);
    Ok(encoded)
}

#[cfg(windows)]
fn decode_record_bytes(stored: &[u8]) -> std::io::Result<Vec<u8>> {
    let Some(ciphertext) = stored.strip_prefix(WINDOWS_DPAPI_PREFIX) else {
        // Unlike the license key there is no legacy plaintext generation to
        // migrate: an unwrapped record is not ours.
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "cloud session record is not DPAPI-wrapped",
        ));
    };
    crate::services::dpapi::unprotect(ciphertext)
}

#[cfg(not(windows))]
fn encode_record_bytes(plaintext: &[u8]) -> std::io::Result<Vec<u8>> {
    Ok(plaintext.to_vec())
}

#[cfg(not(windows))]
fn decode_record_bytes(stored: &[u8]) -> std::io::Result<Vec<u8>> {
    Ok(stored.to_vec())
}

#[cfg(unix)]
fn tighten_permissions(path: &std::path::Path) -> std::io::Result<()> {
    use std::os::unix::fs::{MetadataExt as _, PermissionsExt as _};

    let named = match std::fs::symlink_metadata(path) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(()),
        Err(error) => return Err(error),
    };
    if !named.file_type().is_file() || named.nlink() != 1 {
        return Err(std::io::Error::other(format!(
            "cloud session path '{}' is not one private regular file",
            path.display()
        )));
    }
    let file = std::fs::File::open(path)?;
    let opened = file.metadata()?;
    if opened.dev() != named.dev() || opened.ino() != named.ino() || opened.nlink() != 1 {
        return Err(std::io::Error::other(format!(
            "cloud session path '{}' changed while permissions were checked",
            path.display()
        )));
    }
    file.set_permissions(std::fs::Permissions::from_mode(0o600))?;
    file.sync_all()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn record_roundtrips_through_protected_storage() {
        let directory =
            std::env::temp_dir().join(format!("rspice-cloud-store-{}", uuid::Uuid::new_v4()));
        let path = directory.join("cloud-session.dat");
        let mut record = CloudSessionRecord::new_with_binding_secret().expect("entropy");
        record.refresh_token = Some("refresh-secret".to_owned());
        record.principal_email = Some("engineer@example.com".to_owned());
        save_to_path(&path, &record).expect("save");

        let loaded = load_from_path(&path).expect("load");
        assert_eq!(loaded.refresh_token.as_deref(), Some("refresh-secret"));
        assert_eq!(loaded.device_binding_secret, record.device_binding_secret);

        #[cfg(windows)]
        {
            let raw = std::fs::read(&path).expect("raw bytes");
            assert!(raw.starts_with(WINDOWS_DPAPI_PREFIX));
            assert!(
                !raw.windows(b"refresh-secret".len())
                    .any(|w| w == b"refresh-secret"),
                "the refresh token must never reach disk as plaintext"
            );
        }
        let _ = std::fs::remove_dir_all(&directory);
    }

    #[test]
    fn device_fingerprint_is_unpadded_base64url_sha256_of_the_secret() {
        let record = CloudSessionRecord::new_with_binding_secret().expect("entropy");
        let fingerprint = record.device_fingerprint_sha256().expect("fingerprint");
        let secret = URL_SAFE_NO_PAD
            .decode(&record.device_binding_secret)
            .unwrap();
        assert_eq!(
            fingerprint,
            URL_SAFE_NO_PAD.encode(sha2::Sha256::digest(&secret))
        );
        let decoded = URL_SAFE_NO_PAD.decode(&fingerprint).unwrap();
        assert_eq!(decoded.len(), 32);
    }

    #[test]
    fn a_version_bump_invalidates_the_record() {
        let directory =
            std::env::temp_dir().join(format!("rspice-cloud-store-{}", uuid::Uuid::new_v4()));
        let path = directory.join("cloud-session.dat");
        let mut record = CloudSessionRecord::new_with_binding_secret().expect("entropy");
        record.version = RECORD_VERSION + 1;
        save_to_path(&path, &record).expect("save");
        assert!(load_from_path(&path).is_none());
        let _ = std::fs::remove_dir_all(&directory);
    }
}
