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
//! ([`follow`]). A project's runs whose segments the page does not hold read them from the store
//! when it opens: the page never holds such a segment whole. A view's tiles, ranges and points of
//! it are read in the worker ([`series_request`]); what the page reads of it itself (a cursor's
//! rows, an axis' ends, what a command reads whole) comes a piece at a time, and a read whose
//! pieces are on their way fails as pending and is made again once they are here. A segment no
//! longer than one read is read whole when its run opens, so a command finds it at once.

use std::cell::RefCell;
use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, LazyLock, Mutex, MutexGuard, Weak};

use js_sys::{Array, Float64Array, Reflect, Uint8Array};
use rspice_results::saved_output::SavedOutputMaterializationStatus;
use rspice_results::store_reader::{Segment, SegmentBytes, SegmentDigest, StoreReader};
use rspice_results::waveform::{
    ColumnTile, Read, ReadValue, RetainedWaveform, StoredWindow, WaveformValues, WindowLead,
};
use rspice_results_store::{
    MemoryIo, RunStatus, SegmentIo, SegmentReader, SegmentState, SegmentWriter, StopReason,
    StoreError,
};
use rspice_simulation_contract::worker_transport::{
    ResultsStoreRequest, ResultsStoreResponse, ResultsStoreStanding, WorkerColumn,
    WorkerComparedRun, WorkerComparedWaveform, WorkerComparison, WorkerF64, WorkerLead, WorkerRead,
    WorkerReadValue, WorkerResultsSchema, WorkerWindow,
};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::{JsCast, JsValue};

use crate::state::{ResultsStorageStop, SimulationState};

/// The bytes of rows a write sends at once.
const BYTES_PER_MESSAGE: usize = 8 << 20;

/// The bytes of a kept segment one request for its bytes asks for at once.
const BYTES_PER_READ: u64 = 32 << 20;

/// The bytes of a kept segment the page holds as one piece of what it reads of it itself.
const PIECE: u64 = 1 << 20;

/// The pieces a read the page cannot make yet asks for past the last it needs: what a pass over
/// its rows reads next.
const READ_AHEAD: u64 = 3;

/// The rounds of asking for the pieces a kept segment's reader needs to open it: its start, its
/// end, and what a reader looking for a lost end reads.
const OPEN_ROUNDS: usize = 8;

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
/// kept, held by `to`. With no `to`, the store kept none of it: the page keeps its samples, and
/// the run says they are not kept.
struct Cut {
    from: SegmentDigest,
    to: Option<Arc<Segment>>,
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
    /// Segments the project's runs read that its store does not keep, asked of the other
    /// projects' stores ([`adopt_now`]) once while this store is open.
    sought: HashSet<[u8; 32]>,
    /// Segments loaded and not yet read again by the runs that name them.
    loaded: Vec<Arc<Segment>>,
    /// The kept segments whose reads are made in the worker, by digest.
    stubs: HashSet<[u8; 32]>,
    /// The segments the page holds whole in its memory, with their bytes: a run's own segment
    /// until the store keeps it, or for good where there is no store.
    held: Vec<(Weak<Segment>, u64)>,
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

/// The bytes of results store segments this page holds: the pieces of kept segments it read
/// itself, and the segments it holds whole.
pub fn page_bytes() -> u64 {
    let held: u64 = with(|client| {
        client
            .held
            .retain(|(segment, _)| segment.strong_count() > 0);
        client.held.iter().map(|(_, bytes)| bytes).sum()
    });
    held + pieces().bytes
}

/// Keep `segment`, `bytes` long in the page's memory, in the open project's store, unless this
/// tab cannot write one.
pub(super) fn keep(segment: &Arc<Segment>, bytes: u64) {
    with(|client| client.held.push((Arc::downgrade(segment), bytes)));
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

/// Follow the open project, once a frame: hold its store, cut the runs a full store stopped and
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
            client.sought.clear();
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
    let Some((store, named)) =
        super::named::segments_named_in(bytes).filter(|(_, named)| !named.is_empty())
    else {
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
    let (standing, mut kept) = open_now(Some(&store)).await;
    let mut held = Vec::new();
    if standing == Standing::Held {
        // A copy of a project names the segments of the project it was copied from.
        let missing: Vec<[u8; 32]> = named
            .iter()
            .filter(|digest| !kept.contains_key(*digest))
            .copied()
            .collect();
        kept.extend(adopt_now(&missing).await);
        for digest in named.iter().filter(|digest| kept.contains_key(*digest)) {
            if rspice_results::store_reader::kept_segment(SegmentDigest(*digest)).is_some() {
                continue;
            }
            match stub(SegmentDigest(*digest)).await {
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
    } else {
        // The open project's own store: what it copied in, it keeps now.
        with(|client| client.kept.extend(kept));
    }
    with(|client| {
        client.preloaded = Some((store, held));
        client.preloading = false;
    });
    write_queued();
    repaint();
}

/// Copy into the store this tab holds the segments `digests` that another project's store
/// keeps, and say the bytes of each it keeps now: a copy of a project under a new identity reads
/// the segments of the one it was copied from.
async fn adopt_now(digests: &[[u8; 32]]) -> HashMap<[u8; 32], u64> {
    if digests.is_empty() {
        return HashMap::new();
    }
    let request = ResultsStoreRequest::Adopt {
        digests: digests
            .iter()
            .map(|digest| SegmentDigest(*digest).to_string())
            .collect(),
    };
    match call(&request, Array::new()).await {
        Ok((ResultsStoreResponse::Adopted { segments }, _)) => segments
            .iter()
            .filter_map(|record| {
                let digest = rspice_results::store_reader::parse_digest(&record.digest).ok()?;
                Some((digest.0, record.bytes))
            })
            .collect(),
        Ok((other, _)) => {
            log::warn!("the results stores were not searched: {}", refusal(other));
            HashMap::new()
        }
        Err(reason) => {
            log::warn!("the results stores were not searched: {reason}");
            HashMap::new()
        }
    }
}

/// Delete the results stores of every project but the `live` ones, the one open here and any
/// another tab holds: what projects never saved, or since deleted, left in this browser.
pub async fn sweep(live: impl IntoIterator<Item = String>) {
    let mut live: Vec<String> = live.into_iter().collect();
    live.extend(with(|client| {
        client
            .store
            .iter()
            .cloned()
            .chain(client.preloaded.iter().map(|(store, _)| store.clone()))
            .collect::<Vec<_>>()
    }));
    match call(&ResultsStoreRequest::Sweep { live }, Array::new()).await {
        Ok((ResultsStoreResponse::Swept { removed }, _)) => {
            if !removed.is_empty() {
                log::info!(
                    "deleted the results stores of {} projects this browser keeps nothing else of",
                    removed.len()
                );
            }
        }
        Ok((other, _)) => log::warn!("the results stores were not swept: {}", refusal(other)),
        Err(reason) => log::warn!("the results stores were not swept: {reason}"),
    }
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
        let mut kept = kept;
        // Every segment the page holds now goes into the store it opened: a project saved for
        // the first time, or opened with results this page made before. The ones the page reads
        // through the worker are another project's store's, and are copied from it; the rest
        // are in the page's memory, and are written from their rows.
        let mut open = Vec::new();
        if held {
            let stubs = with(|client| client.stubs.clone());
            let mut adopt = Vec::new();
            for segment in rspice_results::store_reader::open_segments() {
                match segment.reader().digest().map(|digest| digest.0) {
                    Some(digest) if stubs.contains(&digest) => {
                        if !kept.contains_key(&digest) {
                            adopt.push(digest);
                        }
                    }
                    _ => open.push(segment),
                }
            }
            kept.extend(adopt_now(&adopt).await);
        }
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
            digest: Some(digest.to_string()),
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
        // The store has no room to begin it: it keeps none of it, and the run says so.
        ResultsStoreResponse::Full => {
            not_kept(digest);
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
    if full && record.rows == 0 {
        not_kept(digest);
    } else if full {
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
                to: Some(cut),
                rows: record.rows,
                bytes: record.bytes,
                at,
            });
        });
        repaint();
    }
    Ok(())
}

/// A comparison of two runs, as the page states it for the store worker to run.
pub struct StoreComparison<'a> {
    pub candidate: (
        rspice_vocabulary::product::DatasetBinding,
        &'a rspice_results::analysis_result::AnalysisResult<RetainedWaveform>,
    ),
    pub baseline: (
        rspice_vocabulary::product::DatasetBinding,
        &'a rspice_results::analysis_result::AnalysisResult<RetainedWaveform>,
    ),
    pub signals: Vec<String>,
    pub alignment: rspice_results::waveform_comparison::ComparisonAlignmentDraft,
    pub alignment_signal: String,
    pub threshold: f64,
    pub maximum_lag_samples: u32,
    pub absolute_tolerance: f64,
    pub relative_tolerance: f64,
}

/// Run `comparison` in the store worker, which reads the samples the store keeps: the page sends
/// the runs' column references, and reads now only the columns the store does not keep. The
/// comparison's refusal is its answer.
pub fn compare(
    comparison: StoreComparison<'_>,
) -> impl std::future::Future<
    Output = Result<rspice_results::waveform_comparison::ComparisonOutcome, String>,
> + 'static {
    use rspice_results::waveform_comparison::ComparisonAlignmentDraft;
    let mut names = comparison.signals.clone();
    if comparison.alignment != ComparisonAlignmentDraft::AbsoluteXAxis {
        names.push(comparison.alignment_signal.clone());
    }
    let mut held = Vec::new();
    let candidate = compared_run(comparison.candidate, &names, &mut held);
    let baseline = compared_run(comparison.baseline, &names, &mut held);
    let request = candidate.and_then(|candidate| {
        Ok(WorkerComparison {
            candidate,
            baseline: baseline?,
            signals: comparison.signals,
            alignment: comparison.alignment,
            alignment_signal: comparison.alignment_signal,
            threshold: comparison.threshold.into(),
            maximum_lag_samples: comparison.maximum_lag_samples,
            absolute_tolerance: comparison.absolute_tolerance.into(),
            relative_tolerance: comparison.relative_tolerance.into(),
        })
    });
    async move { compare_in_worker(request?, held).await }
}

/// The run `analysis`, `binding`, as the store worker compares it: the waveforms named `names`,
/// each column a window of a segment the store keeps or, for one it does not keep, samples the
/// page reads now and sends in `held`, where the column names its place.
fn compared_run(
    (binding, analysis): (
        rspice_vocabulary::product::DatasetBinding,
        &rspice_results::analysis_result::AnalysisResult<RetainedWaveform>,
    ),
    names: &[String],
    held: &mut Vec<Vec<f64>>,
) -> Result<WorkerComparedRun, String> {
    let mut column = |values: &WaveformValues| -> Result<WorkerColumn, String> {
        if let Some((source, window)) = values.stored_window()
            && let Some((digest, column)) = source.identity()
            && with(|client| client.kept.contains_key(&digest))
        {
            let lead = match window.lead {
                WindowLead::None => WorkerLead::None,
                WindowLead::Value(value) => WorkerLead::Value {
                    value: value.into(),
                },
                WindowLead::Interpolated(fraction) => WorkerLead::Interpolated {
                    fraction: fraction.into(),
                },
            };
            return Ok(WorkerColumn::Stored {
                segment: SegmentDigest(digest).to_string(),
                window: WorkerWindow {
                    column,
                    first: window.first,
                    rows: window.rows,
                    lead,
                },
            });
        }
        let samples = values
            .read_range(0..values.len())
            .map_err(|failure| failure.to_string())?;
        held.push(samples.into_owned());
        Ok(WorkerColumn::Held {
            buffer: (held.len() - 1) as u32,
        })
    };
    let mut waveforms = Vec::new();
    for waveform in analysis
        .waveforms
        .iter()
        .filter(|waveform| names.contains(&waveform.name))
    {
        waveforms.push(WorkerComparedWaveform {
            name: waveform.name.clone(),
            unit: waveform.unit.clone(),
            x: column(&waveform.x)?,
            y: column(&waveform.y)?,
        });
    }
    Ok(WorkerComparedRun {
        binding,
        coordinate_unit: analysis.waveform_coordinate_unit().map(str::to_owned),
        waveforms,
    })
}

/// Run `comparison` in the store worker, with the samples `held` its columns name.
async fn compare_in_worker(
    comparison: WorkerComparison,
    held: Vec<Vec<f64>>,
) -> Result<rspice_results::waveform_comparison::ComparisonOutcome, String> {
    let buffers = Array::new();
    for samples in &held {
        buffers.push(&Float64Array::from(samples.as_slice()));
    }
    drop(held);
    match call(
        &ResultsStoreRequest::Compare {
            comparison: Box::new(comparison),
        },
        buffers,
    )
    .await?
    .0
    {
        ResultsStoreResponse::Compared { outcome } => Ok(*outcome),
        ResultsStoreResponse::Uncompared { reason } => Err(reason),
        other => Err(refusal(other)),
    }
}

/// The store kept none of the page's segment `digest`: the page keeps its samples, and the run
/// that reads it says they are not kept.
fn not_kept(digest: SegmentDigest) {
    with(|client| {
        client.cuts.push(Cut {
            from: digest,
            to: None,
            rows: 0,
            bytes: 0,
            at: None,
        });
    });
    repaint();
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
    let bytes = io.size().map_err(|error| error.to_string())?;
    let reader = SegmentReader::open(Box::new(io) as SegmentBytes).map_err(failed)?;
    let segment = Segment::kept(StoreReader::new(reader), std::path::PathBuf::new()).shared();
    with(|client| client.held.push((Arc::downgrade(&segment), bytes)));
    Ok(segment)
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
                let cut_values = |values: &mut WaveformValues| match &cut.to {
                    Some(to) => match cut_to(values, cut.from, to, cut.rows) {
                        Some(again) => {
                            *values = again;
                            true
                        }
                        None => false,
                    },
                    None => reads(values, cut.from),
                };
                let mut touched = false;
                let mut kept = HashMap::new();
                for waveform in &mut analysis.data.waveforms {
                    let waveform = &mut waveform.data;
                    let mut cut_this = cut_values(&mut waveform.x);
                    cut_this |= cut_values(&mut waveform.y);
                    if let Some(complex) = waveform.complex.as_mut() {
                        cut_this |= cut_values(&mut complex.real);
                        cut_this |= cut_values(&mut complex.imag);
                    }
                    if cut_this && cut.to.is_some() {
                        kept.insert(waveform.name.clone(), waveform.x.len() as u64);
                    }
                    touched |= cut_this;
                }
                // A saved output's receipt counts the samples its waveform holds, which are now
                // the rows the store kept: left at the run's count, the receipt fails its check.
                for receipt in &mut analysis.data.saved_output_receipts {
                    recount(&mut receipt.status, &kept);
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

/// `status` counting the samples `kept` gives the waveforms it names that were cut.
fn recount(status: &mut SavedOutputMaterializationStatus, kept: &HashMap<String, u64>) {
    match status {
        SavedOutputMaterializationStatus::Materialized {
            waveform_name,
            sample_count,
        } => {
            if let Some(rows) = kept.get(waveform_name) {
                *sample_count = *rows;
            }
        }
        SavedOutputMaterializationStatus::MaterializedDcFamily { members } => {
            for member in members {
                if let Some(rows) = kept.get(&member.waveform_name) {
                    member.sample_count = *rows;
                }
            }
        }
        _ => {}
    }
}

/// Where a browser keeps a project's results store, as a stop names it.
pub const STORE_FOLDER: &str = "this browser's storage for RSpice";

/// Whether `values` read the segment `from`.
fn reads(values: &WaveformValues, from: SegmentDigest) -> bool {
    values
        .stored_window()
        .and_then(|(source, _)| source.identity())
        .is_some_and(|(digest, _)| digest == from.0)
}

/// `values` cut to the first `kept` rows, held by `to`, when they read the segment `from`.
fn cut_to(
    values: &WaveformValues,
    from: SegmentDigest,
    to: &Arc<Segment>,
    kept: u64,
) -> Option<WaveformValues> {
    let (source, window) = values.stored_window()?;
    let (digest, index) = source.identity()?;
    if digest != from.0 {
        return None;
    }
    let slot = index.checked_sub(1).map(|slot| slot as usize);
    let rows = kept.saturating_sub(window.first).min(window.rows);
    let column = rspice_results::store_reader::SegmentColumn::new(
        Arc::clone(to),
        slot,
        source.name().to_owned(),
    );
    WaveformValues::stored(
        column,
        StoredWindow {
            first: window.first.min(kept),
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
    let seek: Vec<[u8; 32]> = with(|client| {
        let seek: Vec<[u8; 32]> = wanted
            .iter()
            .filter(|digest| !client.kept.contains_key(*digest) && !client.sought.contains(*digest))
            .copied()
            .collect();
        client.sought.extend(seek.iter().copied());
        seek
    });
    if !seek.is_empty() {
        let store = with(|client| client.store.clone());
        wasm_bindgen_futures::spawn_local(async move {
            let adopted = adopt_now(&seek).await;
            if adopted.is_empty() {
                return;
            }
            with(|client| {
                if client.store == store {
                    client.kept.extend(adopted);
                }
            });
            repaint();
        });
    }
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
            let loaded = stub(SegmentDigest(digest)).await;
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

/// The kept segment `digest` as the page reads it, with its bytes in the store: checked by the
/// worker, and opened from the pieces its reader needs to open it, or from all of it when it is
/// no longer than one read. Its views' reads are made in the worker.
async fn stub(digest: SegmentDigest) -> Result<(Arc<Segment>, u64), String> {
    let len = match call(
        &ResultsStoreRequest::Check {
            digest: digest.to_string(),
        },
        Array::new(),
    )
    .await?
    .0
    {
        ResultsStoreResponse::Checked { len } => len,
        other => return Err(refusal(other)),
    };
    if len > 0 && len <= BYTES_PER_READ {
        fetch_pieces(digest.0, 0, len.div_ceil(PIECE), len).await?;
    }
    let fetch = Arc::new(AtomicBool::new(false));
    for _ in 0..OPEN_ROUNDS {
        let io = WorkerIo {
            digest: digest.0,
            size: len,
            fetch: Arc::clone(&fetch),
        };
        let opened = SegmentReader::open(Box::new(io) as SegmentBytes);
        let waiting = match &opened {
            Ok(reader) => {
                matches!(reader.state(), SegmentState::Recovered)
                    && reader.end_damage().is_some_and(pending)
            }
            Err(error) => pending(error),
        };
        if !waiting {
            let reader = StoreReader::new(opened.map_err(|error| error.to_string())?);
            if reader.digest() != Some(digest) {
                return Err("the segment read is not the one asked for".to_owned());
            }
            fetch.store(true, Ordering::Release);
            with(|client| client.stubs.insert(digest.0));
            return Ok((
                Segment::kept(reader, std::path::PathBuf::new()).shared(),
                len,
            ));
        }
        let mut fetched = false;
        while let Some((digest, first, count, size)) = next_wanted(Some(digest.0)) {
            fetch_pieces(digest, first, count, size).await?;
            fetched = true;
        }
        if !fetched {
            break;
        }
    }
    Err("its start and end could not be read from this browser's storage".to_owned())
}

/// Whether a store failed only because the bytes it read are on their way.
fn pending(error: &StoreError) -> bool {
    matches!(
        error,
        StoreError::Io {
            kind: std::io::ErrorKind::WouldBlock,
            ..
        }
    )
}

/// A piece of a kept segment: the segment's digest and the piece's index in it.
type PieceKey = ([u8; 32], u64);

/// The pieces of kept segments the page holds, for the reads it makes of them itself, under a
/// quarter of the loaded results budget, the least recently read let go first.
#[derive(Default)]
struct Pieces {
    /// Each piece by its segment's digest and its index, with the tick it was last read at.
    held: HashMap<PieceKey, (Arc<[u8]>, u64)>,
    bytes: u64,
    tick: u64,
    /// Pieces a read found missing, each with its segment's length, to be fetched.
    wanted: BTreeMap<PieceKey, u64>,
    fetching: bool,
}

static PIECES: LazyLock<Mutex<Pieces>> = LazyLock::new(Mutex::default);

fn pieces() -> MutexGuard<'static, Pieces> {
    PIECES.lock().unwrap_or_else(|p| p.into_inner())
}

impl Pieces {
    /// Hold `bytes`, read from piece `first` of the segment `digest` on.
    fn hold(&mut self, digest: [u8; 32], first: u64, bytes: &[u8]) {
        for (index, piece) in bytes.chunks(PIECE as usize).enumerate() {
            self.tick += 1;
            let key = (digest, first + index as u64);
            self.wanted.remove(&key);
            let held = (Arc::from(piece), self.tick);
            if let Some((old, _)) = self.held.insert(key, held) {
                self.bytes -= old.len() as u64;
            }
            self.bytes += piece.len() as u64;
        }
        let budget = super::super::run_reader::run_reader().budget() / 4;
        while self.bytes > budget {
            let Some(oldest) = self
                .held
                .iter()
                .min_by_key(|(_, (_, used))| *used)
                .map(|(key, _)| *key)
            else {
                break;
            };
            if let Some((piece, _)) = self.held.remove(&oldest) {
                self.bytes -= piece.len() as u64;
            }
        }
    }
}

/// The next run of wanted pieces of one segment (of `only`, when named), taken from the wanted
/// list: its segment, first piece, count and the segment's length. At most one read's bytes.
fn next_wanted(only: Option<[u8; 32]>) -> Option<([u8; 32], u64, u64, u64)> {
    let mut pieces = pieces();
    let (&(digest, first), &size) = pieces
        .wanted
        .iter()
        .find(|((digest, _), _)| only.is_none_or(|only| only == *digest))?;
    let mut count = 0;
    while count < BYTES_PER_READ / PIECE && pieces.wanted.remove(&(digest, first + count)).is_some()
    {
        count += 1;
    }
    Some((digest, first, count, size))
}

/// Ask the worker for `count` pieces of the kept segment `digest`, `size` bytes long, from piece
/// `first` on, and hold them.
async fn fetch_pieces(digest: [u8; 32], first: u64, count: u64, size: u64) -> Result<(), String> {
    let offset = first * PIECE;
    let len = (count * PIECE).min(size.saturating_sub(offset));
    if len == 0 {
        return Ok(());
    }
    let (response, buffers) = call(
        &ResultsStoreRequest::Read {
            digest: SegmentDigest(digest).to_string(),
            offset,
            len,
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
    if u64::from(chunk.length()) != len {
        return Err(format!(
            "the store answered {} bytes where {len} were asked for",
            chunk.length()
        ));
    }
    pieces().hold(digest, first, &chunk.to_vec());
    Ok(())
}

/// Fetch every wanted piece, one run after another, and ask for a frame as each arrives, so the
/// reads that wanted them are made again.
fn fetch_wanted() {
    {
        let mut pieces = pieces();
        if pieces.fetching || pieces.wanted.is_empty() {
            return;
        }
        pieces.fetching = true;
    }
    wasm_bindgen_futures::spawn_local(async {
        while let Some((digest, first, count, size)) = next_wanted(None) {
            // A store not held now (another project's opening, or the worker's restart) is asked
            // again by the next read that wants the pieces.
            if let Err(reason) = fetch_pieces(digest, first, count, size).await {
                log::debug!("results store segment {}: {reason}", SegmentDigest(digest));
            }
            repaint();
        }
        pieces().fetching = false;
    });
}

/// A kept segment as the page reads it itself, a piece at a time from the worker that holds it.
/// A read of bytes whose pieces are not here asks for them and fails as pending: a later read
/// finds them.
struct WorkerIo {
    digest: [u8; 32],
    size: u64,
    /// Whether a read that misses asks for its pieces in the background: not while the segment
    /// opens, which asks for them itself.
    fetch: Arc<AtomicBool>,
}

impl SegmentIo for WorkerIo {
    fn read_at(&self, offset: u64, buf: &mut [u8]) -> std::io::Result<()> {
        let end = offset
            .checked_add(buf.len() as u64)
            .filter(|end| *end <= self.size)
            .ok_or(std::io::ErrorKind::UnexpectedEof)?;
        if buf.is_empty() {
            return Ok(());
        }
        let (first, last) = (offset / PIECE, (end - 1) / PIECE);
        let mut pieces = pieces();
        let missing =
            (first..=last).find(|piece| !pieces.held.contains_key(&(self.digest, *piece)));
        if let Some(missing) = missing {
            let pieces_in = self.size.div_ceil(PIECE);
            for piece in missing..(last + 1 + READ_AHEAD).min(pieces_in) {
                if !pieces.held.contains_key(&(self.digest, piece)) {
                    pieces.wanted.insert((self.digest, piece), self.size);
                }
            }
            drop(pieces);
            if self.fetch.load(Ordering::Acquire) {
                fetch_wanted();
            }
            return Err(std::io::Error::new(
                std::io::ErrorKind::WouldBlock,
                rspice_results::waveform::PENDING_READ,
            ));
        }
        pieces.tick += 1;
        let tick = pieces.tick;
        for piece in first..=last {
            let Some((bytes, used)) = pieces.held.get_mut(&(self.digest, piece)) else {
                continue;
            };
            *used = tick;
            let start = piece * PIECE;
            let from = offset.max(start);
            let to = end.min(start + bytes.len() as u64);
            buf[(from - offset) as usize..(to - offset) as usize]
                .copy_from_slice(&bytes[(from - start) as usize..(to - start) as usize]);
        }
        Ok(())
    }

    fn write_at(&mut self, _offset: u64, _data: &[u8]) -> std::io::Result<()> {
        Err(std::io::Error::new(
            std::io::ErrorKind::PermissionDenied,
            "a kept segment is only read",
        ))
    }

    fn truncate(&mut self, _len: u64) -> std::io::Result<()> {
        Err(std::io::Error::new(
            std::io::ErrorKind::PermissionDenied,
            "a kept segment is only read",
        ))
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }

    fn size(&self) -> std::io::Result<u64> {
        Ok(self.size)
    }
}

/// A view's `read` of `values`, a series whose run's axis is `x`, made in the worker when both
/// are windows of a kept segment the worker holds: `None` for a series the page reads itself.
/// The read gives `None` when the worker could not be asked, as while another project's store
/// opens: the next frame's read asks again.
pub(crate) fn series_request(
    values: &WaveformValues,
    x: &WaveformValues,
    read: Read,
) -> Option<impl std::future::Future<Output = Option<ReadValue>> + 'static> {
    let window = |values: &WaveformValues| {
        let (source, window) = values.stored_window()?;
        let (digest, column) = source.identity()?;
        let lead = match window.lead {
            WindowLead::None => WorkerLead::None,
            WindowLead::Value(value) => WorkerLead::Value {
                value: value.into(),
            },
            WindowLead::Interpolated(fraction) => WorkerLead::Interpolated {
                fraction: fraction.into(),
            },
        };
        Some((
            digest,
            WorkerWindow {
                column,
                first: window.first,
                rows: window.rows,
                lead,
            },
        ))
    };
    let (digest, values) = window(values)?;
    let (axis, x) = window(x)?;
    if axis != digest
        || !with(|client| client.stubs.contains(&digest))
        || rspice_results::store_reader::kept_segment(SegmentDigest(digest)).is_none()
    {
        return None;
    }
    let read = match read {
        Read::Range { first, count } => WorkerRead::Range {
            first: first as u64,
            count: count as u64,
        },
        Read::Tile { x0, x1, pixels } => WorkerRead::Tile {
            x0: x0.into(),
            x1: x1.into(),
            pixels: pixels as u64,
        },
        Read::LogTile { x0, x1, pixels } => WorkerRead::LogTile {
            x0: x0.into(),
            x1: x1.into(),
            pixels: pixels as u64,
        },
        Read::Point { x, interpolation } => {
            use rspice_results::sampling::SampleInterpolation;
            WorkerRead::Point {
                x: x.into(),
                interpolation: match interpolation {
                    SampleInterpolation::MonotoneCubic => "monotone-cubic",
                    SampleInterpolation::Linear => "linear",
                    SampleInterpolation::Nearest => "nearest",
                }
                .to_owned(),
            }
        }
    };
    let request = ResultsStoreRequest::Series {
        digest: SegmentDigest(digest).to_string(),
        x,
        values,
        read,
    };
    Some(async move {
        match call(&request, Array::new()).await {
            Ok((ResultsStoreResponse::Series { value }, buffers)) => {
                Some(read_value(value, &buffers))
            }
            Ok((other, _)) => {
                log::debug!("a series read was refused: {}", refusal(other));
                None
            }
            Err(reason) => {
                log::debug!("a series read could not be asked: {reason}");
                None
            }
        }
    })
}

/// What a series read the worker answered gave, from its answer and buffers.
fn read_value(value: WorkerReadValue, buffers: &Array) -> ReadValue {
    let floats = |index: u32| -> Result<Vec<f64>, String> {
        buffers
            .get(index)
            .dyn_into::<Float64Array>()
            .map(|values| values.to_vec())
            .map_err(|_| format!("the worker answered no values in buffer {index}"))
    };
    let pair = |pair: Option<(WorkerF64, WorkerF64)>| pair.map(|(x, y)| (x.get(), y.get()));
    let value = match value {
        WorkerReadValue::Samples => floats(0).map(|samples| ReadValue::Samples(samples.into())),
        WorkerReadValue::TileSamples { before, after } => floats(0).and_then(|x| {
            Ok(ReadValue::Tile(Arc::new(ColumnTile::Samples {
                x,
                y: floats(1)?,
                before: pair(before),
                after: pair(after),
            })))
        }),
        WorkerReadValue::TileEnvelope => floats(0).and_then(|min| {
            let touched = buffers
                .get(2)
                .dyn_into::<Uint8Array>()
                .map_err(|_| "the worker answered no touched pixels".to_owned())?
                .to_vec()
                .into_iter()
                .map(|touched| touched != 0)
                .collect();
            Ok(ReadValue::Tile(Arc::new(ColumnTile::Envelope {
                min,
                max: floats(1)?,
                touched,
            })))
        }),
        WorkerReadValue::Point { value } => Ok(ReadValue::Point(value.map(WorkerF64::get))),
        WorkerReadValue::Failed { reason } => Ok(ReadValue::Failed(reason)),
    };
    value.unwrap_or_else(ReadValue::Failed)
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

/// After the open project was saved reading the segments `read`, its retained checkpoints reading
/// `checkpoints`: the store lets go of every segment neither that file, nor the version it
/// replaced, nor a checkpoint, nor this page reads, as a desktop project's folder does after a
/// save.
pub fn saved(
    read: impl IntoIterator<Item = SegmentDigest>,
    checkpoints: impl IntoIterator<Item = SegmentDigest>,
) {
    let read: HashSet<[u8; 32]> = read.into_iter().map(|digest| digest.0).collect();
    let checkpoints: HashSet<[u8; 32]> = checkpoints.into_iter().map(|digest| digest.0).collect();
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
            .chain(checkpoints)
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
