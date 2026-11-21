//! The results stores of this browser, kept in its origin-private file system and reached here,
//! in a dedicated worker, through synchronous access handles.
//!
//! A tab holds one store at a time, the store of the project it has open, under a Web Lock named
//! for that store: another tab of this browser that opens the same project is told the store is
//! held elsewhere and writes nothing to it. A segment the page holds is written here from its
//! rows as a member of the store, and a store that fills stops the write: the segment closes on
//! the whole chunks it holds, its `END` saying the volume filled, and the page keeps that many
//! rows. A kept segment is checked against its name before its bytes go back to a page.

use std::cell::RefCell;
use std::collections::BTreeSet;
use std::sync::Arc;

use js_sys::{Array, Float64Array, Reflect, Uint8Array};
use rspice_results_store::{
    Digest, FreeSpace, Manifest, Member, OpfsIo, OpfsStoreDir, RunStatus, SegmentIo, SegmentReader,
    SegmentState, SegmentWriter, StopReason, StoreError, WatchedIo,
};
use rspice_simulation_contract::worker_transport::{
    KeptSegmentRecord, ResultsStoreRequest, ResultsStoreResponse, ResultsStoreStanding,
    WorkerResultsSchema,
};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{FileSystemDirectoryHandle, FileSystemSyncAccessHandle};

/// The folder of the origin-private file system that holds every project's store.
const STORES: &str = "rspice-results";

type Handle = OpfsIo<FileSystemSyncAccessHandle>;

/// A segment being written from the rows a page sends.
struct Write {
    index: u32,
    analysis: String,
    /// The logical digest of the segment the page holds, which a write that ran to its end
    /// must have.
    digest: Digest,
    writer: SegmentWriter<WatchedIo<Handle>>,
    /// The store filled: the write takes no more rows.
    full: bool,
}

/// The store this tab holds.
struct Store {
    name: String,
    dir: OpfsStoreDir,
    manifest: Manifest,
    _lock: HeldLock,
    writes: Vec<(u32, Write)>,
    next_write: u32,
    /// The kept segments found to hold what their names state, while this tab holds the store.
    checked: BTreeSet<[u8; 32]>,
    /// The kept segment being read, held open between its chunks.
    reading: Option<(Digest, Handle)>,
}

thread_local! {
    static STORE: RefCell<Option<Store>> = const { RefCell::new(None) };
}

/// Answer one request from the page: its JSON and the numeric buffers beside it. The worker's
/// script hands requests over one at a time.
pub(crate) async fn run_request(request: String, buffers: Array) -> Result<JsValue, JsValue> {
    let request: ResultsStoreRequest = serde_json::from_str(&request)
        .map_err(|error| JsValue::from_str(&format!("malformed results store request: {error}")))?;
    let mut store = STORE.with(|held| held.borrow_mut().take());
    let answer = answer(&mut store, request, &buffers).await;
    STORE.with(|held| *held.borrow_mut() = store);
    let (response, out) =
        answer.unwrap_or_else(|reason| (ResultsStoreResponse::Failed { reason }, Array::new()));
    let object = js_sys::Object::new();
    let response =
        serde_json::to_string(&response).map_err(|error| JsValue::from_str(&error.to_string()))?;
    Reflect::set(&object, &"response".into(), &response.into())?;
    Reflect::set(&object, &"buffers".into(), &out)?;
    Ok(object.into())
}

async fn answer(
    store: &mut Option<Store>,
    request: ResultsStoreRequest,
    buffers: &Array,
) -> Result<(ResultsStoreResponse, Array), String> {
    let none = Array::new;
    let response = match request {
        ResultsStoreRequest::Open { store: name } => open(store, name).await?,
        ResultsStoreRequest::Close => {
            store.take();
            ResultsStoreResponse::Closed
        }
        ResultsStoreRequest::Begin { schema, digest } => {
            begin(held(store)?, &schema, &digest, Arc::new(|| None)).await?
        }
        ResultsStoreRequest::Rows { write } => {
            let x = f64_buffer(buffers, 0)?;
            let values = f64_buffer(buffers, 1)?;
            rows(held(store)?, write, &x, &values).await?
        }
        ResultsStoreRequest::Finish { write } => finish(held(store)?, write).await?,
        ResultsStoreRequest::Abandon { write } => {
            let store = held(store)?;
            if let Some(write) = take_write(store, write) {
                abandon(store, write).await;
            }
            ResultsStoreResponse::Abandoned
        }
        ResultsStoreRequest::Check { digest } => check(held(store)?, &digest).await?,
        ResultsStoreRequest::Read {
            digest,
            offset,
            len,
        } => {
            let bytes = read(held(store)?, &digest, offset, len).await?;
            let out = Array::new();
            out.push(&Uint8Array::from(bytes.as_slice()));
            return Ok((ResultsStoreResponse::Bytes, out));
        }
        ResultsStoreRequest::Keep { digests } => keep(held(store)?, &digests).await?,
    };
    Ok((response, none()))
}

fn held(store: &mut Option<Store>) -> Result<&mut Store, String> {
    store
        .as_mut()
        .ok_or_else(|| "this tab holds no results store".to_owned())
}

fn f64_buffer(buffers: &Array, index: u32) -> Result<Vec<f64>, String> {
    buffers
        .get(index)
        .dyn_into::<Float64Array>()
        .map(|values| values.to_vec())
        .map_err(|_| format!("rows buffer {index} is not a Float64Array"))
}

fn hex(digest: &Digest) -> String {
    digest.to_string()
}

fn digest_of(hex: &str) -> Result<Digest, String> {
    let bytes = hex.as_bytes();
    if bytes.len() != 64 {
        return Err(format!("'{hex}' is not a results store's digest"));
    }
    let mut digest = [0_u8; 32];
    for (byte, pair) in digest.iter_mut().zip(bytes.chunks(2)) {
        let pair = std::str::from_utf8(pair).map_err(|error| error.to_string())?;
        *byte = u8::from_str_radix(pair, 16)
            .map_err(|_| format!("'{hex}' is not a results store's digest"))?;
    }
    Ok(Digest(digest))
}

/// Each segment `store` keeps, with its length: the page states what the store keeps from these.
async fn records(store: &Store) -> Vec<KeptSegmentRecord> {
    let mut records = Vec::new();
    for member in store.manifest.members() {
        let (Some(digest), Some(_)) = (member.digest.as_ref(), member.status.as_ref()) else {
            continue;
        };
        let bytes = match &store.reading {
            Some((open, io)) if open == digest => io.size().ok(),
            _ => match store.dir.segment(member.index, false).await {
                Ok(io) => io.size().ok(),
                Err(_) => None,
            },
        };
        records.push(KeptSegmentRecord {
            digest: hex(digest),
            bytes: bytes.unwrap_or(0),
            rows: 0,
        });
    }
    records
}

/// A store's name is a project's id: letters, digits and dashes, and never a path.
fn valid_name(name: &str) -> bool {
    !name.is_empty()
        && name.len() <= 64
        && name
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || byte == b'-')
}

async fn open(store: &mut Option<Store>, name: String) -> Result<ResultsStoreResponse, String> {
    if let Some(open) = store.as_ref()
        && open.name == name
    {
        return Ok(opened(open).await);
    }
    store.take();
    if !valid_name(&name) {
        return Err(format!("'{name}' cannot name a results store"));
    }
    let unavailable = |reason: String| ResultsStoreResponse::Opened {
        standing: ResultsStoreStanding::Unavailable { reason },
        segments: Vec::new(),
    };
    let root = match storage_root().await {
        Ok(root) => root,
        Err(reason) => return Ok(unavailable(reason)),
    };
    if !sync_access_offered() {
        return Ok(unavailable(
            "this browser cannot write its origin-private files from a worker".to_owned(),
        ));
    }
    let lock = match try_lock(&format!("rspice-results-store-{name}")).await {
        Ok(Some(lock)) => lock,
        Ok(None) => {
            return Ok(ResultsStoreResponse::Opened {
                standing: ResultsStoreStanding::HeldElsewhere,
                segments: Vec::new(),
            });
        }
        Err(reason) => return Ok(unavailable(reason)),
    };
    let stores = directory(&root, STORES).await?;
    let dir = OpfsStoreDir::open(&stores, &name, true)
        .await
        .map_err(|error| format!("the results store could not be opened: {error}"))?;
    let manifest = dir
        .load_manifest()
        .await
        .map_err(|error| format!("the results store's manifest could not be read: {error}"))?
        .unwrap_or_default();
    let open = store.insert(Store {
        name,
        dir,
        manifest,
        _lock: lock,
        writes: Vec::new(),
        next_write: 1,
        checked: BTreeSet::new(),
        reading: None,
    });
    Ok(opened(open).await)
}

async fn opened(store: &Store) -> ResultsStoreResponse {
    ResultsStoreResponse::Opened {
        standing: ResultsStoreStanding::Held,
        segments: records(store).await,
    }
}

/// Start writing a segment of `schema`, held by the page as `digest`; `free` says what room the
/// store has, `None` where the browser cannot say, so the write holds room for closing ahead.
async fn begin(
    store: &mut Store,
    schema: &WorkerResultsSchema,
    digest: &str,
    free: FreeSpace,
) -> Result<ResultsStoreResponse, String> {
    let digest = digest_of(digest)?;
    if store
        .manifest
        .members()
        .iter()
        .any(|member| member.digest == Some(digest) && member.status == Some(RunStatus::Completed))
    {
        return Ok(ResultsStoreResponse::Finished {
            record: KeptSegmentRecord {
                digest: hex(&digest),
                bytes: 0,
                rows: 0,
            },
            full: false,
        });
    }
    let index = store
        .manifest
        .members()
        .iter()
        .map(|member| member.index)
        .chain(store.writes.iter().map(|(_, write)| write.index))
        .max()
        .map_or(0, |index| index + 1);
    let (schema, meta) = schema.store();
    let io = store
        .dir
        .segment(index, true)
        .await
        .map_err(|error| format!("the results store could not make a segment: {error}"))?;
    let writer = match SegmentWriter::create(WatchedIo::new(io, free), &schema, &meta) {
        Ok(writer) => writer,
        Err(error) => {
            let _ = store.dir.remove_segment(index).await;
            return Err(error.to_string());
        }
    };
    let write = store.next_write;
    store.next_write += 1;
    store.writes.push((
        write,
        Write {
            index,
            analysis: meta.analysis,
            digest,
            writer,
            full: false,
        },
    ));
    Ok(ResultsStoreResponse::Begun { write })
}

fn take_write(store: &mut Store, write: u32) -> Option<Write> {
    let at = store.writes.iter().position(|(id, _)| *id == write)?;
    Some(store.writes.remove(at).1)
}

async fn abandon(store: &mut Store, write: Write) {
    let index = write.index;
    drop(write);
    let _ = store.dir.remove_segment(index).await;
}

async fn rows(
    store: &mut Store,
    write: u32,
    x: &[f64],
    values: &[f64],
) -> Result<ResultsStoreResponse, String> {
    let Some((_, open)) = store.writes.iter_mut().find(|(id, _)| *id == write) else {
        return Err(format!("no segment is being written as {write}"));
    };
    if open.full {
        return Ok(ResultsStoreResponse::Taken { full: true });
    }
    match open.writer.push_rows(x, values) {
        Ok(()) => Ok(ResultsStoreResponse::Taken { full: false }),
        Err(StoreError::DiskFull) => {
            open.full = true;
            Ok(ResultsStoreResponse::Taken { full: true })
        }
        Err(error) => {
            if let Some(write) = take_write(store, write) {
                abandon(store, write).await;
            }
            Err(error.to_string())
        }
    }
}

async fn finish(store: &mut Store, write: u32) -> Result<ResultsStoreResponse, String> {
    let write = take_write(store, write).ok_or_else(|| format!("no write {write}"))?;
    let Write {
        index,
        analysis,
        digest,
        writer,
        full,
    } = write;
    let status = if full {
        RunStatus::Stopped(StopReason::DiskFull)
    } else {
        RunStatus::Completed
    };
    let receipt = match writer.finish(status.clone()) {
        Ok((receipt, io)) => {
            drop(io);
            receipt
        }
        Err(error) => {
            let _ = store.dir.remove_segment(index).await;
            return Err(error.to_string());
        }
    };
    if !full && receipt.digest != digest {
        let _ = store.dir.remove_segment(index).await;
        return Err(format!(
            "the segment written holds {}, not the {} the page holds",
            hex(&receipt.digest),
            hex(&digest)
        ));
    }
    let mut manifest = store.manifest.clone();
    manifest.upsert(Member {
        index,
        segment: OpfsStoreDir::segment_name(index),
        analysis,
        status: Some(status),
        digest: Some(receipt.digest),
        ..Member::default()
    });
    if let Err(error) = store.dir.save_manifest(&manifest).await {
        let _ = store.dir.remove_segment(index).await;
        return Err(format!(
            "the results store's manifest could not be saved: {error}"
        ));
    }
    store.manifest = manifest;
    store.checked.insert(receipt.digest.0);
    Ok(ResultsStoreResponse::Finished {
        record: KeptSegmentRecord {
            digest: hex(&receipt.digest),
            bytes: receipt.bytes,
            rows: receipt.rows,
        },
        full,
    })
}

fn member_of(store: &Store, digest: &Digest) -> Result<u32, String> {
    store
        .manifest
        .members()
        .iter()
        .find(|member| member.digest.as_ref() == Some(digest) && member.status.is_some())
        .map(|member| member.index)
        .ok_or_else(|| "this browser's results store does not hold it".to_owned())
}

/// The kept segment `digest`, opened for reading and held open until another is read.
async fn reading<'a>(store: &'a mut Store, digest: &Digest) -> Result<&'a Handle, String> {
    if store
        .reading
        .as_ref()
        .is_none_or(|(open, _)| open != digest)
    {
        store.reading = None;
        let index = member_of(store, digest)?;
        let io =
            store.dir.segment(index, false).await.map_err(|error| {
                format!("this browser's results store could not open it: {error}")
            })?;
        store.reading = Some((*digest, io));
    }
    Ok(&store.reading.as_ref().expect("opened above").1)
}

async fn check(store: &mut Store, digest: &str) -> Result<ResultsStoreResponse, String> {
    let digest = digest_of(digest)?;
    let len = {
        let io = reading(store, &digest).await?;
        io.size().map_err(|error| error.to_string())?
    };
    if !store.checked.contains(&digest.0) {
        let (_, io) = store.reading.take().expect("opened above");
        let mut reader = SegmentReader::open(io).map_err(|error| error.to_string())?;
        let SegmentState::Complete {
            digest: recorded, ..
        } = reader.state().clone()
        else {
            return Err("it was never closed: the end its run writes is missing".to_owned());
        };
        let held = reader.digest().map_err(|error| error.to_string())?;
        if held != recorded || held != digest {
            return Err("its rows are not the ones its name states".to_owned());
        }
        store.checked.insert(digest.0);
    }
    Ok(ResultsStoreResponse::Checked { len })
}

async fn read(store: &mut Store, digest: &str, offset: u64, len: u64) -> Result<Vec<u8>, String> {
    let digest = digest_of(digest)?;
    if !store.checked.contains(&digest.0) {
        return Err("a kept segment is read only once it is checked".to_owned());
    }
    let len = usize::try_from(len).map_err(|_| "the read is longer than this memory".to_owned())?;
    let mut bytes = Vec::new();
    bytes
        .try_reserve_exact(len)
        .map_err(|error| format!("{len} bytes could not be held to read: {error}"))?;
    bytes.resize(len, 0);
    reading(store, &digest)
        .await?
        .read_at(offset, &mut bytes)
        .map_err(|error| error.to_string())?;
    Ok(bytes)
}

async fn keep(store: &mut Store, digests: &[String]) -> Result<ResultsStoreResponse, String> {
    let kept: BTreeSet<[u8; 32]> = digests
        .iter()
        .map(|digest| digest_of(digest).map(|digest| digest.0))
        .collect::<Result<_, _>>()?;
    let mut manifest = Manifest::default();
    for member in store.manifest.members() {
        if member.digest.is_some_and(|digest| kept.contains(&digest.0)) {
            manifest.upsert(member.clone());
        }
    }
    if manifest != store.manifest {
        store
            .dir
            .save_manifest(&manifest)
            .await
            .map_err(|error| format!("the results store's manifest could not be saved: {error}"))?;
        store.manifest = manifest;
    }
    store.reading = None;
    let live: BTreeSet<u32> = store
        .manifest
        .members()
        .iter()
        .map(|member| member.index)
        .chain(store.writes.iter().map(|(_, write)| write.index))
        .collect();
    let mut removed = 0;
    let indices = store
        .dir
        .segment_indices()
        .await
        .map_err(|error| error.to_string())?;
    for index in indices.into_iter().filter(|index| !live.contains(index)) {
        if store.dir.remove_segment(index).await.is_ok() {
            removed += 1;
        }
    }
    store.checked.retain(|digest| kept.contains(digest));
    Ok(ResultsStoreResponse::Kept { removed })
}

fn navigator() -> Result<JsValue, String> {
    Reflect::get(&js_sys::global(), &"navigator".into())
        .ok()
        .filter(|navigator| navigator.is_object())
        .ok_or_else(|| "this worker has no navigator".to_owned())
}

async fn awaited(promise: JsValue, what: &str) -> Result<JsValue, String> {
    let promise = promise
        .dyn_into::<js_sys::Promise>()
        .map_err(|_| format!("{what} did not answer a promise"))?;
    wasm_bindgen_futures::JsFuture::from(promise)
        .await
        .map_err(|error| format!("{what}: {}", js_message(&error)))
}

fn js_message(error: &JsValue) -> String {
    Reflect::get(error, &"message".into())
        .ok()
        .and_then(|message| message.as_string())
        .or_else(|| error.as_string())
        .unwrap_or_else(|| format!("{error:?}"))
}

/// The root of this origin's private file system, or why there is none.
async fn storage_root() -> Result<FileSystemDirectoryHandle, String> {
    let storage = Reflect::get(&navigator()?, &"storage".into())
        .ok()
        .filter(|storage| storage.is_object())
        .ok_or_else(|| "this browser offers no storage to a worker".to_owned())?;
    let get = Reflect::get(&storage, &"getDirectory".into())
        .ok()
        .and_then(|get| get.dyn_into::<js_sys::Function>().ok())
        .ok_or_else(|| "this browser has no origin-private file system".to_owned())?;
    let promise = get
        .call0(&storage)
        .map_err(|error| format!("the origin-private file system: {}", js_message(&error)))?;
    awaited(promise, "the origin-private file system")
        .await?
        .dyn_into::<FileSystemDirectoryHandle>()
        .map_err(|_| "the origin-private file system answered another type".to_owned())
}

/// Whether a file handle here can open a synchronous access handle.
fn sync_access_offered() -> bool {
    let prototype = Reflect::get(&js_sys::global(), &"FileSystemFileHandle".into())
        .and_then(|class| Reflect::get(&class, &"prototype".into()));
    prototype
        .and_then(|prototype| Reflect::get(&prototype, &"createSyncAccessHandle".into()))
        .is_ok_and(|create| create.is_function())
}

async fn directory(
    parent: &FileSystemDirectoryHandle,
    name: &str,
) -> Result<FileSystemDirectoryHandle, String> {
    let options = web_sys::FileSystemGetDirectoryOptions::new();
    options.set_create(true);
    awaited(
        parent
            .get_directory_handle_with_options(name, &options)
            .into(),
        "the results stores' folder",
    )
    .await?
    .dyn_into::<FileSystemDirectoryHandle>()
    .map_err(|_| "the results stores' folder answered another type".to_owned())
}

/// A Web Lock this worker holds until it is dropped, or the worker ends.
struct HeldLock {
    release: js_sys::Function,
    _callback: Closure<dyn FnMut(JsValue) -> JsValue>,
}

impl Drop for HeldLock {
    fn drop(&mut self) {
        let _ = self.release.call0(&JsValue::UNDEFINED);
    }
}

/// The exclusive lock `name`, when no tab or worker of this browser holds it.
async fn try_lock(name: &str) -> Result<Option<HeldLock>, String> {
    let locks = Reflect::get(&navigator()?, &"locks".into())
        .ok()
        .filter(|locks| locks.is_object())
        .ok_or_else(|| "this browser cannot tell its tabs apart: it has no Web Locks".to_owned())?;
    let request = Reflect::get(&locks, &"request".into())
        .ok()
        .and_then(|request| request.dyn_into::<js_sys::Function>().ok())
        .ok_or_else(|| "this browser cannot tell its tabs apart: it has no Web Locks".to_owned())?;
    let mut release = None;
    let held = js_sys::Promise::new(&mut |resolve, _| release = Some(resolve));
    let release = release.ok_or("a lock's release could not be made")?;
    let answered = std::rc::Rc::new(RefCell::new(None::<js_sys::Function>));
    let answer = answered.clone();
    let acquired = js_sys::Promise::new(&mut move |resolve, _| {
        *answer.borrow_mut() = Some(resolve);
    });
    let callback = Closure::<dyn FnMut(JsValue) -> JsValue>::new(move |lock: JsValue| {
        let got = !lock.is_null() && !lock.is_undefined();
        if let Some(resolve) = answered.borrow_mut().take() {
            let _ = resolve.call1(&JsValue::UNDEFINED, &JsValue::from_bool(got));
        }
        if got {
            held.clone().into()
        } else {
            JsValue::UNDEFINED
        }
    });
    let options = js_sys::Object::new();
    Reflect::set(&options, &"mode".into(), &"exclusive".into())
        .map_err(|error| js_message(&error))?;
    Reflect::set(&options, &"ifAvailable".into(), &JsValue::TRUE)
        .map_err(|error| js_message(&error))?;
    request
        .call3(
            &locks,
            &JsValue::from_str(name),
            &options,
            callback.as_ref().unchecked_ref(),
        )
        .map_err(|error| format!("the store's lock: {}", js_message(&error)))?;
    let got = awaited(acquired.into(), "the store's lock")
        .await?
        .as_bool()
        .unwrap_or(false);
    if !got {
        let _ = release.call0(&JsValue::UNDEFINED);
        return Ok(None);
    }
    Ok(Some(HeldLock {
        release,
        _callback: callback,
    }))
}
