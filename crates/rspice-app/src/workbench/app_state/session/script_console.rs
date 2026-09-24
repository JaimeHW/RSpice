//! Automation console state — the command history and executor behind the
//! workbench console dock's Automation page.
//!
//! This is session state, not a view: `AppState` owns it so it survives a
//! dock being closed and reopened. The dock that renders it lives at
//! `workbench::docks::console`, which reaches down here for the model.

use crate::simulation::automation::CommandOutput;

/// State for the scripting console
#[derive(Clone, Default)]
pub struct ScriptConsoleState {
    pub input_buffer: String,
    pub history: Vec<ConsoleHistoryItem>,
}

#[derive(Clone, Default)]
pub struct ConsoleHistoryItem {
    pub command: String,
    pub output: CommandOutput,
}
