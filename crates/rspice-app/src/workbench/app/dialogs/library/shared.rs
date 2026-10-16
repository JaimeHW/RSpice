//! Shared library-dialog helpers, including library/cell/view name
//! validation.

use super::{RSpiceApp, save_global_veriloga_library};
use crate::diagnostics::{LogSeverity, LogSource};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub(super) struct DialogActionOutcome {
    pub(super) close: bool,
    pub(super) persist_global_veriloga: bool,
}

pub(super) const LIBRARY_CATALOG_STALE_MESSAGE: &str =
    "The library catalog changed after this dialog opened. Cancel and reopen the operation.";

pub(super) fn validate_lcv_name(name: &str, field_label: &str) -> Option<String> {
    if name.is_empty() {
        return Some(format!("{field_label} cannot be empty"));
    }
    if !name.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return Some(format!(
            "{field_label} must contain only letters, numbers, and underscores"
        ));
    }
    None
}

impl RSpiceApp {
    pub(in crate::workbench::app) fn persist_global_veriloga_library_with_feedback(&mut self) {
        if let Err(err) =
            save_global_veriloga_library(&self.state.library_manager, &self.state.workspace)
        {
            self.state.record(
                LogSeverity::Warning,
                LogSource::Studio,
                format!("The Verilog-A library could not be saved: {err}"),
            );
        }
    }
}
