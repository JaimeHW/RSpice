//! The browser's durable pack store, over IndexedDB.
//!
//! # Why IndexedDB, and why this shape
//!
//! It is the mechanism this application already keeps large signed blobs in:
//! the PDK configuration commits content-addressed archives the same way, for
//! the same reason — Web Storage is synchronous, string-only, and capped
//! somewhere around five megabytes, which a single pack archive can exceed on
//! its own. Both go through [`crate::browser_store`]. What the PDK store builds
//! on it (a compare-and-exchange head, a generation counter, a legacy Web
//! Storage migration) has no equivalent in a pack mirror, which needs four
//! operations over one object store.
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
use crate::browser_store::{self, MODEL_HUB};

const SNAPSHOT_KEY: &str = "catalog/snapshot";
const SERIAL_KEY: &str = "catalog/serial";
const ARCHIVE_PREFIX: &str = "archive/";

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
pub struct BrowserPackMirror;

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
///
/// The state arrives with the reason for each stored pack that could not be
/// read, for the Console: one unreadable pack is one pack fewer, not a failed
/// restore.
pub fn start_browser_pack_restore(
    complete: impl FnOnce(Result<(PersistedHubState, Vec<String>), String>) + 'static,
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

async fn read_persisted_state() -> Result<(PersistedHubState, Vec<String>), String> {
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
    let mut unrestored = Vec::new();
    for key in keys {
        let Some(digest) = key.strip_prefix(ARCHIVE_PREFIX) else {
            continue;
        };
        if archives.len() >= MAX_RESTORED_ARCHIVES {
            unrestored.push(format!(
                "This browser holds more than {MAX_RESTORED_ARCHIVES} stored model packs; the \
                 rest were not restored."
            ));
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
            Err(error) => {
                unrestored.push(format!(
                    "A model pack kept by this browser could not be read: {error}"
                ));
            }
        }
    }
    Ok((
        PersistedHubState {
            serial,
            snapshot,
            archives,
        },
        unrestored,
    ))
}

/// What the browser will promise about the bytes just written.
async fn observe_durability() -> PackStorageStanding {
    let Some(storage) = browser_store::storage_manager() else {
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

async fn storage_boolean(storage: &JsValue, method: &str) -> Option<bool> {
    browser_store::storage_call(storage, method, MODEL_HUB.timeout_ms)
        .await
        .ok()
        .flatten()?
        .as_bool()
}

// ---------------------------------------------------------------------------
// IndexedDB
// ---------------------------------------------------------------------------

async fn get_bytes(key: &str) -> Result<Option<Vec<u8>>, String> {
    let database = browser_store::open(&MODEL_HUB).await?;
    let Some(value) = database.read()?.get(key).await? else {
        return Ok(None);
    };
    if value.is_null() {
        return Ok(None);
    }
    // Stored bytes that are not bytes are a rewritten record. Reporting them
    // as absent is the safe reading: the restore proves whatever it gets, and
    // there is nothing here to prove.
    browser_store::bytes(&value)
        .map(Some)
        .ok_or_else(|| format!("the stored record at {key} is not a byte array"))
}

async fn all_keys() -> Result<Vec<String>, String> {
    let database = browser_store::open(&MODEL_HUB).await?;
    database.read()?.keys().await
}

async fn put(key: &str, bytes: &[u8]) -> Result<(), String> {
    let database = browser_store::open(&MODEL_HUB).await?;
    let transaction = database.write()?;
    transaction
        .put(key, &js_sys::Uint8Array::from(bytes).into())
        .await?;
    transaction.commit().await
}

async fn delete(key: &str) -> Result<(), String> {
    let database = browser_store::open(&MODEL_HUB).await?;
    let transaction = database.write()?;
    transaction.delete(key).await?;
    transaction.commit().await
}
