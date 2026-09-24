//! Asynchronous browser persistence for PDK configuration.
//!
//! Large signed archives never pass through synchronous Web Storage. A small
//! generation head and content-addressed metadata/archive blobs are committed
//! through IndexedDB, with exact readback before success is reported.

use std::path::{Path, PathBuf};

use super::*;

thread_local! {
    static RETAINED_EVENT_HANDLERS: std::cell::RefCell<Vec<RetainedEventHandlers>> =
        const { std::cell::RefCell::new(Vec::new()) };
}

struct RetainedEventHandlers {
    done: std::rc::Rc<std::cell::Cell<bool>>,
    _callbacks: Vec<wasm_bindgen::closure::Closure<dyn FnMut()>>,
}

struct BrowserDatabase {
    value: wasm_bindgen::JsValue,
    _on_version_change: wasm_bindgen::closure::Closure<dyn FnMut()>,
}

impl Drop for BrowserDatabase {
    fn drop(&mut self) {
        let _ = call_value_method(&self.value, "close", &[]);
    }
}

pub(crate) fn start_browser_pdk_config_load(
    path: PathBuf,
    complete: impl FnOnce(Result<BrowserPdkConfigRestore, ConfigError>) + 'static,
) {
    wasm_bindgen_futures::spawn_local(async move {
        complete(load_browser_pdk_config(&path).await);
    });
}

pub(crate) fn start_browser_pdk_config_save(
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
    let transaction = strict_readwrite_transaction(&database.value)?;
    let completion = await_transaction(&transaction);
    let store = transaction_store(&transaction)?;
    let get = call_value_method(
        &store,
        "get",
        &[wasm_bindgen::JsValue::from_str(&prepared.head_key)],
    )
    .map_err(ConfigError::Io)?;
    let existing = await_request(&get).await.map_err(ConfigError::Io)?;
    let previous = if existing.is_undefined() {
        None
    } else {
        let text = existing.as_string().ok_or_else(|| {
            ConfigError::Parse("browser PDK predecessor head is not a JSON string".to_owned())
        })?;
        Some(validate_browser_pdk_head(path, text.as_bytes())?)
    };

    match (expected, &previous) {
        (None, None) => {}
        (None, Some(_)) => {
            abort_transaction(&transaction);
            return Err(ConfigError::Io(
                "browser PDK configuration changed in another tab before first publication"
                    .to_owned(),
            ));
        }
        (Some(_), None) => {
            abort_transaction(&transaction);
            return Err(ConfigError::Io(
                "browser PDK configuration was removed in another tab".to_owned(),
            ));
        }
        (Some(expected), Some(_)) => {
            let text = existing.as_string().expect("validated string predecessor");
            let observed_digest = pdk_content_digest(text.as_bytes());
            let observed_generation = previous.as_ref().expect("validated predecessor").generation;
            if observed_digest != expected.head_digest || observed_generation != expected.generation
            {
                abort_transaction(&transaction);
                return Err(ConfigError::Io(
                    "browser PDK configuration changed in another tab; reload before retrying"
                        .to_owned(),
                ));
            }
        }
    }

    let head_text = std::str::from_utf8(&prepared.head_bytes)
        .map_err(|error| ConfigError::Serialize(error.to_string()))?;
    let put = call_value_method(
        &store,
        "put",
        &[
            wasm_bindgen::JsValue::from_str(head_text),
            wasm_bindgen::JsValue::from_str(&prepared.head_key),
        ],
    )
    .map_err(ConfigError::Io)?;
    let _ = await_request(&put).await.map_err(ConfigError::Io)?;
    completion.await.map_err(ConfigError::Io)?;
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
    let transaction = strict_readwrite_transaction(&database.value)?;
    let completion = await_transaction(&transaction);
    let store = transaction_store(&transaction)?;
    let add = call_value_method(
        &store,
        "add",
        &[
            js_sys::Uint8Array::from(bytes).into(),
            wasm_bindgen::JsValue::from_str(key),
        ],
    )
    .map_err(ConfigError::Io)?;
    let add_result = await_request(&add).await;
    if let Err(error) = add_result {
        abort_transaction(&transaction);
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
    completion.await.map_err(ConfigError::Io)?;

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
    use wasm_bindgen::JsCast as _;
    if !value.is_instance_of::<js_sys::Uint8Array>() {
        return Err(ConfigError::Parse(format!(
            "browser {label} is not a byte array"
        )));
    }
    Ok(js_sys::Uint8Array::new(value).to_vec())
}

fn legacy_storage_key(path: &Path) -> String {
    format!("rspice:pdk-config:v1:{}", path.to_string_lossy())
}

fn browser_local_storage() -> Result<web_sys::Storage, ConfigError> {
    web_sys::window()
        .ok_or_else(|| ConfigError::Io("browser window is unavailable".to_owned()))?
        .local_storage()
        .map_err(|error| ConfigError::Io(js_error(error)))?
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
                js_error(error)
            );
            Ok(None)
        }
    }
}

fn remove_legacy_browser_record(path: &Path) -> Result<(), ConfigError> {
    browser_local_storage()?
        .remove_item(&legacy_storage_key(path))
        .map_err(|error| ConfigError::Io(js_error(error)))
}

async fn browser_storage_status(request_persistence: bool) -> BrowserPdkStorageStatus {
    let Some(storage) = browser_storage_manager() else {
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

fn browser_storage_manager() -> Option<wasm_bindgen::JsValue> {
    let window = web_sys::window()?;
    let navigator =
        js_sys::Reflect::get(&window, &wasm_bindgen::JsValue::from_str("navigator")).ok()?;
    let storage =
        js_sys::Reflect::get(&navigator, &wasm_bindgen::JsValue::from_str("storage")).ok()?;
    (!storage.is_null() && !storage.is_undefined()).then_some(storage)
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
    let callable = js_sys::Reflect::get(storage, &wasm_bindgen::JsValue::from_str(method)).ok()?;
    if callable.is_null() || callable.is_undefined() {
        return None;
    }
    let returned = match call_value_method(storage, method, &[]) {
        Ok(returned) => returned,
        Err(error) => {
            log::warn!("browser StorageManager.{method}() could not be called: {error}");
            return None;
        }
    };
    match await_browser_promise_with_timeout(
        js_sys::Promise::resolve(&returned),
        &format!("browser StorageManager.{method}()"),
        BROWSER_PDK_STORAGE_MANAGER_TIMEOUT_MS,
    )
    .await
    {
        Ok(value) => Some(value),
        Err(error) => {
            log::warn!("browser StorageManager.{method}() failed: {error}");
            None
        }
    }
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
    let transaction = call_value_method(
        &database.value,
        "transaction",
        &[
            wasm_bindgen::JsValue::from_str(BROWSER_PDK_STORE),
            wasm_bindgen::JsValue::from_str("readonly"),
        ],
    )
    .map_err(ConfigError::Io)?;
    let store = transaction_store(&transaction)?;
    let request = call_value_method(&store, "get", &[wasm_bindgen::JsValue::from_str(key)])
        .map_err(ConfigError::Io)?;
    let value = await_request(&request).await.map_err(ConfigError::Io)?;
    Ok((!value.is_undefined()).then_some(value))
}

async fn indexed_db_delete(key: &str) -> Result<(), ConfigError> {
    let database = open_database().await?;
    let transaction = strict_readwrite_transaction(&database.value)?;
    let completion = await_transaction(&transaction);
    let store = transaction_store(&transaction)?;
    let request = call_value_method(&store, "delete", &[wasm_bindgen::JsValue::from_str(key)])
        .map_err(ConfigError::Io)?;
    let _ = await_request(&request).await.map_err(ConfigError::Io)?;
    completion.await.map_err(ConfigError::Io)
}

async fn open_database() -> Result<BrowserDatabase, ConfigError> {
    let window =
        web_sys::window().ok_or_else(|| ConfigError::Io("browser window is unavailable".into()))?;
    let factory = js_sys::Reflect::get(&window, &wasm_bindgen::JsValue::from_str("indexedDB"))
        .map_err(|error| ConfigError::Io(js_error(error)))?;
    if factory.is_null() || factory.is_undefined() {
        return Err(ConfigError::Io("IndexedDB is unavailable".to_owned()));
    }
    let request = call_value_method(
        &factory,
        "open",
        &[
            wasm_bindgen::JsValue::from_str(BROWSER_PDK_DATABASE),
            wasm_bindgen::JsValue::from_f64(BROWSER_PDK_SCHEMA_VERSION as f64),
        ],
    )
    .map_err(ConfigError::Io)?;
    let upgrade_request = request.clone();
    let on_upgrade = wasm_bindgen::closure::Closure::<dyn FnMut()>::new(move || {
        let Ok(database) =
            js_sys::Reflect::get(&upgrade_request, &wasm_bindgen::JsValue::from_str("result"))
        else {
            return;
        };
        let exists = js_sys::Reflect::get(
            &database,
            &wasm_bindgen::JsValue::from_str("objectStoreNames"),
        )
        .ok()
        .and_then(|names| {
            call_value_method(
                &names,
                "contains",
                &[wasm_bindgen::JsValue::from_str(BROWSER_PDK_STORE)],
            )
            .ok()
        })
        .and_then(|value| value.as_bool())
        .unwrap_or(false);
        if !exists {
            let _ = call_value_method(
                &database,
                "createObjectStore",
                &[wasm_bindgen::JsValue::from_str(BROWSER_PDK_STORE)],
            );
        }
    });
    let database = await_open_request(&request, on_upgrade)
        .await
        .map_err(ConfigError::Io)?;
    let database_for_change = database.clone();
    let on_version_change = wasm_bindgen::closure::Closure::<dyn FnMut()>::new(move || {
        let _ = call_value_method(&database_for_change, "close", &[]);
    });
    js_sys::Reflect::set(
        &database,
        &wasm_bindgen::JsValue::from_str("onversionchange"),
        on_version_change.as_ref(),
    )
    .map_err(|error| ConfigError::Io(js_error(error)))?;
    Ok(BrowserDatabase {
        value: database,
        _on_version_change: on_version_change,
    })
}

fn transaction_store(
    transaction: &wasm_bindgen::JsValue,
) -> Result<wasm_bindgen::JsValue, ConfigError> {
    call_value_method(
        transaction,
        "objectStore",
        &[wasm_bindgen::JsValue::from_str(BROWSER_PDK_STORE)],
    )
    .map_err(ConfigError::Io)
}

fn strict_readwrite_transaction(
    database: &wasm_bindgen::JsValue,
) -> Result<wasm_bindgen::JsValue, ConfigError> {
    let options = js_sys::Object::new();
    js_sys::Reflect::set(
        &options,
        &wasm_bindgen::JsValue::from_str("durability"),
        &wasm_bindgen::JsValue::from_str("strict"),
    )
    .map_err(|error| ConfigError::Io(js_error(error)))?;
    call_value_method(
        database,
        "transaction",
        &[
            wasm_bindgen::JsValue::from_str(BROWSER_PDK_STORE),
            wasm_bindgen::JsValue::from_str("readwrite"),
            options.into(),
        ],
    )
    .or_else(|_| {
        call_value_method(
            database,
            "transaction",
            &[
                wasm_bindgen::JsValue::from_str(BROWSER_PDK_STORE),
                wasm_bindgen::JsValue::from_str("readwrite"),
            ],
        )
    })
    .map_err(ConfigError::Io)
}

fn abort_transaction(transaction: &wasm_bindgen::JsValue) {
    let _ = call_value_method(transaction, "abort", &[]);
}

async fn await_open_request(
    request: &wasm_bindgen::JsValue,
    on_upgrade: wasm_bindgen::closure::Closure<dyn FnMut()>,
) -> Result<wasm_bindgen::JsValue, String> {
    let success_request = request.clone();
    let error_request = request.clone();
    let request_for_handlers = request.clone();
    let mut on_upgrade = Some(on_upgrade);
    let reported = std::rc::Rc::new(std::cell::Cell::new(false));
    let reported_for_promise = reported.clone();
    let promise = js_sys::Promise::new(&mut move |resolve, reject| {
        let reported = reported_for_promise.clone();
        let retire = std::rc::Rc::new(std::cell::Cell::new(false));

        let success_reported = reported.clone();
        let success_retire = retire.clone();
        let success_resolve = resolve.clone();
        let success_request = success_request.clone();
        let success = wasm_bindgen::closure::Closure::<dyn FnMut()>::new(move || {
            let result =
                js_sys::Reflect::get(&success_request, &wasm_bindgen::JsValue::from_str("result"))
                    .unwrap_or(wasm_bindgen::JsValue::UNDEFINED);
            if success_reported.replace(true) {
                let _ = call_value_method(&result, "close", &[]);
            } else {
                let _ = success_resolve.call1(&wasm_bindgen::JsValue::UNDEFINED, &result);
            }
            success_retire.set(true);
        });

        let failure_reported = reported.clone();
        let failure_retire = retire.clone();
        let failure_reject = reject.clone();
        let error_request = error_request.clone();
        let failure = wasm_bindgen::closure::Closure::<dyn FnMut()>::new(move || {
            if !failure_reported.replace(true) {
                let error =
                    js_sys::Reflect::get(&error_request, &wasm_bindgen::JsValue::from_str("error"))
                        .unwrap_or_else(|_| {
                            wasm_bindgen::JsValue::from_str("IndexedDB open request failed")
                        });
                let _ = failure_reject.call1(&wasm_bindgen::JsValue::UNDEFINED, &error);
            }
            failure_retire.set(true);
        });

        let blocked_reported = reported.clone();
        let blocked_reject = reject.clone();
        let blocked = wasm_bindgen::closure::Closure::<dyn FnMut()>::new(move || {
            if !blocked_reported.replace(true) {
                let _ = blocked_reject.call1(
                    &wasm_bindgen::JsValue::UNDEFINED,
                    &wasm_bindgen::JsValue::from_str(
                        "IndexedDB schema upgrade is blocked by another RSpice tab",
                    ),
                );
            }
        });

        let upgrade = on_upgrade.take().expect("single IndexedDB open setup");
        let success_value = success.as_ref().clone();
        let failure_value = failure.as_ref().clone();
        let blocked_value = blocked.as_ref().clone();
        let upgrade_value = upgrade.as_ref().clone();
        if let Err(error) =
            retain_event_handlers(retire.clone(), vec![success, failure, blocked, upgrade])
        {
            retire.set(true);
            let _ = reject.call1(
                &wasm_bindgen::JsValue::UNDEFINED,
                &wasm_bindgen::JsValue::from_str(&error),
            );
            return;
        }
        for (property, value) in [
            ("onsuccess", success_value),
            ("onerror", failure_value),
            ("onblocked", blocked_value),
            ("onupgradeneeded", upgrade_value),
        ] {
            if let Err(error) = js_sys::Reflect::set(
                &request_for_handlers,
                &wasm_bindgen::JsValue::from_str(property),
                &value,
            ) {
                retire.set(true);
                let _ = reject.call1(&wasm_bindgen::JsValue::UNDEFINED, &error);
                return;
            }
        }
    });
    let result = await_browser_promise(promise, "IndexedDB open").await;
    if result.is_err() {
        reported.set(true);
    }
    result
}

async fn await_request(request: &wasm_bindgen::JsValue) -> Result<wasm_bindgen::JsValue, String> {
    let success_request = request.clone();
    let error_request = request.clone();
    let request_for_handlers = request.clone();
    let promise = js_sys::Promise::new(&mut move |resolve, reject| {
        let done = std::rc::Rc::new(std::cell::Cell::new(false));
        let success_done = done.clone();
        let success_resolve = resolve.clone();
        let success_request = success_request.clone();
        let success = wasm_bindgen::closure::Closure::<dyn FnMut()>::new(move || {
            if success_done.replace(true) {
                return;
            }
            let result =
                js_sys::Reflect::get(&success_request, &wasm_bindgen::JsValue::from_str("result"))
                    .unwrap_or(wasm_bindgen::JsValue::UNDEFINED);
            let _ = success_resolve.call1(&wasm_bindgen::JsValue::UNDEFINED, &result);
        });
        let failure_done = done.clone();
        let failure_reject = reject.clone();
        let error_request = error_request.clone();
        let failure = wasm_bindgen::closure::Closure::<dyn FnMut()>::new(move || {
            if failure_done.replace(true) {
                return;
            }
            let error =
                js_sys::Reflect::get(&error_request, &wasm_bindgen::JsValue::from_str("error"))
                    .unwrap_or_else(|_| {
                        wasm_bindgen::JsValue::from_str("IndexedDB request failed")
                    });
            let _ = failure_reject.call1(&wasm_bindgen::JsValue::UNDEFINED, &error);
        });
        let success_value = success.as_ref().clone();
        let failure_value = failure.as_ref().clone();
        if let Err(error) = retain_event_handlers(done.clone(), vec![success, failure]) {
            done.set(true);
            let _ = reject.call1(
                &wasm_bindgen::JsValue::UNDEFINED,
                &wasm_bindgen::JsValue::from_str(&error),
            );
            return;
        }
        for (property, value) in [("onsuccess", success_value), ("onerror", failure_value)] {
            if let Err(error) = js_sys::Reflect::set(
                &request_for_handlers,
                &wasm_bindgen::JsValue::from_str(property),
                &value,
            ) {
                done.set(true);
                let _ = reject.call1(&wasm_bindgen::JsValue::UNDEFINED, &error);
                return;
            }
        }
    });
    let result = await_browser_promise(promise, "IndexedDB request").await;
    if result.is_err()
        && let Ok(transaction) =
            js_sys::Reflect::get(request, &wasm_bindgen::JsValue::from_str("transaction"))
        && !transaction.is_null()
        && !transaction.is_undefined()
    {
        abort_transaction(&transaction);
    }
    result
}

fn await_transaction(
    transaction: &wasm_bindgen::JsValue,
) -> impl std::future::Future<Output = Result<(), String>> + 'static {
    let transaction_for_handlers = transaction.clone();
    let error_transaction = transaction.clone();
    let transaction_for_abort = transaction.clone();
    let promise = js_sys::Promise::new(&mut move |resolve, reject| {
        let error_transaction = error_transaction.clone();
        let done = std::rc::Rc::new(std::cell::Cell::new(false));
        let complete_done = done.clone();
        let complete_resolve = resolve.clone();
        let complete = wasm_bindgen::closure::Closure::<dyn FnMut()>::new(move || {
            if !complete_done.replace(true) {
                let _ = complete_resolve.call0(&wasm_bindgen::JsValue::UNDEFINED);
            }
        });
        let failure_done = done.clone();
        let failure_reject = reject.clone();
        let failure = wasm_bindgen::closure::Closure::<dyn FnMut()>::new(move || {
            if failure_done.replace(true) {
                return;
            }
            let error = js_sys::Reflect::get(
                &error_transaction,
                &wasm_bindgen::JsValue::from_str("error"),
            )
            .unwrap_or_else(|_| wasm_bindgen::JsValue::from_str("IndexedDB transaction failed"));
            let _ = failure_reject.call1(&wasm_bindgen::JsValue::UNDEFINED, &error);
        });
        let complete_value = complete.as_ref().clone();
        let failure_value = failure.as_ref().clone();
        if let Err(error) = retain_event_handlers(done.clone(), vec![complete, failure]) {
            done.set(true);
            let _ = reject.call1(
                &wasm_bindgen::JsValue::UNDEFINED,
                &wasm_bindgen::JsValue::from_str(&error),
            );
            return;
        }
        for (property, value) in [
            ("oncomplete", complete_value),
            ("onabort", failure_value.clone()),
            ("onerror", failure_value),
        ] {
            if let Err(error) = js_sys::Reflect::set(
                &transaction_for_handlers,
                &wasm_bindgen::JsValue::from_str(property),
                &value,
            ) {
                done.set(true);
                let _ = reject.call1(&wasm_bindgen::JsValue::UNDEFINED, &error);
                return;
            }
        }
    });
    async move {
        let result = await_browser_promise(promise, "IndexedDB transaction")
            .await
            .map(|_| ());
        if result.is_err() {
            abort_transaction(&transaction_for_abort);
        }
        result
    }
}

fn retain_event_handlers(
    done: std::rc::Rc<std::cell::Cell<bool>>,
    callbacks: Vec<wasm_bindgen::closure::Closure<dyn FnMut()>>,
) -> Result<(), String> {
    RETAINED_EVENT_HANDLERS.with(|retained| {
        let mut retained = retained.borrow_mut();
        retained.retain(|operation| !operation.done.get());
        if retained.len() >= MAX_RETAINED_BROWSER_PDK_EVENT_OPERATIONS {
            return Err("too many browser PDK storage operations are still pending".to_owned());
        }
        retained.push(RetainedEventHandlers {
            done,
            _callbacks: callbacks,
        });
        Ok(())
    })
}

async fn await_browser_promise(
    promise: js_sys::Promise,
    operation: &str,
) -> Result<wasm_bindgen::JsValue, String> {
    await_browser_promise_with_timeout(promise, operation, BROWSER_PDK_ASYNC_TIMEOUT_MS).await
}

async fn await_browser_promise_with_timeout(
    promise: js_sys::Promise,
    operation: &str,
    timeout_ms: u32,
) -> Result<wasm_bindgen::JsValue, String> {
    let race = js_sys::Array::new();
    race.push(&promise);
    race.push(&timeout_promise(operation, timeout_ms));
    wasm_bindgen_futures::JsFuture::from(js_sys::Promise::race(&race))
        .await
        .map_err(js_error)
}

fn timeout_promise(operation: &str, timeout_ms: u32) -> js_sys::Promise {
    let message = format!("{operation} timed out after {} seconds", timeout_ms / 1_000);
    js_sys::Promise::new(&mut move |_resolve, reject| {
        let message = message.clone();
        let timeout_reject = reject.clone();
        let callback = wasm_bindgen::closure::Closure::once_into_js(move || {
            let _ = timeout_reject.call1(
                &wasm_bindgen::JsValue::UNDEFINED,
                &wasm_bindgen::JsValue::from_str(&message),
            );
        });
        let Some(window) = web_sys::window() else {
            let _ = reject.call1(
                &wasm_bindgen::JsValue::UNDEFINED,
                &wasm_bindgen::JsValue::from_str("browser window is unavailable"),
            );
            return;
        };
        if let Err(error) = call_value_method(
            &window.into(),
            "setTimeout",
            &[callback, wasm_bindgen::JsValue::from_f64(timeout_ms as f64)],
        ) {
            let _ = reject.call1(
                &wasm_bindgen::JsValue::UNDEFINED,
                &wasm_bindgen::JsValue::from_str(&error),
            );
        }
    })
}

fn call_value_method(
    receiver: &wasm_bindgen::JsValue,
    name: &str,
    arguments: &[wasm_bindgen::JsValue],
) -> Result<wasm_bindgen::JsValue, String> {
    use wasm_bindgen::JsCast as _;

    let function = js_sys::Reflect::get(receiver, &wasm_bindgen::JsValue::from_str(name))
        .map_err(js_error)?
        .dyn_into::<js_sys::Function>()
        .map_err(|_| format!("browser object has no callable {name} method"))?;
    let args = js_sys::Array::new();
    for argument in arguments {
        args.push(argument);
    }
    function.apply(receiver, &args).map_err(js_error)
}

fn js_error(value: wasm_bindgen::JsValue) -> String {
    js_sys::Reflect::get(&value, &wasm_bindgen::JsValue::from_str("message"))
        .ok()
        .and_then(|message| message.as_string())
        .or_else(|| value.as_string())
        .unwrap_or_else(|| format!("browser error: {value:?}"))
}
