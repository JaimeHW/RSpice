//! The browser's durable records, as the session and a project's autosave
//! keep them: string keys over one store, each holding text or bytes.
//!
//! In the browser the records are IndexedDB's ([`IndexedDbRecords`]), whose
//! share of the disk is the origin's, not the few megabytes local storage
//! holds. A test keeps them in memory under an origin quota
//! ([`MemoryOrigin`]), so what happens when the browser's storage is full is
//! tried without a browser.
//!
//! Every operation is a future, and the work that encodes what is written runs
//! as a browser task of its own ([`spawn_task`]), after the frame that asked
//! for it has painted.

use std::future::Future;
use std::pin::Pin;
use std::rc::Rc;

use rspice_app_state::browser_store;

/// One record's value.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Record {
    Text(String),
    Bytes(Vec<u8>),
}

impl Record {
    /// The bytes the record takes in the store.
    pub fn stored_len(&self) -> usize {
        match self {
            Self::Text(text) => text.len(),
            Self::Bytes(bytes) => bytes.len(),
        }
    }

    pub fn into_text(self) -> Option<String> {
        match self {
            Self::Text(text) => Some(text),
            Self::Bytes(_) => None,
        }
    }

    pub fn into_bytes(self) -> Option<Vec<u8>> {
        match self {
            Self::Bytes(bytes) => Some(bytes),
            Self::Text(_) => None,
        }
    }
}

/// Why a record operation did not complete.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecordError {
    /// The browser refused the write for want of space.
    Full,
    Failed(String),
}

impl RecordError {
    /// A browser store's error, a refusal for want of space told apart.
    pub fn from_browser(error: String) -> Self {
        if browser_store::is_storage_full(&error) {
            Self::Full
        } else {
            Self::Failed(error)
        }
    }
}

impl std::fmt::Display for RecordError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Full => f.write_str(browser_store::STORAGE_FULL),
            Self::Failed(error) => f.write_str(error),
        }
    }
}

impl From<RecordError> for String {
    fn from(error: RecordError) -> Self {
        error.to_string()
    }
}

pub type RecordFuture<'a, T> = Pin<Box<dyn Future<Output = Result<T, RecordError>> + 'a>>;

/// A store of records under string keys.
pub trait DurableRecords {
    fn get<'a>(&'a self, key: &'a str) -> RecordFuture<'a, Option<Record>>;
    /// Writes `record` under `key`, replacing what was there.
    fn put<'a>(&'a self, key: &'a str, record: Record) -> RecordFuture<'a, ()>;
    fn delete<'a>(&'a self, key: &'a str) -> RecordFuture<'a, ()>;
    /// Every key in the store.
    fn keys(&self) -> RecordFuture<'_, Vec<String>>;
}

/// Where the browser keeps the session and the projects' checkpoints.
#[derive(Clone)]
pub struct BrowserRecords {
    /// The session and the runs it holds.
    pub session: Rc<dyn DurableRecords>,
    /// Each project's checkpoints, its autosave among them, beside the
    /// project bindings they belong with.
    pub checkpoints: Rc<dyn DurableRecords>,
}

impl std::fmt::Debug for BrowserRecords {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("BrowserRecords")
    }
}

/// The browser's records: always in the browser; natively, only those a
/// test keeps in memory, and otherwise none, since the desktop keeps its
/// session in a file and its checkpoints beside the project.
pub fn browser_records() -> Option<BrowserRecords> {
    #[cfg(target_arch = "wasm32")]
    {
        Some(BrowserRecords {
            session: Rc::new(IndexedDbRecords(&browser_store::SESSION)),
            checkpoints: Rc::new(IndexedDbRecords(&browser_store::PROJECT_BINDINGS)),
        })
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        KEPT_IN_MEMORY.with(|kept| kept.borrow().clone())
    }
}

#[cfg(not(target_arch = "wasm32"))]
thread_local! {
    static KEPT_IN_MEMORY: std::cell::RefCell<Option<BrowserRecords>> =
        const { std::cell::RefCell::new(None) };
}

/// Keep this thread's browser records in `records`, as a browser would, until
/// the returned guard drops.
#[cfg(all(any(test, feature = "test-support"), not(target_arch = "wasm32")))]
pub fn keep_browser_records_in(records: BrowserRecords) -> KeptInMemory {
    KEPT_IN_MEMORY.with(|kept| *kept.borrow_mut() = Some(records));
    KeptInMemory(())
}

#[cfg(all(any(test, feature = "test-support"), not(target_arch = "wasm32")))]
#[must_use = "the records are the browser's only while this is held"]
pub struct KeptInMemory(());

#[cfg(all(any(test, feature = "test-support"), not(target_arch = "wasm32")))]
impl Drop for KeptInMemory {
    fn drop(&mut self) {
        KEPT_IN_MEMORY.with(|kept| kept.borrow_mut().take());
        TASKS.with(|tasks| tasks.borrow_mut().clear());
    }
}

type Task = Pin<Box<dyn Future<Output = ()>>>;

#[cfg(not(target_arch = "wasm32"))]
thread_local! {
    /// The browser tasks spawned on this thread and not yet run.
    static TASKS: std::cell::RefCell<Vec<Task>> = const { std::cell::RefCell::new(Vec::new()) };
}

/// Run `work` as a browser task of its own, after the frame that asked for
/// it has painted, and paint again when it is done. Natively the task waits
/// for [`run_browser_tasks`], as a browser's would for its event loop.
pub fn spawn_task(ctx: Option<&egui::Context>, work: impl Future<Output = ()> + 'static) {
    let repaint = ctx.cloned();
    let task: Task = Box::pin(async move {
        work.await;
        if let Some(repaint) = repaint {
            repaint.request_repaint();
        }
    });
    #[cfg(target_arch = "wasm32")]
    wasm_bindgen_futures::spawn_local(async move {
        next_browser_task().await;
        task.await;
    });
    #[cfg(not(target_arch = "wasm32"))]
    TASKS.with(|tasks| tasks.borrow_mut().push(task));
}

/// Run every browser task spawned on this thread, and those they spawn, to
/// the end, as the browser's event loop would between frames. Returns how
/// many ran.
#[cfg(all(any(test, feature = "test-support"), not(target_arch = "wasm32")))]
pub fn run_browser_tasks() -> usize {
    let mut ran = 0;
    loop {
        let pending = TASKS.with(|tasks| std::mem::take(&mut *tasks.borrow_mut()));
        if pending.is_empty() {
            return ran;
        }
        for task in pending {
            block_on(task);
            ran += 1;
        }
    }
}

/// Drive a future whose every step is ready at once, as the in-memory
/// records' are, to its answer.
#[cfg(all(any(test, feature = "test-support"), not(target_arch = "wasm32")))]
pub fn block_on<T>(future: impl Future<Output = T>) -> T {
    let mut future = std::pin::pin!(future);
    let mut context = std::task::Context::from_waker(std::task::Waker::noop());
    match future.as_mut().poll(&mut context) {
        std::task::Poll::Ready(answer) => answer,
        std::task::Poll::Pending => panic!("an in-memory record operation waited"),
    }
}

/// Settles once the browser has run whatever else was queued, the frame that
/// asked included: a `setTimeout` of zero is a task of its own, where a
/// promise's continuation would run before the frame is shown.
#[cfg(target_arch = "wasm32")]
async fn next_browser_task() {
    let promise = js_sys::Promise::new(&mut |resolve, _reject| {
        let Some(window) = web_sys::window() else {
            let _ = resolve.call0(&wasm_bindgen::JsValue::UNDEFINED);
            return;
        };
        if window
            .set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, 0)
            .is_err()
        {
            let _ = resolve.call0(&wasm_bindgen::JsValue::UNDEFINED);
        }
    });
    let _ = wasm_bindgen_futures::JsFuture::from(promise).await;
}

/// IndexedDB, one connection per operation, as the rest of the browser's
/// stores open it.
#[cfg(target_arch = "wasm32")]
pub struct IndexedDbRecords(pub &'static browser_store::Schema);

#[cfg(target_arch = "wasm32")]
impl DurableRecords for IndexedDbRecords {
    fn get<'a>(&'a self, key: &'a str) -> RecordFuture<'a, Option<Record>> {
        Box::pin(async move {
            let database = browser_store::open(self.0)
                .await
                .map_err(RecordError::from_browser)?;
            let value = database
                .read()
                .map_err(RecordError::from_browser)?
                .get(key)
                .await
                .map_err(RecordError::from_browser)?;
            Ok(value.and_then(|value| {
                value
                    .as_string()
                    .map(Record::Text)
                    .or_else(|| browser_store::bytes(&value).map(Record::Bytes))
            }))
        })
    }

    fn put<'a>(&'a self, key: &'a str, record: Record) -> RecordFuture<'a, ()> {
        Box::pin(async move {
            let value = match &record {
                Record::Text(text) => wasm_bindgen::JsValue::from_str(text),
                Record::Bytes(bytes) => js_sys::Uint8Array::from(bytes.as_slice()).into(),
            };
            let database = browser_store::open(self.0)
                .await
                .map_err(RecordError::from_browser)?;
            let transaction = database.write().map_err(RecordError::from_browser)?;
            transaction
                .put(key, &value)
                .await
                .map_err(RecordError::from_browser)?;
            transaction
                .commit()
                .await
                .map_err(RecordError::from_browser)
        })
    }

    fn delete<'a>(&'a self, key: &'a str) -> RecordFuture<'a, ()> {
        Box::pin(async move {
            let database = browser_store::open(self.0)
                .await
                .map_err(RecordError::from_browser)?;
            let transaction = database.write().map_err(RecordError::from_browser)?;
            transaction
                .delete(key)
                .await
                .map_err(RecordError::from_browser)?;
            transaction
                .commit()
                .await
                .map_err(RecordError::from_browser)
        })
    }

    fn keys(&self) -> RecordFuture<'_, Vec<String>> {
        Box::pin(async move {
            let database = browser_store::open(self.0)
                .await
                .map_err(RecordError::from_browser)?;
            database
                .read()
                .map_err(RecordError::from_browser)?
                .keys()
                .await
                .map_err(RecordError::from_browser)
        })
    }
}

/// An origin's storage in memory: the records of each of its stores count
/// against one quota, as a browser's do.
#[cfg(any(test, feature = "test-support"))]
#[derive(Debug, Default)]
pub struct MemoryOrigin {
    quota: std::cell::Cell<Option<usize>>,
    used: std::cell::Cell<usize>,
}

#[cfg(any(test, feature = "test-support"))]
impl MemoryOrigin {
    /// An origin that holds at most `quota` bytes, or anything with `None`.
    pub fn new(quota: Option<usize>) -> Rc<Self> {
        Rc::new(Self {
            quota: std::cell::Cell::new(quota),
            used: std::cell::Cell::new(0),
        })
    }

    pub fn set_quota(&self, quota: Option<usize>) {
        self.quota.set(quota);
    }

    /// The bytes its records hold.
    pub fn used(&self) -> usize {
        self.used.get()
    }

    /// One store of this origin, empty.
    pub fn store(self: &Rc<Self>) -> Rc<MemoryRecords> {
        Rc::new(MemoryRecords {
            origin: Rc::clone(self),
            records: std::cell::RefCell::default(),
        })
    }
}

/// One store kept in memory.
#[cfg(any(test, feature = "test-support"))]
#[derive(Debug)]
pub struct MemoryRecords {
    origin: Rc<MemoryOrigin>,
    records: std::cell::RefCell<std::collections::BTreeMap<String, Record>>,
}

#[cfg(any(test, feature = "test-support"))]
impl MemoryRecords {
    /// The record under `key`, read at once.
    pub fn record(&self, key: &str) -> Option<Record> {
        self.records.borrow().get(key).cloned()
    }

    /// Every key, read at once.
    pub fn stored_keys(&self) -> Vec<String> {
        self.records.borrow().keys().cloned().collect()
    }
}

#[cfg(any(test, feature = "test-support"))]
impl DurableRecords for MemoryRecords {
    fn get<'a>(&'a self, key: &'a str) -> RecordFuture<'a, Option<Record>> {
        Box::pin(std::future::ready(Ok(self.record(key))))
    }

    fn put<'a>(&'a self, key: &'a str, record: Record) -> RecordFuture<'a, ()> {
        let replaced = self.records.borrow().get(key).map_or(0, Record::stored_len);
        let used = self.origin.used.get() - replaced + record.stored_len();
        let answer = if self.origin.quota.get().is_some_and(|quota| used > quota) {
            Err(RecordError::Full)
        } else {
            self.origin.used.set(used);
            self.records.borrow_mut().insert(key.to_owned(), record);
            Ok(())
        };
        Box::pin(std::future::ready(answer))
    }

    fn delete<'a>(&'a self, key: &'a str) -> RecordFuture<'a, ()> {
        if let Some(removed) = self.records.borrow_mut().remove(key) {
            self.origin
                .used
                .set(self.origin.used.get() - removed.stored_len());
        }
        Box::pin(std::future::ready(Ok(())))
    }

    fn keys(&self) -> RecordFuture<'_, Vec<String>> {
        Box::pin(std::future::ready(Ok(self.stored_keys())))
    }
}
