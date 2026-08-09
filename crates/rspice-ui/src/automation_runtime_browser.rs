//! Pinned Pyodide Web Worker transport for browser Automation.

use std::{cell::RefCell, collections::VecDeque, rc::Rc};

use js_sys::{Object, Reflect};
use rspice_automation_protocol::{
    Digest, EventEnvelope, RequestEnvelope, RuntimeEvent, RuntimeIdentity, RuntimePlatform,
    RuntimeRequest, RuntimeState, SourceSnapshot,
};
use semver::{Version, VersionReq};
use wasm_bindgen::{JsCast as _, JsValue, closure::Closure};

const WORKER_URL_GLOBAL: &str = "__RSPICE_AUTOMATION_WORKER_URL";
const EXPECTED_RUNTIME_DIGEST: [u8; 32] = [
    0xf7, 0xeb, 0x92, 0x6d, 0x7d, 0xa7, 0x2f, 0x7a, 0x2a, 0x62, 0xb2, 0xe1, 0x7d, 0xfb, 0x6d, 0xc8,
    0xbc, 0xa4, 0x36, 0x44, 0x6c, 0xfa, 0x03, 0x29, 0xc8, 0x95, 0xc4, 0x5d, 0xea, 0xa2, 0xa3, 0x60,
];
const BASE_ENVIRONMENT_DIGEST: [u8; 32] = [
    0xd4, 0x45, 0xb1, 0x44, 0x39, 0x65, 0xbe, 0x4e, 0x6b, 0x1b, 0x19, 0x1e, 0xe0, 0x23, 0x17, 0x6d,
    0xbd, 0x35, 0x43, 0x0a, 0xc3, 0xcd, 0x00, 0x60, 0x34, 0x58, 0x38, 0x4e, 0xa0, 0x3b, 0x85, 0x18,
];
const PYODIDE_VERSION: &str = "314.0.2";
const PYTHON_VERSION: &str = "3.14.2";
const RSPICE_API_VERSION: &str = "1.0.0";
/// Cold WebAssembly compilation is a runtime-startup concern, not user-code
/// wall time. It remains independently bounded so a corrupt or unsupported
/// browser cannot leave an Automation request pending forever.
const RUNTIME_STARTUP_LIMIT_MS: f64 = 120_000.0;

pub(crate) struct BrowserAutomationRuntime {
    worker: Option<ActiveWorker>,
    events: Rc<RefCell<VecDeque<Result<EventEnvelope, String>>>>,
    next_request_id: u64,
    last_sequence: u64,
    runtime_ready: bool,
    active_limits: Option<BrowserLimits>,
}

#[derive(Clone, Copy)]
struct BrowserLimits {
    request_id: u64,
    wall_time_ms: u64,
    deadline_ms: f64,
    runtime_ready: bool,
    output_bytes: u64,
    artifact_bytes: u64,
    observed_output_bytes: u64,
    observed_artifact_bytes: u64,
}

struct ActiveWorker {
    worker: web_sys::Worker,
    _onmessage: Closure<dyn FnMut(web_sys::MessageEvent)>,
    _onerror: Closure<dyn FnMut(web_sys::ErrorEvent)>,
    _onmessageerror: Closure<dyn FnMut(web_sys::MessageEvent)>,
}

impl Drop for ActiveWorker {
    fn drop(&mut self) {
        self.worker.set_onmessage(None);
        self.worker.set_onerror(None);
        self.worker.set_onmessageerror(None);
        self.worker.terminate();
    }
}

impl BrowserAutomationRuntime {
    pub(crate) fn discover() -> Self {
        Self {
            worker: None,
            events: Rc::new(RefCell::new(VecDeque::new())),
            next_request_id: 1,
            last_sequence: 0,
            runtime_ready: false,
            active_limits: None,
        }
    }

    pub(crate) fn availability_reason(&self) -> Option<String> {
        worker_url().err()
    }

    pub(crate) fn ensure_worker(&mut self) -> Result<(), String> {
        if self.worker.is_some() {
            return Ok(());
        }
        let url = worker_url()?;
        let options = web_sys::WorkerOptions::new();
        options.set_type(web_sys::WorkerType::Module);
        let worker = web_sys::Worker::new_with_options(&url, &options).map_err(js_error)?;

        let messages = Rc::clone(&self.events);
        let onmessage = Closure::<dyn FnMut(web_sys::MessageEvent)>::wrap(Box::new(
            move |event: web_sys::MessageEvent| {
                let data = event.data();
                if string_property(&data, "type").as_deref() != Some("automation-event") {
                    return;
                }
                let result = Reflect::get(&data, &JsValue::from_str("envelope"))
                    .map_err(js_error)
                    .and_then(|value| {
                        serde_wasm_bindgen::from_value::<EventEnvelope>(value)
                            .map_err(|error| error.to_string())
                    })
                    .and_then(|envelope| {
                        envelope.validate().map_err(|error| error.to_string())?;
                        Ok(envelope)
                    });
                messages.borrow_mut().push_back(result);
            },
        ));
        worker.set_onmessage(Some(onmessage.as_ref().unchecked_ref()));

        let errors = Rc::clone(&self.events);
        let onerror = Closure::<dyn FnMut(web_sys::ErrorEvent)>::wrap(Box::new(
            move |event: web_sys::ErrorEvent| {
                let message = if event.message().is_empty() {
                    "Browser Python worker failed without an error message.".to_owned()
                } else {
                    event.message()
                };
                errors.borrow_mut().push_back(Err(message));
            },
        ));
        worker.set_onerror(Some(onerror.as_ref().unchecked_ref()));

        let message_errors = Rc::clone(&self.events);
        let onmessageerror = Closure::<dyn FnMut(web_sys::MessageEvent)>::wrap(Box::new(
            move |_event: web_sys::MessageEvent| {
                message_errors.borrow_mut().push_back(Err(
                    "Browser Python worker returned an unreadable message.".to_owned(),
                ));
            },
        ));
        worker.set_onmessageerror(Some(onmessageerror.as_ref().unchecked_ref()));
        self.worker = Some(ActiveWorker {
            worker,
            _onmessage: onmessage,
            _onerror: onerror,
            _onmessageerror: onmessageerror,
        });
        self.last_sequence = 0;
        self.runtime_ready = false;
        Ok(())
    }

    pub(crate) fn send_request(&mut self, request: RuntimeRequest) -> Result<u64, String> {
        if let RuntimeRequest::Launch { snapshot, .. } = &request {
            validate_launch_snapshot(snapshot)?;
            if self.active_limits.is_some() {
                return Err("a browser Python launch is already active".to_owned());
            }
        }
        let request_id = self.next_request_id;
        let next_request_id = self
            .next_request_id
            .checked_add(1)
            .ok_or_else(|| "browser Python request identity space is exhausted".to_owned())?;
        let launch_limits = if let RuntimeRequest::Launch { limits, .. } = &request {
            let now = browser_now_ms()?;
            Some(BrowserLimits {
                request_id,
                wall_time_ms: limits.wall_time_ms,
                deadline_ms: now
                    + if self.runtime_ready {
                        limits.wall_time_ms as f64
                    } else {
                        RUNTIME_STARTUP_LIMIT_MS
                    },
                runtime_ready: self.runtime_ready,
                output_bytes: limits.output_bytes,
                artifact_bytes: limits.artifact_bytes,
                observed_output_bytes: 0,
                observed_artifact_bytes: 0,
            })
        } else {
            None
        };
        let envelope = RequestEnvelope {
            protocol: rspice_automation_protocol::PROTOCOL_VERSION,
            request_id,
            request,
        };
        envelope.validate().map_err(|error| error.to_string())?;
        let value = serde_wasm_bindgen::to_value(&envelope).map_err(|error| error.to_string())?;
        let message = Object::new();
        Reflect::set(
            &message,
            &JsValue::from_str("type"),
            &JsValue::from_str("automation-request"),
        )
        .map_err(js_error)?;
        Reflect::set(&message, &JsValue::from_str("envelope"), &value).map_err(js_error)?;
        let worker = self
            .worker
            .as_ref()
            .ok_or_else(|| "browser Python worker is not running".to_owned())?;
        if let Some(limits) = launch_limits {
            self.active_limits = Some(limits);
        }
        if let Err(error) = worker.worker.post_message(&message) {
            let error = js_error(error);
            // A transport that cannot accept a request is no longer a
            // trustworthy execution boundary. Terminate it so a failed
            // debugger/control message cannot accidentally remove or bypass
            // the active launch limits.
            let _ = self.terminate();
            return Err(error);
        }
        self.next_request_id = next_request_id;
        Ok(request_id)
    }

    pub(crate) fn poll_events(&mut self) -> Vec<Result<EventEnvelope, String>> {
        if let Some(limits) = self.active_limits {
            match browser_now_ms() {
                Ok(now) if now >= limits.deadline_ms => {
                    let message = if limits.runtime_ready {
                        format!(
                            "browser Python exceeded its {} ms wall-time limit",
                            limits.wall_time_ms
                        )
                    } else {
                        format!(
                            "the pinned browser Python runtime did not initialize within {} ms",
                            RUNTIME_STARTUP_LIMIT_MS as u64
                        )
                    };
                    let _ = self.terminate();
                    return vec![Err(message)];
                }
                Ok(_) => {}
                Err(error) => {
                    let _ = self.terminate();
                    return vec![Err(format!(
                        "browser Python resource clock failed closed: {error}"
                    ))];
                }
            }
        }
        let mut output = Vec::new();
        for _ in 0..256 {
            let Some(event) = self.events.borrow_mut().pop_front() else {
                break;
            };
            let event = match event {
                Ok(event) => event,
                Err(error) => {
                    output.push(Err(error));
                    break;
                }
            };
            if event.sequence <= self.last_sequence {
                output.push(Err(format!(
                    "browser Python event sequence regressed from {} to {}",
                    self.last_sequence, event.sequence
                )));
                break;
            }
            self.last_sequence = event.sequence;
            if let RuntimeEvent::Hello { identity } = &event.event
                && identity != &expected_identity()
            {
                output.push(Err(
                    "browser Python identity does not match the pinned RSpice runtime".to_owned(),
                ));
                break;
            }
            if matches!(&event.event, RuntimeEvent::Hello { .. }) {
                self.runtime_ready = true;
                if let Some(limits) = self.active_limits.as_mut()
                    && !limits.runtime_ready
                {
                    limits.runtime_ready = true;
                    match browser_now_ms() {
                        Ok(now) => limits.deadline_ms = now + limits.wall_time_ms as f64,
                        Err(error) => {
                            output.push(Err(format!(
                                "browser Python resource clock failed closed: {error}"
                            )));
                            break;
                        }
                    }
                }
            }
            if let Some(limits) = self
                .active_limits
                .as_mut()
                .filter(|limits| event.request_id == Some(limits.request_id))
            {
                match &event.event {
                    RuntimeEvent::Output { text, .. } => {
                        limits.observed_output_bytes = limits
                            .observed_output_bytes
                            .saturating_add(text.len() as u64);
                        if limits.observed_output_bytes > limits.output_bytes {
                            output.push(Err("browser Python exceeded its output limit".to_owned()));
                            break;
                        }
                    }
                    RuntimeEvent::ArtifactPublished { bytes, .. } => {
                        limits.observed_artifact_bytes =
                            limits.observed_artifact_bytes.saturating_add(*bytes);
                        if limits.observed_artifact_bytes > limits.artifact_bytes {
                            output
                                .push(Err("browser Python exceeded its artifact limit".to_owned()));
                            break;
                        }
                    }
                    RuntimeEvent::State { state, .. }
                        if matches!(
                            state,
                            RuntimeState::Cancelled
                                | RuntimeState::Completed
                                | RuntimeState::Failed
                                | RuntimeState::Terminated
                        ) =>
                    {
                        self.active_limits = None;
                    }
                    _ => {}
                }
            }
            output.push(Ok(event));
        }
        output
    }

    pub(crate) fn terminate(&mut self) -> Result<(), String> {
        self.worker = None;
        self.runtime_ready = false;
        self.active_limits = None;
        self.events.borrow_mut().clear();
        Ok(())
    }
}

fn expected_identity() -> RuntimeIdentity {
    RuntimeIdentity {
        managed: true,
        platform: RuntimePlatform::BrowserWasm,
        architecture: "wasm32".to_owned(),
        runtime_build: format!("pyodide-{PYODIDE_VERSION}"),
        runtime_digest: Digest(EXPECTED_RUNTIME_DIGEST),
        python_version: PYTHON_VERSION.to_owned(),
        python_abi: "cp314-emscripten_5_0_3-wasm32".to_owned(),
        rspice_api_version: RSPICE_API_VERSION.to_owned(),
        protocol: rspice_automation_protocol::PROTOCOL_VERSION,
    }
}

fn validate_launch_snapshot(snapshot: &SourceSnapshot) -> Result<(), String> {
    snapshot.validate().map_err(|error| error.to_string())?;
    require_version("Python", PYTHON_VERSION, &snapshot.python_requirement)?;
    require_version("RSpice API", RSPICE_API_VERSION, &snapshot.api_requirement)?;
    let browser_requirement = snapshot
        .browser_runtime_requirement
        .as_deref()
        .ok_or_else(|| {
            "the governed environment lock does not select a browser Python runtime".to_owned()
        })?;
    require_version(
        "browser Python runtime",
        PYODIDE_VERSION,
        browser_requirement,
    )?;
    if snapshot.environment_digest != Digest(BASE_ENVIRONMENT_DIGEST) {
        return Err(
            "the selected Python environment is not installed in this immutable browser release"
                .to_owned(),
        );
    }
    Ok(())
}

fn require_version(label: &str, actual: &str, requirement: &str) -> Result<(), String> {
    let actual = Version::parse(actual)
        .map_err(|error| format!("the pinned {label} version is invalid: {error}"))?;
    let requirement = VersionReq::parse(requirement)
        .map_err(|error| format!("the governed {label} requirement is invalid: {error}"))?;
    if !requirement.matches(&actual) {
        return Err(format!(
            "pinned {label} {actual} does not satisfy governed requirement {requirement}"
        ));
    }
    Ok(())
}

fn worker_url() -> Result<String, String> {
    let window = web_sys::window().ok_or_else(|| "browser window is unavailable".to_owned())?;
    let value = Reflect::get(&window, &JsValue::from_str(WORKER_URL_GLOBAL)).map_err(js_error)?;
    value
        .as_string()
        .filter(|value| !value.trim().is_empty())
        .ok_or_else(|| "the pinned browser Python worker URL is not installed".to_owned())
}

fn browser_now_ms() -> Result<f64, String> {
    web_sys::window()
        .and_then(|window| window.performance())
        .map(|performance| performance.now())
        .ok_or_else(|| "browser performance clock is unavailable".to_owned())
}

fn string_property(value: &JsValue, name: &str) -> Option<String> {
    Reflect::get(value, &JsValue::from_str(name))
        .ok()
        .and_then(|value| value.as_string())
}

fn js_error(error: JsValue) -> String {
    Reflect::get(&error, &JsValue::from_str("message"))
        .ok()
        .and_then(|value| value.as_string())
        .unwrap_or_else(|| format!("{error:?}"))
}
