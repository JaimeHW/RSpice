//! The browser's durable pack store, over IndexedDB.
//!
//! # Why IndexedDB, and why this shape
//!
//! It is the mechanism this application already keeps large signed blobs in:
//! `state::pdk_config::persistence::browser` commits content-addressed PDK
//! archives the same way, for the same reason — Web Storage is synchronous,
//! string-only, and capped somewhere around five megabytes, which a single
//! pack archive can exceed on its own. What is *not* shared with that module
//! is its transaction machinery, and the reason is measured rather than
//! assumed: this store needs four operations over one object store, and that
//! module needs a compare-and-exchange head, a generation counter and a legacy
//! Web Storage migration, none of which a pack mirror has an equivalent of.
//!
//! # Keys
//!
//! ```text
//! catalog/snapshot          the exact signed snapshot bytes
//! catalog/serial            the highest accepted serial, as decimal text
//! archive/<sha256>          one verified pack archive, under its own digest
//! ```
//!
//! Content-addressed for the archives, so writing the same release twice
//! writes the same key twice and the set of keys *is* the set of installed
//! packs. There is deliberately no index beside them: an index would be a
//! second account of what is installed, kept in the same rewritable storage,
//! and a restore would have to decide which of the two to believe. Reading the
//! identity out of each archive's own signed manifest leaves exactly one
//! answer, and it is the one the signature covers.
//!
//! # Nothing here decides anything
//!
//! This module reads and writes bytes. Whether those bytes are a pack is
//! settled by [`super::hydrate`] under the trust anchor, after this module has
//! handed them over and has no further say.

use wasm_bindgen::JsValue;

use super::{
    DurableHubMirror, PackStorageStanding, PersistedHubState, StoredArchive,
    UNAVAILABLE_AFTER_WRITE, UNAVAILABLE_AT_OPEN,
};

const DATABASE: &str = "rspice-model-hub";
const STORE: &str = "verified-packs";
const SCHEMA_VERSION: u32 = 1;

const SNAPSHOT_KEY: &str = "catalog/snapshot";
const SERIAL_KEY: &str = "catalog/serial";
const ARCHIVE_PREFIX: &str = "archive/";

/// How long any one browser call may take before it is abandoned.
///
/// A storage call that never settles would otherwise hold the restore in its
/// loading phase for the life of the tab, which presents as a workspace that
/// silently never lists a pack.
const ASYNC_TIMEOUT_MS: u32 = 30_000;

/// The most archives one origin may restore.
///
/// A bound rather than a policy: the loop below verifies every archive it
/// reads, and an origin whose storage was filled with plausible keys would
/// otherwise spend the whole of a session's first seconds proving them.
const MAX_RESTORED_ARCHIVES: usize = 512;

/// The standing the last storage call observed.
///
/// Fire-and-forget writes have nowhere to return a failure to, so they leave it
/// here and the workspace reads it. Process-wide rather than thread-local
/// because the store is handed to the worker that performs an install: a
/// refusal observed while writing a pack on the worker has to be readable by
/// the frame that paints the note, and a per-thread cell would have left the
/// two disagreeing about whether this browser keeps anything.
static STANDING: std::sync::Mutex<Option<PackStorageStanding>> = std::sync::Mutex::new(None);

fn standing_slot() -> std::sync::MutexGuard<'static, Option<PackStorageStanding>> {
    STANDING
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
}

fn set_standing(standing: PackStorageStanding) {
    *standing_slot() = Some(standing);
}

fn current_standing() -> PackStorageStanding {
    standing_slot().clone().unwrap_or_default()
}

/// Records a write failure without overwriting a worse standing.
///
/// "Storage is unavailable" and "one write was refused" are different facts,
/// and the first one is the one worth keeping: an origin whose database will
/// not open produces a refusal per write, and letting the last one win would
/// replace the explanation with a symptom.
fn record_write_failure(operation: &str, error: &str) {
    log::warn!("durable model-pack storage could not {operation}: {error}");
    let mut slot = standing_slot();
    if !matches!(slot.as_ref(), Some(PackStorageStanding::Unavailable(_))) {
        *slot = Some(PackStorageStanding::Unavailable(
            UNAVAILABLE_AFTER_WRITE.to_owned(),
        ));
    }
}

/// The browser's mirror. Every write is spawned and none is awaited.
#[derive(Debug, Default)]
pub(crate) struct BrowserPackMirror;

impl DurableHubMirror for BrowserPackMirror {
    fn put_snapshot(&self, bytes: &[u8]) {
        spawn_put(SNAPSHOT_KEY.to_owned(), bytes.to_vec(), "keep the catalog");
    }

    fn put_serial(&self, serial: u64) {
        spawn_put(
            SERIAL_KEY.to_owned(),
            serial.to_string().into_bytes(),
            "keep the accepted catalog serial",
        );
    }

    fn put_archive(&self, digest: &str, bytes: &[u8]) {
        spawn_put(
            format!("{ARCHIVE_PREFIX}{digest}"),
            bytes.to_vec(),
            "keep a verified pack archive",
        );
    }

    fn delete_archive(&self, digest: &str) {
        let key = format!("{ARCHIVE_PREFIX}{digest}");
        wasm_bindgen_futures::spawn_local(async move {
            if let Err(error) = delete(&key).await {
                record_write_failure("drop a pack archive", &error);
            }
        });
    }

    fn standing(&self) -> PackStorageStanding {
        current_standing()
    }
}

fn spawn_put(key: String, bytes: Vec<u8>, operation: &'static str) {
    wasm_bindgen_futures::spawn_local(async move {
        if let Err(error) = put(&key, &bytes).await {
            record_write_failure(operation, &error);
        }
    });
}

/// Reads everything this origin kept, and asks the browser to keep more.
///
/// The persistence request is made here rather than at the first write, and
/// deliberately: `navigator.storage.persist()` may prompt, and the moment a
/// session opens is the moment a prompt is least surprising. Whatever it
/// answers, the standing records it — the note the workspace paints is derived
/// from that answer rather than from an assumption about it.
pub(crate) fn start_browser_pack_restore(
    complete: impl FnOnce(Result<PersistedHubState, String>) + 'static,
) {
    wasm_bindgen_futures::spawn_local(async move {
        let restored = read_persisted_state().await;
        match &restored {
            Ok(_) => set_standing(observe_durability().await),
            Err(error) => {
                log::warn!("model-pack storage could not be opened: {error}");
                set_standing(PackStorageStanding::Unavailable(
                    UNAVAILABLE_AT_OPEN.to_owned(),
                ));
            }
        }
        complete(restored);
    });
}

async fn read_persisted_state() -> Result<PersistedHubState, String> {
    let keys = all_keys().await?;
    let snapshot = get_bytes(SNAPSHOT_KEY).await?;
    let serial = get_bytes(SERIAL_KEY)
        .await?
        .and_then(|bytes| String::from_utf8(bytes).ok())
        // A floor that will not parse reads as none rather than as an error,
        // for the reason the filesystem store's own note gives: a damaged
        // floor must not stop a machine fetching a catalog, and the first
        // snapshot it accepts re-establishes it.
        .and_then(|text| text.trim().parse::<u64>().ok())
        .unwrap_or(super::super::NO_CATALOG_SERIAL);

    let mut archives = Vec::new();
    for key in keys {
        let Some(digest) = key.strip_prefix(ARCHIVE_PREFIX) else {
            continue;
        };
        if archives.len() >= MAX_RESTORED_ARCHIVES {
            log::warn!(
                "this origin holds more than {MAX_RESTORED_ARCHIVES} stored model packs; the \
                 remainder were not restored"
            );
            break;
        }
        // A key that has disappeared between the listing and the read is not a
        // failure of the restore: it is one pack fewer, and the rest still
        // prove.
        match get_bytes(&key).await {
            // The key's own suffix travels with the bytes. It is the name
            // these bytes were filed under, and `hydrate` refuses any pair
            // where the two disagree — which is the whole reason the key is a
            // digest rather than a serial number.
            Ok(Some(bytes)) => archives.push(StoredArchive {
                digest: digest.to_owned(),
                bytes,
            }),
            Ok(None) => {}
            Err(error) => log::warn!("a stored model pack could not be read: {error}"),
        }
    }
    Ok(PersistedHubState {
        serial,
        snapshot,
        archives,
    })
}

/// What the browser will promise about the bytes just written.
async fn observe_durability() -> PackStorageStanding {
    let Some(storage) = storage_manager() else {
        return PackStorageStanding::BestEffort;
    };
    let mut persisted = storage_boolean(&storage, "persisted").await;
    if persisted != Some(true) {
        persisted = storage_boolean(&storage, "persist").await.or(persisted);
    }
    match persisted {
        Some(true) => PackStorageStanding::Persistent,
        // `Some(false)` and `None` are the same promise — none — and stating
        // them differently would invent a distinction the reader cannot act
        // on.
        _ => PackStorageStanding::BestEffort,
    }
}

fn storage_manager() -> Option<JsValue> {
    let window = web_sys::window()?;
    let navigator = js_sys::Reflect::get(&window, &JsValue::from_str("navigator")).ok()?;
    let storage = js_sys::Reflect::get(&navigator, &JsValue::from_str("storage")).ok()?;
    (!storage.is_null() && !storage.is_undefined()).then_some(storage)
}

async fn storage_boolean(storage: &JsValue, method: &str) -> Option<bool> {
    let callable = js_sys::Reflect::get(storage, &JsValue::from_str(method)).ok()?;
    if callable.is_null() || callable.is_undefined() {
        return None;
    }
    let returned = call(storage, method, &[]).ok()?;
    await_promise(js_sys::Promise::resolve(&returned), method)
        .await
        .ok()?
        .as_bool()
}

// ---------------------------------------------------------------------------
// IndexedDB
// ---------------------------------------------------------------------------

/// An open database that closes itself when the last handle drops.
struct Database {
    value: JsValue,
    _on_version_change: wasm_bindgen::closure::Closure<dyn FnMut()>,
}

impl Drop for Database {
    fn drop(&mut self) {
        let _ = call(&self.value, "close", &[]);
    }
}

async fn get_bytes(key: &str) -> Result<Option<Vec<u8>>, String> {
    let database = open().await?;
    let transaction = call(
        &database.value,
        "transaction",
        &[JsValue::from_str(STORE), JsValue::from_str("readonly")],
    )?;
    let store = call(&transaction, "objectStore", &[JsValue::from_str(STORE)])?;
    let request = call(&store, "get", &[JsValue::from_str(key)])?;
    let value = await_request(&request).await?;
    if value.is_undefined() || value.is_null() {
        return Ok(None);
    }
    use wasm_bindgen::JsCast as _;
    if !value.is_instance_of::<js_sys::Uint8Array>() {
        // Stored bytes that are not bytes are a rewritten record. Reporting
        // them as absent is the safe reading: the restore proves whatever it
        // gets, and there is nothing here to prove.
        return Err(format!("the stored record at {key} is not a byte array"));
    }
    Ok(Some(js_sys::Uint8Array::new(&value).to_vec()))
}

async fn all_keys() -> Result<Vec<String>, String> {
    let database = open().await?;
    let transaction = call(
        &database.value,
        "transaction",
        &[JsValue::from_str(STORE), JsValue::from_str("readonly")],
    )?;
    let store = call(&transaction, "objectStore", &[JsValue::from_str(STORE)])?;
    let request = call(&store, "getAllKeys", &[])?;
    let value = await_request(&request).await?;
    Ok(js_sys::Array::from(&value)
        .iter()
        .filter_map(|key| key.as_string())
        .collect())
}

async fn put(key: &str, bytes: &[u8]) -> Result<(), String> {
    let database = open().await?;
    let transaction = readwrite(&database.value)?;
    let completion = await_transaction(&transaction);
    let store = call(&transaction, "objectStore", &[JsValue::from_str(STORE)])?;
    let request = call(
        &store,
        "put",
        &[
            js_sys::Uint8Array::from(bytes).into(),
            JsValue::from_str(key),
        ],
    )?;
    await_request(&request).await?;
    completion.await
}

async fn delete(key: &str) -> Result<(), String> {
    let database = open().await?;
    let transaction = readwrite(&database.value)?;
    let completion = await_transaction(&transaction);
    let store = call(&transaction, "objectStore", &[JsValue::from_str(STORE)])?;
    let request = call(&store, "delete", &[JsValue::from_str(key)])?;
    await_request(&request).await?;
    completion.await
}

/// A read-write transaction, asking for strict durability where it is offered.
fn readwrite(database: &JsValue) -> Result<JsValue, String> {
    let options = js_sys::Object::new();
    js_sys::Reflect::set(
        &options,
        &JsValue::from_str("durability"),
        &JsValue::from_str("strict"),
    )
    .map_err(js_error)?;
    call(
        database,
        "transaction",
        &[
            JsValue::from_str(STORE),
            JsValue::from_str("readwrite"),
            options.into(),
        ],
    )
    .or_else(|_| {
        // Browsers that do not know the option reject the three-argument form
        // outright, and a durability hint is not worth failing a write over.
        call(
            database,
            "transaction",
            &[JsValue::from_str(STORE), JsValue::from_str("readwrite")],
        )
    })
}

async fn open() -> Result<Database, String> {
    let window = web_sys::window().ok_or_else(|| "browser window is unavailable".to_owned())?;
    let factory =
        js_sys::Reflect::get(&window, &JsValue::from_str("indexedDB")).map_err(js_error)?;
    if factory.is_null() || factory.is_undefined() {
        return Err("IndexedDB is unavailable".to_owned());
    }
    let request = call(
        &factory,
        "open",
        &[
            JsValue::from_str(DATABASE),
            JsValue::from_f64(f64::from(SCHEMA_VERSION)),
        ],
    )?;

    let upgrade_request = request.clone();
    let on_upgrade = wasm_bindgen::closure::Closure::<dyn FnMut()>::new(move || {
        let Ok(database) = js_sys::Reflect::get(&upgrade_request, &JsValue::from_str("result"))
        else {
            return;
        };
        let exists = js_sys::Reflect::get(&database, &JsValue::from_str("objectStoreNames"))
            .ok()
            .and_then(|names| call(&names, "contains", &[JsValue::from_str(STORE)]).ok())
            .and_then(|value| value.as_bool())
            .unwrap_or(false);
        if !exists {
            let _ = call(&database, "createObjectStore", &[JsValue::from_str(STORE)]);
        }
    });

    let value = await_open(&request, on_upgrade).await?;
    let closing = value.clone();
    let on_version_change = wasm_bindgen::closure::Closure::<dyn FnMut()>::new(move || {
        // Another tab is upgrading the schema. Holding the connection open
        // would block it indefinitely, which presents in the other tab as a
        // workspace that never finishes loading.
        let _ = call(&closing, "close", &[]);
    });
    js_sys::Reflect::set(
        &value,
        &JsValue::from_str("onversionchange"),
        on_version_change.as_ref(),
    )
    .map_err(js_error)?;
    Ok(Database {
        value,
        _on_version_change: on_version_change,
    })
}

/// Settles an `IDBOpenDBRequest`, keeping its handlers alive until it does.
async fn await_open(
    request: &JsValue,
    on_upgrade: wasm_bindgen::closure::Closure<dyn FnMut()>,
) -> Result<JsValue, String> {
    let success_request = request.clone();
    let error_request = request.clone();
    let target = request.clone();
    let mut on_upgrade = Some(on_upgrade);
    let promise = js_sys::Promise::new(&mut move |resolve, reject| {
        let settled = std::rc::Rc::new(std::cell::Cell::new(false));

        let success_settled = settled.clone();
        let success_request = success_request.clone();
        let success = wasm_bindgen::closure::Closure::<dyn FnMut()>::new(move || {
            if success_settled.replace(true) {
                return;
            }
            let result = js_sys::Reflect::get(&success_request, &JsValue::from_str("result"))
                .unwrap_or(JsValue::UNDEFINED);
            let _ = resolve.call1(&JsValue::UNDEFINED, &result);
        });

        let failure_settled = settled.clone();
        let error_request = error_request.clone();
        let blocked_reject = reject.clone();
        let failure = wasm_bindgen::closure::Closure::<dyn FnMut()>::new(move || {
            if failure_settled.replace(true) {
                return;
            }
            let error = js_sys::Reflect::get(&error_request, &JsValue::from_str("error"))
                .unwrap_or_else(|_| JsValue::from_str("IndexedDB open failed"));
            let _ = reject.call1(&JsValue::UNDEFINED, &error);
        });

        let blocked_settled = settled.clone();
        let blocked = wasm_bindgen::closure::Closure::<dyn FnMut()>::new(move || {
            if blocked_settled.replace(true) {
                return;
            }
            let _ = blocked_reject.call1(
                &JsValue::UNDEFINED,
                &JsValue::from_str("another RSpice tab is upgrading pack storage"),
            );
        });

        let upgrade = on_upgrade.take().expect("one open per request");
        for (property, handler) in [
            ("onsuccess", success.as_ref().clone()),
            ("onerror", failure.as_ref().clone()),
            ("onblocked", blocked.as_ref().clone()),
            ("onupgradeneeded", upgrade.as_ref().clone()),
        ] {
            let _ = js_sys::Reflect::set(&target, &JsValue::from_str(property), &handler);
        }
        // The browser owns these until the request settles; the timeout below
        // bounds how long that is, so leaking them is bounded too.
        retain(vec![success, failure, blocked, upgrade]);
    });
    await_promise(promise, "IndexedDB open").await
}

/// Settles an `IDBRequest`.
async fn await_request(request: &JsValue) -> Result<JsValue, String> {
    let success_request = request.clone();
    let error_request = request.clone();
    let target = request.clone();
    let promise = js_sys::Promise::new(&mut move |resolve, reject| {
        let settled = std::rc::Rc::new(std::cell::Cell::new(false));
        let success_settled = settled.clone();
        let success_request = success_request.clone();
        let success = wasm_bindgen::closure::Closure::<dyn FnMut()>::new(move || {
            if success_settled.replace(true) {
                return;
            }
            let result = js_sys::Reflect::get(&success_request, &JsValue::from_str("result"))
                .unwrap_or(JsValue::UNDEFINED);
            let _ = resolve.call1(&JsValue::UNDEFINED, &result);
        });
        let failure_settled = settled.clone();
        let error_request = error_request.clone();
        let failure = wasm_bindgen::closure::Closure::<dyn FnMut()>::new(move || {
            if failure_settled.replace(true) {
                return;
            }
            let error = js_sys::Reflect::get(&error_request, &JsValue::from_str("error"))
                .unwrap_or_else(|_| JsValue::from_str("IndexedDB request failed"));
            let _ = reject.call1(&JsValue::UNDEFINED, &error);
        });
        for (property, handler) in [
            ("onsuccess", success.as_ref().clone()),
            ("onerror", failure.as_ref().clone()),
        ] {
            let _ = js_sys::Reflect::set(&target, &JsValue::from_str(property), &handler);
        }
        retain(vec![success, failure]);
    });
    await_promise(promise, "IndexedDB request").await
}

/// Settles when the transaction commits, so a caller can report a write only
/// after the browser has actually taken it.
fn await_transaction(
    transaction: &JsValue,
) -> impl std::future::Future<Output = Result<(), String>> + 'static {
    let target = transaction.clone();
    let error_transaction = transaction.clone();
    let promise = js_sys::Promise::new(&mut move |resolve, reject| {
        let settled = std::rc::Rc::new(std::cell::Cell::new(false));
        let complete_settled = settled.clone();
        let complete = wasm_bindgen::closure::Closure::<dyn FnMut()>::new(move || {
            if !complete_settled.replace(true) {
                let _ = resolve.call0(&JsValue::UNDEFINED);
            }
        });
        let failure_settled = settled.clone();
        let error_transaction = error_transaction.clone();
        let failure = wasm_bindgen::closure::Closure::<dyn FnMut()>::new(move || {
            if failure_settled.replace(true) {
                return;
            }
            // A quota refusal arrives here, as an aborted transaction whose
            // error names the reason. It is the sentence the reader is shown.
            let error = js_sys::Reflect::get(&error_transaction, &JsValue::from_str("error"))
                .unwrap_or_else(|_| JsValue::from_str("IndexedDB transaction failed"));
            let _ = reject.call1(&JsValue::UNDEFINED, &error);
        });
        for (property, handler) in [
            ("oncomplete", complete.as_ref().clone()),
            ("onabort", failure.as_ref().clone()),
            ("onerror", failure.as_ref().clone()),
        ] {
            let _ = js_sys::Reflect::set(&target, &JsValue::from_str(property), &handler);
        }
        retain(vec![complete, failure]);
    });
    async move {
        await_promise(promise, "IndexedDB transaction")
            .await
            .map(|_| ())
    }
}

thread_local! {
    /// Handlers the browser still owns.
    ///
    /// A `Closure` dropped while an event source still points at it leaves the
    /// browser calling into freed memory, so each set is held until the
    /// operation that armed it can no longer fire. Every operation here is
    /// bounded by [`ASYNC_TIMEOUT_MS`], and the list is drained from the front
    /// once it grows past what a healthy session ever holds at once.
    static RETAINED: std::cell::RefCell<Vec<Vec<wasm_bindgen::closure::Closure<dyn FnMut()>>>> =
        const { std::cell::RefCell::new(Vec::new()) };
}

/// The most in-flight operations one session keeps handlers for.
const MAX_RETAINED_OPERATIONS: usize = 64;

fn retain(callbacks: Vec<wasm_bindgen::closure::Closure<dyn FnMut()>>) {
    RETAINED.with(|retained| {
        let mut retained = retained.borrow_mut();
        if retained.len() >= MAX_RETAINED_OPERATIONS {
            retained.remove(0);
        }
        retained.push(callbacks);
    });
}

/// Awaits a promise, or gives up on it.
async fn await_promise(promise: js_sys::Promise, operation: &str) -> Result<JsValue, String> {
    let race = js_sys::Array::new();
    race.push(&promise);
    race.push(&timeout(operation));
    wasm_bindgen_futures::JsFuture::from(js_sys::Promise::race(&race))
        .await
        .map_err(js_error)
}

fn timeout(operation: &str) -> js_sys::Promise {
    let message = format!(
        "{operation} timed out after {} seconds",
        ASYNC_TIMEOUT_MS / 1_000
    );
    js_sys::Promise::new(&mut move |_resolve, reject| {
        let message = message.clone();
        let expire = reject.clone();
        let callback = wasm_bindgen::closure::Closure::once_into_js(move || {
            let _ = expire.call1(&JsValue::UNDEFINED, &JsValue::from_str(&message));
        });
        let Some(window) = web_sys::window() else {
            let _ = reject.call1(
                &JsValue::UNDEFINED,
                &JsValue::from_str("browser window is unavailable"),
            );
            return;
        };
        if let Err(error) = call(
            &window.into(),
            "setTimeout",
            &[callback, JsValue::from_f64(f64::from(ASYNC_TIMEOUT_MS))],
        ) {
            let _ = reject.call1(&JsValue::UNDEFINED, &JsValue::from_str(&error));
        }
    })
}

fn call(receiver: &JsValue, name: &str, arguments: &[JsValue]) -> Result<JsValue, String> {
    use wasm_bindgen::JsCast as _;

    let function = js_sys::Reflect::get(receiver, &JsValue::from_str(name))
        .map_err(js_error)?
        .dyn_into::<js_sys::Function>()
        .map_err(|_| format!("browser object has no callable {name} method"))?;
    let args = js_sys::Array::new();
    for argument in arguments {
        args.push(argument);
    }
    function.apply(receiver, &args).map_err(js_error)
}

fn js_error(value: JsValue) -> String {
    js_sys::Reflect::get(&value, &JsValue::from_str("message"))
        .ok()
        .and_then(|message| message.as_string())
        .or_else(|| value.as_string())
        .unwrap_or_else(|| format!("browser error: {value:?}"))
}
