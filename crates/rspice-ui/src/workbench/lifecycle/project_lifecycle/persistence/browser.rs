//! Browser persistence: File System Access handles and their bindings.
//!
//! A browser binding is only usable while the page still holds a live handle
//! with granted permission, so every operation re-proves ownership and
//! permission before it writes — a stored record is never trusted on its own.
//! Permission prompts and picker cancellation are distinguished from real
//! failures, because a user declining is not an error to report as one.

// Every item below is browser-only, so on native there is nothing here to
// need the parent's names.
#[cfg(target_arch = "wasm32")]
use super::*;

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
pub(super) enum BrowserWriteStart {
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
pub(super) async fn run_browser_write(
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
pub(super) async fn run_browser_write_locked(
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
pub(super) enum BrowserPermissionMode {
    Read,
    ReadWrite,
}

#[cfg(target_arch = "wasm32")]
pub(super) async fn ensure_browser_permission(
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
pub(super) async fn read_browser_handle_bytes(
    handle: &wasm_bindgen::JsValue,
) -> Result<Vec<u8>, String> {
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
pub(super) async fn write_browser_handle_bytes(
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
pub(super) enum BrowserWriteFailure {
    ExternalChange(ContentDigest),
    Failed(String),
}

#[cfg(target_arch = "wasm32")]
pub(super) async fn abort_browser_writable(writable: &wasm_bindgen::JsValue) -> Result<(), String> {
    let abort = call_promise_method(writable, "abort", &[])?;
    await_browser_promise(abort, "browser staged file abort")
        .await
        .map(|_| ())
}

#[cfg(target_arch = "wasm32")]
pub(super) fn format_browser_writable_failure(
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
pub(super) async fn restore_browser_binding(
    receipt: &BrowserBindingReceipt,
) -> BrowserRestoreResult {
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
pub(super) struct BrowserBindingPersistRequest<'a> {
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
pub(super) async fn persist_browser_binding(
    request: BrowserBindingPersistRequest<'_>,
) -> Result<(), String> {
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
pub(super) fn metadata_from_record(
    record: &wasm_bindgen::JsValue,
) -> Result<BrowserBindingMetadata, String> {
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
pub(super) fn browser_backend_name(backend: BrowserBindingBackend) -> &'static str {
    match backend {
        BrowserBindingBackend::ExternalFile => "external-file",
        BrowserBindingBackend::Opfs => "opfs",
    }
}

#[cfg(target_arch = "wasm32")]
pub(super) fn browser_backend_from_name(value: &str) -> Result<BrowserBindingBackend, String> {
    match value {
        "external-file" => Ok(BrowserBindingBackend::ExternalFile),
        "opfs" => Ok(BrowserBindingBackend::Opfs),
        _ => Err(format!("unknown browser binding backend {value}")),
    }
}

#[cfg(target_arch = "wasm32")]
pub(super) fn js_u64_field(object: &wasm_bindgen::JsValue, name: &str) -> Result<u64, String> {
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
pub(super) fn js_field(
    object: &wasm_bindgen::JsValue,
    name: &str,
) -> Result<wasm_bindgen::JsValue, String> {
    js_sys::Reflect::get(object, &wasm_bindgen::JsValue::from_str(name)).map_err(js_error)
}

#[cfg(target_arch = "wasm32")]
pub(super) fn js_string_field(
    object: &wasm_bindgen::JsValue,
    name: &str,
) -> Result<String, String> {
    js_field(object, name)?
        .as_string()
        .filter(|value| !value.is_empty())
        .ok_or_else(|| format!("browser binding field {name} is missing or invalid"))
}

#[cfg(target_arch = "wasm32")]
pub(super) fn set_js_field(
    object: &js_sys::Object,
    name: &str,
    value: &wasm_bindgen::JsValue,
) -> Result<(), String> {
    js_sys::Reflect::set(object, &wasm_bindgen::JsValue::from_str(name), value)
        .map(|_| ())
        .map_err(js_error)
}

#[cfg(target_arch = "wasm32")]
pub(super) struct BrowserDatabase {
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
pub(super) async fn open_binding_database() -> Result<BrowserDatabase, String> {
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
pub(super) async fn indexed_db_get(key: &str) -> Result<Option<wasm_bindgen::JsValue>, String> {
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
pub(super) async fn indexed_db_validate_generation(
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
pub(super) async fn indexed_db_compare_exchange_put(
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
pub(super) async fn indexed_db_delete(key: &str) -> Result<(), String> {
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
pub(super) async fn indexed_db_add_unique(
    key: &str,
    value: &wasm_bindgen::JsValue,
) -> Result<(), String> {
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
pub(super) async fn indexed_db_matching_keys(
    prefix: &str,
    suffix: &str,
) -> Result<Vec<String>, String> {
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
pub(super) fn strict_readwrite_transaction(
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
pub(super) async fn await_idb_open_request(
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
pub(super) async fn await_idb_request(
    request: &wasm_bindgen::JsValue,
) -> Result<wasm_bindgen::JsValue, String> {
    await_idb_request_with_handlers(request, Vec::new()).await
}

#[cfg(target_arch = "wasm32")]
pub(super) async fn await_idb_request_with_handlers(
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
pub(super) fn await_idb_transaction(
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
pub(super) async fn await_browser_promise(
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
pub(super) fn browser_timeout_promise(operation: &str) -> js_sys::Promise {
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
pub(super) fn call_value_method(
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
pub(super) fn register_browser_handle(handle: wasm_bindgen::JsValue) -> u64 {
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
pub(super) fn resolve_browser_handle(id: u64) -> Result<wasm_bindgen::JsValue, String> {
    let handle = BROWSER_FILE_HANDLES
        .with(|handles| handles.borrow().get(&id).cloned())
        .ok_or("the browser project handle is no longer live; choose Save as project copy")?;
    validate_browser_read_handle(&handle, false)?;
    Ok(handle)
}

#[cfg(target_arch = "wasm32")]
pub(super) fn validate_browser_read_handle(
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
pub(super) fn validate_browser_write_handle(
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
pub(super) fn permission_options(mode: BrowserPermissionMode) -> wasm_bindgen::JsValue {
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
pub(super) fn browser_web_locks_supported() -> bool {
    browser_lock_manager().is_ok()
}

#[cfg(target_arch = "wasm32")]
pub(super) fn browser_lock_manager() -> Result<wasm_bindgen::JsValue, String> {
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
pub(super) struct BrowserWebLock {
    release: js_sys::Function,
    request: js_sys::Promise,
    _callback:
        wasm_bindgen::closure::Closure<dyn FnMut(wasm_bindgen::JsValue) -> wasm_bindgen::JsValue>,
}

#[cfg(target_arch = "wasm32")]
pub(super) async fn acquire_browser_web_lock(
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
pub(super) async fn release_browser_web_lock(lock: Option<BrowserWebLock>) -> Result<(), String> {
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
pub(super) async fn open_opfs_binding_handle(
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
pub(super) fn call_promise_method(
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
pub(super) fn js_error(value: wasm_bindgen::JsValue) -> String {
    js_sys::Reflect::get(&value, &wasm_bindgen::JsValue::from_str("message"))
        .ok()
        .and_then(|message| message.as_string())
        .or_else(|| value.as_string())
        .unwrap_or_else(|| format!("browser error: {value:?}"))
}

#[cfg(target_arch = "wasm32")]
pub(super) fn js_error_name(value: &wasm_bindgen::JsValue) -> Option<&str> {
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
