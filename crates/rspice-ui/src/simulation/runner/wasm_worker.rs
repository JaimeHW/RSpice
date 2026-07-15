/// Worker IDs cross the JavaScript number boundary twice. Keeping them in the
/// exact u32 integer range avoids precision loss while still leaving more than
/// four billion collision-free requests between wraps.
const MAX_BROWSER_REQUEST_ID: u64 = u32::MAX as u64;

pub(crate) fn next_request_id(current: u64) -> u64 {
    if current >= MAX_BROWSER_REQUEST_ID {
        1
    } else {
        current + 1
    }
}

pub(crate) fn stale_result(active: Option<u64>, incoming: u64) -> bool {
    active != Some(incoming)
}

fn request_id_from_js_number(value: f64) -> Option<u64> {
    (value.is_finite()
        && value >= 1.0
        && value <= MAX_BROWSER_REQUEST_ID as f64
        && value.fract() == 0.0)
        .then_some(value as u64)
}

fn stale_worker_epoch(current: Option<u64>, incoming: u64) -> bool {
    current != Some(incoming)
}

#[cfg(target_arch = "wasm32")]
mod browser {
    use std::cell::RefCell;
    use std::rc::Rc;
    use std::sync::{
        Arc, Mutex,
        atomic::{AtomicBool, Ordering},
    };

    use js_sys::{Object, Reflect};
    use wasm_bindgen::JsCast as _;
    use wasm_bindgen::prelude::*;

    use super::{next_request_id, request_id_from_js_number, stale_result, stale_worker_epoch};
    use crate::simulation::results::SimulationResult;
    use crate::simulation::runner::worker_contract::{
        WorkerProgressSnapshot, WorkerRequest, validate_worker_response_id,
        worker_response_from_value,
    };
    use crate::simulation::runner::{NetlistInput, SimulationError, SimulationRequest};
    use crate::simulation::status::{SimulationProgress, SimulationStatus};

    #[derive(Default)]
    struct WorkerState {
        current_worker_epoch: Option<u64>,
        active_request_id: Option<u64>,
        active_progress: Option<Arc<Mutex<SimulationProgress>>>,
        pending_result: Option<Result<SimulationResult, SimulationError>>,
    }

    pub(crate) struct WorkerHandle {
        state: Rc<RefCell<WorkerState>>,
        worker: Rc<RefCell<Option<web_sys::Worker>>>,
        onmessage: Option<Closure<dyn FnMut(web_sys::MessageEvent)>>,
        onerror: Option<Closure<dyn FnMut(web_sys::ErrorEvent)>>,
        onmessageerror: Option<Closure<dyn FnMut(web_sys::MessageEvent)>>,
        next_id: u64,
        next_worker_epoch: u64,
    }

    impl WorkerHandle {
        pub(crate) fn new() -> Self {
            Self {
                state: Rc::new(RefCell::new(WorkerState::default())),
                worker: Rc::new(RefCell::new(None)),
                onmessage: None,
                onerror: None,
                onmessageerror: None,
                next_id: 0,
                next_worker_epoch: 0,
            }
        }

        pub(crate) fn is_running(&self) -> bool {
            let state = self.state.borrow();
            state.active_request_id.is_some() && state.pending_result.is_none()
        }

        pub(crate) fn has_unpolled_result(&self) -> bool {
            self.state.borrow().pending_result.is_some()
        }

        pub(crate) fn poll_result(&self) -> Option<Result<SimulationResult, SimulationError>> {
            let mut state = self.state.borrow_mut();
            let result = state.pending_result.take();
            if result.is_some() {
                state.active_request_id = None;
                state.active_progress = None;
            }
            result
        }

        pub(crate) fn abort(&self) {
            let active = self.state.borrow().active_request_id;
            if active.is_none() {
                return;
            }

            let mut state = self.state.borrow_mut();
            state.current_worker_epoch = None;
            state.active_request_id = None;
            state.active_progress = None;
            state.pending_result = Some(Err(SimulationError::Aborted));
            drop(state);
            drop_cached_worker(&self.worker);
        }

        fn allocate_request_id(&mut self) -> u64 {
            self.next_id = next_request_id(self.next_id);
            self.next_id
        }

        fn allocate_worker_epoch(&mut self) -> u64 {
            self.next_worker_epoch = self.next_worker_epoch.wrapping_add(1);
            if self.next_worker_epoch == 0 {
                self.next_worker_epoch = 1;
            }
            self.next_worker_epoch
        }

        fn ensure_worker(&mut self) -> Result<web_sys::Worker, SimulationError> {
            if let Some(worker) = self.worker.borrow().as_ref()
                && self.state.borrow().current_worker_epoch.is_some()
            {
                return Ok(worker.clone());
            }

            if let Some(message) = global_worker_error() {
                clear_global_worker_error();
                clear_global_worker();
                return Err(SimulationError::InvalidConfig(message));
            }

            let worker = global_worker().or_else(create_worker).ok_or_else(|| {
                SimulationError::InvalidConfig(
                    "browser simulation worker is not available".to_string(),
                )
            })?;
            let worker_epoch = self.allocate_worker_epoch();
            *self.worker.borrow_mut() = Some(worker.clone());
            self.state.borrow_mut().current_worker_epoch = Some(worker_epoch);
            self.install_handlers(&worker, worker_epoch);
            Ok(worker)
        }

        fn install_handlers(&mut self, worker: &web_sys::Worker, worker_epoch: u64) {
            let state = Rc::clone(&self.state);
            let onmessage = Closure::<dyn FnMut(web_sys::MessageEvent)>::wrap(Box::new(
                move |event: web_sys::MessageEvent| {
                    if stale_worker_epoch(state.borrow().current_worker_epoch, worker_epoch) {
                        return;
                    }
                    handle_worker_message(&state, event.data());
                    request_repaint();
                },
            ));
            worker.set_onmessage(Some(onmessage.as_ref().unchecked_ref()));
            self.onmessage = Some(onmessage);

            let state = Rc::clone(&self.state);
            let worker_cell = Rc::clone(&self.worker);
            let onerror = Closure::<dyn FnMut(web_sys::ErrorEvent)>::wrap(Box::new(
                move |event: web_sys::ErrorEvent| {
                    let message = if event.message().is_empty() {
                        "browser simulation worker failed".to_string()
                    } else {
                        event.message()
                    };
                    let mut state = state.borrow_mut();
                    if stale_worker_epoch(state.current_worker_epoch, worker_epoch) {
                        return;
                    }
                    state.current_worker_epoch = None;
                    if state.active_request_id.is_some() {
                        state.active_request_id = None;
                        state.active_progress = None;
                        state.pending_result = Some(Err(SimulationError::InvalidConfig(message)));
                    }
                    drop_cached_worker(&worker_cell);
                    request_repaint();
                },
            ));
            worker.set_onerror(Some(onerror.as_ref().unchecked_ref()));
            self.onerror = Some(onerror);

            let state = Rc::clone(&self.state);
            let worker_cell = Rc::clone(&self.worker);
            let onmessageerror = Closure::<dyn FnMut(web_sys::MessageEvent)>::wrap(Box::new(
                move |_event: web_sys::MessageEvent| {
                    let mut state = state.borrow_mut();
                    if stale_worker_epoch(state.current_worker_epoch, worker_epoch) {
                        return;
                    }
                    state.current_worker_epoch = None;
                    if state.active_request_id.is_some() {
                        state.active_request_id = None;
                        state.active_progress = None;
                        state.pending_result = Some(Err(SimulationError::InvalidConfig(
                            "browser simulation worker returned an unreadable message".to_string(),
                        )));
                    }
                    drop_cached_worker(&worker_cell);
                    request_repaint();
                },
            ));
            worker.set_onmessageerror(Some(onmessageerror.as_ref().unchecked_ref()));
            self.onmessageerror = Some(onmessageerror);
        }
    }

    impl Drop for WorkerHandle {
        fn drop(&mut self) {
            self.state.borrow_mut().current_worker_epoch = None;
            drop_cached_worker(&self.worker);
            self.onmessage.take();
            self.onerror.take();
            self.onmessageerror.take();
        }
    }

    pub(crate) fn start_worker_request(
        handle: &mut WorkerHandle,
        request: SimulationRequest,
        input: NetlistInput,
        progress: Arc<Mutex<SimulationProgress>>,
        abort_flag: Arc<AtomicBool>,
    ) -> Result<(), SimulationError> {
        if handle.is_running() || handle.has_unpolled_result() {
            return Err(SimulationError::AlreadyRunning);
        }

        abort_flag.store(false, Ordering::SeqCst);
        mark_worker_started(&progress);

        let id = handle.allocate_request_id();
        let worker_request = WorkerRequest::from_runner_parts(id, &request, &input)?;
        let message = worker_message(&worker_request)?;
        {
            let mut state = handle.state.borrow_mut();
            state.active_request_id = Some(id);
            state.active_progress = Some(Arc::clone(&progress));
            state.pending_result = None;
        }

        let worker = match handle.ensure_worker() {
            Ok(worker) => worker,
            Err(error) => {
                let mut state = handle.state.borrow_mut();
                state.active_request_id = None;
                state.active_progress = None;
                return Err(error);
            }
        };

        if let Err(error) = worker.post_message(&message) {
            let mut state = handle.state.borrow_mut();
            state.active_request_id = None;
            state.active_progress = None;
            return Err(SimulationError::InvalidConfig(format!(
                "failed to post simulation request to worker: {}",
                js_error_message(error)
            )));
        }

        Ok(())
    }

    fn handle_worker_message(state: &Rc<RefCell<WorkerState>>, data: JsValue) {
        let message_type = string_property(&data, "type").unwrap_or_default();
        match message_type.as_str() {
            "ready" => {
                web_sys::console::log_1(&JsValue::from_str("RSpice simulation worker ready"))
            }
            "progress" => handle_progress_message(state, &data),
            "result" => handle_result_message(state, &data),
            "error" => handle_error_message(state, &data),
            _ => {}
        }
    }

    fn handle_progress_message(state: &Rc<RefCell<WorkerState>>, data: &JsValue) {
        let id = numeric_property(data, "id").unwrap_or(0);
        let active_progress = {
            let state = state.borrow();
            if stale_result(state.active_request_id, id) {
                web_sys::console::warn_1(&JsValue::from_str(&format!(
                    "Ignoring stale simulation worker progress id {id}"
                )));
                return;
            }
            state.active_progress.as_ref().cloned()
        };

        let Some(active_progress) = active_progress else {
            return;
        };
        let snapshot = Reflect::get(data, &JsValue::from_str("progress"))
            .map_err(js_error_message)
            .and_then(|value| {
                serde_wasm_bindgen::from_value::<WorkerProgressSnapshot>(value)
                    .map_err(|error| error.to_string())
            });
        let Ok(snapshot) = snapshot else {
            web_sys::console::warn_1(&JsValue::from_str(
                "Ignoring malformed simulation worker progress message",
            ));
            return;
        };

        let mut progress = match active_progress.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        snapshot.apply_to(&mut progress);
    }

    fn handle_result_message(state: &Rc<RefCell<WorkerState>>, data: &JsValue) {
        let id = numeric_property(data, "id").unwrap_or(0);
        {
            let state = state.borrow();
            if stale_result(state.active_request_id, id) {
                web_sys::console::warn_1(&JsValue::from_str(&format!(
                    "Ignoring stale simulation worker result id {id}"
                )));
                return;
            }
        }

        let response = Reflect::get(data, &JsValue::from_str("response"))
            .or_else(|_| Reflect::get(data, &JsValue::from_str("result")));

        let result = match response {
            Ok(value) => worker_response_from_value(value).and_then(|response| {
                validate_worker_response_id(id, &response)?;
                response.into_result()
            }),
            Err(error) => Err(SimulationError::InvalidConfig(js_error_message(error))),
        };

        let mut state = state.borrow_mut();
        state.active_request_id = None;
        state.active_progress = None;
        state.pending_result = Some(result);
    }

    fn handle_error_message(state: &Rc<RefCell<WorkerState>>, data: &JsValue) {
        let id = numeric_property(data, "id").unwrap_or(0);
        let message = string_property(data, "error")
            .or_else(|| string_property(data, "message"))
            .unwrap_or_else(|| "browser simulation worker failed".to_string());

        let mut state = state.borrow_mut();
        if stale_result(state.active_request_id, id) {
            web_sys::console::warn_1(&JsValue::from_str(&format!(
                "Ignoring stale simulation worker error id {id}"
            )));
            return;
        }

        state.active_request_id = None;
        state.active_progress = None;
        state.pending_result = Some(Err(SimulationError::InvalidConfig(message)));
    }

    fn global_worker() -> Option<web_sys::Worker> {
        let global = js_sys::global();
        Reflect::get(&global, &JsValue::from_str("__RSPICE_SIM_WORKER"))
            .ok()
            .and_then(|value| {
                if value.is_undefined() || value.is_null() {
                    None
                } else {
                    value.dyn_into::<web_sys::Worker>().ok()
                }
            })
    }

    fn clear_global_worker() {
        let _ = Reflect::set(
            &js_sys::global(),
            &JsValue::from_str("__RSPICE_SIM_WORKER"),
            &JsValue::NULL,
        );
    }

    fn drop_cached_worker(worker: &Rc<RefCell<Option<web_sys::Worker>>>) {
        if let Some(worker) = worker.borrow_mut().take() {
            worker.set_onmessage(None);
            worker.set_onerror(None);
            worker.set_onmessageerror(None);
            worker.terminate();
        }
        clear_global_worker();
    }

    fn global_worker_error() -> Option<String> {
        Reflect::get(
            &js_sys::global(),
            &JsValue::from_str("__RSPICE_SIM_WORKER_ERROR"),
        )
        .ok()
        .and_then(|value| {
            if value.is_undefined() || value.is_null() {
                None
            } else {
                value.as_string()
            }
        })
    }

    fn clear_global_worker_error() {
        let _ = Reflect::set(
            &js_sys::global(),
            &JsValue::from_str("__RSPICE_SIM_WORKER_ERROR"),
            &JsValue::NULL,
        );
    }

    fn create_worker() -> Option<web_sys::Worker> {
        let worker_url = global_worker_url()?;
        let options = web_sys::WorkerOptions::new();
        options.set_type(web_sys::WorkerType::Module);
        web_sys::Worker::new_with_options(&worker_url, &options).ok()
    }

    fn global_worker_url() -> Option<String> {
        Reflect::get(
            &js_sys::global(),
            &JsValue::from_str("__RSPICE_SIM_WORKER_URL"),
        )
        .ok()
        .and_then(|value| value.as_string())
        .filter(|url| !url.trim().is_empty())
    }

    fn worker_message(request: &WorkerRequest) -> Result<JsValue, SimulationError> {
        let message = Object::new();
        Reflect::set(
            &message,
            &JsValue::from_str("type"),
            &JsValue::from_str("run"),
        )
        .map_err(reflect_error)?;
        Reflect::set(
            &message,
            &JsValue::from_str("id"),
            &JsValue::from_f64(request.id as f64),
        )
        .map_err(reflect_error)?;
        let request_value = serde_wasm_bindgen::to_value(request).map_err(|error| {
            SimulationError::InvalidConfig(format!(
                "failed to serialize simulation request for worker: {error}"
            ))
        })?;
        Reflect::set(&message, &JsValue::from_str("request"), &request_value)
            .map_err(reflect_error)?;
        Ok(JsValue::from(message))
    }

    fn mark_worker_started(progress: &Arc<Mutex<SimulationProgress>>) {
        let mut progress = match progress.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        progress.update_status(SimulationStatus::Parsing);
        progress.message = Some("Running simulation in browser worker".to_string());
    }

    fn request_repaint() {
        if let Some(window) = web_sys::window() {
            let callback = Closure::<dyn FnMut()>::once(|| {});
            let _ = window.request_animation_frame(callback.as_ref().unchecked_ref());
            callback.forget();
        }
    }

    fn string_property(value: &JsValue, property: &str) -> Option<String> {
        Reflect::get(value, &JsValue::from_str(property))
            .ok()
            .and_then(|value| value.as_string())
    }

    fn numeric_property(value: &JsValue, property: &str) -> Option<u64> {
        Reflect::get(value, &JsValue::from_str(property))
            .ok()
            .and_then(|value| value.as_f64())
            .and_then(request_id_from_js_number)
    }

    fn reflect_error(error: JsValue) -> SimulationError {
        SimulationError::InvalidConfig(js_error_message(error))
    }

    fn js_error_message(error: JsValue) -> String {
        error
            .as_string()
            .or_else(|| {
                Reflect::get(&error, &JsValue::from_str("message"))
                    .ok()
                    .and_then(|message| message.as_string())
            })
            .unwrap_or_else(|| "unknown JavaScript error".to_string())
    }
}

#[cfg(target_arch = "wasm32")]
pub(crate) use browser::{WorkerHandle, start_worker_request};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn next_request_id_skips_zero_after_wraparound() {
        assert_eq!(next_request_id(0), 1);
        assert_eq!(
            next_request_id(MAX_BROWSER_REQUEST_ID - 1),
            MAX_BROWSER_REQUEST_ID
        );
        assert_eq!(next_request_id(MAX_BROWSER_REQUEST_ID), 1);
        assert_eq!(next_request_id(u64::MAX), 1);
    }

    #[test]
    fn stale_result_rejects_non_active_ids() {
        assert!(!stale_result(Some(8), 8));
        assert!(stale_result(Some(8), 7));
        assert!(stale_result(None, 8));
    }

    #[test]
    fn javascript_request_ids_are_exact_bounded_integers() {
        assert_eq!(request_id_from_js_number(1.0), Some(1));
        assert_eq!(
            request_id_from_js_number(MAX_BROWSER_REQUEST_ID as f64),
            Some(MAX_BROWSER_REQUEST_ID)
        );
        assert_eq!(request_id_from_js_number(0.0), None);
        assert_eq!(request_id_from_js_number(1.5), None);
        assert_eq!(
            request_id_from_js_number(MAX_BROWSER_REQUEST_ID as f64 + 1.0),
            None
        );
        assert_eq!(request_id_from_js_number(f64::NAN), None);
    }

    #[test]
    fn stale_worker_callbacks_cannot_target_a_replacement_epoch() {
        assert!(!stale_worker_epoch(Some(9), 9));
        assert!(stale_worker_epoch(Some(10), 9));
        assert!(stale_worker_epoch(None, 9));
    }
}
