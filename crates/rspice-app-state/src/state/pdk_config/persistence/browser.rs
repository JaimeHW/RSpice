//! Asynchronous browser persistence for PDK configuration.
//!
//! Large signed archives never pass through synchronous Web Storage. A small
//! generation head and content-addressed metadata/archive blobs are committed
//! through IndexedDB, with exact readback before success is reported.

use std::path::{Path, PathBuf};

use super::*;
use crate::browser_store::{self, PDK_CONFIG};

pub fn start_browser_pdk_config_load(
    path: PathBuf,
    complete: impl FnOnce(Result<BrowserPdkConfigRestore, ConfigError>) + 'static,
) {
    wasm_bindgen_futures::spawn_local(async move {
        complete(load_browser_pdk_config(&path).await);
    });
}

pub fn start_browser_pdk_config_save(
    path: PathBuf,
    expected: Option<BrowserPdkConfigReceipt>,
    config: PdkConfig,
    complete: impl FnOnce(Result<BrowserPdkConfigReceipt, ConfigError>) + 'static,
) -> Result<(), ConfigError> {
    let generation = expected.as_ref().map_or(Ok(1), |receipt| {
        receipt.generation.checked_add(1).ok_or_else(|| {
            ConfigError::Serialize("browser PDK generation space is exhausted".to_owned())
        })
    })?;
    let prepared = prepare_browser_pdk_snapshot(&path, &config, generation)?;
    wasm_bindgen_futures::spawn_local(async move {
        complete(
            save_prepared_browser_pdk_snapshot(&path, expected.as_ref(), &config, prepared).await,
        );
    });
    Ok(())
}

async fn load_browser_pdk_config(path: &Path) -> Result<BrowserPdkConfigRestore, ConfigError> {
    let storage_status = browser_storage_status(false).await;
    let head_key = browser_head_key(browser_path_digest(path));
    if let Some(head_value) = indexed_db_get(&head_key).await? {
        let head_text = head_value.as_string().ok_or_else(|| {
            ConfigError::Parse("browser PDK head is not a JSON string".to_owned())
        })?;
        let (config, mut receipt) = load_snapshot_from_head(path, head_text.as_bytes()).await?;
        receipt.storage_status = storage_status;
        return Ok(BrowserPdkConfigRestore {
            config,
            receipt: Some(receipt),
            migrated_legacy_record: false,
            storage_status,
        });
    }

    let Some(legacy_text) = legacy_browser_record(path)? else {
        return Ok(BrowserPdkConfigRestore {
            config: PdkConfig::default(),
            receipt: None,
            migrated_legacy_record: false,
            storage_status,
        });
    };
    let legacy_config: PdkConfig = serde_json::from_str(&legacy_text)
        .map_err(|error| ConfigError::Parse(error.to_string()))?;
    let prepared = prepare_browser_pdk_snapshot(path, &legacy_config, 1)?;
    let receipt = save_prepared_browser_pdk_snapshot(path, None, &legacy_config, prepared).await?;
    if let Err(error) = remove_legacy_browser_record(path) {
        log::warn!(
            "browser PDK configuration migrated, but the legacy Web Storage record could not be removed: {error}"
        );
    }
    Ok(BrowserPdkConfigRestore {
        config: legacy_config,
        storage_status: receipt.storage_status,
        receipt: Some(receipt),
        migrated_legacy_record: true,
    })
}

async fn save_prepared_browser_pdk_snapshot(
    path: &Path,
    expected: Option<&BrowserPdkConfigReceipt>,
    config: &PdkConfig,
    prepared: PreparedBrowserPdkSnapshot,
) -> Result<BrowserPdkConfigReceipt, ConfigError> {
    let storage_status = browser_storage_status(true).await;
    let required_new_bytes = required_new_storage_bytes(&prepared).await?;
    enforce_browser_pdk_storage_admission(storage_status, required_new_bytes)?;

    ensure_indexed_db_blob(
        &browser_metadata_key(
            prepared.head.path_digest,
            prepared.metadata.reference.digest,
        ),
        &prepared.metadata.bytes,
        "PDK configuration metadata",
    )
    .await?;
    for archive in &prepared.archives {
        ensure_indexed_db_blob(
            &browser_archive_key(archive.reference.digest),
            &archive.bytes,
            "signed PDK archive",
        )
        .await?;
    }

    let previous = compare_exchange_head(path, expected, &prepared).await?;
    let observed_head = indexed_db_get(&prepared.head_key)
        .await?
        .and_then(|value| value.as_string())
        .ok_or_else(|| ConfigError::Io("IndexedDB did not retain the PDK head".to_owned()))?;
    if observed_head.as_bytes() != prepared.head_bytes {
        return Err(ConfigError::Io(
            "IndexedDB PDK head readback did not match the committed generation".to_owned(),
        ));
    }

    let (observed_config, mut receipt) =
        load_snapshot_from_head(path, observed_head.as_bytes()).await?;
    receipt.storage_status = storage_status;
    let expected_json = canonical_json_bytes(config, "published PDK configuration")?;
    let observed_json = canonical_json_bytes(&observed_config, "restored PDK configuration")?;
    if observed_json != expected_json {
        return Err(ConfigError::Io(
            "IndexedDB PDK snapshot readback did not reproduce the published configuration"
                .to_owned(),
        ));
    }

    if let Some(previous) = previous
        && previous.metadata.digest != prepared.metadata.reference.digest
    {
        let obsolete_key = browser_metadata_key(previous.path_digest, previous.metadata.digest);
        if let Err(error) = indexed_db_delete(&obsolete_key).await {
            log::warn!(
                "browser PDK configuration committed, but obsolete metadata cleanup failed: {error}"
            );
        }
    }
    Ok(receipt)
}

async fn required_new_storage_bytes(
    prepared: &PreparedBrowserPdkSnapshot,
) -> Result<u64, ConfigError> {
    let mut required = browser_pdk_record_footprint(&prepared.head_key, &prepared.head_bytes)?;
    let metadata_key = browser_metadata_key(
        prepared.head.path_digest,
        prepared.metadata.reference.digest,
    );
    required = required
        .checked_add(
            required_blob_bytes(
                &metadata_key,
                &prepared.metadata.bytes,
                "PDK configuration metadata",
            )
            .await?,
        )
        .ok_or_else(|| {
            ConfigError::Serialize("browser PDK storage estimate overflowed".to_owned())
        })?;
    for archive in &prepared.archives {
        let key = browser_archive_key(archive.reference.digest);
        required = required
            .checked_add(required_blob_bytes(&key, &archive.bytes, "signed PDK archive").await?)
            .ok_or_else(|| {
                ConfigError::Serialize("browser PDK storage estimate overflowed".to_owned())
            })?;
    }
    Ok(required)
}

async fn required_blob_bytes(key: &str, bytes: &[u8], label: &str) -> Result<u64, ConfigError> {
    let Some(existing) = indexed_db_get(key).await? else {
        return browser_pdk_record_footprint(key, bytes);
    };
    if js_bytes(&existing, label)? != bytes {
        return Err(ConfigError::Io(format!(
            "content-addressed browser {label} key contains different bytes"
        )));
    }
    Ok(0)
}

async fn load_snapshot_from_head(
    path: &Path,
    head_bytes: &[u8],
) -> Result<(PdkConfig, BrowserPdkConfigReceipt), ConfigError> {
    let head = validate_browser_pdk_head(path, head_bytes)?;
    let metadata_key = browser_metadata_key(head.path_digest, head.metadata.digest);
    let metadata = indexed_db_get_bytes(&metadata_key, "PDK configuration metadata").await?;
    let mut archives = Vec::with_capacity(head.archives.len());
    for archive in &head.archives {
        archives.push(
            indexed_db_get_bytes(&browser_archive_key(archive.digest), "signed PDK archive")
                .await?,
        );
    }
    restore_browser_pdk_snapshot(path, head_bytes, &metadata, archives)
}

async fn compare_exchange_head(
    path: &Path,
    expected: Option<&BrowserPdkConfigReceipt>,
    prepared: &PreparedBrowserPdkSnapshot,
) -> Result<Option<BrowserPdkHead>, ConfigError> {
    if let Some(expected) = expected
        && expected.head_key != prepared.head_key
    {
        return Err(ConfigError::Io(
            "browser PDK persistence receipt belongs to a different configuration identity"
                .to_owned(),
        ));
    }

    let database = open_database().await?;
    let transaction = database.write().map_err(ConfigError::Io)?;
    let existing = transaction
        .get(&prepared.head_key)
        .await
        .map_err(ConfigError::Io)?;
    let previous = match &existing {
        None => None,
        Some(existing) => {
            let text = existing.as_string().ok_or_else(|| {
                ConfigError::Parse("browser PDK predecessor head is not a JSON string".to_owned())
            })?;
            Some(validate_browser_pdk_head(path, text.as_bytes())?)
        }
    };

    match (expected, &previous) {
        (None, None) => {}
        (None, Some(_)) => {
            transaction.abort();
            return Err(ConfigError::Io(
                "browser PDK configuration changed in another tab before first publication"
                    .to_owned(),
            ));
        }
        (Some(_), None) => {
            transaction.abort();
            return Err(ConfigError::Io(
                "browser PDK configuration was removed in another tab".to_owned(),
            ));
        }
        (Some(expected), Some(_)) => {
            let text = existing
                .as_ref()
                .and_then(|existing| existing.as_string())
                .expect("validated string predecessor");
            let observed_digest = pdk_content_digest(text.as_bytes());
            let observed_generation = previous.as_ref().expect("validated predecessor").generation;
            if observed_digest != expected.head_digest || observed_generation != expected.generation
            {
                transaction.abort();
                return Err(ConfigError::Io(
                    "browser PDK configuration changed in another tab; reload before retrying"
                        .to_owned(),
                ));
            }
        }
    }

    let head_text = std::str::from_utf8(&prepared.head_bytes)
        .map_err(|error| ConfigError::Serialize(error.to_string()))?;
    transaction
        .put(
            &prepared.head_key,
            &wasm_bindgen::JsValue::from_str(head_text),
        )
        .await
        .map_err(ConfigError::Io)?;
    transaction.commit().await.map_err(ConfigError::Io)?;
    Ok(previous)
}

async fn ensure_indexed_db_blob(key: &str, bytes: &[u8], label: &str) -> Result<(), ConfigError> {
    if let Some(existing) = indexed_db_get(key).await? {
        let existing = js_bytes(&existing, label)?;
        if existing == bytes {
            return Ok(());
        }
        return Err(ConfigError::Io(format!(
            "content-addressed browser {label} key contains different bytes"
        )));
    }

    let database = open_database().await?;
    let transaction = database.write().map_err(ConfigError::Io)?;
    let add_result = transaction
        .add(key, &js_sys::Uint8Array::from(bytes).into())
        .await;
    if let Err(error) = add_result {
        transaction.abort();
        if let Some(raced) = indexed_db_get(key).await? {
            let raced = js_bytes(&raced, label)?;
            if raced == bytes {
                return Ok(());
            }
        }
        return Err(ConfigError::Io(format!(
            "could not reserve content-addressed browser {label}: {error}"
        )));
    }
    transaction.commit().await.map_err(ConfigError::Io)?;

    let observed = indexed_db_get_bytes(key, label).await?;
    if observed != bytes {
        return Err(ConfigError::Io(format!(
            "IndexedDB {label} readback did not match staged bytes"
        )));
    }
    Ok(())
}

async fn indexed_db_get_bytes(key: &str, label: &str) -> Result<Vec<u8>, ConfigError> {
    let value = indexed_db_get(key)
        .await?
        .ok_or_else(|| ConfigError::Io(format!("browser {label} is missing")))?;
    js_bytes(&value, label)
}

fn js_bytes(value: &wasm_bindgen::JsValue, label: &str) -> Result<Vec<u8>, ConfigError> {
    browser_store::bytes(value)
        .ok_or_else(|| ConfigError::Parse(format!("browser {label} is not a byte array")))
}

fn legacy_storage_key(path: &Path) -> String {
    format!("rspice:pdk-config:v1:{}", path.to_string_lossy())
}

fn browser_local_storage() -> Result<web_sys::Storage, ConfigError> {
    web_sys::window()
        .ok_or_else(|| ConfigError::Io("browser window is unavailable".to_owned()))?
        .local_storage()
        .map_err(|error| ConfigError::Io(browser_store::js_error(error)))?
        .ok_or_else(|| ConfigError::Io("browser local storage is unavailable".to_owned()))
}

fn legacy_browser_record(path: &Path) -> Result<Option<String>, ConfigError> {
    let storage = match browser_local_storage() {
        Ok(storage) => storage,
        Err(error) => {
            log::warn!(
                "legacy browser PDK migration was skipped because Web Storage is unavailable: {error}"
            );
            return Ok(None);
        }
    };
    match storage.get_item(&legacy_storage_key(path)) {
        Ok(record) => Ok(record),
        Err(error) => {
            log::warn!(
                "legacy browser PDK migration was skipped because Web Storage read was rejected: {}",
                browser_store::js_error(error)
            );
            Ok(None)
        }
    }
}

fn remove_legacy_browser_record(path: &Path) -> Result<(), ConfigError> {
    browser_local_storage()?
        .remove_item(&legacy_storage_key(path))
        .map_err(|error| ConfigError::Io(browser_store::js_error(error)))
}

async fn browser_storage_status(request_persistence: bool) -> BrowserPdkStorageStatus {
    let Some(storage) = browser_store::storage_manager() else {
        return BrowserPdkStorageStatus::default();
    };

    let mut persisted = optional_storage_boolean(&storage, "persisted").await;
    if request_persistence && persisted != Some(true) {
        persisted = optional_storage_boolean(&storage, "persist")
            .await
            .or(persisted);
    }
    let durability = match persisted {
        Some(true) => BrowserPdkStorageDurability::Persistent,
        Some(false) => BrowserPdkStorageDurability::BestEffort,
        None => BrowserPdkStorageDurability::Unknown,
    };

    let estimate = optional_storage_call(&storage, "estimate").await;
    BrowserPdkStorageStatus {
        durability,
        usage_bytes: estimate
            .as_ref()
            .and_then(|estimate| safe_integer_property(estimate, "usage")),
        quota_bytes: estimate
            .as_ref()
            .and_then(|estimate| safe_integer_property(estimate, "quota")),
    }
}

async fn optional_storage_boolean(storage: &wasm_bindgen::JsValue, method: &str) -> Option<bool> {
    optional_storage_call(storage, method)
        .await
        .and_then(|value| value.as_bool())
}

async fn optional_storage_call(
    storage: &wasm_bindgen::JsValue,
    method: &str,
) -> Option<wasm_bindgen::JsValue> {
    browser_store::storage_call(storage, method, BROWSER_PDK_STORAGE_MANAGER_TIMEOUT_MS)
        .await
        .unwrap_or_else(|error| {
            log::warn!("browser StorageManager.{method}() {error}");
            None
        })
}

fn safe_integer_property(value: &wasm_bindgen::JsValue, property: &str) -> Option<u64> {
    let number = js_sys::Reflect::get(value, &wasm_bindgen::JsValue::from_str(property))
        .ok()?
        .as_f64()?;
    (number.is_finite()
        && number >= 0.0
        && number.fract() == 0.0
        && number <= MAX_BROWSER_PDK_GENERATION as f64)
        .then_some(number as u64)
}

async fn indexed_db_get(key: &str) -> Result<Option<wasm_bindgen::JsValue>, ConfigError> {
    let database = open_database().await?;
    database
        .read()
        .map_err(ConfigError::Io)?
        .get(key)
        .await
        .map_err(ConfigError::Io)
}

async fn indexed_db_delete(key: &str) -> Result<(), ConfigError> {
    let database = open_database().await?;
    let transaction = database.write().map_err(ConfigError::Io)?;
    transaction.delete(key).await.map_err(ConfigError::Io)?;
    transaction.commit().await.map_err(ConfigError::Io)
}

async fn open_database() -> Result<browser_store::Database, ConfigError> {
    browser_store::open(&PDK_CONFIG)
        .await
        .map_err(ConfigError::Io)
}
