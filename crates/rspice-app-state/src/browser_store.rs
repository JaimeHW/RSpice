//! The browser's durable records, over IndexedDB.
//!
//! Every database the application keeps in an origin goes through this one
//! implementation. Each holds a single object store with out-of-line string
//! keys, and its upgrade creates that store when it is missing and does
//! nothing else, so opening a database at a newer version never touches the
//! records already in it.
//!
//! A database's name, version and store are part of every user's saved data:
//! changing one strands what is already stored under the old one. Each is a
//! [`Schema`] constant here, pinned by a test.
//!
//! A new kind of record gets a new database rather than a second store in an
//! existing one, because a second store needs a version bump and an older
//! build still open in another tab can then no longer open that database; the
//! browser font cache is its own database for that reason.
//!
//! Every operation opens its own connection, which closes when its
//! [`Database`] drops, and every browser call is bounded by its schema's
//! timeout, so a call that never settles cannot hold its caller forever.

/// One IndexedDB database and its single object store.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Schema {
    pub database: &'static str,
    pub version: u32,
    pub store: &'static str,
    /// How long one browser call may take before it is abandoned.
    pub timeout_ms: u32,
    /// What the caller is told when another tab holds the database open
    /// across an upgrade.
    pub blocked: &'static str,
    /// How many operations may be unsettled at once. One more is refused with
    /// `refusal` rather than started, because its handlers could only be kept
    /// by dropping those of an operation still in flight.
    pub pending: usize,
    pub refusal: &'static str,
}

/// The Model Hub's verified packs: the catalog snapshot, its serial, and each
/// pack archive under its own digest.
pub const MODEL_HUB: Schema = Schema {
    database: "rspice-model-hub",
    version: 1,
    store: "verified-packs",
    // A call that never settles would otherwise hold the pack restore in its
    // loading phase for the life of the tab, which presents as a workspace
    // that silently never lists a pack.
    timeout_ms: 30_000,
    blocked: "another RSpice tab is upgrading pack storage",
    pending: 64,
    refusal: "too many model-pack storage operations are still pending",
};

/// The PDK configuration: a generation head per configuration path, its
/// metadata, and each signed archive under its own digest.
pub const PDK_CONFIG: Schema = Schema {
    database: "rspice-pdk-config",
    version: 1,
    store: "content-addressed-pdk-config",
    timeout_ms: 30_000,
    blocked: "IndexedDB schema upgrade is blocked by another RSpice tab",
    pending: 32,
    refusal: "too many browser PDK storage operations are still pending",
};

#[cfg(target_arch = "wasm32")]
pub use browser::*;

#[cfg(target_arch = "wasm32")]
mod browser {
    use std::cell::{Cell, RefCell};
    use std::rc::Rc;

    use wasm_bindgen::JsValue;
    use wasm_bindgen::closure::Closure;

    use super::Schema;

    type Handler = Closure<dyn FnMut()>;

    /// An open connection, closed when it drops.
    pub struct Database {
        value: JsValue,
        schema: &'static Schema,
        _on_version_change: Handler,
    }

    impl Drop for Database {
        fn drop(&mut self) {
            let _ = call(&self.value, "close", &[]);
        }
    }

    /// Opens `schema`'s database, creating its store if it is missing.
    pub async fn open(schema: &'static Schema) -> Result<Database, String> {
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
                JsValue::from_str(schema.database),
                JsValue::from_f64(f64::from(schema.version)),
            ],
        )?;

        let upgrade_request = request.clone();
        let store = schema.store;
        let on_upgrade = Closure::<dyn FnMut()>::new(move || {
            let Ok(database) = js_sys::Reflect::get(&upgrade_request, &JsValue::from_str("result"))
            else {
                return;
            };
            let exists = js_sys::Reflect::get(&database, &JsValue::from_str("objectStoreNames"))
                .ok()
                .and_then(|names| call(&names, "contains", &[JsValue::from_str(store)]).ok())
                .and_then(|value| value.as_bool())
                .unwrap_or(false);
            if !exists {
                let _ = call(&database, "createObjectStore", &[JsValue::from_str(store)]);
            }
        });

        let value = await_open(schema, &request, on_upgrade).await?;
        let closing = value.clone();
        let on_version_change = Closure::<dyn FnMut()>::new(move || {
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
            schema,
            _on_version_change: on_version_change,
        })
    }

    impl Database {
        /// A read-only transaction over the store.
        pub fn read(&self) -> Result<Transaction<'_>, String> {
            let transaction = call(
                &self.value,
                "transaction",
                &[
                    JsValue::from_str(self.schema.store),
                    JsValue::from_str("readonly"),
                ],
            )?;
            Transaction::over(self, transaction, None)
        }

        /// A read-write transaction over the store, asking for strict
        /// durability where it is offered. Its writes count only once
        /// [`Transaction::commit`] returns.
        pub fn write(&self) -> Result<Transaction<'_>, String> {
            let options = js_sys::Object::new();
            js_sys::Reflect::set(
                &options,
                &JsValue::from_str("durability"),
                &JsValue::from_str("strict"),
            )
            .map_err(js_error)?;
            let transaction = call(
                &self.value,
                "transaction",
                &[
                    JsValue::from_str(self.schema.store),
                    JsValue::from_str("readwrite"),
                    options.into(),
                ],
            )
            .or_else(|_| {
                // Browsers that do not know the option reject the three-argument
                // form outright, and a durability hint is not worth failing a
                // write over.
                call(
                    &self.value,
                    "transaction",
                    &[
                        JsValue::from_str(self.schema.store),
                        JsValue::from_str("readwrite"),
                    ],
                )
            })?;
            let completion = completion(self.schema, &transaction);
            Transaction::over(self, transaction, Some(completion))
        }
    }

    /// One transaction over a database's store.
    pub struct Transaction<'a> {
        database: &'a Database,
        value: JsValue,
        store: JsValue,
        completion: Option<js_sys::Promise>,
    }

    impl<'a> Transaction<'a> {
        fn over(
            database: &'a Database,
            value: JsValue,
            completion: Option<js_sys::Promise>,
        ) -> Result<Self, String> {
            let store = call(
                &value,
                "objectStore",
                &[JsValue::from_str(database.schema.store)],
            )?;
            Ok(Self {
                database,
                value,
                store,
                completion,
            })
        }

        /// The record under `key`, or `None` when there is none.
        pub async fn get(&self, key: &str) -> Result<Option<JsValue>, String> {
            let value = self.request("get", &[JsValue::from_str(key)]).await?;
            Ok((!value.is_undefined()).then_some(value))
        }

        /// Every string key in the store.
        pub async fn keys(&self) -> Result<Vec<String>, String> {
            let keys = self.request("getAllKeys", &[]).await?;
            Ok(js_sys::Array::from(&keys)
                .iter()
                .filter_map(|key| key.as_string())
                .collect())
        }

        /// Writes `value` under `key`, replacing any record already there.
        pub async fn put(&self, key: &str, value: &JsValue) -> Result<(), String> {
            self.request("put", &[value.clone(), JsValue::from_str(key)])
                .await
                .map(|_| ())
        }

        /// Writes `value` under `key`, failing if the key is already taken.
        pub async fn add(&self, key: &str, value: &JsValue) -> Result<(), String> {
            self.request("add", &[value.clone(), JsValue::from_str(key)])
                .await
                .map(|_| ())
        }

        pub async fn delete(&self, key: &str) -> Result<(), String> {
            self.request("delete", &[JsValue::from_str(key)])
                .await
                .map(|_| ())
        }

        /// Discards every write made in this transaction.
        pub fn abort(&self) {
            let _ = call(&self.value, "abort", &[]);
        }

        /// Settles when a read-write transaction commits, so a caller reports
        /// a write only after the browser has actually taken it.
        pub async fn commit(self) -> Result<(), String> {
            let Some(completion) = self.completion else {
                return Ok(());
            };
            let schema = self.database.schema;
            let result = await_promise(completion, "IndexedDB transaction", schema.timeout_ms)
                .await
                .map(|_| ());
            if result.is_err() {
                let _ = call(&self.value, "abort", &[]);
            }
            result
        }

        async fn request(&self, method: &str, arguments: &[JsValue]) -> Result<JsValue, String> {
            let request = call(&self.store, method, arguments)?;
            await_request(self.database.schema, &request).await
        }
    }

    /// The bytes of a stored byte array, or `None` for any other value.
    pub fn bytes(value: &JsValue) -> Option<Vec<u8>> {
        use wasm_bindgen::JsCast as _;
        value
            .is_instance_of::<js_sys::Uint8Array>()
            .then(|| js_sys::Uint8Array::new(value).to_vec())
    }

    /// Settles an `IDBOpenDBRequest`, keeping its handlers alive until it does.
    async fn await_open(
        schema: &'static Schema,
        request: &JsValue,
        on_upgrade: Handler,
    ) -> Result<JsValue, String> {
        let request_for_handlers = request.clone();
        let mut on_upgrade = Some(on_upgrade);
        // Set once the caller has its answer, so a connection that opens later
        // is never handed to anyone.
        let reported = Rc::new(Cell::new(false));
        let reported_for_promise = reported.clone();
        let promise = js_sys::Promise::new(&mut move |resolve, reject| {
            let reported = reported_for_promise.clone();
            let retire = Rc::new(Cell::new(false));

            let success_reported = reported.clone();
            let success_retire = retire.clone();
            let success_resolve = resolve.clone();
            let success_request = request_for_handlers.clone();
            let success = Closure::<dyn FnMut()>::new(move || {
                let result = js_sys::Reflect::get(&success_request, &JsValue::from_str("result"))
                    .unwrap_or(JsValue::UNDEFINED);
                if success_reported.replace(true) {
                    let _ = call(&result, "close", &[]);
                } else {
                    let _ = success_resolve.call1(&JsValue::UNDEFINED, &result);
                }
                success_retire.set(true);
            });

            let failure_reported = reported.clone();
            let failure_retire = retire.clone();
            let failure_reject = reject.clone();
            let error_request = request_for_handlers.clone();
            let failure = Closure::<dyn FnMut()>::new(move || {
                if !failure_reported.replace(true) {
                    let error = js_sys::Reflect::get(&error_request, &JsValue::from_str("error"))
                        .unwrap_or_else(|_| JsValue::from_str("IndexedDB open failed"));
                    let _ = failure_reject.call1(&JsValue::UNDEFINED, &error);
                }
                failure_retire.set(true);
            });

            // A blocked request is reported at once but kept until its eventual
            // success or error, so a connection that opens late is still seen.
            let blocked_reported = reported.clone();
            let blocked_reject = reject.clone();
            let blocked = Closure::<dyn FnMut()>::new(move || {
                if !blocked_reported.replace(true) {
                    let _ = blocked_reject
                        .call1(&JsValue::UNDEFINED, &JsValue::from_str(schema.blocked));
                }
            });

            let upgrade = on_upgrade.take().expect("one open per request");
            let bindings = [
                ("onsuccess", success.as_ref().clone()),
                ("onerror", failure.as_ref().clone()),
                ("onblocked", blocked.as_ref().clone()),
                ("onupgradeneeded", upgrade.as_ref().clone()),
            ];
            arm(
                schema,
                &request_for_handlers,
                &retire,
                &reject,
                bindings,
                vec![success, failure, blocked, upgrade],
            );
        });
        let result = await_promise(promise, "IndexedDB open", schema.timeout_ms).await;
        if result.is_err() {
            reported.set(true);
        }
        result
    }

    /// Settles an `IDBRequest`.
    async fn await_request(schema: &Schema, request: &JsValue) -> Result<JsValue, String> {
        let request_for_handlers = request.clone();
        let promise = js_sys::Promise::new(&mut move |resolve, reject| {
            let done = Rc::new(Cell::new(false));

            let success_done = done.clone();
            let success_request = request_for_handlers.clone();
            let success = Closure::<dyn FnMut()>::new(move || {
                if success_done.replace(true) {
                    return;
                }
                let result = js_sys::Reflect::get(&success_request, &JsValue::from_str("result"))
                    .unwrap_or(JsValue::UNDEFINED);
                let _ = resolve.call1(&JsValue::UNDEFINED, &result);
            });

            let failure_done = done.clone();
            let failure_reject = reject.clone();
            let error_request = request_for_handlers.clone();
            let failure = Closure::<dyn FnMut()>::new(move || {
                if failure_done.replace(true) {
                    return;
                }
                let error = js_sys::Reflect::get(&error_request, &JsValue::from_str("error"))
                    .unwrap_or_else(|_| JsValue::from_str("IndexedDB request failed"));
                let _ = failure_reject.call1(&JsValue::UNDEFINED, &error);
            });

            let bindings = [
                ("onsuccess", success.as_ref().clone()),
                ("onerror", failure.as_ref().clone()),
            ];
            arm(
                schema,
                &request_for_handlers,
                &done,
                &reject,
                bindings,
                vec![success, failure],
            );
        });
        let result = await_promise(promise, "IndexedDB request", schema.timeout_ms).await;
        if result.is_err()
            && let Ok(transaction) =
                js_sys::Reflect::get(request, &JsValue::from_str("transaction"))
            && !transaction.is_null()
            && !transaction.is_undefined()
        {
            let _ = call(&transaction, "abort", &[]);
        }
        result
    }

    /// Arms a transaction's completion. The handlers are set before any
    /// request is made on it, so no outcome can be missed.
    fn completion(schema: &Schema, transaction: &JsValue) -> js_sys::Promise {
        let transaction = transaction.clone();
        js_sys::Promise::new(&mut move |resolve, reject| {
            let done = Rc::new(Cell::new(false));

            let complete_done = done.clone();
            let complete = Closure::<dyn FnMut()>::new(move || {
                if !complete_done.replace(true) {
                    let _ = resolve.call0(&JsValue::UNDEFINED);
                }
            });

            let failure_done = done.clone();
            let failure_reject = reject.clone();
            let error_transaction = transaction.clone();
            let failure = Closure::<dyn FnMut()>::new(move || {
                if failure_done.replace(true) {
                    return;
                }
                // A quota refusal arrives here, as an aborted transaction whose
                // error names the reason. It is the sentence the reader is shown.
                let error = js_sys::Reflect::get(&error_transaction, &JsValue::from_str("error"))
                    .unwrap_or_else(|_| JsValue::from_str("IndexedDB transaction failed"));
                let _ = failure_reject.call1(&JsValue::UNDEFINED, &error);
            });

            let failure_value = failure.as_ref().clone();
            let bindings = [
                ("oncomplete", complete.as_ref().clone()),
                ("onabort", failure_value.clone()),
                ("onerror", failure_value),
            ];
            arm(
                schema,
                &transaction,
                &done,
                &reject,
                bindings,
                vec![complete, failure],
            );
        })
    }

    /// Keeps `handlers` alive and points `target`'s events at them, or rejects
    /// the operation when it cannot.
    fn arm<const N: usize>(
        schema: &Schema,
        target: &JsValue,
        settled: &Rc<Cell<bool>>,
        reject: &js_sys::Function,
        bindings: [(&str, JsValue); N],
        handlers: Vec<Handler>,
    ) {
        if let Err(refusal) = retain(schema, settled.clone(), handlers) {
            settled.set(true);
            let _ = reject.call1(&JsValue::UNDEFINED, &JsValue::from_str(&refusal));
            return;
        }
        for (property, handler) in bindings {
            if let Err(error) = js_sys::Reflect::set(target, &JsValue::from_str(property), &handler)
            {
                settled.set(true);
                let _ = reject.call1(&JsValue::UNDEFINED, &error);
                return;
            }
        }
    }

    struct Retained {
        database: &'static str,
        settled: Rc<Cell<bool>>,
        _handlers: Vec<Handler>,
    }

    thread_local! {
        /// Handlers the browser still owns.
        ///
        /// A `Closure` dropped while an event source still points at it leaves
        /// the browser calling into a dropped handler, so each set is held
        /// until the operation that armed it settles.
        static RETAINED: RefCell<Vec<Retained>> = const { RefCell::new(Vec::new()) };
    }

    fn retain(
        schema: &Schema,
        settled: Rc<Cell<bool>>,
        handlers: Vec<Handler>,
    ) -> Result<(), String> {
        let database = schema.database;
        RETAINED.with(|retained| {
            let mut retained = retained.borrow_mut();
            retained.retain(|r| r.database != database || !r.settled.get());
            let held = retained.iter().filter(|r| r.database == database).count();
            if held >= schema.pending {
                return Err(schema.refusal.to_owned());
            }
            retained.push(Retained {
                database,
                settled,
                _handlers: handlers,
            });
            Ok(())
        })
    }

    /// The origin's `navigator.storage`, where the browser offers one.
    pub fn storage_manager() -> Option<JsValue> {
        let window = web_sys::window()?;
        let navigator = js_sys::Reflect::get(&window, &JsValue::from_str("navigator")).ok()?;
        let storage = js_sys::Reflect::get(&navigator, &JsValue::from_str("storage")).ok()?;
        (!storage.is_null() && !storage.is_undefined()).then_some(storage)
    }

    /// Settles one of the storage manager's methods, or `Ok(None)` when the
    /// browser does not offer it. A failure says whether the call could not
    /// be made or did not succeed.
    pub async fn storage_call(
        storage: &JsValue,
        method: &str,
        timeout_ms: u32,
    ) -> Result<Option<JsValue>, String> {
        let Ok(callable) = js_sys::Reflect::get(storage, &JsValue::from_str(method)) else {
            return Ok(None);
        };
        if callable.is_null() || callable.is_undefined() {
            return Ok(None);
        }
        let returned =
            call(storage, method, &[]).map_err(|error| format!("could not be called: {error}"))?;
        await_promise(
            js_sys::Promise::resolve(&returned),
            &format!("browser StorageManager.{method}()"),
            timeout_ms,
        )
        .await
        .map(Some)
        .map_err(|error| format!("failed: {error}"))
    }

    /// Awaits a promise, or gives up on it after `timeout_ms`.
    pub async fn await_promise(
        promise: js_sys::Promise,
        operation: &str,
        timeout_ms: u32,
    ) -> Result<JsValue, String> {
        let race = js_sys::Array::new();
        race.push(&promise);
        race.push(&timeout(operation, timeout_ms));
        wasm_bindgen_futures::JsFuture::from(js_sys::Promise::race(&race))
            .await
            .map_err(js_error)
    }

    fn timeout(operation: &str, timeout_ms: u32) -> js_sys::Promise {
        let message = format!("{operation} timed out after {} seconds", timeout_ms / 1_000);
        js_sys::Promise::new(&mut move |_resolve, reject| {
            let message = message.clone();
            let expire = reject.clone();
            let callback = Closure::once_into_js(move || {
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
                &[callback, JsValue::from_f64(f64::from(timeout_ms))],
            ) {
                let _ = reject.call1(&JsValue::UNDEFINED, &JsValue::from_str(&error));
            }
        })
    }

    /// Calls `receiver.name(...arguments)`.
    pub fn call(receiver: &JsValue, name: &str, arguments: &[JsValue]) -> Result<JsValue, String> {
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

    /// The sentence a browser error carries.
    pub fn js_error(value: JsValue) -> String {
        js_sys::Reflect::get(&value, &JsValue::from_str("message"))
            .ok()
            .and_then(|message| message.as_string())
            .or_else(|| value.as_string())
            .unwrap_or_else(|| "the browser reported an error without a message".to_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A database's name, version and store are what its records were written
    /// under, so a change to any of them strands every user's records.
    #[test]
    fn every_database_keeps_the_identity_its_records_were_written_under() {
        let identity = |schema: Schema| (schema.database, schema.version, schema.store);
        assert_eq!(
            identity(MODEL_HUB),
            ("rspice-model-hub", 1, "verified-packs")
        );
        assert_eq!(
            identity(PDK_CONFIG),
            ("rspice-pdk-config", 1, "content-addressed-pdk-config")
        );
    }
}
