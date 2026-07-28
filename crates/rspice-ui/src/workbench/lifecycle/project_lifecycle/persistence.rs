//! Canonical project persistence identities and optimistic concurrency.
mod browser;

pub(crate) use browser::*;

#[cfg(not(target_arch = "wasm32"))]
use std::path::Path;
use std::path::PathBuf;

use sha2::{Digest as _, Sha256};

use crate::io::{ProjectFile, ProjectIoError};
use crate::product::ContentDigest;

#[cfg(any(target_arch = "wasm32", test))]
const BROWSER_BINDING_SCHEMA_VERSION: u32 = 2;
#[cfg(any(target_arch = "wasm32", test))]
const MAX_EXACT_BROWSER_GENERATION: u64 = (1_u64 << 53) - 1;
#[cfg(target_arch = "wasm32")]
const BROWSER_BINDING_DATABASE: &str = "rspice-project-bindings";
#[cfg(target_arch = "wasm32")]
const BROWSER_BINDING_STORE: &str = "canonical-file-handles";

/// Storage surface that owns the canonical browser bytes.  The opaque
/// binding UUID is deliberately independent of the logical project UUID so
/// forks, duplicate projects, and multiple browser tabs cannot alias one
/// another's persistence authority.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) enum BrowserBindingBackend {
    ExternalFile,
    Opfs,
}

/// Durable session receipt for a browser canonical binding.  A restored
/// IndexedDB/OPFS record must match every field before it may become an
/// accepted baseline; a different generation or digest is a conflict, not a
/// silently upgraded baseline.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub(crate) struct BrowserBindingReceipt {
    pub(crate) binding_id: uuid::Uuid,
    pub(crate) project_id: String,
    pub(crate) accepted_generation: u64,
    pub(crate) accepted_digest: ContentDigest,
    pub(crate) backend: BrowserBindingBackend,
}

/// Exact native canonical-file authority persisted with the working session.
/// A remembered pathname is only a convenience; restart restoration requires
/// this path, logical project identity, and content digest to match together.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub(crate) struct NativeBindingReceipt {
    pub(crate) canonical_path: PathBuf,
    pub(crate) project_id: String,
    pub(crate) accepted_digest: ContentDigest,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum PersistenceBinding {
    #[cfg(not(target_arch = "wasm32"))]
    Native {
        canonical_path: PathBuf,
        accepted_digest: ContentDigest,
    },
    #[cfg(target_arch = "wasm32")]
    Browser {
        handle_id: u64,
        binding_id: uuid::Uuid,
        backend: BrowserBindingBackend,
        project_id: String,
        accepted_generation: u64,
        display_name: String,
        accepted_digest: ContentDigest,
        /// Generation currently committed to IndexedDB. `None` means the
        /// canonical bytes are verified and live for this session, but the
        /// restart record could not be committed.
        persisted_generation: Option<u64>,
    },
}

impl PersistenceBinding {
    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) fn native_receipt(&self, project_id: &str) -> NativeBindingReceipt {
        match self {
            Self::Native {
                canonical_path,
                accepted_digest,
            } => NativeBindingReceipt {
                canonical_path: canonical_path.clone(),
                project_id: project_id.to_owned(),
                accepted_digest: *accepted_digest,
            },
        }
    }

    #[cfg(target_arch = "wasm32")]
    pub(crate) fn browser_receipt(&self) -> BrowserBindingReceipt {
        match self {
            Self::Browser {
                binding_id,
                backend,
                project_id,
                accepted_generation,
                accepted_digest,
                ..
            } => BrowserBindingReceipt {
                binding_id: *binding_id,
                project_id: project_id.clone(),
                accepted_generation: *accepted_generation,
                accepted_digest: *accepted_digest,
                backend: *backend,
            },
        }
    }

    /// Return restart authority only when the exact accepted generation has
    /// actually been committed to browser binding storage. A live file handle
    /// with a newer session-only generation is useful for retrying in this
    /// tab, but it must never be serialized as durable restart authority.
    #[cfg(target_arch = "wasm32")]
    pub(crate) fn durable_browser_receipt(&self) -> Option<BrowserBindingReceipt> {
        match self {
            Self::Browser {
                accepted_generation,
                persisted_generation,
                ..
            } if browser_generation_has_restart_authority(
                *accepted_generation,
                *persisted_generation,
            ) =>
            {
                Some(self.browser_receipt())
            }
            Self::Browser { .. } => None,
        }
    }
}

#[cfg(any(test, target_arch = "wasm32"))]
const fn browser_generation_has_restart_authority(
    accepted_generation: u64,
    persisted_generation: Option<u64>,
) -> bool {
    match persisted_generation {
        Some(persisted) => persisted == accepted_generation,
        None => false,
    }
}

#[cfg(any(test, target_arch = "wasm32"))]
pub(crate) const fn persisted_generation_after_browser_write(
    durable: bool,
    accepted_generation: u64,
    prior_persisted_generation: Option<u64>,
) -> Option<u64> {
    if durable {
        Some(accepted_generation)
    } else {
        prior_persisted_generation
    }
}

#[cfg_attr(target_arch = "wasm32", allow(dead_code))]
#[derive(Debug, Clone)]
pub(crate) struct UnreadableNativeBinding {
    pub(crate) canonical_path: PathBuf,
    pub(crate) reason: String,
}

impl PersistenceBinding {
    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) fn canonical_path(&self) -> Option<&Path> {
        match self {
            Self::Native { canonical_path, .. } => Some(canonical_path),
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) fn accepted_digest(&self) -> ContentDigest {
        match self {
            Self::Native {
                accepted_digest, ..
            } => *accepted_digest,
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum PersistenceError {
    #[cfg(not(target_arch = "wasm32"))]
    #[error(
        "the project changed outside RSpice; reload it or save a project copy before overwriting external changes"
    )]
    ExternalChange,
    #[cfg(not(target_arch = "wasm32"))]
    #[error("native canonical binding receipt mismatch: {0}")]
    NativeReceiptMismatch(String),
    #[error(transparent)]
    Project(#[from] ProjectIoError),
    #[cfg(not(target_arch = "wasm32"))]
    #[error("{0}")]
    Platform(String),
}

pub(crate) fn serialized_project(
    project: &ProjectFile,
) -> Result<(Vec<u8>, ContentDigest), PersistenceError> {
    let contents = crate::io::project_io::serialize_project_file(project)?;
    let bytes = contents.into_bytes();
    let digest = digest_bytes(&bytes);
    Ok((bytes, digest))
}

pub(crate) fn digest_bytes(bytes: &[u8]) -> ContentDigest {
    ContentDigest::from_bytes(Sha256::digest(bytes).into())
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
    pub(crate) binding_id: uuid::Uuid,
    pub(crate) backend: BrowserBindingBackend,
    pub(crate) project_id: String,
    pub(crate) accepted_generation: u64,
    pub(crate) expected_digest: Option<ContentDigest>,
    pub(crate) persisted_generation: Option<u64>,
}

#[cfg(target_arch = "wasm32")]
#[derive(Debug)]
pub(crate) enum BrowserOpenResult {
    Opened {
        handle_id: u64,
        display_name: String,
        bytes: Vec<u8>,
        digest: ContentDigest,
    },
    Cancelled,
    Failed(String),
}

#[cfg(target_arch = "wasm32")]
#[derive(Debug)]
pub(crate) enum BrowserRestoreResult {
    Missing,
    Restored {
        baseline: Box<ProjectFile>,
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

#[cfg(any(target_arch = "wasm32", test))]
#[derive(Debug, Clone, PartialEq, Eq)]
struct BrowserBindingMetadata {
    schema_version: u32,
    binding_id: String,
    project_id: String,
    accepted_generation: u64,
    accepted_digest: String,
    backend: BrowserBindingBackend,
    display_name: String,
}

#[cfg(any(target_arch = "wasm32", test))]
fn validate_browser_restore_facts(
    metadata: &BrowserBindingMetadata,
    receipt: &BrowserBindingReceipt,
    permission: &str,
    actual_digest: ContentDigest,
) -> Result<ContentDigest, String> {
    let accepted = validate_browser_binding_metadata(metadata, receipt)?;
    if permission != "granted" {
        return Err(format!(
            "browser file permission is {permission}; select the canonical project again"
        ));
    }
    if actual_digest != accepted {
        return Err("canonical browser project changed outside RSpice".to_owned());
    }
    Ok(accepted)
}

#[cfg(any(target_arch = "wasm32", test))]
fn validate_browser_binding_metadata(
    metadata: &BrowserBindingMetadata,
    receipt: &BrowserBindingReceipt,
) -> Result<ContentDigest, String> {
    if metadata.schema_version != BROWSER_BINDING_SCHEMA_VERSION {
        return Err(format!(
            "unsupported browser binding schema {}",
            metadata.schema_version
        ));
    }
    if metadata.binding_id != receipt.binding_id.to_string()
        || metadata.project_id != receipt.project_id
        || metadata.accepted_generation != receipt.accepted_generation
        || metadata.backend != receipt.backend
    {
        return Err("browser binding belongs to a different project identity".to_owned());
    }
    let accepted = metadata
        .accepted_digest
        .parse::<ContentDigest>()
        .map_err(|error| format!("browser binding digest is invalid: {error}"))?;
    if accepted != receipt.accepted_digest {
        return Err("browser binding receipt digest does not match IndexedDB".to_owned());
    }
    Ok(accepted)
}

#[cfg(any(target_arch = "wasm32", test))]
fn validate_browser_binding_identity(
    metadata: &BrowserBindingMetadata,
    receipt: &BrowserBindingReceipt,
) -> Result<(), String> {
    if metadata.schema_version != BROWSER_BINDING_SCHEMA_VERSION {
        return Err(format!(
            "unsupported browser binding schema {}",
            metadata.schema_version
        ));
    }
    if metadata.binding_id != receipt.binding_id.to_string()
        || metadata.project_id != receipt.project_id
        || metadata.backend != receipt.backend
    {
        return Err("browser binding belongs to a different canonical identity".to_owned());
    }
    Ok(())
}

#[cfg(any(target_arch = "wasm32", test))]
fn prove_browser_record_ownership(
    metadata: &BrowserBindingMetadata,
    receipt: &BrowserBindingReceipt,
) -> Result<(), String> {
    validate_browser_binding_identity(metadata, receipt)
}

#[cfg(any(target_arch = "wasm32", test))]
fn validate_binding_generation_commit(
    existing: Option<&BrowserBindingMetadata>,
    binding_id: uuid::Uuid,
    project_id: &str,
    backend: BrowserBindingBackend,
    expected_generation: Option<u64>,
    next_generation: u64,
) -> Result<(), String> {
    if !browser_generation_is_exact(next_generation)
        || expected_generation
            .is_some_and(|value| !browser_generation_is_exact(value) || next_generation <= value)
    {
        return Err("browser binding generation did not advance".to_owned());
    }
    match (existing, expected_generation) {
        (None, None) => Ok(()),
        (Some(metadata), Some(expected))
            if metadata.schema_version == BROWSER_BINDING_SCHEMA_VERSION
                && metadata.binding_id == binding_id.to_string()
                && metadata.project_id == project_id
                && metadata.backend == backend
                && metadata.accepted_generation == expected =>
        {
            Ok(())
        }
        _ => Err(
            "browser binding generation changed in another tab; reopen it or save a project copy"
                .to_owned(),
        ),
    }
}

#[cfg(any(target_arch = "wasm32", test))]
const fn browser_generation_is_exact(generation: u64) -> bool {
    generation > 0 && generation <= MAX_EXACT_BROWSER_GENERATION
}

#[cfg(any(target_arch = "wasm32", test))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BrowserPermissionDecision {
    Verify,
    Reconnect,
}

#[cfg(any(target_arch = "wasm32", test))]
fn browser_permission_decision(permission: &str) -> BrowserPermissionDecision {
    match permission {
        "granted" => BrowserPermissionDecision::Verify,
        _ => BrowserPermissionDecision::Reconnect,
    }
}

#[cfg(any(target_arch = "wasm32", test))]
#[derive(Debug, Clone, PartialEq, Eq)]
enum BrowserBindingCommitOutcome {
    Durable,
    SessionOnly(String),
}

#[cfg(any(target_arch = "wasm32", test))]
fn classify_browser_binding_commit(result: Result<(), String>) -> BrowserBindingCommitOutcome {
    match result {
        Ok(()) => BrowserBindingCommitOutcome::Durable,
        Err(error) => BrowserBindingCommitOutcome::SessionOnly(error),
    }
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
    if !browser_generation_is_exact(target.accepted_generation)
        || target
            .persisted_generation
            .is_some_and(|generation| !browser_generation_is_exact(generation))
    {
        return Err(
            "browser binding generation must be a nonzero JavaScript-exact integer".to_owned(),
        );
    }
    let operation = if let Some(handle_id) = target.handle_id {
        let handle = resolve_browser_handle(handle_id)?;
        BrowserWriteStart::Existing { handle, target }
    } else {
        match target.backend {
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
    let digest = digest_bytes(&bytes);
    let display_name = js_sys::Reflect::get(&handle, &wasm_bindgen::JsValue::from_str("name"))
        .ok()
        .and_then(|name| name.as_string())
        .unwrap_or_else(|| "project.rspiceproj".to_owned());
    let handle_id = register_browser_handle(handle);
    BrowserOpenResult::Opened {
        handle_id,
        display_name,
        bytes,
        digest,
    }
}


#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn observe_native_destination(
    path: &Path,
) -> Result<crate::io::durable_file::ExpectedContent, PersistenceError> {
    crate::io::durable_file::observe_expected_content(path)
        .map_err(|error| PersistenceError::Platform(error.to_string()))
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn publish_canonical_native(
    path: &Path,
    expected: crate::io::durable_file::ExpectedContent,
    bytes: &[u8],
) -> Result<ContentDigest, PersistenceError> {
    match crate::io::durable_file::compare_exchange_bytes(path, expected, bytes) {
        Ok(()) => Ok(digest_bytes(bytes)),
        Err(crate::io::durable_file::CompareExchangeError::Conflict { .. }) => {
            Err(PersistenceError::ExternalChange)
        }
        Err(crate::io::durable_file::CompareExchangeError::Io(error)) => {
            Err(PersistenceError::Platform(error.to_string()))
        }
        Err(error) => Err(PersistenceError::Platform(error.to_string())),
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn read_native_binding(
    path: &Path,
) -> Result<(ProjectFile, PersistenceBinding), PersistenceError> {
    let canonical_path = normalize_native_path(path)?;
    crate::io::durable_file::reconcile_publication(&canonical_path)
        .map_err(|error| PersistenceError::Platform(error.to_string()))?;
    let (project, digest) = crate::io::project_io::load_project_file_with_digest(&canonical_path)?;
    Ok((
        project,
        PersistenceBinding::Native {
            canonical_path,
            accepted_digest: digest,
        },
    ))
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn restore_native_binding(
    path: &Path,
    session_project_id: &str,
    receipt: &NativeBindingReceipt,
) -> Result<(ProjectFile, PersistenceBinding), PersistenceError> {
    if receipt.project_id != session_project_id {
        return Err(PersistenceError::NativeReceiptMismatch(
            "logical project identity differs from the restored session".to_owned(),
        ));
    }
    let canonical_path = normalize_native_path(path)?;
    if canonical_path != receipt.canonical_path {
        return Err(PersistenceError::NativeReceiptMismatch(
            "canonical pathname differs from the accepted pathname".to_owned(),
        ));
    }
    crate::io::durable_file::reconcile_publication(&canonical_path)
        .map_err(|error| PersistenceError::Platform(error.to_string()))?;
    let Some(project) = crate::io::project_io::load_project_file_with_expected_digest(
        &canonical_path,
        receipt.accepted_digest,
    )?
    else {
        return Err(PersistenceError::ExternalChange);
    };
    if project.workspace.project.id().to_string() != receipt.project_id {
        return Err(PersistenceError::NativeReceiptMismatch(
            "project file identity differs from the accepted logical project".to_owned(),
        ));
    }
    Ok((
        project,
        PersistenceBinding::Native {
            canonical_path,
            accepted_digest: receipt.accepted_digest,
        },
    ))
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn normalize_native_path(path: &Path) -> Result<PathBuf, PersistenceError> {
    if path.exists() {
        return std::fs::canonicalize(path)
            .map_err(|error| PersistenceError::Platform(error.to_string()));
    }
    let parent = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));
    let parent = std::fs::canonicalize(parent)
        .map_err(|error| PersistenceError::Platform(error.to_string()))?;
    let name = path.file_name().ok_or_else(|| {
        PersistenceError::Platform(format!("'{}' has no project filename", path.display()))
    })?;
    Ok(parent.join(name))
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn native_paths_refer_to_same_file(
    left: &Path,
    right: &Path,
) -> Result<bool, PersistenceError> {
    // A missing endpoint cannot currently alias an existing filesystem
    // object. In particular, a deleted canonical source must not prevent the
    // user from recovering work to an independently selected destination.
    if !left.exists() || !right.exists() {
        return Ok(false);
    }

    #[cfg(windows)]
    {
        Ok(windows_file_identity(left)? == windows_file_identity(right)?)
    }

    #[cfg(not(windows))]
    {
        let left_path = left;
        let right_path = right;
        let left = std::fs::metadata(left_path)
            .map_err(|error| PersistenceError::Platform(error.to_string()))?;
        let right = std::fs::metadata(right_path)
            .map_err(|error| PersistenceError::Platform(error.to_string()))?;

        #[cfg(unix)]
        {
            use std::os::unix::fs::MetadataExt as _;
            Ok(left.dev() == right.dev() && left.ino() == right.ino())
        }
        #[cfg(not(unix))]
        {
            let _ = (left, right);
            Ok(normalize_native_path(left_path)? == normalize_native_path(right_path)?)
        }
    }
}

#[cfg(windows)]
fn windows_file_identity(path: &Path) -> Result<(u32, u64), PersistenceError> {
    use std::os::windows::io::AsRawHandle as _;
    use windows_sys::Win32::Storage::FileSystem::{
        BY_HANDLE_FILE_INFORMATION, GetFileInformationByHandle,
    };

    let file =
        std::fs::File::open(path).map_err(|error| PersistenceError::Platform(error.to_string()))?;
    let mut information = unsafe { std::mem::zeroed::<BY_HANDLE_FILE_INFORMATION>() };
    let succeeded =
        unsafe { GetFileInformationByHandle(file.as_raw_handle() as *mut _, &mut information) };
    if succeeded == 0 {
        return Err(PersistenceError::Platform(
            std::io::Error::last_os_error().to_string(),
        ));
    }
    let file_index = ((information.nFileIndexHigh as u64) << 32) | information.nFileIndexLow as u64;
    Ok((information.dwVolumeSerialNumber, file_index))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn project_digest_is_exact_over_published_bytes() {
        let bytes = b"project bytes\n";
        assert_eq!(digest_bytes(bytes), digest_bytes(bytes));
        assert_ne!(digest_bytes(bytes), digest_bytes(b"project bytes"));
    }

    #[test]
    fn browser_storage_timeout_message_is_bounded_and_actionable() {
        assert_eq!(
            browser_timeout_message("browser staged file write"),
            "browser staged file write timed out after 15 seconds"
        );
    }

    fn browser_receipt(project_id: &str, digest: ContentDigest) -> BrowserBindingReceipt {
        BrowserBindingReceipt {
            binding_id: uuid::Uuid::from_u128(0x7d9a_1db3_55f2_4da2_82e1_992f_6e65_0f42),
            project_id: project_id.to_owned(),
            accepted_generation: 7,
            accepted_digest: digest,
            backend: BrowserBindingBackend::ExternalFile,
        }
    }

    fn browser_metadata(
        receipt: &BrowserBindingReceipt,
        digest: ContentDigest,
    ) -> BrowserBindingMetadata {
        BrowserBindingMetadata {
            schema_version: BROWSER_BINDING_SCHEMA_VERSION,
            binding_id: receipt.binding_id.to_string(),
            project_id: receipt.project_id.clone(),
            accepted_generation: receipt.accepted_generation,
            accepted_digest: digest.to_string(),
            backend: receipt.backend,
            display_name: "project.rspiceproj".to_owned(),
        }
    }

    #[test]
    fn browser_restore_protocol_distinguishes_prompt_from_revocation() {
        assert_eq!(
            browser_permission_decision("granted"),
            BrowserPermissionDecision::Verify
        );
        assert_eq!(
            browser_permission_decision("prompt"),
            BrowserPermissionDecision::Reconnect
        );
        assert_eq!(
            browser_permission_decision("denied"),
            BrowserPermissionDecision::Reconnect
        );
        assert_eq!(
            browser_permission_decision("unexpected"),
            BrowserPermissionDecision::Reconnect
        );
    }

    #[test]
    fn browser_restore_protocol_rejects_wrong_schema_identity_and_digest() {
        let accepted = digest_bytes(b"accepted");
        let receipt = browser_receipt("project-id", accepted);
        let metadata = browser_metadata(&receipt, accepted);
        validate_browser_binding_identity(&metadata, &receipt)
            .expect("structural browser binding identity matches");
        assert_eq!(
            validate_browser_restore_facts(&metadata, &receipt, "granted", accepted).unwrap(),
            accepted
        );

        let mut wrong_schema = metadata.clone();
        wrong_schema.schema_version += 1;
        assert!(validate_browser_binding_metadata(&wrong_schema, &receipt).is_err());

        let mut wrong_identity = metadata.clone();
        wrong_identity.project_id = "other-project".to_owned();
        assert!(validate_browser_binding_metadata(&wrong_identity, &receipt).is_err());

        let mut newer_generation = metadata.clone();
        newer_generation.accepted_generation += 1;
        assert!(validate_browser_binding_metadata(&newer_generation, &receipt).is_err());

        let mut other_backend = metadata.clone();
        other_backend.backend = BrowserBindingBackend::Opfs;
        assert!(prove_browser_record_ownership(&other_backend, &receipt).is_err());
        assert!(validate_browser_binding_identity(&other_backend, &receipt).is_err());
        assert!(validate_browser_binding_metadata(&other_backend, &receipt).is_err());

        assert!(
            validate_browser_restore_facts(
                &metadata,
                &receipt,
                "granted",
                digest_bytes(b"external change"),
            )
            .is_err()
        );
        assert!(validate_browser_restore_facts(&metadata, &receipt, "denied", accepted,).is_err());
    }

    #[test]
    fn browser_binding_generation_compare_exchange_is_fail_closed() {
        let digest = digest_bytes(b"accepted");
        let receipt = browser_receipt("project-id", digest);
        let metadata = browser_metadata(&receipt, digest);

        validate_binding_generation_commit(
            Some(&metadata),
            receipt.binding_id,
            &receipt.project_id,
            receipt.backend,
            Some(7),
            8,
        )
        .expect("matching generation advances");
        validate_binding_generation_commit(
            None,
            receipt.binding_id,
            &receipt.project_id,
            receipt.backend,
            None,
            1,
        )
        .expect("a fresh opaque binding can commit generation one");

        for (existing, expected, next) in [
            (Some(&metadata), Some(6), 8),
            (Some(&metadata), None, 8),
            (None, Some(7), 8),
            (Some(&metadata), Some(7), 7),
            (None, None, MAX_EXACT_BROWSER_GENERATION + 1),
            (
                Some(&metadata),
                Some(MAX_EXACT_BROWSER_GENERATION + 1),
                MAX_EXACT_BROWSER_GENERATION + 1,
            ),
        ] {
            assert!(
                validate_binding_generation_commit(
                    existing,
                    receipt.binding_id,
                    &receipt.project_id,
                    receipt.backend,
                    expected,
                    next,
                )
                .is_err()
            );
        }
    }

    #[test]
    fn verified_write_with_binding_failure_is_typed_session_only_success() {
        assert_eq!(
            classify_browser_binding_commit(Ok(())),
            BrowserBindingCommitOutcome::Durable
        );
        assert_eq!(
            classify_browser_binding_commit(Err("quota".to_owned())),
            BrowserBindingCommitOutcome::SessionOnly("quota".to_owned())
        );
    }

    #[test]
    fn session_only_browser_write_retains_last_durable_generation_without_restart_authority() {
        assert_eq!(
            persisted_generation_after_browser_write(false, 8, Some(7)),
            Some(7)
        );
        assert!(!browser_generation_has_restart_authority(8, Some(7)));
        assert_eq!(
            persisted_generation_after_browser_write(false, 1, None),
            None
        );
        assert!(!browser_generation_has_restart_authority(1, None));

        assert_eq!(
            persisted_generation_after_browser_write(true, 8, Some(7)),
            Some(8)
        );
        assert!(browser_generation_has_restart_authority(8, Some(8)));
    }

    #[test]
    fn mismatched_browser_record_never_establishes_eviction_ownership() {
        let digest = digest_bytes(b"accepted");
        let receipt = browser_receipt("project-a", digest);
        let mut other_project = browser_metadata(&receipt, digest);
        other_project.project_id = "project-b".to_owned();
        assert!(prove_browser_record_ownership(&other_project, &receipt).is_err());

        let mut other_binding = browser_metadata(&receipt, digest);
        other_binding.binding_id = uuid::Uuid::new_v4().to_string();
        assert!(prove_browser_record_ownership(&other_binding, &receipt).is_err());

        let owned = browser_metadata(&receipt, digest);
        prove_browser_record_ownership(&owned, &receipt)
            .expect("all durable identity fields prove record ownership");
    }
}
