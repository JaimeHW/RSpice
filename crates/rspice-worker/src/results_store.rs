//! The results stores of this browser, kept in its origin-private file system and reached here,
//! in a dedicated worker, through synchronous access handles.
//!
//! A tab holds one store at a time, the store of the project it has open, under a Web Lock named
//! for that store: another tab of this browser that opens the same project is told the store is
//! held elsewhere and writes nothing to it. A segment the page holds is written here from its
//! rows as a member of the store, and a store that fills stops the write: the segment closes on
//! the whole chunks it holds, its `END` saying the volume filled, and the page keeps that many
//! rows. A kept segment is checked against its name before a page reads it: a view's reads of
//! it are made here ([`ResultsStoreRequest::Series`]), and the page asks for its bytes only for
//! what it reads itself.

use std::cell::{Cell, RefCell};
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::sync::Arc;

use js_sys::{Array, Float64Array, Reflect, Uint8Array};
use rspice_results::store_reader::{Segment, SegmentBytes, SegmentColumn, StoreReader};
use rspice_results::waveform::{
    ColumnTile, Read, ReadValue, RetainedWaveform, StoredWindow, WaveformValues, WindowLead,
};
use rspice_results_store::{
    Digest, FreeSpace, Manifest, Member, OpfsIo, OpfsStoreDir, RunStatus, SegmentIo, SegmentReader,
    SegmentState, SegmentWriter, StopReason, StoreError, WatchedIo,
};
use rspice_simulation_contract::worker_transport::{
    KeptSegmentRecord, ResultsStoreRequest, ResultsStoreResponse, ResultsStoreStanding,
    WorkerColumn, WorkerComparedRun, WorkerComparison, WorkerF64, WorkerLead, WorkerRead,
    WorkerReadValue, WorkerResultsSchema, WorkerWindow,
};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{FileSystemDirectoryHandle, FileSystemSyncAccessHandle};

/// The folder of the origin-private file system that holds every project's store.
const STORES: &str = "rspice-results";

/// The bytes of a segment one step of a copy moves.
const COPY_PIECE: u64 = 8 << 20;

type Handle = OpfsIo<FileSystemSyncAccessHandle>;

/// A segment being written from the rows a page sends.
struct Write {
    index: u32,
    analysis: String,
    /// The logical digest of the segment the page holds, which a write that ran to its end
    /// must have; none for rows the page relays from the import worker.
    digest: Option<Digest>,
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
    /// The checked segments a page has read, held open while this tab holds the store: each
    /// with the key of its file.
    open: BTreeMap<[u8; 32], (u32, Arc<Segment>)>,
}

thread_local! {
    static STORE: RefCell<Option<Store>> = const { RefCell::new(None) };
    /// The files of the open segments, by key.
    static FILES: RefCell<HashMap<u32, Handle>> = RefCell::new(HashMap::new());
    static NEXT_FILE: Cell<u32> = const { Cell::new(1) };
}

/// A kept segment's file as its reader reads it: named by a key, so the reader can be held where
/// a reader must be sendable, while the file's handle stays in this worker. Dropping it closes
/// the file.
struct WorkerFile(u32);

impl WorkerFile {
    fn hold(handle: Handle) -> Self {
        let key = NEXT_FILE.with(|next| {
            let key = next.get();
            next.set(key.wrapping_add(1).max(1));
            key
        });
        FILES.with(|files| files.borrow_mut().insert(key, handle));
        Self(key)
    }
}

/// Run `act` with the file `key` names.
fn with_file<T>(key: u32, act: impl FnOnce(&Handle) -> std::io::Result<T>) -> std::io::Result<T> {
    FILES.with(|files| match files.borrow().get(&key) {
        Some(handle) => act(handle),
        None => Err(std::io::Error::other("the segment's file was closed")),
    })
}

fn read_only() -> std::io::Error {
    std::io::Error::new(
        std::io::ErrorKind::PermissionDenied,
        "a kept segment is only read",
    )
}

impl SegmentIo for WorkerFile {
    fn read_at(&self, offset: u64, buf: &mut [u8]) -> std::io::Result<()> {
        with_file(self.0, |handle| handle.read_at(offset, buf))
    }

    fn write_at(&mut self, _offset: u64, _data: &[u8]) -> std::io::Result<()> {
        Err(read_only())
    }

    fn truncate(&mut self, _len: u64) -> std::io::Result<()> {
        Err(read_only())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }

    fn size(&self) -> std::io::Result<u64> {
        with_file(self.0, SegmentIo::size)
    }
}

impl Drop for WorkerFile {
    fn drop(&mut self) {
        FILES.with(|files| files.borrow_mut().remove(&self.0));
    }
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
            begin(held(store)?, &schema, digest.as_deref(), Arc::new(|| None)).await?
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
        ResultsStoreRequest::Adopt { digests } => adopt(held(store)?, &digests).await?,
        ResultsStoreRequest::Sweep { live } => sweep(store.as_ref(), &live).await?,
        ResultsStoreRequest::Series {
            digest,
            x,
            values,
            read,
        } => return series(held(store)?, &digest, x, values, read).await,
        ResultsStoreRequest::Compare { comparison } => {
            compare(held(store)?, *comparison, buffers).await?
        }
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
        let bytes = match store.open.get(&digest.0) {
            Some((key, _)) => with_file(*key, SegmentIo::size).ok(),
            None => match store.dir.segment(member.index, false).await {
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
        open: BTreeMap::new(),
    });
    Ok(opened(open).await)
}

async fn opened(store: &Store) -> ResultsStoreResponse {
    ResultsStoreResponse::Opened {
        standing: ResultsStoreStanding::Held,
        segments: records(store).await,
    }
}

/// Start writing a segment of `schema`, held by the page as `digest` (none for rows the page
/// relays and does not hold); `free` says what room the store has, `None` where the browser
/// cannot say, so the write holds room for closing ahead.
async fn begin(
    store: &mut Store,
    schema: &WorkerResultsSchema,
    digest: Option<&str>,
    free: FreeSpace,
) -> Result<ResultsStoreResponse, String> {
    let digest = digest.map(digest_of).transpose()?;
    if let Some(digest) = digest
        && store.manifest.members().iter().any(|member| {
            member.digest == Some(digest) && member.status == Some(RunStatus::Completed)
        })
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
    // A store with no room for the segment's start, or for what it keeps to close it, keeps none
    // of it, and says so rather than failing: the page states the run is not kept.
    let io = match store.dir.segment(index, true).await {
        Ok(io) => io,
        Err(error)
            if matches!(
                error.kind(),
                std::io::ErrorKind::StorageFull | std::io::ErrorKind::QuotaExceeded
            ) =>
        {
            return Ok(ResultsStoreResponse::Full);
        }
        Err(error) => {
            return Err(format!(
                "the results store could not make a segment: {error}"
            ));
        }
    };
    let writer = match SegmentWriter::create(WatchedIo::new(io, free), &schema, &meta) {
        Ok(writer) => writer,
        Err(error) => {
            let _ = store.dir.remove_segment(index).await;
            if matches!(error, StoreError::DiskFull) {
                return Ok(ResultsStoreResponse::Full);
            }
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
    if let Some(digest) = digest
        && !full
        && receipt.digest != digest
    {
        let _ = store.dir.remove_segment(index).await;
        return Err(format!(
            "the segment written holds {}, not the {} the page holds",
            hex(&receipt.digest),
            hex(&digest)
        ));
    }
    // Relayed rows whose segment the store keeps already are dropped for it: the store keeps a
    // run's samples once.
    if digest.is_none()
        && !full
        && store.manifest.members().iter().any(|member| {
            member.digest == Some(receipt.digest) && member.status == Some(RunStatus::Completed)
        })
    {
        let _ = store.dir.remove_segment(index).await;
        return Ok(ResultsStoreResponse::Finished {
            record: KeptSegmentRecord {
                digest: hex(&receipt.digest),
                bytes: receipt.bytes,
                rows: receipt.rows,
            },
            full,
        });
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

/// The kept segment `digest`, opened for reading and held open while this tab holds the store:
/// checked against its name first, unless it was checked already.
async fn opened_segment<'a>(
    store: &'a mut Store,
    digest: &Digest,
) -> Result<&'a (u32, Arc<Segment>), String> {
    if !store.open.contains_key(&digest.0) {
        let index = member_of(store, digest)?;
        let io =
            store.dir.segment(index, false).await.map_err(|error| {
                format!("this browser's results store could not open it: {error}")
            })?;
        let file = WorkerFile::hold(io);
        let key = file.0;
        let mut reader = SegmentReader::open(Box::new(file) as SegmentBytes)
            .map_err(|error| error.to_string())?;
        if !store.checked.contains(&digest.0) {
            let SegmentState::Complete {
                digest: recorded, ..
            } = reader.state().clone()
            else {
                return Err("it was never closed: the end its run writes is missing".to_owned());
            };
            let held = reader.digest().map_err(|error| error.to_string())?;
            if held != recorded || held != *digest {
                return Err("its rows are not the ones its name states".to_owned());
            }
            store.checked.insert(digest.0);
        }
        let segment = Arc::new(Segment::kept(
            StoreReader::new(reader),
            rspice_results_store::kept_segment_name(digest).into(),
        ));
        store.open.insert(digest.0, (key, segment));
    }
    Ok(store.open.get(&digest.0).expect("opened above"))
}

async fn check(store: &mut Store, digest: &str) -> Result<ResultsStoreResponse, String> {
    let digest = digest_of(digest)?;
    let (key, _) = opened_segment(store, &digest).await?;
    let len = with_file(*key, SegmentIo::size).map_err(|error| error.to_string())?;
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
    let (key, _) = opened_segment(store, &digest).await?;
    with_file(*key, |handle| handle.read_at(offset, &mut bytes))
        .map_err(|error| error.to_string())?;
    Ok(bytes)
}

/// The name of `segment`'s column at `slot` (`None` for its x axis), as a failure to read it
/// names it.
fn column_name(segment: &Segment, slot: Option<usize>) -> String {
    let Ok(schema) = segment.reader().schema() else {
        return String::new();
    };
    match slot {
        None => schema.x.name.clone(),
        Some(slot) => (0..schema.signals.len())
            .find(|signal| {
                schema.slot_of(*signal).is_some_and(|first| {
                    (first..first + schema.signals[*signal].kind.slots()).contains(&slot)
                })
            })
            .map(|signal| schema.signals[signal].name.clone())
            .unwrap_or_default(),
    }
}

/// The window `window` of `segment` as a waveform holds it.
fn window_of(segment: &Arc<Segment>, window: WorkerWindow) -> Result<WaveformValues, String> {
    let slot = window
        .column
        .checked_sub(1)
        .map(|slot| usize::try_from(slot).map_err(|_| format!("there is no column {slot}")))
        .transpose()?;
    let lead = match window.lead {
        WorkerLead::None => WindowLead::None,
        WorkerLead::Value { value } => WindowLead::Value(value.get()),
        WorkerLead::Interpolated { fraction } => WindowLead::Interpolated(fraction.get()),
    };
    WaveformValues::stored(
        SegmentColumn::new(Arc::clone(segment), slot, column_name(segment, slot)),
        StoredWindow {
            first: window.first,
            rows: window.rows,
            lead,
        },
    )
}

/// The read a page asked for.
fn read_of(read: WorkerRead) -> Result<Read, String> {
    let count = |value: u64, what: &str| {
        usize::try_from(value).map_err(|_| format!("the {what} is larger than this memory"))
    };
    Ok(match read {
        WorkerRead::Range { first, count: rows } => Read::Range {
            first: count(first, "range's start")?,
            count: count(rows, "range")?,
        },
        WorkerRead::Tile { x0, x1, pixels } => Read::Tile {
            x0: x0.get(),
            x1: x1.get(),
            pixels: count(pixels, "tile")?,
        },
        WorkerRead::LogTile { x0, x1, pixels } => Read::LogTile {
            x0: x0.get(),
            x1: x1.get(),
            pixels: count(pixels, "tile")?,
        },
        WorkerRead::Point { x, interpolation } => {
            use rspice_results::sampling::SampleInterpolation;
            Read::Point {
                x: x.get(),
                interpolation: match interpolation.as_str() {
                    "linear" => SampleInterpolation::Linear,
                    "nearest" => SampleInterpolation::Nearest,
                    "monotone-cubic" => SampleInterpolation::MonotoneCubic,
                    other => return Err(format!("'{other}' names no interpolation")),
                },
            }
        }
    })
}

/// What `value` gave, as it crosses to the page, with the buffers it crosses in.
fn crossing(value: ReadValue) -> (WorkerReadValue, Array) {
    let buffers = Array::new();
    let value = match value {
        ReadValue::Samples(samples) => {
            buffers.push(&Float64Array::from(&samples[..]));
            WorkerReadValue::Samples
        }
        ReadValue::Tile(tile) => match tile.as_ref() {
            ColumnTile::Samples {
                x,
                y,
                before,
                after,
            } => {
                buffers.push(&Float64Array::from(x.as_slice()));
                buffers.push(&Float64Array::from(y.as_slice()));
                let pair = |pair: &Option<(f64, f64)>| {
                    pair.map(|(x, y)| (WorkerF64::from(x), WorkerF64::from(y)))
                };
                WorkerReadValue::TileSamples {
                    before: pair(before),
                    after: pair(after),
                }
            }
            ColumnTile::Envelope { min, max, touched } => {
                buffers.push(&Float64Array::from(min.as_slice()));
                buffers.push(&Float64Array::from(max.as_slice()));
                let touched: Vec<u8> = touched.iter().map(|touched| u8::from(*touched)).collect();
                buffers.push(&Uint8Array::from(touched.as_slice()));
                WorkerReadValue::TileEnvelope
            }
        },
        ReadValue::Point(value) => WorkerReadValue::Point {
            value: value.map(WorkerF64::from),
        },
        ReadValue::Failed(reason) => WorkerReadValue::Failed { reason },
    };
    (value, buffers)
}

/// Make `read` of the window `values` of the kept segment `digest`, its run's axis the window
/// `x`, and answer what it gave.
async fn series(
    store: &mut Store,
    digest: &str,
    x: WorkerWindow,
    values: WorkerWindow,
    read: WorkerRead,
) -> Result<(ResultsStoreResponse, Array), String> {
    let digest = digest_of(digest)?;
    if !store.checked.contains(&digest.0) {
        return Err("a kept segment is read only once it is checked".to_owned());
    }
    let read = read_of(read)?;
    let (_, segment) = opened_segment(store, &digest).await?;
    let value = match (window_of(segment, x), window_of(segment, values)) {
        (Ok(x), Ok(values)) => rspice_results::waveform::perform(&values, &x, read),
        (Err(reason), _) | (_, Err(reason)) => ReadValue::Failed(reason),
    };
    let (value, buffers) = crossing(value);
    Ok((ResultsStoreResponse::Series { value }, buffers))
}

/// Compare two runs on the samples this store keeps, and on those the page sent beside the
/// request: the comparison reads every sample of the signals it compares here, so the page holds
/// none of them. The comparison's own refusal is its answer, not a failure of the worker.
async fn compare(
    store: &mut Store,
    comparison: WorkerComparison,
    buffers: &Array,
) -> Result<ResultsStoreResponse, String> {
    use rspice_results::waveform_comparison::{ComparedRun, ComparisonOptions, compare_runs};
    let candidate = compared_waveforms(store, &comparison.candidate, buffers).await;
    let baseline = compared_waveforms(store, &comparison.baseline, buffers).await;
    let (candidate, baseline) = match (candidate, baseline) {
        (Ok(candidate), Ok(baseline)) => (candidate, baseline),
        (Err(reason), _) | (_, Err(reason)) => {
            return Ok(ResultsStoreResponse::Uncompared { reason });
        }
    };
    fn run<'a>(
        run: &'a WorkerComparedRun,
        waveforms: &'a [RetainedWaveform],
    ) -> ComparedRun<'a, RetainedWaveform> {
        ComparedRun {
            binding: run.binding,
            coordinate_unit: run.coordinate_unit.as_deref(),
            waveforms,
        }
    }
    let compared = compare_runs(
        run(&comparison.candidate, &candidate[..]),
        run(&comparison.baseline, &baseline[..]),
        &comparison.signals,
        ComparisonOptions {
            alignment: comparison.alignment,
            alignment_signal: &comparison.alignment_signal,
            threshold: comparison.threshold.get(),
            maximum_lag_samples: comparison.maximum_lag_samples,
            absolute_tolerance: comparison.absolute_tolerance.get(),
            relative_tolerance: comparison.relative_tolerance.get(),
        },
        &mut |_, _| true,
    );
    Ok(match compared {
        Ok(outcome) => ResultsStoreResponse::Compared {
            outcome: Box::new(outcome),
        },
        Err(reason) => ResultsStoreResponse::Uncompared { reason },
    })
}

/// The waveforms of `run`, their columns read from this store's segments or from `buffers`.
async fn compared_waveforms(
    store: &mut Store,
    run: &WorkerComparedRun,
    buffers: &Array,
) -> Result<Vec<RetainedWaveform>, String> {
    let mut waveforms = Vec::with_capacity(run.waveforms.len());
    for waveform in &run.waveforms {
        let x = compared_column(store, &waveform.x, buffers).await?;
        let y = compared_column(store, &waveform.y, buffers).await?;
        waveforms.push(RetainedWaveform {
            name: waveform.name.clone(),
            x,
            y,
            unit: waveform.unit.clone(),
            complex: None,
        });
    }
    Ok(waveforms)
}

async fn compared_column(
    store: &mut Store,
    column: &WorkerColumn,
    buffers: &Array,
) -> Result<WaveformValues, String> {
    match column {
        WorkerColumn::Stored { segment, window } => {
            let digest = digest_of(segment)?;
            let (_, segment) = opened_segment(store, &digest).await?;
            window_of(segment, *window)
        }
        WorkerColumn::Held { buffer } => Ok(WaveformValues::from(f64_buffer(buffers, *buffer)?)),
    }
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
    // A file is closed before it is deleted.
    store.open.retain(|digest, _| kept.contains(digest));
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

/// Copy into `store` each segment of `digests` it does not keep that another project's store
/// does, checked against its name once copied. A store another tab holds is that tab's, and is
/// not read.
async fn adopt(store: &mut Store, digests: &[String]) -> Result<ResultsStoreResponse, String> {
    let mut wanted: Vec<Digest> = digests
        .iter()
        .map(|digest| digest_of(digest))
        .collect::<Result<_, _>>()?;
    wanted.retain(|digest| member_of(store, digest).is_err());
    let mut segments = Vec::new();
    if wanted.is_empty() {
        return Ok(ResultsStoreResponse::Adopted { segments });
    }
    let root = storage_root().await?;
    let stores = directory(&root, STORES).await?;
    for name in folder_names(&stores).await? {
        if wanted.is_empty() {
            break;
        }
        if name == store.name || !valid_name(&name) {
            continue;
        }
        let Some(_lock) = try_lock(&format!("rspice-results-store-{name}")).await? else {
            continue;
        };
        let Ok(source) = OpfsStoreDir::open(&stores, &name, false).await else {
            continue;
        };
        let Ok(Some(manifest)) = source.load_manifest().await else {
            continue;
        };
        for member in manifest.members() {
            let Some(digest) = member.digest.filter(|digest| wanted.contains(digest)) else {
                continue;
            };
            if member.status.is_none() {
                continue;
            }
            // A segment that cannot be copied is left out of the answer, and reads as unread
            // where the page reads it.
            if let Ok(record) = copy_member(store, &source, member, &digest).await {
                wanted.retain(|wanted| *wanted != digest);
                segments.push(record);
            }
        }
    }
    Ok(ResultsStoreResponse::Adopted { segments })
}

/// Copy `member` of the store `source` into `store` as a member of its own, checked against
/// `digest` before the manifest names it.
async fn copy_member(
    store: &mut Store,
    source: &OpfsStoreDir,
    member: &Member,
    digest: &Digest,
) -> Result<KeptSegmentRecord, String> {
    let from = source
        .segment(member.index, false)
        .await
        .map_err(|error| format!("it could not be opened: {error}"))?;
    let index = store
        .manifest
        .members()
        .iter()
        .map(|member| member.index)
        .chain(store.writes.iter().map(|(_, write)| write.index))
        .max()
        .map_or(0, |index| index + 1);
    let before = store.manifest.clone();
    let copied = async {
        let mut to = store
            .dir
            .segment(index, true)
            .await
            .map_err(|error| format!("a segment could not be made for it: {error}"))?;
        let len = from.size().map_err(|error| error.to_string())?;
        let mut piece = Vec::new();
        let mut offset = 0;
        while offset < len {
            let take = usize::try_from(COPY_PIECE.min(len - offset))
                .map_err(|_| "a piece is longer than this memory".to_owned())?;
            piece.resize(take, 0);
            from.read_at(offset, &mut piece)
                .map_err(|error| error.to_string())?;
            to.write_at(offset, &piece)
                .map_err(|error| error.to_string())?;
            offset += take as u64;
        }
        to.flush().map_err(|error| error.to_string())?;
        drop(to);
        let mut manifest = store.manifest.clone();
        manifest.upsert(Member {
            index,
            segment: OpfsStoreDir::segment_name(index),
            ..member.clone()
        });
        store.manifest = manifest;
        let (key, segment) = opened_segment(store, digest).await?;
        let len = with_file(*key, SegmentIo::size).map_err(|error| error.to_string())?;
        let rows = segment.reader().rows();
        store
            .dir
            .save_manifest(&store.manifest)
            .await
            .map_err(|error| format!("the results store's manifest could not be saved: {error}"))?;
        Ok(KeptSegmentRecord {
            digest: hex(digest),
            bytes: len,
            rows,
        })
    }
    .await;
    if copied.is_err() {
        store.open.remove(&digest.0);
        store.checked.remove(&digest.0);
        store.manifest = before;
        let _ = store.dir.remove_segment(index).await;
    }
    copied
}

/// Delete every project's store but the `live` projects' and the one this worker holds. A store
/// another tab holds is in use there, and stays.
async fn sweep(store: Option<&Store>, live: &[String]) -> Result<ResultsStoreResponse, String> {
    let root = storage_root().await?;
    let stores = directory(&root, STORES).await?;
    let mut removed = Vec::new();
    for name in folder_names(&stores).await? {
        if !valid_name(&name)
            || live.contains(&name)
            || store.is_some_and(|store| store.name == name)
        {
            continue;
        }
        let Some(_lock) = try_lock(&format!("rspice-results-store-{name}")).await? else {
            continue;
        };
        let options = js_sys::Object::new();
        Reflect::set(&options, &"recursive".into(), &JsValue::TRUE)
            .map_err(|error| js_message(&error))?;
        let remove = Reflect::get(&stores, &"removeEntry".into())
            .ok()
            .and_then(|remove| remove.dyn_into::<js_sys::Function>().ok())
            .ok_or("this browser cannot delete a folder")?;
        let promise = remove
            .call2(&stores, &JsValue::from_str(&name), &options)
            .map_err(|error| js_message(&error))?;
        if awaited(promise, "the store's folder").await.is_ok() {
            removed.push(name);
        }
    }
    Ok(ResultsStoreResponse::Swept { removed })
}

/// The names of the entries of `folder`.
async fn folder_names(folder: &FileSystemDirectoryHandle) -> Result<Vec<String>, String> {
    let keys = Reflect::get(folder, &"keys".into())
        .ok()
        .and_then(|keys| keys.dyn_into::<js_sys::Function>().ok())
        .ok_or("this browser cannot list a folder")?;
    let names = keys.call0(folder).map_err(|error| js_message(&error))?;
    let next = Reflect::get(&names, &"next".into())
        .ok()
        .and_then(|next| next.dyn_into::<js_sys::Function>().ok())
        .ok_or("this browser cannot list a folder")?;
    let mut found = Vec::new();
    loop {
        let step = next.call0(&names).map_err(|error| js_message(&error))?;
        let step = awaited(step, "a folder's listing").await?;
        if Reflect::get(&step, &"done".into()).is_ok_and(|done| done.is_truthy()) {
            return Ok(found);
        }
        if let Some(name) = Reflect::get(&step, &"value".into())
            .ok()
            .and_then(|name| name.as_string())
        {
            found.push(name);
        }
    }
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
