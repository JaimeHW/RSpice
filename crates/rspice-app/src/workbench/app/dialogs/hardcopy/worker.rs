//! Browser transport and application-owned source capture for hardcopy workers.

#[cfg(target_arch = "wasm32")]
pub(crate) use rspice_hardcopy::worker::decode_packaged_publication;
#[cfg(target_arch = "wasm32")]
use rspice_hardcopy::worker::validate_response_buffer_lengths;
#[cfg(target_arch = "wasm32")]
use rspice_hardcopy::worker::{
    HARDCOPY_WORKER_PROTOCOL_VERSION, HardcopyWorkerCommand, HardcopyWorkerOperation,
    HardcopyWorkerRequest, publication_part_count,
};

#[cfg(target_arch = "wasm32")]
mod browser {
    use std::cell::{Cell, RefCell};
    use std::rc::Rc;

    use js_sys::{Array, Object, Reflect, Uint8Array};
    use wasm_bindgen::JsCast as _;
    use wasm_bindgen::prelude::*;

    use super::{
        HARDCOPY_WORKER_PROTOCOL_VERSION, HardcopyWorkerCommand, HardcopyWorkerOperation,
        HardcopyWorkerRequest, publication_part_count, validate_response_buffer_lengths,
    };

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub(crate) struct HardcopyWorkerTicket {
        pub(crate) id: u32,
        pub(crate) epoch: u64,
        pub(crate) generation: u64,
        pub(crate) operation: HardcopyWorkerOperation,
        expected_buffer_count: Option<usize>,
    }

    /// Where a render lands: empty until the worker answers, then either the
    /// page buffers or the failure text, written exactly once.
    type RenderSlot = RefCell<Option<Result<Vec<Vec<u8>>, String>>>;

    struct ActiveHardcopyWorker {
        ticket: HardcopyWorkerTicket,
        worker: web_sys::Worker,
        result: Rc<RenderSlot>,
        _onmessage: Closure<dyn FnMut(web_sys::MessageEvent)>,
        _onerror: Closure<dyn FnMut(web_sys::ErrorEvent)>,
        _onmessageerror: Closure<dyn FnMut(web_sys::MessageEvent)>,
        deadline: crate::time_compat::Instant,
    }

    impl Drop for ActiveHardcopyWorker {
        fn drop(&mut self) {
            self.worker.set_onmessage(None);
            self.worker.set_onerror(None);
            self.worker.set_onmessageerror(None);
            self.worker.terminate();
        }
    }

    thread_local! {
        static NEXT_REQUEST_ID: Cell<u32> = const { Cell::new(0) };
        static ACTIVE_WORKER: RefCell<Option<ActiveHardcopyWorker>> = const { RefCell::new(None) };
    }

    pub(crate) fn start_source_resolution(
        prepared: crate::workbench::hardcopy_adapters::sources::PreparedRetainedHardcopyResolution,
        source_key: String,
        scope: rspice_hardcopy_contract::HardcopyScope,
        epoch: u64,
        generation: u64,
        repaint: egui::Context,
    ) -> Result<HardcopyWorkerTicket, String> {
        let snapshot = prepared
            .into_worker_snapshot_json()
            .map_err(|error| error.to_string())?;
        start(
            epoch,
            generation,
            HardcopyWorkerCommand::ResolveSource { source_key, scope },
            vec![snapshot],
            repaint,
        )
    }

    pub(crate) fn start_preview(
        plan: &rspice_hardcopy_contract::HardcopyPlan,
        source: &crate::workbench::hardcopy_adapters::sources::ResolvedHardcopyDocument,
        metadata: rspice_hardcopy::render::HardcopySceneMetadata,
        page_indices: Vec<usize>,
        dpi: u16,
        epoch: u64,
        generation: u64,
        repaint: egui::Context,
    ) -> Result<HardcopyWorkerTicket, String> {
        let snapshot = source
            .worker_snapshot_json()
            .map_err(|error| error.to_string())?;
        start(
            epoch,
            generation,
            HardcopyWorkerCommand::Preview {
                plan_id: plan.id(),
                expected_plan_digest: plan.content_digest(),
                setup: plan.setup().clone(),
                metadata,
                page_indices,
                dpi,
            },
            vec![snapshot],
            repaint,
        )
    }

    pub(crate) fn start_publication(
        plan: &rspice_hardcopy_contract::HardcopyPlan,
        source: &crate::workbench::hardcopy_adapters::sources::ResolvedHardcopyDocument,
        metadata: rspice_hardcopy::render::HardcopySceneMetadata,
        package_multi_part: bool,
        epoch: u64,
        generation: u64,
        repaint: egui::Context,
    ) -> Result<HardcopyWorkerTicket, String> {
        let snapshot = source
            .worker_snapshot_json()
            .map_err(|error| error.to_string())?;
        start(
            epoch,
            generation,
            HardcopyWorkerCommand::Publication {
                plan_id: plan.id(),
                expected_plan_digest: plan.content_digest(),
                expected_part_count: publication_part_count(plan),
                package_multi_part,
                setup: plan.setup().clone(),
                metadata,
            },
            vec![snapshot],
            repaint,
        )
    }

    fn start(
        epoch: u64,
        generation: u64,
        command: HardcopyWorkerCommand,
        buffers: Vec<Vec<u8>>,
        repaint: egui::Context,
    ) -> Result<HardcopyWorkerTicket, String> {
        if ACTIVE_WORKER.with(|active| active.borrow().is_some()) {
            return Err("A browser hardcopy worker operation is already active.".to_owned());
        }
        let id = allocate_request_id();
        let request = HardcopyWorkerRequest::try_new(id, epoch, generation, command)?;
        let operation = request.operation();
        let expected_buffer_count = Some(request.expected_response_buffer_count());
        let metadata = serde_wasm_bindgen::to_value(&request)
            .map_err(|error| format!("Could not encode hardcopy worker request: {error}"))?;
        let worker_url = worker_url()?;
        let options = web_sys::WorkerOptions::new();
        options.set_type(web_sys::WorkerType::Module);
        let worker =
            web_sys::Worker::new_with_options(&worker_url, &options).map_err(js_error_message)?;
        let ticket = HardcopyWorkerTicket {
            id,
            epoch,
            generation,
            operation,
            expected_buffer_count,
        };
        let result = Rc::new(RefCell::new(None));

        let message_result = Rc::clone(&result);
        let message_repaint = repaint.clone();
        let onmessage = Closure::<dyn FnMut(web_sys::MessageEvent)>::wrap(Box::new(
            move |event: web_sys::MessageEvent| {
                let data = event.data();
                let message_type = string_property(&data, "type");
                if message_type.as_deref() == Some("hardcopy-result") {
                    if numeric_property(&data, "id") != Some(id) {
                        complete_once(
                            &message_result,
                            &message_repaint,
                            Err(
                                "Browser hardcopy worker returned a stale outer response id."
                                    .to_owned(),
                            ),
                        );
                        return;
                    }
                    let parsed = Reflect::get(&data, &JsValue::from_str("response"))
                        .map_err(js_error_message)
                        .and_then(|response| parse_response(&response, ticket));
                    complete_once(&message_result, &message_repaint, parsed);
                } else if matches!(
                    message_type.as_deref(),
                    Some("hardcopy-error") | Some("error")
                ) {
                    let response_id = numeric_property(&data, "id").unwrap_or(0);
                    if response_id != id && response_id != 0 {
                        complete_once(
                            &message_result,
                            &message_repaint,
                            Err("Browser hardcopy worker returned a stale outer error id."
                                .to_owned()),
                        );
                        return;
                    }
                    complete_once(
                        &message_result,
                        &message_repaint,
                        Err(string_property(&data, "error")
                            .or_else(|| string_property(&data, "message"))
                            .unwrap_or_else(|| "Browser hardcopy worker failed.".to_owned())),
                    );
                } else if message_type.as_deref() != Some("ready") {
                    complete_once(
                        &message_result,
                        &message_repaint,
                        Err("Browser hardcopy worker returned an unexpected message.".to_owned()),
                    );
                }
            },
        ));
        worker.set_onmessage(Some(onmessage.as_ref().unchecked_ref()));

        let error_result = Rc::clone(&result);
        let error_repaint = repaint.clone();
        let onerror = Closure::<dyn FnMut(web_sys::ErrorEvent)>::wrap(Box::new(
            move |event: web_sys::ErrorEvent| {
                complete_once(
                    &error_result,
                    &error_repaint,
                    Err(if event.message().is_empty() {
                        "Browser hardcopy worker failed.".to_owned()
                    } else {
                        event.message()
                    }),
                );
            },
        ));
        worker.set_onerror(Some(onerror.as_ref().unchecked_ref()));

        let message_error_result = Rc::clone(&result);
        let message_error_repaint = repaint;
        let onmessageerror = Closure::<dyn FnMut(web_sys::MessageEvent)>::wrap(Box::new(
            move |_event: web_sys::MessageEvent| {
                complete_once(
                    &message_error_result,
                    &message_error_repaint,
                    Err("Browser hardcopy worker returned an unreadable message.".to_owned()),
                );
            },
        ));
        worker.set_onmessageerror(Some(onmessageerror.as_ref().unchecked_ref()));

        let message = Object::new();
        Reflect::set(
            &message,
            &JsValue::from_str("type"),
            &JsValue::from_str("run-hardcopy"),
        )
        .map_err(js_error_message)?;
        Reflect::set(
            &message,
            &JsValue::from_str("id"),
            &JsValue::from_f64(f64::from(id)),
        )
        .map_err(js_error_message)?;
        let request_value = Object::new();
        Reflect::set(&request_value, &JsValue::from_str("metadata"), &metadata)
            .map_err(js_error_message)?;
        let views = Array::new();
        let transfer = Array::new();
        for bytes in buffers {
            let view = Uint8Array::from(bytes.as_slice());
            transfer.push(&view.buffer());
            views.push(&view);
        }
        Reflect::set(&request_value, &JsValue::from_str("buffers"), &views)
            .map_err(js_error_message)?;
        Reflect::set(&message, &JsValue::from_str("request"), &request_value)
            .map_err(js_error_message)?;

        ACTIVE_WORKER.with(|active| {
            *active.borrow_mut() = Some(ActiveHardcopyWorker {
                ticket,
                worker: worker.clone(),
                result,
                _onmessage: onmessage,
                _onerror: onerror,
                _onmessageerror: onmessageerror,
                deadline: crate::time_compat::Instant::now()
                    + std::time::Duration::from_secs(match operation {
                        HardcopyWorkerOperation::ResolveSource
                        | HardcopyWorkerOperation::Preview => 120,
                        HardcopyWorkerOperation::Publication
                        | HardcopyWorkerOperation::PackagedPublication => 1_800,
                    }),
            });
        });
        if let Err(error) = worker.post_message_with_transfer(&message, &transfer) {
            cancel();
            return Err(format!(
                "Could not dispatch browser hardcopy work: {}",
                js_error_message(error)
            ));
        }
        Ok(ticket)
    }

    pub(crate) fn poll(expected: HardcopyWorkerTicket) -> Option<Result<Vec<Vec<u8>>, String>> {
        ACTIVE_WORKER.with(|active| {
            let mut active = active.borrow_mut();
            if active.as_ref().map(|worker| worker.ticket) != Some(expected) {
                return None;
            }
            if active
                .as_ref()
                .is_some_and(|worker| crate::time_compat::Instant::now() >= worker.deadline)
            {
                active.take();
                return Some(Err(
                    "Browser hardcopy worker exceeded its bounded execution deadline.".to_owned(),
                ));
            }
            let result = active
                .as_ref()
                .and_then(|worker| worker.result.borrow_mut().take());
            if result.is_some() {
                active.take();
            }
            result
        })
    }

    pub(crate) fn cancel() {
        ACTIVE_WORKER.with(|active| {
            active.borrow_mut().take();
        });
    }

    pub(crate) fn is_active() -> bool {
        ACTIVE_WORKER.with(|active| active.borrow().is_some())
    }

    fn parse_response(
        response: &JsValue,
        expected: HardcopyWorkerTicket,
    ) -> Result<Vec<Vec<u8>>, String> {
        if numeric_property(response, "protocolVersion") != Some(HARDCOPY_WORKER_PROTOCOL_VERSION) {
            return Err("Browser hardcopy worker returned an unsupported protocol.".to_owned());
        }
        if numeric_property(response, "id") != Some(expected.id) {
            return Err("Browser hardcopy worker returned a stale response id.".to_owned());
        }
        if string_property(response, "epoch") != Some(expected.epoch.to_string()) {
            return Err("Browser hardcopy worker returned a stale epoch.".to_owned());
        }
        if string_property(response, "generation") != Some(expected.generation.to_string()) {
            return Err("Browser hardcopy worker returned a stale generation.".to_owned());
        }
        if string_property(response, "operation").as_deref() != Some(expected.operation.as_str()) {
            return Err("Browser hardcopy worker returned the wrong operation.".to_owned());
        }
        let buffers = Reflect::get(response, &JsValue::from_str("buffers"))
            .map_err(js_error_message)?
            .dyn_into::<Array>()
            .map_err(|_| "Browser hardcopy response buffers are not an array.".to_owned())?;
        let buffer_count = buffers.length() as usize;
        let mut views = Vec::with_capacity(buffer_count);
        let mut lengths = Vec::with_capacity(buffer_count);
        for index in 0..buffers.length() {
            let view = buffers.get(index).dyn_into::<Uint8Array>().map_err(|_| {
                "Browser hardcopy response contains a non-Uint8Array buffer.".to_owned()
            })?;
            lengths.push(view.byte_length() as usize);
            views.push(view);
        }
        validate_response_buffer_lengths(
            expected.operation,
            expected.expected_buffer_count,
            lengths,
        )?;
        Ok(views.into_iter().map(|view| view.to_vec()).collect())
    }

    fn complete_once(
        slot: &RenderSlot,
        repaint: &egui::Context,
        result: Result<Vec<Vec<u8>>, String>,
    ) {
        let mut slot = slot.borrow_mut();
        if slot.is_none() {
            *slot = Some(result);
            repaint.request_repaint();
        }
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
        .ok_or_else(|| "Browser hardcopy worker URL is unavailable.".to_owned())
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
pub(crate) use browser::{
    HardcopyWorkerTicket, cancel, is_active, poll, start_preview, start_publication,
    start_source_resolution,
};
