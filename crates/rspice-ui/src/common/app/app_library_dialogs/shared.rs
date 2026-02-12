use super::{save_global_veriloga_library, ConsoleMessage, RSpiceApp};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub(super) struct DialogActionOutcome {
    pub(super) close: bool,
    pub(super) persist_global_veriloga: bool,
}

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
    pub(super) fn persist_global_veriloga_library_with_feedback(&mut self) {
        if let Err(err) = save_global_veriloga_library(&self.state.library_manager) {
            log::warn!("Failed to persist global Verilog-A library: {}", err);
            self.state
                .push_user_message(ConsoleMessage::warning(format!(
                    "Failed to persist Verilog-A library: {}",
                    err
                )));
        }
    }
}
