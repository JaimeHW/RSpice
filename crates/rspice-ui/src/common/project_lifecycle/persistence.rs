//! Canonical project persistence identities and optimistic concurrency.

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

#[cfg(target_arch = "wasm32")]
pub(crate) fn start_browser_binding_persist(
    binding: PersistenceBinding,
    complete: impl FnOnce(Result<(), String>) + 'static,
) {
    let PersistenceBinding::Browser {
        handle_id,
        binding_id,
        backend,
        project_id,
        accepted_generation,
        display_name,
        accepted_digest,
        persisted_generation,
    } = binding;
    let handle = match resolve_browser_handle(handle_id) {
        Ok(handle) => handle,
        Err(error) => {
            complete(Err(error));
            return;
        }
    };
    wasm_bindgen_futures::spawn_local(async move {
        let lock = match acquire_browser_web_lock(binding_id, true).await {
            Ok(lock) => lock,
            Err(error) => {
                complete(Err(error));
                return;
            }
        };
        let result = persist_browser_binding(BrowserBindingPersistRequest {
            binding_id,
            backend,
            project_id: &project_id,
            accepted_generation,
            expected_generation: persisted_generation,
            handle: &handle,
            digest: accepted_digest,
            display_name: &display_name,
        })
        .await;
        let release = release_browser_web_lock(lock).await;
        if let Err(error) = &release {
            log::warn!("browser binding lock release reported an error after CAS: {error}");
        }
        complete(result);
    });
}

#[cfg(target_arch = "wasm32")]
enum BrowserWriteStart {
    Existing {
        handle: wasm_bindgen::JsValue,
        target: BrowserWriteTarget,
    },
    Picker {
        picker: js_sys::Promise,
        target: BrowserWriteTarget,
    },
    Opfs {
        target: BrowserWriteTarget,
    },
}

#[cfg(target_arch = "wasm32")]
async fn run_browser_write(
    operation: BrowserWriteStart,
    suggested_name: &str,
    persist_binding: bool,
    bytes: &[u8],
) -> BrowserWriteResult {
    let prepared = match operation {
        BrowserWriteStart::Existing { handle, target } => (handle, target, false, None),
        BrowserWriteStart::Picker { picker, target } => {
            let handle = match wasm_bindgen_futures::JsFuture::from(picker).await {
                Ok(handle) => handle,
                Err(error) if js_error_name(&error) == Some("AbortError") => {
                    return BrowserWriteResult::Cancelled;
                }
                Err(error) => return BrowserWriteResult::Failed(js_error(error)),
            };
            if let Err(error) = validate_browser_write_handle(&handle, true) {
                return BrowserWriteResult::Failed(error);
            }
            let handle_id = register_browser_handle(handle.clone());
            let mut target = target;
            target.handle_id = Some(handle_id);
            (handle, target, true, None)
        }
        BrowserWriteStart::Opfs { target } => {
            let handle = match open_opfs_binding_handle(target.binding_id, true).await {
                Ok(handle) => handle,
                Err(error) => return BrowserWriteResult::Failed(error),
            };
            if let Err(error) = validate_browser_write_handle(&handle, false) {
                return BrowserWriteResult::Failed(error);
            }
            let handle_id = register_browser_handle(handle.clone());
            let mut target = target;
            target.handle_id = Some(handle_id);
            (handle, target, true, Some(suggested_name.to_owned()))
        }
    };
    let (handle, target, newly_registered, display_name_override) = prepared;
    let lock = match acquire_browser_web_lock(
        target.binding_id,
        persist_binding || target.backend == BrowserBindingBackend::Opfs,
    )
    .await
    {
        Ok(lock) => lock,
        Err(error) => {
            if newly_registered {
                release_browser_handle(target.handle_id.expect("registered browser handle"));
            }
            return BrowserWriteResult::Failed(error);
        }
    };
    let result = run_browser_write_locked(
        &handle,
        &target,
        display_name_override.as_deref(),
        persist_binding,
        bytes,
    )
    .await;
    let release = release_browser_web_lock(lock).await;
    if let Err(error) = release {
        log::warn!("browser canonical lock release reported an error: {error}");
    }
    if newly_registered
        && matches!(
            result,
            BrowserWriteResult::Cancelled
                | BrowserWriteResult::ExternalChange { .. }
                | BrowserWriteResult::Failed(_)
        )
    {
        release_browser_handle(target.handle_id.expect("registered browser handle"));
    }
    result
}

#[cfg(target_arch = "wasm32")]
async fn run_browser_write_locked(
    handle: &wasm_bindgen::JsValue,
    target: &BrowserWriteTarget,
    display_name_override: Option<&str>,
    persist_binding: bool,
    bytes: &[u8],
) -> BrowserWriteResult {
    if target.backend == BrowserBindingBackend::ExternalFile {
        let permission = match call_promise_method(
            handle,
            "requestPermission",
            &[permission_options(BrowserPermissionMode::ReadWrite)],
        ) {
            Ok(permission) => permission,
            Err(error) => return BrowserWriteResult::Failed(error),
        };
        if let Err(error) =
            ensure_browser_permission(permission, BrowserPermissionMode::ReadWrite).await
        {
            return BrowserWriteResult::Failed(error);
        }
    }
    if persist_binding
        && browser_binding_store_supported()
        && let Err(error) = indexed_db_validate_generation(
            target.binding_id,
            &target.project_id,
            target.backend,
            target.persisted_generation,
        )
        .await
    {
        let observed_digest = match read_browser_handle_bytes(handle).await {
            Ok(current) => digest_bytes(&current),
            Err(read_error) => {
                return BrowserWriteResult::Failed(format!(
                    "{error}; current canonical bytes could not be inspected: {read_error}"
                ));
            }
        };
        return BrowserWriteResult::ExternalChange { observed_digest };
    }
    let expected_before_commit = match read_browser_handle_bytes(handle).await {
        Ok(current) => {
            let observed = digest_bytes(&current);
            if target
                .expected_digest
                .is_some_and(|expected| expected != observed)
            {
                return BrowserWriteResult::ExternalChange {
                    observed_digest: observed,
                };
            }
            // Even a newly picker-selected destination gets a byte receipt
            // after overwrite confirmation. The staged writable checks this
            // again immediately before close, preventing a late external edit
            // from being clobbered.
            observed
        }
        Err(error) => return BrowserWriteResult::Failed(error),
    };
    match write_browser_handle_bytes(handle, bytes, Some(expected_before_commit), target.backend)
        .await
    {
        Ok(()) => {}
        Err(BrowserWriteFailure::ExternalChange(observed_digest)) => {
            return BrowserWriteResult::ExternalChange { observed_digest };
        }
        Err(BrowserWriteFailure::Failed(error)) => return BrowserWriteResult::Failed(error),
    }
    let verified = match read_browser_handle_bytes(handle).await {
        Ok(verified) => verified,
        Err(error) => {
            return BrowserWriteResult::Failed(format!(
                "browser write completed, but read-back verification failed: {error}"
            ));
        }
    };
    let staged_digest = digest_bytes(bytes);
    if digest_bytes(&verified) != staged_digest {
        return BrowserWriteResult::Failed(
            "browser write completed, but read-back bytes do not match the staged project"
                .to_owned(),
        );
    }
    let display_name = display_name_override.map(str::to_owned).unwrap_or_else(|| {
        js_sys::Reflect::get(handle, &wasm_bindgen::JsValue::from_str("name"))
            .ok()
            .and_then(|name| name.as_string())
            .unwrap_or_else(|| "project.rspiceproj".to_owned())
    });
    let handle_id = target.handle_id.expect("prepared browser handle");
    if persist_binding {
        let commit = classify_browser_binding_commit(
            persist_browser_binding(BrowserBindingPersistRequest {
                binding_id: target.binding_id,
                backend: target.backend,
                project_id: &target.project_id,
                accepted_generation: target.accepted_generation,
                expected_generation: target.persisted_generation,
                handle,
                digest: staged_digest,
                display_name: &display_name,
            })
            .await,
        );
        if let BrowserBindingCommitOutcome::SessionOnly(error) = commit {
            return BrowserWriteResult::SavedSessionOnly {
                handle_id,
                binding_id: target.binding_id,
                backend: target.backend,
                project_id: target.project_id.clone(),
                generation: target.accepted_generation,
                display_name,
                digest: staged_digest,
                persistence_error: error,
            };
        }
    }
    BrowserWriteResult::Saved {
        handle_id,
        binding_id: target.binding_id,
        backend: target.backend,
        project_id: target.project_id.clone(),
        generation: target.accepted_generation,
        display_name,
        digest: staged_digest,
    }
}

#[cfg(target_arch = "wasm32")]
#[derive(Debug, Clone, Copy)]
enum BrowserPermissionMode {
    Read,
    ReadWrite,
}

#[cfg(target_arch = "wasm32")]
async fn ensure_browser_permission(
    permission: js_sys::Promise,
    mode: BrowserPermissionMode,
) -> Result<(), String> {
    // `requestPermission` may present a browser-owned prompt. Do not impose
    // the storage I/O timeout on a human decision; no file bytes or durable
    // binding state are mutated while this Promise is pending.
    let state = wasm_bindgen_futures::JsFuture::from(permission)
        .await
        .map_err(js_error)?;
    if state.as_string().as_deref() == Some("granted") {
        return Ok(());
    }
    let label = match mode {
        BrowserPermissionMode::Read => "read",
        BrowserPermissionMode::ReadWrite => "read/write",
    };
    Err(format!(
        "{label} permission for the selected project file was denied"
    ))
}

#[cfg(target_arch = "wasm32")]
async fn read_browser_handle_bytes(handle: &wasm_bindgen::JsValue) -> Result<Vec<u8>, String> {
    let file = await_browser_promise(
        call_promise_method(handle, "getFile", &[])?,
        "browser file metadata read",
    )
    .await?;
    let size = js_sys::Reflect::get(&file, &wasm_bindgen::JsValue::from_str("size"))
        .map_err(js_error)?
        .as_f64()
        .ok_or("browser file size is unavailable")?;
    if !size.is_finite()
        || size < 0.0
        || size > crate::io::project_io::MAX_PROJECT_FILE_BYTES as f64
    {
        return Err("browser project exceeds the supported project-size limit".to_owned());
    }
    let array_buffer = await_browser_promise(
        call_promise_method(&file, "arrayBuffer", &[])?,
        "browser file content read",
    )
    .await?;
    let array = js_sys::Uint8Array::new(&array_buffer);
    if array.length() as u64 > crate::io::project_io::MAX_PROJECT_FILE_BYTES {
        return Err("browser project exceeds the supported project-size limit".to_owned());
    }
    let mut bytes = vec![0_u8; array.length() as usize];
    array.copy_to(&mut bytes);
    Ok(bytes)
}

#[cfg(target_arch = "wasm32")]
async fn write_browser_handle_bytes(
    handle: &wasm_bindgen::JsValue,
    bytes: &[u8],
    expected_before_commit: Option<ContentDigest>,
    backend: BrowserBindingBackend,
) -> Result<(), BrowserWriteFailure> {
    let exclusive_options = js_sys::Object::new();
    set_js_field(
        &exclusive_options,
        "mode",
        &wasm_bindgen::JsValue::from_str("exclusive"),
    )
    .map_err(BrowserWriteFailure::Failed)?;
    let arguments = if backend == BrowserBindingBackend::ExternalFile {
        vec![exclusive_options.into()]
    } else {
        Vec::new()
    };
    let writable = call_promise_method(handle, "createWritable", &arguments)
        .map_err(BrowserWriteFailure::Failed)?;
    let writable = await_browser_promise(writable, "browser writable creation")
        .await
        .map_err(|error| {
            if backend == BrowserBindingBackend::ExternalFile {
                format!("exclusive canonical-file access is unavailable: {error}")
            } else {
                error
            }
        })
        .map_err(BrowserWriteFailure::Failed)?;
    let array = js_sys::Uint8Array::from(bytes);
    let write = match call_promise_method(&writable, "write", &[array.into()]) {
        Ok(write) => await_browser_promise(write, "browser staged file write").await,
        Err(error) => {
            let abort = abort_browser_writable(&writable).await;
            return Err(BrowserWriteFailure::Failed(
                format_browser_writable_failure("write", error, abort),
            ));
        }
    };
    if let Err(error) = write {
        let abort = abort_browser_writable(&writable).await;
        return Err(BrowserWriteFailure::Failed(
            format_browser_writable_failure("write", error, abort),
        ));
    }
    if let Some(expected) = expected_before_commit {
        match read_browser_handle_bytes(handle).await {
            Ok(current) if digest_bytes(&current) == expected => {}
            Ok(current) => {
                return match abort_browser_writable(&writable).await {
                    Ok(()) => Err(BrowserWriteFailure::ExternalChange(digest_bytes(&current))),
                    Err(abort_error) => Err(BrowserWriteFailure::Failed(format!(
                        "canonical browser project changed while staged bytes were pending, and staging abort failed: {abort_error}"
                    ))),
                };
            }
            Err(error) => {
                let abort = abort_browser_writable(&writable).await;
                return Err(BrowserWriteFailure::Failed(
                    format_browser_writable_failure("pre-commit verification", error, abort),
                ));
            }
        }
    }
    let close = match call_promise_method(&writable, "close", &[]) {
        Ok(close) => await_browser_promise(close, "browser staged file publication").await,
        Err(error) => {
            let abort = abort_browser_writable(&writable).await;
            return Err(BrowserWriteFailure::Failed(
                format_browser_writable_failure(
                    "close",
                    format!("{error}; publication outcome is uncertain"),
                    abort,
                ),
            ));
        }
    };
    if let Err(error) = close {
        let abort = abort_browser_writable(&writable).await;
        return Err(BrowserWriteFailure::Failed(
            format_browser_writable_failure(
                "close",
                format!("{error}; publication outcome is uncertain"),
                abort,
            ),
        ));
    }
    Ok(())
}

#[cfg(target_arch = "wasm32")]
enum BrowserWriteFailure {
    ExternalChange(ContentDigest),
    Failed(String),
}

#[cfg(target_arch = "wasm32")]
async fn abort_browser_writable(writable: &wasm_bindgen::JsValue) -> Result<(), String> {
    let abort = call_promise_method(writable, "abort", &[])?;
    await_browser_promise(abort, "browser staged file abort")
        .await
        .map(|_| ())
}

#[cfg(target_arch = "wasm32")]
fn format_browser_writable_failure(
    operation: &str,
    error: String,
    abort: Result<(), String>,
) -> String {
    match abort {
        Ok(()) => format!("browser project {operation} failed and staging was aborted: {error}"),
        Err(abort_error) => format!(
            "browser project {operation} failed: {error}; staging abort also failed: {abort_error}"
        ),
    }
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn start_browser_binding_restore(
    receipt: BrowserBindingReceipt,
    complete: impl FnOnce(BrowserRestoreResult) + 'static,
) {
    wasm_bindgen_futures::spawn_local(async move {
        let result = restore_browser_binding(&receipt).await;
        complete(result);
    });
}

#[cfg(target_arch = "wasm32")]
async fn restore_browser_binding(receipt: &BrowserBindingReceipt) -> BrowserRestoreResult {
    if !browser_binding_store_supported() {
        return BrowserRestoreResult::Unsupported(
            "IndexedDB binding persistence is unavailable".to_owned(),
        );
    }
    let binding_key = receipt.binding_id.to_string();
    let record = match indexed_db_get(&binding_key).await {
        Ok(Some(record)) => record,
        Ok(None) => return BrowserRestoreResult::Missing,
        Err(error) => return BrowserRestoreResult::Retryable(error),
    };
    let metadata = match metadata_from_record(&record) {
        Ok(metadata) => metadata,
        // The opaque key alone is not proof that this session owns a corrupt
        // or future record. Leave it untouched so a stale receipt can never
        // delete another project's binding authority.
        Err(error) => return BrowserRestoreResult::Evicted(error),
    };
    if let Err(error) = prove_browser_record_ownership(&metadata, receipt) {
        // Identity mismatch explicitly means ownership was not established.
        // Ignore the record; never evict it using facts supplied by this
        // untrusted receipt.
        return BrowserRestoreResult::Evicted(error);
    }
    let metadata_digest = match metadata.accepted_digest.parse::<ContentDigest>() {
        Ok(digest) => digest,
        Err(error) => {
            let _ = indexed_db_delete(&binding_key).await;
            return BrowserRestoreResult::Evicted(format!(
                "browser binding digest is invalid: {error}"
            ));
        }
    };
    let handle = match metadata.backend {
        BrowserBindingBackend::ExternalFile => {
            match js_sys::Reflect::get(&record, &wasm_bindgen::JsValue::from_str("handle")) {
                Ok(handle) if !handle.is_null() && !handle.is_undefined() => handle,
                _ => {
                    let _ = indexed_db_delete(&binding_key).await;
                    return BrowserRestoreResult::Evicted(
                        "browser binding has no structured-cloned file handle".to_owned(),
                    );
                }
            }
        }
        BrowserBindingBackend::Opfs => {
            match open_opfs_binding_handle(receipt.binding_id, false).await {
                Ok(handle) => handle,
                Err(error) => return BrowserRestoreResult::Retryable(error),
            }
        }
    };
    if let Err(error) = validate_browser_read_handle(
        &handle,
        metadata.backend == BrowserBindingBackend::ExternalFile,
    ) {
        let _ = indexed_db_delete(&binding_key).await;
        return BrowserRestoreResult::Evicted(error);
    }
    let permission = match metadata.backend {
        BrowserBindingBackend::Opfs => "granted".to_owned(),
        BrowserBindingBackend::ExternalFile => match call_promise_method(
            &handle,
            "queryPermission",
            &[permission_options(BrowserPermissionMode::Read)],
        ) {
            Ok(promise) => {
                match await_browser_promise(promise, "browser file permission query").await {
                    Ok(value) => value.as_string().unwrap_or_else(|| "unknown".to_owned()),
                    Err(error) => {
                        return BrowserRestoreResult::Retryable(format!(
                            "browser file permission query failed: {error}"
                        ));
                    }
                }
            }
            Err(error) => return BrowserRestoreResult::Retryable(error),
        },
    };
    let binding_from_receipt = |handle_id| PersistenceBinding::Browser {
        handle_id,
        binding_id: receipt.binding_id,
        backend: receipt.backend,
        project_id: receipt.project_id.clone(),
        accepted_generation: receipt.accepted_generation,
        display_name: metadata.display_name.clone(),
        accepted_digest: receipt.accepted_digest,
        persisted_generation: Some(metadata.accepted_generation),
    };
    let metadata_matches_receipt = validate_browser_binding_metadata(&metadata, receipt).is_ok();
    if !metadata_matches_receipt {
        let handle_id = register_browser_handle(handle);
        return BrowserRestoreResult::Conflict {
            binding: binding_from_receipt(handle_id),
            observed_digest: metadata_digest,
            reason: format!(
                "another tab committed generation {} while this session accepted generation {}",
                metadata.accepted_generation, receipt.accepted_generation
            ),
        };
    }
    if browser_permission_decision(&permission) == BrowserPermissionDecision::Reconnect {
        let handle_id = register_browser_handle(handle);
        return BrowserRestoreResult::ReconnectRequired {
            binding: binding_from_receipt(handle_id),
        };
    }
    let bytes = match read_browser_handle_bytes(&handle).await {
        Ok(bytes) => bytes,
        Err(error) => {
            return BrowserRestoreResult::Retryable(error);
        }
    };
    let actual_digest = digest_bytes(&bytes);
    let accepted_digest =
        match validate_browser_restore_facts(&metadata, receipt, &permission, actual_digest) {
            Ok(digest) => digest,
            Err(error) => {
                let handle_id = register_browser_handle(handle);
                return BrowserRestoreResult::Conflict {
                    binding: binding_from_receipt(handle_id),
                    observed_digest: actual_digest,
                    reason: error,
                };
            }
        };
    let text = match String::from_utf8(bytes) {
        Ok(text) => text,
        Err(error) => {
            return BrowserRestoreResult::Retryable(format!(
                "canonical browser project is not UTF-8: {error}"
            ));
        }
    };
    let mut baseline = match crate::io::project_io::load_project_text(&text, None) {
        Ok(project) => project,
        Err(error) => {
            return BrowserRestoreResult::Retryable(format!(
                "canonical browser project is invalid: {error}"
            ));
        }
    };
    if baseline.workspace.project.id().to_string() != receipt.project_id {
        let handle_id = register_browser_handle(handle);
        return BrowserRestoreResult::Conflict {
            binding: binding_from_receipt(handle_id),
            observed_digest: actual_digest,
            reason: "canonical browser project identity no longer matches its binding".to_owned(),
        };
    }
    baseline.workspace.project.path = None;
    let handle_id = register_browser_handle(handle);
    BrowserRestoreResult::Restored {
        baseline: Box::new(baseline),
        binding: PersistenceBinding::Browser {
            handle_id,
            binding_id: receipt.binding_id,
            backend: receipt.backend,
            project_id: receipt.project_id.clone(),
            accepted_generation: receipt.accepted_generation,
            display_name: metadata.display_name,
            accepted_digest,
            persisted_generation: Some(receipt.accepted_generation),
        },
    }
}

#[cfg(target_arch = "wasm32")]
struct BrowserBindingPersistRequest<'a> {
    binding_id: uuid::Uuid,
    backend: BrowserBindingBackend,
    project_id: &'a str,
    accepted_generation: u64,
    expected_generation: Option<u64>,
    handle: &'a wasm_bindgen::JsValue,
    digest: ContentDigest,
    display_name: &'a str,
}

#[cfg(target_arch = "wasm32")]
async fn persist_browser_binding(request: BrowserBindingPersistRequest<'_>) -> Result<(), String> {
    let BrowserBindingPersistRequest {
        binding_id,
        backend,
        project_id,
        accepted_generation,
        expected_generation,
        handle,
        digest,
        display_name,
    } = request;
    if !browser_binding_store_supported() {
        return Err("IndexedDB handle persistence is unavailable".to_owned());
    }
    if !browser_generation_is_exact(accepted_generation)
        || expected_generation.is_some_and(|generation| !browser_generation_is_exact(generation))
    {
        return Err(
            "browser binding generation must be a nonzero JavaScript-exact integer".to_owned(),
        );
    }
    let record = js_sys::Object::new();
    set_js_field(
        &record,
        "schemaVersion",
        &wasm_bindgen::JsValue::from_f64(BROWSER_BINDING_SCHEMA_VERSION as f64),
    )?;
    set_js_field(
        &record,
        "bindingId",
        &wasm_bindgen::JsValue::from_str(&binding_id.to_string()),
    )?;
    set_js_field(
        &record,
        "projectId",
        &wasm_bindgen::JsValue::from_str(project_id),
    )?;
    set_js_field(
        &record,
        "acceptedGeneration",
        &wasm_bindgen::JsValue::from_f64(accepted_generation as f64),
    )?;
    set_js_field(
        &record,
        "acceptedDigest",
        &wasm_bindgen::JsValue::from_str(&digest.to_string()),
    )?;
    set_js_field(
        &record,
        "backend",
        &wasm_bindgen::JsValue::from_str(browser_backend_name(backend)),
    )?;
    set_js_field(
        &record,
        "displayName",
        &wasm_bindgen::JsValue::from_str(display_name),
    )?;
    if backend == BrowserBindingBackend::ExternalFile {
        set_js_field(&record, "handle", handle)?;
    }
    indexed_db_compare_exchange_put(
        &binding_id.to_string(),
        &record.into(),
        binding_id,
        project_id,
        backend,
        expected_generation,
        accepted_generation,
    )
    .await
}

#[cfg(target_arch = "wasm32")]
fn metadata_from_record(record: &wasm_bindgen::JsValue) -> Result<BrowserBindingMetadata, String> {
    let schema = js_field(record, "schemaVersion")?
        .as_f64()
        .filter(|value| value.is_finite() && value.fract() == 0.0 && *value >= 0.0)
        .ok_or("browser binding schema version is invalid")? as u32;
    Ok(BrowserBindingMetadata {
        schema_version: schema,
        binding_id: js_string_field(record, "bindingId")?,
        project_id: js_string_field(record, "projectId")?,
        accepted_generation: js_u64_field(record, "acceptedGeneration")?,
        accepted_digest: js_string_field(record, "acceptedDigest")?,
        backend: browser_backend_from_name(&js_string_field(record, "backend")?)?,
        display_name: js_string_field(record, "displayName")?,
    })
}

#[cfg(target_arch = "wasm32")]
fn browser_backend_name(backend: BrowserBindingBackend) -> &'static str {
    match backend {
        BrowserBindingBackend::ExternalFile => "external-file",
        BrowserBindingBackend::Opfs => "opfs",
    }
}

#[cfg(target_arch = "wasm32")]
fn browser_backend_from_name(value: &str) -> Result<BrowserBindingBackend, String> {
    match value {
        "external-file" => Ok(BrowserBindingBackend::ExternalFile),
        "opfs" => Ok(BrowserBindingBackend::Opfs),
        _ => Err(format!("unknown browser binding backend {value}")),
    }
}

#[cfg(target_arch = "wasm32")]
fn js_u64_field(object: &wasm_bindgen::JsValue, name: &str) -> Result<u64, String> {
    js_field(object, name)?
        .as_f64()
        .filter(|value| {
            value.is_finite()
                && value.fract() == 0.0
                && *value > 0.0
                && *value <= MAX_EXACT_BROWSER_GENERATION as f64
        })
        .map(|value| value as u64)
        .ok_or_else(|| format!("browser binding field {name} is missing or invalid"))
}

#[cfg(target_arch = "wasm32")]
fn js_field(object: &wasm_bindgen::JsValue, name: &str) -> Result<wasm_bindgen::JsValue, String> {
    js_sys::Reflect::get(object, &wasm_bindgen::JsValue::from_str(name)).map_err(js_error)
}

#[cfg(target_arch = "wasm32")]
fn js_string_field(object: &wasm_bindgen::JsValue, name: &str) -> Result<String, String> {
    js_field(object, name)?
        .as_string()
        .filter(|value| !value.is_empty())
        .ok_or_else(|| format!("browser binding field {name} is missing or invalid"))
}

#[cfg(target_arch = "wasm32")]
fn set_js_field(
    object: &js_sys::Object,
    name: &str,
    value: &wasm_bindgen::JsValue,
) -> Result<(), String> {
    js_sys::Reflect::set(object, &wasm_bindgen::JsValue::from_str(name), value)
        .map(|_| ())
        .map_err(js_error)
}

#[cfg(target_arch = "wasm32")]
struct BrowserDatabase {
    value: wasm_bindgen::JsValue,
    _on_version_change: wasm_bindgen::closure::Closure<dyn FnMut()>,
}

#[cfg(target_arch = "wasm32")]
impl Drop for BrowserDatabase {
    fn drop(&mut self) {
        let _ = call_value_method(&self.value, "close", &[]);
    }
}

#[cfg(target_arch = "wasm32")]
async fn open_binding_database() -> Result<BrowserDatabase, String> {
    let window = web_sys::window().ok_or("browser window is unavailable")?;
    let factory = js_sys::Reflect::get(&window, &wasm_bindgen::JsValue::from_str("indexedDB"))
        .map_err(js_error)?;
    if factory.is_null() || factory.is_undefined() {
        return Err("IndexedDB is unavailable".to_owned());
    }
    let request = call_value_method(
        &factory,
        "open",
        &[
            wasm_bindgen::JsValue::from_str(BROWSER_BINDING_DATABASE),
            wasm_bindgen::JsValue::from_f64(BROWSER_BINDING_SCHEMA_VERSION as f64),
        ],
    )?;
    let upgrade_request = request.clone();
    let on_upgrade = wasm_bindgen::closure::Closure::<dyn FnMut()>::new(move || {
        let Ok(database) =
            js_sys::Reflect::get(&upgrade_request, &wasm_bindgen::JsValue::from_str("result"))
        else {
            return;
        };
        let exists = js_sys::Reflect::get(
            &database,
            &wasm_bindgen::JsValue::from_str("objectStoreNames"),
        )
        .ok()
        .and_then(|names| {
            call_value_method(
                &names,
                "contains",
                &[wasm_bindgen::JsValue::from_str(BROWSER_BINDING_STORE)],
            )
            .ok()
        })
        .and_then(|value| value.as_bool())
        .unwrap_or(false);
        if !exists {
            let _ = call_value_method(
                &database,
                "createObjectStore",
                &[wasm_bindgen::JsValue::from_str(BROWSER_BINDING_STORE)],
            );
        }
    });
    let database = await_idb_open_request(&request, on_upgrade).await?;
    let database_for_change = database.clone();
    let on_version_change = wasm_bindgen::closure::Closure::<dyn FnMut()>::new(move || {
        let _ = call_value_method(&database_for_change, "close", &[]);
    });
    js_sys::Reflect::set(
        &database,
        &wasm_bindgen::JsValue::from_str("onversionchange"),
        on_version_change.as_ref(),
    )
    .map_err(js_error)?;
    Ok(BrowserDatabase {
        value: database,
        _on_version_change: on_version_change,
    })
}

#[cfg(target_arch = "wasm32")]
async fn indexed_db_get(key: &str) -> Result<Option<wasm_bindgen::JsValue>, String> {
    let database = open_binding_database().await?;
    async {
        let transaction = call_value_method(
            &database.value,
            "transaction",
            &[
                wasm_bindgen::JsValue::from_str(BROWSER_BINDING_STORE),
                wasm_bindgen::JsValue::from_str("readonly"),
            ],
        )?;
        let store = call_value_method(
            &transaction,
            "objectStore",
            &[wasm_bindgen::JsValue::from_str(BROWSER_BINDING_STORE)],
        )?;
        let request = call_value_method(&store, "get", &[wasm_bindgen::JsValue::from_str(key)])?;
        let value = await_idb_request(&request).await?;
        Ok((!value.is_undefined()).then_some(value))
    }
    .await
}

#[cfg(target_arch = "wasm32")]
async fn indexed_db_validate_generation(
    binding_id: uuid::Uuid,
    project_id: &str,
    backend: BrowserBindingBackend,
    expected_generation: Option<u64>,
) -> Result<(), String> {
    let key = binding_id.to_string();
    let existing = indexed_db_get(&key).await?;
    let metadata = existing.as_ref().map(metadata_from_record).transpose()?;
    validate_binding_generation_commit(
        metadata.as_ref(),
        binding_id,
        project_id,
        backend,
        expected_generation,
        expected_generation.unwrap_or(0).saturating_add(1).max(1),
    )
}

#[cfg(target_arch = "wasm32")]
async fn indexed_db_compare_exchange_put(
    key: &str,
    record: &wasm_bindgen::JsValue,
    binding_id: uuid::Uuid,
    project_id: &str,
    backend: BrowserBindingBackend,
    expected_generation: Option<u64>,
    next_generation: u64,
) -> Result<(), String> {
    let database = open_binding_database().await?;
    async {
        let transaction = strict_readwrite_transaction(&database.value)?;
        let completion = await_idb_transaction(&transaction);
        let store = call_value_method(
            &transaction,
            "objectStore",
            &[wasm_bindgen::JsValue::from_str(BROWSER_BINDING_STORE)],
        )?;
        let get = call_value_method(&store, "get", &[wasm_bindgen::JsValue::from_str(key)])?;
        let existing = await_idb_request(&get).await?;
        let metadata = (!existing.is_undefined())
            .then(|| metadata_from_record(&existing))
            .transpose()?;
        if let Err(error) = validate_binding_generation_commit(
            metadata.as_ref(),
            binding_id,
            project_id,
            backend,
            expected_generation,
            next_generation,
        ) {
            let _ = call_value_method(&transaction, "abort", &[]);
            return Err(error);
        }
        let put = call_value_method(
            &store,
            "put",
            &[record.clone(), wasm_bindgen::JsValue::from_str(key)],
        )?;
        let _ = await_idb_request(&put).await?;
        completion.await
    }
    .await
}

#[cfg(target_arch = "wasm32")]
async fn indexed_db_delete(key: &str) -> Result<(), String> {
    let database = open_binding_database().await?;
    async {
        let transaction = strict_readwrite_transaction(&database.value)?;
        let completion = await_idb_transaction(&transaction);
        let store = call_value_method(
            &transaction,
            "objectStore",
            &[wasm_bindgen::JsValue::from_str(BROWSER_BINDING_STORE)],
        )?;
        let request = call_value_method(&store, "delete", &[wasm_bindgen::JsValue::from_str(key)])?;
        let _ = await_idb_request(&request).await?;
        completion.await
    }
    .await
}

/// Publish one browser recovery checkpoint without putting project-sized
/// bytes on the synchronous Web Storage path. Snapshot and manifest use
/// separate strict IndexedDB transactions: exact snapshot readback succeeds
/// before the small manifest becomes visible.
#[cfg(target_arch = "wasm32")]
pub(crate) fn start_browser_checkpoint_publish(
    project_prefix: String,
    manifest_key: String,
    snapshot_key: String,
    snapshot: Vec<u8>,
    manifest: String,
    max_retained: usize,
    complete: impl FnOnce(Result<(), String>) + 'static,
) {
    wasm_bindgen_futures::spawn_local(async move {
        let result = async {
            indexed_db_add_unique(
                &snapshot_key,
                &js_sys::Uint8Array::from(snapshot.as_slice()).into(),
            )
            .await?;
            let observed = indexed_db_get(&snapshot_key)
                .await?
                .ok_or_else(|| "IndexedDB did not retain project checkpoint bytes".to_owned())?;
            let observed = js_sys::Uint8Array::new(&observed).to_vec();
            if observed != snapshot {
                let _ = indexed_db_delete(&snapshot_key).await;
                return Err("IndexedDB checkpoint readback did not match staged bytes".to_owned());
            }
            if let Err(error) =
                indexed_db_add_unique(&manifest_key, &wasm_bindgen::JsValue::from_str(&manifest))
                    .await
            {
                let _ = indexed_db_delete(&snapshot_key).await;
                return Err(error);
            }
            let committed = indexed_db_get(&manifest_key)
                .await?
                .and_then(|value| value.as_string())
                .ok_or_else(|| "IndexedDB did not retain the checkpoint manifest".to_owned())?;
            if committed != manifest {
                let _ = indexed_db_delete(&manifest_key).await;
                let _ = indexed_db_delete(&snapshot_key).await;
                return Err(
                    "IndexedDB checkpoint manifest readback did not match staged text".to_owned(),
                );
            }

            let retention = async {
                let mut manifests =
                    indexed_db_matching_keys(&project_prefix, ".manifest").await?;
                manifests.sort_by(|left, right| right.cmp(left));
                for key in manifests.into_iter().skip(max_retained) {
                    let snapshot = key
                        .strip_suffix(".manifest")
                        .map(|stem| format!("{stem}.snapshot"));
                    indexed_db_delete(&key).await?;
                    if let Some(snapshot) = snapshot {
                        indexed_db_delete(&snapshot).await?;
                    }
                }
                let manifest_keys = indexed_db_matching_keys(&project_prefix, ".manifest").await?;
                let manifest_stems = manifest_keys
                    .iter()
                    .filter_map(|key| key.strip_suffix(".manifest"))
                    .collect::<std::collections::HashSet<_>>();
                let mut orphan_snapshots = indexed_db_matching_keys(&project_prefix, ".snapshot")
                    .await?
                    .into_iter()
                    .filter(|key| {
                        key.strip_suffix(".snapshot")
                            .is_some_and(|stem| !manifest_stems.contains(stem))
                    })
                    .collect::<Vec<_>>();
                orphan_snapshots.sort_by(|left, right| right.cmp(left));
                for key in orphan_snapshots.into_iter().skip(max_retained) {
                    indexed_db_delete(&key).await?;
                }
                Ok::<(), String>(())
            }
            .await;
            if let Err(error) = retention {
                log::warn!(
                    "Browser project checkpoint committed, but retention cleanup was incomplete: {error}"
                );
            }
            Ok(())
        }
        .await;
        complete(result);
    });
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn start_browser_checkpoint_list(
    project_prefix: String,
    complete: impl FnOnce(
        Result<
            (
                Vec<(String, Option<String>, String, Option<Vec<u8>>)>,
                Vec<String>,
            ),
            String,
        >,
    ) + 'static,
) {
    wasm_bindgen_futures::spawn_local(async move {
        let result = async {
            let manifest_keys = indexed_db_matching_keys(&project_prefix, ".manifest").await?;
            let mut records = Vec::with_capacity(manifest_keys.len());
            let mut paired_snapshot_keys = std::collections::HashSet::new();
            for manifest_key in manifest_keys {
                let Some(stem) = manifest_key.strip_suffix(".manifest") else {
                    continue;
                };
                let snapshot_key = format!("{stem}.snapshot");
                paired_snapshot_keys.insert(snapshot_key.clone());
                let manifest = indexed_db_get(&manifest_key)
                    .await?
                    .and_then(|value| value.as_string());
                let snapshot = indexed_db_get(&snapshot_key)
                    .await?
                    .map(|value| js_sys::Uint8Array::new(&value).to_vec());
                records.push((manifest_key, manifest, snapshot_key, snapshot));
            }
            let orphan_snapshots = indexed_db_matching_keys(&project_prefix, ".snapshot")
                .await?
                .into_iter()
                .filter(|key| !paired_snapshot_keys.contains(key))
                .collect();
            Ok((records, orphan_snapshots))
        }
        .await;
        complete(result);
    });
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn start_browser_checkpoint_read(
    snapshot_key: String,
    complete: impl FnOnce(Result<Vec<u8>, String>) + 'static,
) {
    wasm_bindgen_futures::spawn_local(async move {
        let result = indexed_db_get(&snapshot_key).await.and_then(|value| {
            value
                .map(|value| js_sys::Uint8Array::new(&value).to_vec())
                .ok_or_else(|| "browser project checkpoint snapshot is missing".to_owned())
        });
        complete(result);
    });
}

#[cfg(target_arch = "wasm32")]
async fn indexed_db_add_unique(key: &str, value: &wasm_bindgen::JsValue) -> Result<(), String> {
    let database = open_binding_database().await?;
    let transaction = strict_readwrite_transaction(&database.value)?;
    let completion = await_idb_transaction(&transaction);
    let store = call_value_method(
        &transaction,
        "objectStore",
        &[wasm_bindgen::JsValue::from_str(BROWSER_BINDING_STORE)],
    )?;
    let request = call_value_method(
        &store,
        "add",
        &[value.clone(), wasm_bindgen::JsValue::from_str(key)],
    )?;
    let _ = await_idb_request(&request).await.map_err(|error| {
        format!("project checkpoint identity already exists or could not be reserved: {error}")
    })?;
    completion.await
}

#[cfg(target_arch = "wasm32")]
async fn indexed_db_matching_keys(prefix: &str, suffix: &str) -> Result<Vec<String>, String> {
    let database = open_binding_database().await?;
    let transaction = call_value_method(
        &database.value,
        "transaction",
        &[
            wasm_bindgen::JsValue::from_str(BROWSER_BINDING_STORE),
            wasm_bindgen::JsValue::from_str("readonly"),
        ],
    )?;
    let store = call_value_method(
        &transaction,
        "objectStore",
        &[wasm_bindgen::JsValue::from_str(BROWSER_BINDING_STORE)],
    )?;
    let keys_request = call_value_method(&store, "getAllKeys", &[])?;
    let keys = js_sys::Array::from(&await_idb_request(&keys_request).await?);
    Ok(keys
        .iter()
        .filter_map(|key| key.as_string())
        .filter(|key| key.starts_with(prefix) && key.ends_with(suffix))
        .collect())
}

#[cfg(target_arch = "wasm32")]
fn strict_readwrite_transaction(
    database: &wasm_bindgen::JsValue,
) -> Result<wasm_bindgen::JsValue, String> {
    let options = js_sys::Object::new();
    set_js_field(
        &options,
        "durability",
        &wasm_bindgen::JsValue::from_str("strict"),
    )?;
    call_value_method(
        database,
        "transaction",
        &[
            wasm_bindgen::JsValue::from_str(BROWSER_BINDING_STORE),
            wasm_bindgen::JsValue::from_str("readwrite"),
            options.into(),
        ],
    )
}

#[cfg(target_arch = "wasm32")]
async fn await_idb_open_request(
    request: &wasm_bindgen::JsValue,
    on_upgrade: wasm_bindgen::closure::Closure<dyn FnMut()>,
) -> Result<wasm_bindgen::JsValue, String> {
    let success_request = request.clone();
    let error_request = request.clone();
    let request_for_handlers = request.clone();
    let mut on_upgrade = Some(on_upgrade);
    let reported = std::rc::Rc::new(std::cell::Cell::new(false));
    let reported_for_promise = reported.clone();
    let promise = js_sys::Promise::new(&mut move |resolve, reject| {
        let reported = reported_for_promise.clone();
        // A blocked request is reported immediately but retained until its
        // eventual success/error event so a late database is always closed.
        let retire = std::rc::Rc::new(std::cell::Cell::new(false));

        let success_reported = reported.clone();
        let success_retire = retire.clone();
        let success_resolve = resolve.clone();
        let success_request = success_request.clone();
        let success = wasm_bindgen::closure::Closure::<dyn FnMut()>::new(move || {
            let result =
                js_sys::Reflect::get(&success_request, &wasm_bindgen::JsValue::from_str("result"))
                    .unwrap_or(wasm_bindgen::JsValue::UNDEFINED);
            if success_reported.replace(true) {
                let _ = call_value_method(&result, "close", &[]);
            } else {
                let _ = success_resolve.call1(&wasm_bindgen::JsValue::UNDEFINED, &result);
            }
            success_retire.set(true);
        });

        let failure_reported = reported.clone();
        let failure_retire = retire.clone();
        let failure_reject = reject.clone();
        let error_request = error_request.clone();
        let failure = wasm_bindgen::closure::Closure::<dyn FnMut()>::new(move || {
            if !failure_reported.replace(true) {
                let error =
                    js_sys::Reflect::get(&error_request, &wasm_bindgen::JsValue::from_str("error"))
                        .unwrap_or_else(|_| {
                            wasm_bindgen::JsValue::from_str("IndexedDB open request failed")
                        });
                let _ = failure_reject.call1(&wasm_bindgen::JsValue::UNDEFINED, &error);
            }
            failure_retire.set(true);
        });

        let blocked_reported = reported.clone();
        let blocked_reject = reject.clone();
        let blocked = wasm_bindgen::closure::Closure::<dyn FnMut()>::new(move || {
            if !blocked_reported.replace(true) {
                let _ = blocked_reject.call1(
                    &wasm_bindgen::JsValue::UNDEFINED,
                    &wasm_bindgen::JsValue::from_str(
                        "IndexedDB schema upgrade is blocked by another RSpice tab; close the older tab and retry",
                    ),
                );
            }
        });

        let upgrade = on_upgrade.take().expect("single IndexedDB open setup");
        let success_value = success.as_ref().clone();
        let failure_value = failure.as_ref().clone();
        let blocked_value = blocked.as_ref().clone();
        let upgrade_value = upgrade.as_ref().clone();
        if let Err(error) =
            retain_browser_event_handlers(retire.clone(), vec![success, failure, blocked, upgrade])
        {
            retire.set(true);
            let _ = reject.call1(
                &wasm_bindgen::JsValue::UNDEFINED,
                &wasm_bindgen::JsValue::from_str(&error),
            );
            return;
        }
        for (property, value) in [
            ("onsuccess", success_value),
            ("onerror", failure_value),
            ("onblocked", blocked_value),
            ("onupgradeneeded", upgrade_value),
        ] {
            if let Err(error) = js_sys::Reflect::set(
                &request_for_handlers,
                &wasm_bindgen::JsValue::from_str(property),
                &value,
            ) {
                retire.set(true);
                let _ = reject.call1(&wasm_bindgen::JsValue::UNDEFINED, &error);
                return;
            }
        }
    });
    let result = await_browser_promise(promise, "IndexedDB open").await;
    if result.is_err() {
        // A late success observes this and closes the database instead of
        // leaking a connection after the lifecycle operation has timed out.
        reported.set(true);
    }
    result
}

#[cfg(target_arch = "wasm32")]
async fn await_idb_request(
    request: &wasm_bindgen::JsValue,
) -> Result<wasm_bindgen::JsValue, String> {
    await_idb_request_with_handlers(request, Vec::new()).await
}

#[cfg(target_arch = "wasm32")]
async fn await_idb_request_with_handlers(
    request: &wasm_bindgen::JsValue,
    extra_handlers: Vec<BrowserEventHandlerBinding>,
) -> Result<wasm_bindgen::JsValue, String> {
    let success_request = request.clone();
    let error_request = request.clone();
    let request_for_handlers = request.clone();
    let mut extra_handlers = Some(extra_handlers);
    let promise = js_sys::Promise::new(&mut move |resolve, reject| {
        let success_request = success_request.clone();
        let error_request = error_request.clone();
        let request_for_handlers = request_for_handlers.clone();
        let extras = extra_handlers.take().unwrap_or_default();
        let done = std::rc::Rc::new(std::cell::Cell::new(false));
        let success_done = done.clone();
        let resolve = resolve.clone();
        let success = wasm_bindgen::closure::Closure::<dyn FnMut()>::new(move || {
            if success_done.replace(true) {
                return;
            }
            let result =
                js_sys::Reflect::get(&success_request, &wasm_bindgen::JsValue::from_str("result"))
                    .unwrap_or(wasm_bindgen::JsValue::UNDEFINED);
            let _ = resolve.call1(&wasm_bindgen::JsValue::UNDEFINED, &result);
        });
        let failure_done = done.clone();
        let callback_reject = reject.clone();
        let failure = wasm_bindgen::closure::Closure::<dyn FnMut()>::new(move || {
            if failure_done.replace(true) {
                return;
            }
            let error =
                js_sys::Reflect::get(&error_request, &wasm_bindgen::JsValue::from_str("error"))
                    .unwrap_or_else(|_| {
                        wasm_bindgen::JsValue::from_str("IndexedDB request failed")
                    });
            let _ = callback_reject.call1(&wasm_bindgen::JsValue::UNDEFINED, &error);
        });
        let success_value = success.as_ref().clone();
        let failure_value = failure.as_ref().clone();
        let extra_values = extras
            .iter()
            .map(|(property, callback)| (*property, callback.as_ref().clone()))
            .collect::<Vec<_>>();
        let mut callbacks = Vec::with_capacity(2 + extras.len());
        callbacks.push(success);
        callbacks.push(failure);
        callbacks.extend(extras.into_iter().map(|(_, callback)| callback));
        if let Err(error) = retain_browser_event_handlers(done.clone(), callbacks) {
            done.set(true);
            let _ = reject.call1(
                &wasm_bindgen::JsValue::UNDEFINED,
                &wasm_bindgen::JsValue::from_str(&error),
            );
            return;
        }
        let set_handler = |property: &str, value: &wasm_bindgen::JsValue| {
            js_sys::Reflect::set(
                &request_for_handlers,
                &wasm_bindgen::JsValue::from_str(property),
                value,
            )
            .map(|_| ())
            .map_err(js_error)
        };
        let result = set_handler("onsuccess", &success_value)
            .and_then(|()| set_handler("onerror", &failure_value))
            .and_then(|()| {
                for (property, value) in &extra_values {
                    set_handler(property, value)?;
                }
                Ok(())
            });
        if let Err(error) = result {
            done.set(true);
            let _ = reject.call1(
                &wasm_bindgen::JsValue::UNDEFINED,
                &wasm_bindgen::JsValue::from_str(&error),
            );
        }
    });
    let result = await_browser_promise(promise, "IndexedDB request").await;
    if result.is_err()
        && let Ok(transaction) =
            js_sys::Reflect::get(request, &wasm_bindgen::JsValue::from_str("transaction"))
        && !transaction.is_null()
        && !transaction.is_undefined()
    {
        let _ = call_value_method(&transaction, "abort", &[]);
    }
    result
}

#[cfg(target_arch = "wasm32")]
fn await_idb_transaction(
    transaction: &wasm_bindgen::JsValue,
) -> impl std::future::Future<Output = Result<(), String>> + 'static {
    let transaction_for_handlers = transaction.clone();
    let error_transaction = transaction.clone();
    let transaction_for_abort = transaction.clone();
    let promise = js_sys::Promise::new(&mut move |resolve, reject| {
        let error_transaction = error_transaction.clone();
        let transaction_for_handlers = transaction_for_handlers.clone();
        let done = std::rc::Rc::new(std::cell::Cell::new(false));
        let complete_done = done.clone();
        let resolve = resolve.clone();
        let complete = wasm_bindgen::closure::Closure::<dyn FnMut()>::new(move || {
            if complete_done.replace(true) {
                return;
            }
            let _ = resolve.call0(&wasm_bindgen::JsValue::UNDEFINED);
        });
        let failure_done = done.clone();
        let callback_reject = reject.clone();
        let failure = wasm_bindgen::closure::Closure::<dyn FnMut()>::new(move || {
            if failure_done.replace(true) {
                return;
            }
            let error = js_sys::Reflect::get(
                &error_transaction,
                &wasm_bindgen::JsValue::from_str("error"),
            )
            .unwrap_or_else(|_| wasm_bindgen::JsValue::from_str("IndexedDB transaction failed"));
            let _ = callback_reject.call1(&wasm_bindgen::JsValue::UNDEFINED, &error);
        });
        let complete_value = complete.as_ref().clone();
        let failure_value = failure.as_ref().clone();
        if let Err(error) = retain_browser_event_handlers(done.clone(), vec![complete, failure]) {
            done.set(true);
            let _ = reject.call1(
                &wasm_bindgen::JsValue::UNDEFINED,
                &wasm_bindgen::JsValue::from_str(&error),
            );
            return;
        }
        let result = js_sys::Reflect::set(
            &transaction_for_handlers,
            &wasm_bindgen::JsValue::from_str("oncomplete"),
            &complete_value,
        )
        .map_err(js_error)
        .and_then(|_| {
            js_sys::Reflect::set(
                &transaction_for_handlers,
                &wasm_bindgen::JsValue::from_str("onabort"),
                &failure_value,
            )
            .map_err(js_error)
        })
        .and_then(|_| {
            js_sys::Reflect::set(
                &transaction_for_handlers,
                &wasm_bindgen::JsValue::from_str("onerror"),
                &failure_value,
            )
            .map_err(js_error)
        });
        if let Err(error) = result {
            done.set(true);
            let _ = reject.call1(
                &wasm_bindgen::JsValue::UNDEFINED,
                &wasm_bindgen::JsValue::from_str(&error),
            );
        }
    });
    async move {
        let result = await_browser_promise(promise, "IndexedDB transaction")
            .await
            .map(|_| ());
        if result.is_err() {
            let _ = call_value_method(&transaction_for_abort, "abort", &[]);
        }
        result
    }
}

#[cfg(target_arch = "wasm32")]
async fn await_browser_promise(
    promise: js_sys::Promise,
    operation: &str,
) -> Result<wasm_bindgen::JsValue, String> {
    let timeout = browser_timeout_promise(operation);
    let race = js_sys::Array::new();
    race.push(&promise);
    race.push(&timeout);
    wasm_bindgen_futures::JsFuture::from(js_sys::Promise::race(&race))
        .await
        .map_err(js_error)
}

#[cfg(target_arch = "wasm32")]
fn browser_timeout_promise(operation: &str) -> js_sys::Promise {
    let operation = operation.to_owned();
    js_sys::Promise::new(&mut move |_resolve, reject| {
        let timeout_reject = reject.clone();
        let message = browser_timeout_message(&operation);
        let callback = wasm_bindgen::closure::Closure::once_into_js(move || {
            let _ = timeout_reject.call1(
                &wasm_bindgen::JsValue::UNDEFINED,
                &wasm_bindgen::JsValue::from_str(&message),
            );
        });
        let Some(window) = web_sys::window() else {
            let _ = reject.call1(
                &wasm_bindgen::JsValue::UNDEFINED,
                &wasm_bindgen::JsValue::from_str("browser window is unavailable"),
            );
            return;
        };
        if let Err(error) = call_value_method(
            &window.into(),
            "setTimeout",
            &[
                callback,
                wasm_bindgen::JsValue::from_f64(BROWSER_ASYNC_TIMEOUT_MS as f64),
            ],
        ) {
            let _ = reject.call1(
                &wasm_bindgen::JsValue::UNDEFINED,
                &wasm_bindgen::JsValue::from_str(&error),
            );
        }
    })
}

#[cfg(target_arch = "wasm32")]
fn call_value_method(
    receiver: &wasm_bindgen::JsValue,
    name: &str,
    arguments: &[wasm_bindgen::JsValue],
) -> Result<wasm_bindgen::JsValue, String> {
    use wasm_bindgen::JsCast as _;

    let function = js_sys::Reflect::get(receiver, &wasm_bindgen::JsValue::from_str(name))
        .map_err(js_error)?
        .dyn_into::<js_sys::Function>()
        .map_err(|_| format!("browser object has no callable {name} method"))?;
    let args = js_sys::Array::new();
    for argument in arguments {
        args.push(argument);
    }
    function.apply(receiver, &args).map_err(js_error)
}

#[cfg(target_arch = "wasm32")]
fn register_browser_handle(handle: wasm_bindgen::JsValue) -> u64 {
    let id = NEXT_BROWSER_HANDLE_ID.with(|next| {
        let id = next.get();
        next.set(id.checked_add(1).unwrap_or(1));
        id
    });
    BROWSER_FILE_HANDLES.with(|handles| {
        handles.borrow_mut().insert(id, handle);
    });
    id
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn release_browser_handle(id: u64) {
    BROWSER_FILE_HANDLES.with(|handles| {
        handles.borrow_mut().remove(&id);
    });
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn clear_browser_handles() {
    BROWSER_FILE_HANDLES.with(|handles| handles.borrow_mut().clear());
}

#[cfg(target_arch = "wasm32")]
fn resolve_browser_handle(id: u64) -> Result<wasm_bindgen::JsValue, String> {
    let handle = BROWSER_FILE_HANDLES
        .with(|handles| handles.borrow().get(&id).cloned())
        .ok_or("the browser project handle is no longer live; choose Save as project copy")?;
    validate_browser_read_handle(&handle, false)?;
    Ok(handle)
}

#[cfg(target_arch = "wasm32")]
fn validate_browser_read_handle(
    handle: &wasm_bindgen::JsValue,
    external: bool,
) -> Result<(), String> {
    let kind = js_sys::Reflect::get(handle, &wasm_bindgen::JsValue::from_str("kind"))
        .map_err(js_error)?
        .as_string();
    if kind.as_deref() != Some("file") {
        return Err("browser project handle is not a file handle".to_owned());
    }
    let name = js_sys::Reflect::get(handle, &wasm_bindgen::JsValue::from_str("name"))
        .map_err(js_error)?
        .as_string();
    if name.is_none_or(|name| name.trim().is_empty()) {
        return Err("browser project handle has no valid file name".to_owned());
    }
    let methods: &[&str] = if external {
        &["getFile", "queryPermission", "requestPermission"]
    } else {
        &["getFile"]
    };
    for method in methods {
        let value = js_sys::Reflect::get(handle, &wasm_bindgen::JsValue::from_str(method))
            .map_err(js_error)?;
        if !value.is_function() {
            return Err(format!(
                "browser file handle is missing required method {method}"
            ));
        }
    }
    Ok(())
}

#[cfg(target_arch = "wasm32")]
fn validate_browser_write_handle(
    handle: &wasm_bindgen::JsValue,
    external: bool,
) -> Result<(), String> {
    validate_browser_read_handle(handle, external)?;
    let writable = js_sys::Reflect::get(handle, &wasm_bindgen::JsValue::from_str("createWritable"))
        .map_err(js_error)?;
    if !writable.is_function() {
        return Err("browser file handle is missing required method createWritable".to_owned());
    }
    Ok(())
}

#[cfg(target_arch = "wasm32")]
fn permission_options(mode: BrowserPermissionMode) -> wasm_bindgen::JsValue {
    let options = js_sys::Object::new();
    let mode = match mode {
        BrowserPermissionMode::Read => "read",
        BrowserPermissionMode::ReadWrite => "readwrite",
    };
    let _ = js_sys::Reflect::set(
        &options,
        &wasm_bindgen::JsValue::from_str("mode"),
        &wasm_bindgen::JsValue::from_str(mode),
    );
    options.into()
}

#[cfg(target_arch = "wasm32")]
fn browser_web_locks_supported() -> bool {
    browser_lock_manager().is_ok()
}

#[cfg(target_arch = "wasm32")]
fn browser_lock_manager() -> Result<wasm_bindgen::JsValue, String> {
    let window = web_sys::window().ok_or("browser window is unavailable")?;
    let navigator = js_sys::Reflect::get(&window, &wasm_bindgen::JsValue::from_str("navigator"))
        .map_err(js_error)?;
    let locks = js_sys::Reflect::get(&navigator, &wasm_bindgen::JsValue::from_str("locks"))
        .map_err(js_error)?;
    let request = js_sys::Reflect::get(&locks, &wasm_bindgen::JsValue::from_str("request"))
        .map_err(js_error)?;
    if !request.is_function() {
        return Err("Web Locks API is unavailable".to_owned());
    }
    Ok(locks)
}

#[cfg(target_arch = "wasm32")]
struct BrowserWebLock {
    release: js_sys::Function,
    request: js_sys::Promise,
    _callback:
        wasm_bindgen::closure::Closure<dyn FnMut(wasm_bindgen::JsValue) -> wasm_bindgen::JsValue>,
}

#[cfg(target_arch = "wasm32")]
async fn acquire_browser_web_lock(
    binding_id: uuid::Uuid,
    required: bool,
) -> Result<Option<BrowserWebLock>, String> {
    use wasm_bindgen::JsCast as _;

    let locks = match browser_lock_manager() {
        Ok(locks) => locks,
        Err(_error) if !required => return Ok(None),
        Err(error) => return Err(error),
    };
    let mut release_resolver = None;
    let release_promise = js_sys::Promise::new(&mut |resolve, _reject| {
        release_resolver = Some(resolve.clone());
    });
    let release = release_resolver.ok_or("could not initialize browser lock release")?;

    let acquired_resolver = std::rc::Rc::new(std::cell::RefCell::new(None));
    let acquired_resolver_for_promise = acquired_resolver.clone();
    let acquired_promise = js_sys::Promise::new(&mut move |resolve, _reject| {
        *acquired_resolver_for_promise.borrow_mut() = Some(resolve.clone());
    });
    let callback_resolver = acquired_resolver.clone();
    let callback_release = release_promise.clone();
    let callback = wasm_bindgen::closure::Closure::<
        dyn FnMut(wasm_bindgen::JsValue) -> wasm_bindgen::JsValue,
    >::new(move |lock: wasm_bindgen::JsValue| {
        let acquired = !lock.is_null() && !lock.is_undefined();
        if let Some(resolve) = callback_resolver.borrow_mut().take() {
            let _ = resolve.call1(
                &wasm_bindgen::JsValue::UNDEFINED,
                &wasm_bindgen::JsValue::from_bool(acquired),
            );
        }
        if acquired {
            callback_release.clone().into()
        } else {
            wasm_bindgen::JsValue::UNDEFINED
        }
    });
    let options = js_sys::Object::new();
    set_js_field(
        &options,
        "mode",
        &wasm_bindgen::JsValue::from_str("exclusive"),
    )?;
    set_js_field(
        &options,
        "ifAvailable",
        &wasm_bindgen::JsValue::from_bool(true),
    )?;
    let request = call_value_method(
        &locks,
        "request",
        &[
            wasm_bindgen::JsValue::from_str(&format!("rspice-binding-{binding_id}")),
            options.into(),
            callback.as_ref().clone(),
        ],
    )?
    .dyn_into::<js_sys::Promise>()
    .map_err(|_| "Web Locks request did not return a Promise".to_owned())?;
    let acquired = match await_browser_promise(acquired_promise, "Web Lock acquisition").await {
        Ok(value) => value.as_bool().unwrap_or(false),
        Err(error) => {
            // Resolve the held promise before returning. If a broken engine
            // invokes the callback late, it immediately releases the lock.
            let _ = release.call0(&wasm_bindgen::JsValue::UNDEFINED);
            callback.forget();
            return Err(error);
        }
    };
    if !acquired {
        let _ = await_browser_promise(request, "unavailable Web Lock cleanup").await;
        return Err("canonical project is being saved in another RSpice tab".to_owned());
    }
    Ok(Some(BrowserWebLock {
        release,
        request,
        _callback: callback,
    }))
}

#[cfg(target_arch = "wasm32")]
async fn release_browser_web_lock(lock: Option<BrowserWebLock>) -> Result<(), String> {
    let Some(lock) = lock else {
        return Ok(());
    };
    lock.release
        .call0(&wasm_bindgen::JsValue::UNDEFINED)
        .map_err(js_error)?;
    await_browser_promise(lock.request, "Web Lock release")
        .await
        .map(|_| ())
}

#[cfg(target_arch = "wasm32")]
async fn open_opfs_binding_handle(
    binding_id: uuid::Uuid,
    create: bool,
) -> Result<wasm_bindgen::JsValue, String> {
    let window = web_sys::window().ok_or("browser window is unavailable")?;
    let navigator = js_sys::Reflect::get(&window, &wasm_bindgen::JsValue::from_str("navigator"))
        .map_err(js_error)?;
    let storage = js_sys::Reflect::get(&navigator, &wasm_bindgen::JsValue::from_str("storage"))
        .map_err(js_error)?;
    let root = await_browser_promise(
        call_promise_method(&storage, "getDirectory", &[])?,
        "OPFS root access",
    )
    .await?;
    let options = js_sys::Object::new();
    set_js_field(
        &options,
        "create",
        &wasm_bindgen::JsValue::from_bool(create),
    )?;
    let directory = await_browser_promise(
        call_promise_method(
            &root,
            "getDirectoryHandle",
            &[
                wasm_bindgen::JsValue::from_str("rspice-projects"),
                options.clone().into(),
            ],
        )?,
        "OPFS project directory access",
    )
    .await?;
    await_browser_promise(
        call_promise_method(
            &directory,
            "getFileHandle",
            &[
                wasm_bindgen::JsValue::from_str(&format!("{binding_id}.rspiceproj")),
                options.into(),
            ],
        )?,
        "OPFS project file access",
    )
    .await
}

#[cfg(target_arch = "wasm32")]
fn call_promise_method(
    receiver: &wasm_bindgen::JsValue,
    name: &str,
    arguments: &[wasm_bindgen::JsValue],
) -> Result<js_sys::Promise, String> {
    use wasm_bindgen::JsCast as _;

    let function = js_sys::Reflect::get(receiver, &wasm_bindgen::JsValue::from_str(name))
        .map_err(js_error)?
        .dyn_into::<js_sys::Function>()
        .map_err(|_| format!("browser object has no callable {name} method"))?;
    let args = js_sys::Array::new();
    for argument in arguments {
        args.push(argument);
    }
    function
        .apply(receiver, &args)
        .map_err(js_error)?
        .dyn_into::<js_sys::Promise>()
        .map_err(|_| format!("browser {name} method did not return a Promise"))
}

#[cfg(target_arch = "wasm32")]
fn js_error(value: wasm_bindgen::JsValue) -> String {
    js_sys::Reflect::get(&value, &wasm_bindgen::JsValue::from_str("message"))
        .ok()
        .and_then(|message| message.as_string())
        .or_else(|| value.as_string())
        .unwrap_or_else(|| format!("browser error: {value:?}"))
}

#[cfg(target_arch = "wasm32")]
fn js_error_name(value: &wasm_bindgen::JsValue) -> Option<&str> {
    // Returning an owned string would be simpler, but callers only compare
    // one stable DOMException name. Store neither JS references nor names in
    // lifecycle state.
    match js_sys::Reflect::get(value, &wasm_bindgen::JsValue::from_str("name"))
        .ok()
        .and_then(|name| name.as_string())
        .as_deref()
    {
        Some("AbortError") => Some("AbortError"),
        _ => None,
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
