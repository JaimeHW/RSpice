//! Loading and saving PDK configuration.

use std::path::{Path, PathBuf};

use super::*;

#[cfg(any(test, target_arch = "wasm32"))]
use crate::product::ContentDigest;
#[cfg(any(test, target_arch = "wasm32"))]
use sha2::{Digest as _, Sha256};

#[cfg(target_arch = "wasm32")]
mod browser;
#[cfg(target_arch = "wasm32")]
pub(crate) use browser::{start_browser_pdk_config_load, start_browser_pdk_config_save};

#[cfg(any(test, target_arch = "wasm32"))]
const BROWSER_PDK_SCHEMA_VERSION: u32 = 1;
#[cfg(target_arch = "wasm32")]
const BROWSER_PDK_DATABASE: &str = "rspice-pdk-config";
#[cfg(target_arch = "wasm32")]
const BROWSER_PDK_STORE: &str = "content-addressed-pdk-config";
#[cfg(any(test, target_arch = "wasm32"))]
const MAX_BROWSER_PDK_GENERATION: u64 = (1_u64 << 53) - 1;
#[cfg(any(test, target_arch = "wasm32"))]
const MAX_BROWSER_PDK_METADATA_BYTES: usize = 32 * 1024 * 1024;
#[cfg(target_arch = "wasm32")]
const BROWSER_PDK_ASYNC_TIMEOUT_MS: u32 = 30_000;
#[cfg(target_arch = "wasm32")]
const BROWSER_PDK_STORAGE_MANAGER_TIMEOUT_MS: u32 = 5_000;
#[cfg(target_arch = "wasm32")]
const MAX_RETAINED_BROWSER_PDK_EVENT_OPERATIONS: usize = 32;
#[cfg(any(test, target_arch = "wasm32"))]
const BROWSER_PDK_RECORD_OVERHEAD_BYTES: u64 = 4 * 1024;
#[cfg(any(test, target_arch = "wasm32"))]
const BROWSER_PDK_MIN_QUOTA_RESERVE_BYTES: u64 = 8 * 1024 * 1024;
#[cfg(any(test, target_arch = "wasm32"))]
const BROWSER_PDK_MAX_QUOTA_RESERVE_BYTES: u64 = 64 * 1024 * 1024;
#[cfg(any(test, target_arch = "wasm32"))]
const BROWSER_PDK_QUOTA_RESERVE_DIVISOR: u64 = 20;

#[cfg(any(test, target_arch = "wasm32"))]
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct BrowserPdkBlobRef {
    digest: ContentDigest,
    byte_len: u64,
}

#[cfg(any(test, target_arch = "wasm32"))]
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct BrowserPdkHead {
    schema_version: u32,
    generation: u64,
    path_digest: ContentDigest,
    metadata: BrowserPdkBlobRef,
    archives: Vec<BrowserPdkBlobRef>,
}

#[cfg(any(test, target_arch = "wasm32"))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct BrowserPdkConfigReceipt {
    head_key: String,
    generation: u64,
    head_digest: ContentDigest,
    pub(crate) storage_status: BrowserPdkStorageStatus,
}

#[cfg(any(test, target_arch = "wasm32"))]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) enum BrowserPdkStorageDurability {
    Persistent,
    BestEffort,
    #[default]
    Unknown,
}

#[cfg(any(test, target_arch = "wasm32"))]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) struct BrowserPdkStorageStatus {
    pub(crate) durability: BrowserPdkStorageDurability,
    pub(crate) usage_bytes: Option<u64>,
    pub(crate) quota_bytes: Option<u64>,
}

#[cfg(any(test, target_arch = "wasm32"))]
impl BrowserPdkStorageStatus {
    pub(crate) fn available_bytes(self) -> Option<u64> {
        Some(self.quota_bytes?.saturating_sub(self.usage_bytes?))
    }
}

#[cfg(target_arch = "wasm32")]
pub(crate) struct BrowserPdkConfigRestore {
    pub(crate) config: PdkConfig,
    pub(crate) receipt: Option<BrowserPdkConfigReceipt>,
    pub(crate) migrated_legacy_record: bool,
    pub(crate) storage_status: BrowserPdkStorageStatus,
}

#[cfg(any(test, target_arch = "wasm32"))]
struct BrowserPdkBlob {
    reference: BrowserPdkBlobRef,
    bytes: Vec<u8>,
}

#[cfg(any(test, target_arch = "wasm32"))]
struct PreparedBrowserPdkSnapshot {
    #[cfg(target_arch = "wasm32")]
    head_key: String,
    #[cfg(target_arch = "wasm32")]
    head: BrowserPdkHead,
    head_bytes: Vec<u8>,
    metadata: BrowserPdkBlob,
    archives: Vec<BrowserPdkBlob>,
}

impl PdkConfig {
    // =========================================================================
    // Persistence
    // =========================================================================

    /// Get the default configuration file path
    pub fn default_config_path() -> PathBuf {
        dirs::config_dir()
            .or_else(dirs::home_dir)
            .unwrap_or_else(|| PathBuf::from("."))
            .join("rspice")
            .join(CONFIG_FILE_NAME)
    }

    /// Load configuration from the default path. Only the native application
    /// starts this way: tests build a configuration explicitly, and the
    /// browser restores one through the asynchronous persistence workflow.
    #[cfg(all(not(test), not(target_arch = "wasm32")))]
    pub fn load() -> Result<Self, ConfigError> {
        Self::load_from(&Self::default_config_path())
    }

    /// Load configuration from a specific path
    #[cfg(not(target_arch = "wasm32"))]
    pub fn load_from(path: &Path) -> Result<Self, ConfigError> {
        crate::io::durable_file::reconcile_publication(path)
            .map_err(|e| ConfigError::Io(e.to_string()))?;
        if !path.exists() {
            return Ok(Self::default());
        }

        let content = std::fs::read_to_string(path).map_err(|e| ConfigError::Io(e.to_string()))?;

        serde_json::from_str(&content).map_err(|e| ConfigError::Parse(e.to_string()))
    }

    /// Save configuration to the default path
    pub fn save(&self) -> Result<(), ConfigError> {
        self.save_to(&Self::default_config_path())
    }

    /// Save configuration to a specific path
    pub fn save_to(&self, path: &Path) -> Result<(), ConfigError> {
        #[cfg(target_arch = "wasm32")]
        {
            let _ = (path, self);
            Err(ConfigError::Io(
                "browser PDK configuration publication is asynchronous; use the application persistence workflow"
                    .to_owned(),
            ))
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            // Ensure parent directory exists
            if let Some(parent) = path.parent() {
                std::fs::create_dir_all(parent).map_err(|e| ConfigError::Io(e.to_string()))?;
            }

            let expected = crate::io::durable_file::observe_expected_content(path)
                .map_err(|e| ConfigError::Io(e.to_string()))?;
            let content = serde_json::to_vec_pretty(self)
                .map_err(|e| ConfigError::Serialize(e.to_string()))?;

            publish_pdk_config(path, expected, &content)
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn publish_pdk_config(
    path: &Path,
    expected: crate::io::durable_file::ExpectedContent,
    content: &[u8],
) -> Result<(), ConfigError> {
    crate::io::durable_file::compare_exchange_bytes(path, expected, content)
        .map_err(|e| ConfigError::Io(e.to_string()))
}

#[cfg(any(test, target_arch = "wasm32"))]
fn pdk_content_digest(bytes: &[u8]) -> ContentDigest {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    ContentDigest::from_bytes(hasher.finalize().into())
}

#[cfg(any(test, target_arch = "wasm32"))]
fn browser_path_digest(path: &Path) -> ContentDigest {
    pdk_content_digest(path.to_string_lossy().as_bytes())
}

#[cfg(any(test, target_arch = "wasm32"))]
fn browser_head_key(path_digest: ContentDigest) -> String {
    format!("head:{path_digest}")
}

#[cfg(target_arch = "wasm32")]
fn browser_metadata_key(path_digest: ContentDigest, digest: ContentDigest) -> String {
    format!("metadata:{path_digest}:{digest}")
}

#[cfg(target_arch = "wasm32")]
fn browser_archive_key(digest: ContentDigest) -> String {
    format!("archive:{digest}")
}

#[cfg(any(test, target_arch = "wasm32"))]
fn checked_blob_ref(
    bytes: Vec<u8>,
    maximum: usize,
    label: &str,
) -> Result<BrowserPdkBlob, ConfigError> {
    if bytes.len() > maximum {
        return Err(ConfigError::Serialize(format!(
            "{label} is {} bytes, exceeding the {maximum}-byte browser persistence limit",
            bytes.len()
        )));
    }
    Ok(BrowserPdkBlob {
        reference: BrowserPdkBlobRef {
            digest: pdk_content_digest(&bytes),
            byte_len: u64::try_from(bytes.len()).map_err(|_| {
                ConfigError::Serialize(format!("{label} length does not fit in 64 bits"))
            })?,
        },
        bytes,
    })
}

#[cfg(any(test, target_arch = "wasm32"))]
fn browser_pdk_record_footprint(key: &str, bytes: &[u8]) -> Result<u64, ConfigError> {
    let key_bytes = u64::try_from(key.len())
        .map_err(|_| ConfigError::Serialize("browser PDK object key is too large".to_owned()))?;
    let value_bytes = u64::try_from(bytes.len())
        .map_err(|_| ConfigError::Serialize("browser PDK object is too large".to_owned()))?;
    key_bytes
        .checked_add(value_bytes)
        .and_then(|bytes| bytes.checked_add(BROWSER_PDK_RECORD_OVERHEAD_BYTES))
        .ok_or_else(|| ConfigError::Serialize("browser PDK storage estimate overflowed".to_owned()))
}

#[cfg(any(test, target_arch = "wasm32"))]
fn browser_pdk_quota_reserve(quota_bytes: u64) -> u64 {
    (quota_bytes / BROWSER_PDK_QUOTA_RESERVE_DIVISOR)
        .clamp(
            BROWSER_PDK_MIN_QUOTA_RESERVE_BYTES,
            BROWSER_PDK_MAX_QUOTA_RESERVE_BYTES,
        )
        .min(quota_bytes)
}

#[cfg(any(test, target_arch = "wasm32"))]
fn enforce_browser_pdk_storage_admission(
    status: BrowserPdkStorageStatus,
    required_new_bytes: u64,
) -> Result<(), ConfigError> {
    let (Some(quota_bytes), Some(available_bytes)) = (status.quota_bytes, status.available_bytes())
    else {
        return Ok(());
    };
    let reserve_bytes = browser_pdk_quota_reserve(quota_bytes);
    let admitted_bytes = available_bytes.saturating_sub(reserve_bytes);
    if required_new_bytes > admitted_bytes {
        return Err(ConfigError::Io(format!(
            "browser storage has {} bytes available, but this PDK publication requires approximately {} bytes plus a {}-byte safety reserve",
            available_bytes, required_new_bytes, reserve_bytes
        )));
    }
    Ok(())
}

#[cfg(any(test, target_arch = "wasm32"))]
fn canonical_json_bytes<T: serde::Serialize>(
    value: &T,
    label: &str,
) -> Result<Vec<u8>, ConfigError> {
    fn write_value(value: &serde_json::Value, output: &mut Vec<u8>) -> Result<(), ConfigError> {
        match value {
            serde_json::Value::Null => output.extend_from_slice(b"null"),
            serde_json::Value::Bool(value) => {
                output.extend_from_slice(if *value { b"true" } else { b"false" })
            }
            serde_json::Value::Number(value) => {
                output.extend_from_slice(value.to_string().as_bytes())
            }
            serde_json::Value::String(value) => serde_json::to_writer(output, value)
                .map_err(|error| ConfigError::Serialize(error.to_string()))?,
            serde_json::Value::Array(values) => {
                output.push(b'[');
                for (index, value) in values.iter().enumerate() {
                    if index > 0 {
                        output.push(b',');
                    }
                    write_value(value, output)?;
                }
                output.push(b']');
            }
            serde_json::Value::Object(values) => {
                output.push(b'{');
                let mut keys = values.keys().collect::<Vec<_>>();
                keys.sort_unstable();
                for (index, key) in keys.into_iter().enumerate() {
                    if index > 0 {
                        output.push(b',');
                    }
                    serde_json::to_writer(&mut *output, key)
                        .map_err(|error| ConfigError::Serialize(error.to_string()))?;
                    output.push(b':');
                    write_value(&values[key], output)?;
                }
                output.push(b'}');
            }
        }
        Ok(())
    }

    let value =
        serde_json::to_value(value).map_err(|error| ConfigError::Serialize(error.to_string()))?;
    let mut output = Vec::new();
    write_value(&value, &mut output).map_err(|error| {
        ConfigError::Serialize(format!("could not canonicalize {label}: {error}"))
    })?;
    Ok(output)
}

#[cfg(any(test, target_arch = "wasm32"))]
fn prepare_browser_pdk_snapshot(
    path: &Path,
    config: &PdkConfig,
    generation: u64,
) -> Result<PreparedBrowserPdkSnapshot, ConfigError> {
    if generation == 0 || generation > MAX_BROWSER_PDK_GENERATION {
        return Err(ConfigError::Serialize(format!(
            "browser PDK generation {generation} is outside the exact integer range"
        )));
    }

    let path_digest = browser_path_digest(path);
    let mut metadata_config = config.clone();
    let archive_payloads = metadata_config
        .technology_registry
        .take_archives_for_browser_persistence();
    let metadata = checked_blob_ref(
        canonical_json_bytes(&metadata_config, "PDK configuration metadata")?,
        MAX_BROWSER_PDK_METADATA_BYTES,
        "PDK configuration metadata",
    )?;

    let mut archives = Vec::with_capacity(archive_payloads.len());
    let mut identities = std::collections::HashSet::with_capacity(archive_payloads.len());
    for (index, archive) in archive_payloads.into_iter().enumerate() {
        let blob = checked_blob_ref(
            serde_json::to_vec(&archive)
                .map_err(|error| ConfigError::Serialize(error.to_string()))?,
            MAX_PDK_ARCHIVE_BYTES,
            &format!("signed PDK archive {}", index + 1),
        )?;
        if !identities.insert(blob.reference.digest) {
            return Err(ConfigError::Serialize(format!(
                "signed PDK archive digest {} appears more than once",
                blob.reference.digest
            )));
        }
        archives.push(blob);
    }

    let head = BrowserPdkHead {
        schema_version: BROWSER_PDK_SCHEMA_VERSION,
        generation,
        path_digest,
        metadata: metadata.reference.clone(),
        archives: archives
            .iter()
            .map(|archive| archive.reference.clone())
            .collect(),
    };
    let head_bytes =
        serde_json::to_vec(&head).map_err(|error| ConfigError::Serialize(error.to_string()))?;
    Ok(PreparedBrowserPdkSnapshot {
        #[cfg(target_arch = "wasm32")]
        head_key: browser_head_key(path_digest),
        #[cfg(target_arch = "wasm32")]
        head,
        head_bytes,
        metadata,
        archives,
    })
}

#[cfg(any(test, target_arch = "wasm32"))]
fn validate_browser_pdk_head(
    path: &Path,
    head_bytes: &[u8],
) -> Result<BrowserPdkHead, ConfigError> {
    let head: BrowserPdkHead = serde_json::from_slice(head_bytes)
        .map_err(|error| ConfigError::Parse(error.to_string()))?;
    if head.schema_version != BROWSER_PDK_SCHEMA_VERSION {
        return Err(ConfigError::Parse(format!(
            "browser PDK schema {} is unsupported; expected {}",
            head.schema_version, BROWSER_PDK_SCHEMA_VERSION
        )));
    }
    if head.generation == 0 || head.generation > MAX_BROWSER_PDK_GENERATION {
        return Err(ConfigError::Parse(format!(
            "browser PDK generation {} is outside the exact integer range",
            head.generation
        )));
    }
    if head.path_digest != browser_path_digest(path) {
        return Err(ConfigError::Parse(
            "browser PDK head belongs to a different configuration identity".to_owned(),
        ));
    }
    if head.metadata.byte_len > MAX_BROWSER_PDK_METADATA_BYTES as u64 {
        return Err(ConfigError::Parse(format!(
            "browser PDK metadata declares {} bytes, exceeding the {}-byte limit",
            head.metadata.byte_len, MAX_BROWSER_PDK_METADATA_BYTES
        )));
    }
    if head.archives.len() > technology_package::MAX_PDK_ARTIFACTS {
        return Err(ConfigError::Parse(format!(
            "browser PDK head declares {} archives, exceeding the {}-archive limit",
            head.archives.len(),
            technology_package::MAX_PDK_ARTIFACTS
        )));
    }
    let mut archive_digests = std::collections::HashSet::with_capacity(head.archives.len());
    for archive in &head.archives {
        if archive.byte_len > MAX_PDK_ARCHIVE_BYTES as u64 {
            return Err(ConfigError::Parse(format!(
                "browser PDK archive {} declares {} bytes, exceeding the {}-byte limit",
                archive.digest, archive.byte_len, MAX_PDK_ARCHIVE_BYTES
            )));
        }
        if !archive_digests.insert(archive.digest) {
            return Err(ConfigError::Parse(format!(
                "browser PDK archive digest {} appears more than once",
                archive.digest
            )));
        }
    }
    Ok(head)
}

#[cfg(any(test, target_arch = "wasm32"))]
fn validate_browser_blob(
    reference: &BrowserPdkBlobRef,
    bytes: &[u8],
    label: &str,
) -> Result<(), ConfigError> {
    let observed_len = u64::try_from(bytes.len())
        .map_err(|_| ConfigError::Parse(format!("{label} length does not fit in 64 bits")))?;
    if observed_len != reference.byte_len {
        return Err(ConfigError::Parse(format!(
            "{label} length is {observed_len}, expected {}",
            reference.byte_len
        )));
    }
    let observed_digest = pdk_content_digest(bytes);
    if observed_digest != reference.digest {
        return Err(ConfigError::Parse(format!(
            "{label} digest is {observed_digest}, expected {}",
            reference.digest
        )));
    }
    Ok(())
}

#[cfg(any(test, target_arch = "wasm32"))]
fn restore_browser_pdk_snapshot(
    path: &Path,
    head_bytes: &[u8],
    metadata_bytes: &[u8],
    archive_bytes: Vec<Vec<u8>>,
) -> Result<(PdkConfig, BrowserPdkConfigReceipt), ConfigError> {
    let head = validate_browser_pdk_head(path, head_bytes)?;
    validate_browser_blob(&head.metadata, metadata_bytes, "PDK configuration metadata")?;
    if archive_bytes.len() != head.archives.len() {
        return Err(ConfigError::Parse(format!(
            "browser PDK snapshot supplied {} archive blobs, expected {}",
            archive_bytes.len(),
            head.archives.len()
        )));
    }

    let mut config: PdkConfig = serde_json::from_slice(metadata_bytes)
        .map_err(|error| ConfigError::Parse(error.to_string()))?;
    let mut archives = Vec::with_capacity(archive_bytes.len());
    for (index, (reference, bytes)) in head
        .archives
        .iter()
        .zip(archive_bytes.into_iter())
        .enumerate()
    {
        validate_browser_blob(
            reference,
            &bytes,
            &format!("signed PDK archive {}", index + 1),
        )?;
        archives.push(
            serde_json::from_slice(&bytes)
                .map_err(|error| ConfigError::Parse(error.to_string()))?,
        );
    }
    config
        .technology_registry
        .restore_archives_from_browser_persistence(archives)
        .map_err(|error| ConfigError::Parse(error.to_string()))?;

    Ok((
        config,
        BrowserPdkConfigReceipt {
            head_key: browser_head_key(head.path_digest),
            generation: head.generation,
            head_digest: pdk_content_digest(head_bytes),
            storage_status: BrowserPdkStorageStatus::default(),
        },
    ))
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;

    #[test]
    fn browser_snapshot_separates_signed_archives_and_round_trips_exact_state() {
        let path = PathBuf::from("/rspice/browser/pdk_config.json");
        let (archive_bytes, trust, authority) = super::signed_technology_test_fixture();
        let mut config = PdkConfig::new();
        config.publisher_trust_store = trust;
        config
            .technology_registry
            .install_archive_bytes(
                &archive_bytes,
                &config.publisher_trust_store,
                &authority,
                "install browser persistence fixture",
            )
            .expect("install package");

        let prepared =
            prepare_browser_pdk_snapshot(&path, &config, 7).expect("prepare browser snapshot");
        let metadata: PdkConfig =
            serde_json::from_slice(&prepared.metadata.bytes).expect("decode metadata");
        assert!(
            metadata.technology_registry.archives().is_empty(),
            "large immutable archives must not remain embedded in metadata"
        );
        assert_eq!(prepared.archives.len(), 1);
        assert_eq!(
            prepared.archives[0].reference.digest,
            pdk_content_digest(&prepared.archives[0].bytes)
        );

        let archive_blobs = prepared
            .archives
            .iter()
            .map(|archive| archive.bytes.clone())
            .collect();
        let (mut restored, receipt) = restore_browser_pdk_snapshot(
            &path,
            &prepared.head_bytes,
            &prepared.metadata.bytes,
            archive_blobs,
        )
        .expect("restore browser snapshot");
        assert_eq!(receipt.generation, 7);
        assert_eq!(
            serde_json::to_vec(&restored).unwrap(),
            serde_json::to_vec(&config).unwrap()
        );
        assert!(restored.technology_registry.validated_packages().is_empty());
        let trust = restored.publisher_trust_store.clone();
        restored
            .technology_registry
            .revalidate_installed(&trust)
            .expect("runtime trust is re-established explicitly");
        assert_eq!(restored.technology_registry.validated_packages().len(), 1);
    }

    #[test]
    fn browser_snapshot_rejects_tampered_archive_readback() {
        let path = PathBuf::from("/rspice/browser/tamper.json");
        let (archive_bytes, trust, authority) = super::signed_technology_test_fixture();
        let mut config = PdkConfig::new();
        config.publisher_trust_store = trust;
        config
            .technology_registry
            .install_archive_bytes(
                &archive_bytes,
                &config.publisher_trust_store,
                &authority,
                "install tamper fixture",
            )
            .expect("install package");
        let prepared =
            prepare_browser_pdk_snapshot(&path, &config, 1).expect("prepare browser snapshot");
        let mut archives = prepared
            .archives
            .iter()
            .map(|archive| archive.bytes.clone())
            .collect::<Vec<_>>();
        archives[0][0] ^= 0x01;

        let error = restore_browser_pdk_snapshot(
            &path,
            &prepared.head_bytes,
            &prepared.metadata.bytes,
            archives,
        )
        .expect_err("tampered archive must fail closed");

        assert!(error.to_string().contains("digest"));
    }

    #[test]
    fn browser_snapshot_head_is_bound_to_path_and_exact_generation_range() {
        let path = PathBuf::from("/rspice/browser/identity.json");
        let config = PdkConfig::new();
        let prepared =
            prepare_browser_pdk_snapshot(&path, &config, 1).expect("prepare browser snapshot");

        assert!(
            restore_browser_pdk_snapshot(
                Path::new("/rspice/browser/different.json"),
                &prepared.head_bytes,
                &prepared.metadata.bytes,
                Vec::new(),
            )
            .is_err()
        );
        assert!(prepare_browser_pdk_snapshot(&path, &config, 0).is_err());
        assert!(
            prepare_browser_pdk_snapshot(
                &path,
                &config,
                MAX_BROWSER_PDK_GENERATION.saturating_add(1),
            )
            .is_err()
        );
    }

    #[test]
    fn browser_metadata_identity_is_independent_of_hash_map_insertion_order() {
        let path = PathBuf::from("/rspice/browser/canonical.json");
        let mut first = PdkConfig::new();
        first
            .environment_variables
            .insert("PDK_ROOT".to_owned(), "/pdk".to_owned());
        first
            .environment_variables
            .insert("MODEL_ROOT".to_owned(), "/models".to_owned());
        let mut second = PdkConfig::new();
        second
            .environment_variables
            .insert("MODEL_ROOT".to_owned(), "/models".to_owned());
        second
            .environment_variables
            .insert("PDK_ROOT".to_owned(), "/pdk".to_owned());

        let first = prepare_browser_pdk_snapshot(&path, &first, 1).unwrap();
        let second = prepare_browser_pdk_snapshot(&path, &second, 1).unwrap();

        assert_eq!(first.metadata.bytes, second.metadata.bytes);
        assert_eq!(first.metadata.reference, second.metadata.reference);
        assert_eq!(first.head_bytes, second.head_bytes);
    }

    #[test]
    fn browser_storage_admission_preserves_quota_reserve() {
        const MIB: u64 = 1024 * 1024;
        let roomy = BrowserPdkStorageStatus {
            durability: BrowserPdkStorageDurability::Persistent,
            usage_bytes: Some(100 * MIB),
            quota_bytes: Some(512 * MIB),
        };
        enforce_browser_pdk_storage_admission(roomy, 300 * MIB)
            .expect("publication fits while preserving the quota reserve");

        let constrained = BrowserPdkStorageStatus {
            durability: BrowserPdkStorageDurability::BestEffort,
            usage_bytes: Some(116 * MIB),
            quota_bytes: Some(128 * MIB),
        };
        let error = enforce_browser_pdk_storage_admission(constrained, 5 * MIB)
            .expect_err("publication must not consume the safety reserve");
        assert!(error.to_string().contains("safety reserve"));
        assert_eq!(constrained.available_bytes(), Some(12 * MIB));
    }

    #[test]
    fn browser_storage_admission_degrades_safely_when_estimate_is_unavailable() {
        let unknown = BrowserPdkStorageStatus {
            durability: BrowserPdkStorageDurability::Unknown,
            usage_bytes: None,
            quota_bytes: None,
        };
        enforce_browser_pdk_storage_admission(unknown, u64::MAX)
            .expect("IndexedDB remains authoritative when StorageManager is unavailable");
        assert_eq!(unknown.available_bytes(), None);
    }

    #[test]
    fn browser_storage_footprint_accounts_for_keys_values_and_record_overhead() {
        let footprint =
            browser_pdk_record_footprint("archive:abc", &[0_u8; 128]).expect("footprint");
        assert_eq!(
            footprint,
            "archive:abc".len() as u64 + 128 + BROWSER_PDK_RECORD_OVERHEAD_BYTES
        );
    }

    #[test]
    fn save_round_trips_through_durable_publication() {
        let root = unique_temp_dir("round-trip");
        let path = root.join(CONFIG_FILE_NAME);
        let mut config = PdkConfig::new();
        config
            .environment_variables
            .insert("PDK_ROOT".to_string(), "/models".to_string());

        config.save_to(&path).expect("save config");
        let loaded = PdkConfig::load_from(&path).expect("load config");

        assert_eq!(loaded, config);
        std::fs::remove_dir_all(root).expect("remove fixture");
    }

    #[test]
    fn publication_rejects_late_external_change() {
        let root = unique_temp_dir("late-change");
        let path = root.join(CONFIG_FILE_NAME);
        std::fs::write(&path, b"authorized predecessor").expect("write predecessor");
        let expected =
            crate::io::durable_file::observe_expected_content(&path).expect("observe destination");
        let content = serde_json::to_vec_pretty(&PdkConfig::new()).expect("serialize config");
        std::fs::write(&path, b"late external edit").expect("race destination");

        let result = publish_pdk_config(&path, expected, &content);

        assert!(result.is_err());
        assert_eq!(std::fs::read(&path).unwrap(), b"late external edit");
        std::fs::remove_dir_all(root).expect("remove fixture");
    }

    #[test]
    fn display_profile_revisions_round_trip_and_rebind_only_after_package_revalidation() {
        let root = unique_temp_dir("display-profile");
        let path = root.join(CONFIG_FILE_NAME);
        let (bytes, trust, authority) = super::signed_technology_test_fixture();
        let mut config = PdkConfig::new();
        config.publisher_trust_store = trust;
        config
            .technology_registry
            .install_archive_bytes(
                &bytes,
                &config.publisher_trust_store,
                &authority,
                "install persistence fixture",
            )
            .expect("install package");
        let package = config.technology_registry.validated_packages()[0].clone();
        config
            .display_profile_registry
            .publish_and_activate(
                &package,
                PdkDisplayProfileDraft::signed_defaults(&package, "layout-dark", "Layout dark"),
                &authority,
                "publish persistent profile",
            )
            .expect("publish display profile");

        config.save_to(&path).expect("save config");
        let mut loaded = PdkConfig::load_from(&path).expect("load config");

        assert_eq!(
            loaded.display_profile_registry,
            config.display_profile_registry
        );
        loaded
            .display_profile_registry
            .validate_audit_chain()
            .expect("display audit");
        assert!(loaded.technology_registry.validated_packages().is_empty());
        let trust = loaded.publisher_trust_store.clone();
        loaded
            .technology_registry
            .revalidate_installed(&trust)
            .expect("revalidate package");
        let rebound = &loaded.technology_registry.validated_packages()[0];
        assert!(
            loaded
                .display_profile_registry
                .active_for_package(rebound)
                .is_some()
        );
        std::fs::remove_dir_all(root).expect("remove fixture");
    }

    fn unique_temp_dir(label: &str) -> PathBuf {
        let root = std::env::temp_dir().join(format!(
            "rspice-pdk-config-{label}-{}",
            uuid::Uuid::new_v4()
        ));
        std::fs::create_dir_all(&root).expect("create fixture");
        root
    }
}
