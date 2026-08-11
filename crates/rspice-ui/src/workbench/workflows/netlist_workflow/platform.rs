//! Native and browser source pickers.
//!
//! The two targets present the same workflow: the native side resolves a
//! path synchronously, the browser side polls a permission-scoped handle.

use super::import::{NetlistImportMode, netlist_import_start_block_reason};
use super::staging::stage_netlist_import;
use super::*;

#[cfg(not(target_arch = "wasm32"))]
pub(super) fn import_netlist_with_mode(state: &mut AppState, mode: NetlistImportMode) -> bool {
    if let Some(reason) = netlist_import_start_block_reason(state, mode) {
        state.push_user_message(ConsoleMessage::error(format!(
            "{} is unavailable: {reason}",
            mode.dialog_title()
        )));
        return false;
    }
    let transaction =
        match crate::workbench::lifecycle::project_lifecycle::begin_project_replacement(state) {
            Ok(transaction) => transaction,
            Err(error) => {
                state.push_user_message(ConsoleMessage::error(format!(
                    "{} is unavailable: {error}",
                    mode.dialog_title()
                )));
                return false;
            }
        };
    let loaded = show_open_netlist_dialog(mode).and_then(|path| {
        std::fs::read(&path)
            .map(|bytes| (path, bytes))
            .map_err(|error| error.to_string())
    });
    match loaded {
        Ok((path, bytes)) => {
            if let Err(error) =
                crate::workbench::lifecycle::project_lifecycle::validate_project_replacement(
                    state,
                    transaction,
                )
            {
                crate::workbench::lifecycle::project_lifecycle::cancel_transaction(state);
                state.push_user_message(ConsoleMessage::error(format!(
                    "{} was cancelled because the project changed: {error}",
                    mode.dialog_title()
                )));
                return false;
            }
            let display_name = path
                .file_name()
                .map(|name| name.to_string_lossy().to_string())
                .unwrap_or_else(|| path.display().to_string());
            stage_netlist_import(state, transaction, mode, bytes, Some(path), display_name)
        }
        Err(error) => {
            crate::workbench::lifecycle::project_lifecycle::cancel_transaction(state);
            if error != "cancelled" {
                state.push_user_message(ConsoleMessage::error(format!(
                    "SPICE deck import failed: {error}"
                )));
            }
            false
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn open_netlist_project(state: &mut AppState) -> bool {
    let started = import_netlist_with_mode(state, NetlistImportMode::OpenProject);
    route_started_netlist_import(state, started)
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn import_netlist(state: &mut AppState) -> bool {
    let started = import_netlist_with_mode(state, NetlistImportMode::ImportIntoProject);
    route_started_netlist_import(state, started)
}

#[cfg(not(target_arch = "wasm32"))]
pub(super) fn show_open_netlist_dialog(
    mode: NetlistImportMode,
) -> Result<std::path::PathBuf, String> {
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
pub(crate) fn request_dependency_relink(state: &mut AppState, logical_identity: &str) -> bool {
    if let Err(error) = crate::workbench::documents::netlist_document::begin_dependency_relink(
        state,
        logical_identity,
    ) {
        state.push_user_message(ConsoleMessage::error(error));
        return false;
    }
    let picked = rfd::FileDialog::new()
        .add_filter(NETLIST_FILTER.0, NETLIST_FILTER.1)
        .add_filter("SPICE Include", &["inc", "lib", "mdl"])
        .add_filter("All Files", &["*"])
        .set_title("Relink SPICE Dependency")
        .pick_file();
    let Some(path) = picked else {
        crate::workbench::documents::netlist_document::cancel_dependency_relink(state);
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
            crate::workbench::documents::netlist_document::cancel_dependency_relink(state);
            state.push_user_message(ConsoleMessage::error(format!(
                "SPICE dependency relink failed: {error}"
            )));
            return false;
        }
    };
    let source = match rspice_core::netlist::decode_source_bytes(&bytes) {
        Ok(source) => source,
        Err(error) => {
            crate::workbench::documents::netlist_document::cancel_dependency_relink(state);
            state.push_user_message(ConsoleMessage::error(format!(
                "SPICE dependency relink failed: selected source could not be decoded: {error}"
            )));
            return false;
        }
    };
    let display_name = path
        .file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_else(|| path.display().to_string());
    match crate::workbench::documents::netlist_document::commit_dependency_relink(
        state,
        source,
        display_name,
        Some(path.display().to_string()),
    ) {
        Ok(()) => {
            let message = state
                .ui
                .messages()
                .text(crate::workbench::MessageId::NetlistRelinkSucceeded);
            state.push_user_message(ConsoleMessage::info(message));
            true
        }
        Err(error) => {
            crate::workbench::documents::netlist_document::cancel_dependency_relink(state);
            state.push_user_message(ConsoleMessage::error(format!(
                "SPICE dependency relink failed: {error}"
            )));
            false
        }
    }
}

#[cfg(target_arch = "wasm32")]
pub(super) fn import_netlist_with_mode(state: &mut AppState, mode: NetlistImportMode) -> bool {
    if let Some(reason) = netlist_import_start_block_reason(state, mode) {
        state.push_user_message(ConsoleMessage::error(format!(
            "{} is unavailable: {reason}",
            mode.dialog_title()
        )));
        return false;
    }
    match start_browser_netlist_import(state, mode) {
        Ok(()) => true,
        Err(error) => {
            state.push_user_message(ConsoleMessage::error(format!(
                "SPICE deck import failed: {error}"
            )));
            false
        }
    }
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn open_netlist_project(state: &mut AppState) -> bool {
    let started = import_netlist_with_mode(state, NetlistImportMode::OpenProject);
    route_started_netlist_import(state, started)
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn import_netlist(state: &mut AppState) -> bool {
    let started = import_netlist_with_mode(state, NetlistImportMode::ImportIntoProject);
    route_started_netlist_import(state, started)
}

/// Browser picker completions are consumed by the Netlist surface. Route a
/// successfully started open/import transaction to that owning page before
/// the asynchronous file read can complete. Native uses the same transition
/// so the staged review always opens in a deterministic workspace.
pub(super) fn route_started_netlist_import(state: &mut AppState, started: bool) -> bool {
    if started {
        state.workbench.workspace = crate::workbench::state::Workspace::Netlist;
        state.ui.code_workspace.page =
            crate::workbench::documents::code_workspace::CodeWorkspacePage::Netlist;
    }
    started
}

#[cfg(target_arch = "wasm32")]
pub(super) enum BrowserNetlistImportResult {
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
pub(super) struct BrowserNetlistImportCompletion {
    token: crate::workbench::browser::file_import::TextImportToken,
    transaction: crate::product::TransactionId,
    mode: NetlistImportMode,
    result: BrowserNetlistImportResult,
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
pub(crate) fn request_dependency_relink(state: &mut AppState, logical_identity: &str) -> bool {
    if let Err(error) = crate::workbench::documents::netlist_document::begin_dependency_relink(
        state,
        logical_identity,
    ) {
        state.push_user_message(ConsoleMessage::error(error));
        return false;
    }
    let token = match crate::workbench::browser::file_import::try_begin_text_import(
        crate::workbench::browser::file_import::BrowserTextImportKind::Netlist,
    ) {
        Ok(token) => token,
        Err(error) => {
            crate::workbench::documents::netlist_document::cancel_dependency_relink(state);
            state.push_user_message(ConsoleMessage::error(error));
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
pub(crate) fn poll_browser_dependency_relink(state: &mut AppState) -> bool {
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
                crate::workbench::documents::netlist_document::cancel_dependency_relink(state);
                state.push_user_message(ConsoleMessage::error(
                    "A bundle cannot relink one dependency. Select the exact SPICE member source.",
                ));
                return false;
            }
            let source = match rspice_core::netlist::decode_source_bytes(&bytes) {
                Ok(source) => source,
                Err(error) => {
                    crate::workbench::documents::netlist_document::cancel_dependency_relink(state);
                    state.push_user_message(ConsoleMessage::error(format!(
                        "SPICE dependency relink failed: selected source could not be decoded: {error}"
                    )));
                    return false;
                }
            };
            match crate::workbench::documents::netlist_document::commit_dependency_relink(
                state, source, file.name, None,
            ) {
                Ok(()) => {
                    let message = state
                        .ui
                        .messages()
                        .text(crate::workbench::MessageId::NetlistRelinkSucceeded);
                    state.push_user_message(ConsoleMessage::info(message));
                    true
                }
                Err(error) => {
                    crate::workbench::documents::netlist_document::cancel_dependency_relink(state);
                    state.push_user_message(ConsoleMessage::error(format!(
                        "SPICE dependency relink failed: {error}"
                    )));
                    false
                }
            }
        }
        BrowserDependencyRelinkResult::Failed(error) => {
            crate::workbench::documents::netlist_document::cancel_dependency_relink(state);
            state.push_user_message(ConsoleMessage::error(format!(
                "SPICE dependency relink failed: {error}"
            )));
            false
        }
        BrowserDependencyRelinkResult::Cancelled => {
            crate::workbench::documents::netlist_document::cancel_dependency_relink(state);
            false
        }
    }
}

#[cfg(target_arch = "wasm32")]
pub(super) fn start_browser_netlist_import(
    state: &mut AppState,
    mode: NetlistImportMode,
) -> Result<(), String> {
    let transaction =
        crate::workbench::lifecycle::project_lifecycle::begin_project_replacement(state)
            .map_err(|error| error.to_string())?;
    let token = crate::workbench::browser::file_import::try_begin_text_import(
        crate::workbench::browser::file_import::BrowserTextImportKind::Netlist,
    )
    .inspect_err(|_| {
        crate::workbench::lifecycle::project_lifecycle::cancel_transaction(state);
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

#[cfg(target_arch = "wasm32")]
pub(crate) fn poll_browser_netlist_import(state: &mut AppState) -> bool {
    let Some(completion) = BROWSER_NETLIST_IMPORT_RESULT.with(|slot| slot.borrow_mut().take())
    else {
        return false;
    };
    if !crate::workbench::browser::file_import::finish_text_import(completion.token) {
        crate::workbench::lifecycle::project_lifecycle::cancel_transaction_if(
            state,
            completion.transaction,
        );
        return false;
    }
    if let Err(error) = crate::workbench::lifecycle::project_lifecycle::validate_project_replacement(
        state,
        completion.transaction,
    ) {
        crate::workbench::lifecycle::project_lifecycle::cancel_transaction_if(
            state,
            completion.transaction,
        );
        state.push_user_message(ConsoleMessage::error(format!(
            "{} was cancelled because the project changed: {error}",
            completion.mode.dialog_title()
        )));
        return false;
    }
    match completion.result {
        BrowserNetlistImportResult::Loaded(file) => stage_netlist_import(
            state,
            completion.transaction,
            completion.mode,
            file.original_bytes
                .unwrap_or_else(|| file.contents.into_bytes()),
            None,
            file.name,
        ),
        BrowserNetlistImportResult::Failed(error) => {
            crate::workbench::lifecycle::project_lifecycle::cancel_transaction_if(
                state,
                completion.transaction,
            );
            state.push_user_message(ConsoleMessage::error(format!(
                "SPICE deck import failed: {error}"
            )));
            false
        }
        BrowserNetlistImportResult::Cancelled => {
            crate::workbench::lifecycle::project_lifecycle::cancel_transaction_if(
                state,
                completion.transaction,
            );
            false
        }
    }
}
