//! The browser's results stores: each segment the page holds is kept in the open project's store,
//! in this browser's origin-private file system, and a project opened again loads the segments
//! it names from there.
//!
//! The page cannot write or read origin-private files synchronously; a dedicated worker can, so
//! the store is reached through one: a long-lived instance of the simulation worker's script that
//! runs no simulation, so stopping a run never closes the store. One tab holds a project's store
//! at a time. A tab that finds the store held by another writes nothing to it and loads nothing
//! from it, and where the browser keeps no store the results stay in the page's memory; the Save
//! page states which of these this page has ([`standing`]).
//!
//! A segment is written from its rows. When the store fills, the write closes on the rows it
//! holds and the run keeps that many: its waveforms are cut to them and it says why
//! ([`follow`]). A project's runs whose segments the page does not hold are loaded from the store
//! when it opens and read again.

use std::cell::RefCell;
use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::Arc;

use js_sys::{Array, Float64Array, Reflect, Uint8Array};
use rspice_results::store_reader::{Segment, SegmentBytes, SegmentDigest, StoreReader};
use rspice_results::waveform::{StoredWindow, WaveformValues};
use rspice_results_store::{MemoryIo, RunStatus, SegmentReader, SegmentWriter, StopReason};
use rspice_simulation_contract::worker_transport::{
    ResultsStoreRequest, ResultsStoreResponse, ResultsStoreStanding, WorkerResultsSchema,
};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::{JsCast, JsValue};

use crate::state::{ResultsStorageStop, SimulationState};

/// The bytes of rows a write sends at once.
const BYTES_PER_MESSAGE: usize = 8 << 20;

/// The bytes of a kept segment a load asks for at once.
const BYTES_PER_READ: u64 = 32 << 20;

/// What this tab has of the open project's results store.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum Standing {
    /// Nothing asked for yet.
    #[default]
    Unasked,
    /// Asked for; the worker has not answered.
    Opening,
    /// This tab holds it: the results the page holds are kept in it.
    Held,
    /// Another tab of this browser holds it.
    HeldElsewhere,
    /// This browser keeps no store, in its words.
    Unavailable(String),
}

/// A write the store's filling stopped: the page's segment `from` is cut to the rows the store
/// kept, held by `to`.
struct Cut {
    from: SegmentDigest,
    to: Arc<Segment>,
    rows: u64,
    bytes: u64,
    at: Option<f64>,
}

#[derive(Default)]
struct Client {
    /// The store asked for: the open project's id.
    store: Option<String>,
    standing: Standing,
    /// The segments the store keeps, with their bytes where this page knows them.
    kept: HashMap<[u8; 32], u64>,
    /// Segments waiting to be written, each with the store it was made for.
    queued: VecDeque<(String, Arc<Segment>)>,
    writing: bool,
    cuts: Vec<Cut>,
    loading: HashSet<[u8; 32]>,
    /// Kept segments that could not be loaded, and why.
    unloadable: HashMap<[u8; 32], String>,
    /// Segments loaded and not yet read again by the runs that name them.
    loaded: Vec<Arc<Segment>>,
    /// The segments the project's last saved version reads (the one opened, until a save): a
    /// save keeps them beside the ones it reads, as the version it replaced may be restored.
    named: HashSet<[u8; 32]>,
    /// The history's revision and the count of kept segments when it was last looked through
    /// for segments to load.
    scanned: Option<(rspice_results::run_history::RunHistoryRevision, usize)>,
    /// Whether a project file about to be read is having its segments loaded ([`preload`]).
    preloading: bool,
    /// The segments loaded for a project file about to be read, held until that project (by id)
    /// is open.
    preloaded: Option<(String, Vec<Arc<Segment>>)>,
    repaint: Option<egui::Context>,
}

thread_local! {
    static CLIENT: RefCell<Client> = RefCell::new(Client::default());
    static LINK: RefCell<Option<Link>> = const { RefCell::new(None) };
}

fn with<T>(act: impl FnOnce(&mut Client) -> T) -> T {
    CLIENT.with(|client| act(&mut client.borrow_mut()))
}

fn repaint() {
    if let Some(ctx) = with(|client| client.repaint.clone()) {
        ctx.request_repaint();
    }
}

/// What this tab has of the open project's results store.
pub fn standing() -> Standing {
    with(|client| client.standing.clone())
}

/// The bytes of the segments the store keeps that this page knows the length of.
pub fn kept_bytes() -> u64 {
    with(|client| client.kept.values().sum())
}

/// Keep `segment` in the open project's store, unless this tab cannot write one.
pub(super) fn keep(segment: &Arc<Segment>) {
    let queued = with(|client| {
        let store = client.store.clone()?;
        if !matches!(client.standing, Standing::Held | Standing::Opening) {
            return None;
        }
        client.queued.push_back((store, Arc::clone(segment)));
        Some(())
    });
    if queued.is_some() {
        write_queued();
    }
}

//// Follow the open project, once a frame: hold its store, cut the runs a full store stopped and
/// say so, and load and read again the runs whose segments the store keeps and the page does not
/// hold. Returns each run the store stopped, by its sequence, with the stop it keeps.
pub fn follow(
    simulation: &mut SimulationState,
    project: Option<String>,
    ctx: &egui::Context,
) -> Vec<(u64, ResultsStorageStop)> {
    let (followed, preloading) = with(|client| {
        client.repaint.get_or_insert_with(|| ctx.clone());
        (client.store == project, client.preloading)
    });
    // A project file about to be read has the worker's attention until its segments are here.
    if preloading {
        return Vec::new();
    }
    let changed = !followed && {
        let named = segments_read(simulation, false);
        with(|client| {
            client.store = project.clone();
            client.standing = if project.is_some() {
                Standing::Opening
            } else {
                Standing::Unasked
            };
            client.kept.clear();
            client.queued.clear();
            client.loading.clear();
            client.unloadable.clear();
            client.scanned = None;
            client.named = named;
        });
        true
    };
    with(|client| {
        // The project whose segments were loaded before its file was read is open now: its
        // runs read them, and they need holding here no longer.
        if client.preloaded.as_ref().map(|(store, _)| store) == project.as_ref()
            && let Some((_, segments)) = client.preloaded.take()
        {
            client.loaded.extend(segments);
        }
    });
    if changed {
        open_store(project);
    }
    let stops = apply_cuts(simulation);
    load_unread(simulation);
    stops
}

/// Before a project file is read (`bytes`): open its store and load every segment the file names
/// that the store keeps, so its runs open reading their samples. The segments are held until that
/// project is open, and the store goes back to the project open now.
pub async fn preload(bytes: &[u8]) {
    // A file that names no segment has nothing to wait for: its project opens at once, and its
    // store opens once it is open.
    let Some((store, named)) = names_in(bytes).filter(|(_, named)| !named.is_empty()) else {
        return;
    };
    let current = with(|client| {
        client.preloaded = None;
        client.preloading = true;
        client.store.clone()
    });
    // Segments still being written finish first: the worker holds one store at a time.
    while with(|client| client.writing) {
        sleep(20).await;
    }
    let (standing, kept) = open_now(Some(&store)).await;
    let mut held = Vec::new();
    if standing == Standing::Held {
        for digest in named.iter().filter(|digest| kept.contains_key(*digest)) {
            if rspice_results::store_reader::kept_segment(SegmentDigest(*digest)).is_some() {
                continue;
            }
            match load(SegmentDigest(*digest)).await {
                Ok((segment, _)) => held.push(segment),
                Err(reason) => {
                    log::warn!("results store segment {}: {reason}", SegmentDigest(*digest))
                }
            }
        }
    }
    if current != Some(store.clone()) {
        let (standing, kept) = open_now(current.as_deref()).await;
        with(|client| {
            if client.store == current && current.is_some() {
                client.standing = standing;
                client.kept = kept;
            }
        });
    }
    with(|client| {
        client.preloaded = Some((store, held));
        client.preloading = false;
    });
    write_queued();
    repaint();
}

/// The project a file names (its id) and every segment digest it spells.
fn names_in(bytes: &[u8]) -> Option<(String, HashSet<[u8; 32]>)> {
    let file: serde_json::Value = serde_json::from_slice(bytes).ok()?;
    let project = file.pointer("/workspace/project/id")?.as_str()?.to_owned();
    let mut digests = HashSet::new();
    let mut pending = vec![&file];
    while let Some(value) = pending.pop() {
        match value {
            serde_json::Value::String(text) if text.len() == 64 => {
                if let Ok(digest) = rspice_results::store_reader::parse_digest(text) {
                    digests.insert(digest.0);
                }
            }
            serde_json::Value::Array(values) => pending.extend(values),
            serde_json::Value::Object(fields) => pending.extend(fields.values()),
            _ => {}
        }
    }
    Some((project, digests))
}

/// Open `store` in the worker (`None`: close the one it holds), and say what this tab has of it
/// and the segments it keeps.
async fn open_now(store: Option<&str>) -> (Standing, HashMap<[u8; 32], u64>) {
    let request = match store {
        Some(store) => ResultsStoreRequest::Open {
            store: store.to_owned(),
        },
        None => ResultsStoreRequest::Close,
    };
    let answer = call(&request, Array::new()).await;
    if store.is_none() {
        return (Standing::Unasked, HashMap::new());
    }
    let mut kept = HashMap::new();
    let standing = match answer {
        Ok((ResultsStoreResponse::Opened { standing, segments }, _)) => {
            kept = segments
                .iter()
                .filter_map(|record| {
                    let digest = rspice_results::store_reader::parse_digest(&record.digest).ok()?;
                    Some((digest.0, record.bytes))
                })
                .collect();
            match standing {
                ResultsStoreStanding::Held => Standing::Held,
                ResultsStoreStanding::HeldElsewhere => Standing::HeldElsewhere,
                ResultsStoreStanding::Unavailable { reason } => Standing::Unavailable(reason),
            }
        }
        Ok((ResultsStoreResponse::Failed { reason }, _)) | Err(reason) => {
            Standing::Unavailable(reason)
        }
        Ok((other, _)) => Standing::Unavailable(format!("the store answered {other:?}")),
    };
    if let Standing::Unavailable(reason) = &standing {
        log::warn!("this browser keeps no results store: {reason}");
    }
    (standing, kept)
}

fn open_store(project: Option<String>) {
    wasm_bindgen_futures::spawn_local(async move {
        let (standing, kept) = open_now(project.as_deref()).await;
        let current = with(|client| client.store == project);
        if !current || project.is_none() {
            return;
        }
        let held = standing == Standing::Held;
        // Every segment the page holds now goes into the store it opened: a project saved for
        // the first time, or opened with results this page made before.
        let open = if held {
            rspice_results::store_reader::open_segments()
        } else {
            Vec::new()
        };
        with(|client| {
            client.kept = kept;
            client.queued.clear();
            if let Some(store) = &project {
                client
                    .queued
                    .extend(open.into_iter().map(|segment| (store.clone(), segment)));
            }
            client.standing = standing;
        });
        if held {
            write_queued();
        }
        repaint();
    });
}

// Write the queued segments one after another, while this tab holds their store.
fn write_queued() {
    let start = with(|client| {
        if client.writing
            || client.preloading
            || client.standing != Standing::Held
            || client.queued.is_empty()
        {
            return false;
        }
        client.writing = true;
        true
    });
    if !start {
        return;
    }
    wasm_bindgen_futures::spawn_local(async move {
        loop {
            let next = with(|client| {
                if client.preloading || client.standing != Standing::Held {
                    return None;
                }
                let store = client.store.clone()?;
                while let Some((made_for, segment)) = client.queued.pop_front() {
                    if made_for == store {
                        return Some(segment);
                    }
                }
                None
            });
            let Some(segment) = next else {
                break;
            };
            if let Err(reason) = write(&segment).await {
                log::warn!(
                    "{}: this browser's results store could not keep it, so it stays in the \
                     page's memory: {reason}",
                    segment
                        .reader()
                        .run_meta()
                        .map(|meta| meta.analysis)
                        .unwrap_or_default()
                );
            }
        }
        with(|client| client.writing = false);
        repaint();
    });
}

/// Whether every segment made so far has been written, or given up on.
pub fn settled() -> bool {
    with(|client| !client.writing && client.queued.is_empty())
}

/// Wait until [`settled`]: a save waits for it, so the segments its file names are kept.
pub async fn until_settled() {
    while !settled() {
        sleep(20).await;
    }
}

async fn sleep(milliseconds: i32) {
    let promise = js_sys::Promise::new(&mut |resolve, _| {
        let scheduled = web_sys::window().map(|window| {
            window.set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, milliseconds)
        });
        if !matches!(scheduled, Some(Ok(_))) {
            let _ = resolve.call0(&JsValue::UNDEFINED);
        }
    });
    let _ = wasm_bindgen_futures::JsFuture::from(promise).await;
}

/// Write `segment` to the store from its rows. A store that fills keeps the rows it took, and
/// the page cuts the segment to them.
async fn write(segment: &Arc<Segment>) -> Result<(), String> {
    let reader = segment.reader();
    let digest = reader
        .digest()
        .ok_or("the segment states no digest: its run left it without END")?;
    let schema = reader.schema().map_err(|error| error.to_string())?;
    let meta = reader.run_meta().map_err(|error| error.to_string())?;
    let sent = WorkerResultsSchema::of(&schema, &meta)?;
    let begun = call(
        &ResultsStoreRequest::Begin {
            schema: sent,
            digest: digest.to_string(),
        },
        Array::new(),
    )
    .await?;
    let write = match begun.0 {
        ResultsStoreResponse::Begun { write } => write,
        // The store keeps it already.
        ResultsStoreResponse::Finished { record, .. } => {
            with(|client| {
                let bytes = client.kept.entry(digest.0).or_default();
                *bytes = (*bytes).max(record.bytes);
            });
            return Ok(());
        }
        other => return Err(refusal(other)),
    };
    let slots = schema.slots();
    let rows = segment.rows();
    let per = (BYTES_PER_MESSAGE / ((slots + 1) * 8)).max(1) as u64;
    let mut first = 0;
    let mut full = false;
    while first < rows && !full {
        let count = per.min(rows - first) as usize;
        let (x, values) = match rows_of(segment, first, count, slots) {
            Ok(rows) => rows,
            Err(reason) => {
                let _ = call(&ResultsStoreRequest::Abandon { write }, Array::new()).await;
                return Err(reason);
            }
        };
        let buffers = Array::new();
        buffers.push(&Float64Array::from(x.as_slice()));
        buffers.push(&Float64Array::from(values.as_slice()));
        match call(&ResultsStoreRequest::Rows { write }, buffers).await?.0 {
            ResultsStoreResponse::Taken { full: filled } => full = filled,
            other => return Err(refusal(other)),
        }
        first += count as u64;
    }
    let (record, full) = match call(&ResultsStoreRequest::Finish { write }, Array::new())
        .await?
        .0
    {
        ResultsStoreResponse::Finished { record, full } => (record, full),
        other => return Err(refusal(other)),
    };
    let kept = rspice_results::store_reader::parse_digest(&record.digest)?;
    with(|client| client.kept.insert(kept.0, record.bytes));
    if full {
        let cut = cut(segment, record.rows)?;
        if cut.reader().digest() != Some(kept) {
            return Err("the rows the store kept are not the page's first rows".to_owned());
        }
        let at = (record.rows > 0 && schema.x.name != "row")
            .then(|| {
                let mut last = [0.0];
                cut.read(None, record.rows - 1, &mut last).ok()?;
                Some(last[0])
            })
            .flatten();
        with(|client| {
            client.cuts.push(Cut {
                from: digest,
                to: cut,
                rows: record.rows,
                bytes: record.bytes,
                at,
            });
        });
        repaint();
    }
    Ok(())
}

fn refusal(response: ResultsStoreResponse) -> String {
    match response {
        ResultsStoreResponse::Failed { reason } => reason,
        other => format!("the store answered {other:?}"),
    }
}

/// Rows `first .. first + count` of `segment`: their x, and their `slots` value slots row-major.
fn rows_of(
    segment: &Segment,
    first: u64,
    count: usize,
    slots: usize,
) -> Result<(Vec<f64>, Vec<f64>), String> {
    let mut x = vec![0.0; count];
    segment
        .read(None, first, &mut x)
        .map_err(|error| error.to_string())?;
    let mut values = vec![0.0; count * slots];
    let mut column = vec![0.0; count];
    for slot in 0..slots {
        segment
            .read(Some(slot), first, &mut column)
            .map_err(|error| error.to_string())?;
        for (row, value) in column.iter().enumerate() {
            values[row * slots + slot] = *value;
        }
    }
    Ok((x, values))
}

/// `segment` cut to its first `rows` rows, closed as a full store closes it.
fn cut(segment: &Segment, rows: u64) -> Result<Arc<Segment>, String> {
    let reader = segment.reader();
    let schema = reader.schema().map_err(|error| error.to_string())?;
    let meta = reader.run_meta().map_err(|error| error.to_string())?;
    let slots = schema.slots();
    let failed = |error: rspice_results_store::StoreError| error.to_string();
    let mut writer = SegmentWriter::create(MemoryIo::new(), &schema, &meta).map_err(failed)?;
    let per = (BYTES_PER_MESSAGE / ((slots + 1) * 8)).max(1) as u64;
    let mut first = 0;
    while first < rows {
        let count = per.min(rows - first) as usize;
        let (x, values) = rows_of(segment, first, count, slots)?;
        writer.push_rows(&x, &values).map_err(failed)?;
        first += count as u64;
    }
    let (_, io) = writer
        .finish(RunStatus::Stopped(StopReason::DiskFull))
        .map_err(failed)?;
    let reader = SegmentReader::open(Box::new(io) as SegmentBytes).map_err(failed)?;
    Ok(Segment::kept(StoreReader::new(reader), std::path::PathBuf::new()).shared())
}

/// Cut every waveform that reads a segment a full store stopped to the rows the store kept, and
/// mark the run that holds it.
fn apply_cuts(simulation: &mut SimulationState) -> Vec<(u64, ResultsStorageStop)> {
    let cuts = with(|client| std::mem::take(&mut client.cuts));
    let mut stops = Vec::new();
    for cut in cuts {
        for run in &mut simulation.retained.runs {
            let mut cut_here: Option<String> = None;
            for analysis in &mut run.data.analyses {
                let cut_values = |values: &mut WaveformValues| {
                    if let Some(again) = cut_to(values, &cut) {
                        *values = again;
                        true
                    } else {
                        false
                    }
                };
                let mut touched = false;
                for waveform in &mut analysis.data.waveforms {
                    let waveform = &mut waveform.data;
                    touched |= cut_values(&mut waveform.x);
                    touched |= cut_values(&mut waveform.y);
                    if let Some(complex) = waveform.complex.as_mut() {
                        touched |= cut_values(&mut complex.real);
                        touched |= cut_values(&mut complex.imag);
                    }
                }
                if touched {
                    cut_here.get_or_insert_with(|| analysis.label.clone());
                }
            }
            if let Some(analysis) = cut_here {
                let stop = ResultsStorageStop {
                    analysis,
                    at: cut.at,
                    rows: cut.rows,
                    bytes: cut.bytes,
                    free: None,
                    folder: STORE_FOLDER.to_owned(),
                    budget: None,
                };
                run.stop_on_results_storage(stop.clone());
                stops.push((run.id, stop));
            }
        }
    }
    stops
}

/// Where a browser keeps a project's results store, as a stop names it.
pub const STORE_FOLDER: &str = "this browser's storage for RSpice";

/// `values` cut to the rows `cut` kept, when it reads the segment `cut` stopped.
fn cut_to(values: &WaveformValues, cut: &Cut) -> Option<WaveformValues> {
    let (source, window) = values.stored_window()?;
    let (digest, index) = source.identity()?;
    if digest != cut.from.0 {
        return None;
    }
    let slot = index.checked_sub(1).map(|slot| slot as usize);
    let rows = cut.rows.saturating_sub(window.first).min(window.rows);
    let column = rspice_results::store_reader::SegmentColumn::new(
        Arc::clone(&cut.to),
        slot,
        source.name().to_owned(),
    );
    WaveformValues::stored(
        column,
        StoredWindow {
            first: window.first.min(cut.rows),
            rows,
            lead: window.lead,
        },
    )
    .ok()
}

/// Load the kept segments the history's unread runs name, and read those runs again once their
/// segments are here.
fn load_unread(simulation: &mut SimulationState) {
    let loaded = with(|client| !client.loaded.is_empty());
    if loaded {
        read_again(simulation);
        with(|client| client.loaded.clear());
    }
    // The history is looked through again only when it or the store's segments changed.
    let scan = (
        simulation.retained.runs.revision(),
        with(|client| client.kept.len()),
    );
    let unchanged = with(|client| {
        let unchanged = client.standing != Standing::Held || client.scanned.as_ref() == Some(&scan);
        client.scanned = Some(scan);
        unchanged
    });
    if unchanged {
        return;
    }
    let wanted = segments_read(simulation, true);
    let start: Vec<[u8; 32]> = with(|client| {
        let start: Vec<[u8; 32]> = wanted
            .into_iter()
            .filter(|digest| {
                client.kept.contains_key(digest)
                    && !client.loading.contains(digest)
                    && !client.unloadable.contains_key(digest)
                    && rspice_results::store_reader::kept_segment(SegmentDigest(*digest)).is_none()
            })
            .collect();
        client.loading.extend(start.iter().copied());
        start
    });
    for digest in start {
        wasm_bindgen_futures::spawn_local(async move {
            let loaded = load(SegmentDigest(digest)).await;
            with(|client| {
                client.loading.remove(&digest);
                match loaded {
                    Ok((segment, len)) => {
                        client.kept.insert(digest, len);
                        client.loaded.push(segment);
                    }
                    Err(reason) => {
                        log::warn!("results store segment {}: {reason}", SegmentDigest(digest));
                        client.unloadable.insert(digest, reason);
                    }
                }
            });
            repaint();
        });
    }
}

/// The kept segment `digest`, checked by the worker and read into the page's memory, with its
/// bytes in the store.
async fn load(digest: SegmentDigest) -> Result<(Arc<Segment>, u64), String> {
    let hex = digest.to_string();
    let len = match call(
        &ResultsStoreRequest::Check {
            digest: hex.clone(),
        },
        Array::new(),
    )
    .await?
    .0
    {
        ResultsStoreResponse::Checked { len } => len,
        other => return Err(refusal(other)),
    };
    let size = usize::try_from(len).map_err(|_| "it is longer than this page's memory")?;
    let mut bytes = Vec::new();
    bytes
        .try_reserve_exact(size)
        .map_err(|error| format!("this page's memory could not hold its {len} bytes: {error}"))?;
    while (bytes.len() as u64) < len {
        let offset = bytes.len() as u64;
        let ask = BYTES_PER_READ.min(len - offset);
        let (response, buffers) = call(
            &ResultsStoreRequest::Read {
                digest: hex.clone(),
                offset,
                len: ask,
            },
            Array::new(),
        )
        .await?;
        if !matches!(response, ResultsStoreResponse::Bytes) {
            return Err(refusal(response));
        }
        let chunk = buffers
            .get(0)
            .dyn_into::<Uint8Array>()
            .map_err(|_| "the store answered no bytes")?;
        if u64::from(chunk.length()) != ask {
            return Err(format!(
                "the store answered {} bytes where {ask} were asked for",
                chunk.length()
            ));
        }
        let at = bytes.len();
        bytes.resize(at + ask as usize, 0);
        chunk.copy_to(&mut bytes[at..]);
    }
    let reader = SegmentReader::open(Box::new(MemoryIo::from_bytes(bytes)) as SegmentBytes)
        .map_err(|error| error.to_string())?;
    let reader = StoreReader::new(reader);
    if reader.digest() != Some(digest) {
        return Err("the bytes read are not the segment asked for".to_owned());
    }
    Ok((
        Segment::kept(reader, std::path::PathBuf::new()).shared(),
        len,
    ))
}

/// Read again every run of the history that could not read its samples, its signals sharing one
/// axis as on a fresh run, now that more segments are here.
fn read_again(simulation: &mut SimulationState) {
    use rspice_formats::project_results::{read_again, read_events_again};
    use rspice_results::store_reader::ColumnReference;
    for run in &mut simulation.retained.runs {
        if run.unread_results().is_none() {
            continue;
        }
        let mut axes: Vec<(ColumnReference, WaveformValues)> = Vec::new();
        for analysis in &mut run.data.analyses {
            for waveform in &mut analysis.data.waveforms {
                let x = &mut waveform.data.x;
                if let Some(reference) = ColumnReference::of(x) {
                    match axes.iter().find(|(met, _)| *met == reference) {
                        Some((_, axis)) => *x = axis.clone(),
                        None => {
                            if let Some(axis) = read_again(x) {
                                *x = axis;
                            }
                            axes.push((reference, x.clone()));
                        }
                    }
                }
                let complex = waveform
                    .data
                    .complex
                    .iter_mut()
                    .flat_map(|complex| [&mut complex.real, &mut complex.imag]);
                for values in std::iter::once(&mut waveform.data.y).chain(complex) {
                    if let Some(again) = read_again(values) {
                        *values = again;
                    }
                }
            }
            if let Some(
                rspice_results::analysis_payload::AnalysisResultPayload::TransientEvents {
                    digital_traces,
                    real_traces,
                    ..
                },
            ) = analysis.data.result_payload.as_mut()
            {
                for trace in digital_traces {
                    if let Some(again) = read_events_again(&trace.points) {
                        trace.points = again;
                    }
                }
                for trace in real_traces {
                    if let Some(again) = read_events_again(&trace.points) {
                        trace.points = again;
                    }
                }
            }
        }
    }
}

//// After the open project was saved reading the segments `read`: the store lets go of every
/// segment neither that file, nor the version it replaced, nor this page reads, as a desktop
/// project's folder does after a save.
pub fn saved(read: impl IntoIterator<Item = SegmentDigest>) {
    let read: HashSet<[u8; 32]> = read.into_iter().map(|digest| digest.0).collect();
    let open: HashSet<[u8; 32]> = rspice_results::store_reader::open_segments()
        .iter()
        .filter_map(|segment| segment.reader().digest().map(|digest| digest.0))
        .collect();
    let keep = with(|client| {
        let replaced = std::mem::replace(&mut client.named, read.clone());
        if client.standing != Standing::Held {
            return None;
        }
        let queued = client
            .queued
            .iter()
            .filter_map(|(_, segment)| segment.reader().digest().map(|digest| digest.0));
        let keep: HashSet<[u8; 32]> = read
            .iter()
            .copied()
            .chain(replaced)
            .chain(open)
            .chain(queued)
            .collect();
        Some(keep)
    });
    let Some(keep) = keep else {
        return;
    };
    let digests = keep
        .iter()
        .map(|digest| SegmentDigest(*digest).to_string())
        .collect();
    wasm_bindgen_futures::spawn_local(async move {
        match call(&ResultsStoreRequest::Keep { digests }, Array::new()).await {
            Ok((ResultsStoreResponse::Kept { .. }, _)) => {
                with(|client| client.kept.retain(|digest, _| keep.contains(digest)));
            }
            Ok((other, _)) => {
                log::warn!("the results store kept every segment: {}", refusal(other))
            }
            Err(reason) => log::warn!("the results store kept every segment: {reason}"),
        }
    });
}

/// Every segment the history's runs read (`unread_only`: only the runs that cannot read their
/// samples), by digest.
fn segments_read(simulation: &SimulationState, unread_only: bool) -> HashSet<[u8; 32]> {
    use rspice_results::analysis_payload::AnalysisResultPayload;
    let mut read = HashSet::new();
    for run in &*simulation.retained.runs {
        if unread_only && run.unread_results().is_none() {
            continue;
        }
        for analysis in &run.data.analyses {
            for waveform in &analysis.data.waveforms {
                let waveform = &waveform.data;
                let complex = waveform
                    .complex
                    .iter()
                    .flat_map(|complex| [&complex.real, &complex.imag]);
                for values in [&waveform.x, &waveform.y].into_iter().chain(complex) {
                    if let Some((source, _)) = values.stored_window()
                        && let Some((digest, _)) = source.identity()
                    {
                        read.insert(digest);
                    }
                }
            }
            if let Some(AnalysisResultPayload::TransientEvents {
                digital_traces,
                real_traces,
                ..
            }) = analysis.data.result_payload.as_ref()
            {
                let digital = digital_traces
                    .iter()
                    .map(|trace| trace.points.stored_source());
                let real = real_traces.iter().map(|trace| trace.points.stored_source());
                for source in digital.chain(real).flatten() {
                    if let Some((digest, _)) = source.identity() {
                        read.insert(digest);
                    }
                }
            }
        }
    }
    read
}

// The worker that keeps the stores, and the requests it has not answered.
struct Link {
    worker: web_sys::Worker,
    next: u32,
    pending: HashMap<u32, (js_sys::Function, js_sys::Function)>,
    _onmessage: Closure<dyn FnMut(web_sys::MessageEvent)>,
    _onerror: Closure<dyn FnMut(web_sys::ErrorEvent)>,
}

fn worker_url() -> Result<String, String> {
    Reflect::get(&js_sys::global(), &"__RSPICE_SIM_WORKER_URL".into())
        .ok()
        .and_then(|url| url.as_string())
        .filter(|url| !url.trim().is_empty())
        .ok_or_else(|| "the page names no worker to keep its results".to_owned())
}

fn link() -> Result<(), String> {
    if LINK.with(|link| link.borrow().is_some()) {
        return Ok(());
    }
    let options = web_sys::WorkerOptions::new();
    options.set_type(web_sys::WorkerType::Module);
    let worker = web_sys::Worker::new_with_options(&worker_url()?, &options)
        .map_err(|error| js_message(&error))?;
    let onmessage =
        Closure::<dyn FnMut(web_sys::MessageEvent)>::new(move |event: web_sys::MessageEvent| {
            answered(&event.data())
        });
    let onerror =
        Closure::<dyn FnMut(web_sys::ErrorEvent)>::new(move |event: web_sys::ErrorEvent| {
            let message = event.message();
            fail_link(if message.is_empty() {
                "the worker that keeps the results stores failed".to_owned()
            } else {
                message
            });
        });
    worker.set_onmessage(Some(onmessage.as_ref().unchecked_ref()));
    worker.set_onerror(Some(onerror.as_ref().unchecked_ref()));
    LINK.with(|link| {
        *link.borrow_mut() = Some(Link {
            worker,
            next: 1,
            pending: HashMap::new(),
            _onmessage: onmessage,
            _onerror: onerror,
        });
    });
    Ok(())
}

fn field(value: &JsValue, name: &str) -> JsValue {
    Reflect::get(value, &JsValue::from_str(name)).unwrap_or(JsValue::UNDEFINED)
}

fn answered(data: &JsValue) {
    let kind = field(data, "type").as_string().unwrap_or_default();
    let id = field(data, "id").as_f64().map_or(0, |id| id as u32);
    match kind.as_str() {
        "results-store-result" | "results-store-error" => {
            let Some((resolve, reject)) = LINK.with(|link| {
                link.borrow_mut()
                    .as_mut()
                    .and_then(|link| link.pending.remove(&id))
            }) else {
                return;
            };
            if kind == "results-store-result" {
                let _ = resolve.call1(&JsValue::UNDEFINED, data);
            } else {
                let _ = reject.call1(&JsValue::UNDEFINED, &field(data, "error"));
            }
        }
        // The worker could not start: nothing it was asked will be answered.
        "error" if id == 0 => fail_link(
            field(data, "error")
                .as_string()
                .unwrap_or_else(|| "the worker that keeps the results stores failed".to_owned()),
        ),
        _ => {}
    }
}

/// Refuse every unanswered request with `reason`, and let the worker go: the next request
/// starts another.
fn fail_link(reason: String) {
    let Some(link) = LINK.with(|link| link.borrow_mut().take()) else {
        return;
    };
    link.worker.terminate();
    for (_, (_, reject)) in link.pending {
        let _ = reject.call1(&JsValue::UNDEFINED, &JsValue::from_str(&reason));
    }
}

fn js_message(error: &JsValue) -> String {
    field(error, "message")
        .as_string()
        .or_else(|| error.as_string())
        .unwrap_or_else(|| format!("{error:?}"))
}

/// Ask the worker `request`, with `buffers` beside it, and wait for its answer.
async fn call(
    request: &ResultsStoreRequest,
    buffers: Array,
) -> Result<(ResultsStoreResponse, Array), String> {
    link()?;
    let json = serde_json::to_string(request).map_err(|error| error.to_string())?;
    let mut parts = None;
    let promise = js_sys::Promise::new(&mut |resolve, reject| parts = Some((resolve, reject)));
    let parts = parts.ok_or("a request could not be made")?;
    let transfer = Array::new();
    for view in buffers.iter() {
        if let Ok(view) = view.dyn_into::<js_sys::Object>() {
            let buffer = field(&view, "buffer");
            if buffer.is_instance_of::<js_sys::ArrayBuffer>() {
                transfer.push(&buffer);
            }
        }
    }
    let posted = LINK.with(|link| {
        let mut link = link.borrow_mut();
        let link = link
            .as_mut()
            .ok_or("the worker that keeps the results stores is gone")?;
        let id = link.next;
        link.next = link.next.wrapping_add(1).max(1);
        let message = js_sys::Object::new();
        let set = |name: &str, value: &JsValue| {
            Reflect::set(&message, &JsValue::from_str(name), value).map_err(|e| js_message(&e))
        };
        set("type", &"results-store".into())?;
        set("id", &JsValue::from_f64(f64::from(id)))?;
        set("request", &json.into())?;
        set("buffers", &buffers)?;
        link.pending.insert(id, parts);
        link.worker
            .post_message_with_transfer(&message, &transfer)
            .map_err(|error| {
                link.pending.remove(&id);
                js_message(&error)
            })
    });
    posted?;
    let data = wasm_bindgen_futures::JsFuture::from(promise)
        .await
        .map_err(|error| js_message(&error))?;
    let response = field(&data, "response")
        .as_string()
        .ok_or("the store answered no response")?;
    let response: ResultsStoreResponse =
        serde_json::from_str(&response).map_err(|error| error.to_string())?;
    let buffers = field(&data, "buffers")
        .dyn_into::<Array>()
        .unwrap_or_else(|_| Array::new());
    Ok((response, buffers))
}
