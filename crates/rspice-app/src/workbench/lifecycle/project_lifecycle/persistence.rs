//! Canonical project persistence identities and optimistic concurrency.
mod browser;
#[cfg(not(target_arch = "wasm32"))]
mod native;
#[cfg(not(target_arch = "wasm32"))]
pub(crate) use native::*;
#[cfg(not(target_arch = "wasm32"))]
pub(crate) use rspice_project::persistence::native::{
    NativeBinding as PersistenceBinding, UnreadableNativeBinding,
};

// `browser` is entirely `cfg(target_arch = "wasm32")`, so on native this glob
// re-exports nothing and rustc reports it unused. Gating the `use` rather than
// the `mod` keeps the module compiling (and type-checking) on both targets.
#[cfg(target_arch = "wasm32")]
pub(crate) use browser::*;

#[cfg(any(test, target_arch = "wasm32"))]
pub(crate) use rspice_project::persistence::BrowserBindingBackend;
pub(crate) use rspice_project::persistence::{BrowserBindingReceipt, NativeBindingReceipt};

#[cfg(target_arch = "wasm32")]
use rspice_project::persistence::browser::{
    BROWSER_BINDING_SCHEMA_VERSION, BrowserBinding, BrowserBindingCommitOutcome,
    BrowserBindingMetadata, BrowserRestoreCandidate, BrowserRestoreIssue,
    BrowserRestoreMetadataError, BrowserWriteIntent, MAX_EXACT_BROWSER_GENERATION,
    browser_backend_from_name, browser_backend_name, browser_generation_is_exact,
    classify_browser_binding_commit, validate_binding_generation_commit,
};

#[cfg(target_arch = "wasm32")]
use crate::io::ProjectSnapshot;
#[cfg(target_arch = "wasm32")]
use crate::product::ContentDigest;
#[cfg(target_arch = "wasm32")]
use rspice_project::persistence::{ProjectBytes, digest_bytes};

#[cfg(target_arch = "wasm32")]
const BROWSER_BINDING_DATABASE: &str = "rspice-project-bindings";
#[cfg(target_arch = "wasm32")]
const BROWSER_BINDING_STORE: &str = "canonical-file-handles";

#[cfg(target_arch = "wasm32")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum PersistenceBinding {
    Browser {
        handle_id: u64,
        binding: BrowserBinding,
    },
}

#[cfg(target_arch = "wasm32")]
impl PersistenceBinding {
    pub(crate) fn browser_receipt(&self) -> BrowserBindingReceipt {
        match self {
            Self::Browser { binding, .. } => binding.receipt.clone(),
        }
    }

    pub(crate) fn durable_browser_receipt(&self) -> Option<BrowserBindingReceipt> {
        match self {
            Self::Browser { binding, .. } => binding.durable_receipt(),
        }
    }
}

#[cfg(target_arch = "wasm32")]
#[derive(Debug)]
pub(crate) enum BrowserWriteResult {
    Saved {
        handle_id: u64,
        binding_id: uuid::Uuid,
        backend: BrowserBindingBackend,
        project_id: String,
        generation: u64,
        display_name: String,
        digest: ContentDigest,
    },
    SavedSessionOnly {
        handle_id: u64,
        binding_id: uuid::Uuid,
        backend: BrowserBindingBackend,
        project_id: String,
        generation: u64,
        display_name: String,
        digest: ContentDigest,
        persistence_error: String,
    },
    Cancelled,
    ExternalChange {
        observed_digest: ContentDigest,
    },
    Failed(String),
}

#[cfg(target_arch = "wasm32")]
#[derive(Debug, Clone)]
pub(crate) struct BrowserWriteTarget {
    pub(crate) handle_id: Option<u64>,
    pub(crate) intent: BrowserWriteIntent,
}

#[cfg(target_arch = "wasm32")]
#[derive(Debug)]
pub(crate) enum BrowserOpenResult {
    Opened {
        handle_id: u64,
        display_name: String,
        bytes: ProjectBytes,
    },
    Cancelled,
    Failed(String),
}

#[cfg(target_arch = "wasm32")]
#[derive(Debug)]
pub(crate) enum BrowserRestoreResult {
    Missing,
    Restored {
        baseline: Box<ProjectSnapshot>,
        binding: PersistenceBinding,
    },
    ReconnectRequired {
        binding: PersistenceBinding,
    },
    Conflict {
        binding: PersistenceBinding,
        observed_digest: ContentDigest,
        reason: String,
    },
    Evicted(String),
    Retryable(String),
    Unsupported(String),
}

#[cfg(target_arch = "wasm32")]
thread_local! {
    static BROWSER_FILE_HANDLES: std::cell::RefCell<std::collections::HashMap<u64, wasm_bindgen::JsValue>> =
        std::cell::RefCell::new(std::collections::HashMap::new());
    static NEXT_BROWSER_HANDLE_ID: std::cell::Cell<u64> = const { std::cell::Cell::new(1) };
    static RETAINED_BROWSER_EVENT_HANDLERS: std::cell::RefCell<Vec<RetainedBrowserEventHandlers>> =
        const { std::cell::RefCell::new(Vec::new()) };
}

#[cfg(target_arch = "wasm32")]
const MAX_RETAINED_BROWSER_EVENT_OPERATIONS: usize = 32;
#[cfg(any(test, target_arch = "wasm32"))]
const BROWSER_ASYNC_TIMEOUT_MS: u32 = 15_000;

#[cfg(any(test, target_arch = "wasm32"))]
fn browser_timeout_message(operation: &str) -> String {
    format!(
        "{operation} timed out after {} seconds",
        BROWSER_ASYNC_TIMEOUT_MS / 1_000
    )
}

#[cfg(target_arch = "wasm32")]
struct RetainedBrowserEventHandlers {
    done: std::rc::Rc<std::cell::Cell<bool>>,
    _callbacks: Vec<BrowserEventHandler>,
}

#[cfg(target_arch = "wasm32")]
type BrowserEventHandler = wasm_bindgen::closure::Closure<dyn FnMut()>;

#[cfg(target_arch = "wasm32")]
type BrowserEventHandlerBinding = (&'static str, BrowserEventHandler);

#[cfg(target_arch = "wasm32")]
fn retain_browser_event_handlers(
    done: std::rc::Rc<std::cell::Cell<bool>>,
    callbacks: Vec<BrowserEventHandler>,
) -> Result<(), String> {
    RETAINED_BROWSER_EVENT_HANDLERS.with(|retained| {
        let mut retained = retained.borrow_mut();
        retained.retain(|operation| !operation.done.get());
        if retained.len() >= MAX_RETAINED_BROWSER_EVENT_OPERATIONS {
            return Err("too many browser storage operations are still pending".to_owned());
        }
        retained.push(RetainedBrowserEventHandlers {
            done,
            _callbacks: callbacks,
        });
        Ok(())
    })
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn browser_file_picker_supported() -> bool {
    let Some(window) = web_sys::window() else {
        return false;
    };
    js_sys::Reflect::get(
        &window,
        &wasm_bindgen::JsValue::from_str("showSaveFilePicker"),
    )
    .ok()
    .is_some_and(|value| value.is_function())
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn browser_open_file_picker_supported() -> bool {
    let Some(window) = web_sys::window() else {
        return false;
    };
    js_sys::Reflect::get(
        &window,
        &wasm_bindgen::JsValue::from_str("showOpenFilePicker"),
    )
    .ok()
    .is_some_and(|value| value.is_function())
        && browser_web_locks_supported()
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn browser_external_canonical_supported() -> bool {
    browser_file_picker_supported() && browser_web_locks_supported()
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn browser_binding_store_supported() -> bool {
    web_sys::window()
        .and_then(|window| {
            js_sys::Reflect::get(&window, &wasm_bindgen::JsValue::from_str("indexedDB")).ok()
        })
        .is_some_and(|value| !value.is_null() && !value.is_undefined())
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn browser_opfs_supported() -> bool {
    let Some(window) = web_sys::window() else {
        return false;
    };
    let Ok(navigator) =
        js_sys::Reflect::get(&window, &wasm_bindgen::JsValue::from_str("navigator"))
    else {
        return false;
    };
    let Ok(storage) = js_sys::Reflect::get(&navigator, &wasm_bindgen::JsValue::from_str("storage"))
    else {
        return false;
    };
    js_sys::Reflect::get(&storage, &wasm_bindgen::JsValue::from_str("getDirectory"))
        .ok()
        .is_some_and(|value| value.is_function())
        && browser_web_locks_supported()
}

/// Start a canonical browser write while the menu click still owns browser
/// user activation. Existing handles are resolved only through the live
/// registry, permission is rechecked, current bytes are compared, and the
/// written bytes are read back before success is reported.
#[cfg(target_arch = "wasm32")]
pub(crate) fn start_browser_write(
    target: BrowserWriteTarget,
    persist_binding: bool,
    suggested_name: &str,
    bytes: Vec<u8>,
    complete: impl FnOnce(BrowserWriteResult) + 'static,
) -> Result<(), String> {
    use wasm_bindgen::JsCast as _;

    if bytes.len() as u64 > crate::io::project_io::MAX_PROJECT_FILE_BYTES {
        return Err("project exceeds the browser project-size limit".to_owned());
    }
    target.intent.validate_generation()?;
    let operation = if let Some(handle_id) = target.handle_id {
        let handle = resolve_browser_handle(handle_id)?;
        BrowserWriteStart::Existing { handle, target }
    } else {
        match target.intent.backend {
            BrowserBindingBackend::ExternalFile => {
                let window = web_sys::window().ok_or("browser window is unavailable")?;
                let picker = js_sys::Reflect::get(
                    &window,
                    &wasm_bindgen::JsValue::from_str("showSaveFilePicker"),
                )
                .map_err(js_error)?
                .dyn_into::<js_sys::Function>()
                .map_err(|_| "File System Access save picker is unavailable")?;
                let options = project_save_picker_options(suggested_name)?;
                let value = picker.call1(&window, &options).map_err(js_error)?;
                let picker = value
                    .dyn_into::<js_sys::Promise>()
                    .map_err(|_| "save picker did not return a Promise".to_owned())?;
                BrowserWriteStart::Picker { picker, target }
            }
            BrowserBindingBackend::Opfs => BrowserWriteStart::Opfs { target },
        }
    };
    let suggested_name = suggested_name.to_owned();

    wasm_bindgen_futures::spawn_local(async move {
        let result = run_browser_write(operation, &suggested_name, persist_binding, &bytes).await;
        complete(result);
    });
    Ok(())
}

/// Open one canonical browser project under the click's user activation.
/// The selected handle is permission-checked, bounded, and read exactly once;
/// callers receive the digest of those same bytes and a live handle identity.
#[cfg(target_arch = "wasm32")]
pub(crate) fn start_browser_open(
    complete: impl FnOnce(BrowserOpenResult) + 'static,
) -> Result<(), String> {
    use wasm_bindgen::JsCast as _;

    let window = web_sys::window().ok_or("browser window is unavailable")?;
    let picker = js_sys::Reflect::get(
        &window,
        &wasm_bindgen::JsValue::from_str("showOpenFilePicker"),
    )
    .map_err(js_error)?
    .dyn_into::<js_sys::Function>()
    .map_err(|_| "File System Access open picker is unavailable")?;
    let options = project_open_picker_options()?;
    let value = picker.call1(&window, &options).map_err(js_error)?;
    let picker = value
        .dyn_into::<js_sys::Promise>()
        .map_err(|_| "open picker did not return a Promise".to_owned())?;

    wasm_bindgen_futures::spawn_local(async move {
        let result = run_browser_open(picker).await;
        complete(result);
    });
    Ok(())
}

#[cfg(target_arch = "wasm32")]
fn project_open_picker_options() -> Result<wasm_bindgen::JsValue, String> {
    let types = project_picker_types()?;
    let options = js_sys::Object::new();
    set_js_field(
        &options,
        "id",
        &wasm_bindgen::JsValue::from_str("rspice-project-open"),
    )?;
    set_js_field(
        &options,
        "multiple",
        &wasm_bindgen::JsValue::from_bool(false),
    )?;
    set_js_field(
        &options,
        "excludeAcceptAllOption",
        &wasm_bindgen::JsValue::from_bool(true),
    )?;
    set_js_field(&options, "types", &types)?;
    Ok(options.into())
}

#[cfg(target_arch = "wasm32")]
fn project_save_picker_options(suggested_name: &str) -> Result<wasm_bindgen::JsValue, String> {
    let types = project_picker_types()?;
    let options = js_sys::Object::new();
    set_js_field(
        &options,
        "id",
        &wasm_bindgen::JsValue::from_str("rspice-project-save"),
    )?;
    set_js_field(
        &options,
        "suggestedName",
        &wasm_bindgen::JsValue::from_str(suggested_name),
    )?;
    set_js_field(
        &options,
        "excludeAcceptAllOption",
        &wasm_bindgen::JsValue::from_bool(true),
    )?;
    set_js_field(&options, "types", &types)?;
    Ok(options.into())
}

#[cfg(target_arch = "wasm32")]
fn project_picker_types() -> Result<js_sys::Array, String> {
    let extensions = js_sys::Array::new();
    extensions.push(&wasm_bindgen::JsValue::from_str(".rspiceproj"));
    let accept = js_sys::Object::new();
    js_sys::Reflect::set(
        &accept,
        &wasm_bindgen::JsValue::from_str("application/json"),
        &extensions,
    )
    .map_err(js_error)?;
    let project_type = js_sys::Object::new();
    js_sys::Reflect::set(
        &project_type,
        &wasm_bindgen::JsValue::from_str("description"),
        &wasm_bindgen::JsValue::from_str("RSpice project"),
    )
    .map_err(js_error)?;
    js_sys::Reflect::set(
        &project_type,
        &wasm_bindgen::JsValue::from_str("accept"),
        &accept,
    )
    .map_err(js_error)?;
    let types = js_sys::Array::new();
    types.push(&project_type);
    Ok(types)
}

#[cfg(target_arch = "wasm32")]
async fn run_browser_open(picker: js_sys::Promise) -> BrowserOpenResult {
    let selected = match wasm_bindgen_futures::JsFuture::from(picker).await {
        Ok(selected) => selected,
        Err(error) if js_error_name(&error) == Some("AbortError") => {
            return BrowserOpenResult::Cancelled;
        }
        Err(error) => return BrowserOpenResult::Failed(js_error(error)),
    };
    let handles = js_sys::Array::from(&selected);
    if handles.length() != 1 {
        return BrowserOpenResult::Failed(format!(
            "open picker returned {} handles; exactly one project is required",
            handles.length()
        ));
    }
    let handle = handles.get(0);
    if let Err(error) = validate_browser_read_handle(&handle, true) {
        return BrowserOpenResult::Failed(error);
    }
    let permission = match call_promise_method(
        &handle,
        "requestPermission",
        &[permission_options(BrowserPermissionMode::Read)],
    ) {
        Ok(permission) => permission,
        Err(error) => return BrowserOpenResult::Failed(error),
    };
    if let Err(error) = ensure_browser_permission(permission, BrowserPermissionMode::Read).await {
        return BrowserOpenResult::Failed(error);
    }
    let bytes = match read_browser_handle_bytes(&handle).await {
        Ok(bytes) => bytes,
        Err(error) => return BrowserOpenResult::Failed(error),
    };
    let bytes = match ProjectBytes::from_bytes(bytes) {
        Ok(bytes) => bytes,
        Err(_) => {
            return BrowserOpenResult::Failed(
                "browser project exceeds the supported project-size limit".to_owned(),
            );
        }
    };
    let display_name = js_sys::Reflect::get(&handle, &wasm_bindgen::JsValue::from_str("name"))
        .ok()
        .and_then(|name| name.as_string())
        .unwrap_or_else(|| "project.rspiceproj".to_owned());
    let handle_id = register_browser_handle(handle);
    BrowserOpenResult::Opened {
        handle_id,
        display_name,
        bytes,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn browser_storage_timeout_message_is_bounded_and_actionable() {
        assert_eq!(
            browser_timeout_message("browser staged file write"),
            "browser staged file write timed out after 15 seconds"
        );
    }
}
