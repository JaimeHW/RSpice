//! Native and browser source pickers.
//!
//! The two targets present the same workflow: the native side resolves a
//! path synchronously, the browser side polls a permission-scoped handle.

use super::import::NetlistImportMode;
use super::*;

#[cfg(not(target_arch = "wasm32"))]
pub fn show_open_netlist_dialog(mode: NetlistImportMode) -> Result<std::path::PathBuf, String> {
    rfd::FileDialog::new()
        .add_filter(NETLIST_FILTER.0, NETLIST_FILTER.1)
        .add_filter("RSpice Netlist Bundle", &["zip"])
        .add_filter("All Files", &["*"])
        .set_title(mode.dialog_title())
        .pick_file()
        .ok_or_else(|| "cancelled".to_string())
}

/// Reacquire source/permission authority for one exact retained dependency.
/// Native selection commits synchronously; browser selection is completed by
/// `poll_browser_dependency_relink` and remains guarded by both the global
/// picker token and the root document revision captured before the picker.
#[cfg(not(target_arch = "wasm32"))]
pub fn request_dependency_relink(
    model: &mut Model,
    netlist: &mut NetlistDocumentState,
    open_documents: &mut std::collections::HashSet<crate::workbench::state::WorkspaceDocumentId>,
    runs: &mut rspice_app_runs::workbench::slots::runs::RunsState,
    workbench: &mut crate::workbench::state::WorkbenchSession,
    logical_identity: &str,
) -> bool {
    if let Err(error) = crate::workbench::documents::netlist_document::begin_dependency_relink(
        model,
        netlist,
        logical_identity,
    ) {
        workbench.prompt(error);
        return false;
    }
    let picked = rfd::FileDialog::new()
        .add_filter(NETLIST_FILTER.0, NETLIST_FILTER.1)
        .add_filter("SPICE Include", &["inc", "lib", "mdl"])
        .add_filter("All Files", &["*"])
        .set_title("Relink SPICE Dependency")
        .pick_file();
    let Some(path) = picked else {
        crate::workbench::documents::netlist_document::cancel_dependency_relink(netlist);
        return false;
    };
    let bytes = match std::fs::metadata(&path)
        .map_err(|error| error.to_string())
        .and_then(|metadata| {
            if metadata.len() > crate::io::project_io::MAX_PROJECT_FILE_BYTES {
                Err(format!(
                    "Selected dependency exceeds the supported {}-byte project file limit.",
                    crate::io::project_io::MAX_PROJECT_FILE_BYTES
                ))
            } else {
                std::fs::read(&path).map_err(|error| error.to_string())
            }
        }) {
        Ok(bytes) => bytes,
        Err(error) => {
            crate::workbench::documents::netlist_document::cancel_dependency_relink(netlist);
            record_relink_failure(model, &error);
            return false;
        }
    };
    let source = match rspice_core::netlist::decode_source_bytes(&bytes) {
        Ok(source) => source,
        Err(error) => {
            crate::workbench::documents::netlist_document::cancel_dependency_relink(netlist);
            record_relink_failure(
                model,
                &format!("the selected file could not be decoded: {error}"),
            );
            return false;
        }
    };
    let display_name = path
        .file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_else(|| path.display().to_string());
    match crate::workbench::documents::netlist_document::commit_dependency_relink(
        model,
        netlist,
        open_documents,
        runs,
        source,
        display_name,
        Some(path.display().to_string()),
    ) {
        Ok(()) => {
            record_relink_success(model, workbench);
            true
        }
        Err(error) => {
            crate::workbench::documents::netlist_document::cancel_dependency_relink(netlist);
            record_relink_failure(model, &error);
            false
        }
    }
}

fn record_relink_success(model: &mut Model, workbench: &crate::workbench::state::WorkbenchSession) {
    let message = workbench
        .messages()
        .text(crate::workbench::MessageId::NetlistRelinkSucceeded);
    model.record(LogSeverity::Info, LogSource::Netlist, message);
}

fn record_relink_failure(model: &mut Model, error: &str) {
    model.record(
        LogSeverity::Error,
        LogSource::Netlist,
        format!("SPICE dependency relink failed: {error}"),
    );
}

/// Browser picker completions are consumed by the Netlist surface. Route a
/// successfully started open/import transaction to that owning page before
/// the asynchronous file read can complete. Native uses the same transition
/// so the staged review always opens in a deterministic workspace.
pub fn route_started_netlist_import(
    workbench: &mut crate::workbench::state::WorkbenchSession,
    page: &mut crate::workbench::documents::code_workspace::CodeWorkspacePage,
    started: bool,
) -> bool {
    if started {
        workbench.workspace = crate::workbench::state::Workspace::Netlist;
        *page = crate::workbench::documents::code_workspace::CodeWorkspacePage::Netlist;
    }
    started
}

#[cfg(target_arch = "wasm32")]
pub enum BrowserNetlistImportResult {
    Loaded(crate::workbench::browser::file_import::PickedTextFile),
    Failed(String),
    Cancelled,
}

#[cfg(target_arch = "wasm32")]
pub(super) enum BrowserDependencyRelinkResult {
    Loaded(crate::workbench::browser::file_import::PickedTextFile),
    Failed(String),
    Cancelled,
}

#[cfg(target_arch = "wasm32")]
pub(super) struct BrowserDependencyRelinkCompletion {
    token: crate::workbench::browser::file_import::TextImportToken,
    result: BrowserDependencyRelinkResult,
}

#[cfg(target_arch = "wasm32")]
pub struct BrowserNetlistImportCompletion {
    token: crate::workbench::browser::file_import::TextImportToken,
    pub transaction: crate::product::TransactionId,
    pub mode: NetlistImportMode,
    pub result: BrowserNetlistImportResult,
}

#[cfg(target_arch = "wasm32")]
thread_local! {
    static BROWSER_NETLIST_IMPORT_RESULT: std::cell::RefCell<Option<BrowserNetlistImportCompletion>> =
        const { std::cell::RefCell::new(None) };
}

#[cfg(target_arch = "wasm32")]
thread_local! {
    static BROWSER_DEPENDENCY_RELINK_RESULT: std::cell::RefCell<Option<BrowserDependencyRelinkCompletion>> =
        const { std::cell::RefCell::new(None) };
}

#[cfg(target_arch = "wasm32")]
pub fn request_dependency_relink(
    model: &mut Model,
    netlist: &mut NetlistDocumentState,
    _open_documents: &mut std::collections::HashSet<crate::workbench::state::WorkspaceDocumentId>,
    _runs: &mut rspice_app_runs::workbench::slots::runs::RunsState,
    workbench: &mut crate::workbench::state::WorkbenchSession,
    logical_identity: &str,
) -> bool {
    if let Err(error) = crate::workbench::documents::netlist_document::begin_dependency_relink(
        model,
        netlist,
        logical_identity,
    ) {
        workbench.prompt(error);
        return false;
    }
    let token = match crate::workbench::browser::file_import::try_begin_text_import(
        crate::workbench::browser::file_import::BrowserTextImportKind::Netlist,
    ) {
        Ok(token) => token,
        Err(error) => {
            crate::workbench::documents::netlist_document::cancel_dependency_relink(netlist);
            workbench.prompt(error);
            return false;
        }
    };
    crate::workbench::browser::file_import::pick_text_file(
        "SPICE Dependency",
        &["cir", "sp", "spice", "net", "ckt", "inc", "lib", "mdl"],
        move |result| {
            let result = match result {
                Ok(Some(file)) => BrowserDependencyRelinkResult::Loaded(file),
                Ok(None) => BrowserDependencyRelinkResult::Cancelled,
                Err(error) => BrowserDependencyRelinkResult::Failed(error),
            };
            BROWSER_DEPENDENCY_RELINK_RESULT.with(|slot| {
                *slot.borrow_mut() = Some(BrowserDependencyRelinkCompletion { token, result });
            });
        },
    );
    true
}

#[cfg(target_arch = "wasm32")]
pub fn poll_browser_dependency_relink(
    model: &mut Model,
    netlist: &mut NetlistDocumentState,
    open_documents: &mut std::collections::HashSet<crate::workbench::state::WorkspaceDocumentId>,
    runs: &mut rspice_app_runs::workbench::slots::runs::RunsState,
    workbench: &mut crate::workbench::state::WorkbenchSession,
) -> bool {
    let Some(completion) = BROWSER_DEPENDENCY_RELINK_RESULT.with(|slot| slot.borrow_mut().take())
    else {
        return false;
    };
    if !crate::workbench::browser::file_import::finish_text_import(completion.token) {
        return false;
    }
    match completion.result {
        BrowserDependencyRelinkResult::Loaded(file) => {
            let bytes = file
                .original_bytes
                .unwrap_or_else(|| file.contents.into_bytes());
            if bytes.starts_with(b"PK\x03\x04") {
                crate::workbench::documents::netlist_document::cancel_dependency_relink(netlist);
                workbench.prompt(
                    "A bundle cannot relink one dependency. Select the SPICE source file itself.",
                );
                return false;
            }
            let source = match rspice_core::netlist::decode_source_bytes(&bytes) {
                Ok(source) => source,
                Err(error) => {
                    crate::workbench::documents::netlist_document::cancel_dependency_relink(
                        netlist,
                    );
                    record_relink_failure(
                        model,
                        &format!("the selected file could not be decoded: {error}"),
                    );
                    return false;
                }
            };
            match crate::workbench::documents::netlist_document::commit_dependency_relink(
                model,
                netlist,
                open_documents,
                runs,
                source,
                file.name,
                None,
            ) {
                Ok(()) => {
                    record_relink_success(model, workbench);
                    true
                }
                Err(error) => {
                    crate::workbench::documents::netlist_document::cancel_dependency_relink(
                        netlist,
                    );
                    record_relink_failure(model, &error);
                    false
                }
            }
        }
        BrowserDependencyRelinkResult::Failed(error) => {
            crate::workbench::documents::netlist_document::cancel_dependency_relink(netlist);
            record_relink_failure(model, &error);
            false
        }
        BrowserDependencyRelinkResult::Cancelled => {
            crate::workbench::documents::netlist_document::cancel_dependency_relink(netlist);
            false
        }
    }
}

/// Open the browser's picker for a SPICE deck or netlist bundle under the
/// project replacement `transaction`; its completion lands for
/// [`take_browser_netlist_completion`]. A picker already open refuses, and
/// cancels the transaction.
#[cfg(target_arch = "wasm32")]
pub fn pick_browser_netlist(
    platform: &mut rspice_app_services::workbench::slots::platform::PlatformState,
    transaction: crate::product::TransactionId,
    mode: NetlistImportMode,
) -> Result<(), String> {
    let token = crate::workbench::browser::file_import::try_begin_text_import(
        crate::workbench::browser::file_import::BrowserTextImportKind::Netlist,
    )
    .inspect_err(|_| {
        crate::workbench::lifecycle::project_lifecycle::cancel_transaction(platform);
    })?;

    crate::workbench::browser::file_import::pick_text_file(
        "SPICE Deck or RSpice Netlist Bundle",
        &["cir", "sp", "spice", "net", "ckt", "zip"],
        move |result| {
            let event = match result {
                Ok(Some(file)) => BrowserNetlistImportResult::Loaded(file),
                Ok(None) => BrowserNetlistImportResult::Cancelled,
                Err(error) => BrowserNetlistImportResult::Failed(error),
            };
            BROWSER_NETLIST_IMPORT_RESULT.with(|slot| {
                *slot.borrow_mut() = Some(BrowserNetlistImportCompletion {
                    token,
                    transaction,
                    mode,
                    result: event,
                });
            });
        },
    );
    Ok(())
}

/// The browser picker's finished netlist import, once one has landed and
/// its picker token is still current; a stale completion cancels its
/// transaction and is dropped.
#[cfg(target_arch = "wasm32")]
pub fn take_browser_netlist_completion(
    platform: &mut rspice_app_services::workbench::slots::platform::PlatformState,
) -> Option<BrowserNetlistImportCompletion> {
    let completion = BROWSER_NETLIST_IMPORT_RESULT.with(|slot| slot.borrow_mut().take())?;
    if !crate::workbench::browser::file_import::finish_text_import(completion.token) {
        crate::workbench::lifecycle::project_lifecycle::cancel_transaction_if(
            platform,
            completion.transaction,
        );
        return None;
    }
    Some(completion)
}
