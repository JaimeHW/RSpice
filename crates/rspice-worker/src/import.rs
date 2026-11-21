//! The browser's import worker: one picked result file read as it arrives, on a worker the page
//! starts when an import starts and lets go when it ends, so no reader of a file format loads
//! with the page or with the simulation and store workers.
//!
//! A binary rawfile streams: its rows are checked and laid out here as the results store segment
//! takes them, and go to the store worker through the page, which relays them and reads nothing
//! of the file. What the review shows comes back with the digest the store kept the rows under.
//! Any other file is answered as one to read whole.

use js_sys::{Array, Float64Array, Function, Promise, Reflect, Uint8Array};
use rspice_formats::result_import::identify_result_import_format;
use rspice_formats::spice_raw::streamed_import::{
    RawImport, RawImportPolicy, RawImportStart, StreamedRawOutcome,
};
use rspice_results::result_import::ResultImportFormat;
use rspice_results_store::RunMeta;
use rspice_simulation_contract::worker_transport::{
    KeptSegmentRecord, ResultsStoreRequest, ResultsStoreResponse, WorkerResultsSchema,
};
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;

/// The most of a file read before its header must have ended for its rows to be read as they
/// arrive.
const HEAD_BYTES: u64 = 1 << 20;

/// Bytes read between two feeds of the reader.
const FEED_BYTES: u64 = 8 << 20;

/// The bytes of rows one request to the store carries, as the page's own writes send them.
const BYTES_PER_MESSAGE: usize = 8 << 20;

/// What the store says when it has no room for the import.
const FULL: &str = "this browser's storage for RSpice is full: the import keeps nothing";

/// Import `file`, picked as `name`, under the page's `policy` (JSON): `store(request, buffers)`
/// asks the page to relay a results-store request and answers `{ response, buffers }`. Answers
/// the outcome's JSON.
pub async fn import_result_file(
    file: web_sys::Blob,
    name: String,
    policy: String,
    store: Function,
) -> Result<JsValue, JsValue> {
    let policy: RawImportPolicy =
        serde_json::from_str(&policy).map_err(|error| JsValue::from_str(&error.to_string()))?;
    let outcome = import(&file, &name, &policy, &store)
        .await
        .map_err(|reason| JsValue::from_str(&reason))?;
    serde_json::to_string(&outcome)
        .map(|outcome| JsValue::from_str(&outcome))
        .map_err(|error| JsValue::from_str(&error.to_string()))
}

async fn import(
    file: &web_sys::Blob,
    name: &str,
    policy: &RawImportPolicy,
    store: &Function,
) -> Result<StreamedRawOutcome, String> {
    let size = file.size();
    if !size.is_finite() || size < 0.0 {
        return Err(format!("{name} states no size the browser can read"));
    }
    let length = size as u64;
    let head = read(file, 0, length.min(HEAD_BYTES)).await?;
    if head.is_empty()
        || identify_result_import_format(name, &head).ok() != Some(ResultImportFormat::SpiceRaw)
    {
        return Ok(StreamedRawOutcome::WholeFile);
    }
    let mut import = match RawImport::start(&head, length, policy)? {
        RawImportStart::Streamed(import) => import,
        RawImportStart::WholeFile => return Ok(StreamedRawOutcome::WholeFile),
    };
    // The run is named as the desktop names it, so the same file keeps the same segment.
    let meta = RunMeta {
        analysis: format!("Imported \u{00b7} {name}"),
        ..RunMeta::default()
    };
    let schema = WorkerResultsSchema::of(&import.schema(), &meta)?;
    let write = match ask(
        store,
        &ResultsStoreRequest::Begin {
            schema,
            digest: None,
        },
        Array::new(),
    )
    .await?
    {
        ResultsStoreResponse::Begun { write } => write,
        ResultsStoreResponse::Full => return Err(FULL.to_owned()),
        other => return Err(refusal(other)),
    };
    let kept = match read_into(&mut import, file, length, &head, write, store).await {
        Ok(kept) => kept,
        Err(reason) => {
            let _ = ask(store, &ResultsStoreRequest::Abandon { write }, Array::new()).await;
            return Err(reason);
        }
    };
    let imported = import.imported()?;
    if kept.rows != imported.rows as u64 {
        return Err(format!(
            "the store kept {} rows of the {} the file holds",
            kept.rows, imported.rows
        ));
    }
    Ok(StreamedRawOutcome::Kept {
        digest: kept.digest,
        imported: Box::new(imported),
    })
}

/// Every row of the plot, checked and sent to the store: the rows in `head`, the file's first
/// bytes, then the rest of `file` from where `head` ended. Answers what the store kept.
async fn read_into(
    import: &mut RawImport,
    file: &web_sys::Blob,
    length: u64,
    head: &[u8],
    write: u32,
    store: &Function,
) -> Result<KeptSegmentRecord, String> {
    import.feed(head.get(import.data_offset()..).unwrap_or_default())?;
    send(import, write, store).await?;
    let mut read_to = head.len() as u64;
    while read_to < length {
        let next = length.min(read_to + FEED_BYTES);
        let bytes = read(file, read_to, next).await?;
        if bytes.is_empty() {
            break;
        }
        import.feed(&bytes)?;
        send(import, write, store).await?;
        read_to += bytes.len() as u64;
    }
    import.finish()?;
    send(import, write, store).await?;
    match ask(store, &ResultsStoreRequest::Finish { write }, Array::new()).await? {
        ResultsStoreResponse::Finished { full: true, .. } => Err(FULL.to_owned()),
        ResultsStoreResponse::Finished { record, .. } => Ok(record),
        other => Err(refusal(other)),
    }
}

/// Send the store the rows the reader laid out and has not sent, a message's worth at a time.
async fn send(import: &mut RawImport, write: u32, store: &Function) -> Result<(), String> {
    let (x, values) = import.pending();
    if !x.is_empty() {
        let slots = values.len() / x.len();
        let per = (BYTES_PER_MESSAGE / ((slots + 1) * 8)).max(1);
        for first in (0..x.len()).step_by(per) {
            let last = x.len().min(first + per);
            let buffers = Array::new();
            buffers.push(&Float64Array::from(&x[first..last]));
            buffers.push(&Float64Array::from(&values[first * slots..last * slots]));
            match ask(store, &ResultsStoreRequest::Rows { write }, buffers).await? {
                ResultsStoreResponse::Taken { full: false } => {}
                ResultsStoreResponse::Taken { full: true } => return Err(FULL.to_owned()),
                other => return Err(refusal(other)),
            }
        }
    }
    import.clear_pending();
    Ok(())
}

/// Ask the store `request` through the page, with `buffers` beside it.
async fn ask(
    store: &Function,
    request: &ResultsStoreRequest,
    buffers: Array,
) -> Result<ResultsStoreResponse, String> {
    let json = serde_json::to_string(request).map_err(|error| error.to_string())?;
    let promise = store
        .call2(&JsValue::NULL, &JsValue::from_str(&json), &buffers)
        .map_err(|error| message(&error))?
        .dyn_into::<Promise>()
        .map_err(|_| "the page answered no promise for the store".to_owned())?;
    let answer = JsFuture::from(promise)
        .await
        .map_err(|error| message(&error))?;
    let response = Reflect::get(&answer, &JsValue::from_str("response"))
        .ok()
        .and_then(|response| response.as_string())
        .ok_or("the store answered no response")?;
    serde_json::from_str(&response).map_err(|error| error.to_string())
}

/// The bytes of `file` from `start` to `end`.
async fn read(file: &web_sys::Blob, start: u64, end: u64) -> Result<Vec<u8>, String> {
    let slice = file
        .slice_with_f64_and_f64(start as f64, end as f64)
        .map_err(|error| message(&error))?;
    let buffer = JsFuture::from(slice.array_buffer())
        .await
        .map_err(|error| format!("the file could not be read: {}", message(&error)))?;
    Ok(Uint8Array::new(&buffer).to_vec())
}

fn refusal(response: ResultsStoreResponse) -> String {
    match response {
        ResultsStoreResponse::Failed { reason } => reason,
        other => format!("the results store answered {other:?}"),
    }
}

fn message(error: &JsValue) -> String {
    Reflect::get(error, &JsValue::from_str("message"))
        .ok()
        .and_then(|message| message.as_string())
        .or_else(|| error.as_string())
        .unwrap_or_else(|| format!("{error:?}"))
}
