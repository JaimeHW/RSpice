use super::{ConsoleMessage, RSpiceApp, VERILOGA_LIBRARY_NAME};

impl RSpiceApp {
    pub(in crate::common::app) fn process_pending_library_deletions(&mut self) {
        if let Some((lib_name, cell_name)) = self.state.pending_delete_cell.take() {
            let mut deleted = false;
            if let Some(lib) = self.state.library_manager.get_library_mut(&lib_name) {
                deleted = lib.remove_cell(&cell_name);
                if deleted {
                    self.state.push_user_message(ConsoleMessage::info(format!(
                        "Deleted cell '{}' from library '{}'",
                        cell_name, lib_name
                    )));
                }
            }
            if deleted && lib_name == VERILOGA_LIBRARY_NAME {
                self.persist_global_veriloga_library_with_feedback();
            }
        }

        if let Some((lib_name, cell_name, view_name)) = self.state.pending_delete_view.take() {
            let mut deleted = false;
            if let Some(lib) = self.state.library_manager.get_library_mut(&lib_name) {
                if let Some(cell) = lib.get_cell_mut(&cell_name) {
                    deleted = cell.remove_view(&view_name);
                    if deleted {
                        self.state.push_user_message(ConsoleMessage::info(format!(
                            "Deleted view '{}' from cell '{}'",
                            view_name, cell_name
                        )));
                    }
                }
            }
            if deleted && lib_name == VERILOGA_LIBRARY_NAME {
                self.persist_global_veriloga_library_with_feedback();
            }
        }
    }
}
