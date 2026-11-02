//! Browser transport for model qualification: the model editor posts each
//! vector to the simulation worker and collects its outcome on a later frame.
//!
//! One worker serves a session's vectors in turn. The editor repaints every
//! frame while qualification runs, so an outcome is read by polling rather
//! than by waking the interface.

use std::cell::{Cell, RefCell};
use std::collections::{BTreeMap, BTreeSet};

use js_sys::{Object, Reflect};
use wasm_bindgen::JsCast as _;
use wasm_bindgen::prelude::*;

use super::{
    QualificationVectorResult, decode_qualification_worker_response,
    encode_qualification_worker_request,
};
use crate::js_error::js_error_message;
use rspice_model_library::qualification::{QualificationPlatform, QualificationVector};

struct QualificationWorker {
    worker: web_sys::Worker,
    _onmessage: Closure<dyn FnMut(web_sys::MessageEvent)>,
    _onerror: Closure<dyn FnMut(web_sys::ErrorEvent)>,
    _onmessageerror: Closure<dyn FnMut(web_sys::MessageEvent)>,
}

impl Drop for QualificationWorker {
    fn drop(&mut self) {
        self.worker.set_onmessage(None);
        self.worker.set_onerror(None);
        self.worker.set_onmessageerror(None);
        self.worker.terminate();
    }
}

thread_local! {
    static NEXT_REQUEST_ID: Cell<u32> = const { Cell::new(0) };
    static WORKER: RefCell<Option<QualificationWorker>> = const { RefCell::new(None) };
    static PENDING: RefCell<BTreeSet<u32>> = const { RefCell::new(BTreeSet::new()) };
    static ANSWERS: RefCell<BTreeMap<u32, Result<String, String>>> =
        const { RefCell::new(BTreeMap::new()) };
}

/// Post one vector to the browser simulation worker and return the request
/// whose outcome [`take_qualification_outcome`] collects.
pub fn post_qualification_vector(
    vector: &QualificationVector,
    platform: QualificationPlatform,
) -> Result<u32, String> {
    let request = encode_qualification_worker_request(vector, platform)?;
    let id = NEXT_REQUEST_ID.with(|next| {
        let id = next.get().wrapping_add(1).max(1);
        next.set(id);
        id
    });
    let message = Object::new();
    for (key, value) in [
        ("type", JsValue::from_str("run-model-qualification")),
        ("id", JsValue::from_f64(f64::from(id))),
        ("request", JsValue::from_str(&request)),
    ] {
        Reflect::set(&message, &JsValue::from_str(key), &value).map_err(js_error_message)?;
    }
    let worker = active_worker()?;
    PENDING.with(|pending| pending.borrow_mut().insert(id));
    if let Err(error) = worker.post_message(&message) {
        release_qualification_worker();
        return Err(format!(
            "the vector could not be posted to the simulation worker: {}",
            js_error_message(error)
        ));
    }
    Ok(id)
}

/// The outcome of a posted vector once the worker has answered; `None` while
/// it is still solving.
pub fn take_qualification_outcome(
    request: u32,
) -> Option<Result<QualificationVectorResult, String>> {
    let answer = ANSWERS.with(|answers| answers.borrow_mut().remove(&request))?;
    Some(answer.and_then(|response| decode_qualification_worker_response(&response)))
}

/// Stop the worker and forget every request it held.
pub fn release_qualification_worker() {
    WORKER.with(|worker| worker.borrow_mut().take());
    PENDING.with(|pending| pending.borrow_mut().clear());
    ANSWERS.with(|answers| answers.borrow_mut().clear());
}

fn active_worker() -> Result<web_sys::Worker, String> {
    if let Some(worker) =
        WORKER.with(|worker| worker.borrow().as_ref().map(|active| active.worker.clone()))
    {
        return Ok(worker);
    }
    let options = web_sys::WorkerOptions::new();
    options.set_type(web_sys::WorkerType::Module);
    let worker =
        web_sys::Worker::new_with_options(&worker_url()?, &options).map_err(js_error_message)?;

    let onmessage = Closure::<dyn FnMut(web_sys::MessageEvent)>::wrap(Box::new(
        |event: web_sys::MessageEvent| {
            let data = event.data();
            let Some(id) = numeric_property(&data, "id") else {
                return;
            };
            let answer = match string_property(&data, "type").as_deref() {
                Some("model-qualification-result") => string_property(&data, "response")
                    .ok_or_else(|| "the simulation worker answered without an outcome".to_owned()),
                Some("model-qualification-error") | Some("error") => {
                    Err(string_property(&data, "error")
                        .or_else(|| string_property(&data, "message"))
                        .unwrap_or_else(|| "the simulation worker failed".to_owned()))
                }
                _ => return,
            };
            answer_request(id, answer);
        },
    ));
    worker.set_onmessage(Some(onmessage.as_ref().unchecked_ref()));

    let onerror =
        Closure::<dyn FnMut(web_sys::ErrorEvent)>::wrap(Box::new(|event: web_sys::ErrorEvent| {
            answer_every_pending(if event.message().is_empty() {
                "the simulation worker failed".to_owned()
            } else {
                event.message()
            });
        }));
    worker.set_onerror(Some(onerror.as_ref().unchecked_ref()));

    let onmessageerror = Closure::<dyn FnMut(web_sys::MessageEvent)>::wrap(Box::new(
        |_event: web_sys::MessageEvent| {
            answer_every_pending("the simulation worker returned an unreadable message".to_owned());
        },
    ));
    worker.set_onmessageerror(Some(onmessageerror.as_ref().unchecked_ref()));

    WORKER.with(|active| {
        *active.borrow_mut() = Some(QualificationWorker {
            worker: worker.clone(),
            _onmessage: onmessage,
            _onerror: onerror,
            _onmessageerror: onmessageerror,
        });
    });
    Ok(worker)
}

fn answer_request(id: u32, answer: Result<String, String>) {
    if PENDING.with(|pending| pending.borrow_mut().remove(&id)) {
        ANSWERS.with(|answers| answers.borrow_mut().insert(id, answer));
    }
}

fn answer_every_pending(error: String) {
    let pending = PENDING.with(|pending| std::mem::take(&mut *pending.borrow_mut()));
    ANSWERS.with(|answers| {
        let mut answers = answers.borrow_mut();
        for id in pending {
            answers.insert(id, Err(error.clone()));
        }
    });
}

fn worker_url() -> Result<String, String> {
    Reflect::get(
        &js_sys::global(),
        &JsValue::from_str("__RSPICE_SIM_WORKER_URL"),
    )
    .map_err(js_error_message)?
    .as_string()
    .filter(|url| !url.trim().is_empty())
    .ok_or_else(|| "the simulation worker URL is unavailable".to_owned())
}

fn string_property(value: &JsValue, property: &str) -> Option<String> {
    Reflect::get(value, &JsValue::from_str(property))
        .ok()
        .and_then(|value| value.as_string())
}

fn numeric_property(value: &JsValue, property: &str) -> Option<u32> {
    Reflect::get(value, &JsValue::from_str(property))
        .ok()
        .and_then(|value| value.as_f64())
        .filter(|value| {
            value.is_finite()
                && *value >= 1.0
                && *value <= f64::from(u32::MAX)
                && value.fract() == 0.0
        })
        .map(|value| value as u32)
}
