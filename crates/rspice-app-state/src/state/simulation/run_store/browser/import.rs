//! A picked result file read in the browser's import worker: a worker of its own, which the page
//! starts when an import starts and lets go when it ends, so no reader of a file format loads
//! with the page. A binary rawfile streams there into the open project's store: the page relays
//! the store requests the worker makes, holds the store's one write for it, and reads nothing of
//! the file.

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

use js_sys::{Array, Promise, Reflect};
use rspice_formats::spice_raw::streamed_import::{
    RawImportPolicy, RawImported, StreamedRawOutcome,
};
use rspice_results::store_reader::{Segment, parse_digest};
use rspice_simulation_contract::worker_transport::{ResultsStoreRequest, ResultsStoreResponse};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::{JsCast, JsValue};

use super::{
    Standing, call, field, js_message, repaint, sleep, standing, stub, with, worker_url,
    write_queued,
};

/// The write the import worker began and has not ended, by its number in the store.
type OpenWrite = Rc<RefCell<Option<u32>>>;

/// Read the picked `file` in an import worker under `policy`. A binary rawfile streams into the
/// open project's store, and comes back as the plot it read with the segment the store keeps it
/// in, which this page reads through the store. `None` for any other file, and where this page
/// holds no store: the caller reads those whole.
pub async fn import_in_worker(
    file: &web_sys::File,
    policy: &RawImportPolicy,
) -> Result<Option<(Arc<Segment>, RawImported)>, String> {
    if standing() != Standing::Held {
        return Ok(None);
    }
    // The store takes one write at a time: the import's waits for the page's own, and holds
    // theirs back until it ends.
    loop {
        let taken = with(|client| {
            let free = !client.writing && !client.preloading;
            client.writing |= free;
            free
        });
        if taken {
            break;
        }
        sleep(20).await;
    }
    let open = OpenWrite::default();
    let outcome = read_in_worker(file, policy, &open).await;
    // A worker that stopped mid-write leaves its write to the page to end.
    let left = open.borrow_mut().take();
    if let Some(write) = left {
        let _ = call(&ResultsStoreRequest::Abandon { write }, Array::new()).await;
    }
    with(|client| client.writing = false);
    write_queued();
    repaint();
    match outcome? {
        StreamedRawOutcome::WholeFile => Ok(None),
        StreamedRawOutcome::Kept { digest, imported } => {
            let (segment, _) = stub(parse_digest(&digest)?).await?;
            Ok(Some((segment, *imported)))
        }
    }
}

/// The import worker's script, beside the simulation worker's and at its version.
fn import_worker_url() -> Result<String, String> {
    let simulation = worker_url()?;
    let base = web_sys::Url::new(&simulation).map_err(|error| js_message(&error))?;
    let url = web_sys::Url::new_with_base("import-worker.js", &simulation)
        .map_err(|error| js_message(&error))?;
    url.set_search(&base.search());
    Ok(url.href())
}

/// Start an import worker on `file`, relay its store requests until it answers, and let it go.
async fn read_in_worker(
    file: &web_sys::File,
    policy: &RawImportPolicy,
    open: &OpenWrite,
) -> Result<StreamedRawOutcome, String> {
    let options = web_sys::WorkerOptions::new();
    options.set_type(web_sys::WorkerType::Module);
    let worker = web_sys::Worker::new_with_options(&import_worker_url()?, &options)
        .map_err(|error| js_message(&error))?;
    let mut parts = None;
    let answered = Promise::new(&mut |resolve, reject| parts = Some((resolve, reject)));
    let (resolve, reject) = parts.ok_or("the import could not be started")?;
    let onmessage = {
        let (worker, open, reject) = (worker.clone(), Rc::clone(open), reject.clone());
        Closure::<dyn FnMut(web_sys::MessageEvent)>::new(move |event: web_sys::MessageEvent| {
            let data = event.data();
            match field(&data, "type")
                .as_string()
                .unwrap_or_default()
                .as_str()
            {
                "store" => relay(worker.clone(), Rc::clone(&open), data),
                "imported" => {
                    let _ = resolve.call1(&JsValue::UNDEFINED, &field(&data, "outcome"));
                }
                "failed" => {
                    let _ = reject.call1(&JsValue::UNDEFINED, &field(&data, "error"));
                }
                _ => {}
            }
        })
    };
    let onerror =
        Closure::<dyn FnMut(web_sys::ErrorEvent)>::new(move |event: web_sys::ErrorEvent| {
            let message = event.message();
            let reason = if message.is_empty() {
                "the import worker failed".to_owned()
            } else {
                message
            };
            let _ = reject.call1(&JsValue::UNDEFINED, &JsValue::from_str(&reason));
        });
    worker.set_onmessage(Some(onmessage.as_ref().unchecked_ref()));
    worker.set_onerror(Some(onerror.as_ref().unchecked_ref()));
    let started = start(&worker, file, policy);
    let answer = match started {
        Ok(()) => wasm_bindgen_futures::JsFuture::from(answered)
            .await
            .map_err(|error| js_message(&error)),
        Err(reason) => Err(reason),
    };
    worker.terminate();
    drop((onmessage, onerror));
    let outcome = answer?
        .as_string()
        .ok_or("the import worker answered no outcome")?;
    serde_json::from_str(&outcome).map_err(|error| error.to_string())
}

/// Hand the worker `file`, its name and `policy`.
fn start(
    worker: &web_sys::Worker,
    file: &web_sys::File,
    policy: &RawImportPolicy,
) -> Result<(), String> {
    let policy = serde_json::to_string(policy).map_err(|error| error.to_string())?;
    let message = js_sys::Object::new();
    for (name, value) in [
        ("type", JsValue::from_str("import")),
        ("file", file.clone().into()),
        ("name", JsValue::from_str(&file.name())),
        ("policy", JsValue::from_str(&policy)),
    ] {
        Reflect::set(&message, &JsValue::from_str(name), &value)
            .map_err(|error| js_message(&error))?;
    }
    worker
        .post_message(&message)
        .map_err(|error| js_message(&error))
}

/// Relay to the store one request of the import worker's write, and answer it with the store's.
/// The worker writes one segment: it opens, reads and deletes nothing.
fn relay(worker: web_sys::Worker, open: OpenWrite, data: JsValue) {
    wasm_bindgen_futures::spawn_local(async move {
        let id = field(&data, "id");
        let answer = relayed(&open, &data).await;
        let message = js_sys::Object::new();
        let set = |name: &str, value: &JsValue| {
            let _ = Reflect::set(&message, &JsValue::from_str(name), value);
        };
        set("id", &id);
        match answer {
            Ok((response, buffers)) => {
                set("type", &JsValue::from_str("store-result"));
                set("response", &JsValue::from_str(&response));
                set("buffers", &buffers);
            }
            Err(reason) => {
                set("type", &JsValue::from_str("store-error"));
                set("error", &JsValue::from_str(&reason));
            }
        }
        let _ = worker.post_message(&message);
    });
}

async fn relayed(open: &OpenWrite, data: &JsValue) -> Result<(String, Array), String> {
    let request = field(data, "request")
        .as_string()
        .ok_or("the import worker sent no request")?;
    let request: ResultsStoreRequest =
        serde_json::from_str(&request).map_err(|error| error.to_string())?;
    if !matches!(
        request,
        ResultsStoreRequest::Begin { .. }
            | ResultsStoreRequest::Rows { .. }
            | ResultsStoreRequest::Finish { .. }
            | ResultsStoreRequest::Abandon { .. }
    ) {
        return Err("the import worker asked the store for more than its write".to_owned());
    }
    let buffers = field(data, "buffers")
        .dyn_into::<Array>()
        .unwrap_or_else(|_| Array::new());
    let (response, buffers) = call(&request, buffers).await?;
    match (&request, &response) {
        (ResultsStoreRequest::Begin { .. }, ResultsStoreResponse::Begun { write }) => {
            *open.borrow_mut() = Some(*write);
        }
        (ResultsStoreRequest::Finish { .. } | ResultsStoreRequest::Abandon { .. }, _) => {
            *open.borrow_mut() = None;
        }
        _ => {}
    }
    if let ResultsStoreResponse::Finished { record, .. } = &response
        && let Ok(digest) = parse_digest(&record.digest)
    {
        with(|client| client.kept.insert(digest.0, record.bytes));
    }
    let response = serde_json::to_string(&response).map_err(|error| error.to_string())?;
    Ok((response, buffers))
}
