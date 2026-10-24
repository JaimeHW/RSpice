//! Browser-worker transport for governed Verilog-A compilation.
//!
//! Browser builds cannot use `std::thread`, and running the compiler from an
//! editor callback stalls input and rendering. This protocol transfers the exact
//! sealed project bundle to a dedicated module worker. The ordinary simulation
//! worker uses the same immutable JavaScript/Wasm assets, but a separate worker
//! instance and message namespace keep both operation lifecycles independent.

use crate::project_veriloga::worker::{WorkerCompileRequest, WorkerCompileTarget};
use serde::{Deserialize, Serialize};

use super::{VerilogACompileOutcome, project_compile_outcome, transport_failure_outcome};
use crate::project_veriloga::worker::WasmJitWorkerArtifact;
use rspice_design::project_sources::ProjectSourceBundle;
use rspice_veriloga::ExternalCell;
use rspice_veriloga::canonical_ir::digital::DigitalDelaySelect;

/// 3: a request carries the cells the design is compiled against.
/// 4: a request carries the min:typ:max selection the design is compiled
///    under when that is not `typ`.
const VERILOGA_WORKER_PROTOCOL_VERSION: u32 = 4;
const MAX_VERILOGA_WORKER_RESPONSE_BYTES: usize = 128 * 1024 * 1024;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct VerilogAWorkerRequest {
    protocol: u32,
    id: u32,
    bundle: ProjectSourceBundle,
    selected_module: Option<String>,
    /// The cells of the netlist the project's run builds, which the design
    /// may instantiate as leaves. Absent from an earlier protocol's request,
    /// which the protocol check refuses.
    #[serde(default)]
    cells: Option<Vec<ExternalCell>>,
    /// The value of each min:typ:max expression the netlist the project's
    /// run builds keeps. Absent for the typical values.
    #[serde(default, skip_serializing_if = "is_typical")]
    delay_select: DigitalDelaySelect,
}

fn is_typical(delay_select: &DigitalDelaySelect) -> bool {
    *delay_select == DigitalDelaySelect::Typ
}

impl VerilogAWorkerRequest {
    fn try_new(
        id: u32,
        bundle: &ProjectSourceBundle,
        selected_module: Option<&str>,
        cells: Option<&[ExternalCell]>,
        delay_select: DigitalDelaySelect,
    ) -> Result<Self, String> {
        let request = Self {
            protocol: VERILOGA_WORKER_PROTOCOL_VERSION,
            id,
            bundle: bundle.clone(),
            selected_module: selected_module.map(str::to_owned),
            cells: cells.map(<[_]>::to_vec),
            delay_select,
        };
        request.validated_source()?;
        Ok(request)
    }

    fn validated_source(&self) -> Result<WorkerCompileRequest<'_>, String> {
        if self.protocol != VERILOGA_WORKER_PROTOCOL_VERSION {
            return Err(format!(
                "Unsupported Verilog-A worker protocol {}; expected {}.",
                self.protocol, VERILOGA_WORKER_PROTOCOL_VERSION
            ));
        }
        if self.id == 0 {
            return Err("Verilog-A worker request id must be non-zero.".to_owned());
        }
        WorkerCompileRequest::try_new(&self.bundle, self.selected_module.as_deref())
            .map(|request| request.against(self.cells.as_deref(), self.delay_select))
            .map_err(|error| error.to_string())
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct VerilogAWorkerResponse {
    protocol: u32,
    id: u32,
    outcome: VerilogACompileOutcome,
    #[serde(rename = "wasmJitArtifact", skip_serializing_if = "Option::is_none")]
    wasm_jit_artifact: Option<WasmJitWorkerArtifact>,
    #[serde(rename = "wasmJitError", skip_serializing_if = "Option::is_none")]
    wasm_jit_error: Option<String>,
}

impl VerilogAWorkerResponse {
    fn from_request(
        request: VerilogAWorkerRequest,
        target: WorkerCompileTarget,
    ) -> Result<Self, String> {
        let source = request.validated_source()?;
        let compiled = source.compile(target);
        #[cfg(feature = "wasm-jit")]
        let (wasm_jit_artifact, wasm_jit_error) = match compiled
            .as_ref()
            .ok()
            .and_then(|compiled| compiled.wasm_jit.as_ref())
        {
            Some(Ok(artifact)) => (Some(WasmJitWorkerArtifact::from_compiled(artifact)), None),
            Some(Err(error)) => (None, Some(error.to_string())),
            None => (None, None),
        };
        #[cfg(not(feature = "wasm-jit"))]
        let (wasm_jit_artifact, wasm_jit_error) = (None, None);
        let outcome =
            project_compile_outcome(&request.bundle, compiled.map(|compiled| compiled.report));
        let response = Self {
            protocol: VERILOGA_WORKER_PROTOCOL_VERSION,
            id: request.id,
            outcome,
            wasm_jit_artifact,
            wasm_jit_error,
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
pub fn run_worker_request_value(
    value: wasm_bindgen::JsValue,
    target: WorkerCompileTarget,
) -> Result<wasm_bindgen::JsValue, wasm_bindgen::JsValue> {
    let request: VerilogAWorkerRequest = serde_wasm_bindgen::from_value(value)
        .map_err(|error| wasm_bindgen::JsValue::from_str(&error.to_string()))?;
    let response = VerilogAWorkerResponse::from_request(request, target)
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

    use super::{
        DigitalDelaySelect, ExternalCell, ProjectSourceBundle, VerilogACompileOutcome,
        VerilogAWorkerRequest,
    };
    use crate::js_error::js_error_message;

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
        bundle: &ProjectSourceBundle,
        selected_module: Option<&str>,
        cells: Option<&[ExternalCell]>,
        delay_select: DigitalDelaySelect,
        sender: mpsc::Sender<VerilogACompileOutcome>,
        wake: Rc<dyn Fn()>,
    ) -> Result<(), String> {
        if ACTIVE_WORKER.with(|active| active.borrow().is_some()) {
            return Err("A browser Verilog-A compile is already active.".to_owned());
        }

        let id = allocate_request_id();
        let request =
            VerilogAWorkerRequest::try_new(id, bundle, selected_module, cells, delay_select)?;
        let request = serde_wasm_bindgen::to_value(&request)
            .map_err(|error| format!("Could not encode Verilog-A worker request: {error}"))?;
        let worker_url = worker_url()?;
        let options = web_sys::WorkerOptions::new();
        options.set_type(web_sys::WorkerType::Module);
        let worker =
            web_sys::Worker::new_with_options(&worker_url, &options).map_err(js_error_message)?;
        let completed = Rc::new(Cell::new(false));

        let completion_sender = sender.clone();
        let completion_wake = wake.clone();
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
                            &completion_wake,
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
                            &completion_wake,
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
        let completion_wake = wake.clone();
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
                    &completion_wake,
                    &completion_guard,
                    Err(message),
                );
            },
        ));
        worker.set_onerror(Some(onerror.as_ref().unchecked_ref()));

        let completion_sender = sender;
        let completion_wake = wake;
        let completion_guard = completed;
        let onmessageerror = Closure::<dyn FnMut(web_sys::MessageEvent)>::wrap(Box::new(
            move |_event: web_sys::MessageEvent| {
                complete_once(
                    &completion_sender,
                    &completion_wake,
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

    pub fn cancel() {
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
        wake: &Rc<dyn Fn()>,
        completed: &Cell<bool>,
        result: Result<VerilogACompileOutcome, String>,
    ) {
        if completed.replace(true) {
            return;
        }
        let outcome = result.unwrap_or_else(super::transport_failure_outcome);
        let _ = sender.send(outcome);
        wake();
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
}

#[cfg(target_arch = "wasm32")]
pub use browser::cancel;
#[cfg(target_arch = "wasm32")]
pub(super) use browser::start;

#[cfg(test)]
mod tests {
    use super::*;
    use rspice_design::project_sources::{ProjectSourceLanguage, ProjectSourceOwner};

    const TYP: DigitalDelaySelect = DigitalDelaySelect::Typ;

    fn selected_source() -> ProjectSourceBundle {
        ProjectSourceBundle::try_new(
            ProjectSourceOwner::code_workspace(ProjectSourceLanguage::VerilogA),
            ProjectSourceLanguage::VerilogA,
            "worker.va",
            "module worker(p, n); inout p, n; electrical p, n; endmodule\n",
            [],
            [],
        )
        .unwrap()
    }

    fn selected_source_with_profile() -> ProjectSourceBundle {
        use rspice_design::project_sources::{
            ProjectSourceDependency, ProjectSourceFile, ProjectSourceRole, ProjectSourceRoleBinding,
        };
        let mut profile =
            crate::project_veriloga::build_profile::VerilogABuildProfile::starter("worker");
        profile.entry_modules.clear();
        ProjectSourceBundle::try_new_with_roles(
            ProjectSourceOwner::code_workspace(ProjectSourceLanguage::VerilogA),
            ProjectSourceLanguage::VerilogA,
            "worker.va",
            "module worker(p, n); inout p, n; electrical p, n; endmodule\n",
            [
                ProjectSourceFile::try_new(
                    ".rspice/veriloga-build.toml",
                    profile.to_toml().unwrap(),
                )
                .unwrap(),
            ],
            [
                ProjectSourceDependency::try_new("worker.va", ".rspice/veriloga-build.toml")
                    .unwrap(),
            ],
            [ProjectSourceRoleBinding::try_new(
                ".rspice/veriloga-build.toml",
                ProjectSourceRole::VerilogABuildProfile,
            )
            .unwrap()],
        )
        .unwrap()
    }

    #[test]
    fn request_and_response_round_trip_exact_compiled_artifacts() {
        let request =
            VerilogAWorkerRequest::try_new(7, &selected_source(), Some("worker"), None, TYP)
                .unwrap();
        let request: VerilogAWorkerRequest =
            serde_json::from_slice(&serde_json::to_vec(&request).unwrap()).unwrap();
        let response =
            VerilogAWorkerResponse::from_request(request, WorkerCompileTarget::Bytecode).unwrap();
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
        let request =
            VerilogAWorkerRequest::try_new(9, &selected_source(), Some("worker"), None, TYP)
                .unwrap();
        let response =
            VerilogAWorkerResponse::from_request(request, WorkerCompileTarget::Bytecode).unwrap();
        assert!(response.into_outcome(10).is_err());
    }

    #[test]
    fn malformed_transferred_target_matrix_is_rejected_without_panicking() {
        let request =
            VerilogAWorkerRequest::try_new(11, &selected_source(), Some("worker"), None, TYP)
                .unwrap();
        let response =
            VerilogAWorkerResponse::from_request(request, WorkerCompileTarget::Bytecode).unwrap();
        let mut encoded = serde_json::to_value(response).unwrap();
        encoded["outcome"]["Success"]["targets"]["entries"] = serde_json::json!([]);
        let response: VerilogAWorkerResponse = serde_json::from_value(encoded).unwrap();

        let error = response.into_outcome(11).unwrap_err();
        assert!(error.contains("target qualification matrix"), "{error}");
    }

    #[test]
    fn transferred_build_profile_is_revalidated_after_deserialization() {
        let request = VerilogAWorkerRequest::try_new(
            12,
            &selected_source_with_profile(),
            Some("worker"),
            None,
            TYP,
        )
        .unwrap();
        let mut encoded = serde_json::to_value(request).unwrap();
        encoded["bundle"]["files"][0]["content"] =
            serde_json::json!("schema = \"untrusted.profile/v99\"");
        let request: VerilogAWorkerRequest = serde_json::from_value(encoded).unwrap();

        let error = VerilogAWorkerResponse::from_request(request, WorkerCompileTarget::Bytecode)
            .unwrap_err();
        assert!(error.contains("build profile is invalid"), "{error}");
    }

    #[test]
    fn malformed_transferred_specialist_evidence_is_rejected() {
        let request =
            VerilogAWorkerRequest::try_new(13, &selected_source(), Some("worker"), None, TYP)
                .unwrap();
        let response =
            VerilogAWorkerResponse::from_request(request, WorkerCompileTarget::Bytecode).unwrap();
        let mut encoded = serde_json::to_value(response).unwrap();
        encoded["outcome"]["Success"]["specialist"]["module_name"] =
            serde_json::json!("different_module");
        let response: VerilogAWorkerResponse = serde_json::from_value(encoded).unwrap();

        let error = response.into_outcome(13).unwrap_err();
        assert!(error.contains("specialist report"), "{error}");
    }

    /// A design over `half`, a cell its project's deck defines, and over
    /// `missing`, which nothing defines.
    fn design_over(cell: &str) -> ProjectSourceBundle {
        ProjectSourceBundle::try_new(
            ProjectSourceOwner::code_workspace(ProjectSourceLanguage::VerilogA),
            ProjectSourceLanguage::VerilogA,
            "top.va",
            format!("module top(p, n); inout p, n; electrical p, n; {cell} xu(p, n); endmodule\n"),
            [],
            [],
        )
        .unwrap()
    }

    fn deck_cells() -> Vec<ExternalCell> {
        let cells = crate::project_veriloga::preparation::deck_external_cells(
            "* deck\n.subckt half a b\nr1 a b 1k\n.ends\n.end\n",
        );
        assert_eq!(cells.len(), 1, "the deck offers `half`");
        cells
    }

    fn assert_undefined(outcome: VerilogACompileOutcome, module: &str) {
        let VerilogACompileOutcome::Failure(diagnostics) = outcome else {
            panic!("`{module}` is defined nowhere in the project");
        };
        let reported = format!("{diagnostics:?}");
        assert!(
            reported.contains("Undefined module") && reported.contains(module),
            "{reported}"
        );
    }

    /// The editor compiles a design against the cells of the netlist its
    /// project's run builds, in-process on the desktop and in the browser's
    /// worker alike; a name nothing in the project defines is still refused,
    /// by name.
    #[test]
    fn both_editor_compiles_offer_the_project_cells_and_refuse_an_undefined_name() {
        let cells = deck_cells();
        let over_half = design_over("half");

        let VerilogACompileOutcome::Success(report) =
            super::super::compile_source(&over_half, Some("top"), Some(&cells), TYP)
        else {
            panic!("the desktop compile builds `half` from the project's cells");
        };
        assert_eq!(report.canonical_ir.hir.external_leaves.len(), 1);
        assert_undefined(
            super::super::compile_source(&over_half, Some("top"), None, TYP),
            "half",
        );
        assert_undefined(
            super::super::compile_source(&design_over("missing"), Some("top"), Some(&cells), TYP),
            "missing",
        );

        let through_worker = |id, bundle: &ProjectSourceBundle| {
            let request =
                VerilogAWorkerRequest::try_new(id, bundle, Some("top"), Some(&cells), TYP).unwrap();
            let request: VerilogAWorkerRequest =
                serde_json::from_slice(&serde_json::to_vec(&request).unwrap()).unwrap();
            let response =
                VerilogAWorkerResponse::from_request(request, WorkerCompileTarget::Bytecode)
                    .unwrap();
            let response: VerilogAWorkerResponse =
                serde_json::from_slice(&serde_json::to_vec(&response).unwrap()).unwrap();
            response.into_outcome(id).unwrap()
        };
        let VerilogACompileOutcome::Success(report) = through_worker(21, &over_half) else {
            panic!("the worker compile builds `half` from the transferred cells");
        };
        assert_eq!(report.canonical_ir.hir.external_leaves.len(), 1);
        assert_undefined(through_worker(22, &design_over("missing")), "missing");
    }

    /// A request of the protocol before the cell table, or of the one before
    /// the min:typ:max selection, is refused by its protocol, as every
    /// earlier protocol's request is.
    #[test]
    fn a_request_of_an_earlier_protocol_is_refused() {
        for (protocol, dropped) in [(2, "cells"), (3, "delay_select")] {
            let request = VerilogAWorkerRequest::try_new(
                23,
                &design_over("half"),
                Some("top"),
                Some(&deck_cells()),
                DigitalDelaySelect::Max,
            )
            .unwrap();
            let mut encoded = serde_json::to_value(request).unwrap();
            encoded["protocol"] = serde_json::json!(protocol);
            encoded.as_object_mut().unwrap().remove(dropped);
            let request: VerilogAWorkerRequest = serde_json::from_value(encoded).unwrap();

            let error =
                VerilogAWorkerResponse::from_request(request, WorkerCompileTarget::Bytecode)
                    .unwrap_err();
            assert_eq!(
                error,
                format!("Unsupported Verilog-A worker protocol {protocol}; expected 4.")
            );
        }
    }

    /// The browser's worker compiles a design under the min:typ:max
    /// selection of the netlist its project's run builds, which the request
    /// carries: the report it returns is the one the desktop compile builds
    /// under that selection, and a typical request carries no selection.
    #[test]
    fn the_worker_compiles_under_the_transferred_min_typ_max_selection() {
        let held = ProjectSourceBundle::try_new(
            ProjectSourceOwner::code_workspace(ProjectSourceLanguage::VerilogA),
            ProjectSourceLanguage::VerilogA,
            "held.va",
            "module held(p, n); inout p, n; electrical p, n;\n\
             parameter real g = 1.0e-3:2.0e-3:3.0e-3;\n\
             analog I(p, n) <+ g * V(p, n);\nendmodule\n",
            [],
            [],
        )
        .unwrap();
        let typical = serde_json::to_value(
            VerilogAWorkerRequest::try_new(31, &held, Some("held"), None, TYP).unwrap(),
        )
        .unwrap();
        assert!(typical.get("delay_select").is_none(), "{typical}");

        let request =
            VerilogAWorkerRequest::try_new(32, &held, Some("held"), None, DigitalDelaySelect::Min)
                .unwrap();
        let request: VerilogAWorkerRequest =
            serde_json::from_slice(&serde_json::to_vec(&request).unwrap()).unwrap();
        let response =
            VerilogAWorkerResponse::from_request(request, WorkerCompileTarget::Bytecode).unwrap();
        let response: VerilogAWorkerResponse =
            serde_json::from_slice(&serde_json::to_vec(&response).unwrap()).unwrap();
        let VerilogACompileOutcome::Success(from_worker) = response.into_outcome(32).unwrap()
        else {
            panic!("the worker compiles `held` under `min`");
        };
        let compiled = |delay_select| {
            let VerilogACompileOutcome::Success(report) =
                super::super::compile_source(&held, Some("held"), None, delay_select)
            else {
                panic!("the desktop compiles `held`");
            };
            serde_json::to_string(&report.model).unwrap()
        };
        let from_worker = serde_json::to_string(&from_worker.model).unwrap();
        assert_eq!(from_worker, compiled(DigitalDelaySelect::Min));
        assert_ne!(
            from_worker,
            compiled(TYP),
            "the minimum, not the typical value"
        );
    }

    #[test]
    fn transfer_failures_are_deterministic_editor_diagnostics() {
        let VerilogACompileOutcome::Failure(diagnostics) =
            transport_failure_outcome("decode failed".to_owned())
        else {
            panic!("transport failure cannot become success");
        };
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(
            diagnostics[0].message.as_ref(),
            "Browser compiler worker failed"
        );
        assert_eq!(diagnostics[0].detail.as_ref(), "decode failed");
    }
}
