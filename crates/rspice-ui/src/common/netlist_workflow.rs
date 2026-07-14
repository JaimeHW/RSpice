use crate::common::app::{AppState, ConsoleMessage};

pub const NETLIST_FILTER: (&str, &[&str]) = ("SPICE Deck", &["cir", "sp", "spice", "net", "ckt"]);

pub(crate) fn apply_imported_netlist(
    state: &mut AppState,
    source: String,
    source_path: Option<std::path::PathBuf>,
    display_name: &str,
) -> bool {
    if source.trim().is_empty() {
        state.push_user_message(ConsoleMessage::error(format!(
            "SPICE deck import failed: {display_name} is empty"
        )));
        return false;
    }

    state.clear_design_execution_context();
    state.workspace.netlist_source = Some(source.clone());
    state.workspace.netlist_source_path = source_path;
    state.workspace.set_netlist_source_dirty(true);
    state.simulation.netlist_content = source;
    state.ui.netlist.revision = state.ui.netlist.revision.wrapping_add(1);
    state
        .workbench
        .activate(crate::workbench::state::Workspace::Netlist);
    state.push_user_message(ConsoleMessage::info(format!(
        "Imported SPICE deck: {display_name}"
    )));
    true
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn import_netlist(state: &mut AppState) -> bool {
    match show_open_netlist_dialog().and_then(|path| {
        std::fs::read_to_string(&path)
            .map(|contents| (path, contents))
            .map_err(|error| error.to_string())
    }) {
        Ok((path, contents)) => {
            let display_name = path
                .file_name()
                .map(|name| name.to_string_lossy().to_string())
                .unwrap_or_else(|| path.display().to_string());
            apply_imported_netlist(state, contents, Some(path), &display_name)
        }
        Err(error) => {
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
fn show_open_netlist_dialog() -> Result<std::path::PathBuf, String> {
    rfd::FileDialog::new()
        .add_filter(NETLIST_FILTER.0, NETLIST_FILTER.1)
        .add_filter("All Files", &["*"])
        .set_title("Import SPICE Deck")
        .pick_file()
        .ok_or_else(|| "cancelled".to_string())
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn import_netlist(state: &mut AppState) -> bool {
    match start_browser_netlist_import() {
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
enum BrowserNetlistImportResult {
    Loaded(crate::common::browser_file_import::PickedTextFile),
    Failed(String),
    Cancelled,
}

#[cfg(target_arch = "wasm32")]
thread_local! {
    static BROWSER_NETLIST_IMPORT_RESULT: std::cell::RefCell<Option<BrowserNetlistImportResult>> =
        const { std::cell::RefCell::new(None) };
}

#[cfg(target_arch = "wasm32")]
fn start_browser_netlist_import() -> Result<(), String> {
    crate::common::browser_file_import::try_begin_text_import(
        crate::common::browser_file_import::BrowserTextImportKind::Netlist,
    )?;

    crate::common::browser_file_import::pick_text_file(
        NETLIST_FILTER.0,
        NETLIST_FILTER.1,
        |result| {
            let event = match result {
                Ok(Some(file)) => BrowserNetlistImportResult::Loaded(file),
                Ok(None) => BrowserNetlistImportResult::Cancelled,
                Err(error) => BrowserNetlistImportResult::Failed(error),
            };
            BROWSER_NETLIST_IMPORT_RESULT.with(|slot| {
                *slot.borrow_mut() = Some(event);
            });
        },
    );
    Ok(())
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn poll_browser_netlist_import(state: &mut AppState) -> bool {
    let event = BROWSER_NETLIST_IMPORT_RESULT.with(|slot| slot.borrow_mut().take());
    if event.is_some() {
        crate::common::browser_file_import::finish_text_import(
            crate::common::browser_file_import::BrowserTextImportKind::Netlist,
        );
    }
    match event {
        Some(BrowserNetlistImportResult::Loaded(file)) => {
            apply_imported_netlist(state, file.contents, None, &file.name)
        }
        Some(BrowserNetlistImportResult::Failed(error)) => {
            state.push_user_message(ConsoleMessage::error(format!(
                "SPICE deck import failed: {error}"
            )));
            false
        }
        Some(BrowserNetlistImportResult::Cancelled) | None => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn imported_netlist_becomes_dirty_manual_source_in_netlist_workspace() {
        let mut state = AppState::default();
        state.simulation.start_run();
        assert!(state.simulation.has_results());

        let imported = apply_imported_netlist(
            &mut state,
            "deck\nV1 out 0 1\nR1 out 0 1k\n.op\n.end\n".to_owned(),
            Some(std::path::PathBuf::from("bias.cir")),
            "bias.cir",
        );

        assert!(imported);
        assert_eq!(
            state.workbench.workspace,
            crate::workbench::state::Workspace::Netlist
        );
        assert_eq!(
            state.workspace.netlist_source.as_deref(),
            Some("deck\nV1 out 0 1\nR1 out 0 1k\n.op\n.end\n")
        );
        assert_eq!(
            state.workspace.netlist_source_path.as_deref(),
            Some(std::path::Path::new("bias.cir"))
        );
        assert!(state.workspace.netlist_source_dirty);
        assert!(state.workspace.any_dirty());
        assert!(!state.simulation.has_results());
        assert!(state.recent_files.is_empty());
    }

    #[test]
    fn empty_netlist_import_is_rejected_without_clearing_existing_state() {
        let mut state = AppState::default();
        state.workspace.netlist_source = Some("existing\n.op\n.end\n".to_owned());
        state.workspace.netlist_source_path = Some(std::path::PathBuf::from("existing.cir"));
        state.simulation.netlist_content = "existing\n.op\n.end\n".to_owned();

        let imported = apply_imported_netlist(&mut state, " \n\t".to_owned(), None, "empty.cir");

        assert!(!imported);
        assert_eq!(
            state.workspace.netlist_source.as_deref(),
            Some("existing\n.op\n.end\n")
        );
        assert_eq!(
            state.workspace.netlist_source_path.as_deref(),
            Some(std::path::Path::new("existing.cir"))
        );
        assert_eq!(state.simulation.netlist_content, "existing\n.op\n.end\n");
    }
}
