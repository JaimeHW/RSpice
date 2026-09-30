//! Browser PDK import transport, private validation-cache restoration, and worker execution.

use super::*;
use crate::state::pdk_config::ValidatedPdkTechnologyPackage;

const BROWSER_PDK_IMPORT_PROTOCOL_VERSION: u16 = 1;

pub(crate) type BrowserPackageImport = Result<Option<BrowserPackageImportCandidate>, String>;

pub(crate) struct BrowserPackageImportCandidate {
    base: PdkConfig,
    payload: BrowserPdkImportPayload,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct BrowserPdkImportMetadata {
    protocol_version: u16,
    config: PdkConfig,
    authority: PdkAdministrativeAuthority,
    reason: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct BrowserPdkImportPayload {
    protocol_version: u16,
    config: PdkConfig,
    validated_packages: Vec<ValidatedPdkTechnologyPackage>,
    package_id: String,
    revision: String,
    sequence: u64,
}

thread_local! {
    static BROWSER_PACKAGE_IMPORTS:
        std::cell::RefCell<std::collections::VecDeque<BrowserPackageImport>> =
        const { std::cell::RefCell::new(std::collections::VecDeque::new()) };
}

impl BrowserPackageImportCandidate {
    pub(crate) fn resolve(self, current: &PdkConfig) -> Result<PackageImportCandidate, String> {
        if *current != self.base {
            return Err(
                "PDK configuration changed while the browser worker was validating the signed package; the stale candidate was discarded without mutation."
                    .to_owned(),
            );
        }
        let BrowserPdkImportPayload {
            protocol_version,
            mut config,
            validated_packages,
            package_id,
            revision,
            sequence,
        } = self.payload;
        if protocol_version == BROWSER_PDK_IMPORT_PROTOCOL_VERSION {
            config
                .technology_registry
                .restore_worker_validated_packages(validated_packages)
                .map_err(|error| error.to_string())?;
        } else {
            return Err(format!(
                "Unsupported browser PDK import response protocol {protocol_version}."
            ));
        }
        Ok(PackageImportCandidate {
            config,
            package_id,
            revision,
            sequence,
        })
    }
}

pub(crate) fn start_browser_package_import(
    base: PdkConfig,
    archive: Vec<u8>,
    authority: PdkAdministrativeAuthority,
    reason: String,
    wake: impl Fn() + 'static,
) -> Result<(), String> {
    let metadata = BrowserPdkImportMetadata {
        protocol_version: BROWSER_PDK_IMPORT_PROTOCOL_VERSION,
        config: base.clone(),
        authority,
        reason,
    };
    browser_pdk_import_worker::start(metadata, archive, base, std::rc::Rc::new(wake))
}

pub(crate) fn cancel_browser_package_import() {
    BROWSER_PACKAGE_IMPORTS.with(|queue| queue.borrow_mut().push_back(Ok(None)));
}

pub(crate) fn fail_browser_package_import(error: String) {
    BROWSER_PACKAGE_IMPORTS.with(|queue| queue.borrow_mut().push_back(Err(error)));
}

pub(crate) fn take_browser_package_imports() -> Vec<BrowserPackageImport> {
    let completions =
        BROWSER_PACKAGE_IMPORTS.with(|queue| queue.borrow_mut().drain(..).collect::<Vec<_>>());
    if !completions.is_empty() {
        browser_pdk_import_worker::finish();
    }
    completions
}

mod browser_pdk_import_worker {
    use std::cell::{Cell, RefCell};
    use std::rc::Rc;

    use js_sys::{Object, Reflect, Uint8Array};
    use wasm_bindgen::JsCast as _;
    use wasm_bindgen::prelude::*;

    use super::*;

    const MAX_RESPONSE_BYTES: usize = 256 * 1024 * 1024;

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

    thread_local! {
        static NEXT_REQUEST_ID: Cell<u32> = const { Cell::new(0) };
        static ACTIVE_WORKER: RefCell<Option<ActiveWorker>> = const { RefCell::new(None) };
    }

    pub(super) fn start(
        metadata: BrowserPdkImportMetadata,
        archive: Vec<u8>,
        base: PdkConfig,
        wake: Rc<dyn Fn()>,
    ) -> Result<(), String> {
        if ACTIVE_WORKER.with(|active| active.borrow().is_some()) {
            return Err("A browser PDK package validator is already active.".to_owned());
        }
        let metadata = serde_json::to_vec(&metadata)
            .map_err(|error| format!("Could not encode PDK import metadata: {error}"))?;
        let id = NEXT_REQUEST_ID.with(|next| {
            let id = next.get().wrapping_add(1).max(1);
            next.set(id);
            id
        });
        let metadata = transferred_array(&metadata)?;
        let archive = transferred_array(&archive)?;
        let request = Object::new();
        Reflect::set(&request, &JsValue::from_str("metadataBytes"), &metadata)
            .map_err(js_error_message)?;
        Reflect::set(&request, &JsValue::from_str("archiveBytes"), &archive)
            .map_err(js_error_message)?;

        let options = web_sys::WorkerOptions::new();
        options.set_type(web_sys::WorkerType::Module);
        let worker = web_sys::Worker::new_with_options(&worker_url()?, &options)
            .map_err(js_error_message)?;
        let completed = Rc::new(Cell::new(false));

        let success_base = base.clone();
        let success_wake = wake.clone();
        let success_completed = Rc::clone(&completed);
        let onmessage = Closure::<dyn FnMut(web_sys::MessageEvent)>::wrap(Box::new(
            move |event: web_sys::MessageEvent| {
                let data = event.data();
                if numeric_property(&data, "id") != Some(id) {
                    return;
                }
                let result = match string_property(&data, "type").as_deref() {
                    Some("pdk-import-result") => {
                        Reflect::get(&data, &JsValue::from_str("response"))
                            .map_err(js_error_message)
                            .and_then(|response| decode_response(&response))
                            .map(|payload| {
                                Some(BrowserPackageImportCandidate {
                                    base: success_base.clone(),
                                    payload,
                                })
                            })
                    }
                    Some("pdk-import-error") | Some("error") => {
                        Err(string_property(&data, "error")
                            .or_else(|| string_property(&data, "message"))
                            .unwrap_or_else(|| "Browser PDK package validator failed.".to_owned()))
                    }
                    _ => return,
                };
                complete_once(&success_completed, &success_wake, result);
            },
        ));
        worker.set_onmessage(Some(onmessage.as_ref().unchecked_ref()));

        let error_wake = wake.clone();
        let error_completed = Rc::clone(&completed);
        let onerror = Closure::<dyn FnMut(web_sys::ErrorEvent)>::wrap(Box::new(
            move |event: web_sys::ErrorEvent| {
                complete_once(
                    &error_completed,
                    &error_wake,
                    Err(if event.message().is_empty() {
                        "Browser PDK package validator failed.".to_owned()
                    } else {
                        event.message()
                    }),
                );
            },
        ));
        worker.set_onerror(Some(onerror.as_ref().unchecked_ref()));

        let message_wake = wake;
        let message_completed = completed;
        let onmessageerror =
            Closure::<dyn FnMut(web_sys::MessageEvent)>::wrap(Box::new(move |_event| {
                complete_once(
                    &message_completed,
                    &message_wake,
                    Err("Browser PDK package validator returned an unreadable message.".to_owned()),
                );
            }));
        worker.set_onmessageerror(Some(onmessageerror.as_ref().unchecked_ref()));

        let message = Object::new();
        Reflect::set(
            &message,
            &JsValue::from_str("type"),
            &JsValue::from_str("run-pdk-import"),
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
        let transfer = js_sys::Array::of2(&metadata.buffer(), &archive.buffer());
        ACTIVE_WORKER.with(|active| {
            *active.borrow_mut() = Some(ActiveWorker {
                worker: worker.clone(),
                _onmessage: onmessage,
                _onerror: onerror,
                _onmessageerror: onmessageerror,
            });
        });
        if let Err(error) = worker.post_message_with_transfer(&message, &transfer) {
            finish();
            return Err(format!(
                "Could not dispatch browser PDK package validation: {}",
                js_error_message(error)
            ));
        }
        Ok(())
    }

    fn complete_once(completed: &Cell<bool>, wake: &Rc<dyn Fn()>, result: BrowserPackageImport) {
        if completed.replace(true) {
            return;
        }
        BROWSER_PACKAGE_IMPORTS.with(|queue| queue.borrow_mut().push_back(result));
        wake();
    }

    pub(super) fn finish() {
        ACTIVE_WORKER.with(|active| {
            active.borrow_mut().take();
        });
    }

    fn transferred_array(bytes: &[u8]) -> Result<Uint8Array, String> {
        let length = u32::try_from(bytes.len())
            .map_err(|_| "PDK package worker input exceeds browser array limits.".to_owned())?;
        let view = Uint8Array::new_with_length(length);
        view.copy_from(bytes);
        Ok(view)
    }

    fn decode_response(value: &JsValue) -> Result<BrowserPdkImportPayload, String> {
        let protocol = numeric_property(value, "protocolVersion")
            .ok_or_else(|| "PDK import worker response has no protocol version.".to_owned())?;
        if protocol != u32::from(BROWSER_PDK_IMPORT_PROTOCOL_VERSION) {
            return Err(format!(
                "Unsupported PDK import worker protocol {protocol}."
            ));
        }
        let bytes =
            Reflect::get(value, &JsValue::from_str("payloadBytes")).map_err(js_error_message)?;
        let bytes = Uint8Array::new(&bytes);
        let length = usize::try_from(bytes.length())
            .map_err(|_| "PDK import worker response exceeds host limits.".to_owned())?;
        if length == 0 || length > MAX_RESPONSE_BYTES {
            return Err(format!(
                "PDK import worker response contains {length} bytes; the supported range is 1..={MAX_RESPONSE_BYTES}."
            ));
        }
        let mut encoded = vec![0; length];
        bytes.copy_to(&mut encoded);
        serde_json::from_slice(&encoded)
            .map_err(|error| format!("PDK import worker returned invalid candidate data: {error}"))
    }

    fn worker_url() -> Result<String, String> {
        Reflect::get(
            &js_sys::global(),
            &JsValue::from_str("__RSPICE_SIM_WORKER_URL"),
        )
        .map_err(js_error_message)?
        .as_string()
        .filter(|url| !url.trim().is_empty())
        .ok_or_else(|| "Browser PDK package validator URL is unavailable.".to_owned())
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
                    && *value >= 0.0
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

pub(crate) fn run_pdk_import_worker_request_value(
    request: wasm_bindgen::JsValue,
) -> Result<wasm_bindgen::JsValue, wasm_bindgen::JsValue> {
    use js_sys::{Object, Reflect, Uint8Array};
    use wasm_bindgen::JsValue;

    let metadata = Uint8Array::new(&Reflect::get(
        &request,
        &JsValue::from_str("metadataBytes"),
    )?);
    let mut metadata_bytes = vec![
        0;
        usize::try_from(metadata.length()).map_err(|_| {
            JsValue::from_str("PDK import metadata exceeds host limits.")
        })?
    ];
    metadata.copy_to(&mut metadata_bytes);
    let metadata: BrowserPdkImportMetadata = serde_json::from_slice(&metadata_bytes)
        .map_err(|error| JsValue::from_str(&format!("Invalid PDK import metadata: {error}")))?;
    if metadata.protocol_version != BROWSER_PDK_IMPORT_PROTOCOL_VERSION {
        return Err(JsValue::from_str(&format!(
            "Unsupported PDK import protocol {}.",
            metadata.protocol_version
        )));
    }
    let archive = Uint8Array::new(&Reflect::get(&request, &JsValue::from_str("archiveBytes"))?);
    let archive_length = usize::try_from(archive.length())
        .map_err(|_| JsValue::from_str("PDK package exceeds host limits."))?;
    if archive_length == 0 || archive_length > MAX_PDK_ARCHIVE_BYTES {
        return Err(JsValue::from_str(
            "PDK package size is outside supported limits.",
        ));
    }
    let mut archive_bytes = vec![0; archive_length];
    archive.copy_to(&mut archive_bytes);

    let mut config = metadata.config;
    if !config.technology_registry.archives().is_empty() {
        let trust = config.publisher_trust_store.clone();
        config
            .technology_registry
            .revalidate_installed(&trust)
            .map_err(|errors| JsValue::from_str(&errors.join("; ")))?;
    }
    let receipt = config
        .technology_registry
        .install_archive_bytes(
            &archive_bytes,
            &config.publisher_trust_store,
            &metadata.authority,
            &metadata.reason,
        )
        .map_err(|error| JsValue::from_str(&error.to_string()))?;
    let validated_packages = config.technology_registry.take_worker_validated_packages();
    let payload = BrowserPdkImportPayload {
        protocol_version: BROWSER_PDK_IMPORT_PROTOCOL_VERSION,
        config,
        validated_packages,
        package_id: receipt.target.package_id,
        revision: receipt.target.revision,
        sequence: receipt.sequence,
    };
    let encoded = serde_json::to_vec(&payload)
        .map_err(|error| JsValue::from_str(&format!("Could not encode PDK candidate: {error}")))?;
    if encoded.is_empty() || encoded.len() > 256 * 1024 * 1024 {
        return Err(JsValue::from_str(
            "Validated PDK candidate exceeds the browser worker response limit.",
        ));
    }
    let bytes = Uint8Array::new_with_length(
        u32::try_from(encoded.len())
            .map_err(|_| JsValue::from_str("PDK candidate exceeds browser array limits."))?,
    );
    bytes.copy_from(&encoded);
    let response = Object::new();
    Reflect::set(
        &response,
        &JsValue::from_str("protocolVersion"),
        &JsValue::from_f64(f64::from(BROWSER_PDK_IMPORT_PROTOCOL_VERSION)),
    )?;
    Reflect::set(&response, &JsValue::from_str("payloadBytes"), &bytes)?;
    Ok(response.into())
}
