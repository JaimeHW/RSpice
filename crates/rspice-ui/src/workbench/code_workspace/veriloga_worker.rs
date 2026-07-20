//! Browser-worker transport for governed Verilog-A compilation.
//!
//! Browser builds cannot use `std::thread`, and running the compiler from an
//! egui callback stalls input and rendering. This protocol transfers the exact
//! sealed project bundle to a dedicated module worker. The ordinary simulation
//! worker uses the same immutable JavaScript/Wasm assets, but a separate worker
//! instance and message namespace keep both operation lifecycles independent.

use serde::{Deserialize, Serialize};

use super::{SelectedVerilogASource, VerilogACompileOutcome};

const VERILOGA_WORKER_PROTOCOL_VERSION: u32 = 1;
const MAX_VERILOGA_WORKER_RESPONSE_BYTES: usize = 128 * 1024 * 1024;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct VerilogAWorkerRequest {
    protocol: u32,
    id: u32,
    bundle: crate::state::ProjectSourceBundle,
    selected_module: Option<String>,
}

impl VerilogAWorkerRequest {
    fn try_new(id: u32, selected: &SelectedVerilogASource) -> Result<Self, String> {
        let request = Self {
            protocol: VERILOGA_WORKER_PROTOCOL_VERSION,
            id,
            bundle: selected.bundle().clone(),
            selected_module: selected.selected_module().map(str::to_owned),
        };
        request.validate()?;
        Ok(request)
    }

    fn validate(&self) -> Result<(), String> {
        if self.protocol != VERILOGA_WORKER_PROTOCOL_VERSION {
            return Err(format!(
                "Unsupported Verilog-A worker protocol {}; expected {}.",
                self.protocol, VERILOGA_WORKER_PROTOCOL_VERSION
            ));
        }
        if self.id == 0 {
            return Err("Verilog-A worker request id must be non-zero.".to_owned());
        }
        if self.bundle.language() != crate::state::ProjectSourceLanguage::VerilogA {
            return Err("Verilog-A worker received a non-Verilog-A source bundle.".to_owned());
        }
        self.bundle
            .validate()
            .map_err(|error| format!("Verilog-A worker source bundle is invalid: {error}"))?;
        if self.selected_module.as_ref().is_some_and(|module| {
            module.is_empty()
                || module.len() > crate::state::MAX_PROJECT_SOURCE_LOGICAL_PATH_BYTES
                || module.chars().any(char::is_control)
        }) {
            return Err("Verilog-A worker module selection is invalid.".to_owned());
        }
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct VerilogAWorkerResponse {
    protocol: u32,
    id: u32,
    outcome: VerilogACompileOutcome,
}

impl VerilogAWorkerResponse {
    fn from_request(request: VerilogAWorkerRequest) -> Result<Self, String> {
        request.validate()?;
        let response = Self {
            protocol: VERILOGA_WORKER_PROTOCOL_VERSION,
            id: request.id,
            outcome: super::veriloga::compile_project_bundle_source(
                &request.bundle,
                request.selected_module.as_deref(),
            ),
        };
        response.validate_transfer_size()?;
        Ok(response)
    }

    fn into_outcome(self, expected_id: u32) -> Result<VerilogACompileOutcome, String> {
        if self.protocol != VERILOGA_WORKER_PROTOCOL_VERSION {
            return Err(format!(
                "Unsupported Verilog-A worker response protocol {}; expected {}.",
                self.protocol, VERILOGA_WORKER_PROTOCOL_VERSION
            ));
        }
        if self.id != expected_id {
            return Err(format!(
                "Stale Verilog-A worker response id {}; expected {}.",
                self.id, expected_id
            ));
        }
        if let VerilogACompileOutcome::Success(report) = &self.outcome {
            report.validate_integrity().map_err(|error| {
                format!("Verilog-A worker returned an invalid runtime artifact: {error}")
            })?;
        }
        self.validate_transfer_size()?;
        Ok(self.outcome)
    }

    fn validate_transfer_size(&self) -> Result<(), String> {
        let bytes = serde_json::to_vec(self)
            .map_err(|error| format!("Could not encode Verilog-A worker response: {error}"))?;
        if bytes.len() > MAX_VERILOGA_WORKER_RESPONSE_BYTES {
            return Err(format!(
                "Verilog-A worker response exceeds the {}-byte transfer limit.",
                MAX_VERILOGA_WORKER_RESPONSE_BYTES
            ));
        }
        Ok(())
    }
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn run_worker_request_value(
    value: wasm_bindgen::JsValue,
) -> Result<wasm_bindgen::JsValue, wasm_bindgen::JsValue> {
    let request: VerilogAWorkerRequest = serde_wasm_bindgen::from_value(value)
        .map_err(|error| wasm_bindgen::JsValue::from_str(&error.to_string()))?;
    let response = VerilogAWorkerResponse::from_request(request)
        .map_err(|error| wasm_bindgen::JsValue::from_str(&error))?;
    serde_wasm_bindgen::to_value(&response)
        .map_err(|error| wasm_bindgen::JsValue::from_str(&error.to_string()))
}

#[cfg(target_arch = "wasm32")]
mod browser {
    use std::cell::{Cell, RefCell};
    use std::rc::Rc;
    use std::sync::mpsc;

    use js_sys::{Object, Reflect};
    use wasm_bindgen::JsCast as _;
    use wasm_bindgen::prelude::*;

    use super::{SelectedVerilogASource, VerilogACompileOutcome, VerilogAWorkerRequest};

    struct ActiveCompileWorker {
        id: u32,
        worker: web_sys::Worker,
        _onmessage: Closure<dyn FnMut(web_sys::MessageEvent)>,
        _onerror: Closure<dyn FnMut(web_sys::ErrorEvent)>,
        _onmessageerror: Closure<dyn FnMut(web_sys::MessageEvent)>,
    }

    impl Drop for ActiveCompileWorker {
        fn drop(&mut self) {
            self.worker.set_onmessage(None);
            self.worker.set_onerror(None);
            self.worker.set_onmessageerror(None);
            self.worker.terminate();
        }
    }

    thread_local! {
        static NEXT_REQUEST_ID: Cell<u32> = const { Cell::new(0) };
        static ACTIVE_WORKER: RefCell<Option<ActiveCompileWorker>> = const { RefCell::new(None) };
    }

    pub(crate) fn start(
        selected: &SelectedVerilogASource,
        sender: mpsc::Sender<VerilogACompileOutcome>,
        repaint: egui::Context,
    ) -> Result<(), String> {
        if ACTIVE_WORKER.with(|active| active.borrow().is_some()) {
            return Err("A browser Verilog-A compile is already active.".to_owned());
        }

        let id = allocate_request_id();
        let request = VerilogAWorkerRequest::try_new(id, selected)?;
        let request = serde_wasm_bindgen::to_value(&request)
            .map_err(|error| format!("Could not encode Verilog-A worker request: {error}"))?;
        let worker_url = worker_url()?;
        let options = web_sys::WorkerOptions::new();
        options.set_type(web_sys::WorkerType::Module);
        let worker =
            web_sys::Worker::new_with_options(&worker_url, &options).map_err(js_error_message)?;
        let completed = Rc::new(Cell::new(false));

        let completion_sender = sender.clone();
        let completion_repaint = repaint.clone();
        let completion_guard = Rc::clone(&completed);
        let onmessage = Closure::<dyn FnMut(web_sys::MessageEvent)>::wrap(Box::new(
            move |event: web_sys::MessageEvent| {
                let data = event.data();
                match string_property(&data, "type").as_deref() {
                    Some("veriloga-result") => {
                        if numeric_property(&data, "id") != Some(id) {
                            return;
                        }
                        let outcome = Reflect::get(&data, &JsValue::from_str("response"))
                            .map_err(js_error_message)
                            .and_then(|value| {
                                serde_wasm_bindgen::from_value::<super::VerilogAWorkerResponse>(
                                    value,
                                )
                                .map_err(|error| error.to_string())
                            })
                            .and_then(|response| response.into_outcome(id));
                        complete_once(
                            &completion_sender,
                            &completion_repaint,
                            &completion_guard,
                            outcome,
                        );
                    }
                    Some("veriloga-error") | Some("error") => {
                        let response_id = numeric_property(&data, "id").unwrap_or(0);
                        if response_id != id && response_id != 0 {
                            return;
                        }
                        let message = string_property(&data, "error")
                            .or_else(|| string_property(&data, "message"))
                            .unwrap_or_else(|| {
                                "Browser Verilog-A compiler worker failed.".to_owned()
                            });
                        complete_once(
                            &completion_sender,
                            &completion_repaint,
                            &completion_guard,
                            Err(message),
                        );
                    }
                    _ => {}
                }
            },
        ));
        worker.set_onmessage(Some(onmessage.as_ref().unchecked_ref()));

        let completion_sender = sender.clone();
        let completion_repaint = repaint.clone();
        let completion_guard = Rc::clone(&completed);
        let onerror = Closure::<dyn FnMut(web_sys::ErrorEvent)>::wrap(Box::new(
            move |event: web_sys::ErrorEvent| {
                let message = if event.message().is_empty() {
                    "Browser Verilog-A compiler worker failed.".to_owned()
                } else {
                    event.message()
                };
                complete_once(
                    &completion_sender,
                    &completion_repaint,
                    &completion_guard,
                    Err(message),
                );
            },
        ));
        worker.set_onerror(Some(onerror.as_ref().unchecked_ref()));

        let completion_sender = sender;
        let completion_repaint = repaint;
        let completion_guard = completed;
        let onmessageerror = Closure::<dyn FnMut(web_sys::MessageEvent)>::wrap(Box::new(
            move |_event: web_sys::MessageEvent| {
                complete_once(
                    &completion_sender,
                    &completion_repaint,
                    &completion_guard,
                    Err(
                        "Browser Verilog-A compiler worker returned an unreadable message."
                            .to_owned(),
                    ),
                );
            },
        ));
        worker.set_onmessageerror(Some(onmessageerror.as_ref().unchecked_ref()));

        let message = Object::new();
        Reflect::set(
            &message,
            &JsValue::from_str("type"),
            &JsValue::from_str("compile-veriloga"),
        )
        .map_err(js_error_message)?;
        Reflect::set(
            &message,
            &JsValue::from_str("id"),
            &JsValue::from_f64(f64::from(id)),
        )
        .map_err(js_error_message)?;
        Reflect::set(&message, &JsValue::from_str("request"), &request)
            .map_err(js_error_message)?;

        ACTIVE_WORKER.with(|active| {
            *active.borrow_mut() = Some(ActiveCompileWorker {
                id,
                worker: worker.clone(),
                _onmessage: onmessage,
                _onerror: onerror,
                _onmessageerror: onmessageerror,
            });
        });
        if let Err(error) = worker.post_message(&message) {
            cancel();
            return Err(format!(
                "Could not dispatch browser Verilog-A compile: {}",
                js_error_message(error)
            ));
        }
        Ok(())
    }

    pub(crate) fn finish(expected_id: Option<u32>) {
        ACTIVE_WORKER.with(|active| {
            let remove = active
                .borrow()
                .as_ref()
                .is_some_and(|worker| expected_id.is_none_or(|id| worker.id == id));
            if remove {
                active.borrow_mut().take();
            }
        });
    }

    pub(crate) fn cancel() {
        finish(None);
    }

    fn allocate_request_id() -> u32 {
        NEXT_REQUEST_ID.with(|next| {
            let value = next.get().wrapping_add(1).max(1);
            next.set(value);
            value
        })
    }

    fn worker_url() -> Result<String, String> {
        Reflect::get(
            &js_sys::global(),
            &JsValue::from_str("__RSPICE_SIM_WORKER_URL"),
        )
        .map_err(js_error_message)?
        .as_string()
        .filter(|url| !url.trim().is_empty())
        .ok_or_else(|| "Browser Verilog-A compiler worker URL is unavailable.".to_owned())
    }

    fn complete_once(
        sender: &mpsc::Sender<VerilogACompileOutcome>,
        repaint: &egui::Context,
        completed: &Cell<bool>,
        result: Result<VerilogACompileOutcome, String>,
    ) {
        if completed.replace(true) {
            return;
        }
        let outcome = result.unwrap_or_else(super::transport_failure_outcome);
        let _ = sender.send(outcome);
        repaint.request_repaint();
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

    fn js_error_message(error: JsValue) -> String {
        error
            .as_string()
            .or_else(|| {
                Reflect::get(&error, &JsValue::from_str("message"))
                    .ok()
                    .and_then(|message| message.as_string())
            })
            .unwrap_or_else(|| "unknown JavaScript error".to_owned())
    }
}

#[cfg(target_arch = "wasm32")]
pub(crate) use browser::{cancel, start};

pub(super) fn transport_failure_outcome(message: String) -> VerilogACompileOutcome {
    VerilogACompileOutcome::Failure(vec![super::CodeEditorDiagnostic {
        severity: super::CodeEditorSeverity::Error,
        message: "Browser compiler worker failed".to_owned(),
        detail: message,
        source_path: None,
        source: None,
        byte_range: None,
        line: None,
        column: None,
    }])
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{ProjectSourceLanguage, ProjectSourceOwner};

    fn selected_source() -> SelectedVerilogASource {
        SelectedVerilogASource {
            bundle: crate::state::ProjectSourceBundle::try_new(
                ProjectSourceOwner::code_workspace(ProjectSourceLanguage::VerilogA),
                ProjectSourceLanguage::VerilogA,
                "worker.va",
                "module worker(p, n); inout p, n; electrical p, n; endmodule\n",
                [],
                [],
            )
            .unwrap(),
            selected_module: Some("worker".to_owned()),
        }
    }

    #[test]
    fn request_and_response_round_trip_exact_compiled_artifacts() {
        let request = VerilogAWorkerRequest::try_new(7, &selected_source()).unwrap();
        let request: VerilogAWorkerRequest =
            serde_json::from_slice(&serde_json::to_vec(&request).unwrap()).unwrap();
        let response = VerilogAWorkerResponse::from_request(request).unwrap();
        let response: VerilogAWorkerResponse =
            serde_json::from_slice(&serde_json::to_vec(&response).unwrap()).unwrap();

        let VerilogACompileOutcome::Success(report) = response.into_outcome(7).unwrap() else {
            panic!("valid worker fixture must compile");
        };
        assert_eq!(report.abi.module_name.as_str(), "worker");
        report.validate_integrity().unwrap();
    }

    #[test]
    fn stale_worker_response_ids_are_rejected() {
        let request = VerilogAWorkerRequest::try_new(9, &selected_source()).unwrap();
        let response = VerilogAWorkerResponse::from_request(request).unwrap();
        assert!(response.into_outcome(10).is_err());
    }

    #[test]
    fn malformed_transferred_target_matrix_is_rejected_without_panicking() {
        let request = VerilogAWorkerRequest::try_new(11, &selected_source()).unwrap();
        let response = VerilogAWorkerResponse::from_request(request).unwrap();
        let mut encoded = serde_json::to_value(response).unwrap();
        encoded["outcome"]["Success"]["targets"]["entries"] = serde_json::json!([]);
        let response: VerilogAWorkerResponse = serde_json::from_value(encoded).unwrap();

        let error = response.into_outcome(11).unwrap_err();
        assert!(error.contains("target qualification matrix"), "{error}");
    }

    #[test]
    fn browser_worker_script_has_a_separate_compile_protocol() {
        let source = include_str!("../../../web/simulation-worker.js");
        assert!(source.contains("compile-veriloga"));
        assert!(source.contains("runRspiceUiVerilogACompileRequest"));
        assert!(source.contains("veriloga-result"));
        assert!(source.contains("veriloga-error"));
    }

    #[test]
    fn transfer_failures_are_deterministic_editor_diagnostics() {
        let VerilogACompileOutcome::Failure(diagnostics) =
            transport_failure_outcome("decode failed".to_owned())
        else {
            panic!("transport failure cannot become success");
        };
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].message, "Browser compiler worker failed");
        assert_eq!(diagnostics[0].detail, "decode failed");
    }
}
