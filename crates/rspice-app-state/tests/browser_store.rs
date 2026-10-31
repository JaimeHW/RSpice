//! The browser store against a real IndexedDB, in a browser window.

#![cfg(target_arch = "wasm32")]

use rspice_app_state::browser_store::{self, Schema};
use wasm_bindgen::prelude::*;
use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen(inline_js = "
export function deleteDatabase(name) {
    return new Promise((resolve, reject) => {
        const request = indexedDB.deleteDatabase(name);
        request.onsuccess = () => resolve();
        request.onerror = () => reject(request.error);
        request.onblocked = () => reject(new Error('delete blocked'));
    });
}
export function seedDatabase(name, version, store, key, value) {
    return new Promise((resolve, reject) => {
        const request = indexedDB.open(name, version);
        request.onupgradeneeded = () => request.result.createObjectStore(store);
        request.onerror = () => reject(request.error);
        request.onsuccess = () => {
            const db = request.result;
            const transaction = db.transaction(store, 'readwrite');
            transaction.objectStore(store).put(value, key);
            transaction.oncomplete = () => { db.close(); resolve(); };
            transaction.onabort = () => { db.close(); reject(transaction.error); };
        };
    });
}
export function databaseVersion(name) {
    return new Promise((resolve, reject) => {
        const request = indexedDB.open(name);
        request.onerror = () => reject(request.error);
        request.onsuccess = () => {
            const version = request.result.version;
            request.result.close();
            resolve(version);
        };
    });
}
")]
extern "C" {
    #[wasm_bindgen(js_name = deleteDatabase)]
    fn delete_database(name: &str) -> js_sys::Promise;
    #[wasm_bindgen(js_name = seedDatabase)]
    fn seed_database(
        name: &str,
        version: u32,
        store: &str,
        key: &str,
        value: &JsValue,
    ) -> js_sys::Promise;
    #[wasm_bindgen(js_name = databaseVersion)]
    fn database_version(name: &str) -> js_sys::Promise;
}

const fn schema(database: &'static str, version: u32) -> Schema {
    Schema {
        database,
        version,
        store: "records",
        timeout_ms: 15_000,
        blocked: "another tab holds the test database open",
        pending: 32,
        refusal: "too many test operations are pending",
    }
}

static ROUND_TRIP: Schema = schema("rspice-browser-store-round-trip", 1);
static ADD: Schema = schema("rspice-browser-store-add", 1);
static ABORT: Schema = schema("rspice-browser-store-abort", 1);
static UPGRADE: Schema = schema("rspice-browser-store-upgrade", 2);
static CAPPED: Schema = Schema {
    pending: 2,
    ..schema("rspice-browser-store-capped", 1)
};

async fn fresh(schema: &Schema) {
    wasm_bindgen_futures::JsFuture::from(delete_database(schema.database))
        .await
        .expect("delete the test database");
}

async fn stored(schema: &'static Schema, key: &str) -> Option<JsValue> {
    let database = browser_store::open(schema).await.expect("open");
    database
        .read()
        .expect("read transaction")
        .get(key)
        .await
        .expect("get")
}

#[wasm_bindgen_test]
async fn records_written_read_back_list_and_delete() {
    fresh(&ROUND_TRIP).await;
    let database = browser_store::open(&ROUND_TRIP).await.expect("open");
    let write = database.write().expect("write transaction");
    write
        .put(
            "archive/a",
            &js_sys::Uint8Array::from(&[1_u8, 2, 3][..]).into(),
        )
        .await
        .expect("put bytes");
    write
        .put("head:a", &JsValue::from_str("{\"generation\":1}"))
        .await
        .expect("put text");
    write.commit().await.expect("commit");

    let read = database.read().expect("read transaction");
    let bytes = read.get("archive/a").await.expect("get bytes");
    assert_eq!(
        bytes.as_ref().and_then(browser_store::bytes),
        Some(vec![1, 2, 3])
    );
    let text = read.get("head:a").await.expect("get text");
    assert_eq!(
        text.and_then(|value| value.as_string()).as_deref(),
        Some("{\"generation\":1}")
    );
    assert!(read.get("absent").await.expect("get absent").is_none());
    let mut keys = read.keys().await.expect("keys");
    keys.sort();
    assert_eq!(keys, ["archive/a", "head:a"]);

    let delete = database.write().expect("write transaction");
    delete.delete("archive/a").await.expect("delete");
    delete.commit().await.expect("commit delete");
    drop(database);
    assert!(stored(&ROUND_TRIP, "archive/a").await.is_none());
}

#[wasm_bindgen_test]
async fn add_refuses_a_taken_key_and_keeps_the_first_record() {
    fresh(&ADD).await;
    let database = browser_store::open(&ADD).await.expect("open");
    let first = database.write().expect("write transaction");
    first
        .add("checkpoint", &JsValue::from_str("first"))
        .await
        .expect("first add");
    first.commit().await.expect("commit first add");

    let second = database.write().expect("write transaction");
    assert!(
        second
            .add("checkpoint", &JsValue::from_str("second"))
            .await
            .is_err()
    );
    assert!(second.commit().await.is_err());
    drop(database);
    assert_eq!(
        stored(&ADD, "checkpoint")
            .await
            .and_then(|value| value.as_string())
            .as_deref(),
        Some("first")
    );
}

#[wasm_bindgen_test]
async fn an_aborted_write_leaves_the_stored_record() {
    fresh(&ABORT).await;
    let database = browser_store::open(&ABORT).await.expect("open");
    let seed = database.write().expect("write transaction");
    seed.put("head", &JsValue::from_str("kept"))
        .await
        .expect("seed");
    seed.commit().await.expect("commit seed");

    let replace = database.write().expect("write transaction");
    let existing = replace.get("head").await.expect("get in write");
    assert_eq!(
        existing.and_then(|value| value.as_string()).as_deref(),
        Some("kept")
    );
    replace
        .put("head", &JsValue::from_str("replaced"))
        .await
        .expect("put before abort");
    replace.abort();
    assert!(replace.commit().await.is_err());
    drop(database);
    assert_eq!(
        stored(&ABORT, "head")
            .await
            .and_then(|value| value.as_string())
            .as_deref(),
        Some("kept")
    );
}

#[wasm_bindgen_test]
async fn opening_at_a_newer_version_keeps_every_record() {
    fresh(&UPGRADE).await;
    wasm_bindgen_futures::JsFuture::from(seed_database(
        UPGRADE.database,
        1,
        UPGRADE.store,
        "binding",
        &JsValue::from_str("written by an older build"),
    ))
    .await
    .expect("seed version 1");

    assert_eq!(
        stored(&UPGRADE, "binding")
            .await
            .and_then(|value| value.as_string())
            .as_deref(),
        Some("written by an older build")
    );
    let version = wasm_bindgen_futures::JsFuture::from(database_version(UPGRADE.database))
        .await
        .expect("read version");
    assert_eq!(version.as_f64(), Some(2.0));
}

#[wasm_bindgen_test]
async fn an_operation_past_the_pending_cap_is_refused_and_the_others_complete() {
    fresh(&CAPPED).await;
    let database = browser_store::open(&CAPPED).await.expect("open");
    let first = database.write().expect("first write transaction");
    let second = database.write().expect("second write transaction");

    // Both transactions' completions are armed and unsettled, so a write in
    // either one needs a third slot. It is refused rather than given one by
    // dropping the handlers of an operation still in flight.
    let refused = first
        .put("refused", &JsValue::from_str("refused"))
        .await
        .expect_err("a write past the cap is refused");
    assert_eq!(refused, CAPPED.refusal);
    assert!(
        first.commit().await.is_err(),
        "the refused write's transaction aborts"
    );
    second
        .commit()
        .await
        .expect("the transaction still in flight completes");

    let retry = database.write().expect("write transaction");
    retry
        .put("kept", &JsValue::from_str("kept"))
        .await
        .expect("a write under the cap");
    retry.commit().await.expect("commit under the cap");
    drop(database);
    assert!(stored(&CAPPED, "refused").await.is_none());
    assert_eq!(
        stored(&CAPPED, "kept")
            .await
            .and_then(|value| value.as_string())
            .as_deref(),
        Some("kept")
    );
}
